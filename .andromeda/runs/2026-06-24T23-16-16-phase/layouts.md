# layouts extract

## Relevance
Partial — the chunk owns the titlebar count styling/animation, a component within layout-templates §Component-Header / desktop-webview surface; no layout-structure/wireframe change.

## Constraints
- Per layout-templates §Component-Header: titlebar count must render run-state (idle/live/hold/aborted) with color tints `--count-nominal` → `--count-hold` / `--count-blocked` (signature placement).
- Heartbeat animation while `live` is CSS-only (`@keyframes`), NOT a library import (frontend.md hard ban); expression 0.3 functional-motion budget applies. (layout-templates §Wireframe — Run console (HOLD))
- Count must freeze motionless on `hold` and `aborted`; no flashing, no progress-bar animate-to-100%. (layout-templates §Component-Header)
- Titlebar label/phase-line text must flip to convey hold state non-color-alone (e.g., `Conductor · idle` → `HOLD — operator pause`); flip carries `aria-live="assertive"`. (layout-templates §Component-Header + a11y-plan §Motion / Status messages)
- All color/motion tokens referenced by NAME (`var(--count-hold)` etc.), never raw hex/ms/px. (layout-templates IA notes)
- Reduced-motion preference already global (tokens.css); animation drops, color/text end-states remain legible. (layout-templates §Component-Header)

## Patterns to follow
- Count Display role keeps `type-data` role + tabular-nums (layout-templates §Component-Header).
- Motion signal in titlebar pairs text-flip with color tint; the stilled/tinted count is the loudest signal (absence of motion is the event). (layout-templates §Component-Header + Wireframe HOLD)
- Per-state rendering: `idle` (dormant), `live` (heartbeat ticking), `hold` (frozen + tinted), `aborted/blocked` (dimmed, motionless). (layout-templates §Component-Header)
- Footer echoes run-state label (e.g., `conductor · seed 424242 · idle` → `… · HOLD step 14`). (layout-templates §Component-Footer — coordination with titlebar state; NOTE: footer is NOT in this chunk's scope, flagged as later-surface coordination only.)

## Anti-patterns to avoid
- No framer-motion / animation library (CSS `@keyframes` only). (layout-templates §Expression level)
- No flashing, no progress-bar animate-to-100% on pause (the count freezes, not hides or completes). (layout-templates §Expression level)
- No color-alone signalling for hold state; text/glyph + color mandatory. (layout-templates §Component-Header + a11y §Status messages)

## Contract bindings
- **A11y bindings:** aria-live="assertive" on hold-flip (a11y SC 4.1.3 + motion SC 2.3.1); reduced-motion drops animation, color/text remain.
- **Design-system bindings:** `--count-nominal` / `--count-hold` / `--count-blocked` color token semantics; `--motion-micro` / `--ease-quiet` timing by name.
- **Frontend.md bindings:** CSS-only motion (no library); tokens by name; expression 0.3 functional-only.
- **CLI surface mirror:** indicatif spinner stops in place (paused-count primary mirror); titlebar count freeze is webview counterpart (layout-templates §Signature placement) — not changed this chunk, coordination note only.

## Acceptance criteria contributions
- (layouts) Titlebar count renders per run-state: `idle` (dormant), `live` (heartbeat + `--count-nominal`), `hold` (frozen + `--count-hold`), `aborted` (motionless + `--count-blocked`). (layout-templates §Component-Header)
- (layouts) Heartbeat is CSS `@keyframes`/transitions only; no animation library; count motionless in `hold` and `aborted`. (layout-templates §Expression level 0.3)
- (layouts) Hold state signalled non-color-alone: label/phase-line text flips + `aria-live="assertive"`; under reduced-motion animation drops, tint/dim + text remain. (layout-templates §Component-Header + a11y-plan §Motion)
- (layouts) All color/motion values token-bound by NAME; no raw hex/px/ms. (layout-templates IA notes)

## Relevant amendment history
**2026-06-24-frameless-window-shell** (amendment #3): titlebar height reconciled `space-lg` → `space-xl` (32px; to fit 18px Heading phase-line + window-control glyphs). Affects titlebar region layout but NOT the count animation/color behavior this chunk owns; documented as spec-to-implementation reconcile in layout-templates §Component-Header. No impact on signature-animation scope.
