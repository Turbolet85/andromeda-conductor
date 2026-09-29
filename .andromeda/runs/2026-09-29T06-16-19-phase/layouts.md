# layouts extract

## Relevance
partial — no surface, region, component or focus order is created or moved; the chunk touches layout only where the pin shift changes the coverage roll-up caption (all three surfaces) and where the real-model leg's verdict/Blocked lines and per-check detail render in existing structures.

## Constraints
- The coverage roll-up caption's `(N unbacked)` parenthetical qualifies the auto term ONLY and is never a fifth summand; the per-mode counts must still sum to the row count, so dropping four ids from `UNBACKED_AUTO` changes N (8 → 4) while the `43 auto` term stays unchanged (per layout-templates §Surface: cli → §Primary screens (commands), `conductor coverage [--write]`).
- Every number in the caption is manifest-derived, never a literal; the `(N unbacked)` value is sourced from `conductor_core::UNBACKED_AUTO` and reaches the webview as the coverage matrix's `unbacked` prop via the `unbacked_auto` command, never mirrored in TypeScript. Whether any webview test or fixture bakes the count 8 as a literal is research's question (per layout-templates §Surface: cli → §Primary screens (commands)).
- The same roll-up shape renders identically on all three surfaces (Markdown artifact `coverage-matrix.md` · cli `conductor coverage` · webview header strip), so the pin move changes all three in one change, adapted per surface and never forked (per layout-templates §Surface: cli → §Primary screens (commands); §Surface: desktop-webview → §Component — Primary content block 1).
- The coverage matrix row stays four cells (P-ID · Capability · Mode · Status) and the Mode cell for the four ids stays `auto`; backing a capability changes no Mode value and adds no column (per layout-templates §Surface: desktop-webview → §Component — Primary content block 1; §Surface: cli → §Component — Primary content block 1, "coverage matrix is a separate, narrower table").
- The real-model leg's per-P-ID outcomes use the closed six-label lamp set only; a canary that forms no incident renders as `[BLOCKED]` with its named precondition string in the indented detail and `—` measurement columns, never a red error and never a new label (per layout-templates §Surface: cli → §Component — Primary content block 2; §Component — Primary content block 1 (results / SLO table)).
- Any per-check grading the harvest adds for P-031/P-033/P-034/P-044 lands in the Markdown run report's indented per-check detail region: no new column, no new bracket label, no new ANSI/token entry (per layout-templates §Surface: cli → §Component — Primary content block 2, "Per-check detail region").
- The `--live real-model` selector stays a selector of `--live` — not a sixth harness command and not a new stage flag — leading with `conductor preconditions --for <scenario>`, and an unmet subject refuses with `[PRECONDITION]` lines at exit 1 before any leg fires (per layout-templates §Surface: cli → §Primary screens (commands), `scripts/agent-run.{sh,ps1}`).

## Patterns to follow
- `conductor coverage --write` is the regeneration path for `coverage-matrix.md`; per the plan, the committed artifact is that command's render, not a hand edit (per layout-templates §Surface: cli → §Primary screens (commands), `conductor coverage [--write]`).
- The caption's `(N unbacked)` term is omitted entirely at zero; at four it still renders (per layout-templates §Surface: cli → §Primary screens (commands)).
- Indented-detail-line precedent (Blocked precondition string, KnownResidual note) for extra per-check or per-precondition information under a verdict line (per layout-templates §Surface: cli → §Component — Primary content block 2).
- Preflight readiness line with `[OK]` / `[BLOCKED]` prefix and canary result, so a canary failure reads without color (per layout-templates §Surface: cli → §Component — Header / banner).

## Anti-patterns to avoid
- Counting unbacked rows as a separate mode summand, or baking the unbacked count as a literal on any surface (per layout-templates §Surface: cli → §Primary screens (commands)).
- Adding a column to the 6-column results/SLO table or the 4-column coverage table for the diagnostic-quality grading; the IA notes make a new `comfy-table` column without a `--format` flag a breaking change (per layout-templates §Surface: cli → §IA notes).
- Rendering a model-side or canary Blocked as `[FAIL]` / red, or minting a seventh lamp for "real-model deferred" (per layout-templates §Surface: cli → §Component — Primary content block 2; §Surface: desktop-webview → §Component — Primary content block 2).

## Contract bindings
- layouts ↔ tests: the byte comparison of `coverage-matrix.md` against its render (the coverage gate) is the layout caption's enforcement arm; the caption, the pin and the artifact move in the same change (per layout-templates §Surface: cli → §Primary screens (commands)).
- layouts ↔ desktop-webview: the header strip's unbacked count is fed by the `unbacked_auto` command; whether a webview/e2e spec asserts the caption text with a baked count is research's question (per layout-templates §Surface: desktop-webview → §Component — Primary content block 1).
- layouts ↔ a11y: the Status cell is always paired with its status text, and a verdict change is announced. The four rows' standing may move from "Not yet run" only if a run record names them (per layout-templates §Surface: desktop-webview → §Component — Primary content block 1).

## Acceptance criteria contributions
- (layouts) `conductor coverage` caption, `coverage-matrix.md` roll-up and the webview header strip all read `43 auto (4 unbacked)` in the same change, with the per-mode counts still summing to the row count (per layout-templates §Surface: cli → §Primary screens (commands), `conductor coverage [--write]`).
- (layouts) `coverage-matrix.md` equals the `conductor coverage --write` render byte-for-byte after the pin move, with no hand edit (per layout-templates §Surface: cli → §Primary screens (commands)).
- (layouts) A real-model canary that forms no incident renders a `[BLOCKED]` line with a host-path-free named precondition and `—` measurement fields, never `[FAIL]` (per layout-templates §Surface: cli → §Component — Primary content block 2).
- (layouts) The run report adds no new column or bracket label for the four diagnostic-quality ids; any per-check grading appears only as indented detail lines (per layout-templates §Surface: cli → §Component — Primary content block 2, "Per-check detail region").
