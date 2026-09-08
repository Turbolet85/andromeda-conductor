# Session Handoff

**Last Updated:** 2026-09-08T17:07:54Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **1 ahead and unpushed** at wrap
start — `bf7da06`, the setup-project commit. This wrap's bookkeeping commit makes it **2 ahead**. The push
is the operator's act, and it still will not turn CI green: the `a11y` job remains correctly wired and
correctly RED, with the cause measured at the prior chunk.)
**Status:** clean
**Last Commit:** `chore(session): no chunk wrapped — session 129` (this wrap)

## Position
- Done: **no chunk** — 0 pending in master-route. This session ran `/andromeda-setup-project` (CE-2) and
  then this 0-pending wrap; neither promotes or completes a chunk.
- Next: **`/andromeda-phase`** on the first markerless head — **_WebView2 runtime 152+ installed in-job —
  the one variable every passing case shares and the failing case lacks (v2-24 claimed or deferred)_**
  (`working-route.md:127`). It carries `PREREQ: close rust gate deferral (deferred since
  2026-09-08-hosted-runner-webview2-session)` — that chunk MUST run `cargo nextest run --workspace` +
  `clippy` as mandatory gates. No annotation-position `BLOCKED-ON`, so phase will not halt. The sibling is
  *Release build and bundle* (`:129`).
- Coverage **28/32 verified · 4 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-27`) — unchanged; nothing was
  claimed this session. `v2-24` deliberately stays `planned` and pooled.
- **Evolve:** Epoch 6b at **11 chunks** (9 frozen + 2 markerless) — still past the ~10 split threshold. The
  split remains the operator's call.

## Work done
Two commits, **zero source delta** (no `crates/`, no `scripts/`).

`bf7da06` — `/andromeda-setup-project` re-run absorbing pipeline-template change **CE-2**. CLAUDE.md now
imports `.claude/session-handoff.md` ALONE; `architecture.md` joins `master-route.md` as explicitly-read.
Four surfaces changed, 3 insertions / 3 deletions: the imports block, the architecture block's
"Primary source" line (**required** by the drop — "imported below" would otherwise be false), the
maintainer note, and one new pointer-table row naming §Infrastructure Patterns / §Occupied Resources
(21 → 22 rows). 135 lines. Everything else preserved as at `a25a425`: 7 rule files, 11 docs, both
project-evolved harness scripts, code-reviewer, hooks, .gitignore, every seeded artifact. Validation
14/14 health checks, hook smoke 6/6 (both guard directions probed). Template drift across all five
code-graph files: **zero**, byte-compared.

This wrap — bookkeeping only. No report, no fan-out, no master flip, no coverage flip, no route edit.

## Drift resolved
**None — and none was owed.** The 0-pending path runs no fan-out. Its carve-out (facts this wrap itself
measured) was checked properly rather than waved through: the session's one candidate fact is that
CLAUDE.md no longer imports architecture.md. A bare `CLAUDE.md` token sweep of all 7 masters returned 0,
which is exactly the proxy check this project's learnings distrust, so it was widened to six alternate
wordings — that returned **9 hits, all read, all false positives** (Tailwind/Rust `import` tokens;
citations of two unchanged rule files). `security-plan.md:6` ("setup-project materializes `@` imports")
stays true: setup still materializes them, one instead of two. **No master contradicted.**

## Notes
- **Curation: T1 0 · T2 1 · T3 0** (filtered 5 — 4 duplicate, 1 task-specific). The applied entry is a
  **correction**, not an addition: `.claude/rules/host-win32.md`'s generated body says a `cd` in a compound
  command "does not persist"; measured this session, the Bash tool's cwd **persists across calls**, and the
  hazard is inverted — a *remembered* cwd silently rebases the next call's relative paths, which is exactly
  how it bit (a following call resolved `.andromeda/master-route.md` one level too deep and failed as a
  missing file). Written to `## Session Additions` rather than fixed in place, because the false clause is
  setup-project **template** text a re-render would restore.
- **Four curated rules were consulted and held this session** — the bash↔native boundary rule (hook smoke
  6/6 from the Bash tool's own shell), the token-proxy rule (the 9-hit sweep above), the write-time clippy
  ban (the hooks-matrix default was NOT restored — that rule named this exact scenario), and the
  setup-re-render rule (Tier 2/3 validated, not re-rendered). Recorded as evolve *signals*, not friction.
- **Two pipeline defects logged to the friction stream** for the pipeline's owners, neither actionable
  here: curation-guide leaves the *correction-against-a-generated-body* case unowned between two colliding
  rules (its Corrections section says edit in place; its Tier 2 write logic says never touch that region;
  Filter 1's carve-out covers only the additive case), and the shipped rules **template** carries the false
  `cd` mechanic corrected downstream.
- **Last failed command:** none.

## Deferred learnings
None. No candidate hit the Filter 5 cap (1 applied), and no `recurrence-despite-learning` was owed — the
one candidate with that shape is dispositioned as a §Corrections correction instead, per the type's own
disambiguation rule. This ends a three-wrap streak of carrying one.
