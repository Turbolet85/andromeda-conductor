# security extract

## Relevance
partial — Tauri 2 GUI shell introduces new IPC boundary (Tauri commands + Channel) and frontend dependency chain; permission model and core-logic model untouched.

## Constraints
1. Tauri ≥2.10.3 required (security pin, origin-confusion CVE-2026-42184) per security-plan §Dependency Security §Update policy + §Anti-Patterns §Code Patterns.
2. Deny-by-default `capabilities/*.json` ACL with minimal allowlist — only frameless-window permissions; no blanket/wildcard entries (security-plan §Anti-Patterns §Code Patterns).
3. No `shell-open` plugin with scenario-config-derived strings without explicit allow-scope; if used, scoped only (security-plan §Anti-Patterns §Code Patterns, CVE-2025-31477 CVSS 9.3).
4. No remote-origin iframes in bundled webview (security-plan §Anti-Patterns §Code Patterns, GHSA-57fm-592m-34r7).
5. Frontend npm supply-chain gate: `npm audit` clean (0 vulns) + `package-lock.json` committed before merge (security-plan §Dependency Security §Frontend (npm) supply chain).
6. `ObsSink` extension for Tauri backend self-obs → `logs/conductor-tauri.jsonl`; redaction at processor stage; cli sink path unchanged (security-plan §Input Validation note re: obs env-handles).

## Patterns to follow
1. Tauri 2 frameless `decorations: false` + `data-tauri-drag-region` titlebar landing as `banner` landmark.
2. Bound-parameter SQL for `runs.db` writes (security-plan §Input Validation — not new here, established constraint maintained).
3. Tauri runtime under core ownership of the `current_thread` runtime (architecture §Async Runtime Flavor preserved).

## Anti-patterns to avoid
1. NEVER spawn MCP sidecar or anything via shell/eval with operator input; fixed program path + `.env(...)` only (security-plan §Anti-Patterns §Code Patterns; boundary-relevant for later Epoch-9 commands).
2. NEVER embed remote-origin iframes; NEVER expose Tauri commands without deny-by-default ACL (security-plan §Anti-Patterns §Code Patterns §Secrets).
3. NEVER add `shell-open` plugin unscoped or with scenario-derived strings (CVE-2025-31477).

## Contract bindings
- **frontend.md** ↔ **Tauri 2 frameless shell** (decorations, drag-region, design tokens)
- **a11y.md** ↔ **Frameless titlebar landmark + focus ring** (banner, semantic button controls, not-color-alone)
- **obs-plan.md §3** ↔ **Tauri backend self-obs sink** (`logs/conductor-tauri.jsonl`, `service.name` override, processor redaction)
- **design-system.md** ↔ **Tauri IPC capability boundary** (deny-by-default ACL established now, command permissions arrive later)

## Acceptance criteria contributions
1. (security) Tauri ≥2.10.3 pinned in `Cargo.toml`; `cargo audit` + `cargo deny check` pass; no new advisory/license violations.
2. (security) `capabilities/*.json` present with deny-by-default + minimal allowlist (only frameless-window permissions); no blanket/command entries.
3. (security) `npm audit` clean (0 vulnerabilities) + `package-lock.json` committed if `ui/` deps added/changed.
4. (security) Tauri backend self-obs redirects to `logs/conductor-tauri.jsonl`; cli sink path (`logs/agent-latest.jsonl`) unchanged + passing tests.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** — cargo-audit/cargo-deny version floors confirmed; toolchain 1.94.1+ done; tauri ≥2.10.3 forward-pinned (dormant until Tauri GUI). Audit gate when chunk lands.
- **2026-06-15-design-token-typography-bundle** — npm supply-chain gate added: `npm audit` clean + committed `package-lock.json` + vendored fonts (no runtime CDN). Vite 8, Tailwind 4.1, React 19 on this chunk.