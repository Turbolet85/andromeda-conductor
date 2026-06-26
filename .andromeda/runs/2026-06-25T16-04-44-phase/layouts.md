# layouts extract

## Relevance
partial — this chunk adds the picker + run-control UI to the desktop-webview surface, but does not fully implement the coverage matrix, run report, operator-pause dialog, or live-counter Channel

## Constraints
- (layout-templates §Wireframe — Run console (idle)) Desktop is a single frameless window with NO router / breakpoints / browser nav; four run states (idle / live / HOLD / report) are the SAME window in different visual states — the picker + Start/Stop controls live in the control row below the titlebar, above the coverage matrix
- (layout-templates §Component — Primary navigation) Start/Stop are semantic `<button>` controls (not custom elements) wired to `#[tauri::command]` calls; Start enabled only with a selection, Stop only while a run is in flight; picker is fully keyboard-operable with Radix roving focus + visible focus ring; no HTML links, no browser-style back/forward
- (layout-templates §Component — Primary navigation (single-station run control)) Keybindings are first-class and exposed through Radix primitives; Start / Stop each bound to a shortcut (mirroring k9s / lazygit keyboard-first register); the focus ring is visible on every interactive control, and active picker item carries a `border-emphasis` edge so focus is never color-only
- (layout-templates §Wireframe — Run console (idle), control row) Picker is shadcn Command/Select (`color-raised-2`) + Start/Stop Buttons (`radius-sm`); padding `space-sm`; each picker row carries its title + P-ID(s) (text-first, not-color-alone) — exactly which shadcn components (Command vs Select) is a P4 decision
- (layout-templates §Component — Primary content block 1) Coverage matrix and run-report views are NOT rendered in ch3; their layouts are deferred to ch6/ch7; this chunk only stands up the picker+controls that PRECEDE them in the idle state
- (layout-templates §Surface: desktop-webview, Expression level) Motion budget is 0.3 — CSS color transitions only, no animation library; the picker + controls have subtle hover lift (`motion-micro`), no spring / parallax / staggered entrance

## Patterns to follow
- Frameless titlebar remains active as the primary signature placement for the run heartbeat count (ch2 pre-existing); ch3 adds the picker/controls BELOW it
- Picker and button controls route through shadcn/ui (Radix UI Primitives 1.x) with Tailwind v4.1 styling — this is the first shadcn introduction to `ui/`; all Tailwind v4 `@theme` static-stylesheet integration must be verified
- Status (idle/live/HOLD/aborted) is gated in the titlebar `RunState` prop (ch2 pre-existing); Start/Stop enablement on the backend is derived from the same run-lifecycle state; no separate Channel-driven state in ch3

## Anti-patterns to avoid
- Do NOT add a live-counter `Channel` or stream run emission/target status from backend to frontend — that is ch4 (Live-counter Channel stream) and retires the ch2 DEV-only run-state cycler
- Do NOT build the coverage-matrix view, run-report card, operator-pause go/no-go dialog, or operator-checklist components — these are ch5 (Component primitives), ch6 (coverage matrix), ch7 (run-report + checklist), ch8 (operator-pause dialog), respectively
- Do NOT introduce animation libraries, spring physics, or motion beyond CSS `motion-micro` color transitions — the 0.3 expression budget is CSS-only

## Contract bindings
- architecture.md §Standard Contracts (Tauri commands) — `list_scenarios`, `list_suites`, `start_run`, `stop_run` commands + their ACL in `capabilities/*.json` (deny-by-default, no blanket perms)
- design-system.md (expression 0.3, tokens by name) — all color/spacing/radius/motion referenced by token names (`count-nominal`, `color-raised-2`, `radius-sm`, `space-sm`, `space-md`, `motion-micro`), not hex/px values
- a11y-plan.md §Keyboard, §Forms-controls, §Focus, §Visual — keyboard-operable picker (Radix roving focus + type-ahead via cmdk), semantic `<button>` controls, visible focus ring, selection/state not-color-alone (text label paired with any tint)
- security-plan.md §Tauri GUI, §Code Patterns — deny-by-default capability perms, `tauri ≥2.10.3`, no `shell-open` with scenario-derived strings, scenario selection validated against catalog (never shelled), shadcn/Radix/cmdk deps audited (`npm audit` clean)

## Acceptance criteria contributions
- (layouts) Picker + Start/Stop controls render in the control row below the titlebar (layout-templates §Wireframe — Run console (idle), control row).
- (layouts) Picker is fully keyboard-operable with visible focus ring; selection state not-color-alone (layout-templates §Component — Primary navigation focus management).
- (layouts) Start button enabled only when a scenario/suite is selected; Stop button enabled only while a run is in flight (layout-templates §Component — Primary navigation, run-state gating).
- (layouts) Picker rows show title + P-ID(s) in text-first format; picker + buttons styled with shadcn + Tailwind v4.1 borders-only depth (layout-templates §Wireframe — Run console (idle) + §Component — Primary navigation).

## Relevant amendment history
- 2026-06-23-5-command-agent-run-harness: registered `conductor preflight [--json]` verb in cli Primary screens (no impact on ch3 desktop picker/controls, webview-scoped)
- 2026-06-23-line-oriented-output-rendering: registered `conductor coverage [--write]` verb + corrected `indicatif 0.18`→`0.17` in tooling context (no impact on desktop surface)
- 2026-06-24-frameless-window-shell: reconciled titlebar height `space-lg`→`space-xl` in wireframe and Component-Header spec — ch3 picker/controls sit BELOW this titlebar, so the space-xl anchor is already in place
