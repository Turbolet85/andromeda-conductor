# Race witness — the routine a11y arm's readiness race (plan steps 8-9)

The subject: `@axe-core/webdriverio` 4.12.1 opens every `analyze()` with
`execute(() => document.readyState === 'complete')` raced against `FRAME_LOAD_TIMEOUT = 1000 ms`
(`node_modules/@axe-core/webdriverio/dist/index.mjs:15`, `:95-113`); `analyzePromise` → `axeSourceInject` →
`assertFrameReady` makes that probe the FIRST execute of every analysis (`:437`, `:80`), and any miss — a late
`true` included — throws `Page/Frame is not ready`. The fix waits in `axeFindings()` until the same probe answers
`true` in under `RESPONSIVE_MS = 250` ms, then analyzes once. The standing arm `axe completes over a 1.5 s
main-thread stall` makes the next read of `document.readyState` block the main thread for 1.5 s.

The witness has two halves, both required: with the wait BYPASSED (the stall arm calling the raw
`new AxeBuilder({ client: browser }).withTags(WCAG_TAGS).analyze()`) the arm fails with `Page/Frame is not ready`,
and with the wait RESTORED it passes.

Every run below: the Windows dev host, `CONDUCTOR_A11Y_STRICT=1 bash scripts/agent-run.sh run --e2e`.

## Run 1 — 2026-09-30 04:56Z, bypassed, timer-scheduled stall: NO READING (environmental)

Session creation failed before any spec: `session not created: This version of Microsoft Edge WebDriver only
supports Microsoft Edge version 152` — `Current browser version is 154.0.4258.37`. The driver then at
`CONDUCTOR_MSEDGEDRIVER` was msedgedriver 152.0.4191.53; the runtime (the `EdgeUpdate` client key's `pv`) read
154.0.4258.37. No `[webview2 <version> windows]` banner, no spec ran; it measures nothing about the race. The
operator then installed msedgedriver 154.0.4258.37 at the same handle (Authenticode `Valid`, signer
`CN=Microsoft Corporation`, re-read here before run 2).

## Run 2 — 06:11Z, bypassed, timer-scheduled stall (the plan's step-9 form): GREEN — the arm did not discriminate

Banner `[webview2 154.0.4258.37 windows]`, driver 154.0.4258.37: 13 passing, 2 skipped, the stall arm `✓` WITHOUT
the wait. The plan's form — `browser.execute(() => setTimeout(busy 1 500 ms, 0))`, then axe — does not reproduce
the race on this configuration, so as written the arm could pass vacuously.

## Run 3 — 06:12Z, bypassed, timer-scheduled stall + a timing diagnostic (source reverted after)

The stall arm recorded the stall on the page clock and timed one readiness-shaped probe sent right after the
scheduling call:

    [stall-diag] scheduled=1608.2 stallStart=1608.3 stallEnd=3108.3 probeAnsweredAt=3126.3 roundTripMs=17.7

The stall ran its full 1 500 ms, and the probe was answered after it, yet its round trip was 17.7 ms: the
scheduling `execute` itself did not return until the stall had ended — the `setTimeout(…, 0)` fired before that
call's response was delivered — so the scheduling call absorbs the whole stall and axe's probe meets an idle
thread. A longer timer delay cannot fix it deterministically: the probe would then arrive before the stall
starts. The plan's stated mechanism ("a classic-WebDriver `execute` is queued behind the page's synchronous
JavaScript") holds; its placement of the stall does not.

The arm was therefore changed to bind the stall to the probe's OWN evaluation: a one-shot `document.readyState`
getter on `document` that blocks the main thread for 1 500 ms on its first read, deletes itself, and returns the
real value. Without the wait, axe's probe is that first read; with it, the wait's own probe absorbs it.

## Run 4 — 06:14Z, bypassed, probe-bound stall: RED — the race reproduced

Banner `[webview2 154.0.4258.37 windows]`: 12 passing, **1 failing**, 2 skipped; `✖ axe completes over a 1.5 s
main-thread stall` with `Error: Page/Frame is not ready`; `Spec Files: 0 passed, 1 failed`, exit 1. Source
restored immediately after (the only `AxeBuilder` construction is back inside `axeFindings()`).

## Run 5 — plan entry 14 through the gate tool, restored: GREEN

Banner `[webview2 154.0.4258.37 windows]`: **13 passing**, 2 skipped, 0 failing; `✓ axe completes over a 1.5 s
main-thread stall`; `[a11y] verdict asserted — 0 failed · 2 skipped (expected 2) · driven session present`;
exit 0 (gate trail entry 14). The routine arm's pass tally moved 12 → 13 with the skip set unchanged at two, as
the plan predicted.

## Run 6 — restored, repeated: GREEN

Identical to run 5: 13 passing, 2 skipped, the verdict line asserted, exit 0.

## Census

Before run 1 and after runs 1, 5 and 6: six `msedgewebview2.exe` created 2026-09-26 (not these runs'), no
`msedgedriver`, `conductor-tauri`, `tauri-driver` or `node` survivor, `:4317`/`:4318` without a listener.
