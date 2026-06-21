# security extract

## Relevance
partial — config/scope/determinism rules apply; input/output boundaries minimal

## Constraints
- Per §Input Validation, the declarative `expected` block MUST derive `garde::Validate` at load (range rules on `slo_tier`, cross-field comparison-kind validation) per the established garde discipline (security-plan §Input Validation, config-boundary row)
- Error responses to `classify` MUST NOT leak absolute paths / internal struct names (security-plan §Error Handling, error-format row) — returned `Assessment`/`matched` are values only
- Journal-relative latency measurement MUST use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (security-plan §Anti-Patterns § Logging, wall-clock-stamp ban)
- Comparison logic MUST treat `tonic::Status` codes and timing values as first-class typed inputs to the verdict/error wall, never panic (security-plan §Anti-Patterns § Universal)
- `latency_ms` / `slo_tier` computed here MUST NOT be written to `runs.db` as unparameterized SQL (use bound parameters per §Anti-Patterns § Input) — deferred to Epoch 6, but wiring must not violate the boundary

## Patterns to follow
- Serde + garde at `expected` config load (security-plan §Input Validation + §Bootstrap phases input-validation-library-install)
- Type-erased `anyhow` collapse to typed internal errors only at seam edges (security-plan §Error Handling, error-format row)
- Deterministic comparison: same scenario + same observed + same journal timing ⇒ identical `matched`/`Assessment` (chunk acceptance anchor)

## Anti-patterns to avoid
- NEVER deserialize `expected` without garde validation at load (security-plan §Anti-Patterns § Input)
- NEVER use wall-clock stamps for latency measurement — journal-relative only (security-plan §Anti-Patterns § Logging)
- NEVER expose absolute paths or internal struct/field names in returned `Assessment` (security-plan §Anti-Patterns § Logging + Error Handling)

## Contract bindings
obs ↔ tests: no direct binding; acceptance criteria (determinism + no-path-leak) verify via fixture-based unit tests, no live Pulse leg

## Acceptance criteria contributions
- (security) garde validation on all `expected` block fields (grep verifies `#[derive(Validate)]` present + `#[garde(...)]` rules on each comparison/tier field)
- (security) `latency_ms` computed from `std::time::SystemTime`, never tokio virtual clock (code review spot-checks)
- (security) run-report artifact redaction test: `Assessment` returned by evaluator contains no absolute paths from `CONDUCTOR_*` directories or seam-crate struct names (unit test fixture)

## Relevant amendment history
- **2026-06-15-config-validation-surface** — garde pinned 0.22.1; applies to the `expected` block serde + garde at load (same pattern as scenario config)
- **2026-06-15-dependency-audit-gate** — toolchain ≥ 1.94.1 confirmed; ensures evaluator's FFI boundary (if any through tonic/protobuf in timing checks) stays safe, though this chunk's evaluation is pure-Rust
