# Curation log — 2026-09-15-remaining-structurally-dead-declarations-retired

Mode: default (auto-apply). Candidates scanned from the session conversation + the report's
*Decisions & corrections*. Evolve records and `friction-log.ndjson` were NOT treated as candidates
(telemetry of the tool, never a session learning — curation-guide §Analysis scope).

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   + testing.md: inferred-token EXISTENCE check
                                              + testing.md: wide-sweep over-return hazard
  Tier 3 (.claude/docs/session-learnings.md): (none)
  Filters: 2 dup · 0 task-specific · 0 conflict · 2 below-threshold (exactly 0.6 — the lean default)
  Extended: T2/testing.md: "2026-06-23 Absent token-choice traps" + "existence precedes casing"
  Extended: T2/testing.md: "2026-06-22 companion-sweep" + "the wide form over-returns"
  CLAUDE.md size: 136/200 · T1 45.6 KB, 8 over 600 B
```

Tier counts count WRITES — both survivors are in-place extensions of matched entries, one write each.

---

## Applied

### T2 · `.claude/rules/testing.md:62` — extension of the 2026-06-23 `Absent` token-choice entry

**Facet:** the casing check presumes the producer emits SOME spelling — confirm the token EXISTS in the
SUT at all before choosing it; non-collision and emittability are independent properties.

**Proof:** `git grep -c --fixed-strings 'RetroactiveReeval' -- '*.rs'` at `andromeda-pulse` HEAD `83d4060`
exits **1** — zero occurrences tree-wide, and no `retroactive_reeval` sibling. The only `retroactive`
occurrences are two doc comments (`crates/triage/src/lifecycle/mod.rs:57`, `:103`). The token was chosen
BY THIS ENTRY'S OWN trap-(2) advice (non-collision), so the entry's example is the counter-example: the
`Absent` was vacuously true from the day it was authored. Retired this chunk in
`scenarios/threshold-hot-reload.toml`.

**Scoring:** verified-by-measurement +0.4 · specific-technical-detail-with-context +0.2 = 0.6 exactly;
**no-other-home +0.2 → 0.8**. The conditional fired because the general RULE reached no durable home this
wrap: the scenario TOML header and the `scenario.rs` comment carry the INSTANCE, and neither the arch
amendment, the route tail, the playbook/drift-base bases, nor the matrix ledger note carries the rule.

### T2 · `.claude/rules/testing.md:54` — extension of the 2026-06-22 companion-sweep entry

**Facet:** widening the sweep's scope fixes the false negative and buys a false positive — the wide form's
hits must be read, never counted, because a scenario name that doubles as domain vocabulary over-returns.

**Proof:** `grep -rln "activity-floor" crates/` → **9 files**; the same command for the other four names in
this retirement → **1 file each**. Of the nine, only `crates/conductor-core/src/scenario.rs` is a
check-membership pin. The other eight, all read and dispositioned no-change: doc comments naming the
P-013/P-014 fault family (`conductor-faults/src/{train,silence,lib}.rs`, `conductor-core/src/phase_spec.rs`),
load-envelope duration fixtures (`conductor-core/src/load_envelope.rs`), and envelope-exemption note strings
inside test fixtures (`conductor-cli/src/render.rs`, `conductor-report/src/{report,db}.rs`).

**Scoring:** verified-by-measurement +0.4 · specific-technical-detail-with-context +0.2 = 0.6 exactly;
**no-other-home +0.2 → 0.8**. Filter 2 KEEPS this despite naming a non-public identifier: the sweep-hazard
clause overrides the non-public-name reject ("grep for X returns Y because Z" — the hazard IS the name).

---

## Rejected

| candidate | filter | reason |
|---|---|---|
| A plan step can delegate a correction to a step that structurally cannot make it (step 13 → step 12, different functions) | 1 · dup | CLAUDE.md Tier 1, 2026-09-04: "two instructions, each individually unambiguous and jointly contradictory… consistency BETWEEN them is unowned unless someone checks it" — token overlap above the bar. |
| Re-derive a dictated count before committing it (the ruling's "five pins" measured six) | 1 · dup | CLAUDE.md Tier 1 already carries the re-derive rule and the measured instance ("a dictated 6 sites / 3 callers measured 6 / 4"). |
| The P-013/P-014 `ServiceWentSilent` illustration now describes retired declarations | 4 · below threshold | 0.4 measurement + 0.2 detail = **0.6 exactly → rejects** (the lean default). Neither conditional fires: the next promotable entry (`Scenario tier honesty`) does not need it, and the fact HAS a durable home — both scenarios are now named in the arch registry this wrap amended. The rule itself is unaffected; only its illustration is historical. |
| A registry enumerating GROUPS does not move its count when a member joins an existing group — check which grain moved before amending | 4 · below threshold | 0.4 measurement + 0.2 detail = **0.6 exactly → rejects**. `no-other-home` cannot fire: this wrap's P2 amended exactly this reasoning into `architecture-amendments.md` (the sidecar entry states the grain finding and why the count was left alone). |

## To the handoff

- `recurrence-despite-learning: CLAUDE.md Tier 1 — "a stale MECHANISM outlives the stale NAME … sweep for
  what the claim SAYS, not what it is NAMED after"`. The P2 cascade sweep was keyed on the five scenario
  NAMES first; only a second pass keyed on the retired TOKENS (`ServiceWentSilent`, `RetroactiveReeval`)
  reached the three `testing.md` sites that state the retired check-membership without naming any scenario.
  Self-corrected inside the step (the name sweep's `testing.md:56` hit prompted the re-sweep) and 0 of the 6
  token hits were in a master, so nothing shipped wrong. Logged per Filter 1's recurrence clause — a third
  corpus entry is not the remedy.
- **File-size pressure, surfaced not acted on:** `.claude/rules/testing.md` is now **80.9 KB**, past the Read
  tool's 25 000-token cap (health check 4 flags it), and both entries extended here were already over the
  ~1.5 KB Tier-2 entry cap (3 151 B and 2 552 B before this wrap). The two facets are one sentence each and
  belong beside the rules they qualify — routing them to Tier 3 would orphan an actionable authoring rule
  from its rule — but the file needs an operator promotion pass. wrap never auto-promotes.
