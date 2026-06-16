# Design System — Amendments

_Append-only changelog of amendments to `design-system.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-design-token-typography-bundle — §Tokens illustrative CSS `@theme` → `:root`
**Section:** §Surface: desktop-webview / Tokens
**Change:** the token block is now declared on plain `:root` (not `@theme`), with `@import "tailwindcss"` retained for the engine; the light-mode override is `@media (prefers-color-scheme: light) { :root { … } }` (not nested `@theme`). Token NAMES + VALUES are unchanged (the 34-token binding contract). Added a Tailwind-v4 NOTE explaining the reason.
**Why:** Tailwind v4 `@theme`/`@theme static` tree-shakes non-namespace tokens (the chunk observed only 23/34 emit — dropping all `--space-*`, three `--radius-*`, `--motion-micro`, `--ease-quiet`) and forbids `@theme` nested inside `@media`. The shipped `tokens.css` declares all 34 on `:root` (verified: 34/34 emit; Vite's minifier preserves author custom props). Report Deviation #2 / D-design-tokens; user chose the `:root` replacement at the wrap escalation. Cascaded to `.claude/rules/frontend.md`. A generalized `playbook.md` rule was added (spec-illustration → sound-impl reconciliation, when the report proves the invariant = routine).
