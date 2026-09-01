# Fan-out results — 2026-08-31-p-075-assert-round

7 doc-agents, one batch. 17 proposals; 2 escalations resolved with the operator; 0 open on exit.

| doc | proposals | severity | verdict |
|---|---|---|---|
| arch | 4 (1 primary + 3 duplicate-occurrence) | warning | applied |
| security-plan | 4 (1 primary + 3 duplicate-occurrence) | escalate | ESCALATED -> operator ratified -> applied |
| obs-plan | 3 (1 primary + 2 duplicate-occurrence) | warning | applied (+1 orchestrator-raised rmcp correction, operator-ratified) |
| test-plan | 6 (3 primary + 3 duplicate-occurrence) | warning | applied |
| design-system | 0 | - | clean |
| layout-templates | 0 | - | clean |
| a11y-plan | 0 | - | clean |

## Validate (the 6 checks)
1. Playbook — arch/obs/test proposals matched the reconcile-wording-to-shipped-implementation pattern; security's 4 matched NO pattern at escalate severity -> escalated.
2. Cross-contradiction — none. Two arch proposals touch the same section but different sentences (payload carrier vs workspace-key mechanism).
3. Intent-consistency — the report's deviations are justified by measurement; the two-incident control premise was disproved live and is recorded in the report + the harvest module doc. No cap claimed, so no acceptance text to refine.
4. Absence-needs-evidence — the three clean returns each cite the report's explicit 'none - verified' bullets; the security/obs cited sites were re-derived independently by the orchestrator before applying (own sweep found the same set, no more).
5. Expected-amendments reconciliation — all 4 plan entries matched by a proposal (arch x2, test-plan, obs-plan). The 5th item (incident_events residual) is route-resolve's channel, handled at P5.
6. Disproved-claims disposition — #1 test-plan S5 -> tests proposal; #2 two-incident premise -> report + harvest doc + arch dedupe clause; #3 stub item-key -> tests proposal; #4 mcp-contract.toml rmcp -> routed to its owner v2-28, and the FIFTH site found in obs-plan:35 was corrected here under operator ruling.

## Cascade (derived tier)
- .claude/rules/security.md:10 — corpus read-back-ONLY -> MCP tool surface (read + write), file ban unchanged. UPDATED
- .claude/docs/gotchas.md:23 — `workspace` column '= data_dir' qualified to the published-key mechanism. UPDATED
- .claude/docs/tests-summary.md:46 — 'rmcp stub' -> hand-rolled stub + item-key note + feature-gated live leg. UPDATED
- .claude/docs/security-summary.md:14 — named the write direction; corrected 'bounded prost decode' -> JSON-RPC/serde_json (same authored-line reasoning the operator ruled on for rmcp). UPDATED
- .claude/rules/testing.md:62 — incidental pattern match, no read-only claim. NO CHANGE
- .claude/docs/session-learnings.md:148-149 — factual corpus/table record, states no read-only claim. NO CHANGE
- .claude/docs/conventions.md:28, gotchas.md:12/17, stack.md:15, rules/verification-harness.md:18, rules/testing.md:35 — already correct (rmcp removal recorded). NO CHANGE
- .claude/rules/verification-harness.md:40-44 — dated Session Additions, historical record preserved verbatim. NO CHANGE
- CLAUDE.md:37 — preflight-integrity line unaffected; arch flows in by @import. NO CHANGE
- .claude/docs/obs-summary.md — no matching claim. NO CHANGE
