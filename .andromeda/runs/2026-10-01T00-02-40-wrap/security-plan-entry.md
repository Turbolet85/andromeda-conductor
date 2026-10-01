
## 2026-09-30-full-gate-regression-over-the-moved-surfaces — the `run` target refuses an ambiguous P-ID
**Section:** §Input Validation (CLI arguments / stdin row)
**Change:** The row names the `run` target (a scenario name or a P-ID) beside the seed/scenario flags, the `run_id` positional and the `preconditions --for` scenario name. A P-ID that several scenarios name is refused BEFORE any scenario load — an `anyhow` harness fault naming the P-ID, the count and the matching scenario file stems (never a path), `error:` + `hint:` at exit 1; the harness's `SCENARIO=` inherits it; a single-owner P-ID still resolves. Was: the first `read_dir` match, taken silently.
**Why:** The refusal narrows what the boundary admits — no new crossing, no write, no new value reaching an argv, shell or path — so it records validation rather than widening it; the row already justified `--for`'s parse refusal by "a P-ID can name several scenarios".
**Ref:** .andromeda/runs/2026-10-01T00-02-40-wrap/
