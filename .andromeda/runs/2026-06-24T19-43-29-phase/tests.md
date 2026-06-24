# tests extract

## Relevance
relevant — the chunk touches the CLI's error handling and agent-mode output, both testable surfaces with clear test contracts.

## Constraints
- Per test-plan §3 Test Harness Contract: `logs` command must read per-run `<run_id>.jsonl` (ground truth) and sanitized stderr from CLI subprocess; journal format is NDJSON with wall-clock stamps from `std::time::SystemTime`/`Instant`, NOT virtual clock (security anti-pattern).
- Per test-plan §3: artifact sanitization invariant — no absolute host paths, no internal struct names; enforced by golden snapshot + negative test on journal.
- Per test-plan §3: stderr capture via `assert_cmd` `.get_output().stderr`; agent parses via jq or serde.
- Per scope.md: sanitized error format = two-part `error: <short>` + contextual + `hint: <fix>` with zero host-path/struct-name/stack-trace leakage (except under `--debug`/`-v`).
- Per test-plan §2 Agent-runnable invariants: every layer produces machine-parseable output; no interactive prompts (stdin closed to prove "never blocked").
- Per test-plan §10: zero-flakiness budget — flaky tests are quarantined immediately; `retries` forbidden; determinism enforced upstream via seeded generators + virtual-clock mocking.

## Patterns to follow
- Unit test per-seam error mapping via thiserror typed enums; assert on public API envelope contract, not implementation details (test-plan §4 Unit).
- Integration test via assert_cmd + assert_fs sandbox: capture stderr `.get_output().stderr`, assert error output format + absence of leaks via regex (no host_path / struct_name / stack_trace).
- E2E critical path (cli surface): assert_cmd with stdin closed (proves "never blocks on TTY"); verify exit code + sanitized stderr label presence + `runs.db` row + JSONL journal written (test-plan §6 cli scenario).
- Golden test on the envelope shape + sanitization via insta with field-allowlist redaction (test-plan §7, amendment 2026-06-15: exact-string unit serialization goldens, insta for E2E journal redaction).
- Per test-plan §2: `#[tokio::test(flavor="current_thread", start_paused=true)]` for time-sensitive paths + `tokio::time::advance` (no real elapsed-time assertions in virtual-clock context).

## Anti-patterns to avoid
- NEVER stamp emission journal/report from tokio's virtual clock (test-plan §11 Test Data, security anti-pattern); golden test must catch paused-clock leak.
- NEVER use `sleep(N)` for synchronization; wait for explicit signal (envelope state field, JSONL journal line, exit code) (test-plan §11 E2E).
- NEVER assert on implementation details (private fns, internal struct fields); test public seam API + observable envelope/verdict/state output (test-plan §11 Unit).
- NEVER lower coverage threshold to pass build; raise quality, not the bar (test-plan §11 Quality).

## Contract bindings
- obs §3 ↔ tests: dual-sink wiring (`--agent-mode` forces JSON-only file sink `logs/agent-latest.jsonl`); `CONDUCTOR_AGENT_MODE=1` env handle visible to subscriber; self-obs stream (timestamp_ms/level/target/service-identity/run_id) is SEPARATE from emission journal (amendment 2026-06-15 §7; test-plan §3 Log format §188).
- security-plan §Error Handling ↔ tests: sanitization at anyhow edge (no stack traces/host paths/struct names to stderr or artifacts); negative test asserts artifact-sanitization invariant.
- design-system §cli + layout-templates §263/268 ↔ tests: error format (error:/hint: labels), NO colorization when piped, `NO_COLOR`/`TERM=dumb` honored, stdout/stderr separation (all asserted via assert_cmd output capture).
- test-plan §3 ↔ tests: assert_cmd captures stderr; status = `runs.db` row / JSONL journal (no HTTP endpoint); log-format binding = NDJSON + required identity fields (test-plan §3 source of truth; obs derives from here).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-cli` passes for error-handling + sanitization unit tests (thiserror enum mapping, anyhow-edge redaction).
- (tests) assert_cmd E2E: CLI subprocess with stdin closed emits sanitized `error:`/`hint:` on stderr; zero host-path/struct-name leakage asserted via regex; exit code non-zero on failure.
- (tests) `--agent-mode` flag present + forces `CONDUCTOR_AGENT_MODE=1`; routes self-obs JSON to `logs/agent-latest.jsonl`; agent parses via jq -e or serde_json.
- (tests) Golden test on error envelope / log envelope shape (insta with redaction); JSONL journal is distinct from self-obs stream with separate schemas (amendment 2026-06-15 note).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Log format): clarified that the `tracing` self-observation stream (`logs/agent-latest.jsonl`; base fields incl service-identity/run_id) is SEPARATE from the per-run emission journal (`runs/<run_id>.jsonl`, ground-truth envelope) — two schemas must not be conflated. Critical boundary for this chunk's scope: `--agent-mode` wires the self-obs sink, NOT the emission journal.
- **2026-06-16-emission-journal-writer** (§4 Unit): canonical serialization line shape locked via exact-string `assert_eq!` at unit level (matching `verdict.rs`/`report_state.rs`/`scenario.rs` pattern); insta reserved for E2E journal-golden mechanism with redaction. Applies to error-handling surface unit tests (error enum serialization golden).
