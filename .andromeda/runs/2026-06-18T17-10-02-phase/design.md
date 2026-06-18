# design extract

## Relevance
partial — latency shaping chunk involves no UI/visual/motion implementation; no tokens/typography/layout exposure to the design system.

## Constraints
(none) — this chunk lives in the `conductor-emit` crate (a Rust backend trace-emission primitive), outside the visual design surface scope. The design system governs `conductor` (the desktop-webview UI) and `conductor-cli` (terminal output). Latency shaping is a data-generation concern, not a surface.

## Patterns to follow
(none) — design patterns (Component Patterns §1–7, Navigation Pattern, Motion Protocol) do not apply to a deterministic trace-duration primitive.

## Anti-patterns to avoid
(none) — the Anti-Patterns list targets visual/interactive design surfaces (font choice, color misuse, alarm motion, grid layouts, prompt gating), none of which touch latency emission logic.

## Contract bindings
**P-011/P-012 acceptance criteria** — latency primitives produce samples at the ≥50-sample floor per the spec (design-system §Anti-Patterns, rejection of "sample-count guessing"); Pulse's p50/p95/p99 consumption is downstream verification (conductor-verify, Epoch 5), not design-system scope.

## Acceptance criteria contributions
(none) — acceptance criteria for latency shaping live in the chunk's scope (determinism, per-operation independence, integration with raw-OTLP); the design system contributes no pass/fail checks to this chunk.

## Relevant amendment history
(none) — amendment history in `design-system-amendments.md` records only `tokens.css` restructuring (2026-06-15), a desktop-webview concern; no prior amendments touch latency-emission or backend trace primitives.