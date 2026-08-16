# tests extract

## Relevance
Relevant — test-plan §3 is the declared source of truth for the self-obs JSONL line format this chunk extends (three new span names + five new attributes), and the touched crates carry unit/chaos-tier obligations.

## Constraints
- The self-obs stream (`logs/agent-latest.jsonl`) and the per-run emission journal (`runs/<run_id>.jsonl`, the Run-report envelope / SLO ground truth) are SEPARATE artifacts whose schemas must not be conflated — fault-span assertions belong to the former, and the envelope's field set must not grow (per test-plan §3 Log format → "Self-obs stream is a distinct artifact").
- test-plan §3 requires span-lifecycle lines to carry `span` (a name from the bounded §4 set), `span_event` (`new` | `close`), optional `parent`, and the span's own **allowlisted** attributes **on the `new` line only** — a span attribute is gated by the same `conductor-core::redact` allowlist as any event field, so an un-allowlisted attribute is dropped silently and a presence-only test would pass vacuously (per test-plan §3 Log format).
- test-plan §3 declares itself the source of truth for the JSONL format with obs deriving from it, and mandates that the field-allowlist/redaction boundary (no host paths, no internal struct names) be asserted by a negative test (per test-plan §3 Bootstrap phases → log-format-bind-with-obs).
- Timing values must not be asserted against real elapsed wall-clock: drive `tokio::time::advance` under `start_paused = true` and assert scheduled ordering/shape (per test-plan §11 Unit, stack-specific). Conversely, ground-truth stamps/offsets must not originate from tokio's virtual clock (per test-plan §11 Test Data, stack-specific) — the chunk's `fault_start_offset_ms` sits exactly on that seam.
- `conductor-faults` ramp/silence generators are the pure-seeded-function unit tier; the `:4317` port-occupier bind/release is explicitly a chaos/integration concern, not unit (per test-plan §4 What unit tests cover → conductor-faults, and §8).
- The `:4317` port-occupier is the sole deliberate bind and must release; harness verification is a `TcpListener::bind("127.0.0.1:4317")` succeeding after cleanup (per test-plan §3 `cleanup` → Verification, and §11 Universal project-specific).
- Zero-flakiness is binding: no nextest `retries`, a flake is quarantined and root-caused, and the workspace line-coverage gate stays at `--fail-under-lines 60` (per test-plan §10).

## Patterns to follow
- Per-seam crate-local `#[cfg(test)] mod tests` for unit logic plus crate-local `tests/` for slower in-crate integration; selection via `cargo nextest run -p conductor-<seam>` (per test-plan §2 Test directory conventions, §4 Conventions).
- rstest `#[fixture]` for the seeded generator + `#[rstest]` `#[case]` table rows — the natural shape for the three fault kinds as one table (per test-plan §4 Fixture pattern, §7 Fixture library).
- `#[tokio::test(flavor = "current_thread", start_paused = true)]` + `tokio::time::advance` for any scheduling-dependent assertion; time is mocked by the built-in virtual clock, never by monkey-patching (per test-plan §2 Agent-runnable invariants, §8 Mocking libraries).
- Canonical line-shape goldens at unit tier use exact-string `assert_eq!` (the `verdict.rs`/`report_state.rs` serialization-golden pattern); insta is reserved as the E2E journal-golden mechanism with redaction, CI fail-don't-write, never `cargo insta review` (per test-plan §4 conductor-report bullet, §7 Golden artifacts).
- CLI/E2E legs sandbox artifacts through `assert_fs::TempDir` + env redirection and parse each line with `serde_json::from_str` or `jq -e` (per test-plan §3 `status`/`logs`, §7 Per-test isolation).

## Anti-patterns to avoid
- Never assert the fault spans against the emission journal or add them to the Run-report envelope — that conflates the two record shapes §3 keeps distinct (per test-plan §3 Log format).
- Never assert a real wall-clock duration for `fault_duration_ms`/offsets, and never let the virtual clock supply a ground-truth offset (per test-plan §11 Unit + §11 Test Data, stack-specific).
- Never escalate the fault-kind tests toward saturation/load profiles — bounded "typical/high" only — and never `sleep(N)` to synchronize span open/close (per test-plan §11 Test Strategy stack-specific, §11 E2E).

## Contract bindings
- **tests §3 ↔ obs §3/§4:** test-plan §3 Log format owns the self-obs line schema and obs derives from it; this chunk changes that format (new bounded span names + new allowlisted attributes), so §3's span-lifecycle bullet and obs-plan §3 must move in lockstep — the same lockstep the 2026-08-10 amendment executed. Whether the current §3 wording already covers a three-name addition without amendment is a wrap-time question, not a code question.
- **tests §3 ↔ obs redaction layer:** the `conductor-core::redact` allowlist is obs-owned downstream, but test-plan §3 mandates that tests assert that boundary via a negative test; the five new attribute names sit on it.
- **tests §3 `cleanup` ↔ faults/security Vector 6:** the port-occupier's span close must coincide with its bind release, and §3's cleanup verification (rebind probe) is the existing assertion hook.
- **tests §9 ↔ the CI obs-conformance gate that reads `logs/agent-latest.jsonl`:** whether that gate exists today and whether it enumerates the bounded span-name set (and would therefore need extending) is research's question.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-faults` and `-p conductor-run` pass for the new tests, the workspace `cargo nextest run --workspace --profile ci` leg stays green with zero retries, and line coverage holds `--fail-under-lines 60` (per test-plan §3 `run` / §10).
- (tests) For each of `fault.silence` / `fault.ramp` / `fault.port_occupier`, a test asserts the emitted `span_event="new"` line carries the span name AND every named attribute after the redact allowlist, plus a matching `close` line — presence-only assertions are insufficient because non-allowlisted attributes are dropped silently (per test-plan §3 Log format, span-lifecycle line variant).
- (tests) A negative test asserts the allowlist boundary on the new attributes (an un-allowlisted sibling is dropped; no absolute host paths or internal struct names reach the line) (per test-plan §3 Bootstrap phases → log-format-bind-with-obs).
- (tests) Duration/offset assertions are shape- or ordering-based under `start_paused = true` + `tokio::time::advance`, with `fault_start_offset_ms` asserted against the `std::time` journal basis rather than the virtual clock (per test-plan §11 Unit + §11 Test Data, stack-specific).

## Relevant amendment history
- **2026-08-10-scenario-run-root-span-tree** (§3 Log format) — recorded the self-obs stream's two line variants, adding the span-lifecycle line (`span` / `span_event` / optional `parent` / allowlisted attributes on `new` only) and the rule that span attributes pass the same `redact` allowlist as event fields. Why it matters here: this chunk is the direct successor — it adds names into that bounded set, and the amendment's precedent is that a report changing the log format fires D-tests-obs-harness and amends test-plan §3 and obs-plan §3 in lockstep.
- **2026-08-14-canary-fingerprint-feed-capture** (§3 Log format) — the event-line variant was restated as a SET (any `tracing` record at whatever level obs-plan §11 assigns) instead of level literals, after the first `debug`-level self-obs line. Why it matters here: the same de-literalizing discipline governs how three new span names get written into §3 — name the bounded set and its source, not a fresh enumeration that re-stales.
- **2026-06-15-structured-logging-stack** (§3 Log format) — established that the self-obs stream is a distinct artifact from the emission journal and that the two schemas must not be conflated. Why it matters here: it is the rule that keeps the chunk's spans out of the Run-report envelope and out of `runs/<run_id>.jsonl`.
- **2026-08-13-dispatcher-determinism-goldens** (§7 Golden artifacts) — registered the seeded STREAM golden families (`replay__*` · `pacing__*` · `dispatch_wire__*`), one file per family × seed, CI fail-don't-write. Why it matters here: the chunk's own boundary says no change to emission shape, phase timing, or seeded determinism — those committed goldens are the standing evidence for that claim, and this chunk also carries that chunk's timeline doc-comment CARRY.
- **2026-06-16-emission-journal-writer** (§4 conductor-report) — fixed the unit-tier golden mechanism as exact-string `assert_eq!`, with insta reserved for E2E. Why it matters here: it determines the mechanism for any canonical fault-span line-shape golden.
