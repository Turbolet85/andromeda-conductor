# security extract

## Relevance
Partial — UI-only chunk (Titlebar animations + local run-state); most security domains out-of-scope; one constraint applies (npm supply-chain gate).

## Constraints
1. `npm audit` must pass clean (0 vulnerabilities) before merging any `ui/` dependency changes (security-plan §Dependency Security, Frontend (npm) supply chain).
2. `package-lock.json` MUST be committed alongside any npm dependency update (security-plan §Dependency Security, Frontend (npm) supply chain — mirrors `Cargo.lock` discipline).
3. No animation library beyond CSS `@keyframes`/transitions — specifically no framer-motion (security-plan §Security Anti-Patterns § Code Patterns + frontend.md hard ban).
4. No new `#[tauri::command]`, no `Channel`, no capability ACL change — `capabilities/default.json` stays unchanged (scope.md boundary statement; security-plan §Code Patterns deny-by-default).

## Patterns to follow
1. Color/motion values bound by design-token NAME (`var(--count-hold)` etc.), never raw hex/px/ms (scope.md surfaces; design-system.md §Signature).
2. A11y non-color signalling: text flip + `aria-live="assertive"` on HOLD state; reduced-motion already global (tokens.css L78–83) (scope.md A11y signalling + a11y-plan.md §Motion).

## Anti-patterns to avoid
1. NEVER add Tauri `shell-open` plugin with scenario-config-derived strings without explicit allow-scope (security-plan §Anti-Patterns § Code Patterns).
2. NEVER ship Tauri commands without deny-by-default capabilities (security-plan §Anti-Patterns § Code Patterns); this chunk adds zero commands, so no capability change.

## Contract bindings
Design ↔ UI (color token semantics: `--count-nominal` / `--count-hold` / `--count-blocked` per design-system.md §Color); A11y ↔ UI (reduced-motion + aria-live per a11y-plan.md §Motion / §Status messages); Frontend.md ↔ UI (CSS-only motion ban, token naming); npm supply-chain ↔ Dependency Security (npm audit + committed lock).

## Acceptance criteria contributions
- `npm audit` clean (0 vulnerabilities) — no deps added in this chunk per scope, so inherited clean gate remains green (security-plan §Dependency Security).
- `package-lock.json` committed if any npm change made (expected: none) (security-plan §Dependency Security).
- TypeScript strict mode: `tsc` + `vite build` → `ui/dist` succeed with no `any` / `as` (scope.md gates).
- No framer-motion or animation runtime library in `ui/` (security-plan §Code Patterns; grep `package.json` → fail if present).

## Relevant amendment history
1. **2026-06-15-design-token-typography-bundle** — §Dependency Security added Frontend (npm) supply-chain control: `npm audit` clean (0 vulns) + committed `package-lock.json` + vendored fonts (no runtime CDN). Reason: npm ecosystem (React 19 / Vite 8 / Tailwind 4.1) required explicit supply-chain gate. Cascaded to security-summary.md / rules/security.md.
