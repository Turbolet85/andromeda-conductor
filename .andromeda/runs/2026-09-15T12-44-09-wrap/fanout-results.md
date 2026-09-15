# Fan-out results — 2026-09-15-structurally-dead-assertion-class-retired

7 Explore doc-agents, one parallel batch. Detectors scoped from `drift-base.md` (19 total; `D-platform-claim`
is scoped to all seven and was sent to each).

| doc | detectors sent | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions · D-platform-claim | **3 proposals** (1 primary + 2 `dependent-of`) — raw twin `.raw-fanout-arch.md` |
| security-plan | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | **2 proposals** (1 primary + 1 `dependent-of`) — raw twin `.raw-fanout-obs-plan.md` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | `proposals: []` |

**Totals:** 5 proposals across 2 docs · 5 docs clean · 0 entity-escape failures · 0 re-spawns.

## Clean returns — the reasoning each gave (audit record)

- **security-plan** — all four detectors clean. Dependencies "none added, none bumped", `Cargo.lock`
  byte-unchanged at 562; no spawn/preflight/data-dir touchpoint; no new external-input surface (the
  declare-only shape is already governed, 25 scenarios shipped it). Additionally swept its own body for
  `findings-counter-refresh`, the other three scenario names, `declare-only` and the 12/25 corpus counts →
  **0 hits**; noted the `CalibrationRegion` mention at its line 283 is the `Verdict` enum listing in
  §Error Handling, unrelated and untouched by the diff.
- **design-system** — read its three numeric near-misses rather than counting them: `:265` is
  "stormed the fingerprint 12×/30s" (an induced-state sample), `:201` is the Tailwind `@theme` token tally
  "11 of 34", and the other `12`s are the `space-md: 12px` token and a matrix row padding. `25`, `29`, `982`,
  `983` have zero occurrences. No palette row, ANSI-map entry, token label or reuse tally bakes a retired value.
- **layout-templates** — same discipline: its only `12` is the same `12×/30s` storm-rate sample at `:153`;
  `:286` states the per-check detail-region rule by SET ("a scenario that graded nothing … contributes no such
  line"), which remains true with four more members; `:188`'s capability literals are a different denominator
  this chunk did not move.
- **test-plan** — returned the bare `proposals: []` form. Independently corroborated by the report's own
  measurement: `grep -nE` for all four scenario names over `test-plan.md` → **0 hits**, so its §1/§6
  declare-only sites name none of them and no two-site re-base is owed.
- **a11y-plan** — no interactive element added, neither schema changed; the eleven-key obs §6 envelope
  reproduced at its §1 and §3, and the §6 verdict/state ↔ lamp crosswalk, all still match. Swept for
  `findings-counter` / `CountAtLeast` / `declare-only` / `[[expected]]` → **0 hits**. Its platform-stating
  sentences are unfalsified — this chunk retires no platform, runner or driver verdict.

## Validation outcome (orchestrator)

All 5 proposals **routine**, **0 escalations**, so no HALT.

1. **Playbook** — no rule governs cleanly. Rule @171 (measured-scalar literal) subject-matches the count half
   but its qualifier "ONLY a measured-scalar literal … names, contract and form preserved" is FALSE here, since
   a membership claim is retired → NO MATCH per the every-qualifying-clause test. The operator's RECORDED
   direction (wrap directive item 6) settles the two predicted amendments, and the flow's own duplicate-occurrence
   clause ("a duplicated claim never survives a single-site apply"; cascade step 2's same-master duplicate rule,
   explicitly "a routine amendment") governs the three additional sites. Applied; the rule is PROPOSED at the
   wrap card, since a direction settles the proposal and never the class.
2. **Cross-contradiction** — none. The three arch proposals edit distinct sections; the two obs proposals edit
   non-overlapping spans of one line in the same direction.
3. **Intent-consistency** — aligned. The report's one divergence (six further dead blocks found, not retired) is
   justified and operator-ruled, routed to a new Epoch 2 route entry.
4. **Absence needs evidence** — **the report's own sweep claim was DISPROVED and corrected.** Its first draft
   dispositioned `architecture.md:70` as "fingerprint derivation, unrelated" from a clipped view of a
   10,432-character line; the token sits at offset 5712 stating the retired claim, and a third site at `:72`
   uses a different token entirely. Corrected in the report under *Expected amendments* as SWEEP CORRECTION.
5. **Expected-amendments reconciliation** — the plan listed 2 entries; both proposed (arch §Standard Contracts,
   obs-plan §4). Floor met, exceeded by 3 duplicate sites.
6. **Disproved-claims disposition** — all 3 report entries DISPOSED: #1 → arch primary; #2 → obs primary +
   dependent; #3 (`residuals.md`'s "no producer emits that token") → the matrix `notes` premise correction at
   P7.3 per directive item 3, and additionally amended at `architecture.md:72`.
