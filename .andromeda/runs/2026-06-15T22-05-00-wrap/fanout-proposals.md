# P2 fan-out — drift-detector proposals + resolution (2026-06-15-design-token-typography-bundle)

_Consolidated record of the 7 Explore doc-agents (one per spec source). Raw verbose returns stripped to verdicts + proposals (the canonical drift record), consistent with the phase-fanout consolidation. Escalations resolved WITH the user via AskUserQuestion ×3._

## Detector verdicts
| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | **3 proposals** (warning) — §Stack row + §Occupied Resources + §Inherited Defaults: record the frontend npm ecosystem |
| security-plan | D-security-input, D-security-subprocess, D-security-deps | **1 proposal** (escalate, D-security-deps) — §Dependency Security: npm-audit gate; input+subprocess clean |
| design-system | D-design-tokens | **1 proposal** (warning) — §Tokens: `@theme`→`:root` (Tailwind v4 tree-shaking; report Deviation #2) |
| layout-templates | D-layout-surface | clean — token bundle is not a product surface (Epoch 9) |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | **1 proposal** (warning, D-tests-coverage) — §4: frontend tests build-gated, deferred Epoch 9; framework+harness clean |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-redaction | clean — static frontend, no telemetry/no OTLP/no leak (escalate-class D-obs-stack + D-obs-redaction did NOT fire) |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | clean — no interactive product element, no schema change |

## Validation
- **No cross-contradictions** (6 proposals, 6 distinct sections).
- **Intent-consistency:** the 2 deviations (Vite 6→8 audit bump; `@theme`→`:root`) are *justified* divergences ⇒ intent was incomplete ⇒ amend specs to truth.
- **Playbook:** D-tests-coverage matches rule #3 (Foundation defers a downstream-sequenced concern) → routine. The other 5 had no rule match + were first-of-kind/structural → escalate.

## Resolution (user, AskUserQuestion ×3)
1. **Frontend → specs:** ALL 4 — security §Dep-Security + arch §Stack + §Occupied Resources + §Inherited Defaults.
2. **design §Tokens:** replace `@theme` illustration with `:root` + reason.
3. **Playbook:** add the generalized reconciliation rule (spec-illustration → sound-impl when report proves invariant = routine).

## Applied (drift = 0)
- **Spec bodies (6 edits / 4 docs):** architecture.md ×3 · security-plan.md ×1 · design-system.md ×2 (prose + `@theme`→`:root`) · test-plan.md ×1.
- **Sidecars:** architecture-amendments.md · security-plan-amendments.md · test-plan-amendments.md (appended) · design-system-amendments.md (created).
- **Playbook:** rule #5 (generalized reconciliation) appended.
- **Cascade:** stack.md ×2 (←arch) · security.md rule + security-summary.md (←security-plan) · frontend.md rule (←design-system). design-summary/tests-summary/testing.md = no-op (no affected claim).
