# Fan-out results — 2026-08-15-canary-storm-autonomous-band

7 Explore doc-agents, one parallel batch. **6 proposals, all from arch; six docs clean.**
Raw twin kept for arch only (`.raw-fanout-arch.md`); the six clean returns are recorded here, which is
their sanctioned audit artifact.

| doc | verdict | note |
|---|---|---|
| **arch** | **6 proposals** | 5 applied, 1 dismissed — detail below |
| security-plan | `proposals: []` | no new external-input boundary (the contract term was REJECTED at P4, so none was added); sidecar spawn untouched; deps zero-delta and the audit red correctly classed as the advisory-DB fault §Dependency Security already prescribes |
| design-system | `proposals: []` | no new UI element; the doc's only storm-count sample (`design-system.md:265`, "stormed the fingerprint 12×/30s") already reads 12 — the value the code now produces |
| layout-templates | `proposals: []` | no new surface; the live leg exercised the already-documented `conductor preflight [--json]` path; the doc's storm literal (`layout-templates.md:144`) already reads 12 |
| test-plan | `proposals: []` | no new code path; every runner already spec'd; §3 ↔ obs §3 untouched on both sides; the doc bakes no canary-storm count |
| obs-plan | `proposals: []` | no new must-trace op; `emit.batch` witness pre-existing; zero telemetry-dep delta; every path in the report repo-relative, no redaction surface engaged |
| a11y-plan | `proposals: []` | headless cli surface, §1 not-assertable; no violation-schema or obs-envelope change |

Three of the six independently noted that the report's disproved claims are arch/rules surface, not theirs.

## arch proposals — validation outcomes

| # | detector | proposal | outcome |
|---|---|---|---|
| P1 | D-arch-resources | §Occupied Resources `:161` — scope the `BootstrapState::Ready` diagnosis to the baseline-derived cue families | **APPLIED** — routine; expected-amendment #1; report §Spec-claims-disproved #1 |
| P2 | D-arch-resources | §Occupied Resources `:161` — retire "reaching a live incident requires a Pulse-side change"; state the tier band | **APPLIED** — agent raised it `escalate` ("the retired sentence is a live operator directive"); **resolved by the operator's standing WRAP directive 2**, which names this exact correction. Recorded as escalated-and-resolved, not silently downgraded |
| P3 | D-arch-resources (`dependent-of`) | §Established Decisions [Read-Back Dependency Posture] line 60 — the write-path's `≥5 same fingerprint / 30s` must carry the tier band | **APPLIED** — routine; a genuine duplicate occurrence of the claim P2 retires, found by the grep-for-other-occurrences discipline. Applied atomically with its primary |
| P4 | D-arch-resources | §Occupied Resources `:161` — un-retire the second Pulse-side gap; sharpen to producer-dependent | **APPLIED** — routine; expected-amendment #3 (directive 2's third correction); report §Outcome trio |
| P5 | D-arch-resources | §Occupied Resources `:161` — refresh the windowed-gauge reading instruction off the retired six-occurrence size; date the span-count figures | **APPLIED** — routine; genuine derived-value staleness caused by this chunk. **This proposal also exposed a report defect**: the `Counts / qualifiers moved` bullet said "none" when `6 → 12` is a documented derived value; the detector found it under Symbols/APIs anyway. Report bullet corrected during this phase |
| P6 | D-arch-decisions | §Established Decisions — record the P4 rejection (canary sizing stays a compile-time constant, not contract data) as a scoped exception | **DISMISSED** — its premise is false. Grepped: arch states **no** Established Decision locking "SUT facts belong in `contracts/*.toml`"; the three contracts are described in §Occupied Resources as artifacts, not as a general rule. Recording an exception to a rule the doc does not state would author a new decision rather than bring the body to current truth, and the over-reach dismiss family (arch omits realization detail) covers the shape. The P4 rejection already has two homes: report §Decisions & corrections and plan §Constraints & rejected approaches |

## Validate checks

1. **Playbook** — P1/P3/P4/P5 routine (spec-wording→measured-truth reconciliation, the established 2026-06-15 family, here driven by measurement rather than implementation). P2 escalate→resolved by standing directive. P6 no rule matched **and the premise was verifiable** — verified false, so dismissal is grounded, not uneasy-escalate.
2. **Cross-contradiction** — none. P3 is `dependent-of` P2 and consistent with it; primary applied ⇒ dependent applied.
3. **Intent-consistency** — the report DOES diverge from the chunk's working-route entry, which claims preflight "reaches `ready:true`". Measurement outran intent — a **justified** divergence ⇒ intent was incomplete. Dispositioned to **P5 route-resolve** per operator directive 3 (the master record describes actuals), not left silent.
4. **Absence-needs-evidence** — no absence claim among the proposals. The report's own absence claims cite their searches (the trio across 15 ticks; the `crates/triage/` grep establishing a single `BootstrapState::Ready` gate).
5. **Expected-amendments reconciliation** — the plan listed 3. All covered: #1→P1, #2→P2, #3→P4. The fourth plan item (`verification-harness.md`) is a rules file and correctly routes to P3 curation, not the cascade. Floor met; nothing under-ran.
6. **Disproved-claims disposition** — all 3 report entries disposed via P1/P2/P4. The rules-file twin → P3 curation.

## Cascade

- **Cross-master citation grep** — the retired wordings appear in **no other master**. The four residual hits inside architecture.md are the new text naming the claims in order to retire them (verified individually), not survivals.
- **Curation homes (preserve-verbatim, never cascade-edited)** — `.claude/rules/verification-harness.md` carries "Reaching a live incident needs a Pulse-side change … not a Conductor-side retune" and the six-occurrence reading rule → routed to P3 as an in-place chain extension. `.claude/rules/testing.md` also matched `BOOTSTRAP_WINDOW_SECONDS`, but its entry is the `asserted`-check-kind lesson and the 80× arithmetic, **both still true** — no correction owed.
- **Leaf re-derivation** — architecture.md changed ⇒ recompute CLAUDE.md `GENERATED:setup:*` + `.claude/docs/stack.md`. Recomputation yields **no change**: neither leaf carries any amended topic (checked `BootstrapState` · `baseline_state` · `CANARY_STORM_COUNT` · `AUTONOMOUS_THRESHOLD` · fingerprint observer · Pulse-side change · cue evaluator — all absent), and the one leaf claim in the area (CLAUDE.md's "the gate's FIVE named preconditions") is still accurate since the preconditions were untouched. CLAUDE.md 128/200.

**Drift = 0 at exit:** 5 applied · 1 dismissed with verified grounds · 1 escalation raised and resolved · 0 open.
