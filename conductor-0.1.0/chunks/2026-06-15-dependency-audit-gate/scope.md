# Scope — Dependency-audit gate

**Marker:** `2026-06-15-dependency-audit-gate`
**Version:** conductor-0.1.0 · Epoch 1 — Foundation
**Working entry:** _Dependency-audit gate — cargo-audit + cargo-deny over the OTLP/gRPC/SQLite tree, committed Cargo.lock_

## What this builds
The supply-chain **audit gate** for the workspace: the tooling, the policy config, and the first green run
that turn the universal invariant — _"never `cargo build --release` or merge without `cargo-audit` (+
`cargo-deny`) green and a committed, un-drifted `Cargo.lock`"_ — into an enforceable, repeatable check rather
than a maxim. (Authoritative source: `.andromeda/security-plan.md` §Dependency Security.)

- **cargo-audit (RustSec)** over the *committed* `Cargo.lock`, failing on any open RustSec advisory.
  **Refined at P3/P4 (intent-incomplete):** the title names the "OTLP/gRPC/SQLite tree", but those crates
  (opentelemetry-proto, tonic, tonic-prost, prost, rmcp, rusqlite, libsqlite3-sys, tokio, tracing) are
  declared in `[workspace.dependencies]` yet **not referenced**, so they are **not in the lock yet** (they
  land in Epochs 3/5/6). Today's lock is the `conductor-core → garde 0.22.1 / serde / thiserror` tree. This
  chunk therefore establishes the **gate** + a **green baseline over today's lock**; the gate's remit extends
  to the OTLP/gRPC/SQLite tree automatically as those deps are added ("baseline at feature lock"). It is NOT
  this chunk's job to pull those deps in early.
- **cargo-deny** with a committed `deny.toml` spanning the four check families: `advisories` (RustSec DB),
  `bans` (duplicate / yanked / disallowed crates), `licenses` (allowed-license policy for a local-only
  personal tool), and `sources` (crates.io as the sole permitted registry).
- **`Cargo.lock` committed + un-drifted** — the precondition that makes the audit deterministic; verify it is
  checked in (binary workspace) and that a `--locked` build/audit produces no drift.
- **Version-floor confirmation (not bump)** — the toolchain ≥ 1.94.1 (tar-rs CVE-2026-33056) and `tauri`
  ≥ 2.10.3 (origin-confusion CVE-2026-42184) floors named by the security plan are **already satisfied** in
  `Cargo.toml`/`rust-toolchain.toml` (applied in the scaffold chunk); `tauri` is not in the lock yet anyway.
  This chunk **confirms** via audit, it does not bump — and confirms **the garde 0.22.1 downgrade introduced
  no new advisory** (the open item carried from the previous session).

## Boundaries
**In scope**
- Declare cargo-audit + cargo-deny as the project's audit tooling and document the invocation on the commands
  surface (aligning with the existing CLAUDE.md Workflow `cargo audit` entry).
- Author `deny.toml` (advisories / bans / licenses / sources policy).
- First full **green** audit run over the committed `Cargo.lock`; triage/record any advisory hit.
- `Cargo.lock` commit + drift discipline (the `--locked` reproducibility check).

**Out of scope (deferred — do not pull in)**
- CI wiring of the gate into GitHub Actions — owned by the later Epoch-1 chunk _"Base CI + agent-run harness
  skeleton"_. This chunk makes the gate runnable + green **locally**; CI invocation is downstream.
- `cargo-geiger` unsafe-FFI mapping — the security plan names it as *recommended*, not the gate itself; pull
  in only if the P2 security extract elevates it.
- Any production code change (logging, redaction, emission primitives) — separate chunks.
- `cargo build --release` / Tauri bundle — Epoch 10.

## Surfaces / contracts touched
- **`deny.toml`** (new, repo root) — the cargo-deny policy config (the one new artifact).
- **`Cargo.lock`** (existing) — asserted committed + un-drifted; may be regenerated `--locked`.
- **`.claude/docs/commands.md`** — the audit commands as part of the documented gate (the CLAUDE.md Workflow
  block already lists `cargo audit`).
- **No crate source** — this is a gate/config chunk, not a code chunk; no `crates/*/src` changes expected.

## Definition of done (intent — the val-1 anchor)
- `cargo audit` runs **green** over the committed `Cargo.lock`.
- `cargo deny check` runs **green** against a committed `deny.toml` (advisories + bans + licenses + sources).
- `Cargo.lock` is committed and `--locked`-clean (no drift).
- The garde **0.22.1** downgrade is confirmed advisory-free (closes the previous session's open item).
