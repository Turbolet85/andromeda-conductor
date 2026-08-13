# Report — 2026-08-13-dispatcher-determinism-goldens

**Chunk:** Dispatcher determinism goldens — the per-phase emission stream frozen under seed, plus the load-envelope close-out the contract itself names (conductor-timeline/core, v2-06)
**Date:** 2026-08-13
**Commits:** none since `last_wrap` (2026-08-13T17:02:18Z) — this wrap's commit is the first

## Changes (structured — detectors read this)

- **Files:** (from `git status`) — 8 chunk-work modified · 4 new snapshots · 5 bookkeeping
  - modified (chunk work): `crates/conductor-core/src/load_envelope.rs` · `crates/conductor-core/src/lib.rs` · `crates/conductor-run/src/lib.rs` · `crates/conductor-run/tests/dispatch_wire.rs` · `crates/conductor-run/Cargo.toml` · `crates/conductor-timeline/tests/pacing.rs` · `contracts/pulse-load-envelope.toml` · `Cargo.lock`
  - new: `crates/conductor-timeline/tests/snapshots/pacing__fixture_emission_stream_seed_{4317017,7}.snap` · `crates/conductor-run/tests/snapshots/dispatch_wire__storm_stream_seed_{4317017,7}.snap` · the chunk folder `conductor-0.2.0/chunks/2026-08-13-dispatcher-determinism-goldens/` · the phase run-dir `.andromeda/runs/2026-08-13T17-25-00-phase/`
  - bookkeeping: `.andromeda/master-route.md` · `conductor-0.2.0/working-route.md` · `conductor-0.2.0/verification-matrix.json` · `.claude/session-handoff.md` · `.andromeda/friction-log.ndjson`
- **Symbols / APIs:**
  - NEW public: `conductor_core::sustained_storm_ms(&Scenario) -> u64` (a scenario's longest single EMITTING phase), re-exported from `lib.rs` beside `scenario_duration_ms`.
  - CHANGED behaviour, unchanged signatures: `check_load_envelope(&LoadEnvelope, &[Scenario])` and `LoadEnvelope::classify(&Scenario)` — both now judge PER EMITTING PHASE against the two sustained terms instead of whole-scenario duration.
  - NEW private (in `load_envelope`): `phase_breach` (the single shared basis both consumers read), `phase_rate_exceeds`, `BreachTerm`, `PhaseBreach`, `LoadEnvelope::breach_detail`.
  - No new IPC method, endpoint, event, socket, port, or env var. No `CONDUCTOR_*` handle added.
- **Crates / modules:** none added or removed. Changed: `conductor-core::load_envelope` (assertion basis), `conductor-run` (test surface + one dev-dep), `conductor-timeline` (test surface).
- **Dependencies:** `insta` added as a **dev-dependency of `conductor-run`** (already a workspace dep and already in `Cargo.lock` via `conductor-timeline`). `Cargo.lock` moved **1 line** — an `insta` edge inside `conductor-run`'s existing package entry; **zero new `[[package]]` entries**. No runtime dependency added, no version bumped.
- **Schema / config:** `contracts/pulse-load-envelope.toml` — the `[[exempt]]` ledger retired from 2 entries to **0**; the asserted term set inverted (`max_sustained_rate_spans_per_s` + `max_sustained_storm_ms` now asserted per emitting phase; `max_scenario_duration_ms` recorded-but-no-longer-asserted). No struct field added or removed (`exempt` is `#[serde(default)]`, so the empty ledger needs no schema change). No migration.
- **Spec-master edits:** none at report time — the two owed master edits are P2's work (see *Deviations*).
- **Counts / qualifiers moved:**
  - the load-envelope **`[[exempt]]` ledger: 2 → 0**. Docs stating it: `architecture.md` §Occupied Resources names the SET (`an [[exempt]] ledger of {scenario, reason}`) rather than a literal, which is the correct form and not a stale hit; `security-plan.md` §Input Validation names the validation rule ("no duplicate or reason-less `[[exempt]]` entry"), still true; `layout-templates.md` §Surface: cli ("omitted entirely when the run is in-envelope or pinned exempt"), still true.
  - **which envelope term is asserted** changed. `architecture.md:160` states "Only the scenario-duration term is asserted — the rate terms are **derivable but deliberately not yet asserted**" and predicts re-scoping "to emitting-phase duration"; both clauses are now stale (see *Deviations*). The same wording echoes into the distillation `.claude/rules/testing.md:64`.
- **Dev-tool versions:** none. `cargo-audit` remains 0.22.2 (the latest published); no tool installed or upgraded.
- **Reverted / negative API facts:** the planned term — **summed emitting-phase duration** — was authored into the plan and then deliberately NOT implemented. Measured across all 35 committed scenarios it leaves both prior exemptions over the ceiling (`activity-floor` 900 000 ms, `incident-auto-resolution` 610 000 ms), so it changes no gate verdict and retiring the exemptions under it would turn the gate red. The shipped term is a per-phase joint bound instead. Recorded as a PREMISE-CORRECTION in `scope.md`, in `verification-matrix.json#v2-06` `notes`, and in the contract + module doc.
- **Coverage of new surfaces:**
  - `conductor_core::sustained_storm_ms` (new public fn, pure over committed config) → validation n/a (no external input; the manifest it reads is already serde+`validate()` bounds-checked at load) · instrumentation n/a (pure, no I/O) · PII n/a · tests **unit ✓** · a11y n/a · tokens n/a
  - re-scoped `check_load_envelope` / `classify` (changed judgement on an existing surface) → validation ✓ (unchanged `LoadEnvelope::validate()`, still rejecting empty identity/provenance, non-positive terms, reason-less/duplicate exemptions) · instrumentation ✓ (existing `tracing::info!(count = …)` on load, observed emitting `count: 0` on the live smoke) · PII n/a · tests **unit ✓** (13 cases incl. all three ledger directions + the exact-boundary case) · a11y n/a · tokens ✓ (no new palette row, ANSI code, bracket label, lamp state, or sixth `ReportState`; the breach still renders as the existing run-level `[ENVIRONMENT-SUSPECT]` caption)
  - emission-stream goldens (2 tiers × 2 seeds, new committed artifacts) → validation n/a · instrumentation n/a · PII **redacted ✓** (no host path; the dispatch-tier projection excludes every `*_time_unix_nano` field, so no wall-clock value enters a committed artifact) · tests **unit ✓ + property ✓** · a11y n/a · tokens n/a

## Deviations from intent

1. **The plan's CARRY step (a) was implemented by a different term than worded — operator-approved.** The plan (following `contracts/pulse-load-envelope.toml:40-42`) specified summed emitting-phase duration. P3 research measured that it achieves none of the CARRY's stated purpose. The shipped term is a **per-phase joint bound** (no emitting phase over `max_sustained_storm_ms`; none over `max_sustained_rate_spans_per_s`), under which every catalog scenario passes unaided — longest single emitting phase 600 000 ms (exactly at the ceiling), peak rate 4.00 spans/s against 10 000 — so the ledger genuinely retires to `[]`. Approved at the phase P5 review before any code was written.
2. **Both `scenario_duration_ms` consumers re-scoped, not just the gate — operator-approved.** The CARRY named only `check_load_envelope`; `LoadEnvelope::classify` shares the same helper. Leaving them split would let a scenario pass the static gate while a live run captioned it `[ENVIRONMENT-SUSPECT]` on the old basis. Both moved in one commit, with a test asserting they agree across the committed catalog.
3. **Two files edited outside the plan's boundary list**, both direct enablers of listed work: `crates/conductor-run/Cargo.toml` (the `insta` dev-dep, without which plan step 2's golden cannot compile) and `crates/conductor-core/src/lib.rs` (a one-line re-export of the newly added `sustained_storm_ms`, otherwise dead code in a private module). Judged in-scope helpers under the gray-area rule rather than soft-exits.
4. **The proptest landed in `conductor-timeline`, not `conductor-run`.** Plan step 3 named no crate; the timeline tier already carries proptest and needs no gRPC stub per case, which would have risked flakiness against the zero-retry bar.
5. **One acceptance clause had no test behind it.** `v2-06`'s concretized acceptance ends with the LOAD ENVELOPE pointer header being present in all 35 scenario files — verified by hand at phase P5 but untested. Added `every_committed_scenario_points_authors_at_the_envelope` rather than setting `implemented` on a manual check.
6. **Expected amendments (for P2).** Two are owed as master work: `architecture.md` §Occupied Resources (the load-envelope bullet asserts the wrong term set and predicts a re-scope that measurement falsified) and `test-plan.md` §7 Golden artifacts (the committed golden inventory gains two families, `pacing__*` and `dispatch_wire__*`, beside the existing `replay__*`). A third — `scheduler.rs`'s "Total elapsed time per phase is exactly the jittered gap" — is **not amendment-flow material**: all seven masters grep clean of that claim; the wording lives only in `crates/conductor-timeline/src/scheduler.rs:69`, a code doc-comment, and wrap does not edit source. It is resolved at route-resolve as a ride-along CARRY instead.

## Decisions & corrections

- **Operator-approved at the phase P5 review (3 questions, 1 round):** the per-phase sustained-storm term over the literal CARRY wording; both tiers of golden plus a seed-space proptest over a single tier; both `scenario_duration_ms` consumers re-scoped together.
- **A spec↔reality gap SURFACED during the fix-loop, not authored.** `scheduler.rs:69` documents that a phase's total elapsed time is exactly its jittered gap. That holds for the reported `PhaseTransition` stream but not for clock consumption: `paced_slices` divides a gap into sub-millisecond slices and `tokio::time` rounds each sleep **up** to its 1 ms tick, so a phase's emissions overshoot its window by up to one tick each and the excess carries forward. Measured: phase 0's boundary is 12 010 ms, its last emission fires at 12 012 ms across 6 emissions. Out of scope by the chunk's own boundary ("this chunk freezes the behavior, it does not redesign it"), so the true bound is now asserted by a permanent test rather than left implicit.
- **Two tests I authored asserted things that are not true and were corrected to assert the truth** — never the code changed to match. The rate-breach fixture used 10 emissions in 1 ms believing it far exceeded 10 000/s when it is exactly 10 000/s; a new cross-stream invariant asserted a phase's last emission lands exactly on its boundary, which the timer granularity above makes false.
- **Goldens were hand-accepted after verification, never via `cargo insta review`.** Each `.snap.new` was checked for the deterministic expected shape (18 entries matching the declared 6+12 occurrences, correct occurrence ordering, monotonic offsets, two seeds diverging completely, no wall-clock field) before being written as a baseline with the transient `assertion_line:` header stripped, per the 2026-06-22 re-baselining rule.
- **The plan's smoke command was insufficient for its own change.** It listed the bare `agent-run.sh run`, which skips the scenario leg — the only leg exercising the `classify_run` path this chunk changed. The Tier-2 rule curated after the previous chunk caught it before any artifact was read; the leg was fired under `SCENARIO=fingerprint-storm` and freshness confirmed against a pre-run UTC marker.
- **`cargo audit` — ELEVENTH consecutive red, re-pinned silently** under the operator's L5 ratification (origin `2026-08-08-sut-capability-manifest`). Byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` on 0.22.2, true exit 1 — an advisory-DATABASE fault with nothing to raise a floor to. `cargo deny check advisories bans licenses sources` verified exit 0 as the overlapping signal. Standing basis re-verified literally: `Cargo.lock` moved 1 line, zero new `[[package]]`. No floor raise, no `deny.toml` ignore, no CI edit.

## Outcome

All 10 acceptance criteria met. Gates green:

| Gate | Result |
|---|---|
| `cargo nextest run --workspace --profile ci` | **558/558**, zero retries (+14) |
| `cargo nextest run -p conductor-{core,timeline,run}` | 250 / 24 / 23 — all green |
| `cargo test --workspace --doc` | 7 suites ok |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo deny check advisories bans licenses sources` | all four ok, exit 0 |
| `cargo audit` | exit 1 — advisory-DB fault, re-pinned (see *Decisions*) |

Green in 3 fix-loop iterations, no soft-exit trigger, no gate deferral — every changed-surface gate ran plus the full workspace suite.

**Smoke ✓** (boot-path exercised): `agent-run.sh run` exit 0; `SCENARIO=fingerprint-storm agent-run.sh run` → `[BLOCKED]` exit 0 with artifact freshness confirmed (marker 18:16:08Z, artifact 18:16:22Z, `run_id 2026-08-13T18-16-22-290`); the envelope loaded with `count: 0`, proving the retired ledger on the live path, and no `[ENVIRONMENT-SUSPECT]` caption appeared. `status` and `logs` both truthful, Blocked-row NULL rule honored.

`v2-06` recorded `status: implemented` with 14 satisfying test ids; its PREMISE-CORRECTION note is preserved.
