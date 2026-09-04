# design extract

## Relevance
Partial — the chunk is a11y/markup-led, but every touched surface (Titlebar count, ScenarioPicker, CoverageMatrix, RunReport, OperatorPauseDialog, App.tsx) is design-governed, and two findings (3 and 7) collide directly with the signature's announcement contract.

## Constraints
- Every style or markup touch in `crates/conductor-tauri/ui/src/` must bind tokens by NAME (`var(--…)`) — no raw hex, px, or ms literals; the 34-token `:root` block is stated as the binding contract (per design-system.md §Surface: desktop-webview / Tokens).
- The Paused-count hold-point is the signature: on an operator-pause the count must FREEZE at the exact value, tint `--count-nominal` → `--count-hold` over `--motion-micro`, the phase line flip to `HOLD — operator pause`, and design-system.md §Component Patterns 1 requires an `aria-live="assertive"` announcement of that flip. Finding 3 (assertive flips cancelling focus-restore) therefore cannot be fixed by demoting the hold flip to `polite` without contradicting this mandate — whether the shipped Titlebar uses `assertive` on the flip vs. on unrelated phase text is research's question (per §Component Patterns 1, §Brand Identity Signature element).
- Status must never be carried by color/border alone — the verdict lamp is always paired with its text label (`Pass`/`CalibrationRegion`/`Fail`/`Blocked`/`Manual`/`Residual`), and selection is `--border-emphasis` PLUS a text signal. Finding 5's ` · selected` text is a design requirement, not decoration (per §Component Patterns 3 & 4, §Anti-Patterns Universal Bans).
- Motion stays functional-only at expression `0.3`: 150ms `--motion-micro` with `--ease-quiet`, no animation library, no pulse/blink/glow on any status, no flashing red on `Fail`, and ALL transitions dropped under `@media (prefers-reduced-motion: reduce)` — nothing added for announcement/focus purposes may introduce motion (per §Motion, §Motion Hard limits).
- The terminal-state tint must track the recorded run state: abort → `--count-blocked` + dim; and "no result yet" must never collapse into `Fail`, nor `Blocked` render as a red error. Finding 8's client-side `aborted` → settle-to-`idle` while the backend records `Done` is exactly this conflation risk at the token level (per §Component Patterns 1, §Anti-Patterns Rejected Defaults "Conflating 'no result yet' with 'failed'").
- Empty / miss states render prose, never a gray skeleton or placeholder — the shipped strings `No scenarios found.` (matrix) and `No run yet` (run report) are pinned in the plan. Finding 6's picker filter-miss prose must remain announced prose in the same register (per §Component Patterns 3 Empty, §Component Patterns 6 Empty).
- Typography tiers are reserved: the count is JetBrains Mono 500 / 28px / tabular-nums (Display); any visible label added for finding 7 belongs to the IBM Plex Sans Label tier (500 / 12px), and mono-everywhere is an explicit Rejected Default (per §Typography, §Anti-Patterns Rejected Defaults).

## Patterns to follow
- §Component Patterns 1 (Frameless titlebar + Paused-count heartbeat) — governs finding 7's count naming and finding 3's phase-line politeness; the default/hold/abort tint states and the 16px `aria-label`-ed Lucide min/close controls are the existing structure to extend, not replace.
- §Component Patterns 2 (Operator-pause go/no-go dialog) — Radix `AlertDialog`, `role="alertdialog"`, focus trapped, visible `--color-focus` ring, 150ms `--motion-micro` fade; finding 3's focus-restore-after-hold fix lands inside this primitive's contract.
- §Component Patterns 3 (Coverage matrix, dense single-row-per-P-ID) — finding 2's scroll-region change must preserve the row list with `1px --border-subtle` dividers and virtual scroll; it must not become a table/card restructure.
- §Component Patterns 5 (Scenario/suite picker) — `Command`/`Select` over `--color-raised-2`, mono P-ID column in `--color-id-cyan`, keyboard-first, disabled state in `--text-muted` with no color-only signal; findings 1, 5 and 6 all land here.
- §Component Patterns 6 (Run-report view) — per-P-ID lamp + text lines over `--color-raised-1` / `--border-subtle` / `--radius-md`; finding 2's `Run report rows` region change must keep this structure.

## Anti-patterns to avoid
- No mono face for new prose/labels added while fixing findings 6 and 7 — mono is a reserved status tier only (§Anti-Patterns Rejected Defaults, "A monospace font used everywhere").
- No pulse / blink / glow / flashing to draw attention to an announcement or a focus change, and no native OS toast — Conductor observes those, never emits them (§Motion Hard limits, §Anti-Patterns Rejected Defaults).
- No `alert()`/`confirm()`/`prompt()`, no unstyled scrollbars, no hover-only affordance without a keyboard path — the console is keyboard-first (§Anti-Patterns Per-Surface Bans: desktop-webview).

## Contract bindings
- **design ↔ a11y:** §Component Patterns 1's `aria-live="assertive"` hold announcement binds a11y SC 4.1.3 Status Messages, and is the direct counterparty to finding 3 — a11y-plan §4 owns the resolution; design owns the requirement that the hold flip stay announced.
- **design ↔ a11y:** lamp + text pairing (§Component Patterns 3/4) binds SC 1.4.1 Use of Color; the ` · selected` / `aria-current` fix (finding 5) must satisfy both.
- **design ↔ a11y:** the `prefers-reduced-motion: reduce` block in §Tokens binds SC 2.3.3; token contrast pairs (post-2026-09-01 `--text-tertiary` / `--text-muted` values) bind SC 1.4.3 — any new label rendered in a recessive tier must reuse those values, not re-derive.
- **design ↔ tests/spec:** the `nvda-pass-spec.md` / `rows.ts` `expected` + `tokens` strings quote shipped UI strings that design pins (`No scenarios found.`, `No run yet`, `HOLD — operator pause`, the six lamp labels). Tightening a row must match the design-mandated string; if a fix CHANGES a pinned string, design-system.md §Component Patterns 3/6 needs an amendment (precedent: the 2026-09-02 entry below), not a silent divergence.

## Acceptance criteria contributions
- Every markup/style edit in the touched components binds design tokens by name — zero new hardcoded hex, px, or ms literals (per design-system.md §Surface: desktop-webview / Tokens).
- The operator-pause hold still freezes the count in place and color-transitions `--count-nominal` → `--count-hold` over `--motion-micro` / `--ease-quiet`, with the flip still announced and the transition dropped under `prefers-reduced-motion: reduce` (per design-system.md §Motion + §Component Patterns 1).
- Selection/current state in the picker and matrix carries a text signal alongside the `--border-emphasis` edge; no status or selection is conveyed by color/border alone (per design-system.md §Anti-Patterns Universal Bans + §Component Patterns 4).
- The post-Stop terminal tint matches the backend-recorded state — an abort dims to `--count-blocked`, a completed run does not render as blocked/failed, and no state is downgraded to red (per design-system.md §Component Patterns 1 + §Anti-Patterns Rejected Defaults).

## Relevant amendment history
- **2026-09-02-screen-reader-manual-spec** (§Component Patterns 3 Empty / 6 Empty+In-progress / 7 Operator-checklist) — the immediately prior SR pass corrected the plan's baked strings to the shipped ones (`No scenarios found.`, `No run yet`; the in-progress report prose retired; the checklist's only shipped mount is inside the operator-pause dialog). Directly this chunk's area: these are the strings the `sr*` row `expected`/`tokens` assert against, and the precedent for how a shipped-vs-spec string mismatch is resolved (record shipped state, keep design intent).
- **2026-09-01-desktop-a11y-sweep** (§Color Palette Text Hierarchy + §Tokens both themes) — `--text-tertiary` and `--text-muted` were moved for 4.5:1 across all nine a11y-plan pairs. Any recessive-tier text added for findings 6 or 7 must reuse these current values; re-deriving a dim grey would reopen the violation this amendment closed.
- **2026-06-26-component-primitives-library** (§Motion + §Component Patterns 2) — the operator-pause dialog fade is `--motion-micro` (150ms), not a literal 200ms; relevant because finding 3's fix touches that dialog's focus-restore path.
- **2026-06-15-design-token-typography-bundle** (§Tokens) — tokens are declared on plain `:root`, not `@theme` (which tree-shakes `--space-*` / `--radius-*` / `--motion-*`); any token added or referenced during these fixes follows the `:root` form.
