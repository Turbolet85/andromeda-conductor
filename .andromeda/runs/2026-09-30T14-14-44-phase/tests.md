# tests extract

## Relevance
relevant — the chunk changes (or re-binds) the operator-local `sr*` screen-reader leg that test-plan §6 specifies as the third suite family over the one WebdriverIO + tauri-driver stack, and it tests the §1 "untestable (by agent, today)" browse-mode zone's stated remedy (OS-level key injection).

## Constraints
- The `sr*` leg is operator-local and runs on real wall clock: Windows + NVDA host only, never a CI stage, never an `agent-run` verb, never the deterministic tier. Any OS-input change keeps it outside the §9 CI jobs and outside the 5-command surface (per test-plan §6 Drivers per surface, desktop-webview row "The third family…"; §2 Agent-runnable invariants "Deterministic"; §11 CI).
- NVDA's own speech log (`-l 12`) is the driver. The pass record is written from the session's speech log against its action timeline, and the operator's role is REVIEW of the graded record. A regrade on the OS input path must keep this grading basis. Whether the leg's action timeline can already record OS-injected keys, as opposed to WebDriver-injected ones, is research's question (per test-plan §6 Drivers per surface, desktop-webview row).
- Browse-mode rows are recorded findings until OS-level key injection makes them agent-driven. They are never passes and never a manual arm. If step 1 validates `SendInput`, the §1 zone entry names that injection as the route-owned remedy. Driving any browse row on the OS path is still unmeasured, so treat it as a hypothesis (per test-plan §1 Untestable zones, "screen-reader BROWSE-MODE reading").
- The measured working set is a set of runtime × driver PAIRS, recorded with dated evidence paths. The 153 control joined it only as a ratified control. Each regrade verdict must name the configuration it ran under, and a new configuration enters the record the same way: pair, evidence path, control status stated (per test-plan §6 Drivers per surface, desktop-webview row "Passing:" set).
- Harness TS members (`wdio.conf.ts`, the screen-reader spec, any new OS-input helper) are proven only by EXECUTING a leg that loads them. The build gate (`tsc --noEmit` + `vite build`) does not prove ESM loadability, and a member no executing leg reaches stays unproven. No JS/TS unit runner is adopted (per test-plan §4 conductor-tauri/ui entry; §12 via §4).
- Zero-flakiness: no retry policy, and no re-running until a result appears. A leg that grades differently over an unchanged tree is a determinism or precondition finding to explain, never one to retry away. This matches the scope's ban on rerun-until-announced (per test-plan §10 Zero-flakiness budget; §11 Quality).
- Every webview arm, `sr*` included, may open only the harness-lifetime loopback WebDriver pair `:4444`/`:4445`. No other network, and no Conductor-owned listener (per test-plan §11 Universal; §2 Agent-runnable invariants).

## Patterns to follow
- The `sr*` firing and stop forms: a guarded `CONDUCTOR_NVDA` (skip at exit 0 when unset, not a file, or holding a metacharacter), NVDA started before tauri-driver and ready on `NVDA initialized` plus a quiet settle, `activate-window.ps1` with a fixed argv for OS foreground, per-suite runs/scenarios dirs chosen at the one spawn site, then `nvda -q` → `tasklist` census → `tauriDriver.kill()` (per test-plan §6 Drivers per surface, desktop-webview row).
- `wdio run` stays non-interactive and exit-code-readable, headful on the Windows WebView2 host (per test-plan §2 Agent-runnable invariants; §11 Universal).
- Strict routine `--e2e` asserts the PRINTED verdict, not the exit code: zero failed on `Spec Files:`, skips confined to the expected-skip SET (the two live-hold subjects), and the `[webview2 … windows]` banner present. Use it as the regression check whenever `wdio.conf.ts` changes (per test-plan §3 `run` CI stage selectors, `--e2e`).
- A harness-owned env handle gets a per-reader coverage class with its mandated test named. `CONDUCTOR_NVDA` / `CONDUCTOR_MSEDGEDRIVER` are covered by their skip arms at the e2e tier. A reader this chunk COMMITS (for example `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`) states its class and coverage the same way (per test-plan §1 Coverage triggers, Vector 1 bullet).
- Waits between OS-injected keys and the speech-log read use a bounded signal. The leg's quiet settle is bounded at 8 s and the utterance-to-stamp window is 50 ms. No bare sleep synchronises inside a test (per test-plan §2 Agent-runnable invariants; §11 E2E `sleep(N)` ban).

## Anti-patterns to avoid
- Never simulate the physical-keyboard arm (K) with synthetic keys. Never make an operator row a manual smoke step or a graded pass. A row the agent arm cannot reach is a recorded finding with its reason (per test-plan §11 Universal "manual smoke step"; §1 Untestable zones).
- Never use `sleep(N)` for synchronisation inside the leg, and never use xpath or hashed-class selectors. Use role/text/`aria-live` selectors only (per test-plan §11 E2E).
- Never re-run to get "announced", and never add a retry of any kind (per test-plan §10 Zero-flakiness budget; §11 Quality / CI).

## Contract bindings
- tests §6 `sr*` leg ↔ a11y-plan §3 (the SR pass spec and its focus verdict, now bound to the input path). Row classes (`focus` / `live` / `browse`) and grading vocabulary are a11y's. Harness mechanics and the evidence basis are tests'.
- tests §6 `sr*` firing form (`activate-window.ps1` fixed argv; a committed `SendInput` path) ↔ security rule (b)'s seven governed spawn forms. Whether committing OS-level input into the leg is a new form is security's P4 escalation, not a tests decision.
- tests §3 `--e2e` (strict) ↔ §9 CI `a11y` job. A `wdio.conf.ts` change reaches the CI gate, and that CI run follows the commit, so this chunk's wrap cannot prove it.
- tests §1 Coverage triggers (per-reader handle classes) ↔ arch §Occupied Resources registration of any reader or binder this chunk commits.

## Acceptance criteria contributions
- Strict routine `--e2e` (`CONDUCTOR_A11Y_STRICT` set) passes on its printed verdict: zero failed, skips confined to the expected-skip SET, the `[webview2 … windows]` banner present. This holds after any `wdio.conf.ts` / harness change (per test-plan §3 `run`, `--e2e`).
- Every changed or added harness TS member is loaded by at least one EXECUTED leg in this chunk (the relevant `npm run a11y:sr*` run in an operator-granted NVDA slot, or `--e2e`). The build gate alone does not count (per test-plan §4 conductor-tauri/ui entry).
- `run --unit` (`cargo nextest run --workspace --profile ci`) and `cargo clippy --workspace --all-targets -- -D warnings` stay green with no nextest `retries` configured (per test-plan §3 `run`; §10 Zero-flakiness budget).
- Each regraded row's record is graded from the session's NVDA speech log against its action timeline, names runtime × driver × NVDA × OS build × input path, and sits beside the prior chunk's record, not over it. A browse row the OS path cannot reach stays a finding with its reason, never a pass (per test-plan §6 Drivers per surface, desktop-webview row; §1 Untestable zones).
