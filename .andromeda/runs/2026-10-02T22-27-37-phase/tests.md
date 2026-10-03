# tests extract

## Relevance
relevant. The chunk adds a new MCP read-back surface (`retrieve_incident_events`), a fifth required-tool pin, an operator-gated live leg and harvest-tier grades over evidence pinned by digest. All of these are test-plan surfaces.

## Constraints
- **The live round never runs in CI.** Per test-plan §9 *Live-Pulse scenarios*, the round is an operator/local gate: a test file gated by the `live-pulse` cargo feature, invoked directly. §9 names `conductor-run/tests/p075_round_live.rs` as a member of that feature's target set. Such a gated file is invisible to the default lint pass, so §9 requires it to carry its own lint line (`cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`). §11 *CI* bans running live-Pulse scenarios as a CI gate.
- **Grades sit at the harvest tier.** Per test-plan §2 *Agent-runnable invariants* (the `p075_round_live` clause) and §6 *Fingerprint-storm*, P-075 grades run in the default suite, over evidence pinned by digest. They are not graded inside the live leg. The leg prints only integers, booleans and closed words on stdout, and mints no `agent-run` verb or selector.
- **Committed capture evidence follows the §7 rules.** Per test-plan §7 *Self-bootstrapping requirement*, it is read-only and anchored on `CARGO_MANIFEST_DIR`. Each file is checked against its sha256 digest pin before any grade reads it. A capture is evidence, never a fixture copied elsewhere.
- **The required-tool set is a contract test.** Per test-plan §1 *Coverage triggers* (contract-test) and §5 *Pinned MCP contract manifest*, the preflight's required-tool set must match the `contracts/` manifest, and a mismatch ⇒ `Blocked`. Pinning a fifth name therefore binds the CI stub legs: the stub must advertise the tool, or the preflight blocks. Whether `stub_pulse_mcp` lists `retrieve_incident_events`, and which tests hold the four-name set, are research's questions.
- **Stub keys must match the live sidecar.** Per test-plan §5 *query_incident_list* (STUB ITEM-KEY FIDELITY), any stub response for the new tool must use the keys the live sidecar emits. Here those are `incident_id`, `events[].event_kind`, `events[].occurred_unix_nano`, `total` and `truncated`, to be verified at S's source. A stub keyed differently stays green on stub legs and silently empties the read live.
- **Pulse's reaction is never faked as a CI verdict.** Per test-plan §11 *Test Strategy* and §8 *What NOT to mock*, the stub proves wiring only. Assertion 7's real grade comes from the live capture, graded at the harvest tier.
- **Time.**
  - Ground-truth stamps use `std::time` only (test-plan §8 *What to mock*, Time; §11 *Universal*, "NEVER use real time without injection").
  - Any synthetic harvest arm grading the resolve window takes its stamps as injected values, never real elapsed time.
  - Zero-flakiness holds: no nextest `retries` (§10 *Zero-flakiness budget*).
  - Suites must be green under both nextest and `cargo test -p` (§4 *Framework*).

## Patterns to follow
- **The three-tier `mark_incident_resolved` model** (test-plan §5 *mark_incident_resolved*). Arms proven against the stub at the integration tier go through `ReadbackClient` (`conductor-verify/tests/readback.rs`):
  - the applied arm;
  - the declined arm (`VerifyError::JsonRpc`, Display renders the code only).

  The live write grades at the harvest tier on the pinned capture (`lifecycle_harvest`). The new tool should copy this split:
  - an applied/empty/populated read against the stub;
  - an unknown-id `-32603` error mapped to a typed error, never a panic;
  - the live read graded over pinned evidence.
- **The prior round's harvest tests** (test-plan §5 and §6). The prior round is graded by `p075_round_assertion_*` tests in `lifecycle_harvest` and `delegated_timing_harvest`, over a digest-pinned capture that commits only booleans. A fresh round adds its own ids over its own evidence. The prior ids stay pinned to their own evidence (test-plan §7: evidence is read-only).
- **The evidence-pin discipline** (test-plan §6 *Real-model interpretation leg*):
  - a sha256 pin over LF-normalized content, checked before grading;
  - a one-byte tamper arm proving the pin can fail, naming the file and never the text;
  - a source arm holding that no capture text sits in test source.
- **Synthetic arms for every outcome** (test-plan §6, real-model leg precedent). Assertion 7's grader should carry synthetic arms for each failing shape:
  - a `resolved` event before the call;
  - a last event that is not `resolved`;
  - `occurred_unix_nano` outside the window;
  - an out-of-set `event_kind`;
  - an absent sample read as UNGRADED, never met.
- **Firing-form waits sit between drives, never inside a test** (test-plan §11 *E2E*, the span-landing / `--live` quiet-window clause). Any wait the round needs for Pulse's dedupe window is part of the firing form.

## Anti-patterns to avoid
- **No sleeping to synchronise inside a test** (test-plan §11 *E2E*). Wait on an explicit signal, such as the resolve call's response or a read-back result.
- **No live leg as a CI gate, and no retry-once or re-drive policy** (test-plan §11 *CI*; §11 *Quality*; §10 *Zero-flakiness budget*).
- **No monkey-patching of `andromeda-pulse-mcp`** (test-plan §11 *Mocking*). Extend the hand-rolled stub behind the JSON-RPC stdio client seam (DI of transport).

## Contract bindings
- **tests ↔ arch §Standard Contracts (pinned MCP contract manifest).** Per test-plan §3 *boot* and §5, the preflight's missing-tool precondition reads `contracts/mcp-contract.toml`'s required-tool set. Adding the fifth name changes what every preflight (stub and live) must see to reach `ready:true`.
- **tests ↔ security (capture ingest).** The leg reads `CONDUCTOR_RUNS_DIR` and `ANDROMEDA_PULSE_DATA_DIR` as a test binary, so it falls in test-plan §1 *Coverage triggers*' fifth per-reader class. That class's guard is the shared `capture_paths` module, and `capture_paths_guard` is its negative test. Whatever enters `evidence/` is governed by security-plan's scrub rules. The prior leg committed only booleans.
- **tests ↔ obs (harness).** Per test-plan §2 and §3, the leg adds no `agent-run` verb, port or env handle. The 5-command count, the `status` disk read and the envelope/JSONL shapes stay untouched, so §3 ↔ obs-plan §3 still agree.
- **The test-plan's own enumeration will go stale.** Test-plan §1 *Surfaces under test* (ipc-internal, MCP read-back) and §5 *Boundary types* enumerate the four raw tool results. A fifth pinned tool makes that enumeration stale. It is a candidate amendment for wrap to raise, not a change for implement.

## Acceptance criteria contributions
- **Stub-tier read-back.** `cargo nextest run -p conductor-verify --profile ci` passes with arms proven against the stub for `retrieve_incident_events`:
  - a populated read with live-fidelity keys;
  - an empty `events: []`;
  - an unknown id mapped to a typed JSON-RPC error, never a panic;
  - the required-tool contract test held against the five-name manifest, with a missing fifth tool ⇒ `Blocked` on the missing-tool precondition.

  The same suite is green under `cargo test -p conductor-verify` (per test-plan §5 *Pinned MCP contract manifest* / *mark_incident_resolved*; §4 *Framework*).
- **Harvest-tier grades.** `cargo nextest run -p conductor-run --profile ci` passes with seven new graded test ids over this chunk's digest-pinned `evidence/`, plus:
  - a tamper arm;
  - synthetic arms for each assertion-7 failing shape and for an absent sample (UNGRADED, never met).

  The prior six ids are unchanged and still green over their own evidence (per test-plan §6 *Fingerprint-storm* and real-model leg; §7 *Self-bootstrapping requirement*).
- **Lint and default suite.** Both of these are green:
  - `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`
  - `cargo clippy --workspace --all-targets -- -D warnings`

  The live leg stays absent from the default `cargo nextest run --workspace --profile ci`, which passes with no retries configured (per test-plan §9 *Live-Pulse scenarios*; §10 *Zero-flakiness budget*).
- **Coverage.** The workspace line coverage gate stays at `--fail-under-lines 60` (per test-plan §10 *Coverage thresholds*).
