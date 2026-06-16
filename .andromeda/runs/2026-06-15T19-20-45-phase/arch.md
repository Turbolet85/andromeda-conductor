# arch extract

## Relevance
Partial — applies to workspace placement + frontend asset stack pinning; core OTLP/MCP/runtime decisions are deferred to GUI epoch (Epoch 9).

## Constraints
- Code lives in `crates/conductor-tauri/` per workspace boundary rules (arch §Inherited Defaults, §Module Boundaries).
- No Tauri window / frameless config / IPC commands / React components in scope (deferred to Epoch 9; arch §Workspace / Core Structure governs separation of headless CLI from GUI shell).
- Frontend stack MUST pin Tailwind v4.1 + `@tailwindcss/vite` (Oxide engine, static offline) per design-system.md §Surface: desktop-webview (arch §Stack and Technologies acknowledges Tauri 2 + React 19 stacks; this chunk is tokens+fonts bundle only).
- Bundled fonts MUST be zero-CDN (Minimal-tier offline hardening binds arch §Design Philosophy "Headless-drivable core, thin shells" — no runtime fetch surfacing from the frontend layer).
- Workspace discipline: the Tauri bundle remains convenience-only and does NOT alter the CLI release path (arch §Design Philosophy "Headless-drivable core, thin shells" + §Deployment "Local cargo build --release + scripts/agent-run.sh: headless is the source of truth").

## Patterns to follow
- Crate-per-seam Cargo workspace structure: new frontend assets live under `crates/conductor-tauri/` as a peer to the headless core + seam crates, not scattered (arch §Inherited Defaults, §Module Boundaries, directory tree in §Infrastructure Patterns).
- Token names carried verbatim from design-system.md §Tokens (exact match required so downstream Epoch 9 binds `var(--…)` without renaming — this is the standard contract between design-system and implementation layers).
- Stack pinning via `[workspace.dependencies]` in root `Cargo.toml` for any shared Rust deps; npm/Fontsource deps lockfiled (arch §Dependency Security in security-plan.md applies to supply chain).

## Anti-patterns to avoid
- No runtime HTTP/CDN fetches (arch §Design Philosophy "zero-egress" on the network surface; fonts vendored WOFF2 via Fontsource, not Google Fonts CDN).
- No Tauri port / window config / IPC command surface in this chunk (arch §Occupied Resources lists consumed MCP tools + Tauri commands as a locked set; GUI commands are deferred).
- No introduction of network framework / HTTP service (arch §Established Decisions "[Backend Framework] None" — Conductor is a gRPC/MCP *client* only, never a server; Tauri IPC is internal only).

## Contract bindings
Design-system.md ↔ this chunk: token names + typography role scale + color palette are the authoritative reference this chunk makes literal (token names MUST match exactly per scope §Surfaces / contracts touched); security-plan.md ↔ supply chain (npm + Fontsource deps must be pinned + lockfiled, cleared by drift-detector).

## Acceptance criteria contributions
- (arch) Code lives in `crates/conductor-tauri/` per workspace boundary rules (arch §Inherited Defaults).
- (arch) Tailwind v4.1 build stack pinned in `[workspace.dependencies]` (arch §Stack and Technologies, cleared by security drift-detector).
- (arch) No runtime font fetches — vendored WOFF2 only, zero-CDN (arch §Design Philosophy "headless-drivable core, thin shells" + offline hardening).
- (arch) Token names and typography role scale output match design-system.md §Tokens + §Typography exactly (contract binding for Epoch 9 binds).

## Relevant amendment history
2026-06-15-conductor-core-shared-types + 2026-06-15-structured-logging-stack — both prior to this chunk; amended §Stack and Technologies (serde_json 1.0 added, tracing rows confirmed) and §Occupied Resources (CONDUCTOR_SERVICE_NAME / CONDUCTOR_ENV registered). These touches do NOT affect the design-token chunk (different seam: not JSON serialization or obs stack for tokens, only the Tauri frontend asset build). No prior amendments touch `crates/conductor-tauri/` or frontend-scoped decisions (this is Epoch 1 Foundation, so all frontend decisions are novel).
