# Fan-out results — 2026-08-11-faithful-emission-dispatcher

7 Explore doc-agents, one per spec source. 4 proposals from 4 docs; 3 docs returned clean.

| doc | verdict | detectors | outcome |
|---|---|---|---|
| arch | 2 proposals | D-arch-resources · D-arch-decisions | both APPLIED (routine) |
| security-plan | 1 proposal | D-security-input (escalate) | ESCALATED → resolved → APPLIED (both halves) |
| design-system | clean | D-design-tokens · D-design-derived-count | no drift |
| layout-templates | clean | D-layout-surface · D-layout-derived-count | no drift |
| test-plan | 1 proposal | D-tests-obs-harness | superseded — see note |
| obs-plan | 1 proposal | D-obs-stack (escalate) | DISMISSED (playbook 2026-06-21 test-only-use) |
| a11y-plan | clean | D-a11y-surface · D-a11y-obs-schema | no drift |

## Applied (4 docs, 8 body sites + 4 sidecars)

1. **D-arch-resources** → arch §Occupied Resources (`contracts/pulse-load-envelope.toml`): the rate terms move from *declared-not-derivable* to **derivable but not yet asserted**; `EmissionSpec::occurrences` is the field whose absence was the old justification. Matches the plan's Expected amendment #1.
2. **D-arch-decisions** → arch §Established Decisions [Validation Library] + §Conventions Config conventions: the shipped integer-percent error-fraction encoding + the `dive`-never-`skip` mandate + the declared emission-shape model.
3. **D-security-input** (escalated, operator: apply both halves) → security-plan §Input Validation row + Vector 2 trust boundary + §Anti-Patterns: the `[phases.emission]` surface (required `kind`, bounded `occurrences`, per-shape cross-field rules) and the `dive`-never-`skip` mandate.
4. **Cross-master citation fold** (operator-approved scope): `error fraction ∈ [0,1]` appeared verbatim in 5 masters. All 8 sites now name the invariant AND its shipped encoding — architecture ×2, security-plan ×3, obs-plan ×1, test-plan ×2. Zero residual hits verified by grep.

## Cascade (leaves re-derived)
- `.claude/rules/security.md` — validation bullet re-derived + the `dive`/`skip` rule added.
- `.claude/docs/conventions.md` — config-conventions bullet re-derived + the emission-shape model.
- `.claude/docs/security-summary.md` — trust-boundary line re-derived with the `dive` mandate.
- CLAUDE.md + `docs/stack.md` — checked, no stale content (neither distilled the amended literals). Unchanged.
- `.claude/rules/testing.md:64` — carries the superseded "no rate or occurrence-count field" claim INSIDE `## Session Additions`, which is preserved verbatim by contract. Routed to P3 curation as a follow-up entry, never a cascade rewrite.

## Dismissed / declined (with reasoning)

- **D-obs-stack** (escalate severity) proposed recording `conductor-run`'s dev-only `opentelemetry-proto`/`tonic` edge in obs §3 and widening the `default-features = false` follow-up to both dep sites. **Dismissed** under playbook 2026-06-21 (test-only use of a library the spec narrows to production scope, never in a release build, production posture unchanged). The true half — the trim now spans two dep sites — is carried to the route entry that already owns it, not an obs-plan edit.
- **D-tests-obs-harness** proposed correcting test-plan §3's claim that a no-flag `run` executes the full aggregate including scenarios, since the scenario leg fires only under `SCENARIO=`. The proposal is factually right and the report substantiates it, but the same fact is routed to P3 curation per operator directive 4 (the pipeline-side rule is the overseer's to encode). **Superseded, not dismissed** — recorded here so it is not lost.
- **Plan Expected amendment #2** (obs-plan §4 CP1 declared-vs-observed `emission_count`): raised by the orchestrator per the expected-amendments floor since no detector proposed it. **Declined** by the operator — emit.batch now genuinely nests beneath timeline.execute, so a parent span whose attributes compute at open cannot carry an observed count; the hierarchy makes it self-evident.
- **Operator directive 3 had nothing to reject**: the obs detector returned D-obs-instrumentation clean and confirmed §4 CP1 already requires `emission_count`. No CP1 amendment was ever proposed.

**Drift = 0 on exit**: every proposal is applied, dismissed with a cited playbook rule, or resolved with the operator. Zero open.
