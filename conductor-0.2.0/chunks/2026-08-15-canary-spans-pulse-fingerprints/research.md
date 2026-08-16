# Codebase Research — 2026-08-15-canary-spans-pulse-fingerprints

## Scope
- **Depth:** deep · **Reads:** 15 · **Globs/Greps:** 10
- Pulse read **READ-ONLY** at parked HEAD `d090314` (confirmed via `git -C ../andromeda-pulse rev-parse`). No file under that repo was modified.

## Headline

**The tension is settled, LEAD A is falsified, LEAD B is substantially weakened, and a Conductor-side
defect no prior document has named was found: Pulse's `spans` table is `PRIMARY KEY (trace_id, span_id)`,
and Conductor's warm-up span builder emits a CONSTANT identity — `vec![1; 16]` / `vec![1; 8]` — on every
call.** Every warm-up span of every run has ever shared one primary key.

## Files inspected

**Conductor**
- `crates/conductor-run/src/lib.rs` (105-255) — the canary path: `CANARY_STORM_COUNT = 12`, `canary_spec`,
  `emit_canary_storm`, `emit_canary`, `warm_up_canary_service`, `canary_poll`. Two DIFFERENT builders are
  used: warm-up calls `trace_request(...)`, the storm calls `exception_trace_request(...)`.
- `crates/conductor-emit/src/message.rs` (full) — `trace_request` → `ok_span(name)` →
  `span(name, vec![1; 16], vec![1; 8], Vec::new(), ok_status())`. **The identity is a hard-coded constant,
  not seeded, not per-call.** `ok_span` carries NO events.
- `crates/conductor-emit/src/exception.rs` (100-180) — `exception_trace_request` seeds
  `ChaCha8Rng::seed_from_u64(seed)` then `gen_id::<16>` / `gen_id::<8>`: **proper 16/8-byte ids, distinct
  per occurrence** (seed = `base.wrapping_add(i)`), one root span carrying one `exception` event with
  `exception.type` / `.message` / `.stacktrace` string attributes.

**Pulse (read-only, HEAD `d090314`)**
- `crates/buffer/src/schema.rs` (30-39, 135-144) — **`CREATE TABLE spans (… PRIMARY KEY (trace_id, span_id))`.**
- `crates/buffer/src/appender.rs` (33-104) — `build_spans_record_batch`: same empty-id guard as the event
  builder; `if trace_ids.is_empty() { return Ok(None) }`.
- `crates/buffer/src/appender.rs` (322-415) — `build_span_events_record_batch`: the enumeration region and
  guard the directive cites, plus `record_feed_counts` **after** the `is_empty` early return.
- `crates/buffer/src/appender.rs` (477-497) — `append_record_batch_to_table` returns
  `record_batch.num_rows()`; a constraint failure surfaces as `Error::Append` from `append_record_batch`/`flush`.
- `crates/buffer/src/consumer.rs` (34-95) — `run_consumer`: on `Ok(Err(e))` it emits
  `tracing::error!(target: "duckdb.append", reject_reason = …)` and **continues the loop** (it does NOT terminate).
- `crates/buffer/src/consumer.rs` (95-200) — `dispatch_batch`: early-returns `Ok(())` when
  `build_spans_record_batch` yields `None`, **skipping `build_span_events_record_batch` and the observer**;
  `state.record_rows_appended(rows)` only on the success path; `append_table_traced` emits
  `tracing::info!(target: "duckdb.append", rows_appended, duration_ms, table_name)`.
- `crates/buffer/src/consumer.rs` (480-545) — the pinning tests: `rows_ingested` "reflects parent spans
  only, not events".
- `crates/buffer/src/state.rs` (40-95) — `record_rows_appended` / `record_feed_counts` / `snapshot`.
- `crates/ingest/src/grpc.rs` (118-152) — export ordering: `count_spans` → record on tracing span →
  `validate_resource_spans` (early `Err`) → `try_send` (early `Err`) → **`self.state.record_spans(span_count)`**.
- `crates/ingest/src/invariants.rs` (1-60) — `trace_id.len() != 16` / `span_id.len() != 8` ⇒
  `Error::InvariantViolation`, rejecting the WHOLE export.
- `pulse-app/src/main.rs` (640-652, 1145-1162) — `fingerprint_observer` is unconditionally
  `Some(StormObserverAdapter)`; `run_consumer` is spawned only when `buffer_conn` is `Some`.
- `crates/ingest/examples/inject_demo.rs` (105-193) — per-sequence `trace_id(seq)` / `span_id(seq)`
  (**unique ids**), identical `ResourceSpans → ScopeSpans{spans} → events` nesting, multi-span batches.

## Graph impact (code-graph query; `rows: 70`, `db_state: fresh`)
- **`exception_trace_request` / `trace_request` / `emit_canary_storm`** — callers at
  `crates/conductor-run/src/lib.rs:216` (`emit_canary`), `:235` (`warm_up_canary_service`), and four test
  sites in `crates/conductor-run/tests/canary_wire.rs:19, 72, 172, 216`. The emit-side blast radius is
  confined to `conductor-run` + `conductor-emit`; `trace_request`/`ok_span` additionally back other
  emit-crate callers, so a change to `ok_span`'s identity is NOT canary-local — see Files to modify.

## Patterns detected
- **Two builders, two identity disciplines** (`message.rs:151` vs `exception.rs:133-135`): the storm path
  seeds identity per occurrence; the warm-up path hard-codes it. Only the storm path was ever reviewed for
  identity, because only it needed *distinct* identities to form a storm.
- **Pulse fails soft and names the reason** (`consumer.rs:76-82`): every failed dispatch already emits
  `duckdb.append` with `reject_reason`, and every success emits `rows_appended` + `table_name`. This
  instrument exists and has never been captured on a canary leg.
- **Counters are written, not defaulted** (`appender.rs:392-415`): `record_feed_counts` sits *after*
  `if trace_ids.is_empty() { return Ok(None) }`.

## Conventions to follow
- **Emit-seam ownership** — OTLP raw-type construction lives in `conductor-emit` (`message.rs` /
  `exception.rs`); `conductor-run` composes, never constructs (arch extract, Constraints #1).
- **Seeded identity, wall-clock stamps** — identity bytes come from the seed, timestamps from `std::time`
  (`message.rs:76-89` doc comment; testing rule 2026-06-17).
- **`duckdb.append` capture needs the additive form** `RUST_LOG=info,...` — a bare per-target directive
  replaces the default (obs extract, Constraints #7).

## New files to create
- (none)

## Files to modify — **provisional** (see Open questions)
- `crates/conductor-emit/src/message.rs` — `ok_span`'s constant `vec![1; 16]` / `vec![1; 8]` identity. The
  graph shows `trace_request`/`ok_span` has callers beyond the canary, so the fix must give each call a
  distinct identity **without** breaking the determinism contract other callers rely on (a seeded parameter
  is the shape consistent with `exception_trace_request`; an unseeded random id would introduce a second
  non-determinism source, barred by arch extract Constraints #6).
- `crates/conductor-run/src/lib.rs` — `warm_up_canary_service` must pass a per-emission seed once
  `trace_request` takes one.
- `crates/conductor-run/tests/canary_wire.rs` — the wire assertion must additionally assert **identity
  distinctness across the emitted set** (the property whose absence this chunk found); today it asserts
  events-intact against Conductor's own collector, which is blind to a PK collision at a receiver.
- Golden re-lock under `crates/conductor-*/tests/snapshots/` if the emitted wire shape moves (tests extract,
  Acceptance #2).

## Scope premise closure

- **TENSION — `rows_ingested` unit → `[premise-corrected]`.** A row is a **parent-span row appended to the
  `spans` table** (`append_record_batch_to_table` returns `record_batch.num_rows()`; pinned by
  `consumer.rs:542-543`), **not** an export request. The directive's conditional ("if row == export request,
  the gap moves upstream and both leads are demoted") therefore does not fire on its stated antecedent —
  yet **both leads are demoted anyway, for independent reasons below.**
- **LEAD A — empty ids → `[premise-corrected: FALSIFIED]`.** Both canary builders emit non-empty,
  correct-length ids (`exception.rs:134-135` seeded 16/8; `message.rs:151` constant-but-16/8). Independently
  decisive: Pulse's `validate_resource_spans` rejects any wrong-length id with `Error::InvariantViolation`
  for the **whole export**, and `record_spans` runs only *after* that check and *after* `try_send`
  (`grpc.rs:144-150`) — so the measured `span_count: 15` is itself proof that all 15 spans passed the
  id-length invariants and entered the buffer channel.
- **LEAD B — composition / nesting → `[premise-corrected: substantially weakened]`.** `inject_demo` uses the
  **identical** `ResourceSpans → ScopeSpans{spans} → span.events` nesting with `exception.*` event
  attributes. The surviving producer differences are (a) **span identity uniqueness** — the real one — and
  (b) batching cardinality (`inject_demo` multi-span batches vs the canary's 12 single-span unary exports).
- **NEW, unnamed by any prior document** — Pulse's `spans` table is `PRIMARY KEY (trace_id, span_id)`
  (`schema.rs:38`), and every warm-up span shares `([1;16], [1;8])`. The first append succeeds; each
  subsequent one violates the PK, returns `Error::Append`, and is logged-and-skipped by `run_consumer`.
  This alone explains `rows_ingested: 1` for the warm-up and is **exactly the producer-dependence observed**
  (`inject_demo` never collides — its ids are per-sequence).
- **Trio semantics refined** — the directive's reading of what a *written* trio value means is confirmed
  (`appender.rs:392-404`). But `record_feed_counts` sits after the `is_empty` early return, so a **`0` is
  the never-written state** and cannot by itself discriminate: (a) the enumeration collected zero,
  (b) `build_span_events_record_batch` was never called because `build_spans_record_batch` returned `None`,
  or (c) it ran but no span carried events. **`ok_span` carries no events**, so the one warm-up span that
  *did* append takes path (c) — consistent with `span_events_seen: 0` without any storm span being implicated.

## Open questions
- The PK collision accounts for the warm-up, but the storm's 12 spans carry distinct seeded ids and should
  append (predicting `rows_ingested ≈ 13`, not the measured `1`). Whether the storm batches also failed —
  and why — is unresolved from source alone. → **blocks: implementation-scope** (the Files-to-modify list is
  provisional until a leg capturing `duckdb.append` names the cause; `reject_reason` reports it directly,
  and no prior capture included that target).
- Whether `trace_request`'s non-canary callers can absorb a seed parameter without disturbing a committed
  golden. → **blocks: implementation-scope** (graph shows callers beyond the canary; the golden re-lock is
  already an accepted acceptance criterion either way).
