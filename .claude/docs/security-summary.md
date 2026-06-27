# Security Summary — Conductor

_Distilled from `.andromeda/security-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Posture
Minimal-tier local utility: single-developer, local-only, no-cloud, no-multi-tenancy, no user accounts. The only persisted data is self-generated synthetic test telemetry + run-metadata (`runs.db`, JSONL journals). Zero network exposure — Conductor is a loopback gRPC/MCP client with no inbound listener of its own. Residual risk classes: dependency/supply-chain audit (`bundled` SQLite-from-C + OTLP/gRPC/MCP tree), input validation on operator config + `CONDUCTOR_*` path handles, and error sanitization at the `anyhow` edges + run-report artifacts.

**Tier:** Minimal (0)  **Auth approach:** none (no authenticated surface exists)

## Threat model highlights (vector → control)
- CLI args + `CONDUCTOR_*` env path handles → `std::fs::canonicalize` + bounds-check at the CLI edge (outside garde).
- Scenario config files → garde `range` + `#[garde(custom)]` validation at load (the trust boundary).
- Tauri IPC → deny-by-default capabilities; no `shell-open` with derived strings; no remote-origin iframes.
- MCP child stdout (trusted-child) → preflight gate (version/tools/canary) + bounded prost decode; empty canary ⇒ `Blocked`.
- OTLP/gRPC egress → loopback `:4317` only; refused transport ⇒ `Result::Err` (harness fault), not a verdict.
- Port-occupier `:4317` bind → the SOLE deliberate bind, an intentional in-host fault (P-003), released on cleanup.

## Data classifications
| Class | Examples | Handling |
|---|---|---|
| config (low) | `scenarios/*` + `contracts/` manifest | garde-validated at load; operator-authored local files |
| test-telemetry / run-metadata (low, synthetic) | `runs.db`, `<run_id>.jsonl`, `<run_id>.md` | world-readable local files; no confidentiality requirement |
| credential / PII / payment / health | none owned by Conductor | N/A — Pulse's `corpus.db` (plaintext SQLite, P-049 not active live) is out of scope (SUT-owned) |

## Universal anti-patterns
- No scenario without a Pulse P-ID; no inbound network listener of Conductor's own.
- `Cargo.lock` committed + un-drifted; never `cargo build --release`/merge without `cargo-audit` (+ `cargo-deny`) green.
- Frontend npm tree (`conductor-tauri/ui`): `npm audit --omit=dev` clean (production-dep strict; dev-only test-tooling advisories accepted at dev-tree grain) + `package-lock.json` committed; fonts vendored (no CDN). cargo-audit/deny are Rust-only.
- Never let malformed child/transport input panic — typed `Blocked`/`Fail` via the verdict/error wall.
- Never silently downgrade a failed preflight — distinct `Blocked` state with its precondition.
- Never stamp the journal from tokio's virtual clock; never leak host paths / struct names into artifacts.
- Subprocess hardening: fixed sidecar path + `.env(...)` data-dir, never argv/shell interpolation.
- Never add `unsafe` across the OTLP/gRPC/SQLite FFI boundary without review.

## Path-scoped enforcement
See `.claude/rules/security.md` (loads unconditionally — universal).

## Critical decisions
- **Minimal tier** — no compliance triggers; residual risk is supply-chain + input-validation + error-sanitization only.
- **Subprocess-spawn hardening** is the single most architecture-relevant control (MCP-sidecar STDIO injection, CVE-2026-30623).
- **Version floors:** toolchain ≥1.94.1 (tar-rs CVE-2026-33056) — **done** (1.95.0); `tauri` ≥2.10.3 (origin-confusion CVE-2026-42184) — required bump, dormant until the Tauri GUI (Epoch 9).
- **Secret-scanning CI gate** is optional/deferred (Conductor owns no secrets; tool selection left to setup/operator).

---

**Full plan:** `.andromeda/security-plan.md`. Path-scoped rules: `.claude/rules/security.md`.
