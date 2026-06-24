# Scope — Paused-count hold-point signature

**Marker:** `2026-06-24-paused-count-hold-point-signature`
**Version:** conductor-0.1.0 · **Epoch 9 (Desktop control panel) ch2/10**
**Crate:** `conductor-tauri` UI only (`ui/src/components/Titlebar.{tsx,css}` + a small local run-state) — **zero Rust / engine / seam change expected; no new `#[tauri::command]`, no `Channel`, no capability change**

## What it builds

The design-system **Signature** — the *Paused-count hold-point* — made real in the titlebar. The
frameless-window-shell chunk (ch1) shipped a STATIC titlebar count (`Titlebar.tsx` renders `Conductor · idle`
+ a static `00:00:00` tinted `--count-nominal`; ch1 scope explicitly deferred the freeze/tint/resume heartbeat
to "the NEXT chunk (Epoch 9 ch2)"). This chunk gives that count its run-state behavior.

Per design-system.md §Signature / frontend.md: *"the titlebar count freezes at the hold value, tints
`--count-nominal`→`--count-hold`, resumes on proceed / dims `--count-blocked` on abort. The absence of motion
is the event."* Concretely:

1. **Run-state model (local, not live)** — a small typed TS run-state the titlebar renders from: at minimum
   `idle` · `live` · `hold` (operator-pause active) · `aborted`/`blocked`. Driven by LOCAL React state / props
   THIS chunk (a dev-only state driver so the four states can be exercised + tested) — the real backend wiring
   is the later **Live-counter Channel stream** chunk (Epoch 9 ch4). The exact driver shape (state inside
   `Titlebar` vs a prop from `App` vs a tiny dev toggle) is a P4 decision.

2. **Heartbeat (live)** — a subtle CSS-`@keyframes` pulse on the count while `live` (a gentle opacity/scale
   breath, NOT a flashing or progress-to-100% animation). CSS keyframes/transitions ONLY — NO animation library
   (frontend.md hard ban on framer-motion; expression 0.3 = CSS-only motion).

3. **Freeze + tint (hold)** — on `hold` the heartbeat STOPS (count frozen at the hold value) and the count tints
   `--count-nominal`→`--count-hold` (amber). The stilled count + the tint + the text flip ARE the signal — the
   absence of motion is the event.

4. **Resume + abort-dim** — `proceed` returns to `live`/`--count-nominal` (heartbeat resumes); `aborted` dims the
   count `--count-blocked` (slate) and stays motionless. The abort/Fail state is motionless — no flashing
   (a11y SC 2.3.1 / design hard ban).

5. **A11y signalling (non-color · reduced-motion · status-message)** — the hold is conveyed by MORE than
   color/motion: the titlebar label / phase-line text flips (e.g. `Conductor · idle` → a `HOLD` text state) and
   the flip carries `aria-live="assertive"` (a11y SC 4.1.3). `prefers-reduced-motion: reduce` already drops all
   `animation`/`transition` globally (tokens.css L78–83), so under reduced-motion the heartbeat vanishes while
   the freeze/tint/dim END STATES (static colors + text) remain fully legible. Motion is never the only hold
   signal (a11y §Motion).

## Boundaries (what it does NOT touch)

- **No live Tauri `Channel`** — the backend→frontend emission-counter / target-status stream is the
  **Live-counter Channel stream** chunk (Epoch 9 ch4). This chunk animates a LOCAL run-state; it adds NO
  `Channel`, NO `#[tauri::command]`, NO capability permission. `capabilities/default.json` is unchanged.
- **No Rust / engine / seam change** — `conductor-core`'s operator-pause hold model (`PauseResolver`), the
  `Verdict`/`ReportState` types, the scenario model, journal/envelope, `runs.db`, every seam = untouched. No
  `conductor-tauri` `main.rs` / `Cargo.toml` Rust edit expected (UI-only chunk).
- **Not the count's real VALUE** — what number the count ultimately shows (live emission counters) arrives with
  the Channel chunk; this chunk owns the freeze/tint/resume BEHAVIOR + the state model, rendering whatever
  placeholder value the state carries (today's `00:00:00`-style placeholder is fine).
- **No shadcn/Radix, no Lucide, no new `ui/` runtime dep** — the six status-lamp primitives + dialog scaffold
  are the "Component primitives library" chunk (Epoch 9 ch5). A not-color-alone glyph, if needed, is a
  text/inline character, not a new icon dependency. Any `ui/` dep change ⇒ `npm audit` clean +
  `package-lock.json` committed.
- **No a11y TEST harness** — axe / tauri-driver is the two closing Epoch-9 chunks; this chunk honors the static
  a11y rules (not-color-alone, reduced-motion, aria-live) without standing up the test stack.
- **GUI is convenience, never the release gate** — nothing here touches the headless `agent-run` contract
  (frontend.md). The cli paused-count spinner mirror (isatty-gated-operator-pause) is a separate surface,
  unchanged.

## Surfaces / contracts touched

- **`ui/src/components/Titlebar.tsx`** — render per run-state: heartbeat class while `live`, freeze/tint on
  `hold`, abort-dim; label / phase-line text flip + `aria-live="assertive"` on the HOLD flip; the count keeps
  `type-data` + tabular-nums.
- **`ui/src/components/Titlebar.css`** — heartbeat `@keyframes` + per-state classes binding
  `--count-nominal` / `--count-hold` / `--count-blocked` + `--motion-micro` / `--ease-quiet` by NAME (never raw
  hex/ms; frontend.md). Reduced-motion is already global (tokens.css) — do not re-declare per component unless a
  specific transition needs an explicit `motion-reduce` drop.
- **`ui/src/styles/tokens.css`** — gains EXACTLY ONE new motion token, `--motion-heartbeat` (the heartbeat
  period; the existing set tops out at `--motion-micro` 150ms — a flicker, not a breath — and tokens-by-name
  forbids a raw ms). A small, justified **design-system §Motion** addition that wrap reconciles. _(P4 val-1
  intent-incomplete amendment: the original token enumeration above under-specified the period need; the heartbeat
  genuinely requires a long-period token.)_
- **`ui/src/App.tsx`** (likely) — the local run-state driver feeding the titlebar (dev-only state/toggle), if the
  state is not owned inside `Titlebar` itself (P4 decision).
- **design-system.md §Signature / §Color (status/count tier)** — `--count-nominal`/`--count-hold`/`--count-blocked`
  semantics; expression 0.3 functional-motion-only; "absence of motion is the event."
- **a11y-plan.md §Motion / §Status messages / §Visual** — reduced-motion drop; `aria-live="assertive"` HOLD flip;
  not-color-alone (text + glyph, never tint alone); abort/Fail motionless.
- **frontend.md §Design system / §Hard bans** — CSS-only motion (no framer-motion); no progress bar that
  animates-to-100% on pause (it freezes); no flashing on Fail; tokens by name.
- **layout-templates.md §Component-Header / titlebar** — the titlebar count region within the `banner` strip.

## Acceptance intent (full criteria synthesized in plan.md)

- The titlebar count renders the run-states: `live` (heartbeat + `--count-nominal`), `hold` (frozen +
  `--count-hold`), `proceed`→`live` (resume), `aborted` (motionless + `--count-blocked`); driven by a local
  run-state (no live Channel).
- The heartbeat is CSS `@keyframes`/transitions only — NO animation library added; the count is motionless in
  `hold` and `aborted` (no flashing).
- The hold is signalled non-color-alone: the label / phase-line text flips AND carries `aria-live="assertive"`;
  under `prefers-reduced-motion: reduce` the animation drops while the tint/dim + text stay legible.
- Every color/motion value is bound by token NAME (`var(--count-hold)` etc.) — no raw hex/px/ms in `Titlebar.css`.
- No Rust / seam / capability change; deny-by-default ACL unchanged; no new `#[tauri::command]` / `Channel`.
- Gates green: `ui/` `tsc` (strict — no `any`/`as`) + `vite build` → `ui/dist` · `npm audit` clean
  (+ `package-lock.json` committed only if a dep changed — none expected) · workspace `cargo nextest` / clippy
  `-D warnings` / doctest still green (UI-only change; `conductor-tauri` still compiles against the prebuilt
  `ui/dist`) · `cargo audit` / `cargo deny check` unchanged green · `agent-run.sh run` exit 0.
