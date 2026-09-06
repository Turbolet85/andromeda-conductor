# Session Handoff

**Last Updated:** 2026-09-06T11:20:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0` at `59d5b7c` — **4 ahead** after
this adaptation commit. CI has not run since 33954347685 on `59d5b7c`; `ci.yml` last changed `d2ca431`
2026-06-27 and declares only `rust` + `frontend` jobs, so *A11y CI gate*'s BLOCKED-ON stands — re-verified
by reading the single `a11y` hit, which is a comment inside the npm-audit step.)
**Status:** clean
**Last Commit:** `chore(route): operator-requested adaptation — 0-pending wrap`

## Position
- Done: **`2026-09-06-operator-gated-live-suite`** (master's last `complete`, wrapped at `e63be48`), then
  this **0-pending adaptation** correcting one of its conclusions.
- Next: **`/andromeda-phase`** to promote + plan the next markerless head — **_Run-report envelope
  conformance gate — every run journal row schema-complete and host-path-free, build failing on violation_**
  (`working-route.md:113`). No live Pulse needed.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) —
  unchanged; this path claimed nothing.
- **Evolve:** Epoch 6a diagnosed; Epoch 6b is 2 chunks in — no nudge.

## Work done — the correction
The previous wrap concluded `auto-resolve-idle-window`'s failure was a MARGIN problem and widened its window
165 s → 200 s. **That was half right, and the operator's standalone run proved it:** at 200 s it failed the
same way (`2026-09-06T10-58-18-536` → `ManualCheck`, `latency_ms 200048`, 6 fingerprints).

The real dominant constraint is **SUT uptime**: observable 1 needs an EMPTY active set at read-back, which
holds only inside the emitting service's ONE-HOUR bootstrap window. Past it, `service_went_silent` cues raise
new autonomous incidents during the leg's own silent phase — so a longer window is strictly worse.
`BOOTSTRAP_WINDOW_SECONDS = 3_600` (`andromeda-pulse activity_floor.rs:36`, evaluator per emit cycle at
`emitter.rs:186`, HEAD `83d4060`) — all three coordinates re-derived here, plus Pulse's own log.

**Two refinements I found while verifying, both material:**
1. A bare grep for `service_went_silent` gives 7632 hits starting 09:02:03 — which would have *contradicted*
   the cited 10:11:09. 7398 are the evaluator RUNNING (`triage.baseline.…evaluate`). Filtered on
   `target == triage.cue.emit`: 66 cues, first at **10:11:09.723Z** — exactly one hour after the canary
   service's first span, which confirms the bound is the SERVICE's window, not Pulse uptime.
2. **The two failed runs had DIFFERENT causes.** The light-gate re-run (10:07Z) ran wholly INSIDE the window
   with zero silence cues and failed on the margin — its own preflight formed TWO incidents (10:07:18,
   10:07:53) and the later was ~122 s old against the 150 s worst case. The standalone run (11:00Z) failed
   past the window on cues. The operator ruled the CARRY carry BOTH, since tuning for one mis-tunes the other.

## Drift resolved
**2 amendments (full P2 apply-form, self-produced-fact door) · 0 escalations.**
- `test-plan.md` §9 — the composed `run --live` passage now carries the leg's SUT-uptime precondition and
  states explicitly that it is **not run-stable**: an unchanged tree grades differently by uptime alone.
- `architecture.md` §Established Decisions [Read-Back Dependency Posture] — the leg-E "nothing refreshed it"
  narrative is QUALIFIED (not retired) as uptime-bound. Taken as the wrap's call because that is the passage
  the bound qualifies, and the same one that retired the degraded universal hours earlier.
- Cascade: `architecture.md:134` was READ rather than pattern-matched and is NOT a duplicate — it defines the
  route's trigger, which is still true. No leaf body edit owed.

## Notes
- **KNOWN STALE, owned, deliberately not fixed here:** `scenarios/auto-resolve-idle-window.toml:29-32` still
  states the superseded margin model. It is CODE, outside the 0-pending path's scope, and is owned by the
  CARRY pinned to *Halo hue budget re-driven* (`working-route.md:117`) — which also says the 200 s value
  itself is fine and the harvest literals stay as the record of the arm when it was reached.
- **Curation:** Tier 1 ×1 extension (the TIME axis's second face — the SUT's own clock running while the tree
  stands still; and separate two failures by timeline before assigning one cause) · Tier 2 ×1 extension
  (`verification-harness.md` gains mechanic (e): run an absence-subject leg on a freshly-started Pulse and
  record SUT uptime beside the verdict) · Tier 3 ×0 · filtered 0. `CLAUDE.md` **135/200**.
- **Light gate:** no-op by path — this is a 0-pending adaptation with no chunk and no `plan.md` Test
  Commands. No source changed (the only code-adjacent file, the scenario TOML, was deliberately left alone),
  so nothing to re-run; the last full green is `e63be48` (nextest 878/878, both clippy passes, audit+deny).
- **`pulse-app` is DOWN** (stopped 11:05Z by the overseer); census clean, no listeners. Any re-proof of
  observable 1 needs a FRESH data dir and must run inside the first hour of the canary service's life.
- **Last failed command:** none.
