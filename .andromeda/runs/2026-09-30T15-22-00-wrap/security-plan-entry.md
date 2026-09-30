
## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — rule (b) registers its eighth governed form
**Section:** §Security Anti-Patterns → Code Patterns, rule (b); §Input Validation → the screen-reader speech-log ingest row
**Change:**
- Rule (b) was "seven governed forms"; now **eight**. The EIGHTH is `crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1`, the `sr*` legs' per-key OS input spawn:
  - `spawnSync('powershell.exe', [...,'-File', <send-keys.ps1>, '-Key', <constant>])`, `windowsHide`, 20 s, `-File` only, never `-Command`;
  - `-Key` is a closed `ValidateSet` (`Tab` · `ShiftTab` · `h` · `d` · `ArrowDown`), so no operator value is in the vector;
  - its new capability, `SendInput` keystroke synthesis, is bounded by a foreground guard: exit 4 = nothing sent unless the foreground process is `conductor-tauri`; exit 5 = a short insert or Shift still down; a non-zero exit throws with no injected fallback;
  - no listener; array-form, so the seventh stays the sole array-SHAPE exception.
- "The four driver-stack forms" is now five, adding the OS-key form beside the window-activation form.
- The `secret_scan_gate` carve-out was "the count stays seven"; it now "moves no count".
- The W/153 carve-out was "no committed form moved, and the count stays seven"; now no committed form moved by them, their ratification stays scoped to those controls, and the eighth is a separate ratification.
- The speech-log ingest's closed sets gain `input` (`os` · `webdriver` · `mixed` · `none`).
**Why:** a boundary widening (a subprocess boundary gaining a new crossing), escalated under playbook `:124` and ratified live by the founder, relayed by the overseer. The class keeps escalating: a ninth crossing escalates again.
**Kept:** the ordinals "The SEVENTH is …" and "the seventh form the one exception to the array SHAPE" — still true of the seventh form.
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/
