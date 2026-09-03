# Report — 2026-09-03-conductor-tauri-survivors-dispositioned

**Chunk:** conductor-tauri survivors dispositioned — the crate's first mutation score turns into a killing
test or a cited accepted-deliberate entry per survivor
**Date:** 2026-09-03
**Commits:** none since `last_wrap` (2026-09-03T06:15:00Z) — HEAD is still `d86ed0b`; this chunk's work is
uncommitted at authoring time and rides this wrap's commit.

## Changes (structured — detectors read this)

- **Files:**
  - modified: `crates/conductor-tauri/src/commands.rs` (+200, **every line inside `#[cfg(test)] mod tests`**) ·
    `crates/conductor-tauri/src/pause.rs` (+31, test module only) ·
    `crates/conductor-tauri/src/main.rs` (+31, a NEW `#[cfg(test)] mod tests`) ·
    `crates/conductor-tauri/Cargo.toml` (dev-dependency feature)
  - new: `crates/conductor-tauri/tests/fixtures/scenarios/fixture-alpha.toml` ·
    `…/fixture-beta.toml` · `chunks/…/evidence/survivor-dispositions.md` ·
    `chunks/…/evidence/mutation-tally-after.md`
  - **Production code is byte-unchanged.** `cargo mutants` reported `Found 43 mutants` — the same 43 as the
    prior run, which is the mechanical confirmation.
- **Symbols / APIs:** **no new or changed public surface.** No new `#[tauri::command]`, no changed signature,
  no new export. 14 new private test fns + 2 private test helpers (`invoke_expecting_error`,
  `fixture_scenarios_dir`/`workspace_root`/`fixture_capabilities`), all `#[cfg(test)]`. Every existing caller
  of every touched symbol is unchanged — the touched fns kept all their callers (enumerated in research.md
  §Graph impact; `resolve_handle` keeps its 3, `runs_dir` its 3, `tauri_log_path` its 1).
- **Crates / modules:** none added or removed. One new module: `conductor-tauri::main::tests` (`#[cfg(test)]`).
- **Dependencies:** **no package added or bumped.** One dev-dependency FEATURE added:
  `tokio = { …, features = ["rt","sync","macros"] }` → `[…, "time"]`. `Cargo.lock` holds at **564 → 564
  packages** and is byte-unchanged (a feature set is not recorded in the lock). `cargo deny check advisories
  bans licenses sources` green over that lock.
- **Schema / config:** none. No migration, no config key, no violation-schema or scrub-shape change.
- **Spec-master edits:** TWO, both applied at P2 of this wrap (planned at authoring time):
  1. `.andromeda/test-plan.md` §Changelog `2026-08-20 — Mutation instrument adopted` — record the
     `conductor-tauri` accepted-deliberate TRIPLE beside the existing `declares` ×6 entry. **Authority:** the
     plan's `Expected amendments (wrap)` list.
  2. `.andromeda/design-system.md:406` (Decisions log) — retire "Tailwind v4.1 `@theme`" for the `:root`
     truth §Tokens `:201` already carries. **Authority:** this wrap's operator directive, item 3;
     orchestrator-raised, not this chunk's own drift.
- **Counts / qualifiers moved:**
  - **`conductor-tauri` standing mutation survivors: 22 → 3** (and 21→3 on the stable-count reading). Stated
    in `chunks/2026-09-02-…/evidence/mutation-tally.md` and in this chunk's `evidence/mutation-tally-after.md`.
  - **Tier tallies: 19 missed / 6 caught / 15 unviable / 3 timeout → 3 / 25 / 15 / 0.** Timeouts to zero.
  - **`conductor-tauri` test count: 12 → 26.**
  - **test-plan §10's accepted-deliberate roster gains a second instance** (`declares` ×6 was the only one;
    now plus this triple) — the motivating fact for Spec-master edit 1.
  - Workspace nextest total: 781 passing.
- **Dev-tool versions:** none — no external CLI tool installed or upgraded. `cargo-mutants` unchanged.
- **Harness / gate surface:** no agent-run script, xtask verb, CI step or status/verdict shape changed. Two
  gate INVOCATIONS were added to the chunk's own plan Test Commands (a host-path hygiene grep; the
  `agent-run.sh status` boot smoke) — chunk-local, not a change to the harness itself.
- **Cross-project / external claims:** none. No Pulse/SUT fact was read or asserted; no live leg ran.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** ONE, carried in rather than measured here —
  **`verification-matrix.json#v2-25`'s acceptance names "a `tauri::test` mock-runtime run" as the first
  parity arm.** Measured at `2026-09-02-mutation-tier-restored-for-conductor-tauri`: that arm is an
  in-process core run (`conductor_run::{preflight, drive_run}`, using no `tauri::*` item — the relocation to
  a tauri-free package compiles and passes; `test-plan-amendments.md:308-336`). The M1 wrap dispositioned the
  corrective note "carried" because no step owned it. **DISPOSED at this wrap's P7.3** as a PREMISE-CORRECTION
  `notes` narrative on v2-25: the envelope-equality half PROVEN by the in-process arm ↔ CLI subprocess pair
  (the current `ref`); the control-panel-LAUNCHED half DEFERRED to the tauri-driver leg and still OWED
  (test-plan §5). The verified outcome is not weakened and the acceptance text is not rewritten.
- **Expected amendments (from plan):** ONE listed —
  `.andromeda/test-plan.md` §Changelog `2026-08-20`: **carried.** Its motivating fact is the
  `Counts / qualifiers moved` bullet above (test-plan §10's accepted-deliberate roster gains its second
  instance) and the three dispositions enumerated in `evidence/survivor-dispositions.md`.
- **Coverage of new surfaces:** **none — this chunk introduces no external surface, hot-path op or UI
  element.** Every added line is test-side. For completeness on the one artifact a reader might take for a
  new surface: the committed scenarios fixture is read-only test data under `tests/fixtures/`, never loaded
  by a shipped binary → validation n/a (it is *consumed through* the production validator
  `Scenario::from_toml_str_with`, which is the point of the meaning-pinner) · instrumentation n/a · PII n/a
  (host-path hygiene grep run, clean) · tests ✓ (`the_committed_scenarios_fixture_stays_loadable`) ·
  a11y n/a · tokens n/a.

## Deviations from intent

1. **`start_run` was killed by a direct fn call, not through the IPC dispatch.** The plan's step-5 header
   prescribed IPC; its notes flagged that the two `Channel` arguments might make that impractical and
   authored a fallback to accepted-deliberate. *Justification:* a third route the plan did not enumerate
   works — call `start_run` as a plain fn with `Channel::new(|_| Ok(()))` and `app.state()`. The mutated
   surface is the fn body, which a direct call exercises exactly, so the kill is equivalent. **The plan's
   fallback was therefore not needed and `start_run:260` is killed rather than accepted** — a better outcome
   than the plan's own hedge.
2. **One test beyond the plan's steps.** `main.rs` also asserts `obs_sink()` yields `ObsSink::File`.
   *Justification:* `obs_sink` is not itself a survivor, but this states obs-plan §3's **unconditional**
   file-sink mandate directly, which is exactly what that acceptance criterion asks for; it strengthens the
   `tauri_log_path` disposition rather than widening scope (same file, already in the modify-set).
3. **No deviation on the disposition split.** 19 killed + 3 accepted-deliberate is what the plan specified,
   and the three accepted are exactly the three the plan named.

## Decisions & corrections

- **Operator review at P5 (four measured corrections, all verified at HEAD before applying):**
  (a) the run-data kills' MECHANISM was mis-attributed — `resolve_under` canonicalizes `base` BEFORE the
  `..` check (`config_path.rs:18-20` vs `:27-31`) and `crates/conductor-tauri/` has no `runs/`, so the error
  is `base directory is not resolvable` and the traversal guard is never reached; the kill holds but the
  claimed proof did not. (b) the hygiene grep's `[A-Za-z]:[\\/]` also matches the `s:/` in `https://` →
  `\b`-anchored. (c) two wrong coordinates (`pause.rs` :171/:224 → :172/:222) and the `Err` for
  `start_run`/`list_scenarios` arising at `capabilities()`, not the selection check; `Path::ends_with` must
  be component-wise on this Windows host. (d) `test-plan:606` already records the `declares` ×6 disposition
  in the master body, so this chunk's triple belongs beside it — the Expected-amendments list was empty and
  is now populated.
- **Operator ruling at P4:** `main:18` is dispositioned accepted-deliberate rather than chased. Measured
  basis: no `main` that launches the Tauri event loop can be called from a test, and no refactor changes
  that — an extraction moves the testable part out and leaves `main` a thinner unkillable shell.
- **Durable finding (P5 self-observation):** all seven P5 mechanical checks are STRUCTURAL (sections, path
  existence, placeholders, gate listing); none is a predicate over whether a plan step's stated MECHANISM is
  true. A plan can pass every check while claiming a proof its design cannot produce — which is what
  happened, and what the operator's review caught.
- **Durable finding (P3 research):** the three "timeout" survivors were an unbounded `rx.await`, not an
  unobserved return. A test that blocks forever reports as a timeout and masks the assertion that would
  have failed.

## Outcome

**Acceptance criteria — re-asserted against the diff, not the plan text.** All 17 met.

- (tests) Every named survivor killed or accepted-deliberate against a cited rule; no `--fail-under`, no CI
  mutation gate — **MET.** 22 dispositioned: 19 killed, 3 accepted with citations
  (`evidence/survivor-dispositions.md`). Diff introduces no threshold and no CI step.
- (tests) Verdict read from a **fresh** `mutants.out/` — **MET.** `target/mutants-2026-09-03/` verified absent
  before the run; `missed.txt` holds exactly the 3 accepted; all 19 killed present in `caught.txt`.
- (tests) Both runners green, no retries, no runner pin — **MET.** 26/26 under `cargo nextest` and
  `cargo test`; diff adds no `retries` and no `--test-threads`.
- (tests) Committed fixture's MEANING pinned by a Rust round-trip — **MET.**
  `the_committed_scenarios_fixture_stays_loadable` reads it through `list_scenarios_impl` →
  `conductor_core::list_scenarios` → `Scenario::from_toml_str_with`, asserting count AND both identities.
- (security) All six §Input-Validation survivors killed, none accepted — **MET.** All six in `caught.txt`.
- (security) Rejection asserted as REJECTED-never-clamped, error text discriminated — **MET** by
  `resolve_handle_rejects_a_traversal_default_naming_the_traversal`, which asserts the message names the
  traversal AND is not the absent-base error. The ledger records that this is the ONLY row pinning
  rejection; `run_report`/`run_envelope` are killed on the unresolvable-base mechanism and say so.
- (security) audit exit captured before any pipe; deny unconditional and green — **MET.** `cargo audit` exit
  **1** with `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; deny
  exit 0 (advisories ok, bans ok, licenses ok, sources ok). **Signature reproduced byte-identically — the
  46th re-pin**, this probe's PASS.
- (security) Dependency delta stated as PACKAGE COUNT with deny green over the new lock — **MET.** 564 → 564.
- (security) No absolute host path in fixture or ledger — **MET.** `\b`-anchored hygiene grep, no matches.
- (arch) All new code inside `crates/conductor-tauri/`; no new cross-package edge, no bin↔bin edge, no new
  `CONDUCTOR_*` handle / port / on-disk artifact — **MET.** Diff touches only that crate plus chunk artifacts.
- (arch) A plain `cargo nextest` / `cargo test` writes nothing into the repository — **MET.** The fixture is
  read-only; no test seeds a runs dir or sets an env handle.
- (obs) `tauri_log_path`'s two mutants killed by a derivation-asserting test, not by an "optional sink" rule
  — **MET.** Both in `caught.txt`; `the_self_obs_sink_resolves_beside_the_runs_dir` +
  `the_obs_sink_is_a_file_sink_whenever_the_path_resolves`.
- (obs) Handlers keep their manual `info_span!` guards, no `#[tracing::instrument]`, no new span name —
  **MET** by construction: production code byte-unchanged.
- (layouts) No seventh lamp / sixth `ReportState` / new bracket label, column or token — **MET**, same basis.
- (layouts) `run_envelope`'s `Ok(None)` treated as the CORRECT in-envelope render; banner not asserted
  unconditionally present — **MET.** The kill comes from the error arm; the ledger states the reasoning.
- (design) No new color token / ANSI entry / lamp treatment / `ReportState`; no empty-or-`None` return encoded
  as an error or `Fail` — **MET.** The error assertions key on a *supplied traversal id*, never on emptiness.
- (a11y) `conductor-run`'s over-envelope fixture and seeder stay green under the workspace suite — **MET.**
  781/781 including `envelope_fixture.rs`; this chunk touches no `conductor-run` file.

**Gates green (commands run):** `cargo nextest run -p conductor-tauri` (26/26) · `cargo test -p
conductor-tauri` (26/26) · `cargo nextest run --workspace --profile ci` (781/781, exit 0) · `cargo clippy
--workspace --all-targets -- -D warnings` (exit 0) · `cargo mutants -p conductor-tauri --test-tool=nextest
--jobs 2 --output target/mutants-2026-09-03` (43 tested; 3/25/15/0 in 3 m 55 s) · `bash scripts/agent-run.sh
status` (exit 0) · host-path hygiene grep (clean) · `cargo audit` (exit 1, signature) · `cargo deny check
advisories bans licenses sources` (exit 0). **Zero fix iterations** — every gate green on first run. No gate
deferral was available or taken (`Cargo.toml` changed, which is delta by definition).

**Smoke (boot-path changed — `main.rs` is a touchpoint):** `agent-run.sh status` exit 0 with a well-formed
envelope, and `cargo build -p conductor-tauri` exit 0. **Two honest limits:** the status verb drives the
`conductor` CLI bin, not `conductor-tauri`, so it does not prove the Tauri app boots; and the row it returned
is prior-run residue (2026-09-01), correct for a read verb but not this run's artifact. What actually covers
the touchpoint is that `main.rs`'s production bytes are unchanged and the binary compiles. No headful launch
— no UI delta, so the plan lists no self-verify.

**Process hygiene** (implement P4's census, re-measured here against the host process list by name
`conductor|tauri|mutants|msedgedriver|nvda|andromeda`):

| process | started by | final state |
|---|---|---|
| `cargo mutants` (tier re-run) | this run | terminated — exited on its own at 3 m 55 s under a 2400 s `timeout` guard; no SIGKILL, no kill-on-drop |
| `cargo` build/test/clippy invocations | this run | terminated — all foreground, all exited |
| Pulse / WebDriver / NVDA / sidecar | none started | n/a — no live leg in this chunk |

**Zero stragglers.** No listener was opened.
