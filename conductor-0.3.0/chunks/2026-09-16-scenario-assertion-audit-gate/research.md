# Codebase Research — 2026-09-16-scenario-assertion-audit-gate

## Scope
- **Depth:** deep · **Reads:** 20 (14 Conductor · 6 `andromeda-pulse`) · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/testing.md` (79.4 KB, past the Read cap — read as a
  structural extraction: `grep -n` index of all 45 `## Session Additions` entries by line + byte size,
  then offset-bounded reads of `:66`, `:75`, `:76`) · `.claude/rules/verification-harness.md`
  (69.8 KB, same method; index of 14 entries, offset reads of `:64`, `:66`). Both path-scope onto files
  this chunk will touch (`scenarios/**`, `crates/**/src/**/*.rs`, `crates/**/tests/**`), so both
  auto-load at /implement. **No live leg in this chunk**, so no firing-form invocation was gathered.

## Files inspected
- `crates/conductor-report/tests/coverage_gate.rs` (full, 87 lines) — the in-repo precedent: three
  arms (positive over the real committed pair · negative dropping each REAL row in turn · artifact
  byte-compare). Its doc comment states it lives in `conductor-report` **only** because that side of
  the dependency edge reaches both `conductor_core::check_sut_drift` and `CoverageMatrix::render`
  (`:11-13`) — a constraint that does not apply to this chunk.
- `crates/conductor-core/src/scenario.rs` (`:52-73`, `:170-250`) — `SloTier` is a code-native closed
  enum (`<5s`/`<20s`/`<90s`) with `const fn deadline_ms()` → 5000/20000/90000. `check_budgets`
  (`:200-222`) cross-checks `budget_ms` against the tier deadline — **and nothing else**.
- `crates/conductor-core/src/scenario_catalog.rs` (`:42-58`) — `list_scenarios` returns
  `ScenarioSummary { name, p_ids, slo_tier }`: **no phases, so no `gap_ms`**.
- `crates/conductor-core/src/phase_spec.rs` (`:34-53`) — `PhaseSpec.gap_ms: u64`, garde-bounded by
  `MAX_GAP_MS`; the per-phase duration term the tier probe sums.
- `crates/conductor-core/src/load_envelope.rs` (`:367-398`, `:49-63`) — `check_load_envelope`: the
  exact-set ledger gate, graded in BOTH directions against a committed `Vec<Exemption>`.
- `crates/conductor-core/src/error.rs` (`:17-38`) — `CoreError::{Config, Validation, SutDrift,
  UnbackedCoverage, LoadEnvelope}`; the named-variant harness-fault family a new gate would join.
- `crates/conductor-core/src/drift.rs` (`:79`, `:155`) — `check_sut_drift` / `check_scenario_backing`.
- `crates/conductor-verify/src/extract.rs` (`:40-120`) — `observe()` composes `Observation.text` from
  `list_text(query_incident_list)` plus every incident's `retrieve_report` `markdown` segment.
- `crates/conductor-verify/src/slo.rs` (`:51`) — `ComparisonKind::Contains => observed.contains(...)`,
  a plain substring test over that composed text.
- `scenarios/root-span-error-scope.toml`, `scenarios/span-status-error-detection.toml` (full) — the
  two surviving `[[expected]]` blocks, both `Contains`/`"error"`, classes `CalibrationRegion` and
  `Hard` respectively.
- `.gitattributes` (full) — `* text=auto eol=lf` repo-wide, plus the named `coverage-matrix.md` control.
- `.github/workflows/ci.yml` (structure) — three jobs (`rust` · `frontend` · `a11y`), all Windows
  runners; the `rust` job carries 20 named steps including `Coverage-completeness gate` (`:87`).
- `scripts/mutation-gate.py` (`:1-20`, `:137-142`) — reads `missed.txt` at `:137`, diffs against the
  roster at `:140-141`; its only `timeout` token is the module docstring at `:3`.
- `contracts/pulse-load-envelope.toml` (structure) — the `[[exempt]]` ledger, currently **empty** by
  design, with its both-directions contract stated in comments at `:35` and `:47`.
- **SUT (`andromeda-pulse` @ HEAD `83d4060`):** `pulse-app/src/deterministic_inference.rs`
  (`:18`, `:38-46`, `:126`) · `pulse-app/src/inference_runtime.rs` (`:686-712`, `:784`, `:820-885`) ·
  `crates/interpretation/src/schema.rs` (`:115-130`) · `crates/interpretation/src/markdown.rs`
  (`:134`, `:249-300`, `:359-366`) · `crates/triage/src/contract.rs` (`:239-244`, `:405-425`) ·
  `crates/ingest/src/connection.rs` (`:71-75`, the *other* `Severity` enum — not the one in play).

## Graph impact (from the code-graph query)
Query: `callee_name IN ('list_scenarios','check_load_envelope','check_sut_drift','deadline_ms',
'check_budgets') AND callee_kind='fn'`, plane `rust`. Trace:
`.andromeda/runs/2026-09-16T07-13-13-phase/tree-query-2026-09-16-scenario-assertion-audit-gate.json`
— **`rows: 46`**, `db_state: fresh`, `probe_hits: null` (filled only on a 0-row result). Line values
below are the graph's 0-indexed start **+1**, per the cookbook's citation rule.
- **`deadline_ms`** — the widest fan-in; callers span `conductor-verify` (`slo.rs:279`,
  `tests/expected_slo.rs:50`) and the scenario load path. Consuming it rather than re-declaring
  5000/20000/90000 keeps the check on the obs-owned single definition.
- **`list_scenarios`** — **6 call sites** (re-derived by its own bare query, `callee_name =
  'list_scenarios' AND callee_kind = 'fn'`, `rows: 6`; an earlier prose roll-up in this file said "4"
  and listed 5 — both were transcription errors against the same trace, corrected here):
  `conductor-core/src/drift.rs:362` (`the_committed_catalog_matches_the_unbacked_ledger`),
  `conductor-core/src/lib.rs:69` (crate root / re-export), `scenario_catalog.rs:81` (`catalog`),
  `conductor-tauri/src/commands.rs:148` (`list_scenarios_impl`) and `:390` (`test_app`),
  `conductor-tauri/src/main.rs:23` (`main`). **An additive new gate fn does not disturb any of them**;
  a *signature change* to `list_scenarios` would thread through all six.
- **`check_sut_drift`** — called from `drift.rs:362` (its own test) and the `coverage_gate.rs` arms.
- No inbound cross-crate edge would be added by putting a new gate fn in `conductor-core`: every
  member already depends on it.

## Patterns detected
- **Exact-set ledger gate, graded both ways** (`load_envelope.rs:367-398`): the measured over-bound
  set is diffed against a committed pinned set, failing on `unpinned` (new item outside the pin),
  `rotted` (pinned item no longer over) and `lost_subject` (pin naming a scenario that no longer
  exists). `Exemption` carries a `reason` string (`:63`) because an exemption is a judgment, not a
  debt. This is the shipped answer to limb (d).
- **Three-armed static gate over committed data** (`coverage_gate.rs:35-87`): a positive arm over the
  real artifacts, a negative arm that removes each REAL row in turn and asserts the named error, and a
  byte-compare arm. The negative arm is explicitly the one crate-internal tests did not have (`:5-9`).
- **Vacuity guard inside the gate** (`coverage_gate.rs:37-40`): asserts the manifest is non-empty
  *before* grading, "the gate would pass vacuously" otherwise.
- **`CARGO_MANIFEST_DIR`-rooted path resolution** (`coverage_gate.rs:24-32`): a cargo test binary's CWD
  is its own package root, so the workspace root is reached as `CARGO_MANIFEST_DIR/../..`.
- **Sibling-spanning rules live outside garde** (`scenario.rs:200-222` doc comment): garde 0.22.1's
  `custom` is field-level and cannot see a sibling, so `slo_tier`-vs-sibling rules ship as
  `Scenario::check_*()` invoked from the load path.

## Conventions to follow
- **Harness faults are named `CoreError` variants, never a `Verdict`** (`error.rs:17-38`) — a static
  catalog gate's red is `Result::Err`, per the verdict/error wall.
- **Identity-only error text** (`load_envelope.rs:401-404` doc comment): scenario names plus the
  envelope's release/date, never a filesystem path and never an internal type name.
- **Name the SET, never a fresh literal** — `testing.md:66`; the committed corpus counts (36 scenarios,
  2 live `[[expected]]`) are describable but must not be the gate's predicate.
- **Seeded subject discipline for any synthetic control** — `testing.md:75`: seed the subject, keep its
  CONTENT in a committed file under `crates/`, and pin its MEANING with a production-reader round-trip;
  build it so the assertion CAN fail. "A parse failure is loud, a semantic drift is not."
- **A test whose subject is agreement must compare the two relata to EACH OTHER** — `testing.md:76`;
  comparing each against a hand-written literal is two tests, not a parity test.
- **A red baseline validates only the failing direction** — `verification-harness.md:66`: when a gate
  COUNTS occurrences, baseline it red AND hand-check the first green run's hit list before trusting the
  number. This is the curated form of this chunk's own "able to fail as well as to pass".
- **LF is pinned repo-wide** (`.gitattributes:2`) and measured: **0 of 36** `scenarios/*.toml` contain
  CRLF, so a line-joining wrap-tolerant sweep is host-independent.

## Load-bearing mechanism, re-derived at HEAD
**The equality the design needs:** for the two surviving assertions, does `Observation.text` contain
`"error"` *because of the scenario's own stimulus*, or irrespective of it?

**Re-derived (CARRY-2 carried a `measured at` marker; its evidence pointer was spot-checked and its
chain completed) — measured against `andromeda-pulse` HEAD `83d4060`:**

1. `deterministic_inference.rs:126` returns `CANNED_L4_OUTPUT_JSON` **regardless of prompt or schema**
   when `ANDROMEDA_PULSE_L4_DETERMINISTIC` (`:18`) is truthy.
2. That fixture pins `"severity": "autonomous"` (`:42`) → `interpretation::schema::Severity::Autonomous`
   (`schema.rs:121-128`, `#[serde(rename_all = "snake_case")]`).
3. `map_l4_incident_severity` (`inference_runtime.rs:697-703`) maps
   `L4Severity::Autonomous → IncidentSeverity::Error`.
4. `inference_runtime.rs:784` binds it; `:847-868` — the **sole** production `Incident` construction
   site — writes it to `incident.severity` (`triage::contract::Severity`, `:239-244`).
5. `assemble_report` (`markdown.rs:257`) computes `severity_label(incident.severity)`;
   `severity_label` (`:359-366`) maps `Severity::Error → "error"`.
6. `markdown.rs:134` pushes that label into the report markdown; Conductor folds every report's
   markdown into `Observation.text` (`extract.rs:99-104`), which is exactly what
   `ComparisonKind::Contains` substring-searches (`slo.rs:51`).

**Conclusion: CARRY-2 re-derives TRUE.** In deterministic mode every incident renders
`severity_label = "error"` whatever the scenario emitted, so both surviving assertions pass on a token
their own stimulus did not produce. The promotion-time doubt recorded in `scope.md` — that the CARRY's
arithmetic failed to close because the fixture pins `"autonomous"` and not `"error"` — was **my error**:
step 3 is the one-hop mapping the CARRY never named, and it closes the chain.

`v3-04`'s verified acceptance is **also** true and not in conflict: it claims *satisfiability*, which
holds. The two sit at different bars. Per the tests extract's domain ruling (test-plan §11), only the
*satisfiable* bar is gradeable by a static check — a discrimination-against-own-stimulus bar is a
SUT-behaviour claim, answerable only by driving a live Pulse with deterministic mode OFF.

## New files to create
Deferred to P4 — the host fork (a `conductor-core` gate fn + crate-local test vs. a `scripts/` Python
instrument) is unresolved, and it decides every path. What research fixes either way:
- a committed ledger/roster file carrying a `reason` per exempted scenario, on the
  `contracts/pulse-load-envelope.toml` `Exemption { scenario, reason }` model;
- a synthetic control fixture in a crate-local `tests/fixtures/` tree (never inline in harness config).

## Files to modify
Provisional — bounded by the same P4 fork. Not a threading list: the plan as scoped is **additive**
(a new gate fn and its callers), and the graph shows no signature-changing symbol, so no caller set
needs enumerating. If P4 chooses to widen `list_scenarios` to carry phases, its **6** call sites
(`drift.rs:362`, `lib.rs:69`, `scenario_catalog.rs:81`, `commands.rs:148`, `commands.rs:390`,
`main.rs:23`) become boundary members and this list must be reopened.
- `crates/conductor-core/src/lib.rs` — re-export, if a new `pub` gate fn lands (the export surface is
  not a call, so the graph's caller query does not cover it).
- `.github/workflows/ci.yml` — one named step, if P4 picks the CI-gate registration.

## Open questions
1. **Which host — `conductor-core` gate fn + crate-local test, or a `scripts/` Python instrument?**
   → blocks: plan-decision. Research narrows but does not settle it: `conductor-core` already owns the
   closed enum, the `check_*` family and a `tests/` dir, and the dependency-edge reason that pushed
   `coverage_gate.rs` into `conductor-report` does not apply; but arch registers `scripts/`-Python as a
   legitimate operator-instrument home, and the route entry names "a CI gate **or** an operator
   instrument" without choosing.
2. **Does this chunk absorb the `mutation-gate.py` timeout repair (CARRY 1), or route it?**
   → blocks: implementation-scope. The CARRY routes the *failure shape* here, not the repair; no route
   entry names the mutation gate.
3. **Does `list_scenarios` gain a phases-carrying sibling, or does the check load scenarios itself?**
   → blocks: implementation-scope. `ScenarioSummary` deliberately omits phases, so the tier probe
   cannot use it as-is; adding a sibling loader is additive, widening `list_scenarios` is not.
