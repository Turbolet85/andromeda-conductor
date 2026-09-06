# Adaptation record — 0-pending wrap, 2026-09-06

**Path:** SKILL step 6 (0 pending; tree dirty only with `.andromeda/friction-log.ndjson`, the
expected-transient bookkeeping → counts git-CLEAN). No report, no fan-out. HEAD at entry `e63be48`.
**SUT:** Pulse DOWN (stopped 11:05Z) — census clean, no listeners on `:4317`/`:4444`/`:4445`. No live leg
exists on this path; every fact below was measured earlier and is re-read here from artifacts on disk.

## Context correction (supersedes the previous wrap's report + handoff)

That wrap concluded the `auto-resolve-idle-window` failure was a MARGIN problem and widened the window
165 s → 200 s. A standalone operator run at 10:58–11:02Z (`2026-09-06T10-58-18-536`) failed at 200 s the
same way — `ManualCheck`, `latency_ms 200048`, **6 fingerprints**. Verified here from the artifacts.

**Every coordinate in the directive was re-derived, and all held:**
`BOOTSTRAP_WINDOW_SECONDS = 3_600` at `andromeda-pulse crates/triage/src/baseline/activity_floor.rs:36`;
`evaluate_service_went_silent` per emit cycle at `crates/triage/src/cue/emitter.rs:186`; its
`…_post_bootstrap` test at `:1104` (SUT HEAD `83d4060`). From
`pulse-legs/a11y-20260906-110201/logs/agent-latest.jsonl.2026-09-06`: incidents created mid-silence at
11:00:33.775 and 11:02:02.789 (`created:true`, `deduped:false`, `autonomous`); the day's FIRST **emitted**
`service_went_silent` cue at **10:11:09.723Z**, magnitudes 3.0 → 5.0 → 7.03.

**Two re-derivations refined the directive** (both surfaced to the operator; the second changed a decision):

1. *Token-proxy trap avoided.* A bare grep for `service_went_silent` returns **7632** hits with the first at
   09:02:03 — which would have contradicted the 10:11:09 claim. 7398 of those are
   `triage.baseline.service_went_silent.evaluate`, the evaluator RUNNING, not a cue emitted. Filtered on
   `target == triage.cue.emit` (field key `kind`, not `cue_kind`): **66** cues, first at 10:11:09.723Z —
   exactly one hour after the canary service's first span (~09:11Z), which is itself the strongest
   confirmation that the bound is the SERVICE's bootstrap window and not Pulse's uptime.
2. *The two failures have DIFFERENT causes*, so "the cause is not margin" is true of one run only.
   The light-gate re-run's leg A (10:07:08 start, read-back ~10:09:55) ran wholly **inside** the window
   with **zero** silence cues in its span, and failed because its own preflight formed **two** incidents
   (10:07:18 and 10:07:53) and the later was ~122 s old against the 150 s worst case — genuinely the
   margin. The standalone run (11:00Z, past the window) failed on silence cues, where widening is
   counterproductive. **Operator ruling: carry BOTH causes**, since a re-attempt tuning for one would
   mis-tune the other.

## Items and dispositions

| # | Item | Disposition |
|---|---|---|
| 1 | Owner for the re-proof | **CARRY pinned to *Halo hue budget re-driven*** (`working-route.md:117`) — the operator's named entry and disposition, which satisfies the trajectory gate. Text carries BOTH causes with their tuning split, per the operator's answer, each with its `measured at` pointer. |
| 2 | Self-produced-fact amendment | **APPLIED in full P2 form** to **two** masters. `test-plan.md` §9 (the composed-stage passage) gains the SUT-uptime precondition and the explicit statement that the leg is NOT run-stable. `architecture.md` §Established Decisions [Read-Back Dependency Posture] also carries it — the wrap's call, taken YES: that section's leg-E narrative ("nothing refreshed it") is precisely what the bound qualifies, and it is the same passage that retired the degraded universal hours earlier. Both bodies verified after edit; both sidecars appended naming their searches. |
| 3 | `scenarios/auto-resolve-idle-window.toml:29-32` | **NOT edited — deliberately.** It is CODE, not a master, and outside this path's scope. It remains a KNOWN stale comment (it still states the superseded margin model) and is **owned by item 1's CARRY**, which says so explicitly. |
| 4 | Rest of the tail | **Unmoved.** The a11y-plan `:565` CARRY on *A11y CI gate* (`:121`) stands. Its BLOCKED-ON premise re-verified TRUE and not by token: `ci.yml` has exactly one `a11y` hit, and reading it shows a **comment** inside the npm-audit step — the declared jobs are `rust` and `frontend` only, so the a11y job is genuinely still absent. `ci.yml` last changed `d2ca431` (2026-06-27). |
| 5 | Commit | `chore(route): operator-requested adaptation — 0-pending wrap`; `tree.db.commit` re-pointed to the new HEAD. |

## Cascade

Masters amended: `architecture.md`, `test-plan.md`. Swept `nothing refreshed it` / `auto-resolve on schedule`
/ `empty active list` across all seven → 2 hits, both architecture. `:69` is the amended passage; **`:134`
was READ, not pattern-matched, and is NOT a duplicate** — it defines the route's trigger condition (still
true) and already cross-references the amended section. Swept `--live` across the seven → test-plan §2/§3/§9/§11
and layout-templates `:189`; only §9 describes the leg's preconditions, so no dependent amendment. Leaf bodies
(`commands.md:12`, `verification-harness.md:19`) carry the composition but assert no reachability, so the
cascade owed no leaf edit; the uptime fact is ADDITIVE and therefore routed to P3 curation instead — the three
other `verification-harness.md` hits (`:47`, `:50`, `:56`) sit in `## Session Additions` and are
preserve-verbatim.

## Curation

- **Tier 1 ×1 extension** (`CLAUDE.md`, the 2026-08-17 TIME-axis entry): the axis has a second face — the
  SUT's own clock running while the tree stands still; plus separate two failures by timeline before
  assigning one cause.
- **Tier 2 ×1 extension** (`.claude/rules/verification-harness.md`, this session's own 2026-09-06 entry
  gains a fifth mechanic (e)): the auto-resolve leg's precondition is uptime-bound; run an absence-subject
  leg on a freshly-started Pulse and record the SUT's uptime beside the verdict.
- Tier 3 ×0 · filtered 0 · conflicts 0 · deferred 0.
