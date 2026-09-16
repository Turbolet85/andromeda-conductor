#Requires -Version 7
<#
.SYNOPSIS
  CI-only. The a11y routine arm's entry point INSIDE the limited-token scheduled task: witnesses the
  token it actually got, then runs the leg.

.DESCRIPTION
  `A11Y_LIMITED_TOKEN_REGISTER: ok (RunLevel Limited, ...)` records what the scheduler was ASKED to
  register. It does not record what the task actually ran as, and the cause-probe step's own
  `IsElevatedAdmin` reading is taken in the job's elevated shell, not in the task. Without a reading
  from inside the task, "elevation is not the cause" and "the token never dropped" are
  indistinguishable — and a conclusion resting on that ambiguity would not meet this project's bar
  (variation with a control on both sides).

  So this runs as the task's program, prints the token facts before wdio starts, and then invokes the
  leg, propagating its exit code unchanged.

  Integrity level is printed beside the role check because they answer different questions: a task
  registered Limited but silently run elevated shows High integrity, while a genuine drop shows Medium.

.NOTES
  Invoked solely as the scheduled-task program registered by scripts/a11y-limited-token-launch.ps1;
  wired into neither harness shell. Prints handle NAMES and states only, never a resolved path.
#>
[CmdletBinding()]
param()

Set-StrictMode -Version Latest

# The scheduler does not capture a task's console, so these lines would otherwise be lost. They are
# written to a gitignored file under runs/ as well, which the launching step prints into the job log.
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
  Write-Witness "[diag] (task) IsElevatedAdmin: $($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
  Write-Witness "[diag] (task) IsSystem: $($id.IsSystem)"
  Write-Witness "[diag] (task) AuthenticationType: $($id.AuthenticationType)"
} catch {
  Write-Witness "[diag] (task) WindowsIdentity: UNREADABLE ($($_.Exception.GetType().Name))"
}

# The integrity level is the decisive reading: a task registered Limited but silently run elevated
# shows High (S-1-16-12288), a genuine drop shows Medium (S-1-16-8192). It is NOT reachable through
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
    Write-Witness "[diag] (task) IntegrityLevelSid: $(if ($sid) { $sid } else { 'UNREADABLE' })"
    Write-Witness "[diag] (task) IntegrityLevel: $(if ($sid) { $name } else { 'UNREADABLE' })"
  } else {
    Write-Witness '[diag] (task) IntegrityLevel: UNREADABLE (whoami.exe absent)'
  }
} catch {
  Write-Witness "[diag] (task) IntegrityLevel: UNREADABLE ($($_.Exception.GetType().Name))"
}

try {
  Write-Witness "[diag] (task) SessionId: $([System.Diagnostics.Process]::GetCurrentProcess().SessionId)"
} catch {
  Write-Witness '[diag] (task) SessionId: UNREADABLE'
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
  Write-Witness "[diag] (task) WindowStation: $(if ($ok) { $sb.ToString() } else { 'UNREADABLE' })"
} catch {
  Write-Witness "[diag] (task) WindowStation: UNREADABLE ($($_.Exception.GetType().Name))"
}
Write-Witness "[diag] (task) UserInteractive: $([Environment]::UserInteractive)"
Write-Witness "[diag] (task) SessionName: $(if ($env:SESSIONNAME) { $env:SESSIONNAME } else { 'ABSENT' })"

# WHY a Limited run level may yield a High-integrity task: with admin-approval mode off there is no
# split token for the account, so no run level can drop to one and the request is silently a no-op.
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
  Write-Witness "[diag] (task) UAC HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System\${v}: $shown"
}

# WHICH account the task runs as decides whether a filtered token exists at all: the built-in
# Administrator (RID 500) has none unless FilterAdministratorToken is 1, so a Limited run level for
# that account is a no-op even with EnableLUA on.
try {
  $who = [Security.Principal.WindowsIdentity]::GetCurrent()
  $sid = $who.User.Value
  $rid = $sid.Substring($sid.LastIndexOf('-') + 1)
  Write-Witness "[diag] (task) AccountRid: $rid"
  Write-Witness "[diag] (task) AccountIsBuiltinAdministrator: $($rid -eq '500')"
} catch {
  Write-Witness "[diag] (task) AccountRid: UNREADABLE ($($_.Exception.GetType().Name))"
}

# The handles reach this process through the User-scope forwarding the launcher performs; report
# presence only, never a value or a resolved path.
foreach ($n in @('CONDUCTOR_A11Y_STRICT', 'CONDUCTOR_ENV', 'CONDUCTOR_MSEDGEDRIVER', 'EDGEWEBDRIVER')) {
  $present = -not [string]::IsNullOrEmpty([Environment]::GetEnvironmentVariable($n))
  Write-Witness "[diag] (task) handle ${n}: $(if ($present) { 'present' } else { 'ABSENT' })"
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
Write-Witness "[diag] (task) CwdOnEntry: $(if ((Get-Location).Path -eq $repoRoot) { 'repo-root' } else { 'NOT repo-root' })"
Set-Location -LiteralPath $repoRoot

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
Write-Witness "[diag] (task) OrphansReaped: $reaped"
Write-Witness "[diag] (task) OrphansLeft: $left"

# runas detaches, so the launcher cannot read this process's exit code from the process object. The
# sentinel is the channel; an absent one is a distinct launcher failure rather than a silent pass.
$sentinel = Join-Path $PSScriptRoot '..' | Join-Path -ChildPath 'runs/a11y-leg-exit.txt'
New-Item -ItemType Directory -Force -Path (Split-Path -Parent $sentinel) | Out-Null
Set-Content -LiteralPath $sentinel -Value $legExit -Encoding UTF8
exit $legExit
