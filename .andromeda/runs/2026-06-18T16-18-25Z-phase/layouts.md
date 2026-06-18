# layouts extract

## Relevance
Partial — chunk involves raw OTLP log-record emission primitive with no surface creation; no layout structure added.

## Constraints
- Per layout-templates §Surface: desktop-webview, the console window is single-station, no routes, no state-driven screen creation (layout-templates §Primary screens: "four are one surface in different run states"); severity logs remain data emission, not a new surface (layout-templates §IA notes: "single-station console").
- Per layout-templates §Primary content block 2 (run-report view), severity-state mapping survives in existing verdict-lamp components: `CalibrationRegion` → `[HOLD]` ANSI 179, `Fail` → `[FAIL]` ANSI 203, `Blocked` → `[BLOCKED]` ANSI 60 (layout-templates §Component — Primary content block 2). Log severity must map to these if wired to the report later.
- Per layout-templates §Component — Primary content block 1 (coverage matrix), the matrix header echoes frozen step-index while held (layout-templates §Wireframe — Run console (HOLD): "matrix header echoes frozen step-index in count-hold"). Severity emissions do not alter titlebar / matrix focus-order.
- Per layout-templates §IA notes — security: deny-by-default Tauri capabilities, no remote-origin iframes (layout-templates §IA notes: "deny-by-default Tauri capabilities (only start/stop · picker · run-report · operator-pause commands + the one live-counter `Channel`)"). Logs emit over the same loopback gRPC `:4317` as traces; no new port listener.
- Per layout-templates §Surface: cli, verdict lines pair ASCII prefix with color (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]`), never color-alone (layout-templates §Component — Primary content block 2: "every state cell pairs its color with the bracket prefix"). If severity logs feed the report, they reuse this convention.
- Per layout-templates §Decisions Log — cross-surface IA, same design tokens by name across both surfaces, adapted not forked (layout-templates §Decisions Log: "`count-nominal` ↔ ANSI 114, `count-hold` ↔ ANSI 179, `status-fail` ↔ ANSI 203, `count-blocked` ↔ ANSI 60"). Severity emissions use OTel `SeverityNumber`; mapping to display tokens deferred to later chunks.

## Patterns to follow
- Per layout-templates §Component — Header (frameless titlebar), the count is announced to assistive tech on flip from ticking to frozen (layout-templates §Component — Header: "The flip from ticking to frozen is announced to assistive tech"). If severity logs drive a verdict flip (e.g., `Fail`), future layout integration must announce the change.
- Per layout-templates §Component — Primary content block 2, verdict changes are announced (layout-templates §Component — Primary content block 2: "Each verdict/state change is announced (a11y derives the live-region attribute)"). Severity-triggered verdict changes inherit this announcement obligation downstream.

## Anti-patterns to avoid
- Do NOT create a new modal, overlay, or route for severity logs — they remain raw emission primitives (layout-templates §Primary screens: one surface, four run states, no new pages).
- Do NOT render severity lamps in a color-only manner (layout-templates §Component — Primary content block 1: "Every lamp is paired with its status text (`Pass` / `CalibrationRegion` / `Fail` / `Blocked`) so state is never color-only"). If logs appear in the report later, pair severity with text.
- Do NOT emit to any port other than loopback `:4317` (layout-templates §IA notes: "Stays inside loopback gRPC egress to `:4317`; opens NO listener").

## Contract bindings
Severity logs ↔ run-report verdict rendering: the existing report-state mapping (P-IDs + `CalibrationRegion`/`Fail`/`Blocked`/`ManualCheck`/`KnownResidual`) must preserve its three non-verdict states (`ManualCheck` / `KnownResidual` / `Blocked` never red) when severity-driven verdicts are wired downstream. The `SeverityNumber` 17-boundary (≥17 = ERROR) will map to `Fail` lamp on the desktop and `[FAIL]` ANSI 203 on the CLI if/when journal-integrated; layout-templates §run-report §verdict lines reserves those slots.

## Acceptance criteria contributions
- (layouts) No new surface / route / modal created for severity logs; existing run-report verdict components inherit new severity-sourced verdicts downstream.
- (layouts) If severity logs are journal-wired to the report (deferred altitude decision per scope §Boundaries to resolve 2), severity verdicts reuse existing verdict-lamp colors and ASCII prefixes (`[FAIL]` ANSI 203 / `[HOLD]` ANSI 179) — never a new color code.
- (layouts) Severity-state changes announced via live-region attributes if they drive a report verdict flip (per a11y §Focus Order / live-region bindings).

## Relevant amendment history
(none)