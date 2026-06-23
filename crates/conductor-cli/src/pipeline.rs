//! The run pipeline — the composition root that drives one scenario to a [`RunRecord`].
//!
//! The MCP preflight gate is the pivot: when the read-back path is unreachable or not ready, the
//! scenario short-circuits to a `Blocked` record (never a silent downgrade — arch §Read-Back
//! Dependency Posture). On a ready gate, the scenario drives the seeded timeline, coarse-emits per
//! phase signal, reads back, and classifies. The faithful per-scenario emission + per-check read-back
//! extraction are Epoch-10 ("Live-Pulse E2E proof"); here they are coarse (the CI-tested spine is the
//! Blocked path).

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context as _;
use conductor_core::{
    HeadlessResolver, HoldPoint, ReportState, RunRecord, Scenario, Signal, Verdict, now_rfc3339,
    resolve_hold,
};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, DEFAULT_SERVICE_NAME, LogsEmitter, Severity, TraceEmitter, probe_egress,
    severity_logs_request, trace_request,
};
use conductor_timeline::{PhaseTimeline, PhaseTransition, run_timeline};
use conductor_verify::{
    CanaryMarker, ContractManifest, ReadbackClient, evaluate_check, run_preflight,
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

    let manifest = ContractManifest::load(manifest_path).context("load MCP contract manifest")?;
    let canary = CanaryMarker::new("conductor-canary");
    let ready = run_preflight(&client, &manifest, &canary, &data_dir_str)
        .await
        .map(|state| state.ready)
        .unwrap_or(false);
    if !ready {
        tracing::info!("preflight blocked: readiness gate not satisfied");
    }
    Ok(Preflight { client: Some(client), ready })
}

/// Drive one scenario to its [`RunRecord`]: Blocked when the gate is not ready, else the coarse
/// live measured path (emit → read-back → classify).
pub async fn execute_scenario(
    pf: &Preflight,
    scenario: &Scenario,
    run_id: &str,
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
        let resolution = resolve_hold(&HeadlessResolver::proceed(), &hold).await;
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
