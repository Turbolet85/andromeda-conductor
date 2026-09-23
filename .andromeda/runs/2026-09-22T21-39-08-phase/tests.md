# tests extract

## Relevance
relevant: the chunk adds a scenario, an operator-gated real-model live leg reached through the harness, and a harvest-tier grade. It may also move the static gates over committed data. Tier is Minimal plus the coverage triggers (per test-plan §1 Test Scope Summary).

## Constraints
- **Operator-local live gate only.** test-plan §2 (Agent-runnable invariants, Deterministic) allows real-wall-clock, non-seed-deterministic legs only as enumerated operator-local exceptions. §9 (Live-Pulse scenarios) and §11 (CI) require the leg to be reached from a sanctioned live-leg path and never from CI. The sanctioned paths are `workflow_dispatch`, `scripts/agent-run.sh` including `run --live`, a `live-pulse`-feature-gated target invoked directly, or an npm wdio suite. A real-clock leg reached from outside that set reads as a §10 zero-flakiness break. §3 (`cleanup`) requires that Conductor never touch Pulse. §1 (Untestable zones) marks Pulse's process and its encrypted `corpus.db` as untestable from Conductor. So Pulse-side facts, such as reaching the live-writer branch, can be read only through MCP read-back or the Pulse lines a harvest pins.
- **Declare-only row; interpretation graded at the harvest tier.** test-plan §1 (5-command `run`) and §3 (`run` exit-code semantics) require model-interpretive claims to be reported (`CalibrationRegion`, report-for-human) and never turned into a hard-`Fail` exit. §3 (Log format) requires a declare-only scenario to emit no `CheckRecord` line, and the envelope stays at eleven fields. The closing note under §1 Critical paths records the shape for such a family: the row lands `verdict` null / `state=KnownResidual`, and the live claim is graded hard in a `conductor-run` harvest target.
- **Harness shape.** test-plan §3 (5-command implementation → `run` CI stage selectors) requires:
  - the command count stays at five;
  - `--live` stays a sibling `case` branch whose order is read from `live_leg_order`, never restated as a count;
  - each leg's truncating `logs/agent-latest.jsonl` is frozen to `runs/live-suite/{leg}.jsonl`;
  - the envelope, the newest-envelope-line `status` read and the JSONL shapes stay untouched.

  §3 (Bootstrap phases → `5-command-discipline-wire`) requires `.sh` and `.ps1` to have identical semantics. The leading `conductor preconditions` probe refuses at exit 1 on an unmet subject, and §1 (5-command `boot`) grades the `ANDROMEDA_PULSE_*` flag handles by `flag_declared` truthiness. Whether the L4-deterministic flag is one of the graded handles is research's question. If it is, a leg launched under the posture's L4-absent/falsy launch is refused before any drive.
- **Budgets derived, never literal.** test-plan §3 (`boot` → Timeout) requires both budgets to derive from `contracts/pulse-run-contract.toml` (`warmup_ms`, `[incident_formation].min_canary_poll_seconds`). A missing term is a hard exit 2, and a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps up to the floor. §2 (Agent-runnable invariants) applies contract-derived per-leg budgets to the `--live` suite. So the real-model leg's budgets come from those terms, computed against the formation figure measured at the first drive.
- **Scenario authoring.** test-plan §11 (Universal) bans a scenario without a Pulse P-ID. §7 (Seed strategies → Fixture files) requires committed declarative serde+garde config under `scenarios/`. Any new config field or rule needs its garde negative test at load (§1 Coverage triggers, Vector 2); a nested `skip`-instead-of-`dive` must be caught. §3 (`run`) makes the TOML-declared seed govern unless `SEED` is set explicitly. §1 (Coverage triggers, determinism property) requires the emission stream to keep "same seed ⇒ same shape" even though Pulse's reaction does not. §11 (Test Strategy) allows only bounded profiles for the injected cause, never saturation.
- **Static gates over committed data.** test-plan §6 (Coverage-matrix completeness gate) requires three things:
  - the committed `coverage-matrix.md` is byte-equal to `CoverageMatrix::render()`;
  - its Scenario-backing leg holds `check_scenario_backing` to exact-set equality in both directions against `UNBACKED_AUTO`, so pin rot fails;
  - its Scenario-assertion audit leg holds `contracts/scenario-audit-ledger.toml` set-equal to the catalog on both axes: a live `[[expected]]` block, and summed phase `gap_ms` over the declared tier's `SloTier::deadline_ms()`.

  Research must answer two questions. First, does a scenario naming P-033 trip pin rot or move the rendered matrix? Second, does a declare-only scenario whose emission window must outlast ~110 s formation land on the ledger's `gap_ms` axis?
- **Quality gates.** test-plan §10 (Coverage thresholds) keeps `--fail-under-lines 60`. §10 (Zero-flakiness budget) and §4 (Framework) require the harvest target to be deterministic, green under both nextest and `cargo test -p <crate>`, with no retries. §9 (Scoped mutation audit) runs the operator-local `scripts/mutation-gate.py {unit}` tally gate for each crate the chunk touches. §10 (Mutation-survivor disposition) requires every named survivor to end killed or accepted-deliberate.

## Patterns to follow
- **Three-tier split for a claim the envelope cannot carry.** test-plan §5 (Cross-module patterns → `mark_incident_resolved`) and §6 (Restart-suppression; Severity-lifecycle) set the shape:
  1. stub-proven integration arms, where a stub can reach them;
  2. a deterministic `conductor-run` harvest target that asserts hard predicates over verbatim leg captures;
  3. captures produced by a feature-gated operator leg. §9 names `live_suite.rs` under `[features] live-pulse = []` as the `--live` capture tool.

  The scope names `severity_harvest.rs` as the precedent.
- **In-suite negative arms against mis-pairing and vacuous grades.** test-plan §6 has three precedents:
  - Severity-lifecycle pins the preflight-canary mis-pairing as a negative test.
  - The Scenario-assertion audit leg makes an in-suite can-fail demonstration (negative arms plus a sweep control) the standard.
  - Fingerprint-storm records `Contains` failing structurally and `Absent` passing vacuously against text that cannot carry the token.

  Applied here, each of these must be shown not to grade as the target outcome: a match found only below rank 1, a degraded report with no `## Hypotheses` section, and the leg's own preflight-canary incident(s).
- **Firing form and stop form are part of the leg.** test-plan §6 (Drivers per surface → desktop-webview, driven arm) records the full live firing form: `andromeda-pulse-mcp` resolved on the PATH, `ANDROMEDA_PULSE_MCP_ENABLED`, a data dir equal to the live Pulse's, and the quiet window. It also records that a bare invocation reports BLOCKED in ~2 ms, indistinguishable from a real gate failure. The `sr` leg's stop form includes a process census. §9 (Live-Pulse scenarios) confirms a booted Pulse posture by grepping Pulse's log for the TARGET string, never the emitting function name. That is the analogue for proving L4 was off at boot.
- **Mock and fixture fidelity.**
  - test-plan §5 (`query_incident_list`, STUB ITEM-KEY FIDELITY) requires any stub response to mirror the live sidecar's shape, for example a stub `## Hypotheses` rendering used to unit-test rank-1 scoping. A shape only the stub produces goes green against the stub and fails live.
  - §7 (Self-bootstrapping requirement) requires a capture committed under a crate-local `tests/fixtures/` tree to have its meaning pinned by a production reader/writer round-trip. Whether harvest captures are inline or committed fixtures is research's question.
- **Record measurements, not literals.**
  - test-plan §6 (Fingerprint-storm) dates each measurement to its SUT HEAD.
  - §9 (Live-Pulse scenarios) records a verdict that depends on SUT uptime as conditional.
  - The §1 critical-paths "7 paths — the maximum" note and §6's "7 scenarios" qualifier record a new family as an instance or sibling leg, never an eighth path.

  The ~110 s formation figure confirmed at the first drive should be recorded the same way.

## Anti-patterns to avoid
- Running the live leg as a CI gate, or letting the stub stand in for the model's hypothesis as a CI verdict; the stub proves wiring only (per test-plan §11 CI; §11 Test Strategy; §8 What NOT to mock).
- Turning a miss into a failure exit, or re-driving until green. `CalibrationRegion`, `ManualCheck`, `KnownResidual` and `Blocked` are reported states, never a non-zero exit, and no retry-once policy is allowed (per test-plan §11 E2E; §11 Quality; §10 Zero-flakiness budget).
- Sleeping inside a test to synchronise, or grading by a human reading the report. The quiet window is only a harness-level firing-form wait between legs. The grade comes from captures; `ManualCheck` is a reported state, not a manual test step (per test-plan §11 E2E `sleep(N)`; §11 Universal).

## Contract bindings
- **tests §3 ↔ obs-plan §3.** Covers the 5-command discipline, the eleven-field envelope, the `CheckRecord` shape, the newest-envelope-line `status` read, and keeping the self-obs stream separate from the per-run journal. A log-format change is a harness break (test-plan §3 Bootstrap phases → `log-format-bind-with-obs`).
  - The harvest witness must travel on a line that carries its value intact. §6 (Severity-lifecycle) records a Pulse witness whose fields rendered `"<redacted>"` and proved nothing. §3 (Log format) gates every Conductor self-obs field and span attribute through the `conductor-core::redact` allowlist.
  - Whether the hypothesis text reaches a capturable line intact, on either side, is research's question.
- **tests ↔ `contracts/pulse-real-model-leg-posture.md` §The grading rule ↔ arch Standard Contracts.** The posture's miss row (`CalibrationRegion` / `ManualCheck`) uses the same assertion-policy split test-plan §1/§3 state, so those are consistent.
  - One divergence needs reconciling. test-plan §1 (Critical Path 5), §4 (conductor-verify) and §5 (`retrieve_report` with `degraded_mode`) require `degraded_mode ⇒ KnownResidual`, which P-032's path depends on. The posture maps a report with no hypotheses to `Blocked` with the named precondition.
  - The harvest grade must produce the posture's row without moving that production mapping. Whether it can reuse `conductor-verify`'s mapping is research's question.
- **tests ↔ conductor-core/report static gates.** `UNBACKED_AUTO` / `check_scenario_backing`, `coverage-matrix.md` and `contracts/scenario-audit-ledger.toml` (test-plan §6). The next route entry takes the cluster off the pin, with pin and matrix moving together; where this chunk's boundary with it falls is for P3/P4.
- **tests ↔ run contract.** Budget terms come from `contracts/pulse-run-contract.toml` (test-plan §3 `boot` → Timeout).
- **tests ↔ security.**
  - A new `CONDUCTOR_*` handle in the firing form needs the per-reader coverage in test-plan §1 (Coverage triggers, Vector 1).
  - A silently-empty live read-back must surface as `blocked` (§1 Surfaces → ipc-internal; Coverage triggers, Vector 4).
  - `ANDROMEDA_PULSE_DATA_DIR` is passed only via `.env(...)` (§11 Mocking).

## Acceptance criteria contributions
- (tests) The workspace gate is green with the new harvest target included and no live Pulse: `cargo nextest run --workspace --profile ci`, `cargo test --workspace --doc`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`, `cargo test -p <crate>` for each touched crate (runner portability), and `cargo llvm-cov nextest … --fail-under-lines 60` (per test-plan §3 `run`; §4 Framework; §9 Pipeline structure; §10 Coverage thresholds).
- (tests) The static gates are green in the same change, whichever boundary P4 sets: `cargo nextest run -p conductor-report --test coverage_gate --profile ci` and `cargo nextest run -p conductor-core --test scenario_audit_gate --profile ci` (per test-plan §6 Coverage-matrix completeness gate, Scenario-backing leg, Scenario-assertion audit leg).
- (tests) The harvest target maps verbatim captures onto the posture's outcome rows and has in-suite negative arms (per test-plan §6 Severity-lifecycle; §6 Fingerprint-storm; §6 Scenario-assertion audit leg):
  - identifying text present only below rank 1 does not grade `Pass`;
  - no rendered `## Hypotheses` section grades `Blocked` with the named precondition, never a vacuous grade;
  - neither the preflight-canary incident nor a second-incident capture pairs as the live-writer re-generation.
- (tests) The drive stays operator-gated and envelope-neutral (per test-plan §3 `run` and Log format; §9 Live-Pulse scenarios; §11 CI):
  - the row lands `verdict` null / `state=KnownResidual` at exit 0;
  - no `CheckRecord` line is emitted and the eleven-field envelope is unchanged;
  - no CI step invokes the drive;
  - any `live-pulse`-gated target it adds is green under `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`.

## Relevant amendment history
- **`2026-08-09-interpretation-correctness-posture`** (§5, §6). Added the scenario-backing leg: exact-set equality in both directions over `UNBACKED_AUTO`, `Auto` only, runnable in CI. Why: the chunk shipped a gate that §6 did not describe. This is the residual this chunk absorbs (`absorbed:v3-09`), and it is the gate a P-033 scenario would move.
- **`2026-08-16-fingerprint-storm-live-proof` through `2026-08-21-severity-lifecycle-live-proof`** (the five live-proof re-bases; §1 critical paths and their §6 twins in one pass under the two-site rule). Each family moved to declare-only rows, with the live claim graded hard in a `conductor-run` harvest target. Why: under deterministic L4, `retrieve_report` was permanently degraded (`Contains` always failed, `Absent` passed vacuously). P-059's resolution summary was unreachable because no `ResolutionSummary` constructor existed. The auto-resolve tick counters rendered `"<redacted>"`. The restart re-base also fixed seed precedence (`--seed` only on an explicit `SEED`). This chunk is the first to read back with L4 off.
- **`2026-08-21-per-check-latency-measurement`** (§2, §3, §6). The journal now carries two shapes (envelope and `CheckRecord`); typed parses must discriminate between them, and a declare-only scenario emits no check line. Why: a file-wide envelope parse had become a false negative.
- **`2026-08-31-p-075-assert-round`** (§5, §9, §11). A cargo-feature gate was registered as a sanctioned live-leg path that owes its own per-feature clippy. The stub's item key must be the live `incident_id`. Why: the `preflight_spawn.rs` precedent was undocumented, and a stub keyed on `id` stayed green while the live reader saw an empty active set; four legs were lost.
- **`2026-09-01-desktop-a11y-sweep`** (§6, §9, §11). Recorded the driven arm's full firing form: PATH prefix, `ANDROMEDA_PULSE_MCP_ENABLED` + `_L4_DETERMINISTIC`, the live data dir, and a quiet window of at least 120 s plus a 30 s tick. Why: each missing precondition misreads as a real gate failure, and two 12-minute runs were lost to the quiet window. This chunk inverts the L4 handle.
- **`2026-08-10-pulse-run-contract` + `2026-08-13-first-live-green-preflight`** (§3 `boot`). Two derived budgets, never a literal; a missing term exits 2; env values clamp up. Why: a fixed 30 s wrapper killed every live run before warm-up finished, and the two shells disagreed.
- **`2026-09-03-live-pulse-preconditions-probed` + `2026-09-04-preconditions-probe-reads-path-handles-by-presence`** (§1, §3, §6). `boot` became probe-then-preflight, with handles graded by kind (flag handles by truthiness). Why: the `handles-declared` subject could not be met, so the gate was never reached. It matters here because the posture launches with L4 absent or falsy.
- **`2026-09-06-operator-gated-live-suite`, its 0-pending adaptation, and `2026-09-06-halo-hue-budget-re-driven`** (§2, §3, §9, §11). Changes:
  - `--live` added as a sibling branch with no new port, env var or handle;
  - §2's exception set extended;
  - leg order anchored to `live_leg_order`;
  - the `sleep(N)` ban's scope clarified rather than carved out;
  - the auto-resolve leg recorded as not run-stable by SUT uptime;
  - the booted posture confirmed by log TARGET.

  Why: an unenumerated real-clock leg would read as a zero-flakiness break, and the same tree graded differently across runs.
- **`2026-09-06-run-report-envelope-conformance-gate`** (§1, §3). `status` reads the newest envelope line, selected by the `seed` key. Why: a last-line read reported a `CheckRecord` as the envelope (`seed: null`).
- **`2026-09-06-coverage-completeness-gate` + `2026-09-16-scenario-assertion-audit-gate`** (§4, §6, §7). Both static gates over committed data shipped. A ledger row's `discriminates` flag is recorded but never graded, because stimulus-dependence is a live-gate claim. Both are recorded as sibling legs with no §9 stage row. Why: the gates shipped while §6 did not describe them.
