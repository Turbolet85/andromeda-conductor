# Codebase Research — 2026-10-01-per-run-span-identity-in-the-real-model-harness

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 24 (+ 2 code-graph queries on the rust plane)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (79 537 B, past the read cap): structural
  extraction. I indexed headers and introducers (`grep -n "^#\|^- "`, 71 lines) and read lines 17-19, 35-36 and
  48-53 in full by line number. Those carry the `run --live` / `--live real-model` firing forms, the
  determinism rule, fresh-dir-per-leg / data-dir equality, the quiet window and boot-before-run. I did not read the
  remaining Session Additions in full. They cover the SR/NVDA legs, GUI legs and capture-reading rules, and no leg
  in this plan uses them. If P4 adds a live witness, the plan cites lines 50 and 53 for its recipe.
- **Platform issues consulted:** none. No runner-only bullet is in scope, and the one CI verdict is in progress, not red.

## Files inspected
- `crates/conductor-emit/src/exception.rs` (150-200) — `exception_trace_request(service, seed, spec)` seeds
  `ChaCha8Rng::seed_from_u64(seed)` and draws `trace_id` (16 B) then `span_id` (8 B) via `gen_id` (:185-187). Span
  identity is a pure function of `seed`. `fingerprint()` (:163-174) hashes `exception_type` + `\0` + the normalized
  stacktrace only. No identity enters its preimage.
- `crates/conductor-run/src/dispatch.rs` (1-175, 252-326) — `Dispatcher::dispatch` derives
  `seed = emission_seed(self.scenario.seed, index, point.occurrence)` (:78) and hands that ONE seed to every span
  builder: `trace_request` · `error_trace_request` · `exception_trace_request` · `latency_trace_request` ·
  `PiiCorpus::seeded` · `rate_trace_request` · `service_topology_request`. `emission_seed` (:252-256) is a pure mix of
  scenario seed, phase ordinal and occurrence, so identity is unique per slot within a drive and identical across
  drives of the same scenario + seed. That is the d2 replay.
- `crates/conductor-run/src/execute.rs` (20-130, 312-317, 805-830) — `execute_scenario` is the one production entry.
  The CLI `run`/`suite` and the Tauri `drive_run` all funnel through it. It computes `emitted_ms = now_ms()` (:85,
  `std::time`), then `Dispatcher::connect(scenario, DEFAULT_OTLP_ENDPOINT)` (:88). It already holds a wall-clock
  value per execution, read before the dispatcher exists.
- `crates/conductor-run/src/canary.rs` (120-160, 195-230, 280-300) — the canary's identity is per run: storm/warm-up
  seeds derive from `base = now_ms() as u64` (:294), and the marker is `ConductorCanary_{now_ms()}` (:285). This is why
  d2's canary landed and the scenario did not.
- `crates/conductor-run/tests/lifecycle_live.rs` (37, 78-80) — a second in-tree precedent: the live lifecycle test seeds
  its storm from `canary_storm_seed(now_nanos() as u64, tick)`.
- `crates/conductor-run/tests/dispatch_wire.rs` (1-143, 437-513) — loopback-stub capture through the real
  `run_timeline_with` + `Dispatcher` path (`drive_scenario!`, :124-142). `the_same_seed_reproduces_the_same_stream`
  (:437-472) ASSERTS identical `(trace_id, span_id)` across two same-seed drives. `SpanShape` (:66-90) carries
  `trace_id`/`span_id`/`parent_span_id` hex, so the three `dispatch_wire__*` goldens pin identity bytes.
- `crates/conductor-run/tests/pii_harvest.rs` (20, 104-119) — the live PII harvest RE-COMPUTES each slot's corpus from
  a transcription of `emission_seed` (`leg_emission_seed`). The PII content is a function of the same per-slot seed
  that identity is.
- `crates/conductor-run/tests/canary_wire.rs` (10-14, 105-199) — the in-repo statement of Pulse's span-store key and
  the distinct-identity assertions the canary already carries.
- `crates/conductor-emit/src/client.rs` (48-110) — `TraceEmitter::export` carries the `emit.batch` span and the
  wire-shape witness (`spans_missing_ids`). By construction it counts MISSING ids, never repeated ones.
- `crates/conductor-emit/src/span_tree.rs` (35-90) — `gen_id` (:86-90) is `pub(crate)`. `error_trace_request` links
  children by `parent_span_id` within one trace.
- `crates/conductor-emit/src/lib.rs` (12-36) — module list and re-exports.
- `scripts/agent-run.sh` (82, 157-209) — both live legs run `conductor run <scenario> --agent-mode`, so they go through
  `execute_scenario`. `--live real-model` is one leg per invocation.
- `contracts/pulse-real-model-leg-posture.md` (272, 362, 461) — the quiet window between drives is ≥ 150 s after the
  last incident. Below Pulse's retention, so it cannot separate same-seed drives on its own.
- **Pulse (SUT, read-only) at `a2addb3755b3029cb79809b96efdd2522749b179`** (`git rev-parse HEAD` in the Pulse tree;
  its worktree carries only `.andromeda`/handoff bookkeeping modifications, no source edits):
  - `crates/buffer/src/schema.rs:36` — `spans` `PRIMARY KEY (trace_id, span_id)`. `:52` — `span_events`
    `PRIMARY KEY (trace_id, span_id, event_index)`. `:66` / `:82` — metrics and logs key on a timestamp plus `seq`, so
    logs and metrics cannot replay this way.
  - `crates/buffer/src/consumer.rs:60-91` — a dispatch error → `state.record_append_rejection()` + `tracing::error!`
    target `duckdb.append`, `reject_reason = describe_error(&e)`. `:272` maps `Error::Append` → `"append_failed"`.
  - `crates/buffer/src/appender.rs:990-1030` — Pulse's own test
    `append_after_a_constraint_violating_flush_returns_instead_of_hanging`: "a producer restart replays span identities
    already stored, so the composite PK rejects the batch at `flush()`". It asserts "a replayed identity must be
    rejected". The WHOLE batch is rejected, not just the duplicate row.
  - `crates/buffer/src/retention.rs:36-51` + `contract.rs:109` — retention window 600 s by default, sweep cadence
    `retention_seconds.max(60) / 6` = 100 s. A row is evicted 600-700 s after its timestamp. That matches d1's storm
    (19:39:37Z) evicted 19:48-19:51Z.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-01T21-06-20-phase/tree-query-2026-10-01-per-run-span-identity-in-the-real-model-harness.json`)
- **exception_trace_request** — production callers `Dispatcher::dispatch` @ `crates/conductor-run/src/dispatch.rs:124`
  and `emit_canary_storm` @ `crates/conductor-run/src/canary.rs:223`. Test callers: `exception.rs` tests,
  `tests/exception_events.rs:27` and `tests/lifecycle_live.rs` (graph 0-indexed lines +1). This chunk changes neither
  the signature nor the body.
- **gen_id** — 17 call sites over 7 emit modules (exception, latency, message/`ok_span`, pii, rate, span_tree,
  topology; trace rows). Every span builder draws identity from its seed this way, so the replay class covers every
  dispatcher shape, not only exceptions.
- **Dispatcher::connect** (`callee_name = 'connect'`, `callee_file LIKE '%conductor-run/src/dispatch.rs'`) — 1 graph
  row, `execute.rs:88`. `grep -rn "Dispatcher::connect" crates/` adds `tests/dispatch_wire.rs:130` (the
  `drive_scenario!` macro; tests are not indexed). That makes 2 call sites, re-derived by grep.
- **emission_seed** — 1 production caller (`dispatch` @ `dispatch.rs:78`) + its unit test (`:313-316`). Graph rows: 6.
- **execute_scenario** — 0 graph rows under the `callee_file` filter. This is an async-fn index gap, per the cookbook's
  measured class. `grep -rn "execute_scenario(" crates/` finds production callers `conductor-cli/src/commands/run.rs:23`,
  `conductor-cli/src/commands/suite.rs:32` and `conductor-run/src/drive.rs:80`, plus 4 test calls in `execute.rs`.
  The basis is grep. The chunk does not change its signature.

## Patterns detected
- **Wall-clock per-run identity base** (`canary.rs:285,294`; `lifecycle_live.rs:80`): every Conductor emitter that
  already survives back-to-back runs takes a `std::time` value as its identity base. This is the in-tree precedent for
  shape (a).
- **One seed feeds content AND identity** (`dispatch.rs:78` → every builder; `pii_harvest.rs:104-119`): the per-slot
  seed draws both the identity bytes and the content (PII corpus, latency durations, rate counts). A live harvest
  re-derives content from it. So a salt applied to the SEED would move content and break the PII harvest. Only the
  identity may change.
- **Dispatcher-tier identity is pinned** (`dispatch_wire.rs:437-472` + three `dispatch_wire__*` goldens): a dispatcher
  driven without a run-scoped input must keep its identity bytes, and the goldens stay byte-unchanged.
- **Distinct-identity wire assertion** (`canary_wire.rs:142-199`): collects `(trace_id, span_id)` pairs across the
  captured stream and asserts no repeat. This is the assertion shape the new two-drive test copies.

## Conventions to follow
- **Identity RNG**: `ChaCha8Rng` + `seed_from_u64` only (`span_tree.rs:86-90`; arch [Determinism RNG]).
- **Seam**: `conductor-emit` takes primitives only (no `conductor-core` edge). A run-scoped value enters as a `u64`.
- **Clock**: per-run values come from `std::time` (`execute.rs:312-317` `now_ms`), never `tokio::time`.
- **Loopback capture**: `tests/common::start_stub` on `127.0.0.1:0` (`dispatch_wire.rs:9-11,124-142`).
- **Goldens**: never `cargo insta review`. This chunk's goldens must stay byte-unchanged.

## New files to create
- `crates/conductor-emit/src/identity.rs` — the identity re-key primitive. It is a deterministic bijection over a
  request's `trace_id` / `span_id` / `parent_span_id`, derived from a `u64` salt on `ChaCha8Rng`, and preserves
  parent/child linkage and every content byte. It ships with unit tests.
- `crates/conductor-run/tests/span_landing_live.rs` — the operator-gated live witness, added at P4 once the operator
  chose it. It is `#![cfg(feature = "live-pulse")]` and auto-discovered like `real_model_live.rs`, so it needs no
  Cargo.toml entry. It reads two frozen self-obs journals through `capture_paths::runs_dir_from` and Pulse's log
  through `capture_paths::pulse_logs_dir_from`, and prints a summary of counts only.
- `conductor-0.3.0/chunks/2026-10-01-per-run-span-identity-in-the-real-model-harness/evidence/revert-red.md` — the
  two-drive test read RED with the re-key call removed (implement's red-before-green witness)
- `conductor-0.3.0/chunks/2026-10-01-per-run-span-identity-in-the-real-model-harness/evidence/witness-ledger.md` — the
  operator pass: census, slot, sha256s, launch posture, each entry's exit and atoms, run_ids, witness summary,
  teardown

## Files to modify
- `crates/conductor-emit/src/lib.rs` — declare the module and re-export the primitive.
- `crates/conductor-run/src/dispatch.rs` — the dispatcher carries an optional identity salt and re-keys every trace
  request before export. With no salt, identity stays seed-pure. The logs path is untouched.
- `crates/conductor-run/src/execute.rs` — `execute_scenario` gives the dispatcher a per-execution salt taken from the
  `std::time` value it already reads (`emitted_ms`).
- `crates/conductor-run/tests/dispatch_wire.rs` — the new two-same-seed-drives test through the real dispatch path.
  Existing tests and the `drive_scenario!` macro keep their unsalted form.

## Open questions
- Does the chunk also carry an operator-attended LIVE witness? That would be two `conductor run` drives of one
  deterministic exception scenario inside one Pulse retention window, graded on Pulse's own `duckdb.append` /
  `buffer.tick` lines. Or does it stop at the loopback proof plus the Pulse-source reading above? → blocks:
  plan-decision. The live arm needs the operator's slot and `:4317` grant, and a built `pulse-app` at `a2addb3`.
- How does the salt reach the dispatcher: a required `connect` parameter (both call sites change), or a builder
  method (production opts in, test call sites unchanged)? → blocks: plan-decision.
- Is per-run span identity a playbook `Boundary widening` (`.andromeda/playbook.md:124`), for the founder's live word?
  → blocks: plan-decision (the operator directive asks it be raised as one if the playbook names it).

Resolved at P4, 2026-10-01: (1) the operator chose the live witness — "measured over inferred", run on one
deterministic pulse-app using the `a2addb3` binaries if they still stand (sha256 verified first), in the operator's
slot with the `:4317` grant. (2) It is a required `connect` parameter, the decisive lean recorded in plan.md. (3) It is
not a widening, accepted on the clause-by-clause reading.
