# Technology Stack

_Mirrors `.andromeda/architecture.md` §Stack and Technologies. Convenience reference; architecture.md is the primary source. Versions reflect the arch pins + the security-plan required bumps._

## Languages & Runtimes
- **Rust 2024** (cargo 1.85) — primary implementation language; Pulse-consistency mandate.
- **Toolchain ≥ 1.94.1** (`rust-toolchain.toml`) — bumped from arch's MSRV 1.88.0 to clear tar-rs CVE-2026-33056 (security-plan §Dependency Security).
- **tokio 1.48.x** — `current_thread` flavor (zero work-stealing → deterministic emission ordering).

## Core Frameworks
- **No backend/web framework** — Conductor is a gRPC client + MCP client + Tauri IPC host; no HTTP/network service of its own.
- **OTLP emission:** opentelemetry-proto 0.32.0 (`gen-tonic` + trace/metrics/logs) — raw hand-built message structs for byte-level fault control.
- **gRPC transport:** tonic 0.14.6 + tonic-prost 0.14.6 + prost 0.14 (codegen via `tonic-prost-build`).
- **MCP read-back client:** rmcp 1.7.0 (`client`), `serve_client()` over `TokioChildProcess` stdio, protocol `2024-11-05`.
- **Validation:** serde 1.0.x + garde 0.23.0 (`#[derive(Validate)]` range + `#[garde(custom)]` cross-field).
- **Error handling:** thiserror 2.0.18 (per-seam enums) + anyhow 1.0.102 (binary edges).

## Data Storage
- **rusqlite 0.38.0 + libsqlite3-sys 0.38.0** (`bundled` SQLite 3.51.1, JSON1) — synchronous embedded `runs.db` index, raw SQL, no ORM/migrations.
- On-disk artifacts: per-run `<run_id>.jsonl` emission journal + `<run_id>.md` report under `runs/`.

## Messaging & Events
- N/A — single-process modular monolith; in-process Rust calls / `tokio::sync::mpsc` at most. The only queue-like surface is the OTLP gRPC stream to Pulse (the SUT).

## Observability (self-observation — NOT OTel)
- **tracing 0.1.44 + tracing-subscriber 0.3.23** (JSON formatter) — the ONLY self-obs mechanism. No OTel SDK for self-observation (opentelemetry-proto is the PRODUCT fault stream, not self-instrumentation).

## Frontend (desktop-webview GUI — convenience surface)
- **React 19.x** (Vite SPA; Preact 10.x size fallback) + **Tailwind CSS v4.1** (`@theme`, Oxide) + **shadcn/ui** (Radix Primitives) + Lucide React icons.
- **Tauri 2** (≥ 2.10.3 per security-plan) frameless window. Fonts: JetBrains Mono + IBM Plex Sans (self-hosted WOFF2 via Fontsource).
- **cli surface:** clap 4.5 + anstream/anstyle + owo-colors 4.x + indicatif 0.18 + comfy-table 7 + inquire 0.7.

## Development & CI
- **Test:** cargo-nextest 0.9.137 + cargo test --doc; rstest 0.26, proptest 1.9, insta 1.46, assert_cmd 2 + predicates 3, assert_fs 1; coverage cargo-llvm-cov 0.8.7. Webview E2E: @crabnebula/tauri-driver 2.0.9 + WebdriverIO (Linux + xvfb).
- **Lint/format:** clippy + rustfmt. Module-graph audit: cargo-modules / cargo-rail (optional).
- **Supply chain:** cargo-audit 0.22.2 + cargo-deny 0.19.8 (`deny.toml`).
- **A11y:** axe-core 4.12.0 + @axe-core/webdriverio + Lighthouse 13.0.3 + colorjs.io 0.6.1 (operator/local-gated).
- **CI:** GitHub Actions — `cargo build` / nextest / clippy + audit/deny + coverage. Dynamic live-Pulse proof is an operator/local gate, not CI.

## Infrastructure
- Local-only: `cargo build --release` → `conductor-cli` run beside Pulse via `scripts/agent-run.sh`; optional Tauri 2 (v2.10.x) ~3 MB GUI bundle. No Docker/K8s/serverless/cloud.
- Ports: egress OTLP/gRPC to `127.0.0.1:4317` (`:4318` unused); no inbound listener (the `:4317` port-occupier fault is the sole deliberate bind).

## Third-party services
- None — no payment/auth/email/analytics providers (local-only, no accounts).

## Rationale
For why each technology was chosen over alternatives, see `.andromeda/architecture.md` §Established Decisions (each `[Tag]` explains the fork).

## Version updates
1. Update `architecture.md` §Stack first → 2. update `Cargo.toml`/manifest → 3. re-run `/andromeda-setup-project` to refresh this file + rule version refs → 4. verify hooks/tooling → 5. commit `chore: bump {tool} to {version}`.
