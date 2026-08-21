# Session Handoff

**Last Updated:** 2026-08-21T17:22:30Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **32 ahead** after this wrap commit)
**Status:** clean
**Last Commit:** `chore(route): operator-requested adaptation — 0-pending wrap` (preceded by `3db219b`
setup-project re-run)

## Position
- Done: **no chunk** — this was a 0-pending adaptation wrap. The substantive work of the session was the
  **`/andromeda-setup-project` re-run (`3db219b`)**, which found and closed a live tooling break and 13
  distillation-tier cascade gaps. `2026-08-21-per-check-latency-measurement` remains the last completed
  chunk; **v2-19 verified · coverage 19/32 · 13 unclaimed** (all unchanged — this wrap claimed nothing).
- Next: **`/andromeda-phase` on "Delegated timing budgets proven"** (P-025/P-027/P-037/P-045). **The
  external block is CLEARED** — expect a NORMAL promotion, not the Setup HALT the previous handoff
  predicted. Its coordinates are the four metric names in the entry's `CONTEXT` annotation. Then
  **Operator-pause and checklist live firing**; those two close **Epoch 4** → the boundary stack
  (`/andromeda-evolve-diagnose` + code-audit trend run #2).

## Context correction (supersedes the prior handoff — it predated both events)
- **The Pulse visit is DONE.** Pulse chunk `2026-08-21-delegated-timing-observables` (Pulse `624e26a`)
  shipped the three missing timing observables; Pulse then absorbed the code-graph template (`f0c38f5`)
  and parked. Next there: PII scrubber recall. Coordinates verified at Pulse `f0c38f5` / Conductor
  `3db219b`.
- **The standing `BLOCKED-ON` is therefore retired**, exactly as its own text prescribed
  ("clear when those observables ship"). The prior handoff's "Expect a Setup HALT" is void.

## Work done — the setup re-run (`3db219b`)
**A live tooling break, found via a dead pointer row.** `scripts/code-graph.py` was the original 196-line
single-plane version resolving `.andromeda/cache/tree.db` — a path that no longer exists, since the cache
moved to a per-plane layout (`cache/rust/`, `cache/ts/`). Every `query` would have found no DB, declared it
stale, and **silently rebuilt in the retired flat layout**. Its own sentinel proved the drift: a two-line
per-plane format the shipped script cannot produce. Updated all three drift-tracked files from template
(backed up; cookbook tail preserved) and proved it end-to-end — both planes query at `db_state: fresh`.
Health check 11 verifies PRESENCE only, and no check covers pointer rows at all, so nothing could have
caught it.

**13 cascade-gap corrections**, each a leaf diverged from a CORRECT master: `conventions.md` ×5 (crate
count 8→9, rmcp→hand-rolled JSON-RPC, timestamp storage stated INVERTED from arch, three-table `runs.db`,
`[phases.fault]` + `budget_ms`) · `gotchas.md` ×2 (rmcp negotiation, `workspace_root`→`workspace` column) ·
`commands.md` (cleanup: three tables) · `tests-summary.md` (`run_check`/`CheckRecord`/`budget_ms`) ·
`workflow.md` (version-agnostic route path) · **`verification-harness.md`, `testing.md` and
`code-reviewer.md`, which named `rmcp` — absent from `Cargo.toml` and `Cargo.lock` — instead of the real
`stub_pulse_mcp`.** That last group is the sharpest: three auto-loading Tier-2 surfaces pointing agents at
a crate not in the tree.

## Drift resolved
**N/A — no fan-out.** This is the 0-pending path: P1 (report), P2 (drift detection) and the P7 gates do not
run, because there is no chunk to detect drift against. The doc-tier reconciliation that WOULD have been
P2's work was performed at the setup re-run instead, against the masters directly.

## Route edits (P5 — 3 operator-requested adaptations, all factual→AUTO)
1. **External block CLEARED** on *Delegated timing budgets proven*, replaced by a `CONTEXT` annotation
   carrying the chunk's coordinates: `metric.report.render_ms` (P-037, pre-existing) ·
   `metric.constellation.hue_update_ms` (P-025) · `metric.constellation.discovery_ms` (P-027) ·
   `metric.findings.counter_refresh_ms` (P-045) — each a `telemetry.frontend` procedure behind an EXACT
   allowlist leaf, firing proven by Pulse webview tests (positive + paired negative). Evidence: cite the
   Pulse chunk's report + its P-075 `notes`; never copy.
2. **`PREMISE` annotation** on the same entry (research premise, not a task): the Halo State Pulse canvas
   is **ORPHANED** at Pulse HEAD (zero non-test JSX, verified twice) — the severity→hue semantic lives on
   the **constellation DOT**, and `hue_update_ms` measures THAT, while `halo-hue-encoding.toml:27` still
   grades the hue by operator observation in halo wording. Expect a premise refine at the chunk; the
   capability semantic is UNCHANGED. **The canvas's fate is Pulse's own entry — never scope it here.**
3. **Stale-count CARRY extended** on *Dependency polish*: `test-plan.md:25` also says "8 workspace crates"
   — a **third** site the CARRY (naming `obs-plan` :25/:645) would have left stale. Surfaced at the setup
   re-run.

**The 37th audit PREREQ stands byte-unchanged** (verified: signature + `RUSTSEC-2026-0244` intact). `h2`
here is already 0.4.16, so Pulse's new 8th owned ID (RUSTSEC-2026-0258) never enters this repo's set —
expect the probe signature byte-identical.

## Notes
- **Code-graph: first live in-wrap MULTI-PLANE build, and it worked.** Two-line sentinel as predicted:
  `rust ok 27s 2196/10179` · `ts ok 1s 247/377`. No SKIP lines, no `.refresh-stale`. Querying now REQUIRES
  a plane argument (two are detected): `code-graph.py query <run_dir> <marker> "<sql>" <rust|ts>`.
- **Curation: 2 Tier-1 EXTENSIONS, 0 new siblings.** (a) the citation-attribution entry gained an
  OWNERSHIP axis — when a distillation and its master disagree, attribute before fixing, because the two
  stale kinds need opposite treatment; (b) the verify-the-artifact entry gained the presence-vs-currency
  facet — a check can pass because it verifies the wrong property. Filtered: 1 better-homed (per-plane
  query mechanics belong in `scripts/code-graph-cookbook.md`, already updated), plus task-specific
  symbol counts / SHAs. **No conflicts, no deferrals.** CLAUDE.md 130/200.
- **Deliberately NOT corrected** (leaves faithfully inheriting owned-stale masters): `obs-summary.md`
  "8 crates" (CARRY-owned) and `tests-summary.md` "rmcp stub" as a §8 mocking fact (route-owned). Fixing a
  leaf ahead of its master mints a fresh divergence; they self-heal at the next cascade.
- **Watch at the next promotion:** the cleared entry no longer contains the block token anywhere in the
  file (verified 0 occurrences) — a near-miss this wrap was naming the token in prose, which would have
  false-HALTed phase Setup on the very next promotion.
- **No SUT intake this wrap** — the queue stays 13 + one extension.
- **Last failed command:** none.
