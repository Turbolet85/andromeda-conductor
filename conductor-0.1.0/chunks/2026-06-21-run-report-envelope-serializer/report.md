# Report — 2026-06-21-run-report-envelope-serializer

**Chunk:** Run-report envelope serializer — canonical per-scenario-check shape (run_id/seed/scenario/p_ids/verdict/state/slo_tier/fingerprints, Blocked-row null rule) shared by Markdown + runs.db + JSONL
**Date:** 2026-06-21T18:23:37Z
**Commits:** (uncommitted at report time — wrap commits in P7)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/run_record.rs` · `crates/conductor-core/src/verdict.rs` ·
  `crates/conductor-core/src/report_state.rs` · `crates/conductor-verify/src/record.rs` (NEW) ·
  `crates/conductor-verify/src/lib.rs`
- **Symbols / APIs:**
  - NEW `RunRecord::measured(run_id, seed, scenario, p_ids, verdict, state, journal_emitted_at,
    read_back_observed_at, latency_ms, slo_tier, fingerprints) -> RunRecord` (conductor-core, pub assoc fn) —
    the measured-path constructor mirroring the existing `::blocked()`; sets the five measurement `Option`
    fields to `Some`.
  - NEW `Verdict::default_report_state(self) -> ReportState` (conductor-core, pub method) — the default
    auto-path mapping `Pass→Pass`, `Fail→Fail`, **`CalibrationRegion→ManualCheck`**.
  - NEW `CheckOutcome::to_run_record(&self, run_id, seed, scenario, p_ids, journal_emitted_at,
    read_back_observed_at, fingerprints) -> RunRecord` (conductor-verify, pub method) — the producer bridge;
    maps `assessment.verdict → verdict`, applies the default state mapping, carries `slo.latency_ms` +
    `slo_tier`. New cross-seam call edge `conductor-verify → conductor_core::RunRecord` (verify already
    depended on core; star topology preserved).
  - CHANGED (semantics, NOT wire form) `ReportState::ManualCheck` — doc/meaning **widened**: now also the
    terminal state for an auto-measured model-interpretive (calibration-region) check, in addition to the
    operator-checklist no-programmatic-read-back path. The serde wire name (`"ManualCheck"`), label
    (`Manual`), and prefix (`[MANUAL]`) are unchanged.
  - verify `lib.rs`: `mod record;` added (no new `pub use` — the bridge is an inherent method on the
    already-public `CheckOutcome`).
- **Crates / modules:** NEW module `conductor-verify::record`. No new/removed crates.
- **Dependencies:** none added · none bumped.
- **Schema / config:** **envelope wire form UNCHANGED** — the `RunRecord` 11-field serde golden
  (`run_record.rs`) is byte-identical (the refactored test fixture now builds via `::measured(...)` and the
  golden still passes). No new config keys, no violation-schema change.
- **Coverage of new surfaces** (all are internal library APIs — not external-input boundaries, hot-path ops,
  or UI; flags reflect that):
  - `RunRecord::measured` → validation n/a (trusted-producer values, not an external boundary) ·
    instrumentation n/a (pure constructor; the must-trace op is the live run = Epoch 8) · PII redacted✓
    (no new fields; the exactly-11-keys hygiene golden holds — no host paths / struct names) · tests unit✓
    (golden via constructor + "five fields populated" inverse) · a11y n/a · tokens n/a
  - `Verdict::default_report_state` → validation n/a · instrumentation n/a · PII n/a · tests unit✓ · a11y n/a · tokens n/a
  - `CheckOutcome::to_run_record` → validation n/a (consumes the already-classified+redacted `Assessment`;
    `observed`/`expected`/`delta` never reach the envelope) · instrumentation n/a (pure assembly; live run =
    Epoch 8) · PII redacted✓ (exactly-11-keys test proves no `Assessment` field leaks) · tests unit✓ (5
    tests) · a11y n/a · tokens n/a

## Deviations from intent
1. **Test assertion corrected (mine, not a code change):** the bridge's no-leak check first used
   `!s.contains("observed")`, which collided with the field name `read_back_observed_at`. Replaced with the
   exact 11-key equality (the precise guarantee that no `Assessment` field leaks). The code was always correct.
2. **`#[allow(clippy::too_many_arguments)]` on BOTH constructors.** The plan anticipated it for `measured`
   (11 args); `to_run_record` also needed it — clippy counts `&self` (8/7). Both faithfully assemble the
   contract-fixed 11-field envelope; the allow carries a justifying comment.
3. **Export refined:** the plan said "re-export the bridge"; since it is an inherent method on the
   already-public `CheckOutcome`, it needs only `mod record;` — there is no free item to `pub use`.
4. **+1 test beyond the plan** (`fail_outcome_maps_to_fail_state`) for full verdict→state coverage.
5. **Out-of-scope left untouched:** `conductor-report/src/journal.rs`'s test `measured()` fixture still uses
   a struct literal (outside this chunk's Files-to-modify). Still compiles (pub fields). Candidate cleanup
   follow-up.

## Decisions & corrections
- **P4 scope decision A — re-scope to the producer side.** Research found the envelope (`RunRecord`) + serde +
  JSONL serialization already existed (built two epochs early by `2026-06-16-emission-journal-writer`). The
  chunk was re-scoped (scope.md amended) to the genuine gap: the measured constructor + the verdict→state
  mapping + the `CheckOutcome → RunRecord` bridge.
- **P4 mapping decision — `CalibrationRegion → ManualCheck`.** A model-interpretive verdict is auto-measured
  but "reported-for-human, never hard-failed", and `ReportState` has no calibration variant; `ManualCheck`
  ("terminal, awaits a human go/no-go") is the chosen target. This **widens `ManualCheck`'s meaning** beyond
  arch §Read-Back Dependency Posture's current "operator-checklist / no-programmatic-read-back" definition.
  `state` stays an explicit `measured(...)` parameter, so producers with a context-specific state
  (`degraded_mode → KnownResidual`) override the default. (Related but distinct from carried follow-up (c),
  `Decision → ReportState` for operator-pause holds, which stays deferred — this maps `Verdict`, not `Decision`.)
- **Implementation learning (Tier-3 candidate):** clippy's `too_many_arguments` **counts `&self`** — a method
  with 7 non-self args trips `8/7`; methods, not just free fns, need the allow.

## Outcome
- **Acceptance criteria: all met.** measured-golden green via the constructor; measured+blocked share serde
  order; `default_report_state` maps the three verdicts (CalibrationRegion→ManualCheck); the bridge maps
  verdict + default state + `slo.latency_ms` + `slo_tier` + caller context; exactly the 11 owned keys (no
  `Assessment` leak); determinism + JSON round-trip; all new fns infallible (verdict/error wall).
- **Gates green:** `cargo nextest run -p conductor-core -p conductor-verify` 149/149 ·
  `cargo nextest run --workspace --profile ci` **265/265** (+7) · `cargo clippy --workspace --all-targets --
  -D warnings` clean · `cargo test --workspace --doc` 0 failures.
- **Smoke:** skipped — no boot-path change (library-only chunk; the run-producing CLI scenario leg is Epoch
  8). `agent-run.sh status` responded gracefully (usage + exit 2, no crash). The producer path is proven by
  the bridge tests (`CheckOutcome → RunRecord`) + the existing `journal.rs` `RunRecord → JSONL → parse`
  round-trip.
