# Fan-out results — 2026-09-01-webview-self-verify-windows-host

7 Explore doc-agents, one per spec source, all returned. **35 proposals.**

| doc | proposals | verdict |
|---|---|---|
| architecture | 8 (5 `D-arch-resources` incl. 2 dependents · 3 `D-arch-decisions` incl. 1 dependent) | routine, except the scope-law qualification → escalated + resolved |
| security-plan | 8 (7 `D-security-input` incl. 5 dependents · 1 `D-security-subprocess`) | escalate-severity → escalated + resolved |
| design-system | 0 — `proposals: []` | clean |
| layout-templates | 1 (`D-layout-derived-count`) | routine · orchestrator extended it for the `.ps1` omission |
| test-plan | 15 (11 `D-tests-framework` incl. 10 dependents · 4 `D-tests-coverage` incl. 1 dependent) | routine |
| obs-plan | 3 (`D-obs-instrumentation`, 1 primary + 2 dependents) | routine |
| a11y-plan | 0 — `proposals: []` **with an orchestrator note** | clean-but-blind (see below) |

## Clean returns, in their own words

- **design-system** — no new UI element; `tokens design-token✓` on the one new surface; the `--e2e`
  qualifier appears in no palette row, ANSI-map entry, token label or reuse tally. Adjacent literals
  (`11 of 34` token tally, the six-lamp count, `comfy-table 7` / `indicatif 0.18`) verified unmoved.
- **a11y-plan** — neither detector triggers: the chunk adds no interactive UI element (the 3 failing specs
  assert PRE-EXISTING §1 entities), and no violation-schema or obs-schema change occurred.

## Detector blind spot (the reason the plan's floor matters)

The a11y agent reported, unprompted, that its two detectors **structurally cannot** see the platform
framing this chunk disproved: `D-a11y-surface` covers new-UI-element coverage and `D-a11y-obs-schema`
covers violation-JSON-vs-obs-§6. The Linux-only verdict at a11y-plan §3/§9/§11/§12 and the
`browser.emulate` BiDi caveat at `:419` map onto neither, so **a11y returned clean while carrying six stale
sites**. Caught by Validate check 5 (the plan's `Expected amendments` list is the chunk's coverage floor).

The same class made test-plan's own detector under-run its site list: the plan named 6 sites, the doc
carries **8** (`:287` and `:369` additionally defer GUI legs to "Linux+xvfb"). The test-plan agent found
those two itself; a11y's could not.

→ **Resolved with the operator: a new `D-platform-claim` detector was approved and appended to
`drift-base.md`.**

## Escalations (4 raised, 4 resolved with the operator)

1. **Security ban qualifications** → *qualify both, scoped to shipped binaries.* Rule @58 did NOT dismiss
   these: its precondition is that the chunk adds no new boundary or spawn, and this chunk adds both
   (`CONDUCTOR_MSEDGEDRIVER`, the driver spawn). Rule @94 did not apply (not a command name).
2. **Arch scope law "no UI automation"** → *scope the non-goal to the SUT.* Conductor does not automate
   PULSE's UI; driving its OWN webview is in-scope and dev-only.
3. **a11y-plan `:419` emulate caveat** → *record measured-vs-unverified separately.* BiDi script evaluation
   measured dead against wry/WebView2; `browser.emulate` itself untested → `recorded, not established`.
4. **New detector** → *approved*, appended as `D-platform-claim`.

## Orchestrator-raised (Validate checks 5 + 6 — not proposed by any detector)

- `a11y-plan` §3 (Configuration · CI integration · Bootstrap `a11y-ci-gate-wire`) · §9 · §11 · §12 — the
  Linux+xvfb platform verdict (plan-listed; detector-blind).
- `a11y-plan:419` — the emulate caveat (escalation 3).
- `architecture.md` §Established Decisions [Read-Back Dependency Posture] — the CARRY tuple re-scope
  (plan-listed; not a change the report's Changes bullets carry, so no detector could see it).
- `layout-templates.md` §Surface: cli — the `.ps1` omission (plan-listed; the layouts agent covered the
  `--e2e` mapping but greps confirmed `:183` names only `agent-run.sh`).
- `.claude/rules/verification-harness.md` — the false "only a release build reads the bundle" mechanism.
  **Routed to P3 curation, not the cascade**: it lives in `## Session Additions`, a preserve-verbatim
  curation home the cascade must never edit (amendment-flow §Cascade). Its feeding master
  (`architecture.md` §Infrastructure Patterns — Build system) is amended instead, per directive item 3.

## Raw twins

Every return arrived parseable and needed no stripping beyond fence removal; the obs and test-plan returns
carried HTML-escaped `&lt;`/`&gt;` inside YAML values (decoded on apply, `entities=0` verified). No return
failed validation, so this consolidated file is the sanctioned audit artifact for all seven.
