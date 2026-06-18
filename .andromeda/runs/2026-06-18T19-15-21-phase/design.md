# design extract

## Relevance — partial

This chunk covers PII payload corpus emission primitives (backend/emit-crate concerns). Design system applies only to the public API surface exposed to downstream scenario/verification phases. No rendering surfaces (desktop-webview / cli) involved in this chunk.

## Constraints

- Per §Typography, **mono status tier reserved strictly for ID-cyan color + font — never use mono font for general coding UI or data display** (P-IDs, run_id, SLO timings, fingerprints only); the PII corpus values are synthetic fixtures, not status-identifiers (design-system §Typography, Data tier).
- Per §Depth Strategy, **no GPU-heavy compositing or visual effects** — this is backend only; constraint applies if corpus is exposed in logs/UI later.
- Per §Brand Identity (expression level 0.3), **structure preservation principle mirrors no-alarm discipline** — scrubbed PII must leave surrounding context intact (design-system §Brand Identity / Verdict + ReportState visual discipline).
- Per §Anti-Patterns / Color Palette, **color is never used alone to signal state** — if corpus or error states surface in reports, pair color with text labels (design-system §Color Palette / semantic colors table).

## Patterns to follow

- **Deterministic, seeded generation** — mirrors "paused count shows where it stopped" precision discipline; same seed ⇒ same corpus aligns with mission-control repeatability (design-system §Brand Identity).
- **Structure preservation as identity** — PII scrubbing mirrors the "no-alarm, calm under load" signature by leaving surrounding spans/logs/exception context readable; the scrubber's work is invisible to the operator (design-system §Depth Strategy, borders-only elevation).
- **Error reporting without panic** — `EmitError` surface (never panic) mirrors the verdict/report-state discipline: distinct outcomes surface as distinct types, never collapsed into a generic failure (design-system §Color Palette / Verdict vs ReportState).

## Anti-patterns to avoid

- **Hardcoding PII values or using non-deterministic generation** — breaks the seeded reproducibility invariant (design-system §Brand Identity, single-operator precision).
- **Conflating scrub-side logic with emit-side primitives** — scrubbing verdict (P-035/P-048) is Epoch 7, not this chunk; corpus is purely emission, never verdict (design-system §Anti-Patterns / Sameness Test — avoid defaulting to familiar error-handling patterns without domain intent).

## Contract bindings

- **verification ↔ emit:** Downstream `pii-scrub` scenario (P-035/P-048, Epoch 7) reads this corpus via loopback integration test; corpus shape validates scrub fidelity (design-system §Component Patterns / Run-report view — verdict + report-state lamp pairings).
- **cli ↔ emit:** CLI output of run-report surfaces PII-scrubbed (or pre-scrub) payload evidence; must honor `NO_COLOR` + ANSI stripping (design-system §Surface: cli / Platform-Specific Notes).

## Acceptance criteria contributions

- **(design) Corpus values are deterministically seeded and reproducible** — same seed yields identical corpus (design-system §Brand Identity / precision "paused count" discipline).
- **(design) Each PII category is structurally valid** — scrubber can recognize and redact it by shape, never false-positive or collision (design-system §Anti-Patterns / Verdict vs ReportState — distinct types surface distinctly).
- **(design) Corpus embeds across all three signal types (spans, logs, exceptions)** — no silent omission of a category in any signal path (design-system §Depth Strategy / elevation hierarchy — every seam defined, no gaps).
- **(design) Error surfaces as `EmitError`, never panic** — consistency with emit-crate contract; verdict wall surfaces as structured result, not alarm (design-system §Brand Identity / calm under load).

## Relevant amendment history

(none) — amendment 2026-06-15 touched token bundle only (Tailwind v4 `:root` vs `@theme` syntax), not backend contracts or corpus patterns.