# Curation — 2026-10-02-captured-fingerprint-values-elided

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-win32.md: "`grep -i` ABORTS on this host (exit 134) — count case-insensitively in python" (confidence 0.8)
                                              + host-win32.md: "Before a heavy cargo step, stop THIS repo's rust-analyzer flycheck cargo tree by PID, re-reading PIDs" (confidence 0.7)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 1 task-specific/pipeline-owned · 0 conflict · 0 deferred (cap) · 2 recurrence-despite-learning (→ handoff)
  No-other-home: "`grep -i` aborts on this host (exit 134)"

## Applied

- **host-win32.md — `grep -i` aborts.**
  - Proof: wrap P1, the per-master site count. `grep -ciF 'all-digit' .andromeda/security-plan.md` returned
    `Aborted`, rc=134, reproduced bare on a known-present token. A loop over `$(grep -ciF …)` printed blank fields
    for every master with no error.
  - Signals: verified by measurement +0.4, specific detail +0.2, no-other-home +0.2 = 0.8.
  - No-other-home basis: no master amendment or route annotation carries it, and the next promotable entry ("Version
    close on measured evidence"; the two ahead of it are BLOCKED-ON) does not need it.
  - Filter 1: a new facet against the body's `grep -P` clause, which is template text, so it lands as a NEW
    `## Session Additions` entry. The file has no `paths:`, so it was held to the Tier-1 bar: one sentence,
    ~210 B.
- **host-win32.md — the flycheck stop.**
  - Proof: an overseer directive this session ("Before each heavy cargo step stop the rust-analyzer flycheck cargo
    tree by PID (re-read PIDs)").
  - The implement census applied it twice. Both readings found this repo's flycheck tree had already exited, and
    other projects' cargo and mutants trees (viola, andromeda-pulse) were present and left alone.
  - Signals: explicit operator directive +0.4, "each"-language +0.3 = 0.7.
  - Filter 1: `flycheck` / `rust-analyzer` have 0 hits across CLAUDE.md, the rules and session-learnings.

## Rejected
- **"Anchor diff-shaped probes to the chunk base, never HEAD, because the operator pre-CI commit moves HEAD"** (an
  overseer directive): pipeline-owned (the directive cites W182, Andromeda's own plan-authoring item). Its home is
  the pipeline's plan template, not this project's rules.

## Recurrence-despite-learning (→ handoff Deferred learnings)
- **`host-win32.md` 2026-09-08, "the Bash tool's working directory PERSISTS across calls".** A `cd` at the head of
  a call moved the session cwd twice this wrap (into `.andromeda/`, then into the run dir), so later relative paths
  resolved against the wrong base until re-anchored.
- **The CLAUDE.md token-proxy entry, as extended 2026-09-06 ("a pattern WIDER than the one that reads
  naturally").** A conductor-0.2.0 census keyed on `fingerprint_hex[=": ]+hex` counted 13 files where the truth is
  14, because one file separates key and value with a backtick. Caught by cross-checking against scope.md's
  enumeration.
