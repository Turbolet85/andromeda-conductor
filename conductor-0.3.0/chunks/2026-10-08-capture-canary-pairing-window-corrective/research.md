# Codebase Research — 2026-10-08-capture-canary-pairing-window-corrective

## Scope
- **Depth:** deep on four files · **Reads:** 16 · **Globs/Greps:** 15
- **Harness rules consulted:** none — no live leg in this chunk. (`.claude/rules/testing.md` and
  `.claude/rules/verification-harness.md` loaded on the files read; the entries applied are the 2026-09-09 one on
  an arm that is green on both sides of a change, and the 2026-10-03 extension on compiling every feature-gated
  target after a signature change.)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the overseer relay: the bounds of the founder's pick, the d1 and d2 stamps, the three questions.
  - `inputs#I2` — the invocation directive: stop if only a live drive proves the fix.
  - not snapshotted: the sixth series' recorded Pulse log, `~/.cache/pulse-legs/rm-sixth-series/logs/`, one file
    of 8160899 B. `inputs.py snap` refused it (exit 3, "over the 1048576-byte cap — cite it by its location
    instead"), and raw SUT log text may not enter a committed tree in any case. It was read in place, fields only;
    the read's output is `.andromeda/runs/2026-10-08T08-34-24-phase/pairing-probe.txt` (sha256 `2f4e8e41…5305f`)
    and its script is `pairing-probe-script.txt` beside it.

## Files inspected
- `crates/conductor-run/tests/real_model_common/mod.rs` (1-166) — `canary_attempts` at 73-151: takes the lines it
  is given, finds every cue-bearing tick, pairs each retry-storm tick with the first prompt assembly before the
  next retry-storm tick, reads the outcome between that prompt and the next. It has no notion of the emission
  instant. The module carries `#![allow(dead_code)]`.
- `crates/conductor-run/tests/real_model_live.rs` (1-131, 133-164, 538-731, 773-794) — compiled only under
  `#![cfg(feature = "live-pulse")]` (line 27). The emission instant is the `timestamp_ms` of the leg's
  `timeline.execute` span open (85-88). The canary block (721-730) builds `before` = the window's lines whose
  stamp is strictly earlier than the instant, and prints `canary_attempts(&before)`. The stamp parser `iso_ms`
  (774) is private to this file and has three users here (598, 693, 726). Every printed line leaves through
  `emit` → `emit_block` (150-164): `mask_workspace_key`, `redact_value`, `mask_host_paths`, `elide_fingerprints`.
- `crates/conductor-run/tests/real_model_grading/canary_pairing.rs` (full) — eight default-suite tests over
  in-test Pulse log lines, fields only, through a `paired` helper; the last test holds the capture's source to a
  literal with `include_str!`. This is where the new tests belong.
- `crates/conductor-run/tests/real_model_harvest.rs` (1180-1430, and the marker lines) — the rule's marked span is
  lines 76-464 of this file. The capture-text arm (`capture_text_in`, 1367) matches report-section lines of 40
  characters or more only. `GRADING_MODULES` lists 14 files and a walking test holds it to the directory.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (1-175) — `pre_registered` (155) reads the posture
  contract and checks one section's sha256 against a pin; six callers pin six dated sections.
- `crates/conductor-run/tests/real_model_grading/capture_tokens.rs` (54-95) — holds the capture's and the shared
  module's source to every grammar literal the rule reads, `"pipeline-fault"` included.
- `crates/conductor-run/tests/real_model_grading/series_2026_10_07_sixth.rs` (52-64) — the sixth series' measured
  table records d1 and d2 with one `pipeline_fault` token, as printed.
- `crates/conductor-timeline/src/scheduler.rs` (105-125) — `timeline.execute` is the `#[tracing::instrument]` span
  of `run_timeline_observed`, so it opens at the function's entry, before any emission callback runs.
- `contracts/pulse-real-model-leg-posture.md` (headings; 300-325) — "The canary classification" is a bullet at
  306-320 inside `## The drive series` (213-321); its `[corrected 2026-09-29 …]` note is lines 316-320.
- the sixth series' `evidence/attempt-ledger.md` (224-277) and the `canary` lines of its three captures.
- `.andromeda/obs-plan.md:221` and `.andromeda/test-plan.md:260` — the two master sentences on the pairing window.
- `scripts/agent-run.sh` (lines 324-325) — the bundled `run` lints `conductor-run` under `--features live-pulse`.

## The replay (the control that decides the design)

The pairing was ported line for line to a read-only script and replayed over the recorded Pulse log, once per
drive, three ways. Derivation: `pairing-probe-script.txt` run from the repository root; output `pairing-probe.txt`.

| Drive | Pre-fix composition (filter by stamp, then pair) | Selection kept, pairing over the whole window | Whole window, no selection |
|---|---|---|---|
| d1 | equals the committed capture's four `canary` lines, line for line; third line `pipeline-fault … parse=none` | third line `surfaced … parse=ok created=false deduped=true`; count `0 ()` | 5 lines: the scenario's own digest (tick 06:32:51.361Z) prints as a fourth `canary:` line |
| d2 | equals the committed capture, line for line; third line `pipeline-fault … parse=none` | third line `surfaced … parse=ok created=false deduped=true`; count `0 ()` | 5 lines, and the count reads `3 (error_rate_spike,…)` |
| d3 | equals the committed capture, line for line: two lines, count `4 (…)` | unchanged: two lines, count `4 (…)` | 5 lines, count `6 (…)` |

So the port is faithful (it reproduces all three committed captures), the defect is reproducible from recorded
lines alone, the corrected selection prints what Pulse's log bears out, and dropping the selection would be wrong
in two ways at once.

File order around each instant, from the same output (offsets are from the emission instant):

| Drive | Third canary tick | Its prompt | Scenario's own tick | Canary parse `ok` | Canary outcome line | Next prompt (the scenario's) |
|---|---|---|---|---|---|---|
| d1 | -4 ms | +1 ms | +5006 ms | +5779 ms | +5813 ms, `created=false deduped=true` | +5813 ms, two lines later |
| d2 | -2 ms | +2 ms | +5006 ms | +6325 ms | +6358 ms, `created=false deduped=true` | +6358 ms, two lines later |
| d3 | +11 ms | +15 ms | +5004 ms | +6015 ms | +6048 ms, `created=false deduped=true` | +6048 ms, two lines later |

- **Same-stamp order:** the incident outcome line precedes the next prompt assembly in file order on 4 of 4
  same-millisecond pairs in the three windows (the three above and d2's second, at +12290 ms).
- **Stamp order is not file order everywhere:** d1's window holds 1 adjacent stamp inversion in 6200 line pairs
  (d2 and d3 hold none). The selection therefore reads each tick's own stamp, as it does today, never a cut
  position in the file.
- **d2 carries a tier-2 cue tick 730 ms after the instant** (`error_rate_spike`). It is stamped after the
  instant, so it stays out of the closing count.

## The three questions (`inputs#I1` §3)

1. **What the classifier prints after the fix.** For a canary storm whose tick precedes the instant by
   milliseconds: `canary: surfaced t=… cue_kind=retry_storm cue_priority_tier=autonomous parse=ok created=false
   deduped=true skip_reason=none`, the outcome Pulse's own log bears out on d1 and d2. **How the scenario's own
   digest is told apart:** by the SIGN of its tick's stamp against the instant, never by distance and never by an
   identity Pulse's lines do not carry. The instant is taken at `timeline.execute`'s open, before the scenario's
   first emission, and a tier-1 `retry_storm` digest needs the Autonomous band, which the scenario's storm reaches
   about 5 s after the instant (+5002, +5001, +5000 ms on the three drives). So the scenario's tick is always
   stamped after the instant: it is never selected, it gets no `canary:` line, and as the next retry-storm tick it
   closes the last canary's segment. The rule errs one way only: a canary storm whose tick lands after the
   instant (d3, +11 ms) gets no line, exactly as today. It never prints the scenario's digest as a canary.
2. **The failing-first control.** The new d1- and d2-shaped tests, run against the pre-fix composition, read
   `pipeline-fault` with `parse=none`; the replay's first column is that reading, taken from recorded lines.
   Implement reproduces it in Rust before the change and records it.
3. **Architecture's registries are not grown.** No architecture text describes the pairing window: the sweep
   below finds it in obs-plan and test-plan only, and the sixth series' architecture entry says so in its Kept
   line (`arch-history.md`, last item). So this chunk expects no architecture amendment at all. Measured at
   take-up: §Occupied Resources 38114 B, §Established Decisions 38082 B of 38115 B
   (re-derived: `python -X utf8 scripts/arch-registry-check.py measure --file .andromeda/architecture.md`).

## What the proof does not reach

- The default-suite tests exercise the shared function, selection and pairing together. The gated capture's CALL
  of it is one line; it is proven by compile and lint under `--features live-pulse` and by a source literal, never
  by a run. What the capture binary prints on a next real drive is unmeasured until someone drives it, and no
  drive is planned here.
- The in-test lines are built from recorded stamps and line kinds. They prove the capture's reading of those
  lines, never that Pulse surfaced a digest.
- A canary tick stamped after the instant stays unclassified by the capture (the d3 shape). Recovering it would
  need an identity Pulse's digest lines do not carry, or a read of a different surface; both are outside the pick.

## Graph impact (from the code-graph query, plane `rust`; trace `tree-query-2026-10-08-capture-canary-pairing-window-corrective.json`)
- **canary_attempts** — the name resolves to two symbols: the shared function
  (`real_model_common/mod.rs`) and the rule's own private parser of printed tokens
  (`real_model_harvest.rs`, inside the marked span, a different function taking a capture's text). The `calls`
  query returns 6 rows by name; read by argument type, 2 are the shared function's (the `paired` helper,
  `canary_pairing.rs:52`, and the import alias, `real_model_harvest.rs:58`) and 4 are the rule's own.
- **Index gap, named:** `real_model_live.rs` is feature-gated and not indexed; its call (`:728`) and `iso_ms`
  (`:774`) come from a name grep (`grep -rn canary_attempts crates/` → 10 hits in 5 files, read).
- So the changed signature has two call sites to thread: the capture's canary block and the `paired` helper. The
  import alias needs no change. No shipped crate calls it.

## Patterns detected
- **Fields-only in-test log lines** (`canary_pairing.rs:8-54`): `log`, `cue_tick`, `prompt`, `parse_ok`, `created`
  build Pulse-shaped JSON values with a stamp, a target and the few fields the pairing reads. The new shapes need
  full ISO stamps and a deduped outcome line; nothing else.
- **A gated file held from the default suite by a source literal** (`canary_pairing.rs:232-235`,
  `capture_tokens.rs:54-84`): `include_str!` of the capture's source and a `contains` on a literal.
- **A dated correction inside the clause it corrects** (`contracts/pulse-real-model-leg-posture.md:316-320`): a
  bracketed `[corrected {date} ({marker}, {when}): …]` block that keeps the earlier text and says the captures
  keep what they printed.
- **A section-level digest pin** (`real_model_grading/mod.rs:145-174`): a dated section's bytes, from its heading
  to the next `## `, against a sha256 its own ledger recorded.

## Conventions to follow
- **Prefix assertions on `canary:` lines** (`canary_pairing.rs:73-76`): `starts_with` the token, `ends_with` the
  trailing field; a whole-line literal is used only for the closing count line.
- **The degrade direction of a stamp comparison** (`real_model_live.rs:726`): a line whose stamp does not parse is
  never "before the instant"; with no instant at all the whole window is selected.
- **One locked write** (`real_model_live.rs:133-147`): the capture prints nothing directly; a changed line still
  goes through `emit`.
- **Child modules, never new targets** (`real_model_grading/mod.rs:1-4`): the new tests go into the existing
  `canary_pairing.rs`, so the module list of 14 and its walking test are untouched.

## Sweeps
- `canary_attempts` over `crates/`: 10 hits in 5 files · 4 changed (the definition, the capture's call, the
  capture's import list, which gains the stamp parser, and the `paired` helper) · 6 no-change (the harvest's
  import alias; the rule's own function of the same name and its four uses).
- pairing-window statements, pattern `pairing window|canary:. line per|third .canary:. line|before the
  (scenario.s )?emission instant` over the seven masters, `.andromeda/registries`, `.claude/docs`,
  `.claude/rules`, `CLAUDE.md`, `contracts`, `scripts`, `crates`: 9 hits in 6 files · 2 changed in this chunk (the
  shared module's doc comment; the contract clause, by its add-only note) · 2 for the wrap (obs-plan.md:221,
  test-plan.md:260) · 2 no-change (`series_2026_10_07_sixth.rs:59`, a record of what the sixth series' captures
  printed; `real_model_live.rs:128`, an unrelated assertion text). The capture's own comment at
  `real_model_live.rs:721-722` wraps across two lines and is outside the pattern; it changes with the call.
- readers of the posture contract, `pulse-real-model-leg-posture` over `crates/` and `scripts/`: 17 hits · 0
  changed · 1 real read (`real_model_grading/mod.rs:156`), the rest comments.

## Noticed, not this chunk's
- Architecture's registry row for the posture contract says it has "NO Rust reader", and `CLAUDE.md`'s overview
  says no Rust code reads it. A test binary does: `pre_registered` in `real_model_grading/mod.rs:155-174`, called
  by six harvest tests. No shipped code reads it. This chunk adds no reader and corrects no master; the mismatch
  is surfaced at the review.

## New files to create
- `conductor-0.3.0/chunks/2026-10-08-capture-canary-pairing-window-corrective/evidence/` — the failing-first record and the gate readings

## Files to modify
- `crates/conductor-run/tests/real_model_common/mod.rs` — the pairing takes the emission instant and selects ticks by their own stamp; the stamp parser moves here
- `crates/conductor-run/tests/real_model_live.rs` — the canary block hands the pairing the whole window and the instant; the stamp parser is imported
- `crates/conductor-run/tests/real_model_grading/canary_pairing.rs` — the helper's call, and the new tests on the recorded shapes
- `contracts/pulse-real-model-leg-posture.md` — one add-only dated note directly after the 2026-09-29 one

## Open questions
- none — both design leans are decided by an artifact (the capture's feature gate decides where the selection
  lives; the route entry decides where the contract note lands), and each is named at the review.
