# Codebase Research — 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir

## Scope
- **Depth:** deep (mature test harness; five work items across Rust test code, a TS e2e spec, a committed contract and
  a CI run) · **Reads:** 16 · **Globs/Greps:** 27
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, all 28 Session Additions
  (loaded by path when `crates/**/tests/**` was read); `.claude/rules/testing.md` — read in full, all 48 Session
  Additions; `.claude/rules/host-win32.md` — always loaded, 15 additions. Recipe items applied below: 2026-08-18
  (one parent `%TEMP%/pulse-legs/`), 2026-08-16 (fresh dir only WITH a `pulse-app` restart; data-dir equality),
  2026-08-20 (sidecar on `PATH` in POSIX form; a ~0 s `[BLOCKED]` is resolution), 2026-08-13 (Conductor's own
  `ANDROMEDA_PULSE_MCP_ENABLED=true`), 2026-09-23 (a dismissal logs nothing: parse `ok` with no
  `interpretation.incident.created`), 2026-09-02 (process census before/after, a stop form beside the firing form),
  2026-09-10 (a live leg's `expect` atoms read from a recorded log), testing.md 2026-09-12 (restore the old
  implementation to prove an arm discriminates), 2026-09-23 (buffer and write once).
- **Platform issues consulted:** `gh search issues "Page/Frame is not ready" --repo dequelabs/axe-core-npm` → one
  hit, #1209 (open): a `@axe-core/puppeteer` 4.10 user gets `Page/Frame not ready` on some sites, and
  `setLegacyMode(true)` works around it. The fetched issue names no timing budget and no WebDriver cause. It is the
  same string, but not evidence about this race. `gh search issues "FRAME_LOAD_TIMEOUT"` → 0 hits.

## Files inspected
- `crates/conductor-run/tests/real_model_harvest.rs` (1-110, 1440-1815) — the rule section and the grading
  functions (`grade` `:105`, `row` `:93`, `route` `:232`, `structure`/`steps`/`retrieval`, `canary_attempts`). The
  pinned 2026-09-23 capture is the `PINNED_CAPTURE` literal at `:1455-1487`. **The harvest ALREADY reads committed
  evidence at test time**: `committed_capture()` `:1493-1500` and `committed_series_capture()` `:1616-1624`, both
  `CARGO_MANIFEST_DIR`-anchored with `\r\n → \n` normalization. The literals are duplicates, held equal to the files
  by `pinned_literals_equal_the_committed_capture` `:1598` and `each_series_block_equals_its_committed_capture`
  `:1640`. Grading reads the LITERALS (`:1503-1576`, `:1733-1813`), not the files.
- `crates/conductor-run/tests/real_model_series/mod.rs` (1-60 + the `Drive` index) — `Drive {label, file, block}`
  ×6, `EVIDENCE` = the 2026-09-29 chunk's `evidence` dir. The file is 78 879 B
  (`wc -c crates/conductor-run/tests/real_model_series/mod.rs`) and carries every block verbatim, b2's report body
  included (`:636-801`).
- `crates/conductor-run/tests/real_model_common/mod.rs` (full, 243 lines) — `rule_section` `:15`. Its doc says the
  comparison is "never a hash, which would need a dependency", which is stale once `sha2` is a dev-dependency.
  Also `canary_attempts` `:66`, `elide_fingerprints` `:149` (≥8 lowercase hex holding a letter, alnum-bounded),
  `sweep_window` `:187` and `mask_host_paths` `:200`.
- `crates/conductor-run/tests/real_model_live.rs` (60-175, 425-565, 711-731) — every printed line goes through
  `emit`/`emit_block` → `elide_fingerprints(mask_host_paths(redact_value(..)))` `:150-160`, buffered and written
  once by `FlushOnDrop` `:133-147`. `report_sections` `:477` prints Pulse's markdown from its first `## ` VERBATIM,
  including `## Project Context`. `print_pulse_witnesses` `:515` reads `app.boot.workspace_key`
  `workspace_root_basename` fields-only and prints only a boolean `:548-564`. `pulse_log()` `:711-731` resolves
  `ANDROMEDA_PULSE_DATA_DIR` through `capture_paths::pulse_logs_dir_from`.
- `crates/conductor-run/tests/capture_paths/mod.rs` — `pulse_logs_dir_from(value) -> Result<PathBuf, String>`
  (`:37`) canonicalizes the data-dir value and requires a directory before joining `logs/`. Its negative cases live
  in `capture_paths_guard.rs` (5 call sites, graph `:13-83`).
- `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/rm-capture-b2.txt`
  (grep) — `:453 ## Project Context`, `:455 workspace=[redacted: credit_card]`, `:459 - incident #5 @ … — Credit
  Card Issue ([redacted: credit_card])`. The key reaches the report body at TWO sites: the Project Context
  `workspace=` value and every `## Previously Seen` entry's `({workspace})` suffix.
- `…/2026-09-29-…/evidence/attempt-ledger.md:18-32` and `report.md:31` — the key is the data dir's BASENAME
  (`rm-20260923-093840`). The scrubber matched its digit run.
- `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/evidence/rm-capture.txt:256-257` — the two storm
  lines carry `fingerprint_hex=<fingerprint>`, which is fingerprint-shaped under `elide_fingerprints` (8 lowercase hex,
  letters present).
- `contracts/pulse-real-model-leg-posture.md` §The drive series (`:213-321`) — the 2026-09-29 series:
  pre-registered, pass condition (a), decision rule (b), quiet window (c), launch posture per stage (d), slot (e),
  further grades (f). An add-only dated correction is already present (`:307-313`). File is 30 038 B.
- `crates/conductor-run/Cargo.toml` `[dev-dependencies]` — no hashing crate.
- `Cargo.lock` — `sha2 0.10.9` is locked, pulled by `tauri-codegen` and `wry` (package scan). `grep -c '^name = '
  Cargo.lock` → 562. A dev-dependency on `sha2 = "0.10"` adds NO package, only an edge line (the security.md
  2026-09-02 basis).
- `crates/conductor-core/tests/secret_scan_gate.rs:20-45` — the content rules are key-prefix and assignment shaped.
  None matches a bare 64-hex digest, and `Cargo.lock` already carries 562 `checksum = "<64 hex>"` lines with the
  gate green.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts:95-115, 270-292` — `axeFindings()` `:107-113` is the
  ONLY `AxeBuilder` site (`grep -rn AxeBuilder crates/conductor-tauri/ui/test/` → 1 construction). The failing spec
  `:278-281` is the routine arm's FIRST `it`.
- `crates/conductor-tauri/ui/wdio.conf.ts:334-350, 410-431` — `wdio:enforceWebDriverClassic: true` `:340`. The
  `before` hook waits for `readyState === 'complete' && #root.firstChild`, swallowing a thrown poll, 30 s bound.
- `crates/conductor-tauri/ui/node_modules/@axe-core/webdriverio/dist/index.mjs` 4.12.1 — `FRAME_LOAD_TIMEOUT = 1e3`
  (`:15`). `assertFrameReady` (`:96-113`) races `client.execute(() => document.readyState === "complete")` against a
  1 000 ms `setTimeout` reject. The whole block is `try { … } catch { throw new Error("Page/Frame is not ready") }`,
  so a timeout, a thrown execute and a `false` all print the SAME string. `axeSourceInject` `:80` calls it on every
  `analyze()`.
- CI#36635281444, job 109634300221 log (fetched with `gh api …/jobs/109634300221/logs`, 69 653 lines; timeline by
  a scratch parser over the wdio `[0-0]` lines) — see the mechanism below.

## Graph impact (rust plane, `db_state: fresh`, trace `tree-query-…json`, rows 29)
- **`committed_capture`** — 2 callers (`real_model_harvest.rs:1585`, `:1599`). **`committed_series_capture`** — 4
  (`:1642`, `:1653`, `:1667`, `:1670`). **`capture_block`** — 1 (`:1643`). All are harvest-internal: repointing them
  touches one file.
- **`mask_host_paths`** — 4 refs (`real_model_harvest.rs:53` import, `:1132`, `:1144`, `:1158` arms).
  **`elide_fingerprints`** — 7 refs, all harvest. **`rule_section`** — 6 refs, all harvest.
- **Index gap, not a leaf:** the graph holds NO row for `real_model_live.rs` (the `live-pulse` feature gate).
  `grep -n` finds its uses of `rule_section` `:67`, `mask_host_paths`/`elide_fingerprints` `:41`/`:159` and
  `pulse_logs_dir_from` `:713`. Callers there are read by grep, never by the graph.
- **`pulse_logs_dir_from`** — 5 graph refs, all `capture_paths_guard.rs`, plus the grep-only `real_model_live.rs:713`.
- Crate edges: none move. Every changed file is a `conductor-run` test target, a TS e2e spec, or a document.

## Patterns detected
- **The race's mechanism, measured at the run** (the equality the fix needs): axe's `assertFrameReady` fails iff its
  `execute(readyState)` round-trip exceeds `FRAME_LOAD_TIMEOUT = 1000 ms`. In the failing run:
  - the `before` hook's first poll POSTed at `21:49:58.530Z` → `RESULT false` `58.538`;
  - the second poll POSTed at `59.040` → `RESULT true` at `59.640`, a **600 ms** round-trip for a one-line
    `readyState` read;
  - the first spec's `assertFrameReady` probe POSTed at **`21:49:59.654Z`** → `RESULT true` at
    **`21:50:00.827Z` = 1 173 ms**, 173 ms past the budget. The page WAS ready. The answer arrived after the
    library's 1 s timer had already rejected. Every later execute in the run answers in 5-25 ms (`:733-764`).

  So the webview's main thread is busy for ~1-2 s right after `#root` first mounts, and a readiness probe with a
  1 s wall-clock budget races that busy window. Green at `9da18e1` (CI#36632527433) and red at `9785405` over a
  docs-only diff is this race landing on different sides of 1 s.
- **The `before` hook already names this error class** (`wdio.conf.ts:411-414`): "without this every spec fails on
  an empty document ('Page/Frame is not ready' …)". It waits for mount, not for the main thread to go idle after
  mount. That is the gap.
- **Literal pins duplicate committed files** (`real_model_harvest.rs:1598`, `:1640`). The harvest already has the
  file readers, so grading can move onto the files with no new reader shape.
- **Scrub pipeline** — `redact_value` → `mask_host_paths` → `elide_fingerprints`, placeholders `<host-path>` /
  `<fingerprint>` (`real_model_common/mod.rs:144-216`). A key mask is a fourth stage in `emit_block` with a fixed
  placeholder.
- **Series pre-registration** (contract `:213-321`): rule text committed before the first drive, a drive count
  fixed in advance, `pipeline-fault` the only re-fire class, one attempt-ledger row per drive, each drive's
  `rule_record` in its capture ahead of the leg (`each_series_drive_recorded_the_current_rule_before_it_fired`
  `:1648`).

## Conventions to follow
- **Committed-file reader**: `Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(...)` +
  `.replace("\r\n", "\n")` (`real_model_harvest.rs:1616-1624`). Read faults are `expect`-messaged with NO path.
- **Test-binary reader of a data-dir value goes through the guard** (`capture_paths::pulse_logs_dir_from`,
  `real_model_live.rs:713`). A key derived from the data dir takes its basename from the guarded path, never from a
  raw `std::env::var`.
- **Buffered single write** (`real_model_live.rs:133-160`, testing.md 2026-09-23).
- **Rule markers are inviolate** — every drive's recorded rule must equal the current one (`:1648-1661`), so no edit
  may land between `// ---- rule: begin ----` and `// ---- rule: end ----`.
- **Readiness waits are explicit signals with a bound and a named message** (`wdio.conf.ts:415-431`; test-plan §11
  E2E; testing.md §Patterns "never `sleep(N)`").
- **Negative arm proves a gate can fail**, built at test time where possible (test-plan §6 coverage gate; the
  secret-scan gate's in-suite plant `secret_scan_gate.rs:335`).

## New files to create
- `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/rm-capture-2026-09-22-elided.txt` — the 2026-09-22 capture passed once through `elide_fingerprints`, otherwise byte-identical
- `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/attempt-ledger.md` — one row per drive of the new series
- `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/race-witness.md` — the a11y race's reproduction and cannot-recur record, with its configuration
- derived `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/rm-capture-*.txt` by `bash scripts/agent-run.sh run --live real-model` — one capture per drive of the new series, copied from `runs/live-suite/rm-capture.txt`

## Files to modify
- `crates/conductor-run/Cargo.toml` — `sha2 = "0.10"` under `[dev-dependencies]` (already locked; no new package)
- `Cargo.lock` — the one `conductor-run` dependency-list edge line `sha2` adds
- `crates/conductor-run/tests/real_model_harvest.rs` — grade from the committed files; sha256 digest pins; `PINNED_CAPTURE` literal removed; tamper negative arm; key-mask arms; the new series' pins and verdict test
- `crates/conductor-run/tests/real_model_series/mod.rs` — each `block` literal becomes a sha256 digest of its committed file; the new series' drives appended
- `crates/conductor-run/tests/real_model_common/mod.rs` — `mask_workspace_key` (a fourth scrub stage) and the stale "never a hash" doc line
- `crates/conductor-run/tests/real_model_live.rs` — `emit_block` applies the key mask, with the key taken from the guarded data-dir basename
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` — `axeFindings()` waits on the webview's settle signal before `analyze()`; a standing stall arm
- `contracts/pulse-real-model-leg-posture.md` — the new series' design, decision rule and launch posture, add-only, before the first drive

## Sweep records
- `real_model_series` over `crates/ scripts/ .github/`: 1 hit (`real_model_harvest.rs`) · 1 changed.
- `PINNED_CAPTURE` over the same: 1 hit · 1 changed.
- `2026-09-22-interpretation-proven-live` over the same: 1 hit (`real_model_harvest.rs:1495`) · 1 changed (repointed
  to this chunk's elided copy).
- `rm-capture` over the same: 5 files · 3 changed (harvest, live, series) · 2 no-change (`scripts/agent-run.{sh,ps1}`
  clear and write `rm-capture.{txt,err}` under the run dir; the file names are unchanged).
- `AxeBuilder` over `crates/conductor-tauri/ui/test/`: 1 construction site · 1 changed.

## Scope premise closure (scope.md `[inferred]` bullets)
1. Item 1, "grading keeps coverage by reading the committed file after its digest matches; is a test-time read the
   idiom?" — **VERIFIED**: both readers already exist (`:1493`, `:1616`). What moves is only which operand the
   grading reads.
2. Item 2, "the key is known to the capture at write time; mask raw and scrubber-rendered" — **VERIFIED, with a
   correction to the form**: the key is the data dir's basename (ledger `:22-24`, `report.md:31`), derivable from
   the guarded `pulse_logs_dir_from` path's parent. A scrubber-rendered key carries NO key text
   (`[redacted: credit_card]` replaces it whole), so "mask the scrubber-rendered form" is corrected to: mask the RAW
   basename wherever it appears, AND the `workspace=` value of `## Project Context` whatever it holds, so a
   rendering the capture cannot predict still prints as the placeholder. The `({workspace})` suffix of
   `## Previously Seen` entries is covered by the raw-basename mask.
3. Item 3, "(a) digest route over the frozen file vs (b) an elided copy under this chunk" — **resolved to (b) by the
   artifacts**: security-plan `:335` names the remedy "that prefix elided". Route (a) leaves the only graded copy
   un-elided. The frozen file itself keeps the prefix (entry: never an edit to frozen evidence), which is a stated
   residual for the wrap's BREACH-line amendment.
4. Item 4, "nondeterministic, not a regression" — **VERIFIED against THAT RUN** (the runner-only class):
   CI#36635281444 · `9785405` · A11y gate (routine arm), failing at `21:50:00.654Z`, with the mechanism measured
   above. Diff over `9da18e1`: `git diff --name-only 9da18e1 9785405` shows no code, UI or workflow path.
5. Item 4, "the BiDi `Page/Frame is not ready` mechanism on record" — **FALSIFIED as the cause**
   `[premise-corrected: the session is enforced-classic (wdio.conf.ts:340), assertFrameReady took its non-BiDi arm
   (the run's executes POST /execute/sync), and the probe RETURNED true at 1 173 ms; the string is the library's
   catch-all for a missed 1 000 ms budget, not the BiDi evaluation failure]`.
6. Item 5, "letters-only name matches no scrubber pattern" — **VERIFIED at Pulse `a08ae29`**: every numeric pattern
   (`credit_card` `\b(?:\d[ \-]?){13,19}\b`, `ssn` `\b\d{3}[ \-]?\d{2}[ \-]?\d{4}\b`) needs a digit run
   (`git show a08ae29:crates/security/src/scrubber.rs`, `:116-127`). A leaf with no digit at all cannot match them.
   The provider-key and email patterns need prefixes (`sk_`, `ghp_`, `AKIA`, `xox`, `AIza`) or an `@`, which a
   plain lowercase-hyphen leaf avoids.
7. Item 5, "the contract gains the new series add-only" — **VERIFIED**: `## The drive series` exists at `:213`, with
   the add-only dated-correction idiom at `:307`.
8. Causal claim "the credit_card pattern matched the workspace key" — **VERIFIED**: `rm-20260923-093840` is 14 digits
   with one separator, inside the `{13,19}` window. `rm-capture-b2.txt:455` shows the rendering.
9. Causal claim "the model diagnosed exactly that" — **kept as the ledger states it**: b2's incident title is
   "Credit Card Issue" (`rm-capture-b2.txt:459`). Whether the redaction CAUSED `NotIdentified` is what the new
   series tests; nothing here settles it.

## Open questions
- The a11y fix's shape: which settle signal `axeFindings()` waits on (an idle-callback round-trip, a
  readiness-probe round-trip held under axe's own budget, or a vendored patch of `FRAME_LOAD_TIMEOUT`), and whether
  a bounded signal wait before the FIRST analysis falls under the no-retry ban → blocks: plan-decision
- The new series' fixed drive count and decision rule (the operator's pre-registration) → blocks: plan-decision
- Whether the series runs on one fresh clean-named dir for every drive, or on a fresh dir per drive (P-044's witness
  needs a prior same-scope incident, which a fresh dir holds only from its second drive) → blocks: plan-decision
