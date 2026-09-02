# Code Audit — Conductor · Epoch 5 "Verification surfaces" · 2026-09-02T16:19:36Z
mode **trend** · HEAD `29c30b0` · baseline `b8f3332` (Epoch 4 — Lifecycle & delegated timing) · span **1**

No ancestry break. **No trend-break:** every collector's version matches the Epoch-4 record exactly
(jscpd 5.0.16 · tokei 14.0.0 · rust-code-analysis 0.0.25 · cargo-machete 0.9.2 · cargo-mutants 27.1.0 ·
cargo-llvm-cov 0.8.5 · cargo-nextest 0.9.133), so every single-epoch threshold is live.

**The epoch is structurally quiet.** Nine of eleven checks land below threshold and three metrics
improved. Two proposals follow, and the first is not a metric movement at all — it is a measurement
the tier could not take.

| metric | E4 | E5 | Δ |
|---|---|---|---|
| totals loc / files | 17518 / 111 | 18113 / 115 | +595 / +4 |
| duplication pct | 3.66 | **3.58** | −0.08 |
| duplication clones | 84 | 86 | +2 |
| complexity over-ceiling | 6 | 6 | 0 |
| complexity max (cognitive) | 34.0 | 35.0 | +1.0 |
| graph cycles | 0 | 0 | 0 |
| graph cross-unit edges | 16 | 16 | 0 |
| dead-code candidates | 33 | **32** | −1 |
| coverage line | 92.62 | 92.11 | −0.51 |
| sizes file_p50 / p90 | 104 / 360 | 102 / 379 | −2 / +19 |
| **sizes file_max** | 1012 | **1115** | **+103** |
| sizes over_800 | 2 | 2 | 0 |
| churn pct | 11.81 | 33.63 | +21.82 |
| mutation conductor-run | 70.59 | 72.22 | +1.63 |
| mutation conductor-verify | 88.99 | 89.09 | +0.10 |
| mutation conductor-tauri | — | **no score** | tier aborted |

---

## Proposals

### M1 — `mutation-tier-aborted` · mutation.conductor-tauri — 43 mutants planned, **0 tested**

**Movement:** no prior score → no score. `cargo-mutants` exit 4:
`cargo test failed in an unmutated tree, so no mutants were tested`.

`conductor-tauri` is the epoch's most-touched unit (13 changed source files of the 22 touched
overall), and it is the one unit that produced no test-quality measurement.

**Evidence — the resolution chain, verified end to end at source:**

| # | fact | source |
|---|---|---|
| 1 | The failing test is `commands::tests::path7_the_two_surfaces_write_an_equal_envelope_into_one_runs_db` | `_mutants-conductor-tauri.log`; `crates/conductor-tauri/src/commands.rs:487` |
| 2 | It is a **unit test inside a bin target** (`#[cfg(test)]` at `commands.rs:342`); the crate has **no `tests/` directory** | `ls crates/conductor-tauri/tests/` → absent |
| 3 | It calls `assert_cmd::Command::cargo_bin("conductor")` | `commands.rs:516` |
| 4 | `cargo_bin_str` reads `CARGO_BIN_EXE_conductor`, and **falls back** to `legacy_cargo_bin` when unset | `assert_cmd-2.2.2/src/cargo.rs:236-244` |
| 5 | `legacy_cargo_bin` resolves `<target_dir>/conductor.exe` from `current_exe()`, returning `None` if absent → `missing_cargo_bin` panics with the observed message | `cargo.rs:275-292`, `:251-273` |
| 6 | **`conductor-tauri` declares no dependency on `conductor-cli`**, the package that produces the `conductor` binary — the only mention is a comment in its `Cargo.toml` | `crates/conductor-tauri/Cargo.toml:26`; `crates/conductor-cli/Cargo.toml:8-10` |

**Where it is green, and why — measured, not inferred:**

| invocation | result | why |
|---|---|---|
| `cargo llvm-cov nextest --workspace` (this audit) | **767/767 passed**, `path7_*` PASS in 0.293s | `--workspace` builds `conductor-cli`'s bin, so `target/debug/conductor.exe` exists |
| `cargo nextest run -p conductor-tauri` (this audit) | **13/13 passed** | the fallback found `target/debug/conductor.exe`, dated 16:11 — a **pre-existing artifact**, not one this invocation built |
| `cargo mutants -p conductor-tauri` (fresh scratch tree) | **abort** | fresh target dir; `conductor.exe` was never built there, so both the env var and the fallback fail |

**Suspected shape:** the test's CLI arm binds to a build artifact that no edge in Cargo's dependency
graph guarantees. It is genuinely green under the project's own gate — the chunk's `nextest 767
passed` claim is accurate and this audit reproduced it independently — but the greenness is
contingent on a sibling package having been built, not on anything the test or its manifest
declares. Any fresh-tree, per-package build observes the panic. This is the second distinct failure
of this same parity test: the evolve diagnosis run earlier today recorded that its predecessor
never ran the CLI at all (v2-25); the replacement does run it, through an undeclared path.

**Proposal:** two change-directions, not exclusive. (a) Move the test to
`crates/conductor-tauri/tests/` — an integration-test target, which is the case Cargo sets
`CARGO_BIN_EXE_conductor` for and exactly what assert_cmd's own panic text prescribes; that makes
the binding declared rather than incidental. (b) If it must stay a unit test, add the explicit
`conductor-cli` dev-dependency (or an `artifact-dependency`) so the binary's presence is a graph
fact. Either would also restore `conductor-tauri` to the mutation tier, which currently has no
test-quality measurement for the epoch's most-churned crate.

### M2 — `monotonic` · sizes.file_max — worsened at BOTH of the last two diffs

**Movement:** 898 → 1012 → **1115** code lines (Epoch 3 → Epoch 4 → Epoch 5). The file is
`crates/conductor-run/src/lib.rs` at all three boundaries. Raw line counts, for cross-check:
1196 → 1352 → 1530.

This is the only threshold in the table that fires on the metrics themselves, and it is the only
`monotonic` hit among the five scalars tested (duplication.pct, complexity.over_ceiling,
dead.zero_ref_candidates, sizes.file_max, sizes.over_800).

**Evidence — an independent metric converges on the same file.** 24 of the 25 `conductor-run`
mutation survivors live in it. All 25 rows:

| # | site | surviving mutation | age |
|---|---|---|---|
| 1 | `lib.rs:66:8` | `delete ! in preflight` | carried |
| 2 | `lib.rs:197:21` | `replace < with <= in attribute_by_liveness` | **new** |
| 3 | `lib.rs:250:5` | `replace active_incident_ids -> Result<Vec<i64>, VerifyError> with Ok(vec![])` | **new** |
| 4 | `lib.rs:250:5` | `… with Ok(vec![0])` | **new** |
| 5 | `lib.rs:250:5` | `… with Ok(vec![1])` | **new** |
| 6 | `lib.rs:250:5` | `… with Ok(vec![-1])` | **new** |
| 7 | `lib.rs:346:5` | `replace observe_run_contract -> RunContractStatus with Default::default()` | carried |
| 8 | `lib.rs:358:5` | `replace declares -> bool with true` | carried |
| 9 | `lib.rs:358:5` | `replace declares -> bool with false` | carried |
| 10 | `lib.rs:360:21` | `replace \|\| with && in declares` | carried |
| 11 | `lib.rs:360:11` | `replace == with != in declares` | carried |
| 12 | `lib.rs:360:26` | `replace == with != in declares` | carried |
| 13 | `lib.rs:401:5` | `replace warm_up_canary_service -> anyhow::Result<()> with Ok(())` | carried |
| 14 | `lib.rs:402:29` | `replace \|\| with && in warm_up_canary_service` | carried |
| 15 | `lib.rs:402:24` | `replace == with != in warm_up_canary_service` | carried |
| 16 | `lib.rs:402:55` | `replace == with != in warm_up_canary_service` | carried |
| 17 | `lib.rs:405:64` | `replace / with % in warm_up_canary_service` | carried |
| 18 | `lib.rs:405:64` | `replace / with * in warm_up_canary_service` | carried |
| 19 | `lib.rs:518:27` | `delete field degraded from struct Observation expression in execute_scenario` | carried |
| 20 | `lib.rs:559:25` | `replace - with + in execute_scenario` | carried |
| 21 | `lib.rs:559:25` | `replace - with / in execute_scenario` | carried |
| 22 | `lib.rs:715:9` | `replace FaultKind::label -> &'static str with ""` | carried |
| 23 | `lib.rs:715:9` | `replace FaultKind::label -> &'static str with "xyzzy"` | carried |
| 24 | `lib.rs:753:5` | `replace fault_span -> Option<tracing::Span> with None` | carried |
| 25 | `dispatch.rs:163:26` | `replace == with != in Dispatcher<'a>::dispatch` | carried |

**The score movement hides the survivor movement.** `conductor-run` rose 70.59 → 72.22, but:

| | E4 | E5 |
|---|---|---|
| survivor rows | 20 | 25 |
| distinct mutation texts | 18 | 23 |
| carried from baseline | — | **18 (every one)** |
| **killed since baseline** | — | **0** |
| new this epoch | — | 5 |
| mutants planned | 92 | 118 |

The score improved because the *denominator* grew — 26 more mutants, whose other cases were caught.
Not one standing survivor was killed. Six of the six survivors in `conductor-run` and
`conductor-verify` that are new this epoch sit in code the P-075 chunk added
(`attribute_by_liveness`, `active_incident_ids` ×4, `resolve_incident`).

Six of the 18 carried survivors are already **ratified accepted-deliberate** — the Epoch-4 record's
`accepted_deliberate` field records the `declares` class (6 survivors, ratified 2026-08-21, against
`.claude/rules/testing.md:19`), on the ground that env is read at the caller and killing them would
need unsafe env writes in a shared-process test module. That leaves 12 carried survivors with no
recorded disposition.

**Suspected shape:** `conductor-run/src/lib.rs` is the composition root, and it grows every epoch
while its own test coverage does not follow. It is simultaneously the largest file, the sole
`monotonic` hit, and the home of 24 of 25 survivors in its crate.

**Proposal:** two directions. (a) Split the composition root along a seam the survivor clusters
already suggest — `warm_up_canary_service` + `declares` + `observe_run_contract` (the preflight/
contract cluster, survivors 7-18) is a distinct concern from `execute_scenario` + `fault_span` +
`FaultKind::label` (the run cluster, survivors 19-24). (b) Independently of any split, give the 12
carried-without-disposition survivors either killing tests or an `accepted_deliberate` entry citing
a rule, which is what `.claude/rules/testing.md:19` already asks for and what the 6 `declares`
survivors already have. Note (b) is the cheaper half and does not depend on (a).

---

## Informational

- **churn 11.81% → 33.63%** (files_churned 3 → 7, files_touched 22, 3313 adds of which 1114 are
  churn). No threshold is defined for churn in the table; reported as movement. The epoch's
  most-touched file is `crates/conductor-tauri/ui/wdio.conf.ts` at 5 commits — consistent with an
  epoch whose subject was standing up webview verification.
- **Top-N entrant, sizes:** `crates/conductor-tauri/src/commands.rs` (448 code lines) enters the
  top-10 files.
- **Top-N entrant, fan-in:** `conductor-verify common/StubConfig#` at 35 distinct callers — a test
  stub, consistent with the read-back seam hardening.
- **Top-N entrants, hotspots (8):** `conductor-run/tests/lifecycle_live.rs` (10.0),
  `conductor-tauri/src/commands.rs` (6.0), `conductor-run/tests/lamps_fixture.rs` (3.0),
  `conductor-run/tests/envelope_fixture.rs` (1.0), `conductor-verify/tests/readback.rs` (1.0),
  `ui/test/a11y/accessibility.e2e.ts` (0.0), `ui/wdio.conf.ts` (0.0), `conductor-verify/src/lib.rs`
  (0.0).
- **Hotspot scoring is blind to the TS plane.** The three `.ts`/`.tsx` entrants score 0.0 because
  the formula multiplies commits by *Rust* cognitive complexity and the KLOC fallback is also
  Rust-only. `wdio.conf.ts` was the single most-touched file of the epoch and scores 0. The Epoch-4
  record has the same limitation, so the trend is comparable — but the metric cannot see the plane
  where this epoch did most of its work.
- **`lizard` 1.24.0 is on PATH** (via `python -m lizard`; the pip script shim is not). The Epoch-4
  record files `complexity-web` as `tool-missing`, which may have been a probe artifact rather than
  an absent tool. Not collected this run: adding a TS population mid-trend would break comparability
  with both prior records. A separate web-complexity series can start at any boundary the founder
  chooses, and would also give the hotspot formula a non-zero score for the TS plane.
- **The 2 `conductor-verify` timeouts are unchanged and now unowned across all three boundaries** —
  `jsonrpc.rs:49:30` (`replace != with == in JsonRpcSession::request`) and `jsonrpc.rs:70:9`
  (`replace JsonRpcSession::write_message -> Result<(), VerifyError> with Ok(())`). The Epoch-4
  record's `carried_follow_ups` names this as `mutation-timeouts`, `since: boundary #1 (8cba57d)`,
  `owner: null`.
- **`conductor-verify` survivors: 1 killed, 1 new, net flat.** Killed:
  `mark_incident_resolved -> Ok(Default::default())` — the method gained its first production caller
  this epoch. New: `resolve_incident -> Ok(Default::default())` — the new typed wrapper added
  alongside it inherits the same untested-return shape its sibling just shed.
- **`cargo machete`: unchanged.** One unused dependency, `conductor-emit → conductor-core`,
  identical to the Epoch-4 record.

---

## Corrections to prior records (dated line, per audit-pass.md §Corrections)

**2026-09-02 — target record `b8f3332` (Epoch 4), field `commands.complexity`.** The recorded firing
form is `rust-code-analysis-cli --metrics -O json -o {run_dir}/_rca_head -p crates`. Replayed
verbatim it **aborts**: `Error: The output parameter must be a directory` — the tool requires the
output directory to pre-exist. The triple is therefore broken in the same way the Epoch-3 mutants
command was (the `command-field-omission` follow-up that record itself carries). This record's
`commands.complexity` carries the working form (`mkdir -p {run_dir}/_rca_head && …`). No measured
value is affected — only reproducibility. Recorded here rather than by editing `b8f3332`, which is
immutable.

## Collection-method corrections (this run's own recipes — pinned forward)

Three summarizer defects were found and corrected mid-run. Each would have produced a *plausible
wrong number*, and the corrected form is written into this record's `recipes` so the next boundary
inherits it rather than re-deriving it. Recorded because the ledger's value is comparability, and
each of these silently breaks it.

1. **Dead-code tests filter — would have fired a false `dead-growth` proposal.** Excluding a `tests`
   path segment by splitting the raw symbol misses inline `#[cfg(test)]` symbols, because the SCIP
   prefix (`rust-analyzer cargo {crate} {version} `) fuses into the first segment and the marker is
   a *leading* `tests/`. Uncorrected, 21 inline test functions in `conductor-run/src/lib.rs` counted
   as dead code: **53 candidates instead of 32** — a +20 movement against a +5 threshold. The
   collectors reference warns about exactly this ("Reuse verbatim or `dead-growth +5` fires on
   filter drift, not real growth"); the warning paid for itself on its first re-use.
2. **Fan-in symbol form — would have reported 20 false top-N entrants.** The Epoch-4 record stores
   the shortened `{crate} {path}` form; comparing raw SCIP strings makes every row look new. After
   normalizing, there is exactly **1** real entrant.
3. **Mutation completeness predicate — read a half-finished run as complete.** `total_mutants` in
   `outcomes.json` is a *rolling* field that tracks progress, not the plan. Reading it as the
   denominator reported `conductor-run` at 70/70 tested with a score of 58.49 when the plan held
   118 and the true score was 72.22 — a 13.7-point error that would have fired a false
   `mutation-drop` proposal. The plan is `mutants.json`; the predicate now uses it.

---

## Below threshold — no action

- `duplication-up` — 3.66 → 3.58 pct (needs ≥ +0.5pt AND ≥ +15% relative). **Improved.** Clone
  count 84 → 86, duplicated lines 853 → 870, split 41 both-test / 37 both-src / 8 mixed.
- `complexity-creep` — over_ceiling 6 → 6 (needs ≥ +3 AND ≥ +25%). The six over-ceiling functions
  are unchanged in identity: `serve_stub` (35.0), `shape_is_realizable` (31.0), `validate` (18.0),
  `observe` (17.0), `dispatch` (16.0), `run_preflight` (16.0). Percentiles flat (cyclomatic p50/p90
  1.0/4.0; cognitive p50/p90 0.0/1.0).
- `new-cycle` — 0 → 0 cycles over 16 cross-unit edges, both unchanged. Fan-out per unit unchanged
  (conductor-run 6, conductor-cli 4, conductor-tauri 2, the rest 1).
- `dead-growth` — 33 → 32 candidates (needs ≥ +5). **Improved.** Chain 772 → 36 → 32. Unused deps
  unchanged at 1.
- `coverage-drop` — 92.62 → 92.11 line (needs ≤ −2pts). 8398 of 9117 lines hit; branch coverage
  unavailable, as at baseline.
- `mutation-drop` conductor-run — 70.59 → 72.22 (needs ≤ −10pts). See M2: the score rose while zero
  standing survivors were killed.
- `mutation-drop` conductor-verify — 88.99 → 89.09 (needs ≤ −10pts).

---

## Skips

| metric | reason | note |
|---|---|---|
| dead-code-web | tool-missing | knip absent → `npm i -D knip` in `crates/conductor-tauri/ui` |
| mutation-web | tool-missing | StrykerJS absent → `npm i -D @stryker-mutator/core` |
| **mutation-conductor-tauri** | **baseline-test-failure** | see **M1**. **Schema gap:** the `skips[]` reason enum (`tool-missing \| stack-absent \| declined \| budget-exhausted \| no-baseline`) has no value for "the tier ran, generated its plan, and aborted on the unmutated baseline". Filing it under any existing value would be false, so a sixth value is used and disclosed here. |
| mutation-conductor-core | declined | not touched in `b8f3332..HEAD` — out of the touched-unit scope |
| mutation-conductor-cli | declined | not touched in span |
| mutation-conductor-report | declined | not touched in span |
| mutation-conductor-timeline | declined | not touched in span |
| mutation-conductor-emit | declined | not touched in span |
| mutation-conductor-faults | declined | not touched in span |

**Carried forward, unchanged:** `known_residue` — `.andromeda/runs/_wrap_tmp`, 7 tracked files,
introduced `01c6dac` (2026-08-15). Still present, still committed, still not this run's damage.

---

*Audit complete. Nothing here is applied, gated on, or remembered — the record is appended to
`.andromeda/code-metrics.ndjson` (now 3 records) and fixes route through the normal channels at the
founder's call.*
