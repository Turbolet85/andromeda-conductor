# security extract

## Relevance
Partial — this UI chunk adds a new Tauri IPC command surface (read-only) over existing types. Security constraints apply narrowly to command exposure (capabilities hardening), error sanitization at the IPC edge, and npm audit compliance for the UI tree.

## Constraints
1. All new `#[tauri::command]` must be registered in the deny-by-default capabilities file, permitting ONLY the actual commands + the live-counter Channel (per security-plan §Security Anti-Patterns, Code Patterns; prevents unintended IPC exposure).
2. The `RunRecord` returned from the read-only query command must serialize only verdict/state/identity fields, never absolute paths or canonicalized `CONDUCTOR_*` directory names (per security-plan §Error Handling, Run-report artifact sanitization).
3. StatusLamp rendering must never be color-alone — every verdict/state carries text label + glyph per the contract (scope requirement + security-plan §Anti-Patterns, Logging).
4. The `npm audit` gate must pass with 0 vulnerabilities for the `crates/conductor-tauri/ui/` npm tree before merging, with `package-lock.json` committed (per security-plan §Dependency Security, Frontend (npm) supply chain, amended 2026-06-15).
5. If the chunk uses any Tauri plugin (shell-open, file picker), it MUST be explicitly scoped in the capabilities/allow-list (per security-plan §Anti-Patterns, Code Patterns; CVE-2025-31477).
6. Any read-only Tauri command must validate the input request (e.g., run_id parameter existence) before returning data (per security-plan §Anti-Patterns, Input).

## Patterns to follow
1. Data-sourcing: reuse `lampForRecord` projection and the read-only-`#[tauri::command]`-returns-existing-type pattern from coverage-matrix-view (ch6) — `#[derive(Serialize)]` on the core type, `invoke<T[]>` in TS (scope carry).
2. Reuse existing `StatusLamp` + `OperatorChecklist` primitives from component-primitives-library (ch6); do not author new lamp/checklist components (scope carry).
3. Tauri capabilities: declare all commands in a minimal allow-scope (start/stop/picker/run-report/operator-pause + live-counter Channel per security-plan §Anti-Patterns, Code Patterns); deny everything else by default.

## Anti-patterns to avoid
1. NEVER use color-alone to render verdict/state status — always pair color with text label + glyph via StatusLamp (security-plan §Anti-Patterns, Logging).
2. NEVER expose absolute paths or internal struct names in StatusLamp/operator-checklist rendering or in `#[tauri::command]` error returns (security-plan §Anti-Patterns, Logging).
3. NEVER use an unscoped Tauri plugin (shell-open, file picker) — explicitly scope any plugin to the actual file/URL types needed (security-plan §Anti-Patterns, Code Patterns).

## Contract bindings
- **Tauri IPC surface** ↔ security-plan §Threat Model Summary (Attack surface: Tauri IPC vector, bundled webview, in-process boundary, trust verified by capabilities file).
- **npm dependency tree** ↔ security-plan §Dependency Security (Frontend (npm) supply chain: npm audit clean + committed package-lock.json gate mirrors cargo-audit + Cargo.lock).
- **Error handling at Tauri edge** ↔ security-plan §Error Handling (External responses sanitized; run-report artifacts must not leak paths).

## Acceptance criteria contributions
1. (security) All new `#[tauri::command]` registered in capabilities file with DENY-by-default, allowing only the actual commands + live-counter Channel — grep verifies no extra commands exposed.
2. (security) `npm audit` clean (0 vulnerabilities) for `crates/conductor-tauri/ui/` tree; `package-lock.json` committed before merge.
3. (security) Run-report view does not render absolute paths, `CONDUCTOR_*` directory names, or internal struct names — manual inspection confirms serialization excludes these fields.
4. (security) StatusLamp components always pair color + text label + glyph for every verdict/state (never color-alone) — grep verifies no bare-color renders without label.

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — npm supply-chain gate added to §Dependency Security. The `crates/conductor-tauri/ui/` npm tree requires `npm audit clean (0 vulns)` + committed `package-lock.json` as the enforcement gate equivalent to cargo-audit/cargo-deny; npm advisories drive the same floor discipline. (Impact: this chunk touches the npm tree, so the npm-audit-clean gate applies.)
