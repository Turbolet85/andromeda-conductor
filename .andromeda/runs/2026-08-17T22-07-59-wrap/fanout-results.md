# Fan-out results — 2026-08-17-fingerprint-semantics-token-leading

7 doc-agents, one batch. **1 proposal · 6 clean · 0 escalations.**

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | **1 proposal** (D-arch-decisions, warning) | D-arch-resources clean · D-arch-decisions fired |
| security-plan | clean | D-security-input · D-security-subprocess · D-security-deps |
| design-system | clean | D-design-tokens · D-design-derived-count |
| layout-templates | clean | D-layout-surface · D-layout-derived-count |
| test-plan | clean | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count |
| obs-plan | clean | D-obs-instrumentation · D-obs-stack · D-obs-redaction |
| a11y-plan | clean | D-a11y-surface · D-a11y-obs-schema |

Raw twin kept for the one doc carrying proposals: `.raw-fanout-arch.md`.

## The proposal (applied)

**D-arch-decisions** · warning · §Established Decisions [Read-Back Dependency Posture] — retire the
leading-path-segment narrowing for TOKEN-LEADING wording; narrowings 2 → 1; re-pin to the two new tests;
re-base the transcription citations from HEAD `d090314` to `efabe8e`.

Validated **routine** under playbook line 28-30 (spec-illustration → sound-impl reconciliation). Explicitly
NOT the decision-reversal escalate rule (line 97): the locked decision — *Conductor recomputes Pulse's
derivation* — is unchanged and in fact reaffirmed; only its descriptive detail moved, because the SUT moved.
No tech choice proved unworkable.

## Clean returns — the evidence each cited (orchestrator re-verified the absence claims)

- **security-plan** — Dependencies bullet is zero-delta; `is_token_boundary` is a private helper over an
  internally-rendered string (no external-input boundary); the `[phases.emission].variants` value change stays
  inside the existing non-empty-variant-set garde rule; nothing touches the sidecar spawn / preflight /
  data-dir. The 26th `cargo audit` pin re-verifies the documented overlap remedy rather than triggering the
  new-dependency-under-red-audit clause.
- **design-system** — the only new surface self-reports `tokens n/a (no UI)`. Grepped palette rows, the ANSI
  map, token labels and reuse tallies: "fingerprint" appears only as a SET MEMBER of the mono ID-cyan tier and
  as a column name, never as a variant count. The `11 of 34` token tally and the 6-lamp / 5-`ReportState` sets
  are untouched.
- **layout-templates** — no user-facing surface added. Grepped for the variant triple (zero hits; the only
  `identical` matches are "rendered identically"), the leading-segment wording (zero), and `610` (zero). The
  `fingerprints` column samples are ellipsized placeholders, not moved counts.
- **test-plan** — grepped §1 Critical Path 2 and §6 Fingerprint-storm, the two sites the report flagged: both
  describe the storm via `cue_kind = "retry_storm"`, the tier ladder, exactly-one-incident and the empty
  `fingerprints` field — none states variant membership. The `closed at six` literal is the status-label set,
  unrelated. New path carries unit-tier tests per §2/§4; runner matches §2/§3/§4 exactly.
- **obs-plan** — the variant mix is stated NOWHERE (§4's storm block and §1's Critical Path 2 row carry only
  span names and field lists); `fingerprints_in_batch` keeps its name. Instrumentation n/a is compliance, not
  drift — §11 names fingerprint generation an inner-loop hot path to profile before adding a span. Redaction
  moved the SAFE direction: token-leading absolutes now normalize to the empty form (strictly more stripping).
- **a11y-plan** — no interactive UI element; the report states no violation-schema change and the obs §6
  envelope reproduced in §1/§3 is field-identical. `fingerprints[]` carries axe tuples on a11y rows, not
  Conductor's derivation, so the two stay aligned.

## Orchestrator validation (the 6 named checks)

1. **Playbook** — 1 proposal, routine (above). No unease.
2. **Cross-contradiction** — single proposal, no opposing pair.
3. **Intent-consistency** — the proposal matches the plan's Expected amendment 1 and operator directive 2.
4. **Absence needs evidence** — three absence claims re-verified by the orchestrator's own greps rather than
   accepted: the arch §Standard Contracts duplicate (operator-directed sweep — that section points AT
   [Read-Back Dependency Posture] rather than restating the claim, so no `dependent-of` proposal is due);
   test-plan (only hit is an unrelated self-obs "two line variants" at :188); obs-plan (zero hits). The retired
   wording survives at exactly one live site — the one amended — plus the append-only sidecar, which is history
   and is never edited.
5. **Expected-amendments reconciliation** — all 3 plan entries disposed: #1 proposed and applied; #2 (test-plan
   §6/§1) and #3 (obs-plan §4) verified as genuine no-ops with evidence. Floor met, no silent under-run.
6. **Disproved-claims disposition** — both report entries disposed: the P-017 clause by this amendment; the
   derivation-drift consequence recorded in the amended body and the sidecar, and routed to P3 curation as a
   learning candidate.

## Cascade

- **Lateral binds** — test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema: neither side changed, both untouched.
- **Cross-master citation sweep** — grepped all seven masters plus `.claude/` and `CLAUDE.md` for the retired
  wording: hits only at `architecture.md:65` (amended) and `architecture-amendments.md:309-323` (append-only
  history, correctly left standing). No other master cites it; no curation home carries it.
- **Leaf re-derivation** (architecture.md → CLAUDE.md `GENERATED:setup:*` + `.claude/docs/stack.md`) —
  recomputed and **verified no-op in content**: CLAUDE.md's two fingerprint mentions are module-location facts
  (`conductor-emit` owns the primitive) that remain true, and stack.md's blake3 row states Conductor recomputes
  the SUT's derivation rather than its own — which this chunk makes more true, not less. No leaf edit was due.
