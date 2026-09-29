
## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model series, the three-storm canary, the backed diagnostic-quality four
**Section:** §Established Decisions [Read-Back Dependency Posture] (the gate description and the interpretation passage) · §Standard Contracts — Readiness gate · §Standard Contracts (the data-dir / corpus paragraph) · §Occupied Resources — `contracts/pulse-real-model-leg-posture.md`
**Change:**
- [Read-Back Dependency Posture], the interpretation passage:
  - Was "That leg now SHIPS and was driven ONCE (2026-09-23)". Now the leg is driven only as the pre-stated series its posture contract fixes before the first drive, and each counted drive is graded against a rule committed before it fires.
  - The 2026-09-23 drive blocked model-side at the one-storm canary (`NoAttributableIncident → Blocked`).
  - The 2026-09-29 series cleared that block with the three-storm canary and graded one drive `NotIdentified`: its model input carried the workspace key as `[redacted: credit_card]`, because Pulse's scrubber matched the dir name. `v3-09` is not met.
  - Was "the four diagnostic-quality capabilities … sit in the `UNBACKED_AUTO` ledger". Now they are backed: the scenario names them, each is graded by its own harvest arm, and they are out of `UNBACKED_AUTO`. Backing is not verifying.
- The same decision's gate description: the canary is three storms, 90 s apart, under real-model L4.
- Readiness gate: under the real-model L4 posture the canary is `REAL_MODEL_CANARY_STORMS = 3` distinct storms of `CANARY_STORM_COUNT` 12, `REAL_MODEL_CANARY_STORM_GAP = 90 s` apart, with the freshness stamp taken before the first. The reason: the model's surface/dismiss decision is nondeterministic per decision (`k_A = 2`, confirmed `k_B = 2`). The deterministic canary stays one storm, byte-identical.
- The corpus paragraph now points at security-plan's recorded 2026-09-29 breach (series pins in test source) and its route owner.
- The posture-contract entry: the drive series (2026-09-29) joins what is fixed before the first drive, and the Pulse coordinates are read at HEAD `e98d838` (was `83d4060`).
**Why:** the chunk replaced "driven once" with a pre-stated series (overseer ruling D1, founder-delegated), landed the three-storm canary and moved the four ids off the pin (`v3-10`); the founder ruled `v3-09` recorded not met, owned by a new series entry.
**Kept:** history moved out so both registries stay within the size threshold (here and in git).
- The 2026-09-23 block's detail: the model dismissed the canary's one cue-bearing digest, so no incident formed and the preflight blocked after its 600 s poll.
- `blocked_precondition: null`, from the gate's live-reading parenthetical.
- From the posture entry: the "(2026-09-18)" pin date, the `conductor-run/tests/lifecycle_live.rs:20` / `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` locator, and "never a sixth command".
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/
