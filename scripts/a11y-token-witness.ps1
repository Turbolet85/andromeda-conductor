#Requires -Version 7
<#
.SYNOPSIS
  CI-only. The a11y routine arm's entry point INSIDE the medium-integrity launch: witnesses the token
  it actually got, then runs the leg.

.DESCRIPTION
  The launcher's own `A11Y_LIMITED_TOKEN_CHILD_INTEGRITY` line records the label it SET on the token it
  built. That is a reading taken in the launching process, not in the launched one, and the cause-probe
  step's `IsElevatedAdmin` is likewise read in the job's own shell. Without a reading from inside the
  launched process, "integrity is not the cause" and "the label never took" are indistinguishable — and
  a conclusion resting on that ambiguity would not meet this project's bar (variation with a control on
  both sides).

  So this runs as the launched program, prints the token facts before wdio starts, and then invokes the
  leg, propagating its exit code unchanged.

  Integrity level is printed beside the role check because they answer DIFFERENT questions, and the
  2026-09-16 legs established that only the second one moves the outcome: a process can carry the
  administrator role at High integrity and still get no debugging endpoint, while a non-admin process at
  Medium gets one. `runas /trustlevel` strips the group and leaves the label, which is exactly why role
  alone never separated the two.

.NOTES
  Invoked solely as the program launched by scripts/a11y-limited-token-launch.ps1; wired into neither
  harness shell. Prints handle NAMES and states only, never a resolved path.
#>
[CmdletBinding()]
param(
  # 'driver-alone' runs the session-isolation step's own (b) sequence INSIDE this launch instead of the
  # leg. That is the discriminating variation: (b) runs at High today and returns `DevToolsActivePort
  # file doesn't exist`, while the leg at Medium returns `from chrome not reachable`. Running the SAME
  # sequence at Medium holds the instrument and varies only integrity, which separates "the wdio /
  # tauri-driver path produces that error" from "the driver-to-browser step produces it under Medium".
  [ValidateSet('leg', 'driver-alone')]
  [string]$Mode = 'leg',

  # The TRANSPORT the driver uses to reach the browser. Every failure measured so far belongs to 'port'
  # and to its two failure points — the DevToolsActivePort file (absent at High) and the loopback TCP
  # connect (refused at Medium). msedgedriver's own log recommends the pipe instead, in both the High and
  # the Medium run: "Use the --remote-debugging-pipe Chrome switch instead of the default
  # --remote-debugging-port". A pipe bypasses both failure points and has never been exercised AS A
  # TRANSPORT — the workflow's existing pipe-route step launches the app with the switch set but never
  # speaks over it, and its profile-directory observation is not pipe-specific because the bare-app
  # control produces one too.
  [ValidateSet('port', 'pipe')]
  [string]$Transport = 'port',

  # Distinguishes the driver logs of runs that would otherwise share a path (the medium launch and its
  # high-integrity control run the same code).
  [string]$LogTag = 'x'
)

Set-StrictMode -Version Latest

# These lines are written to a gitignored file under runs/ as well as stdout, so the launching step can
# print them into the job log even if the child's console is not the step's own.
$witnessLog = Join-Path $PSScriptRoot '..' | Join-Path -ChildPath 'runs/a11y-token-witness.log'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $witnessLog) | Out-Null
Set-Content -LiteralPath $witnessLog -Value @() -Encoding UTF8

function Write-Witness([string]$Line) {
  Write-Output $Line
  Add-Content -LiteralPath $witnessLog -Value $Line -Encoding UTF8
}

try {
  $id = [Security.Principal.WindowsIdentity]::GetCurrent()
  $principal = New-Object Security.Principal.WindowsPrincipal($id)
  Write-Witness "[diag] (leg) IsElevatedAdmin: $($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
  Write-Witness "[diag] (leg) IsSystem: $($id.IsSystem)"
  Write-Witness "[diag] (leg) AuthenticationType: $($id.AuthenticationType)"
} catch {
  Write-Witness "[diag] (leg) WindowsIdentity: UNREADABLE ($($_.Exception.GetType().Name))"
}

# The integrity level is the decisive reading: a launch whose label never took shows High
# (S-1-16-12288), a genuine drop shows Medium (S-1-16-8192). It is NOT reachable through
# WindowsIdentity.Groups — .NET omits the mandatory label from that collection (measured: 11 groups,
# no S-1-16-* among them) — so it is read from whoami, invoked by ABSOLUTE PATH because a bare
# `whoami` resolves to a POSIX build on a host with MSYS ahead on PATH and rejects /groups.
try {
  $whoami = Join-Path $env:SystemRoot 'System32\whoami.exe'
  if (Test-Path -LiteralPath $whoami) {
    $line = (& $whoami /groups) | Select-String -Pattern 'S-1-16-' | Select-Object -First 1
    $sid = if ($line -and $line.Line -match '(S-1-16-\d+)') { $Matches[1] } else { $null }
    # Named from the SID, never from whoami's label text: the text is localized and the SID is not.
    $names = @{
      'S-1-16-0' = 'Untrusted'; 'S-1-16-4096' = 'Low'; 'S-1-16-8192' = 'Medium'
      'S-1-16-8448' = 'Medium Plus'; 'S-1-16-12288' = 'High'; 'S-1-16-16384' = 'System'
    }
    $name = if ($sid -and $names.ContainsKey($sid)) { $names[$sid] } else { 'unmapped' }
    Write-Witness "[diag] (leg) IntegrityLevelSid: $(if ($sid) { $sid } else { 'UNREADABLE' })"
    Write-Witness "[diag] (leg) IntegrityLevel: $(if ($sid) { $name } else { 'UNREADABLE' })"
  } else {
    Write-Witness '[diag] (leg) IntegrityLevel: UNREADABLE (whoami.exe absent)'
  }
} catch {
  Write-Witness "[diag] (leg) IntegrityLevel: UNREADABLE ($($_.Exception.GetType().Name))"
}

try {
  Write-Witness "[diag] (leg) SessionId: $([System.Diagnostics.Process]::GetCurrentProcess().SessionId)"
} catch {
  Write-Witness '[diag] (leg) SessionId: UNREADABLE'
}

# Window station / desktop is an axis nothing has varied, and it produces this exact signature class:
# a process on a non-interactive station (winsta0 is the only one with a visible desktop) cannot bring
# up a windowed browser host. Reported so the axis stops being invisible in every future reading.
try {
  if (-not ('WinSta' -as [type])) {
    Add-Type -Namespace '' -Name 'WinSta' -MemberDefinition @'
[DllImport("user32.dll", SetLastError=true)] public static extern IntPtr GetProcessWindowStation();
[DllImport("user32.dll", SetLastError=true)] public static extern bool GetUserObjectInformation(IntPtr hObj, int nIndex, System.Text.StringBuilder pvInfo, int nLength, out int lpnLengthNeeded);
'@
  }
  $sb = New-Object System.Text.StringBuilder 256
  $needed = 0
  $ok = [WinSta]::GetUserObjectInformation([WinSta]::GetProcessWindowStation(), 2, $sb, $sb.Capacity, [ref]$needed)
  Write-Witness "[diag] (leg) WindowStation: $(if ($ok) { $sb.ToString() } else { 'UNREADABLE' })"
} catch {
  Write-Witness "[diag] (leg) WindowStation: UNREADABLE ($($_.Exception.GetType().Name))"
}
Write-Witness "[diag] (leg) UserInteractive: $([Environment]::UserInteractive)"
Write-Witness "[diag] (leg) SessionName: $(if ($env:SESSIONNAME) { $env:SESSIONNAME } else { 'ABSENT' })"
Write-Witness "[diag] (leg) Mode: $Mode"

# msedgedriver builds its profile at %TEMP%\scoped_dir<pid>_<rand>\EBWebView. Mandatory integrity denies
# write-UP, so a Medium child that cannot create or write that path makes the browser die immediately —
# which surfaces to the driver as exactly `from chrome not reachable`. This probe is what separates that
# reading from "WebView2 behaves differently at Medium", and it is cheap.
# TEMP is reported because a CreateProcessAsUser child does NOT necessarily inherit the parent's — this
# launcher hands over an explicit environment block, so the child's TEMP is a launcher-side fact worth
# measuring rather than assuming. Reported as leaf segments and booleans, never a resolved path, per this
# script's own contract.
try {
  if (-not $env:TEMP) {
    Write-Witness '[diag] (leg) TempWritable: UNKNOWN (TEMP is ABSENT from this process environment)'
  } else {
    $tempLeaf = Split-Path -Leaf $env:TEMP
    $tempParentLeaf = Split-Path -Leaf (Split-Path -Parent $env:TEMP)
    Write-Witness "[diag] (leg) TempTail: ...\$tempParentLeaf\$tempLeaf"
    $probeDir = Join-Path $env:TEMP 'scoped_dir_probe'
    $probeFile = Join-Path $probeDir 'x'
    New-Item -ItemType Directory -Force -Path $probeDir -ErrorAction Stop | Out-Null
    Set-Content -LiteralPath $probeFile -Value 'x' -NoNewline -ErrorAction Stop
    $readBack = Get-Content -LiteralPath $probeFile -Raw -ErrorAction Stop
    Write-Witness "[diag] (leg) TempWritable: $($readBack -eq 'x') (created, wrote and read back a byte under a scoped_dir-style child)"
    Remove-Item -LiteralPath $probeDir -Recurse -Force -ErrorAction SilentlyContinue
  }
} catch {
  Write-Witness "[diag] (leg) TempWritable: False ($($_.Exception.GetType().Name))"
}

# WHY the two retired routes could not lower the label: with admin-approval mode off there is no split
# token for the account, so a scheduled task's `RunLevel Limited` had nothing to drop to and was a
# silent no-op. These readings stay because they also say whether THIS launch's label survived.
# Keys are PROBED in the provider form the cmdlet needs and REPORTED in the colon-free reg.exe form —
# `HKLM:\` satisfies the host-path gate's drive-letter anchor and `HKLM\` does not (host-win32.md).
# Each key is read in its OWN try, and the property is probed with PSObject.Properties rather than
# member access: under Set-StrictMode -Version Latest, `.$v` on a result lacking the property throws
# PropertyNotFoundException, which aborted this loop after the first key in probe run 35103823579 and
# lost the two readings that matter. An absent value is data here, not an error.
$uac = 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System'
foreach ($v in @('EnableLUA', 'FilterAdministratorToken', 'ConsentPromptBehaviorAdmin')) {
  $shown = 'UNREADABLE'
  try {
    $item = Get-ItemProperty -Path $uac -Name $v -ErrorAction SilentlyContinue
    $prop = if ($item) { $item.PSObject.Properties[$v] } else { $null }
    $shown = if ($prop -and $null -ne $prop.Value) { $prop.Value } else { 'ABSENT' }
  } catch {
    $shown = "UNREADABLE ($($_.Exception.GetType().Name))"
  }
  Write-Witness "[diag] (leg) UAC HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\${v}: $shown"
}

# WHICH account the leg runs as decides whether a filtered token exists at all: the built-in
# Administrator (RID 500) has none unless FilterAdministratorToken is 1. That is why the filtered-token
# routes failed and why this launch derives a restricted token from the caller's instead.
try {
  $who = [Security.Principal.WindowsIdentity]::GetCurrent()
  $sid = $who.User.Value
  $rid = $sid.Substring($sid.LastIndexOf('-') + 1)
  Write-Witness "[diag] (leg) AccountRid: $rid"
  Write-Witness "[diag] (leg) AccountIsBuiltinAdministrator: $($rid -eq '500')"
} catch {
  Write-Witness "[diag] (leg) AccountRid: UNREADABLE ($($_.Exception.GetType().Name))"
}

# The handles reach this process through the User-scope forwarding the launcher performs; report
# presence only, never a value or a resolved path.
foreach ($n in @('CONDUCTOR_A11Y_STRICT', 'CONDUCTOR_ENV', 'CONDUCTOR_MSEDGEDRIVER', 'EDGEWEBDRIVER')) {
  $present = -not [string]::IsNullOrEmpty([Environment]::GetEnvironmentVariable($n))
  Write-Witness "[diag] (leg) handle ${n}: $(if ($present) { 'present' } else { 'ABSENT' })"
}

# runas does NOT inherit the caller's working directory, and the harness resolves its artifact handles
# relative to cwd (CONDUCTOR_RUNS_DIR / CONDUCTOR_SCENARIOS_DIR are repo-relative by design, because
# resolve_under rejects absolute values). Without this the leg starts in the launcher's default
# directory and dies before it reaches wdio — measured locally, the whole leg returning in ~1 minute
# with no cargo or node process ever alive.
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
# Taken BEFORE the leg starts: the reap below keys on it, so a process predating this leg is never
# touched (the attribution rule the census already applies).
$legStartedAt = Get-Date
Write-Witness "[diag] (leg) CwdOnEntry: $(if ((Get-Location).Path -eq $repoRoot) { 'repo-root' } else { 'NOT repo-root' })"
Set-Location -LiteralPath $repoRoot

if ($Mode -eq 'driver-alone') {
  # The session-isolation step's (b) sequence, run HERE so it executes at this launch's integrity. The
  # capability set is copied from that step verbatim and must stay so: browserName 'webview2' is what
  # selects the driver's WebView2-HOST launch mode, and without it the same POST fails on the dev host
  # too — i.e. the omission makes the probe measure itself rather than its subject.
  $app = Join-Path $repoRoot 'target\release\conductor-tauri.exe'
  if (-not (Test-Path -LiteralPath $app)) {
    Write-Witness '[diag] (driver-alone) app binary: NOT BUILT — nothing to measure'
    exit 0
  }
  $drvExe = Join-Path $env:EDGEWEBDRIVER 'msedgedriver.exe'
  if (-not (Test-Path -LiteralPath $drvExe)) {
    Write-Witness '[diag] (driver-alone) msedgedriver: NOT RESOLVED'
    exit 0
  }
  # A distinct port and log per (transport, tag), so concurrent or sequential variants never collide.
  $drvPort = if ($Transport -eq 'pipe') { 9517 } else { 9516 }
  $drvLog = Join-Path $repoRoot "runs/msedgedriver-$Transport-$LogTag.log"
  $drv = Start-Process -FilePath $drvExe `
    -ArgumentList "--port=$drvPort", '--verbose', "--log-path=$drvLog" -PassThru
  Start-Sleep -Seconds 3
  # The switch goes in ms:edgeOptions.args, not in WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: the driver
  # OVERWRITES that variable for the browser it launches (its own log prints the value it set), so a
  # pre-set value cannot survive. Whether the driver actually honoured the request is not assumed — its
  # verbose log is read afterwards for the transport it chose, which is this probe's known-positive check.
  $edgeOpts = @{ binary = $app; webviewOptions = @{} }
  if ($Transport -eq 'pipe') { $edgeOpts['args'] = @('--remote-debugging-pipe') }
  $caps = @{ capabilities = @{ alwaysMatch = @{ browserName = 'webview2'; 'ms:edgeOptions' = $edgeOpts } } } | ConvertTo-Json -Depth 8 -Compress
  Write-Witness "[diag] (driver-alone) transport requested: $Transport"
  $began = Get-Date
  try {
    $r = Invoke-WebRequest -Uri "http://127.0.0.1:$drvPort/session" -Method POST -Body $caps `
      -ContentType 'application/json' -TimeoutSec 120 -SkipHttpErrorCheck
    Write-Witness "[diag] (driver-alone) elapsed: $([int]((Get-Date) - $began).TotalSeconds)s"
    Write-Witness "[diag] (driver-alone) status=$($r.StatusCode)"
    Write-Witness "[diag] (driver-alone) body: $($r.Content)"
  } catch {
    Write-Witness "[diag] (driver-alone) elapsed: $([int]((Get-Date) - $began).TotalSeconds)s"
    Write-Witness "[diag] (driver-alone) threw: $($_.Exception.Message)"
  }
  if (-not $drv.HasExited) { Stop-Process -Id $drv.Id -Force -ErrorAction SilentlyContinue }
  Write-Witness "[diag] (driver-alone) driver log written: $(Test-Path -LiteralPath $drvLog)"
  # Known-positive check on the instrument itself: a requested transport the driver ignored would
  # otherwise be reported as a transport result. The port path prints DevTools HTTP attempts; the pipe
  # path should not. Counts only, so a stale or ignored switch is visible rather than inferred.
  if (Test-Path -LiteralPath $drvLog) {
    $dl = Get-Content -LiteralPath $drvLog -ErrorAction SilentlyContinue
    $httpTries = @($dl | Select-String -SimpleMatch 'DevTools HTTP Request failed').Count
    $pipeMentions = @($dl | Select-String -SimpleMatch 'remote-debugging-pipe').Count
    $launched = @($dl | Select-String -SimpleMatch 'Launching Microsoft Edge').Count
    Write-Witness "[diag] (driver-alone) driver log: launched=$launched httpAttempts=$httpTries pipeMentions=$pipeMentions"
  }
  exit 0
}

# The leg's console is lost under a detached launch, and everything it does BEFORE wdio (frontend
# build, cargo build) prints only there — so a failure in that window produced no visible output at
# all, in CI as much as locally. Captured here and printed by the launcher. Out-File -Encoding utf8
# deliberately: a bare PowerShell redirect writes UTF-16, which greps read as empty (host rule).
$legConsole = Join-Path $repoRoot 'runs/a11y-leg-console.log'
$legErr = Join-Path $repoRoot 'runs/a11y-leg-console.err.log'
# Start-Process, not a capture: `$output = & cmd` keeps the pipeline open until EVERY process holding
# the inherited stdout handle exits, so one surviving grandchild (tauri-driver, still coming up when
# wdio gave up) pins the parent long after the leg is dead — the ~15 minutes of post-mortem measured in
# probe run 35111618735. Here the child owns the file handles and -Wait waits on the child alone.
# A bare `*>` redirect is not the alternative: this project measured PowerShell redirects writing
# UTF-16 with a BOM, which greps then read as empty (host rule).
$proc = Start-Process -FilePath (Get-Command pwsh).Source `
  -ArgumentList '-NoProfile', '-File', "$PSScriptRoot/agent-run.ps1", 'run', '--e2e' `
  -WorkingDirectory $repoRoot -RedirectStandardOutput $legConsole -RedirectStandardError $legErr `
  -NoNewWindow -PassThru
# NOT -Wait. -Wait also drains the output redirection, which orphaned msedgewebview2 processes block by
# holding the inherited handles — measured twice on the dev host and matching the ~15-minute tail after
# a dead leg in CI probe 35111618735. WaitForExit waits on the CHILD alone and is unaffected.
if (-not $proc) {
  # A launch that never started must never reach the exit-code path: $proc is null, $proc.ExitCode would
  # be 0, and the step would report success over a leg that did not run. Measured in CI run 35141676310,
  # where an over-restricted token stopped the leg starting its children and the arm still exited 0.
  Write-Witness '[precondition] A11Y_LEG_START: FAILED (the leg process never started)'
  exit 96
}
$proc.WaitForExit()
$legExit = $proc.ExitCode

# The orphans are the leg's to stop (harness rule: a tool the leg started is the leg's to stop). They
# outlive wdio's own teardown because stopping a driver does not kill the browser hosts it launched.
# Reaped by START TIME so nothing predating this leg is touched.
# Repeated passes with a settle between them, NOT one enumeration: WaitForExit returns when the pwsh
# child exits, but the app tree (driver -> app -> webview hosts) outlives it, so a single pass catches
# only what happens to be alive at that instant. Measured: one pass reaped 7 while 20 in-window
# processes survived, all with readable StartTime, so the miss was ordering and not access.
# Reaped by START TIME throughout, so nothing predating this leg is ever touched.
$reaped = 0
foreach ($pass in 1..4) {
  Start-Sleep -Seconds 3
  $hit = 0
  foreach ($p in @(Get-Process -Name msedgewebview2, conductor-tauri, msedgedriver, tauri-driver -ErrorAction SilentlyContinue)) {
    try {
      if ($p.StartTime -ge $legStartedAt) { Stop-Process -Id $p.Id -Force -ErrorAction Stop; $hit++ }
    } catch { }
  }
  $reaped += $hit
  if ($hit -eq 0) { break }
}
$left = 0
foreach ($p in @(Get-Process -Name msedgewebview2, conductor-tauri, msedgedriver, tauri-driver -ErrorAction SilentlyContinue)) {
  try { if ($p.StartTime -ge $legStartedAt) { $left++ } } catch { }
}
Write-Witness "[diag] (leg) OrphansReaped: $reaped"
Write-Witness "[diag] (leg) OrphansLeft: $left"

# A zero exit is only believable if the leg actually reached wdio. The harness writes its captured wdio
# output unconditionally once it gets that far, so an ABSENT log beside exit 0 is a leg that never ran —
# the exact false green a11y-plan §11 bans, and the shape CI run 35141676310 produced. Downgrading it
# here means the step's own exit carries it, rather than a downstream gate inferring it from a missing
# violation record.
$a11yLog = Join-Path $repoRoot 'runs/a11y-e2e.log'
if ($legExit -eq 0 -and -not (Test-Path -LiteralPath $a11yLog)) {
  Write-Witness '[precondition] A11Y_LEG_UNREACHED: the leg exited 0 without producing its wdio log'
  $legExit = 97
}

# CreateProcessAsUser does not detach, so the launcher reads this process's exit code from its own
# handle. The sentinel is kept as the launcher's FALLBACK for an unusable handle reading — an absent one
# is a distinct launcher failure there, never a silent pass — and costs one small write to keep.
$sentinel = Join-Path $PSScriptRoot '..' | Join-Path -ChildPath 'runs/a11y-leg-exit.txt'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $sentinel) | Out-Null
Set-Content -LiteralPath $sentinel -Value $legExit -Encoding UTF8
exit $legExit
