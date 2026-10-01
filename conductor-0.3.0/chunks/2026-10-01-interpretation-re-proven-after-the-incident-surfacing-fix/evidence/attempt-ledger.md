# Attempt ledger — the 2026-10-01 real-model series

One row per drive fired, in firing order, under `contracts/pulse-real-model-leg-posture.md` §The 2026-10-01 series
(fixed before `d1`). Every drive is recorded; a canary-blocked drive is a measurement, never graded. Authored by
the agent from each drive's capture copy, Pulse's own log and its census; no Pulse file is edited.

## Gate re-check (plan step 6, fired by hand 2026-10-01T19:14:27Z, read at Pulse's committed state, no fetch)

- `a2addb3` is an ancestor of Pulse's upstream (`origin/chore/migrate-pulse-to-v3`, the local tracking ref):
  `merge-base --is-ancestor` exit **0**.
- Commits on Pulse's checked-out branch not on its upstream: **0** (exit 0).
- Pulse's build inputs (`crates`, `pulse-app`, `Cargo.toml`, `Cargo.lock`) with an uncommitted or untracked change:
  **0 lines** (exit 0).
- Pulse `HEAD`: `a2addb3755b3029cb79809b96efdd2522749b179` — the fix commit itself.

## The binaries (plan step 7, on the overseer's build-slot grant)

**Grant:** the overseer, 2026-10-01 (measured by them at the grant: 0 cargo processes on the host, pulse-builder idle
and held, Pulse's build inputs clean at `a2addb3`) — covering both release builds and the proofs. Re-read by the
agent at fire time (19:20:41Z) and again after both builds: HEAD `a2addb3`, build-input porcelain 0 lines both times.

| Binary | Command (in the Pulse tree) | Exit · `Finished` | Built | sha256 | `incident producer skipped` | ` active incident(s); ` | ` active-bypass incident(s); ` |
|---|---|---|---|---|---|---|---|
| `pulse-app.exe`, before (the 2026-09-30 22:25 build, the control) | — | — | 2026-09-30 22:25:37 +02:00 | — | 0 (P5 reading) | — | 1 (P5 reading) |
| `pulse-app.exe` | `cargo build --release -p pulse-app` | 0 · `release` profile in 8m 16s | 2026-10-01 21:28:24 +02:00 | `6476568e0173bd187ff0616af2c777ed2a7d60e4b21d923ad02768cd8625e281` | **1** (exit 0) | — | **0** (exit 1) |
| `andromeda-pulse-mcp.exe` | `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 0 · `release` profile in 3m 04s | 2026-10-01 21:31:55 +02:00 | `29f35540e2fb25bcae3760c24f898a096453253146da1986aafcdcf64cc2a6da` | 0 | 0 | 0 |

- **`pulse-app`: the CONTENT arm holds** — the string the fix introduces is present and the OVERALL wording it
  retired is absent.
- **The sidecar: the PROVENANCE arm holds**, as P5 predicted (the `fcc31b2` sidecar carried neither OVERALL piece
  either: the triage string does not survive the sidecar's link). Its basis: build inputs clean at `a2addb3` before and
  after the build; the build command above, exit 0; sha256 `29f35540…`, different from the `fcc31b2` build's
  `2179caab…`; mtime 2026-10-01 21:31:55 +02:00, after the commit (2026-10-01 20:39:04 +02:00).

Cargo's raw output stays out of evidence (it names host paths); only the exit and the `Finished` line are recorded.

## The dir (plan step 8)

`%TEMP%/pulse-legs` before the leaf was created held 22 entries (`20260901T160708Z`, ten `a11y-*`, `drivenkeys`,
`fullgate`, `fullgatesr`, `huegradedhard`, `rm-20260923-093840`, `rm-clean-series`, `srcontent`, `srcontentthree`,
`srcontenttwo`, `srliveretry`, `srosregrade`); `rm-surfacing-series` was absent. Created with a plain `mkdir`
(exit 0, empty).

## Pre-registration (plan step 9)

The contract section §The 2026-10-01 series — LF-normalized, from its heading line up to the next `## ` heading,
the `contract_section` form — recorded before `d1` fired (the same computation over §The 2026-09-30 series
reproduces that ledger's recorded `0091fe6f…`):

pre-registration sha256: 0232afb1c302c49e408c92246ebfb6090c64c06ef81dea782a4af7656422e841

## The model-run slot and the launch (plan step 10)

**Grant:** the overseer, 2026-10-01 (measured by them: ports free, no `pulse-app`) — `d1`-`d3` back to back with no
confirmation between drives; the AGENT launches `pulse-app` from a cwd outside this repo, takes a census before and
after, and stops it and its children after `d3`; pulse-builder held for the whole series.

- **Pre-series census** (19:34:26Z): no `pulse-app`, `andromeda-pulse-mcp`, `conductor`, `llama-cli`, `cargo` or
  `rustc` process; no listener on `:4317` or `:4318`. Baseline survivors not this series': six `msedgewebview2.exe`
  under `SearchHost.exe` (2026-09-26) and the session's `conhost.exe` rows.
- **Launch** (the agent, a scratchpad `Start-Process -PassThru` script, cwd = the data dir): `pulse-app.exe` from
  `target/release` (sha256 `6476568e…`), PID 34792, created 2026-10-01T19:34:32.810Z. Launch env: the data dir, both
  model paths (each resolving to a file), `ANDROMEDA_PULSE_L4_DETERMINISTIC` and
  `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` removed (both read back unset).
- **Booted posture, from Pulse's own log** (`agent-latest.jsonl.2026-10-01`, read 19:34:47Z, before `d1`):
  `interpretation.model.load` `inference_mode` **`real`**, `load_status` `loaded`, `model_identity`
  `Llama-3.2-3B-Instruct-Q4_K_M`; `app.boot.workspace_key` `workspace_root_basename` **`rm-surfacing-series`** (the
  leaf — the CARRY confirmed), `key_bytes` 68; `app.boot.otlp.grpc.bind` `127.0.0.1:4317` at 19:34:33.830Z;
  **0** `triage.baseline.bootstrap_window.override` lines.
- Conductor's side for every drive: `ANDROMEDA_PULSE_DATA_DIR` the same dir, `ANDROMEDA_PULSE_MCP_ENABLED=true`, the
  L4 flag absent, the sidecar on `PATH` from `target/release` (sha256 `29f35540…`).

## The drives (plan step 11)

Each drive: a census, then the non-priming probe `conductor preconditions --for real-model-interpretation` (plan entry
28), then `bash scripts/agent-run.sh run --live real-model` (entries 29-31) and the copy of
`runs/live-suite/rm-capture.txt`. Every leg exited 0 and printed both atoms (`[live] leg rm: real-model-interpretation`,
`[live] leg rm: froze`), 1 hit each. Grades are the harvest's (`each_2026_10_01_drive_grades_as_the_ledger_records`),
computed by the unchanged rule over the pinned files. Every envelope: `seed` 4317033, `state` `ManualCheck`,
`verdict` null. Pulse logged `prompt_version` `v2.3` on every prompt assembly of the series.

| Label | Slot · timing · run | Route | Grade (P-033) | P-031 / P-034 / P-044 | `canary:` tokens (with `skip_reason`) | The scenario's digest, from Pulse's own log | Key rendering | Census |
|---|---|---|---|---|---|---|---|---|
| **d1** — `rm-capture-d1.txt` | the overseer's grant; the agent's `pulse-app` (PID 34792); probe exit 0 19:35:09-19:35:24Z (`[PRECONDITION] every live-Pulse precondition is satisfied`); fired 19:35:32Z, ended 19:41:52Z; run `2026-10-01T19-35-46-776` | `ReadBack` — incident 2 attributed on the first poll, `degraded_mode=false`, pickup 8597 ms | **`Identified`** | `Pass` / `Pass` / `Blocked` (no prior same-scope incident to retrieve) | `surfaced` (19:36:31Z, created) · `surfaced` (19:38:01Z, deduped) — `skip_reason=none` both | storm Autonomous 19:39:37.031Z; tier-1 `retry_storm` digest 19:39:37.039Z, parse `ok` 19:39:44.509Z, **created** 19:39:44.530Z — surfaced. Skip lines in the window: 4, all `no_cue` (cue-less tier-3). One `inference.error` `json_parse_failed` (19:36:44Z) belongs to a cue-less tier-3 digest | `verbatim` | before: the agent's `pulse-app` tree only; after: no new process |
| **d2** — `rm-capture-d2.txt` | the same grant and launch; quiet window: d1's last incident 19:39:44.530Z, active set 0, probe 156 s after it; probe exit 0 19:42:32Z; fired 19:42:33Z, ended 19:58:47Z; run `2026-10-01T19-42-35-548` | `ReadBack` — preflight READY, emission instant 19:46:20.822Z; `attribution: none within 600s` over 61 polls | not graded — `NoAttributableIncident` | `Blocked` ×3 (no attributable incident) | `surfaced` (19:43:20Z, created) · `surfaced` (19:44:50Z, deduped) — `skip_reason=none` both | **never formed**: from 19:46:21.326Z every append of the scenario's spans was refused (`duckdb.append` `reject_reason=append_failed`, 36 in the window; `buffer.tick` `append_rejections` 36), so no scenario storm was detected and no scenario digest existed to surface or skip. Cause: the scenario's span identity is a deterministic function of its seed (`crates/conductor-emit/src/exception.rs` `exception_trace_request`), so d2 re-sent d1's `(trace_id, span_id)` while Pulse's buffer still held d1's rows (evicted 19:48-19:51Z). Skip lines: 6, all `no_cue` | no report read | after: no new process |
| **d3** — `rm-capture-d3.txt` | the same grant and launch; quiet window: the last incident before it 19:54:31.252Z, active set 0, probe 377 s after it; probe exit 0 20:00:59Z; fired 20:01:00Z, ended 20:07:17Z; run `2026-10-01T20-01-03-329` | `ReadBack` — incident 10 attributed on the first poll, `degraded_mode=false`, pickup 5128 ms | **`NotIdentified`** — rank 1 names the Conductor service and an error-rate spike, no retry token | `Pass` / `Pass` / `Pass` | `surfaced` (20:01:48Z, created) · `surfaced` (20:03:18Z, deduped) — `skip_reason=none` both | storm Autonomous 20:04:53.589Z; tier-1 `retry_storm` digest 20:04:53.596Z, parse `ok` 20:04:57.882Z, **created** 20:04:57.902Z — surfaced (the model titled it an error-rate spike over a `retry_storm` cue). Skip lines: 5 — 4 `no_cue`, 1 `model_resolution_summary` (20:06:40Z, cue-less tier-3) | `verbatim` | after: no new process; then the teardown below |

d2 is a fired, counted drive: its canary surfaced, so it is not the pipeline-fault the re-fire rule names, and nothing
was re-fired. The identical-span-identity gap is routed forward at the wrap on the overseer's word (2026-10-01); no
change in this chunk.

## Series verdict (the pre-registered rule, applied mechanically)

Three drives fired, none re-fired, no fourth. **Graded drives: 2** — d1 `Identified`, d3 `NotIdentified`. Under §The
drive series (a), carried unchanged into §The 2026-10-01 series, a graded `NotIdentified` means **`v3-09` NOT MET**
(`v3_09_is_not_met_by_the_2026_10_01_series`). Recorded, never replaced. No ref test is written.

What the series measured beyond the verdict: with Pulse's surfacing fix live, the real model surfaced every
cue-bearing canary digest in all three drives (6 of 6, `skip_reason=none`) and the scenario's own storm digest in
both drives where it formed (2 of 2), where the 2026-09-30 series dismissed the scenario's digest twice. Within the three leg windows every
no-incident outcome Pulse logged was on a cue-less tier-3 digest (`no_cue` 14, `model_resolution_summary` 1). d1's
rank 1 restates the cue line (the stated limit); d3's names the service and a different cue kind.

## Stated residual — d3's all-digit fingerprint prefix

`rm-capture-d3.txt` carries one 8-character `fingerprint_hex` value of digits only, on two lines (the `suggested` and
the `autonomous` detection of one storm). `elide_fingerprints` keeps an all-digit run by design (a stamp or a seed is
never one), so the four-stage chain passed it. The storm is the preflight canary's second (`scenario_storm=false`,
20:03:18Z), which Conductor itself emits: the prefix is of a SYNTHETIC fingerprint, over synthetic content, carrying no
real data. On the overseer's ruling (2026-10-01, option (a)) it is committed as a stated residual, like the frozen
2026-09-22 file's — no new elision code in this chunk — and the harvest counts it exactly
(`the_2026_10_01_captures_carry_no_fingerprint_and_no_workspace_key`), so it can neither grow nor go unseen.

## Teardown (after d3)

The agent stopped what it started. `pulse-app` PID 34792 (created 19:34:32.810Z) and its 10 descendants (one
`conhost.exe`, nine `msedgewebview2.exe`), each matched by PID and UTC creation time: `CloseMainWindow` returned True
but the app did not exit within 15 s, so it was stopped by force; the first pass found 0 survivors. Census 20:08:18Z:
the non-`conhost` rows equal the pre-series baseline, no listener on `:4317` or `:4318`, no `pulse-app`,
`andromeda-pulse-mcp`, `conductor`, `llama-cli`, `cargo` or `rustc` process.

## The operator pass (plan entries 36-38), on the overseer's word

- Entry 36, `gate.py hygiene`: first `refused 2 files` — the phase run's P5 dry-run transcripts
  `p5-dryrun.txt` and `p5-dryrun-2.txt` (each 18 host-path forms, drive and MSYS, in the gate tool's own header).
  On the overseer's ruling, following the 2026-09-30 precedent, both moved to the gitignored
  `.andromeda/cache/p5-controls/2026-10-01T18-42-55-phase/` (sha256 unchanged, `285bb3ce…` and `96a7d53a…`), a
  `.MOVED.md` note each left in its place, both recorded in `scope-record.md` as a widening on that word; re-run:
  `hygiene: clean`, 37 files read.
- Entry 37: on the overseer's explicit word, the agent made the operator pre-CI commit and ran the guarded push.

## Status smoke (plan entry 32), fired by hand after d3

`bash scripts/agent-run.sh status 2026-10-01T20-01-03-329` (d3's run_id): exit 0; atom
`contains "scenario": "real-model-interpretation"` held (1 hit); the envelope read back that `run_id`, `seed`
4317033, `state` `ManualCheck`, `verdict` null.
