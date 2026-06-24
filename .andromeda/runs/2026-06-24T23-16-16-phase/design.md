# design extract

## Relevance — partial
This chunk implements the design-system **Signature** (paused-count hold-point) as a frontend-only titlebar component. The chunk is UI-scoped and does NOT touch backend, so it covers the visual/motion/a11y aspects of the signature but defers live data wiring to the Channel chunk (Epoch 9 ch4).

## Constraints
1. **Signature freeze + tint behavior** — per design-system §Brand Identity, the count must freeze at the hold value and tint `--count-nominal` → `--count-hold`, never blank or keep running. Absence of motion is the event.
2. **Expression level 0.3 — CSS-only motion** — per design-system §Brand Identity + §Motion, no animation library (framer-motion banned), no spring physics, no parallax. Heartbeat is CSS `@keyframes`/transitions only.
3. **Motion tokens by NAME** — per design-system §Motion, every color/duration is bound to token: `var(--count-nominal)`, `var(--count-hold)`, `var(--count-blocked)`, `var(--motion-micro)` (150ms), `var(--ease-quiet)`. No raw hex/px/ms in component CSS.
4. **Not-color-alone a11y signalling** — per design-system §Iconography (every status paired with text), the hold must flip the titlebar label/phase-line TEXT (e.g. `Conductor · idle` → `HOLD — operator pause`) and carry `aria-live="assertive"`. Tint alone is not the signal.
5. **Abort state motionless + blocked-slate dim** — per design-system §Anti-Patterns (no flashing on Fail), the `aborted`/`blocked` count dims to `--count-blocked` (`#565F89`) and stays motionless.
6. **Reduced-motion override** — per design-system §Motion hard limit, `@media (prefers-reduced-motion: reduce)` already drops all animation/transition globally (tokens.css); heartbeat vanishes but freeze/tint/dim END STATES remain legible.

## Patterns to follow
1. **Heartbeat CSS @keyframes pulse** — a gentle opacity/scale breath on the count while `live` (design-system §Brand Identity); the stillness on hold is the contrast that signals the event.
2. **Titlebar as the signature placement** — per design-system §Component Patterns #1 (Frameless titlebar + Paused-count heartbeat), the count tint + phase-line text flip + label flip form a cohesive signal; no separate status lamp needed here.
3. **Local run-state model (dev-only driver)** — a minimal typed TS state (`idle` · `live` · `hold` · `aborted`/`blocked`) drives the titlebar; the real wiring arrives with the Channel chunk, so this chunk owns the state SHAPE + behavior.
4. **Count renders placeholder + state styling** — the count keeps the `type-data` (tabular-nums) typographic tier and placeholder value; state is expressed via color + motion + text label, not the count value itself changing.

## Anti-patterns to avoid
1. **No animation library (framer-motion / spring physics)** — expression 0.3 hard ban (design-system §Motion); CSS transitions/keyframes only.
2. **No progress bar / hide / animate-to-100% on pause** — design-system Rejected Default; the signature IS the frozen count staying visible + motionless.
3. **No flashing / pulsing / blink on abort/Fail** — design-system hard ban + a11y SC 2.3.1; abort state is motionless slate-violet.

## Contract bindings
**a11y ↔ Motion / Status messages** — the hold signal must carry `aria-live="assertive"` and the text label flip (not color alone); reduced-motion override is already global (tokens.css) so heartbeat + tint animations drop while the freeze/dim + text label stay legible (design-system §Motion + a11y SC 2.3.3 + SC 1.4.1).

## Acceptance criteria contributions
- **(design)** Paused-count run-states render via titlebar: `live` (heartbeat + `--count-nominal`), `hold` (frozen + `--count-hold` amber), `proceed`→`live` (resume), `aborted` (motionless + `--count-blocked` slate) — no animation library; CSS `@keyframes`/transitions only.
- **(design)** Hold is signalled non-color-alone: titlebar label / phase-line text flips and carries `aria-live="assertive"` (design-system §Iconography, a11y SC 1.4.1 + SC 4.1.3).
- **(design)** All color / motion values bound by token NAME (`var(--count-hold)`, `var(--motion-micro)`, `var(--ease-quiet)`) — no raw hex / px / ms in `Titlebar.css` (design-system §Surface: desktop-webview / tokens).
- **(design)** Under `prefers-reduced-motion: reduce` heartbeat drops while freeze/tint/dim + text label remain legible (design-system §Motion).

## Relevant amendment history
- **2026-06-15 design-token-typography-bundle** — token block moved from `@theme` to `:root` (Tailwind v4 tree-shaking fix). Scope introduces NO new tokens, but must honor the token contract via `:root` (no `--motion-*` / `--ease-*` tree-shaking when using `var(--motion-micro)` + `var(--ease-quiet)`). Cascade: scope CSS references existing tokens only.
