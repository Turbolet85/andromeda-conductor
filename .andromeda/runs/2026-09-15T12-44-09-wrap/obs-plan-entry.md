
## 2026-09-15-structurally-dead-assertion-class-retired — findings-counter-refresh restated as declare-only (both statements on the line)

**Section:** §4 Span / Trace Coverage → Known-residual classification path → Delegated-timing family — both occurrences applied.

**Change:**
- The family's declare-only roster widened three → four: `halo-hue-encoding` · `service-constellation-discovery` · `report-render-surface`, joined by `findings-counter-refresh` on 2026-09-15.
- The exclusion clause `findings-counter-refresh is not declare-only (one [[expected]]) … its unmet CountAtLeast floor grading CalibrationRegion` is retired. The body now states that it IS declare-only as of 2026-09-15, that its single `CountAtLeast` floor was retired as structurally dead (the floor graded `evidence_count`, summed from a `span_refs` vector whose only non-test writer in the SUT sets it empty), and that it reaches `KnownResidual` by the same degraded read-back with the declare-only envelope shape the section already specifies — `verdict` null, no `CalibrationRegion` grade from that floor. Its harvest-tier leaf `metric.findings.counter_refresh_ms` is unchanged.

**Why:** the chunk retired that scenario's only `[[expected]]` block, falsifying both halves of the §4 sentence. The claim sits TWICE on line 349 — the roster at offset 66 and the exclusion clause at offset 1402 — so a single-statement apply would have left the enumeration contradicting the fix inside one paragraph.

**Sweep (this pass, after every amendment was authored):** the same 11-pattern sweep recorded in `architecture-amendments.md` for this marker, run over 28 files with a known-positive control. In this doc it returned exactly **2 hits, both on `:349`, both amended**; no other obs-plan site states the retired claim, and `.claude/rules/observability.md` and `.claude/docs/obs-summary.md` carry 0 hits for `declare-only` / `delegated-timing` / the four scenario names, so no obs leaf re-derivation was owed.
