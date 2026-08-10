//! `conductor-run` — the run composition root shared by both shells.
//!
//! Drives one scenario (or a suite) to its [`RunRecord`]s. The MCP preflight gate is the pivot: when
//! the read-back path is unreachable or not ready, the scenario short-circuits to a `Blocked` record
//! (never a silent downgrade — arch §Read-Back Dependency Posture). On a ready gate it drives the
//! seeded timeline, coarse-emits per phase signal, reads back, and classifies. Both `conductor-cli`
//! (the release gate) and `conductor-tauri` (the GUI) call this identically — the "headless-drivable
//! core, thin shells" split (arch §Design Philosophy). The faithful per-scenario emission + per-check
//! read-back extraction are Epoch-10 ("Live-Pulse E2E proof"); here they are coarse (the CI-tested
//! spine is the Blocked path).
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
    EnvelopeStatus, HoldPoint, LoadEnvelope, PauseResolver, ReportState, RunContract,
    RunContractStatus, RunRecord, Scenario, Signal, Verdict, now_rfc3339, redact_value,
    resolve_hold, resolve_under,
};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, DEFAULT_SERVICE_NAME, ExceptionSpec, Frame, LogsEmitter, Severity,
    TraceEmitter, exception_trace_request, fingerprint, probe_egress, severity_logs_request,
    trace_request,
};
use conductor_report::{JournalWriter, RunReport, RunsDb};
use conductor_timeline::{PhaseTimeline, PhaseTransition, run_timeline};
use conductor_verify::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ContractManifest, ReadbackClient, ReadyState,
    ToolPresence, evaluate_check, run_preflight,
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

/// Identical-fingerprint canary exceptions emitted as the storm — over Pulse's `>=5 in 30s` retry-storm
/// floor (P-018) so the gate's incident is raised deterministically.
const CANARY_STORM_COUNT: u64 = 6;

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

/// Emit a unique fingerprint-storm to Pulse's loopback ingest and return the [`CanaryMarker`] the gate
/// asserts reads back. The marker is unique per preflight (a stale corpus can't satisfy the gate on a
/// prior run's canary); the fingerprint — computed to match Pulse's derivation — is the fidelity
/// carrier (Pulse scrubs incident titles). Each occurrence carries a distinct span identity but the
/// same fingerprint, so Pulse counts a storm (security-plan §Threat Model).
async fn emit_canary(contract: &RunContract, warm_up: bool) -> anyhow::Result<CanaryMarker> {
    let marker = format!("ConductorCanary_{}", now_ms());
    let spec = ExceptionSpec::new(
        marker.clone(),
        "conductor preflight canary",
        vec![Frame::new("conductor::run::preflight_canary", "conductor-run/src/lib.rs", 1)],
    );
    let fp = fingerprint(&spec);
    let base = now_ms() as u64;
    let mut traces = TraceEmitter::connect(DEFAULT_OTLP_ENDPOINT).await?;
    if warm_up {
        warm_up_canary_service(&mut traces, contract).await?;
    }
    for i in 0..CANARY_STORM_COUNT {
        traces
            .export(exception_trace_request(DEFAULT_SERVICE_NAME, base.wrapping_add(i), &spec))
            .await?;
    }
    Ok(CanaryMarker::new(marker, fp))
}

/// Carry the canary service out of Pulse's baseline bootstrap before the counted storm, with benign
/// non-error spans spread across the contract's warm-up window. Without it the storm is the
/// service's first-ever traffic, Pulse holds no baseline for it, and the L2 cue evaluator never
/// considers it — the `cues_emitted: 0` the 2026-08-10 workspace-key probe recorded.
async fn warm_up_canary_service(
    traces: &mut TraceEmitter,
    contract: &RunContract,
) -> anyhow::Result<()> {
    let terms = &contract.incident_formation;
    if terms.warmup_ms == 0 || terms.warmup_emissions == 0 {
        return Ok(());
    }
    let gap = std::time::Duration::from_millis(terms.warmup_ms / u64::from(terms.warmup_emissions));
    tracing::info!(count = terms.warmup_emissions, "warming the canary service out of bootstrap");
    for _ in 0..terms.warmup_emissions {
        traces.export(trace_request(DEFAULT_SERVICE_NAME, "canary-warmup")).await?;
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
pub async fn execute_scenario<R: PauseResolver>(
    pf: &Preflight,
    scenario: &Scenario,
    run_id: &str,
    resolver: &R,
) -> anyhow::Result<RunRecord> {
    if !pf.ready {
        return Ok(RunRecord::blocked(
            run_id,
            scenario.seed,
            &scenario.name,
            scenario.p_ids.clone(),
            scenario.slo_tier,
        ));
    }
    let client = pf.client.as_ref().expect("a ready gate implies a connected client");

    probe_egress(DEFAULT_OTLP_ENDPOINT).await.context("OTLP egress liveness to :4317")?;

    let emitted_ms = now_ms();
    let journal_emitted_at = now_rfc3339();
    let timeline = PhaseTimeline::from(scenario);
    let transitions = run_timeline(&timeline, scenario.seed).await.context("timeline scheduling")?;
    coarse_emit(scenario, &transitions).await?;

    // Coarse read-back: the faithful per-check observed-extraction is the Epoch-10 bridge.
    let observed = match client.query_incident_list(None).await {
        Ok(_) => "incidents-listed".to_string(),
        Err(_) => String::new(),
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
        };
        let resolution = resolve_hold(resolver, &hold).await;
        tracing::debug!("operator-checklist hold resolved headless: {}", resolution.decision.label());
        return Ok(manual_record(
            scenario,
            run_id,
            journal_emitted_at,
            read_back_observed_at,
            observed_ms - emitted_ms,
        ));
    }

    let chosen = scenario
        .expected
        .iter()
        .map(|check| evaluate_check(check, &observed, scenario.slo_tier, emitted_ms, observed_ms))
        .max_by_key(|outcome| severity_rank(outcome.assessment.verdict))
        .expect("expected is non-empty");
    Ok(chosen.to_run_record(
        run_id,
        scenario.seed,
        &scenario.name,
        scenario.p_ids.clone(),
        journal_emitted_at,
        read_back_observed_at,
        Vec::new(),
    ))
}

/// Emit one coarse OTLP signal per phase boundary — the Epoch-10 proof makes this scenario-faithful.
async fn coarse_emit(scenario: &Scenario, transitions: &[PhaseTransition]) -> anyhow::Result<()> {
    let mut traces = TraceEmitter::connect(DEFAULT_OTLP_ENDPOINT).await?;
    let mut logs: Option<LogsEmitter> = None;
    for transition in transitions {
        let signal = scenario
            .phases
            .get(transition.index)
            .map(|p| p.emission.signal)
            .unwrap_or(Signal::Traces);
        match signal {
            Signal::Logs => {
                let emitter = match logs.as_mut() {
                    Some(emitter) => emitter,
                    None => logs.insert(LogsEmitter::connect(DEFAULT_OTLP_ENDPOINT).await?),
                };
                let severity = Severity::new(9).expect("9 is a valid SeverityNumber");
                emitter.export(severity_logs_request(DEFAULT_SERVICE_NAME, &[severity])).await?;
            }
            Signal::Traces | Signal::Metrics => {
                traces.export(trace_request(DEFAULT_SERVICE_NAME, &transition.name)).await?;
            }
        }
    }
    Ok(())
}

/// An operator-checklist / declare-only scenario yields a verdict-less `ManualCheck` record
/// (`Lamp::for_record` maps `(ManualCheck, None) → Manual`).
fn manual_record(
    scenario: &Scenario,
    run_id: &str,
    journal_emitted_at: String,
    read_back_observed_at: String,
    latency_ms: i64,
) -> RunRecord {
    RunRecord {
        journal_emitted_at: Some(journal_emitted_at),
        read_back_observed_at: Some(read_back_observed_at),
        run_id: run_id.to_string(),
        seed: scenario.seed,
        scenario: scenario.name.clone(),
        p_ids: scenario.p_ids.clone(),
        verdict: None,
        state: ReportState::ManualCheck,
        latency_ms: Some(latency_ms),
        slo_tier: scenario.slo_tier,
        fingerprints: Some(Vec::new()),
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

/// Persist a run's records across the three artifacts: the JSONL journal (append), the `runs.db`
/// index (one row per scenario), and a single Markdown report for the run. Shared by both shells so
/// the CLI and the GUI write an identical envelope for the same scenario+seed (test-plan Path 7).
pub fn persist(
    runs_dir: &Path,
    run_id: &str,
    records: &[RunRecord],
    envelope: &EnvelopeStatus,
) -> anyhow::Result<()> {
    let mut journal = JournalWriter::create(runs_dir, run_id)?;
    let db = RunsDb::open(runs_dir)?;
    for record in records {
        journal.append(record)?;
        db.insert(record)?;
    }
    db.insert_envelope(run_id, envelope)?;
    RunReport::write(runs_dir, run_id, records, envelope)?;
    Ok(())
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
    for scenario in scenarios {
        if should_abort() {
            persist(runs_dir, run_id, &records, envelope)?;
            emit(RunEvent { stage: RunStage::Aborted, count: records.len() as u64 });
            return Ok(records);
        }
        records.push(execute_scenario(pf, scenario, run_id, resolver).await?);
        emit(RunEvent { stage: RunStage::Progress, count: records.len() as u64 });
    }
    persist(runs_dir, run_id, &records, envelope)?;
    let stage = if records.iter().all(|r| matches!(r.state, ReportState::Blocked)) {
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
    use conductor_core::HeadlessResolver;

    /// A blocked-gate `Preflight` (no connected client) — the no-live-Pulse spine, constructed
    /// directly so the test needs neither a sidecar nor an env handle.
    fn blocked_preflight() -> Preflight {
        Preflight { client: None, ready: false }
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

    fn test_envelope(ceiling_ms: u64, exempt: &[(&str, &str)]) -> LoadEnvelope {
        LoadEnvelope {
            sut_version: "v0.3.0".to_string(),
            captured_at: "2026-08-09".to_string(),
            provenance: "test".to_string(),
            envelope: conductor_core::EnvelopeTerms {
                max_sustained_rate_spans_per_s: 10_000,
                max_sustained_storm_ms: 600_000,
                max_scenario_duration_ms: ceiling_ms,
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
        let record =
            execute_scenario(&blocked_preflight(), &fixture(7), "run-test", &HeadlessResolver::proceed())
                .await
                .expect("the blocked path is infallible");
        assert!(matches!(record.state, ReportState::Blocked));
        assert!(record.verdict.is_none(), "a blocked row carries no verdict");
        assert!(record.journal_emitted_at.is_none() && record.latency_ms.is_none());
        assert_eq!(record.seed, 7);
        assert_eq!(record.scenario, "blocked-fixture");
    }

    #[tokio::test(flavor = "current_thread")]
    async fn blocked_envelope_is_seed_identified() {
        let pf = blocked_preflight();
        let a = execute_scenario(&pf, &fixture(7), "r", &HeadlessResolver::proceed()).await.unwrap();
        let b = execute_scenario(&pf, &fixture(7), "r", &HeadlessResolver::proceed()).await.unwrap();
        assert_eq!(a, b, "same scenario+seed ⇒ identical blocked envelope");
        let c = execute_scenario(&pf, &fixture(9), "r", &HeadlessResolver::proceed()).await.unwrap();
        assert_ne!(a.seed, c.seed, "the seed materially identifies the envelope");
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
}
