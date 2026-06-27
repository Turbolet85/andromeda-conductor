# Design System — Conductor

## Brand Identity

**Personality:** Mission-control patience — armed, nominal, go/no-go: a single-operator console that stays calm and resolute under load, terse status callouts with no alarm in the tone even when a verdict comes back Fail.

**Domain anchors:** Mission-control console (single-station, not a war-room dashboard) · Count / heartbeat (the run's single live mono tick) · Operator-pause / go-no-go hold (the staged "hold before you commit the next timeline step") · Verdict / report states (`Verdict: Pass / Fail / CalibrationRegion`; the 5-valued `ReportState: Pass / Fail / ManualCheck / KnownResidual / Blocked`) · Coverage matrix (the dense single-row-per-P-ID wall of all 60 capabilities).

**Signature element:** The **Paused-count hold-point** — the frozen heartbeat at the operator-pause. A single prominent mono count (run-elapsed / step index) ticks in place at the top of the console tinted nominal-green; when an operator-pause go/no-go prompt fires before a committed timeline step, the count does NOT blank and does NOT keep running — it **freezes at the exact hold value**, the phase line beside it flips to "HOLD — operator pause", and the count tints green → hold-amber for the duration of the hold. On proceed it resumes from the frozen value; on abort it stops and dims to blocked slate-violet. **The absence of motion is the signature** — "paused count shows where it stopped" (NASA Open MCT). The verdict lamp (in-place per-P-ID status light) is retained as a *supporting* status convention, not the signature.

**Expression level:** `0.3` — governs animation intensity, layout freedom, and interaction complexity across the entire project.
- `0.0-0.2`: static, grid-locked, zero animation, dense
- `0.3-0.4`: subtle hover states, fade transitions, quiet focus rings
- `0.5-0.6`: smooth transitions, micro-interactions, loading skeletons
- `0.7-0.8`: staggered entrances, parallax, spring physics
- `0.9-1.0`: scroll-driven, 3D, particles, canvas/WebGL

This number is the SINGLE SOURCE OF TRUTH for how much motion and visual dynamism downstream phases should implement. At `0.3`, Conductor gets in-place CSS color transitions (the count tint, the status-light verdict lamp), subtle hover, and 200ms fades at most — never `framer-motion`, spring physics, parallax, or staggered reveals. Per-surface expression: desktop-webview `0.3`, cli `0.3` (both accepted, flat).

**Design direction:**
The cool, blue-cast calm of a single-operator mission-control console watching one live instrument: a near-black Tokyo-Night slate ground (`#1A1B26`) where the only color is functional status — a green / amber / red / slate-violet tier that reads as *Verdict / ReportState*, never as mood. Elevation is flat inset-border seams (`#2A2E42`) with zero drop shadows — Linear's "instrument-panel density" rather than a Grafana tile wall, the Coverage matrix a dense single-row-per-P-ID list. Color, motion, and a reserved mono ID-cyan typographic tier all carry operational meaning; the held silence of the frozen count is the loudest thing on screen. "A control surface, not a dashboard" — terminal density, dark-default, calm under load, no alarm.

---

## Color Palette

**Rationale:** Every hex derives from the exploration's Color World — the "k9s cluster watch on a Warp terminal" cool blue-cast dev-tool world the user chose over the retro Apollo firing-room palette (Q3 override). The functional status triad (nominal green / hold amber / fail red) + blocked slate-violet is Conductor's specialization of the generic "dark syntax theme": color reads as run state, not decoration. Auto mode is mandatory — every entry carries a **dark (default)** value and a **light variant** (the off-white console under room light) carried verbatim from the Color World's light variants. Mono ID cyan is a fourth *typographic-color* tier reserved strictly for the monospace status face (count / P-IDs / run_id / SLO timings / fingerprints), never used as a generic brand accent.

### Core Colors
| Role | Value (dark default) | Light variant | Usage | Domain anchor |
|------|------|------|-------|---------------|
| Primary | `#7DCFFF` (mono ID cyan) | `#0969DA` | Focus rings, active control highlight, the reserved mono status-tier color (count / P-001..P-060 / run_id / SLO timings / fingerprints) | Linear's "monospace-as-a-typographic-tier" made a color — the Coverage matrix + Fingerprint identity; "blue focus" mapped to the ID-cyan tier (industry-rules) |
| Secondary | `#A9B1D6` (journal line-text) | `#343B58` | UI prose, phase-line label text, run-report body rows, dimmed metadata | The Tokyo-Night foreground grey-lavender — the readable-but-recessive Emission journal / Run report artifact text tier |
| Accent | `#7EE787` (nominal green) | `#1A7F37` | The count/heartbeat tint while on-timeline, the `Pass` verdict lamp | The k9s "healthy" status tier — Count / heartbeat alive + Verdict `Pass`; accent-as-status, never accent-as-brand |

### Surface Scale (elevation hierarchy)
| Level | Value (dark default) | Light variant | Usage |
|-------|------|------|-------|
| Base | `#1A1B26` | `#F4F5F8` | Console app background + frameless titlebar ground (the Mission-control console surface) |
| Raised-1 | `#1F2130` | `#FFFFFF` | Phase-line / count panel, run-report card, coverage-matrix container (+lightness over base, NO shadow) |
| Raised-2 | `#24273A` | `#FBFBFD` | Picker dropdown, popover, tooltip (flat, border-defined) |
| Raised-3 | `#2A2D42` | `#FFFFFF` | Operator-pause go/no-go dialog, modal surface |
| Inset | `#15161F` | `#E9EBF2` | Scenario/seed input fields, the JSONL journal code block (darker than parent) |

### Text Hierarchy
| Level | Value (dark default) | Light variant | Usage |
|-------|------|------|-------|
| Primary | `#C8D0F0` | `#1A1B26` | Phase-line headline, primary labels (lifted from journal-grey for headline contrast) |
| Secondary | `#A9B1D6` | `#343B58` | Run-report body, descriptions, supporting prose (journal line-text) |
| Tertiary | `#717AA0` | `#565F89` | Metadata, `journal_emitted_at` timestamps, captions (muted Tokyo-Night comment) |
| Muted | `#565F89` | `#6E7491` | Disabled labels, placeholders, dimmed-on-abort text (blocked slate-violet) |

### Semantic Colors
| State | Background (dark) | Border (dark) | Text (dark) | Light text |
|-------|------------|--------|------|------|
| Success (`Pass` / on-timeline) | `#142A1C` | `#2E5A3C` | `#7EE787` | `#1A7F37` |
| Warning (`CalibrationRegion` / HOLD) | `#332B14` | `#5C4E22` | `#E3B341` | `#9A6700` |
| Error (`Fail` / port-occupier alarm edge) | `#33191A` | `#5C2E2E` | `#F85149` | `#CF222E` |
| Info / Blocked (`ReportState::Blocked` — never measured) | `#21243A` | `#3A4066` | `#565F89` | `#6E7491` |
| Manual (`ReportState::ManualCheck` — awaiting operator) | `#1E2233` | `#3A4160` | `#A9B1D6` | `#565F89` |
| Residual (`ReportState::KnownResidual` — pre-accepted gap) | `#26242E` | `#4A4458` (rendered dashed) | `#9A93A8` | `#6E6478` |

> **Verdict (3) vs ReportState (5) — a report row carries BOTH (per arch's run-report envelope).** The functional **machine-verdict** triad maps green = `Verdict::Pass`, amber = `Verdict::CalibrationRegion` (model-interpretive, report-for-human) / operator-pause HOLD, red = `Verdict::Fail` (held muted, no flashing). The **ReportState** adds three outcomes the machine did NOT pass/fail and which therefore must never collapse into `Fail`: slate-violet **hollow ring** = `ReportState::Blocked` (present-but-greyed — *never measured*); neutral-lavender **checkbox glyph** = `ReportState::ManualCheck` (*no programmatic read-back — awaiting the operator's eyes*; deliberately OUTSIDE the green/amber/red triad because there is no machine verdict — it renders as an operator-checklist); muted **dashed** dot = `ReportState::KnownResidual` (a *measured but pre-accepted* spec-vs-runtime gap — P-032 the first instance — carrying "expected until {named fix}", visually NOT red). Fail red is the reserved alarm edge held muted per "calm under load, no alarm"; `ManualCheck` and `KnownResidual` are distinct treatments precisely so neither silently reads as `Fail` (the same "no surprise failure" discipline already applied to `Blocked`).

### Border Progression
| Intensity | Value (dark default) | Light variant | Usage |
|-----------|------|------|-------|
| Subtle | `#2A2E42` (inset-border line) | `#D8DBE6` | Section separators, phase-line/count/coverage-matrix-row seams (the flat inset-border elevation — NO drop shadow) |
| Standard | `#363B57` | `#C4C8D8` | Input borders, P-ID list dividers |
| Emphasis | `#565F89` | `#9AA0BE` | Selected coverage-matrix row, active picker item |
| Focus | `#7DCFFF` (mono ID cyan) | `#0969DA` | Focus rings — accessibility-critical, the same ID-cyan as the mono tier so focus reads as "the operator's cursor on the instrument" |

---

## Typography

**Rationale:** Tied to the exploration's `typography-stack-install` spec and library-shortlist pairing #1, made literal. **JetBrains Mono** is the reserved *status tier* — not "mono everywhere" (an explicitly Rejected Default) but mono-as-a-typographic-tier (Linear's discipline): the count/heartbeat, P-001..P-060, run_id stamps, SLO timings, and fingerprints. Its even, legible glyphs (ligatures OFF for ID legibility) make the frozen count read as a precise instrument readout. **IBM Plex Sans** carries the phase line and all prose — the sanctioned humanist-grotesque replacement for Linear's banned Inter (similar metrics, self-hostable). Both are self-hosted WOFF2 via Fontsource — no runtime CDN, satisfying Minimal-tier offline hardening. Inter / Roboto / Arial / Helvetica / system-ui are banned as primary faces.

| Role | Font | Weight | Size | Tracking | Usage |
|------|------|--------|------|----------|-------|
| Display | JetBrains Mono | 500 | 28px / 1.1 | `-0.01em` | The paused-count heartbeat in the frameless titlebar (the signature readout) |
| Heading | IBM Plex Sans | 600 | 18px / 1.3 | `-0.005em` | The phase line ("error-baseline-spike", "HOLD — operator pause"), panel titles |
| Body | IBM Plex Sans | 400 | 14px / 1.5 | `0` | Run-report prose, dialog copy, descriptions |
| Label | IBM Plex Sans | 500 | 12px / 1.35 | `0.01em` | Form labels, button text, menu items, picker rows |
| Code | JetBrains Mono | 400 | 13px / 1.55 | `0` | JSONL emission-journal block, file paths, scenario config snippets |
| Data | JetBrains Mono | 500 | 13px / 1.4 | tabular-nums | P-001..P-060, run_id, SLO timings (`<5s`/`<20s`/`<90s`), latency_ms, fingerprints, coverage-matrix columns — rendered in mono ID cyan (`#7DCFFF`) |

**Loading:** Self-hosted WOFF2 via `@fontsource/jetbrains-mono` + `@fontsource/ibm-plex-sans`, locally vendored into the Tauri bundle (no Google Fonts / CDN at runtime — Minimal-tier offline hardening). Fallback stack: `"JetBrains Mono", ui-monospace, monospace` for the mono tier; `"IBM Plex Sans", ui-sans-serif, sans-serif` for prose.

**Surface-conditional guidance:**
- **desktop-webview:** full table above applies (JetBrains Mono status tier + IBM Plex Sans UI/prose).
- **cli:** Terminal monospace (user's configured terminal font — Conductor does not set a webview face here). Brand is expressed through ANSI color usage, output prefix patterns (`✓`/`✗`/`→`/`•` and `[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`), header formatting (bold/dim via `owo-colors`), and information density — the same reserved mono *status tier* mirrored via color (mono ID cyan → ANSI 117 for P-IDs / run_id / SLO timings / fingerprints), not webview typography.

---

## Spacing

| Token | Value | Usage |
|-------|-------|-------|
| space-micro | 2px | Lamp-glyph-to-label gap, inline status-tier spacing |
| space-xs | 4px | Tight padding within the count/phase-line cluster |
| space-sm | 8px | Component internal padding (buttons, picker rows, input fields) |
| space-md | 12px | Coverage-matrix row padding, panel internal gaps |
| space-lg | 20px | Major section separation (titlebar ↔ matrix ↔ run-report) |
| space-xl | 32px | Console outer margins, dialog padding |

Base unit: **4px** — all values are multiples of this (the Precision & Density "4px base" from the design direction; developer-tool density, tighter than a consumer breathing-room scale, matching Linear instrument-panel density).

**Surface-conditional:** Spacing tokens apply to the `desktop-webview` surface. For `cli`, spacing is terminal-controlled — column alignment is `comfy-table` fixed-width grid + `indicatif` layout, detected from terminal width dynamically (never hardcoded widths).

---

## Depth Strategy

**Chosen approach:** borders-only

**Rationale:** The exploration's Mission-control console anchor and Linear reference both specify "flat inset-border elevation on a near-black ground" with zero drop shadows; the design direction's depth approach is explicitly borders-only. Drop shadows / layered elevation would read as the floating-card SaaS dashboard the project explicitly rejects ("a control surface, not a dashboard"), and glassmorphism is a named Rejected Default. On the cool slate ground, a single near-invisible inset-border line one notch above the ground (`#2A2E42`) defines every seam — phase line, count, coverage-matrix rows — so the surface reads as one cohesive instrument panel, not stacked cards. This also serves the ~3 MB Tauri artifact + performance discipline: no `backdrop-filter`, no GPU-heavy shadow compositing.

**Values:**
- Border width: `1px` solid on all seams and panel edges.
- Subtle seam (default elevation): `1px solid #2A2E42` (dark) / `1px solid #D8DBE6` (light).
- Raised surfaces (popover/dialog): same `1px` inset-border + a `+2–4%` lightness step on the surface fill (Surface Scale above) — NO shadow, NO blur.
- Inset surfaces (inputs, journal block): `1px solid #2A2E42` + a darker fill than parent (`#15161F`).
- Drop shadows: **none.** `box-shadow` is permitted only as a single `0 0 0 2px var(--color-focus)` focus-ring inset on keyboard focus (a ring, not an elevation shadow).

---

## Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| radius-sm | 4px | Inputs, buttons, picker rows, status-lamp chips, small controls |
| radius-md | 6px | Coverage-matrix container, run-report card, count/phase-line panel |
| radius-lg | 8px | Operator-pause go/no-go dialog, modal surfaces |
| radius-full | 9999px | The per-P-ID status-lamp dot (verdict lamp glyph), toggles |

**Personality:** Sharp-leaning — small radii (4–8px) read as technical/instrument-panel precision (Linear/Raycast register), not friendly consumer rounding. The one `radius-full` exception is intentional contrast: the verdict-lamp status dot is a literal "light," so a perfect circle reinforces the mission-control status-light metaphor against the otherwise crisp rectangular console.

**Surface-conditional:** Border Radius applies to `desktop-webview` only. Omit for `cli` (terminal-controlled).

---

## Motion (calibrated to expression level `0.3`)

All motion decisions flow from the expression level set in Brand Identity. At `0.3`, motion is *functional only* — it communicates live equipment state, never decorates.

**Easing:** `ease-out` (`cubic-bezier(0, 0, 0.2, 1)`) — quiet, no overshoot, no spring. State changes settle, they don't bounce.

**Duration scale (adjusted for expression level):**

| Expression | Hover/Focus | Page transition | Entrance | Scroll effects |
|------------|-------------|-----------------|----------|----------------|
| 0.0-0.2 | 0-100ms | instant/none | none | none |
| **0.3-0.4** | **150ms ease-out** | **200ms fade** | **none** | **none** |
| 0.5-0.6 | 200ms ease-out | 300ms slide | stagger 50ms | subtle reveal |
| 0.7-0.8 | 250ms spring | 400ms spring | stagger 80ms | parallax |
| 0.9-1.0 | 300ms spring | 500ms+ custom | stagger 100ms+ | full scroll-driven |

**This project's values:**
- Micro-interactions (hover, focus): 150ms ease-out — control hover background lift, focus-ring fade-in.
- Transitions (panel open, dialog open): 150ms fade (`--motion-micro`) for the operator-pause dialog; in-place status-light color transition (green → amber → red → slate-violet) at 150ms ease-out.
- Entrance animations: **none** (expression < 0.5) — the console and coverage matrix render in place, no staggered reveals.
- Scroll effects: **none** (expression < 0.7).

**Motion tokens** (CSS custom properties on `:root` — bound by `var(--…)` name, never a raw ms/curve):

| Token | Value | Usage |
|-------|-------|-------|
| motion-micro | 150ms | Micro-interactions (hover/focus) + the count-tint state transition (nominal → hold → blocked) |
| motion-heartbeat | 1600ms | The Paused-count's live heartbeat period — the gentle opacity breath whose **halt** is the hold-point signature (value-ticking on the count arrives with the Epoch-9 live-counter `Channel`; until then the breath is the live signal) |
| ease-quiet | cubic-bezier(0, 0, 0.2, 1) | The quiet `ease-out` for every transition (no overshoot, no spring) |

**High-impact moments** (max 1 at this expression level):
The **Paused-count hold-point** (the signature) — when the operator-pause go/no-go prompt fires, the count's tick interval HALTS in place (the value freezes exactly) and the count text-color transitions nominal-green → hold-amber over a single 150ms CSS color transition (the phase line simultaneously swaps to "HOLD — operator pause" in amber). On proceed the tick resumes from the frozen value with a green color transition back; on abort the count stops and color-transitions to blocked slate-violet and dims. This is the *only* choreographed moment, and it is choreographed by the **absence** of motion — the freeze is the event. CLI mirror: the `indicatif` spinner **stops (not hides)** at the hold with a colored `owo-colors` "HOLD" phase line above the `inquire` prompt.

**Hard limits for this expression level:**
At `0.3`, the following are BANNED: no `framer-motion` / animation libraries (zero animation library — CSS transitions only); no spring physics; no parallax; no scroll-driven animation; no staggered entrances beyond initial paint; no 3D transforms; no canvas/WebGL; no pulse / blink / glow on any status (explicitly rejecting the Real-Time-Monitoring preset's pulsing defaults); no flashing red on `Fail` (the verdict lamp resolves in place, motionless); no progress bar that animates-to-100% or hides on the operator-pause (it must freeze). All transitions MUST be dropped under `@media (prefers-reduced-motion: reduce)` — including the count-tint transition on a held count (binding with a11y SC 2.3.3).

---

## Iconography

**Style:** Outlined (consistent stroke weight, matches the crisp instrument-panel register) + custom filled status-lamp glyphs (the verdict lamp is a solid dot — a "light," not an outline).

**Library:** Lucide React (ships with shadcn/ui) for hold / proceed / abort controls and the min/close frameless-titlebar buttons; plus project-specific custom SVG status-tier lamp components for the report states — six visually distinct treatments so none silently reads as another: `Pass` green filled dot · `CalibrationRegion`/HOLD amber filled dot · `Fail` red filled dot · `Blocked` slate-violet **hollow ring** (never measured) · `ManualCheck` neutral-lavender **checkbox glyph** (`☐` pending → `☑` once the operator confirms — outside the green/amber/red verdict triad, because it is not a machine verdict) · `KnownResidual` muted **dashed-ring dot** (measured-but-pre-accepted; carries the "expected until {fix}" note, visually NOT red).

**Size grid:** 16px (titlebar controls, status lamps) / 20px (picker + dialog actions) with 8px padding in containers.

**Rule:** icons clarify, not decorate — if removing an icon loses no meaning, remove it. Every status lamp is paired with a text label (`Pass`/`HOLD`/`Fail`/`Blocked`) per the Color-Only a11y rule; every icon-only titlebar/lamp control carries an `aria-label`.

---

## Surface: desktop-webview

**Platform:** Windows (primary dev host) + macOS + Linux — Tauri 2 bundled webview, single-station console, desktop-bound (no responsive breakpoints, no web/mobile).

**Toolkit / Framework:** React 19.x (Vite, SPA — Preact 10.x size fallback) + Tailwind CSS v4.1 (`@tailwindcss/vite`, Oxide engine, static zero-runtime offline stylesheet) + shadcn/ui (Radix UI Primitives 1.x, copy-paste locally vendored). Tauri 2 (≥ 2.10.3 per security plan) custom **frameless** window: `decorations: false`, custom titlebar with `data-tauri-drag-region`.

### Tokens (platform-specific)

Tailwind v4.1 design tokens, declared on **`:root`** (the `design-tokens-bundle-init` bundle) with `@import "tailwindcss"` loading the engine. Auto mode via `@media (prefers-color-scheme)` overrides; dark is the default block, light follows OS. **NOTE (Tailwind v4):** declare these on plain `:root`, NOT `@theme` — `@theme`/`@theme static` tree-shakes non-namespace tokens (dropping `--space-*` / `--radius-*` / `--motion-*` / `--ease-*`, 11 of 34) and forbids nesting inside `@media`; keep `@import "tailwindcss"` for the engine + utilities. Names + values below are the binding contract:

```css
:root {
  /* Surfaces — dark default */
  --color-base: #1A1B26;          --color-raised-1: #1F2130;
  --color-raised-2: #24273A;      --color-raised-3: #2A2D42;
  --color-inset: #15161F;
  /* Status tier (functional, reads as Verdict/ReportState) */
  --count-nominal: #7EE787;       /* on-timeline + Pass            */
  --count-hold: #E3B341;          /* operator-pause HOLD + CalibrationRegion */
  --status-fail: #F85149;         /* Fail (held muted, no flashing) */
  --count-blocked: #565F89;       /* Blocked — never measured       */
  --status-manual: #A9B1D6;       /* ManualCheck — awaiting operator (neutral, NOT the verdict triad)  */
  --status-residual: #9A93A8;     /* KnownResidual — measured pre-accepted gap (rendered dashed)        */
  /* Reserved mono ID-cyan typographic tier + focus */
  --color-id-cyan: #7DCFFF;       --color-focus: #7DCFFF;
  /* Text + seams */
  --text-primary: #C8D0F0;        --text-secondary: #A9B1D6;
  --text-tertiary: #717AA0;       --text-muted: #565F89;
  --border-subtle: #2A2E42;       --border-standard: #363B57;
  --border-emphasis: #565F89;
  /* Type */
  --font-mono: "JetBrains Mono", ui-monospace, monospace;
  --font-sans: "IBM Plex Sans", ui-sans-serif, sans-serif;
  /* Spacing base 4px / radius / motion */
  --space-micro: 2px; --space-xs: 4px; --space-sm: 8px; --space-md: 12px; --space-lg: 20px; --space-xl: 32px;
  --radius-sm: 4px; --radius-md: 6px; --radius-lg: 8px; --radius-full: 9999px;
  --motion-micro: 150ms; --ease-quiet: cubic-bezier(0,0,0.2,1);
}
@media (prefers-color-scheme: light) {
  :root {
    --color-base: #F4F5F8;          --color-raised-1: #FFFFFF;
    --color-raised-2: #FBFBFD;      --color-raised-3: #FFFFFF;
    --color-inset: #E9EBF2;
    --count-nominal: #1A7F37;       --count-hold: #9A6700;
    --status-fail: #CF222E;         --count-blocked: #6E7491;
    --status-manual: #565F89;       --status-residual: #6E6478;
    --color-id-cyan: #0969DA;       --color-focus: #0969DA;
    --text-primary: #1A1B26;        --text-secondary: #343B58;
    --text-tertiary: #565F89;       --text-muted: #6E7491;
    --border-subtle: #D8DBE6;       --border-standard: #C4C8D8;
    --border-emphasis: #9AA0BE;
  }
}
@media (prefers-reduced-motion: reduce) {
  * { animation: none !important; transition: none !important; }
}
```

### Component Patterns

1. **Frameless titlebar + Paused-count heartbeat (signature, primary placement).** `decorations: false`; a `data-tauri-drag-region` bar hosts the phase line (IBM Plex Sans 600, 18px) on the left and the count (JetBrains Mono 500, 28px, tabular-nums, `--count-nominal`) center, with min/close Lucide controls right (16px, `aria-label`-ed, hover lift 150ms). **Default:** count ticks green in place. **Hold:** count interval halts, value frozen, color → `--count-hold` over 150ms, phase line → "HOLD — operator pause" amber, `aria-live="assertive"` announces the flip. **Abort:** count → `--count-blocked`, dimmed. No drag-region pointer cursor leaking into controls.

2. **Operator-pause go/no-go dialog.** shadcn/ui `AlertDialog` (Radix), `--color-raised-3` fill, `1px solid --border-subtle`, `--radius-lg`, 150ms fade entrance (`--motion-micro`). Two actions: **Proceed** (primary, `--count-nominal` accent) and **Abort** (`--status-fail` text). `role="alertdialog"`; gates every committed timeline step (Confirmation Dialogs guideline). **Loading:** Proceed disables + shows in-flight state during the async step (Loading Buttons — prevents double-commit). Focus trapped; visible `--color-focus` ring.

3. **Coverage matrix (dense single-row-per-P-ID).** NOT a KPI-card grid (Rejected Default) — a single-row-per-P-ID list (Linear instrument-panel density), 12px row padding, `1px --border-subtle` dividers, no shadows. Each row: status-lamp glyph (the 6-state verdict/report-state lamp in #4 — filled dot for a measured verdict · hollow ring `Blocked` · neutral checkbox `ManualCheck` · muted dashed dot `KnownResidual`) + label + P-ID (`P-037`, JetBrains Mono, `--color-id-cyan`) + SLO tier + latency_ms (mono, ID-cyan). **States:** selected row → `--border-emphasis` left edge; hover → `--color-raised-1` lift. **Empty:** "No scenarios loaded" prose, not a gray placeholder. Virtual-scroll for the full 60-row wall.

4. **Verdict / report-state lamp (supporting status convention).** In-place per-P-ID status light on the matrix row: `radius-full` 16px filled dot for the machine-verdict triad — `--count-nominal` (`Pass`) / `--count-hold` (`CalibrationRegion`) / `--status-fail` (`Fail`); `--count-blocked` **hollow ring** (`Blocked` — never measured); `--status-manual` neutral-lavender **checkbox glyph** (`ManualCheck` — awaiting operator confirm, outside the verdict triad, links to the operator-checklist); `--status-residual` muted **dashed-ring dot** (`KnownResidual` — pre-accepted gap, carries the "expected until {fix}" note, never red). Resolves **motionless** (150ms color transition, never flashing — Rejected Default). Always paired with text (`Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`Manual`/`Residual`) + `aria-live` announcement (Color-Only + Error-Messages-announced guidelines).

5. **Scenario/suite picker + start/stop.** shadcn/ui `Command`/`Select` over `--color-raised-2`, mono P-ID column in ID-cyan. Start/stop buttons `--radius-sm`, keyboard-first (first-class shortcuts mirroring k9s/lazygit). **Disabled:** `--text-muted`, no color-only signal. Inputs are `--color-inset` with `1px --border-standard`, `--color-focus` ring on focus.

6. **Run-report view (verdict summary, prose + mono).** The terminal destination of the run flow: `--color-raised-1` card, `1px --border-subtle`, `--radius-md`, IBM Plex Sans body (`--text-secondary`) with mono P-IDs / run_id / SLO timings / latency_ms in `--color-id-cyan`; no shadow. Each P-ID line carries its verdict/report-state lamp + text (`Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`Manual`/`Residual`) per the Color-Only rule. **`ManualCheck` rows** expand into an operator-checklist (Component Pattern 7) — induced state + expected observation ticked y/n ("halo shifted toward burgundy? ☐"), because the claim has no programmatic read-back (distinct from the go/no-go pause, which gates a *future* step; this confirms a *past* observation). **`KnownResidual` rows** render the dashed muted lamp + the "expected to fail until {named fix}" note (P-032's `recent_commits` producer stub until v0.3.0) so a documented residual never shows as a surprise red `Fail`. **Empty (no run started):** "No run yet — pick a scenario/suite to begin" prose in `--text-tertiary`, never a gray skeleton placeholder. **In-progress (run not finished):** the report area shows "Run in progress" prose in `--text-secondary` (the live state lives in the titlebar count + matrix), NOT a partial/failed verdict — this is the distinct "no result yet" state, never downgraded to a red `Fail`.

7. **Operator-checklist (the `ManualCheck` render — the drive+observe surface).** Distinct from the go/no-go operator-pause dialog (#2 gates a *future* committed step); the checklist confirms a *past visual observation* Conductor cannot read back over MCP — the "drive+observe" half of the coverage matrix. Per `ManualCheck` P-ID it renders a card row: the **induced state** (what Conductor drove — "stormed the fingerprint 12×/30s", "model disabled") + the **expected observation** as a yes/no the operator ticks ("halo shifted toward burgundy? ☐", "no OS toast appeared? ☐"). `--color-raised-1` card, `1px --border-subtle`, `--radius-md`, neutral-lavender `--status-manual` checkbox glyphs (NOT the green/amber/red verdict triad — there is no machine verdict here). Space toggles a row (keyboard-first, like the rest of the console); ticking resolves that item's *verdict* to `Pass`/`Fail` while the *report-state* stays `ManualCheck` (operator-confirmed). The footer roll-up surfaces the unticked count so an incomplete manual pass is never mistaken for a finished run. This is where "Conductor never emits a native OS toast" is *checked* (the operator confirms Pulse's behavior), never performed.

### Navigation Pattern

Single-station console, no router/breakpoints: the frameless window IS the surface. The operator moves by keyboard-first run control — pick scenario/suite → start → watch the heartbeat + phase line + live counters (Tauri `Channel`) → answer operator-pause go/no-go holds → read the run-report view. Keybindings are first-class (start/stop/proceed/abort) and exposed via Radix primitives; no browser-style back/forward/URL nav.

### Platform-Specific Notes

- Custom frameless chrome must handle drag region, min/close, and double-click-to-maximize manually; standard window buttons on Windows/Linux, traffic-light-style on macOS.
- Honor security-plan Tauri guardrails: **deny-by-default capabilities file** (only start/stop · picker · run-report · operator-pause commands + one live-counter `Channel`); **no `shell-open` plugin with derived strings**; **no remote-origin iframes** (origin-confusion CVE-2026-42184 + IPC-bypass GHSA-57fm-592m-34r7); Tauri ≥ 2.10.3.
- Style the webview scrollbar (default web scrollbars look foreign in a desktop app); suppress Chromium context menu / devtools / text-selection on non-text elements; no `alert()`/`confirm()`/`prompt()` (use the styled `AlertDialog`).
- Conductor MUST NOT emit native OS toasts (those are Pulse behavior it observes). High-DPI crisp rendering; OS shortcuts (Ctrl+W / Alt+F4 / Cmd+Q) work.

---

## Surface: cli

**Platform:** Windows / macOS / Linux terminal — headless `conductor-cli` (`scripts/agent-run.sh`), the source of truth + release gate. Line-oriented (ratatui full-TUI deliberately omitted).

**Toolkit / Framework:** clap 4.5 + `anstream`/`anstyle` + `owo-colors` 4.x (styling, TTY-gated) + `indicatif` 0.18 (live counters / spinner) + `comfy-table` 7 (P-001..P-060 SLO tables) + `inquire` 0.7 (operator-pause prompts).

### Tokens (platform-specific)

ANSI 256-color codes mapping the Color World (per the library-shortlist mapping), applied via `owo-colors` through `anstream` so output auto-strips ANSI when piped (`!isatty(1)`), and honoring `NO_COLOR` + `TERM=dumb`:

```
Slate ground   #1A1B26  → ANSI 234/235  (backdrop; rarely set — terminal-owned)
Nominal green  #7EE787  → ANSI 114      (Pass / on-timeline heartbeat)
Hold amber     #E3B341  → ANSI 179      (HOLD / CalibrationRegion)
Fail red       #F85149  → ANSI 203      (Fail — no blink)
Blocked violet #565F89  → ANSI 60       (Blocked — never measured)
Manual lavender#A9B1D6  → ANSI 146      (ManualCheck — awaiting operator; paired with [MANUAL] + ? glyph)
Residual mute  #9A93A8  → ANSI 246      (KnownResidual — pre-accepted gap; paired with [RESIDUAL] + ~ glyph)
Mono ID cyan   #7DCFFF  → ANSI 117      (P-001..P-060 / run_id / SLO timings / fingerprints)
Journal text   #A9B1D6  → ANSI 146      (report rows / dimmed metadata)
```

Header style: bold + ANSI 117 (ID-cyan) for section titles; metadata dimmed. Status prefixes are ASCII text + color (never color alone): `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]` and `✓`/`✗`/`?`/`~`/`•`/`→` (TTY only — never emoji in machine-parseable piped output; `?` = `ManualCheck` awaiting operator, `~` = `KnownResidual` pre-accepted gap).

### Component Patterns

1. **Paused-count hold-point (signature, CLI mirror — primary placement).** During a run, an `indicatif` progress/spinner carries the count/heartbeat with a colored phase-line prefix (ANSI 114 green). On the operator-pause go/no-go, the spinner **STOPS in place (not hides)** at the exact value, and a bold ANSI 179 amber `HOLD — operator pause` phase line prints **above** the `inquire` prompt. Spinner appears only after ~200ms (honest progress, not instant churn). The stop — not a hide, not an animate-to-100% — is the signature; it embodies "paused count shows where it stopped." TTY-gated so agent-captured artifacts stay clean.

2. **Operator-pause prompt.** `inquire` proceed/abort confirm, gated behind an `isatty` check. **The headless path is NEVER blocked on an interactive prompt** — when stdin is not a TTY (agent-driven `agent-run.sh`), the prompt is skipped per the configured non-interactive policy and the decision is recorded to the artifact; an interactive operator gets the colored confirm. Proceed/abort decision logged to the run report.

3. **Coverage-matrix / SLO table.** `comfy-table` 6 columns: P-ID (ANSI 117 cyan) · scenario · `state` (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` colored prefix) · `slo_tier` (`<5s`/`<20s`/`<90s`) · `latency_ms` (cyan, right-aligned) · fingerprints. Width detected dynamically from the terminal (never hardcoded). A `Blocked` row carries the named precondition string; measurement columns render as `—`/null, never a red error.

4. **Verdict / report-state lines.** Per-P-ID result printed in place: `✓ P-009  Pass   1840ms <5s` (green) / `✗ P-014  Fail   …` (ANSI 203 red, no blink) / `? P-035  Manual  halo→burgundy? · no OS toast?` (ANSI 146 lavender — `[MANUAL]`, an operator-checklist item with no machine verdict; TTY: `inquire` y/n, headless: recorded unconfirmed) / `~ P-032  Residual  recent_commits stub → v0.3.0` (ANSI 246 muted — `[RESIDUAL]`, a pre-accepted gap, never red) / `• P-022  Blocked  mcp-server feature + ANDROMEDA_PULSE_MCP_ENABLED + matching data-dir` (ANSI 60 violet). Color is always paired with the text prefix (`[PASS]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`/…) for NO_COLOR + screen-reader friendliness.

5. **Error output.** To stderr, sanitized (no absolute host paths / internal struct names / stack traces per security plan): `error: <short>` + contextual detail + `hint: <fix>`. The `error:` label reuses Fail red (ANSI 203), `hint:` the Residual mute (ANSI 246) — tty-gated on stderr (its own `IsTerminal` gate, distinct from the stdout gate), never new colors. Never colorized when piped; the ASCII `error:`/`hint:` labels always stand (never color-alone); stack traces only under `--debug`/`-v`.

### Navigation Pattern

Linear, top-to-bottom stdout — no cursor manipulation, no full-screen redraw (ratatui omitted). The operator runs `conductor-cli` / `agent-run.sh`, reads colored headers → live `indicatif` heartbeat → `inquire` operator-pause holds (interactive TTY only) → `comfy-table` coverage/SLO summary → run-report path. Pipe-friendly: raw artifact data parseable, human messages colored on stderr, the headless agent path strips ANSI and never blocks.

### Platform-Specific Notes

- Always respect `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping (`anstream`). Avoid dark-blue-on-black / dark-red-on-black (ANSI 117 cyan + 114 green chosen for contrast on dark terminals).
- Don't rely on color alone — every status carries an ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`).
- Interactive `inquire` prompts ALWAYS check `isatty` first; the agent-driven headless source-of-truth path is never gated on a prompt (an auth/interactive prompt there would silently break the release gate — security plan).
- Detect terminal width dynamically for `comfy-table`; never hardcode widths or wrap at arbitrary points.

---

## Anti-Patterns (NEVER do these)

### Universal Bans
- NEVER use generic font families: Inter, Roboto, Arial, Helvetica, system-ui default — Conductor uses JetBrains Mono (status tier) + IBM Plex Sans (prose); Inter is the banned Linear-anchor face, replaced here.
- NEVER use purple-gradient-on-white as a color scheme — Conductor's ground is cool Tokyo-Night slate `#1A1B26`; the indigo/violet "AI gradient" is an explicit Rejected Default.
- NEVER use cookie-cutter card grids without adapting card internals to content — the Coverage matrix is a dense single-row-per-P-ID list, not a tile wall.
- NEVER use the same layout for different information types — the heartbeat/phase-line (titlebar), coverage matrix (dense list), and run-report (prose+mono) each get purpose-built layout.
- NEVER use color purely for decoration — every color communicates Verdict / ReportState (green=Pass, amber=HOLD/CalibrationRegion, red=Fail, slate-violet=Blocked, ID-cyan=mono status tier).
- NEVER converge on common "safe" choices across generations — anchor to THIS domain (mission-control console, frozen heartbeat).

### Rejected Defaults (from exploration)
- **A "dashboard" grid of KPI metric cards with big-number tiles + trend sparklines** — rejected because: every OTLP/telemetry tool in training data is a Grafana/Datadog tile wall; Conductor is a single-station Mission-control console ("a control surface, not a dashboard") — one heartbeat + phase line in the titlebar, the Coverage matrix as a dense single-row-per-P-ID list.
- **A flashing / pulsing red banner or animated alert toast on a Fail verdict** — rejected because: alarm = red + motion is the universal reflex, but the personality is "calm under load, no alarm"; Fail is an in-place, motionless `#F85149` verdict lamp on the P-ID row. Conductor must NOT emit native OS toasts (those are Pulse behavior it observes).
- **A determinate progress bar that hides or animates-to-100% during the operator-pause** — rejected because: the signature is the *frozen* heartbeat — the count freezes at the exact hold value and tints amber (CLI: `indicatif` stops-not-hides), embodying "paused count shows where it stopped." Hiding/animating destroys the signature.
- **Indigo/violet "AI gradient" accent + glassmorphism cards** — rejected because: it is the current statistical-average "smart tooling" aesthetic; Conductor uses the cool slate ground with a strict green/amber/red functional status tier and flat inset-border elevation (`#2A2E42`), color-as-state not mood.
- **Conflating "no result yet" with "failed" (graying both the same, or showing Blocked as a red error)** — rejected because: empty-state and error-state are usually collapsed; `Blocked` is a distinct slate-violet (`#565F89`) state carrying the named precondition string — present-but-greyed, *never measured*, never a silent downgrade to red Fail.
- **A monospace font used everywhere for "techy dev-tool vibes"** — rejected because: mono-everything is lazy shorthand; mono is reserved strictly as a status tier (ID cyan `#7DCFFF`) for the count, P-IDs, run_id, SLO timings, fingerprints — UI sans (IBM Plex Sans) for phase line + prose.

### Per-Surface Bans

**desktop-webview:**
- NEVER ship with visible Chromium/WebView2 artifacts (context menu, devtools, text-selection on non-text elements).
- NEVER use unstyled web-style scrollbars (they look foreign in a desktop app).
- NEVER use browser-style navigation (back/forward buttons, URL bar) — single-station console.
- NEVER use hover-only interactions without keyboard alternatives (keyboard-first run control).
- NEVER ignore OS shortcuts (Cmd+Q, Ctrl+W, Alt+F4); NEVER make the window non-resizable without justification.
- NEVER use `alert()`/`confirm()`/`prompt()` — use the styled `AlertDialog` operator-pause.
- NEVER ship Tauri commands without a deny-by-default capabilities file; NEVER embed remote-origin iframes; NEVER use `shell-open` with scenario-derived strings (security plan).
- NEVER add `backdrop-filter`/drop shadows (borders-only depth; GPU + ~3 MB artifact discipline).

**cli:**
- NEVER mix stdout (data) and stderr (messages) without intention; raw artifact data on stdout, human messages on stderr.
- NEVER use emoji in machine-parseable (piped) output; ASCII prefixes only there.
- NEVER wrap text at arbitrary points or hardcode widths — detect terminal width dynamically.
- NEVER print stack traces in normal mode — only under `--debug`/`-v`.
- NEVER use interactive `inquire` prompts without an `isatty` check; NEVER block the headless agent-driven source-of-truth path on a prompt.
- NEVER colorize without checking `NO_COLOR` / `TERM` / pipe status; NEVER rely on color alone (use `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` prefixes).
- NEVER hide the progress spinner or animate it to 100% on the operator-pause — it must STOP in place (signature).

---

## Self-Validation Protocol

Before presenting ANY UI output, downstream implementation phases (per project's specialist plans) must run these checks:

### 1. Swap Test
Replace your typeface with Inter, your colors with Tailwind defaults, your layout with a standard sidebar+cards. If the design doesn't feel meaningfully different → you defaulted. Redo with intent.

### 2. Squint Test
Blur your eyes at the interface. Can you still perceive hierarchy? Does anything jump out harshly? Good craft whispers — nothing should scream.

### 3. Signature Test
Point to the signature element in your output. Can you find it in at least 3 places? If you can't locate it → it doesn't exist. Inject it.

### 4. Token Test
Read your color/spacing values aloud. Do they trace back to the palette and scale above? Random hex values or magic numbers signal no system.

### 5. Sameness Test (from Interface Design)
If another AI given a similar prompt would produce substantially the same output — you have failed. The interface must emerge from THIS product's domain exploration, not from statistical patterns in training data.

---

## Design Decisions Log

_Orchestrator records key decisions here. Manual additions welcome._

2026-06-14: Initial design system generated by /andromeda-design (Phase 4)
- Brand personality: Mission-control patience — armed, nominal, go/no-go (single-operator console, calm + resolute, no alarm) — user Q1 override of the recommended orchestral "Conductor's score".
- Surfaces: desktop-webview (Tauri 2 frameless, React 19 + Tailwind v4.1 `@theme` + shadcn/ui) + cli (`conductor-cli`: clap + owo-colors + indicatif + comfy-table + inquire, ANSI 256, TTY-gated, line-oriented).
- Signature: Paused-count hold-point — the frozen heartbeat at the operator-pause (the absence of motion is the signature; count freezes at the hold value, tints green→amber, resumes on proceed / dims slate-violet on abort) — user Q5 override of the recommended verdict lamp, which is retained as a supporting status convention.
- Color: cool blue-cast Tokyo-Night slate `#1A1B26` dev-tool world (user Q3 override of the retro Apollo firing-room palette) with the functional status tier (green `#7EE787` / amber `#E3B341` / red `#F85149` / blocked slate-violet `#565F89`), mono ID-cyan `#7DCFFF` status tier; Auto mode — dark default + mandatory light variant per Color World entry.
- Expression: 0.3 flat (base / desktop-webview / cli) — functional motion only, zero animation library, in-place CSS color transitions, `prefers-reduced-motion` honored.
- Key rejection: the Grafana/Datadog KPI-card-grid "dashboard" — replaced by the single-station Mission-control console (heartbeat + phase line in the frameless titlebar; Coverage matrix as a dense single-row-per-P-ID list). "A control surface, not a dashboard."
- Security: honors Minimal-tier Tauri guardrails (deny-by-default capabilities, no `shell-open` with derived strings, no remote-origin iframes, Tauri ≥ 2.10.3) and the agent-driven "never block the headless path on an interactive prompt" rule; no styled credential exposure (Conductor owns no credentials).

