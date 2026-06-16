# Security Rules

Universal security requirements (Minimal tier — local-only loopback tool, no network service, no secrets). Applies to all files; loads unconditionally (no `paths:` frontmatter).

**Authoritative source:** `.andromeda/security-plan.md` — §Input Validation, §Dependency Security, §Error Handling, §Security Anti-Patterns.

## Secrets
- Conductor owns NO secrets — never introduce one. Never commit `.env*`, `*.p12`, `*.pem`, `*.cer`.
- No `DATABASE_URL` or cloud-credential env vars — the SQLite path is a local file. Only the non-secret `CONDUCTOR_*` and Pulse-side `ANDROMEDA_*` env handles exist.
- The OS keychain reference (`OsKeychainBackend("com.andromeda.pulse")`) is Pulse-side — never read, store, copy, or embed keychain material; never attempt to decrypt Pulse's `corpus.db`.

## Input validation (§Input Validation)
- Validate ALL scenario config at load with garde (`range` + `#[garde(custom)]` cross-field: error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99, severity-mix sums). An unvalidated serde deserialize bypasses the trust boundary.
- `CONDUCTOR_*` path handles sit OUTSIDE garde — `std::fs::canonicalize` + bounds-check at the `conductor-cli` edge before any `runs.db`/journal write or manifest read (path traversal).
- `runs.db`: rusqlite BOUND parameters only — never `format!`/string-concatenated SQL, even for synthetic data.
- Bound prost/protobuf decode on the MCP read-back path — no unbounded recursion; an empty/malformed canary ⇒ `Blocked`, never a false pass.

## Subprocess & MCP (§Security Anti-Patterns)
- Spawn `andromeda-pulse-mcp` from a FIXED hard-coded program path only — never an operator-chosen command.
- Pass `ANDROMEDA_PULSE_DATA_DIR` strictly via the `.env(...)` builder after rejecting injection metacharacters — never interpolate into argv or a shell (rmcp STDIO injection class, CVE-2026-30623).
- The rmcp client MUST negotiate DOWN to `2024-11-05` (Pulse's hand-rolled server version) — never pin a strict newer default.
- Never silently downgrade a failed preflight (version mismatch / missing tool / empty canary / keychain read-while-write) — surface the distinct `Blocked` state with its named precondition.

## Dependencies (§Dependency Security)
- Audit: `cargo audit` (RustSec) + recommended `cargo deny` (`deny.toml`); build fails on advisory hit.
- `Cargo.lock` committed + un-drifted (makes the audit deterministic). Toolchain ≥1.94.1 (tar-rs CVE-2026-33056); `tauri` ≥2.10.3 (origin-confusion CVE-2026-42184); keep `libsqlite3-sys` current.
- Never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green.
- Frontend (`crates/conductor-tauri/ui`, npm — cargo-audit/deny do NOT cover it): `npm audit` clean (0 vulns) + `package-lock.json` committed before merging frontend changes; fonts vendored (Fontsource WOFF2), no runtime CDN.
- Never add `unsafe` across the OTLP/gRPC/SQLite FFI boundary without review (`cargo-geiger` to map it).

## Error handling (§Error Handling)
- thiserror typed per-seam enums collapse to `anyhow` only at `conductor-cli` / `#[tauri::command]` edges.
- Never leak stack traces, absolute host paths, or internal struct/field names to the operator (cli stderr / Tauri returns) or into run-report artifacts (`<run_id>.md`, `runs.db`, JSONL) — sanitize at the edge + field-allowlist redaction.
- `tonic::Status` codes and MCP error responses are first-class typed verification inputs, never panics (verdict/error wall).

## Tauri GUI (§Code Patterns)
- Deny-by-default capabilities file — allow ONLY start/stop · scenario/suite picker · run-report view · operator-pause + the one live-counter `Channel`.
- No `shell-open` plugin with scenario-derived strings; no remote-origin iframes; Tauri ≥2.10.3.

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._
