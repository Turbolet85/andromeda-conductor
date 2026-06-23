# Codebase Research — 2026-06-23-scrub-pipeline-degraded-report-surface-scenarios

## Scope
- **Depth:** moderate (mature scenario-model; the catalog pattern is established over 7 prior Epoch-7 chunks) · **Reads:** 6 · **Globs/Greps:** 4 · **Code-graph queries:** 2 (trace at `runs/2026-06-23T16-12-27Z-phase/tree-query-*.json`)
- **Distillers (P2):** arch=partial, security=partial, **tests=relevant**, obs=partial (envelope/instrumentation bindings, Epoch-8/9), design=partial + layouts=partial (render/cli-parity bindings, Epoch-8/9), a11y=No domain coverage. Actionable domains for a TOML+test chunk: **arch / security / tests**; design/layouts/obs land mostly as Epoch-8/9 contract bindings.

## Files inspected
- `crates/conductor-core/src/scenario.rs` (model 63–97; tests 127–774) — `Scenario.expected: Vec<ExpectedCheck>` is `#[serde(default)]` (empty = declare-only → `Lamp::Manual`); per-family loader rstest + suite guards; **the `#[cfg(test)] mod tests` closes at line 774** (append the new family before that brace).
- `crates/conductor-core/src/expected.rs` (14–64) — `ClaimClass{Hard,CalibrationRegion}`, `ComparisonKind{Exact,Contains,Absent,CountAtLeast}` (Contains/Absent are substring-tolerant), `ExpectedCheck{kind,class,expected:String(min len 1)}`. **All needed kinds already exist — zero model change.**
- `crates/conductor-core/src/coverage.rs` (107–130, anchor test 161–163) — authoritative classification of the 13 P-IDs: **Auto×8** (P-035/045/047/048/052/053/055/056), **DriveObserve×1** (P-037), **StaticOnly×4** (P-049/050/051/054). No change needed (already classified).
- `crates/conductor-emit/src/pii.rs` (33–248) — `PiiCorpus::seeded(seed)` + `value(category)` + `pii_trace_request`/`pii_logs_request`; 7 categories with seed-**derived** values but **stable category sentinels** (`@example.com`, `sk_live_`, `Bearer `, `password=`) and stable `field_key()` structural keys.
- `scenarios/{error-baseline-spike,halo-breathing-encoding,project-context-grounding,root-span-error-scope}.toml` — representative TOML shapes (all-Hard, empty-expected DriveObserve, declare-only KnownResidual, CalibrationRegion). TOMLs declare phases + `expected` + `slo_tier`; **the emitter binding is NOT in the TOML** (declared intent, wired Epoch-8) — same for every catalog entry.

## Graph impact (from the code-graph query)
- **`Scenario::from_toml_str`** — referenced **only** within `crates/conductor-core/src/scenario.rs` (test loaders @ `scenario.rs:256,265,283,294,307,321,333,349,…`). Adding test call sites is purely additive, same file, same pattern. **0 production callers.**
- **crate `conductor-core`** — 5 inbound consumers (`conductor-{cli,report,tauri,timeline,verify}`), but this change touches **no public symbol** (new test fns + new TOML data only); the `Scenario` API is unchanged ⇒ **zero cross-crate blast radius.**

## Patterns detected
- **Per-family loader rstest** (`scenario.rs:328–363` statistical-anomaly; `699–747` constellation): `#[case("<stem>", &["P-NNN"])]` rows reading `../../scenarios/{stem}.toml`, asserting `name`/`p_ids`; the empty-shape families drop the `!expected.is_empty()` assertion.
- **Per-shape guards** (`scenario.rs:340–363`, `707–747`): all-Hard, `any(Contains)`/`any(CountAtLeast)`/`any(Absent)`; the constellation chunk asserts `is_empty()` for operator-checklist/declare-only members.
- **Suite guard shift** (`scenario.rs:749–773`): constellation moved from `all(non-empty)` to `any(empty) && any(non-empty)` — the precedent this chunk reuses (Hard + declare-only mix).
- **Empty-`expected` DriveObserve / declare-only** (`halo-breathing-encoding.toml`, `project-context-grounding.toml`): omit the `[[expected]]` block entirely.

## Conventions to follow
- TOML wire forms: `p_ids = ["P-035", …]` (transparent `PId`); `slo_tier = "<5s"|"<20s"|"<90s"`; `class = "Hard"`; `kind = "Absent"|"Contains"|"CountAtLeast"` (PascalCase).
- **Seed namespace `4317<NNN>` per P-ID** (constellation used 4317025..036; severity 4317019..023). New seeds 4317035/037/045/052/053/055/056 must feed no golden — verify at implement-time per the 2026-06-22 golden-blast learning (grep `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<seed>.snap`).
- garde-validate at load (`Scenario::from_toml_str` → `CoreError::Validation`); every phase `name` 1–40 chars, `gap_ms` ≤ 3.6e6, `jitter_ms` ≤ 60000.
- **`Absent` and `Contains` of the SAME token must never share a scenario** (2026-06-22 learning) — pii-scrub uses `Absent` on PII sentinels + `Contains` on a *different* structural token.

## New files to create
- `scenarios/pii-scrub.toml` — P-035/047/048; Hard `Absent` over PII sentinels (`@example.com`, `sk_live_`, `Bearer `, `password=`) + Hard `Contains` a structural marker; `<5s`; seed 4317035.
- `scenarios/report-render-surface.toml` — P-037; empty `expected` (operator-checklist/ManualCheck); `<5s` (render <2s); seed 4317037.
- `scenarios/findings-counter-refresh.toml` — P-045; Hard `CountAtLeast "<N>"`; `<5s` (counter ≤1s); seed 4317045.
- `scenarios/cadence-config.toml` — P-052; empty `expected` (operator-config + Epoch-8 measure); `<20s`; seed 4317052.
- `scenarios/threshold-hot-reload.toml` — P-055/056; MIXED — Hard `Absent "<retroactive cue>"` (P-056) + declare-only timing (P-055); `<5s` (reload <2s); seed 4317055.
- `scenarios/degraded-mode-report.toml` — P-053; empty `expected` + documented degraded_mode KnownResidual; `<20s`; seed 4317053.

## Files to modify
- `crates/conductor-core/src/scenario.rs` — **test wiring only** (append before line 774): a loader rstest (6 cases, NO blanket `is_empty` assertion), per-shape guards (pii-scrub Hard Absent+Contains; findings-counter Hard CountAtLeast; threshold-hot-reload P-056 Hard Absent; declare-only `is_empty()` for render/cadence/degraded), and a suite guard exercising `any(non-empty) && any(empty)`. No struct/enum change.

## Open questions
- **pii-scrub `Contains` structural marker token** — the strongest leg is the `Absent` PII sentinels (the P-047/P-048 core); the "structure preserved" `Contains` token is inferred (a surviving service/span name or `field_key`) → declare it substring-tolerant + flag as an Epoch-8 calibration point (P-018/P-036 precedent). Resolve exact token at implement-time against `pii.rs`/Pulse contract.
- **SLO tier choices** (cadence/degraded `<20s`; the timing families `<5s`) are impl picks (the TOML declares the tier; the per-tier *measurement* is Epoch-8) — left open by the route, decided in the plan (deviation-allowed, severity-lifecycle precedent).
- **findings-counter `N`** — the deterministic count of emitted unread/active incidents; pick a small fixed N in the phases (e.g. 3) and assert `CountAtLeast "3"`.
