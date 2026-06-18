# design extract

## Relevance
Partial — spans and error status are emission primitives outside the UI/styling surface; the scope touches no rendered component, color, typography, or motion.

## Constraints
Per design-system.md §Anti-Patterns Universal Bans: color is never purely decorative (§Color Palette, §Anti-Patterns); `Status.Code=ERROR` carries semantic meaning that must bind to the display later (§Verdict / report-state lamp). Per §Brand Identity: "calm under load, no alarm" — the hard ERROR signal will later render as a muted red verdict lamp (#F85149), not flashing / pulsing (§Motion § Hard limits). Per §Surface: cli: status prefixes are paired with color (§Component Patterns 4 — `✗ P-XXX Fail … (ANSI 203 red)`) and never color-alone (§Anti-Patterns Per-Surface Bans cli). Per §Depth Strategy: borders-only elevation (no drop shadows) applies to the run-report view that will later display error results. The determinism rule (§Deterministic ID generation per scope) ensures `trace_id` / `span_id` seeding is reproducible for test coverage — no entropy.

## Patterns to follow
1. **Deterministic ID generation under seed:** per scope boundary, span/trace IDs derive from scenario+seed (no clock reads / entropy), preserving "same scenario+seed ⇒ same stream shape." This mirrors the seeded randomness discipline already in the scaffold (opentelemetry-proto types).
2. **Status as a first-class semantic signal:** `Status.Code=ERROR` is the hard error signal (per scope intent for P-005) and must remain distinct from `Status.Code=OK` and `UNSET` at the builder level, so the verdict/report-state rendering tier (downstream, later chunks) can pair it with the muted red `#F85149` lamp + `[FAIL]` prefix.
3. **Multi-span tree with parent/child linkage:** shared `trace_id` + correct `parent_span_id` (empty for root) ensures the error can sit at root or deep child (P-008 placement); no exotic span-event enrichment here (exception events deferred to P-006).

## Anti-patterns to avoid
1. **No flashing / pulsing animation on the ERROR signal at emission time** — motion is at expression 0.3, functional-only (§Motion § Hard limits); the error status itself is data-only, never animated in the emission layer.
2. **Never emit a redundant color or verdictization at the emit layer** — `Status.Code=ERROR` is the carrier; the palette assignment (`#F85149`, `[FAIL]` ANSI 203) happens at the render tier (run-report / cli output), not here. Emission is agnostic to styling.
3. **Never hardcode widths, ASCII prefixes, or ANSI codes in the emission layer** — those belong to the cli output formatter (downstream); the span-tree builder yields raw protobuf.

## Contract bindings
Emission-to-render: The `Status.Code` field (ERROR / OK / UNSET) binds to the verdict/report-state lamp (desktop-webview per §Component Patterns 4) and the cli status line (per §Component Patterns 4 — `✗ P-XXX Fail` colored ANSI 203 red). The deterministic ID contract (trace_id / span_id seeded) binds to test stability and the later MCP read-back (P-005 / P-008's verdict logic in Epoch 5/7). No binding to a11y here (no rendered UI in this chunk); the binding surfaces when the run-report renders the error verdict (then it must meet contrast §Color Palette + Color-Only rule §Component Patterns 4).

## Acceptance criteria contributions
[ORCHESTRATOR PRUNE: #1–4 below are really arch/emit construction criteria, not design — folded into arch/tests at synthesis. Only #5 (render binding) is design's own, and it is a LATER-chunk concern, not this chunk's DoD.]
1. (emit) Span carries `Status { code: STATUS_CODE_ERROR, message }` with code as a first-class, byte-controllable field (P-005 hard ERROR-status signal).
2. (emit) Multi-span trace correctly constructs shared `trace_id` and parent/child `parent_span_id` linkage (root span has empty `parent_span_id`; children link to parent) — well-formed for P-008 root-vs-deep placement.
3. (emit) Root-vs-child placement selector routes the ERROR status to root span OR deep child at requested depth, producing a valid `ExportTraceServiceRequest` protobuf.
4. (emit) Deterministic: same scenario+seed yields same `trace_id` / `span_id` values across runs (seeded, reproducible, no entropy / clock reads).
5. (downstream render binding) On the run-report view, an ERROR-status span renders the `#F85149` muted red lamp (never flashing) + paired `[FAIL]` text (cli: `✗ P-XXX Fail` ANSI 203 red) — LATER chunk, not this DoD.

## Relevant amendment history
(none) — Design System amendments file contains only one entry (2026-06-15, token CSS block structure for Tailwind v4), which is orthogonal to error-span emission primitives and the render binding.
