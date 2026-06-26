# design extract

## Relevance
Relevant — the chunk implements the signature Paused-count hold-point via live Channel streaming in the desktop-webview titlebar surface.

## Constraints
- Paused-count is the signature element: count freezes at exact hold value, tints nominal-green → hold-amber over 150ms on pause, dims to blocked slate-violet on abort (per design-system §Brand Identity).
- Expression level 0.3 governs motion: functional only, CSS transitions exclusively, zero animation libraries, in-place state changes settle without overshoot (§Brand Identity).
- Titlebar count display: JetBrains Mono 500, 28px, tabular-nums, rendered in `--count-nominal` (on-timeline) / `--count-hold` (pause) / `--count-blocked` (abort) (§Typography Data tier, §Color Palette Semantic Colors).
- Heartbeat breath animation: 1600ms period (`--motion-heartbeat` token) until halted by operator-pause (§Motion tokens, amended 2026-06-24).
- Motion transitions use `ease-quiet` (cubic-bezier(0,0,0.2,1)) and 150ms micro-interaction duration; all transitions dropped under `prefers-reduced-motion: reduce` (§Motion, binding to a11y SC 2.3.3).
- Titlebar elevation: 1px inset-border seam (`--border-subtle: #2A2E42`), zero drop shadows, flat borders-only depth (§Depth Strategy).

## Patterns to follow
- Titlebar count does NOT hide or animate-to-100% on pause — it STOPS (frozen in place), the signature (§Anti-Patterns Rejected Defaults, §Motion hard limits).
- Status color paired with phase-line text label (e.g., "HOLD — operator pause" in amber) — never color-alone (§Color Palette note, a11y SC 1.4.1).
- All color / typography / motion / spacing values bound to token CSS custom properties (`:root` declaration per amended §Surface desktop-webview Tokens) — zero hardcoded hex / px / ms (§Tokens to cite, amended 2026-06-15).
- Live count breath resumes from the frozen hold value on proceed with green color transition back; on abort color transitions to blocked slate-violet (§Component Patterns 1, §Motion high-impact moments).

## Anti-patterns to avoid
- NEVER hide/blank the count on operator-pause; the freeze (not motion) is the signal (§Anti-Patterns Rejected Defaults, §Motion hard limits).
- NEVER use motion libraries (`framer-motion`), spring physics, or canvas animations at expression 0.3 (§Motion hard limits).
- NEVER use color-alone status signals without paired text/icon (Color-Only a11y SC 1.4.1 rule, §Component Patterns 4).

## Contract bindings
- Motion tokens bind to a11y §Animation SC 2.3.3: reduce-motion override mandatory on all transitions (count-tint, heartbeat breath).
- Count-color state binds to a11y §Use of Color SC 1.4.1: status tint always paired with phase-line label text.
- Live Channel architecture bridges to operator-pause orchestration (later Epoch-9 chunk); Tauri resolver stays non-blocking (headless equivalent).

## Acceptance criteria contributions
- (design) Count display uses token colors only: `--count-nominal` (on-timeline) / `--count-hold` (pause) / `--count-blocked` (abort), never hardcoded hex.
- (design) Count tint transition nominal→hold is 150ms ease-out; all motion respects `prefers-reduced-motion: reduce`.
- (design) Heartbeat breath animation: 1600ms period (`--motion-heartbeat`) until frozen by operator-pause; resumes from frozen value on proceed.
- (design) Phase-line text ("HOLD — operator pause") accompanies count-tint color change (never color-alone per a11y SC 1.4.1).

## Relevant amendment history
- **2026-06-24-paused-count-hold-point-signature:** Motion-tokens table added, registering `--motion-heartbeat: 1600ms` (the heartbeat breath period). Directly relevant: the chunk drives the heartbeat with live Channel data; the token is now formally registered and must be declared in `tokens.css` per Epoch-9 implementation.
- **2026-06-15-design-token-typography-bundle:** Tailwind v4 token declaration corrected to `:root` (not `@theme`) with full 34-token inventory emission verified. Relevant: ensures all color / motion / typography tokens emit correctly for the titlebar count and transitions.