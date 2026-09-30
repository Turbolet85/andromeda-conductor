# Live slot — driven leg, SR legs, census

The operator granted the `:4317` slot on 2026-09-30 (the overseer measured 4317/4318 free and no pulse-app, inject_demo
or msedgedriver running; pulse-builder stopped for the slot). Slot held 10:40:21Z → 10:51Z, then reported released.

## SUT provenance
- `andromeda-pulse` `target/release/pulse-app.exe` sha256
  `9e51d1d92e80fdc0b998fe5e1c65fbbd9c5eef4dd9e6b7c4883c5ccad1bf9ab4` — the relayed prefix `9e51d1d9` matches.
  Provenance per pulse-builder (relayed by the overseer): Pulse `71f3369` PLUS its uncommitted P-027 discovery fix.
  Measured here: Pulse HEAD `71f3369`, with uncommitted edits under `crates/triage/` (`baseline/mod.rs`,
  `lifecycle/mod.rs`, `lifecycle/registry.rs`) besides its own bookkeeping files; the exe's mtime is 2026-09-30
  12:26:44 host-local. This is NOT a committed Pulse state.
- `andromeda-pulse-mcp.exe` (the sidecar on the legs' PATH) sha256
  `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634`.

## Launch (agent-launched on the operator's word)
- Fresh data dir `pulse-legs/drivenkeys` under the OS temp dir (letters-only leaf), `ANDROMEDA_PULSE_MCP_ENABLED=true`,
  `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`, `Start-Process` without `-Wait`, cwd = the data dir (outside the repo).
- pulse-app pid 20656, created 2026-09-30T10:40:21.032Z.
- Posture read from Pulse's own log: `inference_mode: "deterministic"` ("L4 deterministic mode active"),
  `app.boot.workspace_key` `workspace_root_basename: "drivenkeys"`, `app.boot.otlp.grpc.bind`
  `bind_address: "127.0.0.1:4317"`; netstat: 4317 and 4318 LISTENING by pid 20656.

## Order and verdicts
| step | command (firing form) | window (UTC) | exit | reading |
|---|---|---|---|---|
| probe (entry 9) | `conductor preconditions` with the Pulse release dir on PATH | 10:41:06 | 0 | `[PRECONDITION] every live-Pulse precondition is satisfied` — no canary |
| driven (entry 10, by hand) | `PATH=…pulse release…:$PATH npm run a11y:driven` | 10:41:15 → 10:45:22 | 0 | `1 passing (4m 2.1s)` · `Spec Files:<TAB> 1 passed, 1 total` |
| sr-empty (entry 8, 3rd run) | `npm run a11y:sr-empty` | 10:45:45 → 10:46:39 | 0 | `1 passing` — DOM half green; NVDA rows below |
| sr live (entry 11, by hand) | `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios PATH=…:$PATH npm run a11y:sr` | 10:47:15 → 10:49:56 | 0 | `1 passing (2m 31s)` · `Spec Files:<TAB> 1 passed, 1 total` |

No `boot` preceded any leg. The driven run's canary incident formed 10:42:48Z/10:42:52Z (`created: true`,
`deduped: false`); its emission ended with the abort at 10:45:21Z; the SR live leg started 10:47:15Z, past a 150 s
quiet window after both.

## The driven run's decisions, from the backend's own log (`runs/driven/logs/conductor-tauri.jsonl`)
- `operator-checklist hold resolved by tauri-dialog: No-Go (1 checklist item(s))` — hold 1 (halo-breathing), Escape.
- `operator-checklist hold resolved by tauri-dialog: Go (1 checklist item(s))` — hold 2 (halo-hue), Ctrl+Enter.
- `run aborted by the operator` — the Ctrl+. stop, confirmed after the last scenario.
The spec asserted exactly one `: No-Go (` and one `: Go (` among the lines added after its own baseline count.

## SR record (`evidence/nvda-pass.json`, copied from the chunk-bundle runs — never the control's)
- `subjects.empty.recorded_at` 2026-09-30T10:46:39.242Z (leg start 10:45:45Z); `subjects.live.recorded_at`
  2026-09-30T10:49:56.719Z (leg start 10:47:15Z). Both this session. `subjects.error` is 2026-09-07 (not run).
- The record's `build_commit` reads HEAD `7ee2fea`; the bundle under test carried this chunk's uncommitted delta
  (the `--e2e` run rebuilt it; the hint marker `run-controls__hint` is in the served JS).
- DOM half: `expectActiveCoverageRow()` held at S0-09 and E0-05 in every run — focus lands on the coverage row with
  `tabindex="0"` + `aria-current="true"` inside the table named `Coverage rows`.
- NVDA half: **S0-09 `not-announced`, E0-05 `not-announced`** (E0-09 `not-run-here`, browse class). Every other
  FOCUS row of both subjects is `not-announced` too (E0-02/E0-03/E0-04/E0-06, S0-01/S0-02/S0-04…S0-07, S1-03/S1-04,
  S2-03/S2-04, S3-02); the live-region rows are heard (S1-01, S2-01, S2-08, S3-01, S3-05 `announced-as-expected`).
  `attach_observed`: live `true`, empty `false`.

## Findings (recorded, never passes)
1. **Focus theft (first two sr-empty runs, 10:09Z and 10:12Z, before the slot).** NVDA's foreground moved off
   Conductor to the **pulse-builder session's Windows Terminal** ("Windows PowerShell terminal ◐ pulse-builder") in
   both, and in the second first to a **WebView2 host console window** (NVDA read its title — the runtime's exe path,
   scrubbed in any committed record — then "terminal blank"). Every focus row of those runs is silent for that reason.
2. **Focus-event silence on a quiet desktop (every run, both subjects).** With pulse-builder stopped, NVDA still binds
   Conductor's window (it reads the document at load), speaks "Landscape" three times (a display event) around
   activation, and then hears NO focus event from the webview until the window closes, while still hearing the
   live regions. **Two-sided control, 10:52Z:** the BASE bundle (`7ee2fea` `ui/src` built into `ui/dist`, the release
   binary relinked; zero Rust delta) under the same `sr-empty` leg is silent from `@foreground` onward in exactly the
   same way (E0-02/E0-03/E0-04 `not-announced`; it then fails at E0-05 by design, the base's stop being the scroll
   div). So the silence predates this chunk — a host condition on WebView2 runtime 154.0.4258.37 / NVDA 2026.2 —
   and the S0-09/E0-05 NVDA regrade is NOT achieved this session. The chunk bundle was rebuilt and relinked after the
   control. Owner: the wrap pins it (plan's SR condition) — no human listening is needed, the agent arm hears nothing.
3. **A WebView2 console window announces itself.** In the control run NVDA also read
   "DevTools listening on ws://127.0.0.1:…" from that console window. Pre-existing (present on the base bundle).

## Census (Win32_Process with parentage; families msedgedriver, conductor-tauri, node, nvda, msedgewebview2,
## tauri-driver, andromeda-pulse-mcp, pulse-app)
| moment | reading |
|---|---|
| before any leg (≈10:07Z) and before the launch (≈10:39Z) | six `msedgewebview2` owned by `SearchHost.exe`, started 2026-09-26 — nothing else |
| after the legs, pulse-app stopped (10:50:36Z) | the six + pulse-app's WebView2 tree (root 25780, parent 20656 gone, created 10:40:21Z, 4 children) |
| final (≈10:51Z) | the six only — identical to the baseline; no LISTENING socket on 4317/4318/4444/4445 (TIME_WAIT remnants only) |

| process | started by | final state |
|---|---|---|
| pulse-app pid 20656 (created 10:40:21Z) | this session, on the operator's word | terminated — `CloseMainWindow` did not exit in 15 s, then `Stop-Process -Force` |
| pulse-app's msedgewebview2 tree (root 25780 + 4) | pulse-app | terminated — exited on its own within ~5 s of its parent |
| tauri-driver / node / msedgedriver / conductor-tauri / msedgewebview2 (each leg) | each leg | terminated by wdio `onComplete` |
| NVDA (each SR run) | each SR leg | terminated by the harness's `nvda -q` |
| andromeda-pulse-mcp sidecars | the legs' app | terminated with the app |
