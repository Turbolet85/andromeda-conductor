# Codebase Research — 2026-09-30-full-gate-regression-over-the-moved-surfaces

## Scope
- **Depth:** deep on the five CARRY sites, moderate on the gate set and the live-drive path · **Reads:** 19 · **Globs/Greps:** 31
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, read whole (auto-loaded into this window at
  P3, `## Session Additions` included: the live-leg recipe, `:19` `run` / `--live`, the SR firing form, the census
  rule, the printed-verdict and `Spec Files:` TAB atom, the admission-probe rule and the slot-gated re-fire rule).
  Also `a11y.md` (6 additions), `testing.md` (48 additions), `frontend.md` (12 additions) and `observability.md`,
  all auto-loaded.
- **Platform issues consulted:** none. No runner-only bullet was folded, and the one CI verdict read (below) is
  green.

## CI verdict read at Setup, re-read at P3
- `84f3b27` (the last wrap's flip = HEAD): **CI#36774237196 — green**, 3/3 checks, 785 s wall. At Setup it read
  `in progress` (the oldest check, "A11y gate (routine arm · axe · contrast · violation JSON)", was 141 s in). Re-read
  twice here with `ci.py conclusion --sha 84f3b27f69c07bbb8ffcc73f19a7f373c45b4723` (trail in the run dir). The
  configuration is the standing `windows-2022` `a11y` job (`ci.yml:289`).

## Files inspected
- `crates/conductor-emit/src/exception.rs` (`:12`, `:28`, `:70`, `:155`, `:158`, `:233`) — C1's link sites. Only
  `:70:45`, `:155:10`, `:158:13` and `:158:58` are flagged. The `NORMALIZED_FRAMES` links at `:12` (module doc),
  `:28` and `:233` are not: `:28`/`:233` document private items and `:12` is inner module doc.
- `crates/conductor-emit/src/pii.rs:102` — C1's fifth site (`PiiCategory::index`).
- `crates/conductor-cli/src/paths.rs` (full) — `load_scenario` (`:51-62`) tries `{target}.toml` by name, then
  `find_by_pid` (`:112-133`), which returns the FIRST `read_dir` hit and skips unloadable files silently.
- `crates/conductor-cli/src/cli.rs` (full) — `scenario_name` (`:74-85`) is the in-tree refusal precedent: a
  `P-NNN`-shaped `--for` value is a clap usage error (exit 2) whose text names the reason.
- `crates/conductor-cli/src/main.rs:20-90` — the `anyhow` edge: `render::error_block(sanitize_error(..),
  hint_for(..))` → `ExitCode::FAILURE`. `hint_for` (`:59-74`) matches stable context markers; `"no scenario
  matches"` → `"list scenarios in scenarios/, or pass a P-ID like P-009"`.
- `crates/conductor-cli/tests/cli_smoke.rs` (`:55-178`, `:596-605`) — the harness `conductor(&dir)` (TempDir cwd,
  copied manifests, `ANDROMEDA_PULSE_DATA_DIR=pulse;injection`), `run_resolves_a_scenario_by_p_id` (`P-009`,
  single-owner), the sanitized-error test's shape (`error:`/`hint:`/no host path/no ESC) and
  `preconditions_for_refuses_a_p_id_at_argument_parsing` (`.code(2)`).
- `scripts/agent-run.sh:100-135, :295-339` — `live_leg_order` (H → B1 → B2 → `sleep 150` → A, then the driven arm),
  `live_leg_budget_sec` (`preflight_budget_sec + n + 60`), the stage flags (`--unit` · `--integration` · `--e2e` ·
  `--live`), and `SCENARIO=` passing `run "$SCENARIO"` straight to the CLI (`:326-330`; `.ps1:376-383`).
- `contracts/pulse-run-contract.toml:32,36` — `warmup_ms = 45000`, `min_canary_poll_seconds = 90`: the per-leg
  preflight floor for the live suite's length.
- `.github/workflows/ci.yml` (step index + `:380-419`) — C4's comment block (`:387-392`) above `A11y routine arm`
  (`:398`); the step's `EDGEWEBDRIVER` shell read (`:404-414`); there is no `cargo doc` / `RUSTDOCFLAGS` in any
  job, and none in `agent-run.sh`.
- `crates/conductor-tauri/ui/knip.json` · `package.json` scripts — knip config (`entry` only, three
  `ignoreDependencies`); `npm run knip` = `knip`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts:290-312` — E0-07 (browse, tokens `3 scenarios` /
  `lamps-fixture`) and E0-10 (`absent:` set, tokens `ENVIRONMENT-SUSPECT`).
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts:830-876` — the empty-subject walk: E0-07 is `h` until
  "Run report" then ArrowDown until "lamps-fixture", from Start; E0-10 is `stamp('E0-10', 'none (subject absent)')`
  at `:875`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts:415, :545-553` — an `absent` row grades
  `subject-absent` with no action and is excluded from the stimulus/clock arms.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md:181, :241` — E0-10's spec row and the
  routed-finding note.
- `conductor-0.3.0/chunks/2026-09-30-the-screen-reader-content-findings-fixed/evidence/nvda-pass.json` — E0-07
  `heard`: "Run report heading level 2", then "Run report region ENVIRONMENT-SUSPECTscenario "over-envelope" phase
  "burst" emits faster than…", then "3 scenarios · run lamps-fixture". E0-10: `subject-absent`, `heard: []`,
  `review_grade: finding`.
- `runs/e2e-fixture/runs.db` (read-only open) — tables `runs` · `run_envelope` · `run_check`; `run_envelope` holds
  **1** row: `('lamps-fixture', 'ENVIRONMENT-SUSPECT', 'scenario "over-envelope" phase "burst" emits faster than the
  proven-good sustained rate of 10000/s …')`.
- `crates/conductor-tauri/ui/src/components/Titlebar.tsx:45` — the "visually-hidden count" moved surface: the
  titlebar count is named by visually-hidden TEXT, not an `aria-label`.
- `.claude/rules/frontend.md:54` · `.andromeda/a11y-plan.md` §11 (`:511-587`) — the C2 citation checked (below).

## Graph impact
- **`find_by_pid`** — 1 caller: `Paths::load_scenario` (`paths.rs:59`). **`load_scenario`** — 2 callers:
  `commands/run.rs:19` (`run`) and `commands/preconditions.rs:30` (`preconditions`, reached only with a scenario
  NAME, since `--for` refuses a P-ID at parse). **`scenario_name`** — used by the `--for` arg (`cli.rs:62`).
  Rust plane, `calls` on `callee_name IN ('find_by_pid','load_scenario','scenario_name')`, 4 rows (trace:
  `tree-query-{marker}.json`). The change stays inside `conductor-cli`, with no signature crossing a crate edge.
- **conductor-emit rustdoc items** — doc-only. The derivation's pins
  (`exception.rs::relative_paths_are_significant_at_every_depth`,
  `::token_leading_absolute_paths_normalize_to_the_same_empty_form`) sit in the same file, and a de-link touches
  no code line.
- **C2 symbols** — ts plane not queried: knip's own report IS the graph question here. Every flagged symbol was
  grepped for cross-file use (`grep -rlw <name> src test`, `.ts`/`.tsx`): **0 cross-file users for all 15**, each
  used inside its own file (2–7 in-file hits).

## Patterns detected
- **Refuse-an-ambiguous-P-ID** (`cli.rs:74-85`): a P-ID-shaped value is refused before any load, and the reason is
  named in the message. The clap path gives exit 2. The `run` path is an `anyhow` harness fault, so a refusal there
  exits via `main.rs:40` `ExitCode::FAILURE` (1) with an `error:` + `hint:` block.
- **Stable-marker hint mapping** (`main.rs:59-74`): a new fault text gets its own `hint_for` arm keyed on a stable
  substring, and `cli_smoke` asserts the specific hint, never the generic `--debug` one
  (`run_with_unknown_target_is_a_sanitized_error_with_a_hint`).
- **Browse rows walked forward from a known start** (`screen-reader.e2e.ts:851-856`): E0-07's walk is the only
  route to the report header, and the banner sits between "Run report" and the header text in NVDA's reading
  order.

## Conventions to follow
- **CLI-edge test shape** (`cli_smoke.rs:140-178`): `conductor(&dir)` with the scenarios copied in, `.failure()`,
  and assertions on `error:`, the specific `hint:`, no TempDir path, and no `\u{1b}`. `CONDUCTOR_RUNS_DIR` is never
  set (test-plan §3 Per-test isolation). `copy_scenario` copies a committed TOML by stem (`cli_smoke.rs:13`).
- **Rustdoc de-link idiom**: plain backticks for a private item (the form `:28`/`:233` already use).
- **ci.yml comment discipline**: comment-only edits leave the step's `run`, `env`, `if:` and `continue-on-error`
  byte-identical (arch §Established Decisions [CI/CD]; security-plan §Dependency Security, third class).

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-full-gate-regression-over-the-moved-surfaces/evidence/nvda-pass.json` — the
  sr-empty regrade record with E0-10 graded against its content (copied from `runs/sr-leg/nvda-pass.json` after the
  slot).
- `conductor-0.3.0/chunks/2026-09-30-full-gate-regression-over-the-moved-surfaces/evidence/` — gate and leg
  captures (the live suite's per-leg journals, the regression log).

## Files to modify
- `crates/conductor-emit/src/exception.rs` — C1: de-link the four flagged intra-doc links (or widen to `pub`; P4).
- `crates/conductor-emit/src/pii.rs` — C1: de-link `PiiCategory::index` at `:102`.
- `crates/conductor-cli/src/paths.rs` — C3: resolve a P-ID over ALL matching scenarios and refuse more than one,
  naming them.
- `crates/conductor-cli/src/main.rs` — C3: a `hint_for` arm for the ambiguous-P-ID fault.
- `crates/conductor-cli/tests/cli_smoke.rs` — C3: the ambiguous-P-ID refusal test.
- `.github/workflows/ci.yml` — C4: the comment at `:387-392` only.
- `crates/conductor-tauri/ui/knip.json` — C2, branch (a): `ignoreExportsUsedInFile`.
- `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` — C2, branch (b): drop `export` on `SUITE_SELECTION`.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — C2, branch (b): drop `export` on `CoverageMode`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` — C2, branch (b): drop `export` on 13 symbols.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` — C2 branch (b) (`RowState`), and C5: E0-10 re-tokened
  to the banner content, `absent` removed.
- `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` — C5: E0-10 driven as a browse row instead of
  `stamp(…, 'none (subject absent)')`.
- `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` — C5: the E0-10 spec row (`:181`) and the
  routed-finding note (`:241`).

## Scope premise closure (applied to scope.md)
- Moved surfaces — **VERIFIED, enumerated** from Epoch 5's seven frozen entries (`working-route.md:58-72`): the
  secret-scan gate; `mutation-gate.py`'s `selftest`; roving matrix focus, the modifier key map and the untransitioned
  ring; the SR harness (OS key path, per-session clock calibration, T-01's second run); the SR content fixes (the
  single `h1`, the `contentinfo` strip, rows named by their cells, the visually-hidden titlebar count at
  `Titlebar.tsx:45`).
- "Both runners" — **VERIFIED**: test-plan §4 Framework (nextest AND full non-doc `cargo test`), §10 (a
  runner-dependent result is a determinism break); doctests are the separate `cargo test --workspace --doc` leg.
- Gate set — **VERIFIED, widened**: to the scope's list, add `cargo test --workspace --doc`; the `live-pulse`
  feature clippy on `conductor-run` (the only crate with the feature); the standalone per-seam build sweep (`--lib`
  ×7, `--bins` ×2, never `--all-targets`); `cargo llvm-cov` at `--fail-under-lines 60`; `mutation-gate.py
  selftest`; `npm run a11y:ownership`; `npm run typecheck:e2e`.
- Live-drive path — **VERIFIED**: `agent-run.{sh,ps1} run --live` = H (`halo-hue-encoding`) → B1/B2
  (`degraded-mode-report` twice) → 150 s window → A (`auto-resolve-idle-window`) → the driven a11y arm. The expected
  length is ≈ 20 min (each leg's preflight is ≥ 45 s of warm-up plus canary; H 180 s and A 180 s of scenario; the
  150 s window; the driven arm's dated 4 m 2.1 s sample), bounded by `live_leg_budget_sec`. Leg A is not run-stable
  (verification-harness.md 2026-09-06 (e)), so its grade is recorded as conditional, never as a regression.
- C2 count — **FALSIFIED**: `npx knip` at HEAD exits 1 with **15** findings (4 unused exports + 11 unused exported
  types), not 12. `readSpeechLog` / `readStamps` / `calibrateClock` / `SUITE_SELECTION` plus `CoverageMode`,
  `Outcome`, `Arm`, `InputPath`, `PassRow`, `SubjectRecord`, `OperatorReview`, `ReviewGrade`, `Timeline`,
  `ClockCalibration` and `RowState`; the three new since 3ddd405 are `calibrateClock`, `InputPath` and
  `ClockCalibration`. All 15 are used inside their own file only.
- C2 "a11y-plan §11 forbids acting on them against the harness" — **FALSIFIED**: a11y-plan §11 (`:511-587`) holds no
  rule about unused exports, knip or harness symbols (grep for harness/dead/unused/delete/remov/lint/export over the
  section: the hits are the carve-out, the lint-only-a11y ban, the fail-fast ban, the readiness ban and the
  one-stack ban). The claim originates in `frontend.md:54`, a rule-file leaf. The CARRY's operative instruction
  ("disposition each symbol, never delete blind") stands as operator direction.
- C3 census — **VERIFIED** (re-derived: `scratchpad/pid_census.py` over `scenarios/*.toml`'s `p_ids`): 37 scenario
  files, 49 distinct P-IDs, exactly 7 ambiguous — P-017 ×2 · P-018 ×2 · P-019 ×3 · P-020 ×3 · P-021 ×2 · P-022 ×2 ·
  P-060 ×3. On this NTFS host `os.listdir` returned sorted order, so the "first" match is alphabetical here by
  accident; `read_dir` order is unspecified in general.
- C4 coordinates — **CORRECTED**: the stale clause is at `ci.yml:389-392` inside the block `:387-392`; the step is at
  `:398` (the CARRY cited `:351-356` at `6008a68`).
- C5 hypothesis — **VERIFIED**: `runs/e2e-fixture/runs.db` carries the over-envelope `run_envelope` row for
  `lamps-fixture` (label `ENVIRONMENT-SUSPECT`), and E0-07's own window heard the banner. So the banner renders in the
  sr-empty subject, and E0-10's absent reason is false. Owed: re-token E0-10 to the banner's label text and drive it
  as a browse row; do not re-seed sr-empty.
- CI verdict — **VERIFIED green** (above).
- No new dependencies, and no ninth harness-spawn form — **VERIFIED as achievable**: knip config or an `export`
  keyword drop adds no package; C5 reuses `browseUntil` over the existing `send-keys.ps1` path.

## Leaves whose stated claim this chunk falsifies (wrap cascade, never edited here)
- test-plan §3 `run` / Test selection (the first-match ambiguity sentence) — C3.
- `verification-harness.md:19` ("a P-ID resolves to the first scenario naming it in directory order"),
  `testing.md:38` ("a P-ID target is determinate only where one scenario names it"), `.claude/docs/commands.md:18`
  — C3.
- `frontend.md:54` ("a11y-plan §11 forbids acting on such a report against the harness"; "residual 12") — C2.

## Open questions
- C1: de-link (plain backticks) or widen the four constants/fn and one method to `pub`? → blocks: plan-decision.
  Lean: de-link; widening adds public API to a crate whose derivation is pinned to Pulse's.
- C2: `ignoreExportsUsedInFile` in `knip.json` (one line, the exports stay) or drop `export` on the 15 symbols? →
  blocks: plan-decision (it decides which of the C2 files above are touched).
- C3: refuse an ambiguous P-ID, or pick deterministically (sorted)? → blocks: plan-decision. Lean: refuse, per the
  `preconditions --for` precedent and every extract.
