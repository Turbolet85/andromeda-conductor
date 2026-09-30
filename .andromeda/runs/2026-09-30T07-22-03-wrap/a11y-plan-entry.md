
## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — readiness before every axe analysis
**Section:** §9 CI Integration (Pipeline integration; the routine arm's CI line) · §11 A11y Anti-Patterns → CI
**Change:**
- §9 gains a bullet, "Readiness before every axe analysis":
  - axe opens every `analyze()` with a `document.readyState` probe raced against a hard 1 000 ms budget, and any miss prints `Page/Frame is not ready` (its catch-all).
  - `axeFindings()` waits until that probe answers in under 250 ms, then analyzes once.
  - The standing stall arm binds a 1.5 s stall to the probe's own read and was measured red with the wait bypassed and green restored.
  - A timer-scheduled stall cannot witness it.
- §9's CI line: the routine arm is green "0 failing, only the expected-skip SET skipped" at run 35208593666 and since (CI#36681853843) (was "12 passing / 0 failing / 2 skipped").
- §11 → CI: NEVER answer `Page/Frame is not ready` with a retry of `analyze()`, a sleep, a skip, `retries`, `continue-on-error` or a patched `FRAME_LOAD_TIMEOUT`.
**Why:**
- CI#36635281444 went red when the probe answered `true` 1 173 ms after it was sent, during the busy window after `#root` mounts; that was the classic-execute race, not BiDi.
- Measured on the dev host at a coherent 154.0.4258.37 pair, and in CI#36681853843 on `windows-2022` 20260920.314.1 at a coherent 131.0.2903.86 pair (13 passing / 2 skipped).
- The plan's timer form passed with the wait bypassed (the scheduling execute absorbed the stall; the next probe's round trip was 17.7 ms), so the witness moved onto the probe.
**Kept:** the 2026-09-01 BiDi narrative under §6 Motion tokens (true of that session), and the run-anchored "12 passing … as measured at run 35208593666" records.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/
