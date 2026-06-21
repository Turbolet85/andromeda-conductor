# Scope — Expected-outcome + SLO timing model

**Marker:** `2026-06-21-expected-outcome-slo-timing-model`
**Epoch:** 5 — Verification & read-back (chunk 5 of 6)
**Primary seam:** `conductor-verify` (with a likely `conductor-core` config surface — see Open seam question)
**Working-route intent:** _Expected-outcome + SLO timing model — per-scenario expected blocks, tier-scaled
tolerance <5s/<20s/<90s. PREREQ: feeds matched/observed/expected into the ClaimClass/classify→Assessment
mechanism from verdict-assertion-policy-split (concrete comparison kinds deferred here)._

## What this chunk builds

The evaluation layer that sits **between** a read-back observation and the existing
`classify(class, matched, observed, expected) → Assessment`. The prior chunk
(`verdict-assertion-policy-split`) shipped `classify` taking `matched: bool` as an already-computed
input and carrying **no timing** at all; both the comparison that produces `matched` and the SLO
timing were explicitly deferred to here. This chunk supplies them:

1. **Per-scenario expected-outcome model** — a declarative block describing what read-back must observe
   for a scenario's check (the comparison *target*) plus which SLO tier its reaction deadline lives in.
   Declarative, serde-deserialized, garde-bounds-checked at load (the established config discipline).

2. **Concrete comparison kinds** (the deferred piece) — the closed set of ways an `observed` value is
   compared against its `expected` target to yield `matched: bool`, drawn from the spec sources. The
   anticipated kinds (to be confirmed in planning against the specialist extracts):
   - exact equality (deterministic hard signals);
   - membership / contains (incident present in `query_incident_list`, fingerprint ∈ set);
   - presence / absence (a field/flag is set);
   - numeric ordering / threshold (p50 ≤ p95 ≤ p99, error-rate band);
   - count-floor (≥50-sample latency, ≥10-span error-rate) — a floor that, when unmet, routes to the
     calibration region rather than hard-failing.

3. **SLO timing model** — the tier-scaled bounded-tolerance deadline:
   - a tier over exactly `<5s` / `<20s` / `<90s` (the closed `slo_tier` enum already named in the
     run-report envelope);
   - latency measured **journal-relative** — `read_back_observed_at − journal_emitted_at`, never
     wall-clock-from-test-start, stamps from `std::time` (not the virtual clock);
   - a hardware-profile-aware tolerance band per tier (architecture §Timing-Tolerance Model);
   - the deadline outcome: **within tolerance** ⇒ the timing dimension passes; **exceeded** ⇒ a
     deterministic/hard deadline hard-fails, while a sample-count-floor / model-interpretive timing
     routes to `CalibrationRegion` (never hard-fails).

4. **Wiring to `classify`** — the comparison result + the timing outcome resolve into the
   `matched: bool` + `ClaimClass` that the existing `classify` consumes, so a check end-to-end becomes:
   `(expected block, observed value, journal timing) → matched + class → classify → Assessment`.

## Boundaries (NOT in this chunk)

- **Not** the run-report serializer / `runs.db` writer / Markdown report (Epoch 6) — this computes
  `latency_ms` + `slo_tier`, it does not persist them.
- **Not** the 60 per-P-ID filled-in scenario `expected` blocks (Epoch 7 scenario catalog) — this builds
  the **model + evaluator**, not the populated catalog. At most one or two illustrative fixtures for tests.
- **Not** the read-back transport (already shipped: `mcp-read-back-client`) — it consumes an already-
  observed value, it does not fetch from MCP.
- **Not** new live-Pulse comparison — pure evaluation logic over already-observed values; CI-testable
  with fixtures, no live leg.
- **Does not** breach the verdict/error wall — evaluation of a well-formed expected block returns a
  value (like `classify`); `Result::Err`/`VerifyError` is reserved for a genuine harness fault
  (e.g. a malformed/garde-invalid expected block at load). The infallible-vs-fallible boundary is a
  planning decision.

## Surfaces / contracts touched

- `conductor-verify` (primary) — comparison-kind + SLO-timing evaluator beside `verdict.rs`; feeds the
  existing `classify`.
- `conductor-core` (likely) — the `expected` block as part of (or alongside) the Scenario config model
  (`scenario.rs` / `phase_spec.rs`), serde + garde; plus the `SloTier` enum if not already present in
  `run_record.rs`. **The core-vs-verify seam split is the open scope question for planning.**
- Run-report envelope fields **computed** here (defined elsewhere): `latency_ms`, `slo_tier` — and the
  `matched`/`verdict` that flow into the envelope's `verdict`/`state`.
- Upstream consumed: `conductor-verify::{ClaimClass, Assessment, classify}` + `conductor-core::Verdict`.

## Open seam question (resolve in planning, P4)

Where does the declarative `expected` block live — in `conductor-core` (extending the Scenario config
model, serde + garde, matching "config/types in core, verification logic in verify") or wholly inside
`conductor-verify`? The star-topology invariant (seam crates import only `conductor_core`) and the
existing split (scenario model in core) bias toward **core for the data model, verify for the evaluator**
— to be confirmed against the architecture + test extracts before finalizing.

## Acceptance anchor

Same scenario + same observed value + same journal timing ⇒ identical `matched`, identical SLO tier
outcome, identical `Assessment` (determinism). A deterministic comparison breach ⇒ hard `Fail`; a
sample-floor / model-interpretive miss ⇒ `CalibrationRegion`, never hard-fail. Journal-relative latency
only. No host-path leak through the redaction edge.
