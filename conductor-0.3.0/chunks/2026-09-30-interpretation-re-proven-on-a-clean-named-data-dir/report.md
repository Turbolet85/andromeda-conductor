# Report — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

**Chunk:** Interpretation re-proven on a clean-named data dir — hermetic CARRYs (digest pins, workspace-key mask,
storm-prefix elision) + the a11y axe-inject race, then a pre-registered real-model series for v3-09 gated on
Pulse's scrubber fix
**Date:** 2026-09-30
**Commits:** `b8e7bca` chore(2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir): operator pre-CI commit, for
the run this chunk's verdict reads (parent `9785405`, the chunk base; `git log --format='%h %s' 9785405..HEAD` → 1)

## Changes (structured — detectors read this)
- **Files** (`git diff --numstat 9785405 HEAD` over `crates/ contracts/ Cargo.lock` → 8 files, +796/−903):
  `Cargo.lock` (+1), `crates/conductor-run/Cargo.toml` (+2), `crates/conductor-run/tests/real_model_harvest.rs`
  (+540/−133), `real_model_series/mod.rs` (+37/−764; 801 → 74 lines), `real_model_common/mod.rs` (+75/−1),
  `real_model_live.rs` (+28/−5), `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (+57),
  `contracts/pulse-real-model-leg-posture.md` (+56, add-only). Chunk evidence (new): `evidence/rm-capture-2026-09-22-elided.txt`,
  `rm-capture-d1.txt`, `-d2.txt`, `-d3.txt`, `attempt-ledger.md`, `race-witness.md`. The phase run dir's
  `p5-controls/hp-leak.txt` MOVED out of the tree (operator word) to the gitignored
  `.andromeda/cache/p5-controls/2026-09-30T04-14-05-phase/hp-leak.txt`, a note `hp-leak.MOVED.md` in its place.
- **Symbols / APIs** (all test-module scope, `crates/conductor-run/tests/`; no crate public API moves):
  - `real_model_common`: NEW `WORKSPACE_KEY_PLACEHOLDER` (`<workspace-key>`), `mask_workspace_key(text, key: Option<&str>)`
    (every `workspace=` line's value masked whatever it holds; the key masked wherever bounded outside
    `[A-Za-z0-9_.-]`), `workspace_rendering(report, key) -> verbatim|scrubbed|absent|unknown-key` (`verbatim` = the value
    or its last path component equals the key). `rule_section` doc no longer claims "never a hash, which would need a
    dependency". Callers: `real_model_live.rs` (both) and `real_model_harvest.rs` (both, tests).
  - `real_model_series`: `Drive { label, file, sha256 }` (the `block` literal field REMOVED); NEW `EVIDENCE_2026_09_30`,
    `SERIES_2026_09_30` (d1-d3). Sole consumer: `real_model_harvest.rs`.
  - `real_model_live.rs`: `emit_block` now applies `mask_workspace_key` FIRST, then `redact_value` → `mask_host_paths` →
    `elide_fingerprints`; NEW `workspace_key()` (the data dir's basename, once, from the guarded
    `capture_paths::pulse_logs_dir_from` path's parent — never a raw env read); the capture prints NEW line
    `pulse-report workspace rendering: {class}` before the report sections (class only, never the value).
  - `real_model_harvest.rs`: `PINNED_CAPTURE` literal DELETED; NEW `sha256_hex`, `check_digest`, `committed`, `pinned`,
    `contract_section`, `capture_2026_09_30`; tests: 71 → 90 `#[test]` (`grep -c '^#\[test\]'` at `9785405` vs HEAD) —
    removed `pinned_literals_equal_the_committed_capture`, `each_series_block_equals_its_committed_capture`; added the
    digest tests, `the_elided_copy_is_the_frozen_capture_through_the_rule`,
    `a_one_byte_change_to_a_pinned_capture_fails_its_digest`, `no_committed_capture_text_sits_in_test_source`, nine
    mask/rendering arms, `the_2026_09_30_series_rule_was_fixed_before_d1`, six 2026-09-30 series tests incl.
    `v3_09_is_not_met_by_the_2026_09_30_series`. The rule markers are untouched (every drive's recorded rule equals it).
  - `accessibility.e2e.ts`: NEW `RESPONSIVE_MS = 250`, `untilResponsive()` (`browser.waitUntil` until
    `execute(() => document.readyState === 'complete')` answers `true` in < 250 ms; 30 s bound, 250 ms interval, thrown
    execute = false); `axeFindings()` calls it before its one `analyze()`; NEW spec `axe completes over a 1.5 s
    main-thread stall` (a one-shot `document.readyState` getter blocking the main thread 1 500 ms on its next read).
  - Env handles: none new. `ANDROMEDA_PULSE_DATA_DIR` gains a second READ in the capture test binary, through the
    existing guard.
- **Crates / modules:** none added/removed; `conductor-run` test targets changed only.
- **Dependencies:** `sha2 = "0.10"` added as a `conductor-run` **dev-dependency** — `sha2 0.10.9` was already locked
  (via `tauri-codegen`, `wry`), so `Cargo.lock` gains one dependency-list edge line and no package
  (`grep -c '^name = ' Cargo.lock` = 562 before and after). A second hashing crate beside `blake3`, test-only.
- **Schema / config:** scrub shapes — the real-model capture's scrub pipeline gains a FOURTH stage, the workspace-key
  mask (placeholder `<workspace-key>`), applied first; the committed capture evidence is now held by sha256 digest pins
  over LF-normalized content (the harvest grades the file only after the digest matches). The graded 2026-09-23 capture
  is this chunk's elided copy (its two `fingerprint_hex=` storm tokens → `<fingerprint>`); the FROZEN 2026-09-22 file
  keeps its prefix (never edited — residual).
- **Spec-master edits:** none (implement is spec-read-only). The reader-less contract
  `contracts/pulse-real-model-leg-posture.md` gained `## The 2026-09-30 series` (add-only, before d1; the 2026-09-29
  record unedited).
- **Counts / qualifiers moved:**
  - The routine a11y arm's pass tally **12 → 13** (skip set unchanged at 2) — measured on the dev host
    (154.0.4258.37 pair, gate entry 14 + a repeat) and in CI#36681853843 (131.0.2903.86 pair). Sites baking `12 passing`
    (`grep -noE '12 passing'`): architecture.md 2 (both on `:60`) + `:250`, test-plan.md `:56`, `:307`, a11y-plan.md
    `:115`, `:465`, `:516` (×2) — most are run-anchored records (run 35208593666 / "as of 2026-09-17"); a detector
    judges which are current-state claims. `11 passing` hits (test 3, a11y 3) are older dated records.
  - `real_model_harvest` tests 71 → 90; workspace nextest 1136 (last run). No master bakes `71 passed` (0 hits in all seven).
  - `real_model_series/mod.rs` 78 879 B → 74 lines; no master bakes its size.
- **Dev-tool versions:** the **driver** `msedgedriver` on the **dev host**: 152.0.4191.53 → 154.0.4258.37, installed by the
  operator at the same `CONDUCTOR_MSEDGEDRIVER` path on 2026-09-30 (~06:10Z; Authenticode `Valid`, signer
  `CN=Microsoft Corporation`, re-read here). The **runtime** WebView2 on the dev host read 154.0.4258.37 (Evergreen
  auto-update; the 152 driver was refused against it). CI image re-read unchanged: `windows-2022` 20260920.314.1, driver and
  runtime both 131.0.2903.86. Not a lockfile subject.
- **Harness / gate surface:** no `scripts/agent-run.*`, verb, flag, selector or CI step change
  (`git diff --numstat 9785405 -- … scripts/agent-run.sh scripts/agent-run.ps1 … .github/` → empty). The routine `--e2e`
  arm gains one spec (above); its skip set is unchanged, so `A11Y_EXPECTED_SKIPS=2` holds. The capture's printed grammar
  gains the `pulse-report workspace rendering:` line (not a rule token).
- **Cross-project / external claims:**
  - Pulse (`andromeda-pulse`), read at committed HEAD with `git show`: `fcc31b2` (`fcc31b21666d…`) is the scrubber fix — the
    `credit_card` arm redacts a digit-group run only when a window of whole groups with 13-19 digits passes Luhn
    (`crates/security/src/scrubber.rs`); pushed (`@{u}` = HEAD = `fcc31b2`, local tracking ref, no fetch).
  - Pulse's workspace key is a filesystem PATH — the detected workspace root, else the data dir
    (`crates/workspace-detector/src/contract.rs` `workspace_key`, `:77-82` @`fcc31b2`).
  - The read-back report is rendered BY THE MCP SIDECAR through `assemble_report`, which scrubs `workspace` and every
    model-written field with the linked scrubber (`crates/interpretation/src/markdown.rs:254-286` @`fcc31b2`), so the
    sidecar binary must carry the fix too.
  - Binaries built from `fcc31b2` (clean worktree) and proven by content (old `credit_card` regex source absent, new
    present): `pulse-app.exe` sha256 `6aed4a9a…`, `andromeda-pulse-mcp.exe` `2179caab…`; the pre-build pulse-app (from
    `e98d838`) read the reverse, as the control.
  - CI: CI#36635281444 on `9785405` red (A11y gate, `Page/Frame is not ready`, folded as scope item 4);
    CI#36681853843 on `b8e7bca` green 3/3, wall 869 s — the a11y job on `windows-2022` 20260920.314.1 at a coherent
    131.0.2903.86 pair, 13 passing / 2 skipped.
  - Relayed, not measured: Pulse's route now carries "Real-model incident surfacing" (founder 2026-09-30: fix in Pulse 0.3.0).
- **Reverted / negative API facts:** the plan's step-9 stall form (`browser.execute(() => setTimeout(busy 1 500 ms, 0))`)
  was written, measured non-discriminating, and replaced before the push; a timing diagnostic written into the stall arm
  for one run was removed. A ledger-side no-fingerprint assertion over the 2026-09-30 attempt ledger was written and
  dropped (the ledger carries sha256 digests + a commit sha by design).
- **Insufficient fixes (written, kept, not the remedy):** Pulse's scrubber fix (`fcc31b2`) — correct and shipped; it
  resolved the 2026-09-29 contamination (no capture reads back `[redacted: credit_card]`; the data dir's leaf occurs 0
  times in every capture); it did NOT make `v3-09` met: the series graded 0 drives because the real model dismissed the
  scenario's storm digest in both emitting drives. Remainder owner: Pulse's "Real-model incident surfacing" entry (founder
  word, relay `conductor-wrap-56-2026-09-30` §1), then a third pre-registered series (relay §2).
- **Spec claims disproved by measurement:**
  1. Plan step 9 + its `## Test Commands` prose (`chunks/{marker}/plan.md`): a 1 500 ms busy loop scheduled by
     `setTimeout(…, 0)` inside a `browser.execute` "pushes axe's probe past its 1 000 ms budget". Measured false:
     with the wait bypassed that arm PASSED (run 2, 13 ✓); a diagnostic showed the stall ran 1 500 ms
     (page clock 1608.3 → 3108.3) but the scheduling execute absorbed it (next probe round trip 17.7 ms). The underlying
     mechanism (a classic execute queues behind the page's synchronous JavaScript) holds; its placement does not.
     Evidence: `evidence/race-witness.md` runs 2-4. Not stated in any master (`FRAME_LOAD_TIMEOUT|readiness race|axeFindings`
     → 0 hits in all seven).
  2. research.md / scope.md: "the key is the data dir's BASENAME". Pulse stamps a PATH (above); the leaf equals the
     data dir's leaf only when detection falls back to the data dir (as under this series' launch — the log's
     `app.boot.workspace_key` basename read `rm-clean-series`). Master sites naming the key: security-plan.md 1 hit, architecture.md 2,
     test-plan.md 1 (`workspace basename|workspace key`) — a detector reads whether any states it is a basename.
  3. a11y-plan.md `:424` attributes `Page/Frame is not ready` to BiDi script evaluation (2026-09-01, true of that
     session). The string is `@axe-core/webdriverio`'s catch-all for ANY missed 1 000 ms readiness probe
     (`index.mjs:95-113`); CI#36635281444's red was the classic-execute timing race, not BiDi (research.md §Scope premise
     closure 5). The master sentence is not wrong about 2026-09-01; it is incomplete as a diagnosis of the string.
- **Expected amendments (from plan):**
  1. security-plan §Security Anti-Patterns → Data Protection, the BREACH line → remedied (digest pins, no corpus text in
     test source, the graded 2026-09-23 copy elided) with the residual that the frozen 2026-09-22 file keeps its prefix —
     **carried** (Schema / config; Symbols). Search: `grep -c BREACH` → security-plan 1 (`:335`), all other masters 0.
  2. security-plan §Input Validation, the real-model capture ingest row → the workspace-key mask stage and the rendering
     witness — **carried** (Symbols; Schema / config). Search: `workspace basename|workspace key` → security-plan 1
     (`:121`), architecture 2, test-plan 1.
  3. test-plan §6 "pinned byte-equal" → a sha256 digest pin per committed evidence file, grading over the file's content;
     §7 fixture wording; the dev-test stack gains `sha2` (the 2026-06-17 `tokio-stream` precedent) — **carried**
     (Dependencies; Schema / config). Search: `byte-equal` → test-plan 2, security-plan 1; `sha2` as the crate name → 0 in all seven
     (the one raw `sha2` hit, security-plan `:335`, is the substring of `sha256`).
  4. architecture §Stack Hashing/digest: `sha2` as a test-only second hashing dependency; §Occupied Resources, the
     posture-contract row: the 2026-09-30 series — **carried** (Dependencies; Spec-master edits bullet names the contract).
     Search: `blake3` → architecture 3, security-plan 3; `pulse-real-model-leg-posture` → architecture 1, test-plan 3.
  5. a11y-plan §9/§11: the routine arm's responsive-probe wait and the standing stall arm, with the run's configuration —
     **carried** (Symbols; Counts; Dev-tool versions; Spec claims disproved 1 — the stall form moved onto the probe, with
     both measured halves, relay §1). Search: `Page/Frame is not ready` → a11y-plan 1 (`:424`); `12 passing` → a11y-plan 3.
- **Coverage of new surfaces:**
  - `mask_workspace_key` / the capture's key mask → validation: key from the guarded canonicalized data-dir path✓ ·
    instrumentation n/a (test binary) · PII redacted✓ (placeholder; the witness prints a class only) · tests unit (9 arms)✓ ·
    a11y n/a · tokens n/a
  - `pulse-report workspace rendering:` capture line → validation n/a · instrumentation n/a · PII redacted✓ (enum only) ·
    tests unit✓ · a11y n/a · tokens n/a
  - digest pins (`check_digest` / `pinned`) → validation sha256✓ · PII: no corpus text in test source✓ (source arm, inverse
    control red→green) · tests unit (tamper arm)✓ · a11y n/a · tokens n/a
  - `untilResponsive()` + the stall spec → validation n/a · instrumentation n/a · PII n/a · tests e2e✓ (both halves) ·
    a11y WCAG (axe) ✓ · tokens n/a

## Deviations from intent
- **The stall arm's form (plan step 9).** The plan's `setTimeout` busy loop did not discriminate (green with the wait
  bypassed). Replaced by a one-shot `document.readyState` getter that blocks 1 500 ms on the probe's own read; the
  inverse control then read RED (`Page/Frame is not ready`) bypassed and GREEN restored (twice). Justification: the
  operator's witness requirement ("cannot pass vacuously") is the acceptance; the plan's form could not meet it.
- **The rendering witness matches a path's last component**, not the value alone — Pulse's key is a path (Spec claims
  disproved 2). The classifier lives in `real_model_common` (not inline in `real_model_live.rs`) so the default-suite
  harvest tests it.
- **The MCP sidecar was rebuilt too** (the overseer named `-p pulse-app`): it renders the graded report through the
  scrubber, so a pulse-app-only rebuild would have graded reports through the old one. Proven by content like pulse-app.
- **`pulse-app` was started and stopped by the agent**, on the overseer's word (contract (d) had kept it the operator's;
  the new contract section records the change).
- **A synthetic run_id** in the `elide_fingerprints` arm equalled b2's real run_id, which the no-capture-text probe greps;
  changed to `2026-01-01T00-00-00-000`.
- **Harvest interpretations:** the source arm reads "report sections" as the first `## ` line through
  `-- end of report sections --`, lines ≥ 40 chars, with a checked-count guard (> 0); the tamper arm checks capture
  lines ≥ 16 chars trimmed. The 2026-09-30 ledger is not held to the fingerprint elision (it carries digests by design).
- **The synthetic host-path control** `hp-leak.txt` (phase P5) moved out of the tree on the overseer's word (hygiene
  refused it); `hp-leak.MOVED.md` names it, its sha256 `27f41469…`, its five forms in words, its new home.
- Scope record: none — `gate.py scope` clean (changed 8 · listed 8 · recorded 0), at implement P4 and at this wrap's P1.

## Decisions & corrections
- Founder, 2026-09-30 (relay §1): `v3-09`'s remaining cause is fixed in **Pulse 0.3.0** ("Real-model incident surfacing");
  a third pre-registered series follows, BLOCKED-ON that entry being committed and pushed (relay §2).
- Overseer, 2026-09-30 (founder-delegated): the series design (three identical drives on one fresh letters-only dir);
  the slot for d1-d3 back to back without asking between drives; the agent launches and stops `pulse-app`; rebuild
  pulse-app from `fcc31b2` and prove it by content before d1; the operator installs msedgedriver 154; the synthetic
  leak control is never committed but stays traceable.
- Measured: a timer-scheduled stall is absorbed by the `execute` that schedules it — a stall meant to meet a later
  WebDriver call must bind to that call's own evaluation.
- Measured: Pulse's workspace key is a path; a "the key is the basename" premise holds only under a fallback launch.
- Corrected in-session: a ledger tally sentence stated "eight digests" (true count ten: nine canary + two scenario,
  from Pulse's log); a ledger assertion that could not fail was dropped rather than kept.
- Sweep hazards: a grep over the `--e2e` log for `Page/Frame is not ready|✖|passing` matched the INJECTED axe library
  source (hits in library code, not the spec result) — key on the `[webview2 … #0-0]` spec-line prefix; a probe carrying
  a backslash pair is refused by the Bash guard — put it in a file; `grep -c` over a clipped combined search was avoided.

## Outcome
Acceptance criteria (re-asserted against the diff):
- (security) no committed-capture text in test source — **met**: the token probe reads 0 (exit 1); the source arm holds
  (inverse control: a planted b2 line → red, removed → green); every pin is sha256, no MD5/SHA-1.
- (tests) both harvest runners green; grading reads the file after its digest; the tamper arm names the file, not the
  text; the elided-copy test holds; the frozen 2026-09-22 file byte-unchanged over `9785405` — **met** (90/90 ×2; the frozen
  probe prints nothing).
- (security) the graded 2026-09-23 capture carries no fingerprint-shaped token; the frozen original keeps its prefix as a
  stated residual — **met** (residual carried to Expected amendment 1).
- (security) the key mask proven by the mask arms; `capture_paths_guard` green; `clippy --features live-pulse` green; the
  captures' leaf count 0 — **met** (9 arms; probe 0 at exit 1).
- (a11y) the race reproduced, then shown unable to recur, both runs recorded with configuration; `[a11y] verdict asserted —
  0 failed` with `(expected 2)`; no retries/sleep/runOnly/skip; `withTags(WCAG_TAGS)` unchanged — **met, via a deviation**:
  the reproducing arm is the probe-bound stall (the plan's form did not reproduce).
- (a11y) CI's a11y job green on the pushed pre-CI commit, run id and configuration named — **met**: CI#36681853843,
  `windows-2022` 20260920.314.1, 131.0.2903.86/131.0.2903.86, 13 passing / 2 skipped.
- (arch) the no-surface diff probe prints nothing — **met**.
- (security) package count 562; `cargo audit` + `cargo deny` exit 0 after a clean advisory-db probe — **met** (1277
  advisories · 562 crates · 7 allowed warnings; deny advisories/bans/licenses/sources ok).
- (arch) the new series fixed in the contract add-only before d1; no Rust reader, no handle, no verb;
  `the_2026_09_30_series_rule_was_fixed_before_d1` holds — **met** (digest `0091fe6f…` recorded before d1).
- (tests) the series ran as pre-registered — exactly d1, d2, d3, each after gate B, the slot and the non-priming probe;
  each `rule_record` equals the current rule; one ledger row each; no re-fire, no fourth drive — **met**.
- (obs) each drive's envelope carries the eleven keys, `verdict` null, `state` ManualCheck or Blocked; no new span —
  **met** (d1 Blocked, d2/d3 ManualCheck; harvest test).
- (tests) `verification-matrix.json#v3-09` graded by the pre-stated rule over the three digest-pinned captures — **graded:
  NOT MET** (0 graded drives: d1 canary-blocked; d2, d3 scenario-side dismissals). Matrix-linked → P7.3: per the founder's
  word the capability returns to the pool for the third series (never flipped `verified`).

Gates (implement's final block, then the operator pass):
- `grep -c "^name = " Cargo.lock` — green (562) · `cargo nextest … real_model_harvest` — green (90) · `cargo test …
  real_model_harvest` — green (90) · `cargo nextest … capture_paths_guard` — green · `cargo clippy -p conductor-run --features
  live-pulse …` — green · `cargo nextest run --workspace --profile ci` — green (1136) · `cargo test --workspace --doc` —
  green · `cargo clippy --workspace …` — green · `cargo fmt --all --check` — green · the no-capture-text `grep` — green
  (0, exit 1) · the no-surface `git diff --numstat 9785405 …` — green (no output) · the frozen-chunks `git diff --numstat
  9785405 …` — green (no output) · `npm … typecheck:e2e` — green · `CONDUCTOR_A11Y_STRICT=1 … run --e2e` — green (13 / 2,
  verdict asserted; earlier excluded while the 152/154 pair was incoherent) · the advisory-db porcelain — green · `cargo
  audit` — green · `cargo deny …` — green.
- `leg = 'operator'` entries, fired by hand and recorded in `evidence/attempt-ledger.md`: gate B part 1 (1 scrubber commit,
  `fcc31b2`), part 2 (0 unpushed) · the `pulse-legs` listing (leaf absent) · the preconditions probe ×3 (exit 0, atom held
  each time) · drives d1-d3 (exit 0; both `[live] leg rm:` atoms held each time) · `status 2026-09-30T05-51-57-039` (exit
  0, atom held) · the evidence host-path probe — green (0) · the captures' key probe — green (0) · `gate.py hygiene` —
  refused once (the phase's synthetic `hp-leak.txt`), clean after the operator-directed move · the push — `PUSHED_SHA=b8e7bca…` ·
  `ci.py conclusion` — `verdict: green` (CI#36681853843).
- Smoke: the status smoke on d3's minted run_id (above); no boot-path change.
Watches: none folded.
Outcome basis: this session's conversation (implement + the operator pass ran in this window); the operator pass's one
commit `b8e7bca` and its CI run CI#36681853843 are recorded in `evidence/race-witness.md`.
Process hygiene (measured against the host list at each step, last at 06:12Z and after the push):

| Process | Started by | Final state |
|---|---|---|
| `pulse-app.exe` PID 31216 + 10 descendants (`msedgewebview2`, conhost) | this run (overseer word) | terminated (forced after `CloseMainWindow` did not exit in 15 s); `:4317`/`:4318` released |
| `conductor` / `andromeda-pulse-mcp` / capture test per drive | the legs | terminated (none survived any drive) |
| `msedgedriver` / `conductor-tauri` / `tauri-driver` / `node` per `--e2e` run (×6) | the legs | terminated (census equal to baseline after each) |
| six `msedgewebview2.exe` created 2026-09-26 | not this run | left running — not this chunk's |
