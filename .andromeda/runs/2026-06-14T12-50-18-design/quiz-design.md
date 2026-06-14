# Brand Quiz — Conductor

## Context (from architecture.md + security-plan.md + tooling-decisions.md)

- Product type: Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness driving a live Pulse through P-001..P-060 on a deterministic seeded timeline. "A control surface, not a dashboard."
- Audience: The Pulse developer — solo, local, technical (single developer-operator; headless path is agent-driven).
- Platforms: `desktop-webview` (Tauri 2 bundled webview) + `cli` (headless `conductor-cli` / `scripts/agent-run.sh`). No web, no mobile.
- Scale intent: Personal — solo developer, local dev host, no cloud, no multi-tenancy.
- Security tier: Minimal (0).
- Family chosen: React.
- Frontend framework: React 19.x (Vite, SPA — not SSR; Preact 10.x size fallback).
- CSS tool: Tailwind CSS v4.1.
- Component library: shadcn/ui (Radix UI Primitives 1.x).
- Mobile framework: N/A.
- CLI surface tooling (Phase 1): clap 4.5 + anstream/anstyle + owo-colors 4 + indicatif 0.18 + comfy-table 7 + inquire 0.7 (TTY-gated, line-oriented; ratatui full-TUI deliberately omitted).

## Q1: Brand Personality

- **Research recommended:** Direction 1 — "Conductor's score — exact downbeat, held silence" (orchestral podium + open score metaphor).
- **User response:** picked 3 (override of recommendation).
- **Final answer:** "Mission-control patience — armed, nominal, go/no-go"
- **Physical-world metaphor:** A small single-operator mission-control console during a timed run — one operator at a station, status lights and a count clock, hold / operator-pause prompts before each committed step.
- **Voice:** Calm and resolute under load — terse status callouts, no alarm in the tone even when a verdict comes back Fail.
- **Reasoning:** A Conductor run is a launch-style sequence (seeded timeline, live emission counters + target status, operator-pause holds, report states incl. **Blocked** precondition = go/no-go); the solo developer-operator beside a live Pulse is a single-station console, not a team war-room dashboard.

## Q2: Reference Products

- **Research recommended:** Linear (desktop-webview) + k9s (CLI) as the top anchor pair.
- **User response:** picked Linear + k9s + NASA Open MCT (elevated Open MCT to co-primary so mission-control vocabulary drives layout language, not just feel).
- **Final answer:**
  - **Linear** — issue-tracker SaaS (desktop-webview anchor). Instrument-panel density without clutter; a monospace face reserved strictly for IDs/codes/shortcuts as a distinct status tier; flat inset-border elevation on a near-black ground. *Learn:* monospace-as-a-typographic-tier for verdicts / P-IDs / SLO timings; single-row dense state held by a fixed spacing grid.
  - **k9s** — Kubernetes cluster TUI (CLI anchor). Persistent status header + anchored keybinding hint row; color-tiered status (green nominal / yellow degraded / red failing) updating in place under load. *Learn:* fixed status-header + pinned hint frame; color-tier convention → Pass / CalibrationRegion / Fail / Blocked as status lights.
  - **NASA Open MCT** — mission-control telemetry framework (vocabulary + layout-language source). Named operational phase line, countdown clock as heartbeat, go/no-go readiness poll, paused-count hold-point. *Learn:* name the phase as a single prominent status line; treat the count as the heartbeat; "paused count shows where it stopped."
  - *(Supporting references, kept in library:* lazygit — persistent bottom keybinding hint bar + confirm-gated actions for operator-pause; Warp — block-per-step collapsible run log + calm low-chroma surface.*)*
- **Reasoning:** Linear + k9s anchor Conductor's two real surfaces on its exact tech (React 19 + Tailwind dark-matte; color-tiered terminal status); Open MCT supplies the literal go/no-go vocabulary the personality borrows, now elevated to drive layout language (named phase / countdown / hold-point).

## Q3: Color Mood

- **Research recommended:** Mood 1 — "Apollo Mission Control firing room, dark and counting" (#0E0F10 / #3FB950 / #D2A24C).
- **User response:** picked 3 (override — chose contemporary dev-tool grounding over the retro Apollo palette).
- **Final answer:**
  - **Mood:** "k9s cluster watch on a Warp terminal" (a cool blue-cast dark dev-tool world)
  - **Example colors:**
    - Slate Ground #1A1B26 — Warp / Tokyo-Night charcoal with a faint blue cast; the "technical / precise" dark dev-tool ground.
    - Nominal Green #7EE787 — the k9s "healthy" status tier → Pass / on-timeline.
    - Degraded Yellow #E3B341 — the k9s "degraded" amber that a resource turns *in place* under load → CalibrationRegion / operator-pause. (A reserved failing red completes the tier → Fail / Blocked.)
- **Reasoning:** A cool blue-cast charcoal with a tight three-step functional status palette makes color read as functional state, not mood — the closest register to a contemporary dev-tool dark theme, matching the Linear / k9s / Warp anchors which are all dark-native.

## Q4: Expression Level

- **Research recommended:** base 0.3, desktop-webview 0.3, cli 0.3.
- **User response:** accepted.
- **Final answer:**
  - **Base:** 0.3
  - **Per-surface:**

    | Surface | Expression | Reasoning |
    |---|---|---|
    | desktop-webview | 0.3 | Linear/Raycast band — subtle hover, fades, in-place status-light color changes over a flat inset-border dark ground; no staggered entrances / parallax / framer-motion. The ~3 MB artifact + "control surface, not a dashboard" mandate forbid motion bloat; state changes read as live equipment. |
    | cli | 0.3 | gh/cargo band — colored headers, dimmed metadata, indicatif progress (spinner after ~200ms), comfy-table SLO tables. Line-oriented (ratatui full-TUI deliberately omitted), TTY-gated so agent-parsed artifacts stay clean; motion is honest progress feedback, never live-redrawn dashboard animation. |
- **Reasoning:** Dev-tool control surface + solo technical operator + "calm under load, no alarm" personality → flat 0.3 functional-motion-only is the end state, not a compromise. Minimal tier permits more, but the "exact downbeat, held silence" spine caps it here.

## Q5: Signature Element

- **Research recommended:** Candidate 1 — "The verdict lamp — in-place status-light resolve on the P-ID row".
- **User response:** picked 2 / "Paused-count only" (override — chose Candidate 2 as the sole signature; the absence of motion *is* the signature).
- **Final answer:**
  - **Element:** "Paused-count hold-point — the frozen heartbeat at the operator-pause"
  - **What it is:** A single prominent mono count (run elapsed / step index — the run's heartbeat) ticks in place at the top of the control surface during a run. When an operator-pause prompt fires before a committed timeline step, the count does NOT blank or keep running — it **freezes at the exact hold value**, the phase label beside it flips to "HOLD — operator pause" in amber, and the count tints green-nominal → amber-hold while held. On proceed it resumes from the frozen value; on abort it stops and dims. Borrows NASA Open MCT's "paused count shows where it stopped" convention.
  - **Implementation notes:** React 19 component holding run-elapsed state, count rendered in the reserved mono face; the freeze is just halting the tick interval + applying an amber Tailwind text class (a CSS color transition at most — zero animation library, within the 0.3 budget). CLI mirror: an indicatif progress/spinner that **stops (not hides)** at the hold, plus a colored owo-colors "HOLD" phase line above the inquire operator-pause prompt — TTY artifacts stay clean.
  - **Core-action tie:** Triggered by the operator-pause go/no-go prompt — the staged "hold before you commit the next step" action that gates every committed timeline step in a run.
- **Reasoning:** The held, frozen count is the literal "Mission-control patience" and the "exact downbeat, held silence" spine in one element — calm, amber-on-slate, no flashing — and is the purest possible expression of a 0.3 functional-motion budget (the absence of motion is the signature). *(The Q5-recommended verdict lamp remains a strong supporting status convention — see research-q5.md Candidate 1 — but the signature is the paused-count per the user's pick.)*

## Q6: Dark/Light/Auto (orchestrator-only)

- **Final answer:** **Auto** — dark (#1A1B26 slate) as the default; a light variant that follows the OS setting.
- **Reasoning:** Product type is a dark-native dev-tool console (the chosen mood IS the cool slate), so dark is the default; the user opted to ALSO ship a light variant following system preference. **Downstream note for Phase 4:** the palette must derive BOTH a dark (default) and a light token value per entry — not dark-only.

## Surface-Specific Confirms (orchestrator-only, conditional)

- **desktop-webview:** **Custom frameless window** — Tauri `decorations: false` with a custom top bar (drag region + run phase/count heartbeat + min/close controls), Linear/Raycast-style. Cohesive single-surface console; hosts the paused-count signature in the bar. (Carries the security-plan guardrails: deny-by-default capabilities, no remote-origin iframes, no `shell-open` with derived strings.)
- **cli:** **Colorful + TTY-gated** — not a new confirm; already specified in Phase 1 tooling-decisions (owo-colors styling via anstream, auto-strips ANSI when piped so agent-captured artifacts stay clean; interactive prompts gated behind a TTY check).

## Decisions Log

`2026-06-14` — Initial brand quiz by `/andromeda-design` Phase 2

- Personality: Mission-control patience — armed, nominal, go/no-go *(user override of recommended "Conductor's score")*
- References: Linear + k9s + NASA Open MCT primary *(user elevated Open MCT)*; lazygit + Warp supporting
- Mood: k9s-on-Warp cool slate #1A1B26 + green/amber/red functional status *(user override of recommended "Apollo firing room")*
- Expression: base 0.3, surfaces (desktop-webview 0.3, cli 0.3) — accepted
- Signature: Paused-count hold-point *(user override of recommended "verdict lamp")*
- Dark/Light: Auto (dark #1A1B26 default + light variant follows system)
- Per-surface confirms: desktop-webview = custom frameless titlebar; cli = colorful + TTY-gated (from Phase 1)
- Notes: The user consistently overrode toward **restraint + contemporary dev-tool grounding** — mission-control over the orchestral score (Q1), the real slate dev-tool world over the retro Apollo palette (Q3), and the frozen-pause "held silence" over the verdict lamp (Q5). **3 of 5 Q recommendations were overridden — downstream phases (Domain Exploration / Library Search / Plan Draft / Layouts) MUST honor the user's final answers, not the sub-agent recommendations.** Auto color mode requires BOTH a dark (default) and a light token set in Phase 4. The Q5-recommended verdict lamp is retained as a supporting (non-signature) status convention.
