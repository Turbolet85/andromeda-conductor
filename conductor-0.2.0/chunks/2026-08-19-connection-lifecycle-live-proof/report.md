# Report — 2026-08-19-connection-lifecycle-live-proof

**Chunk:** connection-lifecycle live proof — Listening/Receiving/Idle/Stalled walk + the :4317 port-occupier
leg, writing the missing port-occupier driver (conductor-run/faults, P-001..P-004)
**Date:** 2026-08-19T23:12Z (legs ran 2026-08-19T22:38–22:52Z)
**Commits:** none yet this chunk (this wrap's commit is the first since 9f50cc2 pii-scrub)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/phase_spec.rs` · `crates/conductor-core/src/scenario.rs` ·
  `crates/conductor-core/src/lib.rs` · `crates/conductor-core/src/error.rs` ·
  `crates/conductor-core/src/load_envelope.rs` · `crates/conductor-timeline/src/convert.rs` ·
  `crates/conductor-run/Cargo.toml` · `crates/conductor-run/src/lib.rs` ·
  `crates/conductor-run/tests/dispatch_wire.rs` · NEW `crates/conductor-run/tests/connection_harvest.rs` ·
  `scenarios/{receiver-lifecycle-state,receiver-failed-port-conflict,last-span-ago-tracking,orthogonal-health-domains}.toml` ·
  `conductor-0.2.0/verification-matrix.json` (v2-15 acceptance concretized at phase P5; ref+implemented at
  implement) · `Cargo.lock` · NEW `conductor-0.2.0/chunks/2026-08-19-connection-lifecycle-live-proof/`
  (scope/research/plan/report + evidence/leg-verdict.md).
- **Symbols / APIs:** NEW pub `FaultSpec { kind }` + `FaultKindSpec::PortOccupier` + pub field
  `PhaseSpec.fault: Option<FaultSpec>` (serde default; garde dive) + pub(crate)
  `fault_phases_are_silent` (the scenario-level cross-field invariant: fault ⇒ occurrences 0), re-exported
  from `conductor_core`. NEW private `conductor-run` items: `PhaseGuard { _span, _occupier }` +
  `phase_guard(...)` (port-parameterized — production passes `OTLP_INGEST_PORT`, tests pass `:0`);
  `execute_scenario` gains the captured-occupy-failure → post-timeline sanitized `Err` (the P4-ratified
  policy). No new port, env var, IPC method, or endpoint; `:4317` remains the sole deliberate bind,
  now actually DRIVEN for a fault-declared phase's window (RAII release = the scheduler's boundary drop).
- **Crates / modules:** no new crate. NEW dependency EDGE `conductor-run → conductor-faults` (both
  already-registered workspace members; the island closed).
- **Dependencies:** ZERO new external crates. `Cargo.lock` delta = exactly one member-dep line
  (`conductor-faults` added to conductor-run's dependency list).
- **Schema / config:** scenario TOML grammar gains the optional per-phase `[phases.fault]` table
  (`kind = "port_occupier"`, closed enum; unknown kind = parse error; fault ⇒ `occurrences = 0` enforced by
  garde at scenario level). All four family TOMLs retired declare-only (`[[expected]]` 4+1 → 0 across the
  family) with the structural measurement in their headers.
- **Spec-master edits:** none pre-wrap (obs-plan §4 CONDITIONAL→unconditional + arch declare-only-note +
  test-plan two-site re-base are THIS wrap's P2, per the plan's Expected amendments).
- **Counts / qualifiers moved:**
  - nextest count 639 → **655** (648 post-model + 7 harvest; docs stating 639 are stale).
  - `receiver-failed-port-conflict` tier `<5s` → `<90s` (leg B4 measured 75074 ms); phase set 1 → 3
    (pre-conflict 20s / port-held 40s / recovery 15s, all occurrences 0).
  - `last-span-ago-tracking` tier `<5s` → `<20s` (leg B2 measured 6079 ms).
  - `receiver-lifecycle-state` tier `<90s` KEPT (B1 69184 ms; A1 69169 ms) ·
    `orthogonal-health-domains` tier `<90s` KEPT deliberately — B3 measured 18188 ms, a 9% margin under
    `<20s`, too thin for the closed set (reasoning recorded in the TOML).
  - Family `[[expected]]` checks: walk 1 / tracker 1 / orthogonal 2 / port-conflict 1 → **0 each**
    (declare-only; the connection family joins fingerprint-storm / error-baseline / latency-regression /
    restart-suppression / pii-scrub).
  - `connection_lifecycle_fixtures_load_and_validate` (asserted non-empty expected) →
    `connection_lifecycle_fixtures_are_declare_only_at_the_harvest_tier` + NEW
    `receiver_failed_port_conflict_declares_the_occupier_fault`.
  - `harvest chain: storm → baseline → restart → pii → + connection_harvest.rs` (now five).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** no `FaultSpec.port` config field (scope law — tests reach `:0` via the
  private builder parameter, never config); no `EmissionShape::PortOccupier` (a fault is not an emission);
  no timeline-crate signature change (`run_timeline_observed` was already guard-generic).
- **Spec claims disproved by measurement:**
  1. The chunk's own research/TOML prediction "the witnessable walk ledger starts from Stalled" —
     FALSIFIED on legs A1/B1: the canary poll exits fast (~1s after the first poll), leaving the state
     Receiving at phase start, and the 45s warm-up paces 3 spans ~15s apart, straddling the 10s Idle
     threshold (a Receiving↔Idle oscillation). CORRECTED in-chunk (the TOML NOTE + the pinned
     `the_warmup_oscillation_straddles_the_idle_threshold_by_construction`); no master states it.
  2. The concretized v2-15 acceptance sub-clause "…logs 'OTLP gRPC receiver bound' **with a recovery
     transition**" — measured unsatisfiable BY CONSTRUCTION on leg B4: bind status is per-process and
     recovery is a process replacement (the relaunched instance boots its own FSM at Listening while the
     failed one dies in ReceiverFailed), so no FSM transition OUT of ReceiverFailed exists to log. The
     proof the same sentence names (the SUT's own successful rebind, pinned) is intact. Disposition:
     the P7 coverage-gate ESCALATION per the wrap directive (refine-with-evidence + PREMISE-CORRECTION
     note, never a silent flip).
  3. obs-plan §4's CONDITIONAL `fault.port_occupier` parentage — the condition RESOLVED REAL on leg B4:
     span_event `new`+`close` with `parent: "timeline.execute"`, exactly `fault_type` + `port`
     (verbatim in leg-verdict). §4 goes unconditional this wrap (the CARRY's promise due).
- **Coverage of new surfaces:**
  - `[phases.fault]` scenario-config table → validation garde✓ (closed enum + dive + the scenario-level
    occurrences-0 invariant) · instrumentation span✓ (`fault.port_occupier` at occupy, §4-conformant,
    measured beneath `timeline.execute` live) · PII n/a · tests unit✓ (phase_spec ×4, scenario-level ×1,
    guard ×4, catalog guards ×2) + harvest ×7 + live legs ×6 · a11y n/a · tokens n/a.
  - The occupy-failure Err path → validation n/a · instrumentation log✓ (ERROR line, allowlisted message,
    no paths) · PII redacted✓ (sanitized context; loopback addr only) · tests unit✓ + live (leg A2:
    exit 1, NO row) · a11y n/a · tokens n/a.
  - No new UI element; the `[RESIDUAL]` rows render through the shipped closed label set (no new
    bracket/state/token).

## Deviations from intent
1. **`boot &&` pairing dropped from the leg invocations** (plan Test Commands defect): `boot` fires its own
   canary whose OPEN incident dedupe-blocks the run's canary on a shared dir (the 2026-08-16 back-to-back
   rule); `conductor run` preflights internally, so the legs went direct-CLI.
2. **Leg A narrowed to A1 (walk, structural grading) + A2 (Err path)** rather than four old-TOML runs: the
   four Contains checks share one source-proven mechanism; fresh-dir-per-run economics; headers worded to
   what A1 measured (family-class on the flagship).
3. **Walk-ledger prediction corrected against measurement** (Changes item 1) — the TOML NOTE now carries the
   measured ledger.
4. **Leg A's old TOMLs served from an in-repo untracked copy** (`.tmp-lega-scenarios`, since
   operator-deleted): `CONDUCTOR_*` path handles are repo-relative by design (`resolve_under` traversal
   guard rejected the `%TEMP%` absolute).
5. **`orthogonal-health-domains` kept `<90s`** though 18188 ms would fit `<20s` — 9% margin too thin
   (recorded in the TOML comment).
   Also minor: `conductor-core/src/lib.rs` re-export added (the caller-threading rule's own clause;
   research folded it under the model file).

## Decisions & corrections
- P4 fork RATIFIED (operator): occupy-failure = harness Err AFTER the timeline (wall-consistent; no row for
  an unapplied fault) — measured live on leg A2. Rejected: warn+continue (silent-downgrade class).
- v2-15 refinement RATIFIED at phase (P4 fork + P5 preview — the second pre-claim instance after v2-14):
  acceptance concretized to the harvest-tier proof; PREMISE-CORRECTION in notes at claim.
- First mid-scenario operator choreography convention: cues read off Conductor's own self-obs stream
  (`fault.silence` new = stop cue · `fault.port_occupier` new = start-into-conflict cue · its close =
  restart cue); worked first try (cue offsets 46s/+20s/+38s).
- The six-item live-leg recipe extended by measurement: CONDUCTOR_* handles are repo-relative; `boot` must
  not precede `run` on a shared dir (double-canary dedupe).
- Pulse FSM facts measured at source+live: thresholds 10s/60s (1s poller); `trigger_reason` is the human
  string "receiver bind failed" (not `bind_failed`); ReceiverFailed transitions log at ERROR, others INFO;
  Stalled is severity `warning`; recovery from ReceiverFailed = process restart only (no rebind loop).
- The 31st `cargo audit` re-check re-pinned on a RESTATED basis — the zero-dep-delta footing ENDED this
  chunk (the member edge moved the lock): audit standalone TRUE exit 1 byte-identical
  (RUSTSEC-2026-0244); deny TRUE exit 0 over the NEW lock; lock delta = one member-dep line, zero new
  external crates (the 2026-08-16 admitting-a-dependency precedent).

## Outcome
All 14 acceptance criteria met. Gates green: `cargo nextest run --workspace --profile ci` **655/655**
zero-retry · `cargo test --workspace --doc` · `cargo clippy --workspace --all-targets -- -D warnings`
(3 first-pass rfind lints in the new harvest test, fixed) · `bash scripts/agent-run.sh run` bundle exit 0
(twice) · `cargo deny check advisories bans licenses sources` TRUE exit 0 over the new lock · `cargo audit`
probed standalone TRUE exit 1 byte-identical (the standing DB fault — never a chunk gate). Smoke:
boot-path changed → `agent-run.sh run` as a P2 gate + `status` MINT-THEN-READ (printed the session-minted
B4 run_id `2026-08-19T22-50-47-826`). Live proof: six legs, every witness landed (leg-verdict.md) — A1
structural grading (verdict Fail/KnownResidual, 69169 ms), A2 the ratified Err path (exit 1, no row),
B1–B4 the shipped declare-only rows (verdict null / KnownResidual / exit 0; 69184 · 6079 · 18188 ·
75074 ms), B4's choreographed conflict with the occupier bound, ReceiverFailed(receiver bind failed) at
ERROR inside the hold, and the release proven by the SUT's own rebind.
