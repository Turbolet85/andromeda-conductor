
## 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir — digest pins, the fifth-class read, the moving tally
**Section:** §1 Test Scope Summary (desktop-webview driver; the fifth env-handle class) · §6 E2E Test Strategy (the real-model leg; the surface/driver table) · §7 Test Data & Fixtures (the provenance SET)
**Change:**
- §6 real-model leg:
  - Every committed capture is held by a sha256 digest pin (the test-only `sha2`) and graded from the file after the digest matches (was "pinned byte-equal to its section").
  - A tamper arm and a no-capture-text source arm join, as do the workspace-key mask arms.
  - The graded 2026-09-23 capture is an elided copy; the frozen 2026-09-22 file keeps its prefix.
  - The 2026-09-30 series graded none, so `v3-09` stays not met.
- §1 fifth class: `real_model_live.rs` reads `ANDROMEDA_PULSE_DATA_DIR` by `pulse_log` and now also by `workspace_key()`, both through the same guard.
- §1 and §6 driver: CI's routine arm reads "0 failing, only the expected-skip SET skipped (run 35208593666, and since, e.g. CI#36681853843)" (was "12 passing / 0 failing / 2 skipped").
- §7: the real-model harvest's committed capture EVIDENCE (a chunk's `evidence/`, read-only, digest-checked before grading) is named beside the fixture family.
**Why:**
- The chunk replaced the literal pins with digest pins and added one spec to the routine arm (tally 12 → 13 on the dev host and in CI#36681853843).
- The set form follows this plan's own moving-count rule at §6.
**Kept:** the "11 passing / 1 failing" probe record, the "(12 as of 2026-09-07)" moving-count record, and the coverage-matrix byte-equality at §6.
**Ref:** .andromeda/runs/2026-09-30T07-22-03-wrap/
