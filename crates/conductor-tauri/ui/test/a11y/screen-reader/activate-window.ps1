# Bring a top-level window to the OS foreground by its exact title.
#
# A window launched by a background process (tauri-driver, spawned by node) is created without activation,
# and NVDA follows the SYSTEM foreground focus, not the webview's document focus — so a driven app that never
# reaches the foreground is never announced (measured 2026-09-02: NVDA read the Pulse window instead). The
# brief synthetic ALT press lifts Windows' foreground lock for the calling process; nothing else is typed.
#
# FindWindow takes the class as IntPtr: a PowerShell $null bound to a `string` P/Invoke parameter arrives as
# "" and matches no class (measured 2026-09-02, exit 3 against a window that existed).
param(
  [Parameter(Mandatory = $true)][string]$Title,
  [string]$ProcessName = 'conductor-tauri'
)

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class ConductorWindow {
  [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr FindWindow(IntPtr cls, string title);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern void keybd_event(byte key, byte scan, uint flags, UIntPtr extra);
}
"@

$handle = [ConductorWindow]::FindWindow([IntPtr]::Zero, $Title)
if ($handle -eq [IntPtr]::Zero) {
  $proc = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
  if ($proc) { $handle = $proc.MainWindowHandle }
}
if ($handle -eq [IntPtr]::Zero) { exit 3 }
[ConductorWindow]::keybd_event(0x12, 0, 0, [UIntPtr]::Zero)   # ALT down
[ConductorWindow]::keybd_event(0x12, 0, 2, [UIntPtr]::Zero)   # ALT up (KEYEVENTF_KEYUP)
[void][ConductorWindow]::ShowWindow($handle, 9)                # SW_RESTORE
[void][ConductorWindow]::SetForegroundWindow($handle)
Start-Sleep -Milliseconds 200
if ([ConductorWindow]::GetForegroundWindow() -ne $handle) { exit 4 }
exit 0
