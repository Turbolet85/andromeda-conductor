# Report — 2026-06-22-activity-floor-restart-suppression-scenarios

**Chunk:** Activity-floor + restart-suppression scenarios (P-013..P-016, P-057) — bursty-train/lunch
no-false-silent + abrupt-silence cue + gap→RestartEvent + surgical 60s suppression + dual-condition bypass
triple catalog TOMLs; all-Hard (conductor-core scenarios)
**Date:** 2026-06-22
**Commits:** (none yet — chunk uncommitted; this wrap commits it) · Epoch 7 (Scenario catalog) ch4/8

## Changes (structured — detectors read this)
- **Files:** `scenarios/activity-floor.toml` (new) · `scenarios/service-went-silent.toml` (new) ·
  `scenarios/restart-suppression.toml` (new) · `crates/conductor-core/src/scenario.rs` (modified — `#[cfg(test)]`
  module only).
- **Symbols / APIs:** **none new in production** — no public fn / type / IPC method / endpoint / export / port /
  socket / env var added or changed. The test module gains 4 fns (a per-family loader `#[rstest]`, an all-`Hard`
  guard `#[rstest]`, an `Absent`-usage `#[test]`, a `Contains`-presence `#[rstest]`). The 3 scenarios **reuse**
  the pre-existing `ComparisonKind::Absent` (P-013) and `ComparisonKind::Contains` (P-014 · P-015/16/57); every
  `[[expected]]` check is `class = "Hard"`.
- **Crates / modules:** none added/removed/changed (only `conductor-core`'s test module).
- **Dependencies:** **none** added or bumped.
- **Schema / config:** **3 new scenario config TOMLs** under `scenarios/` (the activity-floor + restart-suppression
  catalog families). **NO model/schema change** — `Scenario` / `ExpectedCheck` / `ComparisonKind` / `PhaseSpec`
  consumed unchanged via `Scenario::from_toml_str` (the `Absent` kind pre-existed at `expected.rs:41`, P-007
  precedent). No `runs.db` / journal / violation-schema change.
- **Coverage of new surfaces** (one line per new surface):
  - `scenarios/activity-floor.toml` (P-013 scenario config) → validation **garde✓** (`from_toml_str` → garde) ·
    instrumentation **n/a** (declarative config; runtime emission/spans are Epoch-8) · PII **n/a** (no payload;
    synthetic phase labels) · tests **unit✓** (load+garde-validate `#[case]` · all-`Hard` · `Absent`-usage) ·
    a11y **n/a** · tokens **n/a**
  - `scenarios/service-went-silent.toml` (P-014) → validation **garde✓** · instrumentation **n/a** · PII **n/a** ·
    tests **unit✓** · a11y **n/a** · tokens **n/a**
  - `scenarios/restart-suppression.toml` (P-015/16/57) → validation **garde✓** · instrumentation **n/a** (the
    obs-plan §4 restart-suppression spans `bypass_triggered`/`path_type` are Epoch-8 *runtime*, not this config
    chunk) · PII **n/a** · tests **unit✓** · a11y **n/a** · tokens **n/a**

## Deviations from intent
- **1 trivial (justified):** the plan's step-4 kind-usage assertion was implemented as **two** idiomatic test
  fns (a dedicated `#[test]` for the `Absent` leg + a `#[rstest]` for the `Contains` legs) rather than one —
  matches the codebase idiom (P-007 is a single `#[test]`; per-family checks are `#[rstest]`). Same coverage.
- **File count (anticipated, not a plan deviation):** 3 files, vs `scope.md`'s initial "2 TOMLs" estimate —
  resolved via the P4 AskUserQuestion (scope Q2 explicitly flagged the split as the alternative); the approved
  `plan.md` specifies 3 files, and the impl matches the plan.

## Decisions & corrections
- **(P4 user decision) 3-file outcome-coherent split.** A scenario's `Vec<ExpectedCheck>` evaluates against ONE
  read-back with no per-check service/window scoping, so `Absent "ServiceWentSilent"` (P-013) and `Contains
  "ServiceWentSilent"` (P-014) cannot coexist → split so both assert. The same forcing function makes
  restart-suppression's suppressed legs (15s-transient, 8×/3%) **declare-only** (an `Absent "ErrorRateSpike"`
  would contradict the surfaced sustained/bypass spikes) — documented in the TOML + carried to Epoch-8.
- **(research finding) `ComparisonKind::Absent` already exists** (`expected.rs:41`, P-007 precedent) → **zero model
  change**. Code-graph impact: `ComparisonKind` 55 refs · `Scenario` 29 · `ExpectedCheck` 18 — modifying any would
  ripple across conductor-verify/report; reusing the existing variant ripples nothing.
- **(convention reaffirmed) every check `class = "Hard"`** per arch §Probabilistic-Assertion Policy ("suppression/
  bypass logic" + lifecycle timing are deterministic hard pass/fail); the model-interpretive severity that
  consumes these cues is P-020/P-021, a later chunk. (Same all-`Hard` posture the 2026-06-22 playbook rule covers
  for the Epoch-7 floor/threshold scenarios.)
- **(no golden re-baseline)** `replay.rs` loads only `error-baseline-spike.toml` (untouched here); `determinism.rs`
  loads no TOML; a `crates/` grep found no `read_dir`/glob of `scenarios/` — the new seeds 4317013/014/015 feed
  no golden. Goldens verified **UNCHANGED**.
- **(scope-out)** P-013's spec "restart Pulse mid-training, verify histogram restoration" leg is OUT — Conductor
  does not manage the Pulse process.
- **(carry-forward → Epoch-8)** the declare-only suppressed-leg evaluation; the P-057 Report bypass-annotation
  token; the restart-suppression `<20s` tier; the long-timeline (P-013 ~90-min) live-run compression.

## Outcome
- **Acceptance criteria: MET.** The 3 TOMLs deserialize + garde-validate + build a valid `PhaseTimeline`; every
  check is `class="Hard"`; zero `Scenario`/`ExpectedCheck`/`ComparisonKind`/`PhaseSpec` change; no new dependency;
  no new inbound bind (pure-egress; the port-occupier is P-003/ch1).
- **Gates green:** `cargo nextest -p conductor-core` **123/123** (114→123) · `cargo nextest --workspace --profile
  ci` **328/328** (319→328) · `cargo test --doc` ok (3 conductor-faults, 0 new) · `cargo clippy --workspace
  --all-targets -- -D warnings` **clean** · `bash scripts/agent-run.sh run` **EXIT 0**.
- **Smoke:** `agent-run.sh run` (the no-boot-path scenario-chunk smoke per the verification-harness rule —
  `status`/`logs` need a live `run_id` that exists only post-Epoch-8) — **EXIT 0**. Determinism goldens
  **UNCHANGED** (no re-baseline, as predicted).
