# Fan-out results — 2026-08-15-canary-spans-pulse-fingerprints

7 doc-agents, 18 detectors. **2 proposals (both arch), 6 docs clean, 0 escalations.**

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **2 proposals** (D-arch-decisions primary + 1 `dependent-of`) |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count | `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

## Validation (5 named checks + disposition)

1. **Playbook** — both proposals routine; operator directive 2 adjudicated the fingerprint amendment in
   advance (anchor, correction content and per-occurrence sweep policy all specified). No escalation.
2. **Cross-contradiction** — none; the two proposals point the same way (primary + its duplicate).
3. **Intent-consistency** — report aligns with the working-route entry + plan acceptance criteria. The
   fingerprint mismatch is a DISCOVERY the leg made, not a divergence from intent.
4. **Absence needs evidence** — the sweep was re-run by the orchestrator, not taken on the agent's word:
   `computed to match Pulse's derivation` and `fidelity carrier` occur in exactly ONE master
   (`architecture.md:60`); the §Standard Contracts occurrence (line 83) restates the round-trip's proving
   power *without* the phrase, which is why it is a `dependent-of` rather than a second primary.
5. **Expected-amendments reconciliation** — the plan's `Expected amendments (wrap)` named
   `architecture.md` §Occupied Resources (the second-gap paragraph); **no detector proposed it**. Raised by
   the orchestrator as the chunk's coverage floor — routine, substantiated by the report's Outcome.
6. **Disproved-claims disposition** — both entries disposed: (1) the fingerprint claim → proposals 1+2;
   (2) the plan's step-6 golden re-lock prediction → not a spec claim; routed to curation/handoff as an
   instance-level coverage note.

## Applied (4 body edits / 3 logical amendments, all in `architecture.md`)

- **§Established Decisions [Read-Back Dependency Posture]** — the match parenthetical retired; both
  derivations named (FNV-1a 64-bit/16-hex over type + frame functions vs blake3-truncated-16-bytes over
  type + NUL + normalized stacktrace, read back as an 8-char prefix); failure recorded as BY CONSTRUCTION;
  alignment assigned to the successor entry.
- **§Standard Contracts (Readiness gate)** — the round-trip annotated (`dependent-of`): a failed round-trip
  is a derivation mismatch, not evidence of broken data-dir/workspace wiring.
- **§Occupied Resources (`pulse-run-contract.toml`)** — the second gap recorded CLOSED with its cause
  reattributed to Conductor's side (the `spans` PK vs `ok_span`'s constant identity), the fixed leg's
  numbers recorded, and the prior leg's storm-zero-append explanation held explicitly INFERRED-not-proven.
- **Same paragraph, trailing sentence** — the telemetry-reading instruction re-based to carry both
  directions (cumulative discriminators 0 on the broken path, `2`/`1` on the fixed one, with
  `tracked_fingerprints_count` still sampling 0 on a healthy late tick).

## Cascade

- Cross-master grep for every retired wording (`This gap is NOT closed` · `ingest→fingerprint gap` ·
  `both 0 across 31 tick` · `computed to match` · `never invoked`): **no other master carries any of them.**
  The two surviving `architecture.md` hits are the corrected text itself (the negated claim and the
  past-tense symptom description).
- **Curation-home hit (never cascade-edited):** `.claude/rules/verification-harness.md` `## Session
  Additions` carries the stale chain (`gap is NOT closed`, `producer-dependent`, `never INVOKED`,
  `fidelity carrier`) → routed to **P3 curation** as an in-place extension of that entry.
- Leaf re-derivation of the changed source (`architecture.md` → CLAUDE.md `GENERATED:setup:*` +
  `.claude/docs/stack.md`): recomputed from current sources, **no change** — CLAUDE.md's
  Preflight-integrity warning still names the same FIVE preconditions (the amendment explains why the
  canary-fingerprint one currently fires; it does not change the set) and `stack.md` carries none of the
  amended facts.

**Drift = 0 on exit:** 3 amendments applied · 0 escalations open.
