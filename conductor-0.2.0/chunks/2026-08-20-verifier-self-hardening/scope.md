# Scope — 2026-08-20-verifier-self-hardening

**Working-route entry:** Verifier self-hardening — the mutation-audit survivor families gain killing
assertions and the suite passes under either test runner (code-audit baseline 2026-08-20, §B1–§B3)

**Epoch:** 4 — Lifecycle & delegated timing (corrective first chunk, inserted ahead of severity-lifecycle
by founder ruling at the Epoch-3 boundary)

---

## What this builds

Conductor asserts a live Pulse. This chunk asserts *Conductor's own asserting machinery*: the
2026-08-20 code audit measured `conductor-run` at 58.82% and `conductor-verify` at 72.97% mutation
score, meaning covered lines execute with nothing checking their behaviour. The chunk closes the four
named survivor families and removes the reason the suite passes under one test runner and fails under
another.

It is a **test-and-harness-integrity** chunk. It changes no scenario, no contract, no capability claim,
and drives no live Pulse leg.

## In scope

### 1. Kill the four named survivor families

All coordinates below were re-verified against SOURCE at promotion (not read from the mutation text),
per the promotion rule's hypothesis clause. Where the audit's characterisation and the source disagree,
the source wins and the divergence is recorded.

| Family | Site (verified) | What is unasserted |
|---|---|---|
| `declares` ×6 | `crates/conductor-run/src/lib.rs:216-222` + `:206` | The run-contract `shell-declaration` predicate — an env read, trimmed and lowercased, accepted only on `true`/`1`. Stuck at `true`, an unmet term reads as satisfied; the architecture requires a distinct `Blocked` naming each unmet term. **Corrected from ×5: `c-mutation-conductor-run.json` also carries `:206:5 replace observe_run_contract -> RunContractStatus with Default::default()`, the wrapper in the same family. The audit's §B1 table claims "all 28 survivors" but enumerates 27 — this is the omitted row.** |
| `MAX_LINE_BYTES` ×5 | `crates/conductor-verify/src/jsonrpc.rs:79` ×3 + `:18` ×2 (const = 16 MiB) | The read-back per-line size bound `.claude/rules/security.md` REQUIRES. Guard replaced by `false` disables it and nothing notices; `>` moved to `==`/`>=` shifts the boundary. **Corrected from ×3: `:18:34` and `:18:41` mutate the CONSTANT expression `16 * 1024 * 1024` itself (`*`→`+`, collapsing the bound to ~1 MiB / ~17 KiB). Consequence for design: only an AT-LIMIT-accepted case kills those two, so the boundary test must exercise both sides, not just the over-limit rejection.** |
| canary boundaries ×5 | `preflight.rs:385` (freshness) + `preflight.rs:335` (loop guard) | Freshness is `opened > canary.emitted_at_unix_nano` — the after-the-storm semantics a whole prior chunk proved live, with the boundary itself unasserted. |
| journal stamps ×6 | `lib.rs:516-518` `now_ms`, `lib.rs:630-632` `now_unix_nanos` | Both are `SystemTime::now()` reductions ending `unwrap_or(0)`. Journal-relative SLO arithmetic rests on them. |

**Correction to the carried freight — the `preflight.rs:335` family is NOT "poll-deadline arithmetic".**
The audit (§B2) and the route CARRY both call it that. Source says otherwise: `:335` is the retry
loop's **last-attempt guard**, deciding whether to sleep *before the next attempt*. `poll_canary`
computes no deadline at all; its budget is attempts × interval, and `min_canary_poll_seconds` feeds it
upstream at `conductor-run/src/lib.rs:285` through a `.max(...)`. This matters for how the survivors are
killed, not merely how they are described:

- The `+` → `*` mutation collapses the guard to a comparison that is still true on the final
  iteration, so the loop sleeps once more than it should. **The return value is unchanged.**
- Moving `<` to `<=` has the same shape; moving it to `>` or `==` changes which attempts sleep.

So this family is **timing-observable only** and cannot be killed by asserting `poll_canary`'s outcome
— it needs an elapsed/advance assertion (paused `tokio::time`) or an equivalent sleep-count
observation. `[premise-corrected: VERIFIED with a refinement — `start_paused = true` is established (11 uses in `conductor-run/tests/dispatch_wire.rs`, 1 in `conductor-core/tests/operator_pause.rs`), but explicit `tokio::time::advance` is used NOWHERE in the workspace; its one occurrence is a comment stating it was deliberately not needed. The house idiom is `start_paused` + tokio auto-advance, asserting on observed virtual time.]`

**Note on the stamp helpers:** both end `unwrap_or(0)`, so a mutant returning `0` is indistinguishable
from the genuine pre-epoch error path by return value alone. A killing test must assert a *plausible
`std::time` magnitude*, not merely non-zero. `[premise-corrected: VERIFIED — zero test references either helper (all 8 hits are src-side call sites plus the two definitions). NEW CONSTRAINT: both are CRATE-PRIVATE (`fn`, not `pub fn`), and arch bans widening visibility for a test seam — so the assertion must go through a public observable carrying the stamp (`journal_emitted_at`, or `CanaryMarker.emitted_at_unix_nano`), never by calling the helper.]`

### 2. Runner portability — remove the cause (§B3)

`conductor-run::canary_wire the_wire_shape_witness_reaches_the_self_obs_artifact` passes under nextest
and fails under `cargo test`, same commit. CLAUDE.md pins nextest, so the failing runner is the one
nothing runs — and cargo-mutants defaults to `cargo test`, which is why this silently blocked the
mutation tier until it was re-run with `--test-tool=nextest`.

**The carried fix does not match the mechanism — recorded as a correction, resolution deferred to P4.**
The route CARRY prescribes "a per-test temp obs sink". The test at `canary_wire.rs:222` **already has
one** (an `assert_fs::TempDir` holding `obs.jsonl`), so that is not the defect. Verified at source, the
contributors are:

- `init_observability` installs a **process-global** `tracing` subscriber and documents that *the first
  install in a process wins; a later call leaves the global subscriber untouched*
  (`conductor-core/src/obs.rs:68-77`). A per-test FILE cannot isolate a global SUBSCRIBER.
- `canary_wire.rs` holds **5 tests, 4 of which call `emit_canary_storm`** (`:79` via the
  `captured_storm_spans` helper, `:179`, `:229`, `:273`) but **only one installs a subscriber**
  (`:223`). Under `cargo test` all five share a process and run thread-parallel, so the other tests'
  wire-shape debug lines land in the witness test's file and its one-line-per-batch count over-counts.
- `:220` performs an `unsafe` `set_var` of `RUST_LOG` whose own SAFETY comment justifies itself with
  *nextest runs each test in its own process* — a documented nextest-only assumption, unsound under a
  shared process.

`[premise-corrected: VERIFIED BY MEASUREMENT — three runs: the whole binary under `cargo test` FAILS (4 passed, witness failed); that test ALONE passes; `--test-threads=1` passes all 5. The cause is CONCURRENT cross-test interference, not install-ordering and not a broken test — the four siblings’ `emit.batch` debug lines land in the witness’s globally-installed sink while it counts. The carried "per-test temp obs sink" fix is CONFIRMED not the defect. Aside: the failure prints NO panic message, because `init_observability` installs the panic hook alongside the subscriber (`obs.rs:68`) and routes the assertion text into the obs sink — part of why this stayed invisible.]` The fix shape is P4's; scope fixes only that the cause is the global subscriber
(plus the env write), not the sink path — and that the test deliberately reads a REAL artifact produced
by the REAL production init path, so any fix must preserve that premise rather than swap in a
test-only subscriber.

**Generalized at P4 (intent-incomplete, val-1):** the isolation remedy is not confined to the witness test. Any test that manipulates process env or installs the global subscriber carries the same hazard, so such tests get their OWN integration-test file — under `cargo test` each `tests/*.rs` is a separate process and nextest is already process-per-test, so the pattern holds under both runners. This governs the `declares` matrix (§1) as well as the witness (§2).

### 3. Rider — unused dependency

`conductor-cli` declares `tracing` with no `use tracing`, no `tracing::` path, no `#[tracing` attribute
and no bare log macros under `src/` (cargo-machete, independently corroborated by the symbol graph,
§B4). Remove the declaration.

## Acceptance shape (carried, operator-set)

A **scoped** `cargo-mutants` re-run over the two audited units with `--test-tool=nextest` and
`stub_pulse_mcp` **excluded** (operator ruling — the exclusion is recorded in the next audit ledger
record) shows the NAMED survivors killed. `cargo nextest run --workspace --profile ci` stays zero-retry
green. The suite additionally passes under `cargo test` for the affected binary.

## Explicitly NOT in scope (rulings on record)

- `shape_is_realizable` (`conductor-core/src/phase_spec.rs`) — flagged by complexity (§B6); no action
  indicated.
- The `conductor-emit` → `conductor-core` dependency — it exists solely for a rustdoc intra-doc link
  (`error.rs:5`); removing it trades a dependency edge for a broken doc link. **Deliberate keep.**
- Splitting `conductor-run/src/lib.rs` (898 lines, §B8) — **watch only**, no action this chunk.
- Raising the mutation score as a number. The chunk targets the four NAMED families; the score moves as
  a consequence and is not itself the acceptance.
- Any scenario, contract, capability, or live-Pulse work.

## Carried gate obligation

`PREREQ: re-check cargo audit — 33rd consecutive.` Standing deferral since
`2026-08-08-sut-capability-manifest`, ratified at the `2026-08-10-workspace-key-divergence-probe` wrap.
Eligible for the **PROBE-AUTO-SATISFY** tier: signature is `cargo audit` at true exit 1 whose first
diagnostic line reports the duplicate advisory id `RUSTSEC-2026-0244`, together with `cargo deny` at
true exit 0. Reproduce byte-identically then record `probe unchanged, 33rd consecutive`, with no basis
re-authoring. ANY deviation restores the full form. Remedy stays the bounded wait — no floor raise, no
`deny.toml` ignore, no CI edit.

## Boundaries / invariants this chunk must not breach

- **Determinism is the bar** — no nextest retries; any new test is deterministic under the pinned
  runner.
- **Verdict/error wall** — killing tests assert typed values (`Verdict` / `ReportState` / `VerifyError`),
  never panics-as-assertions.
- **Journal stamps from `std::time`** — a stamp test must not reach for tokio's virtual clock; the
  paused-clock technique applies to the poll loop, which is scheduling, not journal stamping.
- **No host paths or internal struct names** in any artifact a test writes or reads back.
