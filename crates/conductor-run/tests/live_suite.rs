//! The live-suite capture — operator/local only, never a CI gate.
//!
//! Gated behind `--features live-pulse` (the `lifecycle_live.rs` precedent, test-plan §5 Drivers): a
//! default `nextest` / `clippy` / release never compiles it, so a host with no Pulse cannot red the
//! suite. The feature declares no dependency, so it adds nothing to `Cargo.lock`.
//!
//! It reads what `scripts/agent-run.{sh,ps1} run --live` already PRODUCED and PRINTS the three
//! observables' measured values. It mints nothing: `logs/agent-latest.jsonl` is written only when
//! `conductor-cli`'s `main` selects the file sink under `--agent-mode`, so an in-process test could
//! never produce it. The capture is the deliverable; the grading lives in `live_suite_harvest.rs`
//! over the frozen lines.
//!
//! FIRING FORM — run the suite first, then this. The whole env block in ONE paste, because a partial
//! set fails SILENTLY as a ~0s `[BLOCKED]` that reads exactly like a genuine SUT-side gate failure:
//!
//! ```text
//! PATH=/d/dev/projects/andromeda-pulse/target/release:$PATH \
//! ANDROMEDA_PULSE_DATA_DIR=<the live pulse-app's dir> \
//! ANDROMEDA_PULSE_MCP_ENABLED=true \
//! ANDROMEDA_PULSE_L4_DETERMINISTIC=true \
//!   bash scripts/agent-run.sh run --live
//!
//! cargo test -p conductor-run --features live-pulse --test live_suite -- --nocapture
//! ```
//!
//! STOP FORM — the wdio arm self-terminates via `onComplete` → `tauriDriver.kill()`; after an
//! interrupted run: `taskkill /F /IM msedgedriver.exe /IM conductor-tauri.exe` plus the node CLI.
//! `pulse-app` is the operator's: a leg never kills what it did not start.
//!
//! THREE OBSERVABLES, each a live-path fact whose mutation-tier site no hermetic test can reach
//! (test-plan §12 classes B and C — landing these does NOT retire those acceptances):
//!
//!   1. `Observation.degraded` on the `AutoResolved` arm. There is no `degraded` field on the
//!      envelope, so the proxy is the state mapping: `manual_record` sets
//!      `state_for(observation, ManualCheck)`, which returns `KnownResidual` iff `degraded`. On a
//!      declare-only scenario `state == KnownResidual` ⟺ `degraded == true`. Which ROUTE produced it
//!      is separated by the `declare-only read-back empty` self-obs line, which only that arm emits.
//!   2. The manual-path `latency_ms` subtraction, as a BOUNDED equality: `now_rfc3339` truncates to
//!      whole seconds while `latency_ms` is millisecond-grained, so the two persisted instants can
//!      only bound it to ±1s. That still separates a subtraction from a sum or a quotient.
//!   3. The readiness-gate line on BOTH arms — present on a not-ready-but-connected gate, absent on a
//!      ready one. Both are needed: the mutant inverts the guard, and an inverted guard passes a
//!      one-arm assertion.

#![cfg(feature = "live-pulse")]

use std::path::{Path, PathBuf};

/// The exact gate line. `canary.rs` carries three other `preflight blocked:` lines — two for the
/// unreachable read-back path and one for a failed canary emission — and the connect arms return
/// EARLY, so a prefix match would conflate a not-ready gate with a sidecar that never connected.
const GATE_BLOCKED: &str = "preflight blocked: readiness gate not satisfied";
const AUTO_RESOLVED: &str =
    "declare-only read-back empty: no active incident outlived the emission window";

/// The runs dir, anchored at the WORKSPACE root — cargo runs a test with the crate dir as cwd, so a
/// repo-relative default would resolve under `crates/conductor-run/` and miss every artifact.
fn runs_dir() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    match std::env::var("CONDUCTOR_RUNS_DIR") {
        Ok(handle) => root.join(handle),
        Err(_) => root.join("runs"),
    }
}

fn capture(leg: &str) -> String {
    let path = runs_dir().join("live-suite").join(format!("{leg}.jsonl"));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("leg {leg} capture at {} unreadable: {e}", path.display()))
}

/// The envelope line, discriminated from the per-check `CheckRecord` line that rides the same
/// journal. A file-wide parse-to-envelope is a false negative on any run that emitted check rows.
fn envelope(journal: &Path) -> serde_json::Value {
    let body = std::fs::read_to_string(journal)
        .unwrap_or_else(|e| panic!("journal {} unreadable: {e}", journal.display()));
    body.lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .find(|v| v.get("slo_tier").is_some() && v.get("check_index").is_none())
        .unwrap_or_else(|| panic!("no envelope line in {}", journal.display()))
}

/// The journal of the run THIS leg produced, linked by the `run_id` its own self-obs stream carries.
///
/// Selecting by scenario name would be wrong here: legs b1 and b2 drive the SAME scenario on purpose,
/// so a newest-wins pick returns b2's blocked row — which has no `latency_ms` at all — for a question
/// asked about b1. The self-obs line carries the run_id, so the link is exact rather than inferred.
fn journal_of(leg: &str, capture: &str) -> PathBuf {
    let run_id = capture
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .find_map(|v| v.get("run_id").and_then(|r| r.as_str()).map(str::to_owned))
        .unwrap_or_else(|| panic!("leg {leg} capture carries no run_id"));
    runs_dir().join(format!("{run_id}.jsonl"))
}

fn secs_of(rfc3339: &str) -> i64 {
    // `YYYY-MM-DDTHH:MM:SSZ` — the envelope's only instant shape (second-precision by construction).
    let (h, m, s) = (&rfc3339[11..13], &rfc3339[14..16], &rfc3339[17..19]);
    h.parse::<i64>().unwrap() * 3600 + m.parse::<i64>().unwrap() * 60 + s.parse::<i64>().unwrap()
}

#[test]
fn capture_the_three_live_observables() {
    let (b1, b2, a) = (capture("b1"), capture("b2"), capture("a"));
    println!(
        "LEG captures: b1={} b2={} a={} lines",
        b1.lines().count(),
        b2.lines().count(),
        a.lines().count()
    );

    // --- observable 3: the gate line on both arms -------------------------------------------------
    for (leg, body, want) in [("b1", &b1, false), ("b2", &b2, true), ("a", &a, false)] {
        let present = body.lines().any(|l| l.contains(GATE_BLOCKED));
        println!("OBS3 leg {leg}: gate-blocked line present={present} (expected {want})");
        if let Some(line) = body.lines().find(|l| l.contains(GATE_BLOCKED)) {
            println!("OBS3 leg {leg} VERBATIM: {line}");
        }
    }

    // --- observable 1: the AutoResolved route on leg A --------------------------------------------
    let a_line = a.lines().find(|l| l.contains(AUTO_RESOLVED));
    println!(
        "OBS1 leg a: auto-resolved line present={}",
        a_line.is_some()
    );
    if let Some(line) = a_line {
        println!("OBS1 leg a VERBATIM: {line}");
    }
    let a_env = envelope(&journal_of("a", &a));
    println!(
        "OBS1 leg a envelope: state={} verdict={}",
        a_env["state"], a_env["verdict"]
    );
    println!("OBS1 leg a VERBATIM ENVELOPE: {a_env}");

    // The contrast is the proof that the state mapping DISCRIMINATES rather than being constant: b1
    // took the graded route on the same declare-only shape and landed ManualCheck.
    let b1_state_env = envelope(&journal_of("b1", &b1));
    println!(
        "OBS1 contrast — leg b1 envelope state={}",
        b1_state_env["state"]
    );

    // --- observable 2: the bounded latency equality on leg B1 -------------------------------------
    let b1_env = envelope(&journal_of("b1", &b1));
    let (emitted, observed) = (
        b1_env["journal_emitted_at"]
            .as_str()
            .expect("journal_emitted_at"),
        b1_env["read_back_observed_at"]
            .as_str()
            .expect("read_back_observed_at"),
    );
    let latency = b1_env["latency_ms"].as_i64().expect("latency_ms");
    let delta_ms = (secs_of(observed) - secs_of(emitted)) * 1000;
    println!(
        "OBS2 leg b1 envelope: state={} emitted={emitted} observed={observed} latency_ms={latency} \
         instants_delta_ms={delta_ms} skew_ms={}",
        b1_env["state"],
        (latency - delta_ms).abs()
    );
    println!("OBS2 leg b1 VERBATIM ENVELOPE: {b1_env}");
}
