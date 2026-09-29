# P-025 hue-shift grade — leg verdict

The one graded drive of `halo-hue-encoding` against a Pulse carrying the P-025 fix `e98d838`, graded by
`contracts/pulse-p025-measurement-contract.md` §The grading rule.

## Before the leg

### (a) The grading rule predates the drive

- `sha256sum contracts/pulse-p025-measurement-contract.md` →
  `9da09cc125d94ac7a4dedd827f018d2f168b02779525904fc2858db6a2286d61`, recorded 2026-09-29T21:01:37Z, before
  any Pulse launch, probe or leg.

### (b) SUT provenance (operator entries 17 and 18, driven by hand; no port used)

- Pulse checkout: branch `chore/migrate-pulse-to-v3`, HEAD `4502d5dcaf0d0859b6e6a12c540221b7d7e4e483`. HEAD
  has moved past the plan's `226554a`, by one commit (`4502d5d`, an operator pre-CI commit). Its diff against
  `226554a` touches only `.github/workflows/ci.yml`, `xtask/src/*`, `pulse-app/tests/quality_gate_workflow.rs`
  and chunk documents. No `pulse-app/src`, MCP-crate or `Cargo.lock` path changes, so the binaries' source
  equals `226554a`'s.
- Pulse's uncommitted working-tree edits are another session's, in `scripts/agent-run.sh`,
  `xtask/src/harness_status.rs` and two chunk documents. None of them is in the `pulse-app` or MCP-crate
  source.
- Entry 18, `git -C <pulse> merge-base --is-ancestor e98d838 HEAD` → exit 0: the checkout carries the P-025 fix.
- Entry 17, `grep -ac tier_effective_at_unix_nano <pulse>/target/release/pulse-app.exe` → `2`, exit 0: the
  binary the leg drives carries the field by content (Pulse's own leg read the same 2).
  - The binary is the EXISTING `pulse-app.exe`, which the overseer designated (mtime 2026-09-29T17:48:25 local).
  - `andromeda-pulse-mcp.exe` was stale (mtime 2026-09-01T18:04:27 local), so it was rebuilt on the overseer's
    word from the same checkout. The overseer dictated `cargo build --release -p andromeda-pulse-mcp`; the
    package is `mcp-server`, and `andromeda-pulse-mcp` is its feature-gated bin
    (`crates/mcp-server/Cargo.toml` at `4502d5d`), so the form run was
    `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server`.
    - The build finished with exit 0 in 4m 17s (new mtime 2026-09-29T23:05:46 local).
    - Pulse's `triage` build script fetched its tokenizer over the network during that build. The fetch
      belongs to Pulse's build, not to Conductor.
- Both probes were re-read inside the operator's slot at 2026-09-29T21:06:50Z, with identical results. The
  contract sha256 was unchanged.

### Slot and launch posture

- The slot was granted by the overseer (founder-delegated): 4317 and 4318 had no listeners on Windows, and the
  Pulse builder was holding its WSL trial loop.
- The agent launched `pulse-app` itself, on the overseer's word, through `Start-Process` without `-Wait`:
  - PID 22800, created 2026-09-29T21:06:56Z;
  - a fresh letters-only data dir `%TEMP%/pulse-legs/huegradedhard`, which is also the cwd, outside this repo;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` and `ANDROMEDA_PULSE_MCP_ENABLED=true`;
  - `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` and `RUST_LOG` removed.
- The booted posture was confirmed from Pulse's own log:
  - `interpretation.model.load` reports `inference_mode: deterministic`;
  - `app.boot.workspace_key` reports `workspace_root_basename: huegradedhard`;
  - `app.boot.otlp.grpc.bind` reports `127.0.0.1:4317`;
  - no `triage.baseline.bootstrap_window.override` line appears.
- The compact widget was open and visible before any dispatch. Pulse shows it at `setup`
  (`window::show_compact_widget`, `pulse-app/src/main.rs:1120`). A window enumeration of PID 22800 read the
  496x279 `andromeda-pulse` window as visible and not minimized, with the dashboard window hidden.
- The sidecar resolved on the leg's `PATH` from the rebuilt Pulse `target/release`.

### (c) Census before — 2026-09-29T21:06:50Z, before the launch

| probe | reading |
|---|---|
| entry 19 (`tasklist` over `conductor` / `andromeda-pulse-mcp` / `pulse-app`) | exit 1, no process |
| `Win32_Process` parentage census (same images plus every descendant of `pulse-app` / `conductor`) | none matched |
| listeners | `:4317` none · `:4318` none |

### (d) NON-PRIMING preconditions (entry 20)

- `conductor preconditions` → exit 0 and `[PRECONDITION] every live-Pulse precondition is satisfied`, at
  21:07:43Z. It fires no canary.

### (e) Pre-leg line count (entry 21)

- `wc -l` of Pulse's newest `agent-latest.jsonl.2026-09-29` → **34573**, taken after readiness (the OTLP bind at
  21:06:57.582Z) and before any dispatch, at 21:09:00Z.

## The leg (entry 22) and the freeze (entry 23)

- One `conductor run halo-hue-encoding --agent-mode`, fired at 21:09:10Z. It ended at 21:13:01Z with exit 0,
  and its verdict line printed `[RESIDUAL] halo-hue-encoding`. All three atoms hold: `exit 0`,
  `lacks [BLOCKED]` and `contains halo-hue-encoding`.
- The freeze was chained directly after it: `logs/agent-latest.jsonl` → `evidence/h.jsonl` (784 lines, one
  `run_id`), exit 0, which printed `"run_id":"2026-09-29T21-09-10-754"`.
- The leg was fired once. No re-fire was needed.

## Envelope (entry 24, the status smoke, mint-then-read)

`bash scripts/agent-run.sh status 2026-09-29T21-09-10-754` → exit 0. The printed `run_id` equals the one the
freeze minted:

| run_id | scenario | seed | state | verdict | latency_ms | slo_tier |
|---|---|---|---|---|---|---|
| `2026-09-29T21-09-10-754` | `halo-hue-encoding` | 4317025 | `KnownResidual` | `null` | 183860 | `<90s` |

The run row is the declare-only family's ordinary degraded read-back outcome. It does not carry the P-025 grade;
the harvest does.

## The window, derived per §The grading rule

- `timeline.execute` `new` (the ONE such line in `evidence/h.jsonl`): `timestamp_ms` 1790716196860.
- Phase-2 start = 1790716196860 + 30000 = **1790716226860**.
- `scenario.run` `close`: **1790716380720**.
- Window: `[1790716226860, 1790716380720]`.

## Every hue sample after the pre-leg count, with its anchor

The slice is `evidence/pulse-hue-lines.jsonl`: 9 lines, byte-copied from Pulse's log from line 34574 on. Only
three targets are kept: `metric.constellation.hue_update_ms`, `interpretation.incident.created`, and
`triage.incident.auto_resolve.tick` where `resolved_count` ≥ 1.

| sample `timestamp` (ms) | in window | `severity_tier` | `duration_ms` | start instant (ts − duration) | nearest preceding `created=true` | anchor error |
|---|---|---|---|---|---|---|
| 1790716196316 | no (before phase-2 start: the preflight canary's rise) | `autonomous` | 438.1103515625 | — | — | not graded |
| 1790716233315 | **yes** | `autonomous` | **684.976318359375** | 1790716232630.02 | 1790716232660 | **29.98 ms** (≤ 1000) |
| 1790716378315 | **yes** | `none` | 430.78955078125 | 1790716377884.21 | — (a fall; not anchored to a creation) | — |

- The later `created=true` at 1790716233636 and the `created=false` dedupe at 1790716243633 follow the rise
  sample, so they do not anchor it.
- The fall's start instant, 1790716377884, sits inside the auto-resolve tick that logged `resolved_count: 2`
  at 1790716377928 (that tick's own `duration_ms` is 44). That is the contracted fall source,
  `resolved_at_unix_nano`, as Pulse's own tick witnesses it.
- The earlier `resolved_count: 1` tick at 1790716317883 is the canary incident's resolution. The canary's dot
  was hidden by then, and it emitted no sample, as the rule predicted.

## The grade, by the rule

**PASS.** The worst in-window `duration_ms` is **684.976318359375 ms ≤ 2000**. The one rise is anchored to its
incident's opening at 29.98 ms. `v3-08` is met: the budget is graded hard at a real measured value.

**Prediction check (not a grade input).** The plan forecast a rise of about 0.5-1.5 s, and the measured rise
is 684.98 ms, inside it. The rule's prose forecast that the end-of-storm fall would land after the dot hid.
Measured, the fall landed IN-window at 430.79 ms: the scenario's incident auto-resolved at the storm's end
(tick 1790716377928, 1 068 ms after phase 2's nominal end at 1790716376860), while the dot was still live.
The rule grades every in-window sample, so that fall is graded, and it passes.

**Where the fall is otherwise witnessed:** Pulse's own `smoke:hue-shift` at `e98d838` (fall anchor errors 25 ms
and 28 ms, `duration_ms` 510 and 578; Pulse `evidence/green-leg.md`).

## Census after, and teardown

The census after (entry 25) was taken at 21:13:13Z, after the leg and before the stop:

| process | pid | parent | created (UTC) | started by | final state |
|---|---|---|---|---|---|
| `pulse-app.exe` | 22800 | 23280 (launcher, gone) | 21:06:56Z | this run (agent, on the overseer's word) | terminated |
| `conhost.exe` | 46240 | 22800 | 21:06:56Z | `pulse-app` | terminated |
| `msedgewebview2.exe` ×9 | 29704 (root) + 19240, 39840, 58304, 22476, 6432, 59036, 19848, 13504 | 22800 → 29704 | 21:06:56-57Z | `pulse-app` | terminated |
| `conductor.exe`, `andromeda-pulse-mcp.exe` | — | — | — | the leg | none survived the leg (absent from the census after) |

- Stop form: the targets were matched by PID AND creation time, and the descendant tree was captured before
  the stop. `CloseMainWindow` returned True, but the root stayed alive, so it was followed by
  `Stop-Process -Force`. Pass 1 then read none alive.
- The census after the stop (21:14:12Z) read none matched, and `:4317` and `:4318` had no listener.
