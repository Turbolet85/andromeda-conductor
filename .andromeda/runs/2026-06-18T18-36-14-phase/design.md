# design extract

## Relevance
Partial — token/color/typography surface bindings; signature motion + expression only (no layout/per-screen craft). NOTE (orchestrator): this chunk is a pure `conductor-emit` primitive that renders NOTHING; these contributions are DEFERRED bindings that apply only when a future surface renders service.name/trace_id — no implementable design acceptance criteria land in this chunk.

## Constraints
- Per design-system §Brand Identity: mission-control console aesthetic, calm under load, no alarm in verdict rendering (fail red held muted, no flashing) — binds to Verdict/ReportState lamp status convention when surfaces emit diagnostics.
- Per design-system §Color Palette: every color communicates Verdict/ReportState functional state (green Pass / amber HOLD / red Fail / slate-violet Blocked / lavender ManualCheck / muted dashed KnownResidual) — never decoration. Multi-service topology spans may cross error boundaries (P-008 cross-service root-vs-deep error placement); if any surface emits a visual status for these span errors, use only the semantic palette tokens.
- Per design-system §Typography: mono status tier (JetBrains Mono 500, 13px, tabular-nums, `--color-id-cyan` `#7DCFFF`) is reserved for P-IDs / run_id / trace_id / service.name stamps + SLO timings / latency_ms / fingerprints. Service names in the emission primitive are part of the "mono status" domain (Service.name Resource attribute carried into OTLP ResourceSpans).
- Per design-system §Motion (expression `0.3`): trace rendering has no entrance animation; any timeline visualization of multi-service spans is static-on-render (no staggered reveals, no parallax, no scroll-driven effects).
- Per design-system §Component Patterns / Verdict lamp: cross-service error states (e.g., root error vs deep error in a downstream service) must emit distinct glyphs / text labels, never color alone.

## Patterns to follow
- Per design-system §Component Patterns #3 (Coverage matrix): if multi-service topology is surfaced as a matrix row extension, follow dense single-row-per-service layout, 12px row padding, `1px --border-subtle` dividers, no shadows — instrument-panel density, not a card grid.
- Per design-system §Typography: trace_id / service.name in any rendered trace view (titlebar, matrix, run-report) are rendered in mono ID-cyan with tabular-nums weight, regardless of surface; mirrors P-ID treatment.

## Anti-patterns to avoid
- NEVER render multi-service trace errors as a generic "error state" without distinguishing root vs deep — the verdict lamp and text label must carry both, paired with the service name so error origin is clear.
- NEVER use a progress bar or animated timeline for multi-service trace spans — at expression `0.3`, spans are static-rendered.
- NEVER embed trace_id / service.name in a generic monospace block without styling — they are status-tier tokens (ID-cyan, tabular-nums).

## Contract bindings
Obs (MCP trace-read) ↔ Design (mono status tier color + typography for trace display) — the MCP read-back of OTLP spans carries `service.name` and `trace_id`; any operator-facing display must follow the mono ID-cyan + tabular-nums convention. (Deferred — no display surface in this chunk.)

## Acceptance criteria contributions
- "(design) [DEFERRED to a rendering chunk] Multi-service trace P-IDs, trace_id, service.name rendered in `--color-id-cyan` with tabular-nums (design-system §Typography)."
- "(design) [DEFERRED] Cross-service error distinction (root vs deep) expressed as text label + distinct lamp glyph, never color alone (design-system §Color Palette + Verdict lamp)."

## Relevant amendment history
2026-06-15-design-token-typography-bundle (§Surface: desktop-webview / Tokens): Tailwind v4 `:root` token declaration (not `@theme` nesting) to preserve all 34 tokens and support `@media (prefers-color-scheme)` nesting. Applies only if multi-service trace display is webview-rendered (not this chunk — emit primitive only).
