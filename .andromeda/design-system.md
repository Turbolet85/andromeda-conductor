# Design System — Conductor

**Conductor has no interface.** Its window — the `desktop-webview` surface — was retired 2026-10-11 (`2026-10-11-conductors-window-retired`, intent R1); the cli surface below is all it renders.

## Brand Identity

**Personality:** Mission-control patience — armed, nominal, go/no-go: a single-operator console that stays calm and resolute under load, terse status callouts with no alarm in the tone even when a verdict comes back Fail.

**Domain anchors:** Mission-control console (single-station, not a war-room dashboard) · Count / heartbeat (the run's single live mono tick) · Operator-pause / go-no-go hold (the staged "hold before you commit the next timeline step") · Verdict / report states (`Verdict: Pass / Fail / CalibrationRegion`; the 5-valued `ReportState: Pass / Fail / ManualCheck / KnownResidual / Blocked`) · Coverage matrix (the dense single-row-per-P-ID wall over the manifest's accepted capability set).

**Signature element:** The **Paused-count hold-point** — the frozen heartbeat at the operator-pause. A single prominent mono count (run-elapsed / step index) ticks in place at the top of the console tinted nominal-green; when an operator-pause go/no-go prompt fires before a committed timeline step, the count does NOT blank and does NOT keep running — it **freezes at the exact hold value**, the phase line beside it flips to "HOLD — operator pause", and the count tints green → hold-amber for the duration of the hold. On proceed it resumes from the frozen value; on abort it stops and dims to blocked slate-violet. **The absence of motion is the signature** — "paused count shows where it stopped" (NASA Open MCT). The verdict lamp (in-place per-P-ID status light) is retained as a *supporting* status convention, not the signature.

**Expression level:** `0.3` — governs animation intensity, layout freedom, and interaction complexity across the entire project.
- `0.0-0.2`: static, grid-locked, zero animation, dense
- `0.3-0.4`: subtle hover states, fade transitions, quiet focus rings
- `0.5-0.6`: smooth transitions, micro-interactions, loading skeletons
- `0.7-0.8`: staggered entrances, parallax, spring physics
- `0.9-1.0`: scroll-driven, 3D, particles, canvas/WebGL

This number is the SINGLE SOURCE OF TRUTH for how much motion and visual dynamism downstream phases should implement. At `0.3`, Conductor gets honest progress feedback only — never `framer-motion`, spring physics, parallax, or staggered reveals. Per-surface expression: cli `0.3`.

**Design direction:**
The cool, blue-cast calm of a single-operator mission-control console watching one live instrument: a near-black Tokyo-Night slate ground (`#1A1B26`) where the only color is functional status — a green / amber / red / slate-violet tier that reads as *Verdict / ReportState*, never as mood. Elevation is flat inset-border seams (`#2A2E42`) with zero drop shadows — Linear's "instrument-panel density" rather than a Grafana tile wall, the Coverage matrix a dense single-row-per-P-ID list. Color, motion, and a reserved mono ID-cyan typographic tier all carry operational meaning; the held silence of the frozen count is the loudest thing on screen. "A control surface, not a dashboard" — terminal density, dark-default, calm under load, no alarm.

---

## Color Palette

**Rationale:** Every hex derives from the exploration's Color World — the "k9s cluster watch on a Warp terminal" cool blue-cast dev-tool world the user chose over the retro Apollo firing-room palette (Q3 override). The functional status triad (nominal green / hold amber / fail red) + blocked slate-violet is Conductor's specialization of the generic "dark syntax theme": color reads as run state, not decoration. Auto mode is mandatory — every entry carries a **dark (default)** value and a **light variant** (the off-white console under room light) carried verbatim from the Color World's light variants. Mono ID cyan is a fourth *typographic-color* tier reserved strictly for the monospace status face (count / P-IDs / run_id / SLO timings / fingerprints), never used as a generic brand accent.

### Core Colors
| Role | Value (dark default) | Light variant | Usage | Domain anchor |
|------|------|------|-------|---------------|
| Primary | `#7DCFFF` (mono ID cyan) | `#0969DA` | Focus rings, active control highlight, the reserved mono status-tier color (count / P-IDs / run_id / SLO timings / fingerprints) | Linear's "monospace-as-a-typographic-tier" made a color — the Coverage matrix + Fingerprint identity; "blue focus" mapped to the ID-cyan tier (industry-rules) |
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
| Tertiary | `#838EBA` | `#565F89` | Metadata, `journal_emitted_at` timestamps, captions. Dark lifted from `#717AA0` so it clears 4.5:1 on every surface it renders on — `--color-raised-2` (coverage thead) and `--color-raised-1` (rows), not only `--color-base` (as measured 2026-09-01: one axe `color-contrast` violation, 18 nodes, all this token) |
| Muted | `#727EB5` | `#636882` | Disabled labels, placeholders, dimmed-on-abort text. Both themes moved to clear 4.5:1 against `--color-inset`, in OPPOSITE directions (dark lifted from `#565F89`, light darkened from `#6E7491`); it is its own recessive text tier and no longer shares a hex with `--count-blocked` |

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

Retired 2026-10-11 with the window (its faces, type scale and font loading are last at `321dc8f`). The cli sets no face: the terminal's own monospace carries all output, brand expressed through ANSI color, the ASCII prefixes and header weight (§Surface: cli).

---

## Spacing

Retired 2026-10-11 with the window (the 4px spacing scale is last at `321dc8f`). For cli, spacing is terminal-controlled — `comfy-table` column alignment and `indicatif` layout, from the detected terminal width, never hardcoded.

---

## Depth Strategy

Retired 2026-10-11 with the window (borders-only elevation is last at `321dc8f`); the cli has no depth.

---

## Border Radius

Retired 2026-10-11 with the window (the radius tokens are last at `321dc8f`); the cli has none — terminal-controlled.

---

## Motion (calibrated to expression level `0.3`)

Retired 2026-10-11 with the window (the CSS motion tokens and transitions are last at `321dc8f`). The cli's motion is honest progress only: the `indicatif` spinner appears after ~200ms and STOPS in place, never hides, at an operator hold; no pulse, blink or flashing on any status.

---

## Iconography

Retired 2026-10-11 with the window (its icon library and SVG lamp glyphs are last at `321dc8f`); the cli's status glyphs are its ASCII prefixes and `✓`/`✗`/`?`/`~`/`•`/`→` (§Surface: cli).

---

## Surface: desktop-webview

Retired 2026-10-11 (`2026-10-11-conductors-window-retired`, intent R1): Conductor has no window. This surface's tokens, components, navigation and platform notes are last at `321dc8f`.

---

## Surface: cli

**Platform:** Windows / macOS / Linux terminal — headless `conductor-cli` (`scripts/agent-run.sh`), the source of truth + release gate. Line-oriented (ratatui full-TUI deliberately omitted).

**Toolkit / Framework:** clap 4.5 + `owo-colors` 4.x with `std::io::IsTerminal` (styling, TTY-gated per stream) + `indicatif` 0.18 (live counters / spinner) + `comfy-table` 7 (P-ID SLO tables) + `inquire` 0.9 (operator-pause prompts).

### Tokens (platform-specific)

ANSI 256-color codes mapping the Color World (per the library-shortlist mapping), applied via `owo-colors` behind a `std::io::IsTerminal` check — decided independently for stdout and stderr, each requiring a terminal AND `NO_COLOR` unset AND `TERM != dumb`, so output carries no ANSI when piped:

```
Slate ground   #1A1B26  → ANSI 234/235  (backdrop; rarely set — terminal-owned)
Nominal green  #7EE787  → ANSI 114      (Pass / on-timeline heartbeat)
Hold amber     #E3B341  → ANSI 179      (HOLD / CalibrationRegion)
Fail red       #F85149  → ANSI 203      (Fail — no blink)
Blocked violet #565F89  → ANSI 60       (Blocked — never measured)
Manual lavender#A9B1D6  → ANSI 146      (ManualCheck — awaiting operator; paired with [MANUAL] + ? glyph)
Residual mute  #9A93A8  → ANSI 246      (KnownResidual — pre-accepted gap; paired with [RESIDUAL] + ~ glyph)
                                        ALSO the shared recessive tier for every NON-lamp use — currently
                                        the `hint:` stderr label (Component Pattern 5), the coverage-matrix
                                        out-of-scope Mode cell (`not-conductors`), and the run-level
                                        `[ENVIRONMENT-SUSPECT]` load-envelope caption and the
                                        `[PRECONDITION]` scheduling caption. None is a lamp
                                        state: the always-rendered text label carries the signal, the tint
                                        only de-emphasizes. Markdown, having no color channel, uses
                                        emphasis (`_not-conductors_`) or the bracket label in a blockquote
                                        as the surface-adapted counterpart.
Mono ID cyan   #7DCFFF  → ANSI 117      (P-IDs / run_id / SLO timings / fingerprints)
Journal text   #A9B1D6  → ANSI 146      (report rows / dimmed metadata)
```

Header style: bold + ANSI 117 (ID-cyan) for section titles; metadata dimmed. Status prefixes are ASCII text + color (never color alone) — the closed per-P-ID set `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`, plus the run-level non-lamp caption SET — `[ENVIRONMENT-SUSPECT]` (load-envelope) and `[PRECONDITION]` (the `conductor preconditions` probe), both ANSI 246 and both outside the lamp column, neither a lamp state nor a `ReportState` and `✓`/`✗`/`?`/`~`/`•`/`→` (TTY only — never emoji in machine-parseable piped output; `?` = `ManualCheck` awaiting operator, `~` = `KnownResidual` pre-accepted gap).

### Component Patterns

1. **Paused-count hold-point (signature, CLI mirror — primary placement).** During a run, an `indicatif` progress/spinner carries the count/heartbeat with a colored phase-line prefix (ANSI 114 green). On the operator-pause go/no-go, the spinner **STOPS in place (not hides)** at the exact value, and a bold ANSI 179 amber `HOLD — operator pause` phase line prints **above** the `inquire` prompt. Spinner appears only after ~200ms (honest progress, not instant churn). The stop — not a hide, not an animate-to-100% — is the signature; it embodies "paused count shows where it stopped." TTY-gated so agent-captured artifacts stay clean.

2. **Operator-pause prompt.** `inquire` proceed/abort confirm, gated behind an `isatty` check. **The headless path is NEVER blocked on an interactive prompt** — when stdin is not a TTY (agent-driven `agent-run.sh`), the prompt is skipped per the configured non-interactive policy and the decision is recorded to the artifact; an interactive operator gets the colored confirm. Proceed/abort decision logged to the run report.

3. **Results / SLO table** (the `run` + `suite` per-check table). `comfy-table` 6 columns: P-ID (ANSI 117 cyan) · scenario · `state` (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` colored prefix) · `slo_tier` (`<5s`/`<20s`/`<90s`) · `latency_ms` (cyan, right-aligned) · fingerprints. Width detected dynamically from the terminal (never hardcoded). A `Blocked` row carries the named precondition string; measurement columns render as `—`/null, never a red error. **The coverage matrix is a SEPARATE, narrower table** — `conductor coverage` renders 4 columns (P-ID · Title · Category · Mode), carries no verdict/state column and therefore no bracket prefix, and terminates in a roll-up caption (§cli Primary screens).

4. **Verdict / report-state lines.** Per-P-ID result printed in place: `✓ P-009  Pass   1840ms <slo_tier>` (green; the tier cell renders the scenario's declared `slo_tier` from the closed `<5s`/`<20s`/`<90s` set — never a baked per-scenario literal) / `✗ P-014  Fail   …` (ANSI 203 red, no blink) / `? P-035  Manual  halo→burgundy? · no OS toast?` (ANSI 146 lavender — `[MANUAL]`, an operator-checklist item with no machine verdict; TTY: `inquire` y/n, headless: recorded unconfirmed) / `~ P-032  Residual  recent_commits stub → v0.3.0` (ANSI 246 muted — `[RESIDUAL]`, a pre-accepted gap, never red) / `• P-022  Blocked  mcp-server feature + ANDROMEDA_PULSE_MCP_ENABLED + matching data-dir` (ANSI 60 violet). Color is always paired with the text prefix (`[PASS]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`/…) for NO_COLOR + screen-reader friendliness. A run whose profile breaches the SUT load envelope additionally prints the run-level `[ENVIRONMENT-SUSPECT]` caption once, above these lines — a qualifier on the run, never a seventh lamp or a sixth `ReportState`.

5. **Error output.** To stderr, sanitized (no absolute host paths / internal struct names / stack traces per security plan): `error: <short>` + contextual detail + `hint: <fix>`. The `error:` label reuses Fail red (ANSI 203), `hint:` the Residual mute (ANSI 246) — tty-gated on stderr (its own `IsTerminal` gate, distinct from the stdout gate), never new colors. Never colorized when piped; the ASCII `error:`/`hint:` labels always stand (never color-alone); stack traces only under `--debug`/`-v`.

### Navigation Pattern

Linear, top-to-bottom stdout — no cursor manipulation, no full-screen redraw (ratatui omitted). The operator runs `conductor-cli` / `agent-run.sh`, reads colored headers → live `indicatif` heartbeat → `inquire` operator-pause holds (interactive TTY only) → `comfy-table` coverage/SLO summary → run-report path. Pipe-friendly: raw artifact data parseable, human messages colored on stderr, the headless agent path strips ANSI and never blocks.

### Platform-Specific Notes

- Always respect `NO_COLOR`, `TERM=dumb`, and piped-stdout ANSI stripping (the shipped per-stream `std::io::IsTerminal` gate). Avoid dark-blue-on-black / dark-red-on-black (ANSI 117 cyan + 114 green chosen for contrast on dark terminals).
- Don't rely on color alone — every status carries an ASCII prefix (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`).
- Interactive `inquire` prompts ALWAYS check `isatty` first; the agent-driven headless source-of-truth path is never gated on a prompt (an auth/interactive prompt there would silently break the release gate — security plan).
- Detect terminal width dynamically for `comfy-table`; never hardcode widths or wrap at arbitrary points.

---

## Anti-Patterns (NEVER do these)

### Universal Bans
- NEVER use purple-gradient-on-white as a color scheme — Conductor's ground is cool Tokyo-Night slate `#1A1B26`; the indigo/violet "AI gradient" is an explicit Rejected Default.
- NEVER use cookie-cutter card grids without adapting card internals to content — the Coverage matrix is a dense single-row-per-P-ID list, not a tile wall.
- NEVER use the same layout for different information types — the heartbeat/phase-line, coverage matrix (dense list), and run-report each get purpose-built layout.
- NEVER use color purely for decoration — every color communicates Verdict / ReportState (green=Pass, amber=HOLD/CalibrationRegion, red=Fail, slate-violet=Blocked, ID-cyan=mono status tier).
- NEVER converge on common "safe" choices across generations — anchor to THIS domain (mission-control console, frozen heartbeat).
- NEVER give a credential a styled surface (a password / token / key field, a masked-secret chip, a "copy key" affordance) on either surface — Conductor owns no credentials, so no cli output renders, masks or styles one.

### Rejected Defaults (from exploration)
- **A "dashboard" grid of KPI metric cards with big-number tiles + trend sparklines** — rejected because: every OTLP/telemetry tool in training data is a Grafana/Datadog tile wall; Conductor is a single-station Mission-control console ("a control surface, not a dashboard") — one heartbeat + phase line in the titlebar, the Coverage matrix as a dense single-row-per-P-ID list.
- **A flashing / pulsing red banner or animated alert toast on a Fail verdict** — rejected because: alarm = red + motion is the universal reflex, but the personality is "calm under load, no alarm"; Fail is an in-place, motionless `#F85149` verdict lamp on the P-ID row. Conductor must NOT emit native OS toasts (those are Pulse behavior it observes).
- **A determinate progress bar that hides or animates-to-100% during the operator-pause** — rejected because: the signature is the *frozen* heartbeat — the count freezes at the exact hold value and tints amber (CLI: `indicatif` stops-not-hides), embodying "paused count shows where it stopped." Hiding/animating destroys the signature.
- **Indigo/violet "AI gradient" accent + glassmorphism cards** — rejected because: it is the current statistical-average "smart tooling" aesthetic; Conductor uses the cool slate ground with a strict green/amber/red functional status tier and flat inset-border elevation (`#2A2E42`), color-as-state not mood.
- **Conflating "no result yet" with "failed" (graying both the same, or showing Blocked as a red error)** — rejected because: empty-state and error-state are usually collapsed; `Blocked` is a distinct slate-violet (`#565F89`) state carrying the named precondition string — present-but-greyed, *never measured*, never a silent downgrade to red Fail.
- **A monospace font used everywhere for "techy dev-tool vibes"** — rejected because: mono-everything is lazy shorthand; mono is reserved strictly as a status tier (ID cyan `#7DCFFF`) for the count, P-IDs, run_id, SLO timings, fingerprints — UI sans (IBM Plex Sans) for phase line + prose.

### Per-Surface Bans

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

History: design-system-amendments.md (live); the log moved verbatim to design-system-amendments-archive.md (U35).

