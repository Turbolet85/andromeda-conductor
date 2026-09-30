import sys

R = ".andromeda/"
EV = "conductor-0.3.0/chunks/2026-09-30-the-sr-pass-regrades-on-the-os-input-path/evidence/"


def edit(doc, pairs):
    p = R + doc
    s = open(p, encoding="utf-8", newline="").read()
    for old, new in pairs:
        n = s.count(old)
        assert n == 1, (doc, n, old[:80])
        s = s.replace(old, new)
    open(p, "w", encoding="utf-8", newline="").write(s)
    print(doc, "edits", len(pairs))


# ---- a11y-plan: Y2 (:113), Y1 + Y3 (:268)
edit("a11y-plan.md", [
    ("the operator reviews; browse-mode rows pending OS-level injection (supplemental, never sole).",
     "the operator reviews; Tab / Shift+Tab / the browse keys ride the OS input path (`send-keys.ps1`), so browse rows "
     "driven there are agent-graded and only browse rows with no OS key in their window stay operator findings "
     "(supplemental, never sole)."),
    ("browse-mode rows stay pending OS-level key injection (WebDriver-injected keys never reach NVDA's browse mode — "
     "measured on every session 2026-09-02; 14 rows recorded as findings, never passes, never a manual arm).",
     "the agent arm sends Tab, Shift+Tab and the browse keys (`h` · `d` · ArrowDown) as OS-level `SendInput` keys "
     "through `send-keys.ps1` (a closed `-Key` set, sent only while `conductor-tauri` holds the foreground, a refusal "
     "throwing with no injected fallback), while the start-of-document reset cycle, picker text, Enter, Space, Escape and "
     "the picker's arrows stay WebDriver-injected and every row records its input path (`os` · `webdriver` · `mixed` · "
     "`none`); on that OS path browse mode is reachable — every OS browse key reaches NVDA's `Input:` log and the browse "
     "rows driven on it are graded on the agent arm (on 2026-09-30, 6 heard and 2 not announced) — while WebDriver-injected "
     "keys still never reach NVDA's browse mode (measured on every session 2026-09-02), so only the browse rows with no OS "
     "key in their window stay recorded findings, never passes, never a manual arm (as measured at `" + EV + "nvda-pass.json`)."),
    ("every focus row last heard on WebView2 152.0.4191.66 (2026-09-07).",
     "every focus row last heard on the injected path on WebView2 152.0.4191.66 (2026-09-07), and on the OS input path on "
     "154.0.4258.37 (2026-09-30)."),
    ("so the silence is measured on the agent arm's injected, driver-launched path only, never as a keyboard user's "
     "experience; a physical keyboard is unmeasured. The cause is recorded, not established: NVDA's object-model choice "
     "and the runtime/driver pair are ruled out; injected keys versus launch under tauri-driver + msedgedriver stay "
     "confounded (arm S-conductor varied both at once); and the Windows cumulative updates and the desktop stay candidates "
     "against the 2026-09-07 heard run on 152.0.4191.66;",
     "so the silence is never measured as a keyboard user's experience. The confound between injected keys and the driver "
     "launch is resolved to the INPUT PATH: in one session under the leg's own tauri-driver + msedgedriver launch, injected "
     "Tabs after the first burst were heard 0 of 5 and 0 of 4 while OS-level `SendInput` Tabs were heard 5 of 5 (as "
     "measured at `" + EV + "confound-control.md`, C1, 2026-09-30), so the silence belongs to the injected path, not to the "
     "driver launch. The agent arm therefore drives Tab, Shift+Tab and the browse keys on the OS path, where its focus rows "
     "are heard on that configuration (runtime × driver 154.0.4258.37 × NVDA 2026.2 × Windows 26200.9457 × the OS input "
     "path, driver-launched): on 2026-09-30 two focus rows, S0-09 and E0-05, read `announced-differently` on missing "
     "content and none was silent (same record, `nvda-pass.json`). A physical keyboard is unmeasured — not run, by founder "
     "ruling (2026-09-30), the cause being already isolated by C1; retired, not pending. The mechanism is recorded, not "
     "established: NVDA's object-model choice, the runtime/driver pair and the driver launch are ruled out; why injected "
     "focus moves stop producing the events NVDA consumes after the first burst is not measured; and the Windows cumulative "
     "updates and the desktop stay candidates for the injected-path silence against the 2026-09-07 heard run on "
     "152.0.4191.66;"),
])

# ---- test-plan: T1 (:47), T2 (:307)
edit("test-plan.md", [
    ("- **Untestable (by agent, today):** screen-reader BROWSE-MODE reading (landmark / heading lists, static prose, table "
     "cells) — NVDA reaches it only through its browse-mode commands, which its OS keyboard hook sees and WebDriver-injected "
     "keys never deliver (measured on every `sr*` session, 2026-09-02). Source: the SR pass (`nvda-pass-spec.md` `browse` "
     "class; 14 rows `not-run-here`). Reason: not a missing driver but a missing key path — OS-level key injection "
     "(`SendInput` / `SendKeys`) would make these rows agent-driven (a route-owned CARRY); until then they are recorded "
     "findings, never passes and never a manual arm.",
     "- **Untestable (by agent, today):** screen-reader BROWSE-MODE rows whose window carries NO OS-level key — NVDA reaches "
     "browse mode (landmark / heading lists, static prose, table cells) only through its browse-mode commands, which its OS "
     "keyboard hook sees and WebDriver-injected keys never deliver (measured on every `sr*` session, 2026-09-02). Since "
     "2026-09-30 the `sr*` legs send Tab, Shift+Tab and the browse keys (`h` · `d` · ArrowDown) through the OS input path "
     "(`send-keys.ps1`, `SendInput`, foreground-guarded), so every browse row driven by an OS key is agent-graded (`input` = "
     "`os`), and the zone narrows to the rest: the pass record's `browse` rows with no OS key in their window, recorded "
     "`not-run-here` on the operator arm — findings, never passes and never a manual arm (as measured at `" + EV +
     "nvda-pass.json`). Source: the SR pass (`nvda-pass-spec.md` `browse` class)."),
    ("and the browse-mode rows stay findings pending OS-level key injection (never a manual arm, never a hard fail).",
     "and the browse rows it drives with an OS key are graded on the agent arm, while those with no OS key in their window "
     "stay findings (never a manual arm, never a hard fail)."),
    ("`activate-window.ps1` (fixed argv) bringing the app window to the OS foreground, ",
     "`activate-window.ps1` (fixed argv) bringing the app window to the OS foreground, a per-key `send-keys.ps1` "
     "(`powershell.exe -File`, a closed `-Key` constant) sending every row's Tab, Shift+Tab and browse key through "
     "`SendInput` only while `conductor-tauri` holds the foreground (exit 4 = nothing sent; a refusal THROWS, never an "
     "injected fallback) with the start-of-document reset cycle and every other key staying WebDriver-injected and every "
     "row recording its input path (added 2026-09-30), "),
])

# ---- security-plan: O1 (:120), S1–S4 (:367)
EIGHTH = (
    " **The EIGHTH is the SR leg's OS-KEY SPAWN (added 2026-09-30, `2026-09-30-the-sr-pass-regrades-on-the-os-input-path`):** "
    "`crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1`, spawned per key by the `sr*` legs "
    "(`screen-reader.e2e.ts` `osKey`) in the window-activation form's shape — `spawnSync('powershell.exe', ['-NoProfile', "
    "'-ExecutionPolicy', 'Bypass', '-File', <send-keys.ps1>, '-Key', <constant>])`, `windowsHide`, a 20 s bound, `-File` only "
    "and never `-Command` — with `-Key` a mandatory closed `ValidateSet` (`Tab` · `ShiftTab` · `h` · `d` · `ArrowDown`), so "
    "no operator value sits anywhere in the vector. Its new capability is keystroke synthesis through `SendInput`, bounded "
    "by a FOREGROUND GUARD: it resolves the foreground window's process and exits 4 with nothing sent unless that process is "
    "`conductor-tauri`, so a key never types into another window, the operator's session included; exit 5 = fewer events "
    "inserted than requested, or Shift still reading down afterwards; a non-zero exit THROWS in the leg and never falls "
    "back to an injected key. It opens no listener. Escalated under playbook `:124` (a subprocess boundary gaining a new "
    "crossing) and ratified by the founder's live word «Да делай» (2026-09-30, relayed verbatim by the overseer); the "
    "2026-09-30 «Да» for arms W and 153 stays scoped to those controls. It is array-form, so the seventh remains the one "
    "exception to the array SHAPE."
)
edit("security-plan.md", [
    ("closed `outcome` / `arm` / `review_grade` sets,",
     "closed `outcome` / `arm` / `review_grade` / `input` sets (`input` ∈ `os` · `webdriver` · `mixed` · `none`, since "
     "2026-09-30),"),
    ("— seven governed forms, none a shell string", "— eight governed forms, none a shell string"),
    ("The four driver-stack forms:", "The five driver-stack forms:"),
    ("— the window-activation step, no operator input anywhere in the vector); ",
     "— the window-activation step, no operator input anywhere in the vector); a second FIXED OS program form of the same "
     "shape (`spawnSync('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', <repo script>, '-Key', "
     "<closed-set constant>])` — the per-key OS input step, `send-keys.ps1`, foreground-guarded, no operator input anywhere "
     "in the vector); "),
    ("with a non-zero exit or a missing journal THROWING rather than seeding silently (added 2026-09-02).",
     "with a non-zero exit or a missing journal THROWING rather than seeding silently (added 2026-09-02)." + EIGHTH),
    ("so it adds no governed form and the count stays seven.", "so it adds no governed form and moves no count."),
    ("no committed form moved, and the count stays seven.",
     "no committed form moved by them — their «Да» stays scoped to those controls, and the eighth form (`send-keys.ps1`) is "
     "a separate ratification."),
])

# ---- architecture: A1 (:261)
edit("architecture.md", [
    ("the screen-reader suites additionally spawn the host NVDA named by `CONDUCTOR_NVDA` and a fixed-argv PowerShell "
     "window-activation script, neither of which opens a listener (measured 2026-09-02).",
     "the screen-reader suites additionally spawn the host NVDA named by `CONDUCTOR_NVDA`, a fixed-argv PowerShell "
     "window-activation script and, per key, a fixed-argv PowerShell key-send script (`send-keys.ps1`, a closed `-Key` set, "
     "foreground-guarded — rule (b)'s eighth governed form, founder-ratified 2026-09-30), none of which opens a listener "
     "(measured 2026-09-02; the key-send script 2026-09-30)."),
])
