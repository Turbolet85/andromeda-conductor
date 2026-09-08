# Session Handoff

**Last Updated:** 2026-09-08T08:11:58Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **2 ahead and unpushed** —
last session's chunk commit plus this session's setup-project commit; this wrap's bookkeeping commit
makes it **3**. The push is the operator's act, and it will NOT turn CI green: the a11y job is
correctly wired and correctly RED, owned by the route entry *Hosted-runner WebView2 session*.)
**Status:** clean
**Last Commit:** `chore(session): no chunk wrapped — session 127` (preceded by
`chore(setup-project): configure Claude Code for Conductor`, `a25a425`)

## Position
- Done: **no chunk this session.** A `/andromeda-setup-project` re-run absorbed a pipeline-template
  change; master-route carries **0 pending** and is unchanged.
- Next: **`/andromeda-phase`** on the first markerless head — **_Hosted-runner WebView2 session — the
  a11y job's first green run (v2-24 claimed or deferred)_** (`working-route.md:125`). No
  annotation-position `BLOCKED-ON` (the two hits on that line are backtick-quoted prose late in the
  CARRY, and the entry states the flag is deliberately not re-raised), so phase will not halt. The
  sibling is *Release build and bundle* (`:127`).
- Coverage **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`) — unchanged; no
  capability was claimed or flipped this session.
- **Evolve:** Epoch 6b at 10 chunks (8 frozen + 2 markerless) — still at the ~10 split threshold;
  surfaced, the split remains the operator's call.

## Work done
`/andromeda-setup-project` re-run to absorb the CLAUDE.md template no longer importing
`.andromeda/master-route.md`. The net change is **two lines**: the `GENERATED:setup:imports` block goes
3 → 2 (`@.andromeda/architecture.md` + `@.claude/session-handoff.md`), and the template's maintainer-note
comment is now carried inside `GENERATED:setup`. Navigation to the route survives through the
pointer-table row, which already complied with the no-baked-version-path rule.
`USER:session-learnings` verified byte-identical (44 488 B, matching SHA) — spliced by anchored edit,
never retyped. CLAUDE.md **135/200**. Validation **14/14 checks, 0 warnings**; hook smoke **6/6**.

Three things measured rather than assumed: `settings.json` re-renders **byte-identical** to the
2026-09-05 hooks-matrix (both PreToolUse guards compare IDENTICAL to its literal JSON, the formatter
matches the Rust prologue, clippy-at-write-time absent) — so no write was performed; the code-graph
pipeline is byte-identical to its templates, so no drift and no views-rebuild is owed; and **no other
GENERATED block differs** — all seven re-derived unchanged from their masters, meaning no cascade gap
and no second template change to absorb.

## Drift resolved
**None — no fan-out ran.** The 0-pending path runs no report and no detectors, so P2 did not execute.
Checked explicitly under that path's own carve-out (amendments whose subject is a fact THIS wrap
measured): this session measured `.claude/` materialization and hook behaviour, which no `.andromeda/`
master states — so there was nothing to amend, and no spec↔reality divergence was left standing.

## Notes
- **Judgment call worth knowing about:** the setup re-run did **not** regenerate `.claude/rules/`,
  `.claude/docs/` or `code-reviewer.md` from their templates. Those templates are generic scaffolds; the
  shipped leaves are project-specific renders. A verbatim re-render would have deleted real content —
  most sharply `verification-harness.md`, whose template prescribes a daemon `boot`, a PID file and a
  heartbeat, against a shipped rule that explicitly says *"Conductor has NO daemon … do not reintroduce
  daemon/PID/endpoint machinery"*. Curated to Tier 3. The hazard is that **no written preserve rule
  covers this class** (`USER:*`, `## Session Additions` and the agent-run scripts each have one; these
  three do not), so the judgment must be made deliberately on every re-run.
- **`agent-run.{sh,ps1}` are drifted from their templates and were kept** (18 KB vs a 3.7 KB template
  body; 21 KB vs 4.3 KB). The evolved scripts are the truth. Backed up alongside CLAUDE.md and
  code-reviewer under `.claude/backup/*.pre-setup-2026-09-08T07-44-00`.
- **Retraction, recorded in the run's validation log:** my first hook smoke test reported all five arms
  failing with `jq present: False`. That measured the PROBE — Windows-native Python spawned a `bash`
  with a different PATH and no `jq`, so every hook took its `command -v jq || exit 0` guard. Re-run in
  the shell the runtime actually uses, all five pass and the formatter genuinely reformats through its
  stdin path.
- **Curation:** T1 0 · T2 1 (`host-win32.md` — the bash↔native boundary changes paths AND environment,
  confidence 0.9) · T3 1 (the `.claude/` leaves are tailored renders, confidence 0.8 via the
  `no-other-home` signal at the exact-0.6 mass point). Filters: 0 dup · 0 task-specific · 0 conflict ·
  0 deferred-by-cap. `CLAUDE.md` **135/200** (no Tier 1 entry this wrap).
- **Last failed command:** none.

## Deferred learnings
1 finding was not applied as a new entry:
- **`recurrence-despite-learning`:** "validate a diagnostic form where the real path PASSES before
  building on what it says when it fails" was minted in CLAUDE.md one session ago, and this session
  reproduced the shape anyway — a smoke-test probe authored and run cold, suspected only after it
  false-red. The corpus entry is correct, so this is a recurrence, not a duplicate and not a
  correction; per the curation contract a third entry is not the remedy, so it is logged here and in
  the friction stream (`recall.corpus-recurrence`) for the pipeline's owners. **Second consecutive
  wrap carrying one** (the prior was the nextest-filter rule). It fired late — at diagnosis rather than
  at authoring — but did catch it before any written claim, unlike the instance before it, which
  reached a committed matrix note.
