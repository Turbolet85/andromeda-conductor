# Fan-out results — the 2026-10-07T13-18-48 wrap

Seven doc-agents, one parallel batch, 28 detector slots (the drift-base's `doc:` names). Each return was stripped of
its commentary and parsed; no return failed the parse.

## Verdicts

- **architecture** — 2 proposals. Stripped: a no-hit note per remaining detector (no new resource; no second owner;
  both registries within target before the pass; no platform verdict retired) and a sweep note.
- **security-plan** — 5 proposals. Stripped: no-hit notes for subprocess, deps and platform; a severity note (all five
  `warning`: the escalate trigger, an unvalidated boundary, is not met); an observation that §Dependency Security
  does not say an empty porcelain precedes a kept GREEN audit reading.
- **design-system** — `proposals: []`. Stripped: a basis note (no UI rendered; 0 hits for every moved value).
- **layout-templates** — `proposals: []`. Stripped: a basis note (no surface added; 0 hits for every moved value).
- **test-plan** — 5 proposals. Stripped: no-hit notes for coverage, framework, obs-harness and platform, and a note
  that two distillations and `architecture.md:70` carry the same wording.
- **obs-plan** — 2 proposals. Stripped: no-hit notes for stack, redaction, ci-gates and platform.
- **a11y-plan** — `proposals: []`. Stripped: a basis note (no interactive element; no schema change; 0 hits).

No `.raw-fanout-*` twin was written for the three empty returns although stripping changed them: each verbatim
return carries a home-rooted path of this host (the prompts named the documents by absolute path), which the
run-dir hygiene read refuses and the repository does not publish. The substance of what was stripped is above.

## Proposals and dispositions

| # | Doc · detector | Section | Proposed change (in short) | Disposition |
|---|---|---|---|---|
| A1 | architecture · D-arch-resources | §Occupied Resources, the posture-contract entry | replace the dated series list with a record-kind statement naming the capture run, byte-neutral | **apply** — check 1 (accurate this-chunk addition; the plan's entry names the change). Applied without the proposal's count literal ("five"): the set is named, the dates move to the sidecar |
| A2 | architecture · D-arch-decisions (dependent of A1) | §Established Decisions, the real-model leg sentence | widen "only as the pre-stated series" to "a pre-stated series or capture run" | **apply, re-derived** — check 1. The rule is kept and the 2026-10-07 capture run named as its one dated, verdict-less record; no class of capture runs is minted |
| S1 | security-plan · D-security-input | §Input Validation, ingest row, leaf-rendering clause | name the capture run beside the four series | **apply** — check 1 |
| S2 | security-plan · D-security-input (dep.) | same row, the data-dir fallback clause | name the capture run | **apply, scoped** — check 4: the fallback is stated on its measured basis (the series' binary by digest, the launch shape, the boot line's basename) |
| S3 | security-plan · D-security-input (dep.) | same row, Boundary cell | "per series drive" covers a non-series run | **apply** — check 1 |
| S4 | security-plan · D-security-input (dep.) | §Security Anti-Patterns → Data Protection, inventory | add the capture run's three captures | **apply** — check 1 |
| S5 | security-plan · D-security-input (dep.) | same, one dated clause | record the operator-owned argv recording that no Conductor artifact holds or reads | **apply** — checks 1 and 5: the plan's approved entry names this change; nothing crosses the capture boundary, so the boundary-widening rule does not match |
| T1 | test-plan · D-tests-derived-count | §6, the leg bullet's opening | widen "only as the pre-stated series" to the contract's record set | **apply, re-derived** — as A2 |
| T2 | test-plan · D-tests-derived-count (dep.) | §2 Deterministic | the same claim | **apply, re-derived** — as A2; "each counted drive" left as written |
| T3 | test-plan · D-tests-derived-count (dep.) | §9 Live-Pulse scenarios | the same claim; the re-fire term deferred to each record's clause | **apply, re-derived** — as A2; the series' re-fire term stands and the run "re-fires only as its own section says" |
| T4 | test-plan · D-tests-derived-count (dep.) | §3 → 5-command implementation | "no counted drive of the series" → "no drive of the pre-registered record" | **reject** — check 4: the clause states no only-claim and holds for every series drive; with the rule kept in the body it restates nothing retired |
| T5 | test-plan · D-tests-derived-count | §6, the bullet's closing | a set statement separating series from the capture run; disambiguate the dated pointer | **apply** — checks 1 and 5; the capture run's dated record is written in the earlier records' form (its grades as observations), the d3 rate and the d2 observation stay out of the body |
| O1 | obs-plan · D-obs-instrumentation | §4, the Real-model posture bullet | one dated live observation | **apply** — check 1; the bracket sentence dropped (a harness-reading fact, not this bullet's) |
| O2 | obs-plan · D-obs-instrumentation | same bullet | the series' pointer names its chunk | **apply** — check 1 |

Check 2 (cross-contradiction): none — A2 and T1-T3 edit one claim in one direction. Check 3 (intent): the report's
deviations each carry a justification inside the chunk's intent; the scope record is empty. Check 5: all four of
the plan's expected amendments are matched (A1 · S1-S5 · T1-T3, T5 · O1). Check 6: the report lists no disproved
claim. D-arch-registry-size, run after Apply: §Established Decisions 38068 B, §Occupied Resources 38114 B,
threshold 38115 B — within target.

Escalations: **0**. Applied: 13 proposals as 11 body edits in four masters; rejected: 1 (T4).
