# Report — 2026-09-30-full-gate-regression-over-the-moved-surfaces

**Chunk:** Full-gate regression over the moved surfaces — scenario corpus, the a11y job and the live-drive path green
under both runners; five CARRYs closed (rustdoc private links, knip exports, ambiguous P-ID, ci.yml comment, SR row
E0-10)
**Date:** 2026-10-01
**Commits:** `fb5e69a chore(2026-09-30-full-gate-regression-over-the-moved-surfaces): operator pre-CI commit, for the
run this chunk's verdict reads` (the only commit since `last_wrap` 2026-09-30T20:37:19Z; basis `git log --format='%h
%s' 84f3b27..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/src/exception.rs` · `crates/conductor-emit/src/pii.rs` ·
  `crates/conductor-cli/src/paths.rs` · `crates/conductor-cli/src/main.rs` · `crates/conductor-cli/tests/cli_smoke.rs`
  · `.github/workflows/ci.yml` · `crates/conductor-tauri/ui/src/components/ScenarioPicker.tsx` ·
  `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` ·
  `crates/conductor-tauri/ui/test/a11y/screen-reader/parse-nvda-log.ts` ·
  `crates/conductor-tauri/ui/test/a11y/screen-reader/rows.ts` · `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts`
  · `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md` (12 source files, basis `git diff
  --name-only 84f3b27 -- crates .github`) + the chunk folder (`evidence/{nvda-pass.json, regression.md,
  live-suite/{h,b1,b2,a}.jsonl}`) + run dirs.
- **Symbols / APIs:**
  - **C3 — OPERATOR-VISIBLE CLI CHANGE.** `conductor run <P-ID>` (and therefore the harness's `SCENARIO=<P-ID>`, which
    passes `run "$SCENARIO"` straight through, `agent-run.sh:326-330` / `.ps1:376-383`) now REFUSES a P-ID that several
    scenarios name, before any load: an `anyhow` harness fault `{P-ID} is named by {n} scenarios ({stem}, {stem}…); run
    one by name` → exit 1 with `error:` + `hint: pass one of the named scenarios instead of the P-ID`. It used to take
    the FIRST scenario naming the P-ID in unsorted `read_dir` order, silently. A single-owner P-ID still resolves; an
    unknown target keeps `no scenario matches "{target}" (by name or P-ID)`; unloadable files are still skipped; the
    by-name branch is untouched. Seven P-IDs are affected (P-017 ×2 · P-018 ×2 · P-019 ×3 · P-020 ×3 · P-021 ×2 ·
    P-022 ×2 · P-060 ×3; research.md's census over `scenarios/*.toml` p_ids, 37 files).
  - `paths.rs` private `find_by_pid` now returns every match `(file stem, Scenario)` over
    `conductor_core::scenario_files` (sorted) instead of the first `read_dir` hit; sole caller `Paths::load_scenario`
    (code-graph rust plane, 1 caller); `load_scenario` keeps its 2 callers (`commands/run.rs:19`,
    `commands/preconditions.rs:30` — the latter by NAME only, since `--for` refuses a P-ID at parse). No signature
    crosses a crate edge.
  - `main.rs` `hint_for` gains one arm keyed on the stable marker ` is named by `.
  - **C1** — five intra-doc links to private items de-linked to plain backticks (`exception.rs` `MAX_FRAMES`,
    `FINGERPRINT_BYTES`, `NORMALIZED_FRAMES`, `normalize_stacktrace`; `pii.rs` `PiiCategory::index`); doc prose only, no
    visibility change, conductor-emit `Cargo.toml` untouched.
  - **C2** — `export` dropped on 15 symbols, each used only in its own file (0 cross-file users, basis `grep -rlw`
    per symbol at P3): `SUITE_SELECTION` (ScenarioPicker) · `CoverageMode` (CoverageMatrix) · `RowState` (rows.ts) ·
    `readSpeechLog`, `readStamps`, `calibrateClock`, `Outcome`, `Arm`, `InputPath`, `PassRow`, `SubjectRecord`,
    `OperatorReview`, `ReviewGrade`, `Timeline`, `ClockCalibration` (parse-nvda-log.ts). Nothing deleted; `knip.json`
    unchanged.
  - SR harness (`screen-reader.e2e.ts`, test-only): `stamp(id, action, sharesPreviousInstant = false)` — a row
    declared to share the previous row's instant is stamped AT that instant; the three declared partners S2-02, S2-08,
    S3-05 pass `true`. The parser (`parse-nvda-log.ts` `SHARED_WINDOW_MS = 50`) is unchanged.
  - **C5** — SR row E0-10 (`rows.ts`) re-tokened to the rendered `p.report__envelope` banner (label text
    `ENVIRONMENT-SUSPECT`), its `absent` key removed; the empty-subject walk drives it as a browse row forward from
    Start (`h` to "Run report", ArrowDown to "ENVIRONMENT-SUSPECT"), ahead of E0-07, which now continues ArrowDown from
    the banner; the `stamp('E0-10', 'none (subject absent)')` line deleted; `nvda-pass-spec.md` E0-10 row + routed-note
    disposition updated. Keys stay on the existing `send-keys.ps1` browse path; sr-empty not re-seeded.
  - No env var, port, socket, IPC method, `Commands` verb or harness verb added.
- **Crates / modules:** none added / removed.
- **Dependencies:** none — `Cargo.lock`, `Cargo.toml`s, `package.json` and `package-lock.json` untouched (delta guard
  entry `{ git diff --name-only 84f3b27 -- … } | grep -vE '^(…touchpoints…)$'`: no output, exit 1).
- **Schema / config:** none (no scenario, contract, `knip.json`, `wdio.conf.ts` or `send-keys.ps1` change).
- **Spec-master edits:** none (implement wrote no master).
- **Counts / qualifiers moved:**
  - `npm run knip` findings 15 → 0, exit 1 → 0 (basis: P3's `npx knip` at HEAD, 4 exports + 11 types; entry `npm run
    knip` exit 0). Stated stale in the leaf `.claude/rules/frontend.md:54` ("the residual 12"); no master states a knip
    count (grep `knip` over the seven: 0 hits).
  - `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p conductor-emit`: 5 errors / exit 101 → exit 0. No master states
    the count (grep `5 warnings|generated 5 warnings|five private` over the seven and `.claude/`: 0 hits).
  - SR regrade tallies: 49 announced-as-expected + 2 subject-absent (E0-10, S1-05) → 50 + 1 (S1-05). No master states
    the tally or E0-10's absent reason (grep `E0-10|49 announced|subject-absent` over the seven: test-plan `:47` and
    a11y-plan `:268-269` hit on the 51-row count / class names only — unchanged, no-change).
  - `cargo test --workspace` passed 1139 → 1140 (the new C3 test; basis plan baseline + entry log, 70 targets);
    nextest 1137 passed (entry 2 log). No master bakes either total.
- **Dev-tool versions:** none — WebView2 runtime / msedgedriver re-read at 154.0.4258.37 (coherent) on the dev host;
  NVDA 2026.2 re-read; CI `a11y` job image `windows-2022` 20260927.320.1 with a coherent 131.0.2903.86 pair, unchanged.
- **Harness / gate surface:**
  - CLI: the ambiguous-P-ID refusal above (operator-visible; the `SCENARIO=<P-ID>` harness path inherits it).
  - SR harness: the shared-instant stamp (above); E0-10 graded as a browse row.
  - `ci.yml`: comment lines only (`:387-392` block; non-comment changed lines 0, entry `git diff -U0 84f3b27 --
    .github/workflows/ci.yml | … | grep -cvE '^[+-][[:space:]]*#'` → 0). No CI step added or removed.
  - conductor-emit's doc gate promoted to the `-D warnings` form in this plan's Test Commands (a plan fence, not a CI
    step; no CI job runs `cargo doc`).
- **Cross-project / external claims:**
  - Pulse `7bb56ea` (live legs): `pulse-app.exe` sha256 `ff677eb06d087ada…1525e478e`, `andromeda-pulse-mcp.exe` sha256
    `2179caab9f7247a5…bf56c34cc634`; sources equal `7bb56ea` product code (Pulse tree uncommitted only in records; every
    tracked product file older than the binary except the generated bindings, byte-equal to HEAD) — basis
    `evidence/regression.md`.
  - CI: **CI#36793057095 on `fb5e69a`** — green, 3/3 checks completed/success (Rust gate · Frontend gate · A11y gate),
    696 s wall; read by `ci.py conclusion --sha HEAD --wait 1200` at the operator pass, overseer-verified.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - `.claude/rules/frontend.md:54` (leaf, 2026-09-07 entry): "a11y-plan §11 forbids acting on such a report against
    the harness" — a11y-plan §11 (`:511-587`) holds no rule on unused exports, knip or harness symbols (research.md P3
    grep over the section); and "the residual 12" — measured 15 at P3, 0 after C2.
  - The route CARRY's knip count "twelve" (`working-route.md:73`, frozen) — measured 15 (research.md; `scope.md`
    premise-corrected). Frozen line: no edit; the correction lives in scope.md + this report.
  - The parser comment "Rows the leg stamps 'in the same instant' … land a few ms apart" (`parse-nvda-log.ts:506`,
    code) — measured 53 ms and 62 ms on this host (`actions.live.jsonl`, first live fire); remedied at the stamp, the
    comment's claim now holds by construction.
- **Expected amendments (from plan):**
  - test-plan §3 run / Test selection (`test-plan.md:151`): "`run` (and the harness's `SCENARIO=`) take the first
    scenario naming the P-ID in unsorted directory order" → an ambiguous P-ID is REFUSED, naming its scenarios —
    **carried** (Symbols / APIs C3). Search: `grep -E 'first scenario naming|in unsorted directory order|directory
    order|ambiguous P-ID|named by several|first scenario'` over the seven: architecture 1 (`:244`, "a P-ID may be named
    by several" — still true, no-change) · test-plan 6 (`:151` the claim → change; `:39`, `:71`, `:128`, `:201`, `:396`
    "a P-ID may be named by several" — still true, no-change) · the other five 0.
  - security-plan §Input Validation CLI-arguments row (`security-plan.md:123`, names the `--for` refusal) — `run
    <P-ID>` now refuses an ambiguous P-ID beside it — **carried** (C3); the detector's call. Search: same pattern →
    security-plan 0 hits; `grep -n 'preconditions --for'` → `:123` the row.
  - Leaf cascade (not masters): `verification-harness.md:19` ("a P-ID resolves to the first scenario naming it in
    directory order"), `testing.md:38` ("a P-ID target is determinate only where one scenario names it"),
    `.claude/docs/commands.md:18` ("resolves to the first in directory order") — **carried** (C3); `frontend.md:54` —
    **carried** (Spec claims disproved).
- **Coverage of new surfaces:**
  - `conductor run <P-ID>` ambiguous-P-ID refusal → validation refused-before-load✓ · instrumentation n/a (a harness
    fault rendered at the `anyhow` edge through `sanitize_error`, as every CLI fault) · PII redacted✓ (stems only; no
    TempDir path and no ESC asserted) · tests integ (`cli_smoke::run_refuses_a_p_id_named_by_several_scenarios`, red
    at base / green now) + smoke (real binary) · a11y n/a (cli) · tokens n/a.
  - SR row E0-10 (test harness) → validation n/a · instrumentation n/a · PII n/a · tests e2e (sr-empty, graded
    `announced-as-expected os`) · a11y SC 1.4.1 banner label as text✓ · tokens n/a.

## Deviations from intent
- **Live `sr` leg — one HARNESS defect, fixed and re-fired once (operator rule 2026-09-30).** The first live fire (slot
  C) graded S2-01 and S2-07 `not-announced`, `heard: []`: the shared-window stamp pairs landed 53 ms / 62 ms apart, past
  the parser's 50 ms tolerance, so each first row got a few-ms slice while its speech landed in its partner's window
  (S3-04→S3-05 landed 44 ms apart and passed by luck). Fixed at the cause in `screen-reader.e2e.ts` (a listed file) —
  the declared partner is stamped at its first row's instant; no threshold changed (plan step 7 / test-plan §11).
  Before the re-fire, the operator's condition held: the rule replayed over this session's sr-empty and sr-error
  captures graded byte-identical (0 stamps moved, 10 + 4 rows), and the same replay over the first live capture graded
  every live row expected. Re-fire (slot C', fresh Pulse dir `fullgatesr`): 51 rows, `not-expected none`. The first
  live result survives only in `evidence/regression.md` (the working record was overwritten by the re-fire).
- Operator review dated 2026-10-01 (the session crossed midnight UTC); the record probe reads `>= "2026-09-30"`, met.
- Beside the plan: `secret_scan_gate` run once over the final tree with `evidence/` present (5/5) — not a listed entry.
- Two pulse-app launches (slots C and C') instead of one — the re-fire's.
- scope record: none — `gate.py scope` clean (changed 12 · listed 12 · recorded 0), 0 recorded.

## Decisions & corrections
- Operator directive (P5 review, restated at this wrap): C3 is an **operator-visible CLI change** — name it in the
  report (done, Symbols / APIs).
- Operator rule applied: a slot-gated leg's harness defect re-fires once on a new slot; the overseer added a
  pre-re-fire condition — replay the fix over the session's other captures and require byte-identical grades.
- Overseer directive: record the pulse-app sha256 and that its sources equal Pulse `7bb56ea` product code (done).
- Wrap directives: curate the shared-instant stamp rule where the harness parser rules already live, not as a new
  section; the route stays as is (next: "Interpretation re-proven…", still BLOCKED-ON Pulse real-model incident
  surfacing).
- Learning (harness): two back-to-back synchronous `appendFileSync` stamps on this host can land 53–62 ms apart, so a
  pairing INFERRED from a timing gap is host-timing-dependent; a pair DECLARED to share an instant must be RECORDED at
  one instant. Measured with its replay control.
- Sweep hazards: the bash-guard hook blocks any command carrying a doubled backslash (a sed over a Windows path) and a
  `cat > file <<` heredoc — route both through the Edit/Write tools or a scratchpad file.

## Outcome
Acceptance criteria, re-asserted against the diff:
- (tests) Both runners + doctests green, no retries / thread knob — **met** (nextest 1137/1137 via `agent-run.sh run
  --unit`; `cargo test --workspace` 70 targets 1140 passed; `--doc` 3 passed).
- (tests) Lint and supply chain green — **met** (fmt; clippy workspace; clippy `live-pulse`; per-seam sweep; llvm-cov
  93.98 % ≥ 60; mutation selftest; porcelain empty; `cargo audit` exit 0 (1277 advisories · 562 deps · 7 allowed);
  `cargo deny` all ok; `Cargo.lock` unchanged).
- (arch/C1) emit `-D warnings` doc exit 0 (was 101), conductor-run form 0, fingerprint pins pass (inside both runners),
  emit `Cargo.toml` unchanged — **met**.
- (tests/C2) knip exit 0; 15 dispositioned, nothing deleted; build / typecheck:e2e / ownership (`0 recorded gaps`)
  green — **met**.
- (C3) ambiguous P-ID refused before any load, `error:` + stems + `hint:`, no host path / ESC, non-zero; single-owner
  still resolves; no new verb — **met** (diff adds no `Commands` variant, no harness verb).
- (C4) stale rationale 0 hits, comment-only diff, env-context gate green inside the suite, CI `a11y` green on the push —
  **met** (CI#36793057095).
- (C5) 51 rows on one configuration after the base, `not-run-here 0`, E0-10 `announced-as-expected os`, absent set
  `S1-05`, `not-expected none`, review transcribed, configuration tuple stated — **met** (`evidence/nvda-pass.json`;
  `evidence/regression.md`).
- (a11y/layouts) `--e2e` strict 0 failed / 2 skipped (expected 2); CI `a11y` green with image + pair — **met**.
- (tests/obs) `run --live` exit 0, B2 `[BLOCKED]` by design, no `[FAIL]`, driven arm `Spec Files:` 1 passed, seven
  base keys 0 missing, leg A conditional with posture from Pulse's log — **met**.
- (security/obs) evidence host-path-free (0), closed sets 0, panics 0, no 4444/4445 listener, no started image, rule
  (b) at eight (send-keys.ps1 / activate-window.ps1 / wdio.conf.ts byte-unchanged; 2 spawn sites) — **met**.
- No matrix claim: `claimed by {marker}: 0` · `unclaimed 0 of 11` — **met**.

Gates (by `run`, implement's run + the operator pass; logs `andromeda-gate/…/implement-2026-09-30T21-26-56`):
- `cargo fmt --all --check` — green · exit 0.
- `bash scripts/agent-run.sh run --unit` — green · exit 0 (1137 passed).
- `cargo test --workspace` — green · exit 0 (1140 passed, 70 targets).
- `cargo test --workspace --doc` — green · exit 0 (3 passed).
- `cargo nextest run -p conductor-cli --profile ci -E 'test(=run_refuses_a_p_id_named_by_several_scenarios)'` —
  green · exit 0 · `contains 1 passed` · `lacks 0 tests run`.
- `cargo run -q -p conductor-cli --bin conductor -- run P-017` (smoke) — green · exit 1 · all three `contains` atoms.
- `cargo clippy --workspace --all-targets -- -D warnings` — green.
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` — green.
- per-seam build sweep — green (304 s).
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p conductor-emit` — green · exit 0.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps -p conductor-run` — green.
- `cargo llvm-cov nextest --workspace --profile ci --no-report` — green (2524 s).
- `cargo llvm-cov report --fail-under-lines 60 …` — green (93.98 %).
- `git -C "$CARGO_HOME/advisory-db" status --porcelain` — green · no output.
- `cargo audit` — green · exit 0.
- `cargo deny check advisories bans licenses sources` — green.
- `python -X utf8 scripts/mutation-gate.py selftest` — green.
- `npm run build` · `npm run typecheck:e2e` · `npm run knip` · `npm run a11y:ownership` · `npm audit --omit=dev` —
  green (ownership `contains 11 claims · 9 owned …` + last line held); the TS / delta gates re-run green after the
  stamp fix.
- `grep -c 'holds only what a workflow, job or step declared' …` — green · exit 1 · last line 0.
- ci.yml non-comment-line probe — green · exit 1 · last line 0.
- delta guard — green · exit 1 · no output.
- rule (b) numstat — green · no output. `spawnSync(` count — green · last line 2.
- driver/runtime coherence — green · `driver 154.0.4258.37 runtime 154.0.4258.37`.
- `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e` — green · exit 0 · `[a11y] verdict asserted — 0 failed`
  · `(expected 2)` (slot A).
- `rm -f runs/sr-leg/nvda-pass.json` — leg operator · driven by hand, exit 0 (slot B).
- `npm run a11y:sr-empty` · `npm run a11y:sr-error` — leg operator · exit 0 each · `Spec Files:\t 1 passed, 1 total`.
- liveness probe — leg operator · exit 0 · `sidecar on PATH` · last line `True` (slots C and C').
- bootstrap-override probe — leg operator · exit 0 (1 line).
- `PATH=… bash scripts/agent-run.sh run --live` — leg operator · exit 0 · all five atoms held (slot C).
- seven-base-keys probe — green · last line 0.
- `sleep 170 && … npm run a11y:sr` — leg operator · exit 0 · `Spec Files:` 1 passed; first fire graded S2-01/S2-07
  off (harness defect, Deviations), re-fire green (slot C').
- evidence copy — leg operator · exit 0.
- record probes — green: `rows 51 not-run-here 0 absent S1-05` · `announced-as-expected os` · `not-expected none` ·
  `one-configuration true review true` · closed sets 0.
- evidence host-path probe — green · last line 0. panics probe — green · 0. 4444/4445 listener — green · 0.
  started-image census — green · 0.
- `gate.py hygiene` — leg operator · exit 0 · `hygiene: clean`.
- guarded push — leg operator · exit 0 · `PUSHED_SHA=fb5e69a8d68ed236b03f89ac1a505af50a20996e`.
- `ci.py conclusion --sha HEAD --wait 1200` — leg operator · exit 0 · `verdict: green` (CI#36793057095, 3/3).
- No `defer`; smoke ran on the changed CLI boot path (above).

Watches: none folded.

Outcome basis: the operator pass ran — pre-CI commit `fb5e69a` (parent `84f3b27`), pushed, and the final HEAD's CI run
CI#36793057095 green; implement's P4 report (this conversation) for everything else, plus the operator's acceptance
("implement accepted") and the wrap directives above.

Process hygiene (implement P4 census, re-measured here — no `msedgedriver` / `conductor-tauri` / `tauri-driver` /
`nvda` / `pulse-app` / `andromeda-pulse-mcp` image running, no 4317/4318/4444/4445 listener):

| process | started by | final state |
|---|---|---|
| pulse-app (`fullgate`) + 10-process tree | this session, slot C | terminated (forced after 15 s grace, 0 survivors) |
| pulse-app (`fullgatesr`) + 10-process tree | this session, slot C' | terminated (forced after 15 s grace, 0 survivors) |
| conductor-tauri · msedgedriver · tauri-driver · NVDA · node | the legs | terminated (self-teardown) |
