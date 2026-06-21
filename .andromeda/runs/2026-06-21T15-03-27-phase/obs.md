# obs extract

## Relevance
Partial — verdict classification mechanism (hard vs calibration-region) is instrumentation-visible; the policy split produces verdict enums that flow through obs spans and JSONL fields, but implementation is primarily domain logic (not obs infrastructure).

## Constraints
- Per obs-plan.md §3 Observability Harness Contract: verdict must be serialized as an enum field (`verdict: "Pass | Fail | CalibrationRegion"`) in the JSONL Run-report envelope on every scenario-result record.
- Per obs-plan.md §4 Span/Trace Coverage (all 7 must-trace scenarios): each scenario span must carry a `verdict` attribute on the root `scenario.run` span at closure, populated only when the verdict is finalized (post-classification).
- Per obs-plan.md §6 Log Coverage: the verdict field is required on every log line (no null verdicts); CalibrationRegion outcomes must be reported with the delta (observed-vs-expected) captured as a structured log field for human review (never hard-failed).
- Per obs-plan.md §10 SLO Invariants: zero-unlogged-panics invariant applies — if verdict classification panics (e.g., on unexpected state enum variant), the panic MUST be captured by `std::panic::set_hook()` and logged via `tracing::error!(panic_msg, backtrace)`.
- Per obs-plan.md §11 Anti-Patterns (Error Reporting): verdict-classification errors MUST NOT be let uncaptured or crash the run; malformed input ⇒ `Blocked` (typed `Ok(...)` verification result), never panic or `Result::Err` leaking to the caller's error path.

## Patterns to follow
- Verdict enum fields in JSONL schema: `"verdict": "Pass | Fail | CalibrationRegion"` (4 character literals, lowercase). Per §6 and amendment 2026-06-16, this is a required field on every JSONL line (no null).
- Hard-path assertion capture: emit a `tracing::info!()` or `tracing::debug!()` span/event on the verdict-evaluation boundary (e.g., `verify.readback` span attributes), labeling the claim class (hard vs calibration-region) and the observed outcome (match / mismatch / delta).
- Calibration-region delta logging: when a model-interpretive assertion's observed value differs from expected, emit `tracing::info!(delta_observed=..., delta_expected=..., assertion_class="calibration_region")` so the human-facing report can render the delta for review (never a `Fail` verdict on the mismatch itself).
- Determinism in classification: all verdict evaluations use wall-clock / structured input only (no RNG, no file I/O, no wall-clock reads inside the classification logic itself — only at the `journal_emitted_at` / `read_back_observed_at` boundaries).

## Anti-patterns to avoid
- NEVER emit a verdict field that is null or missing — every JSONL line MUST carry the verdict enum value (hard pass/fail or CalibrationRegion).
- NEVER hard-fail a model-interpretive assertion on an exact-value mismatch — route to `Verdict::CalibrationRegion`, report the delta, and let the human decide (per §11 Error Reporting anti-pattern on verdict-classification errors and upstream creator brief).
- NEVER introduce new high-cardinality span attributes tied to claim-specific values (e.g., per-P-ID severity delta) — use bounded names (e.g., `verify.readback` only; severity-specific spans, if any, are added in future chunks per §4 must-trace paths).

## Contract bindings
- **obs ↔ tests harness (§3 Observability Harness Contract):** The verdict classification result MUST produce an JSONL Run-report envelope row with the 11 required fields (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, **verdict**, state, latency_ms, slo_tier, fingerprints). Tests harness reads this envelope (binding contract from upstream test-plan §3); verdict is part of the bind.
- **obs ↔ core (Verdict type definition):** `conductor-core::Verdict { Pass, Fail, CalibrationRegion }` is the single source of truth for the enum. `conductor-verify`'s classification module consumes and returns this type (no new cross-seam edge).
- **obs ↔ report generation (Epoch 6 run-report envelope seam):** The verdict classification output is consumed by `conductor-report::generate()` at the seam; the verdict field is serialized to the JSONL envelope (obs owns the field name + enum spelling; report owns the serialization).

## Acceptance criteria contributions
- **(obs) Verdict field present on every scenario-result JSONL line:** verdict enum value (Pass | Fail | CalibrationRegion) is a required field, never null. CI gate: `cargo-nextest` output + `logs/agent-latest.jsonl` scan validates all runs.db / journal rows carry the field.
- **(obs) Deterministic verdict classification:** same inputs (scenario, seed, observed values) produce the same verdict across repeated runs on the same surface (CLI) and across surfaces (CLI vs Tauri for same seed). CI gate: `runs.db` parity check on matched `(scenario, seed)` rows from two surfaces asserts identical verdict/state.
- **(obs) CalibrationRegion outcomes logged with delta:** when a model-interpretive assertion routes to CalibrationRegion, the structured log event MUST carry `delta_observed` + `delta_expected` fields so the human-facing report can render the delta. CI gate: `cargo-nextest` integration test on severity-lifecycle / P-008 weighting scenario asserts delta fields present on CalibrationRegion rows.
- **(obs) Zero-unlogged panics in verdict classification:** if verdict classification panics (e.g., on unexpected state enum variant), the panic MUST be captured by `std::panic::set_hook()` and logged via `tracing::error!(...)`. CI gate: zero-unlogged-panics CI gate scans logs for unstructured `^thread.*panicked` lines (must be zero).

## Relevant amendment history
- **2026-06-16-emission-journal-writer** (§3 Harness Contract / Log format JSON schema): added `read_back_observed_at` field to the Run-report envelope (ISO-8601, null until read-back). Verdict classification happens *after* read-back, so the verdict field is populated in the same envelope as `read_back_observed_at`. This chunk's JSONL output MUST include both fields (no regression).
- **2026-06-15-log-error-boundary-redaction** (§6 Log Coverage, §11 Anti-Patterns): redaction model clarified — host-file paths are masked; `Display`-not-`Debug` at the `anyhow` edge keeps struct names out. If verdict classification errors (e.g., panics on malformed read-back), the error message must be sanitized via `anyhow::Error` display (not debug-dumped) before logging.
