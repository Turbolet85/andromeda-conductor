# arch extract

## Relevance — partial

This chunk stands up the Tauri 2 GUI shell for the optional control panel; it touches one architecture surface (the `ObsSink` API for logging) but does not alter the engine model.

## Constraints

1. **Async runtime ownership** — Tauri's own `multi_thread` runtime is the GUI shell's concern; the core-owned `current_thread` deterministic runtime must stay runtime-agnostic per architecture §Async Runtime Flavor, isolated behind seam boundaries.
2. **Module boundaries enforced** — The only non-`conductor-tauri` cross-crate edit is the mechanical `conductor_core::ObsSink` API extension per architecture §Module Boundaries (compiler-enforced workspace seams; conductor-cli caller must keep compiling).
3. **Tauri version pin** — `tauri ≥2.10.3` mandatory (security pin, origin-confusion CVE-2026-42184) per architecture §Stack and Technologies · §Established Decisions [Backend framework].
4. **No engine/seam model change** — `Verdict`/`ReportState`, scenario model, journal/envelope schema, `runs.db`, timeline/emit/verify/report seams untouched per architecture §Established Decisions (shell-only this chunk).
5. **Occupied resources declared** — Any new crate names, env vars, ports, or on-disk artifacts must be registered in architecture §Occupied Resources; ObsSink's `logs/conductor-tauri.jsonl` sink + `conductor-tauri` service name already recorded in amendments-2026-06-24.
6. **Frontend no-inbound-network** — The optional webview is Tauri 2 SPA over React 19 / Vite / Tailwind v4.1 per architecture §Inherited Defaults (Frontend); no remote-origin iframes, no HTTP service of Conductor's own; the CLI `agent-run` headless path remains the authoritative release gate.
7. **Deny-by-default capability ACL** — architecture §Cross-cutting Patterns (Config management) prohibits blanket Tauri permissions; `capabilities/*.json` must explicitly allowlist ONLY the frameless-window permissions needed at this stage.

## Patterns to follow

1. **Core-owned runtime isolation** — The tokio `current_thread` runtime is instantiated and owned by `conductor-core` (headless-drivable per §Design Philosophy); Tauri's multi-thread runtime and the core runtime are layered, not conflated.
2. **File-based self-observation** — The `conductor-core::ObsSink` enum (already parameterized by the agent-mode-logging chunk) routes backend logs to `logs/conductor-tauri.jsonl` (sibling of runs dir per §Occupied Resources) via the existing `Arc<Mutex<File>>` MakeWriter + processor-stage redaction — no new redaction policy, mirrors the CLI path per obs-plan §3.
3. **Tauri builder with explicit config** — `tauri::Builder` must NOT reuse Tauri's runtime as core's; `tauri.conf.json` declares `build.frontendDist` → `ui/dist`, `build.devUrl` → Vite dev server, app identifier under reserved `com.andromeda.*` namespace per architecture §Occupied Resources (Service/process names).
4. **Semantic frameless titlebar** — The custom titlebar carries `data-tauri-drag-region` and is rendered as a `banner` landmark (per layout-templates / a11y-plan, static conformance only at this stage).

## Anti-patterns to avoid

1. **No tokio multi-thread in core** — Never spawn core's work on Tauri's multi-thread runtime or the inverse; the determinism-under-seed invariant (architecture §Established Decisions) requires single-threaded isolated scheduling.
2. **No blanket Tauri permissions** — Do NOT pre-add later-chunk command permissions (start/stop, picker, run-report, operator-pause) or the live-counter `Channel`; deny-by-default — every permission must trace to a concrete seam feature in scope.
3. **No HTTP service of Conductor's own** — Architecture forbids Tauri webview calling back to a Conductor HTTP backend; the IPC `#[tauri::command]` / `Channel` surfaces (and MCP read-back, OTLP egress) are the only Conductor-internal surfaces per §Established Decisions (Backend framework — none).

## Contract bindings

- **obs-plan §3 (logging) ↔ architecture §Occupied Resources** — Tauri backend's `tracing` JSON sink = `logs/conductor-tauri.jsonl`; `service.name` parameterized via `CONDUCTOR_SERVICE_NAME` env or default `conductor-tauri`; redaction at processor stage (no new policy).
- **security-plan §Tauri GUI / §Code Patterns ↔ architecture §Established Decisions** — Tauri ≥2.10.3, deny-by-default `capabilities/*.json`, no shell-open on scenario strings, no remote iframes.
- **a11y-plan §Landmarks ↔ architecture §Conventions** — Frameless titlebar rendered as `banner` semantic landmark with visible focus ring.
- **frontend.md / design-system.md (expression 0.3) ↔ architecture §Inherited Defaults (Frontend)** — React 19 + Vite ≥8.0.16 + Tailwind v4.1 Oxide; design tokens on `:root`; `ui/` npm with committed `package-lock.json` + `npm audit` gate.

## Acceptance criteria contributions

1. **(arch)** Tauri workspace crate `conductor-tauri` compiles with `tauri ≥2.10.3` + `build.rs` (`tauri_build::build()`) per §Stack and Technologies; `cargo build -p conductor-tauri` green.
2. **(arch)** `tauri.conf.json` declares `build.frontendDist → ui/dist` + `build.devUrl → localhost:<vite>` + app identifier in `com.andromeda.*` namespace per §Occupied Resources.
3. **(arch)** No cross-seam forbidden dep — `conductor-cli` + all seam crates compile unchanged per §Module Boundaries; only `conductor-tauri` and core's `ObsSink` API see changes.
4. **(arch)** Capability ACL `capabilities/*.json` exists + declares ONLY frameless-window permissions — no blanket grants, no later-chunk command permissions pre-added per §Cross-cutting Patterns (deny-by-default).
5. **(arch)** `logs/conductor-tauri.jsonl` populated with Tauri backend `tracing` JSON via extended `ObsSink` per obs-plan §3 + §Occupied Resources; `conductor-cli` sink path (`logs/agent-latest.jsonl`) unchanged per amendment-2026-06-24.

## Relevant amendment history

- **2026-06-24-sanitized-stderr-agent-mode-logging** — Registered `CONDUCTOR_AGENT_MODE` env var + `logs/agent-latest.jsonl` artifact; the `ObsSink` extension parameterization (needed by this chunk) was already landed. This chunk picks the real backend-file sink path (vs the stub `ObsSink::Stderr`).
- **2026-06-15-design-token-typography-bundle** — Registered React 19 + Vite 8.0.16 + Tailwind v4.1 (Oxide) + npm frontend stack + `crates/conductor-tauri/ui/` asset subtree in §Stack and §Occupied Resources. Frameless window + titlebar inherit this foundation.
- **2026-06-14-cargo-workspace-scaffold** — Pinned Rust 2024 MSRV 1.94.1, `tracing`/`tracing-subscriber` in workspace deps. Tauri's log integration depends on the already-registered self-observation stack.