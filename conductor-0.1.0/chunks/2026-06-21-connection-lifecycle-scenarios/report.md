# Report — 2026-06-21-connection-lifecycle-scenarios

**Chunk:** Connection-lifecycle scenarios (P-001..P-004) — Listening/Receiving/Idle→Stalled state walk + orthogonal port-occupier ReceiverFailed; first scenarios/*.toml catalog entries + Scenario expected/SLO TOML wiring (conductor-core/faults)
**Date:** 2026-06-21T22:46:02Z
**Commits:** none yet (chunk uncommitted; wrap commits in P7) · prior HEAD d86f2ab (coverage-matrix-generator)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/scenario.rs` (M) · `crates/conductor-timeline/src/convert.rs` (M, test only) · `crates/conductor-core/src/error.rs` (M, test only) · `scenarios/receiver-lifecycle-state.toml` (NEW) · `scenarios/last-span-ago-tracking.toml` (NEW) · `scenarios/receiver-failed-port-conflict.toml` (NEW) · `scenarios/orthogonal-health-domains.toml` (NEW)
- **Symbols / APIs:** added one public field `Scenario.expected: Vec<ExpectedCheck>` (`#[serde(default)]` + `#[garde(dive)]`) on the existing `conductor-core::Scenario` struct. `ExpectedCheck` / `ClaimClass` / `ComparisonKind` are **reused** (pre-existing, already exported at `lib.rs:33`) — none new. No new public fn / export / IPC method / endpoint / port / socket / env var. `impl From<&Scenario> for PhaseTimeline` unchanged (reads only `phases`+`jitter_ms`).
- **Crates / modules:** none added / removed. Changed: `conductor-core` (scenario module — new field + tests; error.rs test literal) · `conductor-timeline` (convert.rs test literal only). No new workspace crate.
- **Dependencies:** none added / bumped. `rstest` was already a `conductor-core` dev-dependency (Cargo.toml:19); no `Cargo.toml`/`Cargo.lock` change.
- **Schema / config:** the `Scenario` config schema gains the optional `[[expected]]` block (array of `ExpectedCheck` = `{kind, class, expected}`), `#[serde(default)]` so omission stays valid (existing `error-baseline-spike.toml` unaffected). 4 new declarative scenario config files keyed P-001..P-004. No DB migration. No new `runs.db` column.
- **Coverage of new surfaces:**
  - `Scenario.expected` TOML field (new external-input surface) → validation **garde✓** (`#[garde(dive)]` into each `ExpectedCheck`; `expected` string `min=1`; load-time via `Scenario::from_toml_str` → `CoreError`) · instrumentation **n/a** (pure config-model data; no runtime op/span this chunk) · PII **n/a** (declarative tokens; no host paths/secrets) · tests **unit✓** (JSON round-trip · garde-reject-empty-via-dive · default-empty-when-omitted · parse-expected-block) · a11y **n/a** (no UI) · tokens **n/a** (no UI)
  - 4 scenario config files `scenarios/*.toml` (new config artifacts) → validation **garde✓** (each round-trips through `from_toml_str` + validates; non-empty `p_ids`) · instrumentation **n/a** · PII **n/a** · tests **unit✓** (`#[rstest]` committed-fixture table, 4 cases) · a11y **n/a** · tokens **n/a**

## Deviations from intent
- **Smoke skipped (status-only), not a live `run`.** Planned: the plan's Test Commands listed only `agent-run.sh status` (read-only); a live `run` needs a Pulse instance (MCP preflight + `:4317` egress), an operator/local gate, absent here and out of scope. The chunk changed no boot-path code (core library field + config + tests), so a deep boot smoke is N/A. Justification: per the drive+observe posture + fix-loop-protocol (live leg is an environmental dependency). The +8-test delta (301→309) is the real end-to-end proof — the 4 TOMLs load + validate.
- Otherwise **none** — model wiring + 4 TOMLs + tests landed exactly as planned.

## Decisions & corrections
- **P4 user decisions (AskUserQuestion):** (1) **4 files, one P-ID each** (vs combining) — cleanest 1:1 to the spec's four distinct Conductor-verification recipes; (2) **wire `expected` now** (vs config-only-defer) — establishes the declarative read-back shape the whole catalog reuses; closes carried follow-up (d).
- **`holds` deferred** — drive+observe timing claims need no operator go/no-go; the visual lifecycle-badge claim routes to the `ManualCheck` report-state, not a `HoldPoint`. (Carried follow-up (d) is the `expected`+`holds` wiring; `expected` lands now, `holds` when a scenario genuinely gates a non-Conductor step.)
- **`ComparisonKind` NOT extended** for P-002's ±1s tolerance window — kept the model change additive-minimal ("wire expected", not "extend the comparison vocabulary"); the numeric tolerance is the Epoch-8 evaluator's concern.
- **Realization deferred to Epoch-8 driver** — emission on/off + silence (Listening=no spans; Idle/Stalled=stopped) and the P-003 `:4317` bind are realized by the CLI driver over existing emission + `conductor-faults` machinery; this chunk authors declarative config (timing + identity + tier + expected targets) + the model carrier. `EmissionSpec` (`#[non_exhaustive]`) is the noted extension point if in-config silence is later wanted.
- **TOML `expected` tokens are declared intent** (e.g. P-004 `"error"`, P-002 `"Receiving"`); live read-back field-matching is the Epoch-8 evaluator's wiring and tokens may be refined there.

## Outcome
- **Acceptance criteria met.** All 4 TOMLs deserialize + garde-validate → valid `PhaseTimeline`; `expected` round-trips; invalid `ExpectedCheck` rejected via `dive`; omitted defaults empty; each TOML carries non-empty `p_ids`; no `:4317` bind in config/tests; determinism preserved (existing `start_paused` convert/determinism tests green); no new dependency / crate.
- **Gates green** (commands run): `cargo nextest run -p conductor-core -p conductor-timeline` → 117/117 · `cargo nextest run --workspace --profile ci` → **309/309** (301→309, +8) · `cargo test --doc -p conductor-core` → 0 · `cargo clippy --workspace --all-targets -- -D warnings` → clean. Fix-loop: green in 1 iteration.
- **Smoke:** `bash scripts/agent-run.sh status` → exit 0 (read-only; needs a `<run_id>`). Skipped a live `run` — no boot-path change; live leg is the operator/Epoch-8 gate.
