# security extract

## Relevance
partial — UI-only presentational React primitives; security coverage limited to frontend dependency supply chain and Tauri IPC design guardrails.

## Constraints
1. All new npm dependencies MUST pass `npm audit` clean (0 vulnerabilities); `package-lock.json` committed (security plan §Dependency Security §Frontend supply chain; amendment 2026-06-15-design-token-typography-bundle).
2. No vendored fonts or assets from runtime CDNs — all fonts downloaded and committed locally (security plan §Dependency Security §Frontend supply chain).
3. NEVER use Tauri `shell-open` plugin with scenario-config-derived strings or paths; unscoped plugin enables RCE via dangerous protocols (CVE-2025-31477) (security plan §Anti-Patterns §Code Patterns).
4. NEVER embed remote-origin iframes in bundled Tauri webview — bypasses origin checks even in isolation mode (GHSA-57fm-592m-34r7; security plan §Anti-Patterns §Code Patterns).
5. Lamp component state set MUST stay byte-consistent with report/CLI lamp mapping (`ReportState`/`Verdict` enum, verdict-first precedence) — shared vocabulary, no re-definition (security plan §Threat Model Summary §Attack surface §Tauri IPC).

## Patterns to follow
1. Design-token-driven styling via committed `crates/conductor-tauri/ui/src/styles/tokens.css` `:root` tokens (per existing Titlebar/RunControls/ScenarioPicker convention).
2. Accessibility baked in at component level: never-color-alone invariant (color + glyph + text label per state).
3. Tauri 2 webview→backend IPC boundary treated as in-process, not network-exposed (security plan §Threat Model Summary §Attack surface §Tauri IPC).

## Anti-patterns to avoid
1. NEVER add unscoped Tauri plugins (`shell-open`, etc.) that can be driven by component data.
2. NEVER spawn subprocesses or shell commands from UI component event handlers (security plan §Anti-Patterns §Code Patterns).
3. NEVER hardcode credentials, secrets, or absolute paths in component props or render logic (security plan §Anti-Patterns §Logging, §Secrets).

## Contract bindings
- Frontend npm supply-chain auditing binds to Dependency Security §Frontend supply chain (npm audit green + committed lockfile required for PR merge).
- Lamp-state naming binds to report/CLI vocabulary (ReportState/Verdict enums).
- Future Tauri capabilities file (deferred ch8+ch9) binds to this component set — final capabilities file must enumerate only actual commands with deny-by-default scope.

## Acceptance criteria contributions
- (security) `npm audit` clean (0 vulnerabilities) on all new npm dependencies this chunk adds (Radix UI, etc.).
- (security) `package-lock.json` committed — enables reproducible `npm audit` re-run.
- (security) Lamp component state names verifiable as matching report/CLI enum (byte-consistency check).
- (security) No Tauri `shell-open` / unscoped plugin calls in component code or its tests.

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — npm (frontend) supply-chain gate added to §Dependency Security (`npm audit` clean + committed `package-lock.json` + vendored fonts). Directly applicable — this chunk adds Radix UI / Lucide dependencies into the same tree.