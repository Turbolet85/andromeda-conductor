# Report — 2026-08-20-read-back-seam-survivors-closed

**Chunk:** Read-back seam survivors closed — the five out-of-family audit survivors (request-id counter ×2,
the discarded `notifications/initialized` notify, ShapeWitness one-shot ×2) gain killing assertions over the
public `connect_transport` duplex (code-audit baseline 2026-08-20, §B2 remainder)
**Date:** 2026-08-20
**Commits:** (uncommitted at authoring — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/conductor-verify/tests/common/mod.rs` (modified — additive)
  - `crates/conductor-verify/tests/jsonrpc_correlation.rs` (new)
  - `crates/conductor-verify/tests/readback_shape_witness.rs` (new)
  - `crates/conductor-verify/Cargo.toml` (modified — `[dev-dependencies]` only)
  - `Cargo.lock` (modified — one line)
- **Symbols / APIs:** ALL test-support, none reachable from shipped code. Added in
  `tests/common/mod.rs`: `WireEntry` (enum: `Request{id,method}` / `Notification{method}`), `WireLog`
  (Clone, `entries()` / `request_ids()` / private `push`), `DECOY_TOOL` const, two `StubConfig` fields
  (`wire_log: Option<WireLog>`, `decoy_before_nth_request: Option<u32>`), and a private `write_line`
  helper factored out of the existing response-write tail.
  **Remaining-caller facts (not a sole-caller claim):** `serve_stub`'s signature is UNCHANGED and both new
  `StubConfig` fields default to `None`, so all 7 existing `ReadbackClient::connect_transport` call sites
  (`tests/jsonrpc_line_bound.rs:45`/`:62`, `tests/preflight.rs:52`/`:242`, `tests/readback.rs:17`/`:31`/`:119`)
  and all 15 existing `tests/preflight.rs` legs compile and pass untouched — verified by the full suite.
  **No production `pub` surface changed; no `pub(crate)` was widened** (the route entry's explicit boundary).
- **Crates / modules:** none added, removed, or renamed. `conductor-verify` gains 2 test binaries.
- **Dependencies:** `assert_fs` added as a `conductor-verify` **dev-dependency** (`assert_fs.workspace = true`).
  Already `[workspace.dependencies]` (`Cargo.toml:83`) and already consumed by `conductor-run`, so the
  `Cargo.lock` delta is ONE dependency-list line under `conductor-verify` with **zero `[[package]]` nodes
  added or removed** — verified by diff, not assumed.
- **Schema / config:** none.
- **Spec-master edits:** none authored at P1. One is EXPECTED at P2 (test-plan §4, the cargo-mutants
  exit-code line — see *Decisions & corrections*).
- **Counts / qualifiers moved:**
  - workspace nextest **669 → 673**
  - `conductor-verify` suite **92 → 96**
  - `Cargo.lock` **+1 line** (the `assert_fs` dependency-list entry; package set unchanged)
  - scoped mutants over `jsonrpc.rs` + `preflight.rs`: **51 mutants · 38 caught / 0 missed / 11 unviable /
    2 timeout**, against a **33 caught / 5 missed / 11 unviable / 2 timeout** baseline
  - named-survivor disposition: the audit's **5 remaining survivors → 0**; every audit-surfaced survivor
    across the workspace is now killed or classified accepted-deliberate
- **Dev-tool versions:** none. cargo-mutants 27.1.0 and cargo-nextest 0.9.133 are the versions already
  registered at the previous wrap; neither was installed or upgraded here.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  - **The plan's timeout-conversion speculation is measured FALSE.** Plan step 8 allowed that a stub closing
    its side might convert `jsonrpc.rs:49:30` (`!=`→`==`) from TIMEOUT to CAUGHT "for free". It did not —
    **both timeouts persist unchanged** (`:49:30` and `:70:9`). Recorded plainly as a negative. This was
    operator-ratified at phase P4 as observe-and-report, never acceptance, so it fails nothing.
  - **(Resolved pre-implement, recorded for the trail)** the taken-up CARRY asserted the id-counter kill
    "must break correlation, not just arithmetic". Research measured that unachievable against a faithful
    echoing stub — `-=` sends id 0 and is answered with id 0, `*=` sends id 1 and is answered with id 1, both
    pairing correctly. Resolved at phase P4 keep-the-goal-change-the-mechanism: the recorded id-SEQUENCE
    assertion is load-bearing, with a stale-decoy test added as the correlation complement. No spec master
    states the claim, so nothing is owed here.
- **Coverage of new surfaces:** **no new external surface, hot-path op, or UI element.** The chunk adds
  test-support and assertions only.
  - `tests/common/mod.rs::WireLog` (test-support recorder) → validation n/a (no external input) ·
    instrumentation n/a · PII n/a (synthetic fixture data only) · tests ✓ (exercised by all 3 correlation
    legs) · a11y n/a · tokens n/a
  - `tests/jsonrpc_correlation.rs` (3 legs) → tests ✓ unit/integration tier · all other flags n/a
  - `tests/readback_shape_witness.rs` (1 leg, alone in its binary per test-plan §11) → instrumentation ✓
    (it READS the existing self-obs artifact; adds no span, no field, no OTel SDK) · PII n/a · tests ✓ ·
    all other flags n/a

## Deviations from intent

1. **The id/notification assertions landed in a NEW sibling binary `tests/jsonrpc_correlation.rs`, not in
   `tests/jsonrpc_line_bound.rs`.** *Justification:* the plan listed the target as
   "`tests/jsonrpc_line_bound.rs` (or a new sibling binary)" and research §Open questions explicitly left the
   call to implement. `jsonrpc_line_bound.rs`'s module doc scopes it to the per-line size bound; correlation
   is a different property. `jsonrpc_line_bound.rs` is therefore untouched.
2. **`--jobs 2` added to the plan's mutation command.** *Justification:* matches the 2026-08-20 audit
   baseline's concurrency (`c-mutation-conductor-verify.json`), so the two tallies compare on one basis.
3. **The mutation gate ran as a backgrounded command rather than a foreground gate.** *Justification:* the
   run takes ~4 minutes, past the foreground command ceiling; output was redirected to the scratchpad and the
   tallies read from `mutants.out/`.

## Decisions & corrections

- **cargo-mutants' exit code reflects surviving/timeout CLASSES, not run success — gate on the tallies,
  never the exit code.** Measured this chunk: **exit 3** on a run that fully met its acceptance (0 missed,
  all five named survivors killed), the non-zero coming from the two pre-existing timeouts. The project rules
  already record the converse direction (`Found 0 mutants to test` exits 0 — a no-op that reads as a pass);
  this is the other half. *Owed as ONE line in this wrap's test-plan §4 amendment, beside last wrap's
  registrations* (operator directive).
- **Both operator P4 rulings held and are recorded in `scope.md`:** `assert_fs` taken with a
  verify-don't-assume condition on the lock diff (discharged — the diff confirmed zero package-set change),
  and five-only with timeout status reported as an observation that can never fail the chunk.
- **Carried PREREQ discharged — 34th consecutive, COMPACT form, and the record stands as taken.** A deviation
  was detected (this chunk moves the lock), the entitlement was re-derived from the diff (one dependency-list
  line, zero package nodes admitted), and the compact record was then written:
  `probe unchanged, 34th consecutive`. Evidence: `cargo audit` true exit 1 with first diagnostic line
  `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`;
  `cargo deny check advisories bans licenses sources` true exit **0** over the POST-change lock
  ("advisories ok, bans ok, licenses ok, sources ok") — verified, never assumed, per security-plan
  §Dependency Security's admitting-a-dependency-under-a-red-audit rule. The PURE auto-satisfy first fire is
  expected at the 35th, on a zero-delta chunk.
- **Self-caught verification defect (already in the friction ledger):** the first five-survivor check built
  its grep pattern through a failed command substitution, yielding an EMPTY pattern — `grep -qF ""` matched
  every line and printed CAUGHT for all five while testing nothing. Visible only because `sed` printed its
  own error beside the green lines. Re-verified with literal patterns against `caught.txt` (lines 5/6/8/34/35)
  and an empty `missed.txt`.

## Outcome

**Acceptance met.** The scoped `cargo mutants -p conductor-verify --test-tool=nextest -f
crates/conductor-verify/src/jsonrpc.rs -f crates/conductor-verify/src/preflight.rs` run reports a **non-zero
mutant count (51)** — not the `Found 0` no-op — with **all five named survivors killed** and the
previously-killed set still dead (**38 caught = 33 baseline + 5**), and **`missed.txt` empty**. Each of the
five verified literally in `caught.txt`: `jsonrpc.rs:41:22` (`-=`) L5, `jsonrpc.rs:41:22` (`*=`) L6,
`jsonrpc.rs:65:9` (`notify`→`Ok(())`) L8, `preflight.rs:357:9` (`ShapeWitness::list`→`()`) L34,
`preflight.rs:357:12` (delete `!`) L35.

**Gates green (commands run):** `cargo nextest run -p conductor-verify` (96/96) · `cargo test -p
conductor-verify` (exit 0, 96 tests — the runner-portability gate) · `cargo nextest run --workspace --profile
ci` (**673/673**, zero-retry) · `cargo clippy --workspace --all-targets -- -D warnings` (exit 0) ·
`cargo test --doc --workspace` (exit 0) · the scoped `cargo mutants` run · `cargo deny check advisories bans
licenses sources` (true exit 0) · `cargo audit` (true exit 1, the pinned probe signature).

**Fix-loop: 0 iterations** — no gate went red. **Gate deferrals: none** — the Rust delta was non-zero
(`Cargo.toml` + `Cargo.lock` + new test sources), so every workspace gate ran as mandatory.

**Smoke: skipped — no boot-path / UI-surface change.** Verified from the change set rather than assumed: 5
files, all under `crates/conductor-verify/tests/` plus one dev-dep manifest line and one `Cargo.lock` edge
line; zero `src/`, zero binary, zero frontend surface.

**Capabilities claimed: none.** All 15 unclaimed matrix entries need a live Pulse leg, an a11y/webview
surface, or another route entry's work; this chunk drives none. Coverage stays **17/32 verified · 15
unclaimed**. The matrix is untouched.
