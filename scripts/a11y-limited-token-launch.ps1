#Requires -Version 7
<#
.SYNOPSIS
  CI-only. Runs one fixed program under a LIMITED (non-elevated) token and returns its exit code.

.DESCRIPTION
  Hosted Windows runners execute every step with an elevated token, and an elevated token is the
  established cause of `session not created: DevToolsActivePort file doesn't exist` — varied directly
  with a control on both sides on a known-good host, the pair differing on IsElevatedAdmin alone
  (2026-09-12), and confirmed a third time on the runner in CI run 35079315258.

  The app under test is launched by msedgedriver, three levels below the wdio spawn site, so the only
  single point that de-elevates every descendant is the step itself. A scheduled task with a Limited
  run level is the non-interactive form of that drop: a runas credential prompt would break the
  harness's headless invariant.

  Every failure of THIS mechanism prints its own [precondition] line and a distinct exit code, so a
  token-dance failure can never be read as a session failure. That separation is the point: the two
  outcomes call for opposite responses.

  The task's own console is not captured by the scheduler. This is not an evidence loss — the harness
  writes the runner's output to its own log file inside the leg, and the caller prints that file. The
  verdict itself travels as the exit code.

.NOTES
  Invoked solely by .github/workflows/ci.yml's a11y job; wired into neither harness shell.
  Prints handle NAMES and states only, never a resolved path (security-plan §Security Anti-Patterns).
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory)][string]$Program,
  [Parameter(Mandatory)][string[]]$ArgumentList,
  [Parameter(Mandatory)][string]$WorkingDirectory,
  [string[]]$ForwardEnv = @(),
  [int]$TimeoutSeconds = 2400
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# Mirrors wdio.conf.ts's UNSAFE_PATH guard. Every element here is a repo-relative literal the workflow
# supplies, never an operator value; the guard keeps it that way if the call site ever changes.
$unsafe = '[;&|`$<>\r\n"'']'
foreach ($a in $ArgumentList) {
  if ($a -match $unsafe -or $a -match '\s') {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_ARGV: REJECTED (metacharacter or whitespace in an argument)'
    exit 90
  }
}
if (-not (Test-Path -LiteralPath $Program -PathType Leaf)) {
  Write-Output '[precondition] A11Y_LIMITED_TOKEN_PROGRAM: NOT RESOLVED'
  exit 91
}

$taskName = "conductor-a11y-limited-$PID"
$saved = @{}

function Restore-ForwardedEnv {
  foreach ($name in $saved.Keys) {
    [Environment]::SetEnvironmentVariable($name, $saved[$name], 'User')
  }
}

try {
  # A scheduled task does not inherit the caller's process environment. The leg's handles are therefore
  # persisted at User scope for the task's lifetime and restored in `finally` — the same mechanism the
  # endpoint-cause chunk used to hand CONDUCTOR_MSEDGEDRIVER to a differently-elevated shell of the
  # same user.
  foreach ($name in $ForwardEnv) {
    $saved[$name] = [Environment]::GetEnvironmentVariable($name, 'User')
    [Environment]::SetEnvironmentVariable($name, [Environment]::GetEnvironmentVariable($name, 'Process'), 'User')
  }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_FORWARDED: $($ForwardEnv.Count) handle(s)"

  $action = New-ScheduledTaskAction -Execute $Program -Argument ($ArgumentList -join ' ') -WorkingDirectory $WorkingDirectory
  $principal = New-ScheduledTaskPrincipal -UserId "$env:USERDOMAIN\$env:USERNAME" -LogonType Interactive -RunLevel Limited
  $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries -ExecutionTimeLimit (New-TimeSpan -Seconds $TimeoutSeconds)

  try {
    Register-ScheduledTask -TaskName $taskName -Action $action -Principal $principal -Settings $settings -Force | Out-Null
  } catch {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_REGISTER: FAILED (the runner refused a Limited/Interactive principal)'
    Write-Output "[diag] A11Y_LIMITED_TOKEN_REGISTER_REASON: $($_.Exception.GetType().Name)"
    exit 92
  }
  Write-Output '[precondition] A11Y_LIMITED_TOKEN_REGISTER: ok (RunLevel Limited, LogonType Interactive)'

  try {
    Start-ScheduledTask -TaskName $taskName
  } catch {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_START: FAILED'
    exit 93
  }

  $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
  do {
    Start-Sleep -Seconds 5
    $state = (Get-ScheduledTask -TaskName $taskName).State
  } while ($state -ne 'Ready' -and (Get-Date) -lt $deadline)

  if ($state -ne 'Ready') {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_TIMEOUT: leg still $state after ${TimeoutSeconds}s"
    Stop-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
    exit 94
  }

  $result = (Get-ScheduledTaskInfo -TaskName $taskName).LastTaskResult
  # 267011 = "task has not yet run": the scheduler accepted the registration and the start, and still
  # never executed it. That is a mechanism failure, not a leg verdict, so it gets its own code.
  if ($result -eq 267011) {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_NEVER_RAN: the scheduler reports the task never executed'
    exit 95
  }

  Write-Output "[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: $result"
  exit $result
} finally {
  Restore-ForwardedEnv
  Unregister-ScheduledTask -TaskName $taskName -Confirm:$false -ErrorAction SilentlyContinue
}
