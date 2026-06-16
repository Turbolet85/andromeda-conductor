//! JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.

mod journal;

pub use journal::{JournalError, JournalWriter};
