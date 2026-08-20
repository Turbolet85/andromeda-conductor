# Report — 2026-08-20-verifier-self-hardening

**Chunk:** Verifier self-hardening — the four mutation-audit survivor families gain killing assertions and the suite passes under either test runner (code-audit baseline 2026-08-20, §B1–§B3)
**Date:** 2026-08-20
**Commits:** (this chunk's work is uncommitted until this wrap; the preceding commit is `chore(route): operator-requested adaptation — 0-pending wrap`)

## Changes (structured — detectors read this)

- **Files:**
  - NEW `crates/conductor-run/tests/canary_obs_witness.rs` — the relocated wire-shape witness, alone in its own test binary.
  - NEW `crates/conductor-verify/tests/jsonrpc_line_bound.rs` — both-sides read-back line-bound boundary (2 tests).
  - MOD `crates/conductor-run/tests/canary_wire.rs` — witness test removed (−61 lines) with its now-unused `ObsSink`/`init_observability` imports; the four wire-shape tests stay.
  - MOD `crates/conductor-verify/tests/preflight.rs` — +3 tests (+66 lines): equal-stamp boundary, one-nanosecond-after boundary, paused-clock poll-loop sleep count.
  - MOD `crates/conductor-run/src/lib.rs` — **test module only** (+48 lines, 3 tests): journal-stamp magnitude, cross-helper agreement, advance-across-a-real-pause. No production logic changed.
  - MOD `crates/conductor-cli/Cargo.toml` — `tracing` dependency removed.
  - MOD `crates/conductor-verify/Cargo.toml` — tokio dev-dependency gains `test-util`.
  - MOD `Cargo.lock` (−1 line), `.gitignore` (+2 ignore entries + comment).
- **Symbols / APIs:** none added, changed, or removed. No public surface moved: every killing test drives an EXISTING public seam (`ReadbackClient::connect_transport`, `run_preflight`, `CanaryPoll`) or, for the crate-private stamp helpers, the in-crate test module. No new ports, sockets, or env vars. `conductor-cli` keeps its obs-plan §4 cli-row hook through `conductor_core::init_observability` (`main.rs:28`) — the subscriber lives in `conductor-core`, so removing the direct `tracing` edge changes no instrumentation.
- **Crates / modules:** none added or removed. Two new integration-test binaries inside existing crates.
- **Dependencies:** `conductor-cli` → `tracing` **REMOVED** (cargo-machete finding, graph-corroborated: no `use tracing`, no `tracing::`, no `#[tracing`, no bare log macros under `src/`). `conductor-verify` dev-dependency tokio **+`test-util` feature** (required by `start_paused`; matches `conductor-run` and `conductor-core`, which already carry it). `Cargo.lock` delta is exactly one line — the removed edge; **zero packages added or removed**, resolved graph otherwise byte-identical.
- **Schema / config:** none. No migration, no config key, no violation-schema change.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - nextest workspace total **661 → 669** (+8 net: 8 new tests; the witness moved rather than added).
  - `conductor-run/src/lib.rs` mutation survivors **27 → 21** (measured, scoped re-run).
  - `conductor-verify` `jsonrpc.rs`+`preflight.rs` scoped mutation: **33 caught / 5 missed / 11 unviable / 2 timeout**, zero missed among targeted sites.
  - Named-survivor disposition: **22 → 16 killed + 6 classified accepted-deliberate**.
- **Dev-tool versions:** none installed or upgraded. `cargo-mutants 27.1.0` was already present (used by the 2026-08-20 audit); this chunk is its first use as a chunk acceptance instrument.
- **Reverted / negative API facts:** none. Notably NOT done: `declares`/`observe_run_contract` were **not** restructured (see below), and no symbol visibility was widened to `pub` for testability.
- **Spec claims disproved by measurement:**
  1. The code-audit's §B1 introduces its evidence table as "all 28 survivors" but enumerates **27** — it omits `crates/conductor-run/src/lib.rs:206:5 replace observe_run_contract -> RunContractStatus with Default::default()`. Measured against `c-mutation-conductor-run.json` (`survivors` array, 28 entries). Stated in `.andromeda/runs/2026-08-20T18-06-29-code-audit/proposals.md` §B1.
  2. The same audit's §B3 states the `--test-tool=nextest` re-run is "recorded in both `c-mutation-*.json` `command` fields"; those fields read `cargo mutants -p CRATE --output {run_dir}/... --jobs 2` with **no such flag**, so replaying the recorded form reproduces the `cargo test` abort rather than the scores. Measured by reading both JSON files (`test-tool=nextest` occurs only in the prose).
  3. Both are prior-run artifacts (run history, never edited). Disposition: they ride the audit-ledger note below for the next boundary's audit to consume — not a spec-master amendment.
- **Coverage of new surfaces:** no new external surface, hot-path operation, or UI element. All nine touched tests are internal verification surfaces:
  - `canary_obs_witness.rs` (self-obs artifact read-back) → validation n/a · instrumentation ✓ (reads the REAL production `init_observability` path, unchanged) · PII ✓ (asserts the §3 base field set; no host path) · tests ✓ integ · a11y n/a · tokens n/a
  - `jsonrpc_line_bound.rs` (read-back decode boundary) → validation ✓ (asserts the security-mandated bound on BOTH sides) · instrumentation n/a · PII n/a · tests ✓ integ · a11y n/a · tokens n/a
  - `preflight.rs` new legs (canary freshness + poll budget) → validation ✓ (degrade direction asserted) · instrumentation n/a · PII ✓ (the existing host-path leg still green) · tests ✓ integ · a11y n/a · tokens n/a
  - `lib.rs` stamp legs → validation n/a · instrumentation ✓ (asserts `std::time` magnitudes; explicitly NOT tokio's virtual clock) · PII n/a · tests ✓ unit · a11y n/a · tokens n/a

## Deviations from intent

1. **Touchpoints widened to `crates/conductor-run/src/lib.rs` (test module only).** The plan put the `declares` and journal-stamp families in new integration-test files driving `readiness()`/`execute_scenario`. Both are unreachable from outside the crate: `Preflight` has no public constructor and `conductor-run` exposes no command-injection seam (unlike `conductor-verify`'s `connect_transport`, which is exactly what made the line-bound family reachable). `lib.rs` already carries a `blocked_preflight()` helper and four `execute_scenario`/`drive_run` tests in its own `#[cfg(test)]` module for that reason. Surfaced at implement, **founder-ruled to widen** for the stamps; no production logic touched.
2. **`crates/conductor-verify/Cargo.toml` gained tokio `test-util`** (dev-dependency only). Unlisted in the plan, but `start_paused` — which plan step 6 mandates — does not compile without it; matches the two sibling crates already using the idiom.
3. **The plan's two mutation command lines do not work as written.** `cargo-mutants -f` resolves paths from the **workspace root**, not the package, so `-f src/jsonrpc.rs` under `-p conductor-verify` returned `Found 0 mutants to test` with only a `WARN` **and exit 0** — a silent no-op indistinguishable from a clean pass. Working form is the full crate path (`-f crates/conductor-verify/src/jsonrpc.rs`). The `--exclude` for `stub_pulse_mcp` proved unnecessary once the run was scoped by file.

## Decisions & corrections

- **`declares` ×6 are ACCEPTED-DELIBERATE, not deferred debt** (founder ruling). `.claude/rules/testing.md` (2026-08-10) prescribes reading env *at the caller* and passing it in as a typed value "so tests construct the status directly and both branches stay deterministic with no `unsafe` and no env mutation." That rule is precisely why `declares` is a thin env-reading edge with the real logic in the already-tested `RunContract::evaluate`. Killing those mutants would require `unsafe set_var` inside a shared-process test module — the exact hazard this chunk removed. The edge was **not** restructured this chunk.
- **The runner-portability fix removes the cause, not the symptom.** Measured: the witness test passed alone and under `--test-threads=1`, failing only alongside its four `emit_canary_storm` siblings — so the defect is concurrent interference against a **process-global subscriber** (`obs.rs:68-77`, first-install-wins), which a per-test *file* cannot isolate. The carried CARRY's prescription ("a per-test temp obs sink") was already present in the test and was therefore not the defect. Rejected alternative: serializing the binary, which hides the shared-state defect (test-plan §10) and was the diagnostic, not the fix.
- **A frozen-constant stamp passes a magnitude bound forever.** The stamp kills therefore use three assertions, not one: magnitude (both helpers end `unwrap_or(0)`, so `0`/`1`/`-1` are indistinguishable from the genuine error path by sign), cross-helper agreement in a common unit (mutation replaces one helper at a time, so each guards the other), and advance across a **real** `std::thread::sleep` pause — never the virtual clock, which the journal basis may not use.
- **The at-limit case is load-bearing, not symmetry.** Two of the line-bound survivors mutate the constant expression `16 * 1024 * 1024` itself; only a line accepted AT the bound kills them. A guard-only (over-limit) test would have left both alive. This correction came from reading the audit's JSON rather than its prose table.
- **Audit-ledger note (for the next boundary's audit to consume, not a spec amendment):** record the `stub_pulse_mcp` mutation-scope exclusion (operator ruling); record `declares` ×6 as accepted-deliberate with the testing.md 2026-08-10 citation; and record that `c-mutation-*.json`'s `command` field omitted `--test-tool=nextest` while §B3 claimed it was recorded there.

## Outcome

**Acceptance criteria met**, with one criterion re-dispositioned by ruling (the `declares` family — classified rather than killed).

Measured survivor disposition across the four named families (22 total):

| Family | Sites | Result |
|---|---|---|
| `MAX_LINE_BYTES` ×5 | `jsonrpc.rs:18` ×2, `:79` ×3 | killed (9 caught, 0 missed) |
| canary boundaries ×5 | `preflight.rs:385`, `:335` | killed (8 caught, 0 missed) |
| journal stamps ×6 | `lib.rs:517`, `:631` | killed (3/0 and 3/0) |
| `declares` ×6 | `lib.rs:206`, `:218`, `:220` | accepted-deliberate (measured still-surviving, by design) |

`conductor-run/src/lib.rs` survivors went 27 → 21; the delta is exactly the six stamps.

**Gates green** (commands run): `cargo nextest run --workspace --profile ci` → **669/669, zero-retry** · `cargo test -p conductor-run -p conductor-verify` → all binaries green (the portability criterion's own evidence — only this runner can produce it) · `cargo test -p conductor-run --test canary_obs_witness` / `--test canary_wire` → green · `cargo test --doc --workspace` → green · `cargo clippy --workspace --all-targets -- -D warnings` → clean · scoped `cargo mutants --test-tool=nextest` on both crates (full crate-path `-f` forms) → as tabled above. No gate deferrals: this chunk has manifest + lockfile delta, so every changed-surface gate ran.

**Carried PREREQ discharged — 33rd consecutive, FULL form.** `cargo audit` reproduced its signature byte-identically (true exit 1; first diagnostic line `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244` — an advisory-DATABASE fault, not a Conductor defect). Overlap **verified, not assumed**: `cargo deny check advisories bans licenses sources` observed **true exit 0 over the POST-removal `Cargo.lock`**, per security-plan's 2026-08-16 requirement that the deny overlap be re-verified whenever the tree moves. Basis re-derived: the lock delta is one removed dev-edge line with zero package-set change. Remedy stays the bounded wait — no floor raise, no `deny.toml` ignore, no CI edit.

**Smoke ✓** (boot-path condition: the dependency removal changes the `conductor` bin crate's dependency set). The binary boots — `conductor --help` exit 0 with the full verb surface — and its 36-test suite passes, including the `cli_smoke` E2E legs that spawn the real binary.

**Expected amendment (wrap):** `test-plan.md` §4 (tool inventory / floors policy) and §9/§10 (pipeline stages / gates) — `cargo-mutants` is this chunk's acceptance instrument and is registered in none of §4, §9, §10, §12. Direction: register it with a floor (the §4 floors-not-pins precedent), and fold in the mutation-tooling discipline — full-crate-path `-f` forms · `--test-tool=nextest` required · **"Found 0 mutants" is a NO-OP, never a pass** · output gitignored (the `.gitignore` comment already names the audit-vs-ad-hoc split).
