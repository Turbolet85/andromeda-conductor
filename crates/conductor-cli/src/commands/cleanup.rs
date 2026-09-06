//! `conductor cleanup <run_id>` — remove a run's `runs.db` rows across every table it wrote.
//!
//! The teardown half of the harness `cleanup` verb. It lives in the binary rather than in the shells
//! because the prescribed form is rusqlite BOUND parameters (test-plan §3), which the `sqlite3` CLI
//! cannot express — and because no `sqlite3` need be installed for teardown to work at all. The
//! shells keep the file removal; this owns the three tables.

use std::process::ExitCode;

use conductor_report::RunsDb;

use crate::paths::Paths;
use crate::render;

pub fn cleanup(run_id: &str, paths: &Paths) -> anyhow::Result<ExitCode> {
    let mut db = RunsDb::open(&paths.runs_dir)?;
    let removed = db.delete_run(run_id)?;
    println!(
        "cleanup: {} rows removed for {}",
        removed,
        render::paint(run_id, render::ID_CYAN)
    );
    Ok(ExitCode::SUCCESS)
}
