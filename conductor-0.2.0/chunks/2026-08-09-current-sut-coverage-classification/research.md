# Codebase Research — 2026-08-09-current-sut-coverage-classification

## Scope
- **Depth:** deep · **Reads:** 9 · **Globs/Greps:** 6 · **Code-graph queries:** 6

## Files inspected
- `crates/conductor-core/src/coverage.rs` (full) — `CoverageMode` (3 variants + `ALL` + `label`), `CapabilityRow` (`&'static str` fields), `coverage_matrix()`, the `const fn row(...)` helper, `static COVERAGE: [CapabilityRow; 60]`, and 6 unit tests.
- `crates/conductor-core/src/drift.rs` (full) — `KNOWN_UNCLASSIFIED` (22 ids) + `check_sut_drift` three-condition comparison; 7 tests incl. the already-proven empty-ledger case.
- `crates/conductor-core/src/capability_manifest.rs` (full) — `CapabilityManifest { sut_version, captured_at, capabilities: Vec<String> }`, `load` → `validate` (non-empty · `P-NNN` shape · duplicates), `accepts`, the `info!(count=…)` boundary line; 6 tests incl. `load_failure_message_never_contains_the_path`.
- `crates/conductor-report/src/coverage.rs` (full) — `CoverageMatrix::render/write` + `summary_line` (tallies over `CoverageMode::ALL`); 5 tests.
- `crates/conductor-cli/src/render.rs` (:150-220) — `coverage_table_styled` (4-column comfy-table: P-ID / Title / Category / Mode).
- `crates/conductor-tauri/src/commands.rs` (:108-137) — the `coverage_matrix` `#[tauri::command]`, manual `tauri.command.*` span + `count`/`latency_ms` boundary line.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` (full) — the `CoverageMode` TS union (`:6`), `CapabilityRow` interface, `tally()` (`:16-19`), the accessible `<table>` scaffolding.
- `D:\dev\projects\andromeda-pulse\andromeda-pulse-0.3.0\requirements.md` (full) — **the title/category source for P-061..P-078**, grouped under 5 theme headings.
- `D:\dev\projects\andromeda-pulse\andromeda-pulse-0.3.0\verification-matrix.json` (queried) — **the title source for P-079..P-082**, which `requirements.md` does NOT carry.

## Graph impact (from the code-graph query — row counts are the run-dir trace's `rows` field)
- **`CoverageMode`** — **158 refs**: 147 inside `conductor-core/src/coverage.rs` itself (the 60 row literals + tests), 3 `drift.rs`, 1 `lib.rs` (re-export), **6 `conductor-report/src/coverage.rs`**, **1 `conductor-cli/src/render.rs`** (`r.mode.label()` @ `render.rs:167`). Only two out-of-core consumer files.
- **`coverage_matrix`** — **15 refs** across 7 files: `conductor-cli/src/render.rs` (2), `conductor-report/src/coverage.rs` (3), `conductor-tauri/src/commands.rs` (2 — one is core's, one the command's own same-named fn), `conductor-tauri/src/main.rs` (1, the `generate_handler!` registration), plus core-internal (`coverage.rs` 4, `drift.rs` 2, `lib.rs` 1).
- **`CapabilityRow`** — **40 refs**: core 23, report 11, cli 4, tauri 2.
- **`KNOWN_UNCLASSIFIED`** — **2 refs only** (`drift.rs` definition, `lib.rs` re-export). Emptying it has essentially zero blast radius.
- **`conductor-core` crate edges** — **6 rows, all INBOUND, zero outbound**: `conductor-cli`, `conductor-report`, `conductor-run`, `conductor-tauri`, `conductor-timeline`, `conductor-verify` → `conductor-core`. Adding a variant is a downstream-only fan-out; core gains no edge.
- **`conductor-tauri` does NOT reference `CoverageMode`** (0 rows in the 158) — it moves `CapabilityRow` through serde, so the fourth variant reaches the webview as a *string*. The binding site is the TS union at `CoverageMatrix.tsx:6`, not Rust.

## Patterns detected
- **Wire-spelling single-truth** (`coverage.rs:36-42` + `:180-192`): `label()` returns exactly the serde `rename`, asserted by `mode_label_matches_serde_wire` + a JSON round-trip. Every renderer prints `label()` / the serialized string — so one new `#[serde(rename)]` propagates to Markdown, CLI and webview without a per-surface string.
- **Fixed-order tally over `CoverageMode::ALL`** (`conductor-report/src/coverage.rs:62-69`, mirrored in `CoverageMatrix.tsx:16-19`): the footer/summary iterates `ALL`, so a 4th variant auto-appears in the Markdown summary line — but the **TS `tally()` hardcodes its three `by(…)` calls** and will silently under-count until edited.
- **Pure-function render** (`conductor-report/src/coverage.rs:24-38`): no clock, no IO ⇒ exact-string golden-testable; `write` is an atomic `.tmp`→rename overwrite.
- **Manifest boundary shape** (`capability_manifest.rs:36-82`): one `load` → one private `validate`; read faults carry `e.kind()` only, parse faults go through `sanitize_error`. Any schema extension extends `validate`, never a second parse path.
- **Artifact-hygiene leak test** (`conductor-report/src/coverage.rs:107-115`): asserts the rendered Markdown contains none of `CoverageMatrix` / `CapabilityRow` / `CoverageMode` / `coverage_matrix` / host-path prefixes. A new variant's Rust identifier must be added to that list.

## Conventions to follow
- **Rust identifier ≠ rendered string**: the leak test above means the fourth variant's *identifier* must never appear in output; only its `label()` does.
- **`const fn row(...)`** (`coverage.rs:64-71`) is the table's constructor — new rows use it verbatim.
- **Manual span, not the attribute** on the Tauri command (`commands.rs:123`) — already correct; the row-count change needs no instrumentation change.
- **Category vocabulary**: existing rows use the v0.2.0 audit's category names ("Hard Signal Detection", "Privacy & Trust", …). Pulse v0.3.0's equivalents are its 5 theme headings (Window & shell hygiene · State honesty & legibility · AI-debug climax · External verification · Test gap & housekeeping).

## The 22 new capabilities — authoritative source data
`requirements.md` covers **P-061..P-078** only; **P-079..P-082 were minted mid-build as operator-surfaced caps** and exist only in Pulse's `verification-matrix.json` (chunks `2026-07-05` / `2026-07-06` / `2026-07-07` / `2026-07-09`). Both sources read; titles below are verbatim.

| P-ID | Title | Pulse theme | Pulse method |
|---|---|---|---|
| P-061 | Window geometry + movable shell | Window & shell hygiene | e2e |
| P-062 | Window size constraints | Window & shell hygiene | e2e |
| P-063 | Predictable close + honest tray | Window & shell hygiene | e2e |
| P-064 | Suppress browser context menu | Window & shell hygiene | webview |
| P-065 | Canvas not a browser image | Window & shell hygiene | webview |
| P-066 | Widget-to-dashboard navigation | Window & shell hygiene | webview |
| P-067 | Live-only service truth | State honesty & legibility | integration |
| P-068 | Anomaly surfacing | State honesty & legibility | webview |
| P-069 | Legible labeled constellation | State honesty & legibility | webview |
| P-070 | Plain-language connection status | State honesty & legibility | webview |
| P-071 | Self-explaining empty states | State honesty & legibility | webview |
| P-072 | Investigate actions functional | AI-debug climax | e2e |
| P-073 | Deterministic env-gated L4 mode | AI-debug climax | integration |
| P-074 | Tier1 incident-path reliability under load | AI-debug climax | integration |
| P-075 | Conductor e2e + delegated-timing verification | External verification | **dynamic-external** |
| P-076 | Integration UX e2e test | Test gap & housekeeping | e2e |
| P-077 | Demo telemetry injector formalized | Test gap & housekeeping | by-construction |
| P-078 | Agent-headful self-verify harness | Test gap & housekeeping | e2e |
| P-079 | Constellation severity live-wiring (workspace-key reconciliation) | operator-surfaced | integration |
| P-080 | Incidents (Findings) dropdown bounded popover | operator-surfaced | webview |
| P-081 | Traces table live refresh | operator-surfaced | webview |
| P-082 | Traces table internal scroll (fixed hero + toolbar) | operator-surfaced | webview |

**This settles scope.md's one open classification call.** P-075's own Pulse method is literally `dynamic-external` — "proven by an external harness, e.g. Conductor" — so it is unambiguously Conductor's lane, not a third category; and P-076 is Pulse's own `e2e`, so it is not-Conductor's. "The delegation itself" resolves into the two existing buckets rather than needing a new one.

## The literal-60 inventory (verified first-hand by grep, not carried from the extract)
**Assertions (7)** — the tests extract's list, all confirmed:
`core/coverage.rs:142` (`len(), 60`) · `core/coverage.rs:155-156` (`(1..=60)`) · `report/coverage.rs:89` (footer golden `"**Capabilities** 60 · 40 auto · 13 drive+observe · 7 static-only"`) · `report/coverage.rs:104` (row count) · `cli/render.rs:290` (`for n in 1..=60`) · `cli/tests/cli_smoke.rs:195` (`P-001` … `P-060`) · `tauri/commands.rs:327` (`Some(60)`).

**Test fn names (4)** — `core/coverage.rs:141` · `cli/render.rs:288` · `cli/tests/cli_smoke.rs:189` · `tauri/commands.rs:317`, all spelled `…all_sixty_pids` / `…exactly_sixty_capabilities`.

**Doc + help text (8)** — NOT in the extract's list: `core/coverage.rs:3-4`, `:49`, `:59` · `report/coverage.rs:3` · `cli/render.rs:77` · `cli/commands/coverage.rs:1` · `tauri/commands.rs:117` · **`cli/cli.rs:49`, which is clap HELP TEXT — user-visible output, not a comment.**

**False positive to guard:** `cli/render.rs:42` `Lamp::Blocked => 60` is an xterm-256 ANSI color code, not a capability count. Do not touch it.

## Files to modify
- `crates/conductor-core/src/coverage.rs` — 4th `CoverageMode` variant (+`ALL`, `label`, serde rename), 22 rows, module/field/fn doc comments, the two count tests.
- `crates/conductor-core/src/drift.rs` — `KNOWN_UNCLASSIFIED` → `[]`; keep `a_known_gap_entry_that_is_now_classified_is_drift` on its synthetic fixture (it must not reference the now-empty constant).
- `crates/conductor-report/src/coverage.rs` — footer golden + row-count assertion manifest-relative; extend the leak list with the new variant identifier; module doc.
- `crates/conductor-cli/src/render.rs` — the `1..=60` test + doc; `cli.rs:49` help text; `commands/coverage.rs:1` doc.
- `crates/conductor-cli/tests/cli_smoke.rs` — the `P-001`/`P-060` stdout assertion.
- `crates/conductor-tauri/src/commands.rs` — `Some(60)` assertion + the `:117` doc.
- `crates/conductor-tauri/ui/src/components/CoverageMatrix.tsx` — the `CoverageMode` TS union (`:6`) **and `tally()` (`:16-19`), which hardcodes three `by(…)` calls and would silently under-count**.
- Specs: `.andromeda/architecture.md:33` · `.andromeda/design-system.md:7,:257` · `.andromeda/layout-templates.md:121,:177`.

## New files to create
- None. (A per-capability fixture is only needed if P4 chooses the manifest-data representation.)

## Open questions
1. **Representation** — widen the code-native `static COVERAGE` vs move classification into the manifest TOML. Note the research finding that bears on it: `check_sut_drift` compares the manifest against the classification, so making the manifest the classification's source collapses that comparison into a self-check. → P4 (AskUserQuestion).
2. **`P-067`'s mode** — intent calls it "partially observable through the service-registry surface"; Pulse verifies it by `integration`. Auto (read-back-assertable) vs DriveObserve (operator-confirmed) is a judgment the plan must fix.
3. **`layout-templates.md:222`** — the layouts extract flagged a "step 60/60 / 55 Pass" suite caption as illustrative output that "may stay". Surface at P5; not obviously in this chunk's remit.
