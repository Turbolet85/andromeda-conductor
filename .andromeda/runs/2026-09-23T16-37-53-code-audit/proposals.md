# Code Audit — Conductor · Epoch 3 — The a11y capability's terminal · 2026-09-23T17:36:34Z
mode trend · HEAD e799b9e0 · baseline 6861eb63 (Epoch 1 — Foundation: the measurements the closures rest on) · span 2 (Epoch 2 — Scenario assertion hygiene is spanned, never recorded)

**Head overshoot:** 3 commits past the Epoch 3 boundary `e75fcb9d` — Epoch 4's two complete chunks (`2026-09-18-real-model-leg-posture-and-grading-rule`, `2026-09-22-interpretation-proven-live`) plus one route-adaptation commit. Source delta, 23 files, +2672 / −102: `conductor-run/tests/real_model_harvest.rs` +883/−0 · `conductor-run/tests/real_model_live.rs` +722/−0 · `conductor-core/src/run_contract.rs` +318/−36 · `conductor-core/src/preconditions.rs` +255/−14 · `conductor-core/src/scenario.rs` +106/−0 · `conductor-run/src/preconditions.rs` +97/−24 · `conductor-run/tests/real_model_common/mod.rs` +73/−0 · `conductor-cli/tests/cli_smoke.rs` +64/−0 · `conductor-run/src/execute.rs` +39/−0 · `conductor-run/src/canary.rs` +33/−10 · `conductor-cli/src/cli.rs` +20/−0 · `conductor-cli/src/commands/preconditions.rs` +17/−3 · `conductor-run/src/testkit.rs` +15/−2 · `conductor-run/tests/composition_root.rs` +8/−1 · `conductor-run/tests/run_contract_pin.rs` +5/−3 · `conductor-core/src/lib.rs` +4/−2 · `conductor-cli/src/main.rs` +3/−1 · `conductor-run/tests/dispatch_wire.rs` +3/−2 · `conductor-run/src/lib.rs` +2/−2 · `conductor-timeline/src/convert.rs` +2/−1 · `conductor-cli/src/commands/run.rs` +1/−1 · `conductor-core/src/error.rs` +1/−0 · `conductor-core/src/load_envelope.rs` +1/−0 (all under `crates/`). Every metric below INCLUDES this delta; the next record attributes it to this one. The baseline record's own overshoot is 0 commits (at its boundary).

**Trend-break check:** every tool's leading version token equals the baseline's (jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-machete 0.9.2 · cargo-mutants 27.1.0 · cargo-llvm-cov 0.8.5 · cargo-nextest 0.9.133 · knip 6.34.0 · rustc 1.95.0). `code-graph`: the baseline value ("tree.db via scripts/code-graph.py") carries no leading token, so the comparison is UNKNOWN — noted, not a trend-break; this record carries the blob token `d0425fb1`.

## Proposals

None. No threshold fired: there are no cycles; duplication %, over-ceiling complexity and zero-ref candidates are all flat or down; line coverage went up; and no scored unit's mutation score fell by 10 points or more. `monotonic` did not fire for any of the six tracked scalars (see Below threshold). Span 2 would have demoted any single-epoch rule to informational anyway.

## Informational

- **span > 1 · sizes.file_max 1170 → 1221 · over_800 1 → 2** (Epoch 1 → Epoch 3, two boundaries). `crates/conductor-core/src/scenario.rs` 1170 → 1221 code lines; `crates/conductor-run/src/execute.rs` 768 → 801 crosses 800. Both files are in the overshoot delta (`scenario.rs` +106/−0, `execute.rs` +39/−0), so this growth landed in Epoch 4's chunks, not in Epoch 3's. Both scalars worsened only at this latest diff (they were flat 1170 / 1 at Epoch 6b → Epoch 1), so `monotonic` cannot fire yet. If the next boundary grows either one again, it will.
- **count-under-ratio · duplication:** clones 102 → 104 (+2) and duplicated lines 849 → 854 (+5), while pct fell 2.63 → 2.40 (−0.23 pt). The population `total_lines` grew 32 290 → 35 591 (+10.2 %). By path: src pairs 52 → 55 (lines 498 → 519), test pairs 40 → 39 (354 → 340), mixed pairs 10 → 10 (99 → 99). Top standing pair: `conductor-emit/src/latency.rs` ↔ `conductor-emit/src/rate.rs` (15 L ×2 fragments), in the top list since the first record (Epoch 3 — Live proof: the five families, 0.2.0).
- **top-N entrants · complexity top-10:** `print_pulse_witnesses` (cognitive 14, cyclomatic 35) and `poll_once` (14, 20) in `conductor-run/tests/real_model_live.rs`, and `host_path_starts_at` in `conductor-run/tests/real_model_common/mod.rs`, all overshoot test code. They displaced `validate` in `conductor-core/src/load_envelope.rs`, `redact_value` in `conductor-core/src/redact.rs` and `service_topology_request` in `conductor-emit/src/topology.rs` from the top 10. Over-ceiling stays at 6, the same six functions as the baseline.
- **top-N entrants · fan-in top-20:** `conductor-core run_contract/L4Posture#` (59 distinct callers, the real-model posture type) and `conductor-core error/CoreError#Config#` (40) entered. `expected/ClaimClass#` 52 → 48 and `expected/ComparisonKind#` 52 → 46 moved down; `verify common/StubConfig#` and `pause/Decision#` dropped out. The `conductor-core crate/` hub is 141 → 154.
- **top-N entrants · dead-code candidates:** the count is flat at 33 (chain raw 886 → 980 · after the `tests/` segment filter 37 → 37 · after entry points 33 → 33). One candidate is new relative to the baseline's own top-20: `conductor-core run_contract/impl#[L4Posture][Display]fmt()`, a trait-impl reached by dispatch, which is a named false-positive class. The baseline stored only its top-20, so membership beyond that cannot be compared; this run's full list of 33 is in `c-dead.json`. knip (web) is flat at 12 (exports 3 · types 9), the same unconfigured-tool caveat as before. cargo-machete: no unused dependencies.
- **top-N entrants · hotspots:** all ten are new against the baseline's top-10, because the churn window is new. The top three are `conductor-core/src/run_contract.rs` 18 · `conductor-run/tests/real_model_live.rs` 14 · `conductor-core/src/load_envelope.rs` 13.
- **churn:** 5.01 % (197 churned adds / 3 936 adds across 32 files; 4 files touched more than once) over `6861eb63..HEAD`. The baseline window was 0.0 % (258 adds, 17 files).
- **mutation · emit shard not comparable like-for-like:** `conductor-emit` shard 1/4 scored 45.71 → 99.01. The unit's mutant list changed (452 mutants now), so shard 1/4 is a different set than at the baseline. This is not a like-for-like improvement. The 11 timeouts are all in `conductor-emit/src/exception.rs` `skip_absolute_path` (lines 321–326) and are listed in the record's `mutation.timeouts`.
- **mutation · first data point:** `conductor-timeline` 90.62 has no prior audited score.
- **mutation · wall-clock:** `conductor-run` ran 16 min against the 15 min cap. It was complete by the tool's markers (`end_time` set, 123/123) when read, so it is scored; the overrun is disclosed here.
- **mutation · survivors in overshoot code:** 8 of the 17 survivors sit in files the overshoot delta touched: `conductor-cli/src/cli.rs:78` (`&&`→`||` in `scenario_name`) · `conductor-run/src/canary.rs:68` (delete `!` in `preflight_for`) · `conductor-run/src/execute.rs:122` (delete field `degraded` from the `Observation` expression) · `conductor-run/src/execute.rs:165` ×2 (`-`→`+`, `-`→`/` in `execute_scenario`) · `conductor-run/src/preconditions.rs:38`, `:39` (`==`→`!=` in `observe_run_contract`) · `:59` (`declares → false`). Whether each survivor's LINE is in the delta, rather than only its file, was not checked. The full list of 17 is in the record's `mutation.survivors`.

## Below threshold — no action

| Unit | State | Baseline score | This score | Δ | missed |
|---|---|---|---|---|---|
| conductor-cli | complete 116/116 | 97.87 | 96.94 | −0.93 | 3 |
| conductor-run | complete 123/123 | 94.38 | 92.47 | −1.91 | 7 |
| conductor-core | complete 140/140 of shard 1/4 | 100.0 | 100.0 | 0 | 0 |
| conductor-emit | complete 113/113 of shard 1/4 | 45.71 | 99.01 | (shard set differs) | 1 |
| conductor-tauri | complete 46/46 | 90.0 (Epoch 6b) | 90.32 | +0.32 | 3 |
| conductor-timeline | complete 38/38 | — | 90.62 | first | 3 |

- `mutation-drop` fires at −10 pt. Neither drop (cli −0.93, run −1.91) comes near that.
- `monotonic` six scalars, Epoch 6b → Epoch 1 → Epoch 3: duplication.pct 3.25 → 2.63 → 2.40 (improving) · complexity.over_ceiling 6 → 6 → 6 · dead.zero_ref_candidates 34 → 33 → 33 · sizes.file_max 1170 → 1170 → 1221 (worse at one diff only) · sizes.over_800 1 → 1 → 2 (worse at one diff only) · coverage.line 94.3 → 94.4 → 94.7 (improving). None fires.
- coverage.line 94.4 → 94.7 (lines 10 953/11 598 → 11 568/12 220). A rising value is not credited; only a fall is a signal.
- complexity percentiles unchanged (cyclomatic p50/p90 1/4 · cognitive p50/p90 0/1); functions 2 277 → 2 576.
- sizes p50 115 → 123 · p90 473 → 565; totals 25 005 → 27 569 code lines over 136 → 141 files (population only, not a movement).
- graph: cycles 0 → 0; cross-unit edges 16 → 16; fan-out per unit unchanged (run 6 · cli 4 · tauri 2 · faults/timeline/verify/report 1).
- `conductor-verify` and `conductor-report` untouched since the baseline, so not scoped. Their last scores stand in the Epoch 1 record (91.8 · 91.14).

## Skips

- mutation-web: tool-missing. StrykerJS absent → `npm i -D @stryker-mutator/core` in `crates/conductor-tauri/ui`.
- complexity-web: declined. lizard is not on PATH (`pip install lizard`); left out at every boundary to keep the Rust-only complexity series comparable.
- mutation-conductor-core: budget-exhausted for shards 2–4 of 4. Shard 1/4 was complete (140/140; the unit has 559).
- mutation-conductor-emit: budget-exhausted for shards 2–4 of 4. Shard 1/4 was complete (113/113; the unit has 452).
