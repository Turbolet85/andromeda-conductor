# Report — 2026-10-08-capture-canary-pairing-window-corrective

**Chunk:** the real-model capture's canary pairing reads a selected digest's inference across the emission instant,
pinned on the sixth series' recorded shapes; no live drive, nothing recorded re-rendered
**Date:** 2026-10-08
**Commits:** `9b4a0b8` chore(2026-10-08-capture-canary-pairing-window-corrective): operator pre-CI commit, for the
run this chunk's verdict reads (basis: `git log --format='%h %s' 39e197b..HEAD`, 1 row; `39e197b` is the parent of
the oldest pre-CI commit)

**The limit of the proof** (stated wherever this report describes the fix): the fix is proven on in-test lines
built from the 2026-10-07 sixth series' recorded stamps and line kinds, with a failing-first reading. What the
capture binary prints on a next real drive is unmeasured until someone drives it. A canary tick stamped at or
after the emission instant still gets no line (the d3 shape). No drive, no GPU and no `pulse-app` launch happened
in this chunk.

## Changes (structured — detectors read this)
- **Files:** (basis: `git diff --name-only 39e197b`, the pipeline's own folders left out; `gate.py scope` reads
  `changed 4 · listed 4`)
  - `crates/conductor-run/tests/real_model_common/mod.rs` — test-support module shared by the gated capture and
    the default-suite harvest; 46 lines added in 4 ranges (71-80 · 83-86 · 103-105 · 169-197).
  - `crates/conductor-run/tests/real_model_live.rs` — the feature-gated capture (`#![cfg(feature = "live-pulse")]`);
    5 lines added in 2 ranges (41 · 721-724), its local stamp parser and its local filter removed.
  - `crates/conductor-run/tests/real_model_grading/canary_pairing.rs` — a child module of the default-suite
    `real_model_harvest` target; 263 lines added in 3 ranges (53 · 232-490 · 493-495).
  - `contracts/pulse-real-model-leg-posture.md` — 12 lines added in 1 range (321-332), 0 removed.
  - New, in the chunk folder: `evidence/failing-first.md`, `evidence/operator-pass.md`.
- **Symbols / APIs:** all in test code under `crates/conductor-run/tests/`; no shipped crate's API moved.
  - `canary_attempts(lines: &[&Value], emission_ms: Option<i64>) -> Vec<String>` in `real_model_common/mod.rs`
    (was `canary_attempts(lines: &[&Value])`). The instant now SELECTS cue-bearing ticks: a tick prints its
    `canary:` line, or is counted among the other kinds, only when its own stamp parses and is strictly earlier
    than the instant; with no instant every tick is selected. Every tick, selected or not, still bounds a segment,
    and a selected tick's pairing reads its inference wherever those lines are stamped. Before this chunk the
    capture dropped every line not stamped before the instant and paired what was left. The printed line's field
    set and order are unchanged. Limit: proven on in-test lines built from recorded stamps; the capture binary's
    next print on a real drive is unmeasured; a tick at or after the instant gets no line.
  - Callers of the changed signature, 2 (basis: `grep -rn canary_attempts crates/` at the phase, 10 hits in 5
    files, read; re-read at implement): the capture's canary block (`real_model_live.rs`, added range 721-724)
    and the `paired` helper (`canary_pairing.rs`, added line 53). The harvest's import alias
    (`real_model_harvest.rs`) needed no change, and the rule's own private function of the same name in that file
    is a different function and is untouched.
  - `iso_ms(stamp: &str) -> Option<i64>` — moved from the capture (private) to `real_model_common/mod.rs`
    (`pub`, rows 175-196 @176), body unchanged; the capture imports it (added line 41) and its two other users
    there keep reading it. `stamp_ms` (rows 169-173) is a new private helper beside it.
  - New test helpers in `canary_pairing.rs`: `tier_2_tick` (242-259), `deduped` (261-267), `paired_at` (269-272),
    `d1_shape` (274-292), `d2_shape` (294-316), `d3_shape` (318-351).
  - Six new tests in `canary_pairing.rs`:
    `a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d1` (353-366),
    `…_d2` (368-382), `the_scenario_s_own_storm_digest_never_prints_as_a_canary` (384-406),
    `a_canary_tick_stamped_after_the_instant_gets_no_line_d3` (408-432),
    `a_tick_the_instant_cannot_place_is_never_selected` (434-460),
    `the_pre_fix_window_read_the_same_shapes_as_a_pipeline_fault` (462-489). The existing
    `the_capture_prints_pulse_s_no_incident_outcome` gained one assertion (added range 493-495): the capture's
    source holds the call `canary_attempts(&window, emission_ms)`.
  - No port, socket, env var, endpoint, IPC method or export was added or changed.
- **Crates / modules:** none added, removed or changed. No file under any `crates/*/src/` moved (the gate
  `git diff --quiet 39e197b… -- Cargo.lock Cargo.toml … ':(glob)crates/*/src/**' scripts scenarios .github` exits 0).
  No new test target and no new grading module: the tests joined the existing `canary_pairing.rs`.
- **Dependencies:** none added or bumped. `grep -c '^\[\[package\]\]' Cargo.lock` reads 562, as at the chunk base.
- **Schema / config:** none. No scenario, manifest, envelope key, violation schema or redaction shape moved. The
  capture's `canary:` line keeps its grammar; a selected digest's line still leaves the capture through `emit`
  and its scrub chain.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - The `real_model_harvest` target: 133 tests at the chunk base → 139 (basis: the two
    `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` runs in
    `evidence/failing-first.md` and implement's step-1 run). No master states either figure:
    `grep -noE '\b(133|1233|1239|139) (tests|passed)'` over the seven masters, 0 hits; the same for
    `.andromeda/registries`, `CLAUDE.md`, `.claude/docs`, `.claude/rules`, 0 hits.
  - The workspace run read `1239 tests run: 1239 passed, 0 skipped` (the bundled `bash scripts/agent-run.sh run`
    at implement). Its figure at the chunk base was not measured in this chunk.
  - A documented qualifier moved: the masters say a canary digest ticked just before the instant prints a third
    `canary:` line reading `pipeline-fault` because its prompt assembly lies outside the capture's pairing window
    (`.andromeda/obs-plan.md:221`; `.andromeda/test-plan.md:260` describes the same reading for the sixth series'
    d1 and d2). From this chunk the pairing reads that digest's inference across the instant, so on the recorded
    d1 and d2 shapes it reads `surfaced … parse=ok created=false deduped=true skip_reason=none`. The sixth
    series' committed captures keep the `pipeline-fault` tokens they printed; that sentence stays true as the
    sixth series' measured record. Limit: the new reading is proven on in-test lines built from the recorded
    stamps, never on a drive.
  - The two-line case is unchanged: a canary tick stamped at or after the instant still gets no line.
- **Dev-tool versions:** none.
- **Harness / gate surface:** none. No `agent-run` verb, script, CI step, workflow, status shape or verdict shape
  was added or changed.
- **Cross-project / external claims:**
  - `I1 · ../additional/pc-overseer/relays/conductor-phase-pairing-2026-10-08.md · copy no-repo · unchanged`;
    cited at 15 sites in `scope.md`, `research.md` and `plan.md`.
  - `I2 · message: the operator, the /andromeda-phase invocation arguments, 2026-10-08 · copy message · n/a — a
    message has no live source`; cited at 4 sites.
  - `inputs: 2 entries — unchanged 1 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 ·
    uncited 0 · unparsed 0` (`inputs.py verify` at this wrap's P1). No drift.
  - The wrap's own relay, read first at this wrap and not snapshotted (the inputs channel has no wrap `snap`
    step): `~/dev/projects/additional/pc-overseer/relays/conductor-wrap-pairing-2026-10-08.md`, from the pc
    overseer. It carries: the founder ruled this fix into 0.3.0 by dialog at 10:25 local on 2026-10-08
    ("Починить в 0.3.0"); the limit of the proof is stated wherever the fix is described; mint nothing; one new
    `CARRY` on the version-close entry for the registry row; no model text outside `evidence/`.
  - CI: run `CI#37754365520`, on sha `9b4a0b847361b1a98996dbbb80d65b0da887b9d1`, conclusion success, three jobs
    success (`ci.py conclusion` and `gh run view`, both in `evidence/operator-pass.md`). The verdict was taken on
    that tree; this wrap's commit adds to it.
  - Pulse: the 2026-10-07 sixth series' recorded Pulse log was read in place at the phase, fields only
    (`research.md`, External inputs). This chunk's implement and wrap read no Pulse file and launched no Pulse
    process.
  - The local advisory database copy read clean before and after the audit (`git -C … advisory-db status
    --porcelain`, no output, twice); `cargo audit` loaded 1294 advisories over 562 crates, 7 allowed warnings.
- **Reverted / negative API facts:** none shipped and reverted. One intermediate state existed inside the chunk
  by plan: step 2 moved the pre-fix composition into the shared module unchanged, so the failing-first reading
  could be taken through the same function; step 5 replaced it.
- **Insufficient fixes (written, kept, not the remedy):** none. A canary tick stamped at or after the instant
  (the d3 shape) still gets no line; that is the fix's stated limit, named in the contract block and the function's
  doc comment, and recovering it was a rejected approach of the plan.
- **Spec claims disproved by measurement:**
  - Architecture's registry row for `contracts/pulse-real-model-leg-posture.md` says it is the second `contracts/`
    member with "NO Rust reader" (`.andromeda/architecture.md:184`); architecture's keyed contract for the
    directory structure says the same in its `contracts/` tree row
    (`.andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:30`, "the
    two members no Rust code reads"); and two leaves say it (`CLAUDE.md:14`, `.claude/docs/conventions.md:9`).
    Basis: `grep -rnoE 'NO Rust reader|no Rust code reads'` over the seven masters, `.andromeda/registries`,
    `CLAUDE.md`, `.claude/docs`, `.claude/rules`: 8 hits, of which these 4 are about the posture contract, 1 is
    the P-025 contract's row (`architecture.md:183`) and 3 are about two env handles (`security-plan.md:116`,
    `:328`, `.claude/rules/security.md:18`). [corrected at this wrap's Validate: this bullet first read 7 hits and
    3 sites, from a pattern that required 40 characters after the match and so dropped the key file's hit; the
    architecture detector named the fourth site, and the unbounded grep confirmed it. The detectors read the
    earlier figure; no proposal rests on it.]
    A test binary reads it: `pre_registered` at `crates/conductor-run/tests/real_model_grading/mod.rs:155` opens
    the contract and checks one section's sha256, and it has six callers (`grep -n 'pre_registered('` over
    `real_model_grading/*.rs`: `capture_run_2026_10_07.rs:26`, `series_2026_09_30.rs:23`,
    `series_2026_10_01.rs:25`, `series_2026_10_06.rs:25`, `series_2026_10_07.rs:25`,
    `series_2026_10_07_sixth.rs:26`). No shipped code reads it. This chunk added no reader and did not create the
    mismatch; it was noticed at the phase. Disposition asked for by the operator (the wrap relay, section 3): one
    new `CARRY` on the version-close entry, because correcting the row touches §Occupied Resources at 1 B of
    headroom (38114 of 38115 B), so the close frees bytes there first.
  - The plan's step 4 predicted the summary line `139 tests run: 137 passed, 2 failed` for
    `cargo nextest run -p conductor-run --test real_model_harvest --profile ci`. Measured: that profile cancels at
    the first failure and the call printed `43/139 tests run: 41 passed, 2 failed, 0 skipped`; the predicted line
    appeared only with `--no-fail-fast`. Stated in `plan.md` only (no master carries it); both readings are in
    `evidence/failing-first.md`. The prediction's substance held: the d1 and d2 tests red, every other test green.
- **Expected amendments (from plan):** the search that located both entries' sites is the phase's pairing-window
  pattern `pairing window|canary:. line per|third .canary:. line|before the (scenario.s )?emission instant`,
  re-run at this wrap over the seven masters and `.andromeda/registries`: 5 hits in 2 masters
  (`.andromeda/obs-plan.md:221` ×3, `.andromeda/test-plan.md:260` ×2), 0 in the other five masters, 0 in any
  registry file.
  - obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification (Real-model
    posture) — carried: the Counts / qualifiers bullet above (the qualifier that moved). A dated note beside the
    sixth series' sentence at `.andromeda/obs-plan.md:221`: from this chunk a canary digest ticked just before the
    instant prints its log-borne outcome; the `pipeline-fault` reading stays as the sixth series' measured record;
    the two-line case and the read-from-Pulse's-log rule stand; the limit of the proof beside it.
  - test-plan §6 Scenario: Fingerprint-storm (Real-model interpretation leg) — carried: the Symbols / APIs bullet
    (the six default-suite tests that pin the corrective) and the Counts / qualifiers bullet. A dated record of
    the corrective and of the default-suite arms that pin it, beside the sixth series' sentence at
    `.andromeda/test-plan.md:260`, which stays as written; no test or capture count enters the body; the limit of
    the proof beside it.
- **Coverage of new surfaces:**
  - `canary_attempts`' tick selection by the emission instant (test-support code, printed by the gated capture) →
    validation {mechanism✓ — a stamp that does not parse, or one equal to the instant, is never selected; held by
    `a_tick_the_instant_cannot_place_is_never_selected`} · instrumentation {n/a — no span, log line or field; the
    line goes through the capture's existing `emit`} · PII {redacted✓ — closed-enum fields and a stamp only,
    through `emit`'s scrub chain, unchanged} · tests {unit — six new default-suite tests; the gated capture's call
    is proven by compile, lint and a source literal, never by a run} · a11y {n/a} · tokens {n/a}

## Deviations from intent
Three, all reported by implement and accepted by the operator ("Your three deviations are accepted" — the operator,
2026-10-08, given in this session at the operator-pass go; the pc overseer's wrap relay, section 1, says the same).
1. Step 4's listed command printed `43/139 tests run: 41 passed, 2 failed` under the fail-fast `ci` profile, not
   the predicted 139-test line. It was re-run once with `--no-fail-fast` to measure every test, and both readings
   are recorded. Justification: the step's stop rule (any other test red) could not be read from a cancelled run.
2. In the bundled `bash scripts/agent-run.sh run`, the `--features live-pulse` clippy line was a cache hit. The
   compile and lint of the edited capture happened in implement's own post-fix call of the same command (exit 0,
   `Checking conductor-run`); no source changed between the two. CI's `rust` job then ran the bundled default on a
   clean checkout of `9b4a0b8` and concluded success.
3. `the_scenario_s_own_storm_digest_never_prints_as_a_canary` also asserts that with no instant the scenario's
   digest does print, as `surfaced … created=true`. It is inside the plan's wording for that test ("although that
   digest parsed `ok` and created an incident") and makes the test able to fail.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 4 · listed 4 · recorded 0 … ·
excluded 48`, at this wrap's P1; the same at implement's P4).

## Decisions & corrections
- The founder ruled this fix into 0.3.0 by dialog on 2026-10-08 ("Починить в 0.3.0", relayed verbatim by the pc
  overseer); the overseer's own lean had been to leave and record it (the wrap relay, section 2).
- The operator's directions for this wrap (the invocation arguments and the wrap relay): the limit of the proof is
  stated wherever the fix is described; mint nothing; one new `CARRY` on the version-close entry for the registry
  row; no model text outside `evidence/`; the whole wrap in this window.
- The operator pass was performed by the agent on the operator's explicit word, entries named; the commit and the
  push are the operator's acts made on that word (`evidence/operator-pass.md`).
- A hazard found this chunk: `cargo nextest run --profile ci` is fail-fast here, so a plan that predicts the
  summary line of a RED run under it predicts a line the run does not print. A failing-first prediction names
  `--no-fail-fast`, or predicts only the failing names.
- A sweep hazard carried from research: the pairing-window pattern does not match the capture's own comment at the
  call, which wraps across lines; it changed with the call and was found by reading the block.
- The hygiene entry's record is written after its read. At this pass the entry was re-read once with the record in
  the set before the commit, and read clean both times.

## Outcome
Acceptance criteria, each re-asserted against the diff (the four changed files and the two evidence files):
- (tests) the harvest target prints `139 tests run: 139 passed, 0 skipped` — MET (implement's gate run; the
  operator's own run reads 139 of 139).
- (tests) a canary tick milliseconds before the instant prints one `surfaced` line ending `parse=ok created=false
  deduped=true skip_reason=none` on the d1 and d2 shapes, and the scenario's digest prints no `canary:` line — MET,
  on in-test lines built from the recorded stamps. Not measured on a drive.
- (tests) failing-first is measured and recorded — MET (`evidence/failing-first.md`; the entry reading it exits 0).
- (obs) a canary tick after the instant gets no line: the d3-shaped test prints two lines and no third — MET.
- (security) a tick the instant cannot place is never selected — MET.
- (tests) `cargo test -p conductor-run` exits 0 — MET (the harvest target reads 139 passed under it).
- (tests) `bash scripts/agent-run.sh run` exits 0, its `live-pulse` clippy line included — MET (deviation 2 names
  how the gated capture's compile was measured).
- (arch) the contract diff is additions only, one hunk, directly after line 320 — MET (`0`, `1`, `1`); the six
  digest-pinned sections keep their digests (the six `…_was_fixed_before_d1` tests are inside the green harvest
  target).
- (security) nothing recorded moves — MET (the `git diff --quiet 39e197b… -- ':(glob)conductor-0.3.0/chunks/**' …`
  entry exits 0).
- (arch) no shipped crate, manifest, lockfile, script, scenario or workflow is edited; 562 packages — MET.
- (security) no added line reads an environment handle; the `canary:` line still leaves through `emit` — MET.
- (security) `cargo audit` and `cargo deny check advisories bans licenses sources` exit 0, the porcelain empty
  before and after — MET.
- (obs) no span name, span attribute, self-obs log line, allowlist entry or envelope key added; no file under any
  `crates/*/src/` changes — MET.
- (arch) §Occupied Resources and §Established Decisions read 38114 B and 38082 B through implement; the plan names
  no architecture amendment — MET.
- (security) this chunk's `evidence/` holds no absolute host path; `gate.py hygiene` reads `hygiene: clean` at the
  operator pass — MET.
- (tests) the pushed HEAD's CI run reads `verdict: green` — MET, run `CI#37754365520` on `9b4a0b8`.

Gates, by `run`, in order (implement's gate call: `entries 21 · green 18 · red 0 · recorded 0 · timeout 0 ·
not-run 3`):
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` (before the audit) — green · exit 0 · no output.
- `cargo fmt --all --check` — green · exit 0.
- `cargo nextest run -p conductor-run --test real_model_harvest --profile ci` — green · exit 0 · contains
  `139 tests run: 139 passed, 0 skipped`.
- `cargo test -p conductor-run` — green · exit 0.
- `F=…/evidence/failing-first.md && grep -q … && grep -q …` — green · exit 0.
- `git diff 39e197b… -- contracts/pulse-real-model-leg-posture.md | grep -v '^--- ' | grep -c '^-'` — green ·
  exit 1 · last line 0.
- `git diff -U0 39e197b… -- contracts/pulse-real-model-leg-posture.md | grep -c '^@@ '` — green · exit 0 · last line 1.
- `git diff -U0 39e197b… -- contracts/pulse-real-model-leg-posture.md | grep -c '^@@ -320,0 +321,'` — green ·
  exit 0 · last line 1.
- `git diff --quiet 39e197b… -- ':(glob)conductor-0.3.0/chunks/**' …` — green · exit 0.
- `git diff --quiet 39e197b… -- Cargo.lock Cargo.toml ':(glob)crates/*/Cargo.toml' ':(glob)crates/*/src/**' scripts
  scenarios .github` — green · exit 0.
- `git diff 39e197b… -- crates/conductor-run/tests | grep -cE '^\+.*(env::var|var_os)'` — green · exit 1 · last line 0.
- `bash scripts/agent-run.sh run` — green · exit 0 (`1239 tests run: 1239 passed, 0 skipped`, the doctests and the
  three clippy lines).
- `grep -c '^\[\[package\]\]' Cargo.lock` — green · exit 0 · last line 562.
- `cargo audit` — green · exit 0 (1294 advisories · 562 crates · 7 allowed warnings).
- `git -C "${CARGO_HOME:-$HOME/.cargo}/advisory-db" status --porcelain` (after the audit) — green · exit 0 · no output.
- `cargo deny check advisories bans licenses sources` — green · exit 0.
- `python -X utf8 scripts/arch-registry-check.py measure --file .andromeda/architecture.md` — green · exit 0 ·
  contains both registry lines (38114 B, 38082 B).
- `ls …/evidence/failing-first.md >/dev/null && cat …/evidence/* | grep -cE "…"` — green · exit 1 · last line 0.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, driven once by
  hand at the operator pass: exit 0, `hygiene: clean` (`evidence/operator-pass.md`).
- `git diff --quiet && git diff --cached --quiet && git push origin HEAD && echo "PUSHED_SHA=$(git rev-parse HEAD)"`
  — `leg = 'operator'`: exit 0, `PUSHED_SHA=9b4a0b847361b1a98996dbbb80d65b0da887b9d1` (`evidence/operator-pass.md`).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 1500` —
  `leg = 'operator'`: exit 0, `9b4a0b847361 verdict: green · checks 3/3 · wall 651 s · runs CI#37754365520
  completed/success` (`evidence/operator-pass.md`).
- No `defer`, no `recorded` entry, no live round. Smoke: skipped — no boot-path / UI-surface change; the plan lists
  no smoke entry and states the absence.

Watches: none folded.

Outcome basis: the operator pass ran, so the verdicts rest on its final state: one commit from the pre-CI parent,
`9b4a0b8` (`git log --format='%h %s' 39e197b..HEAD`), and that HEAD's CI run `CI#37754365520`, both recorded in
`evidence/operator-pass.md`. Implement's P4 report, as given in this conversation, is the basis for the gate
readings, the deviations and the census; the implement conversation is present in this window, so its evolve
records were not read. Beside it: the operator's go for the pass with the operator's own verification, and the pc
overseer's wrap relay.

What the proof does not reach: what the capture binary prints on a next real drive is unmeasured until someone
drives it; the tests prove the capture's reading of recorded shapes, never that Pulse surfaced a digest; a canary
tick stamped at or after the instant (the d3 shape) stays without a line. `v3-09` is untouched: verified on the
sixth series, three drives, not attributed to the Pulse-side remedy. The sixth series' committed captures keep the
`pipeline-fault` tokens they printed.

Process hygiene (implement P4's census, measured with `ps` after the gates):

| Process | Started by | Final state |
|---|---|---|
| `cargo` (nextest, clippy, fmt) foreground calls | this run | terminated |
| `gate.py run` and its 18 entries | this run | terminated |
| `ci.py conclusion` (the operator pass's CI read) | this run, on the operator's word | terminated (its completion was notified) |

No `pulse-app`, sidecar or driver was started by this chunk.
## New text, by line
Generated by `cites.py added` (cites v1.2); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 39e197b1 (the parent of the oldest pre-CI commit 9b4a0b84) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### contracts/pulse-real-model-leg-posture.md — added 12 line(s) in 1 range(s)
added: 321-332
### crates/conductor-run/tests/real_model_common/mod.rs — added 46 line(s) in 4 range(s)
added: 71-80 · 83-86 · 103-105 · 169-197
  - 84-86 «let selected = |tick: &Value| {»
- 169-173 «fn stamp_ms(line: &Value) -> Option<i64> {»
  - 170-172 «line.get("timestamp")»
- 175-196 @176 «pub fn iso_ms(stamp: &str) -> Option<i64> {»
  - 180-184 «let fraction = stamp»
  - 185-187 «let millis = format!("{fraction:0<3}")»
### crates/conductor-run/tests/real_model_grading/canary_pairing.rs — added 263 line(s) in 3 range(s)
added: 53 · 232-490 · 493-495
- 242-259 «fn tier_2_tick(t: &str) -> [serde_json::Value; 2] {»
  - 243-258 «[»
- 261-267 «fn deduped(t: &str) -> serde_json::Value {»
  - 262-266 «log(»
- 269-272 «fn paired_at(lines: &[serde_json::Value], emission_ms: i64) -> Vec<String> {»
- 274-292 @276 «fn d1_shape() -> Vec<serde_json::Value> {»
  - 278-279 «let [scenario, scenario_assemble] =»
  - 280-291 «vec![»
- 294-316 @296 «fn d2_shape() -> Vec<serde_json::Value> {»
  - 299-300 «let [scenario, scenario_assemble] =»
  - 301-315 «vec![»
- 318-351 @320 «fn d3_shape() -> Vec<serde_json::Value> {»
  - 325-326 «let [scenario, scenario_assemble] =»
  - 327-350 «vec![»
- 353-366 @354 «fn a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d1() {»
  - 357-360 «assert!(»
  - 361-364 «assert!(»
- 368-382 @369 «fn a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d2() {»
  - 373-376 «assert!(»
  - 377-380 «assert!(»
- 384-406 @385 «fn the_scenario_s_own_storm_digest_never_prints_as_a_canary() {»
  - 386-405 «for (shape, instant, scenario_tick) in [»
- 408-432 @409 «fn a_canary_tick_stamped_after_the_instant_gets_no_line_d3() {»
  - 412-415 «assert!(»
  - 416-419 «assert!(»
  - 420-423 «assert!(»
  - 424-427 «assert!(»
  - 428-431 «assert_eq!(»
- 434-460 @435 «fn a_tick_the_instant_cannot_place_is_never_selected() {»
  - 439-440 «let [at_instant, at_instant_assemble] =»
  - 441-450 «let lines = [»
  - 456-459 «assert!(»
- 462-489 @463 «fn the_pre_fix_window_read_the_same_shapes_as_a_pipeline_fault() {»
  - 464-488 @466 «for (shape, instant, tick) in [»
### crates/conductor-run/tests/real_model_live.rs — added 5 line(s) in 2 range(s)
added: 41 · 721-724
