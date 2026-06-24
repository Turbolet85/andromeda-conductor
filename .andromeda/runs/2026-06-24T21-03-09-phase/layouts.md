# layouts extract

## Relevance
Partial — the chunk stands up the desktop-webview frameless window shell; the static titlebar structure is in scope, the paused-count signature motion is explicitly deferred to the next chunk.

## Constraints
- Desktop-webview is a single-station console in one frameless window (`decorations: false`); no router, no breakpoints, no browser nav — per layout-templates §Primary Surfaces.
- Titlebar is a custom drag-region (`data-tauri-drag-region`) rendered as a `banner` landmark with fixed height `space-lg` and a `1px border-subtle` seam (no drop shadow) — per layout-templates §Component — Header, §IA notes (desktop).
- The Paused-count signature placement #1 (titlebar count freeze at the hold value) is out of scope for this chunk; the count lives in the titlebar Display role but renders static/dormant here — per scope.md "NOT the Paused-count hold-point signature … NEXT chunk."
- Window controls (minimize/close) are native `<button>` elements only (never `<div role>`), carry visible focus rings in `color-focus`, positioned in the titlebar right zone — per layout-templates §Component — Header.
- Title bar left/center/right zones cluster horizontally with gaps `space-xs`; left = phase line (Heading), center = count (Display, dormant `count-nominal`), right = window controls (icon grid) — per layout-templates §Component — Header wireframe.
- Outer window margins are `space-xl` and the webview is bounded by the titlebar (frameless) — per layout-templates §Wireframe — Run console (idle).

## Patterns to follow
- Deny-by-default Tauri `capabilities/*.json` with minimal allowlist (only frameless-window-needed permissions at this stage) — per layout-templates §IA notes (desktop) and scope.md §Boundaries.
- Titlebar is a fixed-height `banner` landmark with a bottom `border-subtle` seam; the three-zone flex layout (phase line | count | controls) is replicated across all run states (idle / live / HOLD / report) — per layout-templates §Wireframe — Run console.
- React 19 / Tailwind v4.1 (Oxide static stylesheet) / shadcn/ui primitives (locally vendored Radix) applied to the titlebar — per layout-templates §Surface: desktop-webview and scope.md Surfaces touched.
- Expression level 0.3: no animation library, no spring, no parallax, no staggered entrances (motion reserved for the signature freeze in the next chunk) — per layout-templates §Expression level.

## Anti-patterns to avoid
- Do NOT render the Paused-count freeze/tint/resume heartbeat here; the count is static/dormant in this chunk — per scope.md.
- Do NOT pre-add later-chunk command permissions (start/stop, picker, run-report, operator-pause, Channel) to the capabilities ACL — per scope.md §Boundaries.
- Do NOT use drop shadows, animation libraries, or motion effects in the titlebar; use borders-only depth (`border-subtle` seams) — per layout-templates §Wireframe and §Expression level.

## Contract bindings
- a11y ↔ `banner` landmark (SC 2.4.1); focus order visibility on window controls (`color-focus` visible ring); status never color-alone.
- security ↔ deny-by-default `capabilities/*.json` ACL; Tauri ≥2.10.3; no `shell-open` with derived strings; no remote-origin iframes; Chromium context-menu/devtools suppressed.
- obs ↔ Tauri backend self-obs sink (`logs/conductor-tauri.jsonl` via extended `ObsSink`); `service.name` = `conductor-tauri`; processor-stage redaction.

## Acceptance criteria contributions
- (layouts) Frameless titlebar renders as `banner` landmark with three zones (left: phase line | center: count `count-nominal` | right: window controls) at height `space-lg` and `border-subtle` bottom seam — per layout-templates §Component — Header.
- (layouts) Window is draggable via `data-tauri-drag-region` on the titlebar; drag-region pointer cursor does not leak onto the window-control buttons — per layout-templates §Component — Header.
- (layouts) Window controls (if present) are native `<button>` elements with visible `color-focus` ring and accessible name — per layout-templates §Component — Header and a11y-plan §Landmarks.
- (layouts) Deny-by-default `capabilities/*.json` created with minimal allowlist (only frameless-window permissions); no wildcard/blanket permission; no later-chunk command permissions — per scope.md §Boundaries.

## Relevant amendment history
- 2026-06-23-5-command-agent-run-harness — added `conductor preflight [--json]` verb to §Surface: cli; no scope impact on desktop-webview titlebar layout.
- 2026-06-23-line-oriented-output-rendering — added `conductor coverage [--write]` verb + corrected `report` verb output structure; no scope impact on frameless titlebar shell.