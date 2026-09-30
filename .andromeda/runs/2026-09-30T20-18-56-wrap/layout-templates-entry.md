
## 2026-09-30-the-screen-reader-content-findings-fixed — the footer status strip shipped; the phase line is the h1
**Section:** §Component — Footer (status strip) · the footer rows of the Run console (idle), Run console (HOLD) and Idle-with-a-report wireframes · §Component — Operator-checklist · §Component — Header (frameless titlebar + Paused-count heartbeat)
**Change:**
- Footer: was "Designed, NOT shipped (as of 2026-09-02)", landmark set `banner` + `main` + two `region`s, "height `space-sm`", phase `idle` / `HOLD step 14` / `run complete`, each count token tinted; now SHIPPED 2026-09-30 (`footer.footer`, the `contentinfo` landmark; set `banner` + `main` + the `region`s + `contentinfo`), `--space-xs` block padding, ONE text line with plain-text ` · ` separators, not focusable, not live; `conductor` · `seed <n>` (`—` with no report) · the `RunState` word (`idle` · `live` · `HOLD` · `aborted`, no step index) · one `{n}` + `StatusLamp` per non-zero lamp in `LAMP_ORDER`, the glyph tinted and the label text; the HOLD accent an `aria-hidden` `count-hold` glyph, never a text tint. The unticked-ManualCheck count stays DESIGNED, NOT SHIPPED.
- Wireframes: the three footer rows marked shipped (HOLD row `● HOLD`, roll-up row with seed and run state first).
- Operator-checklist: "the window footer strip is unbuilt" → shipped without the unticked count.
- Header: the phase line renders as the window's single `h1` (was a `span`), drag region and `aria-live` switch kept.
**Why:** this chunk shipped the strip and the heading the masters named route-owned; an 8px `space-sm` box cannot hold a 12px Label line, and `count-hold` as 12px text on the light `color-base` computes ≈4.47:1 (under SC 1.4.3's 4.5:1); separate items with no whitespace read as "conductorseed —idle" to NVDA.
**Ref:** .andromeda/runs/2026-09-30T20-18-56-wrap/
