# Codebase Research — 2026-06-21-expected-outcome-slo-timing-model

## Scope
- **Depth:** moderate · **Reads:** 9 (run_record, scenario, phase_spec, verdict[core], report_state, verify/lib, verify/error, verify/verdict, verify/Cargo + a scenario TOML) · **Graph queries:** 1 (impact of SloTier/classify/Assessment/RunRecord)

## Files inspected
- `crates/conductor-verify/src/verdict.rs` (full) — the direct upstream: `classify(class: ClaimClass, matched: bool, observed, expected) → Assessment`. `matched` is a **pre-computed** bool; there is **no timing input**. `Assessment { verdict, observed, expected, delta }`, `observed`/`expected` redacted via `redact_value`. `ClaimClass { Hard, CalibrationRegion }` lives here.
- `crates/conductor-core/src/scenario.rs` (full) — **`SloTier { Tier5s, Tier20s, Tier90s }` already exists here** (serde-renamed `<5s`/`<20s`/`<90s`). `Scenario` **already carries `slo_tier: SloTier`** (`#[garde(skip)]`). garde discipline: `#[derive(Validate)]`, `#[garde(custom(...))]` field validators, `dive` into `Vec<PhaseSpec>`, `from_toml_str → CoreError::Config | ::Validation`. Stale comment at L97: "latency-target ordering (p50≤p95≤p99) join it when the Epoch-3 latency spec lands" — never added; a candidate cross-field invariant for the expected block.
- `crates/conductor-core/src/phase_spec.rs` (full) — **the forward-compat config-extension precedent:** `EmissionSpec` is `#[non_exhaustive]` + the field is `#[serde(default)]` + `#[garde(skip)]`, so a new optional config member defaults and does NOT break existing `scenarios/*.toml`. Bounds are `const MAX_*` + `#[garde(range(max = ...))]`.
- `crates/conductor-core/src/run_record.rs` (full) — the 11-field envelope `RunRecord` **already exists**: `latency_ms: Option<i64>` (INTEGER ms, journal-relative `read_back_observed_at − journal_emitted_at`), `slo_tier: SloTier`, `verdict: Option<Verdict>`, `state: ReportState`. Schema owned by tests/obs (test-plan §3). **This chunk computes `latency_ms` + `verdict`; it does NOT reshape the envelope.** `RunRecord::blocked(..)` is the only constructor so far.
- `crates/conductor-core/src/verdict.rs` + `report_state.rs` (full) — `Verdict { Pass, Fail, CalibrationRegion }` (CalibrationRegion labels `HOLD`/`[HOLD]`); `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`. **No `Verdict → ReportState` mapping fn exists** — that belongs to the Epoch-6 run-report serializer, NOT this chunk.
- `crates/conductor-verify/src/lib.rs` + `error.rs` + `Cargo.toml` — verify re-exports `{Assessment, ClaimClass, classify}`. `VerifyError` is `#[non_exhaustive]` ("later Epoch-5 chunks extend the surface"). **verify deps: serde, serde_json, toml, rmcp, tokio, thiserror, tracing — NO `garde`.** (core has garde.)
- `scenarios/error-baseline-spike.toml` — current scenario shape: `name`, `p_ids`, `seed`, `slo_tier`, `jitter_ms`, `[[phases]]`. **No `expected` block today.**

## Graph impact (code-graph query → tree-query-2026-06-21-expected-outcome-slo-timing-model.json)
- **`RunRecord`** — referenced in `conductor-core` (lib.rs, run_record.rs) + **`conductor-report/src/journal.rs`** (the JSONL writer). This chunk does not touch RunRecord, so the report seam is unaffected.
- **`SloTier`** — consumed by `Scenario` + `RunRecord` (both core). Adding an inherent method (e.g. `deadline_ms()`) is additive/low-risk.
- **`classify` / `Assessment` / `ClaimClass`** — shipped last chunk with **no downstream callers yet** (the read-back→evaluate→classify→envelope flow is Epoch 6/8). So moving `ClaimClass` to core touches only `verify/verdict.rs` + `verify/lib.rs` (a re-export keeps it source-compatible). Star topology intact: verify→core only.

## Patterns detected
- **Forward-compat optional config** (`phase_spec.rs:34`, `EmissionSpec`): `#[serde(default)]` + `#[non_exhaustive]` lets new config ride existing fixtures — the template for an optional `expected` on `Scenario`.
- **Closed serde-renamed enum** (`scenario.rs:37` `SloTier`; `phase_spec.rs:60` `Signal`): wire forms locked by a golden `assert_eq!` on `to_string`. `ComparisonKind` would follow this (snake_case or explicit rename + a spelling golden).
- **garde field + cross-field validation** (`scenario.rs:19,98`): `#[garde(custom(fn))]` for field rules; container-level cross-field done on the field it concerns (garde 0.22.1 has no container `custom`). The expected block's ordering invariants (p50≤p95≤p99) follow this.
- **Infallible value-returning evaluator** (`verdict.rs:61` `classify`): returns a value, never `Result`; redaction at capture. The new comparison/SLO evaluator mirrors this (verdict/error wall).
- **Exact-string serialization goldens** (`run_record.rs:99`, `verdict.rs:98`): unit goldens are exact `assert_eq!` on `serde_json::to_string`, not insta.

## Conventions to follow
- **Config in core, serde + garde, validated at load** (`scenario.rs:88`): if the expected model is config, it lives in core with the rest of the scenario model and validates via garde → `CoreError::Validation`.
- **`std::time` journal-relative latency, never the virtual clock** (`run_record.rs:14`, obs/tests/security extracts): `latency_ms = read_back_observed_at − journal_emitted_at` as `i64` ms.
- **Redact `observed`/`expected` at capture** (`verdict.rs:69`): any value flowing into an `Assessment`/artifact passes `redact_value` (security + obs extracts).
- **Closed `SloTier` stays exactly 3 variants** (arch/obs/layouts extracts): consume it; do not add variants or a parallel string.

## New files to create
- `crates/conductor-verify/src/slo.rs` (or `compare.rs`) — the evaluator: comparison-kind application (`observed` vs `expected` → `matched: bool`), the journal-relative SLO deadline check (`SloTier` + two instants → latency_ms + within/exceeded), and the combiner that feeds `(matched, ClaimClass)` into `classify`. Infallible.
- `crates/conductor-verify/tests/<name>.rs` — integration tests: comparison-kind matrix (`#[rstest] #[case]`), SLO tier/deadline matrix, determinism (same inputs ⇒ identical Assessment), redaction.
- **(seam-dependent — see Open questions)** the expected-outcome data model (`ComparisonKind`, `ExpectedCheck`/`ExpectedOutcome`) in **core** (`crates/conductor-core/src/expected.rs`) or **verify**.

## Files to modify
- `crates/conductor-verify/src/lib.rs` — `mod` + re-export the new evaluator surface; doc reword.
- **(seam-dependent)** `crates/conductor-core/src/{lib.rs, scenario.rs, verdict.rs?}` — if the model lives in core: add `mod expected` + re-exports, possibly move `ClaimClass` core-ward, optionally an optional `expected` field on `Scenario`. OR `crates/conductor-verify/Cargo.toml` — add `garde` if the model lives in verify.

## Open questions
1. **Crate seam for the expected-outcome model + `ClaimClass` home** (the P4 decision — AskUserQuestion). Core-with-model (move `ClaimClass` to core) vs self-contained-in-verify (verify gains `garde`). Bears on whether the expected block can ride the scenario TOML and honor "config in core".
2. **Attach optional `expected` to `Scenario` now (EmissionSpec precedent) or defer the per-P-ID TOML wiring to Epoch 7** (the scenario-catalog epoch). The scope boundary already excludes the 60 filled-in blocks.
3. **SLO tolerance shape:** model "tier-scaled bounded slack" as `within ⟺ latency_ms < tier_bound` (the tier IS the band) + a fixed slack constant; a *runtime hardware-profile* detector is read as OUT of scope for this chunk (flag at P5). No user question — a stated plan assumption.
