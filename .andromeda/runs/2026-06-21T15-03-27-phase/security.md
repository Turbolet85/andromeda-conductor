# security extract

## Relevance
Relevant — chunk introduces verdict classification logic at a core decision boundary.

## Constraints

1. Verdict classification returns `Ok(Verdict { Pass, Fail, CalibrationRegion })` only; `Result::Err` reserved for harness faults per security-plan.md §Error Handling, never for assertion failures.
2. Hard-path deterministic assertions must compare observed vs expected without wall-clock reads or RNG (determinism discipline per security-plan.md §Logging & Monitoring anti-pattern "NEVER write the journal/report wall-clock stamps from tokio's virtual clock").
3. Calibration-region assertions must never emit `Verdict::Fail` on exact-value mismatch — capture the delta and classify as `CalibrationRegion` per security-plan.md §Error Handling verdict/error wall.
4. Verdict classification module(s) consume only `conductor-core::Verdict` (existing type); no new external input boundaries or env-var reads (security-plan.md §Input Validation boundary table — only scenario config + MCP child stdout + CLI args + `CONDUCTOR_*`/`ANDROMEDA_*` env handles are validation points).
5. No new network surface, port bind, or occupied resource introduced (security-plan.md §Threat Model Summary attack surface — Epoch 5 already has loopback OTLP/gRPC + stdio MCP client; this chunk is decision logic only).

## Patterns to follow

1. Deterministic comparison: same observed/expected inputs ⇒ identical `Verdict` output, every run (architecture's determinism discipline per amendment 2026-06-15-structured-logging-stack).
2. Type-erased error edge: internal `thiserror` enums (`VerifyError`) collapse to `anyhow` only at `conductor-cli` / `#[tauri::command]` boundaries; classification logic stays typed per security-plan.md §Error Handling.
3. Typed verdict wall: `tonic::Status` codes and MCP error responses are first-class verification inputs routed as `Ok(Verdict::Fail)` or `Ok(Verdict::CalibrationRegion)`, never panics per security-plan.md §Error Handling.

## Anti-patterns to avoid

1. NEVER hard-fail on model-interpretive assertions (severity, hypothesis quality, P-008 weighting) — route to `CalibrationRegion` for human review, never `Fail` per security-plan.md §Error Handling verdict/error wall.
2. NEVER introduce wall-clock / tokio virtual-clock reads inside verdict classification — determinism discipline requires `std::time::SystemTime`/`Instant` if any timing logic is ever added (security-plan.md §Logging & Monitoring).
3. NEVER add env-var reads, CLI arg parsing, or file I/O to verdict classification — it is pure decision logic; input boundaries are closed in earlier seams (security-plan.md §Input Validation).

## Contract bindings

Obs ↔ verdict logging: `ReportState` mapping (`Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked`) belongs to run-report envelope (Epoch 6), not this chunk; `Blocked` is pre-produced by preflight gate; verdict classification feeds into that report-state mapping downstream.

## Acceptance criteria contributions

(security) Verdict classification returns `Ok(Verdict)` for all assertion outcomes; `Result::Err` reserved for harness faults only (no hard-failed assertions on exact-value calibration-region mismatches).

(security) Hard-path deterministic assertions use deterministic input comparison (no wall-clock / RNG); calibration-region assertions never emit `Verdict::Fail` on delta.

(security) Verdict module(s) accept `conductor-core::Verdict` only; no new env-var reads, CLI args, or file I/O introduced; no new network/port surface added.

## Relevant amendment history

- **2026-06-15-structured-logging-stack:** `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs identity labels) noted as non-path env-handles requiring no validation — distinct from `CONDUCTOR_*` path handles. Relevant to this chunk because verdict classification may emit context-tagged logs; any new logging must use these pre-validated labels, not introduce new validation boundaries.

- **2026-06-15-dependency-audit-gate:** toolchain bumped to 1.95.0 (≥1.94.1 floor); cargo-audit/cargo-deny audit gates confirmed green. Relevant because verdict classification seam inherits the clean audit + determinism guarantees from the build pipeline (no unsafe FFI, no vulnerable dependencies touching the decision path).
