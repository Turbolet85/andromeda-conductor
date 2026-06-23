# Codebase Research — 2026-06-23-line-oriented-output-rendering

## Scope
- **Depth:** deep · **Reads:** 12 files (full) · **Globs/Greps:** 2 globs · 1 grep · 1 code-graph query

## Files inspected
- `crates/conductor-core/src/lamp.rs` (full) — **THE reuse target.** `Lamp` enum (6 variants: Pass/Fail/Hold/Manual/Residual/Blocked) + `Lamp::for_record(&RunRecord)` (verdict-first mapping, lamp.rs:37) + `Lamp::status_prefix()` → `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (lamp.rs:52) + `Lamp::label()` (lamp.rs:64). Module doc: _"One source of lamp truth, reused by the Markdown report, coverage matrix, cli, and desktop."_ → ch3 is the cli consumer; it adds COLOR over these, never reclassifies.
- `crates/conductor-cli/src/commands/mod.rs` (full) — `print_record` (mod.rs:33) prints `{Lamp::status_prefix()} {scenario}` with comment _"(color is ch3)"_ — the explicit insertion point. `persist` writes journal+db+md. `exit_code` = `FAILURE` iff any `Lamp::Fail`.
- `crates/conductor-cli/src/commands/{run,suite,report,preflight}.rs` (full) — current output paths. `run`→`print_record` (1 line). `suite`→per-scenario `print_record` loop. `report`→`print!("{}", RunReport::render(...))` (Markdown to stdout). `preflight`→already prints `[PASS] preflight ready` / `[BLOCKED] preflight — {precondition}` (preflight.rs:17-22, comment _"color is ch3"_).
- `crates/conductor-cli/src/cli.rs` (full) — clap surface is exactly 4 verbs (Run/Suite/Report/Preflight). **No `coverage` verb exists.**
- `crates/conductor-report/src/report.rs` (full) — **the mirror pattern.** `RunReport::render` is a pure fn; `summary_line` tallies per-lamp via `status_prefix()` (report.rs:70); `scenario_block` leads each section with `Lamp::for_record(rec).status_prefix()`; blocked rows collapse the 5 never-measured fields to `ABSENT="—"` (report.rs:21,86); `wire(serde_json::to_value(...))` yields canonical serde spellings (`SloTier`→`<5s`). Golden-style tests assert determinism + no host-path/struct-name leak.
- `crates/conductor-report/src/coverage.rs` (full) — `CoverageMatrix::render` over `coverage_matrix()` (60 rows: `P-ID | Title | Category | Mode`; tally `60 · 40 auto · 13 drive+observe · 7 static-only`). **Static classification — carries NO lamp/verdict** (distinct from per-run RunRecords). Writes `coverage-matrix.md` via atomic overwrite (`.tmp`→rename).
- `crates/conductor-cli/tests/cli_smoke.rs` (full) — **the E2E contracts to preserve** (see Conventions).
- `crates/conductor-cli/src/pipeline.rs` (full) — `readiness()` returns `ReadyState` (fields: `ready`, `negotiated_protocol_version`, `expected_protocol_version`, `required_tools: Vec<(name, ToolPresence)>`, `data_dir`, `canary_round_trip`, `blocked_precondition`, `checked_at`) — the preflight render source. `execute_scenario` is the suite loop body (one RunRecord per scenario).
- `crates/conductor-cli/Cargo.toml` + root `Cargo.toml` — deps today: clap/tokio/anyhow/tracing/serde_json + the seams. **owo-colors / indicatif / comfy-table are absent** — to add.

## Graph impact (code-graph query → `tree-query-2026-06-23-line-oriented-output-rendering.json`)
- **`Lamp` @ `conductor-core/src/lamp.rs:15`** — 6 variants + `for_record:37` / `status_prefix:52` / `label:64`. Consumers already: `RunReport` (report.rs summary_line + scenario_block), `print_record`/`exit_code` (cli mod.rs), `CoverageMatrix` does NOT use it. → ch3 adds the cli render consumer; **additive, zero core change** (leaf-additive: a new cli render module has no inbound cross-crate callers).
- **`grep owo|indicatif|comfy|is_terminal|NO_COLOR` over `crates/`** → 0 source hits (only the bundled webview `dist` JS). Confirms a greenfield render seam — no existing tty/color machinery to reconcile.

## Patterns detected
- **Single lamp-truth source** (`lamp.rs` module doc): every surface maps through `Lamp::for_record` → `status_prefix`/`label`; never a second classification. ch3 colorizes the SAME projection.
- **Blocked-row `ABSENT="—"` collapse** (report.rs:21,86-96): the 5 never-measured fields render em-dash, never `null`. The comfy-table must apply the identical rule per cell.
- **Canonical serde spelling via `wire()`** (report.rs:160): table cells for `slo_tier`/`state` reuse the serde wire string so cli/Markdown/db/JSONL stay identical.
- **Pure-function, clock-free renders** (report.rs:55, coverage.rs:24): renders are deterministic + golden-tested; keep color/tty-detection at the OUTPUT edge, leaving the data projection pure.
- **`print_record` already lamp-prefixed** (mod.rs:34) — the colorization target; `preflight` non-json already prefix-lined (preflight.rs:17-22).

## Conventions to follow
- **"color is ch3" markers** (mod.rs:32, preflight.rs:6): the spec-sanctioned insertion points; color is added OVER the existing ASCII prefix, never replacing it.
- **No host-path / struct-name leak** (report.rs:293, coverage.rs:108 tests): new render tests must assert the same denylist (`C:\`, `/Users/`, `/home/`, internal struct names).
- **cli_smoke.rs substring contracts (MUST hold):** `run` stdout ⊇ `[BLOCKED]` + scenario name (cli_smoke.rs:73); `suite` stdout ⊇ both scenario names (:108); `report` stdout ⊇ `Run report` + `[BLOCKED]` (:122); preflight-json stdout must NOT contain the host path (:137). → colored output is tty-gated (piped test stdout is plain); **comfy-table must not truncate the scenario column or the `[BLOCKED]` token.**
- **anyhow at the cli edge, thiserror internal** (security/error wall); `tonic::Status`/MCP errors are values, never panics.

## New files to create
- `crates/conductor-cli/src/render.rs` (or `render/` module) — the owo-colors/indicatif/comfy-table seam: colored status line (over `Lamp`), the per-run results comfy-table, the `Lamp`→ANSI-256 color map (114/179/203/60/146/246 + ID-cyan 117), and the indicatif spinner helper (stderr, tty-gated).
- `crates/conductor-cli/src/commands/coverage.rs` — **only if** the static-matrix `coverage` verb is chosen at P4 (open question 1).

## Files to modify
- `Cargo.toml` (root) + `crates/conductor-cli/Cargo.toml` — add `owo-colors` (feature `supports-colors`), `indicatif`, `comfy-table` (workspace dep + cli consumer).
- `crates/conductor-cli/src/commands/mod.rs` — `print_record` → colored; wire `suite`/`report` to `render`.
- `crates/conductor-cli/src/commands/{suite,report,preflight}.rs` — spinner (suite) · results table (suite/report) · colorize lines (preflight).
- `crates/conductor-cli/src/main.rs` + `cli.rs` — register `render` module; add `Coverage` verb **only if** chosen at P4.
- `crates/conductor-cli/tests/cli_smoke.rs` — extend: `NO_COLOR`/piped output is plain (no escape bytes), table contains the names/tokens, no host leak.

## Open questions
1. **"Coverage table" scope + surface** (P4 AskUserQuestion): the per-run results table (suite/report, lamp/state/latency) vs the static 60-P-ID matrix (`coverage_matrix()`, P-ID/title/category/mode) vs both — and if the static matrix is in scope, does it get a new `conductor coverage` verb (a layout-templates amendment, precedented by ch2's `preflight` verb; also closes the carried `coverage-matrix.md`-at-repo-root follow-up) or fold into an existing verb?
2. **`report` rendering** (resolve in P4): switch `report` stdout from raw Markdown to the colored comfy-table (preserving the `Run report <id>` header + `[BLOCKED]` token for the E2E), or keep Markdown and make the table suite-only? Default: switch, with header/token preservation (minimal-surprise terminal UX; the `.md` artifact stays Markdown).
