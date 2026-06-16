//! CLI smoke exemplar — proves the assert_cmd + assert_fs + predicates wiring against the
//! `conductor` binary. The bin only initializes self-observation and exits 0 today; the real
//! agent-run verbs land in Epoch 8 (test-plan §4).

use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn conductor_binary_exits_success() {
    Command::cargo_bin("conductor")
        .expect("conductor binary builds")
        .assert()
        .success();
}

#[test]
fn tempdir_journal_fixture_round_trips() {
    let dir = assert_fs::TempDir::new().expect("create temp dir");
    let journal = dir.child("runs/424242.jsonl");
    journal.touch().expect("touch journal fixture");
    journal.assert(predicate::path::exists());
    dir.close().expect("close temp dir");
}
