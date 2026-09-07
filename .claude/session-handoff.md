# Session Handoff

**Last Updated:** 2026-09-07T08:55:21Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **1 ahead at P6** — the prior
chunk's commit is unpushed, and this wrap's commit makes it 2. No CI push from this wrap, which is why the
*A11y CI gate* entry's `BLOCKED-ON` still stands.)
**Status:** clean
**Last Commit:** `feat(2026-09-06-halo-hue-budget-re-driven): …`

## Position
- Done: **`2026-09-06-halo-hue-budget-re-driven`** (master `complete`).
- Next: **`/andromeda-phase`** on the first markerless head — **_Dependency polish_**
  (`working-route.md:119`; carries several CARRYs incl. Pulse's 8th P-047 category and the
  `cargo check -p conductor-verify --lib` per-seam build red). **No live Pulse needed.**
- Coverage **27/32 verified · 5 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`, `v2-32`) — unchanged:
  this chunk claimed nothing, by design (no `v2-NN` entry covers P-025; `v2-20` excludes it and is already
  `verified`).
- **Evolve:** Epoch 6b at 8 chunks (5 frozen + 3 markerless) — under the ~10 split threshold, no nudge.

## Work done
P-025's ≤2 s hue budget was **re-driven and measured unmeasurable through its own leaf** — and the chunk
ships the proof rather than a pass. `halo-hue-encoding` now drives 360 dispatches at 2/s across two phases,
so the service never goes quiet; on the 2026-09-07 live leg its one in-window sample still read
**14 525.9 ms against 2 000 ms**, matching its offset to the preceding 15 s lifecycle tick to **1.1 ms**.
All 7 samples in the capture fit `duration = offset + k × 15 000`, k ∈ {0,1,2}, within 2 ms.

The cause is that `ServiceRegistryEntry.last_seen_unix_nano` has **no ingest-path writer** — the 15 s tick
stamps it — so the leaf reports staleness-since-tick, independent of dispatch rate. **My P3 research missed
this**: it verified the fire site's reader and the leaf's allowlist, never the field's WRITER, which the
2026-08-16 T1 provenance rule already prescribes. The operator's P5 review caught it and it changed the
deliverable from "attain the bound" to "pin the mechanism, assert no pass arm".

Both CARRYs discharged: the scenario re-tiered `<5s` → `<90s`, and leg A reached `KnownResidual` via the
`ReadBack::AutoResolved` arm under a boot-wide `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS=86399`.

**But the light gate's literal re-run graded leg A `ManualCheck` on the same tree, posture and process** —
and that is a finding, not a flake. The posture provably still works in both runs
(`silence_cues_emitted: 0`, `services_in_bootstrap: 2`), so it removes cause (a) even at ~1 h 28 m uptime;
what separated the runs is cause (b), which it does not touch — the window's only incident landed +45 s in
(cleared 150 s → arm fired) versus +137 s in (~62 s old at read-back → missed). **The posture makes the arm
REACHABLE, not RELIABLE.** Four amendments authored earlier in this same wrap had claimed sufficiency and
were narrowed to this before the commit.

## Drift resolved
**6 amendments across 3 masters · 1 escalation resolved · 1 playbook rule minted · 1 cascade leaf fixed · 0 open.**
- `obs-plan` §4 — P-025's recorded cause: staleness → **tick quantization**, bound recorded unmeasurable
  through that leaf, with its measurement pointer and an explicit SCOPE (this leaf, Pulse HEAD `83d4060`).
- `test-plan` ×2 — §3's `--live` composition re-anchored to `live_leg_order` (leg H first) as SET-NAMING
  rather than a fresh literal; §9's not-run-stable verdict conditioned on the **default** window, carrying
  the TARGET string `triage.baseline.bootstrap_window.override`.
- `architecture` ×3 — the env registry gains `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`; §69's uptime
  bound qualified as the window *in force at boot*; and a same-master duplicate at `:175` (a second
  unqualified fixed-3600) folded in by the cascade sweep, proposed by no detector.
- **Escalation:** arch's pair had no playbook rule and the unease was precedent-shaped — arch's three
  existing `ANDROMEDA_PULSE_*` entries are all handles Conductor *reads*, this one it neither sets nor
  reads. Operator approved apply-both **plus a bounding rule**, minted: an external handle enters the
  registry only when a SHIPPED artifact names it, never one mentioned solely in a report or plan.
- **Cascade:** the retired 4-leg composition was standing in `.claude/rules/verification-harness.md:19`
  (generated body) — fixed. Two further hits sat in preserve-verbatim homes and were routed to P3, not
  edited.

## Notes
- **The witness coordinate was wrong in 5 places** (operator directive): artifacts named the emitting
  function `warn_bootstrap_window`, which appears nowhere in Pulse's log; the persisted line's TARGET is
  `triage.baseline.bootstrap_window.override`. The 3 code/config sites were corrected at this wrap; the
  plan keeps the function name as the record of what was planned.
- **Scope was widened by operator approval**, not re-planning: `operator_pause_harvest.rs` pins the
  scenario's checklist text via `committed("halo-hue-encoding")` — a companion class ("a test whose INPUT
  is the changed artifact") the plan's sweep missed because it enumerated goldens only.
- **Honest limit on the shipped pin:** it asserts against the *preceding* tick — the `k=0` case, correct
  because the scenario now guarantees continuous emission. A quiet service freezes `last_seen` k ticks back.
  Recorded in `evidence/hue-verdict.md`.
- **Pulse intake (not Conductor's):** (1) the SUT-side fix that would make the bound measurable — registry
  copies the activity floor's `last_observed_unix_nanos`, or the fire site reads span arrival; (2)
  `hypothesis:` the 120 s auto-resolve idle clock appears to run from the last cue re-touch, not the last
  span (measured on the leg log; no master states span-idle semantics, so no disproof entry).
- **Curation:** T1 0 new (1 EXTENDED in place — the SUT-clock entry gains "ask whether that clock has a
  LEVER") · T2 1 new + 2 extended (`testing.md` gains the nextest-filter-matches-nothing entry and the
  data-pin companion class; `verification-harness.md` item (e) gains the boot-time-lever facet) · T3 0.
  `CLAUDE.md` **134/200**.
- **Deferred learnings — `recurrence-despite-learning`:** the writer-sweep miss above recurred *past* a
  correct T1 entry. The remedy is a CHECK in the owning step's reference (codebase-research's mechanism
  re-derivation), not a fourth entry.
- **`pulse-app` (PID 10112) is UP** and was left running for the light gate; the operator stops it after
  this commit. The next entry (Dependency polish) needs no live SUT.
- **Last failed command:** none.
