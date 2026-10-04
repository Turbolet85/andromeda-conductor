# Curation — 2026-10-03-p-075-re-round-on-incident-events

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "On the Linux dev host a `pulse-app` launch carries `WEBKIT_DISABLE_DMABUF_RENDERER=1` …"
    Proof: the round's first launch (default posture) died ~1.5 s in on `Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display` after `ui-bridge.ready`; the relaunch with the variable stayed up 23:03:25→23:21:49Z across five legs — chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/round-ledger.md §A Pulse finding.
  Corrections (cap-exempt):
    + testing.md 2026-06-27 entry: the mock-runtime `url` "MUST be `http://tauri.localhost`" corrected to Windows-only.
      Proof: `cargo nextest run --workspace --profile ci` on the Linux dev host failed the six `conductor-tauri` mock-runtime tests with `<cmd> not allowed. Plugin not found` (12 lines), identically on a clean worktree at `6a9ff7c` — report.md Outcome, gate 27.
    + session-learnings.md "The MCP read-back surface is 8 tools, not 4": corrected to 9 at Pulse S2.
      Proof: `retrieve_incident_events` pinned and read live (assertion 7 `[PASS]`); research.md §Pulse at S2 (the tool at `tools.rs:63`).
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred (2 recurrences → handoff)
  No-other-home: "On the Linux dev host a `pulse-app` launch carries `WEBKIT_DISABLE_DMABUF_RENDERER=1`"
  Extended: T2/testing.md: "2026-06-21: To integration-test a real CHILD process …" + "a feature-gated test target that no CI job or harness verb builds rots silently"
    Proof: `crates/conductor-verify/tests/preflight_spawn.rs` (`stub-server`) had not compiled since `CanaryMarker::new` gained a third argument at `b7bb6f8` (2026-08-16); gate 4 red until the companion fix — report.md Deviations.
  CLAUDE.md size: 138/200 · T1 47.8 KB, 9 over 600 B

Scores: the Linux launch recipe 0.4 (measured) + 0.2 (technical detail) + 0.2 (no other durable home: no master or route annotation carries a launch posture) = 0.8; the feature-gated facet 0.4 + 0.2 + 0.2 (no other home) = 0.8.
Recurrences (→ handoff, `recurrence-despite-learning`): host-win32.md 2026-09-08 (a `cd` moved the Bash session cwd three times this session, implement and wrap); CLAUDE.md 2026-08-21 token-proxy entry as extended 2026-09-02 ("sweep for what the claim SAYS, not what it is NAMED after") — a count-keyed sweep found 7 of 15 tool-set sites, the four-name enumeration sweep the rest.
