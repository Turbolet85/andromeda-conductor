# design extract

## Relevance
Partial — this is a CI/WebDriver-session diagnostic chunk with no rendered surface changes, but it runs the a11y/axe gate over the shipped webview whose token contract design owns, and hypothesis-2 (`WEBVIEW2_USER_DATA_FOLDER`) / hypothesis-1 (driver swap) touch the Tauri launch path that the design plan's frameless-window and per-surface bans constrain.

## Constraints
- Any remedy that changes how the Tauri window is launched must preserve `decorations: false` + the `data-tauri-drag-region` custom titlebar — the frameless window IS the surface, per design-system §Surface: desktop-webview (Toolkit/Framework) and §Navigation Pattern. A probe that flips to a decorated window to obtain a session would break the signature placement (§Component Patterns 1), so it is a remedy that must be reverted, not shipped.
- The a11y job's axe run measures the shipped token contract: the 34-token `:root` block with its `@media (prefers-color-scheme: light)` and `@media (prefers-reduced-motion: reduce)` overrides is the binding contract per design-system §Surface: desktop-webview → Tokens. No probe edit may alter, strip, or inline-override those tokens to make a session succeed.
- Text/surface color pairs are already tuned to a measured contrast floor — `--text-tertiary` `#838EBA` and `--text-muted` `#727EB5`/`#636882` exist specifically to clear 4.5:1 (design-system §Color Palette → Text Hierarchy). Since this chunk's deliverable is the a11y gate turning green, any new axe `color-contrast` finding must be resolved by the plan's token values, not by a local hex.
- `@media (prefers-reduced-motion: reduce)` must drop ALL transitions including the count-tint on a held count, per design-system §Motion (Hard limits). If the axe/contrast harness or a probe runs the webview under a forced-motion or forced-colors profile, that override must still be present in the built stylesheet.
- Driver/dependency changes are bounded by the webview surface's declared stack — React 19 + Vite + Tailwind v4.1 + shadcn/ui, Tauri 2 ≥ 2.10.3 per design-system §Surface: desktop-webview → Platform-Specific Notes. Whether the swap in hypothesis 1 (`@crabnebula/tauri-driver` → official `tauri-driver`) disturbs the Tauri minimum version or the deny-by-default capabilities file is research's question.
- Probe-tree hygiene intersects the design plan's offline-hardening rule: fonts are self-hosted WOFF2 vendored into the bundle with no runtime CDN, per design-system §Typography (Loading). A probe must not introduce a Google-Fonts/CDN shortcut to shrink the ~8-minute build cycle.

## Patterns to follow
- The token block lives on plain `:root`, not `@theme` — Tailwind v4 `@theme`/`@theme static` tree-shakes non-namespace tokens (dropping `--space-*`/`--radius-*`/`--motion-*`), per design-system §Surface: desktop-webview → Tokens NOTE. Any build-config touch in the probe path must not reintroduce `@theme`.
- Status is never color-alone: every lamp is paired with its text label (`Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`Manual`/`Residual`) per design-system §Component Patterns 4 and §Iconography (Rule). This is the invariant the axe/contrast leg is there to protect.
- The cli mirror of a blocked/unmeasurable state is the closed ASCII prefix set plus the run-level non-lamp captions (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`, ANSI 246 / `--status-residual`), per design-system §Surface: cli → Tokens. If this chunk needs to report "session not created" as a runner-environment qualifier in CLI output, that recessive tier is the existing home — no new color, no seventh lamp.
- A measured-but-pre-accepted gap renders as `KnownResidual` (muted dashed lamp, "expected until {named fix}"), never red — per design-system §Color Palette (Verdict vs ReportState note) and §Component Patterns 6. This is the design-side counterpart of the chunk's `deferred` disposition of `v2-24`.
- "No result yet" is a distinct state from "failed", per design-system §Anti-Patterns → Rejected Defaults (conflating empty with error). A `v2-24` deferral must not surface anywhere as a red `Fail`.

## Anti-patterns to avoid
- NEVER ship with visible Chromium/WebView2 artifacts — context menu, devtools, text-selection on non-text elements (design-system §Per-Surface Bans → desktop-webview). A probe that enables devtools or remote-debugging surfacing to obtain `DevToolsActivePort` is a diagnostic-only edit and must not land in the shipped config.
- NEVER embed remote-origin iframes and NEVER ship Tauri commands without a deny-by-default capabilities file (design-system §Platform-Specific Notes / §Per-Surface Bans). A driver-side remedy that widens capabilities to let WebDriver attach is out of bounds as a shipped change.
- NEVER introduce raw hex/px/ms literals to satisfy a tooling constraint — tokens are bound by `var(--…)` name (design-system §Motion motion-tokens table, §Surface: desktop-webview → Tokens).

## Contract bindings
- Token contrast ↔ a11y §Contrast (SC 1.4.3, 4.5:1): the a11y job's axe `color-contrast` leg is the enforcement point for design's Text Hierarchy values; the 2026-09-01 token moves were driven by exactly that measurement.
- Motion tokens ↔ a11y §Animation (SC 2.3.3): the `prefers-reduced-motion: reduce` block is a shared mandate — design declares it, the a11y gate is where it would be observed.
- State color + text label ↔ a11y §Use of Color (SC 1.4.1): the never-color-alone rule spans both the webview lamps and the cli bracket prefixes.
- Design ↔ tests/CI: the `a11y` job (`.github/workflows/ci.yml`) and `crates/conductor-tauri/ui/wdio.conf.ts` are the harness that measures design's token + not-color-alone invariants; this chunk owns that harness reaching a session at all.

## Acceptance criteria contributions
- No probe or remedy edit changes any of the 34 `:root` token names or values, nor removes the light-mode or reduce-motion `@media` blocks (per design-system §Surface: desktop-webview → Tokens).
- If the a11y job goes green, the axe run reports zero `color-contrast` violations; any new one is fixed at the token tier, not with a local hex (per design-system §Color Palette → Text Hierarchy).
- Any diagnostic that exposes WebView2 devtools / context menu / remote-debugging surfacing is confined to the probe ref and absent from the merged config (per design-system §Per-Surface Bans → desktop-webview).
- A `deferred` `v2-24` is represented as a non-`Fail` outcome (`Blocked`/`KnownResidual` semantics with its measured reason string), never a red `Fail` and never a silent empty (per design-system §Color Palette Verdict-vs-ReportState note and §Anti-Patterns → Rejected Defaults).

## Relevant amendment history
- **2026-09-01-desktop-a11y-sweep** (§Color Palette → Text Hierarchy; §Tokens both theme blocks) — the last time the a11y/axe gate ran against the webview it produced one `color-contrast` violation, 18 nodes, all `--text-tertiary`; the fix moved `--text-tertiary` dark to `#838EBA` and `--text-muted` to `#727EB5`/`#636882`, solved across a11y-plan §6's nine pairs in both themes. Directly relevant: this chunk is trying to make that same gate run again on the hosted runner, so those values are the expected-clean baseline.
- **2026-06-15-design-token-typography-bundle** (§Tokens) — the `@theme` → `:root` reconciliation, with the reason (v4 tree-shaking drops 11 of 34 tokens). Relevant as a guardrail on any build/config touch in the probe path.
- **2026-09-02-screen-reader-manual-spec** and **2026-09-04-sr-findings-remediation** (§Component Patterns 3/5/6/7) — the prior SR/a11y chunks in this same area corrected shipped empty-state strings (`No coverage data.`, `No scenarios found.`, `No scenarios match.`, `No run yet`) and recorded the checklist's single shipped mount. Relevant because this chunk's SR-chunk predecessor (`2026-09-07-sr-findings-fixed`) is the source of the two CI fixes being carried forward; those exact strings are what the specs assert once a session is finally created.
- **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli Tokens) — `[PRECONDITION]` joined the run-level non-lamp caption set on the existing ANSI 246 / `--status-residual` pair, with set framing and no new palette row. Relevant precedent if this chunk needs to surface a runner-environment qualifier.
