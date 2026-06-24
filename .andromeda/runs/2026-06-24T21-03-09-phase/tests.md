# tests extract

## Relevance
Partial — the chunk adds a GUI shell (conductor-tauri) alongside the existing CLI release gate; test coverage is build-gated (no Rust/nextest unit tests) with later Epoch-9 E2E harness to follow.

## Constraints
- Per test-plan §1 Scope Summary: conductor-tauri is "partially-testable," with the webview leg secondary to the CLI release gate; the GUI is "convenience only."
- Per test-plan §4 Unit Test Strategy: conductor-tauri/ui tests are build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke), NOT Rust/nextest unit tests; frontend webview E2E via tauri-driver is explicitly deferred to Epoch 9 (amendment 2026-06-15-design-token-typography-bundle).
- Per test-plan §2 Test Strategy: no human-in-loop verification; tauri-driver runs `wdio run` headless only.
- Per test-plan §1 Test Scope Summary (Surfaces): desktop-webview uses tauri-driver headless + frameless window carries no responsive breakpoints; valid selector pairing via the Color-Only a11y rule (text/role mandatory).
- Per test-plan §3 Test Harness Contract: the 5-command discipline remains unchanged; Tauri is a secondary surface to the CLI release gate (source of truth).

## Patterns to follow
- Per test-plan §4: build-gate the `ui/` frontend via `tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke (no JS/TS unit runner adopted).
- Per test-plan §1 (Surfaces): desktop-webview E2E via tauri-driver with text-paired status labels (Color-Only a11y rule); `aria-live` HOLD/verdict announcements; role selectors on the frameless titlebar.
- Per test-plan §3: Tauri command/Channel IPC is agent-drivable headless; JSON schema assertion on typed tool responses (later chunks); status-lamp + text-label pairing guarantees reliable selectors.
- Per obs-plan §3 (logs): Tauri backend self-obs routes to `logs/conductor-tauri.jsonl` via the extended `ObsSink` (no new redaction policy); frontend logs `console.log` JSON only (not in scope to wire here).

## Anti-patterns to avoid
- Per test-plan §11 (Anti-Patterns / Unit): do NOT leave unit tests in the `ui/` tree — build-gating (tsc/vite/npm) is the acceptance boundary; no `cargo test` coverage expected for the frontend in Epoch 9 ch1.
- Per scope.md: do NOT pre-add later-chunk command permissions to `capabilities/*.json` — only frameless-window ACL rights are in scope.
- Per test-plan §1 (Surfaces): do NOT adopt UI automation of Pulse — the desktop-webview surface drives conductor's IPC, never Pulse's state directly.
- Per scope.md: do NOT wire the frontend obs sink to a browser OTLP exporter or attempt Tauri-to-Pulse OTLP egress — the backend file sink only.

## Contract bindings
- obs ↔ tests: Tauri backend self-obs sink `logs/conductor-tauri.jsonl` defined at obs-plan §3; redaction reuses processor-stage discipline (no new allowlist).
- frontend.md ↔ tests: frameless window `decorations:false` + `data-tauri-drag-region` titlebar surface; tauri-driver E2E harness binding deferred to Epoch 9.
- security-plan ↔ tests: `tauri ≥2.10.3` (CVE-2026-42184); deny-by-default `capabilities/*.json` minimal allowlist; `npm audit` clean on `package-lock.json` (if `@tauri-apps/api` added).
- architecture ↔ tests: core-owned `current_thread` runtime under Tauri (no Tauri-managed async runtime shift); Tauri `Channel` live-update surface binding deferred.

## Acceptance criteria contributions
- (tests) `cargo build -p conductor-tauri` succeeds with `tauri ≥2.10.3` + `tauri-build` + `build.rs` + `tauri.conf.json` (main window `decorations:false`).
- (tests) `ui/` build gates pass: `tsc --noEmit` + `vite build` succeeds + `npm audit` clean + `vite preview` renders frameless titlebar.
- (tests) Deny-by-default `capabilities/*.json` exists with minimal allowlist (only frameless-window drag/minimize/close); no blanket permissions; no later-chunk command ACLs pre-added.
- (tests) Tauri backend self-obs routes to `logs/conductor-tauri.jsonl`; `conductor-cli` unit tests (nextest) still pass unchanged.
- (tests) `Cargo.lock` + `package-lock.json` (if deps change) committed un-drifted; `cargo audit` + `cargo deny check` green (any new Tauri advisory/license justified).

## Relevant amendment history
- Amendment 2026-06-15-design-token-typography-bundle: conductor-tauri/ui tests are build-gated (no Rust/nextest coverage); frontend E2E harness + tauri-driver deferred to Epoch 9 closing chunks (not blocking this architecture-scaffold chunk).