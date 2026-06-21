# layouts extract

## Relevance — partial

This chunk touches the coverage-matrix surfaces on both desktop and CLI per layout-templates §Desktop-webview / §CLI, but only as **classification-definition** generators, not as **interactive view/render implementation**. The chunk generates the 60-row P-ID classification table; the desktop matrix *view* (Epoch 9) and CLI table *render* (Epoch 8) consume it downstream.

## Constraints

1. **Desktop matrix lifecycle** (per layout-templates §Primary content block 1 / §Wireframe — Run console): Coverage matrix renders as a virtual-scrolled dense list (NOT KPI cards), one row per P-ID, row padding `space-md`, `color-raised-1` container, `radius-md`, `border-subtle` dividers. This chunk defines P-ID ordering/classification; view layout is Epoch 9 (out-of-scope here).
2. **Verdict lamp precedence** (per layout-templates §Component — Primary content block 1/2): The signature convention pairs every status with a glyph (`✓`/`✗`/`⊙`/`~`/`☐`/`⊘`) — reuse `conductor-core::Lamp::for_record` (state precedence), do NOT re-derive lamp logic in the generator.
3. **Status never color-alone** (per layout-templates §IA notes, §Multi-surface coordination): Both desktop + CLI enforce `Blocked` ≠ red, `ManualCheck` = neutral checkbox, `KnownResidual` = muted dashed dot — these non-verdict states are distinct and never collapse into `Fail`. The classification table must preserve this invariant (the mode enum reflects it).
4. **CLI ANSI mapping** (per layout-templates §Surface: cli / §Multi-surface coordination): desktop CSS vars ↔ cli ANSI codes; the classification (auto/drive+observe/static-only) maps to the verdict tiers: `count-nominal` ANSI 114 (Pass-bearing) / `count-hold` ANSI 179 (CalibrationRegion-bearing / drive+observe) / neither for static-only (no live render).
5. **Artifact stability** (per layout-templates §IA notes / §Decisions Log): The `coverage-matrix.md` is machine-parseable downstream; crate split + table schema must be stable across versions — adding a column without a `--format` flag is a breaking change (mirrors CLI `comfy-table` contract).
6. **No live-status before Epoch 7** (per scope §Open questions): The classification-only rendering (no per-P-ID run status before scenarios exist); status column is a follow-up seam wired after Epoch 7 runs populate `runs.db`.

## Patterns to follow

1. **Mode classification** — re-map Pulse's capability audit (`automated-nextest` / `by-construction` / …) to Conductor's three modes (auto / drive+observe / static-only); the `.andromeda/refs/capability-verification-matrix.json` notes field carries timing/UX claims per mode.
2. **Dense tabular render** — the `coverage-matrix.md` artifact mirrors the CLI `comfy-table` column schema (P-ID, scenario label, slo_tier, latency_ms, latency context); use the same text layout, no nested YAML/JSON.
3. **60-row completeness** — enumerate P-001..P-060 with zero gaps; a missing P-ID is a defect; test asserts all 60 present and classified.

## Anti-patterns to avoid

1. **Do NOT re-derive lamp logic** — the 6-state verdict convention (filled dot / hollow ring / checkbox / dashed dot / … per layout-templates §Component — Primary content block 1) is `Lamp::for_record` in conductor-core; import it, do not duplicate.
2. **Do NOT render status-color-alone** — `Blocked`/`ManualCheck`/`KnownResidual` require ASCII/text prefix or glyph paired with ANSI; never rely on color to distinguish them.
3. **Do NOT add live-status columns before Epoch 7** — the classification table is static at Epoch 6; templatizing a "status" column with pending/— placeholder is scope creep (wiring it through `Lamp` prematurely without run data).

## Contract bindings

**conductor-report ↔ coverage-matrix render seam** — the `RunReport::render` golden-test pattern (pure/clock-free, exact-string testable) applies to the coverage-matrix generator; use the same seam design for artifact write + test.

**conductor-core::Lamp ↔ verdict/report-state rendering** — reuse `Lamp::for_record` (do not re-derive precedence); the mode classification feeds the lamp's future status-overlay (Epoch 8/9) but is classification-only here.

## Acceptance criteria contributions

1. "(layouts) Coverage-matrix classification complete: all 60 P-IDs (P-001..P-060) enumerated, each in exactly one mode {auto|drive+observe|static-only}, zero unclassified — defect if any missing."
2. "(layouts) Artifact `coverage-matrix.md` machine-parseable: column schema stable across versions (P-ID / scenario / mode / slo_tier / classification-rationale); mirrors CLI comfy-table structure per layout-templates §Component — Primary content block 1."
3. "(layouts) Lamp precedence reused: verdict/report-state glyphs in the matrix (if rendered) delegate to `conductor-core::Lamp::for_record`, not re-derived."

## Relevant amendment history

(none)
