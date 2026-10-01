# Curation — 2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  + "The operator pass's pre-CI commit and guarded push stay the OPERATOR's acts when the agent performs them on the operator's explicit word …" (confidence 0.7)
  Proof: the overseer's word at this chunk's operator pass ("ON MY WORD you make the operator pre-CI commit with that
  subject and run the guarded push yourself, as in every earlier chunk") and the wrap relay §1 correcting the
  implement report's framing ("Your deviation 3 is the operator's word, not a skill bypass"); signals: explicit user
  correction +0.4, the every-chunk convention +0.3.
  Tier 2 (.claude/rules/*):                   correction (cap-exempt) — verification-harness.md 2026-09-23 "Pulse logs NO line when its model DISMISSES a digest" → `[corrected 2026-10-01: true to Pulse fcc31b2 only …]`
  Proof: the 2026-10-01 series' captures print 15 `interpretation.incident.skipped` lines across the three leg windows
  (d1 4, d2 6, d3 5 — `skip_reason` `no_cue` 14, `model_resolution_summary` 1; evidence `rm-capture-d{1,2,3}.txt`),
  Pulse `a2addb3` `pulse-app/src/inference_runtime.rs:96,955-964` (the report's Spec claims disproved bullet).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 2 below threshold · 1 recurrence (→ handoff)
  - below threshold (exactly 0.6, rejects): the cross-drive span-identity replay (`append_failed` when a seeded
    identity is re-sent inside Pulse's buffer window) — measured +0.4, technical detail +0.2; its home is the minted
    route entry "Per-run span identity in the real-model harness", which carries the fact, so neither conditional
    +0.2 applies.
  - below threshold (exactly 0.6, rejects): `elide_fingerprints` keeps an all-digit run, so an all-digit fingerprint
    prefix passes the chain — measured +0.4, technical +0.2; amended into security-plan this wrap (P2), so
    no-other-home does not apply.
  - recurrence-despite-learning: `.claude/rules/security.md` 2026-09-30 ("A synthetic leak control a gate needs as
    its known positive … lives in the gitignored `.andromeda/cache/`") — a phase run again left a hygiene-refusable
    file in its committed run dir, this time the gate tool's own dry-run TRANSCRIPTS (real host paths in its header);
    moved to the cache on the overseer's word. The remedy is a check in the phase step that writes the transcript.
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 over 600 B
