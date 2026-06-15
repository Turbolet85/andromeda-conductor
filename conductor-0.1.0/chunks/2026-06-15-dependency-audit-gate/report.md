# Report — 2026-06-15-dependency-audit-gate

**Chunk:** Dependency-audit gate — cargo-audit + cargo-deny over the OTLP/gRPC/SQLite tree + committed Cargo.lock
**Date:** 2026-06-15T16:42:50Z
**Commits:** none yet — /andromeda-implement does not commit; this wrap creates the chunk commit.

## Changes (structured — detectors read this)
- **Files:**
  - NEW `deny.toml` (repo root) — cargo-deny policy config.
  - MOD `Cargo.toml` (root) — `publish = false` added to `[workspace.package]`.
  - MOD `crates/conductor-{core,timeline,emit,faults,verify,report,cli,tauri}/Cargo.toml` (8 files) — each adds `publish.workspace = true`.
  - (promotion artifacts from /andromeda-phase, committed with this chunk: `master-route.md`, `conductor-0.1.0/working-route.md`, the chunk folder, the phase run-dir.)
- **Symbols / APIs:** none — no source code touched; no new public fns / IPC methods / endpoints / ports / sockets / env vars / exports.
- **Crates / modules:** none added or removed; the 8 existing workspace crates gain `publish = false` metadata only.
- **Dependencies:** **no new Cargo dependencies** — the locked tree is unchanged (still garde 0.22.1 / serde / thiserror + dev serde_json; OTLP/gRPC/SQLite/rmcp/tokio/tracing remain declared-but-unreferenced). New **external dev tooling** (CLI gates, NOT crate deps, NOT in `Cargo.lock`): cargo-audit 0.22.1 + cargo-deny 0.19.4.
- **Schema / config:** NEW `deny.toml` — cargo-deny v2 policy across `[advisories]` (RustSec) · `[bans]` (multiple-versions=warn, wildcards=deny + allow-wildcard-paths) · `[licenses]` (allow = MIT/Apache-2.0/Unicode-3.0/BSL-1.0; private.ignore=true) · `[sources]` (crates.io only). No DB schema / migrations.
- **Coverage of new surfaces:** none — no new external surface / hot-path op / UI element. The deliverable is build-time supply-chain tooling config.
  - (validation n/a · instrumentation n/a · PII n/a · tests n/a · a11y n/a · tokens n/a — no runtime/UI surface added.)

## Deviations from intent
1. **Scope expanded (+9 manifest edits; user-authorized).** The plan scoped touchpoints to `deny.toml`/`Cargo.lock`/`commands.md` and said "no crate changes." But `cargo deny check` could not go green: the workspace's own `conductor-*` crates were treated as publishable → `error[unlicensed]` ×8 + `error[wildcard]` (path deps) ×8. Marking them `publish = false` (via the existing `[workspace.package]` inheritance idiom) resolved all 16 at once — correct for a local-only, no-cloud tool, and it *strengthens* supply-chain posture rather than weakening the gate. Approved by the user via AskUserQuestion.
2. **Plan step 6 (edit `commands.md`) was a no-op.** setup-project had already generated the `## Supply chain` section listing both `cargo audit --deny warnings` and `cargo deny check advisories bans sources licenses`. No edit needed.
3. **`deny.toml` license allowlist trimmed 10 → 4** ({MIT, Apache-2.0, Unicode-3.0, BSL-1.0}). The plan's note said "additive — only licenses actually present." cargo-deny flagged the other 6 as `license-not-encountered`; trimming honored the additive principle and left zero warnings (the gate fails-closed when new licenses appear as the tree grows).
4. **Regression ran on the default nextest profile, not `--profile ci`.** The plan's Test Command copied the documented `cargo nextest run --workspace --profile ci`, but the `ci` profile (`.config/nextest.toml`) does not exist yet — it is created by the later Epoch-1 chunk "Test framework + fixtures". The command errored `profile 'ci' not found`; ran `cargo nextest run --workspace` (identical 20 tests, 20/20 pass). The intent (no regression) is profile-independent; creating the profile would be out-of-scope (test-framework chunk owns it).

## Decisions & corrections
- **Decision (user-approved):** the `conductor-*` crates are `publish = false` — a local-only personal tool; none is ever published to crates.io. Centralized in `[workspace.package]`, inherited per crate.
- **Finding:** cargo-audit + cargo-deny were **already installed** (the prior handoff said "not installed until this chunk"); `Cargo.lock` was already committed & git-tracked. So the chunk was "confirm + author policy + run green", not "install".
- **Finding:** the version floors (MSRV ≥ 1.94.1 / `tauri` ≥ 2.10.3) were **already satisfied** in the manifest (applied in the scaffold chunk) — the audit confirms, it does not bump; `tauri` is not in the lock anyway.
- **Finding (forward, recurring):** the documented `cargo nextest run --workspace --profile ci` (CLAUDE.md §Workflow · `commands.md` · `.claude/rules/verification-harness.md`) fails with `profile 'ci' not found` until the "Test framework + fixtures" chunk creates `.config/nextest.toml`. This will hit every implement chunk's regression gate until then — same Foundation-sequencing class as the existing playbook rule about interim `cargo test`.
- **Open item closed:** garde 0.22.1 confirmed advisory-free (RustSec scan clean).

## Outcome
- **Acceptance criteria:** all 7 met.
- **Gates green (commands run):**
  - `cargo audit --deny warnings` → exit 0 (30 deps scanned, 0 advisories; garde 0.22.1 clean).
  - `cargo deny check advisories bans sources licenses` → exit 0 (advisories ok · bans ok · licenses ok · sources ok).
  - `cargo build --workspace --locked` → exit 0, **no `Cargo.lock` drift**.
  - `cargo nextest run --workspace` → 20/20 pass (default profile — deviation 4).
  - `cargo clippy --workspace --all-targets -- -D warnings` → exit 0.
- **Smoke:** skipped — no boot-path change (config/metadata only; no binary/daemon/entry-point touched).
