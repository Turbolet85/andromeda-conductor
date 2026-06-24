# obs extract

## Relevance — partial

This chunk is **UI-only** (`ui/src/components/Titlebar.{tsx,css}` — zero Rust / engine / seam change) and implements run-state visualization (live heartbeat, hold freeze+tint, abort dim) with local React state. The observation scope is narrowed to frontend-only CSS/TS rendering — no new backend-to-frontend Channel, no new `#[tauri::command]`, no Rust instrumentation.

## Constraints
1. **No OTel SDK on frontend** (per obs-plan §3 recursion guard): frontend uses console JSON logging only — no browser OTel JS SDK, no OTLP exporter to `:4318`. This chunk renders *visual states*, not telemetry signals.
2. **Run-state is visual + local** (per obs-plan §3 desktop-webview / scope boundaries): the titlebar count reflects *local* run-state (`idle` / `live` / `hold` / `aborted`), driven by dev-only state this chunk, NOT the live Channel emission-counter stream (Epoch 9 ch4). Placeholder count value acceptable.
3. **A11y signalling is non-color + aria-live** (per obs-plan §6 reduced-motion + scope §5): hold conveyed by text + glyph (not tint alone), `aria-live="assertive"` on label flip, static colors remain under `prefers-reduced-motion: reduce`. CSS-only animation.
4. **No new spans, no new logging** (per obs-plan §4 must-trace / §6 critical paths): this chunk adds NO `#[tracing::instrument]`, no `span` in TS, no telemetry signal — it is a rendering of state, not an instrumented operation. The backend `#[tauri::command]` spans land separately.
5. **Capability / seam unchanged** (per obs-plan §3 + scope: "zero Rust / engine / seam change"): `capabilities/default.json` / `conductor-tauri` `Cargo.toml` / `#[tauri::command]` list untouched. The agent-run contract is unaffected.

## Patterns to follow
1. **Run-state enum + React prop or local state** (per obs-plan §3 surface symmetry): a typed TS `RunState = 'idle' | 'live' | 'hold' | 'aborted'` fed to Titlebar via local state or App prop; mirrors backend `ReportState` / `Verdict` enum semantics for symmetry (not persisted; real source is later Channel).
2. **CSS custom-property binding by token name** (per obs-plan §3 + frontend.md): `--count-nominal`, `--count-hold`, `--count-blocked` referenced as `var(--count-hold)` in `Titlebar.css`, never raw hex/rgb/ms.
3. **CSS `@keyframes` + `animation: none` under reduced-motion** (per obs-plan §6 + tokens.css L78–83): heartbeat is a named `@keyframes`; motion vanishes under reduced-motion, leaving tint + text legible.

## Anti-patterns to avoid
1. **No framer-motion or animation library** (obs-plan §3 zero-new-dep posture / frontend.md): CSS `@keyframes` + `transition` only.
2. **No new browser-side telemetry** (obs-plan §3 recursion guard): no `console.log()` emit of instrumentation; do NOT instrument the state-change itself.
3. **No progress bar animating to 100% on pause** (obs-plan §6 + scope): the hold is a freeze, not progress-to-completion; the count remains motionless + tinted.

## Contract bindings
- **Frontend.md / design-system binding**: Titlebar renders per "absence of motion is the event" (§Signature); CSS-only motion (expression 0.3); no animation library.
- **a11y-plan.md binding**: motion signalling defers to text + glyph (§Visual / §Motion); `aria-live="assertive"` on HOLD text flip; static colors pass contrast under reduced-motion.
- **design-system.md binding**: `--count-nominal`, `--count-hold`, `--count-blocked` color token semantics (§Color / §Signature).
- **No backend binding (yet)**: the live Channel stream + emission-counter live-update are Epoch 9 ch4 (out of scope this chunk).

## Acceptance criteria contributions
1. **(UI) Titlebar render-state complete**: renders run-state visually: `live` (heartbeat + nominal color), `hold` (frozen + amber tint), `proceed` → `live` (resume), `aborted` (motionless + slate dim). (obs-plan §3)
2. **(A11y) Hold signalled non-color-alone**: text label flip carries `aria-live="assertive"`; color alone does not convey hold; text + tint + freeze together are the signal; reduced-motion end-states match (no flashing). (obs-plan §6)
3. **(CSS) Motion via tokens, no raw values**: all `--count-*`, `--motion-micro`, `--ease-quiet` referenced as `var(--*)`; no raw hex / px / ms. `tsc` strict + `vite build` green. (obs-plan §3)
4. **(No telemetry regression)**: zero new `#[tracing::instrument]`, zero new `#[tauri::command]`, zero capability change; `cargo clippy -D warnings` green; `capabilities/default.json` unchanged; `agent-run.sh run` exit 0. (obs-plan §4/§11)

## Relevant amendment history
**(none)** — The amendment history touches structured-logging-stack (foundational §3 schema), redaction (§11), and bounded span names (§11 anti-patterns), none of which affect a UI-only rendering chunk. The heartbeat + run-state rendering are design-system visualization rules, deferred from the backend instrumentation / logging harness.
