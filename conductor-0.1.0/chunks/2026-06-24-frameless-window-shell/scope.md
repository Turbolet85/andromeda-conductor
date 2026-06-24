# Scope — Frameless window shell

**Marker:** `2026-06-24-frameless-window-shell`
**Version:** conductor-0.1.0 · **Epoch 9 (Desktop control panel) ch1/10** (opens the epoch)
**Crate:** `conductor-tauri` (bin) + one mechanical `conductor-core` API touch (`ObsSink`) — **zero engine/seam MODEL change expected**

## What it builds

The first REAL Tauri 2 application — `conductor-tauri` is currently a STUB (`src/main.rs` calls
`conductor_core::init_observability("conductor-tauri", None, ObsSink::Stderr)` and exits; `Cargo.toml` has only
`conductor-core` as a dep; there is NO `tauri.conf.json`, NO `capabilities/`, NO `tauri`/`tauri-build`
dependency, NO `build.rs`). This chunk stands up the app shell so the optional GUI control panel has a window
to live in. Four tied pieces, all over the existing `ui/` Vite + React 19 + Tailwind v4.1 skeleton:

1. **Tauri 2 app scaffold** — add `tauri ≥2.10.3` (security pin, origin-confusion CVE-2026-42184) + `tauri-build`
   to `conductor-tauri/Cargo.toml`; add `build.rs` (`tauri_build::build()`); author `tauri.conf.json`
   (the `build.frontendDist` → `ui/dist`, `build.devUrl` → the Vite dev server, app identifier under the
   reserved `com.andromeda.*`-adjacent namespace); replace the stub `main.rs` with a real `tauri::Builder`
   `.run(...)` over the **core-owned `current_thread`** runtime discipline (Tauri's own runtime is the GUI
   shell's concern; the engine stays runtime-agnostic — architecture §Async Runtime Flavor).

2. **Frameless window** — the main window declared `decorations: false` in `tauri.conf.json` (no native OS
   titlebar/chrome), so the webview owns the entire surface (design-system expression 0.3; frontend.md
   "frameless window").

3. **Drag-region titlebar** — a custom titlebar in the webview carrying `data-tauri-drag-region` (the Tauri 2
   attribute that makes a region draggable in a frameless window) so the window can still be moved; rendered as
   the **`banner` landmark** (a11y.md "Landmarks: … `banner` (frameless titlebar)"). Window controls
   (minimize/close) are out of scope unless trivially required to make the frameless window operable — if
   included they are native `<button>` elements (a11y.md semantic-HTML-first), never `<div role>`.

4. **Deny-by-default capabilities** — a `capabilities/*.json` ACL file with NO blanket permissions; it allows
   ONLY what the frameless drag-region window needs at THIS stage (e.g. `core:window` drag/minimize/close
   permissions for the titlebar). The later-chunk command permissions (start/stop · scenario/suite picker ·
   run-report · operator-pause + the one live-counter `Channel`) are NOT added here — they arrive with their
   commands in subsequent Epoch-9 chunks. The posture (deny-by-default, minimal allowlist) is established now.

**(folded CARRY)** **Tauri backend self-obs file sink** — extend `conductor_core::ObsSink` (today `Stderr` +
the agent-file variant) so the Tauri backend logs its `tracing` JSON to **`logs/conductor-tauri.jsonl`**
(obs-plan §3 sink table: "Tauri backend → `logs/conductor-tauri.jsonl`"). `init_observability(…, sink)` was
parameterized by the `2026-06-24-sanitized-stderr-agent-mode-logging` chunk; `conductor-tauri/main.rs` passes
`ObsSink::Stderr` only as a compile-stub placeholder. This chunk picks the real backend-file sink, reusing the
existing `Arc<Mutex<File>>` `MakeWriter` + processor-stage redaction (NO new redaction policy). Whether to add a
distinct `ObsSink` variant vs parameterize the existing agent-file path is a P4 implementation choice.

## Boundaries (what it does NOT touch)

- **No engine/seam MODEL change** — `Verdict`/`ReportState`, the scenario model, the journal/envelope schema,
  `runs.db`, the timeline/emit/verify/report seams are all untouched. The only non-`conductor-tauri` edit is the
  mechanical `conductor_core::ObsSink` extension (+ its existing cli caller stays compiling) — a sink-selection
  API touch, not a verdict/journal change (mirrors how the agent-mode-logging chunk treated the same surface).
- **NOT the Paused-count hold-point signature** — the titlebar's freeze/tint/resume heartbeat (design-system
  "Signature") is the NEXT chunk (Epoch 9 ch2). This chunk renders a static titlebar shell only.
- **No `#[tauri::command]`s, no `Channel`** — start/stop, scenario/suite picker, run-report view, operator-pause,
  and the live-counter `Channel` stream are later Epoch-9 chunks (each brings its own capability permission).
- **No shadcn/Radix component library, no Lucide, no `@tauri-apps/api` data calls** beyond what the drag region
  needs — the six status-lamp primitives + dialog scaffold are the "Component primitives library" chunk.
- **No a11y harness / axe / tauri-driver** — desktop a11y harness + verification are the two closing Epoch-9
  chunks; this chunk only honors the static a11y rules it can (semantic `banner`, focus ring, not-color-alone)
  without standing up the test stack.
- **Not the cli.** `conductor-cli`'s `--agent-mode` → `logs/agent-latest.jsonl` sink is unchanged; this chunk
  adds the SIBLING Tauri sink, it does not refactor the cli path.
- **GUI is convenience, never the release gate** — nothing here may block or alter the headless `agent-run`
  contract (frontend.md). The cli release gate stands as-is.

## Surfaces / contracts touched

- **frontend.md / design-system.md (expression 0.3)** — Tauri 2 (≥2.10.3) frameless `decorations:false` +
  `data-tauri-drag-region` titlebar; React 19 / Vite 8 / Tailwind v4.1 (Oxide) SPA under `ui/`; design tokens
  on `:root` (`ui/src/styles/tokens.css`) referenced by name; borders-only depth, no drop shadows; cool
  Tokyo-Night `--color-base` ground; no router / breakpoints / browser nav (single window IS the surface).
- **layout-templates.md** — the frameless titlebar surface / `banner` strip layout (desktop).
- **a11y-plan.md §Landmarks** — `banner` titlebar landmark; semantic `<button>` window controls; visible focus
  ring; not-color-alone (no a11y TEST harness yet — static conformance only).
- **security-plan.md §Tauri GUI / §Code Patterns** — deny-by-default `capabilities/*.json` (minimal allowlist);
  `tauri ≥2.10.3`; no `shell-open` with scenario-derived strings; no remote-origin iframes; suppress
  Chromium context-menu/devtools/text-selection on non-text; committed `package-lock.json` + `npm audit` clean
  if `@tauri-apps/api` (or any dep) is added to `ui/`.
- **obs-plan.md §3** — Tauri backend self-obs sink = `logs/conductor-tauri.jsonl`; `service.name` =
  `conductor-tauri` (override `$CONDUCTOR_SERVICE_NAME`); redaction at the processor stage; NO OTel SDK, NO
  browser OTLP exporter (the frontend logs `console.log` JSON only — not in scope to wire here).
- **architecture.md §Async Runtime Flavor / §Real-time Strategy** — core-owned `current_thread` runtime under
  Tauri; Tauri `Channel` is the (later) live-update surface; no HTTP/SSE surface of Conductor's own.
- **`conductor_core::ObsSink` (`crates/conductor-core/src/obs.rs`)** — the one cross-crate API touch; the cli
  caller (`conductor-cli/src/main.rs`) must keep compiling.
- **`scripts/agent-run.{sh,ps1}` + `.github/workflows/ci.yml` — build-order coupling (surfaced at P4, amends
  this scope).** `tauri::generate_context!` resolves `build.frontendDist` (`ui/dist`) at COMPILE time, so the
  frontend must be built (`npm ci && npm run build`) before ANY cargo compile of `conductor-tauri` — including
  `cargo nextest --workspace` / `clippy --workspace` in the harness `run` verb and the CI cargo job. Both gain
  a frontend-build step ahead of the cargo gate. This is build-order plumbing, NOT a cli-logic change and NOT a
  6th harness command — the 5-command discipline + the "Not the cli" boundary below still hold.

## Acceptance intent (full criteria synthesized in plan.md)

- `conductor-tauri` is a real Tauri 2 app: `tauri.conf.json` present with the main window `decorations:false`;
  `tauri ≥2.10.3` + `tauri-build` deps + `build.rs`; the app builds (`cargo build -p conductor-tauri`).
- The webview renders a frameless titlebar with `data-tauri-drag-region` as a `banner` landmark; the window is
  draggable; window controls (if present) are semantic `<button>`s with a visible focus ring.
- A deny-by-default `capabilities/*.json` exists with a minimal allowlist (only the frameless-window
  permissions) — no blanket/wildcard permission; no later-chunk command permissions pre-added.
- The Tauri backend's self-obs `tracing` JSON routes to `logs/conductor-tauri.jsonl` via the extended
  `ObsSink`; redaction stays at the processor stage; the cli sink path is unchanged.
- Gates green: workspace `cargo nextest` (incl. `conductor-cli` unchanged) · doctest · clippy `-D warnings` ·
  `cargo audit` + `cargo deny check` (any new Tauri-tree advisory/license surfaced + justified in `deny.toml`,
  never silently skipped) · `Cargo.lock` re-committed un-drifted · `ui/` `npm audit` clean + `package-lock.json`
  committed if `ui/` deps change · `vite build` produces `ui/dist`.
