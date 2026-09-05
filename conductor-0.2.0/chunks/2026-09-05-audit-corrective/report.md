# Report — 2026-09-05-audit-corrective

**Chunk:** Audit corrective — the boundary-#4 code audit's code-facing findings closed at their defects
**Date:** 2026-09-05
**Commits:** `7418f9d chore(route): operator-requested adaptation — 0-pending wrap` (the only commit since
`last_wrap` 2026-09-05T16:46:25Z; this chunk's own work is uncommitted at report time)

## Changes (structured — detectors read this)

- **Files:** 31 modified (+1390 / −2434), 9 new source files, 8 new evidence files. By area —
  **conductor-run** (the split): `src/lib.rs` (1944 → 36 lines) · NEW `src/canary.rs` · `src/preconditions.rs` ·
  `src/lifecycle.rs` · `src/execute.rs` · `src/envelope.rs` · `src/drive.rs` · `src/testkit.rs` ·
  NEW `tests/common/mod.rs` · NEW `tests/composition_root.rs` · `tests/{dispatch_wire,canary_wire,canary_obs_witness}.rs` ·
  `tests/severity_harvest.rs`.
  **conductor-cli**: `src/render.rs` · `src/commands/mod.rs` · `src/pause.rs` · `tests/cli_smoke.rs`.
  **conductor-core**: `src/obs.rs`.
  **conductor-report**: `src/coverage.rs` · `src/lib.rs`.
  **conductor-tauri**: `src/commands.rs`.
  **conductor-verify**: `tests/common/mod.rs` · `tests/{jsonrpc_correlation,jsonrpc_line_bound,readback,preflight,readback_shape_witness,preflight_spawn}.rs`.
  **Formatter-only, outside the modify-set** (8 files: `conductor-cli/src/commands/{coverage,preconditions,preflight,report,run}.rs`,
  `conductor-report/src/{db,journal,report}.rs`) — each verified byte-exactly `rustfmt(HEAD)` by re-formatting
  the HEAD blob and comparing (`scratchpad/fmt_reproduces.py`, 8/8 identical). These predate `rustfmt.toml`
  (added at the previous chunk), so the PostToolUse hook wrapped them when it fired on a sibling write.

- **Symbols / APIs:**
  - **NEW public** (`conductor-report`): `CoverageRollup` (struct: `total` · `in_scope` · `breakdown` ·
    `out_of_scope` · `out_label`) + `pub fn coverage_rollup(rows, unbacked) -> CoverageRollup`, re-exported from
    `conductor_report`. Its sole external caller is `conductor-cli`'s `render::coverage_summary_styled`, over the
    EXISTING `conductor-cli → conductor-report` edge (`crates/conductor-cli/Cargo.toml:14`). `summary_line` keeps
    its name, signature and byte-identical output — obs-plan §4 (`:359`) names both by name and neither moved.
  - **conductor-run public surface: UNCHANGED, 27 items.** `evidence/public-api-{before,after}.txt` (extracted from
    rustdoc `all.html` before and after) diff **identical** — nothing added, nothing removed, nothing renamed.
    Basis: `cargo doc --no-deps -p conductor-run` at both points + `diff`. Newly `pub(crate)` (crate-internal only,
    never public): `Preflight::{client, ready}` · `load_run_contract` · `observe_run_contract` · `now_ms` ·
    `now_unix_nanos`.
  - **NEW private** (`conductor-tauri`): `fn resolve_run_target(run_id) -> Result<Option<(PathBuf, String)>, String>`
    — the hoisted `run_report` / `run_envelope` prologue. **Both callers remain**; each keeps its own
    `tracing::info_span!("tauri.command.<name>").entered()` guard and its own no-run branch. The eight
    `#[tauri::command]` names are unchanged — no command added, renamed or collapsed.
  - **NEW private** (`conductor-cli`): `fn color_enabled(is_terminal, no_color_set, term) -> bool` — the pure
    three-condition tty rule. `stdout_color` / `stderr_color` remain, each still deciding for its OWN stream.
  - **NEW test-only**: `conductor-verify tests/common::bounded` · `conductor-run tests/common::{Capture, start_stub,
    start_trace_stub, Traces, Logs}` · `conductor-run src/testkit` (`#![cfg(test)]`, `pub(crate)` fixtures).
  - **Ports / sockets:** none added. The wire test stubs still bind `127.0.0.1:0` (ephemeral), never `:4317`.
  - **Env vars:** **none added** — no new `CONDUCTOR_*` handle. An env override for the tty gate was considered
    and REJECTED (plan §Constraints; architecture.md §Occupied Resources reserves the namespace).

- **Crates / modules:** no crate added or removed; the 9-member workspace is unchanged. `conductor-run` gains
  **seven** sibling modules — `canary` · `preconditions` · `lifecycle` · `execute` · `envelope` · `drive`, plus
  `testkit` (`#![cfg(test)]`). All snake_case under `crates/conductor-run/src/`, `mod`+`pub use`d from the crate
  root, mirroring the existing `dispatch.rs`. No new dependency EDGE between crates.

- **Dependencies:** **none added, none bumped.** `Cargo.lock` byte-untouched (`git status` clean on it), 564
  packages (`grep -c '^\[\[package\]\]' Cargo.lock`). No crate manifest changed (`git status --short
  crates/*/Cargo.toml` empty). `cargo deny check advisories bans licenses sources` exit 0.

- **Schema / config:** none. The eleven-field run envelope and the nine-key `CheckRecord` are untouched; the
  smoke's persisted run shows the envelope at 11 keys with `runs` 1 / `run_envelope` 1 / `run_check` 0 rows — the
  blocked-row contract (a blocked scenario writes zero check rows).

- **Spec-master edits:** none by implement (specs are read-only there). Three are OWED at this wrap and land
  through P2 — see *Expected amendments*.

- **Counts / qualifiers moved:**
  - `conductor-run/src/lib.rs` **1944 → 36** lines (`wc -l`), the crate's largest file now `execute.rs` at 774.
  - workspace nextest **853 → 873** tests (`cargo nextest run --workspace --profile ci`, both readings).
  - jscpd clone pairs **90 → 84** (jscpd 5.0.16, the audit's form `jscpd crates --format rust --reporters json
    --output <dir> --silent`; `evidence/jscpd-{before,after}-pairs.md`).
  - mutation scores: `conductor-cli` **58.59 → 97.85**, `conductor-verify` **88.33 → 88.62** (timeouts 2 → 0),
    `conductor-tauri` 89.29 → 90.32, `conductor-run` 94.44 → 94.32 (118 mutants both runs). Basis:
    `evidence/mutants/*.counts` (missed caught timeout unviable), read from each run's NESTED `mutants.out/`.
  - **No documented derived value in any of the seven masters states these** — test-plan §12's rosters state
    COORDINATES (moved; see Expected amendments 1), not counts.

- **Dev-tool versions:** none installed or upgraded. cargo-mutants 27.1.0, jscpd 5.0.16, cargo-audit and
  cargo-deny all as previously resolved.

- **Harness / gate surface:** **none changed.** `scripts/agent-run.{sh,ps1}` untouched; no CI step added; no verb
  added. The chunk's own Test Commands replaced the plan's earlier `agent-run.sh status` smoke with
  `conductor preconditions --json` (expected exit 1) at the P5 review — a plan-file change made before implement,
  not a harness change.

- **Cross-project / external claims:** one, and it is the chunk's central measurement. The RustSec **advisory-db**
  (external repo, read at `${CARGO_HOME:-$HOME/.cargo}/advisory-db`): HEAD `5a0ebedf`, `git status --porcelain`
  **empty**, and `cargo audit` **exit 0** — 1239 advisories loaded, 564 crate dependencies scanned, `warning: 18
  allowed warnings found` (17 unmaintained + 1 unsound, all `deny.toml`-adjudicated). Transcript:
  `evidence/audit-probe.md`. This is the reading the migrated PREREQ predicted and is what closes the standing
  deferral at P5.

- **Reverted / negative API facts:** an `rstest` dependency for `conductor-cli` was considered for the
  `color_enabled` truth table and NOT taken — `rstest` is not in that crate's dev-deps and adding it would move a
  `Cargo.lock` edge line for a table a plain `#[test]` expresses. The table ships as a plain test; the lockfile
  stayed byte-untouched.

- **Insufficient fixes (written, kept, not the remedy):** none. Every change landed its intended kill or dedup.

- **Spec claims disproved by measurement:** three, each with its measuring evidence.
  1. **`test-plan.md` states the supply-chain gate as `cargo audit --deny warnings`** at five sites (`:216` §Plan,
     `:456` §9 stage table, `:478` and `:508` §10 build-failure conditions, `:567` §11 stack-specific ban; basis
     `grep -n 'cargo audit' .andromeda/test-plan.md` → 5 hits). Measured 2026-09-05: that form exits **1**
     (`error: 18 denied warnings found!`) against the very advisories `deny.toml` already adjudicates, while CI
     runs the bare `cargo audit` (`.github/workflows/ci.yml:75`, no `continue-on-error`) and passed on run
     33954347685. The stated gate form is red by construction. → Expected amendment 6.
  2. **`architecture.md:196` (§Infrastructure Patterns — Build system) and `security-plan.md:177`
     (§Dependency Security)** both state `cargo audit` red on an **external advisory-DB fault**. Measured false:
     the fault was LOCAL — an untracked `crates/gettext-sys/RUSTSEC-2026-0244.md` left in this host's advisory-db
     checkout, while upstream moved that file to `crates/gettext-rs/` on 2026-08-09 (`e12b689b`), the deferral's
     first day. Post-clean the probe exits 0. → Expected amendment 3.
  3. **The audit's per-unit survivor list for `conductor-core` is a SHARD's, not the unit's.** The file-scoped
     `obs.rs` run tested **203** mutants where the audit's `--shard 1/4` reported 126, exposing 7 `obs.rs`
     survivors the audit never named (`install_panic_hook`, `ObsWriterGuard::flush`, `unix_millis` ×2, three
     `JsonVisitor::record_*` arms — `evidence/mutants/conductor-core-obs.missed.txt`). None is in
     `civil_from_unix`. Owner named in *Outcome*; no master states the contrary, so no amendment is owed.

- **Expected amendments (from plan):** six listed; each entry's motivating fact is in a Changes bullet above.
  1. **test-plan §12 rosters — CARRIED.** Sites located by `grep -n 'accepted-deliberate' .andromeda/test-plan.md`
     → §12 `:609` (conductor-run `declares`), `:610` (conductor-tauri triple), `:611` (conductor-run classes B/C),
     `:612` (conductor-verify pair); 4 hits, all in test-plan, none in any other master (`grep -rn
     'accepted-deliberate' .andromeda/*.md` → test-plan only). Owed: a NEW `conductor-cli` roster (the accepted
     PAIR), and the conductor-run + conductor-tauri coordinates re-pointed by function + column.
  2. **test-plan §4 / per-unit score records — CARRIED** (`:228` mutation instrument; the cli score and
     conductor-verify's timeouts 2 → 0).
  3. **architecture.md:196 + security-plan.md:177 — CARRIED** (fact in *Spec claims disproved* 2).
  4. **obs-plan `target`-value enumeration — NOT CARRIED, this chunk changed no such value.** `grep -n
     'conductor_run' .andromeda/obs-plan.md` → 0 hits for a literal `target` value; the field is described as the
     emitting module path, which is exactly what moved. The measured shift is recorded in evidence, not in a
     master.
  5. **obs-plan §11 bounded span names — NOT CARRIED, none changed** (measured: the post-split producer emits
     `scenario.run`, `report.generate`, `db.insert_run`, `verify.readback.connect` — every name inside the
     bounded set, topology unchanged).
  6. **test-plan `cargo audit --deny warnings` × 5 sites — CARRIED** (fact in *Spec claims disproved* 1).

- **Coverage of new surfaces:**
  - `conductor_report::coverage_rollup` (new public fn) → validation n/a (pure function of its arguments,
    no external input) · instrumentation n/a (no IO, no boundary call) · PII n/a · tests unit
    (`conductor-report` 50 tests incl. the exact-format golden `summary_line_format_is_exact`; `conductor-cli`
    `the_coverage_roll_up_is_derived_from_the_classification`) · a11y n/a (no rendered surface added — the two
    consuming renders are unchanged in output) · tokens n/a (the roll-up carries no color; the cli's out-token
    tint is unchanged and still `OUT_OF_SCOPE_MUTE`).
  - `conductor_tauri::resolve_run_target` (new private fn, IPC prologue) → validation ✓ (`resolve_under` guard
    preserved — an absolute or `..` id is REJECTED, never clamped) · instrumentation ✓ (each caller keeps its own
    `tauri.command.*` span; the helper adds none) · PII ✓ (`sanitize_error` on every `Err`; `Display` never
    `Debug`; mapped strings byte-identical) · tests integration (`conductor-tauri` 26 mock-runtime tests incl.
    `run_report_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir`) · a11y n/a · tokens n/a.
  - `conductor_cli::color_enabled` (new private fn) → validation n/a · instrumentation n/a · PII n/a · tests unit
    (`color_is_enabled_only_when_every_condition_allows_it`, six rows) · a11y n/a (cli is not-assertable,
    a11y-plan §1) · tokens ✓ (no ANSI code, lamp state or bracket label added; `each_lamp_carries_its_own_palette_code`
    pins the closed by-name set).
  - `conductor-run`'s seven modules → validation n/a · instrumentation ✓ (spans byte-identical, verified against a
    produced artifact) · PII ✓ (obs conformance gate green: no absolute host path in any of 13 lines) · tests unit
    + integration (43 relocated unit tests + `composition_root.rs`) · a11y n/a · tokens n/a.

## Deviations from intent

1. **`crates/conductor-run/src/testkit.rs` — a SEVENTH module the plan did not name.** The plan's touchpoints list
   six siblings. The test partition needs shared fixtures (`blocked_preflight`, `observation`, `fixture`,
   `named_fixture`, `occupier_fixture`, `test_envelope`, `emission`, `shell_term`, `contract_of`, `resolved_away`,
   `UNDECLARED_ENV`), and several construct crate-PRIVATE shapes — `blocked_preflight()` sets `Preflight`'s
   `client`/`ready` fields — so they cannot live in `tests/`. A copy per sibling would be exactly the duplication
   this split exists to remove. `testkit.rs` is `#![cfg(test)]`, `pub(crate)` throughout, registered
   `#[cfg(test)] mod testkit;`, and adds nothing to the public surface (the 27-item diff is identical). Its own
   doc comment states this reason.
2. **`crates/conductor-verify/tests/jsonrpc_line_bound.rs` was bounded** though plan step 2 excluded it ("terminates
   on a decode error and needs none"). That judgment is false under both mutants: its `serve_line` stub answers
   only AFTER reading a request line, so `write_message → Ok(())` leaves it silent and `!= → ==` makes the client
   skip the one reply it gets — both block in `read_line`. Measured: the FIRST verify tier re-run still reported 2
   timeouts; bounding this file's two awaits took them to 0. Same crate, same test directory, same work item (W3),
   and the acceptance ("`timeout.txt` is empty for conductor-verify") is unreachable without it.
3. **Fewer tests moved to `tests/` than the plan predicted.** The plan's rule — a test of a private item stays a
   unit test in its owning sibling; a test of `pub` items only moves to `tests/` — was applied as written, and it
   moved less than expected: `blocked_preflight()` touches `Preflight`'s crate-private fields, so the
   `execute_scenario` / `drive_run` / envelope tests cannot move to `tests/` without widening the public API the
   split exists to hold fixed. 43 relocated unit tests stayed in-crate; `tests/composition_root.rs` carries the
   public-API stability test plus the `persist`/`read_envelope` round trip.
4. **Eight files outside the modify-set changed, formatter-only.** Verified byte-exactly `rustfmt(HEAD)` for all
   eight (re-format the HEAD blob, compare — 8/8 identical). Not a scope breach: no semantic edit is possible from
   a diff the formatter reproduces exactly.
5. **The `conductor-run` mutation tier ran detached.** Measured 13 minutes, over this session's 10-minute
   foreground call cap; run with `nohup` and read by polling its log. Same command, same output dir discipline.

## Decisions & corrections

- **Operator P5 review (pre-implement, four corrections to the plan's Test Commands)** — all four applied before
  any code was written: the advisory-db probe's false-green path (`test -z` over a MISSING directory passes while
  `cargo audit` reads another cache → `git -C "$DB" rev-parse HEAD &&` added, with the `${CARGO_HOME:-$HOME/.cargo}`
  fallback stated); the 20 s test bound racing cargo-mutants' own 20 s minimum timeout (→ 5 s, with the
  mechanism-reach rule written into the step); six `cargo test -p` chained with `;` masking a mid-chain red
  (→ `&&`); and `agent-run.sh status` as the smoke, which passes on prior-run residue (→ `conductor preconditions
  --json`, expected exit 1, plus the obs producer as a second boot proof).
- **Two `civil_from_unix` mutants were REMOVED with their dead branch, not accepted.** `secs: u64` makes
  `z = days + 719_468 ≥ 719_468`, so Hinnant's negative-era arm is unreachable for every possible input. Code that
  can be deleted is never accepted-deliberate.
- **The two tty-gate wrapper mutants ARE accepted**, against test-plan §12's 2026-09-04 `sidecar_resolves_on_path`
  rule: their surviving arm reads the HOST's own state (a process whose stdout is a real terminal), every runner
  captures it, and a host-dependent kill would make the roster mean different things on different hosts. The
  rejected alternative (a `CONDUCTOR_COLOR`-class env override) is recorded as part of the disposition.
- **The evidence-hygiene gate fired on a file this run wrote** — the ledger's audit transcript had the RESOLVED
  `$CARGO_HOME` expansion pasted into it. Redacted to `<cargo-home>`; the gate re-passed. The gate caught a real
  host path in a committed artifact, which is the gate working.

## Outcome

**All 15 acceptance criteria met, re-asserted against the diff.** Each below names the measurement, not the plan's
text.

1. **Every named survivor dispositioned** ✓ — 41 cli + 6 `civil_from_unix` + 2 `jsonrpc.rs` timeouts. 39 cli
   killed, 2 accepted with a cited rule; 2 core removed with their dead branch, 4 killed; 2 timeouts killed. No
   numeric threshold introduced. `evidence/disposition-ledger.md`.
2. **Each tier graded on the nested `mutants.out/` tallies** ✓ — `missed.txt` equals EXACTLY that unit's accepted
   set in all five runs; no run reported `Found 0 mutants`; `timeout.txt` empty for conductor-verify.
   `evidence/mutants/*.{counts,missed.txt,timeout.txt}`.
3. **Both runners green, count ≥ 853** ✓ — nextest 873; `cargo test -p` green for all six touched crates (cli 51,
   core 323, verify 112, tauri 26, report 50, run 177); no `retries`, no runner pin.
4. **No host- or runner-dependent assertion added** ✓ — presence assertions through the public wrappers, escape
   absence only through assert_cmd pipes, and the one host-dependent class explicitly accepted with its rejected
   alternative recorded.
5. **`canary_obs_witness.rs` remains its own test binary** ✓ (test-plan §11 — the process-global subscriber).
6. **`paths.rs:73` and `paths.rs:130` KILLED** ✓ — both §Input Validation sites, killing tests named in the ledger.
   No §Input Validation site is accepted-deliberate.
7. **W4a redaction-neutral** ✓ — `resolve_under` guard and `sanitize_error` preserved; error strings byte-identical;
   26 tauri tests green including the traversal-rejection arms.
8. **Audit probe + deny green over an un-drifted lock** ✓ — porcelain empty, `cargo audit` exit 0 (18 allowed
   warnings), `cargo deny` exit 0, 564 packages, `Cargo.lock` untouched. Exits read from the bare commands.
9. **No absolute host path in committed evidence** ✓ — block 8 green after the one redaction described above.
10. **`conductor-run` public item set byte-identical** ✓ — 27 items, `diff` identical;
    `public_api_paths_are_stable` compiles against every path from outside the crate; `observe_preconditions`
    resolves at its cited path. No new crate, edge, `CONDUCTOR_*` handle or port.
11. **The shared roll-up rides the existing cli→report edge and carries the roll-up only** ✓ — tty styling stayed
    in `conductor-cli` (the out-token paint is still there; `conductor-report` has no color).
12. **Span tree unchanged and the obs gate green** ✓ — `scenario.run` at `execute_scenario`, `report.generate` /
    `db.insert_run` run-scoped siblings, all names in the bounded set; 13/13 lines carry the base set; no host
    path; no unstructured panic. `evidence/target-values-{before,after}.txt` record the module-path shift.
13. **Eight distinct `tauri.command.*` spans, `Display` never `Debug`** ✓.
14. **Coverage table shapes and the manifest-derived caption preserved** ✓ — 4-column coverage vs 6-column results;
    `(N unbacked)` on the auto term only; every number derived from `coverage_matrix()`.
15. **The a11y seed binaries still compile and write through the production `persist`** ✓ —
    `envelope_fixture` and `lamps_fixture` are among the 16 conductor-run test binaries, all green; no WCAG claim
    recorded for the cli kills.

**The two jscpd families, dispositioned explicitly (never "absent"):**

- **`conductor-cli/src/render.rs:587 ↔ conductor-report/src/coverage.rs:232` and `:254` (8 lines each) — OUTSIDE
  the family as scoped, not fixed.** The M2(b) family the audit named is the coverage ROLL-UP ARITHMETIC — the
  9-line pair at `render.rs:287 ↔ coverage.rs:83`, which is **gone** (one `coverage_rollup` now counts the terms
  for both surfaces). These two remaining pairs are TEST-side: each crate's own golden asserting `CoverageMode::ALL
  .iter().map(|m| rows.iter().filter(|r| r.mode == *m).count()).sum() == rows.len()` over ITS OWN rendered surface
  (`conductor-cli::coverage_summary_splits_the_in_scope_denominator` ↔
  `conductor-report::summary_counts_sum_to_the_row_count`). Deduplicating them would make one crate's test depend
  on the other's — the two-surface parity is the property under test, so a shared assertion would prove strictly
  less. They pre-date this chunk (present in `jscpd-before-pairs.md` at `render.rs:592 ↔ coverage.rs:174/:194`,
  the same code at pre-split line numbers), so they are not a NEW pair the moves created.
- **`conductor-run/tests/dispatch_wire.rs:367 ↔ :395` (9 lines) — OUTSIDE the family as scoped, not fixed.** The
  M2(c) family is the CROSS-FILE wire scaffolding (`Capture` + `start_stub`, 21 + 13 + 8 + 7 lines across three
  binaries), which is **gone** — `tests/common/mod.rs` owns it. This pair is INTRA-file: two assertion blocks
  inside one binary, present before the chunk at `:374 ↔ :399` (same code, pre-move line numbers). Not created by
  the moves. The sibling `canary_wire.rs:41 ↔ :141` (7 lines) is the same shape, likewise pre-existing at `:81 ↔
  :181`.
- Both readings are basis-checked: every remaining pair in `jscpd-after-pairs.md` has a counterpart in
  `jscpd-before-pairs.md` at the corresponding pre-move coordinates, so **zero NEW pairs were created by this
  chunk's moves** — which is what plan step 11 asks. Total 90 → 84.

**The 7 `obs.rs` survivors — out of scope, surfaced not fixed, OWNER NAMED.** They belong to **boundary #5's full
`conductor-core` mutation tier**: the founder's 30-minute per-unit budget ruling replaces the sharding that
produced the audit's partial view, so the next code-audit measures the whole unit by design and these seven arrive
inside its population rather than as a rediscovery. None is in `civil_from_unix`; this chunk's acceptance (0 missed
inside that function) is met. List: `evidence/mutants/conductor-core-obs.missed.txt`.

**Gates green** — `cargo nextest run --workspace --profile ci` (873) · `cargo test -p` × 6 · `cargo test
--workspace --doc` · `cargo clippy --workspace --all-targets -- -D warnings` (0) · five `cargo mutants` tiers ·
`jscpd` · `cargo doc --no-deps -p conductor-run` · `conductor preconditions --json` (exit 1, three subjects) ·
the obs-conformance producer + gate · the two-step audit probe + `cargo deny` · the evidence-hygiene grep.
**Smoke ✓** (boot-path changed): a fresh `CONDUCTOR_RUNS_DIR`, one run minting `2026-09-05T20-34-45-675` with all
three artifacts, then `conductor report <that id>` naming it back — mint-then-read, not residue.

**Outcome basis:** implement's P4 console report, plus this wrap's operator directive (five items: the `testkit.rs`
deviation, the explicit jscpd dispositions, the boundary-#5 owner for the 7 survivors, the friction-record
retraction, and the P5/P2 amendment routing) — all five are reflected above. Post-implement artifacts read:
`evidence/` (8 files + `mutants/`), `target/p2-final-*.log`, `evidence/audit-probe.md`.

**Process hygiene** — re-measured at report time, host process list readable:

| process | started by | final state |
|---|---|---|
| `cargo` / `rustc` (gates, tiers) | this run | terminated |
| `cargo-mutants` ×5 tiers | this run | terminated (the detached run-tier included) |
| `conductor` (smoke + obs producer) | this run | terminated |
| `jscpd` (before + after) | this run | terminated |
| `pulse-app` | not started — no live Pulse in this chunk | not running |
| `:4317` listener | none | no LISTENING socket |
