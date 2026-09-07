//! The MCP preflight gate and the canary storm that proves the corpus wiring.
//!
//! [`preflight`] is the pivot every scenario turns on: it connects the read-back client, evaluates
//! the run contract, warms the canary service, emits a unique fingerprint-storm and asserts an
//! incident opened AFTER that storm's emission instant (arch §Standard Contracts — Readiness gate).
//! Freshness is the carrier, not payload identity: under deterministic L4 no read-back field varies
//! with what Conductor emitted.

use std::path::{Path, PathBuf};

use anyhow::Context as _;

use conductor_core::{RunContract, now_rfc3339, redact_value};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, ExceptionSpec, Frame, TraceEmitter, exception_trace_request,
    fingerprint, trace_request,
};

use crate::execute::{now_ms, now_unix_nanos};
use crate::preconditions::{load_run_contract, observe_run_contract};
use conductor_verify::{
    CanaryMarker, CanaryOutcome, CanaryPoll, ContractManifest, ReadbackClient, ReadyState,
    ToolPresence, run_preflight,
};

/// The suite-wide preflight outcome — established once, reused by every scenario in a run.
pub struct Preflight {
    /// Crate-visible so the execution core can read the gate; never `pub` — a caller outside the
    /// crate takes the gate's verdict through [`preflight`], not by inspecting it.
    pub(crate) client: Option<ReadbackClient>,
    pub(crate) ready: bool,
}

/// Establish the read-back session + readiness gate once. A failed spawn / handshake (the read-back
/// path unreachable) or an unsatisfied gate yields `ready = false` — a Blocked precondition, never a
/// harness `Err`. A missing / invalid contract manifest on a connected path IS a harness fault.
pub async fn preflight(manifest_path: &Path) -> anyhow::Result<Preflight> {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(PathBuf::from);
    let data_dir_str = data_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let client = match ReadbackClient::connect(data_dir).await {
        Ok(client) => client,
        Err(_) => {
            tracing::info!("preflight blocked: MCP read-back path unreachable");
            return Ok(Preflight {
                client: None,
                ready: false,
            });
        }
    };

    let state = canary_gate(&client, manifest_path, &data_dir_str).await?;
    if !state.ready {
        tracing::info!("preflight blocked: readiness gate not satisfied");
    }
    Ok(Preflight {
        client: Some(client),
        ready: state.ready,
    })
}

/// The full readiness result for the `conductor preflight` verb — the serializable arch readiness
/// shape (arch §Standard Contracts). Reuses the hardened `ReadbackClient::connect` + `run_preflight`;
/// an unreachable read-back path yields a Blocked `ReadyState`, never an `Err`.
pub async fn readiness(manifest_path: &Path) -> anyhow::Result<ReadyState> {
    let data_dir = std::env::var_os("ANDROMEDA_PULSE_DATA_DIR").map(PathBuf::from);
    let data_dir_str = data_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
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
    const UNREACHABLE_PRECONDITION: &str = "mcp-server cargo feature + ANDROMEDA_PULSE_MCP_ENABLED + ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir";
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
        vec![Frame::new(
            "conductor::run::preflight_canary",
            "conductor-run/src/lib.rs",
            1,
        )],
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
            .export(exception_trace_request(
                CANARY_SERVICE_NAME,
                canary_storm_seed(base, i),
                spec,
            ))
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
    Ok(run_preflight(
        client,
        &manifest,
        &status,
        &canary,
        data_dir_str,
        canary_poll(&contract),
    )
    .await?)
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
    tracing::info!(
        count = terms.warmup_emissions,
        "warming the canary service out of bootstrap"
    );
    for i in 0..terms.warmup_emissions {
        let seed = canary_warmup_seed(base, i);
        traces
            .export(trace_request(CANARY_SERVICE_NAME, seed, "canary-warmup"))
            .await?;
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
    CanaryPoll {
        attempts: secs as u32,
        interval: std::time::Duration::from_secs(1),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use opentelemetry_proto::tonic::collector::trace::v1::{
        ExportTraceServiceRequest, ExportTraceServiceResponse,
    };

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

        let mut traces = TraceEmitter::connect(format!("http://{addr}"))
            .await
            .expect("the stub collector accepts");
        let mut contract = contract_of(Vec::new());
        contract.incident_formation.warmup_ms = warmup_ms;
        contract.incident_formation.warmup_emissions = warmup_emissions;

        let started = tokio::time::Instant::now();
        warm_up_canary_service(&mut traces, &contract, 7)
            .await
            .expect("the warm-up completes");
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

        assert_eq!(
            count, 4,
            "every declared pre-roll emission reaches the collector"
        );
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
