# design extract

## Relevance
Partial — the chunk generates a static classification artifact, not a rendered UI surface.

## Constraints
- Per design-system §Brand Identity: all output carries the signature Paused-count hold-point metaphor (the mission-control "precision + frozen state" discipline applies even to definitions); classification must reflect this operational focus, not generic taxonomy.
- Per design-system §Color Palette §Semantic Colors: the three Conductor modes (auto / drive+observe / static-only) will eventually display with Verdict/ReportState lamp treatment (green/amber/red/slate-violet); classification schema must be compatible with lamp-precedence reuse per `Lamp::for_record` (binds to Epoch 8/9 renderers).
- Per design-system §Typography §Data: P-IDs (P-001..P-060) rendered in all downstream surfaces are reserved to JetBrains Mono 13px, 500 weight, `--color-id-cyan` (#7DCFFF dark, #0969DA light), tabular-nums — classification artifact must index by canonical P-ID format.
- Per design-system §Spacing: artifact prose + structured data table use the base 4px unit scale (not applicable to Markdown artifact directly, but schema documentation must reflect precision discipline).
- Per design-system §Anti-Patterns Universal Bans: classification must NOT use generic categories ("Verified" / "Unverified" / "Pending") — use domain-specific modes reflecting the operational story (auto-asserted / operator-confirmed / Pulse-owned).
- Per design-system §Component Patterns §Coverage matrix: the 60-row list is the source definition; classification completeness is non-negotiable (zero gaps, all P-IDs enumerated) — mirrors the console's "dense single-row-per-P-ID wall of all 60 capabilities" visual discipline transposed to definition.

## Patterns to follow
- Per design-system §Anti-Patterns: Verdict/ReportState hierarchy (each P-ID carries one canonical truth — a mode — never collapsed/conflated states like "no result yet" with "failed"); apply the same discipline here: each classification must be distinct and never silently downgrade (static-only is NOT a failed auto, it is an explicit Conductor-out-of-scope).
- Per design-system §Component Patterns §Verdict lamp + §Iconography: status prefixes in artifact (if status column included per open question #1) must mirror CLI prefixes (ASCII `[AUTO]` / `[MANUAL]` / `[STATIC]` never color-alone, paired with lamp glyph where rendered).
- Per design-system §Surface: cli §Tokens: P-ID rendering in artifact must honor the reserved mono ID-cyan tier (visual identity across desktop/cli/artifact).

## Anti-patterns to avoid
- NEVER conflate Pulse's verification modes with Conductor's modes in the classification — the JSON seed carries Pulse's `automated-nextest` / `by-construction` / `manual-verification`; re-map to Conductor's {auto, drive+observe, static-only} without bleeding the distinction (per scope "the lens shift").
- NEVER enumerate P-IDs in random order or skip any — the "dense single-row-per-P-ID wall" discipline means canonical P-001..P-060 sequence, all present, completeness asserted by test (per def-of-done).
- NEVER use generic artifact naming or format — `coverage-matrix.md` is the source-of-truth definition file, kebab-case per the run-report seam convention; it is NOT an overlay or a report view (those come in Epoch 8/9).

## Contract bindings
**Coverage matrix (artifact) ↔ `Lamp::for_record` rendering** — the classification schema must be compatible with lamp precedence (when Epoch 8/9 surfaces render P-ID rows with status lamps, they use `Lamp::for_record` driven by Verdict/ReportState, not by re-derived mode); open question #1 asks whether this chunk includes a live status column (pending Epoch 7 scenarios/runs, likely deferred, but binding must be declared now).

## Acceptance criteria contributions
- **(design) All 60 P-IDs (P-001..P-060) enumerated in canonical order, zero gaps or unclassified entries — completeness golden-test assertion.** (design-system §Component Patterns §Coverage matrix "dense single-row-per-P-ID wall").
- **(design) Each P-ID classified into exactly one of {auto, drive+observe, static-only} — no mode conflation or silent downgrades.** (design-system §Anti-Patterns "Verdict/ReportState hierarchy, never collapse").
- **(design) Artifact path and Markdown format (`coverage-matrix.md`, kebab-case) consistent with run-report seam, ready for golden-test string matching.** (design-system §Component Patterns §Run-report view "pure/clock-free render").
- **(design) Classification schema compatible with `Lamp::for_record` reuse — prepared for Epoch 8/9 renderers to apply lamp treatment without re-deriving precedence.** (design-system §Component Patterns §Verdict lamp, color-as-state binding).

## Relevant amendment history
**(none)** — the design-system.md amendments file contains only one entry (2026-06-15 token CSS refactor, unrelated to coverage classification). The classification domain is new to Epoch 6 and has no prior amendments.
