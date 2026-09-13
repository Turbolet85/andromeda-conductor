# Codebase Research — 2026-09-13-audit-debt-retired-before-epoch-1-closes

## Scope
- **Depth:** deep · **Reads:** 20 · **Globs/Greps:** 15
- **Harness rules consulted:** `.claude/rules/testing.md` — read by structural extraction (the file is 61 KB
  with multi-KB single-line entries; `grep -n` indexed the mutation entries, then offset-bounded reads of
  `:19` `:71` `:72` `:79`), plus `.claude/rules/security.md` and `.claude/rules/host-win32.md`
  (unconditional-load). `verification-harness.md` NOT consulted — **no live leg in this chunk**: the
  `cargo mutants` gates are local cargo invocations, not the 5-command harness surface.

## Files inspected
- `crates/conductor-verify/src/manifest.rs` (full) — `default_path()` at `:34-36` returns
  `PathBuf::from("contracts/mcp-contract.toml")`; `pub`, no gates above it.
- `crates/conductor-verify/src/extract.rs` (`:85-125`, `:222-244`) — the two survivors are the **one-shot
  guards** `if i == 0 {` at `:98` and `:113`, each gating a `log_observed_keys(...)` call.
- `crates/conductor-verify/src/client.rs` (`:178-190`) — `resolve_incident` is a 3-line wrapper delegating
  to `mark_incident_resolved(Some(json!({"incident_id": …})))`.
- `crates/conductor-verify/tests/readback.rs` (`:1-60`, `:200-280`) — `observe_with(StubConfig)` and
  `resolve_with(StubConfig, id)` helpers over an in-process duplex stub.
- `crates/conductor-verify/tests/readback_shape_witness.rs` (header + `:81-117`) — its OWN test binary,
  because `init_observability` is process-global first-install-wins; writes self-obs to a temp
  `obs.jsonl` and filters lines by a stamped `run_id`.
- `crates/conductor-verify/tests/common/mod.rs` (`:67-102`, `:117`, `:201`, `:276`) — `StubConfig` fields
  incl. `wire_log`; the stub serves `mark_incident_resolved` and answers `{resolved, incident_id}`.
- `crates/conductor-cli/src/commands/cleanup.rs` (full, 24 lines) — opens `RunsDb`, `delete_run`, prints
  `cleanup: {n} rows removed for {run_id}`, returns `Ok(ExitCode::SUCCESS)`.
- `crates/conductor-cli/tests/cli_smoke.rs` (structure) — `conductor(dir)` assert_cmd helper, 17 tests.
- `crates/conductor-run/tests/journal_conformance.rs` (`:96`, `:275-281`) — the conformance gate.
- `crates/conductor-report/src/journal.rs` (`:150-185`) and `crates/conductor-verify/src/record.rs`
  (`:130-165`) — the two eleven-key clone sites.
- `crates/conductor-core/src/run_record.rs` (`:17`) — `RunRecord`, the schema the const describes.
- `crates/conductor-emit/tests/egress.rs` (`:1-40`) — the clone family's shared shape.
- `conductor-0.2.0/chunks/2026-09-03-conductor-run-composition-root-survivors-dispositioned/evidence/disposition-ledger.md`
  (`:56`, `:61`) — class B's mint-time coordinates.
- Manifests: `conductor-emit/Cargo.toml`, `conductor-verify/Cargo.toml`, `conductor-cli/Cargo.toml`;
  `.github/workflows/ci.yml` (`:105-114`).

## Graph impact (code-graph, plane `rust`; lines below are EDITOR lines = graph `line` + 1)
- **`crate_edges`** — `conductor-emit → conductor-core` has **NO row**. Seven crates carry the edge
  (`cli`, `faults`, `report`, `run`, `tauri`, `timeline`, `verify`); `conductor-emit` is the only non-core
  member without it, and has **no outbound workspace edge at all**. This corroborates `cargo machete`'s
  `unused_deps` from a second, usage-based source.
- **`conductor-report` and `conductor-verify` BOTH already carry `→ conductor-core`** — so a shared const
  in `conductor-core` adds **no new cross-seam edge** (arch's stated requirement).
- **`cleanup`** — 2 call sites: `commands/mod.rs:11` (re-export) and `main.rs:89` (`dispatch()`). A cli
  subprocess test enters it.
- **`observe`** — callers at `conductor-run/src/execute.rs:98` and `conductor-verify/tests/readback.rs:11`,
  `:22`. An in-crate vehicle exists.
- **`resolve_incident`** — exactly ONE caller workspace-wide: `conductor-run/src/lifecycle.rs:122`. **Zero
  callers inside `conductor-verify`**, which is why `cargo mutants -p conductor-verify` (which runs only
  that package's tests) cannot reach it today.
- **`ContractManifest::default_path`** — `refs` scoped to `%conductor-verify%` returns **0 rows**. The
  name exists on the plane (three other `default_path` definitions in `conductor-core` returned rows), so
  this is a genuine consulted-but-no-match; **cross-checked by grep**: `grep -rn 'default_path'
  crates/conductor-verify/` returns the definition at `manifest.rs:34` and nothing else. Zero call sites
  workspace-wide.

## Patterns detected
- **One-shot guard, three-observation kill** (`extract.rs:98`, `:113`): `.claude/rules/testing.md:72(b)`
  already prescribes this exact case — at two incidents both correct (`i==0`) and mutant (`i!=0`) emit
  exactly one line; at three, correct = 1 and mutant = 2, so `assert_eq!(count, 1)` discriminates. The
  same entry warns to scope the count by the call site's own message text: `log_observed_keys` has
  **exactly four** call sites (`extract.rs:84`, `:99`, `:114`, `preflight.rs:373`), so an unscoped tally
  is contaminated. The two in `observe` are separable by their `tool` argument (`retrieve_report` vs
  `retrieve_telemetry_slice`).
- **The witness is observable ONLY through the self-obs artifact** — `log_observed_keys`
  (`extract.rs:229-234`) emits a `tracing::info!` and returns nothing; the guard has no other effect.
  `readback_shape_witness.rs` is the shipped vehicle for exactly this, and its own header records why it
  is a separate binary (process-global first-install-wins subscriber).
- **The existing resolve tests bypass the mutated wrapper** (`readback.rs:206-219`): `resolve_with` calls
  `client.mark_incident_resolved(...)` DIRECTLY, so `an_applied_resolve_round_trips_pulses_raw_shape` and
  `a_declined_resolve_is_a_typed_json_rpc_value_never_a_panic` never enter `resolve_incident`. This is
  testing.md:72's "does the named vehicle actually ENTER the mutated function (read its body, never its
  name)" — the helper's NAME says resolve; its body does not call the wrapper.
- **`cleanup`'s mutant preserves the exit code** — `ExitCode::default()` is `SUCCESS`, the same value the
  real body returns, while skipping BOTH the `delete_run` and the `println!`. The observable is the
  printed line or the row count, never the exit status (arch's reading, confirmed by the body).
- **Clone family shape** (`conductor-emit/tests/egress.rs:22-40` et al.): a `CapturingService`
  (`tonic::TraceService` capturing the last `ExportTraceServiceRequest` behind `Arc<Mutex<Option<…>>>`)
  plus `start_stub()` (bind `127.0.0.1:0`, serve on an ephemeral port). That pair is what the five clone
  pairs share across six files.

## Conventions to follow
- **Conformance asserts PRESENCE, not exclusivity** — `journal_conformance.rs:96`: "key presence — extra
  keys are allowed (obs-plan §3 keeps the extension point open)". The two clone sites are separate
  `#[cfg(test)]` **exclusivity** assertions over `RunRecord`'s own serialization, which is legitimate
  there; the shared const must not be wired into `journal_conformance` as an exclusivity check.
- **Coverage already excludes any `tests` dir** — `ci.yml:110-114` uses
  `--ignore-filename-regex '[\\/]tests[\\/]'`, a generic path regex. A new `conductor-emit/tests/common/`
  needs **no exclusion-list edit**.
- **The pinned standalone form** (`cargo check -p conductor-emit --lib`, `--all-targets` banned) —
  measured at HEAD as the baseline: **exit 0** (captured bare, not through a pipe).
- **Mutation output goes under gitignored `target/`** (`.claude/rules/testing.md` 2026-09-03) — the repo
  root is NOT gitignored for an `--output` dir; only `mutants.out/` and `mutants.out.old/` are, by name.
  Tallies land NESTED at `{output}/mutants.out/`, read only when `outcomes.json`'s `end_time` is set AND
  `total_mutants == len(mutants.json)` (collectors.md C1).

## New files to create
- `crates/conductor-emit/tests/common/mod.rs` — the shared `CapturingService` + `start_stub()` helpers,
  modelled on `crates/conductor-run/tests/common/mod.rs`.

## Files to modify
- `crates/conductor-verify/src/manifest.rs` — add the `#[cfg(test)]` assertion pinning `default_path()`.
- `crates/conductor-verify/tests/readback.rs` — a test entering `resolve_incident` (the wrapper), not
  `mark_incident_resolved`.
- `crates/conductor-verify/tests/readback_shape_witness.rs` — drive ≥3 incidents; per-tool witness-line
  counts.
- `crates/conductor-cli/tests/cli_smoke.rs` — a `cleanup` test asserting the printed line / row effect
  (the file has **no** `cleanup` coverage today: `grep -n 'cleanup' crates/conductor-cli/tests/*.rs`
  returns nothing).
- `crates/conductor-emit/tests/{egress,error_spans,exception_events,traffic_rate_ramps,multi_service_topology,latency_shaping}.rs`
  — six files (the five clone pairs' union) switch to `mod common;`.
- `crates/conductor-emit/Cargo.toml` — drop `conductor-core.workspace = true` (`:9`).
- `crates/conductor-emit/src/error.rs` (`:5`) — reword the intra-doc link `[`conductor_core::CoreError`]`,
  which is the ONLY resolvable reference and breaks when the dep goes. `latency.rs:8` and `topology.rs:21`
  name the crate in prose backticks and resolve to nothing — they need no edit.
- `crates/conductor-core/src/run_record.rs` — the single-sourced eleven-key const, beside `RunRecord`.
- `crates/conductor-report/src/journal.rs` + `crates/conductor-verify/src/record.rs` — consume the const.
- **Companion sweep** (name grep over the whole rust tree, not a `tests/` subtree): `conductor_core` under
  `crates/conductor-emit/` → 3 hits, all dispositioned above (1 edit, 2 no-change). `default_path` under
  `crates/conductor-verify/` → 1 hit (the definition). `cleanup` in `crates/conductor-cli/tests/` → 0 hits.

## Open questions
- **Does dropping `conductor-core` from `conductor-emit/Cargo.toml` break anything beyond `error.rs:5`?**
  → blocks: implementation-scope. Evidence gathered: no usage edge in the graph, no `conductor_core` token
  in `conductor-emit/tests/`, three doc-comment mentions in `src/`, and the standalone baseline is green
  WITH the dep. No build has been run WITHOUT it — P3 is read-only — so the removability claim stays
  `hypothesis:` and /implement closes it by a `cargo check -p conductor-emit --lib` + `cargo nextest`
  after the edit.
- **Does the `resolve_incident` kill need `-p conductor-verify` reachability only, or does the
  `conductor-run` caller matter?** → blocks: plan-decision. The mutation gate is per-unit, so the kill
  must live in `conductor-verify`; the existing `conductor-run` caller is irrelevant to the tally.
  Resolved in-plan: the new test goes in `conductor-verify/tests/readback.rs`.

## Note for wrap (spec↔reality, not acted on here)
`test-plan.md` §10 describes the coverage exclusion list as naming `conductor-verify/tests/common` **by
path**; the shipped `ci.yml:110-114` uses the generic `--ignore-filename-regex '[\\/]tests[\\/]'`. The
effect is the same or broader, so nothing is blocked — but the wording and the invocation differ.
