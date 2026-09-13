# Code Audit — Conductor · Epoch 6b — Polish & ship · 2026-09-13T12:43:23Z

mode **trend** · HEAD `0f780c6` · baseline `59d5b7c8` (Epoch 6a — Verification follow-ups) · span 1

Ancestry OK. No trend-break: every tool version matches the baseline (jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-mutants 27.1.0 · rustc 1.95.0), so no metric's thresholds are suppressed.

**HEAD overshoots the epoch boundary by 3 chunks** (the first three conductor-0.3.0 Epoch-1 chunks). The source delta from the 6b boundary `b54e6ec` to HEAD is exactly ONE file — `crates/conductor-report/tests/matrix_ledger_gate.rs` (+61/−6, a test file); everything else in those three chunks is CI yaml, scripts and `.andromeda/` documents, none of which is a source path. Every number below therefore includes that one file and nothing else from beyond the boundary.

## The five-record trend

| epoch | loc | files | dup % | clones | dup lines | cog>15 | file max | >800 | cycles | edges | dead | cov % |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Epoch 3 - Live proof: the  | 16543 | 118 | 3.68 | 79 | — | 6 | 898 | 2 | 0 | 16 | 31 | 92.26 |
| Epoch 4 — Lifecycle & dele | 17518 | 111 | 3.66 | 84 | 853 | 6 | 1012 | 2 | 0 | 16 | 33 | 92.62 |
| Epoch 5 — Verification sur | 18113 | 115 | 3.58 | 86 | 870 | 6 | 1115 | 2 | 0 | 16 | 32 | 92.11 |
| Epoch 6a — Verification fo | 19790 | 119 | 3.43 | 90 | 906 | 6 | 1423 | 2 | 0 | 16 | 35 | 93.94 |
| **Epoch 6b — Polish & ship** | **25077** | **135** | **3.25** | **106** | **1049** | **6** | **1170** | **1** | **0** | **16** | **34** | **94.3** |

## Proposals

### M1 — `monotonic` · duplication (absolute) — clones +16, duplicated lines +143

**Movement:** clones 90 → 106 · duplicated lines 906 → 1049 (Epoch 6a → Epoch 6b). Percentage moved the other way: 3.43% → 3.25%.

**Evidence:** the `monotonic` rule needs a scalar worsened at both of the last two diffs. Two qualify, and in fact every measured diff worsens:

| epoch | clones | duplicated lines | dup % | total lines |
|---|---|---|---|---|
| Epoch 3 - Live proof: the  | 79 | — | 3.68% | — |
| Epoch 4 — Lifecycle & dele | 84 | 853 | 3.66% | 23329 |
| Epoch 5 — Verification sur | 86 | 870 | 3.58% | 24318 |
| Epoch 6a — Verification fo | 90 | 906 | 3.43% | 26408 |
| **Epoch 6b — Polish & ship** | **106** | **1049** | **3.25%** | **32324** |

Clones rose at **all four** measured diffs (79 → 84 → 86 → 90 → 106); duplicated lines rose at **all three** for which the field exists (853 → 870 → 906 → 1049). Over the same five records the percentage fell at every single diff (3.68 → 3.66 → 3.58 → 3.43 → 3.25), because total lines grew faster (23 329 → 32 324).

**Suspected shape:** the growth is not diffuse. The five largest clone families are all carried from the baseline and all live in `conductor-emit/tests/` — the same fixture block repeated across emission test files:

| lines | file A | file B |
|---|---|---|
| 44 | conductor-emit/tests/error_spans.rs | conductor-emit/tests/exception_events.rs |
| 42 | conductor-emit/tests/egress.rs | conductor-emit/tests/traffic_rate_ramps.rs |
| 39 | conductor-emit/tests/egress.rs | conductor-emit/tests/error_spans.rs |
| 38 | conductor-emit/tests/egress.rs | conductor-emit/tests/multi_service_topology.rs |
| 36 | conductor-emit/tests/egress.rs | conductor-emit/tests/latency_shaping.rs |

**Proposal:** two independent directions, both for your judgment. (a) The `conductor-emit` test fixtures are the whole top of the list and have been for four epochs — a shared `tests/common/` fixture helper would remove the family rather than the symptom. (b) Separately, note that the audit's own `duplication-up` check keys on **percentage** (`pct ≥ baseline + 0.5pt AND ≥ +15% relative`). On a codebase growing at this rate that check is structurally unable to fire — it has moved the *favourable* way at every diff while the absolute measure worsened at every diff. Whether the check should track the absolute count, or the percentage is the honest metric and this proposal is noise, is a threshold decision only you can make.

## Informational

- **The largest file in the workspace was dismantled.** `conductor-run/src/lib.rs` was the baseline's file_max at **1423** lines and its **#1 B2 hotspot** (score 32.0). It is now **40 lines**, split into 8 modules (`canary` · `dispatch` · `drive` · `envelope` · `execute` · `lifecycle` · `preconditions` · `testkit`) by `2026-09-05-audit-corrective` — literally *"the module split along its named seams"*. file_max fell 1423 → 1170 and files over 800 lines fell 2 → 1, reversing four epochs of monotonic rise (898 → 1012 → 1115 → 1423).
- **Codebase grew 5287 lines (+26.7%)**, 119 → 135 files — by far the largest epoch delta on record (prior deltas +975, +595, +1677). Verified against git: `59d5b7c8..HEAD` over `crates/**/*.rs` is +9377/−3462 = +5915 raw lines, consistent with tokei's +5287 code lines.
- **Complexity is unchanged — exactly.** The over-ceiling set is byte-identical to the baseline: the same 6 functions at the same values (serve_stub 35 · shape_is_realizable 31 · validate 18 · observe 17 · dispatch 16 · run_preflight 16). p50/p90 identical too (cyclomatic 1.0/4.0, cognitive 0/1.0) across +156 new functions.
- **Architecture is unchanged — exactly.** 0 cycles and 16 cross-unit edges for the fifth consecutive record; per-unit fan-out identical.
- **Coverage rose** 93.94 → 94.3% (10940/11601 lines), a fourth consecutive record above 92 and the highest measured. 906/906 tests passed during collection.
- **Churn halved** — 25.36% → 13.29% (1320/9931 adds re-touched, 24 of 129 files). The window is much larger than the baseline's (129 files touched vs 26), so the lower rate is over more work, not less.
- **Dead-code candidates fell** 35 → 34 (chain: 898 raw → 38 after the pinned tests-segment filter → 34 after entry points). Unused deps unchanged: `conductor-emit` → `conductor-core`, standing since the baseline.
- **B2 hotspot top-10 turned over almost completely**, which is expected — the score is commits × complexity and the commit window moved. New #1 is `conductor-core/src/phase_spec.rs` (score 62.0 = 2 commits × cognitive 31); its `shape_is_realizable` is also the #2 complexity offender workspace-wide. It is the one file that is both complex and repeatedly touched.
- **New clone family, cross-crate:** `conductor-report/src/journal.rs:158-181` ↔ `conductor-verify/src/record.rs:137-160`, 24 lines — the only new entrant in the top-10 clone list. Both fragments are the **eleven-key run-report envelope assertion**, inside inline `#[cfg(test)]` modules. `ci.yml:178-179` records the repo's own position that the schema already has two copies ("the structs · obs-plan §3/§6 · this file … would drift from the first two") and declines to add a third; this measurement locates the two literal key enumerations precisely. Worth noting only because a key added to the envelope must be edited in both.
- **`conductor-cli`'s test suite was transformed** — mutation score **58.59 → 96.81**, the largest single movement in this audit. The baseline left 41 of 99 viable mutants alive; this run leaves **3 of 94** ({"mutants": 112, "caught": 91, "missed": 3, "timeout": 0, "unviable": 18}). It was the workspace's weakest unit by a wide margin and is now its second strongest. `crates/conductor-cli/tests/cross_surface_parity.rs` is new in this window and is the plausible cause, though the audit measures the outcome, not the attribution.
- **Mutation held or improved in every scoped unit.** conductor-cli 58.59→96.81 · conductor-core 95.24→100.0 · conductor-run 94.44→94.32 · conductor-tauri 89.29→90.0 · conductor-verify 88.33→88.71. `conductor-core` caught **126/126** on shard 1/4 (the baseline's own scope, so the two are comparable) — 0 missed, 0 unviable. Total surviving mutants across all five units: 25, down from 69.
- **Pre-existing tool droppings at the project root**, surfaced not touched: `mutants.out/` (Sep 3) and `mutants.out.old/` (Aug 20) predate this run and are explicitly gitignored (`.gitignore:49-50`), as are the 18 `target/mutants-*` dirs from earlier chunks. Nothing from this run landed outside its run dir.
- **Web dead-code series STARTS here.** knip 6.34.0 is now installed (it was `tool-missing` at the baseline), reporting 12 candidates across 4 files ({"exports": 3, "types": 9}). **No trend exists yet**, and the count is reported as candidates against an unconfigured tool: this epoch's own friction record measured knip's first run at 20 findings / 20 false positives, because WebdriverIO discovers specs through `wdio.conf.ts` and loads devDependencies through its own plugin resolution — neither is an import edge knip can see.

## Below threshold — no action

- `new-cycle` — cycles 0 → 0, cross-unit edges 16 → 16. Not fired.
- `duplication-up` — pct 3.43 → 3.25 (fell); needs +0.5pt and +15% relative. Not fired. (The absolute movement is M1.)
- `complexity-creep` — over_ceiling 6 → 6 (Δ0); needs +3 and +25%. Not fired.
- `dead-growth` — candidates 35 → 34 (−1); needs +5. Not fired.
- `coverage-drop` — line 93.94 → 94.3 (+0.36pt); needs −2pt. Not fired.
- `mutation-drop` — conductor-cli 58.59 → 96.81 (+38.22pt); needs −10pt. Not fired.
- `mutation-drop` — conductor-core 95.24 → 100.0 (+4.76pt); needs −10pt. Not fired.
- `mutation-drop` — conductor-run 94.44 → 94.32 (-0.12pt); needs −10pt. Not fired.
- `mutation-drop` — conductor-tauri 89.29 → 90.0 (+0.71pt); needs −10pt. Not fired.
- `mutation-drop` — conductor-verify 88.33 → 88.71 (+0.38pt); needs −10pt. Not fired.
- `span > 1` — not applicable, span is 1 (consecutive boundaries).
- `trend-break` — no tool version differs from the baseline.

**Duplication split by path** (new field this run): {"src": 52, "test": 44, "mixed": 10}. Read with care — the classing is by file path, and Rust keeps unit tests inline in `src/`, so genuinely-test clones count as `src`. Both fragments of the new cross-crate family above are test code living in `src/`. No baseline exists for this field.

## Skips

- **mutation-web** — `tool-missing` — StrykerJS absent; recipe: npm i -D @stryker-mutator/core in crates/conductor-tauri/ui
- **complexity-web** — `declined` — lizard 1.24.0 IS available (python -m lizard); not collected, matching the baseline's deliberate decline, to keep the Rust-only complexity series comparable across all five records — a separate web series can start at any boundary
- **mutation-conductor-emit** — `declined` — touched in this window (458 mutants) but out of the operator-chosen baseline-parity scope
- **mutation-conductor-timeline** — `declined` — touched in this window (38 mutants) but out of the operator-chosen baseline-parity scope
- **mutation-conductor-report** — `declined` — touched in this window (89 mutants) but out of the operator-chosen baseline-parity scope; note this is where the epoch's final chunk landed — first measurement still owed
- **mutation-conductor-faults** — `declined` — touched in this window (28 mutants) but out of the operator-chosen baseline-parity scope
- **mutation-conductor-core** — `budget-exhausted` — 504 mutants in the unit; ran --shard 1/4 as the baseline did, so the two are comparable; the remaining 3 shards are untested this run

---

*Evidence twins in this run dir: `c-sizes` · `c-duplication` · `c-complexity` · `c-graph` · `c-dead` · `c-deadweb` · `c-coverage` · `c-churn` · `c-hotspots` · `c-mutation` (+ per-unit) `.json`, and `record.json` as appended. Reproducibility triple is in the ledger record: sha, tool_versions, and the `commands` field carries each firing form. Report-only — nothing here gates, blocks, or is remembered; fixes route through a route entry or chunk at your call.*
