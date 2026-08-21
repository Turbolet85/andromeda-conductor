# Report — 2026-08-21-per-check-latency-measurement

**Chunk:** Per-check latency measurement — a sub-5s budget declarable beneath the closed `slo_tier` set and round-tripping config → verdict → run-report envelope, plus each expected check carrying its own `latency_ms` instead of the run's shared whole-loop value (conductor-core/verify/run/report, v2-19)
**Date:** 2026-08-21
**Commits:** none since `last_wrap` — this chunk's work is uncommitted at wrap entry (the wrap commit carries it)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-core/src/{expected,scenario,run_record,lib}.rs` · `crates/conductor-verify/src/{slo,record}.rs` · `crates/conductor-verify/tests/expected_slo.rs` · `crates/conductor-run/src/lib.rs` · `crates/conductor-report/src/{db,journal,report}.rs` · `crates/conductor-cli/src/commands/{run,suite}.rs` — 13 files, +745/−77.

- **Symbols / APIs:**
  - NEW `conductor_core::CheckRecord` (`run_record.rs:126`) — the per-check envelope record: `run_id` · `scenario` · `check_index` · `kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms`. Serde-serialized; re-exported from `conductor-core/src/lib.rs`.
  - NEW `conductor_core::MAX_BUDGET_MS` (`expected.rs:51`) — the budget ceiling, derived `SloTier::Tier90s.deadline_ms()`, not a re-pinned literal.
  - NEW field `ExpectedCheck::budget_ms: Option<u32>` (`expected.rs:76`), `#[serde(default)]` + `#[garde(range(min = 1, max = MAX_BUDGET_MS))]`.
  - NEW `ExpectedCheck::effective_deadline_ms(tier) -> i64` (`expected.rs:82`).
  - NEW `Scenario::check_budgets(&self) -> crate::Result<()>` (`scenario.rs:167`), invoked from the `from_toml_str` load path (`scenario.rs:128`).
  - NEW `conductor_run::ScenarioOutcome { record, checks }` (`lib.rs:460`).
  - NEW `RunsDb::insert_check` (`db.rs:136`) · `RunsDb::checks_for` (`db.rs:161`) · `JournalWriter::append_check` (`journal.rs:66`) · `CheckOutcome::to_check_record` (`record.rs:53`).
  - **CHANGED SIGNATURES** — `SloTier::deadline_ms` is now `const fn`; `evaluate_slo(deadline_ms: i64, …)` takes an effective deadline where it took `tier: SloTier`; `SloOutcome` gains `deadline_ms` (`slo.rs:24`); `execute_scenario` returns `ScenarioOutcome` where it returned `RunRecord`; `persist(…, checks: &[CheckRecord], …)` and `RunReport::{write,render}(…, checks, …)` each gain one parameter.
  - **Remaining-caller facts (not "sole caller"):** `evaluate_slo` keeps 1 production caller (`evaluate_check`, `slo.rs`) + 6 test call sites (`slo.rs` inline, `tests/expected_slo.rs:42`), all updated. `execute_scenario` keeps 4 call sites (`conductor-cli/src/commands/run.rs:24`, `suite.rs:31`, `conductor-run/src/lib.rs` inside `drive_run`, + inline tests). `persist` keeps 4 (cli run/suite + two in `drive_run`). `RunReport::render/write` keep 12 test call sites in `report.rs`. **`drive_run` keeps its public signature `anyhow::Result<Vec<RunRecord>>` deliberately, so `conductor-tauri` has ZERO callers touched.**
  - `evaluate_check`'s signature is UNCHANGED (still takes `tier`); it derives the effective deadline internally.

- **Crates / modules:** none added or removed. No new cross-seam dependency edge — the shipped edges (`conductor-run → {verify, report, core}`, `conductor-cli → {core, report, verify}`) carry everything.

- **Dependencies:** **none added, none bumped.** `Cargo.toml` and `Cargo.lock` are byte-untouched — zero package admission. (A generic `wire_as_value` helper in `conductor-report` was reverted for exactly this reason — see *Reverted / negative API facts*.)

- **Schema / config:**
  - **NEW `runs.db` table `run_check`** (`CREATE TABLE IF NOT EXISTS`, `db.rs:37`), grain **`(run_id, scenario, check_index)`** as its PRIMARY KEY. Columns: `run_id` TEXT NOT NULL · `scenario` TEXT NOT NULL · `check_index` INTEGER NOT NULL · `kind` TEXT NOT NULL · `verdict` TEXT NOT NULL · `state` TEXT NOT NULL · `latency_ms` INTEGER NOT NULL · `deadline_ms` INTEGER NOT NULL · `budget_ms` INTEGER (NULL when the check inherits its tier). Written with rusqlite bound parameters only. Deliberately its own table, NOT columns on `runs` — the `run_envelope` precedent at a finer grain: the qualifier is CHECK-level while `runs` is keyed `(run_id, scenario)`.
  - **`runs` is UNCHANGED** — still exactly 11 columns, still PK `(run_id, scenario)`. Asserted by a live pragma test.
  - **NEW scenario-config key `budget_ms`** on the `[[expected]]` array-of-tables — optional, integer milliseconds, garde-bounded `1..=MAX_BUDGET_MS`, and cross-field-checked against the scenario's own `slo_tier` at load. **No committed scenario TOML declares one** (the field is additive; the catalog is untouched).
  - **NEW JSONL journal record shape** — `CheckRecord` lines ride `runs/<run_id>.jsonl` beside the eleven-field envelope rows. The envelope's own shape is byte-unchanged.
  - **Load-fault path:** a `budget_ms` above its scenario's tier deadline raises **`CoreError::Config`** (a harness fault, `Result::Err`, at load — never a verdict). NOT `CoreError::Validation`: `error.rs:22` pins that variant as `#[from] garde::Report`, so it cannot carry a hand-written message. This matches its structural sibling `check_capabilities`.

- **Spec-master edits:** none (implement authors none; P2 owns any).

- **Counts / qualifiers moved:**
  - `runs.db` tables **2 → 3** (`runs`, `run_envelope`, **`run_check`**) — arch §Occupied Resources states the table set.
  - workspace nextest **693 → 719**; `conductor-core` +11, `conductor-verify` +5, `conductor-run` +1, `conductor-report` +9.
  - `runs` column count **11 → 11** (explicitly unmoved — the per-check grain went to its own table).
  - envelope field count **11 → 11** (explicitly unmoved).

- **Dev-tool versions:** none.

- **Reverted / negative API facts:**
  - A generic `wire_as_value<T: serde::de::DeserializeOwned>` helper was written in `conductor-report/src/db.rs` and **reverted** — `conductor-report` has no direct `serde` dependency, so it would not compile, and adding one would have been an unplanned dependency-manifest touch. The three enum conversions are inlined the way the shipped `RawRow::into_record` already does it.
  - **`conductor-core/src/redact.rs` was listed in the plan as a touchpoint and was deliberately NOT edited.** `CheckRecord`'s field names never become tracing span attributes — they are report-seam envelope data written to the journal / `runs.db` / Markdown report. This is exactly the **two-record-shapes** split obs-plan §3 draws (the self-obs base line vs the scenario-result record); the field-name allowlist governs only the former, so no allowlist admission was required or appropriate.

- **Spec claims disproved by measurement:**
  - **v2-19's own acceptance text** (authored at this chunk's P5, before the code was read) names two mechanisms that do not hold: *"garde-validated one altitude up on `Scenario::expected`"* — garde 0.22.1's field-level `custom` receives `(&field, &())` and cannot see the sibling `slo_tier`, so the rule structurally cannot be a garde validator; and *"`CoreError::Validation` at load"* — that variant is `#[from] garde::Report` (`error.rs:22`). Both are MECHANISM descriptors; all four numbered OUTCOME assertions hold and are test-proven. Disposition: refine the descriptors + PREMISE-CORRECTION note at the coverage gate (operator-directed, v2-15/v2-16 precedent).

- **Coverage of new surfaces:**
  - `ExpectedCheck.budget_ms` (new scenario-config external input) → validation **garde✓** (`range(min=1, max=MAX_BUDGET_MS)` + the cross-field `check_budgets` at the load path) · instrumentation **n/a** (declarative config, not an operation) · PII **n/a** (an integer bound) · tests **unit✓** (`scenario.rs` rstest valid/invalid matrix + two TOML load-path tests + the catalog guard; `expected.rs` bounds + serde-default) · a11y **n/a** · tokens **n/a**
  - `run_check` table + `RunsDb::{insert_check,checks_for}` (new persistence surface) → validation **n/a** (internal, values already typed) · instrumentation **span✓** (`db.insert_run`, `row_count=1`, the shipped span name — no new span name minted) · PII **redacted✓** (no host path or struct name reaches the row; asserted) · tests **unit✓** (round-trip, duplicate-key `Err`, zero-rows, the 11-column pragma guard) · a11y **n/a** · tokens **n/a**
  - `CheckRecord` JSONL line (new artifact record shape) → validation **n/a** · instrumentation **n/a** (report-seam data, NOT a self-obs line — see the two-record-shapes fact above) · PII **redacted✓** (9-key exact-set assertion; no `CheckRecord` struct name, no `C:\`/`/Users/`) · tests **unit✓** (`journal.rs::check_line_is_its_own_parseable_shape_beside_the_envelope`) · a11y **n/a** · tokens **n/a**
  - Markdown report per-check detail line (`report.rs`) → validation **n/a** · instrumentation **n/a** · PII **redacted✓** (rides the existing leak assertions) · tests **unit✓** (render determinism + leak suite re-run) · a11y **n/a** (cli/Markdown surface is not-assertable per a11y-plan §11 Universal) · tokens **n/a** (indented detail line, no new column, no new ANSI/palette entry — the cli results table stays exactly 6 columns)

## Deviations from intent

1. **The budget invariant is not a garde validator.** Plan step 2 specified `#[garde(custom(...))]` on `Scenario::expected`. garde 0.22.1's field-level `custom` receives `(&field, &())` and cannot read the sibling `slo_tier`, so the rule structurally cannot live there. *Justification:* shipped as `Scenario::check_budgets()` called from `from_toml_str` (`scenario.rs:128`/`:167`) — the exact shape of its structural sibling `check_capabilities`, which exists for the same reason. The plan's cited precedent `fault_phases_are_silent` works as a garde validator only because it needs its OWN field alone. The load path is guarded either way, and a dedicated test pins that `from_toml_str` actually invokes it.
2. **`CoreError::Config`, not `CoreError::Validation`.** *Justification:* `error.rs:22` pins `Validation` as `#[from] garde::Report`; it cannot carry a hand-written message. `Config` is the same harness-fault class (`Result::Err` at load, never a verdict) and is what `check_capabilities` uses.
3. **`redact.rs` was listed but not edited.** *Justification:* `CheckRecord` is report-seam envelope data, never a tracing span attribute — the obs-plan §3 two-record-shapes split. Editing the allowlist would have admitted names that never traverse it.
4. **`conductor-core/src/lib.rs` was edited though unlisted.** *Justification:* the re-export of a new `pub` type. `research.md`'s caller-threading note names "the crate's `lib.rs` re-export when a new pub item must be visible outside" as a boundary member, not a gray-area call.

## Decisions & corrections

- **Operator decision (P4 round 1):** the sub-5s budget is declared **per-check** on `[[expected]]`, not per-scenario. Basis: the corpus is observed once per scenario, so all checks share one latency; a scenario-level budget would give them one shared deadline too, leaving the per-check timing half uniform and the acceptance's second clause vacuous. Per-check budgets make the deadline the differentiator.
- **Premise corrected at P3 research:** `observe()` runs ONCE per scenario and composes ONE `Observation`; each check grades a *projection* of it (`extract.rs:53-62`). So per-check `latency_ms` VALUES are necessarily equal — a genuinely per-check *instant* would require per-check read-back calls, re-architecting the read-back path. Scope and the acceptance were both corrected to the achievable-and-stronger property before implementation began.
- **Standing correction reinforced (three occurrences this session):** silent tooling corruption is the recurring hazard — `printf` collapsed doubled backslashes into invalid JSON escapes; subagent transport HTML-escaped `<`/`>` in all 7 extracts (corrupting exactly the `<5s`/`<20s`/`<90s` tokens this chunk turns on); and a scratchpad `str.replace` pass silently NO-MATCHED twice, reporting success while leaving edits unmade. Each was caught only by an independent verification step, never by the tool reporting failure.
- **Graph-reading correction:** a loose `callee LIKE '%persist%'` impact query returned ~29 rows that were almost all `persistence_seconds` bleed from the harvest-test binaries; the tightened `'%/persist().%'` descriptor returned 0, and the code-graph holds **no symbol row at all** for `execute_scenario` or `persist`. Those caller lists are grep-derived and labelled as such in `research.md`.
- **Audit pin:** `probe unchanged, 36th consecutive` — `cargo audit` true exit 1 whose first diagnostic is `duplicate advisory ID: RUSTSEC-2026-0244`, `cargo deny check advisories bans licenses sources` true exit 0, `Cargo.lock`/`Cargo.toml` byte-untouched (zero package admission, so the auto-satisfy basis carries).

## Outcome

**Acceptance criteria: met.** All four numbered assertions of v2-19's concretized acceptance are test-proven (13 test ids in the matrix `ref`): (1) BUDGET round-trips TOML → serde → garde → `evaluate_check` → the persisted envelope, with an over-tier budget rejected at load as a harness fault; (2) PER-CHECK GRADING — every check's outcome, not only the `max_by_key`-chosen worst, persists at `(run_id, scenario, check_index)` with its own latency, effective deadline and verdict across journal + `runs.db` + Markdown report; (3) NON-VACUITY — two checks sharing one observation instant but declaring different budgets reach DIFFERENT timing verdicts; (4) NULL DISCIPLINE — declare-only and blocked rows emit zero check rows, `RunRecord` keeps its eleven fields, `slo_tier` its closed three-value set.

**Gates — all green, zero deferrals** (every gate ran; this chunk has real `.rs` delta so nothing was deferrable):
- `cargo nextest run -p conductor-core -p conductor-verify -p conductor-run -p conductor-report` → **538/538**
- `cargo nextest run --workspace --profile ci` → **719/719** (was 693), zero retries
- `cargo test -p conductor-core -p conductor-verify -p conductor-run -p conductor-report` (runner-portability) → all ok
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `cargo test --doc --workspace` → ok
- `cargo audit` → exit 1, PROBE-AUTO-SATISFY signature byte-identical · `cargo deny check advisories bans licenses sources` → exit 0 · lock un-drifted

**Smoke ✓ (mint-then-read, boot-path changed).** `SCENARIO=high-severity-log-capture bash scripts/agent-run.sh run` minted `2026-08-21T10-56-50-364` (pre-smoke newest was `2026-08-21T09-16-01-488` — genuinely fresh, not residue), then `bash scripts/agent-run.sh status 2026-08-21T10-56-50-364` read that id back. It proved end-to-end what the unit tier cannot: the `run_check` table bootstraps in a real `runs.db` (`tables: ['runs', 'run_envelope', 'run_check']`), a Blocked row writes **zero** check rows (`run_check rows: 0`, `runs rows: 1`), and the eleven-field envelope is byte-unchanged with correct nulls. No live Pulse, so the row is `[BLOCKED]` by design — the correct spine for a `method: unit` capability.
