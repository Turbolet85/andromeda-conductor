# arch extract

## Relevance — partial

The chunk is UI-only in `conductor-tauri` under React/TypeScript (Titlebar component local state); it does NOT touch the Rust engine, seams, or contracts. Minimal architecture relevance — workspace structure is already established; the chunk is a feature/expression animation on established infrastructure.

## Constraints

- **Inherited runtime stack (frontend):** React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) per architecture §Inherited Defaults (Frontend bullet) — no new frontend runtime deps; `@tauri-apps/api` is already pinned by frameless-window-shell amendment.
- **CSS-only motion (no animation library):** frontend.md hard ban on framer-motion; heartbeat uses CSS `@keyframes`/transitions only per architecture §Inherited Defaults (Frontend) + §Cross-cutting Patterns (zero new deps).
- **Token-by-name binding:** Titlebar.css MUST bind color/timing values by `var(--count-nominal)` / `var(--count-hold)` / `var(--count-blocked)` / `var(--motion-micro)` / `var(--ease-quiet)` — never raw hex/px/ms — per architecture §Conventions (naming patterns "tokens by name").
- **No Rust seam change:** `conductor-core`, verdict/report model, `PauseResolver`, scenarios, journal, `runs.db` remain untouched per architecture §Established Decisions (Core structure — "runtime-agnostic core library") — this chunk drives local React state only.
- **No capability permission change:** `capabilities/default.json` unchanged; deny-by-default ACL static per architecture §Established Decisions (Workspace / Core Structure — headless-drivable + seam-gated permissions) — no `#[tauri::command]` / `Channel` landing-place until Live-counter Channel stream chunk (ch4).
- **No IPC method/surface:** Titlebar renders LOCAL run-state; MCP contracts, OTLP envelope, Tauri command surface all untouched per architecture §Standard Contracts (three pinned surfaces — emit/verify/ui) — the Channel chunk (ch4) is the gateway.
- **A11y color-alone rule:** signalling must use text + tint (not color alone), reduced-motion drop animation while keeping freeze/tint/dim states, `aria-live="assertive"` on the HOLD text flip per architecture §Design Philosophy + a11y-plan §Motion §Visual.

## Patterns to follow

- **Frontend module structure:** UI components live under `crates/conductor-tauri/ui/src/components/` (Titlebar.tsx + Titlebar.css pair) per architecture §Infrastructure Patterns (directory-tree — the optional-GUI asset subtree).
- **Tokens by name:** All design values (colors, durations, easing) are declared in `tokens.css` `:root` and consumed by component CSS via `var(--token-name)` — no inline hex/ms per amendment 2026-06-15-design-token-typography-bundle (frontend asset subtree established).
- **React state for local rendering:** Local `runState` (idle / live / hold / aborted / blocked) is a small TS type housed either inside Titlebar or lifted to App, driven by dev toggle — the Channel chunk (ch4) replaces it with live backend stream per amendment 2026-06-24-frameless-window-shell.

## Anti-patterns to avoid

- **No animation library:** framer-motion, react-spring, or other animation-as-dep forbidden; heartbeat is CSS `@keyframes` only per amendment 2026-06-15-design-token-typography-bundle.
- **No inline styles or hardcoded values:** All colors / durations / easing MUST be `var(--token-name)` — never `#f59e0b` / `200ms` / `cubic-bezier(…)` per §Conventions (Naming patterns).
- **No progress bar animating to 100% on hold:** The count is frozen (stopped), not animated; freeze is the signal per scope.md intent (absence of motion is the event).

## Contract bindings

None explicit — this is a UI-only feature within the already-established Tauri shell. The live Channel stream (ch4) is the next binding point to MCP/emit contracts.

## Acceptance criteria contributions

- (arch) Titlebar.tsx runs on React 19.x + Tailwind v4.1 per §Inherited Defaults (Frontend).
- (arch) All color/motion values in Titlebar.css are bound by `var(--token-name)` per §Conventions (Naming patterns) — no raw hex/ms.
- (arch) No new runtime dep added; `npm audit` clean + `package-lock.json` committed (if any dep changed) per §Stack and Technologies (Desktop frontend row) + §Infrastructure Patterns (Build system — npm audit gate).
- (arch) No `#[tauri::command]` / `Channel` / capability permission change per §Established Decisions (Core structure — headless-drivable seam-gating) — local React state only.
- (arch) `ui/` `tsc --strict` + `vite build` → `ui/dist` + workspace `cargo nextest` / clippy still green (UI-only change; `conductor-tauri` compiles against prebuilt `ui/dist` per amendment 2026-06-24-frameless-window-shell (generate_context! build coupling).

## Relevant amendment history

- **2026-06-15-design-token-typography-bundle:** Established the React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) frontend stack and `crates/conductor-tauri/ui/` asset subtree (npm with committed `package-lock.json` + `npm audit` gate). **Applies:** token naming, motion-only CSS, no animation libraries.
- **2026-06-24-frameless-window-shell:** Registered `logs/conductor-tauri.jsonl`; documented `tauri-build` / `generate_context!` compile-time frontend-before-cargo coupling (affects `agent-run.{sh,ps1}` + CI). **Applies:** `conductor-tauri` bin compiles against prebuilt `ui/dist`.
- **2026-06-23-line-oriented-output-rendering:** Registered owo-colors/indicatif/comfy-table rendering libs. **Cascade:** tokens-by-name rule extended to frontend CSS (all color/motion via `var(--token-name)`).
