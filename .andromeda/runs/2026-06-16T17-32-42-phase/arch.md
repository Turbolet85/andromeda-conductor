# arch extract

## Relevance
Relevant — chunk formalizes the headless CI release-gate path and agent-run harness skeleton that are pinned foundational infrastructure in the architecture.

## Constraints
- Per §Stack and Technologies: Rust 2024 (cargo 1.85, MSRV 1.94.1) + tokio 1.48.x, tonic 0.14.6, rusqlite 0.38.0 (bundled SQLite 3.51.1), serde 1.0.x + garde 0.22.1 (field-level custom only, cross-field via Context), thiserror 2.0.18 + anyhow 1.0.102, tracing 0.1.44 + tracing-subscriber 0.3.23, serde_json 1.0.
- Per §Established Decisions [Async Runtime Flavor] + §Design Philosophy: tokio `current_thread` with zero work-stealing; `#[tokio::main(flavor = "current_thread")]` on CLI path; hand-built `Builder::new_current_thread()` owned by core.
- Per §Infrastructure Patterns — Build system: GitHub Actions runs `cargo build --workspace`, `cargo nextest run --workspace --profile ci` (zero-retry), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --doc`, `cargo audit` (RustSec), `cargo deny`, `cargo-llvm-cov` (llvm-tools-preview), `npm audit` + `vite build` for frontend; `rust-toolchain.toml` pins toolchain; Cargo.lock must be committed/un-drifted.
- Per §Conventions: snake_case Rust; kebab-case crate names; scenarios keyed by P-ID; on-disk artifacts use kebab-case filenames + hyphen-delimited `run_id` (YYYY-MM-DDTHH-MM-SS format for Windows filename compatibility).
- Per §Occupied Resources [Crate names]: workspace members are `conductor-core`, `conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report`, `conductor-cli`, `conductor-tauri` (LOCKED names).
- Per §Inherited Defaults [Module boundaries]: Crate-per-seam Cargo workspace; forbidden cross-seam deps fail to compile.

## Patterns to follow
- Per §Established Decisions [Workspace / Core Structure]: Runtime-agnostic core library (`conductor-core`) + CLI bin (`conductor-cli`) + Tauri bin (`conductor-tauri`) at compile-time boundary; both CLI and GUI call identical core commands, enforcing headless-drivable at the type level.
- Per §Cross-cutting Patterns — Development Style: Agent-driven — `scripts/agent-run.sh` path is the source of truth and release gate; GUI is a thin shell. This signal reads downstream to tests/obs/setup-project specialists.
- Per §Infrastructure Patterns — Deployment model: Local-only; `cargo build --release` produces `conductor-cli` binary run beside Pulse via `scripts/agent-run.sh`; optional Tauri 2 bundler produces ~3 MB GUI installer.
- Per §Conventions [Naming patterns]: `run_id` uses filesystem-safe hyphen-delimited stamp (YYYY-MM-DDTHH-MM-SS-<suffix>) as the `runs.db` primary key and stem of `<run_id>.jsonl` + run report (colons illegal in Windows filenames).

## Anti-patterns to avoid
- Per §Established Decisions [Async Runtime]: Do NOT use multi-threaded tokio runtime in seam crates; determinism is only guaranteed by `current_thread` with zero work-stealing.
- Per §Design Philosophy [Outcomes are values, errors are harness faults]: Do NOT use Rust errors for verification outcomes; `Result::Err` is reserved for harness failures only (config parse, transport down, MCP unreachable) — verdict states are typed values.
- Per §Infrastructure Patterns — CI/CD approach: Dynamic end-to-end scenario proof (error-baseline-spike, fingerprint-storm, restart-suppression) is an explicit operator/local gate, NEVER a CI gate (CI has no Pulse instance).

## Contract bindings
- Tests harness ↔ CI (per §Infrastructure Patterns: nextest `ci` profile zero-retry discipline binds to flakiness budget; test-plan §4 governs exact tool/dep versions).
- Obs conformance ↔ CI (per amendment 2026-06-15-structured-logging-stack: self-obs stack now pinned; obs-plan §3 governs emissions; obs CI conformance gate is Epoch 10).
- Frontend ↔ CI (per amendment 2026-06-15-design-token-typography-bundle: npm audit + vite build gate parallel to cargo gate; frontend lives at `crates/conductor-tauri/ui/` not a Cargo member).

## Acceptance criteria contributions
- (arch) `scripts/agent-run.sh` and `scripts/agent-run.ps1` expose the canonical 5-command skeleton (boot/run/status/cleanup/logs) with consistent dispatch and exit-0 smoke on both shells (per §Occupied Resources workflow patterns + scope intent).
- (arch) GitHub Actions workflow gates `cargo build --workspace`, `cargo nextest` (zero-retry `ci` profile), `cargo clippy -- -D warnings`, `cargo test --doc`, `cargo audit`, `cargo deny`, `cargo-llvm-cov`, and `npm audit`/`vite build` on the dev-OS target, honoring all tools pinned in §Stack and §Infrastructure Patterns.
- (arch) `Cargo.lock` is committed and never drifted; `rust-toolchain.toml` pins Rust 2024 / MSRV 1.94.1 (per amendment 2026-06-14-cargo-workspace-scaffold).
- (arch) Agent-run scripts enforce the headless release-gate principle (per §Design Philosophy and §Cross-cutting Patterns — Development Style).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold** — MSRV raised 1.88.0 → 1.94.1 (§Stack, §Inherited Defaults); security-plan §Dependency Security required bump (tar-rs CVE-2026-33056).
- **2026-06-15-structured-logging-stack** — `CONDUCTOR_SERVICE_NAME` + `CONDUCTOR_ENV` added to §Occupied Resources env-var inventory (obs-plan §3 reads both for `ServiceIdentity`); no CI impact.
- **2026-06-15-design-token-typography-bundle** — frontend stack (React 19.x + Vite 8.0.16 + Tailwind 4.1 + npm) registered in §Stack and §Occupied Resources; frontend CI gate (`npm audit` + `vite build`) now part of the base workflow (amendment ratified at wrap escalation).
- **2026-06-16-test-framework-fixtures-coverage-tooling** — test/coverage toolchain registered in Build system (cargo-nextest zero-retry `ci` profile + `cargo test --doc` + cargo-llvm-cov + dev-test stack rstest/proptest/insta/assert_cmd/assert_fs/predicates); spec-sound-impl alignment.