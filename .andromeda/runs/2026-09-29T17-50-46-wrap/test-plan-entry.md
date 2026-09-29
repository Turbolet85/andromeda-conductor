
## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model leg is a pre-stated series, with the diagnostic-quality grades
**Section:** §2 Test Strategy (the determinism exceptions) · §3 5-command implementation (`--live real-model`) · §6 Real-model interpretation leg · §9 Live-Pulse scenarios
**Change:**
- The single-drive claim is retired at every site. It was:
  - §9: "fired once and never re-driven (only a Conductor-side environment fault may be re-fired)".
  - §6: "operator-gated, fired once".
  - §2: "fired once and graded".
  - §3: "so the one drive is never spent".
- Now the selector is driven only as the pre-stated series of `contracts/pulse-real-model-leg-posture.md`:
  - The series is fixed before its first drive; every drive is recorded and never replaced.
  - Only a Conductor-side environment or pipeline fault is re-fired, and only once.
  - Each counted drive is graded against a rule recorded before it fires; §9's `rule_record` precedes each drive's leg.
- §6:
  - It names the P-ID set: P-018 · P-031 · P-033 · P-034 · P-044.
  - It records the P-031 `structure`, P-034 `steps`, P-044 `retrieval` and `canary:` grades, appended below the recorded 2026-09-23 rule (asserted to be a byte-exact prefix), and every committed capture pinned byte-equal.
  - It records the 2026-09-29 outcome: five counted drives, one graded `NotIdentified → (CalibrationRegion, ManualCheck)`, `v3-09` not met.
**Why:** overseer ruling D1 (founder-delegated) replaced the single drive with a fixed, pre-stated series; the chunk added the three grades and extended the harvest.
**Kept:** §9's "real-model formation AND pickup both still UNMEASURED" stands: the report records no formation or pickup figure for the series.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/
