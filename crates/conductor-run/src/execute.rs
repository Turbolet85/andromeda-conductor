//! The scenario execution core: one scenario driven to its [`RunRecord`].
//!
//! Blocked when the gate is not ready (never a silent downgrade), else the seeded timeline runs with
//! the per-phase dispatcher attached and the read-back grades whatever the scenario declares. The
//! `scenario.run` root span opens here and closes when the scenario returns (obs-plan §4).

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context as _;

use conductor_core::{
    CheckRecord, EmissionShape, EmissionSpec, FaultKindSpec, HoldPoint, PauseResolver, ReportState, RunRecord, Scenario, Verdict, now_rfc3339, resolve_hold,
};
use conductor_emit::{
    DEFAULT_OTLP_ENDPOINT, probe_egress,
};
use conductor_faults::{FaultError, OTLP_INGEST_PORT, PortOccupier};
use conductor_timeline::{PhaseTimeline, PhaseWindow, run_timeline_observed};



use conductor_verify::{
    Observation, ReadBackOutcome, evaluate_check, observe,
};
use crate::canary::Preflight;
use crate::dispatch::Dispatcher;

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

pub(crate) fn now_ms() -> i64 {
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
pub(crate) fn now_unix_nanos() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as i64).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::*;
    use conductor_core::HeadlessResolver;

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
}
