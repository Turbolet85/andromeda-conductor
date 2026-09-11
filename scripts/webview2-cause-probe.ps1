# scripts/webview2-cause-probe.ps1 — read-only probes for the hosted-runner WebView2 endpoint cause.
# Driven by .github/workflows/ci.yml's `a11y` job as a continue-on-error diagnostic, and run locally to
# produce the dev-host control the CI reading is compared against. Diagnose-only: it measures, decides
# nothing, and ALWAYS exits 0 so it can never move a gate's verdict.
#
# Three readings, matching the three candidates architecture.md §Established Decisions [CI/CD] holds OPEN
# after the runtime-major hypothesis was falsified at run 34280136892:
#   (1) the WebView2 / Edge POLICY state the image carries
#   (2) the module version the host processes ACTUALLY LOADED (distinct from the EdgeUpdate `pv` value
#       the install gate asserts — that proves what was installed, not what was mapped)
#   (3) the runner's SESSION identity (interactive with a window station, or a service context)
#
# Run with: .\scripts\webview2-cause-probe.ps1 [-OutFile runs/webview2-probe/control-devhost.txt]

param([string]$OutFile)

Set-StrictMode -Version Latest

# NOT 'Stop': a probe that dies on its first unreadable key reports nothing about the other two sections,
# and an absent key is a READING here rather than a failure. Every probing call is guarded individually.
$ErrorActionPreference = 'Continue'

$out = [System.Collections.Generic.List[string]]::new()
function Emit([string]$Line) { $out.Add($Line) }

# --- (1) policy state -------------------------------------------------------------------------------
# Probed in the PowerShell provider form (`HKLM:\...`) and reported in the reg.exe form (`HKLM\...`).
# The rendering is deliberate: the committed reading is gated by a host-path grep whose drive-letter
# anchor is `[A-Za-z]:[\/]`, which the provider form's `M:\` would match — a false positive on a string
# that is a registry key, not a host file path.
Emit '[diag] (1) policy state'
$policyKeys = @(
    'HKLM:\SOFTWARE\Policies\Microsoft\Edge',
    'HKLM:\SOFTWARE\Policies\Microsoft\EdgeUpdate',
    'HKLM:\SOFTWARE\Policies\Microsoft\EdgeWebView',
    'HKCU:\SOFTWARE\Policies\Microsoft\Edge',
    'HKCU:\SOFTWARE\Policies\Microsoft\EdgeUpdate'
)
foreach ($key in $policyKeys) {
    $shown = $key -replace ':', ''
    if (-not (Test-Path -LiteralPath $key)) {
        Emit "[diag] (1) $shown : ABSENT"
        continue
    }
    try {
        $props = Get-ItemProperty -LiteralPath $key -ErrorAction Stop
        # Value NAMES only, never their contents — a policy value can itself be a host path.
        $names = @($props.PSObject.Properties.Name | Where-Object { $_ -notlike 'PS*' } | Sort-Object)
        Emit "[diag] (1) $shown : present, value-names: $($names.Count)"
        foreach ($n in $names) { Emit "[diag] (1)     name: $n" }
    } catch {
        Emit "[diag] (1) $shown : UNREADABLE ($($_.Exception.GetType().Name))"
    }
}

# --- (2) loaded module versions ---------------------------------------------------------------------
# ModuleName + FileVersion only. FileName is the module's full path and is never printed.
Emit '[diag] (2) loaded module versions'
foreach ($procName in @('msedgewebview2', 'conductor-tauri', 'msedge')) {
    $procs = @(Get-Process -Name $procName -ErrorAction SilentlyContinue)
    if ($procs.Count -eq 0) {
        Emit "[diag] (2) $procName : no live process"
        continue
    }
    Emit "[diag] (2) $procName : $($procs.Count) live process(es)"
    $reported = $false
    foreach ($proc in $procs) {
        if ($reported) { break }
        try {
            # .Modules throws for a process owned by another account — that throw IS a reading, and a
            # discriminating one: it would say the job runs under an account that cannot inspect the
            # webview host.
            $mods = @($proc.Modules | Where-Object { $_.ModuleName -match 'msedgewebview2|msedge_elf|EmbeddedBrowserWebView' })
            if ($mods.Count -eq 0) {
                Emit "[diag] (2) $procName : no matching modules in pid sample"
            } else {
                foreach ($m in $mods) {
                    Emit "[diag] (2) $procName module: $($m.ModuleName) fileversion: $($m.FileVersionInfo.FileVersion)"
                }
            }
            $reported = $true
        } catch {
            Emit "[diag] (2) $procName : ACCESS DENIED reading modules ($($_.Exception.GetType().Name))"
            $reported = $true
        }
    }
}

# --- (3) session identity ---------------------------------------------------------------------------
# .NET and WMI only — this section reads NO environment variable, deliberately: ci.yml already reads
# $env:TEMP and $env:LOCALAPPDATA without registering them in architecture.md §Occupied Resources, and a
# new env read would enlarge that gap for a fact the .NET APIs answer directly.
Emit '[diag] (3) session identity'
try {
    $cur = [System.Diagnostics.Process]::GetCurrentProcess()
    Emit "[diag] (3) SessionId: $($cur.SessionId)"
} catch {
    Emit "[diag] (3) SessionId: UNREADABLE ($($_.Exception.GetType().Name))"
}
try {
    Emit "[diag] (3) UserInteractive: $([Environment]::UserInteractive)"
} catch {
    Emit "[diag] (3) UserInteractive: UNREADABLE ($($_.Exception.GetType().Name))"
}
try {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    Emit "[diag] (3) AuthenticationType: $(if ($id.AuthenticationType) { $id.AuthenticationType } else { 'NONE' })"
    Emit "[diag] (3) IsSystem: $($id.IsSystem)"
    Emit "[diag] (3) IsAuthenticated: $($id.IsAuthenticated)"
    $principal = New-Object Security.Principal.WindowsPrincipal($id)
    Emit "[diag] (3) IsElevatedAdmin: $($principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
} catch {
    Emit "[diag] (3) WindowsIdentity: UNREADABLE ($($_.Exception.GetType().Name))"
}
try {
    # Session 0 is the non-interactive service session; a window station named other than WinSta0, or a
    # session-0 process, is what a service-context runner looks like.
    $cs = Get-CimInstance -ClassName Win32_Process -Filter "ProcessId = $PID" -ErrorAction Stop
    Emit "[diag] (3) Win32_Process SessionId: $($cs.SessionId)"
} catch {
    Emit "[diag] (3) Win32_Process SessionId: UNREADABLE ($($_.Exception.GetType().Name))"
}

Emit '[probe] complete: 3/3 sections'

$out | ForEach-Object { Write-Output $_ }

if ($OutFile) {
    try {
        $dir = Split-Path -Parent $OutFile
        if ($dir -and -not (Test-Path -LiteralPath $dir)) {
            New-Item -ItemType Directory -Force -Path $dir | Out-Null
        }
        Set-Content -LiteralPath $OutFile -Value $out -Encoding utf8
    } catch {
        # Reported, never fatal — the stdout copy is the primary channel.
        Write-Output "[probe] OutFile not written ($($_.Exception.GetType().Name))"
    }
}

exit 0
