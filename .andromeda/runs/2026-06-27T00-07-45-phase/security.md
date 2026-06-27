# security extract

## Relevance
partial — UI component with optional Tauri IPC extension; npm supply-chain constraints apply unconditionally; Tauri IPC constraints apply only if live-data command option chosen

## Constraints
1. `npm audit` must pass with 0 vulnerabilities; `package-lock.json` committed for reproducible install (security plan §Dependency Security / Frontend (npm) supply-chain, amendment 2026-06-15-design-token-typography-bundle)
2. If live-data command option chosen: declare the read-only command in deny-by-default capabilities config (tauri.conf.json or equivalent) with ONLY its name listed among existing allowed commands (start/stop, scenario/suite picker, run-report view, operator-pause, live-counter Channel); no additional scope grants (security plan §Security Anti-Patterns / Code Patterns — "NEVER ship Tauri commands without a minimal capabilities file")
3. `Cargo.lock` must show `tauri` ≥ 2.10.3 if live-data command option chosen (clears origin-confusion CVE-2026-42184 on Windows; constraint already in plan, dormant until Epoch 9 activation) (security plan §Dependency Security / Update policy, §Security Anti-Patterns / Code Patterns)
4. NEVER use Tauri `shell-open` plugin with scenario-config-derived or operator-supplied path/URL strings; require explicit allow-scope if path/URL opening added (security plan §Security Anti-Patterns / Code Patterns, CVE-2025-31477)
5. NEVER embed remote-origin iframes in the bundled webview; iframes bypass origin checks for IPC and combined with CVE-2026-42184 re-expose the in-process IPC boundary (security plan §Security Anti-Patterns / Code Patterns, GHSA-57fm-592m-34r7)

## Patterns to follow
1. Bundled webview (no dev-server port, no remote origins) enforces the in-process IPC boundary isolation assumed by the threat model (security plan §Threat Model Summary / Attack surface — Tauri IPC vector)
2. Design-token-sourced styling + TypeScript strict mode (no `any`, functional components + hooks); inherited from prior Epoch-9 components established in 2026-06-15-design-token-typography-bundle
3. If command added: reads self-generated synthetic runs.db data (trusted-child class, like MCP read-back), not untrusted external input; minimal validation burden beyond command declaration in capabilities

## Anti-patterns to avoid
1. Do NOT ship Tauri commands with permissive or absent capabilities declarations; deny-by-default explicit listing is mandatory (security plan §Security Anti-Patterns / Code Patterns)
2. Do NOT merge with `npm audit` showing vulnerabilities or `package-lock.json` uncommitted
3. Do NOT use npm packages from untrusted sources; `npm audit clean` is the gate

## Contract bindings
- **npm supply-chain** ↔ dependency-security gates (clean audit + committed lock per §Dependency Security / Frontend (npm) supply-chain)
- **Tauri IPC boundary (if live-data command)** ↔ tests-harness (tauri-driver GUI integration tests deferred to ch9, but command structure must be shaped for later testability per §Threat Model Summary / Attack surface — Tauri IPC vector)

## Acceptance criteria contributions
- (security) `npm audit` clean (0 vulnerabilities) on `crates/conductor-tauri/ui/` before merge
- (security) `package-lock.json` committed and un-drifted
- (security) If live-data command option chosen: `tauri` ≥ 2.10.3 in `Cargo.lock`
- (security) If live-data command option chosen: command declared in deny-by-default capabilities (tauri.conf.json) with no additional scope grants

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle:** npm supply-chain gate established — `npm audit` clean (0 vulns) + `package-lock.json` committed + vendored fonts (no runtime CDN) — for conductor-tauri/ui/ frontend deps (React 19, Vite, Tailwind v4.1, Fontsource, TypeScript). Applies because this chunk authors UI code into that npm-gated subtree.
- **2026-06-15-dependency-audit-gate (Note only):** flags that `tauri` ≥ 2.10.3 (clears CVE-2026-42184) is a required forward bump left dormant in `[workspace.dependencies]` until Epoch 9 (this chunk) activates it if the live-data command option is chosen; audit remit for Tauri versions belongs to this chunk's scope.