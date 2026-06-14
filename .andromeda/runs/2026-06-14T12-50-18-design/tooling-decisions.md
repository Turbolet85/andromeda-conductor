# UI Tooling Decisions — Conductor

## Context (from architecture.md + security-plan.md)

- Product type: Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through 60 capabilities (P-001..P-060) on a deterministic seeded timeline. GUI is "a control surface, not a dashboard"; the headless CLI is the source of truth + release gate.
- Audience: The Pulse developer — solo, local, technical. Single developer-operator; the headless path is agent-driven (primary consumer is an agent parsing artifacts; a human operator watches progress + answers operator-pause prompts).
- Platforms: Desktop (Tauri 2 bundled webview) + headless CLI (`conductor-cli` / `scripts/agent-run.sh`). Detected surfaces: `desktop-webview`, `cli`. No web, no mobile.
- Scale intent: Personal — solo developer, local dev host, no cloud, no multi-tenancy.
- Backend framework: None — no web framework (gRPC client + MCP client + Tauri IPC only). Rust 2024 + tokio 1.48.x `current_thread`. There is no HTTP surface and no SSR.
- Mobile framework: N/A.
- Security tier: Minimal (0).

## Family Selected

**React** — selected by the user during the Phase 1 dialog by switching family away from the research sub-agent's overall recommendation (Svelte). The user then accepted React's full coherent stack for the Tauri 2 webview.

The headless `cli` surface uses a surface-specific Rust terminal-presentation stack (`clap` + `anstream`/`anstyle` + `owo-colors` + `indicatif` + `comfy-table` + `inquire`) — this is not a "framework family" choice (it is the de-facto Rust CLI tooling for the second surface) and was accepted as recommended.

## Q1: Frontend framework

- **Research recommended:** Svelte 5.x as the overall pick; React 19.x recommended within the React family block.
- **User response:** switched family to React; accepted the React family coherent stack.
- **Final answer:** **React 19.x** (Vite, SPA — not SSR). Preact 10.x is the pre-vetted drop-in size fallback if the ~3 MB Tauri artifact / cold-start target is threatened.
- **Reasoning:** User preference for React's ecosystem and component catalog; fits the Tauri 2 bundled-webview SPA model (Vite-native, no SSR). Carried-forward caveat: React's VDOM runtime is the heaviest of the researched families for the ~3 MB artifact — Preact 10 is the documented escape hatch.

## Q2: CSS tooling

- **Research recommended:** Tailwind CSS v4.1 (React family coherent stack).
- **User response:** accepted (part of the React coherent stack).
- **Final answer:** **Tailwind CSS v4.1** (`@tailwindcss/vite`, Oxide engine; static zero-runtime stylesheet bundled offline).
- **Reasoning:** Canonical pairing with shadcn/ui; emits a static offline stylesheet (no CDN), satisfying the Minimal-tier offline hardening for the bundled webview (no CDN-loaded fonts/scripts at runtime).

## Q3: Component library

- **Research recommended:** shadcn/ui (Radix UI primitives) — React family coherent stack.
- **User response:** accepted (part of the React coherent stack).
- **Final answer:** **shadcn/ui** (current CLI; Radix UI Primitives 1.x; MIT; copy-paste, locally vendored).
- **Reasoning:** Copy-paste, locally vendored components with no runtime CDN dependency — vendor only the handful of primitives the control surface needs (scenario/suite picker, start/stop, live-counter surfaces, run-report view, operator-pause dialog). Matches "control surface, not a dashboard" + offline hardening.

## Decisions Log

`2026-06-14` — Initial UI tooling decisions by `/andromeda-design` Phase 1

- Framework: React 19.x (webview) · clap 4.5 + anstream/anstyle (CLI)
- CSS: Tailwind CSS v4.1 (webview) · owo-colors 4.x styling, TTY-gated (CLI)
- Components: shadcn/ui (Radix UI 1.x) (webview) · indicatif 0.18 (live counters) + comfy-table 7 (P-001..P-060 SLO tables) + inquire 0.7 (operator-pause prompts) (CLI)
- Notes: User switched the webview family from the research's overall pick (Svelte) to React, then accepted React's full coherent stack. **Preact 10.x** recorded as the pre-vetted bundle-size fallback. Security constraints carried from security-plan.md: bump **Tauri ≥ 2.10.3**, ship a **deny-by-default capabilities file** (only start/stop · picker · run-report · operator-pause commands + one live-counter `Channel`), no `shell-open` plugin with derived strings, no remote-origin iframes. The **CLI stack is TTY-gated** so agent-captured artifacts stay clean and the headless source-of-truth path is NEVER blocked on an interactive prompt. The second-surface (`cli`) tooling was accepted as recommended — it is surface-specific, not a framework-family choice.
