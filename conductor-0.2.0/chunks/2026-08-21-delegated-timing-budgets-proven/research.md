# Codebase Research — 2026-08-21-delegated-timing-budgets-proven

## Scope
- **Depth:** moderate (two repos) · **Reads:** 11 · **Globs/Greps:** 9 · **Graph queries:** 4 (rust plane)

## Files inspected

### Conductor
- `scenarios/{halo-hue-encoding,service-constellation-discovery,report-render-surface,findings-counter-refresh}.toml` (headers + expected blocks) — all four declare `slo_tier = "<5s"`; three carry ZERO `[[expected]]`, `findings-counter-refresh` carries ONE.
- `crates/conductor-verify/src/slo.rs` (10–100) — `SloOutcome` / `evaluate_slo` / `evaluate_check`; the definition of the quantity `budget_ms` bounds.
- `crates/conductor-run/src/lib.rs` (405–430, 477–495) — the `evaluate_check` call site, the shared-latency comment, `ReadBack` / `route_read_back`.
- `crates/conductor-core/src/expected.rs` (47–95) — `ExpectedCheck.budget_ms`, `effective_deadline_ms`, `MAX_BUDGET_MS`.
- `crates/conductor-core/src/scenario.rs` (156–175) — `check_budgets` load-path rule.

### Pulse (SUT, read-only — HEAD `f0c38f5`)
- `crates/ui-bridge/src/telemetry.rs` (159–245) — the three `telemetry.frontend` procedures + their `tracing::info!` targets.
- `pulse-app/src/observability.rs` (941–960, 2249–2260, 2462–2476) — the four allowlist leaves and the file sink.
- `pulse-app/src/incidents_router.rs` (551–566) — the P-037 fire site.
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (90–160) — the P-027 and P-025 fire sites.
- `pulse-app/ui/src/hooks/use-findings.ts` (40–65) — the P-045 fire site + its re-poll cadence.
- `pulse-app/ui/src/report/use-report.ts` (30–60) — what triggers `incidents.get_report`.
- `crates/mcp-server/src/tools.rs` (44–51, 360–380) + `Cargo.toml` (32–33) — the 8-tool surface; `dispatch_retrieve_report` builds in-sidecar.

## Graph impact (rust plane; `db_state: fresh`)
- **`effective_deadline_ms`** — `calls` returned **0 rows**, and the Q4 re-probe CONFIRMED the symbol is
  indexed (`conductor-core 0.1.0 expected/impl#[ExpectedCheck]effective_deadline_ms().`,
  `crates/conductor-core/src/expected.rs`). Per the cookbook that would read as "leaf, no callers" — but a
  grep cross-check found a **real production caller** at `crates/conductor-verify/src/slo.rs:84`, inside
  `evaluate_check`. So this is a **missing `calls` edge**, not a leaf: the symbol is indexed and the caller
  exists, yet no edge joins them. Every impact statement below rests on the grep basis, not the graph.
- **`budget_ms`** — indexed as `expected/ExpectedCheck#budget_ms.`; a same-named but unrelated
  `run_contract/impl#[RunContract]preflight_budget_ms().` also matches loose predicates (the descriptor-
  anchoring hazard the cookbook warns about — the two are unrelated).
- Impact of the four scenario TOMLs is data-only: they are catalog fixtures, so no call edge exists to find.

## Patterns detected
- **Journal-relative latency is the ONLY latency** (`conductor-verify/src/slo.rs:39`):
  `latency_ms = read_back_observed_at_ms − journal_emitted_at_ms`. `evaluate_check` passes
  `check.effective_deadline_ms(tier)` into it (`:84`), so `budget_ms` narrows the deadline on THAT quantity.
- **Checks of one scenario share one latency by construction** (`conductor-run/src/lib.rs:414-416`, verbatim:
  "They share one latency by construction — the corpus is observed once — so a check's own `budget_ms` is
  what can separate its verdict"). The deadline is the only discriminator between two checks' timing verdicts.
- **`CountAtLeast` never hard-fails** (`slo.rs`, the `class` override): an unmet sample floor routes to
  `CalibrationRegion` regardless of declared class. `findings-counter-refresh`'s existing check is exactly
  this kind.
- **Pulse persists its own tracing stream to disk** — `tracing_appender::rolling::daily(&logs_dir,
  "agent-latest.jsonl")` (`pulse-app/src/observability.rs:2462`). This is the harvest surface: an
  agent-parseable JSONL carrying the four `metric.*` lines, the same artifact class prior legs harvested.
- **Each observable sits behind its OWN exact allowlist leaf**, and the field names are NOT uniform:
  `metric.constellation.hue_update_ms` → `duration_ms`, `severity_tier` ·
  `metric.constellation.discovery_ms` → `duration_ms`, `discovered_count` ·
  `metric.findings.counter_refresh_ms` → `duration_ms` · `metric.report.render_ms` → **`value`**,
  `section_count`, `degraded_mode`. Pulse's own comment (`observability.rs:941-945`) records why the leaf
  must be exact: a bare `metric` key exists carrying only `["value","unit","module"]`, and `for_target`
  falls back to the first `.`-segment, so a leaf-less `metric.*` target keeps `value` and has every other
  field REDACTED.

## Conventions to follow
- **Harvest-tier test placement**: `crates/conductor-run/tests/<family>_harvest.rs`, pinning verbatim
  leg-captured lines as fixtures (the storm → baseline → restart → pii → connection → severity chain).
- **Declare-only landing**: a scenario with zero `[[expected]]` lands `verdict: null` / `KnownResidual` and
  emits ZERO `run_check` rows (`conductor-run/src/lib.rs` `route_read_back`; arch §Per-check record).
- **Two-site restatement**: a scenario re-base lands in test-plan §6 AND §1 Critical paths in one pass.
- **Load-path validation**: a sibling-spanning TOML rule ships as `Scenario::check_*()` from
  `from_toml_str` (`scenario.rs:156-175`), never `#[garde(custom)]`.

## New files to create
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — the harvest-tier grading of the four measured
  values against their budgets (2s / 5s / 2s / 1s), over verbatim captured Pulse `metric.*` lines.

## Files to modify
- `scenarios/halo-hue-encoding.toml` — header premise refine (halo wording → constellation dot; see PREMISE).
- `scenarios/service-constellation-discovery.toml` — header records the harvest basis for the ≤5s bound.
- `scenarios/report-render-surface.toml` — header records the drive+observe precondition for P-037.
- `scenarios/findings-counter-refresh.toml` — header records the harvest basis for the ≤1s bound; its
  existing `CountAtLeast` check is a COUNT claim and is NOT the timing claim (leave the check itself alone
  unless the plan decides otherwise).
- `conductor-0.2.0/verification-matrix.json#v2-20` — concretize `acceptance`; correct the `observed_gap`
  inaccuracy via a `notes` PREMISE-CORRECTION (the ledger channel, never the immutable requirements).
- **NOT** `crates/conductor-core/src/coverage.rs` and **NOT** the `UNBACKED_AUTO` pin — measured: no mode
  change is required by this chunk's deliverable, so both coverage gates stay untouched (see below).

## Coverage classification as it stands (measured)
`crates/conductor-core/src/coverage.rs`: **P-025 `DriveObserve`** (:113) · **P-027 `DriveObserve`** (:115) ·
**P-037 `DriveObserve`** (:125) · **P-045 `Auto`** (:133).
`UNBACKED_AUTO` (`drift.rs:61-62`) = `P-031, P-033, P-034, P-039, P-041, P-042, P-043, P-044` — **none of
the four is pinned**. P-045 is `Auto` and already backed by `findings-counter-refresh.toml`; the other three
are `DriveObserve`, which does not participate in `check_scenario_backing` at all. So neither gate moves
unless the plan deliberately re-classifies — which the working entry does not ask for.

**The nuance P4 must weigh (do not decide silently):** research measured that P-025 and P-027 fire
*autonomously* from the telemetry Conductor drives, which sits oddly with their `DriveObserve`
classification. That classification is still defensible — `DriveObserve` covers the *visual* confirmation
(the operator sees the hue/dot), while the delegated *timing number* is a separate, harvestable quantity.
Re-classifying them `Auto` is NOT required to grade the budgets and would pull both coverage gates into the
chunk. Recommend leaving classification untouched and recording the observation.

## Open questions
- Does P-037's ≤2s bound get claimed at all this chunk, given its fire site needs an operator to open a
  report? → blocks: **plan-decision** (P4 must either scope it as an operator-checklist step of the live
  leg, or leave P-037 unclaimed with the reason recorded). This is the one question that decides whether
  `v2-20` can be claimed whole, since its acceptance names all four budgets.
