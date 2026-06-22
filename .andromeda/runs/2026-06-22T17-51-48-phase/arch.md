# arch extract

## Relevance
partial — workspace-boundary and scenario-catalog patterns apply; no new Rust types or emission primitives.

## Constraints
- Code lives in `scenarios/*.toml` files per architecture §Inherited Defaults (Workspace crate names) and §Infrastructure Patterns directory-tree comment.
- Scenario config must deserialize via `Scenario::from_toml_str` and validate with garde; per architecture §Established Decisions [Scenario Config Format].
- All checks must use `class = "Hard"` per architecture §Probabilistic-Assertion Policy (fingerprint identity and storm thresholds are deterministic, not model-interpretive).
- Reuse existing `ComparisonKind`, `ExpectedCheck`, and `Scenario` types; no new Rust model types or dependencies introduced (per §Stack and Technologies — toml 0.9 already pinned).
- Exception-event + seeded fingerprint emission primitive already shipped in conductor-emit (per architecture §Infrastructure Patterns directory-tree and §Cross-cutting Patterns).
- MCP read-back via rmcp 1.7.0 against Pulse's pinned `2024-11-05` protocol verifies outcomes at Epoch-8 runtime (this chunk declares expected outcomes only, per §Standard Contracts — readiness gate + run report envelope).

## Patterns to follow
- Scenario keying to Pulse capability P-ID (P-017 and P-018); "no scenario without a P-ID" per architecture §Cross-cutting Patterns Scope law.
- Declarative TOML with seed from distinct per-file family (per scope's `43170NN` family, distinct per file) and slo_tier assignment (bounded-slack tier from the Timing-Tolerance Model per §Established Decisions).
- Line-insensitive fingerprint grouping over identical/path/line variants (the amended clause (c) from scope); reflect this outcome-coherently in `expected`.

## Anti-patterns to avoid
- No new cross-crate deps or module-boundary violations (per §Compiler-enforced module seams).
- No CalibrationRegion verdicts for fingerprint identity or storm thresholds (both deterministic, not model-interpretive).
- No per-check window/service scoping that would contradict a single read-back result (per scope's outcome-coherence requirement).

## Contract bindings
obs ↔ run-report envelope (structured logging verifies JSON serialization roundtrip of run records per §Standard Contracts); tests ↔ scenario loader (rstest `#[case]` rows extended per scope).

## Acceptance criteria contributions
- (arch) Scenario TOML files parse + garde-validate via `Scenario::from_toml_str`; loader test suite extended with per-family `#[case]` rows all-Green.
- (arch) P-017 fingerprint-identity outcomes (same/different cases) outcome-coherently declared; line-variant insensitivity (amended clause (c)) reflected.
- (arch) P-018 storm-cue thresholds (6-hit→Suggested, 12-hit→Autonomous) both declared, all `class="Hard"`.
- (arch) All checks remain deterministic (no CalibrationRegion); determinism golden (emission journal) UNCHANGED per architecture §Established Decisions [Determinism under a seed].

## Relevant amendment history
2026-06-18-exception-events-fingerprint-control — fingerprint primitive (identical/path/line/type/frame variants) placed in conductor-emit; conductor-faults narrowed to storm fault composition only (this chunk's runtime partner). The `fingerprint()` function + exception-event builder deliver the P-017 identity contract.