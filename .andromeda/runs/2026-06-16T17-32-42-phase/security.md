# security extract

## Relevance
Partial — chunk wires the CI pipeline and harness skeleton, both security-gated per §Dependency Security and §Error Handling.

## Constraints
1. CI pipeline MUST enforce `cargo-audit` ≥0.22 (installed 0.22.1) + `cargo-deny` ≥0.19 (installed 0.19.4) green before build success (security plan §Dependency Security, Bootstrap phases §dep-audit-tooling-install); build fails on advisory hit per supply-chain SLA.
2. `Cargo.lock` MUST remain committed and un-drifted — `cargo-audit`/`cargo-deny` scans are non-deterministic without it (security plan §Dependency Security, Anti-Patterns § Universal).
3. Frontend npm supply chain (`crates/conductor-tauri/ui/`) MUST gate on `npm audit` clean (0 vulnerabilities) + committed `package-lock.json` before merging frontend changes; Vite ≥8.0.16 (esbuild GHSA-gv7w-rqvm-qjhr fix) (security plan §Dependency Security § Frontend (npm) supply chain).
4. Rust toolchain MUST be ≥1.94.1 per `rust-toolchain.toml` to clear tar-rs symlink-chmod CVE-2026-33056 / RUSTSEC-2026-0033 in `cargo build` extraction path (security plan §Dependency Security § Pinning, Anti-Patterns § Code Patterns).
5. Tauri GUI dependency (dormant in Epoch 1, active in Epoch 9) MUST be pinned ≥2.10.3 to clear origin-confusion CVE-2026-42184 on Windows when Tauri surface lands (security plan §Dependency Security § Update policy, Anti-Patterns § Code Patterns).
6. `agent-run.sh` / `agent-run.ps1` entry points MUST NOT add interactive login prompts to the headless source-of-truth path — it is agent-driven by design (security plan §Security Anti-Patterns § Authentication).

## Patterns to follow
1. CLI argument / env-var parsing (seed/scenario flags, `CONDUCTOR_*` overrides) MUST canonicalize file paths + bounds-check before accepting — bounds-check happens at `conductor-cli` edge, outside garde struct validation (security plan §Input Validation table, Anti-Patterns § Input).
2. All `#[tauri::command]` return values and `conductor-cli` stderr outputs MUST be sanitized — no absolute paths, stack traces, internal struct names, or library versions exposed (security plan §Error Handling).

## Anti-patterns to avoid
1. NEVER let `Cargo.lock` drift or go uncommitted — it is prerequisite for deterministic `cargo-audit`/`cargo-deny` scans and `bundled` SQLite C advisory tracking (security plan Anti-Patterns § Universal, §Code Patterns).
2. NEVER run `cargo build --release` (release gate) or merge without `cargo-audit` + `cargo-deny` green — the supply-chain audit is the Minimal-tier residual-risk control (security plan Anti-Patterns § Universal).

## Contract bindings
(none) — security domain is read-only on existing config/CI tooling (no cross-domain implementation needed in this chunk); §Dependency Security observes `deny.toml` + `rust-toolchain.toml` as already-committed constraints.

## Acceptance criteria contributions
1. (security) `cargo audit` (RustSec) passes green in CI, blocking build on advisory hit.
2. (security) `cargo deny check` (advisories + bans + sources + licenses) passes green in CI, blocking build on advisory hit.
3. (security) `npm audit` clean (0 vulnerabilities) for `crates/conductor-tauri/ui/` and `package-lock.json` committed before frontend merge.
4. (security) `Cargo.lock` remains committed and un-drifted (verified by CI diff check if needed).
5. (security) `rust-toolchain.toml` ≥1.94.1 committed (already done — 1.95.0 channel, `rust-version = 1.94.1`).

## Relevant amendment history
- **2026-06-15-dependency-audit-gate**: cargo-audit 0.22.1 / cargo-deny 0.19.4 installed (below researched 0.22.2 / 0.19.8 but audit-gate landed green); reframed versions as minimum floors (external CLI tools, RustSec DB fresh each run); toolchain bump to 1.94.1 **confirmed done** (channel 1.95.0). Tauri ≥2.10.3 (CVE-2026-42184) left as forward required-bump for Epoch 9 when Tauri GUI lands.
- **2026-06-15-structured-logging-stack**: obs identity env-handles (`CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV`) noted as non-path labels (no validation needed), distinct from `CONDUCTOR_*` path handles. Unblocks §Input Validation clarification.
- **2026-06-15-design-token-typography-bundle**: npm supply-chain gate added for `crates/conductor-tauri/ui/` — `npm audit` clean + committed `package-lock.json` + vendored fonts (no runtime CDN); Vite ≥8.0.16 minimum (esbuild GHSA-gv7w-rqvm-qjhr fix).