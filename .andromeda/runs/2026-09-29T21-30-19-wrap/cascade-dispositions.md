# Cascade dispositions — 2026-09-29-hue-shift-budget-graded-hard

The pass amended two masters. `architecture.md:181` is the §Occupied Resources row for the P-025 contract (A1).
`obs-plan.md:350` is §4 Known-residual → Delegated-timing family, the P-025 passage (O1 and O2, merged into one
edit).

## The search

- Patterns are in `cascade-patterns.toml`, derived from every amendment before the first grep. Each keys on what
  a retired claim SAID:
  - `unmeasurable` (the P-025 status);
  - `mechanism pin|no pass arm` (the grading shape);
  - `transitioned_at_unix_nano` (the fall source);
  - `would (have|need) to emit` (the unmet-ask framing);
  - `quantiz` (the retired mechanism, kept only as history);
  - `Conductor measurement, not a transcribed` (the retired single-kind provenance);
  - `lifts only if Pulse|Still Pulse intake` (the intake framing).
- Tool: `cascade.py sweep`, with the baseline `cdb70826` (the parent of the pre-CI commit). Every pattern's
  control fired on the pre-pass masters (`obs-plan.md:350` or `architecture.md:181`).
- Scope: the seven masters and `.andromeda/registries/**`, the three curation homes, both judgment bases, and the
  leaf bodies.

## Rows

| pattern | rows | disposition |
|---|---|---|
| `unmeasurable` | 0 | retired everywhere. The control fired on the pre-pass `obs-plan.md:350` |
| `mechanism pin\|no pass arm` | 0 | retired everywhere |
| `transitioned_at_unix_nano` | 0 | retired everywhere. The corrected fall source is stated in the amended `:350` |
| `would (have\|need) to emit` | 0 | retired everywhere. The control fired on the pre-pass `architecture.md:181` |
| `quantiz` | 1 row: `obs-plan.md:350`, standing, edited, ×2, at `@c3598` and `@c3908` | no change. Both matches are the amended text's own RETIRED-instrument history: "(TICK QUANTIZATION, U(0, 15s) …", which is dated `83d4060` with `as measured at` its evidence, and the pinned test name `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency`. They are not a live claim. Windows read with `cascade.py window` at both offsets |
| `Conductor measurement, not a transcribed` | 0 | retired everywhere |
| `lifts only if Pulse\|Still Pulse intake` | 0 | retired everywhere |

No row was `leaf`, `curation` or `base` for any pattern, so the curation homes, the playbook and drift-base carry
none of the retired wording.

## Step 3: the leaf set, recomputed rather than scanned

- **architecture §Occupied Resources** (a contracts/ row). Its leaves are CLAUDE.md `GENERATED:setup:overview`
  (`:14`, which lists "the P-025 measurement contract" as a `contracts/` member) and
  `.claude/docs/conventions.md:9` (the same membership list). Recomputed: membership and count are unchanged
  (seven members, two with no Rust reader), so there is no text change. The CLAUDE.md pointer table carries no
  per-row prose.
- **obs-plan §4**. Its leaves are `.claude/docs/obs-summary.md`, `.claude/rules/observability.md` and CLAUDE.md
  `GENERATED:setup:warnings`. Recomputed from the amended §4: none of them distills the Known-residual
  delegated-timing passage. `grep -niE 'harvest|known-residual|delegated|P-025|hue'` over the two leaves returns only
  `obs-summary.md:14`, the Tauri `conductor.tick` heartbeat, which is unrelated. So there is no text change.
- **Provenance headers.** `grep -l 'Extracted from .*(obs-plan|architecture)'` over `.claude/docs` and
  `.claude/rules` returns `conventions.md` only, and it is covered above.
