# Scope — Scrub/pipeline/degraded/report-surface scenarios

**Chunk:** `2026-06-23-scrub-pipeline-degraded-report-surface-scenarios`
**Epoch:** 7 (Scenario catalog) · chunk 8/8 — **the LAST of Epoch 7; closes the scenario catalog**
**Working-route intent:** _"Scrub/pipeline/degraded/report-surface scenarios — PII scrub, cadence/hot-reload, model-off, render-timing (P-035, P-037, P-045, P-047..P-056)"_
**P-IDs:** P-035/P-047/P-048 (pii-scrub) · P-037 (report render) · P-045 (findings counter) · P-052 (cadence) · P-055/P-056 (hot-reload) · P-053 (degraded) — **9 drivable**; P-049/P-050/P-051/P-054 — **4 StaticOnly, no scenario**

## What it builds
Declarative `scenarios/*.toml` catalog entries (the established Epoch-7 pattern) for the final four capability
families — PII scrub, report-surface timing, pipeline-ops config, degraded-mode — plus the `conductor-core`
scenario test wiring that loads and guards them. No new emission primitives, no runtime engine code: the
**eighth consecutive Epoch-7 chunk to hold zero model change** (every realization uses an existing field —
Hard via `Absent`/`CountAtLeast`/`Contains`, declare-only via the `#[serde(default)]` empty `expected`).

**Six TOMLs over nine drivable P-IDs** (outcome-coherent grouping — P4 Q1); the four StaticOnly P-IDs get none.

Per family:
- **pii-scrub** (`pii-scrub.toml` — P-035/P-047/P-048, **Auto → Hard**) — drive a PII-laden stream via the
  shipped `conductor-emit` `PiiCorpus` (all seven P-047 categories: email/JWT/bearer/API-key/credit-card/SSN/
  secret `key=value`), so the read-back Report excerpts (P-035) and corpus (P-048 — no raw OTLP attribute
  values stored) show every seeded token SCRUBBED while structure is preserved. Hard `Absent "<seeded token>"`
  over a representative category subset + `Contains "<structural marker>"`. The catalog's first use of `Absent`
  for PII negative assertions; fully realizable now over the existing corpus.
- **report-render-surface** (`report-render-surface.toml` — P-037, **DriveObserve → ManualCheck**) — drive an
  incident; the operator opens the in-app Report surface — render <2s + keyboard-nav + section-collapse is a
  visual/timing claim (matrix-delegated; the <2s measurement is Epoch-8). Empty `expected` (operator-checklist).
- **findings-counter-refresh** (`findings-counter-refresh.toml` — P-045, **Auto → Hard count**) — emit a known
  number of unread active incidents; the findings counter = `len(query_incident_list | unread/active)` is
  deterministic, so Hard `CountAtLeast "<N>"`. The ≤1s refresh TIMING is Epoch-8-measured (declared via SLO
  tier, not asserted as a content token here).
- **cadence-config** (`cadence-config.toml` — P-052, **Auto → declare-only**) — drive a steady workload while
  the operator sets a known cadence config (baseline/accelerated/reflection intervals); the L4-invocation-rate
  match is an Epoch-8 measurement with no standalone content token. Empty `expected` (operator-config +
  deferred measurement).
- **threshold-hot-reload** (`threshold-hot-reload.toml` — P-055/P-056, **MIXED Hard + declare-only**) — drive a
  baseline; the operator hot-reloads a threshold mid-run (P-055, <2s — declare-only timing leg) and the change
  applies PROSPECTIVELY only (P-056 — Hard `Absent "<retroactive cue for the pre-change baseline>"`). One
  operator action observed two ways; the catalog's first MIXED Hard-plus-declare-only single file.
- **degraded-mode-report** (`degraded-mode-report.toml` — P-053, **Auto → declare-only KnownResidual**) — drive
  the same hard-signal workload while the operator disables the model / sets the fallback tier; the
  `retrieve_report(degraded_mode=true)` result is a PRE-ACCEPTED residual (architecture §Standard Contracts
  lists "a `retrieve_report` result returned under `degraded_mode`" as a KnownResidual example), so the scenario
  DECLARES + DOCUMENTS the residual and defers Fail→KnownResidual routing to the Epoch-8 evaluator (the P-032
  precedent — the scenario model has no `state` field; ReportState is producer-assigned). Empty `expected`.

## Boundaries
- **Catalog + test wiring only** — new TOMLs under `scenarios/` + `conductor-core` scenario-model test
  additions; no new `ComparisonKind`, no emission primitive, no engine path. Zero model change is the bar.
- **StaticOnly four get no scenario** — P-049 (encryption at rest), P-050 (cross-project sharing opt-in), P-051
  (transparent storage), P-054 (hardware-profile awareness) are classified `StaticOnly` in `coverage.rs`;
  authoring a live scenario for a statically-verified claim would be wrong. Their coverage is complete via
  classification; this chunk leaves them untouched. (P-049's encrypted-corpus wiring is additionally exercised
  by the existing preflight canary — arch §Standard Contracts.)
- **Operator-config actions, not new faults** — P-052/P-053/P-055/P-056 depend on a Pulse-side config change
  (cadence / model-off / threshold). Per the scope law (no Pulse process management) + the shipped
  operator-pause orchestration (`2026-06-21-operator-pause-orchestration`), these are OPERATOR actions during a
  go/no-go hold, NOT new Conductor fault primitives. The TOML declares the workload + the required operator
  action (in header comments); the live wiring + measurement are Epoch-8/10.
- **Emission primitives reused, not built** — `PiiCorpus` (`conductor-emit/src/pii.rs`, all 7 categories) for
  pii-scrub; the raw trace/log emitters for the rest. None are new (Epoch 3 shipped them).
- **First MIXED-shape pipeline file + first degraded_mode KnownResidual** — `threshold-hot-reload` carries one
  Hard leg (P-056 `Absent`) + one declare-only leg (P-055 timing); `degraded-mode-report` is the first
  KnownResidual whose residual is the `degraded_mode` contract (vs P-032's `recent_commits` stub).
- **Determinism** — seeds chosen must not collide with or perturb existing golden-fed seeds; goldens expected
  UNCHANGED (the scenario-catalog seeds feed no golden, per every prior Epoch-7 chunk).
- **Scope law** — every scenario carries its P-ID; Conductor opens no new listener.

## Surfaces / contracts touched
- `scenarios/*.toml` — six new catalog entries.
- `conductor-core` scenario model (`scenario.rs`) — **test wiring only** (per-family loader rstest +
  class/shape guards + a suite guard exercising the Hard-plus-declare-only mix); no struct change expected.
- Existing `ComparisonKind::{Absent, CountAtLeast, Contains}` + `ClaimClass::Hard` + the `#[serde(default)]`
  empty `expected` — first catalog use of `Absent` for PII-scrub negative assertions.
- ReportState `ManualCheck` (P-037 operator-checklist) + `KnownResidual` (P-053 degraded_mode) — reuse
  (`lamp.rs`, present); no change.
- MCP read-back contract — P-035/047/048 `retrieve_report`/`query_incident_list` scrub read-back; P-045
  `query_incident_list` count; P-053 `retrieve_report(degraded_mode=true)` (declared; live Epoch-8/10).
- Coverage-matrix (`coverage.rs`) already classifies all 13 P-IDs (Auto×8, DriveObserve×1, StaticOnly×4) —
  no change expected.

## Definition of done (acceptance intent)
- Six scenario TOMLs exist — `pii-scrub`, `report-render-surface`, `findings-counter-refresh`, `cadence-config`,
  `threshold-hot-reload`, `degraded-mode-report` — each carrying its P-ID(s), SLO tier, and the resolved
  expected shape; loadable + garde-valid.
- pii-scrub declares Hard `Absent`/`Contains` over the seeded PiiCorpus; findings-counter-refresh declares Hard
  `CountAtLeast`; threshold-hot-reload declares the P-056 Hard `Absent` leg (+ P-055 declare-only timing);
  report-render-surface + cadence-config declare empty `expected` (operator-checklist / deferred-measurement);
  degraded-mode-report declares empty `expected` + DOCUMENTS the degraded_mode KnownResidual.
- The four StaticOnly P-IDs get no TOML (correct realization of their coverage mode); stated in the plan.
- scenario test wiring loads + guards the new families (per-file class/shape purity + a suite guard exercising
  the Hard + declare-only mix).
- Gates green (conductor-core + workspace nextest, clippy `-D`, doctest); goldens UNCHANGED; `agent-run.sh run`
  exit 0.
- **Epoch 7 (Scenario catalog) is CLOSED** — every catalog P-ID covered (auto / drive-observe / static).
