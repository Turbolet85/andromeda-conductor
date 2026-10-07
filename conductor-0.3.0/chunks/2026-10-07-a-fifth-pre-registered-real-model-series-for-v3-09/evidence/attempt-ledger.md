# Attempt ledger — the 2026-10-07 real-model series

One row per drive fired, in firing order, under `contracts/pulse-real-model-leg-posture.md` §The 2026-10-07 series
(fixed before `d1`). Every drive is recorded; a canary-blocked drive is a measurement, never graded. Authored by
the agent from each plan entry's recorded result, each drive's capture copy, Pulse's own log and its census; no Pulse
file is edited. This ledger records handle NAMES, the data dir's leaf, digests, counts and booleans, never a host
path.

## Gate re-check (plan step 1, entries 1-4, fired by hand 2026-10-07T07:38:33Z, read at Pulse's committed state, no fetch)

- `f70be92` is an ancestor of Pulse's upstream (the local tracking ref of `chore/migrate-pulse-to-v3`):
  `merge-base --is-ancestor` exit **0**.
- Commits on Pulse's checked-out branch not on its upstream: **0** (exit 0; the entry's `last line 0` atom holds).
- Pulse's build inputs (`crates`, `pulse-app`, `Cargo.toml`, `Cargo.lock`) with an uncommitted or untracked change:
  **0 lines** (exit 0; the entry's `no output` atom holds). The tree's only dirty paths are wrap bookkeeping (three
  rows in `git status --short`).
- Pulse `HEAD`: `f70be92c2ca13c951330efeffd725e62a484610c` (exit 0; the entry's `last line` atom holds) — the commit
  the series is pinned to. Its code commit `1a2e509` was committed 2026-10-07 08:16:59 +02:00.

## The binaries (plan steps 3-5, entries 5-9, on the overseer's build grant)

**Grant:** the overseer, founder-delegated, 2026-10-07 (the phase directive and the P5 review answer; the chunk's
inputs record them as I2 and I17): both release builds in the Pulse tree and the launch are the agent's;
pulse-builder stays idle.

### The old side of the control (entry 5, fired by hand 2026-10-07T07:38:42Z, BEFORE any build)

Report-only; the command's exit is its last `grep`'s (1) and is not asserted.

| Binary | Built | sha256 | `must name that scope_id value exactly as` | `whose name merely contains it` | `v1.4-fallback` | `v1.4-reflection` |
|---|---|---|---|---|---|---|
| `pulse-app` | 2026-10-06 22:00:21 +02:00 | `f69be5bb8643a479649149f06aef30345b17f8279af9fa0aea6039034f19cf44` | 0 | 0 | 1 | 1 |
| `andromeda-pulse-mcp` | 2026-10-06 22:02:46 +02:00 | `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` | 0 | 0 | 0 | 0 |

Both digests equal the 2026-10-06 series' recorded rebuilt binaries, so these are the `5f77859` builds, and every
count equals the plan's expectation (research.md §The binaries on this host).

### The builds (entries 6-7, fired by hand, one after the other)

Both ran in the Pulse tree with the leg env script sourced (so the build reads the local tokenizer). Cargo's raw
output stays out of evidence (it names host paths); only the exit and the `Finished` line are recorded.

| Binary | Command (in the Pulse tree) | Started · ended | Exit · `Finished` |
|---|---|---|---|
| `pulse-app` | `cargo build --release -p pulse-app` | 07:39:21Z · 07:40:59Z | 0 · `release` profile in 1m 38s |
| `andromeda-pulse-mcp` | `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 07:41:06Z · 07:41:15Z | 0 · `release` profile in 9.01s |

The sidecar's build is short because the first build had already compiled every crate the two binaries share.
Re-read by the agent after both builds (07:41:28Z): Pulse HEAD `f70be92c2ca13c951330efeffd725e62a484610c`,
build-input porcelain 0 lines.

### The new side of the control (entries 8-9, fired by hand 2026-10-07T07:41:28Z, AFTER the builds)

- Entry 8 (asserting): `sentence-a=1 sentence-b=1`, exit **0** — at least one string of the sentence `f70be92` adds
  is in the rebuilt `pulse-app`. On the `5f77859` build the same entry printed `sentence-a=0 sentence-b=0` at exit 1
  (the plan's P4 control).
- Entry 9 (report-only; its exit is the last `grep`'s, 1, and is not asserted):

| Binary | Built | sha256 | `must name that scope_id value exactly as` | `whose name merely contains it` | `v1.4-fallback` | `v1.4-reflection` |
|---|---|---|---|---|---|---|
| `pulse-app` | 2026-10-07 09:40:59 +02:00 | `df1676477222776d3e95edae7d219a4d421f2311ea8f17863233630c1ed8ba4a` | 1 | 1 | 0 | 0 |
| `andromeda-pulse-mcp` | 2026-10-07 09:41:15 +02:00 | `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` | 0 | 0 | 0 | 0 |

Both mtimes follow `1a2e509`'s commit time (2026-10-07 08:16:59 +02:00).

### The proof the section carries (plan step 5's rule, applied to the two tables)

| Binary | String | `5f77859` build | `f70be92` build | Set |
|---|---|---|---|---|
| `pulse-app` | `must name that scope_id value exactly as` | 0 | 1 | present |
| `pulse-app` | `whose name merely contains it` | 0 | 1 | present |
| `pulse-app` | `v1.4-fallback` | 1 | 0 | absent |
| `pulse-app` | `v1.4-reflection` | 1 | 0 | absent |

- **`pulse-app`: the CONTENT arm holds.** The present set is both sentence strings and the absent set is both retired
  version literals; neither set is empty, so the section names all four. sha256 `f69be5bb…` before, `df167647…`
  after.
- **The sidecar: the PROVENANCE arm holds.** Build inputs clean at `f70be92` before and after the build; its build
  command at exit 0; mtime 2026-10-07 09:41:15 +02:00, after the commit. All four strings read 0 on both builds.
  sha256 `6175fc36…` before and `6175fc36…` after: **the digest did not move.** That is not a failure — none of the
  moved text is in that binary (research.md, the eleven-string table), so the relinked file is byte-identical.

## Pre-registration (plan steps 6-7)

The contract section §The 2026-10-07 series was written AFTER the two-sided control above, so the binary proof it
carries was measured on both builds before it was pre-registered. It is add-only: the contract's diff against the
chunk base `29adafa` reads 115 added and 0 deleted lines, and the contract names 6 distinct environment handles,
as it did at the chunk base.

Its digest is the `contract_section` form — the file LF-normalized, from the section's heading line up to and
including the newline before the next `## ` heading — computed by a scratchpad script. The recipe was validated
first: the same script over §The 2026-10-06 series prints `00173912…8097` (8663 bytes) and over §The 2026-10-01
series `0232afb1…e841`, the digests those series' ledgers record.

Recorded 2026-10-07T07:43:19Z, before `d1` and before the launch (9299 bytes). The section is not edited after
this line.

pre-registration sha256: 57b5e96e60aea77486c3b1a3ace2dc09ffc7e43154d874bd3be3a5d12aec7376

## Pre-leg env checks (plan step 8, entries 10-12, fired by hand 07:43:40Z in a shell that sourced the leg env script)

- `ANDROMEDA_PULSE_MODEL_PATH` resolves to a file and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` to an executable:
  `model env present`, exit **0**.
- The model file's sha256: `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` — **equal** to the
  pinned digest (exit 0).
- NVIDIA kernel-module major against the userspace major: `kernel=610 userspace=610`, exit **0**.
- The leg env script exports four handles, read back by name: `ANDROMEDA_PULSE_MODEL_PATH`,
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`, `ANDROMEDA_LLAMA3_TOKENIZER_PATH`.

## The dir (plan step 9, entry 13, fired by hand 07:43:40Z)

The parent `pulse-legs` under the user's cache dir listed one entry, `rm-trigger-series` (the 2026-10-06 series'
dir, not touched), and `rm-fifth-series` was absent: `leaf absent`, exit **0**. Created with `mkdir -p` at 07:43:50Z
(exit 0, empty). The parent now holds the two leaves. No recursive delete anywhere.

## Conductor's leg binaries (plan step 10, 07:43:52Z)

`cargo build -q -p conductor-cli --bin conductor` exit 0; `cargo test -p conductor-run --features live-pulse --test
real_model_live --no-run` exit 0. No compile falls inside the round. The round's selection for `d1` was walked with
the gate tool's `--live-legs --entry 16 --dry-run` (nothing fired): `round: DRY-RUN · legs 1`, entries 14-16
selected, no pre-flight refusal.

## The model-run slot and the launch (plan steps 11-12)

**The go:** the overseer, founder-delegated, 2026-10-07 09:45 local, asked immediately before the launch (the chunk's
inputs record it as I18): `d1`, `d2`, `d3` in this one sitting, on the founder's word of 07:42 local that GPU runs
are allowed today. The builds and the launch stand on the same overseer's earlier grant (inputs I2 and I17).

- **Pre-series census** (07:45:07Z, the round's census command by hand): the pattern
  `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched 0 processes; 0 listeners on `:4317` or `:4318`
  (exit 1, the command's no-app reading).
- **Launch** (the agent, a scratchpad script: `setsid nohup`, cwd = the data dir, outside this repo): the rebuilt
  `pulse-app` by path from `target/release` (sha256 `df167647…`, re-read at the launch), PID 3117394, started
  2026-10-07T07:45:11Z. ONE env block, echoed back by name:
  - `ANDROMEDA_PULSE_DATA_DIR` — a directory whose leaf is `rm-fifth-series`;
  - `ANDROMEDA_PULSE_MODEL_PATH` — a file named `gemma-4-E4B-it-Q4_K_M.gguf`;
  - `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — an executable named `llama-cli`;
  - the leg env script's two other handles, as sourced: `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` and
    `ANDROMEDA_LLAMA3_TOKENIZER_PATH`;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC` and `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` removed, both read back
    unset; `ANDROMEDA_PULSE_MCP_ENABLED` unset on the app's side; no other `ANDROMEDA_` or `CONDUCTOR_` name;
  - **no host rendering lever**: `WEBKIT_DISABLE_DMABUF_RENDERER` unset, `__NV_DISABLE_EXPLICIT_SYNC` not preset.
- **Liveness** (07:45:30Z, 19 s after the start): the process alive with five WebKit children (3117493, 3117502,
  3117503, 3117504, 3117505); listeners on `127.0.0.1:4317` and `127.0.0.1:4318`; `:4317` accepted a connection.
  **No relaunch**: the app stayed up at its own posture, so the plan's one-relaunch arm was never used and Pulse's
  log carries a single boot. Its stderr held one message, a tray-library deprecation warning. The red `boot smoke`
  job of Pulse's CI on `f70be92` (the app booting, then exiting) did not reproduce on this host.
- **Booted posture, from Pulse's own log** (`agent-latest.jsonl.2026-10-07`, 524 lines, read 07:45:43Z, before `d1`):
  - `interpretation.model.load` `inference_mode` **`real`**; `load_status` `loaded`, `model_identity`
    **`gemma-4-E4B-it-Q4_K_M`**, tier `primary`;
  - `app.boot.workspace_key` `workspace_root_basename` **`rm-fifth-series`** (the leaf), `key_bytes` 48;
  - `app.boot.otlp.grpc.bind` and `app.boot.otlp.http.bind` at 07:45:11.977Z;
  - **0** `triage.baseline.bootstrap_window.override` lines;
  - the render-posture line Pulse logged: `app.boot.render.posture` `lever` `__NV_DISABLE_EXPLICIT_SYNC`, `posture`
    `applied` (07:45:11.635Z) — the app set its own lever.
- Conductor's side for every drive: `ANDROMEDA_PULSE_DATA_DIR` the same dir, `ANDROMEDA_PULSE_MCP_ENABLED=true`, the
  L4 handle absent, the sidecar on `PATH` from `target/release` (sha256 `6175fc36…`).

## The drives (plan step 13)

Each drive is one firing of the round tool on that drive's `live` entry: the round steps since the previous leg (the
quiet window, a census, the non-priming probe `conductor preconditions --for real-model-interpretation`), then
`bash scripts/agent-run.sh run --live real-model` and the copy of `runs/live-suite/rm-capture.txt`. Each firing's
entry lines and summary line are kept verbatim in `round-{HHMMSSZ}.txt` beside this ledger, named by the firing's
start. Grades are the harvest's, computed by the unchanged rule over the pinned files (plan step 14), never judged
here.

The real-model canary emits three storms 90 s apart. Its third lands at the scenario's emission instant, so its
digest's tick falls after that instant and the capture prints two `canary:` lines per drive; the third storm's
outcome is read from Pulse's own log in each row, paired by Pulse's serial inference order.

| Label | Slot · timing · run | Route | Grade (P-033) | P-031 / P-034 / P-044 | `canary:` tokens (with `skip_reason`) | The scenario's digest, from Pulse's own log | Key rendering | Census |
|---|---|---|---|---|---|---|---|---|
| **d1** — `rm-capture-d1.txt` | the overseer's go; the agent's `pulse-app` (PID 3117394); census and probe green 07:45:55Z (`[PRECONDITION] every live-Pulse precondition is satisfied`, exit 0); fired 07:45:55Z, ended 07:51:57Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-07T07-45-56-326`; `[MANUAL] real-model-interpretation`; 271 self-obs lines frozen; envelope `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 2 attributed on the first poll, `degraded_mode=false`, pickup 7144 ms | **`Identified`** | `Pass` / `Pass` / `Blocked` (no prior same-scope incident to retrieve) | `surfaced` (07:46:41Z, created) · `surfaced` (07:48:11Z, deduped) — `skip_reason=none` both; 0 other cue-bearing digests | emission instant 07:49:41.416Z; storm Suggested 07:49:43.923Z, Autonomous 07:49:46.431Z; tier-1 `retry_storm` digest 07:49:46.437Z, picked up 07:49:48.560Z behind the canary's third storm, parse `ok` 07:49:54.314Z, **created** 07:49:54.373Z (severity `warn`, tier `suggested`) — surfaced. The canary's third storm (Autonomous 07:49:41.421Z, digest 07:49:41.428Z) parsed `ok` 07:49:48.526Z and deduped 07:49:48.560Z. `prompt_version` `v2.6` on all 10 prompt assemblies; parse `ok` 10 of 10; 147 append lines, 0 rejections. Skip lines in the window: 6, all on cue-less tier-3 digests — 5 `no_cue`, 1 `decision_dismiss` (07:46:16Z, before the first storm). WARN lines: 1 `digest.lww.drop`, 3 `mcp.tools.call.error` | `verbatim` | before: the agent's `pulse-app` and its five WebKit children only, 2 listeners; the round tool reported no survivor |
| **d2** — `rm-capture-d2.txt` | the same go and launch; quiet window: 180 s slept from 07:52:16Z, d1's last created incident 07:49:54.373Z, d2's first canary storm 368 s after it; census and probe green 07:55:16Z (exit 0); fired 07:55:17Z, ended 08:01:19Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-07T07-55-17-733`; `[MANUAL] real-model-interpretation`; 275 self-obs lines frozen; envelope `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 5 attributed on the first poll, `degraded_mode=false`, pickup 11302 ms | **`Identified`** | `Pass` / `Pass` / `Pass` | `surfaced` (07:56:02Z, created) · `surfaced` (07:57:32Z, deduped) — `skip_reason=none` both; 0 other cue-bearing digests before the emission instant | emission instant 07:59:02.779Z; storm Suggested 07:59:05.281Z, Autonomous 07:59:07.780Z; tier-1 `retry_storm` digest 07:59:07.786Z, picked up 07:59:14.081Z behind two queued digests, parse `ok` 07:59:19.267Z, **created** 07:59:19.325Z (severity `error`, tier `autonomous`) — surfaced. Ahead of it in Pulse's serial queue (the pairing rests on that order; Pulse's lines carry no digest identity): the canary's third storm (digest 07:59:02.787Z) deduped 07:59:08.716Z, and a tier-2 cue-bearing digest (07:59:02.982Z) created its own incident 07:59:14.081Z (incident 4), which carries no scenario fingerprint and is not the attributed one. `prompt_version` `v2.6` on all 11 prompt assemblies; parse `ok` 11 of 11; 147 append lines, 0 rejections. Skip lines in the window: 5, all `no_cue` on cue-less tier-3 digests. WARN lines: 13 `digest.lww.drop`, 3 `mcp.tools.call.error` | `verbatim`; the `## Previously Seen` suffix of d1's incident prints `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor |
| **d3** — `rm-capture-d3.txt` | the same go and launch; quiet window: 180 s slept from 08:01:41Z, d2's last created incident 07:59:19.325Z, d3's first canary storm 369 s after it; census and probe green 08:04:41Z (exit 0); fired 08:04:42Z, ended 08:10:45Z (363 s, exit 0, both atoms, the capture fresh); run `2026-10-07T08-04-42-988`; `[MANUAL] real-model-interpretation`; 271 self-obs lines frozen; envelope `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 7 attributed on the first poll, `degraded_mode=false`, pickup 8158 ms | **`NotIdentified`** — rank 1 names a retry storm and the service only as the hyphenated canary identity, which the rule separates from `conductor` | `Pass` / `Pass` / `Pass` | `surfaced` (08:05:28Z, created) · `surfaced` (08:06:58Z, deduped) — `skip_reason=none` both; the capture also counts 4 other cue-bearing digests before the emission instant, all `error_rate_spike` (tier 2), each of which Pulse logged `deduped` | emission instant 08:08:28.082Z; storm Suggested 08:08:30.589Z, Autonomous 08:08:33.367Z; tier-1 `retry_storm` digest 08:08:33.376Z, picked up 08:08:36.240Z behind the canary's third storm, parse `ok` 08:08:42.688Z, **created** 08:08:42.887Z (severity `warn`, tier `suggested`) — surfaced. The canary's third storm (Autonomous 08:08:28.082Z, digest 08:08:28.087Z) parsed `ok` 08:08:36.218Z and deduped 08:08:36.240Z. `prompt_version` `v2.6` on all 16 prompt assemblies; parse `ok` 16 of 16; 147 append lines, 0 rejections. Skip lines in the window: 6, all `no_cue` on cue-less tier-3 digests. WARN lines: 16 `digest.lww.drop`, 3 `mcp.tools.call.error` | `verbatim`; both `## Previously Seen` suffixes print `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor; then the teardown below |

Three drives fired in one sitting, 07:45:55Z to 08:10:45Z. No canary read `pipeline-fault`, so nothing was re-fired; no
fourth drive. Across the three leg windows Pulse logged 37 prompt assemblies, all `v2.6`, and 37 parses, all `ok`;
no `interpretation.inference.error` line and no refused append. The `model_identity` Pulse logged is the boot line's
`gemma-4-E4B-it-Q4_K_M`. Its WARN lines in those windows were queue replacements (`digest.lww.drop`: 1, 13, 16) and
three `mcp.tools.call.error` per drive; no ERROR line.

## Teardown (after d3)

The agent stopped what it started. `pulse-app` PID 3117394 (started 07:45:10Z by the process table, 07:45:11Z by the
launch script's stamp), re-read by PID, start time and name before the signal: `kill -TERM` at 08:11:04Z, exit 0;
the process was gone within 1 s. Post-series census (plan entry 25, by hand, 08:11:07Z): the pattern
`pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched **0** processes and **0** listeners remained on
`:4317` or `:4318` (exit 1, last line `0` — the entry's atoms hold). That equals the pre-series baseline.

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` 3117394 | the agent, on the overseer's go | terminated (SIGTERM, gone within 1 s) |
| its five WebKit children (3117493, 3117502, 3117503, 3117504, 3117505) | `pulse-app` | terminated with it; absent from the post-census |
| `conductor` and `andromeda-pulse-mcp`, one pair per leg | the legs | exited with each leg; absent from every pre-leg census and from the post-census; the round tool reported no survivor on any firing |
| `llama-cli`, one per inference | `pulse-app` | each exited with its inference; absent from the post-census |
| the two Pulse release builds' `cargo` and `rustc` | the agent, on the overseer's build grant | exited with the builds (exit 0 each) |

## Grading (plan step 14) and the series verdict (the pre-registered rule, applied mechanically)

The grades in the table above are the harvest's (`each_2026_10_07_drive_grades_as_the_ledger_records`), computed by
the unchanged rule over the pinned files. They were read red before green: the `measured` table was first written
wrong on purpose in every field of every drive, and each value was then taken from the grading arm's own failure
output, one field per pass — route, grade, the further-grade triple and the canary counts of `d1`, then of `d2`,
then of `d3` (eleven red passes of the arm after the first). The graded set was read the same way, from a verdict
test first asserting an empty set. The harvest then ran green: 119 tests (112 at the chunk base, plus this series'
seven; no ref test). Every envelope: `seed` 4317033, `state` `ManualCheck`, `verdict` null. Every capture reads
`pulse-log inference_mode: real` and `pulse-log workspace basename carries conductor: false`.

Capture pins (sha256 of each committed file; all three are LF):

| Capture | sha256 |
|---|---|
| `rm-capture-d1.txt` | `1f434d2b818f6c2bb0ce184ef83af7bc9d309a951e93f32515fc224f6d0f8770` |
| `rm-capture-d2.txt` | `f5a2cc300e1f9236d6f5e0f9bda6fc9de561b28685a9c9a508eb661330ce791d` |
| `rm-capture-d3.txt` | `cdc403cbcab2b2e92bbd28e5e14fda9880c42406b492e151e18a259071c33815` |

`COMMITTED_CAPTURES` moved 17 → 20: exactly the three `rm-capture*.txt` files this chunk commits.

Three drives fired, none re-fired, no fourth. **Graded drives: 3** — d1 `Identified`, d2 `Identified`, d3
`NotIdentified`. Under §The drive series (a), carried unchanged into §The 2026-10-07 series, a graded `NotIdentified`
means **`v3-09` NOT MET** (`v3_09_is_not_met_by_the_2026_10_07_series`). Recorded, never replaced. No ref test is
written.

Key rendering: `verbatim` on all three graded drives, so no drive is recorded as contaminated.

What the three drives measured, and nothing wider, on the shipped model at prompt `v2.6`:
- every canary digest the capture classified surfaced (6 of 6 `canary:` tokens, `skip_reason=none`), and the
  scenario's own storm digest surfaced in every drive (3 of 3), each attributed on the first poll;
- all three rank-1 statements carry a retry token, in the trigger's words;
- the service facet, which the new sentence now instructs, is where the series split again: d1 and d2 name the
  cue's `scope_id` value as a whole word; d3 names the service only as the hyphenated canary identity — the case
  the sentence rules out and the section's stated limit grades `NotIdentified`. d3's rank-1 rationale line does
  carry the `scope_id` value; the rule grades the rank-1 statement;
- so the instruction was followed on two drives of three. The same split (d1 and d2 `Identified`, d3
  `NotIdentified`, on the same miss) was recorded on 2026-10-06 at prompt `v2.5`; three drives per series do not
  separate the two prompts;
- every no-incident outcome Pulse logged in the three leg windows was on a cue-less tier-3 digest (`no_cue` 16,
  `decision_dismiss` 1).

### A measured covariate across the two series — not a cause

Recorded on the overseer's word (founder-delegated, 2026-10-07; the chunk's inputs record it as I19), who measured
it across both series; re-read by the agent from the six committed captures before it was written here. Each
capture prints one `creating digest corpus retrieval rows:` line: the rows the scenario's creating digest
retrieved from the corpus.

| Series · prompt | Drive | Creating digest's corpus rows | `## Previously Seen` lines | Grade |
|---|---|---|---|---|
| 2026-10-06 · `v2.5` | d1 | 1 | 0 | `Identified` |
| 2026-10-06 · `v2.5` | d2 | 3 | 1 | `Identified` |
| 2026-10-06 · `v2.5` | d3 | 6 | 2 | `NotIdentified` |
| 2026-10-07 · `v2.6` | d1 | 1 | 0 | `Identified` |
| 2026-10-07 · `v2.6` | d2 | 3 | 1 | `Identified` |
| 2026-10-07 · `v2.6` | d3 | 6 | 2 | `NotIdentified` |

The miss is the drive whose creating digest retrieved 6 rows, both times; the drives with 1 and 3 rows read
`Identified` 4 of 4. This is a covariate and nothing more. In both series the row count is fixed by the drive's
position on the one shared dir, so it moves together with everything else position carries — the drive's ordinal,
the earlier incidents in the corpus, the app's uptime, the canary's history — and six drives cannot separate them.
No drive varied the row count at a fixed position, so the table does not say the rows cause the miss.


`crates/conductor-tauri/src/commands.rs` carries one added comment line above `fixture_scenarios_dir`'s doc
comment: the `andromeda:walks-tree` token and one clause. The file differs from `29adafa` by 1 added and 0 deleted
lines (entry 38 reads `1/0`); the marked `.rs` files under `crates` number 9 (entry 37).

## Status smoke (plan entry 45), fired by hand after d3

`bash scripts/agent-run.sh status 2026-10-07T08-04-42-988` (d3's run_id, minted this session; its journal was
written at d3's end), fired 08:17:10Z: exit 0; atom `contains "scenario": "real-model-interpretation"` held
(1 hit); the envelope read back that `run_id`, `seed` 4317033, `state` `ManualCheck`, `verdict` null.

## Evidence hygiene (plan step 16, entries 43-44, through the gate tool)

- The host-path probe over every file of this evidence dir, with the home-rooted and temp-rooted shapes in its
  pattern: **0** at exit 1 (the entry's atoms hold).
- The workspace-key probe over the three captures: **0** occurrences of the leaf at exit 1 (the entry's atoms
  hold). The key reaches a capture only as its placeholder; each `## Previously Seen` suffix prints `<redacted>`.

## The standing gates (plan step 17, entries 26-44, through the gate tool, 08:17Z)

19 entries fired, 19 green, 0 red, in one pass: the real-model harvest 119 of 119; `cargo test -p conductor-run`
376 passed across 27 result lines (the harvest's 119 among them); `cargo test -p conductor-tauri` 27 of 27;
`capture_paths_guard` 8 of 8; the bundled default exit 0 (workspace nextest 1219 of 1219, the doctests, the
workspace lint and both feature-gated lint lines); `cargo fmt --all --check` clean; the package count 562; the
rule diff empty; the contract's deleted-line count 0 and its handle census 6; the frozen-path and frozen-chunk
diffs empty; the advisory-db porcelain empty, then `cargo audit` exit 0 (1290 advisories loaded, 562 crate
dependencies, 7 allowed warnings: 6 `unmaintained`, 1 `unsound`) and `cargo deny check advisories bans licenses
sources` exit 0.

## The operator pass (plan entries 46-48), on the overseer's word

The overseer (founder-delegated), 2026-10-07, after verifying the implement report against the captures (the
chunk's inputs record it as I19): the operator pass as planned — hygiene, the pre-CI commit and guarded push, the
CI read, with one runner flake the agent may re-run once — performed by the agent on that word, then stop before
the wrap.

- Entry 46, `python -X utf8 "$HOME"/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, fired bare at
  08:21:02Z before the commit: exit 0; atom `contains hygiene: clean` held (1 hit) — `hygiene: clean`, 55 files read
  (runs 40, evidence 7, inputs 8), 14 trails and 6 verbatim input copies not read by P1, 0 host paths kept.
