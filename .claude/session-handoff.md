# Session Handoff

**Last Updated:** 2026-06-22T16:49:47Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-22-error-baseline-spike-latency-regression-scenarios — feat: statistical-anomaly scenarios (P-009..P-012) — error-baseline-spike (formalized) + latency-regression catalog TOMLs; CountAtLeast floor + Contains candidate, all-Hard (conductor-core scenarios)

## Position
- Done: **2026-06-22-error-baseline-spike-latency-regression-scenarios** — **Epoch 7 (Scenario catalog) ch3/8.** 2 statistical-anomaly TOMLs: `error-baseline-spike.toml` **formalized** (P-009/P-010 — gaps→90s/60s recipe + 2 `[[expected]]`) + new `latency-regression.toml` (P-011/P-012). Each pairs a **`CountAtLeast` sample-count floor** (P-009 `"10"` / P-011 `"50"`) with a **`Contains` candidate** (`"ErrorRateSpike"` / `"LatencyRegression"`), **all `class="Hard"`**. **First catalog use of `CountAtLeast`. Zero model change, zero new dependency.**
- Next: **Epoch 7 ch4 — Activity-floor + restart-suppression scenarios (P-013..P-016, P-057)** → `/andromeda-phase` to promote + plan.

## Work done
2 files MOD (`error-baseline-spike.toml` formalized; `conductor-core/src/scenario.rs` +2 rstests/+4 cases — slice-based fixture loader + all-Hard/floor/candidate guard); 1 NEW (`latency-regression.toml`); **2 determinism goldens re-baselined** (`replay__fixture_seed_{424242,7}.snap`). Gates: conductor-core **114/114** (110→114) · workspace **319/319** (315→319) · clippy `-D` clean · doctest 0 · `agent-run.sh run` exit 0. Code-graph 1004n/4194e.

## Drift resolved
1 proposal · 1 escalation resolved · **drift = 0**. arch `D-arch-decisions` claimed "all `class=Hard`" contradicts §Probabilistic-Assertion Policy → **DISMISSED** (misfire: `CountAtLeast`+`class=Hard`+the evaluator's unmet→calibration override at `slo.rs:79` are pre-existing; no contradiction) + **playbook rule added** (prevents re-fire on ch4/ch5 floor chunks). 6/7 docs returned `proposals: []`. **0 spec-body amendments, 0 cascade.**

## Notes
- **Key decisions:** (1) **file granularity (P4 AskUserQuestion):** **2 files per-P-ID-pair**, NOT 4 per-P-ID (detection P-010/P-012 baseline-depends on a shared timeline; matches the architecture's canonical `error-baseline-spike` `p_ids=["P-009","P-010"]`). (2) **`CountAtLeast` floor + `Contains` candidate, all `class="Hard"`** (research): the model has no within-tolerance kind, so the ±10%/±15% baseline-MATCH is declare-only (Epoch-8 evaluator). (3) **Candidate tokens:** `"ErrorRateSpike"` (spec-confirmed) + `"LatencyRegression"` (**inferred** — Epoch-8 live-verify calibration point). (4) **SLO tiers:** `<5s` (error-baseline-spike) / `<20s` (latency-regression — may move to `<90s`).
- **Deviation (curated):** research's scenario-name grep missed the **seed-named** determinism goldens that consume `error-baseline-spike.toml`; the planned gap retune changed their frozen shape → re-baselined (verified deterministic: same seed-driven jitter deltas, only base gaps moved).
- **Curation:** 2 Tier-2 (`testing.md` — seed-named-golden blast-radius; `CountAtLeast`-`class=Hard` scenario-authoring). Filtered 1 (file-granularity = task-specific). 0 conflicts, 0 deferred.
- **Follow-up (carried):** (a) `coverage-matrix.md` not yet at repo root (Epoch-8 cli). (g) Epoch-8 CLI driver realizes the catalog **runtime** legs (scenario emit + live MCP verify against `scenario.expected`); **this chunk's Epoch-8 calibration points: the P-012 `"LatencyRegression"` token + the latency-regression `<20s` tier.** (b) test-plan §3 ↔ obs-plan §3 doc-reconcile still deferred. (c/e/f) report.generate obs span · `opentelemetry-proto default-features=false` trim · Epoch-8/9 reuse of `coverage_matrix()`/`Lamp::for_record`.
- **Last failed command:** none.
