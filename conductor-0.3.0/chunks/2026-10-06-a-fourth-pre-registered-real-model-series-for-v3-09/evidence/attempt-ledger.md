# Attempt ledger — the 2026-10-06 real-model series

One row per drive fired, in firing order, under `contracts/pulse-real-model-leg-posture.md` §The 2026-10-06 series
(fixed before `d1`). Every drive is recorded; a canary-blocked drive is a measurement, never graded. Authored by
the agent from each plan entry's recorded result, each drive's capture copy, Pulse's own log and its census; no Pulse
file is edited. This ledger records handle NAMES, the data dir's leaf, digests and booleans, never a host path.

## Gate re-check (plan step 1, entries 1-4, fired by hand 2026-10-06T19:57:33Z, read at Pulse's committed state, no fetch)

- `5f77859` is an ancestor of Pulse's upstream (the local tracking ref of `chore/migrate-pulse-to-v3`):
  `merge-base --is-ancestor` exit **0**.
- Commits on Pulse's checked-out branch not on its upstream: **0** (exit 0).
- Pulse's build inputs (`crates`, `pulse-app`, `Cargo.toml`, `Cargo.lock`) with an uncommitted or untracked change:
  **0 lines** (exit 0). The tree's only dirty paths are wrap bookkeeping (three modified, one untracked, all under
  its `.andromeda` and `.claude` dirs).
- Pulse `HEAD`: `5f77859f8ebbb18fe01f6394a6f94bfc66b9ef34` (exit 0) — the commit the series is pinned to, committed
  2026-10-05 18:22:00 +02:00.

## The binaries (plan step 5, entries 5-11, on the overseer's build grant)

**Grant:** the overseer, founder-delegated, 2026-10-06 (the P4 fork round's answer): both release builds in the Pulse
tree and the launch are the agent's; pulse-builder stays idle. Re-read by the agent after both builds (20:02:57Z):
HEAD `5f77859`, build-input porcelain 0 lines.

Both builds ran in the Pulse tree with the leg env script sourced (so the build reads the local tokenizer), started
19:57:56Z, one after the other. They ran while the contract section was being written, not after it: a concurrency
the plan's step order does not state, with no effect on what `d1` reads.

| Binary | Command (in the Pulse tree) | Exit · `Finished` | Built | sha256 | `hypotheses-item-statement-kv` | `--json-schema-file` | `--grammar-file` |
|---|---|---|---|---|---|---|---|
| `pulse-app`, before (the control) | — | — | 2026-10-04 00:12:33 +02:00 | `23f6ef2bd0b854d9697a4d203b3aea1bf6f40b146420de362112556914976ada` | 0 | 1 | 0 |
| `pulse-app` | `cargo build --release -p pulse-app` | 0 · `release` profile in 2m 25s | 2026-10-06 22:00:21 +02:00 | `f69be5bb8643a479649149f06aef30345b17f8279af9fa0aea6039034f19cf44` | **2** | **0** (exit 1) | 0 (exit 1) |
| `andromeda-pulse-mcp`, before (the control) | — | — | 2026-10-03 23:52:23 +02:00 | `e64f3688ec14144910ab25d461ec4ea99c161fda2e1b8824a3d480f17021e02e` | — | — | 0 (P4 reading) |
| `andromeda-pulse-mcp` | `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 0 · `release` profile in 2m 24s | 2026-10-06 22:02:46 +02:00 | `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` | 0 | 0 | 0 |

- **Entry 8 read red, and its atom cannot hold on a correct build.** `grep -c -a -F -e '--grammar-file'` over the
  rebuilt `pulse-app` reads **0** at exit 1. The source carries the argument at `5f77859`; the binary does not hold
  its 14 bytes as one run. The compiler materializes it from two 8-byte immediates: the halves `--gramma` and
  `mar-file` each read 1 on the rebuilt binary and 0 on the pre-build one. The sibling short arguments (`--top-k`,
  `--min-p`) read 0 the same way. So the string discriminates nothing: it reads 0 before and after.
- **The pre-build controls were measured, not carried.** Cargo kept the pre-build artifacts beside the new ones under
  its `deps` dir; the `pulse-app` one hashes to the pre-build reading above (`23f6ef2b…`), and the "before" row's
  three counts were read from it at 20:04Z.
- **The corrected content pair** (the overseer's word, founder-delegated, 2026-10-06, before any drive; the contract
  section amended, below): `hypotheses-item-statement-kv` — a rule name of the embedded grammar the `--grammar-file`
  argument passes, in 0 files at `a2addb3` and 1 at `5f77859` — reads 0 → **2**, and `--json-schema-file` reads
  1 → **0**. **`pulse-app`: the CONTENT arm holds.** Two further strings new at `5f77859` agree, recorded and not part
  of the pair: the v2.5 instruction's words `first hypothesis statement must name that signal` 0 → 1, and the corpus
  framing note `other or past incidents - context only` 0 → 1.
- **The sidecar: the PROVENANCE arm holds**, as P4 predicted. Entry 10's content reading over the rebuilt sidecar:
  `--grammar-file` 0, the corpus framing note 0, `retrieve_incident_events` 1; the grammar rule name 0. No string
  discriminates it. Its basis: build inputs clean at `5f77859` before and after the build; the build command above,
  exit 0; sha256 `6175fc36…`, different from the pre-build `e64f3688…`; mtime 2026-10-06 22:02:46 +02:00, after the
  commit (2026-10-05 18:22:00 +02:00).

Cargo's raw output stays out of evidence (it names host paths); only the exit and the `Finished` line are recorded.

## Pre-leg env checks (plan step 6, entries 12-14, fired by hand 19:59Z in a shell that sourced the leg env script)

- `ANDROMEDA_PULSE_MODEL_PATH` resolves to a file and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` to an executable:
  `model env present`, exit **0**.
- The model file's sha256: `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` — **equal** to the
  pinned digest (exit 0).
- NVIDIA kernel-module major against the userspace major: `kernel=610 userspace=610`, exit **0**.

## The dir (plan step 7, entry 15)

The parent `pulse-legs` under the user's cache dir did not exist (its listing printed nothing), so
`rm-trigger-series` was absent: `leaf absent`, exit **0**. Created with `mkdir -p` (exit 0, empty). The parent now
holds that one leaf. No recursive delete anywhere.

## Pre-registration (plan step 4, amended once before any drive)

The contract section §The 2026-10-06 series — LF-normalized, from its heading line up to the next `## ` heading,
the `contract_section` form — recorded before `d1` fired. The same computation over §The 2026-10-01 series
reproduces that ledger's recorded `0232afb1…e841`, which validates the recipe (re-run at both readings below).

- **As first written** (19:59:15Z, 7815 bytes): sha256
  `6c4590acc1ff6f7987885a6a526144a51156433df8b94b4662961b9f921cd220`. Superseded before any drive.
- **Why it changed.** Its `pulse-app` content proof named `--grammar-file` as the string a `5f77859` build carries.
  Entry 8 measured that string absent from a correct build (above), so the proof as written could not be met and
  discriminated nothing. The agent stopped before the launch and asked; the overseer (founder-delegated) ruled:
  amend the proof before `d1`, keep both digests and the reason here. The amendment replaces the present half with
  the grammar rule name and adds one dated `[corrected …]` note to the section. The grading rule, the pass
  condition, the drive count, the dir, the canary and re-fire rules and the launch posture are not touched.
- **As fixed before `d1`** (20:04:48Z, 8641 bytes), the digest the harvest recomputes. The contract's diff against
  the chunk base `0b07b2c` reads 111 added and 0 deleted lines.

pre-registration sha256: 00173912ffa684a4eebe120e45a9afb61c1766fff888bada4b5e713bb26f8097

## The model-run slot and the launch (plan steps 9-10)

**The go:** the overseer, founder-delegated, 2026-10-06, asked immediately before the launch (the chunk's inputs
record it as I20): `d1`, `d2`, `d3` in this one sitting with no confirmation between drives; the agent stops
`pulse-app` after `d3`. The builds and the launch stand on the same overseer's earlier grant (inputs I17).

- **Pre-series census** (20:05:34Z, the round's census command by hand): the pattern
  `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched 0 processes; 0 listeners on `:4317` or `:4318`
  (exit 1, the command's no-app reading).
- **Launch** (the agent, a scratchpad script: `setsid nohup`, cwd = the data dir, outside this repo): the rebuilt
  `pulse-app` by path from `target/release` (sha256 `f69be5bb…`, re-read at the launch), PID 4006473, started
  2026-10-06T20:05:39Z. ONE env block, echoed back by name:
  - `ANDROMEDA_PULSE_DATA_DIR` — a directory whose leaf is `rm-trigger-series`;
  - `ANDROMEDA_PULSE_MODEL_PATH` — a file named `gemma-4-E4B-it-Q4_K_M.gguf`;
  - `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — an executable named `llama-cli`;
  - the leg env script's two other handles, as sourced: `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` (the same executable)
    and `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (its build handle);
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC` and `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` removed, both read back
    unset; `ANDROMEDA_PULSE_MCP_ENABLED` unset on the app's side; no other `ANDROMEDA_` or `CONDUCTOR_` name;
  - **no host rendering lever**: `WEBKIT_DISABLE_DMABUF_RENDERER` unset, `__NV_DISABLE_EXPLICIT_SYNC` not preset.
- **Liveness at 10 s** (20:05:50Z): the process alive; listeners on `127.0.0.1:4317` and `127.0.0.1:4318`; `:4317`
  accepted a connection. **No relaunch**: the app stayed up at its own posture, so the plan's one-relaunch arm was
  never used and Pulse's log carries a single boot. Its stderr held one line, a tray-library deprecation warning.
- **Booted posture, from Pulse's own log** (`agent-latest.jsonl.2026-10-06`, 469 lines, read 20:06:09Z, before `d1`):
  - `interpretation.model.load` `inference_mode` **`real`**; `load_status` `loaded`, `model_identity`
    **`gemma-4-E4B-it-Q4_K_M`**, tier `primary`;
  - `app.boot.workspace_key` `workspace_root_basename` **`rm-trigger-series`** (the leaf), `key_bytes` 50;
  - `app.boot.otlp.grpc.bind` `127.0.0.1:4317` and `app.boot.otlp.http.bind` `127.0.0.1:4318` at 20:05:40.989Z;
  - **0** `triage.baseline.bootstrap_window.override` lines;
  - the render-posture line Pulse logged: `app.boot.render.posture` `lever` `__NV_DISABLE_EXPLICIT_SYNC`, `posture`
    `applied` (20:05:40.712Z) — the app set its own lever, as the plan's review predicted.
- Conductor's side for every drive: `ANDROMEDA_PULSE_DATA_DIR` the same dir, `ANDROMEDA_PULSE_MCP_ENABLED=true`, the
  L4 handle absent, the sidecar on `PATH` from `target/release` (sha256 `6175fc36…`).

## The drives (plan step 11)

Each drive is one firing of the round tool on that drive's `live` entry: the round steps since the previous leg (the
quiet window, a census, the non-priming probe `conductor preconditions --for real-model-interpretation`), then
`bash scripts/agent-run.sh run --live real-model` and the copy of `runs/live-suite/rm-capture.txt`. Each firing's
entry lines and summary line are kept verbatim in `round-{HHMMSSZ}.txt` beside this ledger. Grades are the
harvest's, computed by the unchanged rule over the pinned files (plan step 12), never judged here.

The real-model canary emits three storms 90 s apart. Its third lands at the scenario's emission instant, so its
digest's tick falls after that instant and the capture prints two `canary:` lines per drive; the third storm's
outcome is read from Pulse's own log in each row, paired by Pulse's serial inference order.

| Label | Slot · timing · run | Route | Grade (P-033) | P-031 / P-034 / P-044 | `canary:` tokens (with `skip_reason`) | The scenario's digest, from Pulse's own log | Key rendering | Census |
|---|---|---|---|---|---|---|---|---|
| **d1** — `rm-capture-d1.txt` | the overseer's go; the agent's `pulse-app` (PID 4006473); census and probe green 20:06:28Z (`[PRECONDITION] every live-Pulse precondition is satisfied`, exit 0); fired 20:06:28Z, ended 20:12:30Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-06T20-06-29-416`; `[MANUAL] real-model-interpretation`; 275 self-obs lines frozen | `ReadBack` — incident 2 attributed on the first poll, `degraded_mode=false`, pickup 5432 ms | **`Identified`** | `Pass` / `Pass` / `Blocked` (no prior same-scope incident to retrieve) | `surfaced` (20:07:14Z, created) · `surfaced` (20:08:44Z, deduped) — `skip_reason=none` both | emission instant 20:10:14.461Z; storm Suggested 20:10:16.963Z, Autonomous 20:10:19.466Z; tier-1 `retry_storm` digest 20:10:19.470Z, parse `ok` 20:10:26.234Z, **created** 20:10:26.284Z (severity `warn`) — surfaced. The canary's third storm (Autonomous 20:10:14.463Z, digest 20:10:14.468Z) parsed `ok` and deduped 20:10:19.893Z. `prompt_version` `v2.5` on all 10 prompt assemblies; parse `ok` 10 of 10; 0 append rejections. Skip lines in the window: 6, all on cue-less tier-3 digests — 5 `no_cue`, 1 `decision_dismiss` (20:06:45Z, before the first storm) | `verbatim` | before: the agent's `pulse-app` and its five WebKit children only, 2 listeners; the round tool reported no survivor |
| **d2** — `rm-capture-d2.txt` | the same go and launch; quiet window: 180 s slept from 20:12:55Z, d1's last created incident 20:10:26.284Z, d2's first canary storm 375 s after it; census and probe green 20:15:55Z (exit 0); fired 20:15:55Z, ended 20:21:59Z (363 s, exit 0, both atoms, the capture fresh); run `2026-10-06T20-15-56-338`; `[MANUAL] real-model-interpretation`; 279 self-obs lines frozen | `ReadBack` — incident 5 attributed on the first poll, `degraded_mode=false`, pickup 22684 ms | **`Identified`** | `Pass` / `Pass` / `Pass` | `surfaced` (20:16:41Z, created) · `surfaced` (20:18:11Z, deduped) — `skip_reason=none` both | emission instant 20:19:41.384Z; storm Suggested 20:19:43.964Z, Autonomous 20:19:47.224Z; tier-1 `retry_storm` digest 20:19:47.229Z, picked up 20:20:04.068Z behind two queued digests, parse `ok` 20:20:10.469Z, **created** 20:20:10.519Z (severity `error`, tier `autonomous`) — surfaced. Ahead of it in Pulse's serial queue (the pairing rests on that order; Pulse's lines carry no digest identity): the canary's third storm (digest 20:19:41.392Z) deduped 20:19:54.969Z, and a tier-2 cue-bearing digest (20:19:42.020Z) created its own incident 20:20:04.068Z, which carries no scenario fingerprint and is not the attributed one. `prompt_version` `v2.5` on all 12 prompt assemblies; parse `ok` 12 of 12; 0 append rejections. Skip lines in the window: 5, all on cue-less tier-3 digests — 4 `no_cue`, 1 `decision_dismiss` (20:16:44Z) | `verbatim`; the `## Previously Seen` suffix of d1's incident prints `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor |
| **d3** — `rm-capture-d3.txt` | the same go and launch; quiet window: 180 s slept from 20:22:28Z, d2's last created incident 20:20:10.519Z, d3's first canary storm 364 s after it; census and probe green 20:25:28Z (exit 0); fired 20:25:28Z, ended 20:31:30Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-06T20-25-29-351`; `[MANUAL] real-model-interpretation`; 279 self-obs lines frozen | `ReadBack` — incident 7 attributed on the first poll, `degraded_mode=false`, pickup 5893 ms | **`NotIdentified`** — rank 1 names a retry storm and the service only inside the hyphenated canary identity, which the rule separates from `conductor` | `Pass` / `Pass` / `Pass` | `surfaced` (20:26:14Z, created) · `surfaced` (20:27:44Z, deduped) — `skip_reason=none` both; the capture also counts 4 other cue-bearing digests before the emission instant, all `error_rate_spike` (tier 2), each of which Pulse logged `deduped` | emission instant 20:29:14.397Z; storm Suggested 20:29:16.899Z, Autonomous 20:29:19.398Z; tier-1 `retry_storm` digest 20:29:19.404Z, parse `ok` 20:29:25.683Z, **created** 20:29:25.740Z (severity `error`, tier `autonomous`) — surfaced. The canary's third storm (digest 20:29:14.404Z) parsed `ok` and deduped 20:29:20.290Z. `prompt_version` `v2.5` on all 16 prompt assemblies; parse `ok` 16 of 16; 0 append rejections. Skip lines in the window: 6, all `no_cue` on cue-less tier-3 digests | `verbatim`; both `## Previously Seen` suffixes print `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor; then the teardown below |

Three drives fired in one sitting, 20:06:28Z to 20:31:30Z. No canary read `pipeline-fault`, so nothing was re-fired; no
fourth drive. Across the three leg windows Pulse logged 38 prompt assemblies, all `v2.5`, and 38 parses, all `ok`;
no `interpretation.inference.error` line and no refused append. Its WARN lines in those windows were queue
replacements (`digest.lww.drop`: 1, 12, 16) and three `mcp.tools.call.error` per drive.

## Grading (plan step 12) and the series verdict (the pre-registered rule, applied mechanically)

The grades in the table above are the harvest's (`each_2026_10_06_drive_grades_as_the_ledger_records`), computed by
the unchanged rule over the pinned files. They were read red before green: the `measured` table was first written
wrong on purpose in every field, and each value was then taken from the grading arm's own failure output — every
route, grade, further-grade triple and canary count of `d1` and `d2`, and all of `d3` but its route, which was set to
the value the first two drives measured and then held by the arm's route assertion. The graded set was read the same
way, from a verdict test first asserting an empty set. Every envelope: `seed` 4317033, `state` `ManualCheck`,
`verdict` null. Every capture reads `pulse-log inference_mode: real`.

Capture pins (sha256 of each committed file; all three are LF):

| Capture | sha256 |
|---|---|
| `rm-capture-d1.txt` | `27a1222e5f7cce189c8dbe72a74bbd41309d1fcfee4ceec9ad47b74638b9da69` |
| `rm-capture-d2.txt` | `fb77ce86f21329c33b1a4fa4291c75682a14f05d9709f07acce8bb1b1490c192` |
| `rm-capture-d3.txt` | `12fcffb39dfccbad58eec1215b569e8b13e901ed5c8963d784abe864d689a8ae` |

Three drives fired, none re-fired, no fourth. **Graded drives: 3** — d1 `Identified`, d2 `Identified`, d3
`NotIdentified`. Under §The drive series (a), carried unchanged into §The 2026-10-06 series, a graded `NotIdentified`
means **`v3-09` NOT MET** (`v3_09_is_not_met_by_the_2026_10_06_series`). Recorded, never replaced. No ref test is
written.

Key rendering: `verbatim` on all three graded drives, so no drive is recorded as contaminated.

What the series measured beyond the verdict, on the shipped model at prompt `v2.5`:
- every canary digest the capture classified surfaced (6 of 6 `canary:` tokens, `skip_reason=none`), and the
  scenario's own storm digest surfaced in every drive (3 of 3), each attributed on the first poll;
- all three rank-1 statements carry a retry token, the facet the trigger-framing instruction asks for;
- the service facet, which no instruction asks for, is where the series split: d1 and d2 name the service as a
  whole word; d3 names it only as the hyphenated canary identity. The stated limit in the contract section says
  exactly this case grades `NotIdentified`;
- every no-incident outcome Pulse logged in the three leg windows was on a cue-less tier-3 digest (`no_cue` 15,
  `decision_dismiss` 2).

## Teardown (after d3)

The agent stopped what it started. `pulse-app` PID 4006473 (started 20:05:39Z), re-read by PID, start time and name
before the signal: `kill -TERM` at 20:31:51Z, exit 0; the process was gone within 2 s. Post-series census (plan entry
27, by hand, 20:31:54Z): the pattern `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched **0** processes
and **0** listeners remained on `:4317` or `:4318` (exit 1, last line `0` — the entry's atoms hold). That equals the
pre-series baseline.

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` 4006473 | the agent, on the overseer's go | terminated (SIGTERM, gone within 2 s) |
| its five WebKit children (4006556, 4006568, 4006569, 4006570, 4006571) | `pulse-app` | terminated with it; absent from the post-census |
| `conductor` and `andromeda-pulse-mcp`, one pair per leg | the legs | exited with each leg; absent from every pre-leg census and from the post-census; the round tool reported no survivor on any firing |
| `llama-cli`, one per inference | `pulse-app` | each exited with its inference; absent from the post-census |
| the two Pulse release builds' `cargo` and `rustc` | the agent, on the overseer's build grant | exited with the builds (exit 0 each) |

## Status smoke (plan entry 44), fired by hand after d3

`bash scripts/agent-run.sh status 2026-10-06T20-25-29-351` (d3's run_id, minted this session; its journal was
written at d3's end): exit 0; atom `contains "scenario": "real-model-interpretation"` held (1 hit); the envelope
read back that `run_id`, `seed` 4317033, `state` `ManualCheck`, `verdict` null.

## Evidence hygiene (plan step 15, entries 42-43, through the gate tool)

- The host-path probe over every file of this evidence dir, with the home-rooted and temp-rooted shapes in its
  pattern: **0** at exit 1 (the entry's atoms hold).
- The workspace-key probe over the three captures: **0** occurrences of the leaf at exit 1 (the entry's atoms
  hold). The key reaches a capture only as its placeholder; each `## Previously Seen` suffix prints `<redacted>`.
- No committed capture of any series holds a temp-rooted path (0 of 17 files), so the mask extension of plan
  step 13 moves no pin.

## The operator pass (plan entries 45-47), on the overseer's word

The overseer (founder-delegated), 2026-10-06, after verifying the implement report against the tree (the chunk's
inputs record it as I21): the operator pass as planned — hygiene, the pre-CI commit and guarded push, the CI read —
performed by the agent on that word, then stop before the wrap.

- Entry 45, `python -X utf8 "$HOME"/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, fired bare at
  20:43:55Z before the commit: exit 0; atom `contains hygiene: clean` held (1 hit) — `hygiene: clean`, 59 files read
  (runs 43, evidence 7, inputs 9), 15 trails and 7 verbatim input copies not read by P1, 0 host paths kept.
