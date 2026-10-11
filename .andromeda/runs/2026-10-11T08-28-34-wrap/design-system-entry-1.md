
## 2026-10-11-conductors-window-retired — the desktop-webview surface and its webview-only sections retired to stubs
**Section:** the preamble · §Brand Identity (expression level) · §Typography · §Spacing · §Depth Strategy · §Border Radius · §Motion · §Iconography · §Surface: desktop-webview · §Surface: cli → Tokens (the ANSI 246 recessive tier) · §Anti-Patterns → Universal Bans · Per-Surface Bans
**Change:**
- The preamble says in plain words that Conductor has no interface: its window retired and the cli surface is all it renders.
- §Typography, §Spacing, §Depth Strategy, §Border Radius, §Motion, §Iconography and §Surface: desktop-webview are each a one- or two-line retirement stub pointing at `321dc8f` (was the webview's faces and font loading, spacing scale, borders-only elevation, radius, CSS motion tokens, icon library and the whole desktop surface); each stub keeps the cli's own fact — terminal monospace, terminal-controlled spacing, no depth or radius, honest-progress motion with the spinner stopping at a hold, ASCII status glyphs.
- The expression level names cli `0.3` alone and drops the CSS-transition clause; the ANSI 246 tier drops "webview binds the same pair as `var(--status-residual)`"; the desktop-webview per-surface bans and the font-family universal ban left; the credential ban names cli output alone.
**Why:** the window crate, its React webview and `tokens.css` left the tree (intent R1). The stub form and the no-interface line were the operator's choice in this wrap's question dialog.
**Kept:** the colour palette and its token names stand — the cli's ANSI map binds them by name; the console design is restated without window or hold by `Run read from the command line`.
**Ref:** .andromeda/runs/2026-10-11T08-28-34-wrap/
