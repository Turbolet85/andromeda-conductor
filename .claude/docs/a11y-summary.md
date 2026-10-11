# Accessibility Summary — Conductor

_Distilled from `.andromeda/a11y-plan.md`. setup-project Phase 3. wrap-session does not modify._

## A11y tier
**Tier:** Minimal (0). **No assertable surface remains** — Conductor has no interface since 2026-10-11, when its window
(`conductor-tauri` and its React webview) retired (`2026-10-11-conductors-window-retired`, intent R1).

## Harness contract (§3)
Retired with the window: every §3 key is a one-line stub, its full text — dated measurements included — at `321dc8f`.
No axe-core / Lighthouse / colorjs.io run, no tauri-driver / WebdriverIO session, no NVDA leg, no `a11y` CI job and no
`CONDUCTOR_A11Y_STRICT` remain.

## The one surface
- **cli:** not-assertable — no DOM or widget tree. Its ASCII status prefixes (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/…)
  and NO_COLOR text pairing are output-stream discipline, never an a11y verification path.

## Universal anti-patterns
- Never treat the cli surface as accessibility-assertable.
- Never convey state by color alone — every status carries its text prefix (the cli rule, design-system §Surface: cli).

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rule `.claude/rules/a11y.md` is a retirement stub.
