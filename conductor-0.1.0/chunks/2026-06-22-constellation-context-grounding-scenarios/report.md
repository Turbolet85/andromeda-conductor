# Report — 2026-06-22-constellation-context-grounding-scenarios

**Chunk:** Constellation + context-grounding scenarios (P-025..P-027, P-032, P-036) — first DriveObserve operator-checklist (ManualCheck) family + first KnownResidual (P-032) (conductor-core)
**Date:** 2026-06-22T23:30:48Z
**Commits:** uncommitted — this wrap authors the chunk commit (last_wrap commit was 38cfce0 severity-lifecycle)

## Changes (structured — detectors read this)
- **Files:** 5 new `scenarios/*.toml` (`halo-hue-encoding`, `halo-breathing-encoding`,
  `service-constellation-discovery`, `project-context-grounding`, `cross-incident-recurrence`) ·
  `crates/conductor-core/src/scenario.rs` (`#[cfg(test)] mod tests` ONLY — +4 test fns / +11 instances).
- **Symbols / APIs:** NONE new — no production symbol, no public-fn/IPC/endpoint/export/port/socket/env-var
  change. The only `conductor-core` source delta is in the test module (rstest loader + guards).
- **Crates / modules:** none added / removed / changed (conductor-core test module only).
- **Dependencies:** none added / bumped.
- **Schema / config:** 5 new declarative scenario-catalog entries under the already-registered `scenarios/`
  directory (serde + garde, one-per-P-ID; P-025/026/027/032/036). NO schema/migration/config-key change. The
  operator-checklist shape (empty `[[expected]]`) + P-032's KnownResidual declaration are expressed through
  the EXISTING `Scenario.expected` field (`#[serde(default)]`, `scenario.rs:94`) — no model change, no new
  `ComparisonKind`/`ClaimClass`/`SloTier`. One inferred read-back token: `"Previously seen"` (P-036, `Contains`,
  `Hard`) — substring-tolerant, an Epoch-8 calibration point (the RetryStorm/LatencyRegression/Resolved precedent).
- **Coverage of new surfaces:**
  - `scenarios/{halo-hue-encoding,halo-breathing-encoding,service-constellation-discovery,project-context-grounding,cross-incident-recurrence}.toml` (external-input config) → validation **garde✓** (`Scenario::from_toml_str` garde-validates; rstest loader proves all 5 load + validate) · instrumentation **n/a** (declarative config; the must-trace spans are the Epoch-8 timeline/driver's, not this chunk's) · PII **n/a** · tests **unit✓** (rstest loader + operator-checklist/declare-only guard + P-036 Hard-check guard + suite both-shapes guard, conductor-core 151/151) · a11y **n/a** (no UI) · tokens **n/a** (no UI).
  - No new UI element / surface / hot-path operation / external service. The constellation hue/breathing/dot
    RENDER + the ManualCheck/KnownResidual lamp render are Pulse's / Conductor's Epoch-9 surface — NOT added here.

## Deviations from intent
- **SLO tiers for P-026/P-032/P-036 chosen at impl** — plan pinned P-025/P-027 = `<5s` (hue ≤2s, dot ≤5s) and
  left the other three open. Chose `<20s` for all three (breathing tracks a rate *trend*; report-context
  population and recurrence-reference are diagnostic-generation latencies — none an instant signal).
  Justification: within-plan elaboration; each tier is a *declared* value the Epoch-8 live run measures.
- **Suite "note" implemented as a lean test** (`constellation_context_grounding_suite_has_operator_checklist_members`)
  rather than a doc-comment. Justification: a both-shapes guard is stronger than a comment and mirrors the
  `severity_lifecycle_suite_is_mixed_class` precedent.
- **P-032 DoD refinement** — scope.md "P-032 expresses KnownResidual" → "declares + documents; routing
  Epoch-8" (amended in scope.md at phase P5). Justification: research found the scenario model has no `state`
  field (ReportState is producer-assigned per arch §Read-Back Dependency Posture); declare-only is the faithful,
  precedent-consistent realization. Already resolved in the phase — no open gap.

## Decisions & corrections
- **Q1 (phase P4 AskUser):** constellation trio split into **3 per-P-ID files** (5 total) — each a distinct
  visual claim + emission driver (hue=error-state, breathing=rate.rs ramp, constellation=topology.rs). Matches
  the per-value-file precedent + one-TOML-per-P-ID coverage.
- **Q2 (phase P4 AskUser):** P-032 KnownResidual = **declare-only** (empty `expected` + documented residual:
  Pulse `recent_commits` is a stub until v0.3.0, Conductor is its designated detector; the Fail→KnownResidual
  routing is Epoch-8 evaluator-owned). **Zero model change** — consistent with arch "verdict/state independent,
  state set by the producer not the config" + the severity-lifecycle within-cooldown / activity-floor
  declare-only precedent.
- **Zero model change (the inflection):** empty-`expected` IS the built-in drive+observe/operator-checklist
  path (`scenario.rs:91-94` doc comment + `expected_defaults_to_empty_when_omitted`); this is its first catalog
  use. The constellation loader **inverts** every prior family's `!s.expected.is_empty()` assertion (asserts
  `is_empty()` for P-025/026/027/032; the lone Hard check is P-036's).
- **Seeds** 4317025/026/027/032/036 (the `4317{NNN}` convention); verified to feed no committed insta golden.

## Outcome
- **Acceptance criteria — all met:** 5 TOMLs exist + load + garde-validate; P-025/026/027/032 carry empty
  `expected` (operator-checklist/declare-only); P-036 declares one `Hard Contains "Previously seen"`;
  `coverage.rs` untouched (60 P-IDs zero-gap); goldens UNCHANGED (no snapshot references the 5 seeds); no
  new `ComparisonKind`/`ClaimClass`/scenario field (conductor-core delta = test module only).
- **Gates green:** `cargo nextest run -p conductor-core` 151/151 (140→151) · `cargo nextest run --workspace
  --profile ci` 356/356 (345→356) · `cargo clippy --workspace --all-targets -- -D warnings` clean ·
  `cargo test --doc -p conductor-core` 0/ok.
- **Smoke:** `bash scripts/agent-run.sh run` exit 0 (the release-gate path).
