# Report — 2026-09-02-mutation-tier-restored-for-conductor-tauri

**Chunk:** Mutation tier restored for conductor-tauri — the parity test's CLI binary declared in the build graph rather than found by chance
**Date:** 2026-09-03
**Commits:** (none yet — this wrap authors the chunk commit; last commit before it: `chore(route): operator-requested adaptation — 0-pending wrap`)

## Changes (structured — detectors read this)

- **Files:**
  - new `crates/conductor-cli/tests/cross_surface_parity.rs`
  - modified `crates/conductor-tauri/src/commands.rs` · `crates/conductor-tauri/Cargo.toml` ·
    `conductor-0.2.0/verification-matrix.json` · `Cargo.lock`
  - chunk artifacts `conductor-0.2.0/chunks/2026-09-02-mutation-tier-restored-for-conductor-tauri/`
    (`scope.md` · `research.md` · `plan.md` · `report.md` · `evidence/fresh-target-dir-proof.md` ·
    `evidence/mutation-tally.md`)

- **Symbols / APIs:** **No production symbol changed.** Every diff line in `commands.rs` sits inside
  `#[cfg(test)] mod tests` (verified against the diff, not assumed).
  - RELOCATED `path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db` —
    `conductor-tauri/src/commands.rs::commands::tests` → `conductor-cli/tests/cross_surface_parity.rs`
    (its own integration-test binary).
  - RELOCATED helper `stage_repo_root` — moved with it. **Remaining-caller fact:** it had **exactly one**
    caller, the relocated test itself, measured on the rust code-graph plane (`calls` keyed on
    `callee_name`), not assumed from proximity. Zero callers remain in `conductor-tauri`.
  - CHANGED binary resolution inside that test: `assert_cmd::Command::cargo_bin("conductor")` →
    `assert_cmd::Command::new(env!("CARGO_BIN_EXE_conductor"))`. `assert_cmd` remains the assertion
    library; only the PATH RESOLUTION moved from a runtime fallback to a compile-time build-graph fact.
  - REMOVED from `conductor-tauri`'s test module: the `use conductor_core::{read_run_journal,
    HeadlessResolver, ReportState}` import (the relocated test was its last consumer; production code
    at `commands.rs:188` calls `conductor_core::read_run_journal` fully qualified and is untouched).
  - No IPC method, endpoint, export, port, socket or env var added, removed or changed. The eight
    `#[tauri::command]` handlers and their `tauri.command.*` span guards are byte-unchanged.

- **Crates / modules:** none added or removed. `conductor-cli` gains one integration-test TARGET
  (`tests/cross_surface_parity.rs`); `conductor-tauri`'s inline test module loses one test and one
  helper. Workspace membership (9 crates) unchanged. No new crate edge in either direction — in
  particular **no bin↔bin edge** was created (`conductor-tauri` keeps →`conductor-core`/→`conductor-run`
  and zero inbound; measured via `crate_edges`).

- **Dependencies:** **REMOVED** `assert_cmd` and `assert_fs` from `crates/conductor-tauri`
  `[dev-dependencies]` (their sole consumers relocated). Nothing added.
  - `Cargo.lock`: **564 → 564 packages** — no package admitted or removed; the lock moved exactly
    **2 edge lines** (`"assert_cmd",` / `"assert_fs",` dropped from `conductor-tauri`'s dependency
    list). Stated as the package COUNT, never as lockfile byte-identity.
  - `cargo deny check advisories bans licenses sources` — **exit 0** over the new lock
    (`advisories ok, bans ok, licenses ok, sources ok`).
  - Standing supply-chain probe (**the 45th**): `cargo audit` exit **1** with the signature reproduced
    **byte-identically** (`error loading advisory database: parse error: duplicate advisory ID:
    RUSTSEC-2026-0244`) → the deferral re-pins; not a deviation, not a gate failure.

- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape.

- **Spec-master edits:** none applied in P1 (the disproved claim below is P2's to disposition).

- **Counts / qualifiers moved:** **none — verified.** The chunk moves three raw tallies
  (`conductor-tauri` 13 → 12 tests · `conductor-cli` 36 → 37 · the mutation tier 0 → 43 mutants tested),
  and a grep across all seven `.andromeda/` masters and all seven `.claude/rules/` files for those
  values and for any baked mutation score returned **zero hits** — no document bakes a literal this
  chunk stales. (Workspace total is unchanged at 767: the test moved, it did not disappear.)

- **Dev-tool versions:** none. `cargo-mutants 27.1.0` was already installed and is unchanged.

- **Harness / gate surface:** two additions, both recorded in the chunk plan's `## Test Commands` and
  its evidence — **no `scripts/agent-run.{sh,ps1}` verb, no CI step, and no status/verdict shape
  changed.**
  - the **artifact-absent probe** (move `target/debug/conductor.exe` aside, require both packages
    green) — the seconds-cheap standing gate that discriminates this defect class;
  - the **mutation tier now yields a score** for `conductor-tauri` where it previously aborted
    (`cargo mutants -p conductor-tauri --test-tool=nextest --jobs 2`).

- **Cross-project / external claims:** none. No fact in this chunk has its ground truth outside this
  repo (no live Pulse leg, no SUT source read).

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none. The relocation fully resolves the
  defect it was written for (the aborting unmutated-tree baseline); the survivor disposition below is
  separate work the tier's first score REVEALED, not a remainder of this fix.

- **Spec claims disproved by measurement:**
  - **`.andromeda/test-plan.md` §5 (Cross-module patterns → Cross-surface parity, line 287)** states the
    leg as *"the Tauri mock-runtime run and the assert_cmd CLI subprocess run"*. **The first arm is not
    a mock-runtime run.** Measured: the parity test's first arm calls
    `conductor_run::{preflight, drive_run}` directly — the same composition `start_run`'s background
    thread runs — and uses **no `tauri::*` item at all**. The relocation makes this unambiguous: the
    test now lives in `conductor-cli`, a package with **no `tauri` dependency in any section**, and
    compiles and passes there. The `tauri::test` mock runtime remains real and in use — by the six
    other `#[test]` fns that stay in `conductor-tauri` — but it never drove this leg.
    Evidence: `crates/conductor-cli/tests/cross_surface_parity.rs` (the test body) ·
    `crates/conductor-cli/Cargo.toml` (no `tauri`) · the green runs in Outcome below.

- **Expected amendments (from plan):**
  - `test-plan.md` §5 Cross-module patterns → Cross-surface parity — **carried**; its motivating fact is
    the *Spec claims disproved* bullet immediately above.
  - `verification-matrix.json` `v2-25` `notes` PREMISE-CORRECTION — **carried**; same motivating fact.
    (The matrix is a ledger, not a spec master; it is written in P7, not through the amendment flow.)

- **Coverage of new surfaces:** one new file, a test target — it introduces no external surface, no
  hot-path operation and no UI element.
  - `crates/conductor-cli/tests/cross_surface_parity.rs` → validation `n/a` (no external input; the
    only env writes are the test's own `.env()` on a child and one guarded `set_var` in a
    single-test binary) · instrumentation `n/a` (a test, not an instrumented path) · PII `n/a`
    (no payload; the injection lever is a literal metacharacter string) · tests — **this IS the test**,
    green under nextest and `cargo test`, warm, artifact-absent, and cold · a11y `n/a` · tokens `n/a`.

## Deviations from intent

1. **Step 3 read as "replace the resolution, keep the library".** The step said to replace
   `assert_cmd::Command::cargo_bin("conductor")` with `env!("CARGO_BIN_EXE_conductor")`, which admits
   two readings (drop `assert_cmd` for `std::process::Command`, as the cited precedent
   `conductor-verify/tests/preflight_spawn.rs:24` does; or keep the builder and hand it the path).
   **Justification:** the step's own rationale names what RESOLVES the path as the target ("the declared
   edge — not `assert_cmd`'s `legacy_cargo_bin` fallback"), not the assertion library. Keeping
   `assert_cmd::Command::new(env!(…))` satisfies that while preserving `.assert().success()` and the
   array-form argv + child env map security-plan §Security Anti-Patterns rule (b) requires.

2. **One import removed that the plan did not enumerate.** Plan step 5 listed the dev-dependency trim
   but not the test module's `use conductor_core::{…}` line. **Justification:** the relocated test was
   its last consumer, so leaving it would have been a compile warning under `-D warnings`; the compiler
   surfaced it immediately and the removal is inside `commands.rs`, already a listed touchpoint.

3. **A denied `rm -rf` compound, replaced by granular steps.** The cold-run setup was first issued as a
   backgrounded compound containing `rm -rf` in a launch path and was denied by the permission layer —
   the exact shape `.claude/rules/host-win32.md` warns about. **Justification:** none for the mistake;
   the rule was in context and was not applied. Adjusted to granular steps with a timestamped unique
   directory and no `rm`, at a cost of two extra calls. Recorded rather than smoothed over.

## Decisions & corrections

- **Operator, at the P4 plan fork:** relocate to `conductor-cli/tests/` (recommended, taken) over the
  build-at-test-time form and over adding a lib target to `conductor-tauri`; and take BOTH halves of the
  portability acceptance — a cheap repeatable gate plus one recorded cold run — rather than either alone.
- **Operator, at the P5 review:** graph rows are 0-indexed SCIP ranges, so cite EDITOR lines (the graph's
  486/471/501 read as 487/472/502). Corrected in `plan.md` and `research.md`, every value re-confirmed by
  `grep -n` rather than blanket-shifted — the plan also carried grep-sourced citations that were already
  correct and would have been broken by a uniform +1.
- **Operator, at the P5 review:** bound the mutation run's form (`--jobs 2`, a named wall-clock ceiling)
  and, if the ceiling trips, record the partial tally + reason as evidence rather than abandon the tier.
- **Measured, this chunk:** `cargo mutants --output {dir}` writes its tallies to `{dir}/mutants.out/` —
  the tool always creates its own directory inside `--output`. A pre-existing output directory is
  therefore stale by construction: the repo-root `mutants.out/` dated 2026-08-21 read **0 missed / 38
  caught** against a run that produced **19 missed**. Only the run-summary line contradicting the file
  counts prompted the mtime check.
- **Measured, this chunk:** the tier costs **4 m 18 s** for 43 mutants at `--jobs 2` — the plan's 5400 s
  was an explicit budget, never a measurement, and over-estimated by ~20×.

## Outcome

**Acceptance re-asserted against the DIFF** (not against the plan's text):

| criterion | verdict against the diff |
|---|---|
| (tests) tier completes against a PASSING unmutated tree, **tested > 0**, verdict from the tallies | **MET** — 43 planned / **43 tested** (was 43 planned / **0** tested, exit 4). Verdict read from `mutants.out/mutants.out/`, never the exit code (which was 3 and carries no verdict). |
| (tests) parity test green under BOTH runners, artifact-absent probe green for both packages, no runner pin / `retries` / `--test-threads=1` | **MET** — `nextest -p conductor-cli` 37/37 · `cargo test -p conductor-cli` 23+13+1 · `nextest -p conductor-tauri` 12/12 · `cargo test -p conductor-tauri` 12/12 · artifact-absent probe **tauri=0, cli=0** (was exit 100, `CARGO_BIN_EXE_conductor is unset`). No runner pin or retry knob added. |
| (tests) one `CARGO_TARGET_DIR`-fresh run of both packages recorded green | **MET** — **49/49**, exit 0; verified genuinely cold (368 `Compiling` lines incl. `tauri v2.11.3`/`conductor-tauri`/`conductor-cli`, 3.0 GB from empty, `conductor.exe` produced inside that dir). `evidence/fresh-target-dir-proof.md`. |
| (tests) clippy green over the new test target; `nextest --workspace --profile ci` returns 0 | **MET** — clippy `-D warnings` exit 0 · workspace **767/767**. |
| (arch) locked workspace member, no new dependency edge, no nightly feature | **MET** — landed in `conductor-cli`; zero packages added; toolchain pin `1.95.0` untouched. |
| (arch) no bin↔bin edge; `conductor-tauri` keeps its two outbound edges and zero inbound | **MET** — measured on `crate_edges`. |
| (security) 45th probe with exit captured before any pipe; deny unconditional and green | **MET** — audit exit 1, signature byte-identical → re-pins; deny exit 0 before AND after the lock change. |
| (security) delta stated as package count, deny green over the NEW lock, lock committed | **MET** — 564 → 564, 2 edge lines, deny green over the new lock. |
| (security) no survivor in the `#[tauri::command]` boundary-guard wiring dispositioned accepted-deliberate; no committed artifact carries a host path | **MET** — **nothing was dispositioned at all** (see below), so no forbidden acceptance occurred; committed chunk artifacts grep clean for `C:\Users` / `D:\dev` / `AppData`. |
| (layouts) relocated test still asserts an EQUAL envelope, qualifier one SET never forked | **MET** — the assertion body is carried over intact: both arms into one `runs.db` under the non-default `shared-runs` handle, field-by-field comparison BETWEEN the two persisted envelopes, per-run identity/wall-clock fields excluded, `assert_ne!` on the two run-ids. |
| (obs) envelope equality asserted, never trace correlation; no span guard converted to `#[tracing::instrument]` | **MET** — no `traceparent`/OTel introduced; the eight command span guards are byte-unchanged. |
| **(tests) every NAMED survivor killed or classified accepted-deliberate against a cited rule** | **UNMET — routed to its owner.** |

**The unmet criterion, stated plainly.** The tier's FIRST score surfaced **22 standing survivors**
(19 missed + 3 timeout): `commands.rs` 14 · `pause.rs` 5 · `main.rs` 3. The wrap light gate re-ran the
tier against an identical tree and measured **21** (18 missed + 3 timeout): one mutant,
`pause.rs:85 TauriResolver::kind -> "xyzzy"`, classified missed on the first run and **unviable** on
the second, so cargo-mutants' viability classification is not fully run-stable here. The stable set is
21; treat `kind`'s two mutants as one disposition unit. **Eight (seven stable) live in `pause.rs` and
`main.rs`, outside this chunk's touchpoint list.** Six sit in the security-plan §Input Validation
domain that admits no accepted-deliberate — `resolve_handle:38` **is** the `resolve_under` traversal
guard, plus its callers `scenarios_dir:44` / `runs_dir:48` / `manifest_path:52`, and
`run_report:168` / `run_envelope:224`; `run_envelope:224 → Ok(None)` additionally breaks the
layout-templates always-rendered-qualifier invariant. Killing the in-scope 14 needs seeded fixtures
inside `conductor-tauri`'s test module (staged repo root + populated runs dir), because the path
helpers and catalog commands are unobservable without one and the run-data commands are currently
tested at exactly their EMPTY case — which is the mutants' own return value.

This criterion was authored when the survivor set was **unmeasurable by construction** (the tier had
never produced a score). **Nothing was dispositioned by default** — no quiet accepted-deliberate, no
numeric `--fail-under` — which is the correct reading of test-plan §10, whose acceptance is the
disposition itself and never a score. Routed to the entry minted at P5. Evidence:
`evidence/mutation-tally.md`.

**Gates green (commands actually run):** `cargo audit` (expected-red, signature re-pinned) ·
`cargo deny check advisories bans licenses sources` · `cargo nextest run -p conductor-cli` ·
`cargo test -p conductor-cli` · `cargo nextest run -p conductor-tauri` · `cargo test -p conductor-tauri` ·
artifact-absent probe (both packages) · `cargo clippy --workspace --all-targets -- -D warnings` ·
`cargo nextest run --workspace --profile ci` · `timeout 5400 cargo mutants -p conductor-tauri
--test-tool=nextest --jobs 2` · `CARGO_TARGET_DIR=<fresh> cargo nextest run -p conductor-tauri -p conductor-cli`.

**Smoke:** `bash scripts/agent-run.sh status` exit 0, well-formed envelope JSON — but it reported a run
dated 2026-09-01, i.e. **prior-run residue**: it proves the harness reads the artifact tree, never that
this run produced anything. The load-bearing boot proof is the parity test itself, which spawns the real
`conductor` binary as a subprocess and asserts `.success()` — green under three separately captured
conditions (warm tree · binary absent · cold empty target dir). Production code in `commands.rs` is
byte-unchanged, so the GUI's runtime behaviour was not a smoke subject this chunk.

**Process hygiene** (implement P4's census, **re-measured at this wrap** against the host process list
by name — `cargo`, `cargo-mutants`, `conductor`, `conductor-tauri`, `rustc`, `nextest`, `tauri-driver`,
`msedgedriver`, `nvda`):

| process | started by | final state |
|---|---|---|
| cargo build / nextest / clippy / `cargo-mutants` (+ their children) | this run | `terminated` |
| `conductor.exe` subprocesses (the parity test's CLI arm, ×4 conditions) | this run's tests | `terminated` |
| 2 background bash wrappers (cold run, mutation run) | this run | `terminated` (exit 0 each) |

**Zero stragglers.** No listener was opened; no Pulse sidecar, WebDriver or screen-reader process was
involved in this chunk.

**Residue for the operator (disk, not processes):**
- the cold-run target dir `%LOCALAPPDATA%/Temp/claude/D--dev-projects-conductor/<session>/scratchpad/fresh-target-0903`
  — **3.0 GB**, left in place because `rm -r` is denied in this environment; safe to delete by hand.
- the small pre-existing `%TEMP%/conductor-core-run-journal-*` class (not created by this chunk).
