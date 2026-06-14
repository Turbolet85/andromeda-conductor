## 1. Domain Concepts

- **Mission-control console** — the single-operator station beside one live Pulse: a status board, a count clock, and go/no-go hold prompts, not a team war-room dashboard.
- **Seeded timeline** — the deterministic score a run plays: a fixed sequence of phases / cues / rests under a seed, where the same scenario + seed always yields the same emission stream shape.
- **Phase line** — the named operational segment currently executing (e.g. `error-baseline-spike`, `fingerprint-storm`, `restart-suppression`), borrowed from NASA Open MCT's "name the phase as one prominent line."
- **Count / heartbeat** — the run's single prominent mono tick (run-elapsed / step index) at the top of the console; the live pulse that proves the run is alive and moving.
- **Operator-pause / go-no-go hold** — the staged "hold before you commit the next timeline step" prompt that gates every committed step; the human answers proceed/abort at a station.
- **Emission journal** — the on-disk JSONL ground truth: every span/metric/log Conductor emits, timestamped at `journal_emitted_at`, the source of truth the SQLite index merely indexes.
- **Cues / rests / ramps / silence** — the timeline's musical-mechanical primitives: a fault cue fired, a deliberate silence held, a load ramped up or down at exact `tokio::time` gaps.
- **Fingerprint / fingerprint storm** — an incident's identity hash; a storm is a burst of distinct fingerprints flooding the read-back corpus (P-036 "Previously seen" recurrence check).
- **SLO tolerance band / calibration region** — the tier-scaled slack window (`<5s` / `<20s` / `<90s`) around a journal-relative deadline; the calibration region is the soft bucket where model-interpretive timings land instead of hard-failing.
- **MCP read-back preflight / canary** — the once-at-suite-start readiness gate: negotiate protocol `2024-11-05`, assert tool presence, round-trip a canary incident through the shared corpus to prove data-dir wiring before any scenario trusts read-back.
- **Blocked precondition** — the distinct go/no-go-failed state (never a silent downgrade to pass/fail): the named precondition string (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED` + matching data-dir) that must be satisfied before a measurement is even attempted.
- **Coverage matrix** — the wall of all 60 Pulse capabilities (P-001..P-060) classified; zero gaps is the definition of done, "no scenario without a P-ID."
- **Verdict / report states** — the typed outcome lamps: `Verdict { Pass / Fail / CalibrationRegion }` and `ReportState { Pass / Fail / ManualCheck / KnownResidual / Blocked }` — outcomes are values matched on, never caught as errors.
- **Port-occupier fault** — the single deliberate port bind: Conductor seizes `:4317` to exercise Pulse's reaction — the one moment the loopback client becomes a squatter on the ingest port.
- **Run report artifact** — the per-run `<run_id>.md` + `runs.db` row: the agent-parseable envelope (run_id · seed · scenario · p_ids · verdict · slo_tier · latency_ms · fingerprints), the durable record of one mission.

## 2. Color World

- **Slate ground #1A1B26** — the Warp / Tokyo-Night charcoal with a faint blue cast; the console's *mission-control console* surface and frameless titlebar ground (anchors: Mission-control console). Light variant (Auto mode): `#F4F5F8` cool off-white, the same console under room light.
- **Nominal green #7EE787** — the k9s "healthy" status-tier green; the *count / heartbeat* tint while on-timeline and the `Pass` *verdict lamp* (anchors: Count / heartbeat, Verdict states). Light variant: `#1A7F37` (legible green on the off-white console).
- **Hold amber #E3B341** — the k9s "degraded" amber a resource turns *in place* under load; the frozen-count tint and "HOLD — operator pause" *phase line* at the *operator-pause / go-no-go hold*, and the `CalibrationRegion` lamp (anchors: Operator-pause / go-no-go hold, SLO tolerance band / calibration region). Light variant: `#9A6700`.
- **Fail red #F85149** — the reserved failing-red completing the green/amber/red status tier; the `Fail` *verdict lamp* and the *port-occupier fault* alarm edge, held muted (no flashing) per "calm under load, no alarm" (anchors: Verdict / report states, Port-occupier fault). Light variant: `#CF222E`.
- **Blocked slate-violet #565F89** — the desaturated Tokyo-Night comment/muted hue; the `Blocked` *go/no-go-failed* state and dimmed-on-abort count — present-but-greyed, distinct from red Fail because it was never measured (anchors: Blocked precondition, MCP read-back preflight / canary). Light variant: `#6E7491`.
- **Journal line-text #A9B1D6** — the Tokyo-Night foreground grey-lavender; the mono body of the *emission journal* / *run report artifact* rows and dimmed metadata, the readable-but-recessive text tier (anchors: Emission journal, Run report artifact). Light variant: `#343B58`.
- **Mono ID cyan #7DCFFF** — the Tokyo-Night cyan reserved strictly for the monospace status tier — *P-IDs* (P-001..P-060), `run_id` stamps, SLO timings, *fingerprints* — Linear's "monospace-as-a-typographic-tier" made a color (anchors: Coverage matrix, Fingerprint / fingerprint storm). Light variant: `#0969DA`.
- **Inset-border line #2A2E42** — the near-invisible flat divider one notch above the slate ground; Linear's "flat inset-border elevation on a near-black ground" — the seams between *phase line*, *count*, and *coverage-matrix* rows with no drop shadows (anchors: Mission-control console, Coverage matrix). Light variant: `#D8DBE6`.

## 3. Signature Element

- **Element name:** "Paused-count hold-point — the frozen heartbeat at the operator-pause"
- **What it is:** A single prominent mono count (run-elapsed / step index — the run's heartbeat) ticks in place at the top of the control surface during a run, tinted nominal-green (`#7EE787`). When an operator-pause prompt fires before a committed timeline step, the count does NOT blank and does NOT keep running — it **freezes at the exact hold value**, the phase line beside it flips to "HOLD — operator pause" and the count tints green-nominal → hold-amber (`#E3B341`) for the duration of the hold. On proceed it resumes from the frozen value; on abort it stops and dims to blocked slate-violet (`#565F89`). The absence of motion is the signature — it is the literal "paused count shows where it stopped."
- **Implementation notes:** A React 19 component holds run-elapsed state fed by the Tauri 2 IPC `Channel`; the count renders in the reserved mono face inside the custom frameless titlebar (`decorations: false` drag region). The freeze is just halting the tick interval and swapping a Tailwind v4 `@theme` text-color token (a CSS color transition at most — zero animation library, within the 0.3 expression budget); no shadcn/ui component beyond the `inquire`-equivalent operator-pause dialog is required. CLI mirror: an `indicatif` 0.18 progress/spinner that **stops (not hides)** at the hold, plus a colored `owo-colors` "HOLD" phase line above the `inquire` 0.7 operator-pause prompt, TTY-gated via `anstream` so agent-captured artifacts stay clean.
- **Domain tie:** Embodies the Operator-pause / go-no-go hold + Count / heartbeat concepts in one element — mission-control patience made visible, the frozen heartbeat at the go/no-go.

### Bootstrap phases (derive for route / setup-project)

The downstream skills derive the following bootstrap phases from the design exploration + downstream design-system.md / layout-templates.md:

- **design-tokens-bundle-init:** initialize the design token bundle (color / typography / spacing / motion / depth / radius) as Tailwind v4.1 `@theme` CSS custom properties (Oxide engine, static zero-runtime stylesheet bundled offline — no CDN). Must include the full Color World palette above (each entry's dark default + light variant per Auto mode) plus the Signature Element supporting tokens (`--count-nominal: #7EE787`, `--count-hold: #E3B341`, `--count-blocked: #565F89`).
- **typography-stack-install:** install the typography stack per design-system.md §Typography — a self-hosted WOFF2 mono face (e.g. JetBrains Mono / IBM Plex Mono via Fontsource, local-vendored, no runtime CDN per Minimal-tier offline hardening) for the reserved mono status tier (count / P-IDs / run_id / SLO timings / fingerprints), plus a UI sans for the phase line and prose, with a system fallback stack.
- **iconography-registry-install:** install the icon library + custom glyphs per design-system.md §Iconography — Lucide React (ships with shadcn/ui) for hold/proceed/abort controls and the min/close frameless-titlebar buttons, plus project-specific custom SVGs for the status-tier lamp states (Pass / CalibrationRegion / Fail / Blocked) registered as components.
- **component-library-install:** install the ARIA component library named in tooling-decisions — shadcn/ui (Radix UI Primitives 1.x, copy-paste locally vendored), vendoring only the handful the control surface needs (scenario/suite picker, start/stop, live-counter surface, run-report view, operator-pause dialog) — binding contract with the a11y harness.
- **motion-tokens-wire:** wire motion presets per design-system.md §Motion at the 0.3 functional-motion-only budget (CSS color transitions for the count tint + status-light changes, `indicatif` spinner honesty on CLI; no staggered entrances / parallax / framer-motion) plus the `@media (prefers-reduced-motion: reduce)` override — binding contract with a11y SC 2.3.3.
- **theme-provider-wire:** wire the theme provider for Auto mode (dark `#1A1B26` default + light variant following system) via `@media (prefers-color-scheme: dark)` + Tailwind `@theme` token overrides per scheme; both a dark and a light token value per Color World entry are mandatory (per Q6 downstream note).
- **chart-card-primitive-bootstrap:** bootstrap the paused-count heartbeat surface as the first chrome primitive — the frozen-count component lives in the frameless titlebar and is composed by every downstream surface (run-report view, live-counter panel, coverage-matrix header all reference its phase-line + count + status-tint contract).
- **surface-handoff-wire:** wire layout template materialization per Phase 8 layout-templates.md — the two detected surfaces (`desktop-webview` custom-frameless root component scaffold; `cli` line-oriented owo-colors/comfy-table/indicatif presentation), primary-screen mapping to the single-station console (no responsive breakpoints — desktop-bound, no web/mobile).

route uses this list to plan phase ordering (typically: design-tokens-bundle-init → typography-stack-install → iconography-registry-install → component-library-install → motion-tokens-wire → theme-provider-wire → chart-card-primitive-bootstrap → surface-handoff-wire). setup-project uses this list to materialize the design tokens config + component-library install + theme provider wiring before surface-specific implementation begins.

## 4. Defaults to Reject

- **Default:** A "dashboard" grid of KPI metric cards with big number tiles and trend sparklines.
  - **Why tempting:** Every telemetry/observability tool in training data is a grid of metric cards; "OTLP + status" reads as Grafana/Datadog to a statistical average.
  - **Replace with:** A single-station Mission-control console — one prominent Count / heartbeat + Phase line in the frameless titlebar, the Coverage matrix as a dense single-row-per-P-ID list (Linear "instrument-panel density"), not a tile wall. "A control surface, not a dashboard."

- **Default:** A flashing / pulsing red banner or animated alert toast when a verdict comes back Fail.
  - **Why tempting:** Alarm = red + motion is the universal "something broke" reflex; AI reaches for attention-grabbing animation.
  - **Replace with:** An in-place, motionless Fail-red (`#F85149`) *verdict lamp* on the P-ID row — calm under load, no alarm in the tone (Mission-control patience). Conductor must NOT emit native OS toasts (those are Pulse behavior it observes).

- **Default:** A determinate progress bar that hides/disappears or smoothly animates to 100% during the operator-pause.
  - **Why tempting:** Progress bars are the convention for "a long operation," and hiding them on pause feels tidy.
  - **Replace with:** The Paused-count hold-point — the heartbeat *freezes at the exact hold value* and tints amber rather than hiding (CLI: `indicatif` stops-not-hides), embodying the Operator-pause / go-no-go hold "paused count shows where it stopped."

- **Default:** Indigo/violet "AI gradient" accent and glassmorphism cards (the generic modern-SaaS look).
  - **Why tempting:** It is the current statistical-average aesthetic for any "smart tooling" product.
  - **Replace with:** The cool blue-cast Slate ground (`#1A1B26`) with a strict green/amber/red functional status tier and flat inset-border elevation (`#2A2E42`) — color reads as Verdict / report state, not mood (k9s-on-Warp dev-tool world).

- **Default:** Conflating "no result yet" with "failed" — graying both out the same way, or showing Blocked as a red error.
  - **Why tempting:** Empty-state and error-state are usually collapsed into one muted/red treatment.
  - **Replace with:** A distinct Blocked slate-violet (`#565F89`) state carrying the named Blocked precondition string — present-but-greyed and clearly *never measured*, anchored to the MCP read-back preflight / canary, never a silent downgrade to a red Fail.

- **Default:** A monospace font used everywhere for "techy developer-tool vibes."
  - **Why tempting:** Dev tools = monospace-everything is a lazy shorthand.
  - **Replace with:** Mono reserved strictly as a status tier (ID cyan `#7DCFFF`) for the Count / heartbeat, P-IDs (P-001..P-060), `run_id` stamps, SLO timings, and fingerprints — Linear's "monospace-as-a-typographic-tier" — with a UI sans for the phase line and prose.
