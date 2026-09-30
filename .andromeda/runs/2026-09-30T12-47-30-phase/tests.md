# tests extract

## Relevance
partial — the chunk's test surface is the folded Rust-gate PREREQ (unit + lint legs) plus the operator-local `sr*` screen-reader leg family and its evidence; no new unit/integration tier work is expected (no `crates/*/src` delta), so most of the plan's Rust-tier strategy does not bind.

## Constraints
- The PREREQ's two entries are DISTINCT §9 stages, not one: per test-plan §3 `run` (CI stage selectors), `agent-run run --unit` maps to the `cargo nextest run --workspace --profile ci` leg ONLY, while clippy `-D warnings` is the separate Lint stage (per test-plan §9 stage table). Closing the deferral needs both run and read green; per test-plan §11 Quality ("NEVER skip quality gates just this once") and §10 Build failure conditions, a zero Rust delta is not a basis the plan recognises for skipping clippy/nextest. Whether the prior plan's `defer` key and entry forms still match these stage definitions is research's question.
- Read each gate's exit from the BARE command and its printed verdict, never through a pipe; nextest exit semantics are `NextestExitCode` 100/101/4 (per test-plan §2 Agent-runnable invariants; §3 `run` Exit code semantics).
- The `sr*` screen-reader suites are operator-local, real-wall-clock, host-NVDA legs — never CI stages, never an `agent-run` verb, never the deterministic tier (per test-plan §2 Agent-runnable invariants; §6 desktop-webview row, third suite family; §11 CI). Any control (1) arm inherits that footing.
- The SR leg's firing form is part of the leg, not optional: `CONDUCTOR_NVDA` guarded with skip-at-exit-0, NVDA started BEFORE tauri-driver and ready on its initialized log line plus a quiet settle, the leg-owned `nvda.ini` copied per session, OS-foreground activation, per-suite runs/scenarios dirs at the one spawn site, and the stop form (`nvda -q` → census → driver kill) with the pass record written from the speech log against the action timeline (per test-plan §6 desktop-webview row). Whether a control run over Edge 154 or another WebView2 app can reuse this form, or needs a new suite/spawn, is a P3/P4 question — the plan names no non-Conductor SR subject.
- A TS harness member (a new wdio suite, spec, or config branch for the control) is proven ONLY by executing a leg that loads it — never by `tsc`/`vite build` (per test-plan §4 conductor-tauri/ui bullet). A member reachable from no executing leg stays unproven.
- A new committed control page (if control (1) uses a repo fixture) joins the committed fixture family, which must carry its MEANING under test via an executing round-trip, not just exist (per test-plan §7 Self-bootstrapping requirement). Where such a page lives is open (scope defers it to P3/P4).
- Browse-mode rows (E0-09 class) stay recorded findings `not-run-here` — never a pass, never a manual arm — pending OS-level key injection (per test-plan §1 Untestable (by agent, today)).

## Patterns to follow
- The prior chunk's two-slot SR shape: each operator-slotted NVDA leg with its configuration recorded, graded from NVDA's own speech log with the operator's role as REVIEW (per test-plan §6 desktop-webview row, third family).
- Configuration-bound readings: state every measured webview/driver outcome as bound to its recorded runtime × driver × host configuration, never as unconditional (per test-plan §6 desktop-webview row, the working set "is the set of PAIRS" and the configuration-bound endpoint clause). Control (1)'s record should name runtime, driver, NVDA, OS build, desktop and `allowInChromium` in that spirit.
- Strict `--e2e` as the routine-arm regression guard, asserted on the printed verdict: zero failed on `Spec Files:`, skip tally within the expected-skip SET (two live-hold subjects), and the `[webview2 … windows]` session banner (per test-plan §3 `run` CI stage selectors, the 2026-09-07 printed-verdict clause).
- Refreshing the driver after a runtime update is the operator's host task BEFORE the next leg, never inside a wrap (per test-plan §6 desktop-webview row).

## Anti-patterns to avoid
- No rerun-until-announced: re-running an SR leg until a row grades `announced` is the retry-policy class the plan bans — a differing result is a defect or a condition to explain, never retried away (per test-plan §10 Zero-flakiness budget; §11 Quality "NEVER add retry-once policies").
- No `sleep(N)` synchronisation inside a test; waits bind to a signal (NVDA's readiness line, the bounded quiet settle) (per test-plan §11 E2E).
- No manual smoke / "human reviews" test step: an operator at the keyboard is a recorded arm with its reason, never an unmarked manual verification (per test-plan §11 Universal; §1 Untestable).

## Contract bindings
- tests ↔ security: any new harness spawn form for control (1) (e.g. the wdio/msedgedriver stack pointed at Edge) sits under security rule (b)'s seven-form count and would be an EIGHTH crossing; the plan's SR leg reuses the existing WebDriver pair `:4444`/`:4445` and the NVDA detached fixed-argv spawn only (per test-plan §2 Agent-runnable invariants; §11 Universal loopback clause).
- tests ↔ a11y: the SR leg family and its `nvda-pass` grading are the a11y plan's screen-reader test pattern executed through this plan's harness; the three owned regrade reds (focus-row count over `evidence/nvda-pass.json`, S0-09/E0-05, one-configuration flag) are graded against that record (per test-plan §6 desktop-webview row, third family).
- tests ↔ obs: `--e2e` emits its violation record at `runs/a11y/<run_id>.jsonl`, outside the harness `status` glob; the 5-command surface and envelope/JSONL shapes must stay untouched (per test-plan §3 `run` CI stage selectors).

## Acceptance criteria contributions
- (tests) `bash scripts/agent-run.sh run --unit` exits 0 from the bare command (nextest leg, no `NextestExitCode` 100/101/4) AND `cargo clippy --workspace --all-targets -- -D warnings` exits 0, both RUN in this chunk and recorded — not deferred by their key (per test-plan §9 stage table; §3 `run` CI stage selectors; §11 Quality).
- (tests) Strict `--e2e` on the dev host reads 0 failed with skips only within the expected-skip SET and the WebView2 session banner present, read from the printed verdict (per test-plan §3 `run` CI stage selectors).
- (tests) Every NVDA leg run in this chunk records its full configuration and is graded from the speech log; no row is graded a pass by re-running, and browse-class rows stay `not-run-here` findings (per test-plan §6 desktop-webview row; §10 Zero-flakiness budget; §1 Untestable).
- (tests) Any new TS harness member or committed control fixture added for control (1) is exercised by an executing leg in this chunk (not only built/typechecked) (per test-plan §4 conductor-tauri/ui bullet; §7 Self-bootstrapping requirement).
