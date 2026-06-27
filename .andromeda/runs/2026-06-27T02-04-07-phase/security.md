# security extract

## Relevance
Relevant — chunk adds a new Tauri IPC command (`operator-pause`) + resolver implementation, crossing the Tauri IPC trust boundary explicitly named in the threat model.

## Constraints
1. New `operator-pause` `#[tauri::command]` must be added to deny-by-default capabilities file listing ONLY this command + existing shipped commands (start/stop/picker/run-report) + live-counter `Channel`; no new Tauri features (fs/shell-open/http) introduced (security-plan.md §Anti-Patterns § Code Patterns). [NOTE: cross-check vs 2026-06-26 rule — app commands are NOT ACL-gated; only core/plugin permissions are listed.]
2. Command error response must NOT expose internal struct names, absolute paths, or seam-crate implementation details — sanitize at the Tauri boundary when returning `Result` to webview (security-plan.md §Error Handling).
3. `TauriResolver` implementation must not spawn subprocesses, read env vars, or introduce filesystem access — bridges background run thread ↔ webview via `tokio::sync::oneshot` only (security-plan.md §Input Validation § Attack surface: Tauri IPC is local in-process; §Threat Model Summary: "not network-exposed").
4. Hold signal serialization over `Channel` (new `RunStage` or dedicated event) must remain Tauri-serde-json-compatible — no raw pointers, unserializable types, or unbounded recursion (security-plan.md §Data Protection, Minimal tier).
5. npm dependencies added to `crates/conductor-tauri/ui/` for dialog wiring must pass `npm audit` clean (0 vulnerabilities) + committed `package-lock.json` before merge (security-plan.md §Dependency Security, per 2026-06-15-design-token-typography-bundle amendment).

## Patterns to follow
1. Reuse shipped `HoldPoint`/`Decision`/`HoldResolution` types from `conductor_core::pause` without rebuild — type safety at compile time (security-plan.md §Input Validation: config-derived types validated at load, not in GUI).
2. Wire only `open`/`onOpenChange`/`onProceed`/`onAbort`/`allowNoGo` props on the carried `OperatorPauseDialog` scaffold from component-primitives-library; focus-trap/Escape/focus-restore already shipped (§Input Validation — existing controls reused).
3. Use `tokio::sync::oneshot` for resolver bridge — background run awaits operator decision without blocking GUI thread (fits Minimal-tier loopback-only sync pattern, no network exposure).

## Anti-patterns to avoid
1. NEVER ship unscoped Tauri `shell-open` plugin or allow-all capabilities — maintain deny-by-default with explicit command + Channel list only (CVE-2025-31477 / origin-confusion CVE-2026-42184) (security-plan.md §Anti-Patterns § Code Patterns).
2. NEVER embed remote-origin iframes in the bundled webview — dialog is local-only and must remain so (iframes bypass origin checks even in isolation GHSA-57fm-592m-34r7) (security-plan.md §Anti-Patterns § Code Patterns).
3. NEVER spawn subprocesses or read operator-supplied input in the resolver — it is a pure async signal bridge, not a command/env/path boundary (security-plan.md §Anti-Patterns § Code Patterns, Input anti-pattern: subprocess hardening applies only to the Pulse MCP sidecar, not GUI internals).

## Contract bindings
- **design** (ch9 Desktop a11y harness) — GUI integration + axe/keyboard tests deferred per scope; focus-trap/Escape/restore inherited from shipped scaffold + a11y.md.
- **tests** (no new E2E gate stated) — E2E pause-dialog testing deferred to live-Pulse chunk (Epoch 10).
- **(conditional) obs** — if resolver emits logs on hold resolution, apply obs-plan §Logging redaction rules (non-path event metadata only, never absolute paths).

## Acceptance criteria contributions
1. (security) Tauri capabilities file updated ONLY if a new core/plugin permission is needed (per 2026-06-26 rule, app commands aren't ACL-gated); `grep` verifies no allow-all/wildcard, deny-by-default holds (security-plan.md §Anti-Patterns § Code Patterns).
2. (security) `npm audit` clean (0 vulnerabilities) on any new dependencies under `crates/conductor-tauri/ui/` before merge (security-plan.md §Dependency Security, per 2026-06-15-design-token-typography-bundle).
3. (security) `cargo-audit` + `cargo-deny` advisories/bans green on workspace (no new Rust dependencies; if any, must pass gate per security-plan.md §Dependency Security).
4. (security) Manual review: command error return does not leak internal seam-crate struct names or absolute filesystem paths to webview (security-plan.md §Error Handling).

## Relevant amendment history
**2026-06-15-design-token-typography-bundle** — npm (frontend) supply-chain gate added
- **Section:** §Dependency Security (new Frontend (npm) supply chain paragraph)
- **Change:** `npm audit` clean (0 vulnerabilities) + committed `package-lock.json` + vendored fonts (no runtime CDN) required for `crates/conductor-tauri/ui/` ecosystem (React 19 / Vite 8 / Tailwind 4.1 / Fontsource / TypeScript).
- **Why:** GUI wiring added in this chunk uses npm dependency tree not covered by `cargo-audit`/`cargo-deny`; npm-audit-clean gate enforces same floor discipline as Rust (per security-plan.md § Dependency Security update policy: "npm advisories drive the same floor discipline as cargo").
