#Requires -Version 7
<#
.SYNOPSIS
  CI-only. Runs one fixed program under a RESTRICTED (non-elevated) token and returns its exit code.

.DESCRIPTION
  Hosted Windows runners execute every step with an elevated token, and an elevated token is the
  established cause of `session not created: DevToolsActivePort file doesn't exist` on the dev host —
  varied directly with a control on both sides, the pair differing on IsElevatedAdmin alone (2026-09-12).

  The app under test is launched by msedgedriver, three levels below the wdio spawn site, so the only
  single point that de-elevates every descendant is the step itself.

  MECHANISM, and why it is not a scheduled task. The scheduled-task route was tried and is retired by
  measurement: the runner's job account is the built-in Administrator (RID 500) with
  FilterAdministratorToken off, so it has NO split token, and a stored `RunLevel Limited` cannot be
  honoured — the task ran at High integrity three times (see
  chunks/2026-09-16-a11y-ci-gate-at-an-honest-terminal/evidence/limited-token-unavailable-on-runner.md).
  `runas /trustlevel` does not depend on a split token: it builds a restricted token from the caller's.
  It also does not prompt — only `/user:` does — so the harness's headless invariant is preserved.

  runas DETACHES, so the child's exit code is not available through the process object. The leg writes
  it to a sentinel file, which this script waits for; an absent sentinel is a distinct failure, never a
  silent pass.

  Every failure of THIS mechanism prints its own [precondition] line and a distinct exit code, so a
  token-drop failure can never be read as a session failure.

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
# supplies, never an operator value; the guard keeps it that way if the call site ever changes, and it
# is what keeps the composition below outside rule (b)'s eval class.
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

$sentinel = Join-Path $WorkingDirectory 'runs/a11y-leg-exit.txt'
$saved = @{}

try {
  New-Item -ItemType Directory -Force -Path (Split-Path -Parent $sentinel) | Out-Null
  Remove-Item -LiteralPath $sentinel -Force -ErrorAction SilentlyContinue

  # runas starts a new process from this one, so the process environment is inherited; the User-scope
  # forwarding is kept because it is the channel already proven to reach a differently-tokened shell of
  # the same user, and the witness reports handle presence either way.
  foreach ($name in $ForwardEnv) {
    $saved[$name] = [Environment]::GetEnvironmentVariable($name, 'User')
    [Environment]::SetEnvironmentVariable($name, [Environment]::GetEnvironmentVariable($name, 'Process'), 'User')
  }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_FORWARDED: $($ForwardEnv.Count) handle(s)"

  # 0x20000 = "Basic User": a restricted token derived from the caller's, which is what makes this work
  # on an account that has no filtered token of its own.
  $inner = ($ArgumentList -join ' ')
  $command = '"' + $Program + ' ' + $inner + '"'
  $runas = Join-Path $env:SystemRoot 'System32\runas.exe'
  if (-not (Test-Path -LiteralPath $runas -PathType Leaf)) {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_RUNAS: NOT PRESENT'
    exit 92
  }

  Push-Location $WorkingDirectory
  try {
    $p = Start-Process -FilePath $runas -ArgumentList '/trustlevel:0x20000', $command -PassThru -NoNewWindow -Wait
  } finally { Pop-Location }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_RUNAS_EXIT: $($p.ExitCode)"
  if ($p.ExitCode -ne 0) {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_START: FAILED (runas declined to launch)'
    exit 93
  }
  Write-Output '[precondition] A11Y_LIMITED_TOKEN_START: ok (runas /trustlevel Basic User)'

  # runas returns as soon as it has launched, so the leg is still running; wait on its sentinel.
  $deadline = (Get-Date).AddSeconds($TimeoutSeconds)
  while (-not (Test-Path -LiteralPath $sentinel) -and (Get-Date) -lt $deadline) {
    Start-Sleep -Seconds 5
  }
  if (-not (Test-Path -LiteralPath $sentinel)) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_TIMEOUT: no leg sentinel after ${TimeoutSeconds}s"
    exit 94
  }

  $raw = (Get-Content -LiteralPath $sentinel -Raw).Trim()
  $code = 0
  if (-not [int]::TryParse($raw, [ref]$code)) {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_SENTINEL: UNPARSEABLE'
    exit 95
  }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: $code"
  exit $code
} finally {
  foreach ($name in $saved.Keys) {
    [Environment]::SetEnvironmentVariable($name, $saved[$name], 'User')
  }
}
