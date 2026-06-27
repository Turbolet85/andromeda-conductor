# security extract

## Relevance
Partial — this chunk adds npm and Rust test dependencies (Dependency Security primary focus), exercises existing Tauri IPC boundaries through tests (inherits existing validation rules), and applies Tauri anti-patterns to GUI code under test. No new threat boundaries or trust model introduced.

## Constraints
1. (npm frontend audit gate) `npm audit` must be clean (0 vulnerabilities) before merging changes to `crates/conductor-tauri/ui/` devDependencies (per security plan §Dependency Security, Frontend (npm) supply chain).
2. (npm lockfile commitment) `package-lock.json` must be committed to the repo to ensure deterministic, reproducible npm-audit scans (per security plan §Dependency Security, Pinning).
3. (Rust test-dep audit inheritance) Test dependencies under `[dev-dependencies]` inherit the `cargo audit` ≥0.22 + `cargo deny` ≥0.19 gates; `Cargo.lock` must remain committed and un-drifted (per security plan §Dependency Security, Audit tool + Pinning).
4. (Tauri capabilities deny-by-default) Any Tauri plugins (e.g., `shell-open`) used in the GUI code must be explicitly scoped with a deny-by-default capabilities file; unscoped plugins enable RCE (per security plan §Security Anti-Patterns § Code Patterns, CVE-2025-31477).
5. (Tauri IPC test-command scope) `tauri::test` mock-runtime tests exercise ONLY the documented IPC commands (`start_run` / `stop_run` / `run_report` / `resolve_operator_hold`) plus two `Channel`s (live-counter + `HoldPrompt`); no undocumented commands or unscoped capabilities in test setup (per security plan §Security Anti-Patterns § Code Patterns).
6. (unsafe blocks require review) Any `unsafe` added to Tauri bindings or test code must undergo security review before landing (per security plan §Security Anti-Patterns § Universal).

## Patterns to follow
1. Committed lockfiles for auditability — keep `Cargo.lock` and `package-lock.json` committed un-drifted so both `cargo-audit` and `npm audit` scans are deterministic and reproducible (per security plan §Dependency Security, Pinning).
2. npm audit floor discipline — npm advisories flow in with the same rigor as cargo findings (floor-version approach; no rigid dependency allowlist at Minimal tier) (per security plan §Dependency Security, Frontend (npm) supply chain).
3. Mock-runtime test isolation — `tauri::test` harness keeps boundaries clear and deterministic; tauri-driver tests scope to documented command set (per test-plan Path 7 + a11y-plan §3).

## Anti-patterns to avoid
1. NEVER add npm dependencies without `npm audit` clean — merge blocker (per security plan §Anti-Patterns § Code Patterns + §Dependency Security Frontend section).
2. NEVER allow `Cargo.lock` or `package-lock.json` drift or uncommitted state — breaks audit determinism and lets versions float past advisories (per security plan §Anti-Patterns § Universal).
3. NEVER use Tauri `shell-open` plugin unscoped or embed remote-origin iframes — unscoped plugin enables RCE; iframes bypass origin checks and re-expose in-process IPC assumption (per security plan §Anti-Patterns § Code Patterns, CVE-2025-31477 + GHSA-57fm-592m-34r7).

## Contract bindings
- **tests ↔ security:** Test-dependency audit gates (`npm audit` clean, `cargo audit`/`cargo deny` pass, lockfiles committed) are blocking preconditions; build fails if any gate fails. Tests inherit existing Tauri IPC validation rules (no undocumented command calls).
- **a11y ↔ security:** a11y testing (axe, contrast, keyboard) exercises existing components' behavior; a11y rules (WCAG, ARIA) are orthogonal to security and must not override security constraints.

## Acceptance criteria contributions
1. (security) `npm audit` clean (0 vulnerabilities) on all changes to `crates/conductor-tauri/ui/` (Dependency Security §Frontend).
2. (security) `cargo audit` + `cargo deny check` pass on Rust test dependencies (Dependency Security §Audit tool).
3. (security) `Cargo.lock` + `package-lock.json` both committed, un-drifted, and stable (Dependency Security §Pinning).
4. (security) Tauri GUI code properly scoped — deny-by-default capabilities file present if any plugin used; `shell-open` not unscoped (Anti-Patterns § Code Patterns).

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — npm supply-chain gate established (`npm audit` clean + `package-lock.json` committed) for the `crates/conductor-tauri/ui/` tree (frontend devDeps: React 19, Vite, Tailwind, Fontsource). This chunk adds more npm devDeps (axe, lighthouse, colorjs.io, tauri-driver); the gate carries forward as a binding precondition. Resolved with user: "npm-audit-clean gate + committed lockfile is the control."
- **2026-06-15-dependency-audit-gate** — Clarified cargo-audit/cargo-deny version pins as minimum **floors** (0.22.1 / 0.19.4 in-place), not rigid locks (external CLI tools, advisory DB fetched at runtime). Noted tauri ≥2.10.3 as a required forward bump (origin-confusion CVE-2026-42184 on Windows). tauri is dormant in the stack (§Stack declaration pins 2.10.1) but becomes active if this Epoch-9 chunk updates tauri dependencies — the ≥2.10.3 floor applies.