# tests extract

## Relevance
Partial — structured logging stack establishes log format and service-identity fields that bind the harness log-format contract (per §3); the `std::panic::set_hook` zero-unlogged-panics invariant adds a test constraint, but log redaction + error sanitization belong to the next chunk.

## Constraints
1. Per §3 Log format — all self-obs emitted as structured JSONL via `tracing-subscriber` 0.3.x `format::Json`, one object per line; required fields include service-identity + `run_id` on every line (§3 mandatory field set).
2. Per §3 Log format binding — the harness greps the journal file for assertions; format break = harness break (test-data-bootstrap-wire bootstrap phase derives this contract).
3. Per §2 Agent-runnable invariants — no multi-line JSON; every line parseable with `jq -c` or `serde_json::from_str::<RunReportEnvelope>(line)` (determinism requirement).
4. Per §3 Test data bootstrap — wall-clock stamps from `std::time::SystemTime`/`Instant` only, never tokio's virtual clock (security anti-pattern: "Writing journal/report wall-clock stamps from tokio's virtual clock — rejected").
5. Per §4 Unit test strategy — `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` for determinism (no real `setTimeout`); tracing JSON flattening must serialize everything onto one line, no multi-line escape.
6. Per §11 (anti-patterns section, §Unit + §Integration) — log-format unit tests assert the tracing subscriber initialization produces valid JSON per line; negative test on the journal asserts no absolute host paths or internal struct names leak (security anti-pattern).

## Patterns to follow
1. Per §7 Test Data & Fixtures — rstest `#[fixture]` for the seeded generator; per-test isolation via `rusqlite::Connection::open_in_memory()` for unit tests and `assert_fs::TempDir` for cross-process (tracing writes to the bounded per-run journal).
2. Per §3 Status endpoint shape — the Run-report envelope JSON (shared by `runs.db` row, JSONL journal, Markdown report) includes identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `journal_emitted_at`, `read_back_observed_at`, `fingerprints`); service-identity + `run_id` on every line satisfies the correlation key invariant.
3. Per §3 Test harness contract (`logs` command) — the per-run `<run_id>.jsonl` under `runs/` is "the left side of every SLO check" (Creator Brief); agent reads via `jq -e '<predicate>' runs/<run_id>.jsonl` or `serde_json::from_str`, so every line must be independently parseable.

## Anti-patterns to avoid
1. Per §11 Test Anti-Patterns — "Emitting structured logs without ensuring every line is independently parseable JSON (no multi-line escapes, no partial JSON)" — tracing subscriber flattening is mandatory (§3); no exceptions.
2. Per §11 + scope (Redaction is the NEXT chunk) — do NOT implement field-allowlist redaction, host-path scrubbing, or `anyhow` edge sanitization in this chunk; redaction layer composes on top after the stack stands up (scope explicitly defers these to the next chunk).
3. Per scope + §1 test-scope entity `conductor-emit` note — do NOT conflate self-obs (this chunk) with OTLP emission to Pulse (Epoch 3 separate); no OTel SDK in this chunk; structured logs are ground truth, not a side channel to OTLP.

## Contract bindings
Obs ↔ tests harness: §3 Log format (NDJSON + required fields) is the binding contract; the harness greps the file for assertions (test-data-bootstrap-wire phase derives this). §3 Status endpoint shape (Run-report envelope) defines the identity fields (`run_id`, `service.name`) that must appear on every line. §2 Agent-runnable invariants mandate line-by-line parseability (no multi-line JSON).

## Acceptance criteria contributions
1. "(tests) `cargo nextest run -p conductor-<seam>` passes unit tests for the tracing subscriber init surface (call from CLI and Tauri shells, identical semantics)."
2. "(tests) Log-format negative test: all self-obs JSONL lines are independently parseable via `jq -c` / `serde_json::from_str::<RunReportEnvelope>`; no multi-line escapes, no host paths / internal struct names."
3. "(tests) `std::panic::set_hook` capture test: a panic raised in a test subprocess is routed to the structured log stream (one JSON line in `<run_id>.jsonl`) and does NOT crash unlogged."
4. "(tests) Coverage: service-identity + `run_id` present on every emitted line (property test over a deterministic scenario + seeded timeline generator, asserting 100% line coverage of required fields)."

## Relevant amendment history
(none)