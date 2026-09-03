//! Cross-surface envelope parity — test-plan §5/§6 Path 7, verification-matrix v2-25.
//!
//! Lives in `conductor-cli` because this package declares `[[bin]] name = "conductor"`: Cargo builds
//! a package's own bins for that package's test targets and sets `CARGO_BIN_EXE_conductor`, so the
//! CLI arm resolves through a declared dependency edge instead of `assert_cmd`'s `legacy_cargo_bin`
//! fallback to whatever `target/debug/` happened to contain. In `conductor-tauri` — where this test
//! used to live — that variable is never set (Cargo sets it per DECLARING package) and no edge to
//! `conductor-cli` exists, so a fresh target dir made the whole crate's suite abort in an unmutated
//! tree. Same reason `conductor-verify/tests/preflight_spawn.rs` reaches `stub_pulse_mcp` this way.
//!
//! Its own test binary, one test — `cargo test` gives each `tests/*.rs` a process and nextest is
//! already process-per-test, so the `set_var` below is provably alone under BOTH runners
//! (test-plan §11).
//!
//! The Tauri arm drives `conductor_run::{preflight, drive_run}` — the same composition `start_run`'s
//! background thread runs — and needs no `tauri::*` item, which is why the test carries over whole.

use conductor_core::{
    latest_run_id, read_run_journal, EnvelopeStatus, HeadlessResolver, ReportState, Scenario,
};

/// Stage `dir` as a repo-shaped root the `conductor` binary can run in: the scenario it will
/// drive plus every contract a load path reads (capability manifest at scenario load, MCP
/// contract at preflight, load envelope at `classify_run`). Mirrors `cli_smoke`'s helpers.
fn stage_repo_root(dir: &assert_fs::TempDir, scenario_stem: &str) {
    use assert_fs::prelude::*;
    let root = format!("{}/../..", env!("CARGO_MANIFEST_DIR"));
    for (src, dest) in [
        (format!("{root}/scenarios/{scenario_stem}.toml"), format!("scenarios/{scenario_stem}.toml")),
        (format!("{root}/contracts/mcp-contract.toml"), "contracts/mcp-contract.toml".into()),
        (format!("{root}/contracts/pulse-capabilities.toml"), "contracts/pulse-capabilities.toml".into()),
        (format!("{root}/contracts/pulse-load-envelope.toml"), "contracts/pulse-load-envelope.toml".into()),
    ] {
        let text = std::fs::read_to_string(&src).unwrap_or_else(|e| panic!("read {src}: {e}"));
        dir.child(dest).write_str(&text).unwrap();
    }
}

#[tokio::test(flavor = "current_thread")]
async fn path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db() {
    // The two arms are the mandated pair — the SAME conductor_run composition start_run's thread
    // runs, and the `assert_cmd` CLI subprocess — pointed at ONE runs dir so a single runs.db holds
    // both rows.
    //
    // The comparison is BETWEEN THE TWO PERSISTED ENVELOPES, never each against a literal: this
    // test used to assert the Tauri row against hand-written expectations while a comment claimed
    // cli_smoke proved the other side, which is two tests each matching their own expectation in
    // two different databases — not parity.
    //
    // run_id / journal_emitted_at / read_back_observed_at / latency_ms are per-run wall-clock and
    // identity values (obs-plan §4/§5), so they are excluded by construction: each arm writes its
    // OWN journal under its OWN run_id, which is asserted rather than waved away.
    let dir = assert_fs::TempDir::new().unwrap();
    stage_repo_root(&dir, "error-baseline-spike");
    // A NON-default dir name, so CONDUCTOR_RUNS_DIR is load-bearing rather than decorative: if the
    // handle were ignored the CLI would write to the default `runs/` and the read-back below would
    // find nothing. Repo-relative — resolve_under rejects absolute handles by design.
    const SHARED_RUNS: &str = "shared-runs";
    let runs_dir = dir.path().join(SHARED_RUNS);
    let scenario = Scenario::from_toml_str(
        &std::fs::read_to_string(dir.path().join("scenarios/error-baseline-spike.toml")).unwrap(),
    )
    .unwrap();

    // --- CLI arm: the real binary, in a subprocess, rooted at the shared dir.
    // `CARGO_BIN_EXE_conductor` is set by Cargo for this package's test targets, so the path is a
    // build-graph fact rather than an artifact that happened to be present.
    // The injection metachar forces the read-back path unreachable (rejected before any spawn) —
    // the host-independent Blocked lever, so neither arm needs a live Pulse.
    assert_cmd::Command::new(env!("CARGO_BIN_EXE_conductor"))
        .current_dir(dir.path())
        .env("ANDROMEDA_PULSE_DATA_DIR", "pulse;injection")
        .env("CONDUCTOR_RUNS_DIR", SHARED_RUNS)
        .args(["run", "error-baseline-spike"])
        .assert()
        .success();
    let cli_run_id = latest_run_id(&runs_dir)
        .expect("the shared runs dir reads")
        .expect("the CLI arm persisted a run");

    // --- Tauri arm: the composition start_run's background thread runs, same scenario + seed,
    // into the SAME runs dir. Captured after the CLI arm so latest_run_id above is unambiguous.
    // SAFETY: this test binary holds exactly one test, so nothing else in the process reads env
    // concurrently, and it is set before any runtime or thread reads it; the CLI arm passes the
    // same value through the child's own .env instead.
    unsafe { std::env::set_var("ANDROMEDA_PULSE_DATA_DIR", "pulse;injection") };
    let pf = conductor_run::preflight(&dir.path().join("contracts/mcp-contract.toml"))
        .await
        .unwrap();
    conductor_run::drive_run(
        &pf,
        std::slice::from_ref(&scenario),
        "run-path7-tauri",
        &runs_dir,
        &EnvelopeStatus::InEnvelope,
        &HeadlessResolver::proceed(),
        |_| {},
        || false,
    )
    .await
    .expect("the blocked spine is infallible");

    // --- The comparison. Each arm's envelope is read back from its own journal in the one dir.
    assert_ne!(cli_run_id, "run-path7-tauri", "each run carries its own identity");
    let cli = read_run_journal(&runs_dir, &cli_run_id).expect("the CLI arm's journal reads");
    let tauri = read_run_journal(&runs_dir, "run-path7-tauri").expect("the Tauri arm's journal reads");
    assert_eq!(cli.len(), 1, "one scenario ⇒ one envelope per arm");
    assert_eq!(tauri.len(), 1);
    let (cli, tauri) = (&cli[0], &tauri[0]);

    // Rendered as one string so a failure names every field that diverged, not just the first.
    let divergences: Vec<String> = [
        ("verdict", format!("{:?}", cli.verdict), format!("{:?}", tauri.verdict)),
        ("state", format!("{:?}", cli.state), format!("{:?}", tauri.state)),
        ("seed", cli.seed.to_string(), tauri.seed.to_string()),
        ("scenario", cli.scenario.clone(), tauri.scenario.clone()),
        ("p_ids", format!("{:?}", cli.p_ids), format!("{:?}", tauri.p_ids)),
        ("slo_tier", format!("{:?}", cli.slo_tier), format!("{:?}", tauri.slo_tier)),
    ]
    .into_iter()
    .filter(|(_, c, t)| c != t)
    .map(|(field, c, t)| format!("{field}: cli={c} tauri={t}"))
    .collect();
    assert_eq!(divergences.join(" | "), "", "the two surfaces must write an equal envelope");

    // The shared seed is the scenario's, not a per-arm default — parity over a wrong seed on both
    // sides would be equally "equal" and prove nothing.
    assert_eq!(cli.seed, scenario.seed, "both arms ran the scenario's declared seed");
    assert!(matches!(cli.state, ReportState::Blocked), "no live Pulse ⇒ Blocked on both arms");
    assert!(cli.verdict.is_none(), "a blocked row carries no verdict");

    // Each arm wrote its OWN journal — parity is field equality, never artifact identity.
    for id in [cli_run_id.as_str(), "run-path7-tauri"] {
        assert!(runs_dir.join(format!("{id}.jsonl")).is_file(), "{id} has its own journal");
    }
    assert!(runs_dir.join("runs.db").is_file(), "both arms wrote into ONE runs.db");
}
