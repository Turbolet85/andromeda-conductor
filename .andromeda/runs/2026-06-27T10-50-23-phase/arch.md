# arch extract

## Relevance
Partial — frontend tooling / test-harness layer in scope; core engine architecture out of scope.

## Constraints

- **(arch §Design Philosophy) Headless-drivable core, thin shells:** tauri::test mock-runtime tests ONLY exercise existing commands (start_run/stop_run/run_report/resolve_operator_hold) and existing Channels (live-counter, HoldPrompt); no new command surface or engine logic.
- **(arch §Stack and Technologies, Desktop frontend row) Frontend stack locked:** React 19.x + Vite 8.0.16 + Tailwind v4.1 + Fontsource WOFF2, npm with committed package-lock.json. New a11y harness devDependencies (axe, lighthouse, colorjs.io, tauri-driver glue) must pass npm audit clean.
- **(arch §Occupied Resources, Frontend asset subtree) ui/ tree governance:** crates/conductor-tauri/ui/ is not a Cargo member; node_modules/ + dist/ git-ignored, package-lock.json committed (parallel gate to cargo-audit).
- **(arch §Infrastructure Patterns, Build system + frameless-window-shell amendment) Generate_context! build-order:** tauri-build resolves ui/dist at compile time; npm run build MUST complete before conductor-tauri cargo compile. The ensure_frontend step is wired into scripts/agent-run + CI.
- **(arch §Inherited Defaults, Frontend) TypeScript strict + design-token + self-hosted fonts:** tokens on :root (Tailwind v4 @theme), fonts WOFF2 (no CDN), TypeScript strict mode (implicit from prior frontend work).
- **(arch §Standard Contracts, Conventions) No new Tauri command surface:** tauri::test exercisable surface is fixed; the three CARRY batches of deferred tests land on start_run/stop_run/run_report/resolve_operator_hold + the two Channel types, no new contract.

## Patterns to follow

- **(arch §Design Philosophy) Headless is the release gate, GUI is convenience:** The CLI path (scripts/agent-run.sh) is source of truth; tauri::test smoke-checks the thin shell's integration with the same core, never replaces the headless gate.
- **(arch §Cross-cutting Patterns, Development Style: agent-driven) Headless-first verification:** GUI integration tests verify the thin shell works alongside the headless discipline.
- **(arch §Infrastructure Patterns, §Occupied Resources, design-token-typography-bundle amendment) Frontend supply-chain parity:** npm audit clean + package-lock.json committed mirrors cargo audit + Cargo.lock for the Rust side (test-plan §Supply-chain gates).
- **(arch §Stack and Technologies, tauri-driver / axe / Lighthouse / colorjs.io) A11y harness tooling locked:** Per a11y-plan §3, the harness choice (tauri-driver WebDriver + axe-core WCAG + Lighthouse category + colorjs.io token-pair contrast) is authoritative and does not migrate mid-project.

## Anti-patterns to avoid

- **(arch Boundaries) Zero engine/seam model change:** conductor-core/timeline/emit/faults/verify/report/run remain untouched; tauri::test is GUI-integration-only.
- **(arch §Established Decisions, Backend framework) No HTTP service:** tauri-driver is a WebDriver *client*, not a server; no Conductor-side HTTP listener.
- **(arch §Cross-cutting Patterns, Scope law) No scenario authoring, no process management, no live-Pulse tests:** Harness + mock-runtime only. Live a11y verification sweep (incl. live-Pulse checks) is the next chunk.

## Contract bindings

- **test-plan Path 7 cross-surface parity** (Tauri mock-runtime ↔ CLI subprocess, identical runs.db envelope for same seed): tauri::test start_run/stop_run must produce the same run-record shape as CLI (per test-plan §3 / §Paths).
- **a11y-plan §3** (a11y harness, tooling + thresholds): harness choice (tauri-driver, axe, Lighthouse, colorjs.io) + WCAG AA contrast floor + axe rule-set are authoritative here.
- **supply-chain gates** (npm audit + cargo audit + cargo deny): frontend + Rust test deps must pass all gates together (per design-token-typography-bundle amendment).

## Acceptance criteria contributions

1. **(arch) Frontend asset subtree respects generate_context! build-order:** npm run build completes, then ui/dist is bundled into conductor-tauri before any Rust compilation (scripts/agent-run.{sh,ps1} enforce ensure_frontend step per §Infrastructure Patterns Build system).
2. **(arch) npm audit 0 on all a11y harness devDependencies:** axe, lighthouse, colorjs.io, tauri-driver glue pass audit clean; package-lock.json committed (per §Inherited Defaults Frontend).
3. **(arch) tauri::test mock-runtime exercises EXISTING surface only:** start_run/stop_run/run_report/resolve_operator_hold + live-counter Channel + HoldPrompt Channel; zero new Tauri commands or event types (per §Design Philosophy "Headless-drivable core").
4. **(arch) Path 7 hermetic parity test lands and passes:** Tauri mock-runtime start_run/stop_run produces identical runs.db envelope as CLI subprocess for one fixed seed (per test-plan Path 7 + scope "Acceptance intent").

## Relevant amendment history

- **2026-06-15-design-token-typography-bundle:** Registered the frontend stack (React 19.x, Vite 8.0.16, Tailwind v4.1, Fontsource WOFF2, npm with committed package-lock.json) and the ui/ asset subtree (not a Cargo member; node_modules/ + dist/ git-ignored). This chunk lands a11y harness tooling into that locked stack and tree structure. *Why:* prior chunk established frontend scaffold; this chunk inherits the locked stack and ui/ governance.
- **2026-06-24-frameless-window-shell:** Registered the build-order constraint: tauri-build's `generate_context!` resolves ui/dist at COMPILE time, requiring npm run build before conductor-tauri cargo compile; ensure_frontend step wired into scripts/agent-run + CI. This chunk's harness and test infrastructure must respect the ordering (npm build → cargo compile of conductor-tauri). *Why:* prior Tauri shell chunk locked the build coupling; this chunk adds test code that compiles in the same pipeline.
- **2026-06-26-live-counter-channel-stream:** Registered conductor-run library crate (9th workspace member, shared root library for both conductor-cli + conductor-tauri). This chunk's tauri::test fixtures exercise the run composition from conductor-run (preflight + execute_scenario + persist + drive_run). *Why:* prior chunk extracted pipeline into conductor-run; this chunk uses conductor-run as the core dependency for its mock-runtime test harness.