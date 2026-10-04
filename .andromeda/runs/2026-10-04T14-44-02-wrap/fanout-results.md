# Fan-out results — 2026-10-04-second-test-surface-corrective

Seven Explore doc-agents, one parallel batch, prompts substituted from `amendment-flow.md` and sent verbatim (detector
slots 28 = the drift-base's `doc:` names, asserted before sending). Every return stripped of its trailing `#` commentary
(the substance kept below); entity probe over every return: 0 `&lt;` / `&gt;` / `&amp;` — `entities=0`.

## Verdicts

- **architecture** — `proposals: []`. Stripped commentary: D-arch-resources / -decisions / -collision no hit (no port,
  socket, env var, crate; the grading tree is a child module of an existing target); D-arch-registry-size is tooling
  (orchestrator measures after Apply); D-platform-claim: grepped the body + four keyed contracts for clippy /
  non-blocking / PSNative / LASTEXITCODE / bundled default / pwsh — no stating sentence.
- **security-plan** — `proposals: []`. No new input surface, sidecar untouched, Dependencies none; clippy hits `:87`,
  `:190`, `:227` only list clippy as a step.
- **design-system** — `proposals: []`. No UI; none of the moved counts appears in the doc.
- **layout-templates** — `proposals: []`. Commentary kept as a REFINEMENT for the apply: `layout-templates.md:196`
  ("a caller preempting native-command exits diverges the `.ps1` leg with zero script delta") stays true — a caller
  setting `$PSNativeCommandUseErrorActionPreference` still throws at the red cargo line (exit 1, CI#34689135760)
  before the new per-line check runs, so "with that line's exit" holds only for a caller that does not set it.
- **test-plan** — 3 proposals (below).
- **obs-plan** — 2 proposals (below).
- **a11y-plan** — `proposals: []`. No interactive element, no schema change; a11y clippy mentions (`:115`, the
  ci-integration key) list clippy as gated.

## obs-plan

O1 — detector D-obs-ci-gates · warning · §10 → Build / deploy failure conditions · change: replace the clippy line
with a `-D warnings` lint failing `agent-run run`'s bundled default (sh measured 101; ps1 per-line check, red path
unmeasured) and so CI's `rust` dogfood step; the `--e2e` arm runs no clippy · basis `.andromeda/obs-plan.md:490`,
`evidence/carry-measurement.md` §1–§4.
**Disposition: APPLY** — check 1: the plan's `Expected amendments (wrap)` entry names this change itself (a recorded
direction), and playbook `Accurate this-chunk addition` matches (the report's Harness / gate surface + Spec claims
disproved carry it); check 6: disposes the report's obs-plan:490 disproved claim. Text re-derived from the report.

O2 — detector D-obs-ci-gates · warning · §9 → Pipeline integration (Lint / typecheck row) · dependent-of
D-obs-ci-gates · change: clippy's consumer from "CI annotations" to the dogfood step's exit + the job log · basis
`.andromeda/obs-plan.md:443`, `.github/workflows/ci.yml:108-116`.
**Disposition: REJECT as proposed** — the re-derivation tell: its rationale and basis cite `ci.yml:108-116` and "no
problem matcher", neither of which the report carries. **Its fact RAISED by the orchestrator (check 5)**: the plan's
expected entry reads "Keep §9's Lint / typecheck row consistent with it"; the row's "CI annotations" consumer is the
non-blocking mechanism §10's retired line named → APPLY, text re-derived from the report alone.

## test-plan

T1 — detector D-tests-obs-harness · warning · §3 → 5-command implementation (`run` Command body) · change: add the
both-shells first-red-line stop rule after "…CI reaches them through the `rust` job's dogfood step" · basis
`scripts/agent-run.ps1:373-384`, `scripts/agent-run.sh:20`, carry-measurement §1, §3.
**Disposition: REJECT as proposed** — the re-derivation tell (`agent-run.ps1:373-384` is not in the report), and its
change line over-claims "with that line's exit … whoever calls it" (false under a caller setting the preference — the
layout-templates refinement above). **Its fact RAISED by the orchestrator (check 5)**: the plan's expected entry names
the change itself ("the bundled default stops at the first failing line in BOTH shells … so the ps1 no longer depends on
its caller's `$PSNativeCommandUseErrorActionPreference`") → APPLY, text re-derived with the caller distinction.

T2 — detector D-tests-obs-harness · dependent-of D-tests-obs-harness · §3 → 5-command implementation (`run` Exit code
semantics, key file `:12`) · change: non-zero also covers a red build / doctest / `-D warnings` lint line in the bundled
default (that line's cargo exit) · basis carry-measurement §1, key file `:12`.
**Disposition: falls with its primary (atomic group); RAISED by the orchestrator** — a same-claim duplicate the cascade
would otherwise leave: the key file says non-zero means a hard `Fail` only, and the report's quoted measurement
(`agent-run.sh run` exit 101 at the workspace clippy line with `1204 tests run: 1204 passed`) falsifies it for the
bundled default. Playbook `Accurate this-chunk addition` (this chunk measured and changed the bundled default's exit) →
APPLY, text re-derived.

T3 — detector D-tests-obs-harness · dependent-of D-tests-obs-harness · §1 → 5-command requirements (`run`, `test-plan.md:64`)
· change: non-zero also from a red build / doctest / clippy line in the bundled default.
**Disposition: falls with its primary; RAISED by the orchestrator** — same claim restated in §1 (`test-plan.md:64`:
"non-zero = at least one hard `Fail`"), same basis → APPLY, text re-derived.

## Validate checks

1. Playbook — dispositions above; no rule collision; no boundary widening (a harness script's exit becoming stricter
   admits nothing new).
2. Cross-contradiction — none: O1/O2 (obs) and T1–T3 (test-plan) state one fact from two sides; test-plan §3 ↔ obs-plan
   §3 bind unaffected (no envelope / status / log-format change).
3. Intent-consistency — the report's eight deviations are each justified against the plan; scope record none
   (`gate.py scope` clean, 0 recorded); the ps1 edit is the plan's step 9.
4. Absence needs evidence — the "no hit" verdicts each name their search; the report's own site sweep (Expected
   amendments bullet) names patterns, hit counts and per-hit dispositions; the cascade sweep re-checks after Apply.
5. Expected amendments — both entries covered: obs §10 (+ §9 row) by O1 + the O2 raise; test-plan §3 key by the T1
   raise (+ T2/T3).
6. Disproved claims — obs-plan:490 → O1 (APPLY). The plan's ≈490 tokei forecast → no master states it (a plan-internal
   forecast; plan.md is immutable) → disposed to the report's Deviations (5) and curation (sweep hazard).

Escalations: 0.
