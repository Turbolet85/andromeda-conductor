# security extract

## Relevance
partial — the chunk adds no input boundary, spawn, secret or network surface; its security surface is the supply-chain gate set (`cargo deny` license policy, the two committed lockfiles, `npm audit`) and the secret-scan gate over the two new tracked files.

## Constraints
- security-plan §Dependency Security (Pinning) + §Security Anti-Patterns → Universal require `Cargo.lock` to stay committed and un-drifted. A `license` field is package metadata, and whether setting it (via `[workspace.package]` inheritance) moves the lock at all is research's question. Per the security rules' 2026-09-02 addition, state the basis as the package COUNT or an explicit new-package diff, never as lockfile byte-identity.
- security-plan §Dependency Security (Accepted exceptions) makes `deny.toml` the authority on the license allow set. Any `[licenses]` change needs a justifying comment, and the plan's count (9 `[licenses] allow` entries, as of 2026-09-07) is a reconciliation stamp against `deny.toml`, not a second source. Whether `MIT` and `Apache-2.0` are already in `allow` (scope says `deny.toml:55-56`) is research's question to confirm. The P4 fork on `private = { ignore = true }` (still skipping our own crates vs now checking them) is a `deny.toml` policy change if flipped, and it binds this clause.
- security-plan §Dependency Security (Frontend supply chain) requires `package-lock.json` to be committed, and `npm audit --omit=dev` to show 0 production vulnerabilities before merging frontend changes. The `package.json` `license` edit is a frontend-manifest change, so the gate applies. The lockfile's mirrored root field must come from a real npm regeneration.
- security-plan §Security Anti-Patterns → Universal forbids merging or `cargo build --release` without `cargo-audit` and `cargo-deny` green. §Dependency Security's audit↔deny overlap means both run: a green `cargo audit` does not imply a green `cargo deny check licenses`.
- security-plan §Dependency Security (two external-decay faults) requires the advisory-DB checkout's porcelain check BEFORE any red `cargo audit` is classified as external. A red audit in a zero-dependency-delta chunk is not this chunk's defect, and it is never answered with a `deny.toml` ignore.
- security-plan §Bootstrap phases (secret-scanning-ci-gate) + §Security Anti-Patterns → Secrets require the `Secret-scan gate` to cover git's cached and untracked-not-ignored listing. `LICENSE-MIT` and `LICENSE-APACHE` enter that listing and must pass it with no allowlist growth. Whether either text trips a content rule is research's question.

## Patterns to follow
- Supply-chain exceptions go through `deny.toml`'s justified-entry mechanism (security-plan §Dependency Security, Accepted exceptions). A new `allow` or a policy flip carries its reason inline and is never a silent skip.
- A lockfile changes only through its own tool: `cargo` regenerates `Cargo.lock`, and `npm install` regenerates `package-lock.json`. The committed lock is what makes the audit deterministic (security-plan §Dependency Security, Pinning; Frontend supply chain).
- Run the audit↔deny overlap and verify both, never assume either (security-plan §Dependency Security): `cargo audit` plus `cargo deny check advisories bans licenses sources` for Rust, and `npm audit --omit=dev` for the npm tree, which neither Rust tool covers.

## Anti-patterns to avoid
- Hand-editing `package-lock.json` or `Cargo.lock`, or leaving either drifted or uncommitted (security-plan §Security Anti-Patterns → Universal; §Dependency Security, Frontend supply chain).
- Getting `cargo deny check licenses` to green by suppressing a signal class, for example a blanket `allow`, removing a check, or an unjustified entry, instead of a justified `deny.toml` entry (security-plan §Dependency Security, Accepted exceptions and two external-decay faults).
- Adding a secret-scan allowlist entry to admit the license texts without a stated reason (security-plan §Security Anti-Patterns → Secrets: never skip the secret-scanning gate).

## Contract bindings
- CI security gate ↔ test-plan §9/§10 (CI integration and quality gates). The `rust` job's `cargo audit`, `cargo deny check` and `Secret-scan gate` steps are the checks this chunk's own CI run has to pass. Scope says the prior wrap's CI verdict (`c97f697`) must be re-read before this chunk's CI is judged.
- The license policy in `deny.toml` ↔ the architecture's workspace manifest shape: inheriting through `[workspace.package]` is an arch decision, and whether `cargo deny` sees our own crates depends on the `publish = false` + `private.ignore` pairing.

## Acceptance criteria contributions
- `cargo deny check advisories bans licenses sources` exits 0 over the post-change tree, and the `licenses` result matches the P4 fork's outcome: our crates either still skipped by `private.ignore`, or checked and accepted under `MIT OR Apache-2.0` (per security-plan §Dependency Security, Accepted exceptions).
- `cargo audit` exits 0, captured before any pipe, with the advisory-DB porcelain check run first. The package count is unchanged, so the dependency delta is zero (per security-plan §Dependency Security, two external-decay faults; §Security Anti-Patterns → Universal).
- `package-lock.json` is committed, its `packages[""].license` reads `MIT OR Apache-2.0` after an npm regeneration, and `npm audit --omit=dev` reports 0 production vulnerabilities (per security-plan §Dependency Security, Frontend supply chain).
- The `Secret-scan gate` (`crates/conductor-core/tests/secret_scan_gate.rs`) passes with `LICENSE-MIT` and `LICENSE-APACHE` present, and its allowlist is unchanged (per security-plan §Bootstrap phases, secret-scanning-ci-gate).
