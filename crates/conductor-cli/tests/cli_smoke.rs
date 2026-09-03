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
    dir.child(format!("scenarios/{stem}.toml"))
        .write_str(&toml)
        .unwrap();
}

fn copy_manifest(dir: &TempDir) {
    let src = format!(
        "{}/../../contracts/mcp-contract.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let toml =
        std::fs::read_to_string(&src).unwrap_or_else(|e| panic!("read mcp-contract.toml: {e}"));
    dir.child("contracts/mcp-contract.toml")
        .write_str(&toml)
        .unwrap();
}

fn copy_capability_manifest(dir: &TempDir) {
    let src = format!(
        "{}/../../contracts/pulse-capabilities.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let toml = std::fs::read_to_string(&src)
        .unwrap_or_else(|e| panic!("read pulse-capabilities.toml: {e}"));
    dir.child("contracts/pulse-capabilities.toml")
        .write_str(&toml)
        .unwrap();
}

fn copy_load_envelope(dir: &TempDir) {
    let src = format!(
        "{}/../../contracts/pulse-load-envelope.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let toml = std::fs::read_to_string(&src)
        .unwrap_or_else(|e| panic!("read pulse-load-envelope.toml: {e}"));
    dir.child("contracts/pulse-load-envelope.toml")
        .write_str(&toml)
        .unwrap();
}

/// A `conductor` command rooted at `dir` with the read-back path forced unreachable. Every scenario
/// load checks its P-IDs against the capability manifest and every run is judged against the load
/// envelope, so a repo-shaped root always carries both.
fn conductor(dir: &TempDir) -> Command {
    copy_capability_manifest(dir);
    copy_load_envelope(dir);
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
        .stdout(
            predicate::str::contains("[BLOCKED]")
                .and(predicate::str::contains("error-baseline-spike")),
        );

    let record = blocked_state(&dir);
    assert_eq!(record["state"], serde_json::json!("Blocked"));
    assert_eq!(
        record["scenario"],
        serde_json::json!("error-baseline-spike")
    );
    assert!(
        record["verdict"].is_null(),
        "a blocked row carries no verdict"
    );
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
fn run_with_unknown_target_is_a_sanitized_error_with_a_hint() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    let assert = conductor(&dir)
        .args(["run", "no-such-scenario"])
        .assert()
        .failure();
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(
        stderr.contains("error:"),
        "the edge renders an error: label: {stderr}"
    );
    assert!(
        stderr.contains("hint:"),
        "the edge renders a hint: line: {stderr}"
    );
    assert!(
        !stderr.contains(dir.path().to_str().unwrap()),
        "the sanitized edge must not leak the host path: {stderr}"
    );
}

#[test]
fn agent_mode_routes_self_obs_to_the_log_file_not_stderr() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    let assert = conductor(&dir)
        .args(["run", "error-baseline-spike", "--agent-mode"])
        .assert()
        .success();

    // The self-obs JSON stream went to logs/agent-latest.jsonl (sibling of runs/), not stderr.
    let log = dir.child("logs/agent-latest.jsonl");
    log.assert(predicate::path::exists());
    let body = std::fs::read_to_string(log.path()).unwrap();
    let first: serde_json::Value =
        serde_json::from_str(body.lines().next().expect("a self-obs line")).unwrap();
    assert!(
        first.get("run_id").is_some(),
        "every self-obs line carries run_id: {first}"
    );
    assert!(
        body.contains("observability initialized"),
        "the startup line went to the file"
    );

    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(
        !stderr.contains("observability initialized"),
        "agent mode keeps the self-obs JSON off stderr: {stderr}"
    );

    // The Blocked envelope is still persisted — the self-obs sink is a distinct artifact.
    assert_eq!(blocked_state(&dir)["state"], serde_json::json!("Blocked"));
}

/// The must-trace span chain reaches the real artifact (obs-plan §4 Critical Path 1). On this
/// no-Pulse spine the gate blocks before the timeline, so `timeline.execute` / `emit.batch` /
/// `verify.readback*` never run — their nesting beneath the root is proven at the layer's own tier
/// (`conductor_core::obs`), and the live chain is the operator-gated leg.
#[test]
fn a_run_emits_the_scenario_run_root_and_the_report_seam_spans() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir)
        .args(["run", "error-baseline-spike", "--agent-mode"])
        .assert()
        .success();

    let body = std::fs::read_to_string(dir.child("logs/agent-latest.jsonl").path()).unwrap();
    let lines: Vec<serde_json::Value> = body
        .lines()
        .map(|l| serde_json::from_str(l).expect("each line is JSON"))
        .collect();

    let record = |span: &str, event: &str| {
        lines
            .iter()
            .find(|l| {
                l["span"] == serde_json::json!(span) && l["span_event"] == serde_json::json!(event)
            })
            .unwrap_or_else(|| panic!("no {event} record for span {span} in:\n{body}"))
            .clone()
    };

    let root = record("scenario.run", "new");
    assert_eq!(root["seed"], serde_json::json!(424242));
    assert_eq!(root["scenario"], serde_json::json!("error-baseline-spike"));
    assert_eq!(root["p_ids"], serde_json::json!("P-009,P-010"));
    record("scenario.run", "close");

    // The report seam's two spans correlate to the same run by run_id, not by nesting.
    let generate = record("report.generate", "new");
    let insert = record("db.insert_run", "new");
    assert_eq!(generate["state"], serde_json::json!("Blocked"));
    assert_eq!(generate["verdict"], serde_json::json!("null"));
    assert_eq!(insert["row_count"], serde_json::json!(1));
    assert_eq!(generate["run_id"], root["run_id"]);
    assert_eq!(insert["run_id"], root["run_id"]);

    // Artifact hygiene: no span record leaks a host path (security-plan §Error Handling).
    for line in lines.iter().filter(|l| l.get("span").is_some()) {
        let text = line.to_string();
        assert!(
            !text.contains(":\\\\"),
            "host path in a span record: {text}"
        );
        assert!(
            !text.contains("/Users/"),
            "host path in a span record: {text}"
        );
    }
}

#[test]
fn suite_runs_the_catalog_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");
    copy_scenario(&dir, "latency-regression");

    conductor(&dir).args(["suite"]).assert().success().stdout(
        predicate::str::contains("error-baseline-spike")
            .and(predicate::str::contains("latency-regression")),
    );
}

#[test]
fn report_renders_the_latest_run() {
    let dir = TempDir::new().unwrap();
    copy_scenario(&dir, "error-baseline-spike");

    conductor(&dir)
        .args(["run", "error-baseline-spike"])
        .assert()
        .success();
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

    let assert = conductor(&dir)
        .args(["preflight", "--json"])
        .assert()
        .failure();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let state: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("preflight emits json");

    assert_eq!(
        state["ready"],
        serde_json::json!(false),
        "no live Pulse ⇒ not ready"
    );
    assert!(
        state["blocked_precondition"].is_string(),
        "a blocked gate names its precondition"
    );
    assert!(
        !stdout.contains(dir.path().to_str().unwrap()),
        "the readiness json must not leak the host path"
    );
}

#[test]
fn coverage_lists_every_classified_pid() {
    let assert = Command::cargo_bin("conductor")
        .unwrap()
        .arg("coverage")
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    for row in conductor_core::coverage_matrix() {
        assert!(
            stdout.contains(row.p_id),
            "missing {} in coverage output",
            row.p_id
        );
    }
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
    dir.child("coverage-matrix.md")
        .assert(predicate::path::exists());
}

#[test]
fn piped_coverage_output_carries_no_ansi_escapes() {
    // assert_cmd pipes the child's stdout (not a tty) ⇒ color is stripped ⇒ no escape bytes.
    let assert = Command::cargo_bin("conductor")
        .unwrap()
        .arg("coverage")
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(
        !stdout.contains('\u{1b}'),
        "piped output must carry no ANSI escapes"
    );
}

/// Run `conductor preconditions`, returning (stdout, exit-success). The binary resolves through the
/// declared build-graph edge. No `CONDUCTOR_RUNS_DIR` is set: the probe writes no artifact, and that
/// handle is repo-relative by design (`resolve_under` rejects an absolute value), so overriding it
/// to a temp dir fails path resolution before the verb ever runs. The verdict is host-dependent by
/// design — this host runs no Pulse — so the assertions below key on the SHAPE the probe must always
/// emit, never on which arm it took.
fn run_preconditions(args: &[&str]) -> (String, bool) {
    let output = assert_cmd::Command::new(env!("CARGO_BIN_EXE_conductor"))
        .args(args)
        .env("NO_COLOR", "1")
        .output()
        .expect("the probe runs");
    (
        String::from_utf8(output.stdout).unwrap(),
        output.status.success(),
    )
}

#[test]
fn preconditions_names_every_subject_it_reports() {
    let (stdout, satisfied) = run_preconditions(&["preconditions"]);

    assert!(
        stdout.contains("[PRECONDITION]"),
        "the ASCII label always renders: {stdout}"
    );
    assert!(
        !stdout.contains('\u{1b}'),
        "piped output carries no ANSI escapes: {stdout}"
    );
    if !satisfied {
        // A gate's failure must say WHAT it observed, never only a count (test-plan §11).
        assert!(
            stdout.contains(':') && stdout.contains('—'),
            "each unmet subject names its statement and causes: {stdout}"
        );
    }
}

#[test]
fn preconditions_json_is_parseable_and_carries_the_checked_at_stamp() {
    let (stdout, satisfied) = run_preconditions(&["preconditions", "--json"]);

    let value: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(
        value["satisfied"],
        serde_json::Value::Bool(satisfied),
        "exit code matches payload"
    );
    assert!(value["unmet"].is_array());
    let checked_at = value["checked_at"]
        .as_str()
        .expect("checked_at is a string");
    // Colon-delimited RFC-3339, never the filesystem-safe run_id form (arch §Timestamp formats).
    assert!(
        checked_at.contains(':') && checked_at.ends_with('Z'),
        "RFC-3339: {checked_at}"
    );

    for finding in value["unmet"].as_array().unwrap() {
        for key in ["subject", "statement", "causes"] {
            assert!(
                finding[key].is_string(),
                "every finding carries {key}: {finding}"
            );
        }
    }
}

#[test]
fn preconditions_output_leaks_no_host_path_or_env_value() {
    let (plain, _) = run_preconditions(&["preconditions"]);
    let (json, _) = run_preconditions(&["preconditions", "--json"]);

    for text in [&plain, &json] {
        for token in ["%APPDATA%", "/Users/", "/home/", ".cargo", ".rustup"] {
            assert!(!text.contains(token), "host-path token {token} in {text}");
        }
        // The drive-letter token is WORD-ANCHORED: the egress statement carries `127.0.0.1:4317`
        // and an unanchored `[A-Za-z]:[\\/]` matches the `p:/` inside any `scheme://` form.
        let bytes = text.as_bytes();
        for (i, w) in bytes.windows(2).enumerate() {
            let looks_like_drive = w[0].is_ascii_alphabetic() && w[1] == b':';
            let followed_by_sep = matches!(bytes.get(i + 2), Some(b'\\') | Some(b'/'));
            let at_word_boundary = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            assert!(
                !(looks_like_drive && followed_by_sep && at_word_boundary),
                "drive-letter host path in {text}"
            );
        }
        // The probe reports resolvability; it never echoes what it searched.
        assert!(
            !text.contains("PATH="),
            "the PATH value is never echoed: {text}"
        );
    }
}
