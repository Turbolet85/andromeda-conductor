
## 2026-09-30-full-gate-regression-over-the-moved-surfaces — an ambiguous P-ID target is refused
**Section:** §3 (`run` scenario invocation · Test selection)
**Change:**
- `run <P-ID>` (and the harness's `SCENARIO=<P-ID>`, passed straight through) REFUSES a P-ID that several scenarios name, before any scenario load: a harness fault naming the P-ID, the count and the matching scenario stems (never a host path), `error:` + `hint: pass one of the named scenarios instead of the P-ID`, exit 1, held by `cli_smoke::run_refuses_a_p_id_named_by_several_scenarios`. A single-owner P-ID still resolves; seven P-IDs are multiply named at 2026-09-30.
- Was: "`run` (and the harness's `SCENARIO=`) take the first scenario naming the P-ID in unsorted directory order … making `run` do the same is route-owned (CARRY)", per "2026-09-22-interpretation-proven-live — the real-model selector and harvest leg, parity scoped to the deterministic posture, and three stale claims corrected"; Test selection's "selects determinately only where a single scenario names it" retired with it.
**Why:** The route CARRY landed: a silent first-match pick is a guess, and `preconditions --for` already refused a P-ID for the same reason. The refusal narrows what the CLI admits; overseer direction at P5 recorded it needs no founder word, and it is named operator-visible.
**Kept:** every "a P-ID may be named by several scenarios" site (`:39`, `:71`, `:128`, `:201`, `:396`) — still true; `:124`'s 50 ms utterance-to-stamp window — the parser is unchanged.
**Ref:** .andromeda/runs/2026-10-01T00-02-40-wrap/
