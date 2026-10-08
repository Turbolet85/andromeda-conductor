# Failing-first — 2026-10-08-capture-canary-pairing-window-corrective

Measured 2026-10-08 at plan step 4, by /andromeda-implement. The tree: chunk base `39e197b` plus plan steps 2
and 3, so the pairing still drops every line not stamped strictly before the emission instant (the pre-fix
composition, moved into the shared module unchanged) and the six new tests are present.

## Before the fix

### The command as the plan lists it

    cargo nextest run -p conductor-run --test real_model_harvest --profile ci

- exit: 100
- summary line: `Summary [   0.066s] 43/139 tests run: 41 passed, 2 failed, 0 skipped`
- The `ci` profile cancels the run at the first failure, so 96 of the 139 tests were not run by this call. The
  plan predicted `139 tests run: 137 passed, 2 failed` for it; that line is not what this call prints. The
  second call below runs every test.

### The same command with `--no-fail-fast`

    cargo nextest run -p conductor-run --test real_model_harvest --profile ci --no-fail-fast

- exit: 100
- summary line: `Summary [   0.064s] 139 tests run: 137 passed, 2 failed, 0 skipped`
- failed, 2 of 139, both in `real_model_grading::canary_pairing`:
  - `a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d1`
  - `a_canary_tick_just_before_the_instant_reads_the_inference_stamped_after_it_d2`
- No other test is red. The four other new tests are green against the pre-fix composition:
  `the_scenario_s_own_storm_digest_never_prints_as_a_canary`,
  `a_canary_tick_stamped_after_the_instant_gets_no_line_d3`,
  `a_tick_the_instant_cannot_place_is_never_selected`,
  `the_pre_fix_window_read_the_same_shapes_as_a_pipeline_fault`; so is the source assertion added to
  `the_capture_prints_pulse_s_no_incident_outcome`.

### What each failing test printed

Each test's failure message is the paired lines, as a debug list of two strings. Both calls printed the same
two messages.

`…_d1` (the `starts_with` assertion on the first line, `canary_pairing.rs:357`):

    canary: pipeline-fault t=2026-10-08T06:32:46.351Z cue_kind=retry_storm cue_priority_tier=autonomous parse=none created=none deduped=none skip_reason=none
    canary other cue-bearing digests: 0 ()

`…_d2` (the same assertion, `canary_pairing.rs:373`):

    canary: pipeline-fault t=2026-10-08T06:42:51.851Z cue_kind=retry_storm cue_priority_tier=autonomous parse=none created=none deduped=none skip_reason=none
    canary other cue-bearing digests: 0 ()

These lines are the pairing's output over in-test lines built from the recorded stamps and line kinds. They are
not lines of a capture or of Pulse's log.

## After the fix

Measured 2026-10-08 at plan step 6, after step 5 (the instant selects ticks by their own stamp and no longer
drops lines). The command as the plan lists it:

    cargo nextest run -p conductor-run --test real_model_harvest --profile ci

- exit: 0
- summary line: `Summary [   0.078s] 139 tests run: 139 passed, 0 skipped`
- 133 at the chunk base, plus the six new tests. The two tests red above are green; the standing control
  `the_pre_fix_window_read_the_same_shapes_as_a_pipeline_fault` keeps the pre-fix reading of both shapes
  reproducible from the same in-test lines.
