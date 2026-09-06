//! The `runs.db` cross-run SQLite index.
//!
//! [`RunsDb`] is the thin, synchronous index over the JSONL journal ground truth (arch §Run-History
//! Persistence) — a sibling to [`JournalWriter`](crate::JournalWriter). It opens/creates `runs.db`
//! under an already-resolved `runs_dir` (the cli edge applies `CONDUCTOR_RUNS_DIR`), bootstraps a
//! fixed-column schema (`CREATE TABLE IF NOT EXISTS`, no migration framework — arch §Data-model
//! conventions), and persists one [`RunRecord`] per `(run_id, scenario)` check via bound parameters
//! (security-plan §Input Validation). The Blocked-row NULL rule falls out of the envelope — a
//! [`RunRecord::blocked`] row carries `None` in the five measurement fields, which bind to SQL `NULL`.
//! A storage fault is a harness fault ([`RunsDbError`] → `Err`), never a verification verdict (the
//! verdict/error wall). Deliberately off the async runtime (arch §Database).

use std::path::Path;

use conductor_core::{CheckRecord, EnvelopeStatus, RunRecord};
use rusqlite::{Connection, OptionalExtension};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS runs (
    run_id                TEXT    NOT NULL,
    seed                  INTEGER NOT NULL,
    scenario              TEXT    NOT NULL,
    p_ids                 TEXT    NOT NULL,
    verdict               TEXT,
    state                 TEXT    NOT NULL,
    journal_emitted_at    TEXT,
    read_back_observed_at TEXT,
    latency_ms            INTEGER,
    slo_tier              TEXT    NOT NULL,
    fingerprints          TEXT,
    PRIMARY KEY (run_id, scenario)
);
CREATE TABLE IF NOT EXISTS run_envelope (
    run_id                TEXT    NOT NULL PRIMARY KEY,
    classification        TEXT    NOT NULL,
    cause                 TEXT
);
CREATE TABLE IF NOT EXISTS run_check (
    run_id                TEXT    NOT NULL,
    scenario              TEXT    NOT NULL,
    check_index           INTEGER NOT NULL,
    kind                  TEXT    NOT NULL,
    verdict               TEXT    NOT NULL,
    state                 TEXT    NOT NULL,
    latency_ms            INTEGER NOT NULL,
    deadline_ms           INTEGER NOT NULL,
    budget_ms             INTEGER,
    PRIMARY KEY (run_id, scenario, check_index)
);";

/// A harness fault from the runs.db storage seam — never a verification outcome (the verdict/error
/// wall). `#[non_exhaustive]` so later report-seam chunks extend the fault surface.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RunsDbError {
    /// Creating the runs directory failed.
    #[error("runs.db io error: {0}")]
    Io(#[from] std::io::Error),
    /// Opening, schema-creating, writing, or reading `runs.db` failed.
    #[error("runs.db sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// (De)serializing a JSON1 array column (`p_ids` / `fingerprints`) or an enum wire form failed.
    #[error("runs.db json error: {0}")]
    Json(#[from] serde_json::Error),
}

/// The embedded `runs.db` cross-run index — one row per `(run_id, scenario)` check.
///
/// `runs_dir` is the already-resolved artifact directory (the cli edge applies `CONDUCTOR_RUNS_DIR` +
/// `resolve_under`); the index is synchronous and off the async runtime by design (arch §Database).
pub struct RunsDb {
    conn: Connection,
}

impl RunsDb {
    /// Open `runs.db` under `runs_dir` (created if absent), schema bootstrapped, ready to insert.
    pub fn open(runs_dir: &Path) -> Result<Self, RunsDbError> {
        std::fs::create_dir_all(runs_dir)?;
        let db = Self {
            conn: Connection::open(runs_dir.join("runs.db"))?,
        };
        db.conn.execute_batch(SCHEMA)?;
        Ok(db)
    }

    /// Persist one `record` as a row, mapping the envelope onto the fixed column contract.
    ///
    /// The five measurement fields of a blocked record are `None` and bind to SQL `NULL` (the
    /// Blocked-row NULL rule, arch §Standard Contracts); `p_ids` / `fingerprints` are stored as JSON1
    /// TEXT arrays; `seed` is bit-cast to the signed-`INTEGER` column. A duplicate `(run_id, scenario)`
    /// raises the PK constraint as an `Err` — never a silent clobber.
    ///
    /// Carries the `db.insert_run` span (obs-plan §4 Critical Path 1) — a Client-kind span, since
    /// rusqlite is a synchronous boundary off the async runtime. `row_count` is the one row this
    /// statement writes; the field is set at span creation because the layer records span attributes
    /// on the `new` record.
    #[tracing::instrument(name = "db.insert_run", skip_all, fields(row_count = 1))]
    pub fn insert(&self, record: &RunRecord) -> Result<(), RunsDbError> {
        let p_ids = serde_json::to_string(&record.p_ids)?;
        let fingerprints = record
            .fingerprints
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let verdict = match &record.verdict {
            Some(v) => Some(value_as_wire(serde_json::to_value(v)?)),
            None => None,
        };
        let state = value_as_wire(serde_json::to_value(record.state)?);
        let slo_tier = value_as_wire(serde_json::to_value(record.slo_tier)?);
        self.conn.execute(
            "INSERT INTO runs
                (run_id, seed, scenario, p_ids, verdict, state,
                 journal_emitted_at, read_back_observed_at, latency_ms, slo_tier, fingerprints)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                record.run_id,
                record.seed as i64,
                record.scenario,
                p_ids,
                verdict,
                state,
                record.journal_emitted_at,
                record.read_back_observed_at,
                record.latency_ms,
                slo_tier,
                fingerprints,
            ],
        )?;
        Ok(())
    }

    /// Persist one graded check as a row in the per-check table.
    ///
    /// Deliberately its own table rather than columns on `runs`: the qualifier is CHECK-level while
    /// `runs` is keyed `(run_id, scenario)`, so folding it in would either change that key or widen
    /// the eleven-column contract — both of which the envelope pins (arch §Standard Contracts). The
    /// `run_envelope` split is the same reasoning at run-level grain. `latency_ms`/`deadline_ms` are
    /// NOT NULL because only a MEASURED check produces a row at all; `budget_ms` is NULL when the
    /// check inherits its scenario's tier. A duplicate `(run_id, scenario, check_index)` raises the
    /// PK constraint as an `Err`, never a silent clobber.
    #[tracing::instrument(name = "db.insert_run", skip_all, fields(row_count = 1))]
    pub fn insert_check(&self, check: &CheckRecord) -> Result<(), RunsDbError> {
        let kind = value_as_wire(serde_json::to_value(check.kind)?);
        let verdict = value_as_wire(serde_json::to_value(check.verdict)?);
        let state = value_as_wire(serde_json::to_value(check.state)?);
        self.conn.execute(
            "INSERT INTO run_check
                (run_id, scenario, check_index, kind, verdict, state,
                 latency_ms, deadline_ms, budget_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                check.run_id,
                check.scenario,
                check.check_index as i64,
                kind,
                verdict,
                state,
                check.latency_ms,
                check.deadline_ms,
                check.budget_ms.map(i64::from),
            ],
        )?;
        Ok(())
    }

    /// Read back every graded check for `(run_id, scenario)`, in declaration order.
    pub fn checks_for(
        &self,
        run_id: &str,
        scenario: &str,
    ) -> Result<Vec<CheckRecord>, RunsDbError> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, scenario, check_index, kind, verdict, state,
                    latency_ms, deadline_ms, budget_ms
             FROM run_check WHERE run_id = ?1 AND scenario = ?2 ORDER BY check_index",
        )?;
        let rows = stmt
            .query_map(rusqlite::params![run_id, scenario], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, Option<i64>>(8)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        rows.into_iter()
            .map(
                |(
                    run_id,
                    scenario,
                    check_index,
                    kind,
                    verdict,
                    state,
                    latency,
                    deadline,
                    budget,
                )| {
                    Ok(CheckRecord {
                        run_id,
                        scenario,
                        check_index: check_index as usize,
                        kind: serde_json::from_value(serde_json::Value::String(kind))?,
                        verdict: serde_json::from_value(serde_json::Value::String(verdict))?,
                        state: serde_json::from_value(serde_json::Value::String(state))?,
                        latency_ms: latency,
                        deadline_ms: deadline,
                        budget_ms: budget.map(|b| b as u32),
                    })
                },
            )
            .collect()
    }

    /// Persist a run's standing against the SUT load envelope — one row per run, in its own table.
    ///
    /// Deliberately NOT a column on `runs`: the qualifier is run-level, and the eleven-column check
    /// row plus its `(run_id, scenario)` key stay exactly as they are, so `state` remains the closed
    /// five-variant set (arch §Standard Contracts). A repeat `run_id` raises the PK constraint as an
    /// `Err`, never a silent clobber.
    pub fn insert_envelope(
        &self,
        run_id: &str,
        status: &EnvelopeStatus,
    ) -> Result<(), RunsDbError> {
        self.conn.execute(
            "INSERT INTO run_envelope (run_id, classification, cause) VALUES (?1, ?2, ?3)",
            rusqlite::params![run_id, status.label(), status.cause()],
        )?;
        Ok(())
    }

    /// Read back a run's envelope standing — `None` if the run recorded none.
    pub fn get_envelope(&self, run_id: &str) -> Result<Option<EnvelopeStatus>, RunsDbError> {
        let row = self
            .conn
            .query_row(
                "SELECT cause FROM run_envelope WHERE run_id = ?1",
                rusqlite::params![run_id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?;
        Ok(row.map(|cause| match cause {
            Some(cause) => EnvelopeStatus::EnvironmentSuspect(cause),
            None => EnvelopeStatus::InEnvelope,
        }))
    }

    /// Delete every row a run wrote, across all three tables, returning the rows removed.
    ///
    /// Teardown covers `runs`, `run_check` AND `run_envelope` — a run writes all three, so deleting
    /// from `runs` alone leaves orphans at the other two grains (test-plan §3 `cleanup`). Applied in
    /// one transaction so teardown is all-or-nothing, and idempotent: deleting nothing is success,
    /// which is what makes a re-run against a clean state a no-op.
    /// Every statement is a LITERAL with the id bound as `?1` — never `format!`-assembled, not even
    /// over a hard-coded table list (security-plan §Security Anti-Patterns → Input).
    pub fn delete_run(&mut self, run_id: &str) -> Result<usize, RunsDbError> {
        let tx = self.conn.transaction()?;
        let id = rusqlite::params![run_id];
        let removed = tx.execute("DELETE FROM runs WHERE run_id = ?1", id)?
            + tx.execute("DELETE FROM run_check WHERE run_id = ?1", id)?
            + tx.execute("DELETE FROM run_envelope WHERE run_id = ?1", id)?;
        tx.commit()?;
        Ok(removed)
    }

    /// Read back the row for `(run_id, scenario)`, reconstructing the [`RunRecord`] — `None` if absent.
    pub fn get(&self, run_id: &str, scenario: &str) -> Result<Option<RunRecord>, RunsDbError> {
        let raw = self
            .conn
            .query_row(
                "SELECT run_id, seed, scenario, p_ids, verdict, state,
                        journal_emitted_at, read_back_observed_at, latency_ms, slo_tier, fingerprints
                 FROM runs WHERE run_id = ?1 AND scenario = ?2",
                rusqlite::params![run_id, scenario],
                |row| {
                    Ok(RawRow {
                        run_id: row.get(0)?,
                        seed: row.get(1)?,
                        scenario: row.get(2)?,
                        p_ids: row.get(3)?,
                        verdict: row.get(4)?,
                        state: row.get(5)?,
                        journal_emitted_at: row.get(6)?,
                        read_back_observed_at: row.get(7)?,
                        latency_ms: row.get(8)?,
                        slo_tier: row.get(9)?,
                        fingerprints: row.get(10)?,
                    })
                },
            )
            .optional()?;
        match raw {
            Some(r) => Ok(Some(r.into_record()?)),
            None => Ok(None),
        }
    }
}

/// The raw column values of a `runs` row, before the JSON1 arrays + enum wire forms are parsed back
/// into a [`RunRecord`]. Parsing can't happen in the rusqlite row closure — it must return
/// `rusqlite::Error`, not the [`RunsDbError`] a `serde_json` failure produces.
struct RawRow {
    run_id: String,
    seed: i64,
    scenario: String,
    p_ids: String,
    verdict: Option<String>,
    state: String,
    journal_emitted_at: Option<String>,
    read_back_observed_at: Option<String>,
    latency_ms: Option<i64>,
    slo_tier: String,
    fingerprints: Option<String>,
}

impl RawRow {
    fn into_record(self) -> Result<RunRecord, RunsDbError> {
        Ok(RunRecord {
            journal_emitted_at: self.journal_emitted_at,
            read_back_observed_at: self.read_back_observed_at,
            run_id: self.run_id,
            seed: self.seed as u64,
            scenario: self.scenario,
            p_ids: serde_json::from_str(&self.p_ids)?,
            verdict: match self.verdict {
                Some(s) => Some(serde_json::from_value(serde_json::Value::String(s))?),
                None => None,
            },
            state: serde_json::from_value(serde_json::Value::String(self.state))?,
            latency_ms: self.latency_ms,
            slo_tier: serde_json::from_value(serde_json::Value::String(self.slo_tier))?,
            fingerprints: match self.fingerprints {
                Some(s) => Some(serde_json::from_str(&s)?),
                None => None,
            },
        })
    }
}

/// A `Verdict` / `ReportState` / `SloTier` serializes to a JSON string scalar; take the bare string so
/// the stored TEXT spelling stays identical to the JSONL wire form (the `#[serde(rename)]` is the
/// single source of truth). The non-string arm is unreachable for those enums.
fn value_as_wire(value: serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s,
        other => other.to_string(),
    }
}

#[cfg(test)]
impl RunsDb {
    fn open_in_memory() -> Result<Self, RunsDbError> {
        let db = Self {
            conn: Connection::open_in_memory()?,
        };
        db.conn.execute_batch(SCHEMA)?;
        Ok(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;
    use conductor_core::{PId, ReportState, SloTier, Verdict};

    /// The run-level qualifier round-trips through its own table, in both variants.
    #[test]
    fn envelope_status_round_trips_per_run() {
        let db = RunsDb::open_in_memory().unwrap();
        let suspect = EnvelopeStatus::EnvironmentSuspect(
            "scenario \"activity-floor\" runs 3900s, over the ceiling of 600s".to_string(),
        );
        db.insert_envelope("run-suspect", &suspect).unwrap();
        db.insert_envelope("run-ok", &EnvelopeStatus::InEnvelope)
            .unwrap();

        assert_eq!(db.get_envelope("run-suspect").unwrap(), Some(suspect));
        assert_eq!(
            db.get_envelope("run-ok").unwrap(),
            Some(EnvelopeStatus::InEnvelope)
        );
        assert_eq!(
            db.get_envelope("no-such-run").unwrap(),
            None,
            "an unrecorded run reads None"
        );
    }

    #[test]
    fn a_duplicate_envelope_insert_is_a_harness_error_not_a_clobber() {
        let db = RunsDb::open_in_memory().unwrap();
        db.insert_envelope("run-1", &EnvelopeStatus::InEnvelope)
            .unwrap();
        assert!(
            db.insert_envelope("run-1", &EnvelopeStatus::InEnvelope)
                .is_err()
        );
    }

    /// The qualifier lives in its own table precisely so the check row's contract is untouched:
    /// eleven columns, `(run_id, scenario)` key, `state` still the closed five-variant set.
    #[test]
    fn the_runs_table_contract_is_unchanged_by_the_envelope_table() {
        let db = RunsDb::open_in_memory().unwrap();
        let columns: Vec<String> = db
            .conn
            .prepare("SELECT name FROM pragma_table_info('runs')")
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(
            columns,
            [
                "run_id",
                "seed",
                "scenario",
                "p_ids",
                "verdict",
                "state",
                "journal_emitted_at",
                "read_back_observed_at",
                "latency_ms",
                "slo_tier",
                "fingerprints",
            ],
            "the eleven-column envelope contract must not move"
        );

        // and a check row still round-trips beside a recorded run-level qualifier
        db.insert(&measured("run-1", "s")).unwrap();
        db.insert_envelope(
            "run-1",
            &EnvelopeStatus::EnvironmentSuspect("over".to_string()),
        )
        .unwrap();
        assert_eq!(
            db.get("run-1", "s").unwrap().unwrap().state,
            ReportState::Pass
        );
    }

    fn check_record(check_index: usize, budget_ms: Option<u32>) -> CheckRecord {
        CheckRecord {
            run_id: "run-1".to_string(),
            scenario: "s".to_string(),
            check_index,
            kind: conductor_core::ComparisonKind::Contains,
            verdict: Verdict::Pass,
            state: ReportState::Pass,
            latency_ms: 1840,
            deadline_ms: budget_ms.map_or(5_000, i64::from),
            budget_ms,
        }
    }

    #[test]
    fn check_rows_round_trip_in_declaration_order() {
        let db = RunsDb::open_in_memory().unwrap();
        let rows = [check_record(0, Some(2_000)), check_record(1, None)];
        for row in &rows {
            db.insert_check(row).unwrap();
        }
        let back = db.checks_for("run-1", "s").unwrap();
        assert_eq!(
            back, rows,
            "kind/verdict/state wire forms and budgets survive the round trip"
        );
        assert_eq!(back[0].budget_ms, Some(2_000));
        assert_eq!(
            back[1].budget_ms, None,
            "an inherited budget stores NULL, reads back None"
        );
    }

    /// The count-zero teardown verification test-plan §3 has always specified: cleanup covers every
    /// table a run writes, or it leaves orphans behind at the finer grains.
    #[test]
    fn delete_run_empties_all_three_tables_and_leaves_other_runs_alone() {
        let mut db = RunsDb::open_in_memory().unwrap();
        for run_id in ["run-1", "run-2"] {
            db.insert(&RunRecord::blocked(
                run_id,
                7,
                "s",
                vec![PId("P-003".to_string())],
                SloTier::Tier20s,
            ))
            .unwrap();
            db.insert_envelope(run_id, &EnvelopeStatus::InEnvelope)
                .unwrap();
        }
        db.insert_check(&check_record(0, None)).unwrap();

        let count = |db: &RunsDb, table: &str| -> i64 {
            let sql = match table {
                "runs" => "SELECT count(*) FROM runs WHERE run_id = ?1",
                "run_check" => "SELECT count(*) FROM run_check WHERE run_id = ?1",
                _ => "SELECT count(*) FROM run_envelope WHERE run_id = ?1",
            };
            db.conn
                .query_row(sql, rusqlite::params!["run-1"], |r| r.get(0))
                .unwrap()
        };
        assert_eq!(
            (
                count(&db, "runs"),
                count(&db, "run_check"),
                count(&db, "run_envelope")
            ),
            (1, 1, 1),
            "the subject is non-empty at all three grains before the delete — a zero that was \
             always zero would prove nothing"
        );

        assert_eq!(db.delete_run("run-1").unwrap(), 3);
        assert_eq!(
            (
                count(&db, "runs"),
                count(&db, "run_check"),
                count(&db, "run_envelope")
            ),
            (0, 0, 0)
        );
        assert_eq!(
            db.delete_run("run-1").unwrap(),
            0,
            "idempotent: a re-run against a clean state removes nothing and is not an error"
        );

        let survivors: i64 = db
            .conn
            .query_row(
                "SELECT count(*) FROM runs WHERE run_id = ?1",
                rusqlite::params!["run-2"],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(survivors, 1, "teardown is scoped to its own run_id");
    }

    #[test]
    fn a_duplicate_check_key_is_an_err_never_a_clobber() {
        let db = RunsDb::open_in_memory().unwrap();
        db.insert_check(&check_record(0, None)).unwrap();
        assert!(db.insert_check(&check_record(0, None)).is_err());
    }

    #[test]
    fn checks_for_is_empty_when_a_scenario_graded_nothing() {
        // A blocked row and a declare-only scenario write no check rows at all — the per-check
        // grain never synthesizes a value for something that was never measured.
        let db = RunsDb::open_in_memory().unwrap();
        db.insert(&RunRecord::blocked(
            "run-1",
            7,
            "declare-only",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        ))
        .unwrap();
        assert!(db.checks_for("run-1", "declare-only").unwrap().is_empty());
    }

    #[test]
    fn the_runs_row_keeps_its_eleven_columns_beside_the_new_table() {
        let db = RunsDb::open_in_memory().unwrap();
        let cols: Vec<String> = db
            .conn
            .prepare("SELECT name FROM pragma_table_info('runs')")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            cols.len(),
            11,
            "the per-check grain went to its own table: {cols:?}"
        );
    }

    fn measured(run_id: &str, scenario: &str) -> RunRecord {
        RunRecord::measured(
            run_id,
            424242,
            scenario,
            vec![PId("P-009".to_string()), PId("P-010".to_string())],
            Verdict::Pass,
            ReportState::Pass,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:07Z",
            1840,
            SloTier::Tier5s,
            vec!["fp-1".to_string()],
        )
    }

    #[test]
    fn measured_record_round_trips() {
        let db = RunsDb::open_in_memory().unwrap();
        let rec = measured("2026-06-16T21-10-06-abc", "error-baseline-spike");
        db.insert(&rec).unwrap();
        assert_eq!(db.get(&rec.run_id, &rec.scenario).unwrap(), Some(rec));
    }

    #[test]
    fn calibration_manualcheck_variant_round_trips() {
        // Different verdict (CalibrationRegion), state (ManualCheck), tier (<90s), and `Some([])`
        // fingerprints (distinct from a blocked row's `None`) — proves the wire-form mapping over a
        // second set of enum variants.
        let db = RunsDb::open_in_memory().unwrap();
        let rec = RunRecord::measured(
            "2026-06-16T21-10-06-cal",
            7,
            "severity-choice",
            vec![PId("P-019".to_string())],
            Verdict::CalibrationRegion,
            ReportState::ManualCheck,
            "2026-06-16T21:10:06Z",
            "2026-06-16T21:10:30Z",
            24_000,
            SloTier::Tier90s,
            vec![],
        );
        db.insert(&rec).unwrap();
        assert_eq!(
            db.get("2026-06-16T21-10-06-cal", "severity-choice")
                .unwrap(),
            Some(rec)
        );
    }

    #[test]
    fn blocked_row_nulls_the_five_measurement_columns() {
        let db = RunsDb::open_in_memory().unwrap();
        let rec = RunRecord::blocked(
            "2026-06-16T21-10-06-blk",
            7,
            "port-occupier",
            vec![PId("P-003".to_string())],
            SloTier::Tier20s,
        );
        db.insert(&rec).unwrap();
        let nulls: (bool, bool, bool, bool, bool) = db
            .conn
            .query_row(
                "SELECT verdict IS NULL, journal_emitted_at IS NULL, read_back_observed_at IS NULL,
                        latency_ms IS NULL, fingerprints IS NULL
                 FROM runs WHERE run_id = ?1",
                rusqlite::params!["2026-06-16T21-10-06-blk"],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .unwrap();
        assert_eq!(nulls, (true, true, true, true, true));
        // identity + slo_tier + state stay populated and the row reconstructs equal
        assert_eq!(
            db.get("2026-06-16T21-10-06-blk", "port-occupier").unwrap(),
            Some(rec)
        );
    }

    #[test]
    fn fingerprints_and_p_ids_are_json1_queryable() {
        // Proves the arrays are stored as JSON1 TEXT (not delimited) — the shape the deferred P-036
        // recurrence query indexes.
        let db = RunsDb::open_in_memory().unwrap();
        db.insert(&measured("2026-06-16T21-10-06-j1", "error-baseline-spike"))
            .unwrap();
        let lengths: (i64, i64) = db
            .conn
            .query_row(
                "SELECT json_array_length(p_ids), json_array_length(fingerprints)
                 FROM runs WHERE run_id = ?1",
                rusqlite::params!["2026-06-16T21-10-06-j1"],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(lengths, (2, 1));
    }

    #[test]
    fn seed_u64_above_i64_max_round_trips() {
        let db = RunsDb::open_in_memory().unwrap();
        let mut rec = measured("2026-06-16T21-10-06-seed", "error-baseline-spike");
        rec.seed = u64::MAX; // > i64::MAX — exercises the `as i64` / `as u64` bit-cast
        db.insert(&rec).unwrap();
        assert_eq!(
            db.get(&rec.run_id, &rec.scenario).unwrap().unwrap().seed,
            u64::MAX
        );
    }

    #[test]
    fn stored_cells_carry_no_host_paths_or_struct_names() {
        let db = RunsDb::open_in_memory().unwrap();
        db.insert(&measured("2026-06-16T21-10-06-hyg", "error-baseline-spike"))
            .unwrap();
        let blob: String = db
            .conn
            .query_row(
                "SELECT run_id || '|' || scenario || '|' || p_ids || '|' || COALESCE(verdict, '')
                        || '|' || state || '|' || COALESCE(journal_emitted_at, '') || '|' || slo_tier
                        || '|' || COALESCE(fingerprints, '')
                 FROM runs WHERE run_id = ?1",
                rusqlite::params!["2026-06-16T21-10-06-hyg"],
                |r| r.get(0),
            )
            .unwrap();
        for leak in ["C:\\", "/Users/", "/home/", "RunRecord", "RunsDb"] {
            assert!(
                !blob.contains(leak),
                "leaked {leak:?} into a stored cell: {blob}"
            );
        }
    }

    #[test]
    fn sql_metacharacters_in_a_value_are_stored_literally() {
        // Bound parameters: an injection-shaped scenario name is data, not executed SQL.
        let db = RunsDb::open_in_memory().unwrap();
        let evil = "'; DROP TABLE runs;--";
        db.insert(&measured("2026-06-16T21-10-06-evil", evil))
            .unwrap();
        // the table survives and the value round-trips verbatim
        let got = db.get("2026-06-16T21-10-06-evil", evil).unwrap().unwrap();
        assert_eq!(got.scenario, evil);
    }

    #[test]
    fn duplicate_key_insert_is_a_harness_error_not_a_clobber() {
        let db = RunsDb::open_in_memory().unwrap();
        let rec = measured("2026-06-16T21-10-06-dup", "error-baseline-spike");
        db.insert(&rec).unwrap();
        assert!(matches!(db.insert(&rec), Err(RunsDbError::Sqlite(_))));
    }

    #[test]
    fn get_absent_key_returns_none() {
        let db = RunsDb::open_in_memory().unwrap();
        assert_eq!(db.get("absent", "absent").unwrap(), None);
    }

    #[test]
    fn open_creates_runs_db_file_and_round_trips() {
        let dir = TempDir::new().unwrap();
        let db = RunsDb::open(dir.path()).unwrap();
        let rec = measured("2026-06-16T21-10-06-file", "error-baseline-spike");
        db.insert(&rec).unwrap();
        assert!(dir.path().join("runs.db").exists());
        assert_eq!(db.get(&rec.run_id, &rec.scenario).unwrap(), Some(rec));
    }
}
