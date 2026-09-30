# Confound control C1 and the regrade on the OS input path

Chunk `2026-09-30-the-sr-pass-regrades-on-the-os-input-path`, chunk base `ff4f571`. Every reading below comes from
NVDA's own `-l 12` log, its entries paired by HEADER with the session's UTC stamps (NVDA's clock is local, UTC+2,
converted). Host paths are described in words. The raw logs, timelines and per-run snapshots stay under the
gitignored `runs/sr-control/`, and none is committed. The regrade record is `nvda-pass.json` beside this file.

## Verdict

In ONE session, with Conductor launched the way the `sr*` legs launch it (tauri-driver + msedgedriver, repo-root cwd,
the `sr-empty` subject's env), NVDA heard every OS-level `SendInput` Tab (5 of 5) and none of the WebDriver-injected
Tabs after the window's first burst (0 of 5 before the OS phase, 0 of 4 after it). The only variable between the
phases was the input path, so the standing confound resolves to the INPUT PATH: injected keys are the silent
variable, and the driver launch is not. On that validated path the leg's Tab, Shift+Tab and browse keys moved to
OS-level input (`send-keys.ps1`, ratified by the founder's word below), and all three subjects were regraded in one
record: the focus-row count fell from 24 to 2, and both of the 2 are content findings (a missing P-ID token), not
silence. The mechanism is recorded, not established.

**Branch:** H-R

**Founder word:** «Да делай» ("Yes, do it") — the founder, 2026-09-30, live, relayed verbatim by the overseer, in
answer to the `send-keys.ps1` question with the C1 reading in hand (injected 0/5 + 0/4, OS 5/5). The overseer's
relay states its scope: committing `send-keys.ps1` as proposed (closed key set, foreground guard, `-File` fixed argv;
rule (b)'s governed forms seven to eight).

## §Configuration

- WebView2 runtime 154.0.4258.37 (every session's `browserVersion`; the EdgeUpdate client key agrees) · msedgedriver
  154.0.4258.37 (the coherence entry read `driver 154.0.4258.37 runtime 154.0.4258.37` before `--e2e` and again
  before the regrade legs) · Edge browser 154.0.4258.37 (client key `{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}`).
- NVDA 2026.2 (`Starting NVDA version 2026.2`), the portable copy at the leg's registered argv, `-l 12`, over a copy
  of the committed leg `nvda.ini` (synth `silence`; no `[keyboard]` section, so `handleInjectedKeys` is its default).
- OS `CurrentBuild.UBR` 26200.9457 (the registry read 26200 and UBR 0x24f1; NVDA's log opens with
  `Windows 11 25H2 (10.0.26200.9457)`).
- Bundle: strict `--e2e` rebuilt the release binary at the chunk base (`Compiling conductor-tauri v0.2.0` …
  `Finished release`, written 14:44:04Z). The regrade record's `build_commit` is `ff4f571ef04b3755f6552dee129fe918fbfe6305`.
- Launch posture, C1 and every regrade leg: tauri-driver on `:4444` spawning msedgedriver on `:4445`, cwd = repo
  root, the app attached by the session (`browserName: wry`, `tauri:options.application` = the release binary).
- Input path per phase: C1-I1 and C1-I2 WebDriver-injected (W3C actions); C1-O OS-level `SendInput` from the session
  script. Regrade legs: every row's Tab, Shift+Tab and browse key (`h` · `d` · ArrowDown) OS-level through
  `send-keys.ps1`; the start-of-document reset cycle, picker text, Enter, Space, Escape and the picker's arrows
  injected. Each row records its own path in `input`.
- Live subject only: pulse-app from the Pulse tree's release build, sha256
  `e76be39b17bc8eed3d10666088b90443ea2f39e6de27db5dc2f84025a1c44a32`, written 14:23:24Z; Pulse HEAD `c6eb395`
  (its perf chunk's operator pre-CI commit, 14:28:09Z, no product change outside it, per the overseer). Sidecar
  `andromeda-pulse-mcp` sha256 `2179caab9f7247a52cba867d52e0c7de14472dbc642433ca3d27bf56c34cc634`, written 05:17:07Z,
  on the leg's `PATH`. Launched by this session on the overseer's grant, on the fresh letters-only leaf
  `%TEMP%/pulse-legs/srliveretry`, cwd outside the repo, `ANDROMEDA_PULSE_MCP_ENABLED` and
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` true. Posture read from Pulse's own log: `inference_mode` `deterministic`,
  `workspace_root_basename` `srliveretry`, OTLP gRPC bound on `127.0.0.1:4317`. No `boot` before the leg; the
  non-priming probe printed `sidecar on PATH` and `True`. Live run id `2026-09-30T15-13-09-561`.
- Desktop: the parallel pulse-builder session window-free (the overseer measured no nvda, pulse-app, inject_demo or
  msedgedriver before each slot).

## §Arms — C1 (14:46:19–14:47:51 UTC)

Theft check: the pulse-builder terminal's entries fall only at NVDA's start (14:46:21, before activation); no other
window's entry falls inside any stamped key window, and every O key found Conductor in the foreground (no
`foreground-lost` stamp). Witness: NVDA's log holds exactly **5** `Input: kb(desktop):tab` entries — the O keys;
the injected keys add none.

| arm | input path | keys, graded by NVDA's log | verdict |
|---|---|---|---|
| C1-I1 | WebDriver-injected | start-of-document burst 6 of 6 heard (Minimize, Close, Start, the coverage row, the report region, the BODY stop); then 5 Tabs 4 s apart landing Minimize, Close, Start, TR, DIV — 0 heard | silent |
| C1-O | OS-level SendInput, foreground-gated | 5 Tabs landing Minimize, Close, Start, TR, DIV — 5 heard, each 30-60 ms after its Input entry | heard |
| C1-I2 | WebDriver-injected | 5 Tabs: the first landed on BODY (excluded), then Minimize, Close, Start, TR — 0 of 4 heard | silent |

Step 6's rule: I1 `silent` AND O `heard` ⇒ **H** (injection is the variable); the founder's quoted word was relayed
before the branch point was passed ⇒ **H-R**. I2 is recorded and is not decision-bearing.

## §What each reading licenses

Bound to the configuration above; nothing is stated beyond it.
- **C1-I1 silent and C1-O heard in one session, one launch** → under tauri-driver + msedgedriver, OS-path Tabs are
  heard where injected Tabs are silent. The driver launch alone does not silence NVDA; the injected path does.
- **C1-I2 silent after O was heard** → hearing OS keys does not restore the injected path within the session.
- **The regrade** → the agent arm's focus rows are heard on the OS path in every subject (focus rows: 17
  `announced-as-expected` on `os`, 4 on `webdriver` where the reset burst or an injected key carried them, 1 `mixed`,
  2 `announced-differently` on `os`). Browse-mode commands reach NVDA's hook: every OS browse key appears in NVDA's
  `Input:` log, and 6 browse rows are heard and token-graded on the agent arm.
- **Mechanism — recorded, not established.** A reading consistent with every row: after the first burst, injected
  focus moves inside Conductor's driven WebView2 document stop producing the focus events NVDA consumes, while OS-path
  focus moves in the same session do not. No arm measured the events themselves.

## §Regrade — the runs, in order

Each regrade leg was fired once on a granted quiet-desktop slot. Two runs went red on harness defects in this chunk's
own key path; each was snapshotted, fixed in a listed file, and re-fired on a new slot the overseer granted as a
fix-loop re-fire (never a rerun-until-announced). No run was re-fired to chase an announcement.

| run (UTC) | subject | result | reading |
|---|---|---|---|
| 14:58:24 | empty #1 | red — `Spec Files: 0 passed, 1 failed` | harness defect: under OS Tab the start-of-document reset cycle never reached the BODY stop (10 Tabs, all 17 OS keys heard), so E0-02's first Tab landed on BODY and `expectFirstTabLanding` threw. Fix: the reset cycle alone returns to injected Tab; every row key stays OS. |
| 15:01:04 | empty #2 | green — `Spec Files:` TAB `1 passed, 1 total` | graded; 9 OS keys, 9 `Input:` entries |
| 15:02:21 | error | green — `Spec Files:` TAB `1 passed, 1 total` | graded; 3 OS keys, 3 `Input:` entries |
| 15:04:54 | live #1 | red — `focus did not land on "Stop"` at S1-04 | harness defect: after one OS Shift+Tab, NVDA read every later OS Tab as `shift+tab` (15:05:45.691, 15:05:47.128) and its browse-mode Shift+Tab moved focus backwards. The batched Shift release is the inferred cause (it can land while NVDA re-sends the key with injected input ignored). Fix: `send-keys.ps1` releases Shift in its own `SendInput` 300 ms after the key and exits 5 if Shift still reads down. pulse-app stopped; the next live run took a fresh data dir. |
| 15:10:16 | probe | clean on 9 of 9 | the fixed `send-keys.ps1` over the C1 launch: Tab ×3, Shift+Tab, Tab, Shift+Tab ×2, Tab ×2 — NVDA's `Input:` sequence identical to the keys sent, landings Minimize, Close, Start, Close, Start, Close, Minimize, Close, Start, all nine spoken |
| 15:12:24 | live #2 | green — `Spec Files:` TAB `1 passed, 1 total` (1 m 54 s) | graded; 20 OS keys and 20 `Input:` entries, identical in sequence |

The empty and error records predate the Shift fix and stand: the error subject sends no Shift+Tab, and NVDA logged
the empty subject's one post-Shift+Tab ArrowDown as a plain `downArrow`. Theft check over the three graded logs: no
terminal, console or foreign-window entry inside any subject's row span (first stamp to `@end`); the pulse-builder
entries fall before the first stamp or after `@end`. Security findings in the record: 0.

Outcomes in the record (51 rows): `announced-as-expected` 31 · `announced-differently` 8 · `not-announced` 2 ·
`not-run-here` 8 · `subject-absent` 2. The `announced-differently` rows are content findings, graded as measured:
S0-09 and E0-05 (the coverage row read as "row, current" with no P-ID), E0-09 (the same, missing P-019), E0-07 and
E0-08 (browse steps reading the report table's header cells, not `lamps-fixture` / `Blocked`), S0-15, S3-06 and S3-07
(browse steps reading a neighbouring line). S0-13 and S0-14 are `not-announced` on the agent arm: NVDA logged the `h`
and `d` keys and spoke nothing. The `not-run-here` browse rows are the ones whose windows held no OS key (rows with
no action), now noted as "no browse-mode command was driven". The record carries `operator_review: null` — the
operator's review of it is owed after the wrap's read.

## §Reds

| red | disposition | value | configuration or measured cause |
|---|---|---|---|
| focus-row-count | graded | 2 against a bar of 0 (forecast 24 → 0 from S-conductor's 5/5; measured 24 → 2): S0-09 and E0-05, both announced-differently, missing P-001 — the silence is gone and the bar is not met on content, a measured red routed to the successor, never re-run | WebView2 154.0.4258.37 × msedgedriver 154.0.4258.37 × NVDA 2026.2 × Windows 26200.9457 × OS input path (send-keys.ps1) under tauri-driver + msedgedriver |
| S0-09 / E0-05 / E0-09 | graded | S0-09 announced-differently · agent · os; E0-05 announced-differently · agent · os; E0-09 announced-differently · agent · os (missing P-019) — heard, graded by tokens, never not-run-here; the announced-as-expected bar is not met for S0-09 / E0-05 and routes forward with the missing-P-ID finding | the same configuration; the coverage row speaks its role and state ("row, current") and no cell content |
| one-configuration | graded | true: empty 15:01:34Z, error 15:02:42Z, live 15:14:29Z, all after the chunk base (14:11:24Z), in one record | the same configuration for all three; send-keys.ps1's Shift release was revised between error and live, and neither earlier subject's keys were affected (measured above) |

## §K

| arm | subject | status | verdict |
|---|---|---|---|
| K-edge | physical keyboard over Edge 154 | not run — founder ruling, cause already isolated by C1; retired, not pending | not-run |
| K-conductor | physical keyboard over Conductor | not run — founder ruling, cause already isolated by C1; retired, not pending | not-run |
| K-search | physical keyboard over the search panel | not run — founder ruling, cause already isolated by C1; retired, not pending | not-run |

Arm K is RETIRED. Founder ruling, 2026-09-30, live, relayed verbatim by the overseer at this chunk's wrap: «ну раз
уже разобрались не вижу особого смысла париться» ("well, since we've already sorted it out, I don't see much point in
bothering"). It was a desk task until then (clearing event: the founder at the desk for a slot), and it carries into
no successor. It was never simulated by `SendInput`. A physical keyboard therefore stays unmeasured, and that is the
founder's recorded choice, not an open obligation.

## §Census

`Win32_Process` with parentage over nvda, msedge, msedgedriver, conductor-tauri, tauri-driver, node, msedgewebview2,
SearchHost, pulse-app and andromeda-pulse-mcp, plus a listener probe on 4444 / 4445 / 4317 / 4318. The baseline at
every census was SearchHost (created 2026-09-26) and its six `msedgewebview2` children, the OS's own.

| census (UTC) | beyond the SearchHost baseline |
|---|---|
| pre-chunk 14:42:50 | none; no listener |
| post-`--e2e` 14:45:50 | none; no listener |
| post-C1 14:47:57 | none; no listener |
| after empty #1 15:00:36 | none; no listener |
| after empty #2 15:01:54 | none; no listener |
| after live #1, pulse-app stopped 15:08:09 | none; no listener |
| after the probe 15:11:23 | none; no listener |
| after live #2, pulse-app stopped 15:14:56 | none; no listener |

| process | started by | final state |
|---|---|---|
| NVDA (C1, empty ×2, error, live ×2, probe) | this run's legs | terminated (`-q`, polled to absence; the `sr*` harness quits its own) |
| tauri-driver (node), msedgedriver, conductor-tauri and its WebView2 children (C1, probe) | this run's session scripts | terminated (session deleted, the driver tree stopped by parentage) |
| tauri-driver, msedgedriver, conductor-tauri, node (the `sr*` legs) | this run's legs | terminated (the harness's `onComplete`) |
| pulse-app pid 31960 (created 15:03:43Z) and pid 20564 (created 15:12:09Z), each with its sidecar and WebView2 tree | this session, on the overseer's grant | terminated (`CloseMainWindow`, then `Stop-Process` after 15 s; matched by pid and creation time; 4317 / 4318 released) |
| SearchHost + its WebView2 tree | the OS | left running: not this session's |
| pulse-builder's terminal | the operator's parallel session | left running: not this session's |

## §Scripts

Two uncommitted session scripts under the gitignored `runs/sr-control/`. One elision: the loopback base URL is shown
with its scheme's colon bracketed (`http[:]//`), so the host-path hygiene anchor does not read the scheme as a drive
path; the script holds the plain scheme. The guards check each handle for existence, is-a-file and the wdio
metacharacter set, and never print a value.

`runs/sr-control/shift-probe.ps1` (the validation probe) is this script with its key block replaced: the injected
reset cycle to BODY, then nine calls of the committed `send-keys.ps1` (`& powershell.exe -NoProfile -ExecutionPolicy
Bypass -File <send-keys.ps1> -Key <Tab|ShiftTab>`, 3 s apart), each stamped with its exit code and landing.

### `runs/sr-control/driver-os-walk.ps1` (C1)

```powershell
# SR confound control C1: Conductor launched the way the sr* legs launch it (tauri-driver + msedgedriver,
# repo-root cwd, the sr-empty subject's env), then ONE session drives Tab over both input paths:
# I1 = WebDriver-injected (the leg's start-of-document cycle, then 5 Tabs), O = 5 OS-level SendInput Tabs,
# I2 = 5 injected Tabs. NVDA's -l 12 log grades each key. Session script: gitignored, text quoted in evidence.
param([switch]$DryRun)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Set-Location $root
$Mode = 'driver-os'

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class SrOs {
  [StructLayout(LayoutKind.Sequential)] public struct KEYBDINPUT { public ushort wVk; public ushort wScan; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Sequential)] public struct MOUSEINPUT { public int dx; public int dy; public uint mouseData; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Explicit)] public struct InputUnion { [FieldOffset(0)] public MOUSEINPUT mi; [FieldOffset(0)] public KEYBDINPUT ki; }
  [StructLayout(LayoutKind.Sequential)] public struct INPUT { public uint type; public InputUnion u; }
  [DllImport("user32.dll", SetLastError = true)] public static extern uint SendInput(uint n, INPUT[] inputs, int size);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  static INPUT Key(ushort vk, bool up) {
    INPUT i = new INPUT(); i.type = 1; i.u.ki.wVk = vk; i.u.ki.dwFlags = up ? 2u : 0u; return i;
  }
  public static uint Press(ushort[] vks) {
    INPUT[] seq = new INPUT[vks.Length * 2];
    for (int k = 0; k < vks.Length; k++) { seq[k] = Key(vks[k], false); seq[vks.Length * 2 - 1 - k] = Key(vks[k], true); }
    if (seq.Length == 0) return 0;
    return SendInput((uint)seq.Length, seq, Marshal.SizeOf(typeof(INPUT)));
  }
  public static uint ForegroundPid() { uint p; GetWindowThreadProcessId(GetForegroundWindow(), out p); return p; }
}
"@

$dir = Join-Path $root 'runs\sr-control'
$log = Join-Path $dir "nvda-speech.$Mode.log"
$actions = Join-Path $dir "actions.$Mode.jsonl"
if ($DryRun) { $actions = Join-Path $dir "actions.$Mode.dryrun.jsonl" }
$cfg = Join-Path $dir 'nvda-config'
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Stamp([hashtable]$fields) {
  $fp = [SrOs]::ForegroundPid()
  $fn = (Get-Process -Id $fp -ErrorAction SilentlyContinue).ProcessName
  if (-not $fields.ContainsKey('t')) { $fields['t'] = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ') }
  if (-not $fields.ContainsKey('foreground')) { $fields['foreground'] = "$fn" }
  $fields['mode'] = $Mode
  [System.IO.File]::AppendAllText($actions, ($fields | ConvertTo-Json -Compress) + "`n", $utf8)
}

function Descendants([int]$rootPid) {
  $all = Get-CimInstance Win32_Process
  $set = @($rootPid)
  do {
    $grew = $false
    foreach ($p in $all) {
      if ($set -contains [int]$p.ParentProcessId -and -not ($set -contains [int]$p.ProcessId)) { $set += [int]$p.ProcessId; $grew = $true }
    }
  } while ($grew)
  return $set
}

$base = 'http[:]//127.0.0.1:4444'
function Wd([string]$method, [string]$path, [string]$body, [int]$timeout = 60) {
  if ($body) { return Invoke-RestMethod -Method $method -Uri ($base + $path) -ContentType 'application/json' -Body $body -TimeoutSec $timeout }
  return Invoke-RestMethod -Method $method -Uri ($base + $path) -TimeoutSec $timeout
}

# The projection screen-reader.e2e.ts activeName() uses: aria-label, else an interactive element's text, else the tag.
$landingJs = "const el = document.activeElement; if (!el) return ''; const label = el.getAttribute('aria-label'); if (label) return label.trim(); const interactive = ['BUTTON','INPUT','A','SELECT','TEXTAREA'].includes(el.tagName); return interactive ? (el.textContent || '').trim().slice(0, 60) : el.tagName;"
$mountedJs = "return document.readyState === 'complete' && !!(document.querySelector('#root') && document.querySelector('#root').firstChild);"
function ExecBody([string]$js) { return (@{ script = $js; args = @() } | ConvertTo-Json -Compress) }
function Landing([string]$sid) {
  try { return [string](Wd 'Post' "/session/$sid/execute/sync" (ExecBody $landingJs)).value } catch { return '<unreadable>' }
}
# W3C key action for Tab; the escape is built from [char]92 because the Write tool decodes a JSON backslash-u escape.
$tabKey = [string][char]92 + 'uE004'
$tabBody = '{"actions":[{"type":"key","id":"kb","actions":[{"type":"keyDown","value":"' + $tabKey + '"},{"type":"keyUp","value":"' + $tabKey + '"}]}]}'

function InjectTab([string]$sid, [string]$phase, [int]$n) {
  $t = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
  $fg = (Get-Process -Id ([SrOs]::ForegroundPid()) -ErrorAction SilentlyContinue).ProcessName
  [void](Wd 'Post' "/session/$sid/actions" $tabBody)
  Start-Sleep -Milliseconds 150
  $landing = Landing $sid
  Stamp @{ t = $t; phase = $phase; n = $n; input = 'webdriver'; action = 'tab'; landing = $landing; foreground = "$fg" }
  return $landing
}

function OsTab([string]$phase, [int]$n, [int[]]$appTree) {
  $t = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
  $fp = [int][SrOs]::ForegroundPid()
  $fg = (Get-Process -Id $fp -ErrorAction SilentlyContinue).ProcessName
  if (-not ($appTree -contains $fp)) {
    Stamp @{ t = $t; phase = $phase; n = $n; input = 'os'; action = "foreground-lost $fg"; landing = ''; foreground = "$fg" }
    return
  }
  $sent = [SrOs]::Press([uint16[]]@(0x09))
  Start-Sleep -Milliseconds 150
  $landing = Landing $script:sid
  Stamp @{ t = $t; phase = $phase; n = $n; input = 'os'; action = ("tab sent=" + $sent); landing = $landing; foreground = "$fg" }
}

if ($DryRun) {
  Remove-Item -LiteralPath $actions -ErrorAction SilentlyContinue
  $tree = Descendants $PID
  $none = [SrOs]::Press([uint16[]]@())
  $fp = [SrOs]::ForegroundPid()
  Stamp @{ phase = 'dry'; n = 0; input = 'none'; action = ("press-empty=" + $none + " tree=" + $tree.Count + " fgpid-read=" + ($fp -gt 0)); landing = '' }
  $b = ExecBody $landingJs
  $parsed = $b | ConvertFrom-Json
  Write-Output ("dry-run: stamp ok, descendants " + $tree.Count + ", SendInput(empty) " + $none + ", exec body parses " + ($parsed.script -eq $landingJs) + ", tab body parses " + (($tabBody | ConvertFrom-Json).actions[0].actions[0].value.Length -eq 1))
  Get-Content -LiteralPath $actions
  Remove-Item -LiteralPath $actions
  exit 0
}

# Guards, the wdio way: unset, metacharacter or not-a-file -> name the handle and skip at exit 0.
$unsafe = '[;&|`$<>\r\n"'']'
$nvda = $env:CONDUCTOR_NVDA
$driver = $env:CONDUCTOR_MSEDGEDRIVER
foreach ($h in @(@('CONDUCTOR_NVDA', $nvda), @('CONDUCTOR_MSEDGEDRIVER', $driver))) {
  if (-not $h[1] -or $h[1] -match $unsafe -or -not (Test-Path -LiteralPath $h[1] -PathType Leaf)) {
    Write-Output ("skip: " + $h[0] + " is unset, unsafe, or not a file")
    exit 0
  }
}
$held = Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in 4444, 4445 }
if ($held) { Write-Output 'a listener already holds 4444/4445 - not started'; exit 2 }
# Start-Process joins the argv with spaces, so a whitespace-bearing root or driver path is refused rather than split.
if ($root -match '\s') { Write-Output 'repo root holds whitespace - not started'; exit 2 }
if ($driver -match '\s') { Write-Output 'CONDUCTOR_MSEDGEDRIVER holds whitespace - not started'; exit 2 }
$node = (Get-Command node -CommandType Application | Select-Object -First 1).Source
$cli = Join-Path $root 'crates\conductor-tauri\ui\node_modules\@crabnebula\tauri-driver\cli.js'
$app = Join-Path $root 'target\release\conductor-tauri.exe'
$activate = Join-Path $root 'crates\conductor-tauri\ui\test\a11y\screen-reader\activate-window.ps1'
foreach ($f in @($cli, $app, $activate)) { if (-not (Test-Path -LiteralPath $f -PathType Leaf)) { Write-Output ("missing: " + (Split-Path $f -Leaf)); exit 2 } }

Remove-Item -LiteralPath $log, $actions -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cfg | Out-Null
Copy-Item -LiteralPath (Join-Path $root 'crates\conductor-tauri\ui\test\a11y\screen-reader\nvda-config\nvda.ini') -Destination (Join-Path $cfg 'nvda.ini') -Force

# NVDA: the registered argv, array-form, detached; readiness = "NVDA initialized" then a 1.5 s quiet settle.
# Absolute -c/-f, as wdio passes them: portable NVDA resolves a relative path against its own install folder.
$nvdaProc = Start-Process -FilePath $nvda -ArgumentList @('-m', '--no-sr-flag', '-c', $cfg, '-l', '12', '-f', $log) -WorkingDirectory $root -PassThru
$deadline = (Get-Date).AddSeconds(30)
$ready = $false
while ((Get-Date) -lt $deadline) {
  if ((Test-Path -LiteralPath $log) -and ((Get-Content -LiteralPath $log -Raw -ErrorAction SilentlyContinue) -match 'NVDA initialized')) { $ready = $true; break }
  Start-Sleep -Milliseconds 250
}
if (-not $ready) { Write-Output 'NVDA not ready within 30 s'; & $nvda -q; exit 3 }
$size = (Get-Item -LiteralPath $log).Length; $stable = Get-Date; $settleEnd = (Get-Date).AddSeconds(8)
while ((Get-Date) -lt $settleEnd -and ((Get-Date) - $stable).TotalMilliseconds -lt 1500) {
  Start-Sleep -Milliseconds 250
  $now = (Get-Item -LiteralPath $log).Length
  if ($now -ne $size) { $size = $now; $stable = Get-Date }
}
Stamp @{ phase = 'nvda'; n = 0; input = 'none'; action = 'ready'; landing = '' }
Write-Output 'nvda ready'

# tauri-driver the way wdio.conf.ts spawns it: node + the repo-derived cli.js, --native-driver, cwd = repo root,
# the sr-empty subject's env (removed from this script's env once the child holds it).
$env:CONDUCTOR_RUNS_DIR = 'runs/e2e-fixture'
$env:CONDUCTOR_SCENARIOS_DIR = 'runs/sr-leg/empty'
$td = Start-Process -FilePath $node -ArgumentList @($cli, '--native-driver', $driver) -WorkingDirectory $root -WindowStyle Hidden -PassThru `
  -RedirectStandardOutput (Join-Path $dir 'tauri-driver.driver-os.out') -RedirectStandardError (Join-Path $dir 'tauri-driver.driver-os.err')
Remove-Item Env:CONDUCTOR_RUNS_DIR, Env:CONDUCTOR_SCENARIOS_DIR
$up = $false
$deadline = (Get-Date).AddSeconds(20)
while ((Get-Date) -lt $deadline) {
  try { [void](Wd 'Get' '/status' $null 5); $up = $true; break } catch { }
  Start-Sleep -Milliseconds 250
}

$script:sid = $null
if ($up) {
  try {
    $caps = @{ capabilities = @{ alwaysMatch = @{ browserName = 'wry'; 'tauri:options' = @{ application = $app } } } } | ConvertTo-Json -Compress -Depth 6
    $sess = (Wd 'Post' '/session' $caps 120).value
    $script:sid = $sess.sessionId
    Stamp @{ phase = 'session'; n = 0; input = 'none'; action = 'session-created'; landing = ''; browserVersion = [string]$sess.capabilities.browserVersion }
    Write-Output ("session created, browserVersion " + $sess.capabilities.browserVersion)
  } catch { Write-Output 'session not created'; Stamp @{ phase = 'session'; n = 0; input = 'none'; action = 'session-failed'; landing = '' } }
} else { Write-Output 'driver not ready'; Stamp @{ phase = 'session'; n = 0; input = 'none'; action = 'driver-not-ready'; landing = '' } }

if ($script:sid) {
  $sid = $script:sid
  $mounted = $false
  $deadline = (Get-Date).AddSeconds(30)
  while ((Get-Date) -lt $deadline) {
    try { if ((Wd 'Post' "/session/$sid/execute/sync" (ExecBody $mountedJs)).value -eq $true) { $mounted = $true; break } } catch { }
    Start-Sleep -Milliseconds 500
  }
  Stamp @{ phase = 'session'; n = 0; input = 'none'; action = ("mounted " + $mounted); landing = '' }

  # The leg's exact activation argv (screen-reader.e2e.ts bringToForeground).
  & powershell.exe -NoProfile -ExecutionPolicy Bypass -File $activate -Title 'Conductor'
  $act = $LASTEXITCODE
  Stamp @{ phase = 'session'; n = 0; input = 'none'; action = ("activate exit " + $act); landing = '' }
  Write-Output ("activate exit " + $act)

  $appProc = Get-CimInstance Win32_Process | Where-Object { $_.Name -eq 'conductor-tauri.exe' -and ((Descendants $td.Id) -contains [int]$_.ProcessId) } | Select-Object -First 1
  $appTree = @()
  if ($appProc) { $appTree = Descendants ([int]$appProc.ProcessId) }

  # I1: the leg's start-of-document cycle (injected Tabs until the landing reads BODY), then 5 injected Tabs.
  $landing = InjectTab $sid 'I1-cycle' 1
  $k = 1
  while ($landing -ne 'BODY' -and $k -lt 10) { $k += 1; $landing = InjectTab $sid 'I1-cycle' $k }
  Start-Sleep -Seconds 6
  for ($i = 1; $i -le 5; $i++) { Start-Sleep -Seconds 4; [void](InjectTab $sid 'I1' $i) }

  # O: 5 SendInput Tabs, each gated on the foreground belonging to the app's tree; never re-activate.
  Start-Sleep -Seconds 4
  Stamp @{ phase = 'O'; n = 0; input = 'none'; action = 'phase-start'; landing = '' }
  for ($i = 1; $i -le 5; $i++) { Start-Sleep -Seconds 4; OsTab 'O' $i $appTree }

  # I2: 5 injected Tabs.
  Start-Sleep -Seconds 4
  Stamp @{ phase = 'I2'; n = 0; input = 'none'; action = 'phase-start'; landing = '' }
  for ($i = 1; $i -le 5; $i++) { Start-Sleep -Seconds 4; [void](InjectTab $sid 'I2' $i) }

  Start-Sleep -Seconds 4
  Stamp @{ phase = '@end'; n = 0; input = 'none'; action = '@end'; landing = '' }
}

# Teardown: NVDA first so the app's teardown is not what it records last, then the session and the driver tree.
& $nvda -q
$deadline = (Get-Date).AddSeconds(15)
while ((Get-Date) -lt $deadline -and (Get-Process -Name nvda -ErrorAction SilentlyContinue)) { Start-Sleep -Milliseconds 250 }
Stamp @{ phase = 'nvda'; n = 0; input = 'none'; action = 'quit'; landing = '' }
$tree = Descendants $td.Id
if ($script:sid) { try { [void](Wd 'Delete' "/session/$($script:sid)" $null) } catch { Write-Output 'session delete failed' } }
Start-Sleep -Seconds 2
$tree = @($tree) + @(Descendants $td.Id) | Select-Object -Unique
foreach ($id in $tree) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
Stamp @{ phase = 'driver'; n = 0; input = 'none'; action = 'stopped'; landing = '' }
Write-Output 'walk complete'
exit 0
```
