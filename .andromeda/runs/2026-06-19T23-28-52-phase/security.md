# security extract

## Relevance
Partial — chunk involves fault helper construction (input validation on fault parameters), error handling (verdict/error wall), and dependency/supply-chain concerns (no new deps); out of scope: live emit wiring, MCP verification, timeline execution.

## Constraints
- Fault construction failure MUST surface as typed `FaultError` value, never panic (security plan §Error Handling, verdict/error wall) — applies whether helper is infallible or carries minimal validation.
- All new public seam-crate types co-located with their validation (if fallible) use `thiserror` 2.0.18 typed enums collapsed to `anyhow` only at CLI/Tauri command edges (security plan §Error Handling, Code Patterns).
- Permanence/no-resume property must be structurally expressed so the type cannot model a resuming gap (security plan §Input Validation, Verdict/error wall — silent mismatch prevention).
- No new dependencies — abrupt-silence helper stays self-contained per fault siblings' pattern (scope § Boundaries / §What it builds); cargo-audit clean (security plan §Dependency Security § Audit tool, CI integration).
- Determinism by construction — no seed parameter, no seeded randomness (scope §Determinism by construction); deterministic-primitive test only (not different-seeds-diverge per testing-rule); reproducibility across same scenario.

## Patterns to follow
- Peer fault helpers (`PortOccupier`, `EmissionGap`) as reference for infallible vs. fallible constructor discipline — both stay self-contained, no new deps, re-exported from `lib.rs` with crate-doc update.
- Verdict/error wall: typed enums (`FaultError`) inside seam crate, `Ok(...)` for values/success, `Result::Err` for harness faults only (never panics on invalid input).
- Public API surface: new type + `pub use` re-export in `conductor-faults/lib.rs`; crate-doc already forward-references P-014 permanent-silence (scope fulfills existing reference).

## Anti-patterns to avoid
- NEVER panic on invalid fault configuration — fallible constructor errors MUST be typed `FaultError` values (security plan §Error Handling, §Anti-Patterns § Universal).
- NEVER add undocumented validation logic or silent truncation — any bounds applied MUST be explicit in the type or constructor contract.
- NEVER introduce new dependency for the permanence/no-resume model — the type structure itself (absence of resume boundary vs. `EmissionGap` presence) is the control.

## Contract bindings
- obs ↔ testing: obs `fault.silence` span is deferred to later epoch (Epoch 7/8 timeline-driven phase); this chunk ships the primitive only; testing binds to faults seam tests + nextest + clippy `-D warnings` + llvm-cov threshold (acceptance intent, gates).

## Acceptance criteria contributions
- (security) Fault construction failure ⇒ typed `FaultError` (or infallible constructor with no error path); no panic on any input path.
- (security) Permanence/no-resume property structurally expressed in type (comparison test vs. `EmissionGap` distinguishes the two).
- (security) `cargo audit` / `cargo deny advisories` clean (no new dependencies; existing tree audit).

## Relevant amendment history
**2026-06-15-dependency-audit-gate** (partial relevance): Confirms cargo-audit 0.22.1 / cargo-deny 0.19.4 are minimum-floor tools running green; toolchain ≥1.94.1 confirmed done (clears tar-rs CVE-2026-33056). `tauri` ≥2.10.3 forward-required (not yet locked — dormant until Epoch 9 GUI chunk); abrupt-silence helper adds no new deps, so no forward gate applies here.