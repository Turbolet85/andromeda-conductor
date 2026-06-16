# Report — 2026-06-16-emission-journal-writer

**Chunk:** Emission-journal writer — per-run JSONL emission journal (tests/obs-owned line schema, std::time journal_emitted_at stamps, run_id-stemmed never-overwritten)
**Date:** 2026-06-16T21:43:46Z
**Commits:** none yet — uncommitted working tree; this wrap commits the chunk.

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/run_record.rs` (new) · `crates/conductor-core/src/obs.rs` (+`now_rfc3339`) · `crates/conductor-core/src/lib.rs` (exports/doc) · `crates/conductor-report/src/journal.rs` (new) · `crates/conductor-report/src/lib.rs` (wire) · `crates/conductor-report/Cargo.toml` (deps) · `Cargo.lock`
- **Symbols / APIs:** new public — `conductor_core::RunRecord` (struct + `RunRecord::blocked()`), `conductor_core::now_rfc3339() -> String`, `conductor_report::JournalWriter` (`::create`, `::append`), `conductor_report::JournalError`. No IPC methods, endpoints, ports, sockets, or env vars added.
- **Crates / modules:** no crate added/removed. New modules: `conductor-core::run_record`, `conductor-report::journal`. The `conductor-report` seam (previously a bare stub) gains its first code.
- **Dependencies:** `conductor-report` gains edges to `serde_json` (1.0) + `thiserror` (2.0.18) — **both already in the workspace tree** (used by `conductor-core`), §Stack-named, audit-green; + **dev-dependency** `assert_fs` (1.x, test-plan §4-named). No new RUNTIME external crate. `Cargo.lock` updated for the dev-dep edges (assert_fs's dev-only transitive tree); audit/deny to be re-confirmed at the wrap gate.
- **Schema / config:** implements (does not change) the run-report envelope JSONL line — the 10-field tests/obs schema (obs-plan §3 / test-plan §5): `journal_emitted_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints`, in that order; the 4 measurement fields are `Option` → JSON `null` for blocked rows. **Deliberately omits `read_back_observed_at`**: the JSONL schema owner (obs-plan §3 / test-plan §5) carries only `journal_emitted_at` + the computed `latency_ms`; `read_back_observed_at` is the SLO-math intermediate / future `runs.db` column (the arch §Standard Contracts *example* shows it on the runs.db/report row, a distinct-but-overlapping surface).
- **Coverage of new surfaces:**
  - `conductor_report::JournalWriter` (journal artifact WRITE — not an external-input surface; `runs_dir` arrives pre-resolved from the cli edge) → validation n/a · instrumentation ✗ (deferred — no run-path call-site exists until verify/cli; the writer is a standalone lib fn, to be spanned under `report.generate` in Epoch 6) · PII redacted✓ (line carries only the 10 allowlisted schema fields; negative test asserts no host-path/struct-name leak; `JournalError` Display carries no host path) · tests unit✓ (4) · a11y n/a · tokens n/a
  - `conductor_core::RunRecord` (envelope type; `Deserialize` derive present but NOT wired to an external boundary this chunk) → validation n/a · instrumentation n/a · PII redacted✓ (schema fields only) · tests unit✓ (4) · a11y n/a · tokens n/a
  - `conductor_core::now_rfc3339` (`std::time` RFC-3339 `Z` stamper) → validation n/a · instrumentation n/a · PII n/a · tests unit✓ (1; shape only, no wall-clock-value assert) · a11y n/a · tokens n/a

## Deviations from intent
- **Goldens use exact `assert_eq!` instead of `insta`; dropped `insta`+`predicates` dev-deps.** Justification: the **existing codebase already uses exact-string `assert_eq!` goldens for canonical serialization** (`verdict.rs`/`report_state.rs`/`scenario.rs` `serializes_to_canonical_names`) — my approach matches that established pattern, not drift against it. For a single-line deterministic JSON contract, exact-match is a self-contained, equally-hard lock that needs no `cargo-insta` binary/accept round-trip; `insta` is reserved for the large/evolving Markdown report render (Epoch 6). Net dev-deps: just `assert_fs`.
- **No `tracing` instrumentation on the writer (dropped the optional dep).** Justification: a journal-write span isn't in obs-plan's bounded span-name set, and there's no run-path call-site yet — instrumenting a standalone lib fn is premature; it lands under `report.generate` when the writer joins a real run (Epoch 6). Consistent with the playbook deferral rule (a downstream-sequenced cross-cutting concern, no actual obs gap this chunk).
- **Live timeline call-site deferred** (recorded in the P5 `scope.md` amendment): no verdict/state producer exists until verify (Epoch 5) and `run_timeline` emits nothing, so no real `RunRecord` can be assembled at Epoch 2. The chunk ships the type + stamper + writer + tests. (The master record's laconic desc still reads "…wired into the timeline" — master is append-only/immutable except the status flip; authoritative scope is `scope.md`/`plan.md`.)
- **`read_back_observed_at` omitted from the JSONL** (see Schema) — follows the obs/tests schema owner over the arch §Standard Contracts illustrative example.
- **`now_rfc3339` test co-located in `obs.rs`** (next to the fn, mirroring `mint_run_id`) rather than `run_record.rs`. Trivial.

## Decisions & corrections
- **Schema resolution (planning):** the emission journal `runs/<run_id>.jsonl` carries the **scenario-result run-report envelope** (one record per check), NOT per-emission/per-phase events — settled from rules/observability.md + obs-plan §3 (the schema owner). The `scope.md` was amended (P5 intent-incomplete) from its original "journal entry per phase transition / emission event."
- **Crate placement (decided):** the envelope type lives in `conductor-core` (beside `Verdict`/`ReportState` it composes), not `conductor-report` — avoids a `conductor-verify → conductor-report` edge (verify *produces* the envelope in Epoch 5) and matches the shared-vocab precedent.
- **Path layering:** `CONDUCTOR_RUNS_DIR` + `resolve_under` resolution is a cli-edge concern; the writer takes an already-resolved `runs_dir`.
- User approved the plan ("yes") at the phase P5 review.

## Outcome
- **All 9 acceptance criteria met** (AC2 via exact-assert golden ≡ the intended schema lock).
- **Gates green:** `cargo nextest run -p conductor-core -p conductor-report --profile ci` → 66/66 passed (9 new) · `cargo clippy -p conductor-core -p conductor-report --all-targets -- -D warnings` → clean · `cargo test --doc …` → ok (0 doctests). Supply-chain (`cargo audit` + `cargo deny`) to run at the wrap gate (deps changed).
- **Smoke:** skipped — no boot-path change (touchpoints are libs; `agent-run.sh status` executed but needs a `<run_id>` and the writer isn't wired into a run yet).
