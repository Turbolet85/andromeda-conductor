# Session Handoff

**Last Updated:** 2026-08-21T09:56:39Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **29 ahead** after this chunk commit)
**Status:** clean
**Last Commit:** 2026-08-21-severity-lifecycle-live-proof — auto-resolve proved itself on Pulse's own ledger,
and the two witnesses the plan named turned out to be the wrong ones

## Position
- Done: **2026-08-21-severity-lifecycle-live-proof** — five fresh-dir live legs at SUT HEAD `efabe8e`, all
  exit 0, all rows `verdict: null` / `KnownResidual`. **P-022 auto-resolve PROVEN at the predicted instant**
  (created 09:00:28.046 → active set emptied 09:02:45.671 = **137.6s**, inside the 120s window + 30s tick),
  with **new-not-reopen** as an independent second proof (same fingerprint `12dcd67b` → `created=true,
  deduped=false`; Pulse dedups against the ACTIVE set alone). Ingestion lossless: `span_count` **189** =
  15 canary + 12 trigger + 150 dilution + 12 retrigger. **v2-16 verified** (refined — see below).
- Next: **Per-check latency measurement** — sub-5s budgets beneath the closed `slo_tier` set, per-check
  `latency_ms` in the run-report envelope. `/andromeda-phase` to promote + plan. It carries the **36th audit
  PREREQ** in PROBE-AUTO-SATISFY form, basis re-verified to a ZERO-dependency-delta chunk.

## Work done
Zero production-source change. The delta is five scenario TOMLs (re-shaped so the SUT is actually reachable,
then retired declare-only with the measurement in each header), one new harvest binary
(`crates/conductor-run/tests/severity_harvest.rs`, 16 tests), and the five-leg evidence trail. The re-shape is
what made the proof possible: trigger/retrigger 8 → **12** exceptions (8 sat in Pulse's 5..10 dead band and
formed nothing), plus a **150-span OK dilution tail** so the sample-driven 30s error EWMA falls under the cue
threshold BEFORE silence — without it the EWMA freezes high and every tick re-fires a cue that refreshes
`updated_at`, so the 120s idle window never elapses. The three tier files separate by CONFIDENCE alone
(identical magnitude curves 3.330 → 6.549 → 9.661; only the baseline sample count differs), which measured
autonomous / suggested / curious on legs B / C / D.

**The `AutoResolved` arm fired live for the first time** (leg E) — the arm 2026-08-20 shipped unit-pinned and
unexercised. `ack-cooldown` forms no incident of its own, so only the canary's exists; its cues are `curious`
and Tier-1 accepts Autonomous alone, so nothing refreshed it, and it auto-resolved 128.4s in, leaving ~4
minutes of empty list before read-back (`result_count: 0`, envelope `fingerprints: []`).

Gates: workspace nextest **673 → 693**, `conductor-run` **95 → 111**, both runners green, clippy + doctests
clean, zero retries, **zero gate deferrals**. Smoke: `agent-run.sh status <this run's id>` (mint-then-read).

## Drift resolved
7 doc-agents / 18 detectors: **4 docs clean · 3 with proposals · 10 amendments applied · 1 escalation
resolved with the operator · 0 open.** arch 2 (the `AutoResolved` arm is no longer "unexercised live";
declare-only families **6 → 7**) · test-plan 3 (§6 + §1 CP4 twin re-based to declare-only + harvest tier, and
the §6 Steps line no longer invokes `conductor run severity-lifecycle` — **no scenario carries that name**) ·
obs-plan 4 (CP4's family-specific span chain RETIRED as never-built — all four names plus
`auto_resolve_triggered`/`summary_received` measure **zero** occurrences in `crates/`; `lifecycle_phase` /
`severity_choice_calibrated` dropped at all three sites).
**Escalation:** the plan queued "correct the P-022 sample-row label" in layout-templates; verification found
the mismatch **systemic** — 13 sites across 5 P-IDs (P-001/P-002/P-003/P-014/P-022), all contradicting
`coverage_matrix` + the catalog. No detector could propose it (both layout detectors key on NEW surfaces or
MOVED counts). Operator chose the full sweep. Labels only — no wireframe, state, token or lamp moved.
Record: `.andromeda/runs/2026-08-21T09-50-00-wrap/fanout-results.md`.

## Notes
- **v2-16 verified with BOTH contradicted sub-clauses refined** (the v2-15 sub-clause precedent; PREMISE-
  CORRECTION notes carry the measurements). The outcome is proven STRONGER than the original wording, so
  never-weaken holds: (a) the hard witness moved off `triage.incident.auto_resolve.tick` — its
  `resolved_count`/`evaluated_count`/`duration_ms` render `"<redacted>"`, so it proves the observer RAN, never
  that it resolved — onto the unredacted `incidents.list_active.request` `item_count` transition; (b) the
  read-back-absence clause is scoped to what leg E proves (`result_count: 0`), because on leg A read-back
  returned **2** incidents where the in-app ledger showed **1**.
- **Coverage 17/32 → 18/32 verified · 14 unclaimed.** v2-16 is this chunk's only claim.
- **3 SUT intake items recorded** (report §SUT intake — Pulse-side, deliberately NOT scoped into Conductor):
  the corpus/in-app active-list divergence after auto-resolve (NEW); `auto_resolve.tick`'s redacted counters
  (EXTENDS intake #7); the resolution summary being structurally unreachable under det-L4 (NEW, ratified at
  phase P5 — nothing constructs `DigestKind::ResolutionSummary` and the fixture pins the flag false).
- **35th audit PREREQ discharged in the PURE auto-satisfy form** the 34th predicted — the first such fire.
  `cargo audit` exit 1 with `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny` exit 0; lock byte-
  untouched. Record: `probe unchanged, 35th consecutive`. The 36th is pinned on the next entry.
- **Curation: T1 0 · T2 3 · T3 1** (filtered 2). `verification-harness.md` gained a new entry on reading a
  live Pulse capture (pre-leg line count is NOT zero — `pulse-app` logs ~2.2k boot lines before the receiver
  opens; cue lines carry no service identity so attribute by `persistence_seconds`; an aggregate count series
  cannot attribute a DROP, so pair against the EMPTYING). `testing.md` got two in-place extensions: the
  readable-field rule gained its "the witness MOVES" half, and the 2026-06-22 mixed-class family guard is
  retired (the family now carries no checks at all).
- **A stale doc comment survives in `baseline_harvest.rs`** ("under a FRESH data dir 0 is correct by
  construction") — falsified this chunk. The LESSON is curated; the comment itself is a one-line fix for
  whichever chunk next touches the harvest tests. Not route-pinned (no natural owner entry).
- **Last failed command:** none.
