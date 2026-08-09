# Codebase Research — 2026-08-09-out-of-scope-classification-treatment

## Scope
- **Depth:** moderate · **Reads:** 4 source files (full/section) · **Globs/Greps:** 7 · **Graph queries:** 1 (260 rows)

## Files inspected
- `crates/conductor-report/src/coverage.rs` (full, 175 lines) — the Markdown artifact. `CoverageMatrix::render()` writes title → `summary_line(rows)` → a 4-column table (`P-ID | Title | Category | Mode`), one row per capability, `mode.label()` as plain text. `summary_line` is `**Capabilities** {n}` + `· {n} {label}` per `CoverageMode::ALL` — a flat 4-mode tally, no in-scope/out-of-scope split.
- `crates/conductor-cli/src/render.rs:156-171` (`coverage_table_styled`) — the CLI table. **4 columns** (`P-ID | Title | Category | Mode`), `presets::UTF8_FULL`, `ContentArrangement::Disabled`. Only the P-ID cell is tinted (`ID_CYAN`); the Mode cell is a bare `Cell::new(r.mode.label())` with **no tint at all**. Colour is gated by `stdout_color()` (`:101` — isatty + `NO_COLOR` unset + `TERM != dumb`).
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full, 85 lines) — the webview wall. `tally(rows)` renders `{n} capabilities · {n} auto · …` into `<header className="cov__summary">`; the Mode cell is `<td className="cov__mode type-data">{r.mode}</td>` — the raw wire string, one shared class for all four modes.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.css` (full, 82 lines) — `.cov__mode { color: var(--text-secondary) }`, single rule for all modes. **Line 1 carries the stale nit**: "the dense single-row-per-P-ID wall of all 60 capabilities".

## Graph impact (`tree-query-2026-08-09-out-of-scope-classification-treatment.json`, **rows: 260**)
Callers of `%CoverageMode%` / `%tally%` / `%coverage_matrix%`, by file:

| file | rows | meaning for the change |
|---|---|---|
| `crates/conductor-core/src/coverage.rs` | 222 | the classification's own definition + tests — **read-only for this chunk** |
| `crates/conductor-report/src/coverage.rs` | 22 | the Markdown render + its 7 tests — the roll-up edit lands here |
| `crates/conductor-core/src/drift.rs` | 5 | `check_sut_drift` consumes the same static — must stay untouched |
| `crates/conductor-cli/src/render.rs` | 4 | `coverage_table_styled` + its tests — the CLI edit lands here |
| `crates/conductor-tauri/src/commands.rs` | 3 | the `coverage_matrix` command + tests — passes rows through, no change needed |
| `crates/conductor-core/src/lib.rs` | 2 | re-exports |
| `crates/conductor-cli/tests/cli_smoke.rs` | 1 | `coverage_lists_every_classified_pid` E2E |
| `crates/conductor-tauri/src/main.rs` | 1 | handler registration |

**Reading:** the blast radius is exactly the three render seams the scope names. No consumer outside them reads `CoverageMode`, so a presentation-only change has zero engine impact — and `drift.rs`'s 5 rows confirm the drift check keys off the same static, which is why touching the classification would be a cross-cutting change and touching the *render* is not.

## Patterns detected
- **Wire spelling is the display string** (`conductor-core/src/coverage.rs:53`, `CoverageMatrix.tsx:68`) — `label()` returns `"not-conductors"` and every surface prints it verbatim. `layout-templates.md:126` names the value `not-Conductor's` in prose; the surfaces render `not-conductors`. Prose-vs-wire, not a defect, but the treatment should not silently "fix" one into the other.
- **Only the P-ID column is tinted anywhere in the coverage table** (`render.rs:164`, `CoverageMatrix.css:55`) — there is no existing per-mode colour on any surface. This chunk introduces the first one.
- **The desktop↔cli name-pair map is closed and complete** (`design-system.md:288-299`): `--count-nominal`↔114 · `--count-hold`↔179 · `--status-fail`↔203 · `--count-blocked`↔60 · `--status-manual`↔146 · `--status-residual`↔246 · `--color-id-cyan`↔117. **The `--text-*` tiers have NO ANSI counterpart** — so a `--text-tertiary`-based treatment cannot be mirrored to the CLI without adding a map entry (a design-system amendment).
- **Token collision is real** (`tokens.css`, dark block): `--count-blocked`, `--text-muted` and `--border-emphasis` are all `#565F89`. A recessive treatment built on `--text-muted` would render pixel-identical to `Blocked`. `--text-tertiary` (`#717AA0`) is the nearest distinct recessive tier.
- **Non-lamp reuse of a status token is established precedent** (`render.rs:121`, `design-system.md:314`): `hint:` reuses Residual mute / ANSI 246 rather than minting a colour — the pattern the amendment history flags as the governing precedent.
- **Golden-test shape** (`conductor-report/src/coverage.rs:84-105`) — `summary_line_format_is_exact` locks the roll-up string over a *4-row synthetic fixture*, deliberately avoiding the live count. Any roll-up change re-baselines this exact string, not a literal-82 assertion.
- **Leak denylist already names the new vocabulary** (`conductor-report/src/coverage.rs:145-159`) — `no_host_paths_or_struct_names_leak` already denies `"NotConductors"`, `"CoverageMode"`, `"CapabilityRow"`. The negative-guarantee test extends this exact shape.

## Conventions to follow
- **Never color-alone**: `design-system.md:302` — status prefixes are ASCII text + colour; `render.rs:127` pairs every tint with `status_prefix()`. The Mode cell's existing text is the surviving channel.
- **tty/NO_COLOR gating**: `render.rs:101-105` — all CLI colour flows through `stdout_color()`; the treatment must route through `tint(...)` (`render.rs:175`) so comfy-table widths stay correct.
- **Single lamp-truth mirroring**: `ui/src/lamp.ts:13-18` mirrors `conductor-core/src/lamp.rs:52-71` arm-for-arm. Coverage mode must NOT be added to `LAMP_META` — it is a different axis (scope boundary).
- **Deterministic pure render**: `conductor-report/src/coverage.rs:1-8` — no clock, no IO, byte-identical output; `render_is_deterministic` enforces it.
- **Atomic write**: `CoverageMatrix::write` is `.tmp` → rename (`coverage.rs:46-58`), already correct.

## New files to create
- (none) — every surface already exists.

## Files to modify
- `crates/conductor-report/src/coverage.rs` — roll-up line + Mode-cell treatment in the Markdown render; re-baseline `summary_line_format_is_exact`; add the negative-guarantee test.
- `crates/conductor-cli/src/render.rs` — tint the Mode cell in `coverage_table_styled`; add the CLI roll-up (see Open question 1); add the plain/coloured pair + negative-guarantee tests.
- `crates/conductor-cli/src/commands/coverage.rs` — print the roll-up alongside the table, if the roll-up lands on the CLI.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — per-mode class on the Mode cell + the roll-up form in `tally()`.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.css` — the out-of-scope rule; **sweep the line-1 "all 60 capabilities" comment** (carried nit).

## Open questions
1. **The CLI has no roll-up at all.** `commands/coverage.rs:13` prints only the table — the Markdown has `summary_line` and the webview has `tally()`, but `conductor coverage` prints no tally. The working entry names "footer roll-up **and** CLI table" as separate items, so the honest reading is that the CLI gains one. → resolve at P4.
2. **What does an "in-scope-honest" roll-up mean numerically?** Flat 4-mode tally with only visual distinction, vs an explicit in-scope denominator (`82 capabilities · 66 in scope (…) · 16 not-Conductor's`). The obs extract flags that an in-scope denominator **pre-binds `coverage_percent`** for the Epoch-6 completeness gate (obs-plan §4). → resolve at P4.
3. **Which token pair carries the treatment**, given that `--text-*` has no ANSI counterpart and `--text-muted` is pixel-identical to `--count-blocked`. → resolve at P4.

## Spec↔reality notes (surfaced, not fixed here — wrap's reconcile owns these)
- `layout-templates.md:246` heads its block "coverage-matrix / SLO table" and then specifies **6 columns** (`P-ID · scenario · state · slo_tier · latency_ms · fingerprints`) — that is the **results** table (`results_table_styled`, `render.rs:140`). The actual `conductor coverage` table is **4 columns** (`P-ID · Title · Category · Mode`). The layouts extract inherited the conflation and produced a "keep exactly 6 columns" constraint that does not describe this surface; the plan adopts the underlying rule (the column set is a parsed-output contract — do not add columns) and not the literal count. `design-system.md:310` carries the same conflation.
- `layout-templates.md:146` §Component — Footer (status strip) describes an app-level `contentinfo` footer. `App.tsx` has none: the desktop roll-up is the `<header className="cov__summary">` strip inside `CoverageMatrix`. This chunk extends the existing header strip; it does not build the footer landmark.
