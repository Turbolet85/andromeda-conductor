# Tooling Context — Conductor

Extracted by the orchestrator from `architecture.md` + `security-plan.md` for the Phase 1 UI Tooling Quiz research sub-agent. Two UI surfaces are in scope: `desktop-webview` (Tauri 2 control panel) and `cli` (headless `conductor-cli` / `scripts/agent-run.sh`).

## Project Intent (from architecture.md)

- **product_type:** Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO. The GUI is explicitly "a control surface, not a dashboard": scenario/suite picker, start/stop, live emission counters + target status, run-report view, operator-pause prompts. The GUI is convenience only; the headless CLI is the source of truth and release gate.
- **audience / target_users:** The Pulse developer — solo, local, technical. A single developer-operator running Conductor next to a real Pulse instance on the dev host. No non-technical users. The headless path is **agent-driven** (the primary consumer of CLI output is an agent parsing artifacts; a human operator watches progress and answers operator-pause prompts).
- **platforms:** Desktop (Tauri 2 GUI) + headless CLI. Detected surfaces: `desktop-webview`, `cli`. No web app, no mobile.
- **scale_intent:** Personal — solo developer, local dev host, no cloud, no multi-tenancy. Co-located with Pulse on loopback.
- **growth_model:** Modular monolith — one deployable harness with compiler-enforced crate-per-seam module boundaries.
- **development_style:** agent-driven — headless `scripts/agent-run.sh` is the source of truth and release gate; the GUI is a thin shell over the same core commands.

## Stack (from architecture.md — backend/desktop only; frontend framework is design's to decide)

- **language_runtime:** Rust 2024 (cargo 1.85, MSRV 1.88.0 — security plan requires bump to ≥1.94.1) + tokio 1.48.x `current_thread` (deterministic single-threaded scheduler).
- **backend_framework:** None — no web framework. Conductor exposes no HTTP/network service of its own (gRPC client + MCP client + Tauri IPC only). No REST/GraphQL/tRPC. There is NO web frontend served over HTTP — the only "frontend" is the Tauri 2 bundled webview.
- **mobile_framework:** N/A — desktop-only, host-bound.
- **desktop_shell:** Tauri 2 (bundler v2.10.x; security plan requires ≥2.10.3 for origin-confusion CVE-2026-42184). Bundled webview (no frontend dev-server port). The webview's frontend framework / CSS / component library are UNDECIDED and are exactly what this quiz settles.
- **real_time:** Tauri 2 IPC `Channel` (in-app only) streams live emission counters / target status backend→frontend; no SSE/WebSocket/HTTP polling; no native OS toasts.
- **cli_runtime:** `conductor-cli` is a Rust binary (`#[tokio::main(flavor="current_thread")]` + anyhow at the edge). Any CLI presentation tooling (terminal color, tables, progress, prompts for operator-pause) would be Rust crates, not a JS/web stack.

## Security (from security-plan.md)

- **security_tier:** Minimal (0) — single-developer, local-only, no-cloud tool; no PII/payment/health/credentials owned by Conductor; zero network exposure (loopback gRPC/MCP client, no inbound listener). No compliance triggers.
- **auth_approach:** none — no user accounts, no API consumers, no authenticated surface. Forward-guardrail: NEVER add interactive login to the headless source-of-truth path (it is agent-driven by design). SSR-vs-SPA auth considerations are moot — there is no web/HTTP surface at all.
- **tauri_hardening (informs component/plugin choices):** Tauri commands must ship a deny-by-default capabilities file allowing ONLY the actual commands (start/stop, scenario/suite picker, run-report view, operator-pause) + the one live-counter `Channel`. NEVER use the `shell-open` plugin with derived strings; NEVER embed remote-origin iframes in the bundled webview. Prefer self-contained, local, no-network-dependency frontend tooling (no CDN-loaded fonts/scripts at runtime — the webview is offline/bundled).

## Notes for the researcher

- This is a **Tauri 2 desktop webview**, not a browser web app. Frontend framework options are JS/TS frameworks that bundle into a Tauri webview (Vite-based). SSR frameworks (Next.js/Nuxt/SvelteKit-SSR) are a poor fit — Tauri serves a bundled static SPA, not a Node SSR server. Favor SPA/SSG-friendly, Vite-native, lightweight frameworks; bundle size and cold-start matter for a ~3 MB GUI artifact.
- The audience is one technical developer — no need for a heavyweight enterprise component system; a lean, composable, locally-vendored component approach fits the "control surface, not a dashboard" scope.
- The **second surface is a Rust CLI** — please also recommend a coherent set of Rust terminal-presentation crates (color/style, tables, progress/live counters, interactive prompts for operator-pause) as a distinct grouping, since the headless path is the release gate and its operator-facing output is a designed surface too.
