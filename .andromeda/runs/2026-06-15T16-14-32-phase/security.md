# security extract

## Relevance
Relevant — this chunk owns the first green `cargo-audit` + `cargo-deny` run and confirms dependency audit gates for the Minimal-tier residual-risk class.

## Constraints
1. `cargo-audit` 0.22.2 must run green over the committed, un-drifted `Cargo.lock` — this is the minimum bar for any `cargo build --release` or merge (per security-plan.md §Dependency Security; §Universal anti-pattern: "NEVER run `cargo build --release` without `cargo-audit` green").
2. `cargo-deny` 0.19.8 must run green with a committed `deny.toml` covering advisories, bans, licenses, and sources policy — recommended superset that gates banned crates, duplicate versions, untrusted registries, and license compliance (per security-plan.md §Dependency Security).
3. `Cargo.lock` must remain committed and un-drifted; a `--locked` build/audit produces no regeneration — this determinism is the precondition for `cargo-audit`/`cargo-deny` reproducibility (per security-plan.md §Dependency Security; §Universal anti-pattern: "NEVER let `Cargo.lock` drift or go uncommitted").
4. Toolchain pinned to ≥ 1.94.1 (resolves tar-rs symlink-chmod CVE-2026-33056 / RUSTSEC-2026-0033 in the `cargo build` extraction path) and `tauri` ≥ 2.10.3 (resolves origin-confusion CVE-2026-42184 on Windows) — these required bumps must be confirmed by the audit gate (per security-plan.md §Dependency Security; Architecture currently pinned at 2.10.1 / MSRV 1.88.0; bumps are required).
5. `garde` 0.22.1 pinned (per amendment 2026-06-15-config-validation-surface) — verify the downgrade from 0.23.0 introduces no new advisories in the input-validation boundary (per security-plan.md §Input Validation scenario-config boundary row; amendment history confirms this session's downgrade to 0.22.1 due to registry unavailability of 0.23.0).

## Patterns to follow
1. Invocation: `cargo audit --deny warnings` (the RustSec scanner minimum bar) + optional `cargo deny check advisories bans sources licenses` (the superset gate) — both are deterministic over the committed `Cargo.lock` and repeatable in CI (per security-plan.md §Dependency Security; §Bootstrap phases: dep-audit-tooling-install).
2. CI integration: add `cargo install cargo-audit` + `cargo audit` step (and optionally `cargo deny check`) to the existing `.github/workflows/` that already runs `cargo build` / nextest / clippy; build fails on advisory hit (per security-plan.md §Dependency Security: "CI integration").
3. For any new dependency introduced later, re-run the gate to confirm no advisory regression and that dependency sources remain `crates.io` only (per deny.toml `sources` check).

## Anti-patterns to avoid
1. NEVER run `cargo build --release` without `cargo-audit` green and `Cargo.lock` committed (universal ban — security-plan.md §Universal: "NEVER run `cargo build --release` without `cargo-audit` (and recommended `cargo-deny`) green").
2. NEVER let `Cargo.lock` drift uncommitted or allow `--locked` builds to regenerate it — this breaks audit determinism and lets the bundled SQLite C version float past advisory tracking (security-plan.md §Universal: "NEVER let `Cargo.lock` drift or go uncommitted").
3. NEVER adopt an unvetted secret-scanning tool if added later — the plan defers tool selection because no scanner was researched in Phase 2 (per security-plan.md §Secret Management; Decisions Log: "No specific secret-scanning tool was researched… selection is deferred").

## Contract bindings
Tests §CI Integration — the existing GitHub Actions pipeline that runs `cargo build` / nextest / clippy is the target for the optional CI wiring (deferred to the later Epoch-1 chunk "Base CI + agent-run harness skeleton" per scope.md); this chunk makes the gate runnable + green locally.

## Acceptance criteria contributions
1. (security) `cargo audit` passes green over the committed `Cargo.lock` (no open RustSec advisories).
2. (security) `cargo deny check advisories bans sources licenses` passes green against the committed `deny.toml`.
3. (security) `Cargo.lock` is committed and `--locked`-clean (no drift on regeneration).
4. (security) Toolchain ≥ 1.94.1 and `tauri` ≥ 2.10.3 pins confirmed by the audit (tar-rs CVE-2026-33056 and origin-confusion CVE-2026-42184 resolutions).
5. (security) `garde` 0.22.1 downgrade confirmed advisory-free (closes previous session's open item per amendment 2026-06-15-config-validation-surface).

## Relevant amendment history
**2026-06-15-config-validation-surface** — `garde` downgraded from 0.23.0 (unavailable in registry) to 0.22.1; this chunk must confirm the downgrade introduces no new advisories in the input-validation boundary (per §Input Validation scenario-config row and §Bootstrap phases input-validation-library-install). The validation contract remains unchanged: `#[derive(Validate)]` + `#[garde(custom)]` rules at load, `CONDUCTOR_*` path canonicalize-and-bounds-check at the CLI edge.