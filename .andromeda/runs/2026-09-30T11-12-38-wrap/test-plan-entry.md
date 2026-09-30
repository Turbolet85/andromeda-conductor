
## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the driven arm is one live run over its own two-scenario catalog
**Section:** §2 Agent-runnable invariants (Deterministic) · §6 desktop-webview row (driven arm · SR firing-form clause)
**Change:**
- §6: the driven arm is ONE live run over a harness-seeded catalog — `wdio.conf.ts` re-creates `runs/driven/scenarios` clean per run, copies exactly the two `[[checklist]]` scenarios (`halo-breathing-encoding`, `halo-hue-encoding`), sets `CONDUCTOR_SCENARIOS_DIR=runs/driven/scenarios` beside `CONDUCTOR_RUNS_DIR=runs/driven/runs`; two holds behind ONE canary. Real keypresses: Ctrl+Enter from a coverage-matrix row starts it, ArrowDown/ArrowUp while live, hold 1 trap / containment / Space / `role=status` / Escape→NoGo / restoration to the invoking ROW (by P-ID), Ctrl+. stops it, hold 2 Ctrl+Enter proceeds; decisions graded from the backend log's added lines (one `: No-Go (`, one `: Go (`, `run aborted by the operator`), never from the dialog closing. (Was: trap / containment / Space / announce / Escape→NoGo / restoration only.)
- §6 SR clause: "ONLY `sr-empty` and `sr-error` carry a `scenarios` field" is scoped to the SR suites — the driven suite also sets it since 2026-09-30.
- §2: the driven arm's wall-clock sample is `1 passing (4m 2.1s)` for the two-hold run, measured 2026-09-30, a dated sample, not a bound (was "~54s").
**Why:** one run with two holds carries all four shortcuts without a second canary (back-to-back runs dedupe on the open canary incident).
**Kept:** the 15-minute mocha ceiling; the live `sr` subject's shell-supplied `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`.
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/
