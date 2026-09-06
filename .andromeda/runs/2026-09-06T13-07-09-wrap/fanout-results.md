# Fan-out results — 2026-09-06-run-report-envelope-conformance-gate

7 doc-agents, one batch. **16 proposals** across 5 docs; 2 docs clean.

| doc | proposals | verdict |
|---|---|---|
| arch | 1 | D-arch-decisions — widen §Established Decisions [ORM] None — raw SQL to include the transactional teardown. D-arch-resources correctly declined to register the verb (architecture:33 refuses a verb list); D-platform-claim no hit. |
| security-plan | 6 | D-security-input ×5 (3 sites: the `CONDUCTOR_RUNS_DIR` test-reader taxonomy; 2 sites: the CLI `run_id` argv surface) + D-security-subprocess ×1 (harness-spawn rule (b) gains a fifth form). ALL escalate-severity. |
| design-system | 0 | clean — all three detectors evaluated; tokens ✓ reuse, no moved count has a design-system site, no platform verdict retired. |
| layout-templates | 1 | D-layout-surface — §Surface: cli Primary screens under-enumerates the verb set by one. D-layout-derived-count deliberately folded into it (the doc declares §Primary screens the enumerating home at `:252`, not a sample). |
| test-plan | 3 | D-tests-obs-harness ×3 — §3 `cleanup` body (E2), §3 `status` body (E4), + a dependent duplicate at `:65` that the report's own E4 grep could not see (it carries no `**status**` bolding). |
| obs-plan | 5 | D-obs-instrumentation ×5 in two claim-families: the cli `#[tracing::instrument]` row, and the rusqlite WRITE-span rule. D-obs-stack / D-obs-redaction / D-platform-claim no hit. |
| a11y-plan | 0 | clean — no interactive surface added, no schema change; a11y-plan already carries the ELEVEN-key envelope (`:96-109`, `:233-246`), corroborating E1's obs-plan-only scope. |

## Validation outcome

**ROUTINE → apply (5 proposals)**
- arch [ORM] widening — playbook `:28` (spec illustration → sound shipped impl, invariant holds: raw SQL / no ORM stands).
- layouts §cli Primary screens verb bullet — playbook `:61` (the verb surface is layout-templates' concern, `preflight`/`coverage` precedent) + `:127` (SET-NAMING; `:252` already names the source of truth).
- test-plan §3 `cleanup` body, §3 `status` body, §1 `status` duplicate — playbook `:28`; E2 and E4 respectively, the third a `dependent-of` that applies with its primary.

**ESCALATED (11 proposals, 5 decision units)** — see the escalation record below.

## Re-derivations performed by the orchestrator (not inherited)

- **The FORMATTING-ONLY partition**: 22 changed `.rs` files → 15 byte-identical to `HEAD | rustfmt --config-path rustfmt.toml`, 7 semantic; `main.rs` differs by exactly 1 line. Matches the wrap directive's measurement.
- **E3 re-aim**: architecture `:33` refuses a verb list and names layout-templates §cli Primary screens as the enumerating home; layout-templates `:252` states the same in its own voice. The plan's target was wrong; three detectors independently agreed.
- **Playbook `:109` qualifier check**: the rule dismisses the cli `#[tracing::instrument]` misattribution *at line 48*. Verified — the `:48` table's cli row names only "`tracing` 0.1.x crate + `tracing-subscriber` JSON formatter", so the rule is correct about it. But a SECOND "Auto-instrumentation per surface" table at `:280-288` has a cli row that DOES read "`#[tracing::instrument]` on `fn main()` and core scenario handlers". The rule's qualifier (line 48) therefore fails for this proposal, and its note's claim "the mandate is NOT on the cli row" is incomplete. Escalated with a proposed rule refinement.
