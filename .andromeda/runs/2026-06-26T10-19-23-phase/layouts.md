# layouts extract

## Relevance
Partial — chunk modifies the titlebar's `runState` state source (from DEV cycler to live Channel) without changing layout structure, wireframe, or component rendering.

## Constraints
1. per §Primary Surfaces: desktop-webview is the sole surface; no mobile/responsive variants
2. per §Wireframe — Run console (idle/live/HOLD): titlebar is `data-tauri-drag-region`, height `space-xl` (32px), border-subtle seam, no shadow
3. per §Component — Header: titlebar flex row with three zones — left (phase line, Heading role) · center (count, Display role, tabular figures) · right (window controls)
4. per §Component — Header: count displays `count-nominal` when ticking (default), freezes at exact hold value in `count-hold` on operator-pause via `motion-micro` `ease-quiet` color transition
5. per §Expression level (this surface): 0.3 budget — color transitions only, no animation library / spring / parallax; the **absence of motion is the signature**
6. per §IA notes: no polling/SSE — one Tauri `Channel` streams live counters + target status backend → frontend

## Patterns to follow
1. per §Component — Header: the count state machine cycles "ticking in place `count-nominal`" (default) → "frozen at hold value `count-hold`" (operator-pause) → "dimmed `count-blocked`" (abort); the stop/freeze is the event
2. per §Component — Header: live-state changes (ticking → frozen) **must be announced to assistive tech** via live-region attribute; count value changes dropped under reduced-motion preference

## Anti-patterns to avoid
1. per §Expression level: no blink / pulse / glow on count state change — only smooth `motion-micro` color tint
2. per §IA notes: no native OS notifications / toasts (those are Pulse behavior Conductor observes); Channel is in-app only

## Contract bindings
- (layout ↔ a11y): the count state flip (ticking → frozen) announcement triggers a live-region attribute (a11y responsibility; layout specifies the data-binding event)
- (layout ↔ desktop-shell): window-control glyphs follow OS convention (Windows standard buttons vs. macOS traffic-light); drag-region pointer must not leak onto controls

## Acceptance criteria contributions
1. "(layouts) Live count renders in titlebar center zone (desktop-webview §Component — Header) sourced from the live Channel backend→frontend stream."
2. "(layouts) Count value + phase line state updates reflect real run execution with no loss of titlebar motion-signature (frozen hold remains motionless, abort dims to `count-blocked`)."
3. "(layouts) Titlebar height maintained at `space-xl` (32px) per §Wireframe / §Component — Header specs."

## Relevant amendment history
- **2026-06-24-frameless-window-shell** — titlebar height reconciliation: `space-lg` (20px) → `space-xl` (32px) actual (the 20px was insufficient for the 18px Heading-tier phase line + 20px window controls; height was already shipped correct, plan reconciled). Directly applies to this chunk's titlebar region.