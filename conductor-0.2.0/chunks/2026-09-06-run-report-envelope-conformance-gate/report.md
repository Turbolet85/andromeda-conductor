# Report — 2026-09-06-run-report-envelope-conformance-gate

**Chunk:** Run-report envelope conformance gate — every run journal row schema-complete and host-path-free, build failing on violation; carries the cleanup-orphans code fix
**Date:** 2026-09-06T13:10Z
**Commits:** none yet (this wrap's commit is the chunk's first); HEAD at entry `68c8014` — `chore(route): operator-requested adaptation — 0-pending wrap`

## Changes (structured — detectors read this)

- **Files:** 28 tracked-modified + 2 new (`git diff --name-only HEAD | wc -l` → 28; `git ls-files --others --exclude-standard` → 20, of which 2 are source and 18 are this chunk's own pipeline artifacts).

  **SEMANTIC — source (7 modified + 2 new):**
  - `crates/conductor-core/src/run_journal.rs` — the `JournalLine` classifier; `read_run_journal` skips check records; 3 new unit tests
  - `crates/conductor-core/src/lib.rs` — re-export `JournalLine` + `classify_journal_line`
  - `crates/conductor-report/src/db.rs` — `RunsDb::delete_run` + its count-zero unit test
  - `crates/conductor-cli/src/cli.rs` — the `Cleanup { run_id }` variant
  - `crates/conductor-cli/src/commands/mod.rs` — module decl + re-export
  - `crates/conductor-cli/src/main.rs` — the dispatch arm (1 line beyond formatting)
  - `crates/conductor-run/tests/envelope_fixture.rs` — `seed_into` seeds one `CheckRecord`
  - NEW `crates/conductor-cli/src/commands/cleanup.rs` (25 lines)
  - NEW `crates/conductor-run/tests/journal_conformance.rs` (312 lines)

  **SEMANTIC — harness / CI (3):** `scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.github/workflows/ci.yml`

  **FORMATTING-ONLY (15)** — no semantic delta. **Basis (re-derived this wrap, not inherited):** for each,
  `git show HEAD:{f}` piped through `rustfmt --config-path rustfmt.toml --emit stdout` is **byte-identical**
  to the working copy (LF-normalised comparison; script `scratchpad/fmt_only.py`, 22 changed `.rs` files in →
  15 byte-identical, 7 semantic). `main.rs` is in the SEMANTIC set and differs from its HEAD-formatted form by
  **exactly 1 line** (the `Cleanup` dispatch arm). Cause: editing a crate root (`lib.rs` / `main.rs`) makes the
  rustfmt PostToolUse hook recurse the crate's whole module tree; HEAD's copies predate the committed
  edition-2024 `rustfmt.toml`.
  `crates/conductor-cli/src/paths.rs` · `crates/conductor-core/src/{capability_manifest,coverage,drift,error,expected,lamp,load_envelope,pause,phase_spec,redact,report_state,run_record,scenario,scenario_catalog}.rs`

  **Pipeline bookkeeping (3):** `.andromeda/master-route.md` (the promotion append) · `conductor-0.2.0/working-route.md` (the freeze stamp) · `.andromeda/friction-log.ndjson`

- **Symbols / APIs:**
  - NEW `pub enum conductor_core::JournalLine { Envelope(RunRecord), Check(CheckRecord) }`
  - NEW `pub fn conductor_core::classify_journal_line(&str) -> Result<JournalLine>` — the single shape-discrimination rule, shared by the reader and the gate
  - CHANGED `conductor_core::read_run_journal` — signature **unchanged** (`(&Path,&str) -> Result<Vec<RunRecord>>`); behaviour: check-record lines are skipped instead of erroring. **Remaining callers KEPT, none removed:** `conductor-cli/src/commands/report.rs:17`, `conductor-tauri/src/commands.rs:206` (both production), plus `conductor-cli/tests/cross_surface_parity.rs`, `conductor-run/tests/envelope_fixture.rs`, `conductor-run/tests/lamps_fixture.rs`
  - NEW `pub fn conductor_report::RunsDb::delete_run(&mut self, run_id: &str) -> Result<usize, RunsDbError>` — three literal `DELETE … WHERE run_id = ?1` statements in one transaction, rusqlite bound parameters, idempotent. **Sole caller:** `conductor-cli/src/commands/cleanup.rs`
  - NEW clap verb `conductor cleanup <run_id>` (`Commands::Cleanup`) — the **7th** `conductor` verb
  - No IPC method, endpoint, port, socket or env var added or changed. **No new `CONDUCTOR_*` handle.**
    `journal_conformance.rs` READS the existing `CONDUCTOR_RUNS_DIR` (test-binary reader, no-op when unset).

- **Crates / modules:** none added or removed. `conductor-cli` gains the `commands::cleanup` module; `conductor-run` gains the `journal_conformance` test target. No new cross-seam dependency edge.

- **Dependencies:** **none added, none bumped.** `Cargo.lock` untouched (`git status --short Cargo.lock` empty), 564 packages (`grep -c '^name = ' Cargo.lock`).

- **Schema / config:** **no schema change.** The eleven-field envelope and nine-key `CheckRecord` are ASSERTED, not altered; `runs.db`'s three tables and their keys are unchanged (no `CREATE`/`ALTER` touched). No config key added. `#[serde(deny_unknown_fields)]` was explicitly NOT added — obs-plan §3 keeps the per-scenario extension point open.

- **Spec-master edits:** none (wrap's amendment flow owns them; this chunk touched no `.andromeda/` master).

- **Counts / qualifiers moved:**
  - workspace test count **878 → 890** (`cargo nextest run --workspace --profile ci` summary line, both runs this session)
  - `conductor` clap verb count **6 → 7** (`conductor --help` Commands table: run · suite · report · preflight · coverage · preconditions · **cleanup**)
  - `conductor-run` test targets **+1** (`journal_conformance`)
  - **rustfmt reflow (no semantic delta): +1517 / −372 across the 15 FORMATTING-ONLY files** (`git diff --numstat HEAD -- <the 15>`); the semantic set is **+343 / −43 across 10 files** plus 337 new lines. Carried here for the line-based trend; see the FORMATTING-ONLY basis above.
  - `agent-run` verb count **unchanged at 5** (no sixth harness command).

- **Dev-tool versions:** none.

- **Harness / gate surface:**
  - `agent-run.{sh,ps1}` `cleanup` — the `sqlite3`-CLI `DELETE FROM runs` block is REPLACED by `conductor cleanup <run_id>`, behind each shell's existing run-id guard (`valid_run_id` sh:60/230 · `Test-RunId` ps1:58/249). The `rm -f` on the two named artifacts is unchanged. Teardown now covers all three tables.
  - `agent-run.{sh,ps1}` `status` — selects the newest **envelope** row (`jq -s` filtering `has("seed")` / `Where-Object { $_.PSObject.Properties.Name -contains 'seed' }`) instead of the file's last line. No output-shape change.
  - `.github/workflows/ci.yml` — NEW step "Run-journal conformance gate" beside the obs gate, running `cargo nextest run -p conductor-run --test journal_conformance --profile ci` with `CONDUCTOR_RUNS_DIR: runs` over the journal the existing obs producer already writes. Fails the build on violation with a `::error` annotation. No new workflow, no new job.
  - Gate result shape: a plain nextest target; no new bracket label, no new lamp, no new `ReportState` variant.

- **Cross-project / external claims:** none — no SUT read this chunk, no live Pulse (down since 11:05Z). The only external-tool fact is HOST-local and recorded below.

- **Reverted / negative API facts:** the plan's step-9 `tracing::info!(run_id, row_count, …)` boundary log on the new verb was written and then REMOVED. `conductor-cli` declares no `tracing` dependency and uses `tracing::` nowhere in its source (`grep -rn "tracing::" crates/conductor-cli/src` → only the line just written); honouring it meant adding a dependency edge for one log line, against this chunk's no-dependency-delta criterion. The `println!` confirmation is the operator-facing signal. **No `db.*` span was minted** (obs-plan §6's bounded span-name set is untouched).

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **The CARRY's own literal form is unsatisfiable.** It instructs "add the two **bound-parameter** deletes to both shells" while conceding the `sqlite3` CLI has no bind facility. Measured: `sqlite3` is absent from this entire host (`command -v sqlite3` empty in bash; `Get-Command sqlite3` empty in pwsh; `where.exe sqlite3` → none) and `.github/workflows/ci.yml` never installs it (`grep -n sqlite3 ci.yml` → 0 hits), while both shells guarded their DB arm on its presence — so that delete has never executed anywhere observable. Stated in `conductor-0.2.0/working-route.md:113` (the CARRY annotation). **DISPOSED:** the operator ruled at P4 that teardown moves into the binary; the CARRY is discharged in substance and its literal form needs no pin (wrap directive item 5).
  2. **`read_run_journal` parsed every line as `RunRecord`.** Measured with the prebuilt binary: `conductor report 2026-08-21T18-53-35-135` (the one local journal carrying a check row) exited **1** with ``missing field `seed` at line 1 column 218``; the envelope-only control exited 0. test-plan §3's amendment (d) from `2026-08-21-per-check-latency-measurement` predicted exactly this in writing; the spec was corrected and the code never was. **DISPOSED:** fixed this chunk (step 2), pinned by `read_run_journal_returns_envelopes_and_skips_the_check_records_beside_them`, and re-measured green (Test Command 4's report line, exit 0, table renders).
  3. **The harness `status` verb carried the same single-shape assumption** — its shipped comment read "Last line carries the final envelope". Measured on the seeded mixed-shape journal: `seed: null`, `slo_tier: null`, exit 0 — a `CheckRecord`'s fields reported as the envelope, silently. **DISPOSED:** fixed in both shells; after the fix both print `seed: 424242`, `slo_tier: "<90s"`, `scenario: lamps-fixture-fail`. The retired phrasing lived in the SHELL COMMENTS only — `grep -rn "Last line carries\|final envelope"` over the seven masters → **0 hits**, so no master states the falsified claim; the amendment owed is a POSITIVE qualification, carried as E4 below.
  4. **A typed parse does not prove key presence.** The plan asserted at P4 that "a successful parse is simultaneously the shape verdict and the key-presence check". Measured on serde 1.0.229 against a faithful `RunRecord` mirror (plain derive, no field attributes), four controls: all 11 keys → `Ok`; the 5 `Option` keys ABSENT → **`Ok`** with those fields `None`; required key `seed` absent → `Err`; extra unknown key → `Ok`. **DISPOSED:** corrected at the P5 review before implement — the gate carries a separate `serde_json::Value` key-set check and a negative arm that removes an `Option` key. No master states the falsified claim (it was plan text).

- **Expected amendments (from plan):**
  - **E1 — `obs-plan.md` §6, the "Required fields (every log line)" bullet list enumerates TEN envelope keys, omitting `read_back_observed_at`, while §3's and §6's schema blocks show ELEVEN.** CARRIED — its fact is in *Counts/qualifiers* (the gate asserts eleven by key-set) and in *Coverage of new surfaces*. Located: `grep -c "Required fields (every log line)" .andromeda/obs-plan.md` → **1** (at `:444`, list `:445-454`); the same grep over the other six masters → **0**. The heading also predates the two-record-shapes split, so "every log line" over-claims.
  - **E2 — `test-plan.md` §3, the 5-command `cleanup` body describes the deletes as issued by the harness shells.** CARRIED — its fact is in *Harness / gate surface*. Located: `grep -c "DELETE FROM run" .andromeda/test-plan.md` → **1** (at `:162`; the verification at `:164` is unchanged in substance — count-zero on all three tables — but is now implemented in `db.rs`). Same grep over the other six masters → **0**.
  - **E3 — RE-AIMED at this wrap, and the plan's target was wrong.** The plan said "architecture.md §Occupied Resources gains the `cleanup` verb". `architecture.md:33` explicitly states the verb set is "whichever `Commands` (`cli.rs`) declares, **never a literal list here**; layout-templates §Surface: cli Primary screens is the enumerating home", and §Occupied Resources enumerates *service/process names*, not verbs — `grep -n "conductor-cli" .andromeda/architecture.md` → 13 hits, **none a verb enumeration**. The true owner is **`layout-templates.md` §Surface: cli — Primary screens (commands)**, which lists the six `conductor` verbs at `:183-188` (run · suite · report · preflight · preconditions · coverage) and now under-enumerates by one. CARRIED against layout-templates; **architecture is NOT an owner for this fact** (`layout-templates.md:252` independently confirms the same de-literalisation rule).
  - **E4 — NEW at this wrap (wrap directive item 2), not in the plan.** The status contract should read "the newest **ENVELOPE** line, never the last line". Master site: `test-plan.md` §3 `status` Command body at `:158` (`jq -e '<predicate>' runs/<run_id>.jsonl` — generic, and now under-specified against a two-shape journal); `grep -n '\*\*`status`\*\*' .andromeda/test-plan.md` → **1** hit. The parallel site `.claude/rules/verification-harness.md:20` is a RULE FILE, not one of the seven masters — routed to the P2 cascade / P3 curation, never proposed as a master amendment.

- **Coverage of new surfaces:**
  - `conductor cleanup <run_id>` (new CLI verb; external input = argv) → validation **bound-param✓ / garde n/a** — the id reaches SQL only as a rusqlite `?1` parameter and is **never** used to build a path (unlike `report`, which joins `{run_id}.jsonl`), so no traversal surface is introduced; both shells additionally gate it upstream (`valid_run_id` / `Test-RunId`) · instrumentation **✗ — deliberate** (see *Reverted / negative API facts*; no `db.*` span minted, obs §6's bounded set untouched) · PII **n/a** (no value logged; the printed line carries the operator-supplied id only) · tests **unit✓** (`db::tests::delete_run_empties_all_three_tables_and_leaves_other_runs_alone`) + **e2e✓** (both shells driven, Test Commands 4/5) · a11y **n/a** (cli surface, a11y-plan §11 Universal) · tokens **✓ reuse** (`render::paint(.., ID_CYAN)`, no new colour)
  - `RunsDb::delete_run` (new storage-seam op) → validation **bound-param✓** · instrumentation **✗ — deliberate** · PII **n/a** · tests **unit✓** (count-zero on all three tables + idempotency + scoping to its own run_id) · a11y **n/a** · tokens **n/a**
  - `classify_journal_line` / `JournalLine` (new core API) → validation **typed-parse✓** · instrumentation **n/a** (pure fn) · PII **n/a** · tests **unit✓** (3 in `run_journal.rs`) + **integration✓** (8 in `journal_conformance`) · a11y **n/a** · tokens **n/a**
  - CI step "Run-journal conformance gate" (new gate surface) → validation **n/a** · instrumentation **n/a** · PII **redacted✓** — the gate's own failure output is host-path-free by construction (its origin string names the handle + filename, never a resolved path; this was a defect found and fixed during the fix-loop) · tests **e2e✓ — proven RED** (pointed at a journal missing the optional key `latency_ms`, exit 100 naming `keys absent from the row: ["latency_ms"]`) · a11y **n/a** · tokens **n/a**

## Deviations from intent

1. **`crates/conductor-cli/src/main.rs` was edited though it appears in no touchpoint list.** Plan step 9 and the Files-to-modify list both placed the verb dispatch in `commands/mod.rs`; that file only declares and re-exports handler modules — the `Commands` match arm is `main.rs:70-78`. P3 read `commands/mod.rs` and `cli.rs` but never the dispatch site. Justification: the mandatory completion of the plan's own "registration + **dispatch** the new variant" intent; the chunk does not compile without it. One line beyond formatting.

2. **The harness `status` verb was fixed in both shells — beyond step 10's scope.** A SECOND single-shape reader defect of the same class as `read_run_journal`, in a third reader nobody had looked at. Evidence: Test Command 6's printed line over the mixed `lamps-fixture` journal — before, `"seed": null` / `"slo_tier": null`; after, `"seed": 424242` / `"slo_tier": "<90s"` / `"scenario": "lamps-fixture-fail"`, identical on both shells. **Pre-existing** since per-check records landed on 2026-08-21; this chunk's fixture change (step 4) merely made it reproducible. Justification: both shells were already in Files-to-modify (fix-loop Trigger 3's "a listed file needing more edits than research predicted — in scope"), and shipping a harness that misreports envelopes immediately after this chunk certifies journal conformance would be the exact contradiction the chunk exists to remove.

3. **The plan's step-9 `tracing::info!` was dropped.** Justification: `conductor-cli` declares no `tracing` dependency and uses none; adding one for a single log line would breach the chunk's own no-dependency-delta acceptance criterion (`Cargo.lock` unchanged at 564 packages). Routed to the obs detector to decide whether obs-plan §6's boundary-log form needs a CLI-side note (wrap directive item 4).

4. **The gate's own failure origin leaked an absolute host path** (`path.display()` in the externally-pointed arm) — caught by READING the negative proof's output rather than its exit code, and fixed to handle + filename before the final gates. The gate is subject to the rule it enforces.

5. **The plan's P4 claim that a typed parse proves key presence was measured false at the P5 review** and re-synthesised in place (three arms → the `Value` key-set check + an `Option`-key negative arm). Recorded because the plan text a detector reads was corrected mid-flight, not merely elaborated.

## Decisions & corrections

- **Operator fork at P4 — teardown moves INTO the binary.** Presented with three options after `sqlite3` measured absent host-wide and CI-wide; the alternatives (the shells' `sqlite3` deletes as the CARRY literally words them; deferring the CARRY to its own entry) were declined. Rationale carried into the plan: it is the only reading under which the CARRY's own "bound-parameter" wording and test-plan §3:162's `?1` placeholders are both satisfiable, and it removes an undeclared host dependency.
- **Operator correction at P5 (two findings).** (a) Presence needs its own producer — serde accepts an absent `Option` field as `None`; verified independently before applying. (b) The CI step must RUN the Rust gate, not re-list the schema in `jq` — a third copy would drift from the structs and obs-plan §3/§6.
- **Standing preference reinforced:** no `format!`-assembled SQL, not even over a hard-coded table list — `delete_run` uses three literal statements rather than a loop over table names.
- **Wrap-directive correction (this wrap):** the plan's E3 named architecture as the owner of the verb enumeration; measurement shows architecture explicitly refuses that role and layout-templates holds it. Second mis-aimed citation caught in this session by grepping the target before proposing.

## Outcome

**Acceptance criteria, each re-asserted against the DIFF:**

| # | Criterion | Verdict |
|---|---|---|
| 1 | Every envelope line carries all 11 keys **by presence** (key-set check, not typed parse) | **MET** — `journal_conformance` step 5(b); arity pinned by `the_two_registered_shapes_carry_their_contract_arity` |
| 2 | Every `CheckRecord` line judged against its own 9 keys | **MET** — same gate; `budget_ms: null` accepted |
| 3 | Two shapes discriminated by typed parse; envelope-only journal passes | **MET** — `classify_journal_line`; the smoke's real Blocked journal (envelope-only) passed |
| 4 | No journal row carries an absolute host path, sourced from `conductor-core::redact` | **MET** — `redact_value(s) == s` per string field; `is_host_path_token` is private, so the predicate binds the pub fn (no second definition minted) |
| 5 | A violation is RED — all three negative arms fail the gate | **MET** — required-key, **`Option`-key**, and host-path arms all ship and pass; externally-pointed arm measured exit **100** |
| 6 | Governed `verdict`/`state` co-occurrences accepted; Blocked never a failure | **MET** — `CalibrationRegion`+`ManualCheck` row in the produced subject; Blocked row asserted conformant |
| 7 | `conductor report` renders a both-shapes journal; the exit-1 no longer reproduces | **MET** — Test Command 4, exit 0, table renders |
| 8 | `delete_run` leaves `count(*)=0` on all three tables; second call a no-op | **MET** — unit test + measured live: `(3,1,1)` → `(0,0,0)`, second call `0 rows` |
| 9 | Every teardown statement binds `?1`; no `format!`-built SQL; shells issue no SQL | **MET** — three literal statements; both shells now call the binary |
| 10 | Both shells at identical semantics, verb count still 5, run-id guards intact | **MET** — measured by RUNNING both: 5 rows removed → `(0,0,0)` on each |
| 11 | New verb output reuses the shipped render style; no prompt on the headless path | **MET** — `render::paint(.., ID_CYAN)`; no new colour/label; no prompt |
| 12 | The CI step asserts by RUNNING the Rust gate; key lists in exactly one place | **MET in mechanism** — ci.yml invokes the nextest target; **the GitHub run itself is unexecuted here** (no CI push this wrap). Locally proven by the identical invocation (Test Command 6) and by the smoke's real produced journal. |
| 13 | `nextest --workspace` 0 and `clippy --workspace --all-targets -D warnings` 0 | **MET** — 890/890, clippy exit 0, doctests exit 0 |
| 14 | No dependency delta — `Cargo.lock` package count unchanged | **MET** — lockfile untouched, 564 packages |

**Gates green (commands run):** `cargo nextest run -p conductor-core -p conductor-report -p conductor-run -p conductor-cli --profile ci` (618/618) · `cargo nextest run --workspace --profile ci` (890/890) · `cargo test --workspace --doc` (0 tests, exit 0) · `cargo clippy --workspace --all-targets -- -D warnings` (exit 0) · the CARRY probe on **both** shells · the externally-pointed gate arm (8/8) · a deliberate negative run (exit 100).

**Smoke:** fired (boot-path — the cli entry point gained a verb; the plan's Test Commands drive `agent-run.sh`). A hermetic no-Pulse run exited 0 printing `[BLOCKED] error-baseline-spike`, wrote journal + report + `runs.db`; the conformance gate then passed **over that freshly produced real journal** via `CONDUCTOR_RUNS_DIR` — the CI step's exact mechanism, end-to-end, no Pulse. `status` → truthful envelope; `cleanup` → 2 rows removed, all three tables at 0, artifacts gone.

**Outcome basis:** implement's P4 report as given, PLUS this wrap's own re-derivations — the FORMATTING-ONLY basis (re-measured here, not inherited from the directive), the Expected-amendment site greps (which re-aimed E3), and the operator's wrap directive (which added E4 and the Test Command 4 sequencing fix). No claim below rests on implement's report alone where this wrap measured it again.

**Process hygiene:** re-measured at this wrap — `Get-Process` filtered on `pulse|conductor|cargo|rustc|msedgedriver|tauri|nvda|node` → **empty**; `Get-NetTCPConnection -State Listen -LocalPort 4317,4444,4445` → **empty**. Every process this chunk started (`cargo`/`rustc`/`conductor.exe`/`pwsh`) `terminated`. `pulse-app` `not started` — down since 11:05Z by the operator, and no live leg exists on this path. Gitignored residue left: `runs/cleanup-probe/` (empty `runs.db`, no journal — the overseer's probe residue), `runs/smoke-probe/` (empty `runs.db`).
