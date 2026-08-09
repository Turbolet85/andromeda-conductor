# Scope — Out-of-scope classification treatment

**Marker:** `2026-08-09-out-of-scope-classification-treatment`
**Version:** conductor-0.2.0 · Epoch 1 — Foundation: re-aim at the SUT
**Working entry:** _Out-of-scope classification treatment — not-Conductor's rows distinct in coverage matrix,
footer roll-up and CLI table, never Blocked or Fail_

## Intent

The SUT capability manifest accepts 82 capabilities; 16 of them are **not Conductor's to verify** — Pulse's
UI/visual set and its own tooling, outside Conductor's remit by standing non-goal. That boundary is now a
recorded decision in code, but on every rendered surface an out-of-scope row still reads like any other row.
This chunk makes the out-of-scope classification **visually distinct** wherever coverage is rendered, and
guarantees it never reads as a **failure** (`Fail`) or as a **never-measured verdict** (`Blocked`) — the two
states an undifferentiated row is most likely to be mistaken for.

The classification is a statement about *remit*, not about *outcome*. A reader scanning the matrix must be
able to tell, without counting or cross-referencing, that these rows were never in play.

## What already shipped (do NOT rebuild)

Carried forward from `2026-08-09-current-sut-coverage-classification`:

- `CoverageMode::NotConductors` exists in `conductor-core/src/coverage.rs` with the stable wire spelling
  `not-conductors` (serde rename + `label()`, pinned by `not_conductors_wire_spelling_is_stable`).
- **16 rows** are classified into it (P-061..P-066, P-068..P-071, P-076..P-078, P-080..P-082) across four
  themes: *Window & shell hygiene*, *State honesty & legibility*, *Test gap & housekeeping*, *Operator-surfaced*.
- `tally()` counts the fourth mode; the tally sums to the row count on every surface.
- All three surfaces already render the mode **as text**: the Markdown report, the CLI `conductor coverage`
  table, and the webview `CoverageMatrix.tsx`.
- `layout-templates.md` §Component — Primary content block 1 (`:126`) already documents the **Mode cell** and
  its four values, added at that chunk's wrap by operator decision, in explicit anticipation of this chunk.

**This chunk owns only the VISUAL treatment.** The model, the wire spelling, the row set, and the tally are
settled; re-deriving or re-representing them is out of scope.

## What this chunk builds

1. **A distinct out-of-scope treatment on the desktop coverage matrix** — the row reads as out-of-remit at a
   glance, never borrowing the `status-fail` or `count-blocked` token. Per the universal invariant, the
   treatment is **never color-alone**: it pairs with the existing text label (and a glyph if one is added).
2. **The roll-up** — the out-of-scope count is separated from the in-scope denominator so the coverage picture
   is honest: an 82-row tally where 16 rows were never Conductor's should not read as 16 unmeasured rows. The
   tally must still sum to the row count.
   *(Amended at P5 validation-1 — intent-incomplete, not a defect: this item assumed all three surfaces carry
   a roll-up. P3 research found the CLI `conductor coverage` prints **none** — only the Markdown
   `summary_line` and the webview `tally()` exist. So the roll-up work is two things: reshape the two that
   exist, and **add one to the CLI**, which also closes a cross-surface parity gap. On the desktop the roll-up
   is the existing `cov__summary` header strip — `App.tsx` renders no `contentinfo` footer, and this chunk
   does not build one.)*
3. **The CLI `conductor coverage` table** — the same treatment, adapted not forked (design-system rule:
   desktop CSS vars ↔ cli ANSI codes by name), surviving `NO_COLOR` and piping via its text label.
4. **A negative guarantee, asserted by test** — an out-of-scope row never renders the `Fail` or `Blocked`
   status vocabulary on any surface (`[FAIL]`/`[BLOCKED]` prefix, the fail/blocked color tokens, the
   `Blocked` precondition slot).
5. **Extend `layout-templates.md` §Component block 1 (`:126`)** with the out-of-scope treatment — the Mode
   cell entry is already authored; add the treatment to it rather than re-authoring the block. *(Spec bodies
   are read-only to /andromeda-phase and /andromeda-implement — this lands as a wrap-time amendment, planned
   here, applied there.)*

## Boundaries (explicitly NOT this chunk)

- **No change to the classification itself** — not the row set, not which mode a capability sits in, not the
  wire spelling, not `tally()`'s arithmetic.
- **No verdict-model change** — `Verdict` / `ReportState` / `Lamp` are untouched. Out-of-scope is a
  *coverage mode*, orthogonal to the verdict/state axis; this chunk must not mint a new `ReportState`
  or a sixth lamp to represent it.
- **No `coverage-matrix.md` artifact decision** — whether that file becomes a committed artifact is an open
  question owned by the Epoch-6 *Coverage completeness gate* entry. This chunk renders it; it does not
  decide its on-disk fate.
- **No scenario, manifest, or drift-check change** — `contracts/pulse-capabilities.toml` and
  `check_sut_drift` are untouched.
- Not a re-authoring of `layout-templates.md` §Component block 1 — an extension only.

## Surfaces + contracts touched

| Surface | Path | Role |
|---|---|---|
| Classification source | `crates/conductor-core/src/coverage.rs` | `CoverageMode` + rows + `tally()` — read, not remodeled |
| Markdown report | `crates/conductor-report/src/coverage.rs` | `coverage-matrix.md` render + roll-up line |
| CLI table | `crates/conductor-cli/src/render.rs` (`coverage_table_styled`) | comfy-table surface, tty-gated color |
| Webview matrix | `crates/conductor-tauri/ui/src/components/CoverageMatrix.{tsx,css}` | dense one-row-per-P-ID list |
| Layout spec | `.andromeda/layout-templates.md` §Component block 1 (`:126`) | Mode cell — extend at wrap |

**Invariants this chunk is bound by:** status is never color-alone (text label + glyph desktop / ASCII prefix
cli) · design tokens adapted not forked across desktop↔cli · no absolute host paths or internal struct names
in rendered artifacts · `Blocked` ≠ red, and out-of-scope ≠ `Blocked`.

## Carried nit

`crates/conductor-tauri/ui/src/components/CoverageMatrix.css:1` still comments "all 60 capabilities" —
sweep it while in that file (a stale count from the pre-manifest era).
