# Code Audit — Conductor · Epoch 3 "Live proof: the five families" · 2026-08-20T18:33:23Z
mode **baseline** · HEAD `8cba57d` · baseline **none** · span **n/a**

First record in `.andromeda/code-metrics.ndjson`. **Trend judgments begin at the next epoch
boundary** — everything below is absolute, and no threshold in the table could fire except
`new-cycle` (which did not: cycles = 0). Findings are obligation-free: accept, reject, defer, or
modify with no mechanism-side consequence. Nothing here has been applied, and the audit never
edits the code it measures.

Two collection notes that shape how the numbers below should be read:

- **`crate_edges` is symbol-reference derived, not manifest derived.** It disagrees with the Cargo
  manifests in both directions (§B4). Both graphs were computed; both are acyclic.
- **Three metric counts required re-derivation before they were reportable** — the dead-code
  candidate count (§B5), the duplication scope split (§B7), and the mutation baseline itself
  (§B3). The naive value in each case was wrong in a direction that looked plausible.

---

## Baseline findings

### B1 — mutation · `conductor-run` — score **58.82%** (40 caught / 28 missed / 0 timeout; 23 unviable)
**Reading:** line coverage for this crate is **79.68%**. Twenty-one points of covered lines execute
without anything asserting on their behavior — the hollow-coverage gap mutation exists to expose.
Survivors concentrate in five functions rather than scattering, which makes them addressable.

**Evidence — all 28 survivors** (`c-mutation-conductor-run.json`):

| site | mutation | note |
|---|---|---|
| `lib.rs:218:5` ×2 | `declares -> bool` with `true`, with `false` | **the run-contract `shell-declaration` predicate** |
| `lib.rs:220:21` | `\|\|` → `&&` in `declares` | " |
| `lib.rs:220:26`, `:220:11` | `==` → `!=` in `declares` | " |
| `lib.rs:261:5` | `warm_up_canary_service -> Result<()>` with `Ok(())` | whole function replaceable |
| `lib.rs:262:29` | `\|\|` → `&&` in `warm_up_canary_service` | |
| `lib.rs:262:24`, `:262:55` | `==` → `!=` in `warm_up_canary_service` | |
| `lib.rs:265:64` ×2 | `/` → `%`, `/` → `*` in `warm_up_canary_service` | |
| `lib.rs:517:5` ×3 | `now_ms -> i64` with `0`, `1`, `-1` | journal stamp source |
| `lib.rs:631:5` ×3 | `now_unix_nanos -> i64` with `0`, `1`, `-1` | journal stamp source |
| `lib.rs:509:5` ×2 | `severity_rank -> u8` with `0`, `1` | |
| `lib.rs:532:9` ×2 | `FaultKind::label` with `""`, `"xyzzy"` | |
| `lib.rs:570:5` | `fault_span -> Option<Span>` with `None` | |
| `lib.rs:409:25` ×2 | `-` → `+`, `-` → `/` in `execute_scenario` | **`observed_ms - emitted_ms`** |
| `lib.rs:378:27` | delete field `degraded` from `Observation` | routes to `KnownResidual` |
| `lib.rs:66:8` | delete `!` in `preflight` | logging-only (see below) |
| `dispatch.rs:163:26` | `==` → `!=` in `Dispatcher::dispatch` | |

**Verified against source, not inferred from mutation text:**
- `lib.rs:218-222` is `declares(name)` → `std::env::var(name).is_ok_and(|v| v == "true" || v == "1")`,
  the predicate behind a **named preflight precondition**. Every one of its five mutations survives.
  A `declares` stuck at `true` would let an unmet run-contract term read as satisfied; the
  architecture requires that arm to surface a distinct `Blocked` naming each unmet term.
- `lib.rs:409:25` is `observed_ms - emitted_ms` — the journal-relative SLO computation. Precisely:
  it sits on the `manual_record` (operator-checklist) branch, so the honest claim is that the
  latency arithmetic **on that path** is unasserted, not that all SLO math is.
- `lib.rs:66` is `if !state.ready { tracing::info!(...) }` and the function returns
  `Preflight { ready: state.ready }` either way. **This survivor changes no observable behavior** —
  listed for completeness, ranked lowest, and deliberately not counted toward the concern above.

**Proposal:** a direction — add killing tests for `declares` first (five survivors on one predicate
that gates a preflight precondition is the densest real signal in the set), then the two stamp
helpers, where asserting that a journal stamp is a plausible `std::time` value would kill six
survivors at once. `severity_rank` / `FaultKind::label` / `fault_span` are constant-replaceable
accessors whose killing tests are cheap but low-value; they are the tail, not the head.

### B2 — mutation · `conductor-verify` — score **72.97%** (81 caught / 28 missed / 2 timeout; 23 unviable)
**Reading:** line coverage 92.89%. **8 of the 28 survivors are in `src/bin/stub_pulse_mcp.rs`, a test
double, not production code** — so the production-code survivor count is **20**. The score is
computed over all mutants including the stub, per the pinned summarizer.

**Evidence — all 28 survivors** (`c-mutation-conductor-verify.json`), grouped:

*Production (20):*
- `jsonrpc.rs:79:27` guard `line.len() > MAX_LINE_BYTES` → `false`; `:79:38` `>` → `==`, `>` → `>=`
  — **three survivors on the read-back line-size bound.** `.claude/rules/security.md` requires this
  bound explicitly ("a soft per-line size bound" on the MCP decode path). Replacing the guard with
  `false` disables it and no test notices.
- `preflight.rs:385:64` `>` → `>=` in `assert_canary` — the **canary freshness comparison**
  (incident opened *after* the storm's emission instant); the boundary itself is unasserted.
- `preflight.rs:335:32` `<` → `==`/`>`/`<=` and `:335:28` `+` → `*` in `poll_canary` — four
  survivors on the poll-deadline arithmetic that the run contract's `min_canary_poll_seconds`
  floor feeds.
- `preflight.rs:357:9` `ShapeWitness::list` → `()`; `:357:12` delete `!`
- `client.rs:149:9` `mark_incident_resolved` → `Ok(Default::default())` — one of the four required
  MCP contract tools.
- `extract.rs:95:22`, `:110:22` `==` → `!=` in `observe`; `:215:5` `log_observed_keys` → `()`
- `jsonrpc.rs:18:41`, `:18:34` `*` → `+`; `:41:22` `+=` → `-=`, `+=` → `*=` in `request`;
  `:65:9` `notify` → `Ok(())`
- `manifest.rs:35:9` `ContractManifest::default_path` → `Default::default()`

*Test double (8):* `stub_pulse_mcp.rs` — `main` → `()`, `stub_result` → `Default::default()`,
deletion of the `"initialize"` / `"tools/list"` / `"tools/call"` match arms, two `==` → `!=`, one
`||` → `&&`.

**Proposal:** a direction — the `MAX_LINE_BYTES` guard is the one survivor family with a written
rule behind it, so a test that feeds an over-long line and asserts the typed `Blocked`/decode error
would close a stated security requirement rather than just raise a number. The `assert_canary`
boundary and the `poll_canary` deadline arithmetic are the next densest. Whether the stub binary
should be in mutation scope at all is a founder call — excluding it would raise this score to
81/101 = 80.2% without changing any production fact.

### B3 — the test suite is **not runner-portable**, and it silently blocked this tier
**Finding:** `conductor-run::canary_wire the_wire_shape_witness_reaches_the_self_obs_artifact`
**passes under nextest and fails under `cargo test`** — same commit, same test binary. Measured
both ways:

```
cargo test    → running 5 tests … the_wire_shape_witness_reaches_the_self_obs_artifact … FAILED
                test result: FAILED. 4 passed; 1 failed
cargo nextest → 5 tests run: 5 passed, 0 skipped
```

**Suspected shape:** the discriminator is process isolation. nextest runs process-per-test;
`cargo test` runs thread-parallel inside one process. The test asserts against the self-observation
artifact — a process-global sink, truncated per invocation. Under nextest it gets a private process
and a clean file; under `cargo test` it shares both with four siblings.

**Why it was invisible:** `CLAUDE.md` pins `cargo nextest run --workspace --profile ci` as the test
command, so the one runner that fails is the one nothing runs. The zero-retry determinism
discipline is fully satisfied — under nextest the test is perfectly deterministic. `conductor-verify`'s
baseline passed cleanly, so this is specific to `conductor-run`, not workspace-wide.

**Consequence for this audit:** cargo-mutants defaults to `cargo test` and aborted with
`ERROR cargo test failed in an unmutated tree, so no mutants were tested`. Re-run with
`--test-tool=nextest` (recorded in both `c-mutation-*.json` `command` fields). Had the abort been
taken at face value, this record would have carried "mutation: 0 units, tool-missing" instead of
two real scores.

**Proposal:** two directions, either sufficient — give that test a per-test temp obs sink so it
stops depending on process isolation; or record `--test-tool=nextest` as the required invocation
wherever `cargo test` is reachable (a mutation runner, an IDE, a CI matrix job). The first removes
the cause; the second documents it.

### B4 — dependency graph — **0 cycles**, 16 cross-unit edges, and a semantic discrepancy
**Cycles: none**, under *both* readings — this is the headline and it holds. The crate-per-seam
design is intact: `conductor-core` is a pure sink (0 outgoing), `conductor-run` is the composition
root (fan-out 6), the two shells sit on top.

Fan-out: `conductor-run` 6 · `conductor-cli` 3 · `conductor-tauri` 2 · `emit`/`faults`/`report`/
`timeline`/`verify` 1 each · `conductor-core` 0.

**The discrepancy:** `crate_edges` is **symbol-reference derived**, not manifest derived, and the
two sets differ in both directions despite both having 16 members:

| | |
|---|---|
| declared in Cargo.toml, absent from graph | `conductor-emit → conductor-core` |
| in graph, not declared and not referenced | `conductor-cli → conductor-verify` |

Verified at source: `conductor-emit`'s only mention of `conductor_core` is a rustdoc intra-doc link
(`error.rs:5`, `[\`conductor_core::CoreError\`]`) — no code reference, hence no symbol edge. No
`conductor_verify` reference exists anywhere under `crates/conductor-cli/`, so its graph edge is
attribution through a re-export. I built the manifest-derived graph separately and confirmed it is
**also acyclic** (9 units, 16 edges, 0 cycles), so the zero-cycle claim is safe either way.

**Unused dependencies** (`cargo-machete`, independently corroborating the graph):
`conductor-emit` → `conductor-core` · `conductor-cli` → `tracing` (no `use tracing`, no
`tracing::`, no `#[tracing`, no bare log macros under `src/`).

**Proposal:** a direction — record which notion `crate_edges` encodes, because the architecture's
claim that "the `Cargo.toml` dependency edges *are* the architecture" is a statement about the
manifest graph, while the audit's cycle check reads the symbol graph. They agree today; nothing
guarantees they will. On the two unused deps: `conductor-emit`'s `conductor-core` is load-bearing
for rustdoc link resolution only, so removing it trades a dependency edge for a broken doc link —
worth a deliberate decision rather than a reflex `cargo remove`.

### B5 — dead code — **31 candidates**, residual **8** after classing
**The raw number is not reportable.** Three successive values, each plausible, only the last
defensible:

| filter | count | why wrong |
|---|---|---|
| none | 672 | includes all test fns and entry points |
| test *file paths* excluded | 508 | Rust inline `#[cfg(test)] mod tests` puts the marker in the **symbol** path, not the file path |
| symbol `%/tests/%` excluded | 47 | misses a *leading* `tests/` segment (`conductor-run tests/foo()`) |
| **`tests/` path segment excluded** | **31** | defensible |

Of the 31, **23 fall into the false-positive classes the collector table names**: 17 trait-impl
methods reached by dispatch rather than by name (tracing's `Visit` record_* family, `Layer`'s
`on_event`/`on_new_span`/`on_close`, `MakeWriter`, `Write`, `Drop`, `PauseResolver::resolve`) and
6 enum error variants constructed by `#[from]` (`EmitError#Status`, `RunsDbError#Io`/`#Json`,
`JournalError#Io`).

**Residual 8** — and several of these are themselves runtime-invoked: `LatencyProfile::p50_ms`/
`p95_ms`/`p99_ms` · `PortOccupier::occupy_default` · `ReadbackClient::mark_incident_*` (one of the
four MCP contract tools — the runtime-invoked class exactly) · `ContractManifest::default_path` ·
`phase_spec::default_occurrences` (a serde `default` fn, invoked by derive) ·
`preflight::RUN_CONTRACT_PRECONDITION`.

**Proposal:** no removal is indicated by this data — the residual is small and mostly explicable.
The useful direction is for the *next* audit: the trend on this number matters far more than its
level, and `dead-growth` fires at baseline + 5, so the classing recipe above should be reused
verbatim or the threshold will fire on filter drift rather than on real growth.

### B6 — complexity — **6 functions over the cognitive ceiling** of 15
1,732 functions. Cyclomatic p50 **1**, p90 **4**, max **47**. Cognitive p50 **0**, p90 **1**, max **31**.
A very flat distribution: 90% of functions have cognitive complexity ≤ 1.

| cognitive | cyclomatic | function | file |
|---|---|---|---|
| 31 | 30 | `shape_is_realizable` | `conductor-core/src/phase_spec.rs` |
| 27 | 26 | `serve_stub` | `conductor-verify/tests/common/mod.rs` *(test helper)* |
| 18 | 16 | `validate` | `conductor-core/src/run_contract.rs` |
| 17 | 13 | `observe` | `conductor-verify/src/extract.rs` |
| 16 | **47** | `dispatch` | `conductor-run/src/dispatch.rs` |
| 16 | 20 | `run_preflight` | `conductor-verify/src/preflight.rs` |
| 15 | 18 | `is_host_path_token` | `conductor-core/src/redact.rs` *(at ceiling, not over)* |

**Suspected shape:** `dispatch` is the outlier worth naming — cyclomatic 47 against cognitive 16 is
the signature of a wide flat `match` (many arms, little nesting), which is the intended shape for an
emission dispatcher and reads as far worse on cyclomatic alone. `shape_is_realizable` is the
genuine complexity peak on both axes.

**Proposal:** no action indicated at these levels. Recorded as the starting table; `complexity-creep`
fires at baseline + 3 **and** +25%, so from 6 the next audit fires at 9.

### B7 — duplication — **3.68%** (807 / 21,944 lines), 79 clones
**Scope split matters here:** 452 duplicated lines are **test↔test**, 366 are **src↔src**, 68 mixed.
The five largest clones (199 lines) are all `conductor-emit/tests/*` — gRPC stub setup boilerplate.

Largest **src-only** clones: `latency.rs`↔`rate.rs` (17, and again 15) · `scenario.rs`↔itself (15,
14) · `span_tree.rs`↔`topology.rs` (15) · `run_record.rs`↔`db.rs` (14) · `logs.rs`↔`pii.rs` (14) ·
`lib.rs`↔`preflight.rs` (14) · `capability_manifest.rs`↔`drift.rs` (13) · `latency.rs`↔`span_tree.rs` (13).

By unit (both sides counted): `conductor-emit` 994 · `conductor-core` 339 · `conductor-run` 202 ·
`conductor-verify` 112 · rest ≤ 44. TS/TSX: **0.00%**.

**Proposal:** none indicated — no production clone exceeds 17 lines. The one shape worth watching
is `conductor-emit`'s test boilerplate: if a sixth emit test family lands, the test↔test half grows
mechanically and will move the headline percentage without any production duplication appearing.

### B8 — sizes — 118 files, **16,543 code lines**, 9 units
File p50 **88**, p90 **347**, max **898**; **2 files over 800**:
`conductor-run/src/lib.rs` (898) and `conductor-core/src/scenario.rs` (850).

Per-unit: core 4,489 · emit 2,817 · run 2,807 · verify 1,825 · tauri 1,413 · report 1,069 ·
cli 1,004 · timeline 669 · faults 450.

**Suspected shape:** the two >800 files are also the two carrying the most survivors and the most
inline tests respectively — `conductor-run/src/lib.rs` holds 15 inline test functions and 24 of the
28 `conductor-run` survivors.

### B9 — coverage — line **92.26%** (7,829 / 8,486), function 87.98%, branch **null**
Branch is `null`, not `0`: `cargo-llvm-cov`'s LCOV emits no `BRF`/`BRH` for Rust. A zero there would
read as total branch failure, so the caps rule ("a null is evidence of a skip, never a zero")
applies. 661 tests run, 661 passed.

| unit | line |
|---|---|
| `conductor-tauri` | **52.64%** (219/416) |
| `conductor-run` | 79.68% (651/817) |
| `conductor-cli` | 89.02% |
| `conductor-verify` | 92.89% |
| `conductor-core` | 95.87% |
| `conductor-emit` | 97.47% |
| `conductor-faults` | 98.30% |
| `conductor-report` | 99.50% |
| `conductor-timeline` | **100.00%** |

Weakest files: `conductor-tauri/src/main.rs` **0%** (21 lines) · `conductor-tauri/src/commands.rs`
44.18% (292) · `conductor-cli/src/pause.rs` 66.20% (71) · `conductor-run/src/lib.rs` 79.67% (659) ·
`conductor-run/src/dispatch.rs` 79.75% (158).

**Suspected shape:** the two low units are the two shells. That is consistent with the architecture's
own posture — the Tauri GUI is "convenience, not a release gate" — so 52.64% is a recorded design
consequence, not a defect. The docs' `--fail-under-lines 60` gate would fail on `conductor-tauri`
alone if applied per-unit; it passes comfortably workspace-wide at 92.26%.

**Proposal:** coverage is a trend-only metric and a *falling* number is the signal — this record is
the anchor. Worth deciding once, now, whether the 60% gate is meant workspace-wide (currently
passing by 32 points) or per-unit (currently failing on one unit by 7).

---

## Informational

- **Baseline mode:** no `top-N entrants`, no churn, no trend-breaks — all require a prior record.
  Every threshold in the table is inert this run except `new-cycle`, which did not fire.
- **`conductor-run/src/lib.rs` is the concentration point** across three independent metrics: largest
  file (898), most mutation survivors (24 of 28), and a weakest-file coverage entry (79.67%). No
  threshold reads across metrics, so this is offered as an observation rather than a finding.
- **Mutation scope was founder-chosen** (baseline mode): `conductor-run` + `conductor-verify`.
  `conductor-core` (463 mutants), `conductor-emit`, `conductor-faults`, `conductor-report`,
  `conductor-timeline`, `conductor-cli`, `conductor-tauri` were not scoped — their scores are
  absent, not zero.
- **Both scoped units finished inside the 15-min cap** (7m and 6m); no `budget-exhausted` skip.
- **Licensed side effects, as run:** the instrumented coverage build and two mutation runs mutated
  the build cache (`target/`, 78 GB before this run) and cargo-mutants used its own temp trees. All
  gitignored; the next implement session pays a rebuild.

## Below threshold — no action

Nothing can be below threshold in baseline mode — there is no prior value to compare against. The
starting values that *will* be compared next epoch, with the level that fires:

| metric | baseline | fires next at |
|---|---|---|
| `graph.cycles` | 0 | any value > 0 |
| duplication pct | 3.68% | ≥ 4.18% **and** ≥ +15% relative |
| complexity `over_ceiling` | 6 | ≥ 9 **and** ≥ +25% |
| `zero_ref_candidates` | 31 | ≥ 36 |
| coverage line | 92.26% | ≤ 90.26% |
| mutation `conductor-run` | 58.82% | ≤ 48.82% |
| mutation `conductor-verify` | 72.97% | ≤ 62.97% |

## Skips

| metric | reason | recipe |
|---|---|---|
| churn (B1) | `no-baseline` | first record — computable from the next boundary |
| hotspots (B2) | `no-baseline` | depends on churn |
| dead-code, web surface | `tool-missing` | `npm i -D knip` (in `crates/conductor-tauri/ui`) |
| mutation, web surface | `tool-missing` | `npm i -D @stryker-mutator/core` |
| complexity, web surface | `declined` | lizard 1.24.0 **is** available; the TS/TSX surface is 1,132 lines with 0.00% duplication, so it was left out of this baseline. Reversible at any later run — say so and it gets collected. |

Not skipped but worth noting: `scc` is absent (`AVAILABLE` in my first probe was a false read — a
pipe masked its exit code). It is only an alternative to tokei, which is present, so no metric was
affected.

---

*Report-only: nothing here is gated, applied, or remembered. Whether any of these metrics earns gate
status — and where such a gate would live — is a founder decision after 2–3 epochs of data. Evidence
twins: `c-sizes.json` · `c-complexity.json` · `c-duplication.json` · `c-graph.json` · `c-dead.json` ·
`c-coverage.json` · `c-mutation-conductor-run.json` · `c-mutation-conductor-verify.json` ·
`record.json` (appended to `.andromeda/code-metrics.ndjson`).*
