#Requires -Version 7
<#
.SYNOPSIS
  CI-only. Runs one fixed program under a MEDIUM-INTEGRITY token and returns its exit code.

.DESCRIPTION
  Hosted Windows runners execute every step at High integrity, and the mandatory INTEGRITY LABEL — not the
  administrator role — is what decides whether WebView2 grants a debugging endpoint. Established 2026-09-16 by
  variation with a control on both sides across three dev-host legs at runtime 153: admin+High and non-admin+High
  both failed with `DevToolsActivePort file doesn't exist`, non-admin+Medium created the session in six seconds.
  Role and integrity are separate token attributes, and only the second one moves the outcome.

  The app under test is launched by msedgedriver, three levels below the wdio spawn site, so the only single
  point that de-privileges every descendant is the step itself.

  MECHANISM, and why the two earlier routes are retired BY MECHANISM rather than by exhaustion. A scheduled
  task's stored `RunLevel Limited` cannot be honoured: the runner's job account is the built-in Administrator
  (RID 500) with FilterAdministratorToken off, so it has no filtered token to drop to, and the task ran at High
  three times. `runas /trustlevel` builds a restricted token but only STRIPS THE GROUP — it leaves the label at
  High (measured on the runner: role False, integrity S-1-16-12288). Neither can lower the label, which is the
  one thing that matters. This script therefore builds the token itself: DuplicateTokenEx makes a primary
  duplicate of the caller's token, SetTokenInformation lowers its mandatory label to the Medium SID, and
  CreateProcessAsUser launches the program with it. An integrity label may always be lowered, never raised.

  It duplicates rather than RESTRICTS deliberately. The finding this script implements is that the integrity
  label decides the endpoint and the administrator role does not, so the launch moves the label alone.
  CreateRestrictedToken(LUA_TOKEN) moves both — restricting SIDs plus a disabled admin group — and that
  over-restriction was measured to break child-process creation on the runner (CI run 35141676310: the label
  dropped to Medium correctly, then the leg could not start its own children and never reached wdio).

  CreateProcessAsUser does NOT detach, so the child's exit code comes from its own process handle. The leg's
  sentinel file is kept as a FALLBACK only, so an unusable handle reading still cannot become a silent pass.

  The child's environment is built explicitly and handed to CreateProcessAsUser, rather than writing User-scope
  variables as the runas route had to — a CI step should not mutate persistent user environment.

  Every failure of THIS mechanism prints its own [precondition] line and a distinct exit code, so a token-drop
  failure can never be read as a session failure.

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

# Mirrors wdio.conf.ts's UNSAFE_PATH guard. CreateProcessAsUser takes lpCommandLine as a STRING, so this form
# stays the COMPOSING class rather than an arraying one, and this guard is what keeps it outside rule (b)'s
# eval class. Every element is a repo-relative literal the workflow supplies, never an operator value.
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

Add-Type -Namespace A11yTok -Name Native -MemberDefinition @'
[StructLayout(LayoutKind.Sequential)]
public struct SID_AND_ATTRIBUTES { public IntPtr Sid; public uint Attributes; }
[StructLayout(LayoutKind.Sequential)]
public struct TOKEN_MANDATORY_LABEL { public SID_AND_ATTRIBUTES Label; }
[StructLayout(LayoutKind.Sequential)]
public struct PROCESS_INFORMATION { public IntPtr hProcess; public IntPtr hThread; public uint dwProcessId; public uint dwThreadId; }
[StructLayout(LayoutKind.Sequential, CharSet=CharSet.Unicode)]
public struct STARTUPINFO {
  public int cb; public string lpReserved; public string lpDesktop; public string lpTitle;
  public int dwX; public int dwY; public int dwXSize; public int dwYSize;
  public int dwXCountChars; public int dwYCountChars; public int dwFillAttribute; public int dwFlags;
  public short wShowWindow; public short cbReserved2; public IntPtr lpReserved2;
  public IntPtr hStdInput; public IntPtr hStdOutput; public IntPtr hStdError;
}
[DllImport("advapi32.dll", SetLastError=true)]
public static extern bool OpenProcessToken(IntPtr h, uint acc, out IntPtr tok);
[DllImport("advapi32.dll", SetLastError=true)]
public static extern bool DuplicateTokenEx(IntPtr tok, uint acc, IntPtr attrs, int impLevel, int tokType,
  out IntPtr newTok);
[DllImport("advapi32.dll", SetLastError=true)]
public static extern bool SetTokenInformation(IntPtr tok, int cls, IntPtr info, uint len);
[DllImport("advapi32.dll", SetLastError=true)]
public static extern bool GetTokenInformation(IntPtr tok, int cls, IntPtr info, uint len, out uint ret);
[DllImport("advapi32.dll", SetLastError=true, CharSet=CharSet.Unicode)]
public static extern bool ConvertStringSidToSidW(string s, out IntPtr sid);
[DllImport("advapi32.dll", SetLastError=true, CharSet=CharSet.Unicode)]
public static extern bool ConvertSidToStringSidW(IntPtr sid, out IntPtr s);
[DllImport("advapi32.dll", SetLastError=true, CharSet=CharSet.Unicode)]
public static extern bool CreateProcessAsUserW(IntPtr tok, string app, string cmd, IntPtr pa, IntPtr ta,
  bool inherit, uint flags, IntPtr env, string cwd, ref STARTUPINFO si, out PROCESS_INFORMATION pi);
[DllImport("kernel32.dll", SetLastError=true)]
public static extern uint WaitForSingleObject(IntPtr h, uint ms);
[DllImport("kernel32.dll", SetLastError=true)]
public static extern bool GetExitCodeProcess(IntPtr h, out uint code);
[DllImport("kernel32.dll", SetLastError=true)]
public static extern bool CloseHandle(IntPtr h);
'@

$TOKEN_ALL_ACCESS = 0xF01FFu
$TokenIntegrityLevel = 25
$SECURITY_IMPERSONATION = 2
$TOKEN_PRIMARY = 1
$SE_GROUP_INTEGRITY = 0x20
$CREATE_UNICODE_ENVIRONMENT = 0x400u
$MEDIUM_SID = 'S-1-16-8192'
$WAIT_TIMEOUT = 0x102u

function Get-IntegritySid([IntPtr]$Token) {
  $len = 0u
  [void][A11yTok.Native]::GetTokenInformation($Token, $TokenIntegrityLevel, [IntPtr]::Zero, 0, [ref]$len)
  if ($len -eq 0) { return 'UNREADABLE' }
  $buf = [Runtime.InteropServices.Marshal]::AllocHGlobal([int]$len)
  try {
    if (-not [A11yTok.Native]::GetTokenInformation($Token, $TokenIntegrityLevel, $buf, $len, [ref]$len)) { return 'UNREADABLE' }
    $out = [IntPtr]::Zero
    if (-not [A11yTok.Native]::ConvertSidToStringSidW([Runtime.InteropServices.Marshal]::ReadIntPtr($buf), [ref]$out)) { return 'UNREADABLE' }
    return [Runtime.InteropServices.Marshal]::PtrToStringUni($out)
  } finally { [Runtime.InteropServices.Marshal]::FreeHGlobal($buf) }
}

$sentinel = Join-Path $WorkingDirectory 'runs/a11y-leg-exit.txt'
$curTok = [IntPtr]::Zero
$newTok = [IntPtr]::Zero
$envPtr = [IntPtr]::Zero
$tmlPtr = [IntPtr]::Zero
$pi = New-Object A11yTok.Native+PROCESS_INFORMATION

try {
  New-Item -ItemType Directory -Force -Path (Split-Path -Parent $sentinel) | Out-Null
  Remove-Item -LiteralPath $sentinel -Force -ErrorAction SilentlyContinue

  if (-not [A11yTok.Native]::OpenProcessToken([Diagnostics.Process]::GetCurrentProcess().Handle, $TOKEN_ALL_ACCESS, [ref]$curTok)) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_OPEN: FAILED (OpenProcessToken, error $([Runtime.InteropServices.Marshal]::GetLastWin32Error()))"
    exit 92
  }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_PARENT_INTEGRITY: $(Get-IntegritySid $curTok)"

  # A PLAIN primary duplicate, deliberately NOT CreateRestrictedToken. This chunk's own finding is that
  # the INTEGRITY LABEL decides the endpoint and the administrator ROLE does not, so the launch must move
  # the label and nothing else. CreateRestrictedToken(LUA_TOKEN) moves BOTH — it adds restricting SIDs and
  # disables the admin group — and that over-restriction broke child-process creation on the runner:
  # measured in CI run 35141676310, where the token dropped to Medium correctly but the leg could not
  # start its own children (whoami reported ApplicationFailedException from inside it) and the arm never
  # reached wdio. Duplicating and relabelling keeps groups and privileges intact.
  if (-not [A11yTok.Native]::DuplicateTokenEx($curTok, $TOKEN_ALL_ACCESS, [IntPtr]::Zero, $SECURITY_IMPERSONATION, $TOKEN_PRIMARY, [ref]$newTok)) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_DUPLICATE: FAILED (DuplicateTokenEx, error $([Runtime.InteropServices.Marshal]::GetLastWin32Error()))"
    exit 92
  }

  $sid = [IntPtr]::Zero
  if (-not [A11yTok.Native]::ConvertStringSidToSidW($MEDIUM_SID, [ref]$sid)) {
    Write-Output '[precondition] A11Y_LIMITED_TOKEN_LABEL: FAILED (medium integrity SID unresolvable)'
    exit 92
  }
  # TOKEN_MANDATORY_LABEL is laid out by hand rather than via StructureToPtr: assigning through a nested
  # struct property in PowerShell mutates a COPY, so `$tml.Label.Sid = …` silently leaves the SID null and
  # SetTokenInformation fails with ERROR_NOACCESS (998). Measured here before this form replaced it.
  $ptrSize = [Runtime.InteropServices.Marshal]::SizeOf([Type][IntPtr])
  $tmlSize = $ptrSize * 2
  $tmlPtr = [Runtime.InteropServices.Marshal]::AllocHGlobal($tmlSize)
  [Runtime.InteropServices.Marshal]::WriteIntPtr($tmlPtr, 0, $sid)
  [Runtime.InteropServices.Marshal]::WriteInt32($tmlPtr, $ptrSize, $SE_GROUP_INTEGRITY)
  if (-not [A11yTok.Native]::SetTokenInformation($newTok, $TokenIntegrityLevel, $tmlPtr, [uint32]$tmlSize)) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_LABEL: FAILED (SetTokenInformation, error $([Runtime.InteropServices.Marshal]::GetLastWin32Error()))"
    exit 92
  }
  $childIntegrity = Get-IntegritySid $newTok
  Write-Output "[diag] A11Y_LIMITED_TOKEN_CHILD_INTEGRITY: $childIntegrity"
  if ($childIntegrity -ne $MEDIUM_SID) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_LABEL: NOT MEDIUM after set ($childIntegrity)"
    exit 92
  }

  # The child's environment is handed over explicitly. ForwardEnv names are read from THIS process and
  # written into the block; the block is sorted and double-null terminated as CreateProcessAsUser requires.
  $vars = @{}
  foreach ($e in [Environment]::GetEnvironmentVariables('Process').GetEnumerator()) { $vars[$e.Key] = $e.Value }
  foreach ($name in $ForwardEnv) {
    $v = [Environment]::GetEnvironmentVariable($name, 'Process')
    if ($null -ne $v) { $vars[$name] = $v }
  }
  $block = ''
  foreach ($k in ($vars.Keys | Sort-Object)) { $block += "$k=$($vars[$k])" + [char]0 }
  $envPtr = [Runtime.InteropServices.Marshal]::StringToHGlobalUni($block)
  Write-Output "[diag] A11Y_LIMITED_TOKEN_FORWARDED: $($ForwardEnv.Count) handle(s)"

  $si = New-Object A11yTok.Native+STARTUPINFO
  $si.cb = [Runtime.InteropServices.Marshal]::SizeOf($si)
  $cmd = '"' + $Program + '" ' + ($ArgumentList -join ' ')
  if (-not [A11yTok.Native]::CreateProcessAsUserW($newTok, $Program, $cmd, [IntPtr]::Zero, [IntPtr]::Zero,
      $true, $CREATE_UNICODE_ENVIRONMENT, $envPtr, $WorkingDirectory, [ref]$si, [ref]$pi)) {
    $err = [Runtime.InteropServices.Marshal]::GetLastWin32Error()
    if ($err -eq 1314) {
      Write-Output '[precondition] A11Y_LIMITED_TOKEN_START: FAILED (ERROR_PRIVILEGE_NOT_HELD — the caller lacks SE_INCREASE_QUOTA)'
    } else {
      Write-Output "[precondition] A11Y_LIMITED_TOKEN_START: FAILED (CreateProcessAsUser, error $err)"
    }
    exit 93
  }
  Write-Output '[precondition] A11Y_LIMITED_TOKEN_START: ok (CreateProcessAsUser at medium integrity)'

  $waited = [A11yTok.Native]::WaitForSingleObject($pi.hProcess, [uint32]($TimeoutSeconds * 1000))
  if ($waited -eq $WAIT_TIMEOUT) {
    Write-Output "[precondition] A11Y_LIMITED_TOKEN_TIMEOUT: child still running after ${TimeoutSeconds}s"
    exit 94
  }
  $code = 0u
  if (-not [A11yTok.Native]::GetExitCodeProcess($pi.hProcess, [ref]$code)) {
    # The handle reading is unusable; fall back to the leg's own sentinel rather than assume success.
    if (-not (Test-Path -LiteralPath $sentinel)) {
      Write-Output '[precondition] A11Y_LIMITED_TOKEN_SENTINEL: ABSENT (exit code unreadable from the handle)'
      exit 95
    }
    $raw = (Get-Content -LiteralPath $sentinel -Raw).Trim()
    $parsed = 0
    if (-not [int]::TryParse($raw, [ref]$parsed)) {
      Write-Output '[precondition] A11Y_LIMITED_TOKEN_SENTINEL: UNPARSEABLE'
      exit 95
    }
    Write-Output "[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: $parsed (from sentinel)"
    exit $parsed
  }
  Write-Output "[diag] A11Y_LIMITED_TOKEN_LEG_EXIT: $code"
  exit [int]$code
} finally {
  if ($pi.hProcess -ne [IntPtr]::Zero) { [void][A11yTok.Native]::CloseHandle($pi.hProcess) }
  if ($pi.hThread -ne [IntPtr]::Zero) { [void][A11yTok.Native]::CloseHandle($pi.hThread) }
  if ($newTok -ne [IntPtr]::Zero) { [void][A11yTok.Native]::CloseHandle($newTok) }
  if ($curTok -ne [IntPtr]::Zero) { [void][A11yTok.Native]::CloseHandle($curTok) }
  if ($envPtr -ne [IntPtr]::Zero) { [Runtime.InteropServices.Marshal]::FreeHGlobal($envPtr) }
  if ($tmlPtr -ne [IntPtr]::Zero) { [Runtime.InteropServices.Marshal]::FreeHGlobal($tmlPtr) }
}
