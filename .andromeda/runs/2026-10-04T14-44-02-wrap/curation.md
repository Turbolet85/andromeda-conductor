# Curation — 2026-10-04-second-test-surface-corrective

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "`gh run view --log` can drop the tail of a very large job's log — read a CI leg's verdict from that job's own log …" (confidence 0.8)
  Proof: CI#37209452847 — the combined `gh run view --log` capture's a11y routine-arm section ended mid-leg on a WebDriver `execute/async` POST, carrying no `Spec Files:` / `passing` / `skipped` line; the same job's log through `gh api --allow-escape-sequences repos/Turbolet85/andromeda-conductor/actions/jobs/111457469064/logs` (49 000 804 B) carried `19 passing` · `2 skipped` · `Spec Files: 1 passed, 1 total` and `[a11y] verdict asserted - 0 failed | 2 skipped (expected 2) | driven session present`; without the flag the API call refused ("the response contains terminal escape sequences").
  Tier 3 (.claude/docs/session-learnings.md): + "tokei's reading of a `tests/*.rs` file can be nearly all code, so a size forecast scaled from it overshoots" (confidence 0.8)
  Proof: `tokei -o json` over `git show dab66dc:crates/conductor-run/tests/delegated_timing_harvest.rs` → 1130 code / 0 comments / 12 blanks (1222 raw lines); over the five split files → 872 / 50 / 95, root 310 against the plan's ≈490 forecast; the normalized line-multiset comparison (scratchpad `tokcheck.py`) differed only by wiring, `//!` lines, `use super::*;` and one rustfmt re-wrap.
  Filters: 0 dup · 2 task-specific (the `synthetic` string-literal grep hazard; the sha256 restore check — one-off) · 0 conflict · 0 deferred · 1 below threshold (the Linux-host form of the 2026-10-02 flycheck-stop rule — an additive facet at 0.5)
  No-other-home: "tokei's base reading …" · "`gh run view --log` drops a large job's tail …"
  CLAUDE.md size: read at P7 (health check 1)
