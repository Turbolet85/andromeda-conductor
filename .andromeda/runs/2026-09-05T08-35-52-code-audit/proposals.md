# Code Audit — Conductor · Epoch 6a — Verification follow-ups · 2026-09-05T08:35:52Z
mode **trend** · HEAD `59d5b7c` · baseline `29c30b0` (Epoch 5 — Verification surfaces, recorded 2026-09-02T16:19:36Z) · span 1 · 10 commits
ancestry OK · no trend-break (every collector version identical to the baseline record) · source tree clean at collection · record appended to `.andromeda/code-metrics.ndjson` (4th record)

Evidence twins beside this file: `c-{sizes,duplication,complexity,graph,dead,coverage,churn,hotspots}.json`, `c-mutation-{unit}.json` ×5, `q-diff.json` (every scalar + every threshold check), `record.json` (the appended line), `tree-query-code-audit.json` (the graph adoption trace). Founder-facing and obligation-free: nothing here is applied, gated on, or remembered.

## Proposals

### M1 — monotonic · `sizes.file_max` — 1012 → 1115 → **1423**
**Movement:** 1115 → 1423 lines (+308, +27.6 %) at this boundary (Epoch 5 → 6a), after +103 at the one before (Epoch 4 → 5) — worsened at both of the last two diffs. The file is `crates/conductor-run/src/lib.rs`, the run composition root, at every one of the three boundaries.
**Evidence:**
- Sizes: the largest file by 422 lines over the next (`conductor-core/src/scenario.rs` 1001, flat since Epoch 5); `over_800` stays 2 — the growth is one file's. Totals for context: workspace loc 18 113 → 19 790 (+9.3 %), so this file grew three times faster than the tree.
- Hotspots: #1 this span at 32 (4 touching commits — the most of any file — × max cognitive 8); baseline #2 at 10. Its max cognitive is 8, well under the 15 ceiling: the growth is breadth, not depth.
- Churn: 4 of the span's 10 commits touched it (`c-churn.json` per_file_commits).
- Mutation: home of all 5 conductor-run survivors — the accepted-deliberate roster the epoch's own chunk dispositioned (see Informational).
- Duplication pairs involving it this span (from `c-duplication.json`):
<!-- rows: 4 -->
| pair | lines | start lines |
|---|---|---|
| conductor-run/src/lib.rs ↔ conductor-verify/src/preflight.rs | 14 | 96 / 271 |
| conductor-run/src/lib.rs ↔ conductor-run/src/lib.rs | 12 | 96 / 474 |
| conductor-run/src/lib.rs ↔ conductor-run/tests/operator_pause_harvest.rs | 8 | 576 / 42 |
| conductor-core/src/load_envelope.rs ↔ conductor-run/src/lib.rs | 6 | 399 / 1287 |
**Suspected shape:** the composition root absorbs each epoch's new seam — this epoch the preconditions probe (`observe_preconditions`, the per-handle grading wrapper) and the run-contract observation — and its inline `#[cfg(test)]` module grows with it (the epoch's own survivor chunk added 11 tests inside `lib.rs`; the audit's sizes population counts test lines inside `src/`). The Epoch-5 audit hypothesized that its tests were not keeping pace; the 6a research measured the opposite (test lines +94 % vs production +72 % over 2026-08-16..09-02) — so this is a file growing on both halves, not a coverage gap.
**Proposal:** a split along seams the file already names — the preconditions/run-contract observation half, the canary warm-up half, the lifecycle/harvest helpers — into sibling modules of `conductor-run`; and/or moving the inline test module to `tests/` so the production file's count reflects production code. Whether a composition root that grows with every seam is the intended design is the founder's call; the number only says it has done so for three boundaries.

### M2 — monotonic · `duplication.clones` — 84 → 86 → **90**
**Movement:** +2 then +4 clone pairs (Epoch 4 → 5 → 6a); duplicated lines 870 → 906 (+36). The **percentage fell** 3.58 → 3.43 (total lines 24 318 → 26 408, +8.6 %, outgrew the duplicated lines' +4.1 %) — the count is the half moving the wrong way, the ratio the half improving; `duplication-up` (≥ +0.5 pt) does not fire.
**Evidence:**
- Split: both-test 41 pairs / 511 lines → 43 / 526 (+2, +15); both-src 37 / 368 → 39 / 393 (+2, +25); mixed 8 / 77 → 8 / 77 (flat).
- Top-10: identical to the baseline, led by the `conductor-emit/tests` family (36–44 lines); no entrant.
- The four new pairs can only be among the 17 pairs involving a file touched this span (the baseline record kept only its top-10, so the exact four cannot be named; this record stores all 90 pairs so the next boundary can) — all 17:
<!-- rows: 17 -->
| pair | lines | start lines |
|---|---|---|
| conductor-run/src/lib.rs ↔ conductor-verify/src/preflight.rs | 14 | 96 / 271 |
| conductor-cli/src/pause.rs ↔ conductor-cli/src/render.rs | 13 | 123 / 635 |
| conductor-run/tests/canary_obs_witness.rs ↔ conductor-run/tests/dispatch_wire.rs | 13 | 32 / 39 |
| conductor-tauri/src/commands.rs ↔ conductor-tauri/src/commands.rs | 13 | 173 / 232 |
| conductor-run/src/lib.rs ↔ conductor-run/src/lib.rs | 12 | 96 / 474 |
| conductor-tauri/src/commands.rs ↔ conductor-tauri/src/commands.rs | 12 | 409 / 508 |
| conductor-cli/src/render.rs ↔ conductor-report/src/coverage.rs | 9 | 287 / 83 |
| conductor-run/tests/dispatch_wire.rs ↔ conductor-run/tests/dispatch_wire.rs | 9 | 374 / 399 |
| conductor-cli/src/render.rs ↔ conductor-report/src/coverage.rs | 8 | 592 / 174 |
| conductor-cli/src/render.rs ↔ conductor-report/src/coverage.rs | 8 | 592 / 194 |
| conductor-run/src/lib.rs ↔ conductor-run/tests/operator_pause_harvest.rs | 8 | 576 / 42 |
| conductor-run/tests/canary_wire.rs ↔ conductor-run/tests/dispatch_wire.rs | 8 | 24 / 20 |
| conductor-cli/src/render.rs ↔ conductor-report/src/report.rs | 7 | 328 / 231 |
| conductor-run/tests/canary_wire.rs ↔ conductor-run/tests/dispatch_wire.rs | 7 | 31 / 27 |
| conductor-cli/tests/cli_smoke.rs ↔ conductor-cli/tests/cli_smoke.rs | 6 | 315 / 346 |
| conductor-core/src/load_envelope.rs ↔ conductor-run/src/lib.rs | 6 | 399 / 1287 |
| conductor-emit/tests/error_spans.rs ↔ conductor-run/tests/dispatch_wire.rs | 6 | 13 / 26 |
**Suspected shape:** the src-side growth clusters in the two files this epoch touched most — `conductor-tauri/src/commands.rs` self-clones (13 lines at 173/232, 12 at 409/508: a per-command prologue repeated across the commands the epoch added or re-shaped) and `conductor-cli/src/render.rs` ↔ `conductor-report/src/coverage.rs` (9 + 8 + 8 lines: the coverage table/summary rendered once per surface) — while the test-side growth is `dispatch_wire.rs` sharing its wire setup with `canary_wire.rs` / `canary_obs_witness.rs` (13, 8, 7 lines).
**Proposal:** (a) a shared command prologue helper in `commands.rs` for the repeated resolve-and-map-error shape; (b) the CLI ↔ report coverage rendering is a cross-surface pair — either a shared formatter in `conductor-report` the CLI calls, or the two-surface parity is accepted by construction and recorded as such; (c) a `tests/common` fixture for the canary/dispatch wire setup in `conductor-run`. The count is small in absolute terms; the direction is what earned the line.

## Informational

**Mutation tier — 5 units, 552 mutants tested, 69 survivors, 2 timeouts** (`--jobs 2 --test-tool=nextest`, per-unit 15-min cap honored; runtimes tauri 265 s · run 802 s · verify 484 s · cli 550 s · core shard 380 s).
- **conductor-run 72.22 → 94.44 (+22.2 pt):** 25 → 5 survivors; the five are exactly the accepted-deliberate roster the epoch's `conductor-run-composition-root-survivors-dispositioned` chunk classified — nothing new escaped. Not a threshold event (the rule watches drops).
- **conductor-verify 89.09 → 88.33 (−0.76 pt, inside noise):** 12 → 14 survivors; the 2 new are `spawn.rs:116 sidecar_resolves_on_path → true/false`, accepted-deliberate at the sidecar-spawn chunk; the 12 standing are 8 in the `stub_pulse_mcp` test-stub binary, `client.rs:157 resolve_incident`, `extract.rs:95/:110`, `manifest.rs:35 default_path`. **The 2 timeouts are the same pair as Epoch 5** (`jsonrpc.rs:49:30`, `:70:9`) — carried across two boundaries with no owner; the score formula excludes them, so they stay invisible unless named here.
- **conductor-tauri — first COMPLETE score, 89.29** (Epoch 5: 0/43 tested, baseline-test-failure): 3 survivors = the chunk's accepted-deliberate set (`main.rs:18`, `commands.rs:272 start_run`, `:307 run_thread`). No trend judgment (no prior score).
- **conductor-cli — first-ever score, 58.59** (117: 58 caught / 41 missed / 18 unviable). An absolute finding, not a drop: **32 of the 41 survivors are in `render.rs`** — the pure string builders (`paint`, `hold_line`, `envelope_caption`, `coverage_summary`, `wire`, `latency`, `fingerprints`) survive replacement with empty or constant output, both tty gates (`stdout_color`/`stderr_color`) survive inversion, and every branch of the coverage-table styling survives. Direction: golden tests over the string builders plus an env-controlled color gate; the unit is untouched by any prior audit, so this is where a first killing pass would move the most. Complete list below. `render.rs` is also this epoch's sizes-top-10 entrant (521 lines) and a hotspot entrant.
- **conductor-core — shard 1/4, 95.24** (126 of 504: 120 caught / 6 missed / 0 unviable): all 6 survivors are arithmetic mutants inside `obs.rs:224-226 civil_from_unix` (the self-obs timestamp's date arithmetic — `-`/`+` swaps in the days-to-civil conversion that no test pins at a boundary date). The shard ran in 380 s, so the full unit fits in ~25 min — a founder call for the next boundary's cap. Three shards untested (skip: budget-exhausted).

**Top-N entrants**
- Sizes top-10: `conductor-cli/src/render.rs` 521 (new); `conductor-tauri/src/commands.rs` 448 → 583 (7th → 4th); `conductor-run/tests/dispatch_wire.rs` 404 → 435.
- Complexity top-10: none (identical, including order); `over_ceiling` 6 → 6 — `serve_stub` 35 (test helper), `shape_is_realizable` 31, `run_contract::validate` 18, `extract::observe` 17, `dispatch` 16 (cyclomatic 47), `run_preflight` 16.
- Duplication top-10: none.
- Fan-in top-20: `conductor-core capability_manifest/CapabilityManifest#` 36 (new; displaced `conductor-verify common/StubConfig#` 35); shifts elsewhere ≤ 2 (`crate/` 123 → 122, `Scenario#` 99 → 100, `ReportState#` 83 → 85, `VerifyError#` 41 → 42, `EnvelopeStatus#` 41 → 42, `pause/Decision#` 38 → 36).
- Hotspots top-10 (a per-span metric — entrants are expected): 8 new — `conductor-core/src/preconditions.rs` 22, `conductor-core/src/run_contract.rs` 18, `conductor-cli/tests/cli_smoke.rs` 7, `conductor-verify/src/spawn.rs` 6, `conductor-cli/src/render.rs` 6, `conductor-cli/src/commands/preconditions.rs` 5, `conductor-cli/src/main.rs` 4, `conductor-tauri/src/main.rs` 4; carried: `conductor-run/src/lib.rs` 10 → 32, `conductor-tauri/src/commands.rs` 6 → 9. Baseline #1 (`conductor-verify/tests/common/mod.rs` 35) dropped out — untouched this span.

**Dead-code candidates 32 → 35 (+3, under the +5 line) — the movement is filter-invisible false positives, not new dead code.** Chain 819 raw → 39 after the tests-segment filter → 35 after entry points (pinned recipe). Attribution: `obs.rs` — untouched this span — holds 12 of the 35 (`Layer`/`Visit`/`Write`/`MakeWriter` trait-impl methods, all dispatch-reached), so its rows cannot be new; the 4 candidates in files touched this span are ALL false positives: `preconditions.rs` `SIDECAR_PROGRAM` / `EGRESS_TARGET` and `render.rs` `PRECONDITION_LABEL` are referenced only through inline format-string captures (`format!("{SIDECAR_PROGRAM} …")` — grep-verified at `preconditions.rs:189`, `:178`, `render.rs:216`), which the SCIP index does not emit as references — a **fifth false-positive family the collector table does not name** (named here, not subtracted, so the pinned count stays comparable); `PreconditionSubject`'s `Display::fmt` is dispatch-reached. Dropped since baseline: `TauriResolver::kind` (now referenced by a 6a test). Classing of the 35: 19 trait-impl dispatch · 7 derive/attr (`#[from]` error variants ×6, a serde default ×1) · 5 API accessors never called in-tree (`LatencyProfile::{p50,p95,p99}_ms`, `PortOccupier::occupy_default`, `ContractManifest::default_path`) · 4 constants (the 3 format-capture FPs + `RUN_CONTRACT_PRECONDITION`). Unused deps: `conductor-emit → conductor-core` (unchanged). Full list in `c-dead.json`.

**Coverage 92.11 → 93.94 % line** (+1.83 pt; 9 390 / 9 996 lines; 853/853 tests) — trend-only, not celebrated; the mutation tier above is the honest half. Branch data absent as in every prior record.
**Churn 33.63 → 25.36 %** (658 of 2 595 adds churned; 6 of 26 touched files: `lib.rs` 4 commits, `commands.rs` 3, `screen-reader.e2e.ts` 2, `spawn.rs` 2, core `lib.rs` 2, `preconditions.rs` 2).
**Totals (context, never judged):** loc 18 113 → 19 790 (+1 677); files 115 → 119; functions 1 992 → 2 122; units 9.
**Graph:** 0 cycles · 16 cross-unit edges · fan-out identical (run 6, cli 4, tauri 2, timeline/verify/report/faults 1).

### First-audited units — complete survivor lists (absolute findings)
conductor-cli — 41:
<!-- rows: 41 -->
| site | mutation |
|---|---|
| main.rs:55:5 | replace hint_for -> &'static str with "" |
| main.rs:55:5 | replace hint_for -> &'static str with "xyzzy" |
| commands/mod.rs:29:5 | replace exit_code -> ExitCode with Default::default() |
| paths.rs:73:39 | delete ! in Paths::load_all_scenarios |
| paths.rs:130:5 | replace env_seed -> Option<u64> with None |
| pause.rs:31:9 | replace <impl PauseResolver for PromptResolver>::kind -> &'static str with "" |
| pause.rs:31:9 | replace <impl PauseResolver for PromptResolver>::kind -> &'static str with "xyzzy" |
| pause.rs:104:9 | replace <impl PauseResolver for CliResolver>::kind -> &'static str with "" |
| pause.rs:104:9 | replace <impl PauseResolver for CliResolver>::kind -> &'static str with "xyzzy" |
| render.rs:62:5 | replace lamp_code -> u8 with 0 |
| render.rs:62:5 | replace lamp_code -> u8 with 1 |
| render.rs:75:5 | replace paint -> String with String::new() |
| render.rs:75:5 | replace paint -> String with "xyzzy".into() |
| render.rs:95:5 | replace hold_line -> String with String::new() |
| render.rs:95:5 | replace hold_line -> String with "xyzzy".into() |
| render.rs:104:5 | replace envelope_caption -> Option<String> with None |
| render.rs:104:5 | replace envelope_caption -> Option<String> with Some(String::new()) |
| render.rs:104:5 | replace envelope_caption -> Option<String> with Some("xyzzy".into()) |
| render.rs:132:5 | replace coverage_summary -> String with String::new() |
| render.rs:132:5 | replace coverage_summary -> String with "xyzzy".into() |
| render.rs:154:5 | replace stdout_color -> bool with false |
| render.rs:156:53 | replace != with == in stdout_color |
| render.rs:162:5 | replace stderr_color -> bool with true |
| render.rs:162:5 | replace stderr_color -> bool with false |
| render.rs:164:9 | replace && with \|\| in stderr_color |
| render.rs:163:9 | replace && with \|\| in stderr_color |
| render.rs:164:53 | replace != with == in stderr_color |
| render.rs:270:30 | replace == with != in coverage_table_styled |
| render.rs:291:25 | replace != with == in coverage_summary_styled |
| render.rs:293:12 | delete ! in coverage_summary_styled |
| render.rs:299:40 | replace && with \|\| in coverage_summary_styled |
| render.rs:299:18 | replace == with != in coverage_summary_styled |
| render.rs:299:43 | delete ! in coverage_summary_styled |
| render.rs:329:5 | replace wire -> String with String::new() |
| render.rs:329:5 | replace wire -> String with "xyzzy".into() |
| render.rs:338:5 | replace latency -> String with String::new() |
| render.rs:338:5 | replace latency -> String with "xyzzy".into() |
| render.rs:343:5 | replace fingerprints -> String with String::new() |
| render.rs:343:5 | replace fingerprints -> String with "xyzzy".into() |
| render.rs:345:20 | replace match guard v.is_empty() with true in fingerprints |
| render.rs:345:20 | replace match guard v.is_empty() with false in fingerprints |

conductor-core (shard 1/4) — 6:
<!-- rows: 6 -->
| site | mutation |
|---|---|
| obs.rs:224:41 | replace - with + in civil_from_unix |
| obs.rs:224:41 | replace - with / in civil_from_unix |
| obs.rs:226:48 | replace - with + in civil_from_unix |
| obs.rs:226:48 | replace - with / in civil_from_unix |
| obs.rs:226:33 | replace + with - in civil_from_unix |
| obs.rs:226:33 | replace + with * in civil_from_unix |

conductor-tauri — 3:
<!-- rows: 3 -->
| site | mutation |
|---|---|
| main.rs:18:5 | replace main with () |
| commands.rs:272:8 | delete ! in start_run |
| commands.rs:307:5 | replace run_thread with () |

conductor-run — 5:
<!-- rows: 5 -->
| site | mutation |
|---|---|
| lib.rs:68:8 | delete ! in preflight |
| lib.rs:362:5 | replace declares -> bool with false |
| lib.rs:559:27 | delete field degraded from struct Observation expression in execute_scenario |
| lib.rs:600:25 | replace - with / in execute_scenario |
| lib.rs:600:25 | replace - with + in execute_scenario |

conductor-verify — 14 (+ 2 timeouts):
<!-- rows: 16 -->
| site | mutation |
|---|---|
| bin/stub_pulse_mcp.rs:18:5 | replace main with () |
| bin/stub_pulse_mcp.rs:37:60 | replace \|\| with && in main |
| bin/stub_pulse_mcp.rs:45:5 | replace stub_result -> Value with Default::default() |
| bin/stub_pulse_mcp.rs:46:9 | delete match arm "initialize" in stub_result |
| bin/stub_pulse_mcp.rs:51:9 | delete match arm "tools/list" in stub_result |
| bin/stub_pulse_mcp.rs:55:9 | delete match arm "tools/call" in stub_result |
| bin/stub_pulse_mcp.rs:57:21 | replace == with != in stub_result |
| bin/stub_pulse_mcp.rs:69:28 | replace == with != in stub_result |
| client.rs:157:9 | replace ReadbackClient::resolve_incident -> Result<Value, VerifyError> with Ok(Default::default()) |
| extract.rs:95:22 | replace == with != in observe |
| extract.rs:110:22 | replace == with != in observe |
| manifest.rs:35:9 | replace ContractManifest::default_path -> PathBuf with Default::default() |
| spawn.rs:116:5 | replace sidecar_resolves_on_path -> bool with true |
| spawn.rs:116:5 | replace sidecar_resolves_on_path -> bool with false |
| jsonrpc.rs:49:30 (TIMEOUT) | replace != with == in JsonRpcSession::request |
| jsonrpc.rs:70:9 (TIMEOUT) | replace JsonRpcSession::write_message -> Result<(), VerifyError> with Ok(()) |

## Below threshold — no action
- new-cycle: 0 → 0.
- duplication-up: 3.58 → 3.43 % (−0.15 pt; the rule needs ≥ +0.5 pt AND +15 %).
- complexity-creep: over_ceiling 6 → 6; cyclomatic p50/p90 1 / 4, cognitive p50/p90 0 / 1 — all unchanged.
- dead-growth: 32 → 35 (+3; the rule fires at +5) — and the +3 is attributable to false positives (see Informational).
- coverage-drop: +1.83 pt (rule fires at −2).
- mutation-drop: conductor-run +22.22 pt; conductor-verify −0.76 pt (rule fires at −10).
- over_800: 2 → 2 · cross_unit_edges: 16 → 16 · cycles: 0 → 0.
- trend-break: none — jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-machete 0.9.2 · cargo-mutants 27.1.0 · cargo-llvm-cov 0.8.5 · cargo-nextest 0.9.133, all identical to the baseline record.

## Skips
- dead-code-web — tool-missing: knip absent (not in `package.json`, not in `node_modules`) → `npm i -D knip` in `crates/conductor-tauri/ui`.
- mutation-web — tool-missing: StrykerJS absent → `npm i -D @stryker-mutator/core` in `crates/conductor-tauri/ui`.
- complexity-web — declined: lizard 1.24.0 IS available (`python -m lizard`); not collected so the Rust-only complexity series stays comparable with the three prior records — a separate web series can start at any boundary.
- mutation-conductor-core — budget-exhausted: 504 mutants in the unit; the 15-min cap was honored by running `--shard 1/4` (126 mutants, all tested in 380 s) rather than killing cargo's child tree at the cap; shards 2–4 (378 mutants) untested this run.
- mutation-conductor-timeline / -emit / -faults / -report — declined: untouched in `29c30b0..HEAD`.

## Notes for the next boundary (recorded, not proposed)
- This record stores the FULL clone-pair list (`duplication.all_pairs`, 90) and the FULL dead-candidate list (`dead.all_candidates`, 35) beside the capped tops — the prior records' caps are why neither M2's four new pairs nor the dead entrants could be named exactly this time.
- The A5 collector table's false-positive families could gain a fifth: identifiers referenced only through inline format-string captures (`format!("{NAME}")`), which the SCIP index does not surface as references.
- The two `jsonrpc.rs` timeouts have now crossed two boundaries unowned; a timeout has no slot in the score formula, which is precisely why it needs a name here.
- `corrections: []` — no measured-fact correction to a prior record is known at render time.
