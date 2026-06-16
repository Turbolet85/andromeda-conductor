# design extract

## Relevance
partial — the chunk involves the journal-entry *type* (line schema + stored field names) and write-side surface contract (artifact paths, timestamp storage format); the journal is a data artifact (JSONL records, not rendered UI), so most surface/component/animation/spacing rules do NOT apply, but field-allowlist redaction + artifact hygiene + canonical state-name spellings DO bind.

## Constraints
- per design-system §Color Palette: journal entries must never include color names or hex values; if metadata includes state names, they use the canonical Verdict/ReportState names (`Pass` / `Fail` / `CalibrationRegion` / `Blocked` / `ManualCheck` / `KnownResidual`), never color names (color is a downstream render concern).
- per design-system §Typography: the journal's stored timestamp is an integer-millisecond offset (`journal_emitted_at`), not a human-facing RFC-3339 string; typographic tokens are render concerns for downstream phases.
- per design-system §Anti-Patterns §Universal Bans (artifact hygiene): no absolute host paths, no internal struct/field names leak into journal lines — field-allowlist redaction applied at write time.
- per design-system §Surface: cli §Tokens: if the journal is ever rendered to CLI, state-name text (`[PASS]` / `[HOLD]` / `[FAIL]` / `[BLOCKED]`) must pair with ANSI color, never color alone.

## Patterns to follow
- per design-system §Anti-Patterns (Verdict-3 vs ReportState-5): journal entries carrying a machine verdict include both `Verdict` (3-valued) and `ReportState` (5-valued) in the schema; the two are distinct and never collapsed.
- per design-system §Color Palette: the css token names (`--count-nominal`, `--status-fail`, `--count-blocked`, `--status-manual`, `--status-residual`) are internal render tokens, NOT journal field values — the journal uses semantic state names only.
- per design-system §Timestamp contract: store `journal_emitted_at` as integer milliseconds (SLO-math ground truth); downstream phases apply RFC-3339 for display.

## Anti-patterns to avoid
- per design-system §Anti-Patterns §Per-Surface Bans (cli): no emoji in the stored JSON journal lines (machine-parseable JSONL must not carry emoji).
- per design-system §Anti-Patterns §Universal Bans: do not expose internal struct names, field names, or host-specific paths in journal lines — redact before write.
- per design-system §Color Palette §Semantic Colors: never conflate `Verdict` with `ReportState`, so downstream render never silently downgrades `Blocked`/`KnownResidual` to red `Fail`.

## Contract bindings
- **obs ↔ journal schema:** tests/obs specialists own the journal-entry type definition (field names, semantic state values); the writer implements their schema.
- **timeline → journal write:** the current_thread seeded scheduler (chunks 1–2) calls the journal writer on phase transitions / emission events; the writer records a `journal_emitted_at` stamp and appends durably.
- **journal ↔ downstream SLO math (Epoch 5/6):** the stored integer-millisecond `journal_emitted_at` is the ground truth for journal-relative latency math.

## Acceptance criteria contributions
1. (design) Journal entries contain no absolute host paths, no internal struct names — only semantic field names and state values — per design-system §Anti-Patterns §Universal Bans.
2. (design) `journal_emitted_at` originates from `std::time` (SystemTime/Instant), never tokio virtual clock — per design-system §Determinism invariant.
3. (design) Journal I/O failures propagate as `Result::Err` (harness fault), never downgraded to a run verdict — per design-system §Anti-Patterns ("no result yet" ≠ "failed").
4. (design) Verdict/ReportState entries use only the canonical semantic names (Pass / Fail / CalibrationRegion / Blocked / ManualCheck / KnownResidual), never color names or internal tokens — per design-system §Color Palette §Semantic Colors.

## Relevant amendment history
- 2026-06-15-design-token-typography-bundle: the token-block amendment (`:root` vs `@theme`) applies only to §Surface: desktop-webview rendering — the journal is not rendered and carries no typography or spacing tokens. No action for this chunk.