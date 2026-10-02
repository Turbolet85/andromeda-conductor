# Witness ledger: two same-seed drives landing in one Pulse retention window

This is the record of the operator pass, plan steps 7-15. Entries are cited by their plan Test Commands number in
the gate tool's numbering (18-32). This ledger copies in no Pulse log text, no path and no workspace key. Atoms
are quoted from Conductor's own output.

## Slot and grant

- **Slot:** granted by the overseer on 2026-10-01 at about 23:31Z for 20 minutes: "Grant 20 min now". The grant
  carried the overseer's note that 4317/4318 were measured free, no pulse-app was running, and the pulse-builder
  session builds only Pulse's debug target and would launch no pulse-app during the slot.
- **Grant:** the operator's `:4317` grant, for this one launch.
- **Agent-active span:** 23:31:36Z (census) to 23:38:42Z (teardown census), about 7 minutes.

## Census before the launch (entry 18), 23:31:36Z

- Rows matching `pulse-app|andromeda-pulse-mcp|conductor`: **none** (empty table, exit 0).
- Loopback listeners on 4317/4318: **0**.
- Out of the census scope: another session's `cargo nextest run -p pulse-app` debug build was running on the host
  (parent chain cargo, cargo-nextest, cargo, rustc). It is not this pass's process and was left alone.

## Binaries (entry 19), 23:31:36Z, re-measured right before the launch

Exit 0, and both atoms matched the 2026-10-01 ledger (the `a2addb3` build):

- `pulse-app.exe` — `6476568e0173bd187ff0616af2c777ed2a7d60e4b21d923ad02768cd8625e281`
- `andromeda-pulse-mcp.exe` — `29f35540e2fb25bcae3760c24f898a096453253146da1986aafcdcf64cc2a6da`

## Launch (step 8)

The launch script was a Write-tool ASCII `.ps1` in the session scratchpad, run by path. It called `Start-Process`
without `-Wait`, with a fresh data dir (letters-only leaf `spanlanding` under the `pulse-legs` parent) that was also
the cwd. It set `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` and `ANDROMEDA_PULSE_MCP_ENABLED=true`, and removed
`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` and `RUST_LOG`.

- **PID** 52128 · **parent** 41344 · **CreationDate (UTC)** 2026-10-01T23:32:04.004Z

Posture as Pulse's own log recorded it, before any drive:

- `interpretation.model.load`: `inference_mode` = `deterministic`
- `app.boot.workspace_key`: `workspace_root_basename` = `spanlanding` (the key itself is not copied)
- `app.boot.otlp.grpc.bind`: `bind_address` = `127.0.0.1:4317`
- bootstrap-override line: **absent**

## Drives

| Entry | Step | Window (UTC) | Exit | Atoms |
|---|---|---|---|---|
| 20 preconditions | 9 | 23:32:40 → 23:32:47 | 0 | `contains [PRECONDITION] every live-Pulse precondition is satisfied` (1 hit) ✓ |
| 21 drive A | 9 | 23:32:47 → 23:33:44 | 0 | `lacks [BLOCKED]` (0 hits) ✓ · `contains exception-event-capture`: verdict line `[MANUAL] exception-event-capture` ✓ |
| 22 freeze A | 9 | 23:33:44 | 0 | artifact `runs/live-suite/span-a.jsonl` written right after drive A |
| 23 quiet window | 10 | 23:33:50 → 23:36:50 | 0 | 180 s |
| 24 preconditions | 11 | 23:36:59 → 23:37:00 | 0 | `contains [PRECONDITION] every live-Pulse precondition is satisfied` (1 hit) ✓ |
| 25 drive B | 11 | 23:37:00 → 23:37:54 | 0 | `lacks [BLOCKED]` (0 hits) ✓ · `contains exception-event-capture`: verdict line `[MANUAL] exception-event-capture` ✓ |
| 26 freeze B | 11 | 23:37:54 | 0 | artifact `runs/live-suite/span-b.jsonl` written right after drive B |

- **Drive A run_id:** `2026-10-01T23-32-49-374`. **Drive B run_id:** `2026-10-01T23-37-02-440`.
- Both drives ran at the scenario TOML's seed, `4317006`; no `--seed` was passed.
- The `[MANUAL]` verdict is not graded here. Landing is this witness's subject, not the read-back outcome.

## Witness (entry 27, step 12), 23:38:01Z

Exit 0, `test result: ok. 1 passed`. The summary line:

```
span-landing: PASS run_ids=2 delta_ms=253507 retention_ms=600000 reject_lines=0 append_rejections=0 spans_after_b=4
```

- Atoms: `exit 0` ✓ · `contains span-landing: PASS run_ids=2` ✓ · `contains reject_lines=0 append_rejections=0` ✓.
- The two scenario emission instants sat **253.5 s** apart, inside Pulse's own logged **600 s** retention window.
  Pulse refused nothing over the whole launch, which covers both drives and both preflight canaries. Pulse
  appended spans after drive B's emission instant.
- Measured against the prediction: the plan predicted Δ ≈ 300-400 s. Δ measured 253.5 s, because drive A took
  57 s end to end rather than the predicted 2-3 min. That is still well inside the bound, and the witness asserts
  the bound rather than assuming it.

## Teardown (step 13), 23:38:42Z

- PID 52128 was matched by image name AND UTC CreationDate (`2026-10-01T23:32:04`) before any stop.
- `CloseMainWindow()` returned True. The process was still alive after a 15 s wait, so `Stop-Process -Force` was
  issued.
- After the stop: target alive **False** · processes whose parent was 52128 **0** · loopback listeners on 4317/4318
  **0**.
- Census after the teardown (entry 18 form): **none** (empty table, exit 0). Ports released.

## Mint-then-read (entry 28, step 14)

`status` on drive B's run_id `2026-10-01T23-37-02-440` gave exit 0. The envelope read back that same run_id with
`"scenario": "exception-event-capture"` (1 hit) ✓, `seed` 4317006, and `state` `ManualCheck`.

## Evidence hygiene (entries 29 and 30), after this ledger was written

- Entry 29, the evidence host-path probe over `revert-red.md` and this ledger: green, count `0` at exit 1.
- Entry 30, `gate.py hygiene`: exit 0, `hygiene: clean`, with every control firing on its synthetic known positive.

## Advisory-db currency (entry 15), re-run after an overseer act

- The first run (implement P2) read red. Its one porcelain row was an untracked placeholder-id advisory
  (`RUSTSEC-0000-0000.md`, for the crate `matrix-sdk-crypto`, which neither project uses) left beside the advisory
  that later received a real id.
- **Overseer act:** the overseer MOVED that file out of the shared advisory-db copy into the overseer's scratchpad.
  It was kept, not deleted.
- The re-run was green: no output at exit 0, and HEAD == FETCH_HEAD (`6de44551`). The `cargo audit` reading (1279
  advisories · 562 crates · 7 allowed · exit 0) therefore stands on a verified-current copy. A fresh clone read the
  same figures.

## Operator entries: pre-CI commit, push, CI read

These were made by the agent ON THE OPERATOR'S EXPLICIT WORD ("the OPERATOR PASS on my word"). They are the
operator's acts.

- **Pre-CI commit:** `163e0f6`, `chore(2026-10-01-per-run-span-identity-in-the-real-model-harness): operator pre-CI
  commit, for the run this chunk's verdict reads` (parent `2c97d3b`). The tree was clean after it.
- **Entry 31, the push behind the clean-tree guard:** exit 0, `2c97d3b..163e0f6 HEAD -> build/conductor-0.3.0`.
  - Atom `contains PUSHED_SHA=`: held, `PUSHED_SHA=163e0f67b8b73eaf04b9a7f4893f4677d7bcfa38`.
- **Entry 32, `ci.py conclusion --sha HEAD --wait 1500`:** exit 0, polled 22× over 657 s.
  - Atom `contains verdict: green`: held, `163e0f67b8b7 verdict: green · checks 3/3 · wall 649 s`.
  - The run is **CI#36942272745** (push, completed/success), and the overseer verified it.
