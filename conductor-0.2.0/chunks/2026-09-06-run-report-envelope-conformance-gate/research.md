# Codebase Research — 2026-09-06-run-report-envelope-conformance-gate

## Scope
- **Depth:** deep · **Reads:** 12 · **Globs/Greps:** 11 · **Graph queries:** 1 (63 rows, plane `rust`, `db_state=fresh`) · **Binary probes:** 2
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — body §5-command discipline + §Status envelope contract read in full; its 21 `## Session Additions` entries swept for `cleanup` / `run_check` / `run_envelope` / `conformance` / `jq` / `schema` / `journal row` / `host-path` → **0 relevant hits**, so the accumulated live-leg recipe does not bind. No live leg in this chunk.

## Files inspected
- `crates/conductor-report/src/journal.rs` (1-120) — `JournalWriter::append` / `append_check` both `serde_json::to_writer` the record **raw**; no redaction, no validation at the write site.
- `crates/conductor-core/src/run_record.rs` (1-146) — `RunRecord` (11 fields) and `CheckRecord` (9 fields), both `#[derive(Serialize, Deserialize)]` with **no `#[serde(default)]` and no `#[serde(deny_unknown_fields)]`** on either.
- `crates/conductor-core/src/redact.rs` (55-140) — `redact_value` / `sanitize_error` are the only `pub` items; `is_host_path_token` is **private**.
- `crates/conductor-core/src/run_journal.rs` (33-44) — `read_run_journal` parses **every** line as `RunRecord`.
- `.github/workflows/ci.yml` (108-162) — the obs conformance gate: hermetic producer step + assertion step + artifact upload.
- `scripts/agent-run.sh` (225-238) — the `cleanup` verb.
- `scripts/agent-run.ps1` (247-259) — the `cleanup` verb.
- `crates/conductor-tauri/ui/wdio.conf.ts` (71, 90-93, 271) — `seedFixtureRuns()`.
- `crates/conductor-run/tests/fixtures/lamps-journal.jsonl` — 3 lines, all 11-key envelopes.
- `crates/conductor-run/Cargo.toml` (dev-dependencies) — `serde_json` + `assert_fs` already present.
- `.andromeda/obs-plan.md` (444-454) — the "Required fields" enumeration.
- `crates/conductor-run/src/envelope.rs` (36, 40) — the only `append` / `append_check` call sites.

## Graph impact (63 rows, `tree-query-2026-09-06-run-report-envelope-conformance-gate.json`)
- **`append` / `append_check`** — sole production call sites `conductor-run/src/envelope.rs:37` and `:41` (editor lines; graph is 0-indexed), both inside `persist`. Adding a validation step has one place to sit.
- **`persist`** — callers in `conductor-cli/src/commands/{run,suite,mod}.rs`, `conductor-run/src/{drive,lib}.rs`, and tests `composition_root.rs` / `envelope_fixture.rs`. Not being re-signatured here, so no threading owed.
- **`redact_value`** — 13 rows in `redact.rs` itself, then `obs.rs` (5), `preflight.rs` (5), `canary.rs` (4), `extract.rs` (3), `verdict.rs` (3), `pause.rs` (2). **Zero rows in `conductor-report`** — corroborated by grep across all `crates/*/src`.

## Patterns detected
- **Redaction is applied at the PRODUCERS, never at the journal write site** (`conductor-report/src/journal.rs`, no `redact_value`; producers scrub first — `verdict.rs:54-55` `observed`/`expected`, `preflight.rs:164` `data_dir`, `canary.rs:100`/`:292`, `pause.rs:164` `prompt`). obs-plan §11 names "report generation when writing JSONL" as application site (b); at HEAD that site applies nothing. The journal is host-path-free **by upstream discipline across ~6 producers, with nothing asserting it**.
- **The two row shapes are cleanly discriminable and neither can parse as the other** (`run_record.rs:17-42` vs `:126-146`): `RunRecord` requires `seed` / `p_ids` / `slo_tier` / `journal_emitted_at` / `read_back_observed_at` / `fingerprints`, none of which a `CheckRecord` line carries; `CheckRecord` requires `check_index` / `kind` / `deadline_ms` / `budget_ms`, none of which an envelope carries. With no `#[serde(default)]` on either, **a typed parse IS the key-presence check** — it is both discriminator and completeness assertion in one.
- **The hermetic no-Pulse producer already exists in CI** (`ci.yml:118-124`): `ANDROMEDA_PULSE_DATA_DIR: 'pulse;injection'` is rejected before any sidecar spawn → a Blocked envelope at exit 0, and the same invocation writes `runs/<run_id>.jsonl`. The obs gate consumes only `logs/agent-latest.jsonl`; the run journal it also produces is currently unasserted.
- **Committed-fixture provenance is settled** (`crates/conductor-run/tests/fixtures/lamps-journal.jsonl` + `lamps_fixture.rs:29`, `envelope_fixture.rs:115`): a fixture is meaning-pinned by a round-trip through the production reader.

## Measurements taken here
- **Real-journal conformance (open question 4): CLEAN.** Over the 71 journals under `runs/` on this host — **74 envelope rows + 1 check row, 0 rows of any other shape, 0 unparseable**. So the gate's first run is expected GREEN on real artifacts, matching the `2026-06-27-obs-ci-conformance-gate` precedent (PASS-on-real-artifact + FAIL-on-a-removed-field). The gate is a regression guard, not a bug hunt.
- **Host-path scan: 0 hits.** `grep -lE` with the `ci.yml:143` anchor set over all 71 journals → 0 files; over `lamps-journal.jsonl` → 0.
- **The `CheckRecord` arm is thinly exercised in the wild** — exactly ONE check row exists across 71 journals (`runs/2026-08-21T18-53-35-135.jsonl` line 1, `findings-counter-refresh` / `CountAtLeast` / `ManualCheck`), because nearly every catalog scenario is declare-only or blocked.
- **obs-plan §6 field-count divergence CONFIRMED** — `:444` "Required fields (every log line)" enumerates **ten** (`journal_emitted_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`), omitting `read_back_observed_at`, while §3's and §6's schema blocks show eleven.

## A LIVE PRODUCTION DEFECT, measured not reasoned
`conductor_core::read_run_journal` (`run_journal.rs:39`) parses **every** journal line as `RunRecord`, so any run that emitted a per-check record makes the whole read fail. Probed with the prebuilt `target/debug/conductor.exe`:

| Subject | Result |
|---|---|
| `report 2026-08-21T18-53-35-135` (has a check row) | **exit 1** — `error: scenario config error: parse journal line: missing field \`seed\` at line 1 column 218` |
| `report 2026-09-06T09-11-09-325` (envelope-only) | exit 0, the 6-column table renders |

It has **two production callers** — `conductor-cli/src/commands/report.rs:17` (the `conductor report` verb) and `conductor-tauri/src/commands.rs:206` (the `run_report` Tauri command) — so both the CLI report and the webview run-report view fail outright on such a run. Latent since `2026-08-21-per-check-latency-measurement` introduced the second shape. test-plan §3's amendment (d) from that very chunk predicted it in writing ("a file-wide parse-to-envelope is a false negative on any run emitting check rows"); the correction was applied to the SPEC and the reader was never changed. Secondary: the error is mis-classified as `scenario config error` (`CoreError::Config`) for a journal parse.

**Why it belongs to this chunk:** the gate's central requirement is that the two shapes be discriminated, and the shipped reader is the one place that provably does not. A gate certifying rows that two shipped commands cannot read would be the contradiction the chunk exists to remove — and the discrimination rule is single-sourced if the reader and the gate share it.

## Conventions to follow
- Bind the host-path notion to `conductor_core::redact_value` and assert `redact_value(s) == s` per string field — `is_host_path_token` is private (`redact.rs:120`) and must not be duplicated (obs-plan §11 single-location ownership).
- Typed parse over `serde_json::Value` key-set comparison — arch §Stack (Serialization) and the existing readers' idiom; no new dependency (`serde_json` is already a `conductor-run` dev-dep).
- Gate code lands in existing crates only, minting no new cross-seam edge (arch §Module boundaries): the classifier in `conductor-core` beside `read_run_journal`, the gate test in `conductor-run/tests/`.
- `assert_fs::TempDir` + `CONDUCTOR_RUNS_DIR` for a producer that writes run artifacts (test-plan §3 Per-test isolation).
- Both harness shells stay at identical semantics (layout-templates §Surface: cli); each added `DELETE` sits behind the existing run-id gate — `valid_run_id` (`agent-run.sh:60`, applied `:230`) / `Test-RunId` (`agent-run.ps1:58`, applied `:249`).

## New files to create
- `crates/conductor-run/tests/journal_conformance.rs` — the conformance gate: over the committed fixture AND a journal produced in-test through `conductor_run::persist`, assert every line classifies as exactly one registered shape, that both shapes' key sets are complete by typed parse, that a Blocked row's five measurement fields are present-and-null (not absent), that `verdict`/`state` sit in their closed sets, and that every string field survives `redact_value` unchanged. Plus the negative arms: a row with a removed field, and a row with an injected absolute host path, each must FAIL.

## Files to modify
- `crates/conductor-core/src/run_journal.rs` — add the line classifier (a line is an envelope, a check record, or an error) and make `read_run_journal` return envelope rows while tolerating check rows; a line matching neither shape stays a hard error. Export the classifier so the gate asserts through the same rule the reader uses.
- `crates/conductor-core/src/lib.rs` — re-export the new classifier item beside `read_run_journal` (`:63`), so `conductor-run`'s test can reach it.
- `scripts/agent-run.sh` — `cleanup` (`:233`): add `DELETE FROM run_check` and `DELETE FROM run_envelope`, both behind the existing `valid_run_id` gate.
- `scripts/agent-run.ps1` — `cleanup` (`:254`): the same two deletes, behind the existing `Test-RunId` gate, at identical semantics.
- `.github/workflows/ci.yml` — a journal-conformance assertion step over the run journal the existing obs producer (`:118-124`) already writes, beside the obs gate.
- `crates/conductor-run/tests/envelope_fixture.rs` — extend the seeded subject so a `run_check` row and a `CheckRecord` journal line exist for the cleanup count-zero verification and the discrimination arm to bite. **[provisional — see open question 1]**

## Open questions
1. Whether the cleanup count-zero verification needs a seeded `run_check` row of its own, or whether asserting `count(*) = 0` on an already-empty table is verification enough → blocks: implementation-scope (decides whether `envelope_fixture.rs` is touched). A zero that was always zero proves nothing about the delete, so the honest form seeds a row first; the cheaper form does not.
2. Whether the CI step should assert over the produced journal alone or also re-run the Rust gate → blocks: plan-decision. Leaning: the Rust test is the gate (build-failing everywhere, binds to `redact_value`); the CI step adds coverage of a REAL produced artifact the fixture cannot stand in for.

## Scope premise closure
Every `[inferred]` bullet in `scope.md` was re-read against the findings above; `scope.md` has been amended accordingly.
- **VERIFIED** (tag dropped): both-row-shape coverage · build-failing enforcement (home now settled) · "not a schema change" · "not the Markdown report / `runs.db`" · "no live Pulse" · "no new `CONDUCTOR_*` handle".
- **`[premise-corrected]`**: the host-path binding. Scope said the gate mirrors `conductor-core::redact::is_host_path_token`; that function is **private** (`redact.rs:120`) and only `redact_value` / `sanitize_error` are `pub`. The predicate becomes `redact_value(s) == s`, which reuses the same single source without widening the API.
- **Scope gained a section it was missing** (intent-incomplete, not a defect): the `read_run_journal` production defect above. Scope's open question 3 asked whether the two shapes are discriminable *at all*; the answer is yes at the type level and **no at the shipped reader**, which scope had no bullet for.
- The a11y extract's question — whether the new `run_envelope` delete can strand the routine arm's seeded subject — is **answered NO**: `wdio.conf.ts:90-93` `seedFixtureRuns()` does `rmSync(target, {recursive:true, force:true})` then re-creates and re-seeds per session, so the arm cannot inherit a deleted row.
