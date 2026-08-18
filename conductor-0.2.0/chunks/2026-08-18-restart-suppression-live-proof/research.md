# Codebase Research — 2026-08-18-restart-suppression-live-proof

## Scope
- **Depth:** deep (mature codebase + cross-repo SUT verification) · **Reads:** 16 files (ranged) · **Globs/Greps:** ~20 · **Graph queries:** 2 (trace: `.andromeda/runs/2026-08-18T20-36-33-phase/tree-query-2026-08-18-restart-suppression-live-proof.json`)
- SUT read at HEAD `efabe8e` (`/d/dev/projects/andromeda-pulse`, read-only) — same HEAD every prior alignment cites, so no TIME-axis drift since the 2026-08-17 visit.

## Key findings (SUT mechanics at HEAD `efabe8e`)

1. **The restart path is NOT bootstrap/baseline-gated — the make-or-break premise holds.**
   `RestartDetector` (`crates/triage/src/pattern/detector.rs`) keys solely on per-service last-seen
   timestamps: `observe_span` emits `RestartEvent { service, gap_seconds, last_seen_unix_nano }` when a
   gap > threshold is followed by a resume observation (:54-66); first observation never emits; no
   `BootstrapState`/baseline reference anywhere in the file. It is wired as an ingest hot-path
   `SpanObserver` (`pulse-app/src/main.rs:552-566`, `restart_observer.rs` adapter, spawn at `:1318`), so
   the event fires on the RESUME span's arrival — the P-015 "≤2s" is the ingest path, not a tick.
2. **All five threshold constants verified** (`crates/triage/src/cue/thresholds.rs`):
   `DEFAULT_RESTART_GAP_THRESHOLD_SECONDS = 20` (:130) · `DEFAULT_RESTART_SUPPRESSION_WINDOW_SECONDS = 60`
   (:136) · `DEFAULT_SUPPRESSION_PERSISTENCE_CUTOFF_SECONDS = 30` (:143) ·
   `DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER = 10.0` (:114) · `DEFAULT_ABSOLUTE_BYPASS_ERROR_RATE = 0.05`
   (:120). Comparisons are STRICT `>` (`cue/classify.rs::dual_condition_bypass`; relative short-circuits
   over absolute, mirrored by `pattern/suppression.rs::derive_bypass_reason`). The cue evaluator ticks
   every **1s** (`DEFAULT_TICK_INTERVAL`, thresholds.rs:84), so windows open ≤1s after a RestartEvent and
   persistence granularity is fine. Suppression decision = in-window ∧ ErrorRateSpike ∧ persistence <
   cutoff ∧ ¬bypassed → drop (`cue/emitter.rs:95-96`); Pulse ships the 8x/3% · 12x/4% · 6x/7% triple as
   its own unit tests (`pattern/suppression.rs:327-384`).
3. **The shipped TOML encoding cannot realize its suppressed/bypass intents live — re-shape REQUIRED.**
   `absolute_value` for an ErrorRateSpike is the observed error-rate fraction (confirmed by leg A:
   `error_percent = 35` → bypass true on the absolute arm). Consequences for the shipped phases against a
   1% baseline: the "suppressed" burst (20%) and the "8x/3%" leg (encoded `error_percent = 8` → 8%
   absolute) both clear the 0.05 absolute bar → BYPASS, so nothing suppresses; and the bypass triple sits
   wholly OUTSIDE the 60s window (RestartEvent ~t+57s → window ends ~t+117s; triple starts ~t+132s), where
   `evaluate_with_no_active_windows_keeps_all_cues` — the suppress-vs-surface semantics never engage.
   Realizable with integer `error_percent`: suppressed leg ≤4% short-persistence in-window (mag <10 ∧ abs
   <0.05 → dropped); absolute-arm bypass ~7% in-window (mag ≤10 ∧ abs >0.05 → kept, reason
   `absolute_error_rate`); a PURE relative-only bypass is arithmetically impossible (mag >10 forces abs >
   10×base ≥ 10% > 5% at any integer baseline ≥1%) — the relative arm is witnessed by Pulse's own
   relative-over-absolute short-circuit LABEL on a >10x cue (a measured narrowing to record, the
   PathVariant precedent).
4. **Harvest witnesses exist, with exact targets:** `triage.pattern.restart_detect` (cue_kind,
   gap_seconds, restart_window_active) + `triage.pattern.restart_emit` (pattern/mod.rs:52-53, emit sites
   detector.rs:136-147) · per-cue `triage.cue.suppression_check` (cue_kind, persistence_seconds,
   restart_window_active, suppression_bypassed; emitter.rs:93-108) · `triage.cue.suppression_bypass`
   (cue_kind, bypass_reason label; :120-135) · `cues_suppressed` / `bypass_triggered` counters on
   `triage.cue.emit`-sibling heartbeat `triage.cue.tick` (:148-158) · `triage.cue.emit` itself (the
   baseline_harvest surface). A SUPPRESSED cue leaves a check-line + counter increment and no matching
   `cue.emit` line — that pair is the suppressed-leg witness.
5. **Token-gradeability: the prior families' retirement mechanism still holds at HEAD.** The HEAD commit's
   own record: the de-vacuumed fixture populates `evidence_refs` (3 first-party `det-*` refs) reaching
   `fingerprint_refs` + the Report Evidence section — but `trace_id`/`span_ids`/`timestamps_unix_nano`
   stay hardcoded empty in EVERY mode, `degraded_mode = parsed_l4.is_none()` is driven by Resolved-only
   `resolution_summary_text` persistence (an ACTIVE incident renders degraded even in real-model mode),
   and no report branch renders a cue kind. `Contains "RestartEvent"` / `"ErrorRateSpike"` on read-back
   text can only hard-Fail forever → both retire to declare-only; live assertions move to the harvest tier.
6. **Incident/corpus budgeting facts:** priority tiers (classify.rs:23-29): Autonomous = mag ≥5 ∧ conf
   ≥0.9 ∧ persistence ≥30s; Suggested = mag ≥3 ∧ conf ≥0.7. Tier-1 incident formation accepts Autonomous
   alone (coordinator.rs:390, prior chunk's measurement). Confidence = samples/100 capped 1.0 (leg A:
   1.0) → only a ≥30s-persistence phase with mag ≥5 and ≥90 accumulated samples can form the incident.
   Auto-resolve: `DEFAULT_AUTO_RESOLVE_TICK_INTERVAL = 30s` (`pulse-app/src/incident_observer.rs:24`,
   target `triage.incident.auto_resolve.tick` :29); leg A measured the canary incident resolved after
   idling ~180s of benign window. The scenario's pre-error head (~57s: baseline 30 + gap 25 + resume 2)
   is safely under that; the sustained leg is the natural corpus-keeper if shaped to Autonomous.

## Files inspected
- `scripts/agent-run.sh` (:92-95) — the run verb ALWAYS passes `--seed "${SEED:-424242}"`; since the CLI
  flag legitimately outranks the TOML (flags > env > files), the harness fabricates an override the
  operator never asked for. Conforming fix: pass `--seed` only when `SEED` is explicitly set.
- `scripts/agent-run.ps1` (:103-105) — identical forcing; `.sh`/`.ps1` parity obligation.
- `crates/conductor-run/src/lib.rs` (:215-223, :308-309, :330-345, :430-457, :523-534) — the stale
  "`fingerprint_refs` is L4-authored and empty" clause at :222-223 (ride-along); generic span names
  `timeline.execute`/`emit.batch`/`verify.readback*` (no per-family names shipped — obs-plan §4's
  family-specific chain is target prose the declare-only landing supersedes, a wrap-amendment note);
  `emitted_ms`/`journal_emitted_at` stamped BEFORE the timeline (:341-342, the whole-run latency basis);
  declare-only rows → `ManualCheck`, degraded read-back → `KnownResidual` (:430-457); `fault.silence`
  raised from the per-phase hook (:534).
- `crates/conductor-run/src/dispatch.rs` (:60-95, :244) — `EmissionShape::Error { depth, error_percent }`
  realized deterministically per-occurrence (`is_error_occurrence`); pinned by
  `error_occurrences_realize_the_declared_percentage`. No dispatch change needed.
- `crates/conductor-run/tests/baseline_harvest.rs` (:1-103) — the harvest pattern to mirror: parse the
  SUT-side capture (`{data_dir}/logs/agent-latest.jsonl.<date>` on Windows), target-filter, typed
  observation struct, skip-not-fatal, predicates pinned to verbatim leg captures; `CueEmitted` already
  carries `suppression_bypassed` and `absolute_bypass_witness` exists (:99-103).
- `crates/conductor-core/src/scenario.rs` (:415-427, :487-505, :553) — the all-Hard 3-case rstest
  includes restart-suppression (must move out on retirement — `all()` over an empty set passes vacuously)
  and the two declare-only loader-test precedents to mirror.
- `crates/conductor-core/src/redact.rs` (:21-52) — `fault_type`/`fault_duration_ms`/
  `fault_start_offset_ms` already allowlisted (gap-phase span attrs); no new attribute needed — harvest
  witnesses live in PULSE's artifact, not Conductor's span attributes.
- `scenarios/restart-suppression.toml` (full) — 8 phases, 2 Hard `Contains` checks, `<20s`, seed 4317015;
  comments carry the unrealizable triple labels (`8x/3pct` encodes `error_percent = 8`).
- `scenarios/error-baseline-spike.toml` (tail) — the declare-only retirement shape: mechanism comment +
  empty expected list.
- `conductor-0.2.0/chunks/2026-08-18-error-baseline-spike-live-proof/evidence/leg-verdict.md` (:45-50) —
  the auto-resolve measurement cited above.
- SUT (read-only, HEAD `efabe8e`): `crates/triage/src/cue/thresholds.rs` (:84, :95-160),
  `pattern/detector.rs`, `pattern/suppression.rs` (map + tests :327-470), `cue/classify.rs` (:1-75),
  `cue/emitter.rs` (:90-160, :242-310), `cue/mod.rs` (:42-55), `pattern/mod.rs` (:52-54),
  `pulse-app/src/main.rs` (:552-566, :1317-1318), `pulse-app/src/restart_observer.rs` (:18-40),
  `pulse-app/src/incident_observer.rs` (:24-29), `contract.rs` (re-export map).

## Graph impact
- **`%restart%` descriptors in Conductor:** exactly 1 — the loader test
  `scenario/tests/activity_floor_and_restart_suppression_checks_are_all_hard()` (conductor-core). No
  existing restart driver/module; `restart_harvest` is name-free; the new test file is purely additive.
- **`conductor-run` crate edges:** recorded in the trace (outbound → core/emit/timeline/verify/report;
  inbound = the two bins) — a crate-local integration test adds zero cross-crate blast. Trace file is the
  authority for row counts.
- **Caller threading:** none — no signature changes anywhere; the TOML is data, the harness scripts are
  unindexed shell, the comment edit is doc-only. Catalog-wide gates that re-run over the re-shaped TOML:
  `check_load_envelope` (conductor-core unit tier) + the scenario loader tests.

## Patterns detected
- **Harvest-tier proof** (`baseline_harvest.rs:1-103`, `storm_harvest.rs`): TEST-ONLY affordance, nothing
  wired into the run path; predicates pinned to verbatim leg captures; parse-skip-not-fatal (the
  verdict/error wall applied to evidence).
- **Declare-only retirement** (`error-baseline-spike.toml` tail + `scenario.rs:415-427/:553`): empty
  `[[expected]]` + a mechanism comment naming WHY each check cannot grade + a loader test asserting
  emptiness with the reason — never a silent deletion.
- **Declare-only row routing** (`lib.rs:430-457`): `ManualCheck` base, `KnownResidual` under degraded
  read-back, verdict stays `None` — the non-Blocked expected state leg A already exhibited.
- **Deterministic error realization** (`dispatch.rs:87-93`): realized rate = declared `error_percent`
  exactly, so live magnitudes track TOML arithmetic (modulo Pulse's EWMA drift across phases — leg A
  measured 3.09-3.19 for a 3.5x intent).

## Conventions to follow
- Live-leg recipe accumulates in `.claude/rules/verification-harness.md` and wins over route wording
  (six-item set: sidecar on PATH · `ANDROMEDA_PULSE_L4_DETERMINISTIC` + `ANDROMEDA_PULSE_DATA_DIR` in the
  launching shell · `ANDROMEDA_PULSE_MCP_ENABLED` in Conductor's env · poll ≥ contract floor · pulse-app
  cwd outside this repo · paired RUST_LOG only on a boot leg).
- SUT constants transcribed into comments/tests cite `file:line` at the named HEAD (storm/baseline
  harvest precedent) — the transcribed-record posture, never a Conductor measurement.
- Fresh data dir + `pulse-app` restart per leg; evidence under `chunks/{marker}/evidence/` with
  host-path/struct-name hygiene (grep-clean gate).
- No stream/replay golden consumes this TOML (`determinism.rs:17` is a hand-built `Phase` label
  coincidence; `dispatch_wire__*` goldens are fingerprint-storm-driven; `fixture_seed_424242` replay
  golden consumes error-baseline-spike.toml) — the re-shape triggers NO golden re-freeze.

## New files to create
- `crates/conductor-run/tests/restart_harvest.rs` — restart/suppression harvest: parsers for
  `triage.pattern.restart_detect`/`restart_emit`, `triage.cue.suppression_check`,
  `triage.cue.suppression_bypass`, and the `triage.cue.tick` counters (reuse the `CueEmitted` parsing
  shape for `triage.cue.emit`); predicates for the four witnesses (restart fired with gap_seconds >20 ·
  suppressed leg = check-line ∧ no matching emit ∧ counter >0 · absolute-arm bypass kept with reason
  `absolute_error_rate` · relative-labeled bypass with reason `relative_magnitude`); pinned to verbatim
  leg captures once the leg runs.
- `conductor-0.2.0/chunks/2026-08-18-restart-suppression-live-proof/evidence/` — leg captures +
  leg-verdict.md.

## Files to modify
- `scenarios/restart-suppression.toml` — re-shape phases so every intended outcome is inside Pulse's
  actual math and window (per Key finding 3), retire both `[[expected]]` to declare-only with the
  mechanism comment, re-declare `slo_tier` `<20s` → `<90s`.
- `crates/conductor-core/src/scenario.rs` — move `restart-suppression` out of the all-Hard rstest
  (:487-503; leave activity-floor + service-went-silent) and add the declare-only loader test (mirror
  :424/:553 precedents).
- `crates/conductor-run/src/lib.rs` (:222-223) — align the stale "empty under deterministic L4" doc
  comment with the measured `det-*` triple (the CARRY ride-along).
- `scripts/agent-run.sh` (:95) + `scripts/agent-run.ps1` (:103-105) — SEED disposition: pass `--seed`
  only when `SEED` is explicitly set, so a TOML-declared seed is not silently overridden; semantic parity
  both shells.
- (P5, ledger not spec) `conductor-0.2.0/verification-matrix.json` — v2-13 claim + concretized acceptance.

## Open questions
- none — the plan-decision facts (window math, bypass arithmetic, witness surfaces, SEED precedence,
  retirement mechanism) are all source-verified above; remaining unknowns (exact live magnitudes under
  EWMA drift, leg timing) are what the live leg itself measures.
