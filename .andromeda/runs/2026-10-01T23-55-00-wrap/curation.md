# Curation — 2026-10-01-per-run-span-identity-in-the-real-model-harness

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  (none)
  Tier 2 (.claude/rules/*):                   + testing.md: "A grep for a span NAME over a Conductor self-obs journal also matches every CHILD line …"
  Tier 3 (.claude/docs/session-learnings.md): (none)
  Correction (cap-exempt):                    testing.md 2026-06-18 entry — "The seed governs span identity + timing" [corrected 2026-10-01]
  Filters: 4 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below threshold
  No-other-home: "A grep for a span NAME over a Conductor self-obs journal also matches every CHILD line"
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 over 600 B
```

## Applied
- **Correction**, `.claude/rules/testing.md` (Session Additions, the 2026-06-18 seed/fingerprint entry). "The seed governs
  span identity + timing" was made FALSE on the production path by this chunk.
  - Proof: `conductor-0.3.0/chunks/2026-10-01-per-run-span-identity-in-the-real-model-harness/evidence/revert-red.md`. With
    the re-key removed, two same-seed salted drives share every identity (union 13 vs 26). Restored, they share none.
  - Proof: `evidence/witness-ledger.md`, the live witness PASS on Pulse `a2addb3`.
  - It is a measured correction, so it is cap-exempt (curation-guide §Corrections). Its cascade-sweep row is
    `.claude/rules/testing.md:51`, routed here from P2.
- **Tier 2**, `.claude/rules/testing.md`: the span-name sweep hazard. Confidence: +0.4 (verified by measurement; it
  changed the design, because the witness keys its start line on `span` + `span_event`) +0.2 (specific technical detail)
  +0.2 (no-other-home: carried by no route annotation, master or playbook rule) = 0.8.
  - Proof: `runs/live-suite/rm.jsonl`, read this session. `grep -m2 '"timeline.execute"'` returned the
    `timeline.execute` span-`new` line AND an `emit.batch` child line carrying `"parent":"timeline.execute"`. The
    witness `crates/conductor-run/tests/span_landing_live.rs` keys on `span == "timeline.execute"` and
    `span_event == "new"`, and asserts exactly one such line per journal.

## Filtered
- **dup:** operator-word pre-CI commit (CLAUDE.md T1 2026-10-01).
- **dup:** advisory-db untracked residue (`security.md` 2026-08-09 as corrected 2026-09-05). The porcelain probe caught it
  as designed, so this is the check working, not a recurrence.
- **dup:** Bash-guard transports for a doubled backslash and a cat-heredoc file target (`host-win32.md` Transports /
  Encoding).
- **dup:** pulse-app launch / stop recipe (auto-memory `agent-may-launch-pulse-app-when-directed`).
- **below threshold:** a link `exit code: 143` under host CPU contention is the gate bound's SIGTERM, not a toolchain
  defect. Score 0.4 measured + 0.2 detail + 0.2 no-other-home − 0.3 one-off = 0.5.
