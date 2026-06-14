## Expression Level

### Base level

**Recommended base:** 0.3

**Reasoning:** Product type pins this low — Conductor is a developer tool / B2B-utility control surface explicitly framed as "a control surface, not a dashboard," and the dev-tool band caps typically at ≤ 0.4. The audience is a single solo, local, technical Pulse-developer (no non-technical users), and power users / developers prefer restrained, non-distracting motion; combined with the "Mission-control patience — armed, nominal, go/no-go" personality whose entire voice is "calm and resolute under load, terse status callouts, no alarm even on Fail," anything expressive would contradict the brand. Although security_tier is Minimal (0) — which would *permit* more polish — scale_intent is Personal solo-operator and the design spine is "exact downbeat, held silence" (restrained type-and-grid timing, not animation), so 0.3 (subtle hover/fades, the Linear/Raycast band) is the ceiling, not a compromise.

### Per-surface adjustments

| Surface | Expression | Delta vs base | Reasoning |
|---|---|---|---|
| desktop-webview | 0.3 | +0.0 | Inherits the web scale (Tauri 2 webview); Linear is the literal webview anchor and sits at exactly 0.3 — subtle hover, fades, in-place status-light color changes (green/amber/red) over a flat inset-border dark ground, no staggered entrances or parallax. The ~3 MB artifact + "control surface, not a dashboard" mandate forbids framer-motion-class bloat; state changes (counter ticks, verdict lamps) should read as live equipment, not animated UI. |
| cli | 0.3 | +0.0 | Deliberately line-oriented (clap + anstream + owo-colors + indicatif + comfy-table + inquire) — colored headers, dimmed metadata, indicatif progress/spinners, comfy-table SLO tables = the 0.3 band (gh/cargo register), NOT the 1.0 full-TUI band the k9s reference itself occupies, since ratatui was explicitly omitted as heavier than this release-gate harness warrants. The TTY-gated, agent-parsed source-of-truth path must keep artifacts clean, so motion is honest progress feedback (spinner after delay, in-place status tier), never live-redrawn dashboard animation. |

## Recommended

**Final expression map:**
- Base: 0.3
- desktop-webview: 0.3
- cli: 0.3

**Reasoning:** A dev-tool / control-surface harness for a single technical operator under a "Mission-control patience — armed, nominal, go/no-go" personality lands a flat 0.3 across both surfaces — Linear-grade restraint on the webview, cargo/gh-grade colored-CLI feedback on the line-oriented terminal, with the deliberate ratatui omission keeping the cli row at 0.3 rather than k9s's native 1.0. Minimal security tier permits more but the brand's "exact downbeat, held silence" spine and Personal solo-operator scale make restraint the correct end state, not a constraint to overcome.

**Research basis:** WebSearch June 2026 — Envato "UX/UI trends 2026: calm interfaces, transparent AI and the end of visual theatrics" and UXPin "12 UX/UI Design Trends... 2026" (motion as functional communication layer / state-conveying, not decorative; calmer micro-interactions replacing extravagant motion); Adobe Design (Medium, Mar 2026) "Animation that fails safely" (hard caps on duration/iterations — defensive motion); Evil Martians "CLI UX best practices: progress displays" + techbytes "Rust & TUI for monitoring" (spinners only after ~200ms delay, determinate progress bars, 15-30 FPS caps); tech-insider "Tauri vs Electron 2026" (Tauri 2.10.3, performance-over-bloat path). Reinforced by training-data-2026 surface anchors: Linear (desktop-webview, 0.3) and k9s/lazygit (CLI/TUI references, scaled down to line-oriented 0.3 here).
