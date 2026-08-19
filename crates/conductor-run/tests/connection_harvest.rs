//! Live-leg evidence harvest for the connection-lifecycle family (P-001..P-004).
//!
//! The family's four authored read-back checks were retired to declare-only after measurement:
//! connection state reaches NO MCP read-back surface — Pulse's FSM surfaces only via the TauRPC
//! `connection.current_state` resolver, the `pulse://stream/connection-state` broadcast, and
//! tracing lines — so the Hard `Contains` checks ("Stalled" / "Receiving" / "ReceiverFailed" /
//! "Idle"+"error") graded against corpus text that cannot carry the tokens. Leg A measured it:
//! run `2026-08-19T22-38-54-528` (old walk TOML, check intact) landed verdict `Fail` (structural)
//! / state `KnownResidual` / latency 69169 ms — the fingerprint-storm / baseline / pii precedent.
//! The live claims therefore grade HERE, on Pulse's own `connection.state.transition` /
//! `app.boot.otlp.grpc.bind` lines, exactly as the four prior harvests do.
//!
//! Witnesses measured live (SUT at HEAD `efabe8e`, fresh data dir per leg under
//! `%TEMP%/pulse-legs/`) and pinned VERBATIM below:
//!
//! - **Walk (P-001, leg B1, run `2026-08-19T22-44-56-113`, verdict null / KnownResidual /
//!   69184 ms / `<90s` held):** the preflight warm-up paces spans ~15 s apart, so the pre-phase
//!   ledger OSCILLATES Receiving <-> Idle across the 10 s threshold; the walk's own tail then
//!   crosses both thresholds in order — `Receiving -> Idle` at lag 10790 (>=10 s + one poller
//!   tick) and `Idle -> Stalled` at lag 60788 (>=60 s, severity `warning`).
//! - **Tracker (P-002, leg B2, run `2026-08-19T22-47-17-426`, 6079 ms / `<20s` held):** every
//!   fresh-span transition reports `last_span_ago_ms` under one second (539-588) and every Idle
//!   crossing lands inside [threshold, threshold + 1 s poller tick] (10496-10589) — the +/-1 s
//!   tracker claim witnessed at the crossings.
//! - **Orthogonality (P-004, leg B3, run `2026-08-19T22-48-30-247`, 18188 ms / `<90s`):** the
//!   100%-error burst raises NO alert state — zero `ReceiverFailed` in the leg's ledger; the
//!   clean stop crosses `Receiving -> Idle` at lag 10392.
//! - **Conflict (P-003, leg B4, run `2026-08-19T22-50-47-826`, 75074 ms / `<90s`, the
//!   choreographed leg):** with Conductor's occupier holding `:4317` (bound at the port-held
//!   phase open, +20 s cue), the relaunched pulse-app logs ERROR `bind failed`
//!   (`os error 10048`) and the FSM transitions `Listening -> ReceiverFailed` with
//!   `trigger_reason: "receiver bind failed"` at severity `critical`, level ERROR; after the
//!   phase-boundary RAII release the restarted pulse-app logs "OTLP gRPC receiver bound" — the
//!   release proven by the SUT's own successful rebind. (Recovery is a process replacement:
//!   bind status is per-process, so no FSM transition OUT of ReceiverFailed exists to log.)
//! - The ratified occupy-failure policy's live witness (leg A2): occupy against a live Pulse
//!   holding `:4317` -> AddrInUse -> sanitized harness Err AFTER the timeline, exit 1, NO run
//!   row — recorded in the chunk's leg-verdict; the mechanics are unit-tested in
//!   `conductor-run/src/lib.rs::tests`.
//!
//! TEST-ONLY affordance (the storm/baseline/restart/pii-harvest precedent): nothing here is
//! wired into the run path and nothing harvested reaches a Conductor artifact. Capture basis on
//! Windows: `{data_dir}/logs/agent-latest.jsonl.<date>`, whole-file per leg (fresh dir per leg).

/// One `connection.state.transition` observation (Pulse's FSM poller line, emitted on change).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Transition {
    from_state: String,
    to_state: String,
    last_span_ago_ms: u64,
    trigger_reason: String,
    severity: String,
    level: String,
    timestamp: String,
}

fn transitions(lines: &[&str]) -> Vec<Transition> {
    lines
        .iter()
        .map(|line| {
            let v: serde_json::Value = serde_json::from_str(line).expect("pinned line parses");
            let f = &v["fields"];
            Transition {
                from_state: f["from_state"].as_str().unwrap().to_string(),
                to_state: f["to_state"].as_str().unwrap().to_string(),
                last_span_ago_ms: f["last_span_ago_ms"].as_u64().unwrap(),
                trigger_reason: f["trigger_reason"].as_str().unwrap().to_string(),
                severity: f["severity"].as_str().unwrap().to_string(),
                level: v["level"].as_str().unwrap().to_string(),
                timestamp: v["timestamp"].as_str().unwrap().to_string(),
            }
        })
        .collect()
}

/// The walk leg's full transition ledger (B1) — warm-up oscillation, then the walk tail.
const WALK_LEDGER: &[&str] = &[
    r#"{"fields":{"deployment.environment":"production","from_state":"Listening","last_span_ago_ms":861,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:44:57.043Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10861,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:07.042Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":860,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:12.046Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10856,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:22.043Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":855,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:27.046Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10851,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:37.042Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":837,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:42.045Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10790,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:45:57.045Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":60788,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"warning","to_state":"Stalled","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:46:47.043Z"}"#,
];

/// The tracker leg's ledger (B2) — three known intervals, lag under a second on every fresh span.
const TRACKER_LEDGER: &[&str] = &[
    r#"{"fields":{"deployment.environment":"production","from_state":"Listening","last_span_ago_ms":588,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:18.082Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10589,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:28.083Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":572,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:33.082Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10573,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:43.082Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":559,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:48.085Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10559,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:47:58.084Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":539,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:03.079Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10496,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:20.082Z"}"#,
];

/// The orthogonality leg's ledger (B3) — a 100%-error burst, then the clean stop; no alert state.
const ORTHOGONAL_LEDGER: &[&str] = &[
    r#"{"fields":{"deployment.environment":"production","from_state":"Listening","last_span_ago_ms":557,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:30.875Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10541,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:40.860Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":531,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:45.866Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10526,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:48:55.861Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":526,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:49:00.875Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10516,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:49:10.864Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":494,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Receiving","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:49:15.866Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Receiving","last_span_ago_ms":10392,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"info","to_state":"Idle","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:49:29.865Z"}"#,
    r#"{"fields":{"deployment.environment":"production","from_state":"Idle","last_span_ago_ms":60387,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"warning","to_state":"Stalled","trigger_reason":""},"level":"INFO","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:50:19.860Z"}"#,
];

/// The conflict leg's one FSM alert (B4): ReceiverFailed while Conductor's occupier held `:4317`.
const CONFLICT_TRANSITION: &str = r#"{"fields":{"deployment.environment":"production","from_state":"Listening","last_span_ago_ms":0,"service.name":"com.andromeda.pulse","service.version":"0.1.0","severity":"critical","to_state":"ReceiverFailed","trigger_reason":"receiver bind failed"},"level":"ERROR","message":"connection state transition","target":"connection.state.transition","timestamp":"2026-08-19T22:51:56.254Z"}"#;

/// The conflict leg's three bind lines: instance 1 bound, instance 2 refused into the hold,
/// instance 3 rebound after the phase-boundary release.
const CONFLICT_BIND_LINES: &[&str] = &[
    r#"{"fields":{"bind_address":"127.0.0.1:4317","deployment.environment":"production","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"OTLP gRPC receiver bound","target":"app.boot.otlp.grpc.bind","timestamp":"2026-08-19T22:50:44.494Z"}"#,
    r#"{"fields":{"bind_address":"127.0.0.1:4317","deployment.environment":"production","reason":"OTLP receiver bind failed: Only one usage of each socket address (protocol/network address/port) is normally permitted. (os error 10048)","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"ERROR","message":"bind failed","target":"app.boot.otlp.grpc.bind","timestamp":"2026-08-19T22:51:56.246Z"}"#,
    r#"{"fields":{"bind_address":"127.0.0.1:4317","deployment.environment":"production","service.name":"com.andromeda.pulse","service.version":"0.1.0"},"level":"INFO","message":"OTLP gRPC receiver bound","target":"app.boot.otlp.grpc.bind","timestamp":"2026-08-19T22:52:36.445Z"}"#,
];

/// Pulse's FSM thresholds (crates/ingest/src/connection.rs) plus its 1 s poller tick — a crossing
/// lands inside [threshold, threshold + tick + margin].
const IDLE_MS: u64 = 10_000;
const STALLED_MS: u64 = 60_000;
const CROSSING_SLACK_MS: u64 = 2_000;

#[test]
fn the_walk_ledger_crosses_both_thresholds_in_order() {
    let ledger = transitions(WALK_LEDGER);
    let stalled = ledger.iter().rfind(|t| t.to_state == "Stalled").expect("the walk reaches Stalled");
    assert!(
        (STALLED_MS..STALLED_MS + CROSSING_SLACK_MS).contains(&stalled.last_span_ago_ms),
        "the Stalled crossing fires at the 60 s threshold: {stalled:?}"
    );
    assert_eq!(stalled.severity, "warning", "Stalled is the warning tier");

    let idle = ledger
        .iter()
        .rfind(|t| t.to_state == "Idle" && t.timestamp.as_str() < stalled.timestamp.as_str())
        .expect("an Idle crossing precedes Stalled");
    assert!(
        (IDLE_MS..IDLE_MS + CROSSING_SLACK_MS).contains(&idle.last_span_ago_ms),
        "the Idle crossing fires at the 10 s threshold: {idle:?}"
    );

    let receiving = ledger
        .iter()
        .rfind(|t| t.to_state == "Receiving" && t.timestamp.as_str() < idle.timestamp.as_str())
        .expect("the walk's emission precedes the quiet tail");
    assert!(receiving.last_span_ago_ms < 1_000, "a fresh span flips to Receiving within a second");
}

#[test]
fn the_warmup_oscillation_straddles_the_idle_threshold_by_construction() {
    // The preflight warm-up paces spans ~15 s apart — past the 10 s Idle threshold — so the
    // pre-phase ledger legitimately oscillates. Pinned so a future pacing change is visible.
    let ledger = transitions(WALK_LEDGER);
    let oscillations = ledger.iter().filter(|t| t.from_state == "Idle" && t.to_state == "Receiving").count();
    assert!(oscillations >= 2, "the warm-up cadence produces repeated Idle -> Receiving flips");
}

#[test]
fn the_tracker_reports_elapsed_within_the_poller_tick_of_truth() {
    let ledger = transitions(TRACKER_LEDGER);
    for t in ledger.iter().filter(|t| t.to_state == "Receiving") {
        assert!(t.last_span_ago_ms < 1_000, "a fresh span is tracked within a second: {t:?}");
    }
    for t in ledger.iter().filter(|t| t.to_state == "Idle") {
        assert!(
            (IDLE_MS..IDLE_MS + CROSSING_SLACK_MS).contains(&t.last_span_ago_ms),
            "an Idle crossing reports threshold + at most one tick: {t:?}"
        );
    }
    assert!(ledger.iter().any(|t| t.to_state == "Idle"), "the intervals produce crossings to grade");
}

#[test]
fn the_conflict_leg_witnesses_receiver_failed_while_the_occupier_held() {
    let t = &transitions(&[CONFLICT_TRANSITION])[0];
    assert_eq!(t.to_state, "ReceiverFailed");
    assert_eq!(t.trigger_reason, "receiver bind failed");
    assert_eq!(t.severity, "critical");
    assert_eq!(t.level, "ERROR", "a ReceiverFailed transition logs at error level");

    let failed: serde_json::Value = serde_json::from_str(CONFLICT_BIND_LINES[1]).unwrap();
    assert_eq!(failed["message"], "bind failed");
    assert_eq!(failed["level"], "ERROR");
    let reason = failed["fields"]["reason"].as_str().unwrap();
    assert!(reason.contains("(os error 10048)"), "the OS names the address conflict: {reason}");
    assert_eq!(failed["fields"]["bind_address"], "127.0.0.1:4317");
}

#[test]
fn the_release_is_proven_by_the_suts_own_rebind() {
    let stamps: Vec<String> = CONFLICT_BIND_LINES
        .iter()
        .map(|l| {
            let v: serde_json::Value = serde_json::from_str(l).unwrap();
            v["timestamp"].as_str().unwrap().to_string()
        })
        .collect();
    let messages: Vec<String> = CONFLICT_BIND_LINES
        .iter()
        .map(|l| {
            let v: serde_json::Value = serde_json::from_str(l).unwrap();
            v["message"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(messages, ["OTLP gRPC receiver bound", "bind failed", "OTLP gRPC receiver bound"]);
    assert!(stamps[0] < stamps[1] && stamps[1] < stamps[2], "bound -> refused into the hold -> rebound after the release: {stamps:?}");
}

#[test]
fn no_false_alert_off_the_conflict_leg() {
    for (name, ledger) in
        [("walk", WALK_LEDGER), ("tracker", TRACKER_LEDGER), ("orthogonal", ORTHOGONAL_LEDGER)]
    {
        assert!(
            transitions(ledger).iter().all(|t| t.to_state != "ReceiverFailed"),
            "{name} leg raises no alert state"
        );
    }
    // The orthogonality claim's sharp half: a 100%-error burst is APPLICATION health, and the
    // connection domain still walks Receiving -> Idle cleanly.
    let orthogonal = transitions(ORTHOGONAL_LEDGER);
    let clean_stop = orthogonal.iter().rfind(|t| t.to_state == "Idle").unwrap();
    assert!((IDLE_MS..IDLE_MS + CROSSING_SLACK_MS).contains(&clean_stop.last_span_ago_ms));
}

#[test]
fn no_pinned_line_carries_a_host_path_or_conductor_internal() {
    for line in WALK_LEDGER
        .iter()
        .chain(TRACKER_LEDGER)
        .chain(ORTHOGONAL_LEDGER)
        .chain(CONFLICT_BIND_LINES)
        .chain(std::iter::once(&CONFLICT_TRANSITION))
    {
        assert!(!line.contains(":\\"), "no drive-letter path in a pinned line: {line}");
        assert!(!line.contains("/home/") && !line.contains("/Users/"), "{line}");
        assert!(!line.contains("PhaseGuard") && !line.contains("RunRecord"), "{line}");
    }
}
