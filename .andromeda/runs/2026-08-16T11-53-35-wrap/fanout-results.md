# Fan-out results — 2026-08-16-fault-application-spans

7 Explore doc-agents, one per spec source, run in one parallel batch against
`conductor-0.2.0/chunks/2026-08-16-fault-application-spans/report.md`.

**Totals: 6 proposals (all obs-plan) · 6 applied · 0 rejected · 0 escalations.**
Six docs returned `proposals: []`. Their returns carried explanatory commentary after the YAML (stripped
before parsing); the substance of each verdict is consolidated below rather than as near-empty raw twins.
The one return carrying proposals has its raw twin at `.raw-fanout-obs-plan.md`.

| doc | detectors | verdict |
|---|---|---|
| architecture.md | D-arch-resources · D-arch-decisions | clean |
| security-plan.md | D-security-input · D-security-subprocess · D-security-deps | clean |
| design-system.md | D-design-tokens · D-design-derived-count | clean |
| layout-templates.md | D-layout-surface · D-layout-derived-count | clean |
| test-plan.md | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count | clean |
| obs-plan.md | D-obs-instrumentation · D-obs-stack · D-obs-redaction | **6 proposals** (all D-obs-instrumentation) |
| a11y-plan.md | D-a11y-surface · D-a11y-obs-schema | clean |

## Clean verdicts — the reasoning worth keeping

- **architecture.md** — no new port / socket / endpoint / IPC method / env var / workspace crate; the new
  public symbols (`PhaseWindow`, `run_timeline_observed`) are the library-symbol class arch deliberately
  omits (playbook 2026-06-19/06-21). `tracing` is already in §Stack as the self-obs library.
- **security-plan.md** — D-security-input: no new external-input surface (`classify_fault` reads an
  already-garde-validated config struct; the five new allowlist names are OUTPUT redaction, bounded labels
  and integers). D-security-subprocess: the spawn / preflight / data-dir files are not in the changed set.
  D-security-deps: the dependency delta is admitted under §Dependency Security's 2026-08-16 clause — deny
  VERIFIED green over the new lock, zero new `[[package]]`. The reverted `tracing-subscriber` dev-dep never
  landed, so it is not a change.
- **design-system.md / layout-templates.md / a11y-plan.md** — no rendered surface, no interactive element,
  no violation-schema change; every Coverage row is `a11y n/a · tokens n/a`. The a11y agent additionally
  confirmed the envelope it reproduces verbatim (11 fields) is untouched, and that a11y-plan §3's standing
  claim "obs §4 currently has no focus-relevant spans" stays accurate.
- **test-plan.md** — every new surface carries a test at its mandated tier, with the port-occupier at the
  INTEGRATION tier §4 explicitly requires for the `:4317` bind/release. Both moved counts were grepped:
  `31` / `36` / `588` / `597` return no matches in test-plan.md; the allowlist references name the mechanism,
  never its size — correct set-naming, explicitly not a hit. §3 and obs §3 both describe the span-lifecycle
  line by SET ("the bounded §4 span name", "the span's own allowlisted attributes"), so three new names and
  five new attributes are absorbed identically on both sides — no one-sided change, no bind drift.

## obs-plan proposals (all applied)

| # | section | change | kind |
|---|---|---|---|
| 1 | §4 Fault-injection spans — placement paragraph | spans open from the run-path per-phase hook, not inside `conductor-faults`; created-not-entered; only `port_occupier` is in-crate | primary |
| 2 | §1 instrumentation-scope, `conductor-faults` row | same claim restated in the entity table | `dependent-of` #1 |
| 3 | §4 `Fault.ramp` attributes | `ramp_factor` 0.0-1.0 → **−1.0..1.0**, normalized signed slope | primary |
| 4 | §4 `Fault.port_occupier` | attributes narrowed to `fault_type` + `port`; hold witnessed on `message` at `debug`; parentage CONDITIONAL | primary |
| 5 | §1 telemetry-trigger, `chaos-instrumentation` row | same flat attribute triple restated | `dependent-of` #4 |
| 6 | §6 Log Coverage — log-levels table | fault-span lifecycle `debug` → `info`, with the reasoning recorded beneath the table | primary |

## Validation (6 checks)

1. **Playbook** — all six match the 2026-06-15 rule *"an amendment reconciles a spec's illustrative mechanism
   or wording to the sound implementation actually shipped, where the chunk report demonstrates the invariant
   still holds"* → routine. Note the 2026-06-20 deferred-span dismiss rule does NOT apply: its precondition is
   "the current chunk emits no span", and that rule itself names this as the wiring epoch where the entry
   lands. 0 escalations.
2. **Cross-contradiction** — none; six edits to one doc, all in one direction.
3. **Intent-consistency** — the report's divergences (port-occupier parentless; the smoke's firing form) are
   justified and were ratified at phase P4/P5 as intent-incomplete, not defects.
4. **Absence needs evidence** — #1 rests on the code-graph trace cited in the report
   (`tree-query-2026-08-16-fault-application-spans.json`, 0 rows twice); the count-absence claims rest on
   greps the agents ran and reported.
5. **Expected-amendments reconciliation** — the plan's list carried 3 (§4 range · §4 placement · §4
   port-occupier parentage); all 3 proposed. The operator-directed 4th (§6 levels) was also proposed. Nothing
   under-ran.
6. **Disproved-claims disposition** — all 4 entries in the report's *Spec claims disproved by measurement*
   bullet are matched by a proposal (#1+#2, #3, #4+#5, #6). None left silent.

## Cascade

- Grepped all seven masters + the three preserve-verbatim curation homes for the retired wording
  ("fault application phase", "spans around each fault", `ramp_factor`, "fault application" in a level
  context): **zero hits outside obs-plan and its sidecar**.
- `fault.silence`/`fault.port_occupier` appear in `playbook.md`'s 2026-06-20 deferred-span rule. That rule is
  a class rule and is not falsified — its fault-span half is simply spent, since the spans now emit. Flagged
  to the operator rather than edited (playbook is an operated validation artifact, not a spec master).
- Leaf re-derivation of the one changed source: `.claude/rules/observability.md` gained a fault-span
  placement + level bullet; `.claude/docs/obs-summary.md` recomputed with **no delta** (it distills tier /
  harness / SLO / identity / redaction / anti-patterns / decisions — none touched; its "never log in a hot
  path at `info`" line stays true and is what the §6 note explicitly preserves).
