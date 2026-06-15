# tests extract

## Relevance
partial — establishes the workspace structure downstream test chunks reference; no testable behavior in this chunk itself.

## Constraints
- Per test-plan §1: the 8 crate-per-seam members (6 lib, 2 bin) are the boundary for per-seam `cargo nextest run -p conductor-<seam>` unit isolation.
- Per test-plan §3: `[workspace.dependencies]` + `[workspace.package]` centralize pinned versions so later chunks reference `{dep}.workspace = true`; enables the 5-command discipline + CI stage flags.
- Per test-plan §9: workspace structure + committed `Cargo.lock` are prerequisite for the deterministic `Cargo.lock`-keyed cache + reproducible cargo-audit gate.
- Per test-plan §10: `cargo build --workspace` is the smoke gate; coverage tooling keys on the resolved `Cargo.lock`.
- Per test-plan §10 (citing security): `rust-toolchain.toml` ≥1.94.1 (tar-rs CVE); committed+undrifted `Cargo.lock` is a build-failure condition.

## Patterns to follow
- Per test-plan §3: the 8 crates host per-seam `#[cfg(test)]` modules + crate-local `tests/`; the scaffold establishes the `crates/conductor-*` layout these inhabit.
- Per test-plan §2: snake_case source + test fns; golden snapshots under `<crate>/tests/snapshots/` (insta) added later.

## Anti-patterns to avoid
- Per test-plan §11: do NOT introduce cross-seam deps beyond seam→core (the Cargo.toml structure is compiler-enforced and blocks later violations).
- Per test-plan §3: no daemon / no inbound listener — never add a listener bind outside the `:4317` port-occupier fault scope.

## Contract bindings
- **tests ↔ obs**: the log-format binding (test-plan §3 ↔ obs-plan §3) defines per-run JSONL at `runs/<run_id>.jsonl`; this scaffold establishes the crates that will implement journal emission later.

## Acceptance criteria contributions
- (tests) `cargo build --workspace` succeeds (green skeleton — prerequisite for all later test layers) per test-plan §10.
- (tests) All 8 crates exist with correct names + crate-types; a forbidden cross-seam edge fails the build per test-plan §1 + §11.
- (tests) `rust-toolchain.toml` pins ≥1.94.1; root manifest `edition = "2024"` workspace-wide per test-plan §10.
- (tests) `Cargo.lock` committed (deterministic cargo-audit + reproducible builds) per test-plan §9/§10.

## Relevant amendment history
(none)