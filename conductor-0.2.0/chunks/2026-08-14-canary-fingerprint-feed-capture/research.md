# Codebase Research — 2026-08-14-canary-fingerprint-feed-capture

## Scope
- **Depth:** deep · **Reads:** 9 (5 Conductor, 4 SUT) · **Globs/Greps:** 8 · **Graph queries:** 1 (53 rows)

The chunk's discriminator is a claim about the SUT's reaction, so research covered BOTH sides: Conductor's
canary emission path, and — since `D:\dev\projects\andromeda-pulse` is on disk — Pulse's actual receive path.
Reading the SUT source is hypothesis-narrowing, not settlement (a term about the SUT's behavior is only ever
tested by a live leg); it is recorded here so the capture is designed against the real mechanism.

## Files inspected

### Conductor
- `crates/conductor-emit/src/exception.rs` (100–220) — `fingerprint()` (FNV-1a over `exception_type` + frame
  functions) and `exception_trace_request()`: builds ONE root span per call, `trace_id`/`span_id` seeded from
  `seed`, `Status.Code=ERROR`, and exactly one `Event { name: "exception" }` whose **event attributes** carry
  `exception.type` / `exception.message` / `exception.stacktrace` (`:153-164`).
- `crates/conductor-emit/src/client.rs` (full) — `TraceEmitter::export` is the `emit.batch` span
  (`:67`), `fields(emission_count = count_spans(&request))`. **`count_spans` counts SPANS, not events**
  (`:74-81`) — the shipped span attribute cannot distinguish "span with an exception event" from "span
  without one". This is the instrumented boundary the obs extract points at.
- `crates/conductor-run/src/lib.rs` (100–250) — the canary path: `emit_canary` (`:175-194`) builds one
  `ExceptionSpec` with marker `ConductorCanary_{now_ms}`, computes `fp`, connects a `TraceEmitter`, optionally
  runs `warm_up_canary_service`, then exports `CANARY_STORM_COUNT = 6` (`:111`) requests each seeded
  `base.wrapping_add(i)` — **distinct span identity, identical fingerprint**. `warm_up_canary_service`
  (`:200-215`) exports `warmup_emissions` benign `trace_request(…, "canary-warmup")` spans (no exception
  event) spaced `warmup_ms / warmup_emissions`. `canary_poll` (`:222-230`) clamps up to the contract floor.
- `crates/conductor-core/src/redact.rs` (21–57) — `ALLOWLISTED_FIELDS` is a closed 27-name list. It carries
  `emission_count` / `record_count` / `count` / `message` but **no event-, exception- or fingerprint-shaped
  name**. A new span attribute named for this capture would be dropped silently at the processor stage.
- `contracts/pulse-run-contract.toml` (`[incident_formation]`) — `warmup_ms = 45000`, `warmup_emissions = 3`,
  `min_canary_poll_seconds = 90`. **3 warm-up + 6 storm = the 9 spans Pulse logged** — the span count
  reconciles exactly, confirming all nine requests reached Pulse.

### SUT (`D:\dev\projects\andromeda-pulse`, read-only)
- `crates/buffer/src/appender.rs` (320–400) — `build_span_events_record_batch` iterates
  `rs.scope_spans → ss.spans → span.events` and reads the three keys from **`event.attributes`**
  (`:345-350`). **The direction and arity the prior chunk transcribed are CORRECT** — Conductor writes event
  attributes and Pulse reads event attributes. One skip condition worth noting: a span with an empty
  `trace_id` or `span_id` is `continue`d (`:340-342`); Conductor's are both seeded non-empty. The observer
  fires per row with a non-null fingerprint (`:394-397`).
- `crates/buffer/src/fingerprint.rs` (1–95) — `compute_exception_fingerprint` returns `None` only when
  `exception_type` is `None` or empty; otherwise blake3 over `type ++ \0 ++ normalize_stacktrace(stack)`.
  Confirms the transcribed `None`-condition.
- `crates/triage/src/pattern/storm.rs` (210–250, 370–435) — **the decisive find, below.**
- `pulse-app/src/main.rs` (525–545, 1260–1268) — `fingerprint_observer` is wired
  `Some(Arc::new(StormObserverAdapter::new(…)))` (`:600-601`), and `start_storm_detector` is spawned with
  `DEFAULT_HEARTBEAT_INTERVAL` — the comment states **15s**. So the observer is NOT a null: that branch of the
  hypothesis space is closed.

## Graph impact (53 rows, `callee LIKE '%exception_trace_request%' OR '%fingerprint%'`)
- **`crates/conductor-emit/src/exception.rs`** — 11 rows (definition site + its own unit tests).
- **`crates/conductor-run/src/lib.rs`** — 11 rows: the canary composition is the single largest consumer
  outside the definition; `emit_canary` is where `fingerprint()` and `exception_trace_request()` meet.
- **`crates/conductor-verify/src/preflight.rs`** — 4 rows (`CanaryMarker::new` `:52`, `assert_canary` `:366`)
  — the read-back half that consumes the marker.
- **`crates/conductor-verify/src/extract.rs`** — 4 rows (`fingerprint_refs` extraction).
- Remaining 23 rows are report/db/render/`run_record` carrying the envelope's `fingerprints` array, plus
  test files. **No caller of `exception_trace_request` outside `conductor-emit` and `conductor-run`** — a
  change to the canary's payload shape has a two-file blast radius, and the envelope `fingerprints` column is
  a separate downstream concern this chunk does not touch.

## Patterns detected
- **The `emit.batch` boundary is already instrumented and already allowlisted** (`client.rs:67`,
  `redact.rs:43`) — a shape witness can ride `emission_count`'s existing seam without a new allowlist entry
  if it is carried as key-names-only text on `message` (obs §6 boundary-wrapper precedent).
- **Ephemeral loopback capture is the sanctioned wire channel** — `conductor-emit/tests/egress.rs`'s
  `start_stub()` (`TcpListener::bind("127.0.0.1:0")` → `TcpListenerStream` → `Arc<Mutex<…Request>>`) and
  `conductor-emit/tests/exception_events.rs`, which already asserts the three event attributes on an ERROR
  span. The real `:4317` is never bound by a test.
- **Wire goldens exclude `*_time_unix_nano` rather than masking** (`conductor-run/tests/dispatch_wire.rs` +
  its `snapshots/`) — the projection discipline any new golden must follow.

## Conventions to follow
- **Env-derived conditions are read at the caller and passed in as typed values**, never `std::env::var`
  inside a gate (`conductor-run/src/lib.rs:151-168` `observe_run_contract` / `declares`; testing.md 2026-08-10).
- **Fingerprint is a pure function of content, never the seed** (`exception.rs:115-123`) — a capture asserting
  fingerprint identity across the 6 storm occurrences must not vary it by seed.
- **Wall-clock from `std::time`** — `now_ms()` in the canary marker; never tokio's virtual clock.

## Files to modify
- `crates/conductor-emit/src/client.rs` — the `emit.batch` boundary, if the witness lands at the emitter.
- `crates/conductor-run/src/lib.rs` — `emit_canary`, if the witness lands at the canary composition (the only
  place that knows the storm's intended fingerprint + occurrence count).
- `crates/conductor-run/tests/` — a new wire-tier test file (the canary path currently has **none**).
- `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/` — the evidence artifact.
- (No `redact.rs` change if the witness rides `message`; a new allowlist entry is the alternative and is a
  two-sided obs↔tests change.)

## New files to create
- `crates/conductor-run/tests/canary_wire.rs` — the ephemeral-stub capture of `emit_canary`'s actual requests.
- the chunk's evidence artifact (name/form is P4's; the working entry calls it a capture).

## The decisive find — `tracked_fingerprints_count` is a windowed gauge, not a latched counter

`run_one_storm_cycle` (`storm.rs:370-415`) **evicts first, then reports**: it retains only timestamps within
`window_seconds`, drops fingerprints whose timestamp list empties, and only then sets
`fingerprints_tracked: detector.fingerprints.len()`. The constants are `DEFAULT_STORM_WINDOW_SECONDS = 60`,
`DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30`, thresholds 5 (Suggested) / 10 (Autonomous), and the tick fires
every **15s** (`start_storm_detector` + `main.rs:1265-1268`).

Three consequences the scope's premise does not account for:

1. **Even a perfectly working storm reads `1`, never `6`.** `fingerprints.len()` counts DISTINCT fingerprints
   in a `DashMap`; the canary deliberately emits six *identical*-fingerprint exceptions. So "6 spans arrived"
   and "`tracked_fingerprints_count` should be 6" were never the same claim.
2. **The gauge alone cannot carry the conclusion.** The storm lands at ≈T+45s (after the 45s warm-up); the
   fingerprint is evicted at ≈T+105s; the run continued ~90s past the storm. A tick sampled after T+105s
   would legitimately report 0 even on a working path, so the gauge constrains only the instant it was
   sampled.
3. **The latched signals on the same line do carry it — and they agree.** `storms_detected_total` and
   `fingerprints_evicted_total` are cumulative (`fetch_add` / `load`, `:387-393`) and immune to the window;
   `fingerprints_evicted_total ≥ 1` would prove a fingerprint was tracked and later aged out. Both were
   already at zero: `two-launch-verdict.md:187-189` records "`tracked_fingerprints_count: 0`,
   `fingerprint_count: 0`, `fingerprints_evicted_total: 0` **across every sample**", and the operator's saved
   **arm-3** tail carries a tick ~4 minutes *after* that arm's storm — postdating the whole canary window —
   with `storms_detected_total: 0` and `fingerprints_evicted_total: 0`. Since cumulative counters reset per
   process, nothing had entered arm 3's detector feed by then. Zero evictions ⇒ nothing ever aged out ⇒
   nothing was ever tracked.

   (Recorded as a correction: this section originally inferred from (2) that the observed zero was consistent
   with a fully working path. It is not — the committed evidence in `two-launch-verdict.md` already carried
   the latched signal, and was not checked before drawing that inference. What survives from (1)–(2) is the
   *expectation* fix: a working storm reads 1, never 6.)

   Detection itself runs **inline** in `record_occurrence` at observation time (`:210-250`), not on the tick,
   so a Suggested cue at count ≥5 fires on the sixth occurrence independent of tick timing — another reason
   tick cadence is not the explanation.

This does not settle the fork — it raises the prior on the arrival/intake half without saying which side owns
it. Conductor may not have put intact events on the wire, or Pulse may not have taken them in. The honest
split is unchanged: Conductor owns proving what it sends (no existing test does), and the live witness records
what arrives.

**Evidence-availability constraint:** the raw per-arm Pulse logs are NOT in this repo — only the distilled
`two-launch-verdict.md` and the prior chunk's `report.md`. Confirming the pattern across arms 1–2 (all tick
lines per arm, gauge + both cumulative fields) requires the operator's saved logs.

## Open questions
- Where does the shape witness live — the `emit.batch` emitter boundary (`client.rs`, generic over every
  batch) or the canary composition (`run/lib.rs`, which knows the intended fingerprint and occurrence count)?
  → blocks: **plan-decision** (P4 resolves before synthesis).
- Does the chunk re-run the live leg to re-read Pulse's tick line with the sampling window respected, or does
  it close on the Conductor-side proof alone and hand the Pulse-side reading over? → blocks: **plan-decision**.
- Does the witness ride `message` (key-names-only, no allowlist change) or claim a new allowlisted field
  (a two-sided obs↔tests amendment)? → blocks: **implementation-scope** (file list provisional until fixed).
