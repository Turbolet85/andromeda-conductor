# Curation — 2026-09-24-architecture-registries-compacted-under-the-read-cap

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): + "gate.py delta --defer-check voids on a BASENAME, and --only will not fire a deferred entry" (confidence 0.8)
                                              + "A same-section anchor check cannot tell WHICH registration a rewrite restates" (confidence 0.8)
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  No-other-home: "gate.py delta --defer-check voids on a BASENAME, and --only will not fire a deferred entry"
  No-other-home: "A same-section anchor check cannot tell WHICH registration a rewrite restates"
  CLAUDE.md size: 137/200 · T1 62.9 KB
```

## Proof

- **defer-check basename + `--only`**
  - The void, as the tool printed it (implement run `2026-09-24T07-39-40-implement`):
    `gate.py delta --defer-check rust` printed
    `defer voided — .andromeda/runs/2026-09-24T06-20-18-phase/security.md read by crates/conductor-verify/tests/jsonrpc_line_bound.rs`.
  - Line 3 of that test is a doc comment naming `.claude/rules/security.md`.
  - The follow-up `gate.py run --only 8,9` printed `8 unit not run — defer (key)` / `9 lint not run — defer (key)`,
    `entries 9 · … not-run 9`.
  - Both entries were then driven by hand: nextest `1067 tests run: 1067 passed`, clippy exit 0.
  - Tier 3 rather than Tier 2: the harness-invocation class's home, `verification-harness.md`, and `testing.md`
    are both past the Read cap (handoff health note), and tiebreaker 2 prefers Tier 3.
- **Same-section anchor**
  - This wrap's faithfulness review: ledger row `31363eb4ccbf` (the `CONDUCTOR_MSEDGEDRIVER` "Unset … SKIP at
    exit 0" sentence) passed arm (f) with its anchor on the `CONDUCTOR_NVDA` bullet's identical phrase.
  - It was re-anchored to its own bullet before apply, and `check` then passed again.
  - Record: `fanout-results.md` §Owed faithfulness review.
