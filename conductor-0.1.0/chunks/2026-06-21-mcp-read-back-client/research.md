# Codebase Research — 2026-06-21-mcp-read-back-client

## Scope
- **Depth:** moderate (greenfield *for this crate*; deep on the sibling seam pattern + workspace deps) · **Reads:** 5 · **Globs/Greps:** 4 · **code-graph queries:** 1

## Files inspected
- `crates/conductor-verify/Cargo.toml` (full) — **stub**: only `conductor-core.workspace = true`. Needs rmcp + tokio + thiserror + tracing + serde/serde_json deps added, plus dev-deps.
- `crates/conductor-verify/src/lib.rs` (full) — **one-line doc only** (`//! MCP read-back client (rmcp), preflight readiness gate, verdict logic.`). This chunk fills the *client* third; preflight + verdict are later chunks.
- `Cargo.toml` (workspace, full) — `rmcp = "1.7.0"` is ALREADY declared in `[workspace.dependencies]` (line 43) **with no features**; `tokio = "1.48"`, `thiserror = "2.0.18"`, `tracing`, `serde`/`serde_json`, and dev-deps `rstest`/`proptest`/`insta`/`assert_cmd`/`assert_fs`/`predicates` all present. `resolver = "3"`, edition 2024, rust-version 1.94.1.
- `crates/conductor-faults/src/error.rs` (full) — the **`VerifyError` template**: `#[derive(Debug, thiserror::Error)] #[non_exhaustive] pub enum FaultError` with per-variant `#[error("…")]`, `#[source]` chaining for `io::Error` (faults/error.rs:17-22), and one Display unit test per variant.
- `crates/conductor-faults/src/lib.rs` (full) — the **seam layout + re-export pattern**: one module per concept (`mod gap; mod port_occupier; …`) + `mod error;`, `pub use` re-exports, and a crate-doc that cites architecture §sections + the verdict/error wall.

## Graph impact (code-graph query → `tree-query-2026-06-21-mcp-read-back-client.json`)
- **`conductor-verify`** — defines ONLY the crate node (`crate/` at `crates/conductor-verify/src/lib.rs`); **zero** `crate_edges` into or out of it. Greenfield: nothing references it yet, so no callers to break and no downstream blast radius from this chunk's new API.

## Patterns detected
- **Typed error enum** (`crates/conductor-faults/src/error.rs:13-38`): `#[derive(Debug, thiserror::Error)]` + `#[non_exhaustive]` + per-variant `#[error]`, `#[source]`-chained `io::Error`, struct-like variants carrying context (`addr`, `gap`, `min`). `VerifyError` mirrors this exactly.
- **Seam module structure** (`crates/conductor-faults/`): `src/{concept}.rs` modules + `src/error.rs` + `src/lib.rs` re-exports; integration tests in `crates/conductor-faults/tests/{concept}.rs` (e.g. `tests/port_occupier.rs`). Unit tests live in `#[cfg(test)] mod tests` inside each module.
- **Crate-doc convention** (`faults/lib.rs:1-16`): the lib.rs `//!` doc names the seam, lists shipped items, cites architecture §anchors, and states the verdict/error-wall posture.

## Conventions to follow
- **Verdict/error wall** (`faults/error.rs:1-6` doc; CLAUDE.md invariant): `VerifyError` is `Result::Err` = harness fault ONLY (spawn failure, transport down). Protocol-mismatch / tool-absence become typed *values* the later gate chunk maps to `Blocked` — NOT errors here.
- **Workspace-inherited deps** (`Cargo.toml` lines 20-69): add deps as `rmcp = { workspace = true, features = [...] }`, `tokio.workspace = true`, etc. — never re-pin a version in the crate.
- **Testing** (`.claude/rules/testing.md`): per-seam `cargo nextest run -p conductor-verify`; rstest `#[case]` rows for the metacharacter matrix; mock Pulse with an **rmcp stub (in-process duplex + `TokioChildProcess` for the spawn/`.env()` path)** — never fake Pulse's *reaction* as a CI verdict; live leg is local/operator-gated.
- **Obs** (`.claude/rules/observability.md`): bounded span name `verify.readback*` via `#[tracing::instrument]`; instrument the MCP read-back seam with a manual client span (not auto-instrumented); NO OTel SDK; sanitize host paths in error logs.

## New files to create
- `crates/conductor-verify/src/error.rs` — `VerifyError` thiserror enum (spawn-failure, transport, protocol, tool-call, decode variants; `#[non_exhaustive]`).
- `crates/conductor-verify/src/client.rs` (or `readback.rs`) — the rmcp client: hardened `TokioChildProcess` spawn (fixed path + `.env(ANDROMEDA_PULSE_DATA_DIR)` after metacharacter rejection + platform-default resolution), `serve_client()` session, negotiated-version accessor (`peer_info()` → `2024-11-05`), typed `call_tool`/`list_all_tools` wrappers for the 4 tools.
- `crates/conductor-verify/tests/*.rs` — integration tests (rmcp in-process stub session; spawn/`.env` path where a stub child is feasible).

## Files to modify
- `crates/conductor-verify/Cargo.toml` — add `rmcp = { workspace = true, features = ["client", <child-process transport feature>] }` + `tokio`/`thiserror`/`tracing`/`serde`/`serde_json` (workspace), dev-deps `rstest`/`tokio`(test features).
- `crates/conductor-verify/src/lib.rs` — module decls + `pub use` re-exports + crate-doc (replace the stub line).
- `Cargo.lock` — WILL drift when rmcp is first resolved; must be re-committed un-drifted and `cargo audit` + `cargo deny` re-run green (rmcp pulls a large transitive tree — supply-chain checkpoint).

## Open questions
1. **rmcp 1.7.0 feature names** — `client` is confirmed by arch; the child-process transport feature (likely `transport-child-process`) must be verified against rmcp 1.7.0's actual `Cargo.toml` at implement time. (resolve at /andromeda-implement)
2. **Tool-name constants location** — define the 4 tool-name consts (`query_incident_list`, …) in this client chunk (the typed wrappers need them) vs defer to the contracts manifest (next chunk). Recommendation: consts here; the manifest chunk references them. (P4)
3. **Test depth of the spawn path** — in-process rmcp duplex covers session/negotiation/tool-call logic without a child; the `TokioChildProcess` + `.env` hardening path needs a stub child binary. Decide whether the child-spawn leg is exercised here or its pure-logic parts (metacharacter rejection, data-dir resolution) are unit-tested and the full spawn deferred to the preflight chunk. (P4)
