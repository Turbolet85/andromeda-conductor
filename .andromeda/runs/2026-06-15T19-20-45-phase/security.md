# security extract

## Relevance
Partial — new npm/Fontsource dependency surface + offline hardening; CSS/token layer has no input validation, auth, or data handling.

## Constraints
1. All new npm dependencies (Tailwind v4.1, `@tailwindcss/vite`, `@fontsource/*`) MUST pass `npm audit` or equivalent lockfile audit before merge (security plan §Dependency Security; npm supply-chain control).
2. `package-lock.json` (or `yarn.lock` / `pnpm-lock.yaml`) MUST be committed to ensure deterministic npm installs and audit reproducibility (security plan §Dependency Security, Pinning subsection).
3. Tailwind build output (compiled stylesheet) MUST NOT expose absolute file paths, workspace paths (`CONDUCTOR_*` canonicalized directories), or internal project structure in CSS comments or error output (security plan §Error Handling, run-report artifact sanitization — forward-guardrail for build artifacts).
4. Fontsource WOFF2 fonts MUST be vendored locally; no runtime CDN fetches or external font service calls (security plan §Threat Model Summary, Minimal-tier offline hardening — "no runtime CDN").
5. No unvetted Tauri plugins (e.g., `shell-open`) may be introduced in this chunk or delegated to Epoch 9 without explicit allow-scope (security plan §Security Anti-Patterns §Code Patterns, CVE guard).

## Patterns to follow
1. Vendored font delivery via `@fontsource/` under `node_modules` — matching the local-only deployment model and avoiding runtime CDN fetches.
2. Static Tailwind v4.1 `@theme` CSS generation via `@tailwindcss/vite` (Oxide engine, zero-runtime) — no dynamic token injection or build-time eval.
3. New npm dependency additions trigger an `npm audit` check as part of the build or pre-commit, consistent with the Minimal-tier supply-chain audit discipline.

## Anti-patterns to avoid
1. NEVER use Google Fonts, CDN-hosted web fonts, or any runtime font fetch — fonts must be bundled as static WOFF2 (security plan §Threat Model Summary, offline hardening).
2. NEVER interpolate user-supplied or build-directory paths into the compiled stylesheet — keep CSS output free of absolute paths or workspace references (security plan §Security Anti-Patterns §Logging; forward-guardrail for artifacts shared across hosts).
3. NEVER introduce npm dependencies without verifying they pass `npm audit`/`cargo-deny`-equivalent lockfile check (security plan §Dependency Security, supply-chain control).

## Contract bindings
Design ↔ Security: Token names MUST match `design-system.md` §Color Palette / §Typography exactly (no renaming); Tauri scope decisions (deferred to Epoch 9) MUST be gated by `deny-by-default` capabilities file (security plan §Security Anti-Patterns §Code Patterns). · Frontend a11y binding (deferred): `@media (prefers-reduced-motion: reduce)` + `@media (prefers-color-scheme: light)` are in scope now (per scope.md acceptance intent); full a11y verification harness is Epoch 9.

## Acceptance criteria contributions
- (security) `npm audit` (or equivalent lockfile audit) passes for all new dependencies; `package-lock.json` is committed.
- (security) Compiled Tailwind stylesheet contains no absolute paths, workspace paths, or internal project structure in CSS or comments.
- (security) Fontsource WOFF2 fonts are verified to be vendored locally; no CDN/external font-service URLs appear in the CSS or HTML.
- (security) Tauri GUI scope (window, frameless config, deny-by-default capabilities) is documented as Epoch 9 deferred decision; no unvetted plugins introduced prematurely.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** — cargo-audit/cargo-deny versions reframed as minimum floors (0.22.1 / 0.19.4 installed, 0.22.2 / 0.19.8 researched floors); toolchain bump confirmed done (1.95.0, MSRV 1.94.1 satisfies tar-rs symlink-chmod CVE-2026-33056 requirement). Implication for this chunk: npm audit gate follows the same floor discipline (external tool, fresh DB each run); new Fontsource/Tailwind deps must clear `npm audit` with the installed tool version.
- **2026-06-15-structured-logging-stack** — obs identity env-handles (`CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV`) noted as non-path labels (no validation required). Implication for this chunk: if the Tauri GUI ever emits structured logs with these labels, sanitize them client-side (out of scope for CSS-only token bundle).
