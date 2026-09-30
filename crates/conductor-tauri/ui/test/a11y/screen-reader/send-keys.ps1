# Send ONE key to Conductor through the OS input path (SendInput), and only while Conductor holds the foreground.
#
# Why it exists: OS-level keys reach NVDA's keyboard hook and WebDriver-injected keys do not. Measured 2026-09-30
# in one session under the sr* legs' own launch (tauri-driver + msedgedriver): after the window's first burst,
# injected Tabs were heard 0 of 5 and 0 of 4 while SendInput Tabs were heard 5 of 5
# (chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/confound-control.md).
#
# The key set is closed and carries no operator value. Exit 4: the foreground process is not conductor-tauri, so
# nothing was sent (a key into another window would type into the operator's session). Exit 5: SendInput
# inserted fewer events than requested, or Shift still reads down afterwards.
param(
  [Parameter(Mandatory = $true)][ValidateSet('Tab', 'ShiftTab', 'h', 'd', 'ArrowDown')][string]$Key
)
$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class ConductorKeys {
  [StructLayout(LayoutKind.Sequential)] public struct KEYBDINPUT { public ushort wVk; public ushort wScan; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Sequential)] public struct MOUSEINPUT { public int dx; public int dy; public uint mouseData; public uint dwFlags; public uint time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Explicit)] public struct InputUnion { [FieldOffset(0)] public MOUSEINPUT mi; [FieldOffset(0)] public KEYBDINPUT ki; }
  [StructLayout(LayoutKind.Sequential)] public struct INPUT { public uint type; public InputUnion u; }
  [DllImport("user32.dll", SetLastError = true)] public static extern uint SendInput(uint n, INPUT[] inputs, int size);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  static INPUT Key(ushort vk, bool up, bool extended) {
    INPUT i = new INPUT(); i.type = 1; i.u.ki.wVk = vk;
    i.u.ki.dwFlags = (up ? 2u : 0u) | (extended ? 1u : 0u);
    return i;
  }
  public static uint Send(ushort vk, bool up, bool extended) {
    INPUT[] one = new INPUT[] { Key(vk, up, extended) };
    return SendInput(1u, one, Marshal.SizeOf(typeof(INPUT)));
  }
  [DllImport("user32.dll")] public static extern short GetAsyncKeyState(int vk);
  public static uint ForegroundPid() { uint p; GetWindowThreadProcessId(GetForegroundWindow(), out p); return p; }
}
"@

$foreground = Get-Process -Id ([ConductorKeys]::ForegroundPid()) -ErrorAction SilentlyContinue
if (-not $foreground -or $foreground.ProcessName -ne 'conductor-tauri') { exit 4 }

# ArrowDown is an extended key: without the flag Windows reports numpad 2, which NVDA binds to a review command.
switch ($Key) {
  'Tab' { $vk = [uint16]0x09; $shift = $false; $extended = $false }
  'ShiftTab' { $vk = [uint16]0x09; $shift = $true; $extended = $false }
  'h' { $vk = [uint16]0x48; $shift = $false; $extended = $false }
  'd' { $vk = [uint16]0x44; $shift = $false; $extended = $false }
  'ArrowDown' { $vk = [uint16]0x28; $shift = $false; $extended = $true }
}
# Shift is released only after NVDA has handled the key it modifies. NVDA runs its own browse-mode Tab and
# Shift+Tab scripts and re-sends the key while ignoring injected input; a Shift-up injected inside that window is
# never seen, and NVDA then holds Shift for every later key (measured 2026-09-30: after one Shift+Tab, NVDA read
# each plain Tab as shift+tab and moved focus backwards).
[uint32]$sent = 0
if ($shift) { $sent += [ConductorKeys]::Send([uint16]0x10, $false, $false) }
$sent += [ConductorKeys]::Send($vk, $false, $extended)
$sent += [ConductorKeys]::Send($vk, $true, $extended)
if ($shift) {
  Start-Sleep -Milliseconds 300
  $sent += [ConductorKeys]::Send([uint16]0x10, $true, $false)
}
$expected = if ($shift) { [uint32]4 } else { [uint32]2 }
if ($sent -ne $expected) { exit 5 }
Start-Sleep -Milliseconds 50
if (([ConductorKeys]::GetAsyncKeyState(0x10) -band 0x8000) -ne 0) { exit 5 }
exit 0
