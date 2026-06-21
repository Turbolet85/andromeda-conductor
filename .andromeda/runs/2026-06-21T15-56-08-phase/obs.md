# obs extract

## Relevance
Partial — SLO timing model is core to obs tier (Minimal) + critical paths §4; expected-outcome evaluation and timing measurement are obs instrumentation concerns, but the comparison logic itself (the "kinds" list) is independent of obs domain.

## Constraints
- Per §1 Obs Scope Summary: Conductor self-obs is Minimal tier, structured tracing logs only (JSONL via `tracing-subscriber`), no OTel SDK for self-observation (creator-explicit anti-pattern §11 Telemetry Strategy)
- Per §3 Observability Harness Contract: `latency_ms` computed as `read_back_observed_at − journal_emitted_at` (wall-clock from `std::time::SystemTime`, never tokio virtual clock §11 Project-specific bans), ISO-8601 format
- Per §4 Span / Trace Coverage: the verification path (`verify.readback*` spans) must emit span attributes `latency_ms` on each MCP call; spans close on MCP response receipt; no W3C `traceparent` (correlation is `run_id` field only)
- Per §10 SLO Invariants: performance budgets are tier-closed (`<5s` / `<20s` / `<90s`); SLO enforcement is JSON field assertion at report-generation time, not a histogram instrument; zero-unlogged-panics invariant applies (all panics captured via `std::panic::set_hook()` + logged via `tracing::error!`)
- Per §6 Log Coverage: the run-report envelope's `slo_tier` field is a closed enum; additional scenario-specific fields populate the JSONL per critical path (e.g., `bypass_triggered`, `lifecycle_phase`, `degraded_mode_response`); all records must carry `journal_emitted_at`, `read_back_observed_at`, `run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`

## Patterns to follow
- Instrument the verification seam (`verify.readback`, `verify.readback_*`) with `#[tracing::instrument]` spans (§4 Must-trace paths); each span carries `latency_ms` attribute measured wall-clock, ISO-8601 stamps for `journal_emitted_at`/`read_back_observed_at` on every log line
- Emit boundary-call logs for MCP readback: method name + latency_ms + error (if any) at `info` level (§6 Boundary-call wrappers)
- Model expected outcomes declaratively, serde-deserialized, with garde bounds-checking on load (established config discipline from chunk `verdict-assertion-policy-split` §3)
- Tag every scenario-result record with `run_id` (the correlation key, no W3C trace context; see §3 Correlation); parity across surfaces is `runs.db` envelope comparison (same seed ⇒ same verdict/state), not trace propagation

## Anti-patterns to avoid
- NEVER use tokio's virtual clock (`tokio::time::Instant`) for journal timestamps — wall-clock only (§11 Project-specific bans); this affects the `read_back_observed_at` stamp computation
- NEVER introduce an OTel SDK + exporter for self-observation (§11 Telemetry Strategy) — breaks `current_thread` determinism + pollutes the PRODUCT OTLP stream
- NEVER skip the `run_id` field or use W3C `traceparent` for correlation (§11 Spans / Traces, §3 Correlation) — Conductor is a single-process deterministic harness, not a distributed system
- NEVER leak absolute host paths or internal struct names in logs / run-report / runs.db (§11 Logs, §6 Log conformance check) — the redaction layer masks absolute host-file paths (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`, backtrace file paths) → `<redacted>`; field-name allowlist + `Display`-not-`Debug` at `anyhow` edge keeps struct names out

## Contract bindings
- **obs ↔ tests harness** (§3 Observability Harness Contract / Log format JSON schema): Run-report envelope is the binding contract from test-plan §3 (status-shape + polled fields); the 11-field JSONL schema (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) is test-owner's source of truth; amendment history 2026-06-16 re-aligned obs-plan §3 to the owner (added `read_back_observed_at`)
- **obs ↔ core config model** (scope open seam question): the declarative `expected` block lives in `conductor-core` (Scenario config model, serde + garde) per star-topology invariant; verification logic + evaluator live in `conductor-verify`

## Acceptance criteria contributions
- (obs) SLO timing measured journal-relative only: `latency_ms = read_back_observed_at − journal_emitted_at` wall-clock (std::time, never tokio virtual clock); both timestamps ISO-8601 on every log line.
- (obs) Run-report envelope all 11 fields present on every scenario-result record; `slo_tier` is closed enum (`<5s` / `<20s` / `<90s`); verdict-assignment is deterministic (same scenario + same observed + same journal timing ⇒ identical verdict/state across surfaces).
- (obs) Verify boundary calls instrumented with `#[tracing::instrument]` spans; method name + latency_ms + error logged at `info` level per §6 Boundary-call wrappers; span attributes include `mcp_method`, `latency_ms`; spans close on MCP response receipt.
- (obs) No host-path leak through redaction edge: absolute file paths → `<redacted>`; struct names out via field-name allowlist + Display-not-Debug; allowlisted `target` module path preserved.

## Relevant amendment history
- **2026-06-16-emission-journal-writer**: added `read_back_observed_at` (ISO-8601, null until read-back) as 2nd envelope field; latency_ms stays `read_back_observed_at − journal_emitted_at`. Reconciled obs-plan §3 to test-plan owner (the binding contract); run-report envelope now 11 fields (journal_emitted_at, read_back_observed_at + 9 others). This chunk's expected-outcome + SLO timing model **depends on** the `read_back_observed_at` field being present in the envelope for latency computation.
