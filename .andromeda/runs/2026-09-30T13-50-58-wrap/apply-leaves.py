"""Cascade step 3 — re-derive the three leaves the sweep named from the amended masters (preserve Session Additions)."""
import pathlib
R = pathlib.Path('D:/dev/projects/conductor')

def edit(rel, old, new):
    p = R / rel
    s = p.read_bytes().decode('utf-8')
    assert s.count(old) == 1, (rel, s.count(old))
    p.write_bytes(s.replace(old, new).encode('utf-8'))
    back = p.read_bytes().decode('utf-8')
    assert back.count(new) == 1, rel
    print(rel, 'ok', len(s), '->', len(back))

# a11y-plan §3 → rules/a11y.md Testing bullet
edit('.claude/rules/a11y.md',
     "its FOCUS verdict is configuration-bound: on WebView2 runtime/driver 154.0.4258.37, Windows 26200.9457, NVDA 2026.2 "
     "the agent arm hears focus only in the window's first burst under BOTH NVDA Chromium object models while live regions "
     "still speak (2026-09-30; cause recorded, not established), so state every SR verdict with its runtime × driver × NVDA "
     "× OS build",
     "its FOCUS verdict is configuration- and input-path-bound: on WebView2 runtime/driver 154.0.4258.37, Windows "
     "26200.9457, NVDA 2026.2 the agent arm (WebDriver-injected keys, the app launched under tauri-driver + msedgedriver) "
     "hears focus only in the window's first burst under BOTH NVDA Chromium object models while live regions still speak, "
     "and equally so at the 153.0.4234.48 pair — while OS-level keys (`SendInput`, which NVDA handles as physical) into the "
     "bundle launched by path are heard on every focus change, so a keyboard user on the OS path is NOT measured silent "
     "(2026-09-30; cause recorded, not established — the object model and the runtime/driver pair ruled out, injected keys "
     "vs the driver launch still confounded), so state every SR verdict with its runtime × driver × NVDA × OS build × input "
     "path")

# a11y-plan §3 → docs/a11y-summary.md item 5
edit('.claude/docs/a11y-summary.md',
     "on 154.0.4258.37 / Windows 26200.9457 / NVDA 2026.2 the focus rows are silent after the window's first burst under "
     "both NVDA object models, 2026-09-30, cause recorded, not established)",
     "on 154.0.4258.37 / Windows 26200.9457 / NVDA 2026.2 the agent arm's focus rows are silent after the window's first "
     "burst under both NVDA object models, and at the 153.0.4234.48 pair too, on the WebDriver-injected, driver-launched "
     "path only — OS-level keys into the bundle are heard on every focus change, 2026-09-30; cause recorded, not "
     "established: the object model and the runtime/driver pair ruled out, injected keys vs the driver launch confounded)")

# security-plan rule (b) → rules/security.md Subprocess bullet
edit('.claude/rules/security.md',
     "was recorded as such on 2026-09-24, operator-directed. The count stays seven.",
     "was recorded as such on 2026-09-24, operator-directed. So do two founder-ratified session-level CONTROLS on the dev "
     "host (2026-09-30, «Да» to «W и 153 — да?», relayed by the overseer): a gitignored session script's WebDriver session "
     "to the Edge browser on the registered `:4445` (arm W), and the existing `sr-empty` leg with an operator-supplied "
     "153 msedgedriver admitted by a pre-execution Authenticode + SHA-256 + version check (arm 153) — a binary is never "
     "executed before its signature verdict is read. The count stays seven.")
