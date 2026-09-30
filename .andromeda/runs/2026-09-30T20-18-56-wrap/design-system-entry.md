
## 2026-09-30-the-screen-reader-content-findings-fixed — the window footer strip ships without the unticked count
**Section:** §Surface: desktop-webview → Component Patterns 7 (Operator-checklist)
**Change:** was "the window footer strip is unbuilt as of 2026-09-02"; now the strip SHIPPED 2026-09-30 as the single-line `contentinfo` status strip (seed · run state · a count per non-zero lamp) and does NOT carry the unticked count, so the checklist view's `role=status` roll-up stays that count's only surface.
**Why:** this chunk shipped the footer strip the masters named route-owned-not-shipped; the unticked count rides the report-site checklist render, still designed and unbuilt.
**Ref:** .andromeda/runs/2026-09-30T20-18-56-wrap/
