# tests extract

## Relevance
Relevant — this chunk is squarely a tests-domain chunk: it enforces the §4 runner-portability gate, the §11 Integration process-global-singleton ban, and the §10 zero-flakiness rule.

## Constraints
- `test-plan` §4 requires `cargo test -p <crate>` (full, non-doc) be a **standing runner-portability gate beside nextest** — the suite must be green under BOTH, and §4 states that only the shared-process runner can produce that evidence (a test relying on nextest's per-test process for isolation passes under nextest and fails here). Whether the repo currently satisfies this for `conductor-faults` and the four unmeasured crates is research's question.
- `test-plan` §11 → Integration bans relying on nextest's per-test process to isolate a test from a **process-global, first-install-wins singleton**, naming `conductor_core::init_observability` as the shipped case and stating a per-test `TempDir` sink does not isolate it. The prescribed remedy is **a test binary of its own** (one `tests/*.rs` file per such test — process-per-test under `cargo test` and under nextest, so it holds under both runners).
- `test-plan` §11 → Integration explicitly rejects **serializing the file** (`--test-threads=1` or a nextest profile knob) as a remedy — it hides the shared-state defect rather than removing it (§10).
- `test-plan` §10 (Zero-flakiness budget) requires **a runner-dependent result be fixed at the cause**, never pinned to a runner; retries are banned outright (`retries` > 0 forbidden; quarantine-and-fix, not retry-once) — restated in §11 → CI and §11 → Quality.
- `test-plan` §11 → E2E bans `sleep(N)` for synchronisation, with the 2026-09-06 scope clarification closing on "nothing inside a test may sleep to synchronise" — the harness-between-legs carve-out (the `--live` 150 s quiet window) is not available to an in-test fix.
- `test-plan` §4 (`conductor-faults` bullet) places ramp/silence/fingerprint generators at the pure-seeded unit tier and the `:4317` **bind/release at the chaos/integration tier (§8)** — so this subject legitimately lives at the crate-local `tests/` tier, which §2 (Directory pattern) also mandates for in-crate integration.
- `test-plan` §11 → Universal (project-specific) holds the `:4317` port-occupier as the **sole deliberate bind**, which must release on cleanup and must never become a general-purpose listener — any restructuring of the test must preserve that.

## Patterns to follow
- The §11 Integration remedy shape: give the singleton-dependent test **its own `tests/*.rs` binary**, since `cargo test` gives each integration test file its own process and nextest is already process-per-test — the one restructuring §11 names as holding under both runners.
- §2 / §4 conventions: crate-local `tests/` for slower in-crate integration, snake_case `#[test]` / `#[tokio::test]` fn names, per-seam crate boundary as the grouping (`cargo nextest run -p conductor-<seam>`).
- §11 → E2E's signal discipline: synchronise on an **explicit signal** (a journal line, an envelope `state` field, an exit code, a role/`aria-live` node), never on elapsed time — the substitute a race fix must reach for instead of a wait.
- §5 setup/teardown: per-test fresh in-memory `runs.db` / per-test `assert_fs::TempDir` remains the isolation pattern for DB and file state; §11 records that this guarantee **does not extend** to process-global state, which is the distinction this chunk turns on.
- §4 Mutation instrument records that `--test-tool=nextest` is REQUIRED because the default `cargo test` aborts the run in an unmutated tree **wherever a test is not runner-portable** — the same shared-process runner behaviour this chunk is measuring.

## Anti-patterns to avoid
- Serializing the suite (`--test-threads=1`, a nextest/libtest profile knob, or a `serial_test`-style lock) to make the fault crate green — banned by `test-plan` §11 → Integration as hiding the defect.
- Adding a retry or pinning to nextest — banned by `test-plan` §10 (Zero-flakiness) and §11 → CI/Quality ("do NOT set nextest `retries`").
- An in-test `sleep`/fixed wall-clock wait to let the subscriber install settle — banned by `test-plan` §11 → E2E, whose 2026-09-06 clarification refused exactly this carve-out.

## Contract bindings
- **tests ↔ obs:** the subject assertion reads lines written through `conductor_core::init_observability`, whose per-line self-obs shape `test-plan` §3 (Log format — "Self-obs stream is a distinct artifact") owns as the source of truth that obs-plan §3 derives from; the field allowlist the assertion exists to prove is obs-plan §4. Any restructuring must leave the §3 self-obs / emission-journal split intact and must not conflate the two artifacts.
- **tests ↔ CI (§9):** the portability gate is distinct from the §9 pipeline's `Unit` stage (`cargo nextest run --workspace --profile ci`) and its separate `Doctest` stage (`cargo test --workspace --doc`); §9 also records that `live-pulse`-feature-gated targets sit **outside both runner-portability runs**, which bounds what a `cargo test -p <crate>` measurement of `conductor-run` / `conductor-verify` is expected to cover.

## Acceptance criteria contributions
- `cargo test -p conductor-faults` (full, non-doc, default parallel libtest) exits 0 **and** `cargo nextest run -p conductor-faults` stays green — both runners, no pin (per `test-plan` §4 runner-portability gate).
- Each of `conductor-verify` · `conductor-run` · `conductor-cli` · `conductor-tauri` carries a recorded measured `cargo test -p <crate>` result; any red one is dispositioned as a determinism break fixed at the cause, never as a runner to pin (per `test-plan` §10 Zero-flakiness budget).
- The delta contains no nextest `retries`, no `--test-threads=1` / profile serialization of the subject, and no in-test `sleep` (per `test-plan` §11 → Integration, §11 → Quality/CI, §11 → E2E).
- Workspace gates stay green: `cargo nextest run --workspace --profile ci` plus `cargo clippy -D warnings` (per `test-plan` §10 Build failure conditions).

## Relevant amendment history
- **2026-08-20-verifier-self-hardening — §11 Integration, "process-global singleton isolation ban"** (the direct ancestor of this chunk): the ban was minted on a measured case — the wire-shape witness passed alone and under `--test-threads=1`, failing only alongside its four siblings, i.e. concurrent interference against the global subscriber that a per-test temp FILE cannot isolate. **Serializing was rejected as hiding the defect.** This chunk's subject is the same mechanism in a different crate.
- **2026-08-20-verifier-self-hardening — §4 + §10 + §12**: the same wrap made `cargo test -p <crate>` a standing runner-portability gate ("only the shared-process runner exposes a test relying on nextest's per-test process for isolation"), recorded in §10 that a runner-dependent result is a determinism break to fix at the cause, and logged it in §12 as a decision future chunks must not re-litigate — which is why the chunk's three pre-banned remedies are not re-openable here.
- **2026-09-06-operator-gated-live-suite — §11 `sleep(N)` ban scope**: a proposed carve-out permitting a fixed wall-clock wait on the operator-local gate was **explicitly REJECTED**; only a scope clarification was applied (the 150 s quiet window is a harness wait BETWEEN legs, never in-test), closing with "nothing inside a test may sleep to synchronise." This is the amendment behind the scope's "pre-banned … rejected explicitly at 2026-09-06-operator-gated-live-suite".
- **2026-08-31-p-075-assert-round — §9 / §11 CI**: a cargo-feature-gated test file (`[features] live-pulse = []`) is kept out of default nextest/clippy/release **and out of both runner-portability runs**, and owes its own per-feature clippy — relevant when measuring `conductor-run` / `conductor-verify` under `cargo test -p`, since gated targets are outside the gate's subject.
- No amendment in the history records a prior `conductor-faults` runner-portability change, consistent with the chunk's "no green ever existed for this form" premise; confirming that at HEAD is research's question.
