# Report — 2026-08-18-restart-suppression-live-proof

**Chunk:** restart-suppression live proof — gap/resume restart event, surgical suppression window and the
bypass triple (P-015, P-016, P-057) — the triple narrowed by measurement to suppressed + absolute arms
**Date:** 2026-08-18
**Commits:** none since last_wrap — this wrap's commit carries the whole chunk (11 modified + 3 untracked)

## Changes (structured — detectors read this)
- **Files:** `scenarios/restart-suppression.toml` (full re-shape) · `crates/conductor-core/src/scenario.rs`
  (4 loader-test sites) · `crates/conductor-run/src/lib.rs` (canary service split + doc comment) ·
  `crates/conductor-run/tests/canary_wire.rs` (follows the split) · `scripts/agent-run.sh` +
  `scripts/agent-run.ps1` (conditional `--seed`) · NEW `crates/conductor-run/tests/restart_harvest.rs` ·
  NEW `conductor-0.2.0/chunks/2026-08-18-restart-suppression-live-proof/` (scope/research/plan/evidence) ·
  `conductor-0.2.0/verification-matrix.json` (v2-13 claimed at phase P5; `implemented` deliberately
  withheld) · bookkeeping (friction-log, handoff pre-wrap).
- **Symbols / APIs:** NEW `pub const CANARY_SERVICE_NAME: &str = "conductor-canary"`
  (`conductor-run/src/lib.rs`) — the preflight canary's warm-up + storm emissions re-attributed from
  `DEFAULT_SERVICE_NAME` ("conductor"), which stays the scenario dispatcher's identity;
  `DEFAULT_SERVICE_NAME` import dropped from `conductor-run/src/lib.rs` and `canary_wire.rs`. No signature
  changes anywhere. Harness surface: the `agent-run.{sh,ps1}` scenario leg passes `--seed` ONLY when `SEED`
  is explicitly set (files-over-env-defaults precedence); no new verb/flag/prompt.
- **Crates / modules:** no new crates, no new cross-seam edges; one new integration-test bin
  (`restart_harvest.rs`, test-only affordance — nothing wired into the run path).
- **Dependencies:** none (zero delta — `Cargo.toml`/`Cargo.lock` untouched).
- **Schema / config:** `restart-suppression.toml` re-shaped to 10 phases against the SUT's measured
  semantics (young-window pulses at ~1 span/s; two gap/resume cycles; a mature autonomous keeper), retired
  both `[[expected]]` Hard `Contains` checks to declare-only with the mechanism comment, re-declared
  `slo_tier` `"<20s"` → `"<90s"`, seed unchanged (4317015). No run-report-envelope / runs.db /
  self-obs-schema change.
- **Spec-master edits:** none in-chunk (implement authors nothing; P2 owns the sweep).
- **Counts / qualifiers moved:** restart-suppression `slo_tier` `<20s` → `<90s` (layout-templates sample
  rows state tier literals for this scenario — the five sites disagree today: HOLD wireframe `<20s`, four
  others `<5s`); the scenario's `[[expected]]` count 2 → 0 (declare-only family grows to 5: both
  fingerprint, both statistical, restart-suppression); the all-Hard loader-test set shrank to
  activity-floor + service-went-silent; restart-suppression left the Contains-presence loader set.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** the harvest's first draft carried a `triage.cue.tick` counter parser +
  a counter clause in the suppressed-witness predicate — removed same-session (never committed) after the
  live leg measured both counters REDACTED (`"<redacted>"`); a relative-arm bypass predicate was planned
  and never shipped (unreachable at SUT constants, below).
- **Spec claims disproved by measurement** (each needs disposition; leg evidence:
  `evidence/leg-verdict.md` + `evidence/leg-witnesses.jsonl`, run `2026-08-18T21-40-46-519`, SUT HEAD
  `efabe8e`):
  1. **"persistence" as spike DURATION** — Pulse computes `persistence_seconds = snapshot.samples`, the
     service's CUMULATIVE sample count (`crates/triage/src/cue/evaluate.rs:55`; leg: the drop band opened
     at the service's 10th span). Any master text describing P-016 suppression as "<30s persistence
     spikes" *as a time property of the spike* states a false mechanism; suppression-eligibility and
     bypass-attribution exist only while the SERVICE holds 10..29 samples, once per data dir. Known
     statement sites: obs-plan §4 restart-suppression row (family span chain + `bypass_triggered` envelope
     extra — target prose never built, superseded by the declare-only landing); test-plan §6
     "Restart-suppression incl. one bypass case" + §1 Critical Path 3 (hard-pass/fail read-back claim —
     the two-site rule applies); layout-templates restart-suppression sample rows (tier literals).
  2. **The relative-magnitude bypass arm is UNREACHABLE for error cues at shipped SUT constants** —
     alpha_short/alpha_long = (1/30)/(1/300) = 10 equals `DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER`, and the
     0.01 baseline floor caps the transient young-service peak at ~9.7x (3 consecutive errors). Leg: all
     10 bypass triggers carried reason `absolute`. The P-057 live claim is the absolute arm + the
     neither-condition suppressed case.
  3. **`triage.cue.tick`'s `cues_suppressed`/`bypass_triggered` counters are unreadable live** — both
     read `"<redacted>"` (Pulse's default-deny field allowlist predates the chunk-#63 counters). Drop
     evidence rides check-line + emit-absence.
  4. **The old TOML encoding could never realize its intents** — the "suppressed" legs' absolute rates
     (8%, 20%) cleared the 0.05 absolute-bypass bar and the bypass triple sat wholly outside the 60s
     window; superseded by the re-shape (any master text describing the OLD phase table/triple is stale).
  5. Matrix-only (not a master): the concretized v2-13 acceptance's literals — persistence-as-duration
     wording, bypass labels `absolute_error_rate`/`relative_magnitude` (Pulse's `BypassReason::label()`
     emits `absolute`/`relative`), the tick-counter clause, the relative-arm clause — all premise-disproved
     while every substantive witness PASSED. Disposition = the P7 coverage-gate escalation
     (refine-with-evidence recommended, v2-12 precedent).
- **Coverage of new surfaces:**
  - `restart_harvest.rs` (test-only) → validation n/a · instrumentation n/a · PII n/a (grep-clean of host
    paths/struct names) · tests unit ✓ (10 tests incl. 7 pinned verbatim leg lines) · a11y n/a · tokens n/a
  - `CANARY_SERVICE_NAME` (canary identity) → validation n/a (const) · instrumentation ✓ (existing
    spans/logs unchanged) · PII n/a · tests ✓ (`canary_wire.rs` attribution + warm-up tests updated) ·
    a11y n/a · tokens n/a
  - re-shaped TOML → validation garde ✓ (load-validated; loader tests) · instrumentation ✓ (both gap
    phases raised `fault.silence` at info, offsets 7006ms/76436ms, parent `timeline.execute`) · PII n/a ·
    tests ✓ (loader + `check_load_envelope` + live leg) · a11y n/a · tokens n/a

## Deviations from intent
1. **Plan step 2's candidate phase table replaced wholesale** — the design premise
   (persistence-as-duration, mature-service witnesses) was measured false BEFORE any code was written;
   the operator ratified proceed-with-redesign at the surfaced premise disproof. Justification:
   `research.md` missed `cue/evaluate.rs:55`; the redesign is source-grounded and every witness landed at
   its predicted sample offset.
2. **`CANARY_SERVICE_NAME` split (unplanned surface)** — preflight runs inside every scenario leg and the
   canary shared the dispatcher's service identity, which would age every scenario's young-sample window
   and inflate its baseline. In-scope per scope.md "leg plumbing as needed"; `canary_wire.rs` updated as
   the crate-local consequence (its attribution test now pins the canary service).
3. **No tick-counter clause, no relative-arm predicate** in the shipped harvest — both measured
   unavailable/unreachable (Changes items 2-3).
4. **A fourth loader-test site re-shaped** (`activity_floor_and_restart_suppression_fixtures_load_and_validate`
   asserted non-empty expected) — the P3 research grep map listed three; same retirement pattern applied.
5. **v2-13 `implemented` write withheld** — a green test built against a reality the acceptance's literals
   contradict must not flip silently; the refinement escalation is this wrap's to resolve.
6. Landed exactly per plan: the SEED disposition (live-proven — the leg ran with no `SEED` env and the
   envelope records `seed: 4317015`), the `lib.rs:222-223` comment alignment, the audit PREREQ full form.

## Decisions & corrections
- Operator ratified the young-window redesign at the surfaced premise disproof (AskUserQuestion,
  "Proceed with redesign").
- SEED disposition resolved as *stop overriding TOML-declared seeds* per the arch files-over-env-defaults
  precedence; `.sh`/`.ps1` parity; proven live.
- Design method that worked: place error clusters at exact sample offsets via short front-loaded pulse
  phases (`is_error_occurrence` is positional); pace ~1 span/s so samples track seconds; ascend rates
  never required — the young-window bands are what matter.
- A verification-mode surface can be structurally unreadable: a counter can exist, be documented, and
  read `"<redacted>"` live because the field allowlist predates it — check the allowlist (or measure)
  before grading on any counter.
- Shell discipline (repeat offenders): a pipeline's `$?` is the tail's, not the probe's — capture true
  exits standalone; `$TMPDIR` is unset in this Git Bash — always use the scratchpad path.
- Operator curation candidate (directive 5): all leg data dirs under one parent
  (`%TEMP%/pulse-legs/<ts>`) so periodic cleanup is one sweep — verification-harness home, beside
  fresh-dir-per-leg.
- Three next-Pulse-visit intake items (leg-verdict §New measurements): persistence semantics vs the
  capability spec's duration wording · the `triage.cue.tick` allowlist gap · the α-ratio = multiplier
  coincidence.
- Friction-id discipline: every evolve record this chunk took `date -u` first and wrote the literal
  timestamp (13 records, zero guessed-ahead ids).

## Outcome
All 12 plan acceptance criteria met. Workspace gates green and re-verified after fixture pinning:
`cargo nextest run --workspace --profile ci` **633/633 zero-retry** · `cargo test --workspace --doc`
clean (7 suites) · `cargo clippy --workspace --all-targets -- -D warnings` clean ·
`cargo deny check advisories bans licenses sources` TRUE exit 0 (observed) · `cargo audit` = the
byte-identical RUSTSEC-2026-0244 advisory-DB parse fault at TRUE exit 1 (29th consecutive in-chunk probe;
zero new packages — deferral re-pins full-form). Live leg (the smoke, in firing form:
`SCENARIO=restart-suppression bash scripts/agent-run.sh run`, no SEED, no RUST_LOG, fresh data dir,
six-item recipe): row `KnownResidual`/verdict null/non-Blocked, `latency_ms 168192`, `<90s`,
seed 4317015; every witness landed at the predicted sample offsets (2 restarts gaps 25s/26s · drops at
persistence 10-13 with zero leaks · 10 absolute-arm triggers at 20-29 · 41 plain-kept at 38-49 ·
22 autonomous cues from persistence 90); 7 verbatim lines pinned into `restart_harvest.rs`; evidence
hygiene grep CLEAN; both `fault.silence` spans present; zero panic lines both sides; in-envelope
(no `[ENVIRONMENT-SUSPECT]`). Matrix: v2-13 claimed, `implemented` withheld pending this wrap's
refinement escalation.
