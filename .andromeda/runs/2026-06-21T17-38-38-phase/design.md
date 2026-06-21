# design extract

## Relevance
Partial — the chunk is backend-only (Rust serialization, no UI rendering) but consumes semantic types (Verdict, ReportState) that bind to design's color/state contracts.

## Constraints
- Run-report envelope MUST carry both Verdict (3-valued: Pass / Fail / CalibrationRegion) and ReportState (5-valued: Pass / Fail / ManualCheck / KnownResidual / Blocked) per design-system §Color Palette, which distinguishes machine-verdict from operator-state outcomes; converging these into a single field would collapse semantic distinctions (design-system §Color Palette, Verdict vs ReportState distinction)
- Blocked envelope rows MUST populate only identity + slo_tier, leaving verdict/latency/fingerprints as JSON null per design-system §Color Palette ("never measured" is the semantic anchor — see design-system Component Patterns §Coverage matrix for rendering expectation)
- Timestamp instants in payloads MUST use RFC-3339 colon-delimited format (human/JSON tier) to preserve alignment with design-system §Surface: desktop-webview token spec (timestamps as tertiary text carry `--text-tertiary`, matching the journal-line artifact convention)
- The enum values {Pass, Fail, ManualCheck, KnownResidual, Blocked} MUST match the exact state names that downstream Markdown rendering (Epoch 6 chunk 3) and coverage-matrix component (Epoch 6 chunk 4) consume per design-system §Component Patterns §Verdict/report-state lamp
- run_id MUST remain filesystem-safe hyphenated form (YYYY-MM-DDTHH-MM-SS-<suffix>) as it is the runs.db PK + artifact stem; the serialized form is the single identity anchor across three storage layers per architecture §Standard Contracts

## Patterns to follow
- Verdict / ReportState as distinct Rust enum types (not converged) mirrors design-system §Color Palette's semantic tier strategy: machine-verdict (green/amber/red triad) vs operator-outcome (5-state lattice including Blocked/Manual/Residual as non-verdict outcomes)
- Artifact hygiene (no absolute paths / internal names in serialized form per scope) aligns with design-system §Anti-Patterns "never leak absolute host paths"; canonical serde naming enforces design's operational-meaning-only rule
- Per-P-ID latency_ms + slo_tier (<5s/<20s/<90s) carry mono ID-cyan (`#7DCFFF`) in rendered form per design-system §Typography Data tier; envelope serialization must preserve these as discrete fields so rendering layer can format them consistently

## Anti-patterns to avoid
- NEVER collapse Verdict + ReportState into a single enum (design-system explicitly reserves the 5-state lattice to avoid silent downgrade of Blocked/Manual/Residual to red Fail; design-system §Color Palette and §Component Patterns both anchor on this distinction)
- NEVER invent phantom latency/verdict values for Blocked rows — the "never measured" semantic is only honored if those fields are actually null per design-system §Color Palette "present-but-greyed, never measured"

## Contract bindings
Renders → desktop-webview coverage-matrix + run-report prose (Epoch 6 chunks 3–4): the ReportState enum bindings to the 6-state status-lamp glyph set (6 visually distinct treatments — green filled dot, amber filled dot, red filled dot, slate-violet hollow ring, neutral-lavender checkbox glyph, muted dashed-ring dot — design-system §Iconography). Verdict bindings to the color-paired-with-label rule per design-system §Color Palette and a11y binding (no color alone).

## Acceptance criteria contributions
- (design) ReportState enum values exactly match the six status-lamp glyphs: Pass / Fail / CalibrationRegion (Verdict) + ManualCheck / KnownResidual / Blocked (ReportState) per design-system §Iconography status-lamp set.
- (design) Blocked rows serialize verdict/latency/fingerprints as JSON null (never phantom values) to preserve the "never measured" semantic per design-system §Color Palette.
- (design) Timestamp instants use RFC-3339 colon-delimited format in human/JSON serialization (run-report prose styling / artifact-layer contract per design-system §Typography tertiary text).

## Relevant amendment history
2026-06-15-design-token-typography-bundle (§Tokens illustrative CSS): the token-declaration pattern (`@import "tailwindcss"` on `:root` + dark default + light variant in `@media (prefers-color-scheme: light)`) applies downstream; the run-report envelope does NOT carry design tokens (backend-only), but the Markdown rendering layer (Epoch 6 chunk 3) will consume this CSS structure per the amendment's Tailwind v4 discipline.
