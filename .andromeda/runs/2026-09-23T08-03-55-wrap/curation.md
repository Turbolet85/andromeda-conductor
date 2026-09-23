CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none new · 1 correction (below)
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "Pulse logs NO line when its model DISMISSES a digest — the witness is a parse `ok` with no `incident.created`, and `cue_present` separates the cue-bearing digest" (confidence 0.8)
                                              + testing.md: "A capture test that prints under `cargo test -q … -- --nocapture` must buffer its output and write it ONCE" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none new · 1 correction (below)
  Corrections (cap-exempt, measurement-backed, edited in place with a dated tag):
    - verification-harness.md Session Additions (the 2026-08-16 fingerprint entry) ×2 clauses — "no read-back field varies" / "no read-back surface at any width"
    - verification-harness.md Session Additions (c) — "real-model formation (~110s measured)" → unmeasured (the plan's named Tier-2 correction)
    - host-win32.md body, *Encoding & heredocs* — "The quoted heredoc itself is sound below the cut" → except backslash pairs
    - CLAUDE.md USER:session-learnings (the 2026-08-09 entry's 2026-08-16 PROVENANCE extension) — "the real computed value reaches no read-back surface at all"
    - session-learnings.md (the 8-tool read-back surface entry) — "Pulse's computed fingerprint reaches no read-back surface at all"
  Via the P2 cascade, not curation (generated bodies re-derived from their masters): the plan's other two Tier-2 items — `verification-harness.md:19` (`--live` gains the `real-model` selector) and `security.md`'s READ SET sentence (the real-model absence arm).
  Filters: 1 dup-recurrence · 0 task-specific · 0 conflict · 0 deferred by cap
  Recurrence-despite-learning (→ handoff): the `cd`-persists entry (host-win32.md Session Additions, 2026-09-08) — a `cd` in a compound command re-based later calls, in implement AND three times in this wrap; the COMPLETION axis (CLAUDE.md 2026-08-09 entry, 2026-08-31 extension) — A17 reached validate framed "code reading only, not live-measured" while committed evidence already measured it.
  Load-bearing: "Pulse logs NO line when its model DISMISSES a digest" → Diagnostic-quality cluster off the drift pin
  No-other-home: "A capture test that prints under `cargo test -q … -- --nocapture` must buffer its output and write it ONCE"
  Extended: none
  CLAUDE.md size: 137/200 · T1 48.5 KB, 9 over 600 B

## Proofs

- **Tier 2 · verification-harness.md — the Dismiss witness.**
  Proof: the drive's B2 classification (report §Decisions & corrections): Pulse's own log at 07:40:25Z — `digest.runtime.cadence_tick {cue_present: true, mode: tier1}` + `digest.assemble.request {cue_kind: retry_storm, cue_priority_tier: autonomous}`, then `interpretation.inference.request {result: success, duration_ms: 4379, model_tier: primary}` and parse `ok`, and NO `interpretation.incident.created`; every other digest in the window cue-less tier-3. The creation predicate's silent exits (`is_resolution_summary` / `Decision::Dismiss` / `L4Severity::None`) read at Pulse `83d4060` (report §Cross-project claims). Signals: verified by measurement +0.4 · specific technical detail +0.2 · load-bearing for "Diagnostic-quality cluster off the drift pin" +0.2 = 0.8.
- **Tier 2 · testing.md — buffer a `-q --nocapture` capture.**
  Proof: report Deviation 4 — the pre-drive no-Pulse dry run showed libtest's `i` mark for the ignored `rule_record` prefixing the capture's first line under B5's fixed command; fixed by the `FlushOnDrop` guard in `crates/conductor-run/tests/real_model_live.rs`, and the live capture's grammar lines parsed clean (harvest 39/39). Signals: verified by measurement +0.4 · specific technical detail +0.2 · reached no other durable home +0.2 = 0.8.
- **Corrections — the fingerprint read-back (three entries).**
  Proof: `conductor-0.2.0/chunks/2026-09-10-release-build-and-bundle/evidence/leg1-2026-09-10T19-32-57-092.jsonl` — the envelope's `fingerprints` carries `0bddf438a748f326f07436169ea20a6a` beside the `det-*` triple, equal to that run's own self-obs line `canary fingerprint computed 0bddf438…` (`…selfobs.jsonl` line 9); ten committed 2026-09-10 envelopes carry such a 32-hex value; mechanism `grounded_fingerprint_hashes` at Pulse `83d4060` (report §Cross-project claims). Master amended at P2 (architecture A17–A19, operator "Amend now, as measured").
- **Correction — ~110 s.**
  Proof: report §Spec claims disproved 2 — the figure is `crates/conductor-run/tests/lifecycle_live.rs:20` under `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` (`conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md:161`), re-attributed at `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/plan.md:102`; the 2026-09-23 drive measured no formation.
- **Correction — the quoted heredoc and backslash pairs.**
  Proof: three payloads collapsed `\\` → `\` inside a quoted heredoc sent through the Bash tool — implement: a `\\n` in a Python heredoc planted real newlines in two Rust string literals, and `grep '\\'` died with "Trailing backslash" (report §Decisions & corrections (a), (b)); this wrap: `split('\\')` in a `python - <<'EOF'` heredoc raised `SyntaxError … split('\')`, and a backslash-bearing `grep` spot-check read back 0 while a script-file check found the text intact. A quoted heredoc gives bash no escape processing, so the collapse is upstream of bash.
