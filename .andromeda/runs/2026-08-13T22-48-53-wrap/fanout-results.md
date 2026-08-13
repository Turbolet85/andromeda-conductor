# Fan-out results — 2026-08-13-first-live-green-preflight

7 doc-agents, one per spec source. Prompt sent verbatim per amendment-flow.md.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 1 — D-arch-resources → §Occupied Resources (warm-up claim measured false) |
| security-plan | drift | 2 — D-security-input → §Input Validation (shell-side contract readers); D-security-subprocess → §Threat Model auth model (MCP_ENABLED locus) |
| design-system | clean | `proposals: []` — cited its grep (no old 30s literal in any palette row / ANSI map / token label / reuse tally) |
| layout-templates | clean | `proposals: []` — cited its grep (no PREFLIGHT_TIMEOUT / timeout / budget / warmup / poll / 30 hit in any caption, wireframe, sample or selector label) |
| test-plan | drift | 1 — D-tests-derived-count → §3 `boot` Timeout (two budgets, derived not literal) |
| obs-plan | clean | `proposals: []` (bare, no evidence — main verified independently: the key-set line rides EXISTING spans on the allowlisted message field, so no §4/§6 invariant is violated) |
| a11y-plan | clean | `proposals: []` — no UI element added or changed this chunk |

## Validation (main)

- **Playbook** — arch: operator directive 2 pre-authorized the amendment's content (the directive is the resolution, not a fresh escalation). security x2: declared `escalate`, downgraded by the agent with accepted reasoning — the 2026-08-10 rule ("a runtime-parsed `contracts/` artifact earns an §Input Validation row") applies because this chunk ADDED a reader boundary, so the 2026-06-23 dismiss rule's "added NO new external-input boundary" precondition is unmet; the subprocess proposal is the 2026-06-15 wording→sound-impl class with every hardening invariant verified live (protocol 2024-11-05 negotiated, `.env(...)`-only data dir, data_dir redacted). test-plan: same wording→sound-impl class, and the plan predicted it.
- **Cross-contradiction** — none. Four distinct sections across three docs.
- **Intent-consistency** — the live outcome diverged from the plan's prediction (F10 unconfirmed) but is justified in the report as a faithful measurement; `scope.md` was amended at phase P5 and the working-route entry is frozen. No intent amendment owed.
- **Absence needs evidence** — design + layouts cited their greps. obs + a11y returned bare `[]`; a11y trivially correct (no UI), obs verified independently by main.
- **Expected-amendments floor** — plan listed two. `test-plan §3` proposed by the detector; **`obs-plan §6` was NOT** — raised by main per the contract (routine: the report substantiates a new must-log line at the `verify.readback` boundary §6 enumerates).

**Result: 5 amendments across 4 docs · 0 escalations open.**
