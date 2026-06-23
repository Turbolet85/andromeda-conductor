//! CLI E2E smoke — the verb surface + the no-Pulse `Blocked` spine, driven via `assert_cmd`.
//!
//! Each invocation forces the read-back path unreachable (an injection-metacharacter
//! `ANDROMEDA_PULSE_DATA_DIR`, rejected before any spawn) so the run deterministically reaches the
//! `Blocked` envelope on any host without spawning the live Pulse sidecar (the measured leg is
//! operator-gated — Epoch-10).

use assert_cmd::Command;
use assert_fs::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;

fn copy_scenario(dir: &TempDir, stem: &str) {
    let src = format!("{}/../../scenarios/{stem}.toml", env!("CARGO_MANIFEST_DIR"));
    let toml = std::fs::read_to_string(&src).unwrap_or_else(|e| panic!("read {stem}.toml: {e}"));
    dir.child(format!("scenarios/{stem}.toml")).write_str(&toml).unwrap();
}

fn copy_manifest(dir: &TempDir) {
    let src = format!("{}/../../contracts/mcp-contract.toml", env!("CARGO_MANIFEST_DIR"));
    let toml = std::fs::read_to_string(&src).unwrap_or_else(|e| panic!("read mcp-contract.toml: {e}"));
    dir.child("contracts/mcp-contract.toml").write_str(&toml).unwrap();
}

/// A `conductor` command rooted at `dir` with the read-back path forced unreachable.
fn conductor(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("conductor").expect("conductor binary builds");
    cmd.current_dir(dir.path());
    cmd.env("ANDROMEDA_PULSE_DATA_DIR", "pulse;injection");
    cmd
}

fn blocked_state(dir: &TempDir) -> serde_json::Value {
    let runs = std::fs::read_dir(dir.path().join("runs")).expect("runs dir exists");
    let journal = runs
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().and_then(|x| x.to_str()) == Some("jsonl"))
        .expect("a journal was written");
    let text = std::fs::read_to_string(journal).unwrap();
    serde_json::from_str(text.lines().next().expect("a journal line")).unwrap()
}

#[test]
fn help_lists_the_verbs() {
    Command::cargo_bin("conductor")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("run")
                .and(predicate::str::contains("suite"))
                .and(predicate::str::contains("report"))
                .and(predicate::str::contains("preflight"))
                .and(predicate::str::contains("coverage")),
        );
}

#[test]
fn no_subcommand_is_a_usage_error() {
    Command::cargo_bin("conductor").unwrap().assert().failure();
}

#[test]
fn run_by_name_blocks_without_pulse_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir)
        .args(["run", "error-baseline-spike"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[BLOCKED]").and(predicate::str::contains("error-baseline-spike")));

    let record = blocked_state(&dir);
    assert_eq!(record["state"], serde_json::json!("Blocked"));
    assert_eq!(record["scenario"], serde_json::json!("error-baseline-spike"));
    assert!(record["verdict"].is_null(), "a blocked row carries no verdict");
}

#[test]
fn run_resolves_a_scenario_by_p_id() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir)
        .args(["run", "P-009"])
        .assert()
        .success()
        .stdout(predicate::str::contains("error-baseline-spike"));
}

#[test]
fn run_with_unknown_target_is_an_error() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir).args(["run", "no-such-scenario"]).assert().failure();
}

#[test]
fn suite_runs_the_catalog_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");
    copy_scenario(&dir, "latency-regression");

    conductor(&dir).args(["suite"]).assert().success().stdout(
        predicate::str::contains("error-baseline-spike").and(predicate::str::contains("latency-regression")),
    );
}

#[test]
fn report_renders_the_latest_run() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir).args(["run", "error-baseline-spike"]).assert().success();
    conductor(&dir)
        .args(["report"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Run report").and(predicate::str::contains("[BLOCKED]")));
}

#[test]
fn preflight_json_blocks_without_pulse_and_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    copy_manifest(&dir);

    let assert = conductor(&dir).args(["preflight", "--json"]).assert().failure();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let state: serde_json::Value = serde_json::from_str(stdout.trim()).expect("preflight emits json");

    assert_eq!(state["ready"], serde_json::json!(false), "no live Pulse ⇒ not ready");
    assert!(state["blocked_precondition"].is_string(), "a blocked gate names its precondition");
    assert!(
        !stdout.contains(dir.path().to_str().unwrap()),
        "the readiness json must not leak the host path"
    );
}

#[test]
fn coverage_lists_all_sixty_pids() {
    Command::cargo_bin("conductor")
        .unwrap()
        .arg("coverage")
        .assert()
        .success()
        .stdout(predicate::str::contains("P-001").and(predicate::str::contains("P-060")));
}

#[test]
fn coverage_write_regenerates_the_matrix_file() {
    let dir = TempDir::new().unwrap();
    Command::cargo_bin("conductor")
        .unwrap()
        .current_dir(dir.path())
        .args(["coverage", "--write"])
        .assert()
        .success();
    dir.child("coverage-matrix.md").assert(predicate::path::exists());
}

#[test]
fn piped_coverage_output_carries_no_ansi_escapes() {
    // assert_cmd pipes the child's stdout (not a tty) ⇒ color is stripped ⇒ no escape bytes.
    let assert = Command::cargo_bin("conductor").unwrap().arg("coverage").assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(!stdout.contains('\u{1b}'), "piped output must carry no ANSI escapes");
}
