# Codebase Research — 2026-06-21-run-report-envelope-serializer

## Scope
- **Depth:** deep · **Reads:** 6 files (full) · **Globs/Greps:** 4 · **code-graph:** 1 query (`tree-query-2026-06-21-run-report-envelope-serializer.json`)

## ⚠ Headline finding — the envelope already exists
The chunk's headline deliverable — *"the canonical run-report envelope shared by Markdown + runs.db
+ JSONL"* — **is already implemented** as `RunRecord` in `conductor-core`, built two epochs early by
`2026-06-16-emission-journal-writer` (it needed the schema to write a JSONL line). What exists today:
- `RunRecord` struct, 11 fields, `#[derive(Serialize, Deserialize, PartialEq, Eq, …)]`.
- The canonical **wire form is golden-locked** (`run_record.rs:99` — exact-string `assert_eq!`): field
  order, enum spellings, bare-string `p_ids`, `"<5s"` slo_tier.
- `RunRecord::blocked(...)` constructor + the **Blocked-row null rule** (5 `Option` fields → JSON `null`),
  golden-tested (`run_record.rs:104`, `journal.rs:152`).
- JSON **round-trip + determinism** tests (`run_record.rs:127`, `:138`).
- **JSONL serialization is done**: `conductor-report::JournalWriter` appends `RunRecord` to
  `runs/<run_id>.jsonl`, with hygiene tests asserting no host path / no `RunRecord` literal / exactly the
  11 keys (`journal.rs:128`).
- Timestamp helpers exist: `now_rfc3339()` (colon RFC-3339 `…Z`) + `mint_run_id()` (hyphen stem), both
  `std::time`, both tested (`obs.rs:122`, `:109`).

⇒ This is a **val-1 divergence (intent largely already satisfied)**. The genuine remaining gap is the
**producer side**, not the type/serde/JSONL. Must be resolved with the user at P4 (AskUserQuestion).

## Files inspected
- `crates/conductor-core/src/run_record.rs` (full) — the envelope. Only `::blocked()` exists; a **measured
  record is built ad-hoc via struct literal** (test-only). No `::measured(...)` canonical constructor.
- `crates/conductor-report/src/journal.rs` (full) — `JournalWriter::append(&RunRecord)` → JSONL. The sole
  cross-seam consumer of `RunRecord` today. (`conductor-report::lib.rs` exports only `JournalWriter`/`JournalError`.)
- `crates/conductor-verify/src/slo.rs` (full) — the **producer-side outcome**: `CheckOutcome { assessment,
  slo: SloOutcome{latency_ms, within_tolerance}, slo_tier }` from `evaluate_check(...)`. `Assessment` carries
  `verdict`. There is **no `CheckOutcome → RunRecord` bridge** and **no `Verdict → ReportState` mapping**.
- `crates/conductor-core/src/lib.rs` (full) — exports `RunRecord`, `Verdict`, `ReportState`, `PId`, `SloTier`,
  `mint_run_id`, `now_rfc3339`, `redact_value`. Star topology: core is the dependency root.
- `crates/conductor-core/src/obs.rs` (full) — `now_rfc3339()` / `mint_run_id()` (the envelope's instant +
  id sources).

## Graph impact (from the code-graph query)
- **`run_record/RunRecord#`** — referenced at `conductor-core/src/lib.rs:35` (re-export) and
  `conductor-report/src/journal.rs` (the JSONL consumer). Low blast radius: adding a constructor is purely
  additive (no signature change to existing fields).
- **`CheckOutcome` / `Assessment`** — **no cross-seam reference** (used only inside `conductor-verify`). A
  bridge to `RunRecord` would be a *new* edge `conductor-verify → conductor-core::RunRecord` (legal: verify
  already depends on core).

## Patterns detected
- **Twin-constructor + `Option` null rule** (`run_record.rs:45`): `::blocked()` sets the 5 measurement
  fields to `None`; a `::measured(...)` would mirror it for the happy path.
- **Exact-string golden lock** (`run_record.rs:98`, per testing-rule + obs note): conductor-report/core
  serialization uses `assert_eq!` on the literal JSON, NOT insta (insta is reserved for E2E journal goldens).
- **`std::time` instants only** (`obs.rs:122`): `now_rfc3339`/`mint_run_id` never read tokio's virtual clock.
- **Hygiene-by-construction** (`journal.rs:128`): owned-schema serde keys only; a stray field would fail the
  exactly-11-keys assertion — the artifact-hygiene guarantee is a test, not a runtime scrub.

## Conventions to follow
- **Schema owner is test-plan §3** (injected obs/test rules): the 11-field envelope + `read_back_observed_at`
  (2nd) are owned by test-plan §3; obs-plan §3 / a11y-plan §3 reproduce it and may drift — implement against
  test-plan §3 / the existing golden, never a reproduction.
- **Verdict/error wall**: envelope construction is infallible (values, no `Result::Err`).
- **Canonical-name serde**: no `#[serde(rename)]` drift from the golden; new constructors must keep the
  golden green.
- **Star topology**: `RunRecord` + any plain constructor stay in `conductor-core`; a typed `CheckOutcome`
  bridge (if in scope) lives in `conductor-verify` (core must not depend on verify).

## New files to create
- None expected (additive to `run_record.rs`; possibly a small bridge module in `conductor-verify`) —
  contingent on the P4 scope decision.

## Files to modify (contingent on P4 decision)
- `crates/conductor-core/src/run_record.rs` — add `RunRecord::measured(...)` (mirror `::blocked()`) + golden.
- `crates/conductor-verify/src/{slo.rs|verdict.rs|lib.rs}` — IF the producer bridge + `Verdict → ReportState`
  mapping are in scope (overlaps carried follow-up (c)).

## Open questions (resolve at P4 — AskUserQuestion)
1. **The envelope already exists — what is this chunk's actual deliverable?** Options: (A) re-scope to the
   genuine gap = measured constructor + `CheckOutcome → RunRecord` producer bridge + `Verdict → ReportState`
   mapping; (B) minimal ratify/harden = measured constructor + a "shared shape" contract test only; (C)
   already-satisfied → roll back the promotion and fold the remainder into Epoch-6 chunk 2 (runs.db).
2. **`Verdict → ReportState` mapping** (needed by A) is a design decision + overlaps carried follow-up (c)
   (`Decision → ReportState::ManualCheck`). In scope here, or deferred to the runs.db / Epoch-8 wiring?
3. Crate placement of any bridge: core stays the envelope owner; bridge → verify (per star topology).
