//! JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.

mod db;
mod journal;

pub use db::{RunsDb, RunsDbError};
pub use journal::{JournalError, JournalWriter};
