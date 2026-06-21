//! JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.

mod db;
mod journal;
mod report;

pub use db::{RunsDb, RunsDbError};
pub use journal::{JournalError, JournalWriter};
pub use report::{ReportError, RunReport};
