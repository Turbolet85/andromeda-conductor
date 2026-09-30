ROOT = ""


def edit(path, pairs):
    p = ROOT + path
    s = open(p, encoding="utf-8", newline="").read()
    for old, new in pairs:
        n = s.count(old)
        assert n == 1, (path, n, old[:80])
        s = s.replace(old, new)
    open(p, "w", encoding="utf-8", newline="").write(s)
    print(path, "edits", len(pairs))


edit(".claude/rules/security.md", [
    ("governs **SEVEN** forms across THREE loci", "governs **EIGHT** forms across THREE loci"),
    ("An EIGHTH crossing escalates again — this class never becomes routine.",
     "The EIGHTH arrived 2026-09-30 (escalated, ratified live by the founder, «Да делай», relayed by the overseer): "
     "`crates/conductor-tauri/ui/test/a11y/screen-reader/send-keys.ps1`, the `sr*` legs' per-key OS input spawn in the "
     "window-activation form's shape — `powershell.exe -File` with a closed `-Key` `ValidateSet`, no operator value, "
     "foreground-guarded (exit 4 = nothing sent unless `conductor-tauri` holds the foreground; exit 5 = a short insert or "
     "Shift still down; a refusal throws, never an injected fallback), no listener. A NINTH crossing escalates again — this "
     "class never becomes routine."),
    ("a binary is never executed before its signature verdict is read. The count stays seven.",
     "a binary is never executed before its signature verdict is read. No committed form moved by them — their «Да» stays "
     "scoped to those controls, and the eighth form is a separate ratification."),
])

edit(".claude/rules/a11y.md", [
    ("the operator reviewing the record; browse-mode rows are findings pending OS-level key injection, never passes and "
     "never a manual arm; Windows + NVDA host only)",
     "the operator reviewing the record; Tab / Shift+Tab / the browse keys `h` · `d` · ArrowDown ride the OS input path "
     "(`send-keys.ps1`, `SendInput`, foreground-guarded, a refusal throwing with no injected fallback) while the reset "
     "cycle and every other key stay WebDriver-injected, each row recording its input path; browse rows driven on the OS "
     "path are agent-graded, and only browse rows with no OS key in their window stay findings, never passes and never a "
     "manual arm; Windows + NVDA host only)"),
    ("its FOCUS verdict is configuration- and input-path-bound: on WebView2 runtime/driver 154.0.4258.37, Windows "
     "26200.9457, NVDA 2026.2 the agent arm (WebDriver-injected keys, the app launched under tauri-driver + msedgedriver) "
     "hears focus only in the window's first burst under BOTH NVDA Chromium object models while live regions still speak, "
     "and equally so at the 153.0.4234.48 pair — while OS-level keys (`SendInput`, which NVDA handles as physical) into the "
     "bundle launched by path are heard on every focus change, so a keyboard user on the OS path is NOT measured silent "
     "(2026-09-30; cause recorded, not established — the object model and the runtime/driver pair ruled out, injected keys "
     "vs the driver launch still confounded),",
     "its FOCUS verdict is configuration- and input-path-bound: on WebView2 runtime/driver 154.0.4258.37, Windows "
     "26200.9457, NVDA 2026.2 WebDriver-INJECTED keys into the app launched under tauri-driver + msedgedriver are heard "
     "only in the window's first burst under BOTH NVDA Chromium object models while live regions still speak, and equally "
     "so at the 153.0.4234.48 pair, while OS-level keys (`SendInput`, which NVDA handles as physical) are heard on every "
     "focus change — and in ONE session under the leg's own driver launch (C1, 2026-09-30: injected 0/5 and 0/4 after "
     "the burst, OS 5/5) the confound resolves to the INPUT PATH, not the driver launch, so the agent arm drives Tab / "
     "Shift+Tab / the browse keys on the OS path and its focus rows are heard there (mechanism recorded, not established — "
     "the object model, the runtime/driver pair and the driver launch ruled out; a physical keyboard not run, retired by "
     "founder ruling, the cause already isolated by C1),"),
])

edit(".claude/docs/a11y-summary.md", [
    ("the operator reviews; browse-mode rows pending OS-level key injection)",
     "the operator reviews; Tab / Shift+Tab / the browse keys ride the OS input path via `send-keys.ps1`, so browse rows "
     "driven there are agent-graded and only rows with no OS key in their window stay operator findings)"),
    ("OS-level keys into the bundle are heard on every focus change, 2026-09-30; cause recorded, not established: the "
     "object model and the runtime/driver pair ruled out, injected keys vs the driver launch confounded)",
     "OS-level keys are heard on every focus change; C1, 2026-09-30, resolved the confound to the INPUT PATH in one "
     "session under the leg's own driver launch — injected 0/5 and 0/4, OS 5/5 — so the agent arm now drives its keys on "
     "the OS path, where its focus rows are heard; mechanism recorded, not established: the object model, the "
     "runtime/driver pair and the driver launch ruled out; a physical keyboard not run, retired by founder ruling)"),
])

edit(".claude/docs/tests-summary.md", [
    ("Screen-reader browse-mode reading is agent-untestable today (a missing key path, not a missing driver).",
     "Screen-reader browse-mode rows are agent-untestable only where no OS-level key is sent in the row's window: since "
     "2026-09-30 the `sr*` legs send Tab / Shift+Tab / the browse keys through the OS input path (`send-keys.ps1`), so "
     "the rows they drive are agent-graded (WebDriver-injected keys still never reach browse mode)."),
])
