# Report — 2026-06-16-scenario-config-model

**Chunk:** Scenario-config model — declarative per-phase emission spec (serde + garde), wires Scenario→PhaseTimeline into the seeded scheduler (conductor-core + conductor-timeline)
**Date:** 2026-06-16T20:48:47Z
**Commits:** none yet (work uncommitted; this wrap creates the chunk commit)

## Changes (structured — detectors read this)
- **Files:**
  - New: `crates/conductor-core/src/phase_spec.rs`, `crates/conductor-timeline/src/convert.rs`, `scenarios/error-baseline-spike.toml`
  - Modified: `crates/conductor-core/src/scenario.rs`, `crates/conductor-core/src/lib.rs`, `crates/conductor-core/src/error.rs`, `crates/conductor-core/Cargo.toml`, `crates/conductor-timeline/src/lib.rs`, `Cargo.toml` (workspace), `Cargo.lock`
- **Symbols / APIs:**
  - New public (conductor-core): `PhaseSpec { name: String, gap_ms: u64, emission: EmissionSpec }`; `EmissionSpec { signal: Signal }` (`#[non_exhaustive]`, `+ fn new`, `+ Default`); `enum Signal { Traces, Metrics, Logs }` (serde snake_case, `Default = Traces`); `Scenario::from_toml_str(&str) -> Result<Scenario>`; two new public fields on `Scenario`: `phases: Vec<PhaseSpec>`, `jitter_ms: u64`.
  - New public (conductor-timeline): `impl From<&Scenario> for PhaseTimeline` (no new named export — trait impl).
  - New re-exports from conductor-core: `EmissionSpec`, `PhaseSpec`, `Signal`.
  - No IPC methods · no HTTP endpoints · no ports/sockets · no env vars added.
- **Crates / modules:** added modules `conductor_core::phase_spec`, `conductor_timeline::convert`. No new crates, none removed.
- **Dependencies:** added `toml = "0.9"` to `[workspace.dependencies]` + `conductor-core` (resolves `toml 0.9.12`; transitive `toml_parser 1.1.2`, `toml_datetime`, `winnow 0.7.15`). `Cargo.lock` updated. `cargo audit` clean; `cargo deny check` ok (advisories/bans/licenses/sources).
- **Schema / config:** new on-disk scenario config schema in **TOML** (the P4 user decision). Keys: `name`, `p_ids[]`, `seed`, `slo_tier` (`<5s`/`<20s`/`<90s`), `jitter_ms`, `[[phases]]` with `name`, `gap_ms`, optional `emission.signal`. garde bounds: phase `name` 1..=40 chars, `gap_ms` ≤ 3_600_000, `jitter_ms` ≤ 60_000, `phases` non-empty (dive). No `runs.db` / SQLite migration.
- **Coverage of new surfaces:**
  - `scenarios/*.toml` scenario config (external input) → validation **garde✓** · instrumentation n/a (config layer; emits no telemetry, per obs-plan §1) · PII n/a · tests **unit✓** (core) · a11y n/a · tokens n/a
  - `Scenario::from_toml_str` (parse boundary) → validation **garde✓** (validates after deserialize) · PII/path-leak **redacted✓** (parse-error text via `conductor_core::sanitize_error`) · tests **unit✓** (config-error / validation-error / fixture-load) · a11y n/a · tokens n/a
  - `From<&Scenario> for PhaseTimeline` (internal conversion — not an external surface) → validation n/a (consumes an already-garde-validated `Scenario`) · instrumentation n/a (not a must-trace path; `run_timeline` it feeds is already instrumented) · tests **unit✓** (order/timing map + determinism both directions under `start_paused`) · a11y n/a

## Deviations from intent
1. **`emission` is `#[serde(default)]` (defaults to `Signal::Traces`).** The plan described a minimal `EmissionSpec` but left config-optionality unspecified; making it default lets the fixture match the **P4-approved TOML preview** (which omitted `emission` from `[[phases]]`). The field is present, settable, and tested — ergonomic refinement, not a scope change.
2. **Tests use plain `#[test]` + factory helper fns (`scenario_with`/`phase`/`scenario`), not rstest `#[fixture]`/`#[case]`.** The plan's tests-criterion suggested rstest factories; I matched the **existing conductor-core convention** (the pre-existing `scenario.rs` tests use `scenario_with` + `for` loops, no rstest). Per code-writing-discipline "match the detected style". rstest stays available; converting these + the pre-existing tests is a separate refactor, out of this chunk's scope. Runner is still cargo-nextest.
3. **Concrete bounds chosen (plan delegated the numbers):** `MAX_GAP_MS = 3_600_000` (1h — fits the Epoch-4 bursty-train ~10-min quiet window), `MAX_JITTER_MS = 60_000` (1 min), phase `name` 1..=40 (design AC). Rationale documented inline.

## Decisions & corrections
- **User decision (P4, AskUserQuestion):** scenario config on-disk format = **TOML** (adds the `toml` dependency), chosen over JSON. Sets the convention for all 60 future scenario files; rationale = ergonomic hand-authored config + comments.
- **Design call (not a user correction):** the emission descriptor is intentionally minimal — `EmissionSpec { signal }` over the three OTLP signal classes, `#[non_exhaustive]` — with the concrete emission taxonomy (severity, fingerprints, latency targets, ramps) deferred to the Epoch-3 emission seam per the route's epoch boundary.
- No user corrections to the implementation this session; plan approved as-is, gates green first pass.

## Outcome
- **Acceptance criteria met:** yes — TOML scenario loads→validates→converts to `PhaseTimeline`; parse failure → `CoreError::Config`, garde failure → `CoreError::Validation` (verdict/error wall, no panic); conversion deterministic both directions; phase-name legibility + bounds enforced; one fixture under `scenarios/`.
- **Gates green:** `cargo nextest run -p conductor-core -p conductor-timeline --profile ci` (66/66) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `cargo test --doc` (pass) · `cargo audit` (clean) · `cargo deny check` (ok) · `cargo llvm-cov nextest -p conductor-core -p conductor-timeline --fail-under-lines 60` (**89.52%**; `phase_spec.rs` & `convert.rs` 100%).
- **Smoke:** `bash scripts/agent-run.sh status` → exit 0 (no boot-path change this chunk; harness skeleton intact). Real boot smoke n/a (no binary/entry-point touched).
