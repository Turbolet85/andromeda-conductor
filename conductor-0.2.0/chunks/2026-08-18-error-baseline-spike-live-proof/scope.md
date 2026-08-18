# Scope — 2026-08-18-error-baseline-spike-live-proof

**Working entry (intent title, verbatim):** error-baseline-spike live proof — baseline convergence, ramp and
candidate persistence over the sample floors (P-009..P-012)

**Version:** conductor-0.2.0 · Epoch 3 — Live proof: the five families
**Promoted:** 2026-08-18 · **Matrix candidate:** v2-12 ("error-baseline-spike family live-proven",
`chunk:null` / `planned` at promotion — whether this chunk claims it is decided at P5, never here)

_P3 premise closure applied 2026-08-18 — every `[inferred]` tag below is resolved (VERIFIED or
`[premise-corrected: …]`); the evidence chain is `research.md`._

## What this chunk builds

The live, operator-gated proof leg for the statistical-anomaly family (P-009..P-012): drive the family
scenarios against a live deterministic-L4 Pulse and grade the reaction on the surfaces that can actually
carry it — settled per surface at P3, against the SUT's source at HEAD `efabe8e`. Journal + runs.db +
leg-verdict evidence recorded under this chunk's folder.

Concretely:

1. **Token-gradeability settlement — SETTLED at P3, both halves.** (a) The `retrieve_report` half of the
   inherited CARRY STANDS, with its mechanism now known: `degraded_mode = parse(resolution_summary_text)
   .is_none()` and `resolution_summary_text` is `None` at incident creation, attached only at resolution
   (`pulse-app/src/inference_runtime.rs:742`, `:512-519`) — so during any leg with an open incident the
   report renders the DEGRADED branch (title/status/severity/symptom/evidence refs only,
   `interpretation/markdown.rs:296-327`), whose content is deterministic-fixture constants; no branch renders
   a cue kind, and Pulse's own spelling is snake_case anyway. `Contains "ErrorRateSpike"` /
   `"LatencyRegression"` would hard-Fail forever. (b) The `CountAtLeast` floors grade `evidence_count` =
   `span_refs` count = `incident.evidence_refs.span_ids.len()` = **0 by construction** (`span_ids =
   Vec::new()` at creation, no writer) — a permanent CalibrationRegion measuring nothing. The visit's
   "de-vacuumed `evidence_refs`" is real but lands on `fingerprint_refs` (3 `det-*` fixture constants),
   which feeds the envelope's `fingerprints`, not the count and not the tokens. → Both checks retire to
   declare-only; the live assertions move to the harvest surface (the v2-11 disposition, per-surface
   evidence in `research.md` §Key findings 1).
2. **The live legs.** VERIFIED at P3: the family = both shipped catalog TOMLs —
   `scenarios/error-baseline-spike.toml` (P-009/P-010, seed 424242) + `scenarios/latency-regression.toml`
   (P-011/P-012, seed 4317011) — and the leg drives both, one leg per scenario, FRESH data dir + `pulse-app`
   restart between legs (two independent dedupe/retention mechanisms). Operator recipe is the
   **SIX-item** set `[premise-corrected: .claude/rules/verification-harness.md accumulates the recipe and
   wins over the route entry's point-in-time five]`: sidecar built + on PATH ·
   `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` + `ANDROMEDA_PULSE_DATA_DIR` in the launching shell ·
   `ANDROMEDA_PULSE_MCP_ENABLED=true` in Conductor's own env · poll budget ≥ the contract floor ·
   `pulse-app` launched from a cwd outside this repo (Conductor from its repo root) · paired
   `RUST_LOG=info,conductor_emit=debug` when the wire witness is wanted, boot-leg only, never on an
   invocation that also runs the test suite.
3. **Baseline / sample-floor premise.** VERIFIED at P3: the family is SAMPLE-gated Pulse-side —
   `min_ewma_samples = 10` (P-009) and `min_latency_samples = 50` (P-011), `crates/triage/src/cue/
   thresholds.rs:99,:107`, and only the silence family checks `BootstrapState` — and the shipped phases
   clear both floors as-is (error: 60+40 spans; latency: 180+180). No phase re-shape; **phase timing stays
   untouched** (the seed-named replay golden `fixture_seed_424242` consumes this exact TOML's
   gaps/seed/jitter).
4. **Re-calibration under the family acceptance's own clause.** `slo_tier` re-checked against the
   measurement basis: `emitted_ms` is stamped BEFORE the timeline runs (`conductor-run/src/lib.rs:338`), so
   whole-run `latency_ms` ≈ the scenario window (150s / 180s) + poll — over every tier deadline (Tier90s =
   90_000 ms) by construction. Once the checks retire nothing grades the tier (manual/residual rows evaluate
   no SLO); the tier value moves to `<90s` as the honesty-maximizing available bucket (the v2-11 precedent —
   a P4 lean, reviewable), and per-check latency remains v2-19's.
5. **The transcription-equality ride-along.** `[premise-corrected: dispatch.rs:87-123 emits no exception
   events for Error/Latency shapes and both baseline cues carry fingerprint: None (cue/evaluate.rs:74,:141)
   — the family's own emissions can never fire a storm line]` The carry still discharges HERE, via the
   preflight CANARY each leg fires: a 12-occurrence exception storm that writes
   `triage.pattern.storm.detected` with Pulse's 8-hex `fingerprint_hex`, while Conductor computes the
   matching 32-hex at emit (`lib.rs:227`) and currently logs it NOWHERE. One message-borne
   `tracing::info!` witness on the canary path + a prefix-8 harvest comparison completes the live equality
   test. If measured and disagreeing, it is a TRANSCRIPTION defect to raise — never a scenario failure.

## Folded freight (from the working entry — obligations this chunk absorbs)

- **CARRY (2026-08-15-canary-spans-pulse-fingerprints) — Plain-path stream golden, ride-along candidate.**
  Both committed `dispatch_wire__*` goldens are Exception-shape; `EmissionShape::Plain` with
  `Signal::Traces` (`dispatch.rs:82`, and the plain branch inside Error at `:95-98` — the same
  `trace_request` builder) is asserted only by occurrence-count tests. This family's TOMLs exercise the
  Error shape (with its majority-plain slots) and the Latency shape → a seeded `dispatch_wire` golden
  driven by `error-baseline-spike.toml` covers the cited `:95` branch. NOT a work item on its own — the
  plan decides take/decline.
- **CARRY (2026-08-16-fingerprint-storm-live-proof) — ungradeable-token hazard.** Settled as §1 above; the
  `slo_tier` re-check of §4 inherited from the same carry.
- **CARRY (2026-08-17-fingerprint-semantics-token-leading) — live derivation equality unproven.** As §5
  above; discharged via the canary witness + harvest comparison on this chunk's legs.
- **PREREQ — cargo audit re-check.** Standing bounded deferral since 2026-08-08-sut-capability-manifest
  (operator-ratified at the 2026-08-10 wrap; re-pins silently). Basis: advisory-DATABASE fault — byte-identical
  `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1, no released tool can read the DB, so no floor
  exists to raise. At this chunk's gates: re-run `cargo audit`, record the result; VERIFY `cargo deny check`
  green across all four classes (never assume the overlap); close the deferral the moment it parses. Do NOT
  raise a floor, do NOT add a `deny.toml` ignore, do NOT edit CI.

## Boundaries

- **No Pulse-side change.** The SUT repo is read-only reference; findings are cited by path/line, never
  copied. Anything the leg proves Pulse-side lands as evidence pointers.
- **Operator-gated, never CI.** Live legs are a local operator gate (architecture §CI/CD); any unit/golden
  additions stay deterministic under the zero-retry `ci` profile.
- **Scenario TOMLs move only under the acceptance's re-calibration clause** (check retirement, `slo_tier`,
  header prose) — the v2-11 disposition class; gap_ms/seed/jitter_ms are NOT touched.
  `[premise-corrected: one additive production line rides along]` Scenario model and seam-crate APIs are
  unchanged; the only production-code delta is a single allowlist-clean, message-borne `tracing::info!` in
  `conductor-run::emit_canary` (the §5 witness), plus test-only harvest modules.
- **No new dependencies** (VERIFIED at P3 — the harvest tier reuses `serde_json`/`assert_fs`, already
  dev-deps of `conductor-run`). If one lands anyway, the dep-kind boundary rule + the audit/deny admission
  condition apply.
- **Specs are immutable here** — a falsified premise goes to `research.md` + the matrix `notes` channel;
  amendments are wrap's, source comments ride the chunk.
- Scope law / trust boundary unchanged: no new listener; loopback gRPC/MCP-client only.

## Evidence expectations

`chunks/2026-08-18-error-baseline-spike-live-proof/evidence/leg-verdict.md` (+ harvested Pulse
`agent-latest.jsonl.<date>` slices and Conductor `logs/agent-latest.jsonl` slices, cited by leg window);
`runs/<run_id>.jsonl` + `runs.db` rows retained as the acceptance's evidence pointers. VERIFIED shipped at
P3: the `verify.readback.observe` key-set witness fires for `retrieve_report` / `retrieve_telemetry_slice`
on the first non-empty pass (`conductor-verify/src/extract.rs::log_observed_keys`) with the baseline pinned
in `conductor-verify/tests/readback.rs` — record the witness lines and diff them against the pinned baseline
before trusting any measured verdict; this family's leg is the first to reach those readers with a
non-empty corpus.

## P3 premise-closure record (was "Premises to close at P3")

1. Family membership + leg scope → VERIFIED (both TOMLs, both driven; fresh dir per leg).
2. Sample floors met by shipped phases → VERIFIED (60+40 ≥ 10 EWMA; 180+180 ≥ 50; timing untouched).
3. Fingerprint path → CORRECTED (family never fingerprints; the carry discharges via the canary + a new
   message-borne witness).
4. No engine/seam change → CORRECTED (grading path unchanged; one additive self-obs line in
   `emit_canary`; test-only harvest modules).
5. `[[expected]]` shapes today → VERIFIED (CountAtLeast "10"/"50" Hard + Contains "ErrorRateSpike"/
   "LatencyRegression" Hard; slo_tier `<5s`/`<20s`).
6. Harvest surface → VERIFIED: `triage.cue.emit` (kind/priority/scope/magnitude/absolute_value/
   persistence_seconds/confidence/suppression_bypassed; deliberately NO scope_id) + corroborators
   `cadence.trigger` (tier2-skip carries cue_kind+priority on cpu-primary) and `triage.cue.tick` counters.
   Consts: `crates/triage/src/cue/mod.rs:42-44`.
7. No new dependency → VERIFIED.

Two P3 findings the plan must also absorb: **both scenarios top out at Suggested tier** (magnitude 3.5× /
3.0× < the ≥5× Autonomous bar; Tier-1 accepts Autonomous alone and Tier-2 cycles are skipped on
cpu-primary), so NO incident forms from this family on this host — the acceptance's "candidate" is the CUE,
proven on the harvest surface, and "non-blocked" rides the canary incident keeping the corpus non-empty
with `degraded` routing rows to KnownResidual; and **whole-run latency exceeds every tier by construction**
(§4), which is v2-19's to fix properly.
