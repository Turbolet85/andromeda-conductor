# Phase 4 Review — round 1

## Findings audit requested (F3, F4)

- **F4 (v2-06 + v2-07) — already first-class.** Epoch 1, chunk `SUT load envelope — proven-good storm bounds for scenario authors plus environment-suspect flagging of over-envelope runs`. Both halves carried: bounds recorded where scenario authors see them, and over-envelope runs flagged environment-suspect rather than reported as Pulse failures. No change.
- **F3 (v2-05) — was a trailing clause, now its own chunk.** It sat as `…with the interpretation-correctness posture recorded` appended to the classification chunk — a sub-clause on a chunk whose acceptance is about classification, i.e. exactly the shape that evaporates at implement time. Promoted to Epoch 1 chunk `Interpretation-correctness posture — one real-model leg with a known root cause and asserted top hypothesis, or a recorded deferral naming its owner`, which names both acceptable outcomes so neither can be satisfied by silence. Classification chunk rewritten to `…the boundary a recorded decision` (its own scope, undiluted). Epoch 1: 6 → 7 chunks.

## Deferred-item decisions (operator)

1. **Secret-scanning CI gate — OUT of 0.2.0**, and recorded as a known unrealized bootstrap residual (see below). Rationale is the operator's: outside the authored intent, Conductor owns no secrets by design.
2. **a11y violation JSON aligns to obs — ACCEPTED, on binding direction, not amendment recency.** The binding chain is test-plan §3 log format → obs-plan §6 → a11y violation JSON, and the standing rule is that downstream aligns to upstream: **a11y aligns to obs, never the reverse.** The a11y-plan amendment is therefore **MANDATORY, not optional**, and belongs in the chunk that builds the emission.

   **FORMAT vs CARRIER — the amendment must not conflate these** (correcting an inverted rationale recorded earlier in this run):

   - **FORMAT — not obs's discretion, not re-openable.** `obs-plan.md:425` §6 "Log Coverage" opens by declaring its schema a *"Binding contract from upstream-context Section 5 Test Plan Excerpt → Test Harness Contract Summary (reproduced verbatim below)"*. §6's schema is **inherited verbatim from test-plan §3**; obs transports it, obs does not pick it. a11y's violation JSON conforms to that schema, full stop — not a recency question and not a shape obs may choose within.
   - **CARRIER — genuinely obs's call, settled per §9 inside the amendment.** `obs-plan.md:501` §9 "CI Integration" is the artifact table (which file, when produced, where stored, how an agent retrieves it — `logs/agent-latest.jsonl`, nextest JSON, supply-chain report). Whether the violation record rides the run-report envelope or a self-obs line, and how CI uploads it, is decided there.

   The earlier framing ("obs-owned format … that selection is obs's to make") collapsed the two and was wrong. If it rode into the amendment, the amendment could select a record shape that is **not** §6's schema and silently break the test-plan §3 → obs §6 → a11y chain — precisely the drift class the align-downstream-to-upstream rule exists to prevent.

   The route line stays as written — `aligned to the obs-owned format` is the right altitude for a WHAT line; it is the reasoning above that the amendment inherits.
3. **Sweep split from gate — ACCEPTED as proposed.** `Desktop a11y sweep` is now explicitly operator-gated against a live Pulse; `A11y CI gate` runs the same specs on Linux+xvfb without Pulse and fails on any violation. This matches the standing constraint that live-Pulse legs are operator-gated and never CI gates. (Zero-retry dropped from the gate line — it is a universal invariant in CLAUDE.md §Critical Warnings and test-plan §10, not a per-chunk marker.)

## Held as-is (not re-opened, per operator)

- `scenario.run` root span in Epoch 2 — instrumentation before the work it observes.
- `Dispatcher determinism goldens` insert — replacing `coarse_emit` invalidates 0.1.0's committed goldens.

## Notes for future runs

- **Known unrealized bootstrap residual: `secret-scanning-ci-gate`** (security-plan §Bootstrap phases, §Secret Management). Absent from `.github/workflows/ci.yml` and from `.gitignore`'s `*.p12`/`*.pem`/`*.cer` guards. It has now survived a full version un-built. **No drift detector will ever surface it** — detectors read the chunk report, not plan-versus-reality — so it only exists as long as it is written down. Durable copy recorded in `conductor-0.2.0/requirements.md` §Carried residuals.
- **Next-version validator blindness (why two a11y inserts were rejected).** In next-version mode a Phase 2 validator reads only its specialist plan plus the draft route — it cannot see what previous versions delivered, so it reads its plan's whole bootstrap surface as unrealized and proposes re-installing shipped tooling. Here a11y proposed installing the axe/Lighthouse harness and the colorjs.io contrast harness, both shipped at `2026-06-27-desktop-a11y-harness-setup`. Expect this class of false-positive Insert on every next-version route run; check proposed bootstrap inserts against the prior version's route before applying.
