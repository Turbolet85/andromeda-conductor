# Scope — 2026-08-18-restart-suppression-live-proof

**Working entry (intent title, verbatim):** restart-suppression live proof — gap/resume restart event,
surgical suppression window and the bypass triple (P-015, P-016, P-057)

**Version:** conductor-0.2.0 · Epoch 3 — Live proof: the five families
**Promoted:** 2026-08-18 · **Matrix candidate:** v2-13 ("restart-suppression family live-proven",
`chunk:null` / `planned` at promotion — whether this chunk claims it is decided at P5, never here)

_P3 premise closure applied 2026-08-18 — every `[inferred]` tag below is resolved (VERIFIED or
`[premise-corrected: …]`); the evidence chain is `research.md`._

## What this chunk builds

The live, operator-gated proof leg for the restart-suppression family (P-015/P-016/P-057): drive
`scenarios/restart-suppression.toml` against a live deterministic-L4 Pulse (fresh data dir, the
accumulated operator recipe) and grade the reaction on surfaces that can actually carry it, settled
per-surface BEFORE any leg trusts an `[[expected]]` check. Journal + runs.db row + leg-verdict evidence
recorded under this chunk's folder.

Concretely:

1. **Token-gradeability settlement FIRST** (inherited CARRY class, twice-proven pattern). The TOML ships
   two Hard `Contains` checks ("RestartEvent", "ErrorRateSpike"). Both prior live families measured that
   no `retrieve_report` surface carries cue tokens under deterministic L4 (degraded branch renders fixture
   constants; Pulse's own spelling is snake_case) — expect the same retirement to declare-only with the
   live assertions moving to the harvest tier, but settle by measurement/source at P3, inheriting neither
   the blanket pessimism nor the blanket reversal (the 2026-08-17 `evidence_refs` de-vacuuming corrected
   the CARRY's blanket form). (VERIFIED at P3 — no report branch renders a cue kind at any time, an Active incident renders the degraded branch in every mode, and the HEAD-commit de-vacuuming populated only `fingerprint_refs` with `det-*` fixture constants; retirement to declare-only + harvest-tier stands)
2. **Live-viability premises closed at P3, against the SUT's source at HEAD:**
   - The restart-event path (gap>20s → resume) is NOT gated on `BootstrapState::Ready` — arch records the
     only bootstrap gate in `crates/triage/` sits inside `evaluate_service_went_silent` (the P-014 silence
     family). If restart detection turns out baseline-gated, a fresh-dir leg can never fire it and the
     proof needs re-aiming. (VERIFIED at P3 — `pattern/detector.rs` consults no BootstrapState/baseline: per-service last-seen gap only, first observation never emits)
   - The harvest surface carries restart/suppression/bypass witnesses: `suppression_bypassed=true` was
     MEASURED on the error-baseline leg's `triage.cue.emit` line, so the bypass witness exists; what line
     (if any) witnesses the RestartEvent itself and the P-016 suppress-vs-surface decision is unverified.
     (VERIFIED at P3 — `triage.pattern.restart_detect`/`restart_emit`, per-cue `triage.cue.suppression_check`, `triage.cue.suppression_bypass` with `bypass_reason`, and `cues_suppressed`/`bypass_triggered` counters on `triage.cue.tick`)
   - Thresholds VERIFIED at SUT HEAD `efabe8e` (gap 20s · window 60s · persistence cutoff 30s · bypass
     strict `>10.0x` relative / `>0.05` absolute error rate; `cue/thresholds.rs:114-143`), and Pulse ships
     the 8x/3% · 12x/4% · 6x/7% triple as its own suppression unit tests — but the SHIPPED PHASE ENCODING
     is live-unrealizable on two axes, so a TOML re-shape is REQUIRED, not optional
     `[premise-corrected: the 'suppressed' legs' absolute rates (8%, 20%) clear the 0.05 absolute-bypass
     bar so nothing suppresses; the bypass triple sits wholly OUTSIDE the 60s window (RestartEvent ~t+57s,
     window ends ~t+117s, triple starts ~t+132s); and a pure relative-only bypass is unrealizable at
     integer error_percent (mag>10 forces abs>10%>5%) — the relative arm is witnessed by Pulse's own
     relative-over-absolute short-circuit label]`
   - The dispatcher realizes per-phase `error_percent` faithfully for Error-shape phases (proven for the
     error-baseline family; same shape family here). The gap phase (`occurrences = 0`) is the
     emission-gap-resume lever. (VERIFIED at P3 — deterministic percentage realization pinned by `dispatch.rs::error_occurrences_realize_the_declared_percentage`; the `occurrences = 0` gap phase raises `fault.silence` from `conductor-run/src/lib.rs:534`)
3. **The live legs.** Fresh data dir + `pulse-app` restart per leg; the accumulated operator recipe from
   `.claude/rules/verification-harness.md` (it accumulates and wins over point-in-time route wording);
   poll budget ≥ the contract floor; envelope classification recorded. `slo_tier` re-checked against
   measured `latency_ms` — the scenario window is ~177s, so expect the `<20s` declaration to move to
   `<90s` under the same measurement basis both prior families hit. (VERIFIED basis at P3 — `emitted_ms` is stamped pre-timeline at `lib.rs:341`, declare-only rows evaluate no SLO, and `<90s` is the shipped honesty-bucket precedent)
4. **The three-fact CARRY absorbed (verbatim obligations):**
   1. **SEED disposition owned here:** `scripts/agent-run.sh` forces `SEED=424242` onto scenario legs
      when `SEED` is not given, overriding TOML-declared seeds (this TOML declares 4317015). Land the
      one-line disposition: stop the harness overriding TOML-declared seeds, or record the override as
      intended. `.ps1` parity if the line moves.
   2. **Auto-resolve window budgeting:** Pulse auto-resolves idle incidents on 30s ticks; a ≥~2-3min
      window of non-cue-raising traffic outlives the canary incident and the read-back honestly Blocks.
      This family's gap phase is exactly that class while its error phases re-raise identities — budget
      which phases keep the corpus alive when designing the leg + the row's expected state.
   3. **Ride-along:** `crates/conductor-run/src/lib.rs:223` doc comment still reads "fingerprint_refs is
      L4-authored and empty under deterministic L4" — stale (the fixture populates a `det-*` triple,
      measured 2026-08-18); align it with this chunk's conductor-run edit.
5. **PREREQ — 28th consecutive `cargo audit` re-check, FULL form (basis changed at the last wrap):**
   re-run `cargo audit`; on the byte-identical RUSTSEC-2026-0244 DB fault (true exit 1), re-pin on the
   NEW basis — "audit SURFACE unchanged (zero new packages) + `cargo deny check` VERIFIED true exit 0
   across all four classes over the current lock" — deny observed, never echoed. Close the deferral the
   moment audit parses. Do NOT raise a floor, do NOT add a `deny.toml` ignore, do NOT edit CI.
6. **Gates:** workspace nextest `ci` zero-retry + doctests + clippy + `cargo deny` green; any TOML
   re-shape re-runs `check_load_envelope` and re-freezes affected goldens deliberately (never silently);
   harvest predicates pinned to verbatim leg captures (the `baseline_harvest.rs` precedent).

## Boundaries (MUST NOT)

- No Pulse-side changes — SUT findings are recorded as next-visit intake only (the second-incident
  dedupe-sibling precedent), never acted on cross-repo from this chunk.
- Not the latency-regression re-proof (its own Epoch-3 entry owns P-011/P-012), not per-check latency
  (v2-19), not the port-occupier driver (the connection-lifecycle entry owns it).
- No new `[[expected]]` check trusted without per-surface gradeability proof — an `Absent` that cannot
  fail is the vacuous-green class both prior families retired.
- Scenario timing untouched unless measurement compels a re-shape; replay/stream goldens re-frozen
  deliberately with the reason recorded.
- Scope law: no new inbound listener; the scenario keeps its manifest-accepted P-IDs; no scenario DSL.

## Surfaces / contracts touched

- `scenarios/restart-suppression.toml` — expected-check reshape + any measured re-declarations
  (`slo_tier`; phases only if compelled).
- `crates/conductor-run` — leg plumbing as needed, the `lib.rs:223` comment ride-along, and the harvest
  parser/predicates for this family (extend `baseline_harvest.rs` or a sibling, pinned to leg captures).
  (VERIFIED at P3 — `baseline_harvest.rs`/`storm_harvest.rs` precedent; `restart_harvest` is name-free per the code-graph)
- `scripts/agent-run.sh` (+ `.ps1` parity) — the SEED-forcing disposition, one line either way.
- `conductor-0.2.0/verification-matrix.json` — v2-13 claim decision at P5 (concretize per claim-exit
  invariant; the acceptance's "within SLO" and "hard pass/fail" wording must survive concretization
  against what the surfaces can actually grade, or the claim narrows honestly).
- Chunk folder: `research.md` · `plan.md` · `evidence/` (leg captures + leg-verdict).
