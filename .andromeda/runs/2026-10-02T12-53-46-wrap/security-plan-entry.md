
## 2026-10-02-p-075-assert-round-against-pulse — span-landing ingest path and stale-pair refusal
**Section:** §Input Validation → the span-landing witness ingest row
**Change:**
- The two frozen Conductor self-obs journals now read `runs/span-landing/span-{a,b}.jsonl` under `CONDUCTOR_RUNS_DIR`. The firing form clears those two named files, non-recursively, before drive A. Was `runs/live-suite/span-{a,b}.jsonl`.
- The How column adds that a STALE pair is refused before any grading. `pair_is_current` requires drive A at or after the live Pulse log's first line, drive B at or before its last, and A before B. A pair left from an earlier pass is therefore never graded against the current log.
- The guards are unchanged: `capture_paths::runs_dir_from` → `resolve_under` and `pulse_logs_dir_from`. The `CONDUCTOR_RUNS_DIR` test-reader count stays at four.
**Why:** the chunk moved the pair to its own dir, out of reach of the `--live` suite's `rm -f`, and added the stale-pair refusal. The refusal narrows what is graded and widens nothing, so the row records it as validation.
**Kept:** the `ANDROMEDA_PULSE_DATA_DIR` reader rosters (the declaration-only READ SET row and the spawn-propagation row) are unchanged. The new `p075_round_live` passes the handle only to the shipped `ReadbackClient::connect` and joins no path, like `lifecycle_live.rs`, which no row lists, so it is not a VALUE reader.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/
