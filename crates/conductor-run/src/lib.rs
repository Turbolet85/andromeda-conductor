//! `conductor-run` — the run composition root shared by both shells.
//!
//! Drives one scenario (or a suite) to its [`RunRecord`]s. The MCP preflight gate is the pivot: when
//! the read-back path is unreachable or not ready, the scenario short-circuits to a `Blocked` record
//! (never a silent downgrade — arch §Read-Back Dependency Posture). On a ready gate it drives the
//! seeded timeline with the per-phase [`dispatch`]er attached, so each phase's declared emission
//! shape reaches the wire inside that phase's own window, then reads back and classifies. Both
//! `conductor-cli` (the release gate) and `conductor-tauri` (the GUI) call this identically — the
//! "headless-drivable core, thin shells" split (arch §Design Philosophy). Read-back is per-check:
//! `conductor_verify::observe` composes one pass over the corpus tools and each check grades against
//! the value its `ComparisonKind` reads.
//!
//! The resolver is generic ([`execute_scenario`]`<R: PauseResolver>`) — the CLI passes its interactive
//! `CliResolver`, the GUI the core [`HeadlessResolver`] — never a trait object
//! (`PauseResolver::resolve` returns `impl Future`, so it is not object-safe; this mirrors the generic
//! `conductor_core::resolve_hold`).

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context as _;
use serde::Serialize;

use conductor_core::{
    CheckRecord, EmissionShape, EmissionSpec, EnvelopeStatus, FaultKindSpec, HoldPoint,
    LoadEnvelope, OBSERVED_HANDLES, PauseResolver, PreconditionObservation, Preconditions,
    PreconditionsStatus, ReportState, RunContract, RunContractStatus, RunRecord, Scenario, Verdict,
    now_rfc3339, redact_value, resolve_hold, resolve_under,
};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, ExceptionSpec, Frame, TraceEmitter,
    exception_trace_request, fingerprint, probe_egress, trace_request,
};
use conductor_faults::{FaultError, OTLP_INGEST_PORT, PortOccupier};
use conductor_report::{JournalWriter, RunReport, RunsDb};
use conductor_timeline::{PhaseTimeline, PhaseWindow, run_timeline_observed};

mod dispatch;
pub use dispatch::{DispatchError, Dispatcher};
use conductor_verify::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ContractManifest, Observation, ReadBackOutcome,
    ReadbackClient, ReadyState, ToolPresence, VerifyError, evaluate_check, observe, run_preflight,
    sidecar_resolves_on_path,
};

/// The suite-wide preflight outcome — established once, reused by every scenario in a run.
pub struct Preflight {
    client: Option<ReadbackClient>,
    ready: bool,
}

/// Establish the read-back session + readiness gate once. A failed spawn / handshake (the read-back
/// path unreachable) or an unsatisfied gate yields `ready = false` — a Blocked precondition, never a
/// harness `Err`. A missing / invalid contract manifest on a connected path IS a harness fault.
pub async fn preflight(manifest_path: &Path) -> anyhow::Result<Preflight> {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(PathBuf::from);
    let data_dir_str = data_dir.as_ref().map(|p| p.display().to_string()).unwrap_or_default();

    let client = match ReadbackClient::connect(data_dir).await {
        Ok(client) => client,
        Err(_) => {
            tracing::info!("preflight blocked: MCP read-back path unreachable");
            return Ok(Preflight { client: None, ready: false });
        }
    };

    let state = canary_gate(&client, manifest_path, &data_dir_str).await?;
    if !state.ready {
        tracing::info!("preflight blocked: readiness gate not satisfied");
    }
    Ok(Preflight { client: Some(client), ready: state.ready })
}

/// The full readiness result for the `conductor preflight` verb — the serializable arch readiness
/// shape (arch §Standard Contracts). Reuses the hardened `ReadbackClient::connect` + `run_preflight`;
/// an unreachable read-back path yields a Blocked `ReadyState`, never an `Err`.
pub async fn readiness(manifest_path: &Path) -> anyhow::Result<ReadyState> {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(PathBuf::from);
    let data_dir_str = data_dir.as_ref().map(|p| p.display().to_string()).unwrap_or_default();
    match ReadbackClient::connect(data_dir).await {
        Ok(client) => canary_gate(&client, manifest_path, &data_dir_str).await,
        Err(_) => {
            tracing::info!("preflight blocked: MCP read-back path unreachable");
            let manifest =
                ContractManifest::load(manifest_path).context("load MCP contract manifest")?;
            Ok(unreachable_state(&manifest, &data_dir_str))
        }
    }
}

/// The Blocked `ReadyState` for an unreachable read-back path — mirrors `preflight_boot`'s Err arm
/// (the canonical precondition is arch §Standard Contracts).
fn unreachable_state(manifest: &ContractManifest, data_dir: &str) -> ReadyState {
    const UNREACHABLE_PRECONDITION: &str =
        "mcp-server cargo feature + ANDROMEDA_PULSE_MCP_ENABLED + ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir";
    ReadyState {
        ready: false,
        negotiated_protocol_version: None,
        expected_protocol_version: manifest.expected_protocol_version.clone(),
        required_tools: manifest
            .required_tools
            .iter()
            .map(|name| (name.clone(), ToolPresence::Absent))
            .collect(),
        data_dir: redact_value(data_dir).into_owned(),
        canary_round_trip: CanaryOutcome::Skipped,
        blocked_precondition: Some(UNREACHABLE_PRECONDITION.to_string()),
        checked_at: now_rfc3339(),
    }
}

/// Identical-fingerprint canary exceptions emitted as the storm. Clearing Pulse's retry-storm cue floor
/// (P-018) is NOT sufficient: the floor only raises a Suggested cue, and Pulse's Tier-1 coordinator
/// accepts Autonomous cues alone (triage `cadence/coordinator.rs`), so a storm in the Suggested band
/// forms no incident and the gate can never reach `ready:true`. Sized with headroom over triage's
/// `DEFAULT_AUTONOMOUS_THRESHOLD` (`pattern/storm.rs`), which is compared with `>=`, and small enough
/// that the unpaced burst lands well inside one `DEFAULT_DETECTION_SUB_WINDOW_SECONDS`.
pub const CANARY_STORM_COUNT: u64 = 12;

/// The canary's own service identity — deliberately DISTINCT from the dispatcher's scenario
/// service. Pulse keys `persistence_seconds` (the service's cumulative sample count,
/// `andromeda-pulse crates/triage/src/cue/evaluate.rs:55` at HEAD `efabe8e`) and its error-rate
/// EWMAs per service, so a shared identity would age every scenario's suppression semantics and
/// inflate its baseline denominator with preflight storm traffic before phase 1 ever emits.
pub const CANARY_SERVICE_NAME: &str = "conductor-canary";

/// What one resolve-lifecycle probe observed: the active set before the write, the id it resolved,
/// and the active set after. The two sets are what the verdict is computed from — the write's own
/// `{resolved, incident_id}` answer says only that Pulse accepted the call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleObservation {
    pub before: Vec<i64>,
    pub resolved: i64,
    pub after: Vec<i64>,
}

/// The verdict a lifecycle probe earns. The two `Proven*` arms attribute the active-set change to
/// Conductor's write; every other arm names why it cannot.
///
/// `Eq` is deliberately absent: `ProvenByLiveness` carries the measured idle seconds, and a float
/// has no total equality. Comparisons in tests use `PartialEq` on exact captured values.
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleVerdict {
    /// The resolved id left the active set AND a control id stayed — Pulse's own auto-resolver
    /// cannot produce this, because it would have to spare exactly the incident we did not write to.
    ///
    /// UNREACHABLE against Pulse as measured (2026-09-01, HEAD `83d4060`): a second concurrent
    /// incident cannot be formed, because the producer dedupes against any OPEN incident regardless
    /// of fingerprint. Kept because it is the stronger attribution if that ever changes.
    Proven { control: i64 },
    /// The resolved id left the active set while its telemetry was FRESH. Pulse auto-resolves only
    /// an incident idle for `AUTO_RESOLVE_IDLE_SECONDS`, so a removal inside that window cannot be
    /// the resolver's — which makes Conductor's write the only remaining cause. This is the
    /// attribution the single-incident reality actually supports.
    ProvenByLiveness { idle_seconds: f64 },
    /// The resolved id left, but no control survived. Indistinguishable from the 120s idle
    /// auto-resolve (30s resolver tick), so it proves nothing on its own.
    Unattributable,
    /// The resolved id is still active — the write did not take effect.
    StillActive,
    /// Fewer than two incidents were active, so no control could be held back.
    NoControl,
}

/// Choose the incident to resolve, keeping a control behind. Returns `(resolve, control)` — the
/// LAST id is resolved and the FIRST held, so the control is the older incident (typically the
/// preflight canary's, which is already open when a scenario's own storm forms).
///
/// Fewer than two active incidents yields `None`: a single-incident probe cannot separate Conductor's
/// write from Pulse's auto-resolver, and running it anyway would mint evidence that reads as proof.
pub fn select_resolve_target(active: &[i64]) -> Option<(i64, i64)> {
    match active {
        [control, .., resolve] => Some((*resolve, *control)),
        _ => None,
    }
}

/// Pulse's auto-resolve idle threshold. An incident whose telemetry is fresher than this cannot be
/// the resolver's to take, which is what lets a single-incident leg attribute the removal.
pub const AUTO_RESOLVE_IDLE_SECONDS: f64 = 120.0;

/// Grade a lifecycle observation by LIVENESS — the attribution available when only one incident can
/// be active at a time.
///
/// `idle_seconds` is the age of the resolved incident's most recent emission at the instant of the
/// write. Below [`AUTO_RESOLVE_IDLE_SECONDS`] the auto-resolver is excluded by construction, so the
/// id leaving the active set is attributable to Conductor's write. At or above it the observation is
/// real but `Unattributable`: the resolver could have produced exactly the same disappearance.
pub fn attribute_by_liveness(
    observation: &LifecycleObservation,
    idle_seconds: f64,
) -> LifecycleVerdict {
    if observation.before.is_empty() {
        return LifecycleVerdict::NoControl;
    }
    if observation.after.contains(&observation.resolved) {
        return LifecycleVerdict::StillActive;
    }
    if idle_seconds < AUTO_RESOLVE_IDLE_SECONDS {
        LifecycleVerdict::ProvenByLiveness { idle_seconds }
    } else {
        LifecycleVerdict::Unattributable
    }
}

/// Grade a lifecycle observation. The control's SURVIVAL is the load-bearing half: `query_incident_list`
/// is active-only, so a resolved incident leaves the surface — but so does an auto-resolved one, and
/// only a spared control separates the two.
pub fn evaluate_lifecycle(observation: &LifecycleObservation, control: i64) -> LifecycleVerdict {
    if observation.before.len() < 2 {
        return LifecycleVerdict::NoControl;
    }
    if observation.after.contains(&observation.resolved) {
        return LifecycleVerdict::StillActive;
    }
    if observation.after.contains(&control) {
        LifecycleVerdict::Proven { control }
    } else {
        LifecycleVerdict::Unattributable
    }
}

/// Read the active set, resolve one incident by id, and read it back — the first production caller of
/// `mark_incident_resolved`.
///
/// ORDERING: `query_incident_list` is active-only, so this empties what it resolves. Run it LAST in a
/// run, or in a run of its own; anything reading back afterwards sees the shrunken set.
///
/// A declined write is Pulse REFUSING, not a Conductor fault — it returns `Err(VerifyError::JsonRpc)`
/// for the caller to grade, never a panic.
pub async fn probe_resolve_lifecycle(
    client: &ReadbackClient,
    resolve: i64,
) -> Result<LifecycleObservation, VerifyError> {
    let before = active_incident_ids(client).await?;
    // The incident id is NOT in `conductor-core::redact::ALLOWLISTED_FIELDS`, so it rides the
    // allowlisted `message` rather than a span attribute the processor stage would drop.
    tracing::info!(message = %format!("resolve-lifecycle: resolving incident {resolve}"));
    let _ = client.resolve_incident(resolve).await?;
    let after = active_incident_ids(client).await?;
    Ok(LifecycleObservation { before, resolved: resolve, after })
}

/// The active incident ids from a `query_incident_list` result.
///
/// Accepts EITHER item key. The live sidecar emits `incident_id` (measured 2026-09-01 against Pulse
/// HEAD `83d4060`); the in-process stub emits `id`. Reading only one of them yields an empty list on
/// a populated corpus — a silent degrade indistinguishable downstream from a genuinely empty active
/// set, which is exactly how this reader shipped its first live leg. `conductor-verify`'s own
/// `incident_ids` already carries the same tolerance.
async fn active_incident_ids(client: &ReadbackClient) -> Result<Vec<i64>, VerifyError> {
    let list = client.query_incident_list(None).await?;
    Ok(list
        .get("items")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|i| {
                    i.get("incident_id").or_else(|| i.get("id")).and_then(|v| v.as_i64())
                })
                .collect()
        })
        .unwrap_or_default())
}

/// Seed for the `i`-th canary storm occurrence — ascending from `base`.
pub fn canary_storm_seed(base: u64, i: u64) -> u64 {
    base.wrapping_add(i)
}

/// Seed for the `i`-th canary warm-up emission — DESCENDING from `base` while
/// [`canary_storm_seed`] ascends, so the two ranges cannot overlap. Span identity is a function of
/// the seed, and a receiver keying its span store on `(trace_id, span_id)` drops a repeat instead of
/// storing it, so an overlap would silently cost the storm an occurrence.
pub fn canary_warmup_seed(base: u64, i: u32) -> u64 {
    base.wrapping_sub(u64::from(i) + 1)
}

/// The canary's synthetic exception. `marker` is the unique-per-preflight `exception.type`, which is
/// what makes the fingerprint unique to this run (a stale corpus cannot satisfy the gate on a prior
/// run's canary); the frame path is relative by construction.
pub fn canary_spec(marker: &str) -> ExceptionSpec {
    ExceptionSpec::new(
        marker,
        "conductor preflight canary",
        vec![Frame::new("conductor::run::preflight_canary", "conductor-run/src/lib.rs", 1)],
    )
}

/// Emit the canary storm through `traces` — [`CANARY_STORM_COUNT`] occurrences seeded from `base`, so
/// each carries a DISTINCT span identity while the fingerprint, a pure function of content, stays
/// identical across all of them. The transport is injected rather than fixed at
/// [`DEFAULT_OTLP_ENDPOINT`], so this exact emission loop is assertable against an ephemeral loopback
/// stub instead of only against a live Pulse.
pub async fn emit_canary_storm(
    traces: &mut TraceEmitter,
    spec: &ExceptionSpec,
    base: u64,
) -> anyhow::Result<()> {
    for i in 0..CANARY_STORM_COUNT {
        traces
            .export(exception_trace_request(CANARY_SERVICE_NAME, canary_storm_seed(base, i), spec))
            .await?;
    }
    Ok(())
}

/// Emit the canary fingerprint-storm, then run the readiness gate over it. Emission failure (OTLP
/// egress down) is a Blocked precondition, never a harness `Err`; a missing / invalid manifest IS a
/// harness fault. Shared by [`preflight`] + [`readiness`].
async fn canary_gate(
    client: &ReadbackClient,
    manifest_path: &Path,
    data_dir_str: &str,
) -> anyhow::Result<ReadyState> {
    let manifest = ContractManifest::load(manifest_path).context("load MCP contract manifest")?;
    let contract = load_run_contract().context("load Pulse run contract")?;
    let status = observe_run_contract(&contract);
    // An unmet launch condition means no incident can form, so the warm-up would only spend its
    // window to reach the same block — emit the storm, skip the wait.
    let canary = match emit_canary(&contract, status.is_satisfied()).await {
        Ok(canary) => canary,
        Err(e) => {
            tracing::info!(
                "preflight blocked: canary emission failed ({})",
                redact_value(&e.to_string())
            );
            return Ok(canary_blocked_state(&manifest, data_dir_str));
        }
    };
    Ok(run_preflight(client, &manifest, &status, &canary, data_dir_str, canary_poll(&contract))
        .await?)
}

/// Resolve + load the recorded run contract. A fixed in-repo path with no `CONDUCTOR_*` override,
/// guarded the same way as the capability manifest and load envelope; a read / parse / bounds
/// failure is a harness fault, never a silent default.
fn load_run_contract() -> anyhow::Result<RunContract> {
    let base = std::env::current_dir().context("resolve current directory")?;
    let path = resolve_under(&base, &RunContract::default_path())?;
    Ok(RunContract::load(&path)?)
}

/// Evaluate the contract against what Conductor can honestly observe: declarations in its OWN
/// environment — the shell that also launches `pulse-app`. Never a claim about `pulse-app` itself.
fn observe_run_contract(contract: &RunContract) -> RunContractStatus {
    let declared = contract
        .observed_env()
        .into_iter()
        .filter(|name| declares(name))
        .map(str::to_string)
        .collect();
    contract.evaluate(&declared)
}

/// Whether an env var carries an affirmative declaration. Presence alone is not enough — an
/// explicit `false` declares the opposite of the term it would otherwise satisfy.
fn declares(name: &str) -> bool {
    std::env::var(name).is_ok_and(|v| {
        let v = v.trim().to_ascii_lowercase();
        v == "true" || v == "1"
    })
}

/// Probe the live-Pulse preconditions BEFORE a leg is scheduled, so an absent SUT is named once
/// rather than absorbed by each chunk, gate and wrap in turn.
///
/// Deliberately NOT routed through [`canary_gate`]: a preflight fires its own storm and Pulse
/// dedupes a new incident against any open one on the `(kind, scope, scope_id)` tuple, so probing
/// via the gate would prime exactly the state the following leg collides with (architecture
/// §Established Decisions [Read-Back Dependency Posture]). Nothing here emits, binds or spawns.
///
/// The three observations happen HERE and enter the evaluator as values, keeping the judgment a
/// pure function of its inputs (the [`observe_run_contract`] shape above).
pub async fn observe_preconditions() -> PreconditionsStatus {
    let observation = PreconditionObservation {
        egress_reachable: probe_egress(DEFAULT_OTLP_ENDPOINT).await.is_ok(),
        sidecar_resolved: sidecar_resolves_on_path(),
        declared: OBSERVED_HANDLES
            .iter()
            .filter(|name| declares(name))
            .map(|name| (*name).to_string())
            .collect(),
    };

    let status = Preconditions::evaluate(&observation);

    // A boundary fact inside the caller's own span — the bounded span-name set is a deliberate
    // contract and is not widened for a new boundary (obs-plan §6 Boundary-call wrappers). The
    // unmet subjects ride `message`; no field here carries a path, a PATH value or an env value.
    if status.is_satisfied() {
        tracing::info!("live-Pulse preconditions satisfied");
    } else {
        let unmet =
            status.unmet().iter().map(|u| u.subject.id()).collect::<Vec<_>>().join(", ");
        tracing::warn!("live-Pulse preconditions unmet: {unmet}");
    }

    status
}

/// Emit a unique fingerprint-storm to Pulse's loopback ingest and return the [`CanaryMarker`] the gate
/// asserts against. Each occurrence carries a distinct span identity but the same fingerprint, so Pulse
/// counts a storm (security-plan §Threat Model).
///
/// The gate's carrier is the marker's emission STAMP, taken after any warm-up and immediately before
/// the counted storm, so only an incident opened past that instant satisfies it. Neither the marker
/// nor the fingerprint can carry it: Pulse scrubs incident titles, and its computed fingerprint
/// reaches no read-back surface (`fingerprint_refs` is L4-authored and payload-invariant under deterministic L4 — a constant `det-*` triple).
async fn emit_canary(contract: &RunContract, warm_up: bool) -> anyhow::Result<CanaryMarker> {
    let marker = format!("ConductorCanary_{}", now_ms());
    let spec = canary_spec(&marker);
    let fp = fingerprint(&spec);
    // Message-borne (the key-set-witness channel): the value must reach the self-obs artifact so a
    // live leg can compare it against the 8-hex prefix Pulse's storm line carries — the one surface
    // where the transcribed derivation meets Pulse's own (architecture §Read-Back Dependency Posture).
    tracing::info!("canary fingerprint computed {fp}");
    let base = now_ms() as u64;
    let mut traces = TraceEmitter::connect(DEFAULT_OTLP_ENDPOINT).await?;
    if warm_up {
        warm_up_canary_service(&mut traces, contract, base).await?;
    }
    let emitted_at = now_unix_nanos();
    emit_canary_storm(&mut traces, &spec, base).await?;
    Ok(CanaryMarker::new(marker, fp, emitted_at))
}

/// Spread benign non-error spans across the contract's warm-up window before the counted storm.
///
/// Its recorded purpose — carrying the service out of Pulse's baseline bootstrap — is DISPROVED twice
/// over: that gate needs 3,600s per service wall-clock, which no seconds-scale pre-roll reaches, and
/// the canary's RetryStorm path consults no baseline at all, so it never crossed that gate. Retained
/// because it is harmless and gives the service prior traffic; it is not what makes the canary work.
async fn warm_up_canary_service(
    traces: &mut TraceEmitter,
    contract: &RunContract,
    base: u64,
) -> anyhow::Result<()> {
    let terms = &contract.incident_formation;
    if terms.warmup_ms == 0 || terms.warmup_emissions == 0 {
        return Ok(());
    }
    let gap = std::time::Duration::from_millis(terms.warmup_ms / u64::from(terms.warmup_emissions));
    tracing::info!(count = terms.warmup_emissions, "warming the canary service out of bootstrap");
    for i in 0..terms.warmup_emissions {
        let seed = canary_warmup_seed(base, i);
        traces.export(trace_request(CANARY_SERVICE_NAME, seed, "canary-warmup")).await?;
        tokio::time::sleep(gap).await;
    }
    Ok(())
}

/// The canary poll budget, derived from `CONDUCTOR_PREFLIGHT_TIMEOUT` (seconds, default 30) — one
/// attempt per second so the gate waits out Pulse's storm-detection + ingest latency before Blocking.
/// The contract's floor raises it: L3's digest cadence alone is 20-60s with L4 behind it, so the
/// bare default could expire before an incident exists even once a cue fires. The env handle still
/// overrides upward; it can no longer sit below the SUT-derived floor.
fn canary_poll(contract: &RunContract) -> CanaryPoll {
    let secs = std::env::var("CONDUCTOR_PREFLIGHT_TIMEOUT")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(30)
        .max(contract.incident_formation.min_canary_poll_seconds)
        .max(1);
    CanaryPoll { attempts: secs as u32, interval: std::time::Duration::from_secs(1) }
}

/// The Blocked `ReadyState` when the canary cannot be emitted (OTLP egress to `:4317` unreachable) — a
/// distinct precondition from an unreachable read-back path, never a false pass.
fn canary_blocked_state(manifest: &ContractManifest, data_dir: &str) -> ReadyState {
    ReadyState {
        ready: false,
        negotiated_protocol_version: None,
        expected_protocol_version: manifest.expected_protocol_version.clone(),
        required_tools: manifest
            .required_tools
            .iter()
            .map(|name| (name.clone(), ToolPresence::Absent))
            .collect(),
        data_dir: redact_value(data_dir).into_owned(),
        canary_round_trip: CanaryOutcome::Skipped,
        blocked_precondition: Some(
            "canary emission failed: OTLP egress to 127.0.0.1:4317 unreachable".to_string(),
        ),
        checked_at: now_rfc3339(),
    }
}

/// Drive one scenario to its [`RunRecord`]: Blocked when the gate is not ready, else the coarse
/// live measured path (emit → read-back → classify). Generic over the resolver — the CLI passes its
/// `CliResolver`, the GUI a [`HeadlessResolver`] — so the run pipeline carries no shell dependency.
///
/// Carries the `scenario.run` root span (obs-plan §4 Critical Path 1). It sits here, at the
/// composition root, because all three production paths funnel through this fn — so `timeline.execute`,
/// `emit.batch` and `verify.readback*` nest beneath it identically headless and under Tauri.
/// `report.generate` / `db.insert_run` are NOT descendants: [`persist`] is a sibling of this fn, and
/// under a suite one `persist` serves N scenarios whose spans have already closed — they correlate by
/// `run_id` instead.
#[tracing::instrument(
    name = "scenario.run",
    skip_all,
    fields(
        run_id = %run_id,
        seed = scenario.seed,
        scenario = %scenario.name,
        p_ids = %scenario.p_ids.iter().map(|p| p.0.as_str()).collect::<Vec<_>>().join(","),
    )
)]
pub async fn execute_scenario<R: PauseResolver>(
    pf: &Preflight,
    scenario: &Scenario,
    run_id: &str,
    resolver: &R,
) -> anyhow::Result<ScenarioOutcome> {
    if !pf.ready {
        return Ok(ScenarioOutcome::without_checks(RunRecord::blocked(
            run_id,
            scenario.seed,
            &scenario.name,
            scenario.p_ids.clone(),
            scenario.slo_tier,
        )));
    }
    let client = pf.client.as_ref().expect("a ready gate implies a connected client");

    probe_egress(DEFAULT_OTLP_ENDPOINT).await.context("OTLP egress liveness to :4317")?;

    let emitted_ms = now_ms();
    let journal_emitted_at = now_rfc3339();
    let timeline = PhaseTimeline::from(scenario);
    let mut dispatcher = Dispatcher::connect(scenario, DEFAULT_OTLP_ENDPOINT)
        .await
        .context("OTLP emission egress")?;
    let mut occupy_failure: Option<FaultError> = None;
    run_timeline_observed(
        &timeline,
        scenario.seed,
        |window| phase_guard(scenario, &window, emitted_ms, OTLP_INGEST_PORT, &mut occupy_failure),
        async |point| dispatcher.dispatch(point).await,
    )
    .await
    .context("timeline scheduling")?;
    if let Some(refused) = occupy_failure {
        // The declared fault never applied, so no record exists to grade — a harness fault, never a
        // row (ratified at phase P4: Err after the timeline; the observer hook is infallible).
        return Err(anyhow::Error::new(refused))
            .context("the fault-declared phase could not apply its port occupier");
    }

    let observation = match route_read_back(observe(client).await, scenario.expected.is_empty()) {
        ReadBack::Graded(observation) => observation,
        ReadBack::AutoResolved => {
            tracing::info!(
                "declare-only read-back empty: no active incident outlived the emission window"
            );
            Observation { degraded: true, ..Observation::default() }
        }
        ReadBack::Blocked => {
            tracing::info!("scenario blocked: read-back yielded no gradable observation");
            return Ok(ScenarioOutcome::without_checks(RunRecord::blocked(
                run_id,
                scenario.seed,
                &scenario.name,
                scenario.p_ids.clone(),
                scenario.slo_tier,
            )));
        }
    };
    let observed_ms = now_ms();
    let read_back_observed_at = now_rfc3339();

    if scenario.expected.is_empty() {
        let hold = HoldPoint {
            scenario: scenario.name.clone(),
            p_id: scenario.p_ids[0].clone(),
            step: "operator-checklist".to_string(),
            prompt: "Observe the operator-checklist claim for this scenario".to_string(),
            allow_no_go: true,
            checklist: scenario.checklist.clone(),
        };
        let resolution = resolve_hold(resolver, &hold).await;
        // `info`, not `debug`: a hold resolution is a state transition (obs-plan §6), and the
        // default filter is INFO — a debug line is no witness at all. The resolver kind comes from
        // the resolution rather than being assumed, so the line stays true under an attended
        // resolver; both ride the allowlisted `message` field (obs-plan §4).
        tracing::info!(
            "operator-checklist hold resolved by {}: {} ({} checklist item(s))",
            resolution.resolver_kind,
            resolution.decision.label(),
            hold.checklist.len()
        );
        return Ok(ScenarioOutcome::without_checks(manual_record(
            scenario,
            run_id,
            journal_emitted_at,
            read_back_observed_at,
            observed_ms - emitted_ms,
            &observation,
        )));
    }

    // Every check is graded, not just the worst: the collapse below picks the scenario ROW, while
    // each outcome also lands as its own `CheckRecord`. They share one latency by construction —
    // the corpus is observed once — so a check's own `budget_ms` is what can separate its verdict.
    let outcomes: Vec<_> = scenario
        .expected
        .iter()
        .map(|check| {
            evaluate_check(
                check,
                &observation.observed_for(check.kind),
                scenario.slo_tier,
                emitted_ms,
                observed_ms,
            )
        })
        .collect();
    let checks = outcomes
        .iter()
        .zip(scenario.expected.iter())
        .enumerate()
        .map(|(index, (outcome, check))| {
            outcome.to_check_record(run_id, &scenario.name, index, check)
        })
        .collect();
    let chosen = outcomes
        .iter()
        .max_by_key(|outcome| severity_rank(outcome.assessment.verdict))
        .expect("expected is non-empty");
    let mut record = chosen.to_run_record(
        run_id,
        scenario.seed,
        &scenario.name,
        scenario.p_ids.clone(),
        journal_emitted_at,
        read_back_observed_at,
        observation.fingerprints.clone(),
    );
    record.state = state_for(&observation, record.state);
    Ok(ScenarioOutcome { record, checks })
}

/// A scenario's run outcome: the envelope row plus the per-check records behind it.
///
/// `checks` is empty for a blocked row and for a declare-only scenario — neither graded anything, so
/// there is nothing per-check to record (arch §Standard Contracts).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioOutcome {
    /// The scenario-level envelope row — the worst check's verdict, lamp chosen verdict-first.
    pub record: RunRecord,
    /// Each expected check's own outcome, in the scenario's declaration order.
    pub checks: Vec<CheckRecord>,
}

impl ScenarioOutcome {
    /// A row with no per-check grain behind it.
    fn without_checks(record: RunRecord) -> Self {
        Self { record, checks: Vec::new() }
    }
}

/// What one read-back outcome means for THIS scenario — a value in every case (the verdict/error
/// wall), decided without touching the client so both arms stay testable.
#[derive(Debug, PartialEq)]
enum ReadBack {
    /// The corpus held incidents: grade against what they carried.
    Graded(Observation),
    /// A declare-only scenario found no active incident. After a green gate — whose canary proved
    /// this run reaches the corpus — an emptied active list is Pulse's own auto-resolve lifecycle
    /// outliving nothing, not an unmet precondition, and a scenario that grades nothing cannot pass
    /// falsely on it. The pre-accepted residual.
    AutoResolved,
    /// Nothing gradable and no residual to claim: a named precondition, never a measured row. A
    /// scenario carrying checks stays here because an `Absent` check would pass trivially on an
    /// empty observation, and a failed call stays here whatever the scenario declares
    /// (security-plan §Anti-Patterns → Input: never a false pass-as-empty).
    Blocked,
}

fn route_read_back(outcome: ReadBackOutcome, declare_only: bool) -> ReadBack {
    match outcome {
        ReadBackOutcome::Observed(observation) => ReadBack::Graded(observation),
        ReadBackOutcome::EmptyCorpus if declare_only => ReadBack::AutoResolved,
        ReadBackOutcome::EmptyCorpus | ReadBackOutcome::CallFailed(_) => ReadBack::Blocked,
    }
}

/// The report state a read-back earns: a degraded response is the pre-accepted residual
/// (arch §Standard Contracts), overriding the state it would otherwise carry.
///
/// It overrides the STATE only — `verdict` is what was measured and stays independent, which is what
/// lets `Lamp::for_record` render the row verdict-first while still marking it residual. Degradation
/// is a property of the SUT's response, not of the scenario, so it applies wherever it is observed.
fn state_for(observation: &Observation, measured: ReportState) -> ReportState {
    if observation.degraded { ReportState::KnownResidual } else { measured }
}

/// An operator-checklist / declare-only scenario yields a verdict-less `ManualCheck` record
/// (`Lamp::for_record` maps `(ManualCheck, None) → Manual`).
///
/// A degraded read-back overrides the state to `KnownResidual` — the pre-accepted residual
/// (arch §Standard Contracts). The two states are distinct terminals: `ManualCheck` awaits a human,
/// `KnownResidual` records a measured, already-accepted deviation. The verdict stays `None` either
/// way: a declare-only scenario asserts nothing, so degradation changes what the row MEANS, not what
/// it measured.
fn manual_record(
    scenario: &Scenario,
    run_id: &str,
    journal_emitted_at: String,
    read_back_observed_at: String,
    latency_ms: i64,
    observation: &Observation,
) -> RunRecord {
    RunRecord {
        journal_emitted_at: Some(journal_emitted_at),
        read_back_observed_at: Some(read_back_observed_at),
        run_id: run_id.to_string(),
        seed: scenario.seed,
        scenario: scenario.name.clone(),
        p_ids: scenario.p_ids.clone(),
        verdict: None,
        state: state_for(observation, ReportState::ManualCheck),
        latency_ms: Some(latency_ms),
        slo_tier: scenario.slo_tier,
        fingerprints: Some(observation.fingerprints.clone()),
    }
}

fn severity_rank(verdict: Verdict) -> u8 {
    match verdict {
        Verdict::Pass => 0,
        Verdict::CalibrationRegion => 1,
        Verdict::Fail => 2,
    }
}

fn now_ms() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// Which fault a phase applies, if any — the classification behind the `fault.*` spans.
#[derive(Debug, Clone, Copy, PartialEq)]
enum FaultKind {
    /// A deliberate silence window: the phase declares no emissions and its gap simply elapses.
    Silence,
    /// A traffic ramp, carrying its normalized signed slope.
    Ramp { factor: f64 },
}

impl FaultKind {
    /// The bounded `fault_type` label (obs-plan §4).
    fn label(self) -> &'static str {
        match self {
            Self::Silence => "silence",
            Self::Ramp { .. } => "ramp",
        }
    }
}

/// Classify a phase's declared emission as a fault application. Only the two kinds the run path can
/// apply are classified — `Breathing` is a sibling rate curve with no reserved span name, and the
/// port-occupier is instrumented where it binds (obs-plan §11 bounds the span-name set).
fn classify_fault(emission: &EmissionSpec) -> Option<FaultKind> {
    if emission.occurrences == 0 {
        return Some(FaultKind::Silence);
    }
    match emission.shape {
        EmissionShape::Ramp { from_rate, to_rate, .. } => {
            Some(FaultKind::Ramp { factor: ramp_factor(from_rate, to_rate) })
        }
        _ => None,
    }
}

/// A ramp's normalized signed slope: negative when the rate falls, `0.0` when it is flat, and ±1.0
/// at the extreme. Direction is part of the value, so a read of the log tells a rise from a fall.
fn ramp_factor(from_rate: u32, to_rate: u32) -> f64 {
    let peak = from_rate.max(to_rate);
    if peak == 0 {
        return 0.0;
    }
    (f64::from(to_rate) - f64::from(from_rate)) / f64::from(peak)
}

/// The `fault.*` span for a phase that applies one, held by the scheduler for that phase's window.
///
/// Created, never entered: entering it would re-parent every `emit.batch` raised during the phase
/// onto the fault span, and obs-plan §4 Critical Path 1 nests those beneath `timeline.execute`. The
/// offset is journal-relative against the run's `std::time` emission stamp, never the virtual clock.
fn fault_span(scenario: &Scenario, window: &PhaseWindow<'_>, emitted_ms: i64) -> Option<tracing::Span> {
    let emission = &scenario.phases.get(window.index)?.emission;
    let kind = classify_fault(emission)?;
    let fault_type = kind.label();
    let fault_duration_ms = window.gap.as_millis() as u64;
    let fault_start_offset_ms = now_ms().saturating_sub(emitted_ms).max(0) as u64;

    Some(match kind {
        FaultKind::Silence => tracing::info_span!(
            "fault.silence",
            fault_type,
            fault_duration_ms,
            fault_start_offset_ms
        ),
        FaultKind::Ramp { factor } => tracing::info_span!(
            "fault.ramp",
            fault_type,
            fault_duration_ms,
            fault_start_offset_ms,
            ramp_factor = factor
        ),
    })
}

/// What the scheduler holds for one phase's window: the phase's `fault.*` span, and the port
/// occupier when the phase declares one. The boundary drop IS the RAII release (obs-plan §4) —
/// both fields exist only to be held, hence the underscores.
struct PhaseGuard {
    _span: Option<tracing::Span>,
    _occupier: Option<PortOccupier>,
}

/// Build the value held for `window`. A fault-declaring phase binds its occupier here, at phase
/// open; a refused bind is recorded into `failure` — the observer hook is infallible by design, so
/// the harness fault surfaces after the timeline completes, never as a panic or a silent row.
/// `occupier_port` is a parameter so tests bind `:0` (never the real ingest port — test-plan §10);
/// the production call site passes [`OTLP_INGEST_PORT`].
fn phase_guard(
    scenario: &Scenario,
    window: &PhaseWindow<'_>,
    emitted_ms: i64,
    occupier_port: u16,
    failure: &mut Option<FaultError>,
) -> PhaseGuard {
    let occupier = scenario.phases.get(window.index).and_then(|p| p.fault).and_then(|fault| {
        match fault.kind {
            FaultKindSpec::PortOccupier => match PortOccupier::occupy(occupier_port) {
                Ok(occupier) => Some(occupier),
                Err(refused) => {
                    tracing::error!("port occupier could not bind: the port is already held");
                    failure.get_or_insert(refused);
                    None
                }
            },
        }
    });
    PhaseGuard { _span: fault_span(scenario, window, emitted_ms), _occupier: occupier }
}

/// Wall-clock unix nanos — the unit Pulse stamps `opened_at_unix_nano` in, so the canary's emission
/// instant compares directly against it. `std::time`, never the virtual clock.
fn now_unix_nanos() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as i64).unwrap_or(0)
}

/// Persist a run's records across the three artifacts: the JSONL journal (append), the `runs.db`
/// index (one row per scenario, plus one row per graded check), and a single Markdown report for the
/// run. Shared by both shells so the CLI and the GUI write an identical envelope for the same
/// scenario+seed (test-plan Path 7).
///
/// `checks` carries the per-check grain behind the rows; it is empty when nothing was graded (a
/// blocked run, or an all-declare-only suite).
pub fn persist(
    runs_dir: &Path,
    run_id: &str,
    records: &[RunRecord],
    checks: &[CheckRecord],
    envelope: &EnvelopeStatus,
) -> anyhow::Result<()> {
    let mut journal = JournalWriter::create(runs_dir, run_id)?;
    let db = RunsDb::open(runs_dir)?;
    for record in records {
        journal.append(record)?;
        db.insert(record)?;
    }
    for check in checks {
        journal.append_check(check)?;
        db.insert_check(check)?;
    }
    db.insert_envelope(run_id, envelope)?;
    RunReport::write(runs_dir, run_id, records, checks, envelope)?;
    Ok(())
}

/// Read back a run's load-envelope standing — the run-level qualifier the desktop run-report renders.
///
/// The read-side counterpart to [`persist`]'s `insert_envelope`. It lives here, not in the GUI shell,
/// because `conductor-tauri` has no `conductor-report` edge and the composition root already owns the
/// `RunsDb` seam (arch §Established Decisions [Module Boundaries]). A run that recorded no envelope row
/// yields `None` — an absence, never an `Err` (arch §Conventions — Error handling).
pub fn read_envelope(runs_dir: &Path, run_id: &str) -> anyhow::Result<Option<EnvelopeStatus>> {
    let db = RunsDb::open(runs_dir)?;
    let status = db.get_envelope(run_id)?;
    tracing::info!(
        run_id = %run_id,
        message = status.as_ref().map_or("no envelope row", |s| s.label()),
        "read run envelope"
    );
    Ok(status)
}

/// Judge a run against the pinned SUT load envelope, before it is driven.
///
/// A run is environment-suspect if ANY of its scenarios breaches the envelope: the qualifier is
/// about whether the run could be evidence at all, and one over-envelope scenario is enough to
/// stall the SUT's append path for the rest of it. Scenarios are judged in catalog order, so the
/// reported cause is deterministic.
///
/// This is a VALUE on every path — never an `Err`. Driving the SUT too hard is an outcome about the
/// run, not a Conductor fault (arch §Cross-cutting Patterns "Verdict/error wall").
pub fn classify_run(envelope: &LoadEnvelope, scenarios: &[Scenario]) -> EnvelopeStatus {
    scenarios
        .iter()
        .map(|s| envelope.classify(s))
        .find(EnvelopeStatus::is_suspect)
        .unwrap_or(EnvelopeStatus::InEnvelope)
}

/// A live run-progress event streamed to the GUI (the Tauri `Channel` payload). `count` is the number
/// of scenarios completed so far — the live counter the titlebar reads; the faithful per-signal
/// emission count is the Epoch-10 bridge (a Blocked run emits no signals, so it ticks only per
/// scenario).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct RunEvent {
    /// The progress stage this event reports.
    pub stage: RunStage,
    /// Scenarios completed so far in this run.
    pub count: u64,
}

/// The stage a [`RunEvent`] reports. Serializes to its lowercase name; the GUI maps `progress` → live,
/// the terminal stages (`blocked`/`done`) → idle, and `aborted` → aborted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStage {
    /// A scenario completed (or the run just started, `count = 0`).
    Progress,
    /// Terminal — every record blocked (no live Pulse).
    Blocked,
    /// Terminal — at least one record was measured.
    Done,
    /// Terminal — the operator stopped the run.
    Aborted,
}

/// Drive `scenarios` to their [`RunRecord`]s through `resolver`, persisting the run once and
/// streaming a [`RunEvent`] per scenario through `emit`; `should_abort` is polled between scenarios
/// (the GUI's stop button). The terminal stage is `Blocked` when every record blocked (the no-live-
/// Pulse spine), else `Done`. Generic over the resolver (`PauseResolver::resolve` returns
/// `impl Future`, so never `dyn`) — the GUI shell passes its `TauriResolver` to gate the
/// operator-checklist holds on the live-Pulse path; the agent/test path passes a
/// [`conductor_core::HeadlessResolver`] that never blocks. The faithful per-emission counter is the
/// Epoch-10 bridge; `count` here ticks per scenario.
#[allow(clippy::too_many_arguments)] // one parameter per distinct run input; bundling them would
// hide the run-level envelope standing behind a struct both shells would have to construct anyway
pub async fn drive_run<R, E, A>(
    pf: &Preflight,
    scenarios: &[Scenario],
    run_id: &str,
    runs_dir: &Path,
    envelope: &EnvelopeStatus,
    resolver: &R,
    mut emit: E,
    should_abort: A,
) -> anyhow::Result<Vec<RunRecord>>
where
    R: PauseResolver,
    E: FnMut(RunEvent),
    A: Fn() -> bool,
{
    emit(RunEvent { stage: RunStage::Progress, count: 0 });
    let mut records = Vec::with_capacity(scenarios.len());
    let mut checks = Vec::new();
    let mut aborted = false;
    for scenario in scenarios {
        if should_abort() {
            aborted = true;
            break;
        }
        let outcome = execute_scenario(pf, scenario, run_id, resolver).await?;
        records.push(outcome.record);
        checks.extend(outcome.checks);
        emit(RunEvent { stage: RunStage::Progress, count: records.len() as u64 });
    }
    // Polled again after the last scenario: the loop-head check alone cannot see a stop pressed DURING
    // the final scenario, so such a run settled `Done` and the GUI's announced abort was overwritten
    // by `idle` (NVDA pass 2026-09-02, S3-05).
    if !aborted && should_abort() {
        aborted = true;
    }
    persist(runs_dir, run_id, &records, &checks, envelope)?;
    let stage = if aborted {
        tracing::info!(run_id, count = records.len(), "run aborted by the operator");
        RunStage::Aborted
    } else if records.iter().all(|r| matches!(r.state, ReportState::Blocked)) {
        RunStage::Blocked
    } else {
        RunStage::Done
    };
    emit(RunEvent { stage, count: records.len() as u64 });
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    use conductor_core::{CheckKind, ContractTerm, HeadlessResolver, IncidentFormation};
    use opentelemetry_proto::tonic::collector::trace::v1::{
        ExportTraceServiceRequest, ExportTraceServiceResponse,
    };

    /// A blocked-gate `Preflight` (no connected client) — the no-live-Pulse spine, constructed
    /// directly so the test needs neither a sidecar nor an env handle.
    fn blocked_preflight() -> Preflight {
        Preflight { client: None, ready: false }
    }

    fn observation(degraded: bool) -> Observation {
        Observation {
            text: "active\nRetryStorm".to_string(),
            evidence_count: 6,
            degraded,
            fingerprints: vec!["fp-1".to_string()],
        }
    }

    #[test]
    fn a_degraded_read_back_overrides_the_state_and_leaves_the_verdict_alone() {
        // Every state a measured row can carry is overridden to the residual...
        for measured in [ReportState::Pass, ReportState::Fail, ReportState::ManualCheck] {
            assert_eq!(state_for(&observation(true), measured), ReportState::KnownResidual);
        }
        // ...and an undegraded read-back changes nothing.
        for measured in [ReportState::Pass, ReportState::Fail, ReportState::ManualCheck] {
            assert_eq!(state_for(&observation(false), measured), measured);
        }
    }

    #[test]
    fn a_declare_only_empty_read_back_routes_to_the_auto_resolve_residual() {
        assert_eq!(route_read_back(ReadBackOutcome::EmptyCorpus, true), ReadBack::AutoResolved);

        // What the arm hands the declare-only path, and the row it earns: measured, pre-accepted,
        // verdict-less — never the Blocked row a missing precondition would produce.
        let r = manual_record(
            &fixture(1),
            "2026-08-20T00-00-00-abc",
            "2026-08-20T00:00:00Z".to_string(),
            "2026-08-20T00:00:03Z".to_string(),
            3_000,
            &Observation { degraded: true, ..Observation::default() },
        );
        assert_eq!(r.state, ReportState::KnownResidual);
        assert_eq!(r.verdict, None);
        assert_eq!(r.fingerprints, Some(vec![]));
        assert_eq!(r.latency_ms, Some(3_000));
    }

    #[test]
    fn a_checks_bearing_empty_read_back_stays_blocked() {
        // The false-pass guard: an `Absent` check would grade trivially true against an empty
        // observation, so only a scenario that grades NOTHING may claim the residual.
        assert_eq!(route_read_back(ReadBackOutcome::EmptyCorpus, false), ReadBack::Blocked);
    }

    #[test]
    fn a_failed_read_back_call_stays_blocked_even_for_a_declare_only_scenario() {
        // Transport trouble is not a pre-accepted residual, whatever the scenario declares.
        let failed = ReadBackOutcome::CallFailed("transport refused".to_string());
        assert_eq!(route_read_back(failed.clone(), true), ReadBack::Blocked);
        assert_eq!(route_read_back(failed, false), ReadBack::Blocked);
    }

    #[test]
    fn a_populated_corpus_grades_whatever_the_scenario_declares() {
        let o = observation(false);
        assert_eq!(
            route_read_back(ReadBackOutcome::Observed(o.clone()), true),
            ReadBack::Graded(o.clone())
        );
        assert_eq!(route_read_back(ReadBackOutcome::Observed(o.clone()), false), ReadBack::Graded(o));
    }

    #[test]
    fn an_operator_checklist_row_stays_manual_check_on_an_undegraded_read_back() {
        // The regression guard: routing degraded_mode must not sweep the declare-only scenarios
        // (7 of the 9 empty-`expected` catalog entries are operator-checklist, not residual).
        let r = manual_record(
            &fixture(1),
            "2026-08-13T00-00-00-abc",
            "2026-08-13T00:00:00Z".to_string(),
            "2026-08-13T00:00:01Z".to_string(),
            1_000,
            &observation(false),
        );
        assert_eq!(r.state, ReportState::ManualCheck);
        assert_eq!(r.verdict, None);
        assert_eq!(r.fingerprints, Some(vec!["fp-1".to_string()]));
    }

    #[test]
    fn every_check_survives_the_collapse_not_just_the_worst() {
        use conductor_core::{ClaimClass, ComparisonKind, ExpectedCheck, SloTier};

        // Two checks, one observation instant, different budgets: the worst verdict governs the
        // scenario ROW, while both outcomes still reach the per-check grain. Before this chunk the
        // passing check was discarded at `max_by_key` and reached no sink at all.
        let checks = [
            ExpectedCheck {
                kind: ComparisonKind::Contains,
                class: ClaimClass::Hard,
                expected: "RetryStorm".to_string(),
                budget_ms: Some(500),
            },
            ExpectedCheck {
                kind: ComparisonKind::Contains,
                class: ClaimClass::Hard,
                expected: "RetryStorm".to_string(),
                budget_ms: Some(4_000),
            },
        ];
        let o = observation(false);
        let (emitted, observed) = (0, 1_000);
        let outcomes: Vec<_> = checks
            .iter()
            .map(|c| evaluate_check(c, &o.observed_for(c.kind), SloTier::Tier90s, emitted, observed))
            .collect();
        let records: Vec<_> = outcomes
            .iter()
            .zip(checks.iter())
            .enumerate()
            .map(|(i, (out, c))| out.to_check_record("r", "two-check", i, c))
            .collect();

        assert_eq!(records.len(), 2, "both checks recorded, not just the surviving one");
        assert_eq!(records[0].check_index, 0);
        assert_eq!(records[1].check_index, 1);
        assert_eq!(records[0].latency_ms, records[1].latency_ms, "one observation, one latency");
        assert_eq!(records[0].verdict, Verdict::Fail, "1000ms missed its 500ms budget");
        assert_eq!(records[1].verdict, Verdict::Pass, "1000ms met its 4000ms budget");
        assert_eq!(records[0].deadline_ms, 500);
        assert_eq!(records[1].deadline_ms, 4_000);

        let worst = outcomes
            .iter()
            .max_by_key(|out| severity_rank(out.assessment.verdict))
            .unwrap();
        assert_eq!(worst.assessment.verdict, Verdict::Fail, "the row still carries the worst");
    }

    #[test]
    fn a_measured_row_keeps_its_verdict_when_degradation_overrides_the_state() {
        use conductor_core::{ClaimClass, ComparisonKind, ExpectedCheck, PId, SloTier};

        let check = ExpectedCheck {
            kind: ComparisonKind::Contains,
            class: ClaimClass::Hard,
            expected: "RetryStorm".to_string(),
            budget_ms: None,
        };
        let o = observation(true);
        let outcome =
            evaluate_check(&check, &o.observed_for(check.kind), SloTier::Tier5s, 0, 1_000);
        let mut record = outcome.to_run_record(
            "2026-08-13T00-00-00-abc",
            1,
            "degraded-fixture",
            vec![PId("P-053".to_string())],
            "2026-08-13T00:00:00Z",
            "2026-08-13T00:00:01Z",
            o.fingerprints.clone(),
        );
        record.state = state_for(&o, record.state);

        // The state says "pre-accepted residual"; the verdict still says what was measured. Keeping
        // them independent is what lets the report render verdict-first over a residual row.
        assert_eq!(record.verdict, Some(Verdict::Pass));
        assert_eq!(record.state, ReportState::KnownResidual);
        assert_eq!(record.fingerprints, Some(vec!["fp-1".to_string()]));
    }

    #[test]
    fn a_declare_only_row_under_a_degraded_read_back_is_residual_not_manual() {
        // P-053's routing: an empty-`expected` scenario never reaches `evaluate_check`, so the
        // record-level assignment is the only path that can mark it residual at all.
        let r = manual_record(
            &fixture(1),
            "2026-08-13T00-00-00-abc",
            "2026-08-13T00:00:00Z".to_string(),
            "2026-08-13T00:00:01Z".to_string(),
            1_000,
            &observation(true),
        );
        assert_eq!(r.state, ReportState::KnownResidual);
        assert_eq!(r.verdict, None, "a declare-only scenario asserts nothing — no verdict is invented");
    }

    fn fixture(seed: u64) -> Scenario {
        let toml = format!(
            "name = \"blocked-fixture\"\np_ids = [\"P-001\"]\nseed = {seed}\nslo_tier = \"<5s\"\njitter_ms = 0\n[[phases]]\nname = \"p1\"\ngap_ms = 100\n"
        );
        Scenario::from_toml_str(&toml).expect("fixture scenario validates")
    }

    fn named_fixture(name: &str, gap_ms: u64) -> Scenario {
        let toml = format!(
            "name = \"{name}\"\np_ids = [\"P-001\"]\nseed = 1\nslo_tier = \"<5s\"\njitter_ms = 0\n[[phases]]\nname = \"p1\"\ngap_ms = {gap_ms}\n"
        );
        Scenario::from_toml_str(&toml).expect("fixture scenario validates")
    }

    /// A single fault-declaring silence phase — the port-conflict shape.
    fn occupier_fixture() -> Scenario {
        let toml = "name = \"occupier-fixture\"\np_ids = [\"P-003\"]\nseed = 3\nslo_tier = \"<90s\"\njitter_ms = 0\n[[phases]]\nname = \"port-held\"\ngap_ms = 100\n[phases.emission]\nkind = \"plain\"\noccurrences = 0\n[phases.fault]\nkind = \"port_occupier\"\n";
        Scenario::from_toml_str(toml).expect("fixture scenario validates")
    }

    #[test]
    fn a_fault_phase_guard_binds_and_its_drop_releases() {
        let scenario = occupier_fixture();
        let window =
            PhaseWindow { index: 0, name: "port-held", gap: std::time::Duration::from_millis(100) };
        let mut failure = None;
        let guard = phase_guard(&scenario, &window, 0, 0, &mut failure);
        assert!(failure.is_none(), "an ephemeral bind succeeds");
        let addr = guard._occupier.as_ref().expect("the fault phase binds an occupier").local_addr();
        assert!(std::net::TcpListener::bind(addr).is_err(), "held while the guard lives");
        drop(guard);
        std::net::TcpListener::bind(addr).expect("the boundary drop released the bind");
    }

    #[test]
    fn a_plain_phase_guard_carries_no_occupier() {
        let scenario = fixture(7);
        let window =
            PhaseWindow { index: 0, name: "p1", gap: std::time::Duration::from_millis(100) };
        let mut failure = None;
        let guard = phase_guard(&scenario, &window, 0, 0, &mut failure);
        assert!(guard._occupier.is_none(), "no fault declaration, no bind");
        assert!(failure.is_none());
    }

    #[test]
    fn a_refused_occupy_is_captured_for_the_post_timeline_check() {
        let held = PortOccupier::occupy(0).expect("pre-hold an ephemeral port");
        let port = held.local_addr().port();
        let scenario = occupier_fixture();
        let window =
            PhaseWindow { index: 0, name: "port-held", gap: std::time::Duration::from_millis(100) };
        let mut failure = None;
        let guard = phase_guard(&scenario, &window, 0, port, &mut failure);
        assert!(guard._occupier.is_none(), "the held port refuses the second bind");
        assert!(matches!(failure, Some(FaultError::Bind { .. })), "captured, not panicked");
        drop(held);
    }

    /// The ratified occupy-failure policy's mechanics: the hook is infallible, the timeline
    /// completes, and the captured failure survives to the post-timeline check.
    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn occupy_failure_is_captured_across_the_timeline_and_surfaces_after() {
        let held = PortOccupier::occupy(0).expect("pre-hold an ephemeral port");
        let port = held.local_addr().port();
        let scenario = occupier_fixture();
        let timeline = PhaseTimeline::from(&scenario);
        let mut failure = None;
        run_timeline_observed(
            &timeline,
            scenario.seed,
            |window| phase_guard(&scenario, &window, 0, port, &mut failure),
            async |_| Ok::<(), std::convert::Infallible>(()),
        )
        .await
        .expect("a refused bind is not a scheduling error — the timeline completes");
        assert!(
            matches!(failure, Some(FaultError::Bind { .. })),
            "the failure survives to the post-timeline check"
        );
        drop(held);
    }

    /// `storm_ms` drives the asserted per-phase sustained-storm window; `named_fixture` builds a
    /// single emitting phase, so its gap is the storm.
    fn test_envelope(storm_ms: u64, exempt: &[(&str, &str)]) -> LoadEnvelope {
        LoadEnvelope {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-09".to_string(),
            provenance: "test".to_string(),
            envelope: conductor_core::EnvelopeTerms {
                max_sustained_rate_spans_per_s: 10_000,
                max_sustained_storm_ms: storm_ms,
                max_scenario_duration_ms: 600_000,
            },
            exempt: exempt
                .iter()
                .map(|(scenario, reason)| conductor_core::Exemption {
                    scenario: scenario.to_string(),
                    reason: reason.to_string(),
                })
                .collect(),
        }
    }

    #[test]
    fn read_envelope_round_trips_what_persist_wrote_and_is_none_for_an_unknown_run() {
        let dir = assert_fs::TempDir::new().unwrap();
        let suspect = EnvelopeStatus::EnvironmentSuspect("storm-scenario over the sustained bound".to_string());

        persist(dir.path(), "run-suspect", &[], &[], &suspect).unwrap();
        persist(dir.path(), "run-clean", &[], &[], &EnvelopeStatus::InEnvelope).unwrap();

        assert_eq!(read_envelope(dir.path(), "run-suspect").unwrap(), Some(suspect));
        assert_eq!(
            read_envelope(dir.path(), "run-clean").unwrap(),
            Some(EnvelopeStatus::InEnvelope),
            "an in-envelope run records a standing too — the banner's absence is the RENDERER's choice"
        );
        assert_eq!(
            read_envelope(dir.path(), "never-ran").unwrap(),
            None,
            "a run with no envelope row is an absence, never an Err"
        );
    }

    #[test]
    fn classify_run_flags_the_first_breaching_scenario_deterministically() {
        let envelope = test_envelope(1_000, &[("idle-long", "deliberate quiet")]);

        assert_eq!(
            classify_run(&envelope, &[named_fixture("a", 500), named_fixture("b", 900)]),
            EnvelopeStatus::InEnvelope
        );
        assert_eq!(
            classify_run(&envelope, &[named_fixture("idle-long", 9_000)]),
            EnvelopeStatus::InEnvelope,
            "a pinned exemption keeps the run in-envelope"
        );
        assert_eq!(classify_run(&envelope, &[]), EnvelopeStatus::InEnvelope);

        let suite = [named_fixture("ok", 500), named_fixture("first-breach", 5_000), named_fixture("second-breach", 9_000)];
        let status = classify_run(&envelope, &suite);
        let cause = status.cause().expect("a breaching suite names its cause");
        assert!(cause.contains("first-breach"), "catalog order decides the reported cause: {cause}");
        assert!(!cause.contains("second-breach"), "only the first breach is reported: {cause}");
    }

    /// `v2-07` — an over-envelope run is classified environment-suspect, distinct from `Fail`, and
    /// the classification round-trips through all three artifacts with the breach named as the cause.
    #[tokio::test(flavor = "current_thread")]
    async fn an_over_envelope_run_is_environment_suspect_not_fail() {
        let dir = assert_fs::TempDir::new().unwrap();
        let scenario = named_fixture("over-envelope-fixture", 900_000);
        let envelope = classify_run(&test_envelope(600_000, &[]), std::slice::from_ref(&scenario));
        assert!(envelope.is_suspect(), "the fixture must breach for this test to mean anything");

        let records = drive_run(
            &blocked_preflight(),
            std::slice::from_ref(&scenario),
            "run-envelope",
            dir.path(),
            &envelope,
            &HeadlessResolver::proceed(),
            |_| {},
            || false,
        )
        .await
        .expect("an envelope breach is never a harness Err");

        // distinct from Fail: the qualifier never rewrites a check state
        assert!(records.iter().all(|r| r.state != ReportState::Fail), "no row became a Fail");
        assert!(records.iter().all(|r| r.state == ReportState::Blocked), "states are unchanged");

        // the runs.db round-trip, read through a bound-parameter query
        let db = RunsDb::open(dir.path()).expect("runs.db opens");
        assert_eq!(db.get_envelope("run-envelope").unwrap(), Some(envelope.clone()));

        // the Markdown report names the breach as the cause
        let md = std::fs::read_to_string(dir.path().join("run-envelope.md")).unwrap();
        assert!(md.contains("[ENVIRONMENT-SUSPECT]"), "{md}");
        assert!(md.contains("over-envelope-fixture"), "the report names the breaching scenario: {md}");
        assert!(!md.contains("[FAIL]"), "an envelope breach never renders as Fail: {md}");

        // the JSONL journal is untouched by the run-level qualifier (eleven per-check fields)
        let journal = std::fs::read_to_string(dir.path().join("run-envelope.jsonl")).unwrap();
        assert!(journal.contains("\"Blocked\""), "the journal keeps its own envelope: {journal}");
        assert!(
            !journal.contains("ENVIRONMENT-SUSPECT"),
            "the run-level qualifier does not leak into the per-check journal: {journal}"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn an_in_envelope_run_records_its_standing_too() {
        let dir = assert_fs::TempDir::new().unwrap();
        drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-in-env",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |_| {},
            || false,
        )
        .await
        .unwrap();

        let db = RunsDb::open(dir.path()).unwrap();
        assert_eq!(
            db.get_envelope("run-in-env").unwrap(),
            Some(EnvelopeStatus::InEnvelope),
            "every run records a standing, so an absent row means a bug, not an in-envelope run"
        );
        let md = std::fs::read_to_string(dir.path().join("run-in-env.md")).unwrap();
        assert!(!md.contains("ENVIRONMENT-SUSPECT"), "{md}");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn execute_scenario_blocks_when_gate_not_ready() {
        let outcome =
            execute_scenario(&blocked_preflight(), &fixture(7), "run-test", &HeadlessResolver::proceed())
                .await
                .expect("the blocked path is infallible");
        let record = outcome.record;
        assert!(matches!(record.state, ReportState::Blocked));
        assert!(record.verdict.is_none(), "a blocked row carries no verdict");
        assert!(record.journal_emitted_at.is_none() && record.latency_ms.is_none());
        assert_eq!(record.seed, 7);
        assert_eq!(record.scenario, "blocked-fixture");
        assert!(outcome.checks.is_empty(), "a blocked row grades nothing, so it records no checks");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn blocked_envelope_is_seed_identified() {
        let pf = blocked_preflight();
        let a = execute_scenario(&pf, &fixture(7), "r", &HeadlessResolver::proceed()).await.unwrap();
        let b = execute_scenario(&pf, &fixture(7), "r", &HeadlessResolver::proceed()).await.unwrap();
        assert_eq!(a, b, "same scenario+seed ⇒ identical blocked envelope");
        let c = execute_scenario(&pf, &fixture(9), "r", &HeadlessResolver::proceed()).await.unwrap();
        assert_ne!(a.record.seed, c.record.seed, "the seed materially identifies the envelope");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_streams_progress_then_blocked_and_persists() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-drive",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| events.push(ev),
            || false,
        )
        .await
        .expect("drive_run is infallible on the blocked path");

        assert_eq!(records.len(), 1);
        assert!(matches!(records[0].state, ReportState::Blocked));
        assert_eq!(events.first().unwrap().stage, RunStage::Progress);
        assert_eq!(events.last().unwrap().stage, RunStage::Blocked);
        assert_eq!(events.last().unwrap().count, 1);
        assert!(
            dir.path().join("run-drive.jsonl").is_file(),
            "the JSONL journal was persisted"
        );
        let journal = std::fs::read_to_string(dir.path().join("run-drive.jsonl")).unwrap();
        assert!(journal.contains("\"Blocked\""), "the journal carries the Blocked envelope");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_honors_abort_before_the_first_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| events.push(ev),
            || true,
        )
        .await
        .unwrap();
        assert!(records.is_empty(), "an immediate abort runs no scenario");
        assert_eq!(events.last().unwrap().stage, RunStage::Aborted);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drive_run_reports_an_abort_raised_during_the_last_scenario() {
        let dir = assert_fs::TempDir::new().unwrap();
        // Stop is pressed while the FINAL scenario is executing, so the flag is only set once that
        // scenario has completed — the loop-head poll can never observe it.
        let stop = std::cell::Cell::new(false);
        let mut events = Vec::new();
        let records = drive_run(
            &blocked_preflight(),
            &[fixture(7)],
            "run-abort-last",
            dir.path(),
            &EnvelopeStatus::InEnvelope,
            &HeadlessResolver::proceed(),
            |ev| {
                if ev.stage == RunStage::Progress && ev.count == 1 {
                    stop.set(true);
                }
                events.push(ev);
            },
            || stop.get(),
        )
        .await
        .unwrap();

        assert_eq!(records.len(), 1, "the scenario already running still completes");
        assert_eq!(
            events.last().unwrap().stage,
            RunStage::Aborted,
            "a stop during the last scenario reports Aborted, never Done/Blocked"
        );
        assert_eq!(events.last().unwrap().count, 1);
        let journal = std::fs::read_to_string(dir.path().join("run-abort-last.jsonl")).unwrap();
        assert!(
            journal.contains("\"Blocked\""),
            "the completed scenario's envelope is persisted before the abort is reported"
        );
    }

    fn emission(occurrences: u32, shape: EmissionShape) -> EmissionSpec {
        EmissionSpec::shaped(conductor_core::Signal::Traces, occurrences, shape)
    }

    #[test]
    fn a_zero_occurrence_phase_classifies_as_silence_whatever_its_shape() {
        assert_eq!(classify_fault(&emission(0, EmissionShape::Plain)), Some(FaultKind::Silence));
        assert_eq!(
            classify_fault(&emission(0, EmissionShape::Ramp { from_rate: 1, to_rate: 9, windows: 3 })),
            Some(FaultKind::Silence),
            "a declared silence is silence even under a ramp shape — the gap is what elapses"
        );
    }

    #[test]
    fn only_the_two_run_path_faults_classify() {
        assert_eq!(
            classify_fault(&emission(4, EmissionShape::Ramp { from_rate: 10, to_rate: 100, windows: 5 })),
            Some(FaultKind::Ramp { factor: 0.9 })
        );
        assert_eq!(classify_fault(&emission(4, EmissionShape::Plain)), None);
        assert_eq!(
            classify_fault(&emission(4, EmissionShape::Breathing {
                center_rate: 50,
                amplitude: 10,
                period_windows: 2,
                windows: 4,
            })),
            None,
            "breathing is a sibling rate curve with no reserved span name (obs-plan §11)"
        );
    }

    #[test]
    fn the_ramp_factor_carries_direction_and_steepness() {
        assert_eq!(ramp_factor(10, 100), 0.9);
        assert_eq!(ramp_factor(100, 10), -0.9, "a falling ramp is distinguishable from a rising one");
        assert_eq!(ramp_factor(90, 100), 0.1);
        assert_eq!(ramp_factor(50, 50), 0.0);
        assert_eq!(ramp_factor(0, 0), 0.0, "the degenerate declaration yields a value, never a panic");
    }

    /// A wall-clock instant comfortably before this code existed, and one far past any plausible run.
    /// Both helpers end `unwrap_or(0)`, so a stamp that collapses to a small constant is
    /// indistinguishable from the genuine pre-epoch error path by sign alone — the assertion has to
    /// be on MAGNITUDE, which is what the journal-relative SLO arithmetic actually depends on.
    const PLAUSIBLE_FLOOR_MS: i64 = 1_700_000_000_000;
    const PLAUSIBLE_CEILING_MS: i64 = 4_000_000_000_000;

    #[test]
    fn the_journal_stamp_helpers_read_plausible_wall_clock_instants() {
        let ms = now_ms();
        assert!(
            (PLAUSIBLE_FLOOR_MS..PLAUSIBLE_CEILING_MS).contains(&ms),
            "now_ms must be epoch MILLIS from std::time, not a constant or a unit slip: {ms}"
        );

        let nanos = now_unix_nanos();
        assert!(
            (PLAUSIBLE_FLOOR_MS * 1_000_000..PLAUSIBLE_CEILING_MS * 1_000_000).contains(&nanos),
            "now_unix_nanos must be epoch NANOS from std::time, not a constant or a unit slip: {nanos}"
        );
    }

    #[test]
    fn the_two_journal_stamp_helpers_denote_the_same_instant() {
        // Mutation replaces one helper at a time, so holding the pair to each other kills a mutant in
        // EITHER even where its own magnitude bound might be met: they read the same clock moments
        // apart and must agree once reduced to a common unit.
        let ms = now_ms();
        let nanos = now_unix_nanos();
        let skew_ms = (nanos / 1_000_000 - ms).abs();
        assert!(
            skew_ms < 5_000,
            "the two stamps must denote one instant in different units; skew {skew_ms}ms \
             (ms={ms}, nanos={nanos})"
        );
    }

    #[test]
    fn the_journal_stamp_advances_across_a_real_pause() {
        // A stamp frozen at any constant satisfies both bounds above forever. Only movement across a
        // genuine wall-clock wait proves the helper reads the clock on every call — and this must be
        // `std::thread::sleep`, never tokio's virtual clock, which the journal basis may not use.
        let before = now_unix_nanos();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let after = now_unix_nanos();
        assert!(after > before, "the stamp must advance across a real pause: {before} -> {after}");
    }

    /// An env var no shell declares. The run-contract assertions below turn on its ABSENCE, so a
    /// real handle would make them pass or fail on the operator's environment instead of on the
    /// gate's logic — and reading env at the caller is what keeps this testable without `unsafe`
    /// (`.claude/rules/testing.md` 2026-08-10).
    const UNDECLARED_ENV: &str = "CONDUCTOR_UNDECLARED_RUN_CONTRACT_PROBE";

    fn shell_term(id: &str, env: &str) -> ContractTerm {
        ContractTerm {
            id: id.to_string(),
            statement: "a launch condition the runner's shell does not declare".to_string(),
            check: CheckKind::ShellDeclaration,
            env: Some(env.to_string()),
            causes: "the declaration is absent from this environment".to_string(),
        }
    }

    fn contract_of(terms: Vec<ContractTerm>) -> RunContract {
        RunContract {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-09-03".to_string(),
            provenance: "test fixture".to_string(),
            incident_formation: IncidentFormation {
                warmup_ms: 1_000,
                warmup_emissions: 4,
                min_canary_poll_seconds: 90,
            },
            terms,
        }
    }

    #[test]
    fn observe_run_contract_names_the_term_this_environment_does_not_declare() {
        let contract = contract_of(vec![shell_term("l4-deterministic", UNDECLARED_ENV)]);
        let status = observe_run_contract(&contract);

        assert!(!status.is_satisfied(), "an undeclared shell term is unmet");
        assert_eq!(status.unmet().len(), 1, "one term, one unmet entry");
        assert_eq!(status.unmet()[0].id, "l4-deterministic", "the gate names the term individually");
        assert_ne!(
            status,
            RunContractStatus::default(),
            "a status carrying an unmet term must be distinguishable from the empty default"
        );
    }

    #[test]
    fn observe_run_contract_is_satisfied_when_no_term_is_a_shell_declaration() {
        // Only a ShellDeclaration term can be unmet: a term whose truth lives on the SUT's side is
        // recorded and never blocks (arch §Standard Contracts).
        let contract = contract_of(vec![ContractTerm {
            id: "corpus-plaintext".to_string(),
            statement: "true or false entirely on the SUT's side".to_string(),
            check: CheckKind::DeclaredNotObservable,
            env: None,
            causes: "not observable from here".to_string(),
        }]);
        assert!(observe_run_contract(&contract).is_satisfied());
    }

    fn resolved_away(before: Vec<i64>, resolved: i64) -> LifecycleObservation {
        LifecycleObservation { before, resolved, after: Vec::new() }
    }

    #[test]
    fn liveness_attribution_turns_on_the_idle_threshold_exactly() {
        let observation = resolved_away(vec![7], 7);

        // Strictly BELOW the threshold Pulse's 120s auto-resolver is excluded by construction, so
        // the id leaving the active set is attributable to Conductor's write...
        assert_eq!(
            attribute_by_liveness(&observation, AUTO_RESOLVE_IDLE_SECONDS - 0.5),
            LifecycleVerdict::ProvenByLiveness { idle_seconds: AUTO_RESOLVE_IDLE_SECONDS - 0.5 }
        );

        // ...and AT the threshold it is not. This is the only value where `<` and `<=` disagree, so
        // it is the only case that separates the shipped bound from a relaxed one.
        assert_eq!(
            attribute_by_liveness(&observation, AUTO_RESOLVE_IDLE_SECONDS),
            LifecycleVerdict::Unattributable,
            "at the threshold the resolver could have produced the same disappearance"
        );
    }

    /// Serve a line-delimited JSON-RPC session over one end of an in-process duplex, answering
    /// `query_incident_list` with `items` — supplied independently of the request, because a stub
    /// that echoes what it was given answers a mutated reader as agreeably as a correct one
    /// (`.claude/rules/testing.md` 2026-08-20).
    async fn serve_incident_list(server_io: tokio::io::DuplexStream, items: Vec<i64>) {
        use tokio::io::{AsyncBufReadExt as _, AsyncWriteExt as _};

        let (reader, mut writer) = tokio::io::split(server_io);
        let mut lines = tokio::io::BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            let Ok(req) = serde_json::from_str::<serde_json::Value>(&line) else { continue };
            // A notification carries no id and earns no response.
            let Some(id) = req.get("id").cloned() else { continue };
            let result = match req.get("method").and_then(serde_json::Value::as_str) {
                Some("initialize") => serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": { "name": "stub", "version": "0" },
                }),
                _ => serde_json::json!({
                    "items": items
                        .iter()
                        .map(|id| serde_json::json!({ "incident_id": id }))
                        .collect::<Vec<_>>(),
                }),
            };
            let response = serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": result });
            if writer.write_all(format!("{response}\n").as_bytes()).await.is_err() {
                break;
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn active_incident_ids_reads_every_id_the_corpus_returns() {
        // The set is deliberately unlike each degenerate replacement — two entries, neither 0 nor 1
        // nor -1 — so `Ok(vec![])`, `Ok(vec![0])`, `Ok(vec![1])` and `Ok(vec![-1])` all differ from
        // it simultaneously.
        let expected = vec![41_i64, 42_i64];
        let (client_io, server_io) = tokio::io::duplex(8192);
        let server = tokio::spawn(serve_incident_list(server_io, expected.clone()));

        let client =
            ReadbackClient::connect_transport(client_io).await.expect("the stub session initializes");
        let ids = active_incident_ids(&client).await.expect("the active set reads back");

        drop(client);
        server.abort();

        assert_eq!(ids, expected, "every id the corpus returned must reach the caller");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn active_incident_ids_is_empty_only_when_the_corpus_is() {
        let (client_io, server_io) = tokio::io::duplex(8192);
        let server = tokio::spawn(serve_incident_list(server_io, Vec::new()));

        let client =
            ReadbackClient::connect_transport(client_io).await.expect("the stub session initializes");
        let ids = active_incident_ids(&client).await.expect("an empty active set reads back");

        drop(client);
        server.abort();

        assert!(ids.is_empty(), "an empty corpus yields an empty set, got {ids:?}");
    }

    #[test]
    fn the_fault_label_is_the_bounded_obs_plan_token() {
        // obs-plan §4 pins these literals as the `fault_type` attribute values; the classification
        // tests above assert the VARIANT and never the emitted string, which is what let a mutated
        // label survive.
        assert_eq!(FaultKind::Silence.label(), "silence");
        assert_eq!(FaultKind::Ramp { factor: 0.5 }.label(), "ramp");
    }

    #[test]
    fn a_fault_declaring_phase_opens_a_span_and_a_plain_phase_does_not() {
        let window =
            PhaseWindow { index: 0, name: "p1", gap: std::time::Duration::from_millis(100) };

        // A declared silence (`occurrences = 0`) is a fault application...
        assert!(
            fault_span(&occupier_fixture(), &window, 0).is_some(),
            "a declared silence window opens fault.silence"
        );
        // ...while an emitting plain phase declares none.
        assert!(
            fault_span(&fixture(1), &window, 0).is_none(),
            "a plain emitting phase has no reserved fault span (obs-plan §11)"
        );
        // A window past the declared phases yields nothing rather than panicking.
        let past_end =
            PhaseWindow { index: 9, name: "absent", gap: std::time::Duration::from_millis(1) };
        assert!(fault_span(&occupier_fixture(), &past_end, 0).is_none());
    }

    /// A loopback OTLP trace collector for the warm-up pre-roll. Bound to an ephemeral port, never
    /// `:4317` — that one is reserved for the port-occupier fault (test-plan §10). The warm-up needs
    /// a real collector because `TraceEmitter::connect` opens an actual channel.
    #[derive(Clone, Default)]
    struct WarmupCapture {
        requests: std::sync::Arc<std::sync::Mutex<Vec<ExportTraceServiceRequest>>>,
    }

    #[tonic::async_trait]
    impl opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceService
        for WarmupCapture
    {
        async fn export(
            &self,
            request: tonic::Request<ExportTraceServiceRequest>,
        ) -> Result<tonic::Response<ExportTraceServiceResponse>, tonic::Status> {
            self.requests.lock().unwrap().push(request.into_inner());
            Ok(tonic::Response::new(ExportTraceServiceResponse::default()))
        }
    }

    /// Drive the warm-up pre-roll against a stub collector; yields the emission count and the
    /// virtual time the pre-roll consumed.
    async fn drive_warmup(warmup_ms: u64, warmup_emissions: u32) -> (usize, std::time::Duration) {
        use opentelemetry_proto::tonic::collector::trace::v1::trace_service_server::TraceServiceServer;

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let capture = WarmupCapture::default();
        let requests = std::sync::Arc::clone(&capture.requests);
        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(TraceServiceServer::new(capture))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
                .ok();
        });

        let mut traces =
            TraceEmitter::connect(format!("http://{addr}")).await.expect("the stub collector accepts");
        let mut contract = contract_of(Vec::new());
        contract.incident_formation.warmup_ms = warmup_ms;
        contract.incident_formation.warmup_emissions = warmup_emissions;

        let started = tokio::time::Instant::now();
        warm_up_canary_service(&mut traces, &contract, 7).await.expect("the warm-up completes");
        let elapsed = started.elapsed();

        let count = requests.lock().unwrap().len();
        (count, elapsed)
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn the_warm_up_emits_its_declared_count_paced_by_the_declared_gap() {
        // The pacing divisor is the only thing this window can witness, and the three readings are
        // mutually distinct by construction: `warmup_ms / warmup_emissions` = 250ms x 4 = 1000ms,
        // `%` = 0ms, `*` = 4000ms x 4 = 16000ms. The bound below admits only the first.
        let (count, elapsed) = drive_warmup(1_000, 4).await;

        assert_eq!(count, 4, "every declared pre-roll emission reaches the collector");
        assert!(
            elapsed >= std::time::Duration::from_millis(900)
                && elapsed < std::time::Duration::from_millis(2_000),
            "the pre-roll is paced at warmup_ms / warmup_emissions (expected ~1000ms), got {elapsed:?}"
        );
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn a_zero_length_warm_up_window_emits_nothing() {
        let (count, _) = drive_warmup(0, 4).await;
        assert_eq!(count, 0, "a zero-length window declares no pre-roll");
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn a_zero_warm_up_emission_count_emits_nothing() {
        let (count, _) = drive_warmup(1_000, 0).await;
        assert_eq!(count, 0, "a zero emission count declares no pre-roll");
    }
}
