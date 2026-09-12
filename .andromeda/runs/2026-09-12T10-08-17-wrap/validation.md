# Validate — 10 proposals, 2 groups

## 1. Playbook check

**Amendment-2 group (3 proposals: arch `:59` D-platform-claim · test-plan §9 primary · test-plan §6 dependent) → ROUTINE** under playbook `:149` (a master's own explicitly-provisional claim retired by the measurement the sentence itself names as its precondition). All three bounding clauses hold:

- **(a) DIRECTNESS** ✓ — the sentence names its own precondition twice over. "returned NO reading at all *because that probe's step runs after the isolation step stops the app*" — arm A removed exactly that cause by moving the invocation inside the live window. "NOT a demonstrated cause, *since no probe varied it*" — arm B varied exactly it. Both read from the artifacts the sentences name (CI runs `34645345201` / `34654076633`, the chunk's `evidence/reading.md`).
- **(b) EXPLICIT STATUS** ✓ — the replacement states what is now measured (module reading taken; skew retired by control; elevation established) AND what remains OPEN with its owner (the mechanism is `hypothesis:` and unmeasured; the endpoint is still closed; the remedy is owned by the `v3-02` route entry).
- **(c) DISCRIMINATOR SHAPE** ✓ — not a PARTIAL result: the reading was taken and the variation completed. The skew, which WAS a discriminator, is written FALSIFIED with the new open variable named.

**Amendment-1 group (7 proposals: security `:185` primary + `:86`, `:87` · arch `:205` primary + `:59`, `:147`, `:248`) → ESCALATE.** Playbook `:149` is the only candidate rule and it **fails clause (a)**, which is decisive:

> the measurement must be the sentence's OWN named precondition … A measurement of the same subject reached by another route does NOT qualify and stays escalate.

Both primaries name their retire-condition explicitly, and it is **measurably unmet**:
- `security-plan.md:185` — "always-latest Evergreen, **deliberately and temporarily** … pinned once it actually GATES"
- `architecture.md:205` — "The float is scoped to the probe: **once the `a11y` job actually GATES** (the endpoint opens and the routine arm runs to completion), the runtime is pinned"

Measured at run `34654076633`: the A11y job is `failure`, and `[diag] (a) bare-app DevToolsActivePort first seen: never within 90s`. The job does not gate. The posture changes for a *different* reason — the probe's SUBJECT moved from the runtime major (falsified as the discriminator at run `34280136892`) to the driver/runtime skew, against which fetching the newest is the confound rather than the measurement.

Checked and NOT matching: the boundary-widening rule (escalate, never-routine) — its subject is a chunk that **widens** what crosses a hardened boundary; this chunk strictly **narrows** the CI-time fetch. Subject mismatch, no match.

**Resolution:** the operator's wrap directive, item 3, rules exactly this framing ("`security-plan.md:185`'s always-latest rationale is EXHAUSTED by arm C rather than contradicted — its stated subject was the runtime major, which run 34280136892 falsified as the discriminator, and the outbound fetch now narrows to the below-floor case"). Recorded as **escalated → resolved by operator directive**, with the failed clause named. No playbook rule is proposed: the directive resolves this instance, and minting a rule that routinised a posture change whose own named retire-condition is unmet would be exactly the silent precedent-widening the escalate branch exists to prevent.

**Binding constraint on the applied text:** no amended body may state or imply that the `a11y` job now gates. The pin-once-it-gates disposition stands, unmet.

## 2. Cross-contradiction
`architecture.md:59` receives one proposal from each group. **Complementary, not opposing** — one narrows the provisioning mechanism (conditional below the floor), the other retires the cause verdict (reading taken, elevation established). Applied as a single coherent rewrite of that bullet. No other section collision across the 10.

## 3. Intent-consistency
The chunk's working-route entry — "the unread module-version probe placed where the app is alive, and the elevation difference varied" — is fully discharged, and both plan acceptance criteria for it read MET against the diff. Arm C was **not** in the original intent: it was minted mid-implement on the operator's ruling after arm A's reading exposed the skew. That is a *justified* divergence (planning could not have known the confound existed until arm A measured it) ⇒ intent was incomplete ⇒ the plan was amended at the time, operator-directed. Consistent; no escalation.

## 4. Absence needs evidence
- Report's "no spec states the driver/runtime skew as a claim": measured across all seven masters; the single apparent hit (`test-plan.md:307`) was READ and is the Windows-host SET line, not a skew claim. Absence holds.
- Each clean return cites the grep establishing its zero (design-system, layout-templates, obs-plan, a11y-plan).
- The seven-master sweeps for `always-latest` / `0.645` / `demonstrated cause` were run by the orchestrator at report authoring, and corrected a sole-owner claim into a two-owner one before the fan-out read it.

## 5. Expected-amendments reconciliation
Both plan entries are covered — and **exceeded**. The plan named one site each; the report's sweep found two each; the `dependent-of` mechanism found **7** and **3**. No entry under-ran. Five of amendment 1's seven sites carry no `always-latest` token at all.

## 6. Disproved-claims disposition
| # | claim | disposition |
|---|---|---|
| 1 | always-latest rationale exhausted | amendment-1 group (7 sites) — escalated, resolved by directive |
| 2 | `architecture.md:59` no-reading + unvaried elevation | amendment-2 group (3 sites) — routine |
| 3 | driver/runtime skew retired as a cause | DISPOSED via amendment 2 — no master states it as a claim (measured); the one adjacent sentence, `a11y-plan.md:115`, states a cross-major pair measured *working* and is consistent, not falsified. test-plan §6's "leaving the image open with no named candidate" IS retired, by amendment 2's dependent |
| 4 | `v3-02`'s terminal no longer a candidate permanent exclusion | routed to **P5 route-resolve** as the entry annotation (operator directive item 4) — not a spec amendment |

All four DISPOSED. Zero left silent.

## Outcome
10 proposals: **3 routine · 7 escalated-and-resolved · 0 rejected · 0 open.**
