# Session Handoff

**Last Updated:** 2026-09-10T16:08:38Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **2 ahead at wrap start**, so
**3 ahead and unpushed** after this chunk's commit. The push stays **NOT load-bearing** — no commit in the
chain touches `ci.yml`, and this chunk has zero source delta, so no gate's first CI run waits on any of them.)
**Status:** clean
**Last Commit:** `feat(2026-09-10-live-pulse-in-lane-scenario-round)` (this wrap)

## Position
- Done: **`2026-09-10-live-pulse-in-lane-scenario-round`** — P-067, P-072 and P-079 each driven against a
  live operator-launched Pulse to a **non-blocked** verdict, with journal + `runs.db` evidence per scenario
  and a durable copy committed under the chunk's `evidence/`.
- Next: **`/andromeda-phase`** on the final markerless entry — **_Release build and bundle_**
  (`working-route.md`). It carries 4 `CARRY`s, 0 `PREREQ`, 0 `BLOCKED-ON`, so phase will not halt.
- Coverage **29/32 verified · 1 deferred · 2 unclaimed** (`v2-21`, `v2-27`). `v2-04` verified this wrap.
  `done-test` flips when both remaining ids read `verified` or `deferred` — the release chunk owns `v2-27`,
  and `v2-21` is claimable there by a by-construction argument.
- **Version-close arithmetic unchanged:** the version closes on *Release build and bundle*; nothing moves
  to 0.3.0.

## Work done
Three live legs, each fired behind a **150 s quiet window** (Pulse's 120 s idle + a full 30 s resolver tick)
because Pulse dedupes a new incident against ANY open incident and every `conductor run` fires its own
preflight canary — back-to-back, legs 2 and 3 would each have landed the `Blocked` row `v2-04` forbids.
Verdicts: `[MANUAL]` · `[MANUAL]` · `[HOLD]` (P-079's `CalibrationRegion` lamp). `run_check` rows measured
**0 / 0 / 1**, exactly the declare-only vs checks-bearing contract. **Zero source delta** — the deliverable
is driven evidence plus two recorded findings.

## Drift resolved
**1 amendment applied · 0 escalations · drift = 0.** All seven detectors returned `proposals: []`. The one
amendment was **orchestrator-raised under Validate check 5**, not proposed by any detector: `obs-plan.md:349`
glossed `read_back_observed_at − journal_emitted_at` as "(Conductor's MCP round-trip)", which this chunk
measured false — the delta covers the whole emission window (legs of 18s/35s/30s recorded `latency_ms`
18169/35120/30212 while the sidecar's own calls logged `duration_ms` 0–22). Playbook `:28` governed it as
routine; the passage's conclusion is unchanged and strengthened. Sweep found the gloss at that site only
(9 latency-formula hits read; `grep -rn "MCP round-trip" .claude/ CLAUDE.md` → 0), so **0 leaves** needed
re-derivation. Caught only because the obs detector volunteered an observation OUTSIDE its four invariants —
recorded as a structural blind spot: no obs-plan detector covers latency semantics.

## Notes
- **Plan corrected between runs (operator-directed, disclosed in the report).** The three live legs carried
  `contains P-067|P-072|P-079` — atoms the CLI never prints (it prints the scenario name; `grep -c 'P-0'`
  over every leg log returns 0). Corrected to the scenario names before the light gate. **Authoring cause,
  now curated:** a `leg` entry is exempt from P5's baseline run by the `new`/`baseline` invariant, and that
  baseline is what catches an unsatisfiable expectation on every other entry — so a live leg's `expect`
  atoms have **no mechanical check at all** today.
- **P-079 carries TWO structurally-dead assertions**, routed as ONE owned item to
  `.andromeda/residuals.md` (`open`, target: next) per the operator's ruling: the always-false
  `CountAtLeast >= 1` over an always-empty `span_refs`, and an **unattainable `slo_tier = "<20s"`**
  (30 s of declared phases; measured 30 212 ms against a 20 000 ms deadline — a NEW finding this chunk).
  Neither blocks the release; both route to `CalibrationRegion`. Two `CountAtLeast` siblings remain
  (`findings-counter-refresh.toml:49`, `pulse-run-contract.toml:58`) — take the class together, not P-079
  alone, which would contradict an accepted sibling.
- **Supply chain moved with a byte-unchanged lockfile:** 1243 advisories · 562 packages · **7** allowed
  (was 1239 · 562 · 17 on 2026-09-07), both gates exit 0. External advisory-DB movement; the
  `security.md` clause was extended in place with today's reading.
- **Live-leg posture for the next round** (unchanged, and re-verified from Pulse's own log): default
  bootstrap window in force (`grep -c 'triage.baseline.bootstrap_window.override'` → 0), zero silence cues
  emitted (the 1177 `service_went_silent` hits are all evaluator *cycle* lines).
- Curation: Tier 2 ×1 new (`verification-harness.md`) + 1 extended in place (`security.md`). 2 candidates
  deduped as `recurrence-despite-learning` against correct existing entries.
- **Last failed command:** none.
