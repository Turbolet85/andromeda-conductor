# Report — 2026-08-20-latency-regression-re-proof

**Chunk:** latency-regression re-proof — the regression made short relative to accumulated history and read back on a surface that survives the window (conductor-run/emit, scenarios, P-011/P-012)
**Date:** 2026-08-20T17:10:10Z
**Commits:** none since `last_wrap` (2026-08-19T23:31:30Z) — this chunk is uncommitted; the prior HEAD is `a4efd26` (2026-08-19-connection-lifecycle-live-proof)

## Changes (structured — detectors read this)

- **Files:**
  - `scenarios/latency-regression.toml` — phase re-shape (2 phases → 4) + header re-based to the measured mechanism
  - `crates/conductor-run/src/lib.rs` — `route_read_back` + `ReadBack` (crate-private) + 4 unit tests
  - `crates/conductor-run/tests/baseline_harvest.rs` — kind-dependent `cue_line` fixture + 2 verbatim-leg-pin tests
  - `conductor-0.2.0/chunks/2026-08-20-latency-regression-re-proof/evidence/leg-verdict.md` (new)
  - chunk artifacts (`scope.md` · `research.md` · `plan.md`) + `.andromeda/runs/2026-08-20T15-58-10-phase/`
- **Symbols / APIs:** `route_read_back(ReadBackOutcome, bool) -> ReadBack` and `enum ReadBack {Graded, AutoResolved, Blocked}` — both **crate-private** in `conductor-run`; **no public API, no IPC method, no endpoint, no port, no env var** added or changed. `execute_scenario`'s signature is unchanged (the graph query confirmed zero caller threading).
- **Crates / modules:** none added / removed / changed — the edit is internal to `conductor-run`.
- **Dependencies:** **none.** `Cargo.toml` and `Cargo.lock` are byte-untouched this chunk (zero dependency delta).
- **Schema / config:** `scenarios/latency-regression.toml` — declarative phase data only. Four phases (`baseline` 180000ms/180 occ/50 samples · `spike-a` 40000ms/24 occ/1 sample · `baseline-b` 110000ms/110 occ/50 samples · `spike-b` 40000ms/24 occ/1 sample). `name` / `p_ids` / `seed` (4317011) / `slo_tier` (`<90s`) / `jitter_ms` / declare-only (zero `[[expected]]`) all UNCHANGED. No migration, no config key, no violation-schema change.
- **Spec-master edits:** none at authoring time (P2 owns any).
- **Counts / qualifiers moved:**
  - workspace nextest **655 → 661** (+6: 4 routing tests in `conductor-run/src/lib.rs`, 2 leg-pin tests in `baseline_harvest.rs`)
  - `latency-regression` phase count **2 → 4**; its emitting-phase peak declared rate **0.033/s → 1/s** (occurrences/gap, the envelope's basis) and its wire rate **2 spans/s → 50 spans/s**
  - the scenario's measured emission window **~180s → 373.8s** (`latency_ms` 373784)
  - the six-family declare-only list is **unchanged at six** (this chunk moves no family in or out)
  - `UNBACKED_AUTO`, the capability manifest, the coverage classification, and the `[[exempt]]` ledger (still exactly empty) are all **unchanged**
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  1. **`scenarios/latency-regression.toml`'s own header claimed a "5-min window" t-digest baseline.** Measured false: the long baseline is a t-digest PAIR read as union(current, previous) rotating on a ~60s cycle. **Self-corrected in this chunk** — the header is re-based to the measured mechanism. Evidence: `evidence/leg-verdict.md` §Why the re-shape works.
  2. **`.andromeda/input.md:105` states the latency-regression stimulus recipe as "known p50/p95/p99 for 90s (±15%, ≥50-sample floor) → 3× p99 for 90s → candidate after 60s persistence".** That 90/90 shape produces ZERO cues against the live SUT. **Already recorded as disproved at 2026-08-18** (v2-12 `notes` PREMISE-CORRECTION); this chunk supplies the measured replacement. `input.md` is not one of the seven spec masters and is not wrap-amendable — recorded here for disposition, no amendment proposed.
  3. **`contracts/pulse-load-envelope.toml`'s `max_sustained_rate_spans_per_s` is computed as `occurrences / gap_ms`** (`crates/conductor-core/src/load_envelope.rs:174-188`) — i.e. DISPATCHES per second — while a `Latency`/`Ramp` phase puts `samples`/`windows` spans on the wire per dispatch. This chunk widens the divergence **50×** (50 wire spans/s counted as 1/s). Still **~200× under** the 10000/s bound, so no verdict moves today; a denser future scenario could breach the real bound while passing the gate. **SURFACED, not authored** — the fix is Conductor-side (count `occurrences × samples`, or rename the term to what it measures) and needs an owner at route-resolve.
  4. **`research.md`'s mid-leg reading that the long t-digest pair never rotates on this host** (inferred from `baseline tracker tick` returning 0 lines) is corrected in `evidence/leg-verdict.md`: rotation demonstrably occurs (`persistence_seconds` reset 2456 → 68 mid-leg); the tick line is simply not a usable rotation witness here.
- **Coverage of new surfaces:**
  - `route_read_back` (crate-private read-back routing arm) → validation n/a (no external input; it consumes an already-typed `ReadBackOutcome`) · instrumentation ✓ (`tracing::info!` on the residual arm, run_id-carrying, allowlisted) · PII n/a · tests unit ✓ (4: the residual route + two negative twins + the graded route) · a11y n/a (no UI surface) · tokens n/a
  - `scenarios/latency-regression.toml` (re-shaped declarative config) → validation garde ✓ (loads through `Scenario::from_toml_str`; p50≤p95≤p99 per phase, `occurrences`/`samples` within `MAX_OCCURRENCES`, `kind` present) · instrumentation ✓ (per-phase `emit.batch`; no fault phases declared) · PII n/a (synthetic latency spans, no corpus) · tests ✓ (catalog guards `scenario.rs:441-467` + `check_load_envelope` + the live leg) · a11y n/a · tokens n/a

## Deviations from intent

1. **Live-leg invocation: `conductor.exe run … --agent-mode` directly, not the plan's `agent-run.sh boot` then `run`.** Justification: `boot` fires its own preflight canary whose OPEN incident dedupe-blocks the run's canary on the same data dir (`.claude/rules/verification-harness.md`, 2026-08-19 rule (a)). The run's internal preflight IS the gate. Same plan defect the prior chunk recorded and resolved identically. `agent-run.sh status` was still run, and read back the exact `run_id` this leg minted (mint-then-read satisfied).

2. **Fixture alignment widened from `scope` alone to `scope` + `absolute_value`.** The plan's step 4 named `scope: "operation"`; both fields are kind-dependent at the SUT source (`cue/evaluate.rs:85-140` — the latency path carries a latency in ms, the error path an error rate), so shipping one aligned and one error-shaped would have left the fixture half-faithful. The live line confirmed both (`operation`, 3024.0 ms).

3. **The phase re-shape's numbers differ from the approved plan's.** Plan: `baseline` 200 occurrences × 20 samples (~20 spans/s) + spikes 10 × 1 (~0.25/s). Shipped: `baseline` 180 × 50 (**50 spans/s** on the wire) + spikes **24 × 1**, plus the `baseline-b` refill at the same density. Justification is the TOML header's own two bounds, both load-bearing for detectability: **(a)** the anomaly must stay ≤~1% of the long union or it becomes the history it is judged against — 24 spike spans against a 50/s baseline is ~0.3%, where the plan's thinner baseline left far less margin; **(b)** the ≥50-sample floor reads the long pair's CURRENT-window count, which resets at every rotation, so a 50/s baseline re-clears the floor ~1s after each rotation instead of ~2.5s, shrinking the low-φ dead zone. The outcome settles it: **31 cues where the plan's own predecessor shape produced zero.** Recording it here because it is a material out-of-plan change — it was disclosed in the Files/Changes surface but belongs in Deviations. **This is the second consecutive chunk with that omission shape** (the pii chunk disclosed a `scenario.rs` change in Files and omitted it from Deviations); captured in the friction record as a recurring authoring gap, not a one-off.

## Decisions & corrections

- **Ratified at P5, shipped as ratified:** a declare-only scenario whose post-green-preflight read-back finds an empty active list lands the pre-accepted auto-resolve residual (`KnownResidual` / verdict null) instead of `Blocked`. Checks-bearing scenarios and every `CallFailed` keep `Blocked` — a scenario that grades nothing cannot pass falsely on an empty observation, which is what confines the carve-out.
- **The arm shipped UNEXERCISED live, and the report says so.** This leg's read-back found `query_incident_list` = 1: the canary's own error-rate EWMA kept emitting `error_rate_spike` cues (375 of them) that refreshed the incident's `updated_at` and held off the 120s idle auto-resolve, so the row reached `KnownResidual` by the pre-existing DEGRADED path. Unit-tier coverage stands (3 tests pin the three routes); this leg is not evidence the new arm fires. Stated as an honest limit rather than left to be inferred from a green row.
- **Two-pulse design is load-bearing, not insurance.** `persistence_seconds` reset **2456 → 68** mid-leg — direct evidence the long pair rotates and resets both the sample floor and the confidence denominator, on a phase anchored to pulse-app's clock that no scenario can control. Pulse A opened *curious* (magnitude 2.895) while its short window was still baseline-diluted; the rotation landed and the next cues went *suggested* at 3.57. A single pulse would have been a coin flip. (The `persistence_seconds` name-vs-semantics divergence this corroborates — the field is a sample COUNT — is the already-known Pulse-visit intake item #6, not a new finding.)
- **Correction to a mid-leg reading of mine:** I read `baseline tracker tick` = 0 as "the long pair never rotates on this host" and said so mid-flight. The rotation evidence above disproves it; `evidence/leg-verdict.md` carries the correction.
- **A leg failure that was mine, not the SUT's:** the first attempt returned `[BLOCKED]` in 0s because my leg script put a Windows-form `D:/…` entry on `PATH`, which bash split on the drive-letter colon, so the MCP sidecar resolved nowhere. At row level that is indistinguishable from a genuine SUT-side gate failure — worth carrying as a diagnostic rule.
- **PREREQ discharged:** the 32nd consecutive `cargo audit` re-check ran and is red with the byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` (advisory-DATABASE fault). `cargo deny check advisories bans licenses sources` observed **true exit 0** over the committed, un-drifted lock — the overlap verified, not assumed. Remedy stays the bounded wait: no floor raise, no `deny.toml` ignore, no CI edit.

## Outcome

**Acceptance criteria: all met.**

- **The headline measurement — `latency_regression` cues 0 → 31.** The shipped 90/90 ramp produced ZERO across its entire prior leg; the re-shape produced **31 `triage.cue.emit` lines**, priority histogram **27 suggested · 4 curious**, magnitude **3.50–3.68** (over the 3.0 Suggested bar, under the 5.0 Autonomous bar by design), confidence **1.0**, `scope "operation"`, `suppression_bypassed true` via the absolute arm (`absolute_value` 3024–3073 ms > 1000). **First cue at +13s into spike-a**, inside the predicted window. **Both pulses fired** (A: 16:56:48–16:57:21 · B: 16:59:18–16:59:48).
- **Row:** `verdict: null` · `state: "KnownResidual"` (non-Blocked) · `latency_ms: 373784` (journal-relative) · `slo_tier: "<90s"` · `seed: 4317011` (the TOML's — the SEED-forcing retirement holds) · fingerprints the constant payload-invariant `det-*` triple. Exit 0, rendered `[RESIDUAL]`.
- **Ingestion clean at the new density:** every `spans` `duckdb.append` recorded `rows_appended: 50` against 50-span batches — zero PK-collision drops across 290 dispatches. **No false positives:** zero latency cues during either baseline phase.
- **Gates green (commands run):** `cargo nextest run -p conductor-run --profile ci` (92) · `cargo nextest run -p conductor-core --profile ci` (256) · `cargo nextest run --workspace --profile ci` (**661/661**, zero-retry) · `cargo test --doc --workspace` · `cargo clippy --workspace --all-targets -- -D warnings` · `cargo deny check advisories bans licenses sources` (exit 0) · `cargo audit` (exit 1 — the ratified bounded wait, 32nd) · `bash scripts/agent-run.sh status` (read back the minted `run_id`).
- **No gate deferrals** — Rust source delta present, so every Rust gate ran; no language was untouched.
- **No golden re-pin needed:** no committed golden consumes this scenario or its seed (goldens are seeded 424242 / 4317017 / 7) — a consulted-and-empty finding, not a skip.
- **Smoke:** fired (boot-path + harness-listed). The live leg IS the smoke; `status` confirmed the row.
- **Verification matrix: nothing claimed, nothing flips.** No unclaimed capability names P-011/P-012; the routed-forward latency half lives inside the already-`verified` v2-12, whose `notes` name this route entry. Completion is recorded by the master flip plus a v2-12 `notes` addendum.
