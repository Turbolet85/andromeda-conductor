# Fan-out results — 2026-08-14-canary-fingerprint-feed-capture

7 doc-agents, 18 detectors, one parallel batch. All returned clean YAML; no return needed stripping, so no
raw twins were warranted.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | `proposals: []` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · **D-tests-obs-harness** · D-tests-derived-count | **1 proposal** |
| obs-plan | **D-obs-instrumentation** · D-obs-stack · D-obs-redaction | **1 proposal** |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` |

## Validation (5 checks)

1. **Playbook** — both proposals matched the 2026-06-15 spec-illustration→sound-impl rule (body → current
   truth; invariant preserved, only the form differs). Routine.
2. **Cross-contradiction** — none. The two proposals edit different docs in complementary directions
   (obs §6 records new instrumentation; test-plan §3 widens a level enumeration to admit it).
3. **Intent-consistency** — the report matches the chunk's working-route entry and plan acceptance criteria;
   all four deviations carry justifications. No divergence.
4. **Absence needs evidence** — every negative return cited its basis: design/layouts/tests each grepped
   their doc for the moved count (577/581, zero hits); arch/security/a11y cited the report's own Changes
   bullets. Accepted.
5. **Expected-amendments reconciliation** — the plan listed three:
   - obs-plan §6 → proposed by D-obs-instrumentation ✓
   - architecture.md §Occupied Resources (conditional on the capture's outcome) → **no detector proposed it;
     raised by the orchestrator** and applied routine, since the report substantiates it comprehensively.
     The blind class is the documented one: an *existing* recorded finding gaining an extension, which a
     "is every new resource registered?" invariant cannot see.
   - test-plan §2/§7 → correctly no amendment (the ephemeral-stub gRPC mechanism was already registered).

## Cascade

The citation grep over all seven masters found no stale quotation of either amended passage. It DID hit a
preserve-verbatim-adjacent leaf: `.claude/rules/observability.md` taught `RUST_LOG=conductor_timeline=debug`
— the bare per-target form this chunk measured as harmful. Tracing it to source found the same form in
obs-plan §3's per-module level table and §11's hot-path rule, where the table was internally inconsistent
under literal use (following it would silence the very crates it sets to `info`).

That became a **fourth amendment** (obs-plan §3 + §11 → the additive `RUST_LOG=info,{crate}=debug` form),
with `observability.md` re-derived from it. Remaining leaves checked and genuinely no-op: `obs-summary.md`,
`tests-summary.md`, `testing.md` carry neither amended wording; CLAUDE.md's generated blocks and
`stack.md` summarize at a grain the arch amendment does not disturb (verified by grep, not assumed).

**Result: 4 amendments applied across 3 masters (obs-plan ×2 sections, test-plan, architecture) · 0
escalations · 0 open.**
