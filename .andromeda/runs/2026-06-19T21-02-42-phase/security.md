# security extract

## Relevance
Partial — chunk is a fault helper (input validation, error handling surfaces) within the Minimal-tier architecture; crypto/auth/secrets/HTTPS/compliance rules are out of scope.

## Constraints

1. **Input validation on gap duration and ordering (per security-plan.md §Input Validation):** The gap/resume helper MUST validate the exact gap duration as >20s floor and reject malformed gap/resume ordering at load time; invalid configurations surface as typed `FaultError` VALUE, never panic (Verdict/error wall discipline).

2. **Deterministic fault generation under seed (per security-plan.md §Threat Model Summary, attack-surface CLI input class):** The gap length MUST be reproducible across runs — same seed/scenario inputs ⇒ identical gap shape; no seeded jitter perturbation that could place a gap below the 20s restart-detection threshold (determinism contract).

3. **Typed error handling for invalid config (per security-plan.md §Error Handling):** Extend `FaultError` enum in `conductor-faults/src/error.rs` to name gap/resume validation variants (e.g. `InvalidGapDuration`, `MalformedGapResume`); keep internal detail opaque to `anyhow` edges (seam-level `thiserror` only, collapsed to type-erased `anyhow` at CLI/IPC boundaries).

4. **No unpredictable delay injection (per security-plan.md §Threat Model Summary, determinism cross-cutting):** The gap duration MUST NOT draw from random/unbounded async timers; use only operator-supplied configuration + seed-stable operations. Any await/delay that could perturb the exact gap must be explicit in the design + documented (or avoided entirely if the gap is structure-only, not timed).

## Patterns to follow

1. **Garde + serde co-location (per §Input Validation table, scenario-config boundary):** If the gap/resume helper is configurable as a struct (deserialized from scenario config), derive `#[derive(serde::Deserialize, garde::Validate)]` and attach `#[garde(range)]` to the duration field; place validation rules alongside the serde struct definition in the owning seam crate.

2. **Typed FaultError variants for seam-level diagnostics (per §Error Handling):** Use `thiserror::Error` enum variants (`#[from]` derives) for `InvalidGapDuration { duration_ms, min_floor_ms }`, etc.; keep the error hierarchy internal to `conductor-faults`; do NOT propagate raw values to CLI/IPC.

3. **Determinism test witness (per scope.md acceptance intent):** Add a determinism test that instantiates the gap/resume helper with fixed seed/inputs, serializes/returns the gap structure, runs again with identical seed → assert exact field equality (no floating-point approximation).

## Anti-patterns to avoid

1. **NEVER use unbounded async timers or random jitter inside the gap/resume helper (per §Security Anti-Patterns § Universal, determinism discipline):** The determinism contract mandates reproducible-exact gap length; `tokio::time::sleep` or `rand::thread_rng()` inside the helper breaks it. If timing is needed, it belongs in the timeline integration layer (later epochs), not the fault structure.

2. **NEVER panic on validation failure (per §Verdict/error wall § Error Handling):** Invalid gap config MUST surface as `Result::Err(FaultError)` routed through the harness-fault class, never `unwrap()` or `panic!()` — the verdict wall classifies panics as hard failures that corrupt run classification.

3. **NEVER hardcode the 20s floor in multiple places without a named constant (per §Security Anti-Patterns § Code Patterns, maintainability):** Use a single `const MIN_GAP_MS: u64 = 20_000;` at the module/crate level, referenced in validation + docs; prevents silent floor creep.

## Contract bindings

**obs ↔ faults:** The `fault.silence` span (obs rule names `fault.{silence,ramp,port_occupier}`) is deferred to the timeline-integration epoch (Epochs 7/8); this chunk produces the fault structure only. No span emission here.

**tests ↔ faults:** Determinism test + bounds validation test (gap ≤ 20s ⇒ error) are acceptance criteria; both routed through the seam-level test harness (nextest, llvm-cov, doctest).

## Acceptance criteria contributions

- **(security) Invalid gap (≤20s floor) ⇒ typed `FaultError`, no panic.** Grep verifies: `FaultError::InvalidGapDuration` or named variant present in `conductor-faults/src/error.rs`; test instantiates with gap=15s, asserts `Err(_)`.

- **(security) Deterministic gap under seed — reproducible exact length.** Instantiate with seed S, capture gap_ms; instantiate again with seed S, assert `gap_ms` field matches exactly (unit test).

- **(security) Malformed gap/resume ordering rejected at construction.** Test with resume before gap or missing gap marker, asserts `Err(FaultError::MalformedGapResume)` or equivalent.

- **(security) conductor-faults tests pass (nextest), doctest green, clippy `-D warnings`, llvm-cov threshold.** CI gate.

## Relevant amendment history

**(none)** — The amendments log (2026-06-15 entries) touched config validation (garde versioning), dependency audit floors (cargo-audit/toolchain/tauri), and npm supply chain; none directly apply to the fault-helper chunk (no input-struct deserialize scope, no new deps flagged, no npm surface). The determinism + validation contracts are in the base security-plan.md, not amendments.
