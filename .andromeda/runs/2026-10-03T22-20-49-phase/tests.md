# tests extract

## Relevance
relevant. The chunk's work sits mostly in the test tier: a feature-gated live leg, harvest-tier graders over digest-pinned evidence, a fifth pinned MCP tool that the stub and the contract test must carry, and a `std::time` resolve-window stamp.

## Constraints
- The live round is an operator/local-gate leg only and never a CI stage. It is a CARGO-FEATURE-GATED target behind `live-pulse`, and the plan requires it to stay out of default `nextest` / `clippy` / release builds and out of both runner-portability runs. Because a gated file is invisible to the default lint pass, the leg owes its own lint line, `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` (per test-plan §9 Live-Pulse scenarios; §11 CI).
- The plan places the round's grades at the HARVEST tier, in default-suite tests over digest-pinned evidence (`p075_round_assertion_*` in `lifecycle_harvest` / `delegated_timing_harvest`). The live leg prints only integers, booleans and closed words on stdout, and mints no `agent-run` verb or selector (per test-plan §2 Agent-runnable invariants, `p075_round_live` clause). A committed capture is EVIDENCE: it is read-only, anchored on `CARGO_MANIFEST_DIR`, and checked against its sha256 digest pin before any grade reads it. It is never a fixture copied elsewhere (per test-plan §7 Self-bootstrapping requirement).
- §1 Coverage triggers (contract-test) and §5 (Pinned MCP contract manifest) require a contract test. It asserts that the preflight's required-tool set matches the pinned `contracts/` manifest, and that a mismatch yields `Blocked`. Pinning `retrieve_incident_events` therefore obliges the stub's tool list and that assertion to carry five names. Whether the stub's default list, `READBACK_TOOLS` and `readback_shape_witness` already agree at five is research's question (per test-plan §1 Coverage triggers; §5 Cross-module patterns).
- §5 sets STUB ITEM-KEY FIDELITY: a stub response must use the keys the live sidecar emits, or a reader goes silently empty against a populated live corpus. Any stub arm for the new tool must mirror S2's response shape: `incident_id`, the `events[]` of `event_kind` / `occurred_unix_nano`, `total` and `truncated`. Malformed input must degrade to typed values and never panic (per test-plan §5 `query_incident_list` and `retrieve_telemetry_slice / retrieve_report` bullets; §1 Coverage triggers, Vector 4(d)).
- The resolve call's request→response window is a ground-truth stamp. The plan requires it from `std::time` (SystemTime/Instant), never tokio's virtual clock. The virtual clock is for scheduling only (per test-plan §8 What to mock, Time; §11 Test Data; §11 Universal "NEVER use real time without injection").
- The suite must be green under BOTH `cargo nextest` and `cargo test -p <crate>`, with zero nextest retries. A runner-dependent result is a determinism defect (per test-plan §4 Framework; §10 Zero-flakiness budget).
- §9 Scoped mutation audit states that mutation runs happen at the epoch-boundary audit ONLY, never per chunk. This chunk therefore owes no `mutation-gate.py` run (per test-plan §9 Scoped mutation audit).

## Patterns to follow
- The three-tier shape §5 records for `mark_incident_resolved` (per test-plan §5 `mark_incident_resolved` bullet):
  - the applied and declined arms are stub-proven at the integration tier through `ReadbackClient` in `conductor-verify/tests/readback.rs`;
  - the live write grades at the harvest tier on a pinned capture driven by a feature-gated operator leg.
  The natural template for `retrieve_incident_events` puts its happy path and its unknown-id `-32603` error arm, read as a typed error, at the stub tier, and assertion 7 at the harvest tier. Whether `ReadbackClient` has any method for this tool is research's question.
- The real-model harvest's digest-pin arms (per test-plan §6 Fingerprint-storm, real-model interpretation leg):
  - synthetic arms for every outcome (PASS / FAIL / UNGRADED-absent);
  - a one-byte tamper arm showing the pin can fail, naming the file and never the text;
  - a source arm holding that no capture text sits in test source.
- The stub-plus-raw-result discipline (per test-plan §5 Boundary types covered, Module ↔ external; §8 Mocking libraries):
  - the hand-rolled `stub_pulse_mcp`, either in-process duplex or spawned as a child;
  - a JSON-schema assertion on the RAW tool result;
  - DI of the transport, never a monkey-patch.
- The live leg's FULL FIRING FORM belongs to the leg (per test-plan §6 Drivers per surface, the driven arm's firing-form clause; §11 E2E quiet-window clause):
  - a `PATH` that resolves `andromeda-pulse-mcp` by fixed name;
  - `ANDROMEDA_PULSE_MCP_ENABLED` and `ANDROMEDA_PULSE_L4_DETERMINISTIC`;
  - `ANDROMEDA_PULSE_DATA_DIR` equal to the live Pulse's data dir.

  A bare invocation is `Blocked` in about 2 ms with every tool `absent`, which at row level looks exactly like a SUT-side failure.
- The `stub-server`-gated `preflight_spawn.rs` is the in-repo precedent for a gated target (per test-plan §9 Live-Pulse scenarios).

## Anti-patterns to avoid
- NEVER `sleep(N)` inside a test to synchronise. The AFTER read waits on the resolve response, not on a timer. A firing-form quiet window between legs or drives is allowed only where it waits out a documented Pulse dedupe (per test-plan §11 E2E).
- NEVER run the live-Pulse leg as a CI gate, and NEVER fake Pulse's reaction as a CI verdict. The stub proves wiring only (per test-plan §11 CI; §11 Test Strategy).
- NEVER add retries or a re-drive-for-pass policy. A flake is a determinism break, fixed at its cause (per test-plan §10 Zero-flakiness budget; §11 Quality).

## Contract bindings
- **tests ↔ arch:** `contracts/mcp-contract.toml` `required_tools` gates the preflight's missing-tool precondition. §5 binds the stub and the contract test to that manifest. The five-named-preconditions count is an arch/security invariant the test must not grow (per test-plan §5 Cross-module patterns, MCP `initialize` preflight readiness gate).
- **tests ↔ security:** the new tool is a new input class on the sidecar-stdout boundary. §1 Vector 4 requires the bounded JSON-RPC decode, and a malformed response becomes a typed `Blocked`/`Fail`, never a panic. Committed evidence and live-leg stdout carry no host paths (per test-plan §1 Coverage triggers; §3 Status endpoint shape, sanitization sentence).
- **tests ↔ obs:** the resolve window and journal stamps come from `std::time`, so journal-relative SLO math holds (per test-plan §3 `logs`; §3 Log format).
- **Master-currency flag for wrap.** test-plan §2 currently describes `p075_round_live` as reading back "through the four registered MCP tools". §1's ipc-internal surface also enumerates four raw tool results (`query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`). §5 lists no `retrieve_incident_events` bullet. Once the fifth tool is pinned these become stale target-state prose and a wrap amendment candidate. They are not edited here.

## Acceptance criteria contributions
- `cargo nextest run -p conductor-verify --profile ci` and `cargo nextest run -p conductor-run --profile ci` pass. The run must include the new stub-tier `retrieve_incident_events` arms (happy path and unknown-id typed error) and the round's new `p075_round_assertion_*` harvest tests, including a tamper arm proving each new digest pin can fail. Both crates are also green under `cargo test -p <crate>` (per test-plan §4 Framework; §7 Self-bootstrapping requirement).
- The preflight contract test asserts the required-tool set equals the pinned manifest's five names. A stub whose `tools/list` omits `retrieve_incident_events` yields `Blocked` on the EXISTING missing-tool precondition (per test-plan §1 Coverage triggers, contract-test; §5 Pinned MCP contract manifest).
- `cargo clippy --workspace --all-targets -- -D warnings` and `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` are both green (per test-plan §9 Live-Pulse scenarios; §9 Pipeline structure, Lint).
- Workspace line coverage stays ≥ 60% under `cargo llvm-cov nextest --fail-under-lines 60` (per test-plan §10 Coverage thresholds).
