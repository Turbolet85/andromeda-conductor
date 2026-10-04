# Code Audit — conductor · Epoch 5 — Polish & ship · 2026-10-04T09:51:11Z
mode trend · HEAD 88de1805 · baseline e799b9e0 (Epoch 3 — The a11y capability's terminal) · span 2

- **Overshoot (this run):** 2 commits past the Epoch 5 boundary `07c8f113` (two 0-pending wraps), **0 source files** in the delta.
- **Overshoot (baseline):** the baseline record carried 3 commits / 23 source files past the Epoch 3 boundary (Epoch 4's first two chunks + one route adaptation). That delta is attributed to the baseline's record and not re-diffed here; this run's window metrics start at the baseline sha.
- **Span 2:** the Epoch 4 boundary was not recorded (the founder ruled this run at HEAD over a worktree). Every single-epoch threshold is demoted to Informational with the span named. `new-cycle` and `monotonic` still run.
- **Host change:** this is the FIRST ledger record measured on the Linux dev host (x86_64-unknown-linux-gnu). All 7 prior records were measured on the Windows host. rca, tokei, jscpd and the graph are host-neutral reads of the source. Coverage and mutation execute code: read their deltas with the host beside them.
- **Trend-breaks:** duplication (jscpd 5.0.16 → 5.4.0) and coverage (cargo-llvm-cov 0.8.5 → 0.9.1). Their thresholds are suppressed and their deltas are labeled `trend-break — tool upgrade`.

## Proposals

### M1 — monotonic · sizes.file_max — 1221 → 2142 (+921)
**Movement:** 1170 (Epoch 1 — Foundation: the measurements the closures rest on) → 1221 (Epoch 3 — The a11y capability's terminal) → 2142 (Epoch 5 — Polish & ship). It worsened at both of the last two diffs, with tokei 14.0.0 unchanged.
**Evidence:** the max file is `crates/conductor-run/tests/real_model_harvest.rs` at 2142 code lines (680 at the baseline; window numstat +1793/−74). The next largest file is `crates/conductor-core/src/scenario.rs` at 1226. The same file holds 4 of the duplication top-10 rows as self-clones: 31 L (lines 2147/2422), 18 L (lines 2128/2403), 16 L (lines 1916/2123), 16 L (lines 1916/2398). It is hotspot #4 (score 32.0 = 4 window commits × max cognitive 8). Four chunk pre-CI commits touched it in the window: 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin, 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir, 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix and 2026-10-02-captured-fingerprint-values-elided (the real-model series' grading and digest-pin work).
**Suspected shape:** the real-model harvest test grows one grading arm per series and per assertion. Its repeated grading blocks show as self-clones, and the file roughly tripled in one epoch.
**Proposal:** split `real_model_harvest.rs` by series or assertion family, and lift the repeated grading fixture (the self-clone family at lines 1916–2422) into the existing `tests/real_model_common` module.

### M2 — monotonic · sizes.over_800 — 2 → 4 (+2)
**Movement:** 1 → 2 → 4 over the same three records. It worsened at both diffs, with tokei unchanged.
**Evidence:** every file over 800 code lines (n = `sizes.over_800`):
| file | code lines | baseline |  |
|---|---|---|---|
| crates/conductor-run/tests/real_model_harvest.rs | 2142 | 680 | entrant |
| crates/conductor-core/src/scenario.rs | 1226 | 1221 | standing |
| crates/conductor-run/tests/delegated_timing_harvest.rs | 1130 | 623 | entrant |
| crates/conductor-run/src/execute.rs | 802 | 801 | standing |
**Suspected shape:** both entrants are test harvest files. `delegated_timing_harvest.rs` grew +558/−36 in the window, across three chunks: 2026-09-29-hue-shift-budget-graded-hard, 2026-10-02-p-075-assert-round-against-pulse and 2026-10-03-p-075-re-round-on-incident-events. The two standing files are source files and moved by a handful of lines.
**Proposal:** structure the two harvest files per scenario or assertion family, as for M1. The standing `scenario.rs` and `execute.rs` hold steady, so they need no direction this boundary.

## Informational

- **Runner portability — `conductor-core` is unmeasurable by cargo-mutants copy mode (new this epoch).** The first core invocation failed its unmutated baseline. `secret_scan_gate::the_workspace_holds_no_secret_shaped_string` panics when `git ls-files` exits 128 ("the gate has no subject"), because the temp copy carries no `.git`. The gate arrived on 2026-09-24 in this epoch (`2026-09-24-secret-scanning-ci-gate`), after the baseline's core run. Re-run with `--copy-vcs true`, the unit scored 140/140, so the score is measured, but the default form no longer works for this unit. Evidence: `evidence-core-nocopyvcs.json`, `_mutants-conductor-core-nocopyvcs.log`. A direction for the founder: either the gate tolerates a VCS-less tree by naming it a skip, or the project pins `--copy-vcs true` in its own mutation form (test-plan names the tool).
- **duplication (trend-break — tool upgrade, span 2):** pct 2.40% → 2.99% (+0.59pt, +24% rel). That would satisfy `duplication-up`, but it is suppressed by the jscpd 5.0.16 → 5.4.0 break and the span. Clones 104 → 130, duplicated lines 854 → 1226; population total_lines 35591 → 41066 (+15.4%). Split: src 55/519 → 56/510 · test 39/340 → 64/617 · mixed 10/99 → 10/99 (pairs/lines). The growth is almost entirely in tests (+25 pairs, +277 lines); src gained 1 pair and fell 9 lines. The top standing pair is `conductor-core/src/scenario.rs` self-clone, 17 L, standing since the Epoch 3 record. The tool-version share of the movement is unmeasured: jscpd 5.0.16 was not re-run.
- **Duplication top entrants (5):** `conductor-run/tests/real_model_harvest.rs` ↔ `conductor-run/tests/real_model_harvest.rs` 31 L; `conductor-run/tests/real_model_live.rs` ↔ `conductor-run/tests/span_landing_live.rs` 21 L; `conductor-run/tests/real_model_harvest.rs` ↔ `conductor-run/tests/real_model_harvest.rs` 18 L; `conductor-run/tests/real_model_harvest.rs` ↔ `conductor-run/tests/real_model_harvest.rs` 16 L; `conductor-run/tests/real_model_harvest.rs` ↔ `conductor-run/tests/real_model_harvest.rs` 16 L.
- **coverage (trend-break — tool upgrade + host change):** line 94.7 → 94.7 (lines found 12220 → 12542, hit 11568 → 11883); 1197/1197 tests passed. Branch is not reported (null, as before).
- **Mutation — new survivors this epoch** (excluding units first scored here and line-drifted standing survivors): `crates/conductor-emit/src/identity.rs:49:15` `replace ^= with |= in xor_in_place` and `replace ^= with &= in xor_in_place` (the per-run span identity code, `2026-10-01-per-run-span-identity-in-the-real-model-harness`); `crates/conductor-run/src/canary.rs:203:14` `replace > with >= in emit_canary_storms`, `canary.rs:206:61` `replace * with +` and `replace * with /`. The baseline's `execute.rs:122/165` survivors stand at `:126/:169` (line drift), not gone.
- **Mutation scores (span 2, so `mutation-drop` is informational; no unit moved ≥ 10pt):** conductor-cli 96.94 → 96.91 · conductor-core 100.0 → 100.0 · conductor-emit 99.01 → 97.09 · conductor-faults — → 95.83 · conductor-report — → 91.14 · conductor-run 92.47 → 90.29 · conductor-tauri 90.32 → 90.32 · conductor-timeline 90.62 → 90.91 · conductor-verify — → 89.34. faults, report and verify were not in the baseline scope: this is their first score. core and emit are shard 1/4, as at the baseline.
- **Standing timeouts, unowned:** conductor-emit holds 11 timeouts, all in `exception.rs` `skip_absolute_path` / `skip_line_number_suffix` (lines 321–348). The baseline also had 11. They are invisible to the score formula and carry across boundaries.
- **Pre-schema baseline mutation:** the baseline record has no `host` and no `not_measured`, so its host-excluded share is UNKNOWN, never 0. It was NOT recomputed: the cover evaluates against the host that built, the baseline was measured on Windows, and a faithful recompute needs a Windows host at `e799b9e0`. No `corrections[]` fill is made, and the score comparisons above carry this caveat.
- **Hotspot entrants (7):** `crates/conductor-verify/tests/common/mod.rs` 72.0; `crates/conductor-run/tests/lifecycle_harvest.rs` 30.0; `crates/conductor-core/tests/workflow_env_gate.rs` 26.0; `crates/conductor-run/src/dispatch.rs` 16.0; `crates/conductor-verify/src/preflight.rs` 16.0; `crates/conductor-run/tests/p075_round_live.rs` 14.0; `crates/conductor-run/tests/span_landing_live.rs` 14.0.
- **Complexity top entrant:** `analyse` in `conductor-core/tests/workflow_env_gate.rs` (cognitive 26), the workflow env-context gate (`2026-09-24-secret-scanning-ci-gate`).
- **Sizes top entrant:** `crates/conductor-run/tests/real_model_live.rs` (651).
- **Dead-code top rotation:** `conductor-core obs/impl#[ObsWriter][`MakeWriter<'a>`]make_writer().`; `conductor-run dispatch/DispatchError#Emit#`; `conductor-core scenario_audit/impl#[ScenarioAuditLedger]default_path().` entered the top-20. The candidate count is unchanged (33 → 33, chain 1129 → 36 → 33).
- **churn (span 2):** 5.01% → 38.99% (3134/8037 adds; 17 of 57 files touched more than once across 15 commits). The most-touched files: `crates/conductor-run/tests/real_model_common/mod.rs` ×4; `crates/conductor-run/tests/real_model_harvest.rs` ×4; `crates/conductor-run/tests/real_model_live.rs` ×4; `crates/conductor-run/tests/real_model_series/mod.rs` ×4; `crates/conductor-tauri/ui/test/a11y/screen-reader.e2e.ts` ×4.
- **Web dead code:** knip 12 → 0 findings (exit 0, clean).
- **Web complexity (first collection, lizard 1.24.0, cyclomatic only):** 86 functions, p50 1 / p90 4; 1 over 15: `App` in `ui/src/App.tsx` (24). There is no baseline, so this starts a series.

## Below threshold — no action

- complexity.over_ceiling 6 → 8 (+2; `complexity-creep` needs +3 and +25%). The new members are `analyse` (26) and `print_pulse_witnesses` (14 → 18, `conductor-run/tests/real_model_live.rs`). p50/p90 are unchanged (cyc 1/4, cog 0/1); functions 2576 → 3061; max `serve_stub` 35 → 36.
- dead.zero_ref_candidates 33 → 33. Unused deps: none (cargo-machete).
- graph: cycles 0 → 0, cross-unit edges 16 → 16, fan-out unchanged. Fan-in top: `ReportState#` 110 → 122, `Verdict#` 92 → 101, `crate/` 154 → 158.
- sizes p50 123 → 127, p90 565 → 581. The populations (totals.loc 27569 → 32147, files 141 → 150) are never movements.
- monotonic, not fired: duplication.pct (trend-break at the second diff), coverage.line (trend-break; not worse), complexity.over_ceiling (flat at the first diff), dead.zero_ref_candidates (flat).
- count-under-ratio: not applicable. clones and duplicated_lines rose with pct rising too, not held or fallen.

## Skips

- mutation-web — tool-missing (StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui)
- mutation-conductor-core — budget-exhausted (ran --shard 1/4 only (140/140 of the shard; 559 in the unit); the other 3 shards are untested this run)
- mutation-conductor-emit — budget-exhausted (ran --shard 1/4 only (114/114 of the shard; 456 in the unit); the other 3 shards are untested this run)
