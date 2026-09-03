# Codebase Research — 2026-09-03-conductor-tauri-survivors-dispositioned

## Scope
- **Depth:** deep · **Reads:** 12 · **Globs/Greps:** 9 · **Graph queries:** 4 (rust plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/testing.md` — 32 Session Additions indexed structurally
  (52 KB over 77 lines, multi-KB per line); entries `:63` `:64` `:66` `:67` `:72` `:75` `:76` `:77`
  read in full. No live-Pulse leg in this chunk; `.claude/rules/verification-harness.md` not
  consulted (no scenario/harness run against a real process).

## Files inspected
- `crates/conductor-tauri/src/commands.rs` (30-135, 160-300, 342-471) — every `commands.rs` survivor
  site plus the six shipped tests that produce the tier's only `caught` results.
- `crates/conductor-tauri/src/main.rs` (full, 49 lines) — `main`, `obs_sink`, `tauri_log_path`.
  **Carries no `#[cfg(test)] mod tests` at all**, which is the direct reason all 3 of its mutants survive.
- `crates/conductor-tauri/src/pause.rs` (50-115, 160-224) — `HoldGate::{arm,deliver}`,
  `TauriResolver::kind`, `resolve_operator_hold`, and the 6 shipped tests.
- `crates/conductor-core/src/config_path.rs` (1-80) — `resolve_under` + its three negative tests.
- `crates/conductor-cli/src/paths.rs` (grep) — the parallel `resolve_handle`; **no test module**.
- `crates/conductor-tauri/Cargo.toml` (full) — dev-deps are `tauri[test]`, `tokio`, `serde_json` only.
- `crates/conductor-run/tests/envelope_fixture.rs` + `tests/fixtures/over-envelope.toml` (full) — the
  committed-fixture + production-writer precedent the scope named.

## Graph impact (rust plane, `db_state: fresh`, 4 queries in `tree-query-…json`)
- **`resolve_handle`** (`commands.rs` def_line 36) — **3 callers, all in `commands.rs`**: `scenarios_dir`
  @43 · `runs_dir` @47 · `manifest_path` @51 (graph lines, 0-indexed; editor 44/48/52). Those are exactly
  the three survivor coordinates, so the survivor lines ARE the call sites.
- **`runs_dir`** — 3 callers: `run_report` @169 · `run_envelope` @225 · `start_run` @266.
  **`scenarios_dir`** — 2: `list_scenarios` @114 · `start_run` @260. **`manifest_path`** — 1: `start_run` @267.
- **`tauri_log_path`** — exactly 1 caller, `obs_sink` @ `main.rs:39`; `obs_sink` — exactly 1 caller,
  `main` @ `main.rs:17`. **No test reaches either.**
- **`resolve_under`** — 20 call sites. Inside `conductor-tauri`: `resolve_handle` @39 · `capabilities` @59 ·
  `load_envelope` @70 · `load_one` @101 · `run_report` @172 · `run_envelope` @228 · **`tauri_log_path`
  @ `main.rs:45`**. Its three negative tests (`accepts_an_in_scope_relative_path`,
  `rejects_parent_dir_traversal`, `rejects_an_out_of_scope_absolute_path`) live in
  `conductor-core/src/config_path.rs` — **the guard's reject semantics are already pinned; what is
  unpinned is whether the Tauri edge CALLS it.** That is precisely what the six must-kill mutants remove.
- **`crate_edges`** — `conductor-tauri → conductor-core` and `→ conductor-run`, **zero inbound**. Confirms
  no cross-package fixture reuse is available and no bin↔bin edge exists to widen.
- Existing test coverage, from the caller set: `invoke` is called by 5 tests (`coverage_matrix`,
  `unbacked_auto`, `run_report`, `run_envelope`, `stop_run`). **`list_scenarios` and `start_run` are
  registered in `test_app` but never invoked by any test.**

## Patterns detected
- **The guard is parameterised, and that is the whole key** (`commands.rs:37-41`): `resolve_handle(var,
  default)` reads `current_dir()`, takes `std::env::var(var)` **or the `default` argument**, and returns
  `resolve_under(...)`. Because `default` is a plain parameter, both the accept and the reject arms are
  reachable by passing an unset var name with two different defaults — **no staged repo root, no
  `unsafe { set_var }`**. This is what falsified the scope's LIKELY-SHAPE first clause.
- **`resolve_under` rejects, never clamps** (`config_path.rs:17-45`): absolute → Err, any `..` component
  → Err, resolved-outside-base → Err. `base` must exist (it is canonicalized); **`candidate` need not**.
  So `runs_dir()` succeeds even when no `runs/` directory exists — which makes the traversal arm of
  `run_report` / `run_envelope` reachable with an empty tree.
- **The existing run-data tests assert the mutants' own answers** (`commands.rs:437-459`):
  `run_envelope_command_returns_null_when_no_run_has_an_envelope` asserts `standing.is_null()`, which is
  exactly `Ok(None)`; `run_report_command_returns_a_well_formed_record_list` asserts only that the payload
  deserializes as `Vec<RunRecord>`. Both pass against their mutant unchanged.
- **The three timeouts are a HANG, not a blind spot** (`pause.rs:65-70`, `:166`, `:192`):
  `HoldGate::deliver` ends `sender.is_some_and(|tx| tx.send(decision).is_ok())`. Every mutant on
  `deliver` or on `resolve_operator_hold` stops the oneshot ever being sent, and
  `gate_delivers_the_decision_to_the_awaiting_hold` / `resolve_operator_hold_command_delivers_the_decision`
  then block forever on `rx.await` — so the binary times out before `deliver_is_a_no_op_without_a_pending_hold`
  (which WOULD fail the `-> true` mutant) can report. Bounding those two awaits converts timeout → caught.
- **`main` cannot be called from a test** (`main.rs:17-34`): it runs `init_observability`, builds the
  Tauri app and calls `.run(tauri::generate_context!())`, which launches a real GUI event loop and blocks.
  This is the one survivor with no observation surface at the Rust tier.
- **Committed-fixture precedent** (`envelope_fixture.rs`): fixture CONTENT in a committed file, the row
  written by the production writer `conductor_run::persist`, semantics pinned by a Rust round-trip, and a
  seeding entry point that is a **no-op unless `CONDUCTOR_E2E_SEED_DIR` is set**. It uses
  `assert_fs::TempDir`, which `conductor-tauri` does **not** carry.

## Conventions to follow
- **Ask what the mutant makes DIFFERENT on the surface you observe** before writing an assertion
  (`.claude/rules/testing.md:72`, 2026-08-20) — the governing rule for this chunk; it is the entry that
  produced the previous crate's survivor closure.
- **Never `std::env::var` inside a gate; read env at the caller and pass a typed value**
  (`.claude/rules/testing.md:67`, 2026-08-10) — the standing rule test-plan §10 cites as the shipped
  accepted-deliberate precedent. See Open question 1: its remedy is a refactor this chunk bans.
- **Mock-runtime IPC dispatch** (`commands.rs:361-400`): `mock_builder().manage(...).invoke_handler(
  generate_handler![...])` + `mock_context(noop_assets())` + `get_ipc_response`, already shipped with
  `test_app` / `main_window` / `invoke` helpers to extend.
- **Known-residual ledgers are exact-set in both directions** (`.claude/rules/testing.md:66`) — the shape
  any accepted-deliberate disposition list should take, so it can only shrink under compulsion.
- Manual `tracing::info_span!("tauri.command.<name>").entered()` on every handler; never
  `#[tracing::instrument]` (obs-plan §4). All 8 handlers carry it at HEAD.

## New files to create
- `crates/conductor-tauri/src/main.rs` — a `#[cfg(test)] mod tests` (new module, existing file) is the
  only way to reach `tauri_log_path`; a binary crate's private items are unreachable from `tests/`.
- (Conditional, only if P4 takes the populated-subject route) `crates/conductor-tauri/tests/fixtures/…`
  — a fixture that must live UNDER the crate dir, since `resolve_under`'s base is the crate dir at
  `cargo test` time and absolute/`..` candidates are rejected.

## Files to modify
- `crates/conductor-tauri/src/commands.rs` — extend `#[cfg(test)] mod tests` (14 survivors).
- `crates/conductor-tauri/src/pause.rs` — bound the two awaits at `:171` and `:224`; add a `kind`
  assertion (4-5 survivors).
- `crates/conductor-tauri/src/main.rs` — add a test module (3 survivors; 2 killable, `main` is question 1).
- `crates/conductor-tauri/Cargo.toml` — **only if** a dev-dependency is needed. The manifest is a boundary
  member per the caller-threading rule. Note the leading design (below) needs none.
- No production file changes and **no signature changes**, so there is no caller set to thread: every
  survivor's callers are already enumerated above and all are either in-crate or tests.

**Seam facts.** `conductor-tauri` is a BIN crate with zero inbound edges, so nothing it does here is
visible to another package. `tauri` is a normal dependency and additionally a dev-dependency with
`features = ["test"]`; `assert_fs` is a workspace dependency used by other members but **absent from
`conductor-tauri`'s dev-deps** — adding it is a test-only edge, and under the standing red-audit rule the
basis is the PACKAGE COUNT plus `cargo deny` green over the new lock, never lockfile byte-identity.

**The load-bearing equality, verified.** The design needs: *a mutant returning `Ok(default)` / `Ok(None)`
/ `Ok(vec![])` yields a DIFFERENT observable than correct code, for inputs a test can supply without env
mutation.* Verified for the six must-kills — correct code returns **`Err`** for a traversal `run_id`
(`run_report`/`run_envelope` @ `commands.rs:172`/`:228`) or a `..` default (`resolve_handle`), where every
mutant returns `Ok`. Verified for the three timeouts (bounded await turns a hang into a fast failure).
**Not** verified for `main:18` — no input distinguishes it.

## Open questions
1. **`main:18` has no observation surface and no standing rule prescribing its untested shape.** Killing
   it needs calling `main()`, which launches the GUI event loop. The nearest precedent
   (`.claude/rules/testing.md:67`) prescribes a refactor — read env at the caller — which is production
   change this chunk's scope bans, and `main` is not an env-reading edge anyway. → blocks: **plan-decision**
   (accept-deliberate against which rule, or surface as a finding).
2. **Two routes to the run-data kills, and they cost very differently.** (a) The **guard arm** —
   `run_report(Some("../escape"))` / `run_envelope(Some("../escape"))` assert `Err`, needing no fixture, no
   dev-dependency and no env mutation, because `runs_dir()` succeeds against a non-existent `runs/`.
   (b) The **populated arm** — a seeded runs dir, which `resolve_under` forces UNDER the crate dir and
   therefore requires `unsafe { set_var }` for `CONDUCTOR_RUNS_DIR`, plus a fixture and probably
   `assert_fs`. Route (a) alone kills both mutants; route (b) additionally satisfies the layouts/a11y
   preference for a populated over-envelope subject — which the `--e2e` arm and
   `envelope_fixture.rs::the_persisted_standing_round_trips_through_the_production_reader` already cover
   elsewhere. → blocks: **plan-decision**.
3. **Firing form for the verifying re-run.** test-plan §4 mandates `-f` with full workspace-root paths,
   but the measured 43/43 basis used `-p conductor-tauri` alone. The package holds exactly three `.rs`
   sources (`build.rs` was not mutated), so `-p` already scoped to what `-f` would name and the two are
   **equivalent here** — keeping `-p` preserves comparability with the 43-mutant basis. → blocks:
   **implementation-scope** (a note for the plan's Test Commands, not a fork).
