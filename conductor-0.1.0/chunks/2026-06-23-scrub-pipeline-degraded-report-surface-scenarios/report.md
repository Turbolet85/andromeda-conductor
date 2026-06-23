# Report — 2026-06-23-scrub-pipeline-degraded-report-surface-scenarios

**Chunk:** Scrub/pipeline/degraded/report-surface scenarios (P-035, P-037, P-045, P-047..P-056) — closes Epoch 7 catalog, zero model change
**Date:** 2026-06-23
**Commits:** (none yet — this wrap commits the chunk)

## Changes (structured — detectors read this)
- **Files:** 6 NEW `scenarios/*.toml` (`pii-scrub`, `report-render-surface`, `findings-counter-refresh`, `cadence-config`, `threshold-hot-reload`, `degraded-mode-report`); 1 MOD `crates/conductor-core/src/scenario.rs` (**test wiring only** — +5 `#[cfg(test)]` fns / +12 rstest cases, appended after the constellation suite guard). Phase/wrap artifacts: `master-route.md`, `working-route.md`, the chunk folder, run-dirs.
- **Symbols / APIs:** **NONE** — no new/changed public fn · IPC method · endpoint · export · port · socket · env var. The only additions are private `#[cfg(test)]` test fns; the `Scenario`/`ExpectedCheck` model is unchanged.
- **Crates / modules:** none added / removed / changed (test wiring inside the existing `conductor-core`; zero cross-crate blast — `Scenario::from_toml_str` has 0 production callers).
- **Dependencies:** none added / bumped.
- **Schema / config:** 6 new declarative scenario config TOMLs, each garde-validated at load via `Scenario::from_toml_str` (existing `Scenario` fields only — no new config KEY, no struct/enum change, no `ComparisonKind` addition). New read-back/detector-output tokens declared in `[[expected]]`: `Absent` sentinels `@example.com` / `sk_live_` / `Bearer ` / `password=` (synthetic PII-category **pattern prefixes** — no key material), `Contains "user.email"` (the email `field_key`, structural metadata), `Absent "RetroactiveReeval"` (inferred P-056 no-fire token), `CountAtLeast "3"`. No DB schema change, no migration.
- **Coverage of new surfaces:**
  - `scenarios/*.toml` (6 config files) → validation **garde✓** (`Scenario::from_toml_str` garde-validates p_ids/phases/jitter/expected at load) · instrumentation **n/a** (declarative config — no runtime operation; the emit/verify/report spans are the Epoch-8 driver's, per obs §4) · PII **n/a-to-Conductor** (the `pii-scrub` scenario's synthetic PII is the PRODUCT fault stream emitted AT Pulse `:4317` to exercise Pulse's P-047 scrubber — that is the test's POINT; Conductor's own self-obs/`runs.db`/run-report carry only the envelope, no raw PII; the TOML sentinels are pattern prefixes, not secrets) · tests **unit✓** (rstest loader + per-shape guards in `conductor-core`) · a11y **n/a** (no UI) · tokens **n/a** (no UI).

## Deviations from intent
All within the plan's explicit "inferred-token / Epoch-8 calibration" guidance — refinements, not scope changes:
1. **pii-scrub Absent sentinels = 4, not all 7 categories** — grounded in `conductor-emit/src/pii.rs::generate()`: only email (`@example.com`), API-key (`sk_live_`), bearer (`Bearer `), secret (`password=`) carry a STABLE substring sentinel; JWT/credit-card/SSN emit seed-varying values with no stable literal → left to the Epoch-8 evaluator. Realizes the plan's "representative subset" with a principled basis.
2. **pii-scrub `Contains` marker = `user.email`** — the email category's retained `field_key` (structural metadata P-048 keeps while scrubbing the value); distinct from the `@example.com` value sentinel ⇒ no Absent/Contains contradiction (testing.md 2026-06-22 rule). The plan left the structural token inferred.
3. **P-056 Absent token = `RetroactiveReeval`** — deliberately a non-candidate retroactive marker (not a prospective candidate type like `ErrorRateSpike`), so it cannot collide with a legitimately-surfaced post-reload candidate. Inferred, flagged Epoch-8 calibration.
4. **findings-counter + threshold-hot-reload share one guard fn** — both single-Hard-check auto members; cosmetic test granularity.

No scope deviation: zero model change held, the 4 StaticOnly P-IDs got no TOML, only `scenarios/` + `scenario.rs` test wiring touched (research's New files / Files-to-modify exactly).

## Decisions & corrections
- **(P4 Q1)** outcome-coherent grouping → 6 files (user decision).
- **(P4 Q2)** Hard where a deterministic read-back token exists, else declare-only (user decision).
- **Zero model change held — 8th consecutive Epoch-7 chunk.** pii-scrub Hard (Absent×4 + Contains); findings-counter Hard `CountAtLeast`; threshold-hot-reload MIXED (P-056 Hard `Absent` + P-055 declare-only timing); report-render / cadence / degraded-mode declare-only (empty `expected`).
- **First catalog `Absent` for PII-scrub negative assertions**; **first MIXED Hard+declare-only single file** (threshold-hot-reload); **first `degraded_mode` KnownResidual** (declare-only + documented; vs P-032's `recent_commits` residual) — both KnownResidual + ManualCheck reached via the producer-assigned / empty-`expected` path (the scenario model has no `state` field).
- **Inferred tokens** (`user.email`, `RetroactiveReeval`, the exact post-scrub PII forms, the cadence/render/reload SLO measurements) are Epoch-8 calibration points (the P-018/P-036 precedent).
- **StaticOnly four (P-049/050/051/054) deliberately get no TOML** — coverage complete via `coverage.rs` classification; authoring a live scenario for a statically-verified claim would be wrong (test-plan §11).
- **Epoch 7 (Scenario catalog) is CLOSED** — every drivable catalog P-ID now has a scenario; coverage matrix unchanged (already classifies all 13).

## Outcome
- **Acceptance criteria met** — all 6 TOMLs load + garde-valid; pii-scrub Hard Absent/Contains over the seeded PiiCorpus sentinels; findings-counter Hard `CountAtLeast`; threshold-hot-reload P-056 Hard `Absent` (+ P-055 declare-only); report-render/cadence/degraded declare empty `expected`; degraded-mode documents the KnownResidual; the 4 StaticOnly untouched.
- **Gates green** (commands run): `cargo nextest run -p conductor-core` → **163/163** (151→163); `cargo nextest run --workspace --profile ci` → **368/368** (356→368); `cargo clippy --workspace --all-targets -- -D warnings` → clean; `cargo test --doc -p conductor-core` → 0; determinism insta goldens **UNCHANGED** (seeds 4317035/037/045/052/053/055 feed none — only 424242/7 feed goldens).
- **Smoke:** `bash scripts/agent-run.sh run` → **exit 0**.
