# Scope — Full-gate regression over the moved surfaces

Marker `2026-09-30-full-gate-regression-over-the-moved-surfaces` · conductor-0.3.0 · Epoch 5 — Polish & ship ·
working-route.md:73 · chunk base `84f3b27` (W182, operator-stated).

## The entry, as written
"Full-gate regression over the moved surfaces — scenario corpus, the `a11y` job and the live-drive path, every gate
green under both runners", plus five CARRYs (folded below, each as a hypothesis until P3 closes it).

## Operator directives at take-up (2026-09-30, the invocation's arguments, verbatim in substance)
- **evolve-diagnose waits.** It is not run in this chunk, and Epoch 4's undiagnosed state is not this chunk's work.
- **Arm K is retired** (founder ruling, standing since `2026-09-30-the-sr-pass-regrades-on-the-os-input-path`).
  No physical-keyboard arm is run, planned or simulated.
- **Nothing deferred.** Every CARRY below is owed in THIS chunk. A CARRY that cannot close here is surfaced to the
  operator, never routed forward on the agent's own word.
- **Operator slots.** Every window run (the webview / `--e2e` / `a11y:driven` legs) and every NVDA run takes the
  operator's quiet-desktop slot, and every live leg (Pulse on `:4317`) needs the operator's 4317 grant. **Before each
  one: stop and ask, and give the expected length.** The agent never starts one on its own.
- **`CONDUCTOR_NVDA` = `D:/dev/tools/nvda/nvda.exe` (NVDA 2026.2), passed per command.** It is not set in the session
  environment and is never persisted.

## What this chunk builds
1. **A full-gate regression over the surfaces Epoch 5 moved** — enumerated at P3 from Epoch 5's seven frozen
   entries (`working-route.md:58-72`):
   - the secret-scan gate;
   - `mutation-gate.py`'s `selftest`;
   - roving matrix focus, the modifier key map and the untransitioned focus ring;
   - the SR harness (the OS-input key path `send-keys.ps1`, the per-session NVDA clock calibration, T-01's second
     un-stopped run);
   - the SR content fixes (the single `h1`, the `contentinfo` strip, coverage rows named by their cells, and the
     visually-hidden titlebar count at `Titlebar.tsx:45`);
   - the scenario corpus (named by the entry) and the `a11y` CI job.
2. **"Every gate green under both runners"** — "both runners" is test-plan §4's standing pair: `cargo nextest run`
   AND the full non-doc `cargo test`, where a runner-dependent result is a determinism break (§10). Doctests are the
   separate `cargo test --workspace --doc` leg. The gate set is the project's full set:
   - fmt · clippy `-D warnings` · the `live-pulse` feature clippy on `conductor-run`;
   - nextest `--profile ci` + `cargo test --workspace` + `--doc`;
   - `cargo llvm-cov` at `--fail-under-lines 60`;
   - the standalone per-seam build sweep (`--lib` ×7, `--bins` ×2, never `--all-targets`);
   - `cargo audit` + `cargo deny`, with the advisory-db porcelain probe first;
   - rustdoc `-D warnings` for `conductor-emit` (C1) and `conductor-run`;
   - the frontend gates (`npm run build`, `typecheck:e2e`, `a11y:ownership`, `knip` after C2, `npm audit
     --omit=dev`);
   - `mutation-gate.py selftest`;
   - the scenario-corpus static gates (coverage-completeness, scenario-audit, drift / backing) and the hygiene gates
     (secret-scan, workflow env-context), which run inside the workspace suite;
   - the `a11y` job's routine arm, strict `--e2e` locally, and the CI read of the chunk's push.

   `[premise-verified at P3: test-plan §4 / §10; the widening is P3's gate enumeration]`
3. **The live-drive path** — `scripts/agent-run.{sh,ps1} run --live`: H (`halo-hue-encoding`) → B1 and B2
   (`degraded-mode-report` fired twice) → a 150 s quiet window → A (`auto-resolve-idle-window`) → the driven a11y arm
   (`live_leg_order`, `agent-run.sh:107-134`).
   - It is operator-gated (the 4317 grant), and the window legs also take the quiet-desktop slot.
   - Expected length ≈ 20 min: each leg's preflight is ≥ 45 s warm-up plus the canary; H and A are 180 s of scenario
     each; the 150 s window; the driven arm's dated sample is 4 m 2.1 s. The length is bounded by
     `live_leg_budget_sec`.
   - Leg A is not run-stable (verification-harness.md 2026-09-06 (e)), so its grade is recorded as conditional, with
     Pulse's uptime and bootstrap posture beside it, never read as a regression.

   `[premise-verified at P3]`

## CARRYs, folded (hypotheses; P3 closes each against the artifact)
- **C1 — conductor-emit rustdoc private-item links.** Public docs link five private items: `exception.rs:70:45`
  (`MAX_FRAMES`), `:155:10` (`FINGERPRINT_BYTES`), `:158:13` (`NORMALIZED_FRAMES`), `:158:58`
  (`normalize_stacktrace`), `pii.rs:102:55` (`PiiCategory::index`). So `RUSTDOCFLAGS="-D warnings" cargo doc
  --no-deps -p conductor-emit` baselines RED at exit 101 (measured at `2026-09-13-audit-debt-retired-before-epoch-1-
  closes`). Fix the five (make them `pub` or de-link to plain backticks). Then promote conductor-emit's doc gate from
  the narrowed form (`expect`: exit 0 · `lacks conductor_core` · `contains generated 5 warnings`) to the `-D warnings`
  form. From then on the 5-warning count is no longer a pin (operator disposition, 2026-09-13 wrap).
  - Coordinates re-read at take-up: lines 70, 155 and 158 of `exception.rs` and 102 of `pii.rs` carry exactly those
    links. `exception.rs` also links `NORMALIZED_FRAMES` at `:12`, `:28` and `:233`. P3 measures which of these are
    flagged; the CARRY's count of five is a hypothesis.
- **C2 — `npm run knip` exits 1 on twelve pre-existing unused exports** in `src/components/ScenarioPicker.tsx`,
  `CoverageMatrix.tsx`, `test/a11y/screen-reader/parse-nvda-log.ts` and `rows.ts` (measured at
  `2026-09-17-keyboard-and-focus-order-coverage-ownership`; basis
  `…/evidence/knip-at-HEAD-3ddd405.log`). DISPOSITION each symbol: used, removed, or excluded with its reason
  (`ignoreExportsUsedInFile`-style). Never delete blind (operator direction). Only then does the gate assert
  exit 0.
  - `[premise-corrected: npx knip at HEAD exits 1 with 15 findings, not 12 — 4 unused exports + 11 unused
    exported types. The three added since 3ddd405 are calibrateClock, InputPath and ClockCalibration. All 15 are
    used inside their own file only (0 cross-file users)]`
  - `[premise-corrected: a11y-plan §11 (lines 511-587) states no rule on unused exports or harness symbols. The
    citation originates in the rule-file leaf frontend.md:54, which the wrap cascade corrects. "Never delete
    blind" stands as operator direction]`
- **C3 — ambiguous P-ID resolution.** `conductor run <P-ID>` and the harness's `SCENARIO=<P-ID>` resolve to the FIRST
  scenario naming the P-ID, in unsorted `read_dir` order (`conductor-cli` `find_by_pid`, re-read at take-up at
  `crates/conductor-cli/src/paths.rs:112`). Seven P-IDs are named by several scenarios (P-017 · P-018 · P-019 · P-020
  · P-021 · P-022 · P-060; census at the 2026-09-22 wrap). Refuse an ambiguous P-ID the way
  `preconditions --for` already does, or pick deterministically. test-plan §3 records the ambiguity in the meantime.
  `[premise-verified at P3: re-counted over scenarios/*.toml p_ids — 37 files, 49 distinct P-IDs, exactly these 7
  ambiguous]`
- **C4 — ci.yml comment reworded.** The comment above `A11y routine arm` gives `EDGEWEBDRIVER`'s shell read the reason
  "the ${{ env.* }} expression context holds only what a workflow, job or step declared". That reason was measured
  false for a `GITHUB_ENV`-written key at CI run 36006370951. Reword it to the measured mechanism: the shell read is a
  choice, and only an image-set runner variable fails to resolve. **Coordinate drift:** the CARRY cites
  `:351-356` at `6008a68`. At HEAD the comment block is `ci.yml:387-392`, the stale clause is at `:389-392`, and the
  step is at `:398`.
- **C5 — SR row E0-10's grade.** E0-10 is graded subject-absent ("the seeded fixture records no run_envelope row"),
  yet the ENVIRONMENT-SUSPECT banner was HEARD in E0-07's window in every sr-empty session of 2026-09-30 (measured at
  `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-content-findings-fixed/evidence/nvda-pass.json`). The operator
  review graded E0-10 a FINDING. Owed: grade E0-10 against its expected content (the banner's label as text, never a
  colour) instead of its absent reason. Row located at take-up: `test/a11y/screen-reader/rows.ts:308`
  (`subject: 'empty'`, `state: 'idle-report'`, `cls: 'browse'`).
  - Hypothesis (marker kept verbatim): "`runs/e2e-fixture`, which sr-empty reads, now carries the envelope row the
    `--e2e` arm's seed persists — unmeasured; confirm from the seeded dir before choosing between re-tokening the
    row and seeding sr-empty without it".
    `[premise-verified at P3: runs/e2e-fixture/runs.db holds exactly 1 run_envelope row, ('lamps-fixture',
    'ENVIRONMENT-SUSPECT', …), and E0-07's window in the prior nvda-pass.json heard the banner. So the choice is
    re-tokening E0-10 to the banner's label text as a browse row; sr-empty keeps the seeded row]`
  - Closing C5 needs an NVDA run: operator slot, asked with its length.

## CI verdict read at Setup (5a)
- `84f3b27` (the last wrap's flip = HEAD): **CI#36774237196**. At Setup it was `in progress` (checks 3/3 open; the
  oldest running check was "A11y gate (routine arm · axe · contrast · violation JSON)" at 141 s), so it was not read
  as green then. `[premise-verified at P3: re-read green — 3/3 checks, 785 s wall, completed/success]`

## Boundaries
- In: C1–C5, the full-gate regression, the live-drive path (operator-granted), and every fix a red gate on a moved
  surface requires (the chunk is a regression, so a red on a moved surface is its work).
- Out: arm K (retired) · evolve-diagnose (waits) · the U35 door · the blocked "Interpretation re-proven…" entry
  (BLOCKED-ON Pulse) and `v3-09` · Pulse's own UI · changes to `send-keys.ps1`'s closed key set (a governed form) ·
  new dependencies. `[premise-verified at P3: no CARRY needs a package — C2 is a knip config line or an export
  keyword, C5 reuses browseUntil]`
- The eighth harness-spawn form is registered, and this chunk mints no ninth. `[premise-verified at P3: C5 drives
  E0-10 through the existing send-keys.ps1 browse path]`
