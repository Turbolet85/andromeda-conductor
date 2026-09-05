# Security Summary — Conductor

_Distilled from `.andromeda/security-plan.md`. setup-project Phase 3. wrap-session does not modify._

## Posture
Minimal-tier local utility: single-developer, local-only, no-cloud, no-multi-tenancy, no user accounts. The only persisted data is self-generated synthetic test telemetry + run-metadata (`runs.db`, JSONL journals). Zero network exposure — Conductor is a loopback gRPC/MCP client with no inbound listener of its own. Residual risk classes: dependency/supply-chain audit (`bundled` SQLite-from-C + OTLP/gRPC/MCP tree), input validation on operator config + `CONDUCTOR_*` path handles, and error sanitization at the `anyhow` edges + run-report artifacts.

**Tier:** Minimal (0)  **Auth approach:** none (no authenticated surface exists)

## Threat model highlights (vector → control)
- CLI args + the RUST-READ `CONDUCTOR_*` env path handles → `std::fs::canonicalize` + bounds-check at the CLI edge (outside garde). `CONDUCTOR_MSEDGEDRIVER` and `CONDUCTOR_NVDA` are harness-only (read by `wdio.conf.ts`, never by a Conductor binary) → existence + `isFile` + shell-metacharacter rejection at the wdio edge, then an array-form spawn (the driver path as an argv element, the NVDA path as the program of a fixed-argv detached spawn). `CONDUCTOR_E2E_SEED_DIR` is a third class (2026-09-02) — harness-SET, TEST-read, never read by a shipped binary; a repo-relative constant carried in the spawn's env map, guarded by neither rule, a no-op when unset. The `sr*` leg's speech-log ingest is a boundary of its own: untrusted third-party text, bounded and host-path-scrubbed before the evidence record is committed. The sidecar spawn suppresses the child's console window on Windows (`CREATE_NO_WINDOW` under `#[cfg(windows)]`, shipped 2026-09-04), closing the measured host-path disclosure the pane title carried.
- Scenario config files → garde `range` + `#[garde(custom)]` validation at load (the trust boundary); nested spec fields must `dive`, never `skip` — a skipped struct is never descended into, so its rules never run.
- Tauri IPC → deny-by-default capabilities; no `shell-open` with derived strings; no remote-origin iframes.
- MCP child stdout (trusted-child) → preflight gate (version/tools/run-contract terms/canary) + bounded line-delimited JSON-RPC decode (serde_json recursion limit + a per-line size bound); empty canary ⇒ `Blocked` under one of the gate's five named preconditions (a zero-incident corpus names the app-sidecar workspace-key agreement, never a generic string). The channel carries a WRITE as well as read-back — `mark_incident_resolved` — whose applied response and declined JSON-RPC error ride the same path and controls.
- Committed `contracts/` manifests (capability set · load envelope · run contract) → explicit `validate()` at load behind a fixed `default_path()` → `resolve_under`, no `CONDUCTOR_*` override; absent/malformed is a hard harness fault.
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
- **External decay forks on the fault:** a TOOL fault (scanner too old / fixed bug) → raise the floor; an advisory-DATABASE fault (the RustSec DB itself won't parse, so no release can read it) → floor-raising is unexecutable, so it's a bounded wait with the audit↔deny overlap verified green. Prove which by re-running the latest published tool. **A new dependency MAY land during that wait, but only on a `cargo deny check advisories bans licenses sources` verified green over the NEW lockfile** — deny is then the sole coverage, and the deferral stops resting on "no dependency delta".
- **Secret-scanning CI gate** is optional/deferred (Conductor owns no secrets; tool selection left to setup/operator).

---

**Full plan:** `.andromeda/security-plan.md`. Path-scoped rules: `.claude/rules/security.md`.
