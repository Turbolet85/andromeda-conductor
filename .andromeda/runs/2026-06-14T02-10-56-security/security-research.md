## Stack Vulnerabilities

### Cargo / Rust toolchain — tar-rs symlink chmod (CVE-2026-33056 / RUSTSEC-2026-0033)

- **Version:** Fixed in tar-rs 0.4.45; Rust toolchain fixed in 1.94.1 (project MSRV is 1.88.0)
- **Last release:** 2026-03-26
- **Status:** actively maintained
- **Fits because:** Threat-assessment Tier-0 explicitly names "dependency/supply-chain audit (notably `bundled` SQLite compiled from C source and the OTLP/gRPC/MCP dependency tree)" as the residual risk class. This flaw lets a malicious crate `chmod` arbitrary directories during `cargo build` extraction by following a symlink — it lives in the exact build path the architecture relies on (`cargo build --release` is the release gate, `bundled` SQLite compiles C from source pulling extra build-time crates).
- **Key detail:** `unpack_in` followed symlinks when changing permissions, so a crate tarball with a symlink to `/etc` or `%APPDATA%` re-targets the chmod outside the extraction dir. crates.io deployed an upload block on 2026-03-13 and audited all historical crates (none were exploiting it). Mitigation for Conductor: pin toolchain ≥ 1.94.1 in `rust-toolchain.toml` (currently pinned at MSRV 1.88.0 — bump required), keep `Cargo.lock` committed.
- **Source:** https://blog.rust-lang.org/2026/03/21/cve-2026-33056

### Tauri 2 — Origin Confusion / local-only IPC invocation (CVE-2026-42184)

- **Version:** Affected 2.0 through 2.11.0; fixed in 2.10.3 (architecture pins 2.10.1 → VULNERABLE)
- **Last release:** 2026-06-03
- **Status:** actively maintained
- **Fits because:** The attack surface (threat-assessment §2) lists Tauri IPC as an in-process vector and asserts it is "not network-exposed." This CVE undermines exactly that assumption on the Windows dev host (the architecture's stated platform): on apps using custom protocols, a remote page on an attacker-registered subdomain matching the app's custom scheme can fire an IPC command gated to local origins only. Conductor's optional GUI bin (`conductor-tauri`) uses Tauri 2 commands.
- **Key detail:** Moderate severity, Windows & Android, custom-protocol apps. Bump `tauri` (and the bundler) to ≥ 2.10.3. Note the related earlier class GHSA-57fm-592m-34r7 (iframes bypassing origin checks for IPC even in isolation mode) — Conductor should not embed remote-origin iframes in its bundled webview.
- **Source:** https://github.com/tauri-apps/tauri/security/advisories

### Tauri 2 — Shell plugin RCE via untrusted `open` (CVE-2025-31477)

- **Version:** Fixed in tauri-plugin-shell 2.2.1
- **Last release:** 2025-04-02
- **Status:** actively maintained
- **Fits because:** Conditional — applies only if `conductor-tauri` adds the shell plugin. Conductor's design is a control panel that spawns the Pulse sidecar; if the GUI ever opens paths/URLs via the shell plugin, untrusted input into `open` enables RCE through dangerous protocols (CVSS 9.3 — `file://`/`smb://`/`nfs://` reachable via the system protocol handler when the default scope is improperly validated).
- **Key detail:** 2.2.1 distinguishes an unset scope from an explicitly-disabled scope and fixes the broken protocol allowlist. Mitigation: avoid the shell-`open` plugin in the GUI; if used, set an explicit allow-scope and never pass scenario-config-derived strings to it.
- **Source:** https://github.com/tauri-apps/plugins-workspace/security/advisories/GHSA-c9pr-q8gx-3mgp

### rmcp (MCP Rust SDK) — STDIO transport command/argument injection (CVE-2026-30623 / OX Security design flaw)

- **Version:** No upstream patch — Anthropic classifies as by-design (STDIO execution model); mitigation is caller-side; rmcp current 1.7.0 (architecture's pinned version)
- **Last release:** 2026-04-15
- **Status:** actively maintained
- **Fits because:** This is the single most architecture-relevant finding. Threat-assessment §2 names the "MCP read-back child-process stdout" vector — Conductor spawns `andromeda-pulse-mcp` via `TokioChildProcess` (stdio) and propagates `ANDROMEDA_PULSE_DATA_DIR` into the child's environment/argv. The OX disclosure (part of a 14-CVE, 200k-server sweep) is precisely: configuration values flowing into the subprocess command line of an stdio MCP spawn. Conductor's data-dir handle is operator-supplied via the `CONDUCTOR_*`/`ANDROMEDA_*` env namespace.
- **Key detail:** Anthropic's position: input sanitization between config and the spawn call is the developer's responsibility; allowlists were bypassed via argument injection (e.g. `npx -c`). Mitigation for Conductor: spawn `andromeda-pulse-mcp` as a fixed, hard-coded program path (never an operator-chosen command); treat `ANDROMEDA_PULSE_DATA_DIR` strictly as an env value passed via the `.env(...)` builder (the established Pulse pattern) — never interpolate it into an argv string or a shell; reject path values containing argument-injection metacharacters before spawn.
- **Source:** https://www.ox.security/blog/mcp-supply-chain-advisory-rce-vulnerabilities-across-the-ai-ecosystem/

### rmcp — DNS rebinding in Streamable HTTP server transport (CVE-2026-42559 / GHSA-89vp-x53w-74fx)

- **Version:** Fixed in rmcp ≥ 1.4.0 (architecture pins 1.7.0 → already past the fix; NOT APPLICABLE to Conductor's usage)
- **Last release:** 2026-04-10
- **Status:** actively maintained
- **Fits because:** Documented as a NON-issue for Conductor to prevent a false alarm downstream. The advisory affects only the Streamable HTTP server transport (missing Host-header validation, CVSS 8.8). Conductor is an MCP client over stdio (`serve_client()` / `TokioChildProcess`), runs no HTTP server transport, and is already on 1.7.0 (> 1.4.0). No action required.
- **Key detail:** The fix (released in rmcp 1.4.0) adds Host/Origin header validation to the Streamable HTTP server transport; Conductor never instantiates that transport, so it carries no exposure and needs no change beyond staying ≥ 1.4.0 (it is on 1.7.0).
- **Source:** https://github.com/modelcontextprotocol/rust-sdk/security/advisories/GHSA-89vp-x53w-74fx

### tonic 0.14 / prost 0.14 — protobuf decode DoS lineage (RUSTSEC-2024-0376, RUSTSEC-2020-0002, RUSTSEC-2024-0437)

- **Version:** tonic 0.14.6 + prost 0.14.0 (architecture's pins); no open advisory against the 0.14 line as of 2025-2026
- **Last release:** 2026-05-07
- **Status:** actively maintained
- **Fits because:** Attack surface §2 includes OTLP/gRPC egress and the trusted-child boundary; the architecture hand-builds raw OTLP protobuf and decodes `tonic::Status` from Pulse. The relevant historical class is protobuf-decode DoS: RUSTSEC-2024-0376 (tonic accept-loop DoS, fixed 0.12.3 — Conductor is on 0.14, unaffected), RUSTSEC-2020-0002 (prost stack overflow on crafted input, fixed 0.6.1), and RUSTSEC-2024-0437 (uncontrolled-recursion crash in the `protobuf` crate). None has an open advisory against 0.14, but the pattern (untrusted-length / deep-recursion protobuf → stack/heap exhaustion) is live.
- **Key detail:** Conductor decodes responses from a locally-spawned, same-user child (lower exposure), but the verdict/error wall already treats `tonic::Status` as typed input rather than panicking — keep that. Action: `cargo audit` in CI to catch any future 0.14-line advisory; ensure prost decoding of child responses is bounded (no unbounded recursion on the read-back path). prost 0.14.0 released 2025-06-13.
- **Source:** https://rustsec.org/advisories/RUSTSEC-2024-0376.html

### rusqlite 0.38.0 / libsqlite3-sys 0.38.0 (bundled SQLite 3.51.1)

- **Version:** rusqlite 0.38.0, bundled SQLite 3.51.1 (architecture's pins; current)
- **Last release:** 2025-12-20
- **Status:** actively maintained
- **Fits because:** Threat-assessment §6 explicitly calls out "`bundled` SQLite compiled from C source" as a Tier-0 supply-chain concern, and §2 lists local-file/config parsing. Bundling compiles SQLite's C from source, so its CVEs become Conductor's CVEs and bypass any OS-managed libsqlite. The historical reference class is CVE-2022-35737 (array-bounds overflow on billion-byte string args, fixed 3.39.2) — not applicable to 3.51.1, but illustrates the bundled-C exposure.
- **Key detail:** No known CVE against bundled 3.51.1 in 2025-2026. Because `runs.db` content is self-generated synthetic data (threat-assessment §1, not user input), SQL-injection exposure is near-nil — but the architecture uses raw SQL with no ORM, so still use bound parameters (not string-formatted SQL) for the `run_id`/`seed`/fingerprint-JSON1 writes. Keep `libsqlite3-sys` current so SQLite patch bumps flow in; `cargo audit` tracks the `bundled` C version via the crate advisory.
- **Source:** https://github.com/rusqlite/rusqlite/releases

### serde 1.0.228 + garde 0.23.0 (config validation)

- **Version:** serde 1.0.228 (current), garde 0.23.0 (architecture's pin)
- **Last release:** 2025-09-27
- **Status:** actively maintained
- **Fits because:** Config-file parsing is an explicit attack vector (§2) and garde is the architecture's load-time bounds enforcer (error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99). Tier-0 names "input validation on operator-supplied scenario config and env-var path overrides" as a residual risk this stack must cover.
- **Key detail:** Maintenance caveat — garde's maintainer publicly noted reduced personal investment (specifically the Axum integration) in 2025; the GitHub repo is not archived and the core derive/validation path remains active (garde 0.23.0 released 2026-05-23, 1M+ downloads). Conductor uses only the serde-struct derive + `range`/`custom` cross-field rules (no Axum integration), so the at-risk surface does not apply. Action: keep garde's load-time validation as the trust boundary for all deserialized scenario config; re-evaluate alternatives only if the repo is archived. The env-var path handles (`CONDUCTOR_RUNS_DIR`/`SCENARIOS_DIR`/`CONTRACT_MANIFEST`) are outside garde's struct validation — canonicalize and bounds-check those paths separately at the CLI edge.
- **Source:** https://github.com/jprochazk/garde/releases

---

## Dependency Audit Tools

### cargo-audit (RustSec)

- **Version:** 0.22.2
- **Last release:** 2026-06-05
- **Status:** actively maintained
- **Fits because:** The canonical, ecosystem-standard scanner and the minimum bar for the Tier-0 "dependency/supply-chain audit" residual-risk class. Audits `Cargo.lock` against the RustSec advisory DB and uniquely flags unmaintained crates (no other ecosystem does) — directly covering the OTLP/gRPC/MCP/bundled-SQLite tree the threat assessment calls out.
- **Key detail:** CI integration: add a `cargo install cargo-audit` + `cargo audit` step to the existing `.github/workflows/` (the architecture already runs build/nextest/clippy there). Has an experimental `fix` feature to auto-bump vulnerable requirements. Because Conductor commits `Cargo.lock`, the audit is deterministic.
- **Source:** https://docs.rs/crate/cargo-audit/latest

### cargo-deny (Embark Studios)

- **Version:** 0.19.8
- **Last release:** 2026-05-28
- **Status:** actively maintained
- **Fits because:** Superior superset for this stack — beyond RustSec advisories it gates banned crates, duplicate versions, untrusted sources, and license policy. For a `bundled`-SQLite-from-C, multi-crate OTLP/gRPC/MCP graph (threat-assessment §6), the "multiple versions of the same crate" and "trusted sources" checks add supply-chain assurance cargo-audit alone does not. The recommended 2026 practice is cargo-audit report-only first, then cargo-deny with advisory blocking on high/critical.
- **Key detail:** Single `deny.toml` config; add `cargo deny check advisories bans sources licenses` to GitHub Actions. PR #838 reworked its advisory engine to track RustSec exactly via a nightly cross-check cron. Dual MIT/Apache-2.0 licensed.
- **Source:** https://github.com/EmbarkStudios/cargo-deny/releases

### cargo-auditable

- **Version:** 0.7.4
- **Last release:** 2026-03-04
- **Status:** actively maintained
- **Fits because:** Optional but well-matched: the architecture ships a release binary (`conductor-cli`) as the source of truth. cargo-auditable embeds the dependency list into that binary so the shipped artifact is self-describing for `cargo audit bin <binary>` / SBOM tooling (Syft/Anchore) without needing the source tree — useful when the release binary is run beside Pulse on another dev host.
- **Key detail:** Wrap the release build as `cargo auditable build --release`. Low-cost, no runtime overhead; purely additive metadata. Lower priority than cargo-audit/cargo-deny for a single-developer Tier-0 tool.
- **Source:** https://github.com/rust-secure-code/cargo-auditable

### cargo-geiger (optional — unsafe-code attention map)

- **Version:** 0.13.0
- **Last release:** 2025-08-31
- **Status:** actively maintained
- **Fits because:** Optional visibility tool. Because `bundled` SQLite and the raw-protobuf/tonic FFI path pull `unsafe` (threat-assessment §6 flags the C-source SQLite), cargo-geiger maps where `unsafe` appears across the dependency graph, helping prioritize review of the OTLP/gRPC/SQLite FFI boundary. Not a vulnerability scanner — informational only.
- **Key detail:** Run ad-hoc, not in the merge gate. Lowest priority of the four; include only if unsafe-surface review is wanted.
- **Source:** https://github.com/geiger-rs/cargo-geiger/releases

---

## Framework Security Features

Conductor exposes no HTTP/network server (threat-assessment §2: no public API, no inbound listener; §3: auth "none"; architecture Stack: "Backend Framework: None"). The conventional web-framework security features — CSRF tokens, server-side XSS escaping, SQL-injection-preventing ORM layers, HTTPS/TLS termination, security headers, rate limiting — are therefore not applicable, and adding them would contradict the single-user local model (matching the Minimal tier). The only "framework-equivalent" mitigations that apply are the ones covered below plus the cross-cutting items already named: garde validation at the config boundary, bound SQL parameters in the rusqlite storage seam (despite no ORM), and the MCP preflight readiness gate as the inbound-verification trust boundary.

### Rust language / tokio 1.48.x (memory & concurrency safety as the baseline)

- **Version:** Rust 2024 (toolchain ≥ 1.94.1 recommended; MSRV 1.88.0), tokio 1.48.x
- **Last release:** 2026-03-26
- **Status:** actively maintained
- **Fits because:** The baseline mitigation for Conductor's own-code threat classes. Memory safety (no buffer overflows / use-after-free) and data-race freedom in safe Rust eliminate the entire memory-corruption CVE class; the `current_thread` tokio flavor (architecture decision) additionally removes work-stealing concurrency races by construction; and the verdict/error wall means malformed child/transport input is a typed `Result`/`Status` value, not a panic. This directly covers the in-process Tauri IPC and trusted-child-stdout surfaces from §2 that no web framework guards.
- **Key detail:** Safe Rust gives memory/data-race safety and the typed-error wall for free, but three things must be added by hand: (1) safety of the `unsafe`/FFI boundaries Rust cannot check — the `bundled` SQLite C and raw-protobuf/tonic codegen (mitigate via cargo-audit/cargo-geiger + keeping `libsqlite3-sys` current); (2) input validation on operator-supplied data (garde at config load; explicit path canonicalization for the `CONDUCTOR_*` env handles); (3) subprocess-spawn hardening for the MCP stdio child (fixed program path, env via `.env(...)`, no config-into-argv) — the language gives no protection against the MCP design-flaw command-injection class. Bump the toolchain to ≥ 1.94.1.
- **Source:** https://blog.rust-lang.org/2026/03/21/cve-2026-33056

### Tauri 2 (2.10.3) — built-in app-security model

- **Version:** 2.10.3 (architecture pins 2.10.1 — bump to pick up CVE-2026-42184)
- **Last release:** 2026-06-03
- **Status:** actively maintained
- **Fits because:** Tauri's app-security model is the framework-equivalent control for the one in-process surface Conductor does expose — the webview→backend IPC of the optional `conductor-tauri` GUI (§2). Its capability/permission system (deny-by-default IPC, commands explicitly allowed per-window), configurable webview CSP, the Isolation pattern (sandboxed intermediate iframe that verifies/mangles IPC payloads), and automatic OS-WebView security patching map onto exactly the IPC and origin-confusion risks (CVE-2026-42184) the threat model flags.
- **Key detail:** Built-in for free: per-window capability gating, webview CSP, the Isolation pattern, and OS-WebView auto-patching (2.10.x also hardened iframe API exposure and fs/http plugin scope validation). Must be added by Conductor: (1) upgrade to ≥ 2.10.3 — 2.10.1 is exposed to CVE-2026-42184 (origin confusion on custom protocols, Windows); (2) author a minimal capabilities file allowing only the architecture's actual commands (start/stop, scenario/suite picker, run-report view, operator-pause) plus the one live-counter `Channel`; (3) set a restrictive CSP and do not embed remote-origin iframes (avoids the GHSA-57fm-592m-34r7 iframe-IPC-bypass class); (4) avoid the shell-`open` plugin (CVE-2025-31477) or scope it explicitly. The architecture's "bundled webview, no frontend dev-server port" choice already minimizes this surface.
- **Source:** https://v2.tauri.app/security/