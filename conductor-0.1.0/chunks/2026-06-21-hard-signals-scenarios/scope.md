# Scope — Hard-signals scenarios (P-005..P-008)

**Marker:** `2026-06-21-hard-signals-scenarios`
**Version:** conductor-0.1.0 · **Epoch 7 (Scenario catalog) — chunk 2 of 8**
**Working-route intent:** _Hard-signals scenarios — ERROR/exception/severity-boundary/root-vs-deep checks (P-005..P-008)_

## What it builds

The **hard-signals family of the `scenarios/` catalog** — declarative TOML scenario config keyed to Pulse's
deterministic hard-signal floor **P-005..P-008** (`pulse-capability-spec.md` §2 "Hard Signal Detection").
These are the definitional, baseline-free signals present in OpenTelemetry semantic conventions — detectable
from the first emitted span, regardless of model availability (Pulse's honest minimum when the model is off):

- **P-005 — Span Status Error Detection** — emit spans with explicit `Span.Status.Code = ERROR`; Pulse surfaces
  them as candidates within a <500ms-p99 detection budget. **(Hard.)** Auto leg: candidate emission reaches the
  severity classifier within budget (MCP read-back); operator leg: halo hue + constellation dot color reflect
  the resulting severity.
- **P-006 — Exception Event Capture** — emit spans carrying OTel `exception` span events
  (`exception.type` / `exception.message` / `exception.stacktrace`, synthetic stack traces); Pulse captures +
  stores them, accessible to the pattern detector (P-017) + Report generator. **(Hard.)**
- **P-007 — High-Severity Log Capture** — emit OTLP log records spanning `SeverityNumber` across the **17
  boundary**; ERROR/FATAL (≥17) contribute to the hard-signal layer, WARN-and-below (<17) do NOT.
  **(Hard — the deterministic two-sided boundary.)**
- **P-008 — Root-Span Error Scope Distinction** — emit traces where **(a)** only an internal child span carries
  ERROR (root succeeds) vs **(b)** both child + root carry ERROR; root-vs-deep is detected deterministically
  (`parent_span_id IS NULL`) and surfaced to the model as a fact. **Per the v2.1 spec amendment, the Conductor
  clause is a `CalibrationRegion` check, NOT a hard assert** — (b) should TEND to produce equal-or-higher
  model-assessed severity than (a); the severity *weighting* is model-side (P-020), with no deterministic
  multiplier.

So **P-005..P-007 carry `class = "Hard"` expected checks** (deterministic Pass/Fail); **P-008 carries
`class = "CalibrationRegion"`** (routes to `ManualCheck` via the default `Verdict → ReportState` mapping) — the
one deviation from the hard-signal family, driven entirely by the amendment.

This is **catalog wiring over primitives that already exist** (Epoch 3): error-spans (P-005 / P-008),
exception-events-fingerprint-control (P-006), severity-logs / the 17-boundary (P-007), multi-service-topology
(P-008 root-vs-child linkage). **No new emit primitive.** The `Scenario.expected: Vec<ExpectedCheck>` TOML
surface was wired in ch1, so this chunk *consumes* it (target: zero further `Scenario`/`ExpectedCheck` model
change). **No live MCP run** happens here — that is the Epoch-8 CLI driver + Epoch-10 E2E.

## Requirement source of truth

- **`.andromeda/refs/pulse-capability-spec.md` §2 (P-005..P-008)** — THE normative source; each P-ID carries a
  "Conductor verification" clause with the concrete emit recipe + a boundary clause.
- **The v2.1 amendment** (`pulse-capability-spec.md` §Amendments line ~809 +
  `.andromeda/runs/2026-06-12T18-52-00-spec-amendment-p008-root-weighting-model-side/amendment.md`) — P-008's
  calibration-region reclassification (root-vs-deep weighting is model-side per P-020, not a deterministic
  multiplier). **This MUST shape the P-008 scenario's `class`.**
- **Existing precedent:** ch1's `scenarios/{receiver-lifecycle-state,last-span-ago-tracking,
  receiver-failed-port-conflict,orthogonal-health-domains}.toml` — the catalog TOML shape
  (`name · p_ids · seed · slo_tier · jitter_ms · [[phases]] {name, gap_ms}` + `[[expected]] {kind, class, expected}`).

## Boundaries

- **In:** the hard-signal scenario TOML file(s) under `scenarios/` (P-005..P-008); their `[[phases]]` +
  `[[expected]]` blocks (`Hard` for P-005..P-007; `CalibrationRegion` for P-008); per-scenario SLO tier (`<5s`
  fits the 500ms-p99 budget); fixture round-trip tests proving each deserializes + garde-validates + builds a
  valid `PhaseTimeline` through the existing scheduler.
- **Out:** the live MCP read-back verification *run* (Epoch-8 CLI driver / Epoch-10 E2E); any **new emit
  primitive** (all exist from Epoch 3 — this chunk only *drives* them from config); the OTHER Epoch-7 families
  (error-baseline-spike, activity-floor, fingerprint-storm, severity-lifecycle, constellation, scrub/pipeline —
  each its own later chunk); the CLI/desktop surfaces (Epoch 8/9).
- **Scope law:** every scenario carries its P-ID(s); **no new inbound listener** — these are pure-egress
  scenarios (no port-occupier; that was P-003's ReceiverFailed leg, already in ch1).

## Surfaces / contracts touched

- **Artifact:** new `scenarios/*.toml` hard-signal files (P-005..P-008) — the primary deliverable.
- `conductor-core::Scenario` / `ExpectedCheck` (`scenario.rs`, `expected.rs`) — *consumed* via `from_toml_str`;
  extended only if a comparison kind / claim class is genuinely missing for these checks.
- `conductor-core::{ClaimClass, ComparisonKind, SloTier}` — class selection (`Hard` vs `CalibrationRegion`),
  per-check comparison kind, per-scenario tier.
- Emit primitives **referenced, not modified:** `conductor-emit` error-spans (P-005/P-008), exception events
  (P-006), severity logs (P-007), multi-service topology (P-008 root-vs-child parent linkage).
- Requirement contract: `pulse-capability-spec.md` §2 per-P-ID "Conductor verification" clauses + the P-008 amendment.

## Open questions — resolve in planning (P4)

1. **File granularity** — 4 TOMLs (one per P-ID, mirroring ch1's "distinct verification recipe ⇒ distinct file"
   decision) vs fewer combined files. P-005..P-008 are four distinct recipes → likely 4 files. Confirm (likely
   AskUserQuestion, consistent with ch1).
2. **P-007 two-sided boundary** — how "ERROR/FATAL (≥17) contribute, WARN-and-below (<17) do NOT" is encoded:
   one scenario walking multiple severities + an `expected` check asserting both the ≥17 contribution and the
   <17 non-contribution. Does the existing `ComparisonKind` express a negative/non-contribution assertion, or
   is a new kind needed?
3. **P-005 candidate/latency check** — what the auto `expected` asserts (candidate present / severity
   contribution) given the <500ms-p99 budget is itself an SLO-tier concern the Epoch-8 evaluator owns; keep the
   TOML declarative (the timing math is not this chunk's).
4. **P-008 calibration encoding** — how the (a)-vs-(b) *relative tendency* (b ≥ a) is expressed as phases + a
   `CalibrationRegion` expected block, given the comparison is relative, not absolute. Likely a single
   calibration-classed marker the Epoch-8 evaluator interprets; confirm the `expected` shape can carry the
   tendency or is simply a `CalibrationRegion` marker routed to `ManualCheck`.
5. **Model-extension depth** — confirm the ch1 `expected` surface covers all four recipes with **no** further
   `Scenario`/`ExpectedCheck` change (the goal), or whether a *minimal* field/kind addition is unavoidable for
   the P-007 boundary or P-008 tendency.

## Definition of done (chunk-level)

- The hard-signal scenario TOML(s) for P-005..P-008 exist under `scenarios/`, deserialize + garde-validate via
  `Scenario::from_toml_str`, and produce a valid `PhaseTimeline` through the existing scheduler — proven by
  fixture round-trip tests (mirrors the ch1 + `error-baseline-spike` fixture tests).
- P-005..P-007 expected checks are `class = "Hard"`; **P-008 is `class = "CalibrationRegion"`** (the amendment),
  with valid/invalid garde `#[case]` rows wherever the model is touched.
- **No new emit primitive and no new inbound bind** introduced; all referenced primitives are the existing
  Epoch-3 ones.
- Determinism preserved: same scenario+seed ⇒ same stream shape (driven under `start_paused`).
- Gates: `cargo nextest` workspace green · clippy `-D warnings` clean · doctest 0 · no new dependency unless justified.
