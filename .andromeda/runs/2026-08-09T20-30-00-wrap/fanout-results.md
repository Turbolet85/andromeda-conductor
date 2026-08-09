# Fan-out results — 2026-08-09-in-lane-sut-scenarios

7 Explore doc-agents, one per spec source, one parallel batch. Report was the sole input.
**0 re-spawns · 0 parse failures.** 5 of 7 carried prose outside the YAML block (stripped; noted as
`contract.proposal-format` friction) — none failed to parse.

## Verdicts

| doc | proposals | verdict |
|---|---|---|
| arch | 0 | clean — Symbols/Crates/Dependencies all "none"; `scenarios/` is registered at directory grain, `UNBACKED_AUTO` is an existing registered ledger whose value only moved |
| security-plan | 0 | clean — three new TOMLs are `validation garde✓` through the existing `Scenario` shape; no spawn/preflight/data-dir touch; zero dependency delta; the `cargo audit` red is the already-documented advisory-DB bounded wait |
| design-system | 0 | clean (vacuous) — every surface flagged `tokens n/a (no UI delta)`; the caption change is a derived value through existing markup |
| **layout-templates** | **1** | **D-layout-surface → routine, APPLIED** |
| test-plan | 0 | clean — new paths carry unit tests at the mandated tier, seeded determinism, zero retries; every command is the test-plan's own |
| obs-plan | 0 | clean |
| a11y-plan | 0 | clean — no interactive UI element added; no violation-schema or `slo_tier` enum change |

## The one proposal

**D-layout-surface** · severity warning · §Surface: cli → Primary screens → `conductor coverage [--write]`
Roll-up caption literal `(11 unbacked)` → `(10 unbacked)`.
Rationale cited the report's `Counts / qualifiers this chunk moved` bullet — the bullet family added to the
report this chunk specifically so a detector would have a fact to bind to. The agent also correctly noted the
strict "new surface" half of its invariant was NOT violated (no UI surface touched); it fired on the
documented-region-content half.

**Validation:** playbook-matched to the 2026-06-15 spec-illustration → sound-impl rule → **routine**.
No cross-contradiction (single proposal). Intent-consistent. Evidence-grounded (verified against the doc
directly at line 178). Applied + sidecar appended.

## Self-raised (orchestrator, check 5)

The plan carried `Expected amendments (wrap): none anticipated` — an **empty coverage floor**, so check 5 had
nothing to reconcile against. One amendment was raised by the orchestrator anyway:

**test-plan §6 Selector strategy** — "mono P-ID tokens (`P-001`..`P-060`)" no longer covers the
coverage-matrix rows this chunk creates (first entries above P-060). De-hardcoded to "the manifest's accepted
set" per the 2026-08-08-sut-capability-manifest precedent, rather than substituting `P-001..P-082` which
re-stales on the next SUT release. **Operator-confirmed.**

Provenance worth recording: the **tests doc-agent saw this and declined to raise it** — "§6's selector
strategy names mono P-ID tokens (`P-001`..`P-060`) … not what any of my three invariants guard". The finding
survived only because it rode in the prose the strip step discards.

## Detector growth (operator-approved)

Third consecutive recurrence of "an existing documented element's count/qualifier went stale" falling outside
every drift-base invariant (out-of-scope treatment ×2, now the unbacked count + the P-ID range). drift-base's
format scopes a detector to exactly one doc, so the cross-doc invariant was appended as two entries:

- `D-layout-derived-count` (layout-templates — sample captions, wireframes, selector labels)
- `D-tests-derived-count` (test-plan — selector labels, fixture counts, tier tables)

Both carry the fix-discipline in their `check`: **name the set, never substitute a fresh literal that
re-stales.** No playbook rule was added — nothing escalated, so no verdict-pattern was owed; the gap was
detection, not judgment.

## Cascade

- **Cross-master citations:** grepped all 7 bodies for both old values — CLEAN. (`layout-templates-amendments.md:37`
  carries `(11 unbacked)` but is append-only sidecar HISTORY, correctly untouched.)
- **Lateral binds:** test-plan §3 ↔ obs-plan §3 unaffected (the edit was §6); a11y ↔ obs schema untouched.
- **Leaves:** `design-summary.md:32` and `tests-summary.md:24` both name the *mechanism* (`(N unbacked)`, "the
  `UNBACKED_AUTO` pin") rather than the literal, so re-derivation from the amended sources yields identical
  text — **0 leaf edits**. The de-hardcode discipline paying off.

**Drift = 0.** 2 amendments applied · 0 escalations · 1 operator dialogue round (detector scope + self-raise).
