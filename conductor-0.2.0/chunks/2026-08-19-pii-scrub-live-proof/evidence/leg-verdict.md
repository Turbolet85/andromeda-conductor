# Leg verdict — pii-scrub live proof

**Runs:** leg A `2026-08-19T20-37-25-933` (checks intact — the retirement's measurement leg) · leg B
`2026-08-19T20-42-57-839` (the shipped declare-only shape — the witness/pin leg). Fresh data dirs per leg
under `%TEMP%/pulse-legs/` (`2026-08-19T2050-legA` · `2026-08-19T2110-legB`), one-sweep cleanup convention.
**SUT:** andromeda-pulse HEAD `efabe8e` (unchanged since every prior measurement — the TIME-axis obligation
discharged for this chunk); the ratified binary pair (`pulse-app.exe` 2026-08-17 23:04 ·
`andromeda-pulse-mcp.exe` 2026-08-17 22:27).
**Recipe:** the six-item set — `ANDROMEDA_PULSE_DATA_DIR` (fresh per leg) · `ANDROMEDA_PULSE_MCP_ENABLED=1` ·
`ANDROMEDA_PULSE_L4_DETERMINISTIC=true` · sidecar via `PATH` (debug dir) · `pulse-app` launched FROM the data
dir · NO `SEED`, NO `RUST_LOG`. The TOML-declared seed 4317035 governed both legs (envelope `seed` match).

## Leg A — the as-shipped grading measured (then retired)

| Prediction (research, source-proven) | Measured |
|---|---|
| 4 Hard `Absent` sentinels pass VACUOUSLY (no read-back surface carries payload content) | passed; no sentinel on the composed text |
| Hard `Contains "user.email"` fails STRUCTURALLY (no surface can carry it) | failed → worst verdict **`Fail`** |
| Degraded read-back overrides state (det-L4 permanent `degraded_mode`) | `state: KnownResidual` (WARN "served under degraded mode"); renders `[RESIDUAL]`, **exit 0** |
| Fingerprints payload-invariant | the constant `det-*` triple, again |
| `<5s` tier — plan's re-tier conditional | **HELD: `latency_ms` 4098** (journal-relative) → tier KEPT, unlike the long siblings |

The five checks graded nothing (vacuous green / structural red — the fingerprint-storm precedent), so the
TOML retired to declare-only with this measurement recorded in its header, and
`scenario.rs::pii_scrub_is_declare_only_with_the_measurement_recorded` re-based onto the retired shape.

## Leg B — the shipped shape: every witness landed

Envelope: `verdict: null` · `state: KnownResidual` (ManualCheck overridden by the degraded read-back — the
sibling landing, non-Blocked) · `latency_ms: 4115` (`<5s` held again) · `seed: 4317035` · exit 0.

`duckdb.append` table over the leg window (31,951-line slice, pre-leg count 3,280):

| table · rows_appended | count | reading |
|---|---|---|
| `spans` · 2 | ×2 | the two pii trace batches (root `pii.attributes` + child `pii.exception`), per-occurrence identity — no PK drop |
| `span_events` · 1 | ×2 | the exception event per occurrence (values embedded in message + stacktrace on the wire) |
| `log_records` · 7 | ×2 | **the PK-collision fix live: 7/7 both occurrences, zero same-stamp drops** (also observed on leg A — proven twice) |
| `spans` · 1 | ×15 | canary: 3 warm-up + 12 storm spans (per-request appends) |
| `span_events` · 1 (canary share) | 12 of the 14 | the storm's exception events |

Canary ladder on the same leg: `triage.pattern.storm.detected` at `occurrence_count: 5 / suggested` then
`10 / autonomous` (fingerprint_hex `b5fe50e7`) — the Tier-1 incident former; `query_incident_list` went
0 → 1 at the second poll. Reconfirms the suggested@5 → autonomous@10 ladder and the 8-hex prefix convention.

## Hygiene — non-vacuous negatives

- **Pulse's own obs stream: ZERO sentinel hits** across the whole 31,951-line leg-B slice (affixes + JWT
  header) — no payload value leaks into Pulse's telemetry.
- **Conductor artifacts (journal `2026-08-19T20-42-57-839.jsonl` · report `.md` · `runs.db` row ·
  `logs/agent-latest.jsonl`): ZERO hits** on all five stable affixes, the SSN shape (`\d{3}-\d{2}-\d{4}`),
  the PAN shape (`\b4\d{15}\b`), absolute host paths, and internal struct names.
- Non-vacuous per the producer-ran discipline: the payloads provably existed on the wire in the SAME window
  (the ingestion witnesses above), and the artifacts are THIS run's (run_id match) — so the zeros measure
  withholding, not absence of a producer.
- The committed pins carry no content either: `pii_harvest.rs::no_corpus_value_or_affix_reaches_a_pinned_line`
  recomputes all four leg corpora (28 values, via the transcribed `emission_seed` over seed 4317035) + the
  5 affixes = 33 needles, zero hits.

## Gates

nextest **639/639** zero-retry ci profile (633 prior + 1 stamp test + 5 harvest) · doctests + clippy
`-D warnings` green (full bundled `agent-run.sh run`, exit 0; nextest count re-proven standalone after the
first read landed tail-truncated) · `check_load_envelope` green with `[[exempt]]` still exactly empty (no
phase re-shape — the pii phases sit orders of magnitude inside both sustained terms).

**PREREQ (30th consecutive `cargo audit` re-check):** `cargo audit` standalone TRUE exit 1, byte-identical
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — the external
advisory-DB fault persists; `cargo deny check advisories bans licenses sources` standalone TRUE exit 0
("advisories ok, bans ok, licenses ok, sources ok") — the overlap VERIFIED, not assumed. Zero dependency
delta this chunk (`Cargo.toml`/`Cargo.lock` untouched), so the deferral re-pins on the compact basis
(deferred since `2026-08-08-sut-capability-manifest`). No floor raise, no `deny.toml` ignore, no CI edit.

## New measurements (this chunk) — next Pulse visit intake

1. **Bare provider-key recall gap:** Conductor's ApiKey value (`sk_live_…`, bare) matches NO pattern in
   Pulse's scrub catalog — `scrubber.rs`'s `api_key` regex requires a key-name token inside the value
   (`(api[_-]?key|access[_-]?token|secret[_-]?key|auth[_-]?token)[\s=:]+…`); 6/7 category recall against
   this corpus. Source-cited (walk of all 7 patterns at HEAD `efabe8e`); not externally measurable (see 2).
   Conductor deliberately keeps the bare shape — the corpus tests the catalog, not vice versa.
2. **No external scrub observability exists:** no counter, no per-redaction tracing line anywhere in
   `crates/buffer` / `crates/security`; the scrubbed value-bearing fields live in the in-memory buffer no
   corpus tool reads; `spans`/`log_records` store no attribute columns. Suggestion: a redaction-count field
   on `duckdb.append` (or `buffer.tick`) would make P-047 externally gradeable.
3. **`log_records` PK drop semantics:** PK `(ts_unix_nano, resource_hash, severity_number)` silently drops
   same-severity same-nanosecond records (the 2026-08-15 defect class on a second table). Conductor now
   works around it emit-side (`pii_log_record` distinct per-record stamps, base + index); flagged as
   ingestion-semantics intake.

## Honest limits

- **Scrubbing itself was NOT externally witnessed — no observable exists** (measurement 2). The scrub claim
  rests on the SUT's source (whole-field `[REDACTED:{category}]` at persistence; fingerprint over the RAW
  pre-scrub stacktrace; attribute columns absent by schema) and Pulse's own negative-canary tests — recorded
  as source-verified, never as a Conductor measurement.
- Freshness remains the read-back carrier; nothing here strengthens payload identity (unchanged det-L4
  limit). `degraded_mode: true` permanent under det-L4 reconfirmed on both legs.
- The `<5s` tier held because this family's read-back completes ~4s after the last emission — a property of
  its short phases, not a general revision of the siblings' `<90s` re-tiers.

## Artifacts

- `leg-witnesses.jsonl` — the 8 verbatim witness lines (also pinned byte-identical in
  `crates/conductor-run/tests/pii_harvest.rs`); `envelope-status.json` — the mint-then-read status readout.
- `runs/2026-08-19T20-37-25-933.{jsonl,md}` (leg A) · `runs/2026-08-19T20-42-57-839.{jsonl,md}` (leg B) +
  their `runs.db` rows.
- Capture basis: `{data_dir}/logs/agent-latest.jsonl.2026-08-19`, leg B pre-leg line count 3,280 → post-leg
  35,231 (slices retained in the session scratchpad, not committed — the witness extract above is the
  committed record).
