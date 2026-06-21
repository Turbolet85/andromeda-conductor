# Report — 2026-06-21-markdown-run-report

**Chunk:** Markdown run report — per-scenario Pass/Fail/ManualCheck/KnownResidual/Blocked render + verdict-first lamp precedence (conductor-report)
**Date:** 2026-06-21T20:13:24Z
**Commits:** none yet — this wrap creates the chunk commit (the `runs-db-index` commit predates this work).

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-core/src/lamp.rs`
  - MOD `crates/conductor-core/src/lib.rs` (`mod lamp;` + `pub use lamp::Lamp;`)
  - NEW `crates/conductor-report/src/report.rs`
  - MOD `crates/conductor-report/src/lib.rs` (`mod report;` + `pub use report::{ReportError, RunReport};`)
- **Symbols / APIs (new public library surface only — no ports/sockets/IPC/endpoints/env vars):**
  - `conductor-core`: `enum Lamp { Pass, Fail, Hold, Manual, Residual, Blocked }`; `Lamp::for_record(&RunRecord) -> Lamp` (the verdict-first resolver); `Lamp::status_prefix(&self) -> &'static str`; `Lamp::label(&self) -> &'static str`.
  - `conductor-report`: `struct RunReport`; `RunReport::render(run_id: &str, &[RunRecord]) -> String` (pure, no IO/clock); `RunReport::write(runs_dir: &Path, run_id: &str, &[RunRecord]) -> Result<PathBuf, ReportError>` (create_new, never-overwrite); `enum ReportError { Io }` `#[non_exhaustive]`.
- **Crates / modules:** `conductor-core` +`mod lamp`; `conductor-report` +`mod report`. No crate added/removed.
- **Dependencies:** **none added.** Render uses stdlib + `conductor-core` + the already-present `serde_json` (enum wire forms) + `thiserror` (error). No `Cargo.toml` change in either crate.
- **Schema / config:** none. Consumes the existing `RunRecord` envelope unchanged — no new envelope field, no migration, no config key, no `runs.db` schema change.
- **Coverage of new surfaces** (the only new surface is an on-disk artifact write — not external/network, not a UI element):
  - `runs/<run_id>.md` (Markdown report artifact write) → validation n/a (renders already-validated envelope values; `runs_dir` resolved at the cli edge, Epoch 8) · instrumentation ✗ (no `report.generate` span yet — emitted by the Epoch-8 cli/timeline caller; the primitive has no driver) · PII redacted✓ (hygiene test asserts no host-path / struct-name leak; blocked-row null rule renders `—`, never `null`) · tests unit✓ (15 new) · a11y n/a (Markdown is not interactive UI; status-never-color-alone honored via the ASCII prefix) · tokens n/a (Markdown carries no design tokens; status encoded by the `[PASS]`/…/`[BLOCKED]` ASCII prefix)

## Deviations from intent
- **Seed shown in the summary only when uniform across records** (omitted otherwise), and not repeated per-row. Plan/scope said "summary includes seed"; `seed` is a per-record envelope field (runs.db keys it per-record), so a single summary seed is only correct under the run invariant — guarding on uniformity avoids a misleading value. Minor refinement, not a divergence.
- **`render` is a public `RunReport::render`** (reachable via the exported `RunReport`), so cli/desktop can reuse the pure render-to-string later — consistent with the plan's "pure, golden-testable render".
- **`wire()` uses the existing `serde_json` dep** to take enums' canonical wire spelling (`<5s` etc.) drift-free, mirroring `db.rs::value_as_wire` — no new crate added, honoring the plan's dependency rule.
- The two decisions flagged at phase review were **user-confirmed and implemented as planned** (not deviations): `KnownResidual`/`Blocked` are state-driven (checked before the verdict arms); `render` reads no wall-clock.

## Decisions & corrections
- **Verdict-first lamp precedence is realized as the shared `Lamp` type in `conductor-core`** — the first of four surfaces (coverage-matrix Epoch 6 ch4, cli Epoch 8, desktop Epoch 9 reuse it unchanged). Every P2 distiller converged on the core placement; user confirmed at phase review.
- **Precedence ordering:** `Blocked`/`KnownResidual` are state-driven and checked *before* the verdict arms, because `RunRecord::measured()` always supplies a `Verdict` — a naive "verdict-first else state" would mis-render a `KnownResidual` row as its raw verdict (`[FAIL]`/`[PASS]`), defeating the state's purpose. Verdict-first governs only Pass/Fail/Hold and the `CalibrationRegion → [HOLD]` (not `[MANUAL]`) case.
- **Renderer is a pure function of `(run_id, records)`** — no clock read, so the artifact is deterministic + exact-golden-able; a future "generated at" is injected by the cli edge, never read in the seam.
- **`write` uses `create_new`** so a repeat run_id is a loud `Err`, never a silent clobber (mirrors `RunsDb`'s loud duplicate-key rule).
- **Fingerprints distinction:** `Some([])` (measured, none) → `(none)`; `None` (never measured) → `—`.

## Outcome
- **All acceptance criteria met** (verdict-first lamp incl. CalibrationRegion→`[HOLD]`; KnownResidual→`[RESIDUAL]` even with a verdict; blocked row identity+tier with measurement fields em-dashed, never `null`; no host-path/struct-name leak; never-overwrite; deterministic render; no new dep).
- **Gates green:** `cargo nextest run -p conductor-core -p conductor-report` (114/114) · `cargo nextest run --workspace --profile ci` (**290/290**, +15) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `cargo test --doc -p conductor-core -p conductor-report` (0 doctests, ok).
- **Smoke:** skipped — no boot-path change (library-only; cli wiring is Epoch 8; `agent-run.sh status` needs a `<run_id>` and does not consume the renderer yet). No regression — the full workspace incl. `conductor-cli` compiles clean under clippy.
