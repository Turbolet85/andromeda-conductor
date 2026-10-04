# Curation — 2026-10-04-host-portable-tauri-ipc-tests

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   corrected testing.md (2026-06-27 mock-runtime entry, gotcha 1) ·
                                              extended host-win32.md (2026-10-02 `grep -i` entry)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 1 task-specific · 0 conflict · 0 deferred (cap) · 1 recurrence → handoff
  Extended: T2/host-win32.md: "`grep -i` ABORTS on this host" + "ugrep refuses bounded-repeat + alternation patterns"

## Applied
- **Correction (cap-exempt)** — `.claude/rules/testing.md` 2026-06-27 entry, gotcha (1): was "the request `url` MUST
  be `"http://tauri.localhost"` … any other value (e.g. `tauri://localhost`) fails dispatch"; now "the dispatching
  window's own URL, `window.url()` — per-OS … an error-expecting helper must reject that text".
  Proof: the chunk's Linux gate run (`cargo nextest run -p conductor-tauri --profile ci` → 27/27 with `window.url()`)
  and the inverse control (the Windows literal restored on Linux → 19 passed / 8 failed, the three error-expecting
  callers failing on the ACL-refusal guard); tauri 2.11.3 `manager/mod.rs:339-346`. report.md §Spec claims disproved (b).
- **Extension** — `.claude/rules/host-win32.md` 2026-10-02 `grep -i` entry + the ugrep complexity-limit facet.
  Proof: this wrap's P1 master sweep — `grep -noE '.{0,60}(…|…).{0,60}'` over the seven masters printed `ugrep: error:
  error at position 187 … exceeds complexity limits` and no hits for that term; the python `re` re-run found the
  `architecture.md:214` site. report.md §Decisions & corrections, sweep hazard (1).

## Filtered
- dup — "restore the old implementation to prove a new guard discriminates" (the implement inverse control):
  testing.md 2026-09-09 as extended 2026-09-12 states it.
- dup — "measure a claim before writing it into evidence" (the `#[cfg(windows)]` count): CLAUDE.md 2026-08-09 entry.
- task-specific — the overseer's "anchor any diff-shaped probe to the chunk base `f5076ad`, not HEAD" (W182): a
  per-chunk directive the pipeline's own gate tools already enforce (gate.py scope / cascade.py read the pre-CI
  parent); the overseer's ledger carries it.

## Recurrence → handoff
- recurrence-despite-learning: host-win32.md 2026-09-08 (the Bash tool's cwd persists) — a `cd {run_dir}` at the head
  of a probe moved the session cwd this wrap (the second session running; deferred at the last wrap too).
