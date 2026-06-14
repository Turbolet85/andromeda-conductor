# Library Shortlist — Conductor

## Color Palettes

(Top 3, anchored to quiz mood + exploration color world. All three are dark-native dev-tool grounds with a green/amber/red functional status tier — the closest library bases to the Tokyo-Night Color World; Phase 4 customizes hexes toward the exact anchors, deriving the mandated light variant per Auto mode.)

### 1. Developer Tool / IDE (SaaS & Software #81)

- **Primary:** #1E293B | **Secondary:** #334155 | **Accent:** #22C55E
- **Background:** #0F172A | **Card:** #1B2336 | **Border:** #475569
- **Match reason:** This is the single closest library base to the project. Quiz Q1/Q3 demand a "control surface, not a dashboard" on a cool blue-cast slate ground with a green run-status tier; the exploration's slate ground `#1A1B26` and inset-border `#2A2E42` map almost directly onto this entry's slate-blue `#0F172A` bg + `#1B2336` card (the same blue-cast charcoal family, not a neutral grey), and the "Code dark + run green" accent `#22C55E` is the library's nearest match to the nominal-green `#7EE787` Pass/heartbeat tint. The slate-blue primary/secondary (`#1E293B`/`#334155`) supply the flat inset-border elevation Linear-style register the exploration calls for. Phase 4 shifts the bg from `#0F172A` to the warmer-blue `#1A1B26` and adds the amber/red/violet status tiers (this base only ships green). **CLI (ANSI 256) mapping of the Color World it represents:** slate ground `#1A1B26`→ANSI 234/235; nominal green `#7EE787`→ANSI 114; hold amber `#E3B341`→ANSI 179; fail red `#F85149`→ANSI 203; blocked slate-violet `#565F89`→ANSI 60; mono ID cyan `#7DCFFF`→ANSI 117; journal text `#A9B1D6`→ANSI 146 — the green/amber/red triad is the owo-colors status-tier convention mirrored from k9s.

### 2. Financial Dashboard (Finance & Fintech #6)

- **Primary:** #0F172A | **Secondary:** #1E293B | **Accent:** #22C55E
- **Background:** #020617 | **Card:** #0E1223 | **Border:** #334155
- **Match reason:** Selected from a *different* category to avoid three near-identical SaaS slates, yet it carries the same functional intent the quiz wants: "Dark bg + green positive indicators" is precisely the k9s "healthy = green status-tier" convention from Q3, and the deep blue-black `#020617` ground + `#0E1223` card are the darkest blue-cast option in the library — a strong reference for the near-black frameless-titlebar ground the exploration's Mission-control console anchor describes (`#1A1B26` with a faint blue cast). Its `#334155` border models the barely-there inset divider (`#2A2E42` in the Color World). Green-as-status (not green-as-brand) matches the "color reads as Verdict / report state, not mood" Defaults-to-Reject rule. Phase 4 lifts the ground one notch toward `#1A1B26` (Conductor's slate is slightly warmer/lighter than this trading-terminal black) and layers in amber/red/violet.

### 3. Smart Home / IoT Dashboard (Emerging Tech #24)

- **Primary:** #1E293B | **Secondary:** #334155 | **Accent:** #22C55E
- **Background:** #0F172A | **Card:** #1B2336 | **Border:** #475569
- **Match reason:** Chosen as the "live-equipment / status-light" reference rather than a code-editor one: "Dark tech + status green" is the closest library analog to Conductor's signature behavior — a console of in-place status indicators turning green/amber under load (the verdict lamp + paused-count). The IoT framing matches the exploration's "state changes read as live equipment" (Q4) and the single-station Mission-control console better than a generic SaaS palette, while sharing the same blue-cast slate ground (`#0F172A`/`#1B2336`) and green status accent the other two use, so the three stay coherent. Note: this and #81 share hexes (both are slate+run-green dev grounds); they are kept as distinct *reasoning anchors* (IDE-density vs. live-status-board) for Phase 4, which will diverge them via the amber/red/violet tiers. Honors the Minimal security tier (no conservative-palette constraint applies — but all three are high-contrast WCAG-friendly dark grounds regardless).

## Font Pairings

(Top 3, anti-pattern-checked. **Surface note — desktop-webview is primary:** these drive the Tauri webview. The Q2 Linear anchor uses **Inter**, which is BANNED — none is carried here; #1 below is the explicit metric-equivalent replacement. The exploration mandates a self-hosted WOFF2 mono *status tier* (count / P-IDs / run_id / SLO timings / fingerprints), NOT mono-everywhere, paired with a UI sans for the phase line + prose. **CLI surface:** uses terminal monospace (user-configured); Conductor mirrors the same self-hosted mono status tier via owo-colors — brand is expressed through color usage + output structure, not webview typography.)

### 1. JetBrains Mono + IBM Plex Sans (Developer Mono #9)

- **Mood:** code, developer, technical, precise, functional
- **Heading / Mono tier:** JetBrains Mono, 400/500, monospace — the reserved status tier (count, P-001..P-060, run_id, SLO timings, fingerprints)
- **Body / UI:** IBM Plex Sans, 300–700, sans-serif — phase line + prose
- **Google Fonts:** https://fonts.google.com/share?selection.family=IBM+Plex+Sans:wght@300;400;500;600;700|JetBrains+Mono:wght@400;500;600;700
- **Match reason:** This is the exploration's typography spec made literal. The exploration's `typography-stack-install` names "JetBrains Mono / IBM Plex Mono via Fontsource" for the self-hosted mono tier, and JetBrains Mono is exactly this pairing's mono face; IBM Plex Sans is the IMPORTANT-CONTEXT-suggested Inter replacement (similar humanist-grotesque metrics, self-hostable via Fontsource for the Minimal-tier offline/no-CDN mandate). It directly serves Linear's "monospace-as-a-typographic-tier" (Q2) and the Defaults-to-Reject "mono reserved strictly as a status tier, UI sans for prose" rule. Both self-host as WOFF2 — no runtime CDN.
- **Anti-pattern check:** PASS — JetBrains Mono and IBM Plex Sans are both clear of the ban list (Inter/Roboto/Arial/Helvetica/Open Sans/Lato/system-ui/Space Grotesk). This pairing is also the sanctioned replacement for the banned-Inter Linear anchor.

### 2. Exo + Roboto Mono (Science/Tech #47) — body-font swap required

- **Mood:** science, technology, data, precise (reads as instrument/telemetry)
- **Heading / UI:** Exo, 300–700, sans-serif — modern technical phase-line face
- **Body / Mono tier:** Roboto Mono → **replace with JetBrains Mono or IBM Plex Mono**
- **Google Fonts:** https://fonts.google.com/share?selection.family=Exo:wght@300;400;500;600;700|Roboto+Mono:wght@300;400;500;700
- **Match reason:** Offered as the "telemetry instrument" alternative to #1 — the mood (science / research / data / precise) maps to the NASA Open MCT mission-control-telemetry vocabulary the personality borrows (Q2) and the SLO-timing / fingerprint data tier. Exo gives a slightly more technical phase-line voice than IBM Plex Sans without tipping into sci-fi glow.
- **Anti-pattern check:** Exo is clear. **Roboto Mono is NOT on the literal ban list, but "Roboto" is the Google-default family the bans target — and the exploration explicitly self-hosts JetBrains Mono / IBM Plex Mono.** Replace Roboto Mono with **JetBrains Mono** (or IBM Plex Mono) to keep the mono status tier on the mandated self-hosted face. With that swap: PASS.

### 3. Share Tech Mono + Fira Code (Tech/HUD Mono #51) — mono-tier source only, demote heading

- **Mood:** tech, hud, data, monospaced, precise
- **Heading / HUD:** Share Tech Mono, 400, monospace — classic sci-fi/HUD mono (use sparingly, if at all)
- **Body / Mono tier:** Fira Code, 300–700, monospace — viable self-hosted mono-tier alternative to JetBrains Mono
- **Google Fonts:** https://fonts.google.com/share?selection.family=Fira+Code:wght@300;400;500;600;700|Share+Tech+Mono
- **Match reason:** Listed for its HUD/mission-control register ("Best for: ... developer tools, dashboards") which rhymes with the Open MCT countdown-clock heartbeat, and because Fira Code is a solid second-choice for the self-hosted mono status tier (ligatures off for ID legibility). **Caveat for Phase 4:** the quiz personality is "calm under load, no alarm" / "exact downbeat, held silence" — the sci-fi-HUD flavor of Share Tech Mono risks reading as alarm-y / retro-Apollo, which the user explicitly *overrode away from* in Q3. Prefer pairing #1; if a more technical mono is wanted, take Fira Code as the mono tier and keep a humanist UI sans (IBM Plex Sans) for prose rather than adopting Share Tech Mono as a display face.
- **Anti-pattern check:** PASS on the ban list (Share Tech Mono, Fira Code both clear). Soft flag: HUD display mono conflicts with the "no alarm, no retro-Apollo" quiz direction — demote to mono-tier-only.

## Design Direction

- **Primary direction:** Precision & Density — "Cold like a terminal, dense like a trading floor." Best-for line literally reads "Developer tools, trading platforms, analytics, **monitoring**"; references are **Linear, Raycast, Bloomberg Terminal** — the exact Q2 desktop-webview anchor (Linear) and the instrument-panel-density register. Monospace-or-tight-sans + tabular numbers, 4px base, **borders-only no shadows**, **monochrome with functional color only (red/green for data)** — a one-to-one fit for the slate ground + flat inset-border (`#2A2E42`) + green/amber/red status tier and the reserved mono ID cyan tier.
  - **Secondary direction (merge):** Data & Analysis — "Like a **mission control center**, information-rich but organized... **Dark mode default**, borders for definition... color-coded data." This is the literal Q1 "single-operator mission-control console" + Open MCT vocabulary and the Auto dark-default mandate. Merge: Precision & Density supplies the *density/elevation/type* spine; Data & Analysis supplies the *mission-control color-coded-status + dark-default* framing. (Utility & Function — Vercel/Supabase/GitHub, "neutral with semantic colors for status only" — is the tertiary touchpoint for the internal-tool restraint.)
- **Style preset:** Dark Mode (OLED) (#7) as the base register, structured by Real-Time Monitoring (#31) for the live status-light semantics.
  - From **Dark Mode (OLED)**: dark-native, high-contrast (7:1+) text, **minimal glow / low white emission**, visible focus, `color-scheme: dark`. Honor the Auto-mode caveat — this preset is dark-only, so Phase 4 derives the light variant separately (every Color World entry already carries a light value).
  - From **Real-Time Monitoring**: "live **status indicators**, alert colors critical(red)/warning(amber)/normal(green), connection status" — the verdict-lamp + count-heartbeat semantics. **Reject this preset's pulsing/blink/glow animation defaults** — they violate Q4's 0.3 functional-motion budget and the Defaults-to-Reject "no flashing/pulsing red alert." Conductor's status changes are *in-place CSS color transitions only*; the absence of motion is the signature.
- **Key properties:**
  - **Keywords:** terminal density, instrument-panel, mission-control, dark-default, tabular numbers, functional-color-only, status lights, monospace status tier
  - **Effects:** in-place status-light color transition (green→amber→red→slate-violet), subtle hover, fades; **no** pulse/blink/glow/parallax/staggered entrance/framer-motion (0.3 budget); `prefers-reduced-motion` honored
  - **Color focus:** cool blue-cast slate ground; one functional green/amber/red/slate-violet status tier reading as Verdict/ReportState (never as mood); mono ID cyan as a fourth reserved typographic-color tier; **explicitly reject** AI-indigo gradient + glassmorphism + KPI-card grid (Defaults to Reject)
  - **Depth approach:** borders-only — flat inset-border elevation (`#2A2E42`) on the near-black ground, zero drop shadows

## Industry Rules

(Product type = developer tool / control-surface harness. Merged from the two closest entries — **Developer Tool / IDE** (primary) + **Cybersecurity Platform** (for the dark-default + real-time-status-monitoring discipline) — with web-specific rules filtered out for the desktop-webview + CLI surfaces.)

- **Product type match:** Developer Tool / IDE (#81), merged with Cybersecurity Platform (#80)
- **Primary style:** Dark Mode (OLED) + Minimalism (from #81). Reinforced by #80's "Cyberpunk UI + Dark Mode (OLED)" dark-default discipline — **but adopt only the dark-default + terminal-feel, NOT the cyberpunk neon/glitch** (conflicts with the "calm under load, no alarm" quiz personality and the user's Q3 override away from retro/neon).
- **Dashboard style:** Real-Time Monitor + Terminal (from #81); #80 adds "Real-Time Monitoring." **Reframe per exploration:** this is a single-station *control surface*, not a multi-widget dashboard — the Coverage matrix is a dense single-row-per-P-ID list (Linear instrument-panel density), and the heartbeat/phase-line live in the frameless titlebar. Honor Defaults-to-Reject: **no KPI-card grid, no heat-map wall.**
- **Color focus:** "Dark syntax theme colors + Blue focus" (#81). Map "blue focus" to the exploration's mono **ID cyan `#7DCFFF`** reserved tier; #80's "Matrix Green + Terminal feel" maps to the **nominal-green `#7EE787`** status tier (the k9s healthy convention). The functional status triad (green/amber/red) + blocked slate-violet is Conductor's specialization of the generic "dark syntax theme."
- **Anti-patterns (industry-specific bans to honor):**
  - **Light mode default** (#81, #80) — Conductor is dark-default (Auto mode ships light as the OS-following variant, never the default).
  - **Slow performance** (#81) — the ~3 MB Tauri artifact + cold-start target; Preact 10 is the documented size escape hatch (settled in Phase 1 — do not re-litigate).
  - **Poor data viz / poor threat visualization** (#80) — but DO NOT over-correct into a Grafana/Datadog tile wall (project's own Defaults-to-Reject overrides the generic "more viz" industry reflex).
  - **(From exploration, carried as binding industry anti-patterns):** no AI-indigo gradient + glassmorphism; no flashing/pulsing red on Fail; no progress bar that hides/animates-to-100% on the operator-pause (it must freeze in place); never gray "no result yet" the same as "failed" (Blocked = distinct slate-violet, never red).
- **Key considerations:**
  - **Keyboard shortcuts** (#81) — first-class; the operator drives start/stop/proceed/abort; Radix/shadcn primitives must expose keybindings (mirrors k9s/lazygit pinned hint row).
  - **Fast performance** (#81) — zero animation library, CSS-transition-only motion, offline-bundled static Tailwind stylesheet (no CDN).
  - **Syntax-highlighting / command-palette heritage** (#81) — translate to the mono ID-cyan status tier + the scenario/suite picker, not literal code syntax highlighting.
  - **Real-time monitoring + threat display** (#80) — translate to the live emission counters / target status over the Tauri `Channel` and the in-place status-light verdict lamp (calm, not alarmist).
  - **Dark mode default** (#80) — confirmed; pair with the mandated light variant for Auto mode.

## UX Guidelines

(Top 10, severity-prioritized, surface-filtered for desktop-webview primary. Mobile/touch-only and web-marketing guidelines excluded. Each is high-relevance to the control surface's core interactions: operator-pause confirm, live status, keyboard-driven run control, motion restraint.)

| # | Guideline | Severity | Platform | DO | DON'T |
|---|---|---|---|---|---|
| 1 | **Confirmation Dialogs** (#35) — gates the operator-pause go/no-go hold, the core staged action | high | all | Confirm before irreversible/committed actions (the operator-pause before each committed timeline step) | Proceed/commit a step without an explicit operator confirm |
| 2 | **Reduced Motion** (#9) — binding with the 0.3 budget + signature freeze + a11y SC 2.3.3 | high | all | Check `prefers-reduced-motion: reduce` and drop the count-tint CSS transition | Ignore motion settings; force any transition on a held/frozen count |
| 3 | **Color Only** (#37) — verdict/status must not be color-alone | high | all | Pair every status tint with text/icon (Pass/HOLD/Fail/Blocked label + lamp glyph) | Convey Pass/Fail/Blocked by green/amber/red/violet color alone |
| 4 | **Color Contrast** (#36) — dark-ground legibility | high | all | Hold ≥4.5:1 for journal text + status text on the `#1A1B26` slate ground (and the light variant) | Low-contrast grey-on-slate metadata that fails on the dark ground |
| 5 | **Focus States** (#28) — keyboard-driven console | high | all | Visible focus ring on every interactive control (picker, start/stop, proceed/abort) | Remove the focus outline without an equivalent indicator |
| 6 | **Error Feedback** (#33) — Fail/Blocked must be legible, not silent | high | all | Show the verdict in place on the P-ID row + the named Blocked-precondition string | Silent failure, or a generic "Something went wrong" with no precondition |
| 7 | **Error Messages (announced)** (#44) — a11y for verdict changes | high | all | Use `aria-live`/`role=alert` so a Fail/Blocked verdict + HOLD phase flip are announced | Visual-only status change with no screen-reader announcement |
| 8 | **Loading States / Indicators** (#10 / #78) — honest progress, not frozen UI | high | all | Show count/heartbeat progress; on hold the count **freezes in place** (CLI: indicatif stops-not-hides) | Leave the surface frozen with no feedback, or hide/animate progress to 100% on pause |
| 9 | **Loading Buttons** (#32) — prevent double-commit during async run control | high | all | Disable start/proceed and show in-flight state during the async step | Allow repeated clicks on start/proceed while a step is processing |
| 10 | **ARIA Labels** (#40) — icon-only frameless-titlebar + lamp controls | high | all | `aria-label` the min/close titlebar buttons, hold/proceed/abort icons, and status-lamp glyphs | Icon-only buttons (Lucide) with no accessible name |
