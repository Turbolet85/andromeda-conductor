# tests extract

## Relevance
relevant — a live operator-gated round graded at the harvest tier, plus a harness-capture change (the span-landing subdir CARRY) to an operator-gated `live-pulse` target.

## Constraints
- A live-Pulse round is an operator/local gate only and must never be a CI gate. Its drivers are a `live-pulse`-feature-gated `conductor-run` test target invoked directly, or the `agent-run run --live` stage. Because a gated file is invisible to the default lint pass, any new or edited gated target owes its own lint line, `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` (per test-plan §9 Live-Pulse scenarios; §11 CI).
- A live claim grades hard PASS/FAIL at the HARVEST tier: a default-suite `conductor-run/tests/*_harvest.rs` reads the leg's pinned capture. The precedent set is storm, baseline, restart, pii, connection, severity and lifecycle (per test-plan §6 Fingerprint-storm / Severity-lifecycle; §5 `mark_incident_resolved`). Whether existing harvests already grade assertions 1–2 against a new binary, and whether any harvest grades the four budget metrics, is research's question. The plan names no grader for `hue_update_ms`, `discovery_ms`, `render_ms` or `counter_refresh_ms`.
- The `mark_incident_resolved` claim is runtime-STATE fidelity only, PROVEN-BY-LIVENESS (the incident leaves the active set), and never payload fidelity. `probe_resolve_lifecycle` must stay out of `execute_scenario` and every run path. The DECLINED arm is stub-only, permanently. Readers key on `incident_id` (per test-plan §5 `mark_incident_resolved` and `query_incident_list`).
- Every test-binary reader of `CONDUCTOR_RUNS_DIR` or the `ANDROMEDA_PULSE_DATA_DIR` value must resolve through the shared `conductor-run/tests/capture_paths` guard. That guard's mandated negative test is the default-suite target `capture_paths_guard` (per test-plan §1 Test Scope Summary, the fifth path-handle class). A relocated span-landing capture path must keep resolving through it.
- A live leg that waits out a SUT precondition does so as a firing-form wait BETWEEN drives or legs: the 150 s quiet window, or the span-landing 180 s window (Pulse's `(kind, scope, scope_id)` dedupe). Nothing inside a test may sleep to synchronise (per test-plan §11 E2E).
- Reported states (`Blocked`, `ManualCheck`, `KnownResidual`, `CalibrationRegion`) are not non-zero exits; only a hard `Fail` is (per test-plan §3 `run` exit semantics; §11 E2E). How the round's UNGRADED (absent-sample) outcome maps onto these states is not stated by the plan. It is P3's to settle, and it must never read as a pass.
- Pulse's reaction is never faked as a CI verdict, because the stub canary proves wiring only (per test-plan §8 What NOT to mock; §11 Test Strategy).

## Patterns to follow
- The `--live` firing form (per test-plan §3 `run`, `--live` selector):
  - a leading `conductor preconditions` probe that refuses at exit 1 before any leg fires;
  - each leg's `logs/agent-latest.jsonl` frozen to `runs/live-suite/{leg}.jsonl`, because that sink truncates per invocation;
  - the real-model arm's NON-RECURSIVE removal of NAMED capture files (`runs/live-suite/{rm.jsonl,rm-capture.txt,rm-capture.err}`) before its leg. This is the in-plan precedent for the CARRY's pre-drive-A clear.
- The `span_landing_live` witness shape (per test-plan §2 agent-runnable invariants):
  - two same-seed `conductor run` drives against one live Pulse, with a 180 s quiet window between them;
  - graded on Pulse's own log;
  - prints one integers-only `span-landing: PASS|FAIL …` line;
  - mints no `agent-run` verb or selector.
- Committed live-capture EVIDENCE under a chunk's `evidence/` tree (per test-plan §7 Self-bootstrapping; §6 real-model leg):
  - read-only and `CARGO_MANIFEST_DIR`-anchored;
  - sha256 digest-pinned over LF-normalized content, and graded only after the digest matches;
  - a tamper arm proves the pin can fail;
  - no capture text sits in test source;
  - never copied elsewhere as a fixture.

  The plan states this for the real-model harvest. Whether a new P-075 harvest adopts it is a P4 choice.
- Harvest assertions are pinned on Pulse's verbatim log lines. A mis-pairing gets its own negative test (per test-plan §6 Severity-lifecycle, where pairing a creation with the canary's own resolution is pinned as a negative). Attribute every sample to the scenario's emission, never to the preflight canary.

## Anti-patterns to avoid
- NEVER wire the live round, or any `live-pulse` target, into `.github/workflows/ci.yml` as a gate (per test-plan §11 CI; §9).
- NEVER re-run to obtain a pass, and never add nextest `retries`. A run-to-run difference is a finding to record with its cause, consistent with the round's no-re-drive posture (per test-plan §10 Zero-flakiness budget; §11 Quality). The §9 note applies here: a live leg over an unchanged tree can grade differently by SUT uptime, so record the verdict as conditional rather than re-driving.
- NEVER `sleep(N)` inside a test to synchronise on a Pulse-side event (per test-plan §11 E2E).

## Contract bindings
- **tests ↔ obs:** the `--live` capture freezes the self-obs sink `logs/agent-latest.jsonl` per leg. Its base fields and its event/span-lifecycle line variants are obs-plan §3's (per test-plan §3 Log format). A grep over span names must allow for child lines.
- **tests ↔ security:**
  - path handles: the `capture_paths` guards (per test-plan §1);
  - the harness-side clear: non-recursive and by pattern inside a known dir (`.claude/rules/security.md` 2026-09-06; test-plan §3 `run`);
  - the sidecar spawned by fixed NAME through the inherited `PATH`, with `ANDROMEDA_PULSE_DATA_DIR` passed via `.env(...)` only (per test-plan §11 Mocking).
- **tests ↔ arch:** `runs/live-suite/` and its second writer are registered under arch §Occupied Resources. Moving the span pair to its own subdir changes that registry row, which is arch's to amend.
- **tests ↔ contracts:**
  - `contracts/pulse-p025-measurement-contract.md` defines how assertion 3 is graded;
  - `contracts/pulse-run-contract.toml` floors the `boot` preflight budgets (`warmup_ms`, `min_canary_poll_seconds`; per test-plan §3 `boot`).

## Acceptance criteria contributions
- `cargo nextest run --workspace --profile ci` is green, and includes every new or edited default-suite harvest grader the round's graded test ids name (per test-plan §3 `run`; §6 harvest tier).
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` exits 0 over the edited gated targets, `span_landing_live.rs` included (per test-plan §9 Live-Pulse scenarios).
- `capture_paths_guard` stays green under nextest. The span-landing witness reads `span-{a,b}.jsonl` from its own harness-owned subdir, which the shells clear by named-file, non-recursive removal before drive A, so a stale pair can never be graded (per test-plan §1 Test Scope Summary; §3 `run`).
- No `.github/workflows/ci.yml` step invokes the round or any `live-pulse` target, and the workspace line-coverage gate `--fail-under-lines 60` holds (per test-plan §11 CI; §10 Coverage thresholds).
