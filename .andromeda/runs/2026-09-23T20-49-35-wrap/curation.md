# Curation — 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "Before a plan names a test fixture framework for a crate's tests, confirm that crate declares it" (confidence 0.8)
      Proof: operator directive at /andromeda-implement invocation. rstest is absent from crates/conductor-run/Cargo.toml [dev-dependencies], and adding it would have turned the plan's `git diff --quiet HEAD -- Cargo.lock` gate red. Plain #[test] shipped, and capture_paths_guard ran 8/8 under both runners. Score: operator correction +0.4, verified by measurement +0.4.
    + host-win32.md: "A multi-file count probe reads GREEN when a file is missing" (confidence 0.8; one sentence, 277 B, under the always-loaded ~600 B bar)
      Proof: the P5 baseline in phase run 2026-09-23T19-58-21. The unguarded host-path hygiene probe read exit 1 / last line 0 with both scanned files absent (pipefail reported stage 2's exit 1 over stage 1's exit 2), and the test -f guard fixed it in the plan. Score: verified by measurement +0.4, specific technical detail +0.2, no-other-home +0.2 (report Decisions only; no route / master / playbook home).
  Tier 3 (.claude/docs/session-learnings.md):
    + "The code graph does not index feature-gated test files" (confidence 0.8)
      Proof: in phase run 2026-09-23T19-58-21, tree-query trace rows 12/19/1. runs_dir / pulse_log / capture / journal_of in the live-pulse-gated files resolved only to collisions (conductor-tauri commands.rs:45, conductor-core obs.rs tests), and callers were settled by grep. Score: verified +0.4, detail +0.2, no-other-home +0.2. It is an additive facet of CLAUDE.md's 2026-08-08 code-graph entry, re-homed to Tier 3 because that Tier-1 entry is far past its cap.
  Corrections (cap-exempt):
    ~ .claude/rules/verification-harness.md (2026-06-27 entry, "Extended 2026-08-16" clause): "under deterministic L4 **no** read-back field varies … there is none" gets a bracket correction, true at efabe8e only.
      Proof: this wrap's P2 cascade sweep found the clause uncorrected, the one hit of the retired claim family left standing in a curation home after this chunk corrected ten code sites. architecture.md:93 records the 83d4060 fingerprint_refs union, measured in the 2026-09-10 envelopes.
  Filters:
    - 0 dup
    - 2 not candidates (pipeline-tool telemetry, routed to the friction log instead: gate.py reading `${NAME}` as an env handle, and the late fan-out audit trail)
    - 0 conflict
    - 0 deferred
  Load-bearing: none
  No-other-home: "A multi-file count probe reads GREEN when a file is missing" · "The code graph does not index feature-gated test files"
  CLAUDE.md size: 137/200 · T1 unchanged (no Tier-1 write)
