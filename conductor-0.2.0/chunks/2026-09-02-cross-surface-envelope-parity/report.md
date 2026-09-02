# Report — 2026-09-02-cross-surface-envelope-parity

**Chunk:** Cross-surface envelope parity — CLI and Tauri writing an identical envelope for one scenario and
seed, the stale rmcp wording reconciled, and the desktop load-envelope banner's rendered-DOM proof taken up
from the lamps chunk (Epoch 5's last entry; v2-25 + v2-28)
**Date:** 2026-09-02
**Commits:** none yet — this wrap authors the chunk commit.

## Changes (structured — detectors read this)

- **Files:**
  - New: `crates/conductor-run/tests/fixtures/over-envelope.toml` · `crates/conductor-run/tests/envelope_fixture.rs` ·
    `conductor-0.2.0/chunks/2026-09-02-cross-surface-envelope-parity/{scope,research,plan,report}.md` ·
    `…/evidence/rmcp-classification.md`
  - Modified: `crates/conductor-tauri/src/commands.rs` · `crates/conductor-tauri/Cargo.toml` ·
    `crates/conductor-tauri/ui/wdio.conf.ts` · `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` ·
    `contracts/mcp-contract.toml` · `crates/conductor-verify/src/lib.rs` · `Cargo.lock` ·
    `conductor-0.2.0/verification-matrix.json` · `.andromeda/master-route.md` +
    `conductor-0.2.0/working-route.md` (both phase's promotion writes)

- **Symbols / APIs:**
  - **No shipped public API changed.** `conductor_run::{persist, read_envelope, classify_run}` and every
    `#[tauri::command]` keep their signatures; the caller sets are untouched (`read_envelope` @
    `conductor-tauri/src/commands.rs:243` — grep basis, the code-graph cannot see a caller inside a
    `#[tauri::command]` body; `classify_run` @ `conductor-cli/src/commands/run.rs:21`, `suite.rs:25`,
    `commands.rs:269`).
  - New test target `conductor-run::envelope_fixture` (4 tests) + private helper `seed_into`.
  - `conductor-tauri` test module: `path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db`
    (replaces the removed `path7_tauri_persists_the_same_blocked_envelope_as_the_cli`) + private helper
    `stage_repo_root`. Both are test-module items with no non-test callers.
  - **NEW ENV VAR — `CONDUCTOR_E2E_SEED_DIR`**, in the reserved `CONDUCTOR_*` namespace. Read ONLY by
    `crates/conductor-run/tests/envelope_fixture.rs`; set only by `crates/conductor-tauri/ui/wdio.conf.ts`'s
    `onPrepare`; **never read by a shipped binary**. It names the repo-relative fixture runs dir to seed, and
    the seeding test is a no-op when it is unset. **It is NOT in architecture.md §Occupied Resources'
    `CONDUCTOR_*` enumeration** (which lists ten handles) — an unregistered resource.
  - No new ports, no new sockets, no new listener. No new crate.

- **Crates / modules:** none added or removed. `conductor-run` gains a test target; `conductor-tauri` gains a
  dev-dependency.

- **Dependencies:** `assert_cmd` added to `crates/conductor-tauri/[dev-dependencies]` as
  `{ workspace = true }`. It was ALREADY a `[workspace.dependencies]` entry (root `Cargo.toml:82`) and already
  used by `conductor-cli`, so **no package enters the tree**: `Cargo.lock` package count 564 → 564; the lock
  delta is exactly one dependency-EDGE line (`"assert_cmd",` under `conductor-tauri`). `cargo deny check
  advisories bans licenses sources` verified green over the new lock. No npm dependency moved
  (`package.json` / `package-lock.json` untouched).

- **Schema / config:** none. No migration, no new config key, no violation-schema or scrub/redaction change.
  The `run_envelope` table is unchanged — this chunk WRITES a row through the existing production writer.

- **Spec-master edits:** none. Implement is read-only on the seven masters; the required edits are listed
  under *Spec claims disproved by measurement* and are this wrap's Expected amendments.

- **Counts / qualifiers moved:**
  - **a11y-plan §4 (per-component catalog, load-envelope banner row) + §6** record the banner's rendered-DOM
    axe as **UNRUNNABLE on the routine arm**, with the render-independent `--status-residual` token pair as
    its *"only asserting coverage until a rendered-DOM subject exists"*. Both qualifiers are now false: the
    subject exists, the spec runs, and axe is clean against that DOM state.
  - **test-plan §5** records *"the GUI leg of cross-surface parity"* as DEFERRED to the tauri-driver harness
    leg. The mock-runtime ↔ CLI parity leg now exists at the integration tier; the `Channel`
    frame-sequence half of that same deferral bullet remains deferred and is untouched.
  - No numeric count baked in any doc moved (checked: no doc states the workspace test count or the routine
    arm's assertion count).

- **Dev-tool versions:** `msedgedriver` **151.0.4129.101 → 152.0.4191.53** (operator-performed before this
  chunk, matching the WebView2 runtime; confirmed at research by `--version`). Not a lockfile dependency —
  a host CLI tool named by `CONDUCTOR_MSEDGEDRIVER`. The 151 binary is retained beside it.

- **Harness / gate surface:**
  - `wdio.conf.ts::seedFixtureRuns` changed from a plain `copyFileSync` of the committed journal to invoking
    `cargo test -q -p conductor-run --test envelope_fixture` with `CONDUCTOR_E2E_SEED_DIR`, and now THROWS on
    a non-zero exit or a missing journal (a silent seed failure would restore the green-over-nothing this
    subject removes). The clean re-create of the fixture dir is preserved and is what keeps the envelope
    INSERT unique-keyed.
  - The routine `--e2e` arm gained two assertions and lost one skip: the banner-label spec now ASSERTS
    (its `this.skip()` guard is gone) and a new spec runs axe against that DOM state.
  - `scripts/agent-run.{sh,ps1}`, the 5-command surface, the status/verdict shapes and the CI steps are
    UNCHANGED.

- **Cross-project / external claims:** none. No Pulse-side fact was measured this chunk — the optional
  live-Pulse pass was not run (Pulse not booted; operator-selected as non-gating).

- **Reverted / negative API facts:** the test `path7_tauri_persists_the_same_blocked_envelope_as_the_cli` was
  REMOVED rather than kept alongside the new one. Deliberate: a second test would have meant a second
  `std::env::set_var` site in one binary, which `cargo test`'s shared-process runner exposes — the
  process-global-singleton hazard `.claude/rules/testing.md` names. One site is why the runner-portability
  gate passes.

- **Spec claims disproved by measurement:**
  1. **`.andromeda/test-plan.md` — 24 rmcp-bearing lines.** Classified per-site in
     `evidence/rmcp-classification.md`: **21 AMEND** (:33 :53 :114 :143 :153 :243 :254 :264 :272 :275 :303
     :317 :416 :431 :437 :457 :494 :522 :554 :556 :596 — each describes the CURRENT client or the CURRENT
     stub), **2 LEAVE** (:88, :558 — the rmcp STDIO injection vulnerability CLASS), **1 MIXED** (:91 — leave
     the quoted anti-pattern title, amend the "negotiates DOWN" mechanism clause). rmcp was removed
     2026-06-27 and is absent from `Cargo.lock`. Evidence: `grep -c -i rmcp` = 24.
  2. **`.andromeda/obs-plan.md` §4 — 2 rmcp mentions** (the span-kinds line "outbound calls (MCP readback via
     rmcp…)" and the auto-instrumentation table's conductor-verify row "None (rmcp is a seam)"). Same defect
     class; outside v2-28's three named documents.
  3. **`.andromeda/a11y-plan.md` §4 + §6** — the banner's rendered-DOM axe recorded UNRUNNABLE and the token
     pair recorded as its only asserting coverage. Measured false: 10 specs passed on WebView2
     152.0.4191.53 including both banner specs.
  4. **`.andromeda/test-plan.md` §5** — the "GUI leg of cross-surface parity" deferral, now discharged for
     the parity half (the `Channel` frame-sequence half stands).
  5. **`.andromeda/architecture.md` §Occupied Resources — Environment variables** — the `CONDUCTOR_*`
     enumeration omits `CONDUCTOR_E2E_SEED_DIR`, a handle this chunk introduces. (An unregistered resource
     rather than a false claim, recorded here so the arch detector has the fact.)

- **Coverage of new surfaces:**
  - `CONDUCTOR_E2E_SEED_DIR` (harness-only env handle, test-read) → validation `n/a` (never reaches a shipped
    binary; the value is joined under the workspace root by a test, and the repo-relative discipline is the
    wdio caller's) · instrumentation `n/a` · PII `n/a` (no value logged; the seeder prints only a record
    count and the handle's VALUE, which is a repo-relative fixture path, never a host path) · tests ✓ (the
    seeder IS a test; its unset no-op path runs in every ordinary suite execution) · a11y `n/a` · tokens `n/a`
  - over-envelope banner DOM state (a rendered state of the SHIPPED component — this chunk adds no UI
    element) → validation `n/a` · instrumentation `n/a` · PII `n/a` · tests ✓ e2e (2 assertions on the
    routine arm) · a11y ✓ (axe `wcag2a`/`wcag2aa`/`wcag21aa` clean; the `ENVIRONMENT-SUSPECT` text label
    asserted present, so the state is not color-alone) · tokens ✓ (`--status-residual` bound by name in the
    shipped component, unchanged here)

## Deviations from intent

1. **A new `CONDUCTOR_*` env var was introduced, against the plan's own acceptance criterion** ("no new
   workspace crate, no new port, and no new `CONDUCTOR_*` env var"). The criterion's SUBJECT — the parity
   proof — holds: parity rides the existing `CONDUCTOR_RUNS_DIR` handle and added nothing. The new handle
   belongs to the SEEDER, which the criterion did not contemplate, and it follows the shape architecture.md
   already documents twice (`CONDUCTOR_MSEDGEDRIVER`, `CONDUCTOR_NVDA`: read only at the harness edge, never
   by a shipped binary, inert when unset). Reported as a deviation rather than absorbed, and it owes an arch
   §Occupied Resources registration (see *Spec claims disproved* 5).
2. **One seeded run, not two.** The plan's step 4 implied seeding the envelope subject beside the lamps
   journal. `run_report` and `run_envelope` BOTH default to `conductor_core::latest_run_id`, so a second
   seeded run would have taken the latest slot and the shipped coverage-lamp assertions would silently have
   begun reading the wrong run. Restructured to one run carrying both subjects — the lamps journal's records
   persisted under the lamps run-id WITH an `EnvironmentSuspect` standing. Lamps behaviour is unchanged.
3. **The old `path7_*` test was folded, not kept.** Plan step 7 permitted either; folding was required for
   the one-`set_var`-site reason above.
4. **`Cargo.lock` moved.** Both the plan and the operator's review edit predicted byte-unchanged. Measured:
   one edge line. The substantive basis (admits no package; 564 → 564; deny green over the new lock) holds —
   only the prediction's form was wrong. `Cargo.lock` records per-package dependency EDGES, not only the
   package set, so adding an already-locked crate to another member's dev-deps always writes a line.
5. **`CONDUCTOR_RUNS_DIR` made load-bearing in the parity test.** The first green shared the runs dir by
   CWD, but v2-25's concretized acceptance says the arms are pointed at one `runs.db` *via
   `CONDUCTOR_RUNS_DIR`*. Rather than record a mechanism gap, the shared dir is now a NON-default name
   (`shared-runs`), so an ignored handle would make the CLI write to `runs/` and the read-back find nothing.
6. **A second clause was corrected in `conductor-verify/src/lib.rs`.** Beyond the plan-named "rmcp
   client/transport foundation" claim at `:3`, the same doc comment's `:7` said the client *"negotiates the
   protocol version down"* — the rmcp-era mechanism; the hand-rolled client READS the version from the
   `initialize` result. Same defect class, corrected with it.

## Decisions & corrections

- **Operator P5 review returned two structural edits before approval, both accepted.** (a) The standing
  advisory-DB deferral had been written to run `cargo deny` only if `Cargo.lock` moved — wrong basis: deny is
  the OVERLAP probe that keeps the supply chain covered while `cargo audit` is externally red, so it runs
  unconditionally. Restored in full form: audit's exit captured BEFORE any pipe, deny unconditional, basis
  re-verified. (b) v2-28's claim path would have halted the coverage gate by construction (claimed but never
  reaching `implemented`, since its decisive artifact lands at wrap) — resolved with a by-construction `ref`
  plus the verifying grep listed as a Test Command whose producer is wrap's amendment.
- **The directive's live-Pulse premise was withdrawn by the operator** after research falsified it:
  `classify_run` runs before `preflight` and `persist` writes `insert_envelope` unconditionally on both
  shells, so the banner subject lands with or without Pulse. Operator-selected: committed subject +
  optional live pass, the live pass never a gate. It was not run (Pulse not booted).
- **A shipped test asserted less than its name.** `path7_tauri_persists_the_same_blocked_envelope_as_the_cli`
  never ran the CLI — it compared the Tauri row against hand-written literals while a comment claimed
  `cli_smoke` proved the other side. v2-25 was measurably unmet behind a green test carrying its name.
- **`cargo audit` remains externally red** on `duplicate advisory ID: RUSTSEC-2026-0244` — the standing
  deferral's 44th probe. Chain: 43 consecutive re-pins through the lamps chunk, ONE un-probed chunk
  (`2026-09-02-screen-reader-manual-spec`, which ran no supply-chain probe under its zero-delta deferral),
  then this 44th. `cargo deny` green throughout, which is the overlap doing its job.

## Outcome

**All acceptance criteria met.** Gates run and green:
`npm run build` · `cargo nextest run --workspace --profile ci` (**767 passed, 0 failed**) ·
`cargo clippy --workspace --all-targets -- -D warnings` · `cargo test --workspace --doc` ·
`cargo test -p conductor-run` + `cargo test -p conductor-tauri` (runner portability) ·
`cargo deny check advisories bans licenses sources` (exit 0) ·
`CONDUCTOR_MSEDGEDRIVER=… bash scripts/agent-run.sh run --e2e`.

`cargo audit` exit **1** — the standing advisory-DB deferral, not a chunk defect.

**Smoke / UI self-verify:** the `--e2e` leg ran as a P2 gate on a live **WebView2 152.0.4191.53** session:
**10 passing**, the only 2 skips being the pre-existing driven-arm specs (hold dialog, checklist) whose
subject needs a live Pulse. The two banner specs passed — the first rendered-DOM proof of the load-envelope
banner. Pre- and post-leg `tasklist` censuses matched exactly; no process survivors. The seeded row read
back as `lamps-fixture | ENVIRONMENT-SUSPECT | scenario "over-envelope" phase "burst" emits faster than the
proven-good sustained rate of 10000/s`.

**Matrix:** `v2-25` and `v2-28` both `implemented` with refs. v2-28's verifying grep
(`grep -n -i rmcp .andromeda/test-plan.md`) is RED BY DESIGN until this wrap's P2 applies the 21 amend + 1
mixed sites; it turns green in the light gate and the entry flips then.
