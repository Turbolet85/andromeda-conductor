# arch extract

## Relevance
relevant — dependency audit is a critical foundational gate that locks the architecture's supply-chain integrity and affects all downstream chunks.

## Constraints
1. Lock `Cargo.lock` as the single source of truth for reproducible builds per arch §Stack and Technologies (cargo 1.85) + §Established Decisions [Language / Runtime Rust 2024].
2. Audit OTLP/gRPC stack (opentelemetry-proto 0.32.0, tonic 0.14.6, tonic-prost 0.14.6, prost 0.14) per arch §Stack and Technologies and §Occupied Resources.
3. Audit MCP client rmcp 1.7.0 against the version-pinned contract per arch §Established Decisions [MCP Read-Back Client].
4. Confirm validation stack (serde 1.0.x + garde 0.22.1 post-downgrade) is advisory-free per arch §Stack and Technologies + §Established Decisions [Validation Library] (garde 0.22.1 was ratified after 0.23.0 proved unbuildable).
5. Audit SQLite FFI tree (rusqlite 0.38.0, libsqlite3-sys 0.38.0 `bundled` → SQLite 3.51.1) per arch §Stack and Technologies + §Established Decisions [Database].
6. Enforce MSRV ≥1.94.1 (tar-rs CVE-2026-33056) per §Stack and Technologies and §Infrastructure Patterns Build system, and Tauri ≥2.10.3 (origin-confusion CVE-2026-42184) per the scope document.
7. Verify determinism discipline (tokio 1.48.x `current_thread` flavor, no async-entanglement in rusqlite path) is preserved in the locked tree per arch §Design Philosophy and §Established Decisions [Async Runtime Flavor].

## Patterns to follow
1. **Workspace dependency lock discipline** — `Cargo.lock` is binary and committed as the reproducibility anchor; `--locked` builds must produce zero drift per arch §Infrastructure Patterns Build system.
2. **Supply-chain baseline at feature lock** — the audit gate runs after every new public dependency is added or version is bumped and before merge; this chunk establishes that baseline and the gate itself.
3. **RustSec + deny policy stacking** — cargo-audit catches known CVEs; cargo-deny adds layers for yanked/duplicate/disallowed crates and license/source validation per the scope document.

## Anti-patterns to avoid
1. Do not introduce new public dependencies outside the pinned set without re-running the full audit gate (scope: audit tooling only, not code changes that add transitive deps).
2. Do not allow Cargo.lock drift; if it drifts, regenerate it `--locked` and commit, never hand-edit.
3. Do not weaken the allowed-license policy in deny.toml; local-only tool has restrictive criteria per arch §Conventions (Validation).

## Contract bindings
- **security §Dependency Security** (upstream) — this chunk closes the audit gate the security plan prescribes; drift-detection in later chunks depends on this gate's green baseline.
- **tests harness** (downstream) — golden tests will consume the locked dependency tree; CI (downstream chunk _"Base CI + agent-run harness skeleton"_) gates CI builds on audit-green.
- **obs §Self-observation** (upstream trust) — tracing 0.1.44 + tracing-subscriber 0.3.23 were added in prior chunk; audit now confirms no drift.

## Acceptance criteria contributions
- (arch) `Cargo.lock` is committed and a `--locked` build produces zero drift per §Infrastructure Patterns Build system.
- (arch) `cargo audit` runs green over the full locked tree; all historical CVE pins (MSRV ≥1.94.1 for tar-rs, Tauri ≥2.10.3) are confirmed in the lock.
- (arch) `cargo deny check` runs green against deny.toml (advisories + bans + licenses + sources policy) per §Stack and Technologies + §Conventions [Interface surfaces].
- (arch) The garde 0.22.1 downgrade (post-0.23.0 registry unavailability per amendments) is confirmed advisory-free; no new CVE introduced by the version floor adjustment.

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold** (MSRV raised 1.88.0 → 1.94.1): Workspace now pins 1.95.0 toolchain with `rust-version = "1.94.1"` MSRV floor (tar-rs CVE-2026-33056). This chunk verifies cargo-audit catches that floor.
- **2026-06-14-cargo-workspace-scaffold** (tracing added): Self-observation stack (tracing 0.1.44 + tracing-subscriber 0.3.23) was added to §Stack and Technologies. This chunk audits no new CVE in that row.
- **2026-06-15-config-validation-surface** (garde downgrade 0.23.0 → 0.22.1): garde 0.23.0 + `derive` proved unbuildable (garde_derive 0.23.0 missing from registry); downgrade to 0.22.1 was authorized. This chunk closes the open item: confirm no new advisory at 0.22.1.