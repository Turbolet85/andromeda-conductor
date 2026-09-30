
## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the three unbuilt keyboard claims shipped and owned; the ring's mechanism restated
**Section:** §1 (surfaces Notes · Critical paths → View run report · triggers → motion-sensitive · harness spec → CI integration) · §3 (Keyboard test harness · CI integration → Command) · §4 → Coverage matrix · §5 (run-console-live · idle-with-report · Per-surface keyboard shortcuts) · §6 Motion tokens · §9 (E2E artifacts row · Pipeline integration) · §10 failure conditions · §11 (Strategy · Visual · Screen Reader)
**Change:**
- §5 retires "No suite asserts … today" (×3). Coverage matrix = a roving row focus through ONE tab stop (`tabindex="0"` + `aria-current="true"` on the current `tr.cov__row`, ArrowUp/ArrowDown ±1 clamped, Home/End); owner of run-console-live row navigation = the driven arm, of idle-with-report = the routine arm (CI).
- The current row is `aria-current` (not `aria-selected`) + a `--border-emphasis` edge, distinct from the `--color-focus` ring; §4's row is table semantics.
- Shortcuts are a modifier map (was "via Radix primitives"): Ctrl+Enter start / proceed, Ctrl+. stop, Escape abort; one in-webview `window` listener on the buttons' own handlers + the dialog's `onKeyDown`; `aria-keyshortcuts` + a visible hint line; single letters, Ctrl+W, Alt+F4 unbound. Owner the driven arm.
- "Virtual-scrolled rows" → every row renders (no virtualization), off-screen rows reached by focus scroll-into-view; the §11 ban kept for any future virtualization.
- "Focus-ring fade-in" dropped from §1 / §6 / §10 / §11: the ring is an untransitioned `outline`.
- §11 Strategy extended: live row navigation ← the routine row-nav spec on the same component; shortcuts ← the routine key-map spec; `inPlace` now checker-enforced; enumeration `11 claims · 9 owned (5 operator-local, carve-out) · 2 n/a-by-construction · 0 recorded gaps`.
**Why:** the chunk built what §5 designed (P4 fork, overseer, founder-delegated) and asserted it — routine arm 16 passing / 2 skipped (expected 2), the driven one-live-run green; the ring mechanism was a spec-to-code reconciliation (the code always drew an outline).
**Kept:** §6's "Formerly this read … focus-ring fade-in" provenance sentence (history); §5 run-console-idle and the HOLD bullets.
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/
