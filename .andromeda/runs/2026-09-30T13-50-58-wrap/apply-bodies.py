"""P2 apply — A1+A2 (a11y-plan:268), T1 (test-plan:307), S1 (security-plan:367), AR4+AR5 (architecture:193-194).
Each edit asserts its anchor occurs exactly once, writes LF-preserving bytes, and re-reads to confirm."""
import pathlib
A = pathlib.Path('D:/dev/projects/conductor/.andromeda')
DA = '«Да»'
Q = '«W и 153 — да?»'

def edit(name, old, new):
    p = A / name
    s = p.read_bytes().decode('utf-8')
    n = s.count(old)
    assert n == 1, (name, n, old[:60])
    s2 = s.replace(old, new)
    p.write_bytes(s2.encode('utf-8'))
    back = p.read_bytes().decode('utf-8')
    assert new in back and old not in back, name
    print(name, 'ok', len(s), '->', len(back))

# A1 + A2 — a11y-plan §3 Screen reader test pattern
a11y_old_start = 'The focus verdict is CONFIGURATION-BOUND: '
s = (A / 'a11y-plan.md').read_text(encoding='utf-8')
i = s.index(a11y_old_start)
end_tok = "and NVDA's object-model choice is ruled out);"
j = s.index(end_tok, i) + len(end_tok)
a11y_old = s[i:j]
a11y_new = (
    "The focus verdict is CONFIGURATION- and INPUT-PATH-BOUND: on WebView2 runtime/driver 154.0.4258.37 / 154.0.4258.37, "
    "Windows 26200.9457 and NVDA 2026.2 the agent arm — WebDriver-injected keys into the app launched under tauri-driver + "
    "msedgedriver — hears the webview's focus only in the window's first burst and then no later focus change, under BOTH "
    "of NVDA's Chromium object models (its default in-process IA2 buffer and `[UIA] allowInChromium = 2`), while live "
    "regions still speak, and those agent-arm focus rows grade `not-announced` there (as measured at "
    "`conductor-0.3.0/chunks/2026-09-30-the-screen-reader-pass-grades-again-on-this-host/evidence/cause-control.md`, "
    "2026-09-30); the same injected, driver-launched path is equally silent at the runtime/driver pair 153.0.4234.48 / "
    "153.0.4234.48 (as measured at `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/evidence/"
    "no-boundary-control.md`, arm R153-empty, 2026-09-30). An NVDA user on that 154 configuration who presses Tab through "
    "the OS keyboard path hears every focus change in Conductor — OS-level `SendInput` keys, which NVDA handles as "
    "physical keys, each logged as `Input: kb(desktop):tab` and followed by its focus utterance, into the bundle launched "
    "by path with no driver (arm S-conductor, same record) — and Edge 154 is heard under both input paths (arms S-edge "
    "and W-edge), so the silence is measured on the agent arm's injected, driver-launched path only, never as a keyboard "
    "user's experience; a physical keyboard is unmeasured. The cause is recorded, not established: NVDA's object-model "
    "choice and the runtime/driver pair are ruled out; injected keys versus launch under tauri-driver + msedgedriver stay "
    "confounded (arm S-conductor varied both at once); and the Windows cumulative updates and the desktop stay "
    "candidates against the 2026-09-07 heard run on 152.0.4191.66;"
)
assert a11y_old.count('\n') == 0 and len(a11y_old) < 1000, len(a11y_old)
edit('a11y-plan.md', a11y_old, a11y_new)

# T1 — test-plan §6 desktop-webview row: the measured PAIR set gains the 153 control pair
t_anchor = ' with the SET as what the sample evidences. **Failing:**'
t_new = (
    ' with the SET as what the sample evidences. ∪ {WebView2 Runtime 153.0.4234.48 × msedgedriver 153.0.4234.48} — '
    "the dev host's `sr-empty` leg on 2026-09-30, a one-off CONTROL: the runtime selected per command with the WebView2 "
    "loader's `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` over the on-disk Evergreen 153.0.4234.48 folder (no committed reader), "
    "the driver operator-supplied on `CONDUCTOR_MSEDGEDRIVER` and Authenticode-verified before it executed; the session "
    "was created (`Spec Files: 1 passed, 1 total`, `webview2_runtime` = `153.0.4234.48`) and its focus rows E0-02..E0-06 "
    "graded `not-announced` as on 154 (as measured at `conductor-0.3.0/chunks/2026-09-30-the-sr-cause-isolated-on-this-host/"
    "evidence/nvda-pass.153.json`) — ratified as a control only: founder, live, relayed by the overseer, to "
    + Q + ": " + DA + ". **Failing:**"
)
edit('test-plan.md', t_anchor, t_new)

# S1 — security-plan rule (b): the two ratified session-level crossings, count unchanged
s_anchor = 'so it adds no governed form and the count stays seven.'
s_new = (
    "so it adds no governed form and the count stays seven. Two founder-ratified session-level crossings on the dev "
    "host, 2026-09-30 (`2026-09-30-the-sr-cause-isolated-on-this-host`), sit outside the three loci too and add no "
    "governed form: arm W, a gitignored session script that validated `CONDUCTOR_MSEDGEDRIVER` the wdio way, started that "
    "driver with the fixed argv `--port=4445` on the registered dev-only port, drove ONE WebDriver session to the Edge "
    "browser and tore both down; and arm 153, the existing `sr-empty` leg run with an operator-supplied msedgedriver "
    "153.0.4234.48 on `CONDUCTOR_MSEDGEDRIVER`, admitted by a pre-execution `Get-AuthenticodeSignature` check (`Valid`, an "
    "`O=Microsoft Corporation` signer) plus a SHA-256 and version match BEFORE the binary executed at all — a binary is "
    "never executed before its signature verdict is read. Both are CONTROLS only, ratified by the founder live, relayed "
    "by the overseer, to " + Q + ": " + DA + "; no committed form moved, and the count stays seven."
)
edit('security-plan.md', s_anchor, s_new)

# AR4 + AR5 — architecture env rows: "only reader" narrowed to the only COMMITTED reader
edit('architecture.md',
     'Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts` (never by a shipped binary)',
     'Its only COMMITTED reader is `crates/conductor-tauri/ui/wdio.conf.ts` (never a shipped binary)')
edit('architecture.md',
     '`crates/conductor-tauri/ui/wdio.conf.ts`, the ONLY reader — never a shipped binary)',
     '`crates/conductor-tauri/ui/wdio.conf.ts`, the only COMMITTED reader — never a shipped binary)')
