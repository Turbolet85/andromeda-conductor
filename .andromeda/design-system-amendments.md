# Design System — Amendments

_Append-only changelog of amendments to `design-system.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-design-token-typography-bundle — §Tokens illustrative CSS `@theme` → `:root`
**Section:** §Surface: desktop-webview / Tokens
**Change:** the token block is now declared on plain `:root` (not `@theme`), with `@import "tailwindcss"` retained for the engine; the light-mode override is `@media (prefers-color-scheme: light) { :root { … } }` (not nested `@theme`). Token NAMES + VALUES are unchanged (the 34-token binding contract). Added a Tailwind-v4 NOTE explaining the reason.
**Why:** Tailwind v4 `@theme`/`@theme static` tree-shakes non-namespace tokens (the chunk observed only 23/34 emit — dropping all `--space-*`, three `--radius-*`, `--motion-micro`, `--ease-quiet`) and forbids `@theme` nested inside `@media`. The shipped `tokens.css` declares all 34 on `:root` (verified: 34/34 emit; Vite's minifier preserves author custom props). Report Deviation #2 / D-design-tokens; user chose the `:root` replacement at the wrap escalation. Cascaded to `.claude/rules/frontend.md`. A generalized `playbook.md` rule was added (spec-illustration → sound-impl reconciliation, when the report proves the invariant = routine).

## 2026-06-24-sanitized-stderr-agent-mode-logging — §cli "Error output" documents the error:/hint: token reuse
**Section:** §Surface: cli / Output structure (5. Error output)
**Change:** noted the `error:` label reuses Fail red (ANSI 203) and `hint:` the Residual mute (ANSI 246), tty-gated on a stderr-specific `IsTerminal` gate (distinct from the stdout gate), the ASCII labels always present (never color-alone).
**Why:** D-design-tokens fired on the new `error:`/`hint:` cli surface. The detector proposed adding a "Hint grey" palette ROW — CORRECTED at validation: ANSI 246 is the EXISTING "Residual mute" token, not a new color, so NO palette entry was added; the error edge REUSES two shipped tokens (203/246) and the invariant holds (token-based, never color-alone — report Coverage tokens design-token✓). Applied as a use-site note on the existing "Error output" entry (the spec-illustration → sound-impl reconciliation routine rule), not a duplicate color. Cascade: design-summary.md / rules/frontend.md carry no per-cli-token detail → no-op.

## 2026-06-24-paused-count-hold-point-signature — §Motion gains a Motion-tokens table (registers --motion-heartbeat)
**Section:** §Motion (calibrated to expression level 0.3)
**Change:** Added a Motion-tokens table (mirroring the §Spacing / §Border-Radius token tables) registering `--motion-micro` (150ms), `--motion-heartbeat` (1600ms), `--ease-quiet`; the heartbeat-period row ties the live count breath to the signature (value-ticking on the count arrives with the Epoch-9 live-counter `Channel`). Token NAMES/VALUES otherwise unchanged.
**Why:** The chunk added `--motion-heartbeat: 1600ms` to `tokens.css` (the heartbeat period — the existing `--motion-micro` 150ms is a flicker, and tokens-by-name forbids a raw ms); user-approved at /andromeda-phase P5 (scope val-1 intent-incomplete). §Motion previously documented motion as prose + a duration-scale expression table with no named-token inventory, so the new token was unregistered (D-design-tokens drift). Cascade: design-summary.md §Spacing&motion motion line gains `--motion-heartbeat`; rules/frontend.md carries no motion-token inventory → no-op.
