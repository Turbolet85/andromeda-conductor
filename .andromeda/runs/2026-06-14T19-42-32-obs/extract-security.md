## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal (0)
- **Justification:** Single-developer, local-only, no-cloud, no-multi-tenancy tool with no user accounts (auth "none"), where the only persisted data is self-generated synthetic test telemetry and run-metadata in an embedded SQLite index and on-disk artifacts (no PII/payment/health/credentials owned by Conductor), and there is zero network exposure (loopback gRPC/MCP client with no inbound listener of its own).

### Logging-Sensitive Vectors

(No logging-sensitive vectors enumerated — Phase 3 will apply default-not-log discipline to High/Critical data classifications.)

### Anti-Patterns Rejected

- **Account enumeration in auth flows** — rejected because: single-user local tool (no user accounts); if an auth/recovery surface is ever added, this ban becomes active immediately.
- **Weak recovery questions** — rejected because: not applicable to Minimal local tool; forward-guardrail if authentication is ever introduced.
- **Skipping rate limiting on auth endpoints** — rejected because: no authenticated surface exists today; if Conductor ever grows an authenticated control surface beyond local IPC, rate limiting is mandatory.
- **Interactive login in headless path** — rejected because: headless `scripts/agent-run.sh` is agent-driven and must remain non-interactive (auth prompt would break the release gate).
- **Path handles without canonicalize + bounds-check** — rejected because: `CONDUCTOR_*` env path handles sit outside garde struct validation and must be validated at the `conductor-cli` edge before any read/write.
- **Interpolating operator-supplied values into argv or shell** — rejected because: `ANDROMEDA_PULSE_DATA_DIR` must be passed strictly via `.env(...)` builder; argument-injection metacharacters must be rejected before spawn (CVE-2026-30623).
- **String concatenation for SQL queries** — rejected because: must use rusqlite bound parameters for `run_id`/`seed`/fingerprint writes to `runs.db`, even though content is self-generated synthetic data.
- **Skipping garde validation on config deserialization** — rejected because: garde's `range` + `#[garde(custom)]` cross-field rules enforce trust boundary (error fraction ∈ [0,1], p50≤p95≤p99); unvalidated `serde` deserialize bypasses this.
- **Unbounded protobuf decoding from child stdout** — rejected because: must enforce bounded recursion on read-back path; empty canary round-trip must surface as `blocked` state, never false pass.
- **Deprecated crypto algorithms** — rejected because: if any fingerprint/hash is added to `runs.db` or journals, must use modern hash algorithms (MD5, SHA-1, DES, RC4, ECB mode forbidden).
- **Copying or decrypting Pulse's encrypted corpus.db** — rejected because: owned and encrypted by Pulse; Conductor only round-trips canary incident through MCP read-back.
- **Persisting secrets to unencrypted runs.db or journals** — rejected because: these store only self-generated synthetic telemetry; treat as world-readable local files.
- **Disabling TLS verification or downgrading to plaintext** — rejected because: today loopback-only, but any promotion beyond loopback must not silently skip transport verification.
- **Logging secrets, tokens, or Authorization headers** — rejected because: Conductor owns no secrets; this ban keeps it that way if future integration introduces them.
- **Exposing stack traces, absolute paths, or internal struct names to operator** — rejected because: sanitize at `anyhow` edge and in run-report artifacts.
- **Leaking absolute host paths in run-report artifacts** — rejected because: `<run_id>.md`, `runs.db` rows, JSONL journals must carry verdict/state/identity fields only; no canonicalized `CONDUCTOR_*` directories or internal seam-crate names.
- **Using virtual clock for journal/report timestamps** — rejected because: must use `std::time::SystemTime`/`Instant`; wrong clock source corrupts ground-truth artifact and breaks journal-relative SLO math.
- **Spawning MCP sidecar via shell with operator-supplied input** — rejected because: fixed program path + `.env(...)` only; no config-into-argv (rmcp STDIO design flaw).
- **Using Tauri shell-open plugin with scenario-derived strings unscoped** — rejected because: if GUI ever opens paths/URLs, must set explicit allow-scope; unscoped plugin enables RCE via dangerous protocols (CVE-2025-31477).
- **Embedding remote-origin iframes in bundled Tauri webview** — rejected because: iframes bypass origin checks for IPC even in isolation mode, combined with origin-confusion CVE (GHSA-57fm-592m-34r7, CVE-2026-42184).
- **Shipping Tauri commands without deny-by-default capabilities file** — rejected because: must author minimal capabilities allowing only actual commands (start/stop, picker, run-report, operator-pause) plus live-counter `Channel`.
- **Pinning rmcp client to strict newer protocol default** — rejected because: must negotiate DOWN to `2024-11-05` (Pulse's hand-rolled server version); strict-newer default is silent-mismatch class the preflight exists to prevent.
- **Following symlinks during crate extraction on pre-1.94.1 toolchain** — rejected because: must bump `rust-toolchain.toml` to ≥ 1.94.1 to clear tar-rs symlink-chmod flaw (CVE-2026-33056 / RUSTSEC-2026-0033).
- **Running release build or merging without cargo-audit green** — rejected because: dependency/supply-chain audit is the Minimal-tier residual-risk control for `bundled`-SQLite-from-C + OTLP/gRPC/MCP tree.
- **Allowing Cargo.lock to drift or go uncommitted** — rejected because: makes `cargo-audit`/`cargo-deny` scan non-deterministic; lets `bundled` SQLite C version float past advisory tracking.
- **Adding scenario without Pulse P-ID or introducing inbound listener** — rejected because: both violate architecture scope law / trust boundary and widen attack surface beyond loopback-client model.
- **Letting malformed child/transport input panic** — rejected because: `tonic::Status` codes and MCP error responses are first-class typed verification inputs; panic on read-back path corrupts run classification.
- **Silently downgrading failed preflight to pass/fail/manual-check** — rejected because: must surface as distinct `blocked` state with named precondition string, never false pass.
- **Adding unsafe code across OTLP/gRPC/SQLite FFI boundary without review** — rejected because: `bundled`-SQLite C and raw-protobuf/tonic codegen are the FFI surface the language cannot check; must keep `libsqlite3-sys` current.
- **Skipping required dependency bumps** — rejected because: `tauri` ≥ 2.10.3 (origin-confusion CVE-2026-42184) and toolchain ≥ 1.94.1 (tar-rs) are required over current pins.

### Data Classifications

- **config** (Sensitivity: Low) — obs handling: OK to log structured; appears in: `scenarios/` directory, deserialized into seam crates.
- **test-telemetry / run-metadata** (Sensitivity: Low) — obs handling: OK to log structured; appears in: `runs/` directory (`<run_id>.jsonl` journals + `<run_id>.md` reports) and `runs.db` index; content is self-generated synthetic OTLP fault data, not user-derived.
- **credential** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A for Conductor (OS keychain access is Pulse-side, exercised via spawned sidecar; Conductor owns no secrets).
- **PII** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no user accounts, no user-supplied content; only synthetic incident/fingerprint data Conductor emits).
- **payment** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no payment/billing SDK, no transactions).
- **health** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no medical data).
