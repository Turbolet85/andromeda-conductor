# arch extract

## Relevance
Partial — verdict classification logic and assertion-policy split apply to all deterministic decision-logic; SLO timing model (deferred), per-scenario `expected` config (deferred), and report-state mapping (deferred to Epoch 6) are out of scope.

## Constraints
- Per architecture.md §Design Philosophy — "Outcomes are values, errors are harness faults" — verdicts (`Pass`/`Fail`/`CalibrationRegion`) are typed return values in `Ok(...)`; harness faults (config parse, transport down, MCP unreachable) alone route to `Result::Err`.
- Per §Established Decisions [Probabilistic-Assertion Policy] — deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) hard-fail on mismatch; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) route to `CalibrationRegion` (report-for-human), never hard-fail.
- Per §Occupied Resources (Crate names) — verdict logic lives in `conductor-verify` seam (the read-back client and preflight gate already occupy this crate); no new cross-seam dependency beyond the existing `conductor-core` dep.
- Per §Conventions (Interface surfaces) — error handling uses typed `thiserror` enums per seam crate (e.g. `VerifyError`); `anyhow` only at binary edges.
- Per §Cross-cutting Patterns (Verdict/error wall) — verification outcomes are `match`-able types, not caught as exceptions; the rule applies uniformly across seam crates.

## Patterns to follow
- Verdict classification is deterministic (same inputs ⇒ same output); no wall-clock read inside the classification itself (per §Design Philosophy "Determinism under a seed").
- Hard-path evaluator is a deterministic compare of observed-vs-expected against the assertion's policy class (declared up front as a property of the claim); model-interpretive assertions never emit `Fail` on exact-value mismatch.
- Capture observed-vs-expected delta for `CalibrationRegion` verdicts as human-facing signal in the report (later Epoch 6 wiring).
- Module naming follows the `conductor-verify` interior module pattern already established by the MCP read-back and preflight logic in the same seam.

## Anti-patterns to avoid
- Do NOT route a model-interpretive assertion to `Fail` on an exact-value mismatch; route it to `CalibrationRegion` unconditionally.
- Do NOT introduce a `Result::Err` for a verification outcome; outcomes are `Ok(Verdict)`.
- Do NOT add wall-clock reads or RNG inside the verdict-classification logic (breaks determinism under seed).

## Contract bindings
- **Verdict/error wall ↔ test harness:** Verdict classification returns `Ok(Verdict)` for all outcomes (pass/fail/calibration-region); test fixtures verify the classification by `match`-ing the returned `Verdict`, not by catching errors.
- **Assertion-policy class ↔ next chunk (SLO timing + expected-outcome config):** This chunk owns the *classification mechanism*; the next chunk wires in concrete per-scenario `expected` values and SLO tier bindings — the chunk consumes this mechanism's verdict-classification module(s) to map expected/observed to a verdict.

## Acceptance criteria contributions
- (arch) Verdict-classification logic lives in `conductor-verify` per §Occupied Resources crate boundaries.
- (arch) Classification returns `Ok(Verdict)` in all cases (deterministic verdict classification always succeeds); `Result::Err` reserved for harness faults per §Design Philosophy.
- (arch) Model-interpretive assertions route to `CalibrationRegion`, never hard-fail to `Fail` on exact-value mismatch per §Established Decisions [Probabilistic-Assertion Policy].
- (arch) No new env var, port, or cross-seam dependency beyond `conductor-core` per §Occupied Resources (assertion-policy is purely internal logic).

## Relevant amendment history
(none) — no prior amendments touch verdict-classification or assertion-policy logic (the `architecture-amendments.md` file records only dependency/stack/module-ownership amendments from Epochs 1–4; Epoch 5 chunks are prior and do not yet have amendments recorded for verdict-classification specifics).
