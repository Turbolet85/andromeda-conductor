# security extract

## Relevance
partial — establishes the workspace + toolchain foundation; touches dependency-audit + toolchain hardening, but no input-validation/error-handling runtime boundaries yet.

## Constraints
1. `rust-toolchain.toml` MUST pin ≥1.94.1 per security-plan §Dependency Security + Bootstrap phases (clears tar-rs symlink-chmod CVE-2026-33056 / RUSTSEC-2026-0033).
2. `Cargo.lock` MUST be committed per security-plan §Dependency Security (deterministic cargo-audit/cargo-deny; stops the bundled-SQLite C version floating past advisory tracking).
3. `tauri` MUST pin ≥2.10.3 in `[workspace.dependencies]` per security-plan §Dependency Security (origin-confusion CVE-2026-42184; arch's 2.10.1 is exposed).
4. All 8 crates declared (6 lib, 2 bin) with `conductor-*` names per security-plan §Input Validation (crate-per-seam enforcement) + arch §Module Boundaries.
5. `[workspace.package]` declares centralized version + `edition = "2024"` per security-plan §Dependency Security (uniform pinning for later audit).
6. No floating semver in `[workspace.dependencies]` per security-plan §Dependency Security (pinned so cargo-audit is deterministic).

## Patterns to follow
1. `[workspace.dependencies]` is the single source of truth for pinned versions; seam crates reference `.workspace = true` (security-plan §Dependency Security).
2. `conductor-core` stays a minimal compiling library this chunk; shared-type bodies + input-validation seams attach in later chunks (security-plan §Input Validation boundary deferral).
3. Workspace positioned for cargo-audit 0.22.2 (+ optional cargo-deny 0.19.8) CI gates later — this chunk's pinning is the prerequisite (security-plan §Dependency Security).

## Anti-patterns to avoid
1. NEVER `tauri` < 2.10.3 or toolchain < 1.94.1 — required bumps per security-plan §Security Anti-Patterns (Universal).
2. NEVER floating semver in workspace deps — pin exact per security-plan §Dependency Security.
3. NEVER omit/drift `Cargo.lock` per security-plan §Security Anti-Patterns (Universal).

## Contract bindings
- **security ↔ tests/CI**: the cargo-audit / cargo-deny CI gate is wired in a later CI chunk (security-plan §Dependency Security §CI integration); the workspace structure + committed `Cargo.lock` here are its prerequisites.

## Acceptance criteria contributions
- (security) `rust-toolchain.toml` pins ≥1.94.1; workspace is edition-2024.
- (security) `tauri` pinned ≥2.10.3 in `[workspace.dependencies]`.
- (security) `Cargo.lock` committed; `.gitignore` does not suppress it.
- (security) `cargo build --workspace` succeeds (green skeleton; the audit gate is added in the CI chunk).

## Relevant amendment history
(none)