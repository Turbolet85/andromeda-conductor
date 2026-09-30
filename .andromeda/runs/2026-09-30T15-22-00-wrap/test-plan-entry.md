
## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — the browse-mode zone narrowed; the SR firing form gains the OS key path
**Section:** §1 Test Scope Summary → Untestable zones (screen-reader browse mode); §6 E2E Test Strategy → desktop-webview row, the screen-reader family
**Change:**
- §1 was "screen-reader BROWSE-MODE reading … 14 rows `not-run-here` … not a missing driver but a missing key path — OS-level key injection would make these rows agent-driven (a route-owned CARRY)". Now the zone covers only browse rows whose window carries NO OS-level key:
  - since 2026-09-30 the `sr*` legs send Tab, Shift+Tab and `h` / `d` / ArrowDown through `send-keys.ps1` (`SendInput`, foreground-guarded), so every browse row driven by an OS key is agent-graded (`input` = `os`);
  - the rest stay `not-run-here` on the operator arm, named as a set, not a count;
  - injected keys still never deliver browse commands.
- §6 was "the browse-mode rows stay findings pending OS-level key injection". Now the rows it drives with an OS key are agent-graded and the rest stay findings. The firing form gains a per-key `send-keys.ps1` (`powershell.exe -File`, a closed `-Key` constant, sent only while `conductor-tauri` holds the foreground, exit 4 = nothing sent, a refusal throws with no injected fallback). The reset cycle and every other key stay injected, and each row records its input path.
**Why:** the key path the zone named as its remedy now exists and was measured on all three subjects (every OS browse key in NVDA's `Input:` log).
**Kept:** the zone itself, narrowed rather than deleted; the `focus` / `live` agent-driven sentence.
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/
