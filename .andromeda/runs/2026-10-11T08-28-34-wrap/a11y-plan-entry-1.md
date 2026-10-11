
## 2026-10-11-conductors-window-retired — no assertable surface remains; the webview harness retired to stubs
**Section:** the preamble · §1 · §2 · §3 (intro) · §4 · §5 · §6 · §9 · §10 · §11 · §3 → every key (A11y testing tool pick · WCAG criteria mapping · Structured violation JSON schema · Focus management · Keyboard · Screen reader test pattern · Contrast verification · CI integration · Bootstrap phases) and the index labels
**Change:**
- The preamble says in plain words that Conductor has no interface: its window, and with it the one assertable a11y surface, retired; the cli is not-assertable (no DOM or widget tree).
- §1, §2, §4, §5, §6, §9, §10 and §11 are each a one-line retirement stub pointing at their full text, dated measurements included, at `321dc8f` (was the desktop-webview harness: axe-core / colorjs.io over the tauri-driver / WebdriverIO session, the NVDA screen-reader leg, the `a11y` CI job, `CONDUCTOR_A11Y_STRICT`, the claim-ownership checker, the four must-be-accessible paths). §11 keeps one ban: never treat the cli surface as accessibility-assertable.
- Every §3 key file is a one-line stub and the index carries no labels; §3's Contracts pointer stands.
**Why:** the window crate, its webview harness and its CI job left the tree (intent R1). The form — stub window-only sections, one or two lines, the no-interface line near the top — was the operator's choice in this wrap's question dialog. The dated measurements stay as history at `321dc8f` and in this sidecar's earlier entries.
**Ref:** .andromeda/runs/2026-10-11T08-28-34-wrap/
