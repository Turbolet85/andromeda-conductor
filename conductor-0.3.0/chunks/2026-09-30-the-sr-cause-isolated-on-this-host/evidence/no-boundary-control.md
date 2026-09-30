# No-boundary control — where NVDA's focus silence on Conductor's webview lives

Chunk `2026-09-30-the-sr-cause-isolated-on-this-host`, chunk base `4460307`. Every reading below comes from NVDA's
own `-l 12` log, entries paired with the session's UTC stamps by entry HEADER (NVDA's clock is local, UTC+2,
converted). Host paths are described in words. The raw `nvda-speech.*.log` files stay under the gitignored
`runs/sr-control/` and `runs/sr-leg/`, and none is committed.

## Verdict

Conductor's focus changes ARE heard on WebView2 154.0.4258.37 / Windows 26200.9457 / NVDA 2026.2 when the Tab
arrives through the OS keyboard path (arm S, every one of 5 Tabs). Edge 154 is heard under both input paths (S and W).
The silence after the first burst reproduces only on the WebDriver-injected path inside Conductor's WebView2 host,
and it reproduces unchanged on runtime 153.0.4234.48 (arm 153). So the runtime/driver pair does not separate it, and
the cause is bound to the injected path × Conductor's driven WebView2 host. The mechanism is recorded, not
established.

## §PREREQ — the Rust gate deferral, closed

Deferred since `2026-09-30-the-screen-reader-pass-grades-again-on-this-host`. Both entries ran in this chunk with no
`defer` key and read green from the bare command:

| entry | exit | printed verdict |
|---|---|---|
| `bash scripts/agent-run.sh run --unit` | 0 | `Summary … 1136 tests run: 1136 passed, 0 skipped` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `Finished dev profile … target(s)` (no diagnostic) |

## §Configuration

Common to every arm unless stated otherwise:
- OS `CurrentBuild.UBR` 26200.9457 (re-read 2026-09-30T13:25Z; both NVDA logs open with `Windows 11 25H2
  (10.0.26200.9457)`).
- NVDA 2026.2 (`Starting NVDA version 2026.2` in every log), the portable copy at the leg's registered argv, `-l 12`,
  over a copy of the committed leg `nvda.ini` (no `[UIA]` section, so `allowInChromium` is the default; no
  `[keyboard]` section, so `handleInjectedKeys` is its default, true). Synth `silence`.
- 154 coherence entry: `driver 154.0.4258.37 runtime 154.0.4258.37` (runtime from the EdgeUpdate client key `pv`,
  colon-free form `HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}`).
- Edge browser entry: `edge-browser 154.0.4258.37` (client key `{56EB18F8-B008-4CBD-B6D2-8C97FE7E9062}`), the same
  Chromium build as the runtime.
- Bundle: strict `--e2e` rebuilt the release binary at the chunk base (`Compiling conductor-tauri v0.2.0` …
  `Finished release`, written 2026-09-30T13:24:25Z). The 153 record's `build_commit` reads `4460307…`.
- SearchHost's WebView2 tree: six `msedgewebview2` processes created 2026-09-26, all on runtime 153.0.4234.48 (read
  from each process's command line, version only, at 13:25Z and again unchanged at every census).
- Desktop: the parallel pulse-builder session in a window-free operator pass (the overseer measured no nvda,
  pulse-app, inject_demo or msedgedriver before each slot). Its terminal appears in the logs only between targets.

| arm | runtime | driver | Edge build | host | input path | launch | window (UTC) |
|---|---|---|---|---|---|---|---|
| S-edge | Edge 154.0.4258.37 (browser, not WebView2) | none | 154.0.4258.37 | Edge InPrivate, data: page | OS (`SendInput`) | by the session script | 13:39:40–13:40:05 |
| S-conductor | 154.0.4258.37 | none | — | release bundle, `sr-empty` env | OS (`SendInput`) | by path, repo root cwd, no driver | 13:40:12–13:40:37 |
| S-search | 153.0.4234.48 (SearchHost's own tree) | none | — | Windows search panel | OS (`SendInput`) | Win+S | 13:40:42–13:40:58 |
| W-edge | Edge 154.0.4258.37 (browser) | 154.0.4258.37 on `:4445` | 154.0.4258.37 | Edge InPrivate under WebDriver, data: page | injected (W3C actions) | by msedgedriver | 13:42:43–13:43:07 |
| R153-empty | **153.0.4234.48** (loader folder override) | **153.0.4234.48** (operator-supplied) | — | release bundle, `sr-empty` leg | injected (W3C actions) | by tauri-driver + the 153 msedgedriver | 13:44:41–13:45:17 |

## §Arms

Theft check, per arm: a focused console logs terminal entries (verification-harness.md 2026-09-30). In every arm
the pulse-builder terminal's entries fall only BEFORE the first target stamp, BETWEEN targets (after the prior
window closed) or AFTER `@end`, never inside a graded window, and no other window's foreground entry appears inside
one. Live-region speech: none of these pages carries a live region that changed during the walk, so no live-region
row is graded here; the committed record's live-region rows are untouched.

| arm | `Input:` | focus utterances (UTC, paired with the stamp) | theft | reading |
|---|---|---|---|---|
| S-edge | 5 `kb(desktop):tab` | Tab 13:39:44.70 → 13:39:44.747 "One, button"; 13:39:48.84 → 13:39:48.862 "Two, button"; 13:39:52.95 → 13:39:52.968 "Three, button"; 13:39:57.03 → 13:39:57.071 Edge's own region, "Tab bar, navigation landmark", "Search tabs, button, collapsed"; 13:40:01.17 → 13:40:01.264 "tab control", the page's tab "selected, 1 of 1" | none | heard |
| S-conductor | 5 `kb(desktop):tab` | window up 13:40:09 ("Conductor", "Conductor - Web content, region", document, banner "Conductor · idle"); Tab 13:40:16.74 → 13:40:17.020 "Minimize window, button" (after a document re-announce); 13:40:20.85 → 13:40:20.888 "Close window, button"; 13:40:24.90 → 13:40:24.955 "main landmark, Run controls, grouping, Start, button, unavailable, Control+Enter"; 13:40:28.96 → 13:40:29.087 "Capability coverage matrix, region", "Coverage rows, table", "row, current"; 13:40:33.02 → 13:40:33.070 "Run report, region, Run report rows, table, with 4 rows and 6 columns …" | none | heard |
| S-search | 3 `kb(desktop):tab` (+ `windows+s`, `escape`) | Win+S 13:40:39.32 → "Search, window", "Search box, edit, blank"; Tab 13:40:46.38 → 13:40:46.411 "CoreInput"; Tab 13:40:50.41 → nothing; Tab 13:40:54.45 → nothing | none | silent |
| W-edge | 0 | session created 13:42:40.305; page "SR control, document", heading; Tab 13:42:47.72 → 13:42:47.727 "One, button"; 13:42:51.76 → 13:42:51.772 "Two, button"; 13:42:55.81 → 13:42:55.818 "Three, button"; 13:42:59.86 → 13:42:59.880 Edge's region, "Tab bar", "Search tabs, button, collapsed"; 13:43:03.91 → nothing (focus in Edge's own tab strip; S's fifth Tab moved within it) | none | heard |
| R153-empty | 0 (injected) | session attached 13:44:35.404 to `browserVersion 153.0.4234.48`; the harness's start-of-document cycle (6 Tabs, 13:44:36) heard: "Minimize window", "Close window", "Start … Control+Enter", "coverage rows table, row current", "Run report …"; then E0-02 13:44:45.17, E0-03 13:44:49.23, E0-04 13:44:53.30, E0-05 13:44:57.37, E0-06 13:45:01.45 (each one Tab) → no utterance at all until the terminal's entry at 13:45:17.94, after `@end` 13:45:17.79 | none | silent |
| K-edge | — | not run | — | not-run |
| K-conductor | — | not run | — | not-run |
| K-search | — | not run | — | not-run |

K clearing event: the founder at the machine for a desk slot (he was at work on 2026-09-30; not granted during this
/implement). K is never simulated by S.

Witnesses: S log `grep -c 'Input: kb(desktop):tab'` = **13** (5 + 5 + 3; gate entry green). W log `grep -c 'Input: '`
= **0** at exit 1 (the injected path), and the script printed `session created`. The 153 record's `webview2_runtime`
= `153.0.4234.48`, its session banner `[webview2 153.0.4234.48 windows]` agrees, and `Spec Files: 1 passed, 1 total`
held. Its `webview2_runtime_registry` still reads 154.0.4258.37, which is expected: the loader variable overrides the
folder for this app, not the machine's installed version. `attach_observed.empty` reads `false` on 153, as on the
prior 154 records. It is derived from "a graded focus row was heard" (`parse-nvda-log.ts:400-402`), not from a
separate attach signal: NVDA did bind the window (the start-of-document cycle was heard).

Two S starts failed before this reading, and no key was sent in either: the first passed NVDA relative `-c`/`-f`
paths, which the portable copy resolves against its own install folder (it was quit, and the folder tree it created
there removed); the second hit PowerShell 5's missing `[ushort]` accelerator after Edge opened, before the first Tab
(NVDA and that Edge tree were stopped). The third start is the reading above.

## §The 153 rule, with its reading

Rule (plan step 7): SKIP only when S-edge is heard AND W-edge is heard AND S-conductor is silent; RUN in every other
combination; a void S or W runs 153.

Reading: S-edge heard, W-edge heard, **S-conductor heard** → the skip condition fails → **RUN**. The runtime folder was
re-listed immediately before the arm (`153.0.4234.48`, `154.0.4258.37`, `SetupMetrics`). The operator-supplied driver
passed admission BEFORE it executed: `Get-AuthenticodeSignature` → `status Valid`, signer `CN=Microsoft Corporation,
O=Microsoft Corporation, …`, SHA-256 equal to the overseer's `4AE19C1D…0402E`, file ProductVersion 153.0.4234.48. Then
the entry in full printed `driver 153.0.4234.48` at exit 0. Nothing was fetched by this session.

## §What each reading licenses

Bound to the configuration rows above; nothing here is stated beyond them.
- **S-conductor heard** → OS-path input is heard in Conductor on 154.0.4258.37 / 26200.9457 / NVDA 2026.2, over a
  bundle launched by path with no driver. The prior "silent after the first burst" is bound to the injected,
  driver-launched path.
- **S-edge heard, W-edge heard** → Edge 154 is heard on both input paths, under automation included. CDP/W3C
  injection is not silent across the whole of 154.
- **R153-empty silent** → the runtime/driver pair (154 vs 153) does not separate the silence at a fixed OS build,
  desktop and bundle. E0-02..E0-06 read `not-announced` with empty `heard` on 153, exactly as the prior default-arm
  `sr-empty` records on 154 (10:46Z, 11:33Z, 12:19Z).
- **S-search silent after one Tab** → SearchHost on runtime 153 varies app AND runtime. It licenses nothing about 154
  or about Conductor.

## §Candidates — narrowed only by the arms that varied them

- **Ruled out (varied and separated nothing):** NVDA's Chromium object model (prior chunk, `allowInChromium = 2`);
  the runtime/driver pair 154 vs 153 (arm 153).
- **Ruled out as sufficient:** injection alone on 154 (W-edge heard); the WebView2 runtime or Chromium 154 alone
  (S-conductor heard).
- **Standing:** the combination that every silent run shares and every heard run lacks: injected keys delivered to
  Conductor's WebView2 host launched under tauri-driver + msedgedriver. S-conductor varied TWO things at once, the
  input path and the absence of the driver launch, so this chunk does not separate them. The 2026-09-07 heard run
  (runtime 152, the same injected, driver-launched path) keeps the cumulative updates KB5124008 / KB5129195 and the
  desktop standing for the injected-path silence.
- **Mechanism — recorded, not established.** A reading consistent with every row: after the first burst,
  injected focus moves inside Conductor's driven WebView2 document stop producing the focus events NVDA consumes,
  while OS-path focus moves in the same bundle do not. No arm measured the events themselves.

## §USER consequence

Re-stated to what was measured. The prior record (`…/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/
evidence/cause-control.md` §The USER consequence, unedited) described an NVDA user who hears no focus change after
the first burst. That was measured through WebDriver-injected Tab and stated as such. An NVDA 2026.2 user on WebView2
154.0.4258.37 / Windows 26200.9457 who presses Tab through the OS keyboard path (arm S: `SendInput`, which NVDA
handles as a physical key by default) hears every focus change in Conductor: Minimize, Close, Start, the coverage
matrix and the report grid. The silence is a property of the agent arm's injected, driver-launched path on this host,
not a measured property of a keyboard user's experience. A physical keyboard (K) remains unmeasured.

## §The three owned reds — routed, none graded

Over the committed record `…/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/nvda-pass.json`,
unchanged by this chunk (this chunk's 153 record is `nvda-pass.153.json`, a control, never a regrade).

| red | disposition | owner | measured cause |
|---|---|---|---|
| focus-row-count | routed | the successor route entry the wrap pins it to (route-resolve) | agent-arm focus rows go silent after the first burst only on the injected, driver-launched path; heard on the OS path (S-conductor) and unchanged on 153 (R153-empty) |
| S0-09 / E0-05 | routed | the successor route entry the wrap pins it to (route-resolve) | the same injected-path silence (E0-05 `not-announced` on 153 as on 154); E0-09 stays a browse finding |
| one-configuration | routed | the successor route entry the wrap pins it to (route-resolve) | the live subject still carries the prior chunk's 10:49:56Z run; no regrade leg fired, by rule |

Regrade condition, for all three: re-graded only after the injected-path cause is removed or the agent arm moves to a
path that is heard (OS-level keys reach NVDA's hook, and browse mode with it). Never by a rerun until a row is
announced.

## §Ratifications

- Founder, 2026-09-30, live, relayed by the overseer, on the desk slot: «Я еще пока на работе, не могу, пусть сам
  проверит как может» ("I'm still at work, I can't; let it check for itself however it can"). It licensed S as the
  no-human primary. It ratifies no widening.
- Founder, 2026-09-30, live, relayed verbatim by the overseer, to «W и 153 — да?» ("W and 153 — yes?"): «Да» ("Yes").
  It licensed arm W (a WebDriver session driving Edge 154 on the registered `:4445`) and arm 153 (the WebView2 loader's
  executable-folder variable, set per command, over the on-disk 153 runtime with the operator-supplied 153 driver), as
  CONTROLS.
- No removal lever ran: no product lever (`additionalBrowserArgs` / the loader's browser-arguments variable), no
  runtime pin, no Windows rollback. No committed file changed (the delta guard over the chunk base prints nothing).
- Each slot was granted by the overseer's word before it fired: S ("go S"), W ("Go — desktop is quiet"), 153 ("go 153",
  with the driver path).

## §Scripts

Both live under the gitignored `runs/sr-control/` and are never committed. Their text as run is below. One elision:
the W script's loopback base URL is shown with its scheme's colon bracketed (`http[:]//`), so the host-path hygiene
anchor does not read the scheme as a drive path; the script itself holds the plain scheme. Guards: each handle is
checked for existence, is-a-file and the wdio metacharacter set, and a value is never printed.

The 153 arm used no session script. It fired the existing `npm run a11y:sr-empty` leg with `CONDUCTOR_MSEDGEDRIVER`
set per command to the operator-supplied 153 driver and `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` set per command to the
on-disk Evergreen runtime folder for 153.0.4234.48.

### `runs/sr-control/key-walk.ps1` (arms S and K)

```powershell
# SR cause control, arm S (synthetic) / arm K (physical). Session script: gitignored, text quoted in evidence.
# NVDA at the leg's registered argv over a copy of the committed leg profile; stimuli stamped as UTC JSON lines.
param(
  [Parameter(Mandatory = $true)][ValidateSet('synthetic', 'physical')][string]$Mode
)
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Set-Location $root

$unsafe = '[;&|`$<>\r\n"'']'
$nvda = $env:CONDUCTOR_NVDA
if (-not $nvda -or $nvda -match $unsafe -or -not (Test-Path -LiteralPath $nvda -PathType Leaf)) {
  Write-Output 'skip: CONDUCTOR_NVDA is unset, unsafe, or not a file'
  exit 0
}

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class SrKeys {
  [StructLayout(LayoutKind.Sequential)] public struct KEYBDINPUT { public ushort wVk; public ushort wScan; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Sequential)] public struct MOUSEINPUT { public int dx; public int dy; public uint mouseData; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Explicit)] public struct InputUnion { [FieldOffset(0)] public MOUSEINPUT mi; [FieldOffset(0)] public KEYBDINPUT ki; }
  [StructLayout(LayoutKind.Sequential)] public struct INPUT { public uint type; public InputUnion u; }
  [DllImport("user32.dll", SetLastError = true)] public static extern uint SendInput(uint n, INPUT[] inputs, int size);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
  [DllImport("user32.dll")] public static extern void keybd_event(byte key, byte scan, uint flags, UIntPtr extra);
  static INPUT Key(ushort vk, bool up) {
    INPUT i = new INPUT(); i.type = 1; i.u.ki.wVk = vk; i.u.ki.dwFlags = up ? 2u : 0u; return i;
  }
  public static uint Press(ushort[] vks) {
    INPUT[] seq = new INPUT[vks.Length * 2];
    for (int k = 0; k < vks.Length; k++) { seq[k] = Key(vks[k], false); seq[vks.Length * 2 - 1 - k] = Key(vks[k], true); }
    return SendInput((uint)seq.Length, seq, Marshal.SizeOf(typeof(INPUT)));
  }
  public static uint ForegroundPid() { uint p; GetWindowThreadProcessId(GetForegroundWindow(), out p); return p; }
}
"@

$dir = Join-Path $root 'runs\sr-control'
$log = Join-Path $dir "nvda-speech.$Mode.log"
$actions = Join-Path $dir "actions.$Mode.jsonl"
$cfg = Join-Path $dir 'nvda-config'
Remove-Item -LiteralPath $log, $actions -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cfg | Out-Null
Copy-Item -LiteralPath (Join-Path $root 'crates\conductor-tauri\ui\test\a11y\screen-reader\nvda-config\nvda.ini') -Destination (Join-Path $cfg 'nvda.ini') -Force
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Stamp([string]$target, [string]$action, [int]$n) {
  $fp = [SrKeys]::ForegroundPid()
  $fn = (Get-Process -Id $fp -ErrorAction SilentlyContinue).ProcessName
  $line = (@{ t = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ'); mode = $Mode; target = $target; action = $action; n = $n; foreground = "$fn" } | ConvertTo-Json -Compress)
  [System.IO.File]::AppendAllText($actions, $line + "`n", $utf8)
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

function Foreground([IntPtr]$hwnd, [int[]]$pids) {
  if ($pids -contains [int][SrKeys]::ForegroundPid()) { return 'already' }
  [SrKeys]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero)
  [SrKeys]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)
  [void][SrKeys]::ShowWindow($hwnd, 9)
  [void][SrKeys]::SetForegroundWindow($hwnd)
  Start-Sleep -Milliseconds 300
  if ($pids -contains [int][SrKeys]::ForegroundPid()) { return 'activated' } else { return 'failed' }
}

function Deliver([string]$target, [int]$count, [uint16]$vk) {
  if ($Mode -eq 'physical') {
    Stamp $target 'window-open' 0
    Write-Output ("PHYSICAL: press Tab {0} times now, about 4 s apart - 25 s window ({1})" -f $count, $target)
    Start-Sleep -Seconds 25
    Stamp $target 'window-close' 0
    return
  }
  for ($i = 1; $i -le $count; $i++) {
    Start-Sleep -Seconds 4
    $sent = [SrKeys]::Press(@($vk))
    Stamp $target ("tab sent=" + $sent) $i
  }
  Start-Sleep -Seconds 4
  Stamp $target '@end' 0
}

function WaitWindow([scriptblock]$pick, [int]$seconds) {
  $deadline = (Get-Date).AddSeconds($seconds)
  while ((Get-Date) -lt $deadline) {
    $p = & $pick
    if ($p) { return $p }
    Start-Sleep -Milliseconds 250
  }
  return $null
}

# NVDA: the registered argv, array-form, detached; readiness = "NVDA initialized" then a 1.5 s quiet settle.
# Absolute -c/-f, as wdio passes them: portable NVDA resolves a relative path against its own install folder.
# Start-Process joins the argv with spaces, so a whitespace-bearing root is refused rather than split.
if ($root -match '\s') { Write-Output 'repo root holds whitespace - not started'; exit 2 }
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
Stamp 'nvda' 'ready' 0
Write-Output 'nvda ready'

$page = 'data:text/html,<title>SR%20control</title><h1>SR%20control</h1><button>One</button><button>Two</button><button>Three</button>'

# (a) Edge 154, InPrivate, on the typed data: page.
$edge = Start-Process -FilePath 'msedge' -ArgumentList @('--inprivate', $page) -PassThru
$edgeWin = WaitWindow { Get-Process -Name msedge -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowTitle -like '*SR control*' } | Select-Object -First 1 } 30
if ($edgeWin) {
  Start-Sleep -Seconds 3
  $fg = Foreground $edgeWin.MainWindowHandle (Descendants $edgeWin.Id)
  Stamp 'edge' ("foreground " + $fg) 0
  Write-Output ("edge window, foreground " + $fg)
  Deliver 'edge' 5 0x09
  $edgeTree = Descendants $edge.Id
  [void]$edgeWin.CloseMainWindow()
  Start-Sleep -Seconds 3
  foreach ($id in $edgeTree) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
  Stamp 'edge' 'closed' 0
} else { Stamp 'edge' 'no-window' 0; Write-Output 'edge: no window' }

# (b) Conductor's release bundle under the sr-empty subject's env, launched by path from the repo root.
$env:CONDUCTOR_RUNS_DIR = 'runs/e2e-fixture'
$env:CONDUCTOR_SCENARIOS_DIR = 'runs/sr-leg/empty'
$app = Start-Process -FilePath (Join-Path $root 'target\release\conductor-tauri.exe') -WorkingDirectory $root -PassThru
Remove-Item Env:CONDUCTOR_RUNS_DIR, Env:CONDUCTOR_SCENARIOS_DIR
$appWin = WaitWindow { $p = Get-Process -Id $app.Id -ErrorAction SilentlyContinue; if ($p -and $p.MainWindowHandle -ne 0) { $p } } 30
if ($appWin) {
  Start-Sleep -Seconds 3
  $fg = Foreground $appWin.MainWindowHandle (Descendants $app.Id)
  Stamp 'conductor' ("foreground " + $fg) 0
  Write-Output ("conductor window, foreground " + $fg)
  Deliver 'conductor' 5 0x09
} else { Stamp 'conductor' 'no-window' 0; Write-Output 'conductor: no window' }
$appTree = Descendants $app.Id
foreach ($id in $appTree) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
Stamp 'conductor' 'stopped' 0

# (c) The Windows search panel (SearchHost), opened by Win+S; Escape closes it.
Start-Sleep -Seconds 2
if ($Mode -eq 'synthetic') { [void][SrKeys]::Press(@(0x5B, 0x53)) } else { Write-Output 'PHYSICAL: press Win+S now' }
Stamp 'search' 'open' 0
Start-Sleep -Seconds 3
Stamp 'search' 'opened' 0
Deliver 'search' 3 0x09
if ($Mode -eq 'synthetic') { [void][SrKeys]::Press(@(0x1B)) } else { Write-Output 'PHYSICAL: press Escape now'; Start-Sleep -Seconds 5 }
Stamp 'search' 'escape' 0
Start-Sleep -Seconds 2

& $nvda -q
$deadline = (Get-Date).AddSeconds(15)
while ((Get-Date) -lt $deadline -and (Get-Process -Name nvda -ErrorAction SilentlyContinue)) { Start-Sleep -Milliseconds 250 }
Stamp 'nvda' 'quit' 0
Write-Output 'walk complete'
exit 0
```

### `runs/sr-control/edge-webdriver-walk.ps1` (arm W)

```powershell
# SR cause control, arm W: a WebDriver session driving Edge 154, keys injected over the driver (the CDP path).
# Session script: gitignored, text quoted in evidence. Driver on the registered dev-only native-driver port 4445.
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
Set-Location $root
$Mode = 'webdriver'

$unsafe = '[;&|`$<>\r\n"'']'
$nvda = $env:CONDUCTOR_NVDA
$driver = $env:CONDUCTOR_MSEDGEDRIVER
foreach ($h in @($nvda, $driver)) {
  if (-not $h -or $h -match $unsafe -or -not (Test-Path -LiteralPath $h -PathType Leaf)) {
    Write-Output 'skip: CONDUCTOR_NVDA or CONDUCTOR_MSEDGEDRIVER is unset, unsafe, or not a file'
    exit 0
  }
}
$held = Get-NetTCPConnection -State Listen -ErrorAction SilentlyContinue | Where-Object { $_.LocalPort -in 4444, 4445 }
if ($held) { Write-Output 'a listener already holds 4444/4445 - not started'; exit 2 }

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class SrWin {
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
  [DllImport("user32.dll")] public static extern void keybd_event(byte key, byte scan, uint flags, UIntPtr extra);
  public static uint ForegroundPid() { uint p; GetWindowThreadProcessId(GetForegroundWindow(), out p); return p; }
}
"@

$dir = Join-Path $root 'runs\sr-control'
$log = Join-Path $dir "nvda-speech.$Mode.log"
$actions = Join-Path $dir "actions.$Mode.jsonl"
$cfg = Join-Path $dir 'nvda-config'
Remove-Item -LiteralPath $log, $actions -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $cfg | Out-Null
Copy-Item -LiteralPath (Join-Path $root 'crates\conductor-tauri\ui\test\a11y\screen-reader\nvda-config\nvda.ini') -Destination (Join-Path $cfg 'nvda.ini') -Force
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Stamp([string]$target, [string]$action, [int]$n) {
  $fp = [SrWin]::ForegroundPid()
  $fn = (Get-Process -Id $fp -ErrorAction SilentlyContinue).ProcessName
  $line = (@{ t = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ'); mode = $Mode; target = $target; action = $action; n = $n; foreground = "$fn" } | ConvertTo-Json -Compress)
  [System.IO.File]::AppendAllText($actions, $line + "`n", $utf8)
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

$base = 'http[:]//127.0.0.1:4445'
function Wd([string]$method, [string]$path, [string]$body) {
  if ($body) { return Invoke-RestMethod -Method $method -Uri ($base + $path) -ContentType 'application/json' -Body $body -TimeoutSec 60 }
  return Invoke-RestMethod -Method $method -Uri ($base + $path) -TimeoutSec 60
}

# NVDA: the registered argv, array-form, detached; readiness = "NVDA initialized" then a 1.5 s quiet settle.
# Absolute -c/-f, as wdio passes them: portable NVDA resolves a relative path against its own install folder.
# Start-Process joins the argv with spaces, so a whitespace-bearing root is refused rather than split.
if ($root -match '\s') { Write-Output 'repo root holds whitespace - not started'; exit 2 }
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
Stamp 'nvda' 'ready' 0
Write-Output 'nvda ready'

$page = 'data:text/html,<title>SR%20control</title><h1>SR%20control</h1><button>One</button><button>Two</button><button>Three</button>'
$drv = Start-Process -FilePath $driver -ArgumentList @('--port=4445') -WindowStyle Hidden -PassThru
$up = $false
$deadline = (Get-Date).AddSeconds(15)
while ((Get-Date) -lt $deadline) {
  try { if ((Wd 'Get' '/status' $null).value.ready) { $up = $true; break } } catch { }
  Start-Sleep -Milliseconds 250
}
$sid = $null
if ($up) {
  try {
    $caps = '{"capabilities":{"alwaysMatch":{"browserName":"MicrosoftEdge","ms:edgeOptions":{"args":["--inprivate"]}}}}'
    $sid = (Wd 'Post' '/session' $caps).value.sessionId
  } catch { Write-Output 'session not created' }
} else { Write-Output 'driver not ready' }

if ($sid) {
  Write-Output 'session created'
  Stamp 'edge' 'session-created' 0
  [void](Wd 'Post' "/session/$sid/url" ('{"url":"' + $page + '"}'))
  Start-Sleep -Seconds 3
  $tree = Descendants $drv.Id
  $win = Get-Process -Name msedge -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 -and ($tree -contains $_.Id) } | Select-Object -First 1
  $fg = 'no-window'
  if ($win) {
    if ((Descendants $win.Id) -contains [int][SrWin]::ForegroundPid() -or $tree -contains [int][SrWin]::ForegroundPid()) { $fg = 'already' }
    else {
      [SrWin]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero)
      [SrWin]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)
      [void][SrWin]::ShowWindow($win.MainWindowHandle, 9)
      [void][SrWin]::SetForegroundWindow($win.MainWindowHandle)
      Start-Sleep -Milliseconds 300
      if ($tree -contains [int][SrWin]::ForegroundPid()) { $fg = 'activated' } else { $fg = 'failed' }
    }
  }
  Stamp 'edge' ("foreground " + $fg) 0
  Write-Output ("edge window, foreground " + $fg)
  $tabKey = [string][char]92 + 'uE004'
  $tab = '{"actions":[{"type":"key","id":"kb","actions":[{"type":"keyDown","value":"' + $tabKey + '"},{"type":"keyUp","value":"' + $tabKey + '"}]}]}'
  for ($i = 1; $i -le 5; $i++) {
    Start-Sleep -Seconds 4
    [void](Wd 'Post' "/session/$sid/actions" $tab)
    Stamp 'edge' 'tab' $i
  }
  Start-Sleep -Seconds 4
  Stamp 'edge' '@end' 0
  try { [void](Wd 'Delete' "/session/$sid" $null) } catch { Write-Output 'session delete failed' }
  Stamp 'edge' 'session-deleted' 0
}

$tree = Descendants $drv.Id
Start-Sleep -Seconds 2
foreach ($id in $tree) { Stop-Process -Id $id -Force -ErrorAction SilentlyContinue }
Stamp 'driver' 'stopped' 0

& $nvda -q
$deadline = (Get-Date).AddSeconds(15)
while ((Get-Date) -lt $deadline -and (Get-Process -Name nvda -ErrorAction SilentlyContinue)) { Start-Sleep -Milliseconds 250 }
Stamp 'nvda' 'quit' 0
Write-Output 'walk complete'
exit 0
```

## §Census

`Win32_Process` with parentage over nvda, msedge, msedgedriver, conductor-tauri, tauri-driver, node, msedgewebview2,
SearchHost, pulse-app and andromeda-pulse-mcp. The baseline at every census was SearchHost (created 2026-09-26) and
its six `msedgewebview2` children, the OS's own and never this session's. Edge's own background traffic in the
InPrivate windows is the browser's, not the leg's; it gets no row.

| census (UTC) | beyond the SearchHost baseline |
|---|---|
| pre-chunk 13:23:16 | none |
| post-`--e2e` 13:25:49 | none |
| pre-S 13:36:39 | none |
| after S start 2 aborted 13:38:44 | nvda ×1 (13:38:29) and an msedge tree ×19 (13:38:33), both parented by the dead script → stopped (`nvda -q`; the tree's root) → re-read 0 |
| pre-S (start 3) | none |
| post-S 13:41:09 | an msedge tree ×9 created 13:40:06, rooted at Edge's own background relaunch (`--no-startup-window`) when the leg closed its window → stopped by its root → re-read none |
| pre-W | none; no 4444/4445 listener |
| post-W 13:43:17 | none; no 4444/4445 listener |
| pre-153 | none; no 4444/4445 listener |
| post-153 13:45:29 | none; no 4444/4445 listener (the leg's own record: node ×2 before and after, the wdio runner's, gone at exit) |

| process | started by | final state |
|---|---|---|
| NVDA (5 sessions: S starts 1–3, W, 153) | this run's legs | terminated (`-q`, polled to absence; the 153 leg's harness quit its own) |
| Edge InPrivate trees (S start 2, S start 3, its background relaunch) | this run's S leg | terminated |
| Edge under WebDriver | this run's W leg | terminated (session deleted, the driver's tree stopped) |
| msedgedriver 154 on `:4445` | this run's W leg | terminated |
| conductor-tauri (S, by path) | this run's S leg | terminated |
| msedgedriver 153, tauri-driver, conductor-tauri, node (153 leg) | this run's 153 leg | terminated (the harness's `onComplete`) |
| SearchHost + its WebView2 tree | the OS | left running: not this session's |
| pulse-builder's terminal | the operator's parallel session | left running: not this session's |
