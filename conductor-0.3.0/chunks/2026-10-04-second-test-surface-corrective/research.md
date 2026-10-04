# Codebase Research — 2026-10-04-second-test-surface-corrective

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11 (python sweeps from scratchpad scripts; ugrep refuses or mis-anchors the multi-term forms on this host)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — structural sweep for `clippy` / `PSNativeCommand` / `LASTEXITCODE` / `bundled default` (python `re` over the whole file): 0 additions bear on the `run` verb's failure semantics beyond the `run` line itself (`:19`); no live Pulse leg in this chunk. `.claude/rules/testing.md` (auto-loaded) — the mutation-instrument discipline and the 2026-10-04 child-module split rule apply.
- **Platform issues consulted:** none — no runner-only bullet folded (CI on `dab66dc` was in progress at take-up, nothing red); the one CI fact used is read from a RECORDED run's log (below), not a search summary.

## Live survivor measurement (overseer directive at take-up)

Measured on this host (Linux dev host) at HEAD `dab66dc`, cargo-mutants 27.1.0 (`cargo mutants --version`), each run scoped
by file AND function, output under the gitignored `target/phase-mutants/` (`git check-ignore -q` → ignored):

- `cargo mutants -p conductor-emit -f crates/conductor-emit/src/identity.rs --re 'xor_in_place' --test-tool=nextest --jobs 2 --output target/phase-mutants/emit`
  — 3 mutants; tallies `missed 2 · caught 1 · timeout 0 · unviable 0`, equal to `outcomes.json` and conserving 3.
  `missed.txt`: `replace ^= with &= in xor_in_place` · `replace ^= with |= in xor_in_place` — **both LIVE**.
  `caught.txt`: `replace xor_in_place with ()`.
- `cargo mutants -p conductor-run -f crates/conductor-run/src/canary.rs --re 'emit_canary_storms' --test-tool=nextest --jobs 2 --output target/phase-mutants/run`
  — 6 mutants; tallies `missed 3 · caught 3 · timeout 0 · unviable 0`, equal to `outcomes.json` and conserving 6.
  `missed.txt`: `replace > with >= in emit_canary_storms` · `replace * with + in emit_canary_storms` ·
  `replace * with / in emit_canary_storms` — **all three LIVE**. `caught.txt`: `replace emit_canary_storms -> anyhow::Result<()> with Ok(())`
  · `replace > with < …` · `replace > with == …`.
- Both runs exited **2** while their tallies are complete — the exit code carries no verdict (test-plan §4), read from the
  tallies. Unmutated baseline green in both (10 s / 12 s build).
- Footing: test-plan §9 states the mutation instrument runs "at the epoch-boundary code audit ONLY, never per chunk
  (founder ruling, 2026-09-30)". This research-time measurement ran on the overseer's explicit take-up directive ("Measure
  the three survivors … as LIVE on this host before planning their kills"); whether the kills are PROVEN by a second
  targeted run is a P4 fork (Open questions).

## Files inspected
- `crates/conductor-emit/src/identity.rs` (full, 165 lines) — `rekey_trace_identity` (`:27`) draws a 16-byte trace mask then an 8-byte span mask from `ChaCha8Rng::seed_from_u64(salt)` and XORs every non-empty id through private `xor_in_place` (`:47-51`, `*byte ^= m;` at `:49`). Five inline tests (`:95-164`): linkage + shared trace id, same-salt determinism, distinct salts disjoint, no content byte moved, empty parent stays empty. None applies the re-key twice, so none asserts the XOR involution the doc comment states ("a bijection per salt (XOR with a fixed mask is its own inverse)", `:20`).
- `crates/conductor-run/src/canary.rs` (`:128-240`, `:405-490`) — `emit_canary_storms` (`:197-209`, private) sleeps `REAL_MODEL_CANARY_STORM_GAP` (90 s, `:176`) only `if n > 0` (`:203`) and seeds storm `n` from `base.wrapping_add(n * CANARY_STORM_COUNT)` (`:206`, `CANARY_STORM_COUNT = 12` at `:132`); `emit_canary_storm` (`:216-230`) seeds occurrence `i` with `canary_storm_seed(base, i) = base + i` (`:142`). Inline tests reach the private fn through `drive_storms` (`:436-452`) over a loopback `TraceServiceServer` stub on `127.0.0.1:0` (`stub_collector`, `:409-432`), under `start_paused`. `the_real_model_canary_is_three_distinct_storms_ninety_seconds_apart` (`:468-489`) asserts the export count (36) and storm starts RELATIVE TO THE FIRST arrival — so a leading 90 s sleep before storm 0 shifts every start equally and passes, and it never inspects span identity.
- `crates/conductor-emit/src/exception.rs` (`:180-205`) — `exception_trace_request` derives `trace_id`/`span_id` ONLY from `ChaCha8Rng::seed_from_u64(seed)` (`:185-187`); the spec contributes content, never identity. So identity distinctness across the 36 canary occurrences is exactly seed distinctness.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (full structure, 1222 raw lines; tokei 14.0.0 → 1130 code) — module doc `:1-48`; `mod evidence_pin;` `:52`; grading helpers `:57-342` (`DelegatedBound`, `bounds`, `harvest_since`, `is_target`, `num_field`, `observations`, `grade`, the tick / incident / window helpers, `timestamp_ms`, `days_from_civil`, `HueSample`, `hue_samples_in_window`, `grade_in_window`, `tiered_hue_samples_in_window`, `incident_anchor_error_ms`, `tick_times_ms`, `tick_offset_ms`, `p025_window`); inline `mod tests` `:343-1222` in eight families (below). Reads committed evidence ONLY through `evidence_pin::{committed, pinned, check_digest, sha256_hex}` (`:351-565`); reads no `CONDUCTOR_*` / `ANDROMEDA_*` handle (python sweep for `env::var|CONDUCTOR_|ANDROMEDA_|capture_paths`: 0 hits). Its verbatim Pulse self-obs lines (`:741-830`, `:1093-1190`) are Pulse TRACING lines committed since 2026-08-21 — not corpus-rendered model text — and the split moves them byte-unchanged.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (`:1-40`) + `series_2026_09_29.rs` (`:1-30`) — the split precedent: a `tests/` subdirectory module (never a target of its own), children `use crate::*;`, `#![allow(dead_code)]` at the module root, `#[test]` fns living in the children.
- `scripts/agent-run.sh` (`:20`, `:280-330`) — `set -euo pipefail` (`:20`); the bundled default `""` arm runs nextest, doctest and the three `clippy … -- -D warnings` lines (`:321-325`); the `--e2e` arm (`:297-316`) runs `cargo build --release` + the wdio leg and NO clippy line.
- `scripts/agent-run.ps1` (`:12`, `:368-390`) — `$ErrorActionPreference = 'Stop'` (`:12`); the bundled default `''` arm runs the same five cargo lines (`:373-379`) with NO `$LASTEXITCODE` check after any of them, and the script never sets `$PSNativeCommandUseErrorActionPreference` (python sweep: 0 hits). A native non-zero therefore aborts it ONLY when the CALLER set that preference.
- `.github/workflows/ci.yml` (`:105-117`) — the `rust` job's "Test + lint (dogfood agent-run)" step (`shell: pwsh`) sets `$PSNativeCommandUseErrorActionPreference = $true` before `.\scripts\agent-run.ps1 run`.
- CI run 34689135760 (sha `e3ff4e5f`, `gh run view --log-failed`, the only failed dogfood step among the 17 non-probe failed runs listed by `gh run list --workflow ci.yml --status failure --limit 40` → `gh api …/jobs`) — the step died at `NativeCommandExitException … agent-run.ps1:280 … Program "cargo.exe" ended with non-zero exit code: 100` on a nextest red, `##[error]Process completed with exit code 1`. The recorded witness that, UNDER the CI preference, a native non-zero in the bundled default aborts the step. No CI run has ever carried a clippy-only red (every other failure in that listing is the a11y job, or `Supply-chain — cargo audit` at `30156520449`).
- `scripts/mutation-roster.toml` (`:1-80`) — `[[unit]]`s: `conductor-cli`, `conductor-verify`, `conductor-emit` (scoped `files` = `exception.rs`, `latency.rs`); no row names `identity.rs` or `canary.rs` (python sweep: 0 hits). `conductor-run` is not a registered unit. So the kills write no roster row and no citation-home entry.
- `.andromeda/runs/2026-10-04T09-27-47-code-audit/proposals.md` (M2 and the Coverage/Mutation bullets) — re-read; coordinates and descriptions as the entry folded them.

## Graph impact (from the code-graph query)
Trace: `.andromeda/runs/2026-10-04T12-48-50-phase/tree-query-2026-10-04-second-test-surface-corrective.json`, plane `rust`, `db_state: fresh`, `rows: 31` (`calls` by `callee_name IN (xor_in_place, rekey_trace_identity, emit_canary_storms, emit_canary_storm, canary_storm_specs)`, `callee_kind = 'fn'`; lines cited +1).
- **xor_in_place** — 5 callers, all `rekey_trace_identity` @ `crates/conductor-emit/src/identity.rs:37,38,39,41,42`. Private leaf; the kill lands at `rekey_trace_identity`'s unit tier.
- **rekey_trace_identity** — production caller `Dispatcher::export_traces` @ `crates/conductor-run/src/dispatch.rs:244`; re-export @ `crates/conductor-emit/src/lib.rs:31`; 7 test call sites in `identity.rs`'s own module. No signature change → no threading.
- **emit_canary_storms** — 2 callers: `emit_canary` @ `crates/conductor-run/src/canary.rs:300` (production) and the inline test helper `drive_storms` @ `canary.rs:439`. Reachable ONLY from the inline module (private fn), and reachable there through an injectable `TraceEmitter` over a loopback stub — never the fixed `:4317` (the test-plan §10 `member-3` reachability trap does not apply).
- **emit_canary_storm** — called by `emit_canary_storms` (`canary.rs:206`) and by `canary_wire.rs` (`:38`, `:159`, `:209`), `canary_obs_witness.rs` (`:46`) — unchanged by this chunk.
- No signature changes anywhere → no caller threading.

## Patterns detected
- **Child modules of ONE target** (`crates/conductor-run/tests/real_model_grading/mod.rs:1-7`): a `tests/<name>/mod.rs` subdirectory module is never a cargo target; children `use crate::*;` and see the root's private items; the root sees a child only through `pub(crate)`.
- **Inline-module reach for a private fn** (`crates/conductor-run/src/canary.rs:436-452`): `drive_storms` drives `emit_canary_storms` against `stub_collector` under `#[tokio::test(flavor = "current_thread", start_paused = true)]`; the stub's `requests` log holds every `ExportTraceServiceRequest`, so span ids are readable per occurrence.
- **Identity is a function of the seed alone** (`crates/conductor-emit/src/exception.rs:185-187`).

## Conventions to follow
- **Mutation kills by observable value**: assert on a returned identity / count / arrival instant, never by adding a log field (obs-plan §4; `.claude/rules/testing.md` 2026-08-20 "what the mutant makes DIFFERENT on the surface you observe").
- **Paused-clock timing assertions** measure virtual instants from a stamp taken BEFORE the call (`tokio::time::Instant::now()`), never wall-clock (`.claude/rules/testing.md` §Determinism).
- **A split keeps the target name and every cited test path** (`.claude/rules/testing.md` 2026-10-04; obs-plan §4 cites `delegated_timing_harvest::tests::p075_round_assertion_{3..6}_*` and `…::tests::p075_reround_assertion_{3..6}_*`, `delegated_timing_harvest.rs::p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value`, `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency`; matrix v3-08's `ref` cites `delegated_timing_harvest.rs::tests::p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value`; the closed 0.2.0 matrix v2-20 cites `delegated_timing_harvest.rs::{p027_…, p037_…, p045_…}` by file).

## Split sizing (tokei-rule count over HEAD's line spans; doc comments counted as code, plain `//` not — the scratchpad counter reads 1070 where tokei reads 1130, so figures are a floor to be re-measured by tokei)
| family | lines | code | cited live? |
|---|---|---|---|
| module doc + uses | 1-56 | 53 | — |
| grading helpers | 57-342 | 264 | `grade_in_window` named in obs-plan §4 / v3-08 by NAME only |
| tests prologue + p025 window derivation tests | 343-372 | 27 | no |
| P-075 round | 373-464 | 80 | yes — `::tests::p075_round_assertion_*` |
| P-075 re-round + digest tamper | 465-570 | 93 | yes — `::tests::p075_reround_assertion_*` |
| synthetic bound unit tests | 571-731 | 150 | no |
| 2026-08-21 legs | 732-842 | 97 | yes — v2-20 by file (closed 0.2.0 ledger) |
| P-025 window-selection synthetics | 843-952 | 84 | no |
| P-025 hard-grade synthetics | 953-1081 | 111 | no |
| 2026-09-07 re-driven leg | 1082-1153 | 55 | yes — obs-plan §4 by name |
| 2026-09-29 graded leg | 1154-1222 | 56 | yes — obs-plan §4 + v3-08 `::tests::` |

Keeping helpers in the root leaves ~725 counted (≈ 766 tokei) — under 800 by a thin margin; moving the helpers and the three uncited synthetic families into `tests/delegated_timing_grading/` leaves the root ≈ 461 counted (≈ 490 tokei) with EVERY cited test at its current path. Baseline at HEAD: `cargo nextest list -p conductor-run --test delegated_timing_harvest` → 36 tests, all under `tests::`; `cargo nextest run -p conductor-run --test delegated_timing_harvest --profile ci` → `36 tests run: 36 passed, 0 skipped` (exit 0).

## Sweep record
- `delegated_timing_harvest` (python scan of `git ls-files`, live masters / registries / rules / docs / contracts / crates / scripts / the 0.3.0 matrix and route; run dirs, chunk dirs and sidecars excluded): 26 hits in 9 files · 0 changed by the chosen shape · 26 no-change (every cited test stays at its path; the target keeps its name; `code-metrics.ndjson` is a metrics record).
- every fn name in the harvest file against the same live set (`cited_tests.py`): only `p025_the_re_driven_leg_…` (obs-plan) and `p025_the_graded_leg_…` (obs-plan, 0.3.0 matrix) hit by exact name; the P-075 families are cited by `{3..6}_*` pattern under `::tests::`; every synthetic-family test name: 0 live hits.
- `xor_in_place` / `emit_canary_storms` in `scripts/mutation-roster.toml`: 0 hits.

## New files to create
- `crates/conductor-run/tests/delegated_timing_grading/mod.rs` — the grading helpers moved verbatim from the root (`pub(crate)` on each item the root's tests use), plus the child-module declarations
- `crates/conductor-run/tests/delegated_timing_grading/synthetic_bounds.rs` — the synthetic bound unit tests moved verbatim (`:571-731` at HEAD)
- `crates/conductor-run/tests/delegated_timing_grading/window_selection.rs` — the P-025 window-selection synthetics moved verbatim (`:843-952`)
- `crates/conductor-run/tests/delegated_timing_grading/p025_hard_grade.rs` — the P-025 hard-grade synthetics moved verbatim (`:953-1081`)
- `conductor-0.3.0/chunks/2026-10-04-second-test-surface-corrective/evidence/` — the liveness tallies before and after the kills, the split's before/after test-name lists, and the CARRY measurement record

## Files to modify
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — helpers and the three uncited synthetic families move out; `mod delegated_timing_grading;` + a glob use; every cited family stays in `mod tests`
- `crates/conductor-emit/src/identity.rs` — an inline test asserting the per-salt involution (re-key twice under one salt restores every id byte-exact)
- `crates/conductor-run/src/canary.rs` — inline tests asserting storm 0 arrives at the call's own virtual instant, and that all 36 real-model occurrences carry distinct span identities
- `scripts/agent-run.ps1` — only if P4's CARRY fork takes the self-set preference line (see Open questions)

## Open questions
- The CARRY's ps1 half: `agent-run.ps1 run` blocks on a red cargo line only when its CALLER sets `$PSNativeCommandUseErrorActionPreference` (CI does; a bare local invocation does not, and there is no `pwsh` on this host to measure it). Fix it in the script (one line, so every caller gets CI's semantics) or record it as the measured-by-reading residual and amend obs §10 per surface? → blocks: plan-decision
- How the kills are PROVEN at implement: a second targeted `cargo mutants` run of the same two scoped forms (the before/after twin of the overseer-directed measurement) against test-plan §9's epoch-boundary-only ruling, or inverse controls only (hand-apply each mutation, the new test fails, restore)? → blocks: plan-decision
