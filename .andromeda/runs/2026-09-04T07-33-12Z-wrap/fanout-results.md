# Fan-out results — 2026-09-04-sr-findings-remediation

7 Explore doc-agents, one per spec source, one parallel batch. **15 proposals** returned.

Transport note: five returns HTML-escaped YAML metacharacters (`&gt;-` folded-scalar markers in
security-plan; `&lt;table&gt;` in layout-templates and a11y-plan). Decoded before parsing; the
`entities=0` probe passes on the decoded text. The raw returns differ from what is recorded here ONLY
by that escaping, so this consolidated file is the audit artifact for all seven (no separate
`.raw-fanout-{doc}.md` carries anything the inverse-encode of this file would not reproduce).

## Verdicts

| doc | proposals | claims |
|---|---|---|
| arch | 3 | `declares()` defect (1 primary + 2 dependents) |
| security-plan | 2 | `declares()` defect (1 primary + 1 dependent) |
| design-system | **0** | clean on all three detectors; flagged the empty-state entry as outside its invariants |
| layout-templates | 3 | empty-state split (1) · `declares()` defect (1 primary + 1 dependent) |
| test-plan | 4 | `declares()` defect (1 primary + 2 dependents) · fifth `sr` handle (1) |
| obs-plan | **0** | clean on all four detectors |
| a11y-plan | 3 | empty-state split (1 primary + 2 dependents) |

## Claim A — the `declares()` probe defect (10 proposals, 5 docs)

Every proposal records one measured fact: `observe_preconditions` (`crates/conductor-run/src/lib.rs:380`)
passes all three `ANDROMEDA_PULSE_*` names through `declares()` (`:359`), which accepts only a value of
`"true"`/`"1"`. `ANDROMEDA_PULSE_DATA_DIR` is a PATH, so it can never declare; `conductor preconditions`
cannot exit 0 under any environment, and `agent-run boot`'s leading arm has short-circuited before every
preflight since `480bc66`.

Sites:
1. `architecture.md` §Standard Contracts — Liveness equivalent **(primary)**
2. `architecture.md` §Occupied Resources — `CONDUCTOR_PREFLIGHT_TIMEOUT` *(dependent)*
3. `architecture.md` §Occupied Resources — `ANDROMEDA_PULSE_MCP_ENABLED` *(dependent)*
4. `security-plan.md` §Security Anti-Patterns → Universal **(primary)** — withdraws "not a downgrade"
5. `security-plan.md` §Input Validation — the `ANDROMEDA_PULSE_*` read-set row *(dependent)*
6. `layout-templates.md` §cli Primary screens → `conductor preconditions` **(primary)**
7. `layout-templates.md` §cli Primary screens → `conductor preflight` *(dependent)*
8. `test-plan.md` §3 Test Harness Contract → `boot` **(primary)**
9. `test-plan.md` §1 Test harness requirements → `boot` *(dependent)*
10. `test-plan.md` §6 E2E critical-path scenarios — five step-1 lines *(dependent)*

**Playbook: rule at `playbook.md:118` GOVERNS → routine.** Every qualifying clause holds: an
escalate-severity boundary detector (D-security-subprocess), a MEASURED product defect with zero delta at
the defect's site, already dispositioned by the operator's wrap directive as a route candidate, and the
directive NAMES THE DEFECT ITSELF — site (`lib.rs:359`), mechanism (truthy-only gate on a path handle),
symptom (exit 1 under a correct env; `boot` short-circuiting since `480bc66`) — not a class. Applied as a
RECORD: each body states the measurement with its evidence pointer and names the fix as route-owned
(directive item 1 mints the corrective entry).

## Claim B — the empty-state string split (4 proposals + 1 orchestrator-raised)

`No scenarios match.` (`ScenarioPicker.tsx:29`, row S0-16) is the picker's FILTER-MISS prose, now announced
from a persistently-mounted `aria-live="polite"` region; `No scenarios found.` (`App.tsx:258`, row E0-01) is
the empty-CATALOG prose. Two states, two strings, two rows.

Sites:
1. `a11y-plan.md` §3 must-announce inventory **(primary)**
2. `a11y-plan.md` §11 Visual real-prose ban (`:540`) *(dependent)*
3. `a11y-plan.md` §1 harness-spec must-announce list (`:113`) *(dependent — found by the agent's own sweep;
   NOT named in the plan's Expected-amendments list)*
4. `layout-templates.md` §Primary content block 1 States (`:138`)
5. `design-system.md` §Component Patterns **5** (picker) — **orchestrator-raised** under Validate check 5
   (the plan's list named design-system; no detector proposed it). Pattern 5 carries no empty/filter-miss
   state at all, so the addition lands there.

**Playbook: rule at `playbook.md:28` GOVERNS → routine** (a spec's wording reconciled to the shipped
implementation; the invariant — empty states are real announced prose, never a placeholder — is intact and
only the enumeration was incomplete).

## Claim C — the fifth `sr` firing-form handle (1 proposal)

`test-plan.md` §6 desktop-webview drivers row: only `sr-empty`/`sr-error` carry a `scenarios` field at the
one spawn site (`wdio.conf.ts:126`); the live `sr` subject takes
`CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios` from the invoking shell. **Routine** under `playbook.md:28`
and explicitly requested by operator directive item 4.

## Validation checks

1. **Playbook** — every proposal matched a governing rule (`:118` for claim A, `:28` for B and C). No
   two-rule collision. No no-match/uneasy proposal.
2. **Cross-contradiction** — none. Within each claim every proposal states the same fact at a different
   site; no two proposals edit one section in opposing directions.
3. **Intent-consistency** — claims A and C are discoveries, not divergences: the chunk's intent (fix eight
   findings, re-run the leg) did not anticipate them, and the operator's directive items 1 and 4 ratify
   both. Justified divergence ⇒ intent was incomplete.
4. **Absence needs evidence** — each proposal cites its site; the two sweep-found sites
   (`a11y-plan.md:113`, the five `test-plan` §6 step-1 lines) name the lines they were found at. The
   sidecar entries name the search performed.
5. **Expected-amendments reconciliation** — the plan listed four entries:
   - `a11y-plan.md` §3 — proposed ✓ (plus §11 and §1)
   - `design-system.md` §Component Patterns 3 Empty — not proposed → **orchestrator-raised**, but
     re-aimed at Pattern 5 (see the ESCALATION below for the Pattern 3 residue)
   - `layout-templates.md` §Primary content block 1 States — proposed ✓
   - `test-plan.md` §6 (terminal stage) — **no amendment owed**: grep confirms test-plan states no
     last-scenario-Stop terminal stage, so there is nothing to retire. Dispositioned, not silent.
6. **Disproved-claims disposition** — the report's three entries:
   - #1 `conductor preconditions` unsatisfiable → DISPOSED by claim A (10 sites) + directive item 1
   - #2 the plan's findings-3/8 deferral premise → DISPOSED by record: the plan is a chunk artifact, not
     amendable by P2; the withdrawal is stated in the report and the leg verdict
   - #3 finding 1 confirmed unfixed → DISPOSED by route: operator directive item 2 pins it as a CARRY

## ESCALATION (1) — raised to the operator before apply

`design-system.md:257` §Component Patterns **3** (Coverage matrix) states its **Empty** as the shipped
`No scenarios found.` prose. Measured during validation: the coverage section's shipped empty is
`No coverage data.` (`App.tsx:298`); `No scenarios found.` is the *scenario-section* empty (`:258`). The
attribution is wrong at the component level — but this chunk did not cause it (it landed at the 2026-09-02
wrap) and the report does not carry `No coverage data.`, so correcting it asserts a fact outside the
report's Changes. Raised rather than applied silently.
