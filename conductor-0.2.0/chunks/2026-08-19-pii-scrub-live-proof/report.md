# Report — 2026-08-19-pii-scrub-live-proof

**Chunk:** pii-scrub live proof — seven PII categories scrubbed across corpus and report excerpts, verdicts
recorded without excerpt persistence (conductor-run/emit, scenarios, P-035/P-047/P-048)
**Date:** 2026-08-19
**Commits:** none since last_wrap (this wrap's commit is the chunk commit; HEAD at f57fd81)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-emit/src/pii.rs` (M — log-record stamps + doc header + 1 new unit test) ·
  `crates/conductor-core/src/scenario.rs` (M — 2 test re-bases, see Deviations #1) ·
  `scenarios/pii-scrub.toml` (M — declare-only retirement, measurement in header) ·
  `crates/conductor-run/tests/pii_harvest.rs` (NEW — 8 verbatim pins · 5 tests) ·
  `conductor-0.2.0/verification-matrix.json` (M — v2-14 linked at phase P5, `implemented` at implement) ·
  `conductor-0.2.0/chunks/2026-08-19-pii-scrub-live-proof/` (NEW — scope/research/plan/report + evidence) ·
  `.andromeda/master-route.md` + `conductor-0.2.0/working-route.md` (phase promotion append + freeze stamp,
  riding this commit) · `.andromeda/runs/2026-08-19T19-17-45-phase/` (NEW — phase audit trail) ·
  friction-log/handoff bookkeeping.
- **Symbols / APIs:** `pii_logs_request` (pub, conductor-emit) — BEHAVIOR: records now carry strictly
  increasing `time_unix_nano` (one base reading + per-record index offset; signature unchanged);
  `pii_log_record` (private) gains the stamp parameter. NO new env var, port, IPC method, endpoint, or
  export. No preflight/sidecar-spawn/data-dir change.
- **Crates / modules:** none added or removed; no new cross-seam dependency edge.
- **Dependencies:** none — `Cargo.toml` / `Cargo.lock` untouched (zero dependency delta; the audit-deferral
  basis holds).
- **Schema / config:** `scenarios/pii-scrub.toml` — all five `[[expected]]` checks removed (declare-only);
  phases/seed/tier/jitter unchanged. No `runs.db` schema change (row lands via the existing
  ManualCheck→KnownResidual routing; 11 columns, closed 5-state set intact).
- **Spec-master edits:** none applied pre-wrap — the plan's `Expected amendments (wrap)` list (5 entries,
  2 conditional) is this wrap's P2 coverage floor.
- **Counts / qualifiers moved:**
  - nextest workspace count **633 → 639** (+1 `log_records_carry_distinct_strictly_increasing_stamps`,
    +5 `pii_harvest` tests) — docs stating the old count: test-plan (any literal), handoff.
  - `scenarios/pii-scrub.toml` `[[expected]]` **5 → 0** (declare-only; measurement in the TOML header) —
    docs describing pii-scrub as a graded/Hard family: test-plan §6 scenario row + §1 Critical-Path-1 twin,
    architecture §Standard Contracts declare-only family note (currently names error-baseline only),
    obs-plan §4 pii row if it asserts read-back evidence.
  - `slo_tier` **`<5s` KEPT** — the plan's `<90s` re-tier conditional resolved AGAINST re-tiering (measured
    4098 ms leg A / 4115 ms leg B, journal-relative). Doc sample rows showing P-035 with `<5s` remain TRUE.
  - Scrub-family suite composition: Hard-shape members **3 → 2** (pii-scrub moved to the declare-only side;
    the scenario.rs suite-guard comment re-based; guard assertions unchanged — both shapes still present).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:** the family's read-back-gradeable premise (five checks against
  "Report excerpts + corpus") measured structurally ungradeable under det-L4 — **already DISPOSED** into the
  v2-14 PREMISE-CORRECTION note (`verification-matrix.json#v2-14`, operator-ratified claim+reword at the
  phase P4 fork) + the TOML header's measurement record; evidence `evidence/leg-verdict.md` + research.md
  §The central measurement. No undisposed disproof remains.
- **Coverage of new surfaces:**
  - `pii_logs_request` stamp behavior (changed emit path, no new input surface) → validation n/a (no
    external input) · instrumentation ✓ (existing `emit.logs_batch` span covers; no new span) · PII
    redacted✓ (zero excerpt content in any artifact/pin — hygiene grep + the 33-needle negative) · tests
    unit✓ (`log_records_carry_distinct_strictly_increasing_stamps`) + harvest integ✓ (`log_records 7×2`
    live) · a11y n/a · tokens n/a.
  - `crates/conductor-run/tests/pii_harvest.rs` (TEST-ONLY — no runtime surface, nothing wired into the run
    path) → tests unit✓ (5/5); PII redacted✓ (sentinel-absence negative over its own pins); others n/a.

## Deviations from intent
1. **`crates/conductor-core/src/scenario.rs` re-based (2 tests) — NOT in the plan's modify-list.** The old
   `pii_scrub_asserts_scrub_via_hard_absent_and_structure_via_contains` asserted the pre-retirement TOML
   shape (all-Hard + Absent sentinels + Contains marker), which the declare-only retirement made false; it
   is replaced by `pii_scrub_is_declare_only_with_the_measurement_recorded`, and the suite-guard comment's
   membership lists moved pii-scrub to the declare-only side (assertions untouched). Justification: a
   data-file-consuming companion — the same class as an integration test calling a changed signature; the
   plan's touchpoints carried the site as a provisional "loader/golden pins discovered at the step-5 grep"
   entry, and research flagged the class (the restart chunk re-shaped 4 such sites).
2. **Legs invoked via the harness leg-branch command line directly** (`cargo run -q -p conductor-cli --bin
   conductor -- run pii-scrub --agent-mode`, SEED unset) instead of two full `agent-run.sh run` wrappers —
   bounded-subprocess discipline + no double full-gate; identical command line to the harness's own leg
   branch; the full bundled gate ran once after (exit 0).
3. **The "canary 12-row storm batch" harvest predicate re-shaped to measured reality** — the canary appends
   per-request (12 × `spans 1`), not one 12-row batch; the pinned incident-formation witness became the
   `storm.detected` ladder pair (suggested@5 → autonomous@10) instead.
4. **The `<5s`→`<90s` re-tier conditional resolved to KEEP `<5s`** (measured 4098/4115 ms — this family's
   read-back completes ~4 s after its short phases, unlike the long siblings).
5. **`pii_harvest.rs` authored after leg B** rather than in the P1 write pass — pins are leg-derived (the
   harvest-pattern sequencing; parsers/predicates cannot pin lines that do not exist yet).
6. **First full-gate read was tail-truncated (own pipeline)** — the nextest Summary was re-proven standalone
   (639/639, zero-retry) rather than trusted through a `tail -30` window.

## Decisions & corrections
- **v2-14 disposition (operator fork, phase P4):** claim + re-word — acceptance concretized to the
  achievable live proof (ingestion witnesses + source-pinned scrub semantics + declare-only retirement +
  hygiene negative), PREMISE-CORRECTION in `notes`; the twice-ratified v2-12/v2-13 shape.
- **The corpus keeps the bare `sk_live_` ApiKey shape** — the corpus tests Pulse's catalog, not vice versa;
  the 6/7 recall gap is SUT-visit intake, never a Conductor-side reshape.
- **Three SUT intake items** (bare `sk_live_` recall gap · no external scrub observability · `log_records`
  PK drop semantics) recorded in `evidence/leg-verdict.md` §New measurements — they travel via the
  overseer's Pulse-visit intake queue, NOT this route's tail (operator directive, this wrap).
- **Audit deferral:** 30th consecutive re-check clean on the compact basis (audit standalone TRUE exit 1,
  byte-identical RUSTSEC-2026-0244 DB fault; deny standalone TRUE exit 0 — overlap VERIFIED; zero dependency
  delta). Re-pins onto connection-lifecycle as the **31st**, origin marker unchanged
  (`2026-08-08-sut-capability-manifest`).
- **Leg housekeeping:** both legs under `%TEMP%/pulse-legs/<ts>` per the operator convention (one-sweep
  cleanup); slices retained in the session scratchpad, witness extracts committed in evidence.

## Outcome
All 13 acceptance criteria met. Gates green (named): `cargo nextest run --workspace --profile ci` —
**639/639, zero-retry** (re-proven standalone) · `cargo test --workspace --doc` · `cargo clippy --workspace
--all-targets -- -D warnings` · full `bash scripts/agent-run.sh run` bundle exit 0 (incl. `ensure_frontend`)
· `check_load_envelope` green, `[[exempt]]` exactly empty · PREREQ probes standalone (audit exit 1
byte-identical fault / deny exit 0). Smoke: **two live legs** against fresh-dir deterministic-L4 Pulse at
SUT HEAD `efabe8e` — leg A `2026-08-19T20-37-25-933` (checks intact: verdict `Fail` from the structural
`Contains`, vacuous `Absent` passes, state `KnownResidual` under the degraded read-back — the retirement's
measurement) · leg B `2026-08-19T20-42-57-839` (shipped shape: verdict `null` / `KnownResidual`,
non-Blocked, every ingestion witness landed — `spans 2×2` / `span_events 1×2` / **`log_records 7×2`, the
PK fix proven live on both legs** / the suggested@5→autonomous@10 ladder); `agent-run.sh status` truthful
after both. Artifact hygiene: ZERO sentinel/shape/host-path/struct-name hits across journal, report,
`runs.db`, self-obs, the 31,951-line Pulse slice, and the committed pins — non-vacuous (ingestion witnessed
same-window). v2-14 `implemented` with the 3-part ref; full record `evidence/leg-verdict.md`.
