# Report — 2026-06-23-conductor-run-suite-report-verbs

**Chunk:** conductor run/suite/report verbs — clap CLI over current_thread bootstrap; wires timeline/emit/verify/report seams behind the three verbs, opens Epoch 8 (conductor-cli)
**Date:** 2026-06-23T18:00:59Z
**Commits:** none yet — this wrap creates the chunk commit (prior `last_wrap` boundary commit was the Epoch-7 closer)

## Changes (structured — detectors read this)
- **Files:**
  - *new:* `crates/conductor-cli/src/cli.rs` · `src/paths.rs` · `src/pipeline.rs` · `src/commands/{mod,run,suite,report}.rs`
  - *modified:* `crates/conductor-cli/Cargo.toml` · `src/main.rs` · `tests/cli_smoke.rs` · workspace `Cargo.toml` · `Cargo.lock`
  - *route/folder (phase):* `.andromeda/master-route.md` (pending record) · `conductor-0.1.0/working-route.md` (freeze stamp) · `conductor-0.1.0/chunks/2026-06-23-conductor-run-suite-report-verbs/`
- **Symbols / APIs:** the `conductor` bin gains the clap verbs `run <scenario|P-ID> --seed` · `suite [--filter] [--seed]` · `report [<run_id>]`. New CLI-crate-internal items only: `cli::{Cli,Commands}` · `paths::Paths` · `pipeline::{Preflight, preflight, execute_scenario}` · `commands::{run,suite,report}`. **No new public library API, no IPC method, no endpoint, no port** (scope law: the CLI is a gRPC/MCP client + artifact reader/writer; no inbound listener added).
- **Env vars (consumed, all pre-existing in arch §Occupied Resources):** `CONDUCTOR_SCENARIOS_DIR` · `CONDUCTOR_RUNS_DIR` · `CONDUCTOR_CONTRACT_MANIFEST` · `CONDUCTOR_SEED` (CLI `--seed` precedence) · `ANDROMEDA_PULSE_DATA_DIR` (propagated to the sidecar via conductor-verify). **None newly introduced.**
- **Crates / modules:** `conductor-cli` gains modules `cli`/`paths`/`pipeline`/`commands`. New resolved cross-crate dep edges `conductor-cli → {conductor-timeline, conductor-emit, conductor-verify, conductor-report}` — the composition-root edges (previously cli → core only). Library seams remain mutually independent; no forbidden edge.
- **Dependencies:** **added `clap` 4 (`derive`)** — NEW to `[workspace.dependencies]` (workspace `Cargo.toml`) + consumed by `conductor-cli`. `conductor-cli` also now consumes existing workspace deps `tokio` (features `macros`,`rt`), `anyhow`, `tracing`, `serde_json`. `Cargo.lock` updated (clap + transitive clap_builder/anstream/anstyle*/strsim). No version bumps to existing deps.
- **Schema / config:** none — the run-report envelope (`RunRecord`, 11 fields) is unchanged; no new config key, no migration, no scenario-model change (zero core/seam model change).
- **Coverage of new surfaces:**
  - `conductor run/suite/report` (CLI verb surface) → validation **garde✓** (`Scenario::from_toml_str` + `resolve_under` path-handle guard at the edge) · instrumentation **partial** (seam child-spans `timeline.execute`/`emit.batch`/`verify.readback.*` fire on the live path; the CLI-level root `scenario.run` span was NOT added — `run_id`-on-every-line correlation IS satisfied via `init_observability`) · PII **n/a** (CLI handles no PII; `sanitize_error` redaction at the anyhow edge✓) · tests **e2e✓** (7 `assert_cmd` cases) · a11y **n/a** (CLI, not webview) · tokens **n/a** (ASCII `Lamp::status_prefix()` `[PASS]/…`, never color-alone; no design tokens)

## Deviations from intent
- **`clap` added to the workspace manifest** (research listed only `conductor-cli/Cargo.toml` + assumed all deps were already workspace deps). Justified: `clap` was absent from `[workspace.dependencies]`; added via the conventional mechanism (CLI references `clap.workspace = true`), tightly coupled to the in-scope CLI manifest edit.
- **`rmcp` deliberately NOT added to the CLI** (plan implied a read-back result inspection). Justified: matched `query_incident_list`'s `Result` with `_` (no `CallToolResult` field access) + a coarse observed marker — keeps the CLI seam-thin; faithful observed-extraction is the deferred Epoch-10 bridge.
- **`serde_json` added to CLI deps** (regular + dev). Justified: the `report` verb parses JSONL→`RunRecord`; the E2E asserts the Blocked state. In-scope (`conductor-cli/Cargo.toml`).
- **Preflight ordering: connect-before-manifest-load.** Justified: the no-Pulse path reaches `Blocked` without needing a `contracts/` manifest in cwd (hermetic test); a missing manifest on a *connected* path is still a harness `Err`.
- **Hermetic E2E technique** — force read-back-unreachable via an injection-metacharacter `ANDROMEDA_PULSE_DATA_DIR` (rejected before any spawn) → deterministic `Blocked` on any host without spawning the live sidecar. Justified: deterministic + hermetic; exercises the real read-back-unreachable precondition (not a mock/skip).
- **clippy `collapsible_if` → edition-2024 let-chains** (paths.rs ×2, report.rs ×1) — in-scope lint cleanup (Rust 1.95).
- **obs root `scenario.run` span not added at the CLI level** (plan step 5 / obs-plan §4 / obs-extract AC2). Justified-but-incomplete: the seam child-spans cover timeline/emit/verify, and the obs `run_id`-correlation invariant is satisfied via `init_observability("conductor", Some(run_id))`; the CLI root span is a small deferred follow-up (low impact — the live path where it would parent children is itself Epoch-10-gated). **Carried to handoff.**
- **Live measured path is dormant in ch1 (canary reality).** Within the documented Epoch-10 deferral: a live run's preflight blocks on the data-dir canary round-trip until faithful canary emission lands (Epoch-10), so a ch1 run produces a `Blocked` envelope in both CI (connect-fails) and a default live Pulse (canary not yet faithful). The emit→read-back→classify orchestration is wired, compiles, and is correct, gated behind `ready`. Not a spec contradiction (scope amendment + plan notes already place faithful emission/canary in Epoch-10).

## Decisions & corrections
- **P4 AskUserQuestion → "coarse live + full Blocked path"**: ch1 wires the full orchestration; the two faithful content bridges (per-scenario emission; per-check read-back observed-extraction) are deferred to Epoch-10 ("Live-Pulse E2E proof").
- **Zero core/seam model change held** — the CLI is the composition root; the seams are mutually independent (timeline/emit/verify/report each depend only on `conductor-core`), so the CLI is the first/only place the full pipeline composes.
- **Empty-`expected` (operator-checklist / declare-only) scenarios → ManualCheck record via direct `RunRecord` construction** (`verdict: None`, `state: ManualCheck`) — confirmed representable today (`Lamp::for_record` lamp.rs:44 maps `(ManualCheck, None) → Manual`); no core constructor added.
- **Exit-code discipline**: 0 unless any record lamps `Fail`; `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` exit 0 (test-plan §1).
- **`clap` is a new dependency** → `cargo audit`/`deny` should run at the release/CI gate (clap is advisory-clean).

## Outcome
- **Acceptance criteria:** met, except the obs root-span criterion (partial — see Deviations; child-spans + run_id correlation present). Verbs, current_thread bootstrap, no-Pulse→Blocked persist (journal+db+md), suite aggregation, report render, ASCII status prefixes, sanitized edge, garde/path-guard — all realized.
- **Gates (commands run):** `cargo nextest run -p conductor-cli --profile ci` → 7/7 · `cargo nextest run --workspace --profile ci` → **373/373** · `cargo test --workspace --doc` → ok (3 doctests) · `cargo clippy --workspace --all-targets -- -D warnings` → clean. Goldens UNCHANGED (no fixture/seed touched).
- **Smoke (boot-path changed):** `bash scripts/agent-run.sh run` → exit 0.
