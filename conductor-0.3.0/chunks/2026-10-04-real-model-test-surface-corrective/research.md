# Codebase Research — 2026-10-04-real-model-test-surface-corrective

## Scope
- **Depth:** deep (one 2602-line test file, its two shared modules, one gate, two harness shells) · **Reads:** 14 · **Globs/Greps:** 12
- **Harness rules consulted:** `.claude/rules/verification-harness.md` and `.claude/rules/testing.md`, both whole, Session Additions included (both loaded in this window). Applied: testing.md 2026-06-21 + its 2026-10-03 extension (compile every feature-gated target), 2026-09-07 nextest selector form (`--test <target>`, never a bare positional), 2026-09-09 baseline-against-exact-targets, 2026-09-10 per-crate `cargo test` sequencing, 2026-09-12 inverse-control-for-a-both-sides-green arm, 2026-09-16 fixture-prose-must-not-spell-its-absence. No live leg in this chunk.
- **Platform issues consulted:** none — no runner-only bullet was folded (Setup 5a read no red), and no CI-reading entry sits outside the operator pass.

## Files inspected
- `crates/conductor-run/tests/real_model_harvest.rs` (full outline; 1553-2002 read) — 2602 lines (`wc -l`), 102 tests (`grep -c '#\[test\]'` = 102 = `cargo nextest list -p conductor-run --test real_model_harvest` 102 lines). Families by line: rule section 64-452; synthetic rule arms a01-a21 + posture table 509-766; P-031/P-034/P-044 further grades + canary tokens 768-1060; witness / route / trace arms 1061-1118; host-path mask + sweep window 1119-1199; canary attribution over Pulse log lines 1200-1428; fingerprint elision + capture-token arms 1429-1552; digest helpers 1553-1603; the 2026-09-23 pinned capture + rule-predates + tamper + no-capture-text arms 1605-1808; the 2026-09-29 series 1810-1993; the 2026-09-30 series 1995-2197; the 2026-10-01 series + the 14-capture population arm 2199-2479; workspace-key mask arms 2481-2602.
- `crates/conductor-run/tests/real_model_series/mod.rs` (full, 97 lines) — three `Drive` tables (`SERIES` ×6, `SERIES_2026_09_30` ×3, `SERIES_2026_10_01` ×3) + their evidence dirs; data only.
- `crates/conductor-run/tests/real_model_common/mod.rs` (outline + 1-26) — `rule_section`, the canary-line pairing, `elide_fingerprints`, `mask_workspace_key`, `workspace_rendering`, `sweep_window`, `mask_host_paths`. Carries NO `#![allow(dead_code)]` today.
- `crates/conductor-run/tests/real_model_live.rs` (1-75) — `#![cfg(feature = "live-pulse")]`; `rule_record` (`:66-71`) prints `rule_section(include_str!("real_model_harvest.rs"))`.
- `crates/conductor-run/tests/evidence_pin/mod.rs` (1-45) — ALREADY ships `sha256_hex` · `check_digest` · `committed` · `pinned` (`#![allow(dead_code)]`), used by `lifecycle_harvest.rs` and `delegated_timing_harvest.rs`; the harvest's own `:1553-1590` are private copies of the same four.
- `crates/conductor-core/tests/secret_scan_gate.rs` (160-200, 276-300) — `workspace_files()` `:172-197` spawns `git -C <root> ls-files -z --cached --others --exclude-standard` and `assert!`s success at `:185-189`; the headline test `:280-301` keeps two vacuity guards (`:284-287` listed nothing, `:288-291` the listing lacks this gate) plus `:293-296` no text file scanned.
- `.github/workflows/ci.yml` (step list; 76-80, 112-116) — `Secret-scan gate` (`:76-80`) runs `test -n "$(git ls-files)" || … exit 1` BEFORE `cargo nextest run -p conductor-core --test secret_scan_gate --profile ci`; `Test + lint (dogfood agent-run)` (`:112-116`) runs `.\scripts\agent-run.ps1 run`.
- `scripts/agent-run.sh` (248-321) · `scripts/agent-run.ps1` (343-375) — the bundled-default `run` arm is nextest + `test --doc` + `clippy --workspace --all-targets -- -D warnings` (`.sh:319-321`, `.ps1:373-375`).
- `crates/conductor-verify/Cargo.toml` (full) — `stub-server = ["tokio/io-std"]`; `[[bin]] stub_pulse_mcp` `required-features = ["stub-server"]` (`:26-29`).
- `crates/*/Cargo.toml` `[features]` — exactly two feature sets in the workspace: `conductor-verify/stub-server`, `conductor-run/live-pulse`. Gated files: `preflight_spawn.rs` (stub-server); `real_model_live.rs` · `live_suite.rs` · `lifecycle_live.rs` · `p075_round_live.rs` · `span_landing_live.rs` (live-pulse) — `grep -rln 'cfg(feature = "live-pulse")\|cfg(feature = "stub-server")' crates/` = 6 files.
- `.andromeda/runs/2026-10-04T09-27-47-code-audit/_mutants-conductor-core-nocopyvcs.log` (898 B, whole) — the unmutated copy-mode baseline panics at `secret_scan_gate.rs:185:5`, "`git ls-files` exited exit status: 128 — the gate has no subject".

## Graph impact (from the code-graph query; rust plane, `db_state: fresh`, trace `tree-query-2026-10-04-real-model-test-surface-corrective.json` rows 38)
- **`rule_section`** — callers at `real_model_harvest.rs` 1537 · 1716 · 1718 · 1837 · 1840 · 2059 · 2061 · 2250 · 2252 (editor lines). `real_model_live.rs:67` is ABSENT from the graph: the file is `live-pulse`-gated and so not indexed. Grep finds it (`grep -n rule_section crates/conductor-run/tests/real_model_live.rs` → `:42` import, `:67` call). The graph's blind spot to feature-gated targets is the CARRY's own subject, measured here.
- **`workspace_files`** — one caller, `the_workspace_holds_no_secret_shaped_string` at `secret_scan_gate.rs:282`.
- **`mask_workspace_key` · `elide_fingerprints`** — every caller in the graph sits in `real_model_harvest.rs` (lines 1433-2566). The live capture's callers are again unindexed (feature-gated).
- No pub signature in any `src/` changes. The modify-set is `tests/` plus two shell scripts, so crate edges and shipped callers are untouched.

## Patterns detected
- **The rule is CODE whose BYTES are pinned** (`real_model_harvest.rs:64-452`, between the whole-line markers `// ---- rule: begin ----` / `// ---- rule: end ----`). `rule_section` (`real_model_common/mod.rs:15-21`) extracts it, and four tests compare it to the rule recorded INTO committed captures: `rule_predates_the_drive` `:1710-1726` (byte-exact PREFIX vs the 2026-09-23 record), and `each_series_drive_recorded_the_current_rule_before_it_fired` `:1834`, `each_2026_09_30_…` `:2057`, `each_2026_10_01_…` `:2248` (byte EQUALITY). The captures are frozen evidence under sha256 pins. So no byte between the markers may change, including a `pub` added to a rule item, an indentation, or a re-wrap.
- **Rust privacy decides where the rule may live.** The rule's items are private (`fn row`, `enum Grade`, `fn grade`, …). A sibling module (`real_model_common`, a new `mod x;`) cannot see them, and adding `pub` changes the pinned bytes. A CHILD module of the file holding the rule CAN see them (`use super::*` / `use crate::*` reach an ancestor's private items). The only way to reach them from another test TARGET is a textual `include!` of a file holding the rule verbatim.
- **Per-series triplication** (the audit's self-clones): each dated series repeats one 8-test shape — capture loader · digest arm · rule-recorded arm · no-fingerprint arm · `measured_*` table · grades-as-the-ledger arm · eleven-keys arm · v3-09-not-met arm. Measured pairs: 31 L envelope arm (2147/2422), 18 L grades arm (2128/2403), 16 L (1916/2123, 1916/2398).
- **Shared `tests/<name>/mod.rs` modules carry `#![allow(dead_code)]`** — `conductor-run/tests/common/mod.rs:7`, `capture_paths/mod.rs:10`, `evidence_pin/mod.rs:8`, `conductor-verify/tests/common/mod.rs:5`, `conductor-emit/tests/common/mod.rs:7` (`grep -rn 'allow(dead_code' crates/*/tests/`). `real_model_common` is the exception today.
- **The no-capture-text arm reads a FIXED list** (`:1772-1777`): `include_str!` of `real_model_harvest.rs`, `real_model_series/mod.rs`, `real_model_common/mod.rs`, `real_model_live.rs`. A new source file is unscanned unless it joins that list.

## Conventions to follow
- **Committed-file reads** anchor at `CARGO_MANIFEST_DIR` + `../..` (`evidence_pin/mod.rs:35-41`); errors name the repo-relative file, never the text.
- **Gated-target lint form** is `cargo clippy -p <crate> --features <feat> --all-targets -- -D warnings` (test-plan §9, the `live-pulse` set precedent). Measured at HEAD: `-p conductor-verify --features stub-server` exit 0 · `-p conductor-run --features live-pulse` exit 0, 0 warnings. The CARRY's weaker `cargo check --tests -p conductor-verify --features stub-server` also exits 0. Logs in the gitignored `target/phase-probe/`.
- **Both harness shells move together** — the bundled default at `.sh:319-321` / `.ps1:373-375`; CI's dogfood step runs the `.ps1`.
- **A skip line is path-free** (obs-plan §9 hygiene row; testing.md 2026-09-03 host-path anchor).

## New files to create
- `crates/conductor-run/tests/real_model_grading/` — branch A (child modules of the harvest root, one per family and series; see Open questions)
- `crates/conductor-run/tests/real_model_rule/` — branch B (the rule verbatim, `include!`d by each grading target)
- `crates/conductor-run/tests/real_model_harvest_*.rs` — branch B's per-series targets

## Files to modify
- `crates/conductor-run/tests/real_model_harvest.rs`
- `crates/conductor-run/tests/real_model_common/mod.rs`
- `crates/conductor-run/tests/real_model_live.rs`
- `crates/conductor-core/tests/secret_scan_gate.rs`
- `scripts/agent-run.sh`
- `scripts/agent-run.ps1`
- `crates/conductor-core/src/drift.rs`
- `scenarios/real-model-interpretation.toml`
- `contracts/pulse-real-model-leg-posture.md`

## Sweep record
- `real_model_harvest` over tracked files outside `.andromeda/runs/` (python over `git ls-files`): 45 files. **Live commands naming the target: 0** — `scripts/agent-run.{sh,ps1}` name only `--test real_model_live` (`.sh:185`, `.ps1:281`), and `ci.yml` names no `real_model_*` target. Path citations whose truth depends on WHERE the rule and grading live: architecture.md:71 (`row()` home) · obs-plan.md:221 · security-plan.md:121 (the reader named `real_model_harvest`) · test-plan.md:260 · `.claude/docs/tests-summary.md:22` · `contracts/pulse-real-model-leg-posture.md:144` · `crates/conductor-core/src/drift.rs:55` (doc) · `scenarios/real-model-interpretation.toml:27` (comment) · `verification-matrix.json` v3-10 `ref` (`real_model_harvest.rs::{a01..a21, p031_*, p034_*, p044_*, the_pinned_capture_…}`) and v3-09 notes. Under branch A every one stays TRUE (the rule, `row()` and the target keep their file and name); under branch B the last three code/contract sites are modify-set members and the four masters are wrap amendments. The three subject files themselves (`real_model_harvest.rs`, `real_model_common/mod.rs:1`, `real_model_live.rs:8`, `:67`) are the modify-set. The other 33 files are chunk records, ledgers and route history — historical, no change.
- `stub-server` over `.github/workflows/ci.yml`, `scripts/agent-run.sh`, `scripts/agent-run.ps1`: 0 · 0 · 0 (re-derived at `1208ca5`, python line scan).

## Open questions
- **Split shape** → blocks: plan-decision. A (recommended): one `real_model_harvest` target. The rule stays byte-identical in `real_model_harvest.rs`, and each family and series moves into a child module under `tests/real_model_grading/` that sees the rule's private items. No target rename, no master citation goes stale, and no `include!`. B: separate test targets, each `include!`-ing a verbatim rule file. Each then needs `#![allow(dead_code)]` at its target root, because every target uses a different subset of the rule. The rule's file path moves, so the live capture's `rule_record`, three code/contract citations and four masters move with it.
- **Gate placement and feature set** → blocks: plan-decision. The bundled default of `agent-run run` in both shells (CI reaches it through the dogfood step; no `ci.yml` edit; no rule-(b) count moves, since it is the same plain cargo form as the existing clippy line) vs a new named `rust`-job step. And `stub-server` alone (the CARRY) vs both feature sets (testing.md 2026-10-03; test-plan §9's owed `live-pulse` clippy line — `real_model_live.rs` reads the rule file, so this chunk's own change is only compiled by a `live-pulse` build).
