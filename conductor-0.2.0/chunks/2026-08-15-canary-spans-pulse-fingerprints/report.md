# Report — 2026-08-15-canary-spans-pulse-fingerprints

**Chunk:** Canary spans Pulse enumerates — the `rows_ingested` unit settled first, then empty-ids vs
message-shape discriminated at the buffer boundary, the Conductor-side fix landed and the live leg re-run;
`v2-10` claimable only on a leg reaching `ready:true` (conductor-run/emit)
**Date:** 2026-08-16
**Commits:** none yet — this wrap authors the chunk commit.

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-emit/src/message.rs` (+41/−6) · `crates/conductor-emit/tests/egress.rs`
  (+1/−1) · `crates/conductor-run/src/dispatch.rs` (+4/−2) · `crates/conductor-run/src/lib.rs` (+19/−4) ·
  `crates/conductor-run/tests/canary_wire.rs` (+56/−2) · `conductor-0.2.0/verification-matrix.json` (+1/−1,
  a `notes` line only) · `conductor-0.2.0/working-route.md` (freeze stamp) · `.andromeda/master-route.md`
  (promotion record) · new chunk folder (`scope.md`, `research.md`, `plan.md`, this report, `evidence/`).
  **5 source files; 0 new source files.**
- **Symbols / APIs:**
  - **CHANGED (public, breaking):** `conductor_emit::trace_request(service_name, span_name)` →
    `trace_request(service_name, seed: u64, span_name)` — a seed is now required, matching its four sibling
    builders (`exception_trace_request` / `error_trace_request` / `latency_trace_request` /
    `rate_trace_request`, all `(service, seed, …)`). Span identity is now `ChaCha8Rng::seed_from_u64(seed)` →
    `gen_id::<16>` / `gen_id::<8>`, replacing the constant `vec![1; 16]` / `vec![1; 8]`.
  - **CHANGED (private):** `conductor_emit::message::ok_span(seed, name)`.
  - **NEW (public):** `conductor_run::canary_storm_seed(base, i)` and
    `conductor_run::canary_warmup_seed(base, i)` — the canary's two seed derivations, single-sourced so the
    warm-up's descending range and the storm's ascending range are provably disjoint and testable.
  - Callers threaded: `dispatch.rs:82`, `dispatch.rs:95` (both pass the pre-existing per-occurrence
    `emission_seed(scenario.seed, phase_index, occurrence)` computed at `dispatch.rs:69`), `lib.rs:236`
    (warm-up), `tests/egress.rs:61`.
  - No new IPC method, endpoint, port, socket, or env var. Egress unchanged at `127.0.0.1:4317`.
- **Crates / modules:** none added, removed, or re-scoped.
- **Dependencies:** **none added, none bumped.** `rand_chacha` / `rand_core` were already
  `conductor-emit` dependencies (used by `span_tree.rs` / `exception.rs`). **`Cargo.lock` zero lines.**
- **Schema / config:** none. No scenario TOML, no contract manifest, no migration.
- **Spec-master edits:** none at authoring time — the fingerprint correction is proposed to P2 below.
- **Counts / qualifiers moved:** workspace test count **581 → 584** (+3 tests:
  `message.rs::same_seed_reproduces_identity_and_distinct_seeds_diverge`,
  `message.rs::identity_is_distinct_across_a_run_of_seeds`,
  `canary_wire.rs::the_whole_canary_emission_carries_unique_span_identity`). `CANARY_STORM_COUNT` unchanged
  at 12.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  1. **`architecture.md:60` §Read-Back Dependency Posture** — the parenthetical *"the fingerprint — computed
     to match Pulse's derivation — is the fidelity carrier, not a title echo"*. **MEASURED FALSE.** Conductor
     derives FNV-1a 64-bit → 16 hex chars over `exception_type` + frame **functions**
     (`conductor-emit/src/exception.rs:115-123`); Pulse derives **blake3 truncated to 16 bytes** over
     `exception_type` + `\0` + `normalize_stacktrace(stacktrace)`, read back as an **8-char hex prefix of
     the first 4 bytes** (`andromeda-pulse crates/buffer/src/fingerprint.rs:79-110`, HEAD `d090314`).
     Equality is impossible **by width alone**; algorithm and input differ on top of it. The canary
     round-trip's last precondition therefore fails **by construction** until the derivations are aligned —
     alignment is owned by the successor route entry, not this chunk. Evidence:
     `evidence/leg-verdict.md` §3, live leg `run_id 2026-08-16T08-17-48-786`.
  2. **`plan.md` step 6** predicted the seeded stream goldens would need re-locking. They did not: both
     `dispatch_wire__*` snapshots are driven by `scenarios/fingerprint-storm.toml` (Exception shape — all 36
     golden spans are `name: "root"` carrying an `exception` event), so **no committed golden drives the
     Plain-trace path** this chunk changed. Determinism is instead proven at the builder tier by the new
     both-directions seed test. (Instance-level coverage gap — recorded, not fixed here.)
- **Coverage of new surfaces:** no new external surface, hot-path op, or UI element. The one changed surface:
  - `conductor_emit::trace_request` (OTLP span-identity derivation) → validation `n/a` (no external input;
    the seed is a caller-supplied `u64`) · instrumentation `span/log✓` (rides the existing `emit.batch`
    wire-shape witness, unchanged) · PII `n/a` (identity bytes are seeded, carry no user content) · tests
    `unit✓ + integ✓` (2 unit in `message.rs`, 1 integration in `canary_wire.rs`, plus the live leg) · a11y
    `n/a` (headless wire path, no DOM) · tokens `n/a`.

## Deviations from intent

1. **`crates/conductor-emit/tests/egress.rs` was edited though absent from the plan's and research's
   Files-to-modify lists.** *Justification:* the P3 code-graph impact query returned the `conductor-run`
   callers but omitted this crate-local integration-test caller; rustc `E0061` surfaced it during the edit
   burst. Judged in-scope — a mechanical one-argument call-site update in the same crate as the changed
   builder, compile-forced by the enumeration mechanism the plan deliberately chose.
2. **Two public functions added that the plan did not specify** (`canary_storm_seed`,
   `canary_warmup_seed`). *Justification:* plan step 5 requires asserting identity uniqueness across
   warm-up **and** storm together, but `warm_up_canary_service` is private and its derivation was inline —
   unreachable as written. Single-sourcing the derivation lets the test assert the real production code
   rather than a drifting copy. Both live in `lib.rs`, already a listed touchpoint.
3. **Plan step 6's golden re-lock did not fire.** *Justification:* see Spec claims disproved #2 — nothing to
   re-lock, because no golden covers the changed path. The determinism acceptance criterion is still met via
   the both-directions seed test; the coverage gap is recorded as an instance note.
4. **The plan's prescribed console tee of `pulse-app` produced a 0-byte file.** *Justification:*
   `pulse-app` is a Windows GUI-subsystem binary and never attaches to a console. Its real sink is
   `{data_dir}/logs/agent-latest.jsonl.<date>`; the leg's evidence was harvested from there and sliced to the
   leg window by pre-leg line count. The empty `evidence/pulse-app-leg.log` is retained deliberately — the
   0 bytes *is* the finding.
5. **The "one cause" conclusion is held at INFERRED, not measured.** *Justification:* this leg measures the
   fix (27 appends, zero rejects, every counter alive). *Why* the prior leg's 12 distinct-id storm spans
   appended zero rows was never observed — that chunk captured no `duckdb.append` lines (its report contains
   zero), and its data dir no longer exists (searched 2026-08-16: no `agent-latest.jsonl.2026-08-15` in the
   default platform dir, the scratchpad trees, or the Pulse repo). The appender-poisoning reading is
   consistent with both measurements and is not proven by either.

## Decisions & corrections

- **Operator directive (phase):** settle the `rows_ingested` unit BEFORE weighing the two leads — it could
  demote both. It did resolve them, though not via its stated antecedent (a row is a parent-span row, not an
  export request), and both leads fell on independent evidence.
- **Operator directive (implement/wrap):** capturing the SUT's own log is mandatory, not optional, when the
  decisive evidence is SUT-side. Vindicated exactly — every load-bearing number in this chunk came from
  Pulse's telemetry, none from Conductor's.
- **Operator correction (wrap):** keep the honesty register on "one cause" — prove the fix, mark the prior
  leg's explanation inferred unless its data dir survives to be grepped. It did not; the wording holds.
- **Claim discipline reaffirmed:** `v2-10` stays pooled this chunk even though the leg advanced further than
  any before it. The decline is deterministic and final per-chunk; the successor claims it on a leg reaching
  `ready:true`, with this chunk's `evidence/leg-verdict.md` as the recorded basis.
- **Finding — Pulse's log sink on Windows:** `{data_dir}/logs/agent-latest.jsonl.<date>`, not the console.
  Belongs in the live-leg recipe.
- **Finding — `agent-run.sh status` exits 0 on a stale envelope** (returned a 2-day-old
  `run_id 2026-08-14T16-33-10-333` with no run this session). A bare `status` is not evidence of the current
  run; a freshness guard is an instance-level candidate for the script.
- **Source twins deliberately NOT edited here** (they ride the successor chunk with the fix, per the
  scheduler.rs-comment precedent): `lib.rs:218` (`emit_canary`'s doc restating the fingerprint match claim)
  and `warm_up_canary_service`'s stale bootstrap rationale.

## Outcome

**Acceptance criteria: met, except the two the live leg's `ready:true` would have carried** — which this
chunk never claimed (`v2-10` was declined at P5 by design).

- (arch) distinct 16/8-byte `(trace_id, span_id)` across the whole canary emission — **met**
  (`canary_wire.rs::the_whole_canary_emission_carries_unique_span_identity`; live witness
  `spans_missing_ids=0` on all 15 batches).
- (arch) change exposed by `conductor-emit`, no new occupied resource — **met**.
- (tests) `cargo nextest run --workspace --profile ci` **584/584, zero retries**; `cargo test --workspace
  --doc` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` clean; targeted
  `-p conductor-emit -p conductor-run -p conductor-verify` 188/188. **Green on iteration 1, no fixes.**
- (tests) determinism — **met** via the both-directions seed test; goldens pass unchanged in
  fail-don't-write mode (see Deviations #3 for why they did not move).
- (obs) live-leg evidence captured with the additive `RUST_LOG=info,conductor_emit=debug`, recording ingest
  `span_count`, buffer `rows_ingested`, the trio, and **every** `duckdb.append` line — **met**.
- (obs) no new span name outside the bounded set, no non-allowlisted attribute, no OTel SDK — **met**.
- (security) `cargo audit` **exit 1**, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` — the
  **20th pin**, origin preserved; `cargo deny check advisories bans licenses sources` **true exit 0**
  (`advisories ok, bans ok, licenses ok, sources ok`), verified not assumed; `Cargo.lock` zero lines — **met**.
- (security/design) a not-ready leg reports `[BLOCKED]` with a named, host-path-free precondition
  (`data_dir: "<redacted>"`), no `Result::Err`, no downgrade — **met**.
- (layouts) `conductor preflight` emits `ReadyState` and exits non-zero on `ready:false` — **met**.

**Live leg (operator-gated, `run_id 2026-08-16T08-17-48-786`, fresh data dir, deterministic L4):** the fix is
proven on Pulse's own telemetry — **27 `duckdb.append` lines, ZERO `reject_reason`** (3 warm-up spans + 12
storm spans + 12 span_events). The `buffer.tick` trio went **`0/0/0` → `12/12/12`** with `rows_ingested`
**1 → 15**; `storms_detected_total` **0 → 2**, including `severity_hint: "autonomous"` at
`occurrence_count: 10` — the canary cleared Pulse's Autonomous band for the first time — and an incident
formed (`item_id: 1`). Preflight nonetheless ends `ready:false` at the **last** precondition,
`"canary fingerprint not found in telemetry slice"`, for the by-construction reason recorded above.
Pulse's repo verified untouched (HEAD `d090314`, zero source files modified); port `4317` released.
