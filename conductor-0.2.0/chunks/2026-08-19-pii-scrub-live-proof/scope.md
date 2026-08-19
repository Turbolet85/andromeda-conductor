# Scope — 2026-08-19-pii-scrub-live-proof

**Working entry (verbatim intent):** pii-scrub live proof — seven PII categories scrubbed across corpus and
report excerpts, verdicts recorded without persisting excerpt content (P-035, P-047, P-048)

**Version:** conductor-0.2.0 · Epoch 3 — Live proof: the five families
**Candidate capability:** v2-14 "pii-scrub family live-proven" — method `dynamic-external`, status `planned`,
`chunk:null`; outcome-level acceptance: "All seven PII categories emitted across spans, logs and exceptions
read back scrubbed in both corpus and report excerpts with structure preserved; verdicts are recorded without
persisting excerpt content into Conductor artifacts."

## What this chunk proves
- The pii-scrub family (`scenarios/pii-scrub.toml`, P-035/P-047/P-048) driven end-to-end against a live
  deterministic-L4 Pulse: the seven P-047 PII categories (email / JWT / bearer / API key / credit card / SSN /
  secret key=value) embedded across spans, logs and exception events reach Pulse intact, and Pulse's ingestion
  of every carrier is measured live (per-table `duckdb.append` counts, incident formation).
- NO MCP read-back surface can carry scrub evidence for this family — source-proven at SUT HEAD `efabe8e` on
  four independent grounds (span attributes never stored; value-bearing stored fields live in the in-memory
  buffer no corpus tool reads; `retrieve_telemetry_slice` projects only the constant `evidence_refs`;
  `retrieve_report` renders only L4-fixture/incident fields). The TOML's 4 Hard `Absent` checks pass only
  VACUOUSLY and the Hard `Contains "user.email"` fails STRUCTURALLY, so the checks retire to declare-only
  WITH the measurement recorded (the storm/baseline precedent), and the live claim grades at the harvest
  tier. [premise-corrected: research.md §The central measurement — supersedes the open "which surface
  witnesses scrubbing" premise]
- Scrub SEMANTICS are proven at source and pinned: whole-field `[REDACTED:{category}]` markers at
  persistence, fingerprint computed pre-scrub, 6/7 category recall against Conductor's corpus — with the
  bare `sk_live_…` ApiKey value matching NO pattern in Pulse's catalog (a real SUT recall gap → Pulse-visit
  intake, alongside "no external scrub observability exists"). [premise-corrected: scrubber.rs walk +
  appender.rs:280-375 — replaces the assumption that the leg itself could measure scrub outcomes]
- Verdict rows recorded WITHOUT persisting excerpt content: no raw PII payload lands in `runs.db`, the JSONL
  journal, the Markdown report, the self-obs streams, or any committed test pin — the journal carries only
  RunRecord envelopes by construction, and a sentinel-absence negative (recomputing the per-occurrence
  corpora from the seed) pins it.
- The leg's live witnesses pinned verbatim into a NEW `crates/conductor-run/tests/pii_harvest.rs` (the
  `restart_harvest.rs` pattern: parsers + witness predicates + pinned verbatim leg lines, TEST-ONLY).

## Boundaries
- No Pulse-side changes; Conductor stays a loopback gRPC/MCP client (scope law). Deterministic-L4 mode is a
  Pulse-side declaration Conductor only reads (run-contract shell-declaration proxy).
- The pii phases sit orders of magnitude inside the load envelope's asserted sustained terms (2 occurrences ·
  2s gaps vs 10000 spans/s · 600s); no re-shape, `[[exempt]]` stays empty; `check_load_envelope` still runs
  as the standing catalog gate. [verified — research.md Files inspected]
- Excerpt handling: assertions cite category labels, counts, and Pulse's own `[REDACTED:{category}]` marker
  convention — never a raw excerpt value — in test pins as well as in run artifacts.
- The one Conductor-side product fix is emit-local: `pii_log_record` gains distinct per-record timestamps so
  the 7 same-severity records cannot collide on Pulse's `log_records` PK `(ts_unix_nano, resource_hash,
  severity_number)`. [premise-corrected: schema.rs:175-186 — a new finding, the 2026-08-15 PK-collision class
  on a different table; the spans path is already collision-free via per-occurrence `emission_seed`]
- Live-leg data dir under `%TEMP%/pulse-legs/<ts>` (operator-directed housekeeping convention).

## PREREQ folded from the working entry (verbatim)
PREREQ: re-check `cargo audit` — **30th consecutive**, standing deferral since `2026-08-08-sut-capability-manifest`;
ratified at the `2026-08-10-workspace-key-divergence-probe` wrap (re-pins silently; no further ratification HALT).
Basis RE-VERIFIED at `2026-08-18-restart-suppression-live-proof` and RESTORED to the compact premise:
advisory-DATABASE fault — byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` at true exit 1 (probed
standalone in-chunk after a pipeline `$?` first read tail's exit — capture true exits standalone), and ZERO
dependency delta this chunk (`Cargo.toml`/`Cargo.lock` untouched), so the audit SURFACE is unchanged. Overlap
VERIFIED, not assumed: `cargo deny check advisories bans licenses sources` observed true exit 0. Remedy is the
bounded wait alone — do NOT raise a floor, do NOT add a `deny.toml` ignore, do NOT edit CI. Close the deferral
the moment it parses. Full rationale: `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

## Surfaces / contracts touched
- `scenarios/pii-scrub.toml` — the graded surface: 5 checks retired to declare-only with the measurement
  recorded; `<5s` → `<90s` re-tier candidate (row latency is read-back-completion-bound like both siblings —
  confirmed from the leg). [verified — sole pii/scrub catalog entry]
- `crates/conductor-emit/src/pii.rs` — the log-record timestamp fix (+ unit test); no other emit change (the
  corpus, carriers and sentinels ship as-is). [verified — pii.rs read in full]
- `crates/conductor-run/tests/pii_harvest.rs` — NEW harvest module. [verified pattern — restart_harvest.rs]
- `contracts/pulse-run-contract.toml` preflight terms (preflight runs inside the leg under `conductor-canary`);
  `contracts/pulse-capabilities.toml` membership of P-035/P-047/P-048 confirmed at lines 48/60/61. [verified]
- `runs.db` / JSONL journal / Markdown report / self-obs logs — the no-excerpt-content hygiene surface.
- `conductor-0.2.0/verification-matrix.json` — v2-14 disposition per the P4/P5 operator fork.

## P3 premise closure (was: Open premises)
- WHICH surface witnesses scrubbing → CLOSED: none does; see "What this chunk proves" bullet 2 and
  research.md §The central measurement.
- Whether the current expected blocks are gradeable → CLOSED: structurally ungradeable under det-L4
  (vacuous Absent / structural-fail Contains); retirement with measurement is the remedy.
- P-035 vs P-047 vs P-048 split → CLOSED from the TOML header + SUT source: P-047 = ingestion-time
  pattern scrub (appender/scrubber); P-048 = no raw OTLP attribute values stored (holds BY SCHEMA for
  span/log attributes; scrubbed-field storage for exception/body/template text); P-035 = anonymized Report
  excerpts (an IN-APP Report surface — the MCP sidecar path renders no telemetry-excerpt section).
- Remaining plan-decision fork (v2-14 claim-with-refined-acceptance vs stay-pooled) → carried to P4/P5,
  operator-resolved.
