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
- 2026-06-23: A green `cargo audit` does NOT imply a green `cargo deny check`. `cargo audit` exits 0 on `unmaintained`/license advisories (it reports them as *allowed warnings*), whereas `cargo deny` (deny.toml schema-v2 defaults) **denies** unmaintained advisories AND any non-allowlisted license. So the supply-chain gate MUST run BOTH (CI's `ci.yml` has separate `cargo audit` + `cargo deny check` steps); a chunk that checks only `cargo audit` can leave `cargo deny` latently red — this chunk found `foldhash 0.2.0`'s `Zlib` license had been failing `cargo deny check licenses` on HEAD, unnoticed because the prior session ran only audit. When a NEW dep pulls a non-actionable item, extend `deny.toml` with a JUSTIFIED entry (never a silent skip — the file mandates a comment): `[advisories] ignore` for an unmaintained-but-non-vulnerable transitive with no upstream fix, `[licenses] allow` for a permissive (OSI+FSF) license. (line-oriented-output-rendering chunk)
- 2026-06-24: Standing up the Tauri 2 tree (`tauri`/`wry`/`tao`/`webview2-com`/gtk) trips `cargo deny check` on BOTH advisories AND licenses, all non-actionable: ~16 `unmaintained` advisories (10 gtk-rs GTK3 = the Linux webview path, never compiled on the Windows target; 5 `unic-*` via `urlpattern`→`tauri-utils`; 1 `proc-macro-error`) + the `MPL-2.0` and `Apache-2.0 WITH LLVM-exception` licenses. None is a vulnerability and none has a safe upgrade (transitive Tauri deps), so each is a JUSTIFIED `deny.toml` entry per the rule above; `cargo audit` stays exit 0 (it reports unmaintained as warnings). Expect the same on any future Tauri bump/plugin — extend `deny.toml`, never silent-skip. (frameless-window-shell chunk)
- 2026-06-26: Tauri 2 app-defined `#[tauri::command]`s (registered via `invoke_handler` + `generate_handler!`) are NOT gated by the capability ACL — only `core:`/plugin permissions are listed in `capabilities/*.json`. So adding Conductor's own commands (`list_scenarios`/`start_run`/`stop_run`) needs NO new permission entry, and the deny-by-default posture holds with the existing window-only allowlist unchanged (the picker/start-stop perms the ACL description once anticipated are a no-op for app commands). Only edit `capabilities/*.json` when a command transitively needs a NEW core/plugin permission (a future shell/fs/dialog plugin). Do NOT reflexively add per-command allowlist entries — they are not part of the app-command security model; the real boundary is validating the command's input (e.g. `start_run` runs `conductor_core::validate_selection` against the catalog before use, and resolves dirs via `resolve_under`). (scenario-suite-picker-start-stop chunk)
