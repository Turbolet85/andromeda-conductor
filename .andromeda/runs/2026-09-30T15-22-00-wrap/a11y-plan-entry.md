
## 2026-09-30-the-sr-pass-regrades-on-the-os-input-path — the SR agent arm moves to the OS input path
**Section:** §3 Harness Contract → Per-surface test spec (desktop-webview → NVDA), the browse-mode and focus-verdict clauses; §1 Scope Summary → Screen reader test pattern
**Change:**
- The browse-mode clause was "browse-mode rows stay pending OS-level key injection (… 14 rows recorded as findings …)". Now:
  - the agent arm sends Tab, Shift+Tab and `h` / `d` / ArrowDown as OS-level `SendInput` keys through `send-keys.ps1` (a closed key set, only while `conductor-tauri` holds the foreground, a refusal throwing with no injected fallback);
  - the reset cycle and every other key stay WebDriver-injected;
  - every row records its input path (`os` · `webdriver` · `mixed` · `none`);
  - browse mode is reachable on the OS path, and the browse rows driven there are agent-graded (on 2026-09-30, 6 heard and 2 not announced);
  - injected keys still never reach browse mode, so only browse rows with no OS key in their window stay findings.
  - §1 restates the same.
- The confound "injected keys versus launch under tauri-driver + msedgedriver stay confounded" (the previous entry for this section) is retired, resolved to the INPUT PATH: in one session under the leg's own driver launch, injected Tabs after the first burst 0/5 and 0/4, OS Tabs 5/5.
  - The injected-path silence is now stated for the WebDriver-INJECTED path, no longer "the agent arm".
  - The agent arm hears focus on the OS path on 154.0.4258.37 × NVDA 2026.2 × Windows 26200.9457: two focus rows read `announced-differently` on missing content, none silent.
  - The measured set gains 154.0.4258.37 on the OS path.
  - The mechanism is recorded, not established. The driver launch is ruled out beside the object model and the runtime/driver pair; the cumulative updates and the desktop stay candidates for the injected-path silence.
- "A physical keyboard is unmeasured" keeps its clause and gains its status: not run, by founder ruling, the cause being already isolated by C1; retired, not pending.
**Why:** the chunk's confound control (C1) and the three-subject regrade on the OS path measured it. The key path moved on the founder's live ratification. The founder retired arm K at the wrap, relayed by the overseer.
**Kept:** the 2026-09-02 measurement that WebDriver-injected keys never reach browse mode (still true of that path); the 153-pair silence (a dated measurement of the injected path).
**Ref:** .andromeda/runs/2026-09-30T15-22-00-wrap/
