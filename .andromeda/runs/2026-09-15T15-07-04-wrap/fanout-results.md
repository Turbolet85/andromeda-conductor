# Fan-out results — 2026-09-15-remaining-structurally-dead-declarations-retired

7 Explore doc-agents, one per spec source, one parallel batch. **1 proposal · 6 clean.**

| doc | verdict | detectors evaluated |
|---|---|---|
| arch | **1 proposal** (`D-arch-resources`, warning) — raw twin at `.raw-fanout-arch.md` | D-arch-resources · D-arch-decisions · D-platform-claim |
| security-plan | `proposals: []` | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim |
| design-system | `proposals: []` | D-design-tokens · D-design-derived-count · D-platform-claim |
| layout-templates | `proposals: []` | D-layout-surface · D-layout-derived-count · D-platform-claim |
| test-plan | `proposals: []` | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim |
| obs-plan | `proposals: []` | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim |
| a11y-plan | `proposals: []` | D-a11y-surface · D-a11y-obs-schema · D-platform-claim |

## Clean-return substance worth keeping

- **obs-plan independently reproduced the report's `not carried` disposition.** Each of the five retired
  scenario names returns 0 hits in `obs-plan.md`; the only roster (`:349`) is the delegated-timing family,
  none of which is this chunk's. Its generic clause (`:229`, "a blocked or declare-only scenario emits no
  check line at all") is REINFORCED by five more retirements, not falsified. The 4 572-char `:349` was
  resolved by bounded offset read, not from the grep view.
- **test-plan independently confirmed the registry literal is single-sited.** "The declare-only registry
  literal (`NINE families`) lives only at `architecture.md:135`"; test-plan's own ten `declare-only`
  mentions are per-scenario statements carrying no count and no closure qualifier, so §6's series is
  **incomplete, not stale-valued**. All five names and their P-IDs return 0 hits there.
- **design-system read its hits rather than counting them.** Its `7`/`8` matches are expression-level band
  labels (`0.7-0.8`), a crate version (`comfy-table 7`) and `8px` token values — none a palette row, ANSI-map
  entry, token label or reuse tally.
- **layout-templates resolved both long lines by offset** (the 2 261-char `:190` and the 743-char `:286`);
  its only `declare-only` site names the behaviour/SET and bakes no literal.
- **a11y-plan cleared an adjacent item correctly.** The `nvda-pass-spec.md:73` disposition sits in the
  report's Outcome, not in a Changes bullet, and that file is absent from the six-file `Files` list — so
  a11y-plan's "51 rows" statement records nothing this chunk changed.
- **security-plan surfaced one honest near-miss and dispositioned it.** §Dependency Security states
  "564 crates" (`:178`) against this report's 562 packages; it sits inside the dated 2026-09-05
  re-adjudication history, explicitly "a reconciliation stamp, not a second source" (`:182`), and this
  chunk's `Cargo.lock` is byte-unchanged — so the number is a point-in-time stamp this chunk did not move.
  **Orchestrator disposition: not this chunk's drift.** If 564 is stale against HEAD it is prior debt from
  the 2026-09-07 `indicatif` bump that dropped `number_prefix`; the report's Changes carry no lock delta, and
  the detector contract makes the report the single source of what changed.

## Orchestrator correction to the arch proposal (applied form differs)

`D-arch-resources` proposed re-basing `architecture.md:135` on the premise that **both** the count and the
set are stale, and offered a reframing ("every committed scenario ships declare-only EXCEPT the two"). The
count half is **wrong**, and the report's own fourth Counts bullet carried the same error.

Re-derived from the clause itself (offset read of the 1 701-char line): the registry enumerates **21
scenarios in 9 groups** — `fingerprint-storm` · `error-baseline-spike` · `latency-regression` ·
`restart-suppression` · `pii-scrub` · the connection family · the delegated-timing family · the
severity-lifecycle family · the structurally-dead-assertion class. This chunk's five scenarios join the
**existing** structurally-dead-assertion class, so:

- the FAMILY COUNT does **not** move — NINE stays correct;
- that class's membership grows **3 → 8**, and the enumerated scenario total **21 → 26**.

Applied per amendment-flow §Apply ("the applied text is RE-DERIVED from the invariant + the report's fact —
never pasted from the proposal's `change` line"): the class enumeration gains the five with their date, the
count stays, and a durable non-re-staling anchor is added naming the two scenarios that still declare live
checks. The proposal's wholesale reframing was NOT taken — it would change the registry's subject from
"families retired to declare-only" to "all declare-only scenarios", discarding the retirement-history the
registry exists to carry.

## Validation

| check | outcome |
|---|---|
| 1 · playbook | No rule matches a registry-enumeration growth. Not structural, not surprising — direct precedent (this registry moved EIGHT→NINE one chunk ago; re-based to the full SET 2026-08-19). The operator's RECORDED direction (the plan's Expected-amendments entry, approved at P5) settles the amendment → **apply**, and propose the rule at the wrap card. |
| 2 · cross-contradiction | N/A — one proposal. |
| 3 · intent-consistency | Aligned. One correction owed to the report itself (the count-stale claim), applied below. |
| 4 · absence-needs-evidence | The single-site claim is corroborated three ways: the orchestrator's `grep -noE '(EIGHT\|NINE\|TEN\|ELEVEN) (declare-only\|families)'` → 1 hit; test-plan's agent independently; and the cascade sweep recorded in the sidecar entry. |
| 5 · expected-amendments | 4 plan entries, all dispositioned: arch **carried** (this proposal) · obs-plan **not carried** (0 hits, confirmed twice) · test-plan **not raised** (incompleteness is not drift; no stale value measured) · matrix#v3-04 notes **P7.3**. |
| 6 · disproved-claims | The report's one entry (`scenario.rs:1299-1302`, P-056 satisfiability) is DISPOSED — corrected in code by this chunk, and confirmed absent from every spec master (`grep -rn 'RetroactiveReeval' .andromeda/*.md` → 0). |

**Escalations: 0.**
