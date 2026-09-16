# layouts extract

## Relevance
Partial — the chunk edits `scenarios/*.toml` and tests, not a surface, but `slo_tier` is a rendered cell on both surfaces and my plan carries per-scenario sample rows that bake tier literals.

## Constraints
- The tier cell renders one of exactly `<5s` / `<20s` / `<90s`, the closed set, on the cli 6-column results/SLO table (P-ID · scenario · state · slo_tier · latency_ms · fingerprints) — per layout-templates §Surface: cli → Component — Primary content block 1. A fourth value would break downstream parsers, which §cli IA notes → Command model names as a breaking change class. Whether every committed TOML already declares only these three is research's question.
- `slo_tier` has **no** render site on the desktop coverage matrix: the row is four cells (P-ID · Capability · Mode · Status) and §Surface: desktop-webview → Component — Primary content block 1 records `slo_tier`/`latency_ms` as deliberately NOT among them. A re-tier therefore cannot touch the matrix; its only webview site is the run-report verdict line (§Component — Primary content block 2, mono identifiers in Data role).
- A re-tier moves what the Markdown run report prints as the judged deadline: §Surface: cli → Component — Primary content block 2 (per-check detail region) requires each indented per-check line to carry "the deadline it was judged against — its declared budget, or the scenario tier's when it inherits". Whether the inheriting checks in the re-tiered scenarios already print the tier-derived value is research's question.
- The stated-reason comment for a tier that does not hold its duration gets **no** surface render: §cli Component — Primary content block 2 holds the per-P-ID bracket-label set closed at six, with no new column, ANSI entry or token; §desktop Component — Primary content block 2 holds the lamp set closed at six likewise.
- The run-level `[ENVIRONMENT-SUSPECT]` qualifier is scoped to the breaching emitting phase and the sustained terms, rides outside the lamp column, and is omitted entirely when in-envelope — per §cli Output structure (`conductor run`) and §desktop Component — Primary content block 2. §layout-templates states it as a run-level qualifier, so a tier re-declaration is not its input; the scope's expectation that `contracts/pulse-load-envelope.toml` stays untouched is consistent with that, and confirming it is research's question.
- The tier cell is one value adapted per surface, never forked — desktop verdict line ↔ cli table cell ↔ Markdown artifact, per §cli IA notes → Multi-surface coordination. A scenario's re-declared tier must read identically on all three.

## Patterns to follow
- **De-literalized sample cells.** §Wireframe — Run console (HOLD) and §cli Component — Primary content block 2 already render `restart-suppression`'s tier as the `<slo_tier>` placeholder with the closed set named in prose. Any sample row whose scenario this chunk re-declares follows that form rather than receiving a fresh literal.
- **Set-naming over baked values.** §cli Primary screens (`conductor coverage`) states every roll-up number is manifest-derived, never a literal; the same discipline governs tier cells sourced from the scenario's TOML declaration.
- **Non-measured cells render `—` / null.** §cli Component — Primary content block 1 and §desktop Component — Primary content block 2: `Blocked` / `Manual` rows print `—` in the measurement columns and are never red. A scenario that grades nothing contributes no per-check detail line (§cli block 2).
- **Status text always paired.** §cli Signature placement / IA notes → Pipe discipline: every state cell carries its ASCII bracket prefix so the row survives `NO_COLOR` and piping; tier and latency render in the ID-cyan mapping (ANSI 117) as Data.

## Anti-patterns to avoid
- Do not mint a fourth tier, a seventh lamp, a sixth `ReportState`, or a new bracket label to carry the stated reason — the sets are closed (§cli Component — Primary content block 2; §desktop Component — Primary content block 2).
- Do not re-pin a fresh tier literal into a sample row for a re-declared scenario — de-literalize instead (§Wireframe — Run console (HOLD) tier-cell note).
- Do not add a `slo_tier` column to the coverage matrix or to the 4-column `conductor coverage` table to expose the honesty state (§desktop Component — Primary content block 1; §cli Primary screens `conductor coverage`).

## Contract bindings
- **layouts ↔ tests harness** — the scope's "tests pinning a specific scenario's tier or deadline" bind to the rendered tier cell and to the per-check detail region's judged-deadline line (§cli Component — Primary content block 2); the harness leg is `scripts/agent-run.{sh,ps1} run --unit/--integration` (§cli Primary screens).
- **layouts ↔ obs / run artifact** — the Markdown `runs/<run_id>.md` per-check detail region is the layout-owned render of the inherited deadline (§cli Component — Primary content block 2); its values come from the verify seam's arithmetic, which the scope flags as an open P3 premise.
- **layouts ↔ architecture** — `SloTier::deadline_ms()` and its three variants are the source of the cell's closed value set; layouts renders, never defines it (§cli Component — Primary content block 1).

## Acceptance criteria contributions
- (layouts) Every scenario's rendered tier cell is one of `<5s` / `<20s` / `<90s`; no new tier value appears on the cli results/SLO table, the desktop verdict line, or the Markdown artifact (per layout-templates §Surface: cli — Component — Primary content block 1).
- (layouts) The coverage-matrix row stays four cells and the `conductor coverage` table stays 4 columns — a re-tier adds no `slo_tier`/deadline column to either (per layout-templates §Surface: desktop-webview — Component — Primary content block 1).
- (layouts) Where a re-declared scenario is named in a sample row, the tier cell is the `<slo_tier>` placeholder rather than a fresh literal (per layout-templates §Wireframe — Run console (HOLD)).
- (layouts) The stated-reason comment introduces no new column, bracket label, lamp, token or ANSI entry on any surface; the per-P-ID label set stays closed at six (per layout-templates §Surface: cli — Component — Primary content block 2).

## Relevant amendment history
- `2026-08-18-error-baseline-spike-live-proof` — swept the P-009 `error-baseline-spike` sample tier `<5s` → `<90s` at five sites after the TOML re-declared it, on the "whole-run latency exceeds every tier by construction; `<90s` is the closest honest bucket" rationale. That is exactly this chunk's Situation 1/2 posture, and `error-baseline-spike` is in its Situation 1 list — so the five `<90s` sample sites are the prior art the chunk's posture rests on.
- `2026-08-18-restart-suppression-live-proof` — the same scenario's five sample tier literals were replaced by the `<slo_tier>` placeholder rather than re-pinned, after the doc's five sites disagreed with each other and with the shipped value. This minted the de-literalization rule for the tier cell; `restart-suppression` is again in this chunk's Situation 1 list.
- `2026-08-19-connection-lifecycle-live-proof` — de-literalized the idle-console `P-001`/`P-002`/`P-003` tier cells after two family tiers moved, explicitly on "de-literalize, never re-pin a fresh literal".
- `2026-08-16-fingerprint-storm-live-proof` — established the de-literalization playbook rule for the derived-value class after a literal re-staled three times; this is why a tier sweep in this chunk should not substitute fresh numbers.
- `2026-08-21-per-check-latency-measurement` — added the per-check detail region carrying "the deadline it was judged against (its declared budget, or the scenario tier's when it inherits)", with no new column or bracket label. This is the render site a re-tier propagates into.
- `2026-08-21-severity-lifecycle-live-proof` — full sweep correcting mislabeled sample-row P-IDs against the coverage classification; precedent that sample rows in this doc are held to the committed scenario catalog, not left approximate.
