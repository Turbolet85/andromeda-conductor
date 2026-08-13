# Fan-out results — 2026-08-13-dispatcher-determinism-goldens

7 Explore doc-agents, one parallel batch. **3 proposals · 4 clean · 0 escalations.**
Raw twins kept for the three proposal-carrying docs; the four clean returns are recorded here
(each carried trailing evaluation notes after its `proposals: []`, stripped before parsing).

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **1 proposal** (resources) · decisions clean |
| security-plan | D-security-input · -subprocess · -deps | clean (`proposals: []`) |
| design-system | D-design-tokens · D-design-derived-count | clean (`proposals: []`) |
| layout-templates | D-layout-surface · D-layout-derived-count | **1 proposal** (derived-count) · surface clean |
| test-plan | D-tests-coverage · -framework · -obs-harness · -derived-count | **1 proposal** (coverage/§7 inventory) · other 3 clean |
| obs-plan | D-obs-instrumentation · -stack · -redaction | clean (`proposals: []`) |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | clean (`proposals: []`) |

## Proposals (all validated routine, all applied)

**1. arch · D-arch-resources · warning · §Occupied Resources, the `contracts/pulse-load-envelope.toml` bullet**
The bullet asserted "Only the scenario-duration term is asserted — the rate terms are *derivable but
deliberately not yet asserted*" and predicted a re-scope "to emitting-phase duration … carried as chunk
work". Both clauses stale. Applied: the two sustained terms are the asserted pair judged per emitting phase;
`max_scenario_duration_ms` recorded-not-asserted; one shared basis for gate and caption; ledger empty; the
falsified summed-duration prediction recorded with its measurement.

**2. layout-templates · D-layout-derived-count · warning · §Surface: cli, `conductor run` output structure**
Verified by direct grep at `layout-templates.md:188-190`: the sample caption baked
`scenario "…" runs 900s, over the proven-good envelope ceiling of 600s` — a caption `classify` can no longer
produce. Applied: the sample now names the breaching emitting phase and which sustained term it left, naming
the term SET rather than a fresh literal (the detector's own re-stale guidance). **Not in the plan's
Expected-amendments list — a genuine detector find.**

**3. test-plan · D-tests-coverage · warning · §7 Test Data & Fixtures, `Golden artifacts` row**
The row enumerated only the envelope/journal goldens while the chunk committed 4 new stream goldens in two
new families. Applied: two families named (`replay__*` · `pacing__*` · `dispatch_wire__*`), stated as a set
plus the per-family × seed rule; stream goldens EXCLUDE `*_time_unix_nano` rather than redacting; the
never-`cargo insta review` rule made explicit.

## Clean returns — substance

- **security-plan** — the only Dependencies change is `insta` as a **dev**-dep of `conductor-run`, already a
  workspace dep with zero new `[[package]]`; no new external-input boundary (the contract artifact and its
  `validate()` are unchanged); no sidecar-spawn/data-dir touch. All three detectors' preconditions unmet.
- **design-system** — no `hardcoded✗` flag; the `[ENVIRONMENT-SUSPECT]` caption and its ANSI 246 Residual-mute
  reuse are unchanged, and the doc states neither an exempt-ledger count nor an asserted envelope term, so the
  derived-count grep produced no hit.
- **obs-plan** — no new must-trace operation (`sustained_storm_ms` is pure, no I/O); `insta` is snapshot
  testing, not telemetry, so no OTel SDK/exporter; the new committed artifacts carry no host path and the
  dispatch-tier projection excludes every `*_time_unix_nano`, so the redaction surface is unchanged.
- **a11y-plan** — no interactive UI element added (all Symbols are Rust-internal, a11y `n/a` on all three
  coverage rows); neither the obs JSONL envelope nor the a11y violation schema is touched.

## Validation (main)

| check | outcome |
|---|---|
| Playbook | all 3 match *spec-illustration → sound-impl* (2026-06-15); arch additionally the `contracts/` artifact-row rule (2026-08-10). No escalate verdict, no uneasiness. |
| Cross-contradiction | none — 3 disjoint sections across 3 docs |
| Intent-consistency | report deviations #1/#2 are *justified* divergences, operator-approved at phase P5 on measurement and already recorded as PREMISE-CORRECTION in `scope.md` + `verification-matrix.json#v2-06` notes. The working-route entry is `[marker]`-frozen — untouched. |
| Absence-needs-evidence | all 3 proposal targets verified by direct grep before applying (`architecture.md:160`, `layout-templates.md:188-190`, `test-plan.md:391`); each clean return cited its own basis |
| Expected-amendments floor | plan listed 3: arch ✓ proposed · test-plan §7 ✓ proposed · `scheduler.rs:69` doc-comment **ruled out of amendment flow by operator directive** (all seven masters grep clean of that claim; the wording is source, and wrap does not edit source) → routed to P5 route-resolve as a ride-along CARRY. Floor met, +1 bonus find. |

## Cascade

- **Step 1** — 3 spec bodies edited, each re-read and verified before its sidecar was appended.
- **Step 2** — cross-master grep over all seven for the old wording of each amended passage
  (`derivable but` / `not yet asserted` / `declared-not-derivable`; `runs 900s` / `envelope ceiling of 600s` /
  `Only the scenario-duration`; `insta 1.46.1 JSON snapshots`): **zero residual hits**. The only matches for
  `emitting-phase duration` are the newly written narrations of the falsified prediction in `architecture.md:160`
  and `contracts/pulse-load-envelope.toml:44`. No lateral bind engaged (test-plan §3 ↔ obs-plan §3 untouched;
  a11y ↔ obs schema untouched).
- **Step 3** — leaf distillations recomputed from the now-current sources: **no change**. The amended content
  sits below distillation grain — `CLAUDE.md:14` names the `contracts/` artifact category, `stack.md:37` and
  `.claude/rules/testing.md:18` name insta at framework grain, `security-summary.md:15` names `validate()`-at-load;
  all remain true verbatim. `CLAUDE.md`'s eleven Critical Warnings carry no envelope-term detail.
- **Carried to P3 curation:** `.claude/rules/testing.md:64` — a `## Session Additions` entry whose 2026-08-11
  extension states the load envelope's rate terms are "*derivable but not yet asserted*" and that "asserting
  emitting-phase duration is carried as its own chunk". The cascade **preserves `## Session Additions`
  verbatim**, so this is not a cascade edit; it is an in-place curation extension (P3), the channel that
  entry's two prior extensions already used.

**drift = 0** — every proposal applied, zero escalations, cascade closed.
