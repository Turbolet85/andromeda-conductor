# Codebase Research — 2026-06-22-error-baseline-spike-latency-regression-scenarios

## Scope
- **Depth:** moderate · **Reads:** 5 (`expected.rs`, `phase_spec.rs`, `scenario.rs`, `error-baseline-spike.toml`, `span-status-error-detection.toml`) · **Globs/Greps:** 5 (CountAtLeast usage · error-baseline-spike refs · SloTier · candidate tokens · ch1/ch2 `[[expected]]` shape) · **Code-graph:** 1 query (`from_toml_str` callers → `tree-query-*.json` adoption trace).

## Files inspected
- `crates/conductor-core/src/expected.rs` (full) — `ComparisonKind { Exact, Contains, Absent, CountAtLeast }`, `ClaimClass { Hard, CalibrationRegion }`, `ExpectedCheck { kind, class, expected: String (len ≥1) }`. **`CountAtLeast` is documented verbatim as "the sample-count floor (≥50-sample latency / ≥10-span error-rate)"** — the exact P-009/P-011 floors. No tolerance/approximate-match kind exists.
- `crates/conductor-core/src/phase_spec.rs` (full) — `PhaseSpec { name (1..=40), gap_ms (≤ MAX_GAP_MS = 3_600_000), emission: EmissionSpec { signal: Signal } }`. **No duration/repeat field** — a phase has a single base gap. `Signal { Traces (default), Metrics, Logs }`. `EmissionSpec` is `#[non_exhaustive]` (Epoch-3 seam extends it). `MAX_GAP_MS` = 1h ⇒ the 90s/60s recipe windows fit in `gap_ms` directly.
- `crates/conductor-core/src/scenario.rs` (full, 418 ln) — `Scenario { name, p_ids: Vec<PId>, seed, slo_tier: SloTier, phases: Vec<PhaseSpec>, jitter_ms, expected: Vec<ExpectedCheck> (#[serde(default)]) }`; `from_toml_str` → `CoreError::Config`/`Validation` (verdict/error wall). `SloTier { Tier5s "<5s", Tier20s "<20s", Tier90s "<90s" }`. Garde: `p_ids` non-empty + `P-NNN` 001..=060 + no-duplicate; `phases` non-empty `dive`; `expected` `dive`. **The existing fixture rstests assert `p_ids == vec![PId(single)]`** — a 2-P-ID scenario needs a new test shape.
- `scenarios/error-baseline-spike.toml` (full) — **already exists** (P-009/P-010, seed 424242, `<5s`, phases `baseline` gap 2000 + `spike` gap 1000) but **carries NO `[[expected]]` block** (placeholder gaps from its schema-exercise origin). This chunk formalizes it.
- `scenarios/span-status-error-detection.toml` + greps of `receiver-lifecycle-state.toml` / `high-severity-log-capture.toml` — the `[[expected]] { kind, class, expected }` shape (e.g. `Contains`/`Hard`/`"Stalled"`; the two-sided `Contains`/`"ERROR"` + `Absent`/`"WARN"`).

## Graph impact (from the code-graph query)
- **`Scenario::from_toml_str`** — all callers are **tests inside `scenario.rs`** (`committed_fixture_loads_and_validates` :294, `connection_lifecycle_fixtures_…` :307, `hard_signal_fixtures_…` (via the same path), the malformed/invalid-reject tests, the P-007/P-008 guards). **No production caller** (the Epoch-8 CLI driver does not exist yet). ⇒ adding 2 TOMLs + 1 rstest has **zero blast radius** beyond the `scenario.rs` test module; no symbol signature changes.
- **`ComparisonKind::CountAtLeast`** — consumed only by `conductor-verify/src/slo.rs` (`compare` + the `class = if CountAtLeast && !value_matched { calibration }` rule, :79) and its tests, already passing for `"50"`. Adding TOML checks of this kind exercises an already-wired, already-tested path; **no verify change**.
- **`"error-baseline-spike"` (string literal)** — appears in `run_record.rs` :118/:135 + `db.rs` :221 golden tests as the scenario *name*, not a file load. Editing the TOML file does **not** touch these. (They pin the run-report envelope / runs.db row shape.)

## Patterns detected
- **Catalog TOML shape** (`scenarios/*.toml`, ch1/ch2): `name · p_ids · seed · slo_tier · jitter_ms` + `[[phases]]{name, gap_ms}` + `[[expected]]{kind, class, expected}`. A header comment cites the P-ID + spec clause + the Hard/CalibrationRegion rationale (e.g. `span-status-error-detection.toml:1-4`).
- **Fixture rstest** (`scenario.rs:305`, `:319`): `#[rstest] #[case(stem, p_id)]` → read `scenarios/{stem}.toml` → `from_toml_str` → assert `name == stem`, `p_ids == vec![PId(p_id)]`, `!expected.is_empty()`. **The single-`p_id` assertion does not fit a 2-P-ID scenario** — the new test takes a P-ID **slice**.
- **Class-guard tests** (`scenario.rs:329` P-008 all-CalibrationRegion; `:343` P-007 two-sided Contains+Absent): a targeted `#[test]` asserting a P-ID-specific class/kind invariant. The all-`Hard` counterpart here is a guard asserting every check on both new scenarios is `class == Hard`.
- **`CountAtLeast` semantics** (`slo.rs:79`): declared `class = Hard` + floor **met** ⇒ hard pass; floor **unmet** ⇒ evaluator softens to calibration (never hard-fail a not-yet-converged baseline). Exactly the P-009/P-011 baseline-floor semantics.

## Conventions to follow
- **Verdict/error wall** (`scenario.rs:107`): `from_toml_str` returns `Err` only on parse/validation (harness fault); the verdict/state is a downstream value. The catalog only declares; the evaluator (Epoch-8) classifies.
- **Declare-only deferral** (ch2 precedent): tolerance math (±10%/±15%) + persistence-window + SLO timing are the Epoch-8 evaluator's; the TOML declares phases + expected markers only (`scenario.rs:116` notes the p50≤p95≤p99 invariants "join when the Epoch-3 latency spec lands" — not this chunk).
- **Seed convention**: ch2 used P-ID-derived seeds (`4317005` for P-005). `error-baseline-spike` keeps its canonical `424242`; the new `latency-regression` takes a distinct traceable seed (`4317011`, P-011-derived).
- **Header comment** citing `pulse-capability-spec.md §3` per-P-ID + the floor/multiplier/persistence numbers.

## New files to create
- `scenarios/latency-regression.toml` — P-011/P-012; seed `4317011`; `slo_tier = "<20s"` (P-012 has no explicit p99 budget and a 60s-persistence signal — the middle tier, vs `<5s`); phases `baseline` (gap 90000) + `ramp` (gap 90000, Traces); `[[expected]]` `CountAtLeast`/`Hard`/`"50"` (P-011 50-sample floor) + `Contains`/`Hard`/`"LatencyRegression"` (P-012 candidate — **inferred token**, Epoch-8 calibration point).

## Files to modify
- `scenarios/error-baseline-spike.toml` — formalize: header comment (P-009/P-010 + §3 recipe); retune phase gaps `baseline` 2000→90000 (90s converge) + `spike` 1000→60000 (60s @ 3.5×, ≥30s persistence); add `[[expected]]` `CountAtLeast`/`Hard`/`"10"` (P-009 10-span floor) + `Contains`/`Hard`/`"ErrorRateSpike"` (P-010 candidate — **spec-confirmed token**, spec line 244). Keep `name`, `p_ids`, `seed 424242`, `jitter_ms 50`.
- `crates/conductor-core/src/scenario.rs` (test module only) — add a `statistical_anomaly_fixtures_load_and_validate` `#[rstest]` taking a P-ID **slice** (`#[case("error-baseline-spike", &["P-009","P-010"])]`, `#[case("latency-regression", &["P-011","P-012"])]`) asserting load + `p_ids` match + `!expected.is_empty()`; plus an all-`Hard` guard `#[test]` asserting every check on both files is `class == ClaimClass::Hard`, and (optionally) a guard asserting the `CountAtLeast` floor check is present on each. **No change to any non-test code.**

## Open questions
- **File granularity (P4 AskUserQuestion):** 2 files (per-pair — the architecture's `error-baseline-spike` precedent + the baseline→detection timeline dependency) vs 4 (per-P-ID, ch1/ch2 mirror). Recommend **2**. *(Resolved-leaning; confirm with the user per the ch1/ch2 catalog-shape convention.)*
- **P-012 SLO tier** `<20s` vs `<5s` — minor; declared bucket, Epoch-8 calibrates the journal-relative measurement. Leaning `<20s`.
- **P-012 candidate token** `"LatencyRegression"` inferred (spec uses lowercase prose; no PascalCase token) — Epoch-8 live-verify calibration point; substring-`Contains` tolerant.
