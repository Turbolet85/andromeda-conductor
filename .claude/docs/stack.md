# Technology Stack

_Mirrors `.andromeda/architecture.md` §Stack and Technologies. Convenience reference; architecture.md is the primary source. Versions reflect the arch pins + the security-plan required bumps._

## Languages & Runtimes
- **Rust 2024** (cargo 1.85) — primary implementation language; Pulse-consistency mandate.
- **Build toolchain 1.95.0 · MSRV 1.94.1** (`rust-toolchain.toml` channel + `[workspace.package].rust-version`) — clears tar-rs CVE-2026-33056 (per arch §Stack + security-plan §Dependency Security).
- **tokio 1.52.3** — `current_thread` flavor (zero work-stealing → deterministic emission ordering).
- **rand_chacha 0.9 (`ChaCha8Rng`) + rand_core 0.9 (`SeedableRng`)** — seedable, platform-stable PRNG; the timeline scheduler's sole non-determinism (seeded per-gap jitter), `seed_from_u64` for cross-platform/version-stable reproducibility.

## Core Frameworks
- **No backend/web framework** — Conductor is a gRPC client + MCP client + Tauri IPC host; no HTTP/network service of its own.
- **OTLP emission:** opentelemetry-proto 0.32.0 (`gen-tonic` + trace/metrics/logs) — raw hand-built message structs for byte-level fault control.
- **gRPC transport:** tonic 0.14.6 + tonic-prost 0.14.6 + prost 0.14 (codegen via `tonic-prost-build`).
- **MCP read-back client:** hand-rolled line-delimited JSON-RPC over tokio child stdio (rmcp removed 2026-06-27 — Pulse's `tools/call` is non-MCP-compliant); protocol `2024-11-05` read from the `initialize` result; raw `serde_json::Value` tool results.
- **Validation:** serde 1.0.x + garde 0.22.1 (`#[derive(Validate)]` range + `#[garde(custom)]` field/cross-field). A cross-field rule that must read a SIBLING field cannot be a garde validator under 0.22.1 (`custom` receives `(&field, &())`) — it ships as a load-path `Scenario::check_*()` called from `from_toml_str`.
- **Serialization:** serde_json 1.0 — canonical-name JSON for the run-report envelope + per-run JSONL journal (serde companion; report-seam runtime dep).
- **Scenario config:** toml 0.9 — declarative TOML scenario files under `scenarios/` (serde-deserialized + garde-validated via `Scenario::from_toml_str`); chosen over JSON for hand-author ergonomics + inline comments (P4 decision); audit/deny-clean.
- **Error handling:** thiserror 2.0.18 (per-seam enums) + anyhow 1.0.104 (binary edges).

## Data Storage
- **blake3 1.8.6** (`blake3 = "1"`) — exception-fingerprint derivation in `conductor-emit`; a NORMAL (non-dev) dep, version-matched to Pulse's own pin because Conductor recomputes the SUT's derivation, not its own
- **rusqlite 0.38.0 + libsqlite3-sys 0.36.0** (`bundled` SQLite 3.50.4, JSON1) — synchronous embedded `runs.db` index, raw SQL, no ORM/migrations.
- On-disk artifacts: per-run `<run_id>.jsonl` emission journal + `<run_id>.md` report under `runs/`.

## Messaging & Events
- N/A — single-process modular monolith; in-process Rust calls / `tokio::sync::mpsc` at most. The only queue-like surface is the OTLP gRPC stream to Pulse (the SUT).

## Observability (self-observation — NOT OTel)
- **tracing 0.1.44 + tracing-subscriber 0.3.23** (JSON formatter) — the ONLY self-obs mechanism. No OTel SDK for self-observation (opentelemetry-proto is the PRODUCT fault stream, not self-instrumentation).

## Frontend (desktop-webview GUI — convenience surface)
- **React 19.x** + **Vite 8.0.16** (`@vitejs/plugin-react`; Preact 10.x size fallback) + **Tailwind CSS v4.1** (Oxide via `@tailwindcss/vite`; tokens on `:root`, not `@theme` — v4 tree-shakes non-namespace tokens) + **shadcn/ui** (Radix Primitives) + Lucide React icons + **`@tauri-apps/api`** (window/IPC client). Package manager **npm** (`package-lock.json` committed, `npm audit --omit=dev` gate); SPA under `crates/conductor-tauri/ui/`.
- **Tauri 2** (≥ 2.10.3 per security-plan; resolves 2.11.x) frameless window (`decorations:false`) via **`tauri-build`** — `generate_context!` resolves `ui/dist` at COMPILE time, so the frontend builds before any workspace cargo compile of `conductor-tauri` (the `ensure_frontend` step in `agent-run.{sh,ps1}` + CI). Fonts: JetBrains Mono + IBM Plex Sans (self-hosted WOFF2 via Fontsource).
- **cli surface:** clap 4.5 + owo-colors 4.x gated by `std::io::IsTerminal` (per stream: stdout and stderr decide independently) + indicatif 0.18 + comfy-table 7 + inquire 0.9.

## Development & CI
- **Test:** cargo-nextest (pinned runner; zero-retry `ci` profile in `.config/nextest.toml`) + `cargo test --doc`; dev-test stack rstest 0.26 · proptest 1.x · insta 1.x · assert_cmd 2 · assert_fs 1 · predicates 3; coverage cargo-llvm-cov (needs the `llvm-tools-preview` toolchain component). External CLI-tool versions are reference floors; `Cargo.lock` is authoritative for crate deps (test-plan §4). Webview E2E: @crabnebula/tauri-driver 2.0.9 + WebdriverIO (Linux + xvfb).
- **Lint/format:** clippy + rustfmt. Module-graph audit: cargo-modules / cargo-rail (optional).
- **Supply chain:** cargo-audit 0.22.2 + cargo-deny 0.19.8 (`deny.toml`) for the Rust tree; `npm audit --omit=dev` (0 production-vuln gate; dev-only test-tooling advisories accepted at dev-tree grain) + committed `package-lock.json` for the `conductor-tauri/ui` frontend tree.
- **A11y:** axe-core 4.12.0 + @axe-core/webdriverio + Lighthouse 13.0.3 + colorjs.io 0.6.1 (operator/local-gated).
- **CI:** GitHub Actions — `cargo build` / nextest / clippy + audit/deny + coverage. Dynamic live-Pulse proof is an operator/local gate, not CI.

## Infrastructure
- Local-only: `cargo build --release` → `conductor-cli` run beside Pulse via `scripts/agent-run.sh`; optional Tauri 2 (2.11.3) ~3 MB GUI bundle. No Docker/K8s/serverless/cloud.
- Ports: egress OTLP/gRPC to `127.0.0.1:4317` (`:4318` unused); no inbound listener (the `:4317` port-occupier fault is the sole deliberate bind).

## Third-party services
- None — no payment/auth/email/analytics providers (local-only, no accounts).

## Rationale
For why each technology was chosen over alternatives, see `.andromeda/architecture.md` §Established Decisions (each `[Tag]` explains the fork).

## Version updates
1. Update `architecture.md` §Stack first → 2. update `Cargo.toml`/manifest → 3. re-run `/andromeda-setup-project` to refresh this file + rule version refs → 4. verify hooks/tooling → 5. commit `chore: bump {tool} to {version}`.
