# Codebase Research — 2026-09-06-operator-gated-live-suite

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 16 · **Graph queries:** 3 (rust plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL as a structural extraction
  (the file tripped the 51.7 KB output cap; its 20 `## Session Additions` entries run 246–13 378 bytes on a
  SINGLE line each, so a line-length index drove four offset-bounded reads covering lines 15–36 and 40–59, every
  indexed span). **20 additions applied** — the live-leg recipe below is assembled from them, not improvised.
  Also consulted: `.claude/rules/observability.md` + `.claude/rules/testing.md` (both `paths:`-cover the chunk's
  likely modify-set; the obs one also covers `scripts/agent-run.*`).

## Files inspected
- `crates/conductor-run/src/execute.rs` (48–185, 219–268) — `execute_scenario`'s spine: the three-arm
  `route_read_back` match, `manual_record`, `state_for`, the declare-only branch. The chunk's W3 observables 1+2
  both live here.
- `crates/conductor-run/src/canary.rs` (29–62) — `preflight`'s two early-return arms and the gate guard;
  `Preflight`'s crate-private fields. W3 observable 3 lives here.
- `crates/conductor-run/src/envelope.rs` (26–56) — `persist` writes journal + `runs.db` rows +
  `insert_envelope` (unconditional) + the Markdown report; `read_envelope` is its read-side twin.
- `crates/conductor-core/src/obs.rs` (212–226) — `now_rfc3339` and `unix_millis`.
- `crates/conductor-core/src/run_record.rs` (17–33) — the eleven-field envelope's actual field list.
- `crates/conductor-verify/src/extract.rs` (33–46) — `Observation`, incl. `degraded: bool`.
- `crates/conductor-emit/src/client.rs` (23) — `DEFAULT_OTLP_ENDPOINT`.
- `crates/conductor-run/tests/lifecycle_live.rs` (1–32) — the one feature-gated live driver, with its
  one-paste firing form and the two pacing facts in its own header.
- `crates/conductor-run/tests/severity_harvest.rs` (1–40, plus data-source probe) — the harvest-tier grading
  shape over frozen leg lines.
- `crates/conductor-run/Cargo.toml` (`[features]`) · `crates/conductor-tauri/ui/package.json` (scripts) ·
  `scripts/agent-run.sh` (73–155) · `crates/conductor-cli/src/cli.rs` (verb set) ·
  `crates/conductor-cli/src/paths.rs` (82–89).

## Graph impact (rust plane, `db_state: fresh`)
- **`preflight`** — 8 call sites: `conductor-cli/src/{main.rs, commands/mod.rs, commands/run.rs,
  commands/suite.rs}`, `conductor-tauri/src/commands.rs`, `conductor-run/src/{lib.rs (re-export),
  composition_root-test}`, `conductor-cli/tests/cross_surface_parity.rs`. Both bins reach the gate directly.
- **`persist`** — 12 sites across `conductor-run/src/{drive.rs, envelope.rs, lib.rs}`,
  `conductor-cli/src/commands/{mod.rs, run.rs, suite.rs}`, `conductor-run/tests/{composition_root.rs,
  envelope_fixture.rs}`.
- **`drive_run`** — 9 sites incl. `conductor-tauri/src/commands.rs` and `conductor-cli/tests/cross_surface_parity.rs`.
- **`observe_preconditions`** — 4 sites (`preconditions.rs` ×2, `lib.rs` re-export, `tests/composition_root.rs`).
- **`route_read_back`** — 7 sites, ALL inside `execute.rs` (1 production at `:88`, 6 in unit tests). Crate-private.
- **`manual_record`** — 4 sites, all `execute.rs`. **`emit_canary`** — 1 site, `canary.rs`.
- **GRAPH GAP (recorded, not a leaf):** `callee_name = 'execute_scenario'` returned **0 rows AND the script's
  own probe warned the name matches NO indexed symbol on the rust plane** — only the `execute` *module* is
  indexed. Grep settles it: defined at `crates/conductor-run/src/execute.rs:48` as
  `pub async fn execute_scenario<R: PauseResolver>`, with **3 production callers**
  (`conductor-cli/src/commands/run.rs:23`, `conductor-cli/src/commands/suite.rs:32`,
  `conductor-run/src/drive.rs:81`) plus 3 in-file tests. A generic fn the index misses; per the cookbook this
  is "not a leaf", and the caller set above comes from grep.

## Patterns detected
- **The capture/grade split** (`crates/conductor-run/tests/lifecycle_live.rs:30` header): the live driver
  "PRINTS its observation rather than asserting a pinned expectation: the capture is the deliverable, and the
  grading lives in `lifecycle_harvest.rs` over the frozen lines." **9 `*_harvest.rs` targets** implement the
  graded half, reading `{data_dir}/logs/agent-latest.jsonl.<date>` sliced to the leg window by a pre-leg line
  count (`severity_harvest.rs:30`). This is the existing evidence convention — W2 reuses it.
- **The empty-feature live gate** (`crates/conductor-run/Cargo.toml`): `live-pulse = []` declares no
  dependency, so it adds zero package nodes and keeps the leg off default `nextest`/`clippy`/release.
- **Two distinct blocked lines, not one** (`canary.rs:49`/`:79` vs `:59`): `"preflight blocked: MCP read-back
  path unreachable"` is the CONNECT-failure arm and returns early; `"preflight blocked: readiness gate not
  satisfied"` at `:59` is the gate arm CARRY-2 observable 3 names. A fourth, `:189`, covers canary-emission
  failure. Any assertion must key on the exact string, not the `preflight blocked` prefix.
- **`persist` writes `insert_envelope` unconditionally** (`envelope.rs:43`), before `RunReport::write` — so a
  run-level envelope row exists for every persisted run regardless of preflight outcome.

## Conventions to follow
- **Second-precision persisted instants.** `conductor_core::obs::now_rfc3339` (`obs.rs:212-219`) formats from
  `d.as_secs()` — the two envelope instants are truncated to whole seconds, while `latency_ms` is computed
  independently as `observed_ms - emitted_ms` from millisecond-grained `now_ms()`. Any persisted-record equality
  for W3 observable 2 is therefore a BOUNDED check (|latency_ms − 1000·Δsecs| < 1000), never an exact one.
- **The declare-only state mapping is the persisted proxy for `degraded`.** `manual_record`
  (`execute.rs:245-266`) sets `state: state_for(observation, ReportState::ManualCheck)`, and `state_for`
  (`:233-235`) returns `KnownResidual` iff `observation.degraded`. So on a declare-only scenario the envelope's
  `state` field discriminates exactly: `KnownResidual` ⟺ degraded, `ManualCheck` otherwise.
- **Live-leg recipe (assembled from the 20 harness-rule additions; every item load-bearing):** `PATH` prefix in
  POSIX form (`/d/…`, never `D:/…` — bash splits on the drive-letter colon, 2026-08-20) resolving
  `andromeda-pulse-mcp`, verified with `which` before the leg · `ANDROMEDA_PULSE_DATA_DIR` **equal to the live
  `pulse-app`'s dir** (data-dir EQUALITY; a per-leg dir of Conductor's own empties every read-back —
  2026-08-16 as corrected 2026-09-04) · `ANDROMEDA_PULSE_MCP_ENABLED=true` in Conductor's OWN env (inherited;
  `spawn.rs` passes only the data dir via `.env(...)`) · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` ·
  `CONDUCTOR_SCENARIOS_DIR` (repo-relative) where a trimmed catalog is needed · `RUST_LOG=info,conductor_emit=debug`
  in the PAIRED additive form and ONLY on a leg that does not also run the test suite. Whole block in ONE paste —
  a partial set fails SILENTLY as a ~0s `[BLOCKED]` (2026-08-22).
- **Never `boot` immediately before a leg that fires its own preflight** (2026-08-19 (a), extended 2026-09-01):
  `conductor run` and any GUI/wdio driven leg fire their own canary, which dedupes against boot's still-open
  incident. The cure is the QUIET WINDOW — ≥120s idle + a 30s resolver tick after ANY preceding canary.
- **`boot` writes NO run artifacts** (2026-08-16): no journal, no `runs.db` row, no `agent-latest.jsonl`
  refresh. Any criterion needing those requires a SCENARIO leg (`SCENARIO=<name|P-ID> … run`).
- **Verbs that exit 0 while proving nothing:** `status` prints the newest envelope on disk regardless of whether
  a run happened (MINT-THEN-READ is the fix), and `run --e2e` returns 0 both on a full pass and on a total
  `CONDUCTOR_MSEDGEDRIVER`-unset skip — read the SPEC LIST and reconcile passing/skipped tallies.
- **Process census before AND after, per subject** (2026-09-02), reported as a table with `terminated` /
  `left running: {why} + who stops it`; `pulse-app: left running — operator stops it`. A leg never kills what
  it did not start.

## New files to create
- `conductor-0.2.0/chunks/2026-09-06-operator-gated-live-suite/evidence/` — the chunk's evidence record
  (shape decided at P4; the harvest capture/grade split is the convention it must not fork).
- The invocation's own file(s) — path decided at P4 with the shape (see Open questions).

## Files to modify
Provisional pending the P4 shape decision; the caller sets below are enumerated from the graph result + grep.
- `scripts/agent-run.sh` + `scripts/agent-run.ps1` — **both, at identical semantics**, if W1 lands as a harness
  stage. The 5-command discipline forbids a 6th command without a test-plan amendment, so a live suite lands as
  a STAGE FLAG under an existing verb, not a new verb.
- `crates/conductor-run/Cargo.toml` — if a second feature gate is needed beside `live-pulse`.
- `crates/conductor-run/tests/` — the live driver + its harvest twin (the `lifecycle_live.rs` →
  `lifecycle_harvest.rs` pair is the in-repo precedent).
- `crates/conductor-tauri/ui/package.json` + `wdio.conf.ts` — only if the suite composes the wdio arms.
- Callers of any signature this chunk changes: `preflight` (8), `persist` (12), `drive_run` (9),
  `observe_preconditions` (4) — enumerated above. **A design that only READS persisted records changes no
  signature and threads no caller**, which is the cheaper shape.

## Open questions
- **Which invocation SHAPE?** → blocks: **plan-decision**. test-plan §9's sanctioned live-leg SET has exactly
  four members (`workflow_dispatch` · `scripts/agent-run.sh` · a cargo-feature-gated test file invoked directly ·
  an npm-script wdio suite). A new `conductor-cli` verb is NOT a member of that set as written, and the harness's
  5-command discipline blocks a 6th `agent-run` command — so the live-set-compatible options are a STAGE FLAG on
  an existing `agent-run` verb, or a `live-pulse`-gated test target, or both. P4 resolves.
- **Adopt or decline the per-run canary identity (CARRY-1)?** → blocks: **plan-decision**. Research adds one
  fact the entry did not have: reaching W3 observable 3's NEGATIVE arm (`readiness gate not satisfied` present)
  requires a CONNECTED client but an UNSATISFIED gate, and the cheapest way to produce that is exactly the
  back-to-back-canary dedupe the quiet window exists to avoid. Salting `scope_id` would remove that lever along
  with the hazard. P4 weighs both with the operator.
- **How many live legs does the evidence need?** → blocks: **implementation-scope**. Observable 3 needs BOTH
  arms (present on not-ready, absent on ready) to kill `canary.rs:58:8`'s inverted guard, and each leg needs its
  own quiet window; observables 1+2 ride one declare-only scenario leg. The leg count sets the wall-clock cost.

## Scope premise closure (performed 2026-09-06, before this file was written)
Every `[inferred]` bullet in `scope.md` is now VERIFIED (tag dropped) or `[premise-corrected: …]`. Two mechanism
claims were re-derived at HEAD per the marker rule; one W1 premise was falsified and corrected upward.
- **CARRY-1 dedupe tuple** (`measured at inference_runtime.rs:811`, HEAD `83d4060`) — evidence pointer
  spot-checked still-true at HEAD: the line is
  `.find(|inc| inc.kind == kind && inc.scope == scope && inc.scope_id == scope_id)`. **VERIFIED.**
- **CARRY-2 unreachability mechanisms** — all three re-derived: `DEFAULT_OTLP_ENDPOINT` is the fixed
  `"http://127.0.0.1:4317"` (`conductor-emit/src/client.rs:23`); `Preflight`'s fields are `pub(crate)`
  (`canary.rs:32-33`) with a doc comment stating they are never `pub`; the sidecar is spawned by fixed NAME
  through the inherited `PATH`. **VERIFIED.**
- **W1's "at least four firing forms"** — **[premise-corrected: SIX families / eight invocations —
  `agent-run boot`, `agent-run run` (+ the conditional `SCENARIO=` scenario leg), `agent-run run --e2e`
  (→ `npm run a11y`), `cargo test -p conductor-run --features live-pulse --test lifecycle_live`,
  `npm run a11y:driven`, and `npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error`]**.
- **W2's capture/grade precedent** — **VERIFIED**: 9 `*_harvest.rs` targets grade over frozen lines;
  `lifecycle_live.rs:30` states the print-not-assert split in its own words.
- **The three CARRY-2 coordinates** (re-aimed at P1) — **VERIFIED** against the post-split tree and
  cross-confirmed by test-plan §12's independently re-pointed roster.
