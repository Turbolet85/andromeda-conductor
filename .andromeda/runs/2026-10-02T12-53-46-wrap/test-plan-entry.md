
## 2026-10-02-p-075-assert-round-against-pulse — the P-075 round leg and graders; `degraded_mode` per-read-back
**Section:** §2 Test Strategy → Deterministic · §5 → `mark_incident_resolved` · §6 Fingerprint-storm → Verification signal · §9 → Live-Pulse scenarios
**Change:**
- §2:
  - The real-wall-clock operator-local exception list gains `p075_round_live` (2026-10-02). It drives one `canary_spec` storm against one live Pulse and reads back through the four registered MCP tools, resolving through the shipped `probe_resolve_lifecycle`. It compares the emitted fingerprint in-process, prints only integers, booleans and closed words, and adds no `agent-run` verb or selector. Its grades sit at the harvest tier over digest-pinned evidence (`p075_round_assertion_*`).
  - The `span_landing_live` description gains its `runs/span-landing/` journals, cleared by name before drive A, and its stale-pair refusal.
- §5: `mark_incident_resolved` was re-graded at Pulse S `03ec944`, giving `ProvenByLiveness` again (idle 12 ms) through the unchanged probe and `attribute_by_liveness`. This is held by `lifecycle_harvest::p075_round_assertion_2_runtime_state_fidelity`.
- §6: the token checks stay declare-only.
  - Was: "because `retrieve_report` is permanently `degraded_mode` in this mode". Now: retired 2026-08-18, when the read-back was degraded. `degraded_mode` is per-read-back and never mode-wide.
  - Evidence: an incident read back `false` under deterministic L4 at S (`evidence/p075-leg.txt`).
  - Added: since 2026-10-02, `lifecycle_harvest::p075_round_assertion_1_read_back_content_fidelity` grades the emitted fingerprint's membership in `fingerprint_refs`.
- §9: the `live-pulse` gated SET gains `conductor-run/tests/p075_round_live.rs` under the same clippy line. It is named as a set, with no count literal.
**Why:** the round measured `degraded_mode: false`. That falsifies the mode-wide clause, which tests history had already flagged stale and which arch's per-read-back clause contradicted. The chunk also added a new gated live leg and its graders.
**Kept:** :323 / :324's degraded read-back is the CI stub path of `error-baseline-spike`, which is not a live-mode claim. :284's "DECLINED arm stub-ONLY and permanently so" is unrelated. No §9 or §11 site stated the span-landing input path.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/
