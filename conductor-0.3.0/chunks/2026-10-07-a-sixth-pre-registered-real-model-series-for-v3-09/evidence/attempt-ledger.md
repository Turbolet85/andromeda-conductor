# Attempt ledger — the 2026-10-07 sixth real-model series

One row per drive fired, in firing order, under `contracts/pulse-real-model-leg-posture.md` §The 2026-10-07 sixth
series (fixed before `d1`). Every drive is recorded; a canary-blocked drive is a measurement, never graded. Authored
by the agent from each plan entry's recorded result, each drive's capture copy, Pulse's own log and its census; no
Pulse file is edited. This ledger records handle NAMES, the data dir's leaf, digests, counts and booleans, never a
host path.

## The word for the build (plan step 1)

The operator's word, given in the arguments of this chunk's implement on 2026-10-07 at about 23:01 local and kept
verbatim as the chunk's input I17: Pulse's wrap is committed and pushed as `f18c631` on top of `9bfefb8`, the diff
between the two over `crates pulse-app xtask` is empty, Part A runs tonight on the CPU only, and the chunk stops
before Part B. The same word covers both release builds in the Pulse tree. It asked the agent to verify that
nothing dirty in Pulse's tree is a build input; the readings below are that check.

## Gate re-check (plan step 1, entries 1-5, fired by hand 2026-10-07T21:01:51Z, read at Pulse's committed state, no fetch)

- `9bfefb8` is an ancestor of Pulse's upstream (the local tracking ref of `chore/migrate-pulse-to-v3`):
  `merge-base --is-ancestor` exit **0**.
- Commits on Pulse's checked-out branch not on its upstream: **0** (exit 0; the entry's `last line 0` atom holds).
- Pulse's build inputs (`crates`, `pulse-app`, `xtask`, `Cargo.toml`, `Cargo.lock`) with an uncommitted or untracked
  change: **0 lines** (exit 0; the entry's `no output` atom holds).
- Files under those build inputs that differ between `9bfefb8` and `HEAD`: **0** (exit 0; the entry's `no output`
  atom holds).
- Pulse `HEAD`: `f18c631bd545c50466c6177c5ae658dc61f7d2e7`, committed 2026-10-07T22:58:10+02:00, equal to its
  upstream's tracking ref. It is Pulse's wrap commit on top of the pin
  `9bfefb812297bbdea610423a21568b3262bdb7ed`, committed 2026-10-07T20:38:39+02:00. The binaries are built at
  `f18c631`; the series is pinned to `9bfefb8`.

Read beside the entries, by the agent:

- The tree's dirty paths were three modified bookkeeping files (the friction log, one evolve record under a run
  dir, the session handoff). None lies under a build input. After the builds a fourth row had appeared, an
  untracked run dir of a setup re-run, also outside the build inputs.
- What `f18c631` changes against `9bfefb8`: 48 paths, every one under `.andromeda`, `.claude`, the version
  workspace or the root `CLAUDE.md`; none under a build input.
- No `cargo`, `rustc` or `pulse-app` process was running on the host before the builds.
- Not covered by these probes: three git-ignored generated dirs sit under `pulse-app` (`gen`, `ui/dist`,
  `ui/node_modules`). They are not tracked, so no git probe reads them. The files they are generated from are
  unchanged since `f70be92` (the product diff between `f70be92` and `9bfefb8` lists two files under
  `crates/triage/src/digest` and two under `pulse-app/examples`, nothing else).

## The binaries (plan steps 3-5, entries 6-10, on the operator's word above)

### The old side of the control (entry 6, fired by hand 2026-10-07T21:02:12Z, BEFORE any build)

Report-only; the command's exit is its last `grep`'s (1) and is not asserted.

| Binary | Built | Size (bytes) | sha256 | `must name that scope_id value exactly as` | `whose name merely contains it` | `v1.4-fallback` | `v1.4-reflection` |
|---|---|---|---|---|---|---|---|
| `pulse-app` | 2026-10-07 09:40:59 +02:00 | 98326120 | `df1676477222776d3e95edae7d219a4d421f2311ea8f17863233630c1ed8ba4a` | 1 | 1 | 0 | 0 |
| `andromeda-pulse-mcp` | 2026-10-07 09:41:15 +02:00 | 47415880 | `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` | 0 | 0 | 0 | 0 |

Both digests equal the 2026-10-07 series' recorded rebuilt binaries, so these are the `f70be92` builds, and every
count equals the plan's expectation (research.md §The binaries on this host). The stop condition of step 3 did not
fire.

### The builds (entries 7-8, fired by hand, one after the other)

Both ran in the Pulse tree with the leg env script sourced (so the build reads the local tokenizer). Cargo's raw
output stays out of evidence (it names host paths); only the exit, the `Finished` line and the names of the crates
it compiled are recorded.

| Binary | Command (in the Pulse tree) | Started · ended | Exit · `Finished` | Crates compiled |
|---|---|---|---|---|
| `pulse-app` | `cargo build --release -p pulse-app` | 21:02:22Z · 21:04:05Z | 0 · `release` profile in 1m 43s | `triage`, `interpretation`, `mcp-server`, `ui-bridge`, `config-watcher`, `pulse-app` |
| `andromeda-pulse-mcp` | `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 21:04:11Z · 21:04:27Z | 0 · `release` profile in 15.68s | `triage`, `interpretation`, `mcp-server` |

Re-read by the agent after both builds (21:04:33Z): Pulse HEAD `f18c631bd545c50466c6177c5ae658dc61f7d2e7`,
build-input porcelain 0 lines.

### The new side of the control (entries 9-10, fired by hand 2026-10-07T21:04:33Z, AFTER the builds)

- Entry 9 (asserting): it printed `pulse-app=44f28608c4e9c33685d8bf1329a47aab1641d017d0516b7e7d2669e72f43428d` and
  `digest moved, both built after the commit`, exit **0**. Both atoms hold. Over the `f70be92` builds the same
  entry printed the old digest and exited 1 at its digest test (the plan's P4 control).
- Entry 10 (report-only; its exit is the last `grep`'s, 1, and is not asserted):

| Binary | Built | Size (bytes) | sha256 | `must name that scope_id value exactly as` | `whose name merely contains it` | `v1.4-fallback` | `v1.4-reflection` |
|---|---|---|---|---|---|---|---|
| `pulse-app` | 2026-10-07 23:04:05 +02:00 | 98326504 | `44f28608c4e9c33685d8bf1329a47aab1641d017d0516b7e7d2669e72f43428d` | 1 | 1 | 0 | 0 |
| `andromeda-pulse-mcp` | 2026-10-07 23:04:26 +02:00 | 47415880 | `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9` | 0 | 0 | 0 | 0 |

Both mtimes follow `9bfefb8`'s commit time (2026-10-07 20:38:39 +02:00).

### The proof the section carries (plan step 5's rule, applied to the two tables)

| Binary | Command | Exit · `Finished` | Built | sha256 before | sha256 after | Digest moved | Lineage strings before | after |
|---|---|---|---|---|---|---|---|---|
| `pulse-app` | `cargo build --release -p pulse-app` | 0 · 1m 43s | 2026-10-07 23:04:05 +02:00 | `df167647…ba4a` | `44f28608…428d` | **yes** | 1 / 1 / 0 / 0 | 1 / 1 / 0 / 0 |
| `andromeda-pulse-mcp` | `cargo build --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` | 0 · 15.68s | 2026-10-07 23:04:26 +02:00 | `6175fc36…d2a9` | `6175fc36…d2a9` | **no** | 0 / 0 / 0 / 0 | 0 / 0 / 0 / 0 |

- **`pulse-app`: the four facts hold.** The build inputs were clean and equal to `9bfefb8`'s (the gate re-check);
  its build command exited 0; its mtime follows `9bfefb8`'s commit time; its sha256 differs from the recorded
  `f70be92` build's. Its size moved too, by 384 bytes. Step 5's stop condition (an unmoved digest) did not fire.
- **The four lineage strings read the same on both builds**, as research predicted: they discriminate nothing
  between an `f70be92` build and this one, and no named content token does.
- **What the moved digest does not show.** No same-source rebuild of `pulse-app` has been compared on this host, so
  the digest is read together with its provenance and never alone. It is not a content proof that the remedy is in
  the binary.
- **The sidecar: build provenance holds, and its digest did not move.** The same clean inputs; its build command at
  exit 0, with `triage` recompiled inside it; an mtime after the commit. Its sha256 and its size are the `f70be92`
  build's. That is not a failure (the plan, step 5). Why the relinked file is byte-identical is not measured.

## Pre-registration (plan steps 6-7)

The contract section §The 2026-10-07 sixth series was written AFTER the two-sided control above, so the binary
proof it carries was measured on both builds before it was pre-registered. It carries the stop rule, written
before any launch. It is add-only: the contract's diff against the chunk base `902d12c` reads 115 added and 0
deleted lines, and the contract names 6 distinct environment handles, as it did at the chunk base. The secret-scan
gate read green over the tree that holds the section and this ledger (`conductor-core --test secret_scan_gate`, 7
of 7, before the digest was recorded).

Its digest is the `contract_section` form — the file LF-normalized, from the section's heading line up to and
including the newline before the next `## ` heading — computed by a scratchpad script. The recipe was validated
first: the same script over §The 2026-10-07 series prints `57b5e96e…7376` (9299 bytes) and over §The 2026-10-07
capture run `51389269…87da` (5436 bytes), each equal to its pin in code.

Recorded 2026-10-07T21:08:29Z, before `d1` and before any launch (9112 bytes). The section is not edited after
this line.

pre-registration sha256: 01cf94c55845834391edd14c041152ad1c3fad6d4b2b7377221d5d8488f44fb5

## The dir (plan step 8, entry 11, fired by hand 2026-10-07T21:08:06Z)

The parent `pulse-legs` under the user's cache dir listed three entries — `rm-fifth-series`, `rm-recorded-run`,
`rm-trigger-series`, the earlier dirs, not touched — and `rm-sixth-series` was absent: `leaf absent`, exit **0**.
Created with `mkdir -p` at 21:08:09Z (exit 0, empty). The parent now holds the four leaves. No recursive delete
anywhere.

## Conductor's leg binaries (plan step 9, 2026-10-07T21:08:14Z)

`cargo build -q -p conductor-cli --bin conductor` exit 0; `cargo test -p conductor-run --features live-pulse --test
real_model_live --no-run` exit 0. No compile falls inside the round. The round's selection for `d1` was walked with
the gate tool's `--live-legs --entry 17 --dry-run` (nothing fired): `round: DRY-RUN · legs 1`, entries 15-17
selected, no pre-flight refusal.

## Part A ends here (2026-10-07T21:08:29Z)

- Nothing was launched tonight and no GPU was used. Census at the end of Part A, the round's census command by
  hand: the pattern `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched 0 processes; 0 listeners on
  `:4317` or `:4318`.
- Part B has not started. It waits for daytime and the operator's go, on the operator's word of this run (input
  I17). The env checks (entries 12-14) are fired in the sitting's own shell, and `pulse-app`'s sha256 is re-read
  there against `44f28608…428d` before the launch.

## The go (plan step 10)

**The go:** the founder, by dialog on 2026-10-08 at 07:05 local, his own pick, "Да, запускай": the sixth series,
three drives on the real model, about 30 minutes of GPU, daytime. Relayed by the pc overseer (the chunk's input
I18) and stated in the operator's arguments of this implement re-entry (input I19, about 08:26 local). The launch
and the stop of `pulse-app` stand inside that grant.

## Re-read before the launch (2026-10-08T06:27:55Z, by the agent, a fresh session)

Part A was not redone. What the sitting re-read:

- **Pulse moved since Part A, in bookkeeping only.** Its `HEAD` is `18a872d0e4496b292f8bc10472da835bec0406c4`,
  committed 2026-10-08T07:09:46+02:00 (a setup commit), with the pin `9bfefb8` an ancestor (`merge-base
  --is-ancestor` exit 0). Files under the build inputs (`crates`, `pulse-app`, `xtask`, `Cargo.toml`,
  `Cargo.lock`) that differ between `9bfefb8` and that `HEAD`: **0** (exit 0). Build inputs with an uncommitted or
  untracked change: **0 lines** (exit 0). Against the build's own `HEAD` `f18c631` it changes 13 paths: 9 under
  `.andromeda`, 3 under `.claude`, and the root `CLAUDE.md`. So the binaries were **not rebuilt**: the launched
  `pulse-app` is the file Part A built, at `f18c631`.
- **Both binaries are the ones Part A recorded.** `pulse-app` sha256
  `44f28608c4e9c33685d8bf1329a47aab1641d017d0516b7e7d2669e72f43428d`, 98326504 bytes, built 2026-10-07 23:04:05
  +02:00; `andromeda-pulse-mcp` sha256 `6175fc36be6577b195470f136a124fe8690e4029745f3651ab46ce19043ad2a9`,
  47415880 bytes, built 2026-10-07 23:04:26 +02:00. Each equals its row in the new-side table above.
- **The dir.** The parent `pulse-legs` holds the four leaves; `rm-sixth-series` held 0 entries.
- **The GPU.** 32 C, 44 W, 0 % utilisation, 1163 MiB in use by two desktop clients; no `llama-cli`, `cargo` or
  `rustc` process on the host.

## Pre-leg env checks (plan step 11, entries 12-14, fired by hand 2026-10-08T06:27:55Z in a shell that sourced the plain leg env script)

- `ANDROMEDA_PULSE_MODEL_PATH` resolves to a file and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` to an executable:
  `model env present`, exit **0**.
- The model file's sha256: `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87` — **equal** to the
  pinned digest (exit 0).
- NVIDIA kernel-module major against the userspace major: `kernel=610 userspace=610`, exit **0**.
- The leg env script exports four handles, read back by name: `ANDROMEDA_PULSE_MODEL_PATH`,
  `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`, `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`, `ANDROMEDA_LLAMA3_TOKENIZER_PATH`.
  The script sourced is the plain one. The pass-through script was not sourced, and both model-binary handles name
  the runner `llama-cli` itself.

## The launch (plan step 11)

- **Pre-series census** (06:27:55Z, the round's census command by hand): the pattern
  `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched 0 processes; 0 listeners on `:4317` or `:4318`.
- **The round's selection for `d1`** was walked again with the gate tool's `--live-legs --entry 17 --dry-run`
  (nothing fired): `round: DRY-RUN · legs 1`, entries 15-17 selected, no pre-flight refusal. The tool's stamp in
  this run's trail is `gate v1.12 · bdf1c88a`; its sha8 differs from Part A's after a pipeline deploy of
  2026-10-08 (input I18 §3).
- **Launch** (the agent, a scratchpad script: `setsid nohup`, cwd = the data dir, outside this repo): the rebuilt
  `pulse-app` by path from `target/release` (sha256 `44f28608…428d`, re-read by the launch script itself), PID
  3816613, started 2026-10-08T06:28:35Z by the script's stamp (06:28:34Z by the process table). ONE env block,
  echoed back by name:
  - `ANDROMEDA_PULSE_DATA_DIR` — a directory whose leaf is `rm-sixth-series`, empty at the launch;
  - `ANDROMEDA_PULSE_MODEL_PATH` — a file named `gemma-4-E4B-it-Q4_K_M.gguf`;
  - `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` — an executable named `llama-cli`;
  - the leg env script's two other handles, as sourced: `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` (an executable
    named `llama-cli`) and `ANDROMEDA_LLAMA3_TOKENIZER_PATH`;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC` and `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` removed, both read back
    unset; `ANDROMEDA_PULSE_MCP_ENABLED` unset on the app's side; no other `ANDROMEDA_` or `CONDUCTOR_` name;
  - **no host rendering lever**: `WEBKIT_DISABLE_DMABUF_RENDERER` unset, `__NV_DISABLE_EXPLICIT_SYNC` not preset.
- **Liveness** (06:28:50Z, 14 s into the process's life by its own elapsed time): the process alive with five
  WebKit children (3816713, 3816722, 3816723, 3816731, 3816732); listeners on `127.0.0.1:4317` and
  `127.0.0.1:4318`; `:4317` accepted a connection at 8 s and again at 14 s. **No relaunch**: the app stayed up at
  its own posture, so the plan's one-relaunch arm was never used and Pulse's log carries a single boot. Its stderr
  held one message, a tray-library deprecation warning.
- **Booted posture, from Pulse's own log** (`agent-latest.jsonl.2026-10-08`, 267 lines, read 06:28:50Z, before
  `d1`):
  - `interpretation.model.load` `inference_mode` **`real`**; `load_status` `loaded`, `model_identity`
    **`gemma-4-E4B-it-Q4_K_M`**, tier `primary`;
  - `app.boot.workspace_key` `workspace_root_basename` **`rm-sixth-series`** (the leaf), `key_bytes` 48;
  - `app.boot.otlp.grpc.bind` and `app.boot.otlp.http.bind` at 06:28:35.576Z;
  - **0** `triage.baseline.bootstrap_window.override` lines;
  - the render-posture line Pulse logged: `app.boot.render.posture` `lever` `__NV_DISABLE_EXPLICIT_SYNC`, `posture`
    `applied` (06:28:35.316Z) — the app set its own lever;
  - 264 INFO lines and 3 WARN lines at that read, no ERROR line.
- Conductor's side for every drive: `ANDROMEDA_PULSE_DATA_DIR` the same dir, `ANDROMEDA_PULSE_MCP_ENABLED=true`, the
  L4 handle absent, the sidecar on `PATH` from `target/release` (sha256 `6175fc36…d2a9`).

## The drives (plan step 12)

Each drive is one firing of the round tool on that drive's `live` entry: the round steps since the previous leg (the
quiet window, a census, the non-priming probe `conductor preconditions --for real-model-interpretation`), then
`bash scripts/agent-run.sh run --live real-model` and the copy of `runs/live-suite/rm-capture.txt`. Each firing's
entry lines and summary line are kept verbatim in `round-{HHMMSSZ}.txt` beside this ledger, named by the firing's
start. Grades are the harvest's, computed by the unchanged rule over the pinned files (plan step 13), never judged
here.

The real-model canary emits three storms 90 s apart, the third within milliseconds of the scenario's emission
instant. Each storm's outcome is read from Pulse's own log in each row, paired by Pulse's serial inference order.
How many `canary:` lines a capture prints (two or three) and what the third one reads is recorded per drive and
explained under the table.

| Label | Slot · timing · run | Route | Grade (P-033) | P-031 / P-034 / P-044 | `canary:` tokens (with `skip_reason`) | The scenario's digest, from Pulse's own log | Key rendering | Census |
|---|---|---|---|---|---|---|---|---|
| **d1** — `rm-capture-d1.txt` | the founder's go; the agent's `pulse-app` (PID 3816613); census and probe green 06:28:59Z (`[PRECONDITION] every live-Pulse precondition is satisfied`, exit 0); fired 06:29:00Z, ended 06:35:02Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-08T06-29-01-231`; `[MANUAL] real-model-interpretation`; 275 self-obs lines frozen; envelope 11 keys, `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 2 attributed on the first poll, `degraded_mode=false`, pickup 5813 ms | **`Identified`** | `Pass` / `Pass` / `Blocked` (no prior same-scope incident to retrieve) | `surfaced` (06:29:46Z, created) · `surfaced` (06:31:16Z, deduped) · `pipeline-fault` (06:32:46.351Z, `parse=none`) — `skip_reason=none` on all three; 0 other cue-bearing digests. **Pulse's own log reads the third storm parsed `ok` and deduped: not a pipeline fault, no re-fire** (below) | emission instant 06:32:46.355Z; storm Suggested 06:32:48.856Z, Autonomous 06:32:51.357Z; tier-1 `retry_storm` digest 06:32:51.361Z, picked up 06:32:52.168Z behind the canary's third storm, parse `ok` 06:32:58.062Z, **created** 06:32:58.120Z (severity `warn`, tier `suggested`) — surfaced. The canary's third storm (Autonomous 06:32:46.347Z, digest 06:32:46.351Z, prompt assembled 06:32:46.356Z) parsed `ok` 06:32:52.134Z and deduped 06:32:52.168Z. The creating digest's corpus rows as the capture prints them: 1. `prompt_version` `v2.6` on all 9 prompt assemblies; parse `ok` 9 of 9; 9 inference requests, all `success`; 147 append lines, 0 rejections. Skip lines in the window: 5, all on cue-less tier-3 digests — 4 `no_cue`, 1 `decision_dismiss` (06:29:40Z, before the first storm). WARN lines: 1 `digest.lww.drop` in the bracket, 3 `mcp.tools.call.error` just after it (the capture's own read-back); no ERROR line | `verbatim`; 0 `## Previously Seen` entries | before: the agent's `pulse-app` and its five WebKit children only, 2 listeners; the round tool reported no survivor and no history move |
| **d2** — `rm-capture-d2.txt` | the same go and launch; quiet window: 180 s slept from 06:36:05Z, d1's last created incident 06:32:58.120Z, d2's first canary storm 413 s after it; census and probe green 06:39:05Z (exit 0); fired 06:39:06Z, ended 06:45:08Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-08T06-39-06-798`; `[MANUAL] real-model-interpretation`; 271 self-obs lines frozen; envelope 11 keys, `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 5 attributed on the first poll, `degraded_mode=false`, pickup 12290 ms | **`Identified`** | `Pass` / `Pass` / `Pass` | `surfaced` (06:39:51Z, created) · `surfaced` (06:41:21Z, deduped) · `pipeline-fault` (06:42:51.851Z, `parse=none`) — `skip_reason=none` on all three; 0 other cue-bearing digests before the emission instant. **Pulse's own log reads the third storm parsed `ok` and deduped: not a pipeline fault, no re-fire** (below) | emission instant 06:42:51.853Z; storm Suggested 06:42:54.353Z, Autonomous 06:42:56.854Z; tier-1 `retry_storm` digest 06:42:56.859Z, picked up 06:43:04.143Z behind two queued digests, parse `ok` 06:43:10.174Z, **created** 06:43:10.232Z (severity `warn`, tier `suggested`) — surfaced. Ahead of it in Pulse's serial queue (the pairing rests on that order; Pulse's lines carry no digest identity): the canary's third storm (Autonomous 06:42:51.847Z, digest 06:42:51.851Z, prompt assembled 06:42:51.855Z) parsed `ok` 06:42:58.178Z and deduped 06:42:58.211Z, and a tier-2 `error_rate_spike` cue-bearing digest (06:42:52.583Z) created its own incident 06:43:04.143Z (incident 4), which carries no scenario fingerprint and is not the attributed one. The creating digest's corpus rows as the capture prints them: 3. `prompt_version` `v2.6` on all 12 prompt assemblies; parse `ok` 12 of 12; 12 inference requests, all `success`; 147 append lines, 0 rejections. Skip lines in the window: 6, all on cue-less tier-3 digests — 5 `no_cue`, 1 `decision_dismiss` (06:39:39Z, before the first storm). WARN lines: 12 `digest.lww.drop` in the bracket, 3 `mcp.tools.call.error` just after it; no ERROR line | `verbatim`; 1 `## Previously Seen` entry (d1's incident), its suffix printing `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor and no history move |
| **d3** — `rm-capture-d3.txt` | the same go and launch; quiet window: 180 s slept from 06:45:34Z, d2's last created incident 06:43:10.232Z, d3's first canary storm 370 s after it; census and probe green 06:48:35Z (exit 0); fired 06:48:35Z, ended 06:54:37Z (362 s, exit 0, both atoms, the capture fresh); run `2026-10-08T06-48-35-676`; `[MANUAL] real-model-interpretation`; 271 self-obs lines frozen; envelope 11 keys, `state` `ManualCheck`, `verdict` null | `ReadBack` — incident 7 attributed on the first poll, `degraded_mode=false`, pickup 6048 ms | **`Identified`** | `Pass` / `Pass` / `Pass` | `surfaced` (06:49:20Z, created) · `surfaced` (06:50:50Z, deduped) — `skip_reason=none` both, two lines; the capture also counts 4 other cue-bearing digests before the emission instant, all `error_rate_spike` (tier 2), each of which Pulse logged `deduped` | emission instant 06:52:20.721Z; storm Suggested 06:52:23.222Z, Autonomous 06:52:25.721Z; tier-1 `retry_storm` digest 06:52:25.725Z, picked up 06:52:26.769Z behind the canary's third storm, parse `ok` 06:52:32.603Z, **created** 06:52:32.661Z (severity `warn`, tier `suggested`) — surfaced. The canary's third storm (Autonomous 06:52:20.725Z, digest 06:52:20.732Z) parsed `ok` 06:52:26.736Z and deduped 06:52:26.769Z. The creating digest's corpus rows as the capture prints them: 6. `prompt_version` `v2.6` on all 15 prompt assemblies; parse `ok` 15 of 15; 15 inference requests, all `success`; 147 append lines, 0 rejections. Skip lines in the window: 5, all `no_cue` on cue-less tier-3 digests. WARN lines: 16 `digest.lww.drop` in the bracket, 3 `mcp.tools.call.error` just after it; no ERROR line | `verbatim`; 2 `## Previously Seen` entries, both suffixes printing `<redacted>` | before: the agent's `pulse-app` tree only, 2 listeners; the round tool reported no survivor and no history move; then the teardown below |

Three drives fired in one sitting, 06:28:59Z to 06:54:37Z. **Nothing was re-fired and there is no fourth drive.**
Across the three leg windows Pulse logged 36 prompt assemblies, all `v2.6`, and 36 parses, all `ok`; no
`interpretation.inference.error` line, no `interpretation.inference.skipped` line and no refused append. The
`model_identity` Pulse logged is the boot line's `gemma-4-E4B-it-Q4_K_M`. Its WARN lines in those windows were queue
replacements (`digest.lww.drop`: 1, 12, 16); three `mcp.tools.call.error` lines follow each bracket; the whole log
(25988 lines, 06:28:35Z to 06:55:02Z) holds no ERROR line.

### The third `canary:` line of d1 and d2 — a printed token Pulse's own log does not bear out

The section's re-fire clause reads the canary **from Pulse's own log**: a `pipeline-fault` is no parse `ok` for its
cue-bearing digest, or an inference error or skip. Applied to each drive:

| Drive | Third storm's digest tick | Emission instant | Its prompt assembly | Its parse | Its outcome | Inference error or skip | Reading |
|---|---|---|---|---|---|---|---|
| d1 | 06:32:46.351Z | 06:32:46.355Z (4 ms later) | 06:32:46.356Z | `ok` 06:32:52.134Z | deduped 06:32:52.168Z | none | surfaced — not a pipeline fault |
| d2 | 06:42:51.851Z | 06:42:51.853Z (2 ms later) | 06:42:51.855Z | `ok` 06:42:58.178Z | deduped 06:42:58.211Z | none | surfaced — not a pipeline fault |
| d3 | 06:52:20.732Z | 06:52:20.721Z (11 ms earlier) | (the first after its tick) | `ok` 06:52:26.736Z | deduped 06:52:26.769Z | none | surfaced — not a pipeline fault |

So no drive's canary reads `pipeline-fault` from Pulse's own log, and the clause's "No other outcome re-fires"
applies: all three drives stand as fired.

Why the captures of d1 and d2 print the token anyway, read in the capture's code (`real_model_live.rs`, the
canary block; `real_model_common/mod.rs`, `canary_attempts`): the capture pairs canary digests over the Pulse
lines stamped strictly BEFORE the scenario's emission instant. In d1 and d2 the third storm's digest tick fell 4 ms
and 2 ms before that instant and its prompt assembly 1 ms and 2 ms after it, so the pairing saw a retry-storm tick
with no prompt and printed `pipeline-fault` with `parse=none`. In d3, and in all nine drives of the two earlier
series and the capture run on this host (each committed capture prints two `canary:` lines and no `pipeline-fault`
token), the tick fell after the instant and the third storm got no `canary:` line at all. The order of those
two stamps is a race of a few milliseconds between Conductor's last canary storm and its emission stamp; this
series did not change it. The captures keep what they printed (the contract's own 2026-09-29 note on a printed
token that Pulse's log did not bear out states the same handling), and the harvest's `measured` table records the
tokens as printed. The capture's pairing window is harness code outside this chunk's scope and was not touched
between drives; the finding is reported for the wrap.

## Teardown (after d3)

The agent stopped what it started. `pulse-app` PID 3816613 (started 06:28:34Z by the process table, 06:28:35Z by the
launch script's stamp), re-read by PID, start time and name before the signal: `kill -TERM` at 06:55:02Z, exit 0;
the process was gone within 2 s. Pulse's log ends 06:55:02.679Z on one `app.exit` WARN line. Post-series census
(plan entry 26, by hand, 06:55:10Z): the pattern `pulse-app|andromeda-pulse|llama-cli|conductor|WebKit` matched
**0** processes and **0** listeners remained on `:4317` or `:4318` (exit 1, last line `0` — the entry's atoms
hold). That equals the pre-series baseline.

The app's stderr over the whole sitting held three messages: the tray-library deprecation warning at boot, and two
`Gtk-CRITICAL` assertion messages (`gtk_widget_get_scale_factor`) at 06:48:11Z and 06:51:55Z. The app stayed up
through both; the second falls inside d3's bracket, 26 s before its emission instant.

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` 3816613 | the agent, on the founder's go | terminated (SIGTERM, gone within 2 s) |
| its five WebKit children (3816713, 3816722, 3816723, 3816731, 3816732) | `pulse-app` | terminated with it; absent from the post-census |
| `conductor` and `andromeda-pulse-mcp`, one pair per leg and per probe | the legs | exited with each leg; absent from every pre-leg census and from the post-census; the round tool reported no survivor on any firing |
| `llama-cli`, one per inference | `pulse-app` | each exited with its inference; absent from the post-census |
| the round tool's three firings and their `sleep 180` steps | the agent | each exited with its round (`round: COMPLETE`, tool exit 0) |

## Grading (plan step 13) and the series verdict (the pre-registered rule, applied mechanically)

The grades in the table above are the harvest's (`each_2026_10_07_sixth_drive_grades_as_the_ledger_records`),
computed by the unchanged rule over the pinned files. They were read red before green: the `measured` table was
first written wrong on purpose in every field of every drive, and each value was then taken from the grading arm's
own failure output, one field per pass — route, grade, the further-grade triple and the canary counts of `d1`, then
of `d2`, then of `d3` (twelve red passes of the arm; the thirteenth ran green). The graded set was read the same
way, from a verdict test first asserting an empty set. The harvest then ran green: 133 tests (125 at the chunk
base, plus this series' eight: the seven that mirror the 2026-10-07 series' module and the ref test). Every
envelope: 11 keys, `seed` 4317033, `state` `ManualCheck`, `verdict` null. Every capture reads `pulse-log
inference_mode: real` and `pulse-log workspace basename carries conductor: false`.

The canary counts the harvest pins are the tokens as the captures PRINT them: `d1` and `d2` read 2 `surfaced` and
1 `pipeline-fault`, `d3` reads 2 `surfaced`. The printed `pipeline-fault` tokens are the window-edge reading
explained above; they are inputs to no grade and to no re-fire.

Capture pins (sha256 of each committed file; all three are LF):

| Capture | sha256 |
|---|---|
| `rm-capture-d1.txt` | `6b9b0bec36b0250eb993fc3196f04f5328626a055b232ef6b675bd84c42554c0` |
| `rm-capture-d2.txt` | `040c45301f2c0a31df9486fd5bcb160ad4ea17a43dda033ffefe7b7dd58a1620` |
| `rm-capture-d3.txt` | `326916766fd639311629f1cdde4fe8422773190f5c789d2260357d56c9ab947c` |

`COMMITTED_CAPTURES` moved 23 → 26: exactly the three `rm-capture*.txt` files this chunk commits.

Three drives fired, none re-fired, no fourth. **Graded drives: 3** — d1 `Identified`, d2 `Identified`, d3
`Identified`. The real-model witnesses hold on each graded drive (`inference_mode` `real`, no `det-` evidence
ref) and each launch-cwd witness reads clear. Under §The drive series (a), carried unchanged into §The 2026-10-07
sixth series — at least one drive graded and every graded drive `Identified` — **`v3-09` is MET**
(`v3_09_is_met_by_the_2026_10_07_sixth_series`). Because the condition holds, the separate ref test is written:
`v3_09_ref_identified_with_the_real_model_witnesses_2026_10_07_sixth`, which reads only the committed captures.

The stop rule's consequence, in the section's own words: "If it meets the pass condition below, `v3-09` is
verified on three drives." and "Either way the next route entry is the version close." The clause's other arm (no
seventh series, `v3-09` leaving 0.3.0) does not apply. Marking the capability verified and moving the route are the
wrap's acts; this implement recorded the ref and `implemented`.

Key rendering: `verbatim` on all three graded drives, so no drive is recorded as contaminated.

What the three drives measured, and nothing wider, on the shipped model at prompt `v2.6`, against the binary built
from `9bfefb8`'s product tree:
- per Pulse's own log every canary storm surfaced (9 of 9: three per drive, each parsed `ok` and created or
  deduped); the captures printed 8 `canary:` tokens for them, 6 `surfaced` and 2 `pipeline-fault`;
- the scenario's own storm digest surfaced in every drive (3 of 3), each attributed on the first poll;
- all three rank-1 statements name `conductor` as a whole word and carry a retry token, and all three carry the
  cue line's `scope_id` value in the cue line's own form;
- no ranked hypothesis of any drive (8 in all: 3, 3 and 2), and no justification line under one, names the canary
  identity. So the case the rule's known weakness covers — a rank 1 that names `conductor` as a whole word and
  also names the canary — did not occur, and no grade here rests on it;
- every no-incident outcome Pulse logged in the three leg windows was on a cue-less tier-3 digest (`no_cue` 14,
  `decision_dismiss` 2).

What they do not measure:
- **A met series is three drives and is not read as proof beyond them** (the section's stated limit).
- **The series holds no in-drive witness of the remedy.** No prompt was recorded, and the capture's corpus-row
  line prints the candidate count before the selection. That the narrowed selection is in the launched binary
  rests on the build provenance and the moved digest alone (the binaries table above).
- **Three `Identified` drives do not separate the remedy from the run-to-run spread.** The 2026-10-07 capture
  run, on the `f70be92` build without the remedy, also read d1, d2 and d3 `Identified` as observations. Across the
  four live runs on this host the third position now reads 2 misses of 4. That is a rate, not a cause.

### The covariate line, as printed — not a remedy witness

| Drive | Creating digest's corpus rows (candidates, before the selection) | `## Previously Seen` entries | Envelope fingerprints | Grade |
|---|---|---|---|---|
| d1 | 1 | 0 | 2 | `Identified` |
| d2 | 3 | 1 | 0 | `Identified` |
| d3 | 6 | 2 | 0 | `Identified` |

The row counts repeat the two earlier series' and the capture run's (1, 3, 6): the line counts candidates, so the
remedy does not move it. The envelope fingerprint count is recorded as each capture prints it and is graded by
nothing. Beside it, as printed: in d1 both listed incidents read `seen_active=true`; in d2 and d3 the attributed
incident read `seen_active=false`. Why the count differs between drives was not measured.

## Status smoke (plan entry 43), fired by hand after d3

`bash scripts/agent-run.sh status 2026-10-08T06-48-35-676` (d3's run_id, minted this session; its journal was
written at d3's end), fired 06:59:44Z: exit 0; atom `contains "scenario": "real-model-interpretation"` held
(1 hit); the envelope read back that `run_id`, `seed` 4317033, `state` `ManualCheck`, `verdict` null.

## Evidence hygiene (plan step 14, entries 41-42, through the gate tool)

- The host-path probe over every file of this evidence dir, with the home-rooted and temp-rooted shapes in its
  pattern: **0** at exit 1 (the entry's atoms hold).
- The workspace-key probe over the three captures: **0** occurrences of the leaf at exit 1 (the entry's atoms
  hold). The key reaches a capture only as its placeholder; each `## Previously Seen` suffix prints `<redacted>`.

Both were read before this ledger's last sections were written; the block is fired once more after the ledger's
last edit, and that pass's readings are in the implement report and the run dir's trail.

## The standing gates (plan step 15, entries 27-42, through the gate tool, 07:00Z)

16 entries fired, 16 green, 0 red, in one pass: the real-model harvest 133 of 133; `cargo test -p conductor-run`
390 passed across 27 result lines (the harvest's 133 among them); `capture_paths_guard` 8 of 8; the bundled default
exit 0 (workspace nextest 1233 of 1233, the doctests, the workspace lint and both feature-gated lint lines);
`cargo fmt --all --check` clean; the package count 562; the rule diff empty; the contract's deleted-line count 0
and its handle census 6; the frozen-path and frozen-chunk diffs empty; the advisory-db porcelain empty, then
`cargo audit` exit 0 and `cargo deny check advisories bans licenses sources` exit 0; both evidence probes 0.

**The first `cargo audit` reading was discarded and the entry re-read.** The porcelain entry read 0 lines, but the
audit's own fetch then moved the local advisory-db copy and left one untracked pre-id-assignment file (the `wasapi`
advisory under its placeholder id, equal to its tracked renamed twin apart from the id), so that scan ran over an
unclean copy (security.md, the 2026-10-07 extension). The one named file was removed, the copy's porcelain read 0
lines with `HEAD` equal to `FETCH_HEAD`, and entries 38-40 were fired again through the gate tool: 3 green. The
reading that stands: `cargo audit` exit 0, 1294 advisories loaded, 562 crate dependencies, 7 allowed warnings
(6 `unmaintained`, 1 `unsound`); the porcelain 0 lines after it as before it; `cargo deny` `advisories ok, bans ok,
licenses ok, sources ok`.

The block was fired once more, whole, after this ledger's last edit of that pass (07:03:57Z): 16 of 16 green with
the same counts, the advisory-db porcelain 0 lines before and after. The scope read (`gate.py scope`) read
`scope: clean — changed 6 · listed 6`.

## The operator pass (plan entries 44-46), on the operator's word

The operator, 2026-10-08 at about 09:06 local, in this implement session after its report (the chunk's inputs
record it as I20): the operator pass — hygiene, the pre-CI commit and guarded push, the CI read — performed by the
agent on that explicit word, then stop before the wrap, which the operator sends. In the same word the operator
states he verified the verdict himself: the section digest re-derived, the harvest 133 of 133 by his own run, each
rank-1 statement's two facets with no hypothesis naming the canary, and the third canary storm of d1 and d2 parsed
`ok` and deduped in Pulse's log lines, so no re-fire was owed. The commit and the push are the operator's acts,
made by the agent on his word.

- Entry 44, `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene`, fired bare at 07:07:00Z
  before the commit: exit 0; atom `contains hygiene: clean` held (1 hit) — `hygiene: clean`, 63 files read (runs 45,
  evidence 7, inputs 11), 17 trails and 9 verbatim input copies not read by P1, 0 host paths kept.
- Entry 45: on the operator's explicit word, the agent made the operator pre-CI commit `e3fa847`
  (`e3fa847239e075a8d9627fa9496b51c6840b62ae`, 77 files) and fired the entry as written at 07:07:14Z: the
  clean-tree guard held, exit 0, and the push printed `PUSHED_SHA=e3fa847239e075a8d9627fa9496b51c6840b62ae`
  (`902d12c..e3fa847` on `build/conductor-0.3.0`); atom `contains PUSHED_SHA=` held.
- Entry 46, `ci.py conclusion --sha HEAD --wait 1800`, fired after the push at 07:07:21Z, ended 07:19:14Z: exit 0;
  atom `contains verdict: green` **held** on the first reading — `verdict: green · checks 3/3 · wall 703 s`, run
  CI#37741509455 `completed/success` on `e3fa847239e075a8d9627fa9496b51c6840b62ae`, polled 24 times over 713 s. No
  re-run was fired. The run id the CI acceptance names is CI#37741509455.

This section was written after the push, so it rides the wrap's commit, not the pre-CI one.
