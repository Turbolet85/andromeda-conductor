# Report — 2026-08-09-out-of-scope-classification-treatment

**Chunk:** Out-of-scope classification treatment — the 16 not-Conductor's rows visually distinct across the
Markdown / CLI / webview coverage surfaces + an in-scope-honest roll-up, never reading as Fail or Blocked;
visual-only over the shipped CoverageMode (conductor-report/cli/tauri)
**Date:** 2026-08-09
**Commits:** none since `last_wrap` (the prior wrap's `dd15dc3` is the last commit; this chunk is uncommitted
until P7)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-report/src/coverage.rs` (M)
  - `crates/conductor-cli/src/render.rs` (M)
  - `crates/conductor-cli/src/commands/coverage.rs` (M)
  - `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (M)
  - `crates/conductor-tauri/ui/src/components/CoverageMatrix.css` (M)
  - No new files. No file deleted. `conductor-core` untouched (the classification is read, not remodelled).

- **Symbols / APIs:**
  - **Added (public):** `conductor_cli::render::coverage_summary() -> String` — the CLI coverage roll-up line.
  - **Added (private):** `conductor_report::coverage::mode_cell(CoverageMode) -> String` ·
    `conductor_cli::render::coverage_summary_styled(bool) -> String` ·
    `conductor_cli::render::OUT_OF_SCOPE_MUTE: u8 = 246` (const).
  - **Changed (private, same signature):** `conductor_report::coverage::summary_line(&[CapabilityRow]) -> String`
    — output shape changed (see Schema / config) · `conductor_cli::render::coverage_table_styled(bool)` —
    Mode cell now tinted for the out-of-scope mode; `table.enforce_styling()` applied when `color`.
  - **Changed (webview):** `CoverageMatrix.tsx` `tally()` output shape; Mode `<td>` gains a modifier class.
  - **No change:** `Verdict` · `ReportState` · `Lamp` · `CoverageMode` (variants, wire spellings, `ALL`, `label()`)
    · `CapabilityRow` · `coverage_matrix()` · `check_sut_drift` · the run-report envelope · `runs.db` columns.
  - **No new** `#[tauri::command]`, IPC method, `Channel`, endpoint, port, socket, or env var.

- **Crates / modules:** none added, removed, or renamed. Edits confined to three existing crates
  (`conductor-report`, `conductor-cli`, `conductor-tauri/ui`), all reading `conductor-core`. No new
  cross-crate dependency edge (`render.rs` already imported from `conductor_core`; the import list gained
  `CoverageMode`, an existing re-export at `conductor-core/src/lib.rs:36`).

- **Dependencies:** **none added, none bumped, none removed.** `Cargo.lock` and
  `crates/conductor-tauri/ui/package-lock.json` both verified un-drifted (`git status` clean on both).

- **Schema / config:** no migration, no config key, no violation schema. One **rendered-output shape** change,
  identical across all three surfaces:
  - was: `**Capabilities** 82 · 43 auto · 16 drive+observe · 7 static-only · 16 not-conductors`
  - now: `**Capabilities** 82 · 66 in scope (43 auto · 16 drive+observe · 7 static-only) · 16 not-conductors`
  - CLI + webview carry the same shape without the `**Capabilities**` prefix (`82 capabilities · 66 in scope
    (…) · 16 not-conductors`). Per-mode counts still sum to the row count. The CLI surface **gained** a
    roll-up (it previously printed none).
  - Markdown out-of-scope Mode cell is now emphasised: `_not-conductors_` (in-scope cells unchanged).

- **Coverage of new surfaces:**
  - `conductor coverage` roll-up line (CLI stdout) → validation n/a (no input) · instrumentation n/a (a
    presentation render of a static in-code classification; no DB query, no gate assertion — obs §4
    Critical-Path-6 spans belong to the Epoch-6 completeness gate) · PII n/a · tests unit ✓
    (`coverage_summary_splits_the_in_scope_denominator`, leak test extended) · a11y n/a (cli surface is not
    WCAG-assertable, a11y-plan §1) · tokens design-token ✓ (xterm 246 ↔ `--status-residual`, existing pair)
  - `coverage-matrix.md` Mode cell + roll-up (Markdown artifact) → validation n/a · instrumentation n/a · PII
    n/a · tests unit ✓ (`summary_line_format_is_exact` re-baselined, `out_of_scope_rows_never_read_as_fail_or_blocked`,
    `summary_line_holds_on_degenerate_row_sets`, leak test green) · a11y n/a · tokens n/a (no colour channel;
    emphasis is the surface-adapted recessive treatment)
  - `CoverageMatrix` Mode cell `.cov__mode--out-of-scope` (webview UI element) → validation n/a ·
    instrumentation n/a (no new command; the existing `tauri.command.coverage_matrix` span is untouched) · PII
    n/a · tests build-gated (`tsc --noEmit` + `vite build` ✓; `conductor-tauri/ui` carries no nextest unit
    tier per test-plan §4) · a11y **deferred-recorded** (the headful axe/contrast/keyboard leg is
    display-gated to Linux+xvfb + live Pulse; the not-color-alone property holds by construction — the
    `not-conductors` text label is unchanged and always rendered) · tokens design-token ✓
    (`var(--status-residual)`, no new `:root` var)

## Deviations from intent

1. **`table.enforce_styling()` added to `coverage_table_styled`** — not among the plan's ten steps.
   *Justification:* comfy-table runs its own tty probe and silently drops cell styling under captured stdout,
   so the module's documented "ONE `stdout_color()` / tty decision" (render.rs:5-9) did not in fact govern the
   table: the positive residual-mute assertion was unobservable and the negative fail/blocked-code assertion
   was **vacuously true**. Gating `enforce_styling()` on the existing `color` flag makes the documented
   invariant real and is a no-op in production (`color` is only true when stdout is already a tty). Chosen
   over weakening the assertion (fix-loop anti-pattern). `results_table_styled` retains the double gate —
   out of this chunk's scope.
2. **Markdown treatment mechanism inferred, not surfaced.** Plan step 2 required a Markdown Mode-cell
   treatment but named no mechanism ("textual only, consistent with the label the other surfaces print").
   *Justification:* Markdown has no colour channel; emphasis is its recessive register, making it the
   surface-adapted counterpart of the residual-mute tint. The label text is unchanged, so the never-color-alone
   selector still matches across all three surfaces.
3. **Two report-seam tests beyond the plan:** `summary_line_holds_on_degenerate_row_sets` (empty +
   all-out-of-scope sets) and the leak test extended over the roll-up. *Justification:* the plan's obs
   acceptance criterion requires the roll-up arithmetic cannot panic on a degenerate set; `rows.len() - out`
   needed the proof.

## Decisions & corrections

- **Operator decision (phase P4, roll-up semantics):** the roll-up names an **in-scope denominator** alongside
  the full row count, and the **CLI gains a roll-up** (it had none — only Markdown and the webview did).
  Chosen over a flat tally, accepting that it pre-binds `coverage_percent`'s meaning for the Epoch-6
  completeness gate.
- **Operator decision (phase P4, token choice):** the treatment binds **`--status-residual` ↔ ANSI 246**, the
  one recessive tier that already exists as a name-pair on both surfaces, rather than `--text-tertiary` (which
  has no ANSI counterpart and would need a new design-system map entry). Trade accepted: it must stay
  distinguishable from `KnownResidual`, whose identity is carried by its dashed-ring glyph in the **lamp**
  column — a different column from the Mode cell.
- **Correction applied at planning:** one `label()` spelling on all three surfaces. An option preview showed
  the webview rendering the prose form `not-Conductor's` while Markdown/CLI showed the wire `not-conductors`;
  a second display form would break the cross-surface text selector the never-color-alone tests key on, so the
  single `CoverageMode::label()` spelling is used everywhere. `layout-templates.md:126`'s `not-Conductor's`
  remains prose naming the value.
- **Validation-1 amended `scope.md` (intent-incomplete):** the scope assumed all three surfaces carried a
  roll-up; research found the CLI printed none.
- **Operator directive (this wrap):** the `cargo audit` red is **external decay** — record it as a bounded
  deferral (re-check next chunk; if it persists, raise the cargo-audit floor to the fixed release), **no
  `deny.toml` entry, no CI edit**; and propose an External-decay rule into `playbook.md` on this second
  occurrence.
- **Operator directive (this wrap):** the four Expected amendments below ride the owned path — detectors +
  cascade, sidecars per doc.

## Expected amendments (authored by NO ONE yet — for this wrap's detectors)

Recorded in `plan.md` §Implementation notes; `/implement` authored none of them. Listed here so the P2
detectors can find them from the report alone:

1. **`layout-templates.md` §Component — Primary content block 1 (`:126`)** — the Mode-cell entry documents the
   four values but not their treatment. **Direction:** EXTEND that entry with the out-of-scope treatment (the
   `--status-residual` ↔ ANSI 246 name-pair, the label-carries-the-signal rule, and the in-scope roll-up
   form). Do not re-author the block — it was authored at the previous chunk's wrap in explicit anticipation
   of this one.
2. **`obs-plan.md` §4** (coverage-matrix completeness gate) — its `coverage_percent` / `p_id_count_expected`
   attributes are now **pre-bound** by a roll-up whose denominator excludes the 16 out-of-scope rows.
   **Direction:** record the denominator's meaning so the Epoch-6 gate and this roll-up cannot diverge
   silently. Keep it manifest-derived; never a literal (82 or 66).
3. **`design-system.md` §Surface: cli** — the `--status-residual` ↔ ANSI 246 pair now has a **second non-lamp
   reuse** (the out-of-scope Mode cell), alongside the `hint:` label documented at `:314`. **Direction:**
   record the second reuse so the ANSI map stays an accurate account of where the tier is used.
4. **`layout-templates.md` (`:246`) + `design-system.md` (`:310`)** — both head a block "coverage-matrix / SLO
   table" and then enumerate the **results** table's 6 columns (`P-ID · scenario · state · slo_tier ·
   latency_ms · fingerprints`), while `conductor coverage` renders **4** (`P-ID · Title · Category · Mode`).
   **Pre-existing conflation, NOT introduced by this chunk** — surfaced during phase-P3 research and recorded
   in `research.md` §Spec↔reality notes. **Direction:** separate the two tables, or re-title the block.

## Outcome

**Acceptance criteria: met.** All 12 criteria in `plan.md` §Acceptance Criteria hold — treatment lands in the
Mode cell of the existing dense list (no new column/card/grouping break) · tokens by name with zero new
`:root` vars and zero new ANSI map entries · no `[FAIL]`/`[BLOCKED]` prefix, no ANSI 203/60, no precondition
slot on any coverage surface (tested on both Rust surfaces) · label renders with colour stripped · roll-up
carries the in-scope denominator with per-mode counts still summing to the row count, all manifest-derived ·
golden re-baselined not relaxed · no new deps/commands/capabilities/env handles · leak tests green over the
new output · no telemetry primitive added · no panic on degenerate row sets.

**Gates green** (commands run): `cargo nextest run -p conductor-report -p conductor-cli` (64/64) ·
`cargo nextest run --workspace --profile ci` (**447/447**, +6, zero retries) ·
`cargo test --workspace --doc` (ok) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) ·
`npm --prefix crates/conductor-tauri/ui run build` (ok) · `bash scripts/agent-run.sh run` (exit 0) ·
`cargo deny check` (advisories · bans · licenses · sources — all ok). Green in **1 fix iteration**.

**`cargo audit` — RED, external decay, not this chunk's.** Exits 1 with
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`, identically on a
fresh fetch and with `-n` against the cached clone — the RustSec database itself will not parse.
Independently reproduced. This chunk has **zero dependency delta** and both lockfiles are un-drifted, and the
overlapping supply-chain signal is intact (`cargo deny check advisories` ok). Handled as a **bounded
deferral** per operator directive: re-check at the next chunk; if it persists, raise the cargo-audit floor to
the fixed release. No `deny.toml` entry, no CI edit. Note: CI's separate `cargo audit` step will hit the same
wall until upstream fixes the duplicate.

**Smoke ✓** (UI surface changed + `agent-run.sh` listed). `agent-run.sh run` ran as a P2 gate (exit 0,
recorded not re-run). The real binary was driven under `timeout 120`: `conductor coverage` printed
`82 capabilities · 66 in scope (43 auto · 16 drive+observe · 7 static-only) · 16 not-conductors` with no
escape bytes when piped; `conductor coverage --write` produced a Markdown artifact whose out-of-scope rows
carry `_not-conductors_` and which contains **zero** bracket verdict tokens. That artifact was inspected and
**deleted** — `plan.md` declares zero new files and `coverage-matrix.md`'s fate is an open decision owned by
the Epoch-6 *Coverage completeness gate* entry. The headful a11y leg is display-gated (Linux+xvfb + live
Pulse) and was **skipped-recorded**, never silently passed.

**Verification matrix:** no capability claims this marker (phase linked none — verified programmatically), so
the `ref`/`implemented` write was a no-op. `v2-30` (*Live per-P-ID verdict lamps*) is recorded in `plan.md`
§Provenance as **partially advanced**: its second clause (a no-run-record row must render a distinct
not-yet-run treatment rather than a failure treatment) is the same never-reads-as-failure discipline this
chunk hardens in the same component, but its first clause needs the Epoch-5 live verdict join. It stays owned
by the Epoch-5 entry.
