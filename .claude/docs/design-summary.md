# Design Summary — Conductor

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md`. setup-project Phase 3. wrap-session does not modify._

## Brand identity
**Mission-control patience — armed, nominal, go/no-go.** A single-operator console that stays calm and resolute under load: terse status callouts, no alarm in the tone even on a Fail. "A control surface, not a dashboard" — terminal density, dark-default, cool blue-cast Tokyo-Night slate. Explicitly NOT a Grafana/Datadog KPI tile wall. **Signature:** the *Paused-count hold-point* — the frozen heartbeat at the operator-pause (the absence of motion is the event). **Expression level 0.3** (functional motion only — CSS transitions, no animation library).

## Key design tokens (selected — dark defaults; full table in design-system.md)
| Token | Value | Use |
|---|---|---|
| `--color-base` | `#1A1B26` | console ground (Tokyo-Night slate) |
| `--count-nominal` | `#7EE787` | on-timeline heartbeat + `Pass` |
| `--count-hold` | `#E3B341` | operator-pause HOLD + `CalibrationRegion` |
| `--status-fail` | `#F85149` | `Fail` (held muted, no flashing) |
| `--count-blocked` | `#565F89` | `Blocked` (never measured) |
| `--color-id-cyan` | `#7DCFFF` | reserved mono status tier + focus ring |

### Typography
- **Status tier (reserved):** JetBrains Mono — count / P-IDs / run_id / SLO timings / fingerprints (in `--color-id-cyan`).
- **Prose / phase line:** IBM Plex Sans. Self-hosted WOFF2 via Fontsource (no runtime CDN). Inter/Roboto/Arial banned.

### Spacing & motion
- **Base unit:** 4px (developer-tool density). **Motion:** `--motion-micro` 150ms + `--motion-heartbeat` 1600ms (the live paused-count breath), `--ease-quiet` (no overshoot); reduced-motion drops ALL transitions. Depth is borders-only (`--border-subtle` seams, no drop shadows).

## Primary surfaces
- **Run console** — one frameless window in four run-states: idle / live / HOLD / report-terminal (no router, no breakpoints).
- **cli** — `conductor run|suite|report`, line-oriented (clap + owo-colors + indicatif + comfy-table + inquire), ANSI 256 TTY-gated.

## Component patterns
- **Frameless titlebar + paused-count heartbeat** — the signature's primary placement (freezes/tints/resumes).
- **Operator-pause go/no-go dialog** — shadcn `AlertDialog`, gates every committed timeline step.
- **Coverage matrix** — dense single-row-per-P-ID list (NOT a KPI-card grid), virtual-scrolled; its Mode cell renders one of four classifications, the out-of-scope one recessive via `--status-residual` ↔ ANSI 246 ↔ Markdown emphasis (never `--status-fail`/`--count-blocked`, and never a lamp). Its roll-up caption qualifies the auto term with `(N unbacked)` — plain text, no token, a qualifier on that summand and never a fifth one, omitted at zero.
- **Verdict/report-state lamp** — six visually distinct treatments (`Pass`/`CalibrationRegion`/`Fail` filled dots · `Blocked` hollow ring · `Manual` checkbox · `Residual` dashed dot), always text-paired. The lamp tokens are the lamps' — but `--status-residual` is also the shared recessive tier for two NON-lamp uses (the cli `hint:` label, the out-of-scope Mode cell).
- **Operator-checklist** — the `ManualCheck` render (induced state + expected observation, ticked y/n) — the drive+observe surface.

## Universal bans
- No generic fonts (Inter/Roboto/Arial/system-ui); no purple-gradient-on-white "AI gradient" + glassmorphism.
- No KPI-card-grid "dashboard"; no color purely for decoration (color = Verdict/ReportState).
- No flashing/pulsing on `Fail`; no progress bar that hides/animates-to-100% on the operator-pause (it freezes).
- No mono-everywhere — mono is the reserved status tier only.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`. Path-scoped rules: `.claude/rules/frontend.md` + `a11y.md`.
