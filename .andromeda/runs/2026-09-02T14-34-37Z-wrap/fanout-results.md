# Fan-out results — 2026-09-02-cross-surface-envelope-parity

7 doc-agents, one per spec source. `D-platform-claim` is scoped to all seven docs and was sent to each.

| doc | proposals | verdict |
|---|---|---|
| design-system | 0 | clean — banner is a rendered state of the SHIPPED component (tokens ✓, no new UI element); no moved qualifier restated here; no platform/driver verdict in the doc |
| layout-templates | 0 | clean — banner already maps to wireframe + component entries (:26, :85-87, :144, :187, :197-206, :280); samples carry elided placeholders and name the SET, not literals |
| architecture | 1 | D-arch-resources — register `CONDUCTOR_E2E_SEED_DIR` (11th `CONDUCTOR_*` handle) |
| obs-plan | 2 | D-obs-instrumentation — §4 span-kinds line + auto-instrumentation table (primary + 1 dependent) |
| a11y-plan | 3 | D-platform-claim — retire the banner's UNRUNNABLE verdict at §4:333, §6:404, §9:465 (primary + 2 dependents) |
| security-plan | 7 | D-security-input ×6 (handle registration + enumeration widenings + spawn forms) · D-security-subprocess ×1 (protocol-version ban mechanism) |
| test-plan | 29 | D-tests-framework ×23 (the rmcp payload) · D-tests-derived-count ×1 · D-platform-claim ×1 · D-tests-obs-harness ×3 · D-tests-coverage ×1 |
| **total** | **42** | 34 routine · 8 escalated-and-resolved |

## Validation (the 6 named checks)

1. **Playbook** — 34 routine. The rmcp/mechanism reconciliations match the 2026-06-15 spec-wording→sound-impl
   rule; the a11y/test-plan platform retirements and the harness/provenance restatements are substantiated
   directly by the report. 8 escalated (below).
2. **Cross-contradiction** — none. a11y §4/§6/§9 and test-plan §6:305 retire the same banner verdict in their
   own docs; consistent, not opposing.
3. **Intent-consistency** — ONE divergence: the chunk introduced `CONDUCTOR_E2E_SEED_DIR` against its own plan
   acceptance criterion ("no new `CONDUCTOR_*` env var"). Escalated; operator ratified the divergence as
   justified — the criterion's subject (the parity proof) holds unaltered, and the handle belongs to the
   seeder, which the criterion did not contemplate.
4. **Absence needs evidence** — the rmcp site count was re-derived independently (`grep -c -i rmcp
   .andromeda/test-plan.md` = 24) rather than taken from any enumeration. Two enumerations had already proved
   incomplete this session (the originating CARRY named 1 site; the phase distiller claimed 26).
5. **Expected-amendments reconciliation** — the plan's floor was the 24-site test-plan payload + obs-plan §4
   ×2. Both covered and EXCEEDED: see the two findings below.
6. **Disproved-claims disposition** — all five report entries disposed: test-plan rmcp → 23 proposals ·
   obs-plan §4 → 2 · a11y §4/§6 → 3 · test-plan §5 deferral → 1 · arch env registration → 1.

## Two sites the report's own enumeration missed

Both found by the doc-agents' duplicate-occurrence grep, and both verified independently before applying:

- **`a11y-plan.md:465`** (§9 E2E row) — carries the routine-arm carve-out "only a spec whose DOM state that
  fixture cannot produce — today the load-envelope banner — context-skips, never passing green". The report
  named §4 and §6 only.
- **`test-plan.md:278`** — carries the retired "negotiates DOWN to `2024-11-05`" mechanism and **contains no
  `rmcp` token**, so the report's rmcp-keyed classification structurally could not reach it. The stale
  MECHANISM outlives the stale LIBRARY NAME; a token-keyed sweep is not a semantic one.

A follow-up semantic sweep for the mechanism wording across all seven masters and the derived tier found:
`security-plan.md:357` (proposed), `test-plan.md:91` + `:278` (proposed), and two cascade leaves —
`.claude/docs/gotchas.md:10`, `.claude/rules/security.md:25` — both re-derived in cascade step 3.

## Escalations (8 proposals, 2 questions, both resolved with the operator)

- **`CONDUCTOR_E2E_SEED_DIR` handle class** (arch ×1, security ×6, test-plan ×1). Playbook rule 115 covers a
  dev-harness `CONDUCTOR_*` handle but fails 4 of 5 preconditions here: host-tool path (no — repo-relative
  artifact dir) · read solely by `wdio.conf.ts` (no — read by a Rust test, wdio only sets it) · behind the
  wdio-edge `isFile`/metacharacter guard (no — harness-owned constant, not operator input) · named on the
  plan's expected-amendments list (no — the plan forbade the handle). The security agent independently
  classified it "the THIRD handle class". **Resolved: register as a third class, apply all 8, and mint a
  playbook rule for the class** so the next one is routine.
- **Epoch 6a ordering** (route-resolve, P5). The directive's enumeration differs from today's file order while
  its 6b enumeration matches exactly. **Resolved: the directive's enumeration governs** — A11y CI gate ·
  SR findings remediation · Sidecar spawn without a console window.
