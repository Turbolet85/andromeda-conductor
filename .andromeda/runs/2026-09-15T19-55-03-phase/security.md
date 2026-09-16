# security extract

## Relevance
Partial — the chunk edits an enumerated external-input surface (scenario config) but adds no new boundary, spawn, listener, or dependency.

## Constraints
- Committed `scenarios/*.toml` are an enumerated external-input boundary whose trust boundary is garde `range`/`dive` **and** the load-path `Scenario::check_*()` arm TOGETHER; a tier edit must not remove, relax, or route around either (per security-plan §Input Validation, scenario-config row).
- `check_budgets` requires each `[[expected]].budget_ms` ≤ **that scenario's own `slo_tier` deadline** — a sibling-reading load-path rule. Re-tiering moves the deadline that rule reads, so it must still hold for every re-declared file; whether any Situation-2/3 scenario declares `budget_ms` today is research's question (per security-plan §Input Validation, scenario-config row; §Security Anti-Patterns → Input).
- If any tier-vs-duration validator is introduced (here or by the sibling gate entry), it MUST ship as a `Scenario::check_*()` invoked from `from_toml_str` raising `CoreError::Config`, never as `#[garde(custom)]` — in garde 0.22.1 a field validator receives `(&field, &())`, sees no sibling, and would look enforced while testing nothing (per security-plan §Security Anti-Patterns → Input; §Bootstrap phases → input-validation-library-install).
- Closed-set discipline: a shape/tier discriminator outside its closed set must remain a hard load fault (deserialize error / `CoreError::Config`), never an inferred default and never silently widened — the scope's "no new tier" boundary and this rule are the same requirement (per security-plan §Input Validation, scenario-config row, `[phases.fault]` closed-`kind` precedent).
- The garde ceiling `MAX_BUDGET_MS` is **derived** from `SloTier::Tier90s.deadline_ms()`, so the tier ladder is itself a validation input; the scope's "ladder untouched" boundary is what keeps that ceiling stable (per security-plan §Input Validation, scenario-config row).
- Tests moved by a re-tier must not drop the pin that the load path actually invokes each `check_*()` — the bootstrap phase mandates that pin exist (per security-plan §Bootstrap phases → input-validation-library-install).
- Config failures stay on the harness-fault side of the verdict/error wall (`Result::Err` / `CoreError::Config`), never collapsing into a `Verdict`/`ReportState` value (per security-plan §Error Handling).

## Patterns to follow
- Two co-equal validation mechanisms: garde derives co-located with the serde structs in their owning seam crate for single-field bounds, plus the explicit `Scenario::check_*()` arm from `from_toml_str` for anything sibling-spanning (`check_capabilities` · `check_budgets` · `check_checklist`) (per security-plan §Input Validation, How column).
- `dive`, never `skip`, on every nested spec field — a skipped nested struct deserializes entirely unvalidated because garde never descends into it (per security-plan §Input Validation; §Security Anti-Patterns → Input).
- Closed enum + required discriminator with no inferred default, as `FaultKindSpec` establishes for `[phases.fault].kind` (per security-plan §Input Validation, scenario-config row).
- Committed SUT-facing manifests (`contracts/pulse-load-envelope.toml`) are never defaulted and never silently widened; if the scope's "envelope untouched" expectation fails at P3, any edit there inherits that rule (per security-plan §Input Validation, committed-manifests row).

## Anti-patterns to avoid
- NEVER write a sibling-reading invariant as `#[garde(custom)]` (per security-plan §Security Anti-Patterns → Input).
- NEVER mark a nested spec field `#[garde(skip)]`, nor deserialize scenario config without validation at load (per security-plan §Security Anti-Patterns → Input).
- NEVER let committed artifacts (run reports, `runs.db` rows, JSONL journals, the per-chunk `evidence/` tree) carry absolute host paths or internal seam-crate struct names (per security-plan §Security Anti-Patterns → Logging; §Error Handling, run-report artifact sanitization).

## Contract bindings
- **tests** — the load-path-invocation pin for `check_*()` (security-plan §Bootstrap phases) is realized as a test the tests domain owns; the re-tier's test enumeration must preserve it.
- **tests §CI Integration** — the dependency/supply-chain gate (`cargo audit`, `cargo deny check advisories bans licenses sources`) is the security gate as a job in the single workflow (per security-plan §Dependency Security, CI integration).
- **arch / `conductor-core`** — `SloTier`'s closed set and `deadline_ms()` are read-only inputs to the security validation boundary (`MAX_BUDGET_MS` derivation + `check_budgets`); arch owns the ladder, security owns what breaks if it moves.

## Acceptance criteria contributions
- Every re-declared scenario still loads clean through `from_toml_str` with both arms passing — garde bounds and the load-path `check_*()` set, `check_budgets` in particular, since a tier move changes the deadline it reads (per security-plan §Input Validation, scenario-config row).
- `slo_tier` values stay inside the closed set, and an out-of-set value remains a hard load fault, never defaulted or inferred (per security-plan §Input Validation, scenario-config row).
- No validator added for this property is written as `#[garde(custom)]` over a sibling-reading invariant; grep of any new rule shows the `Scenario::check_*()` form (per security-plan §Security Anti-Patterns → Input).
- `cargo audit` and `cargo deny check advisories bans licenses sources` green over an un-drifted committed `Cargo.lock` before merge (per security-plan §Security Anti-Patterns → Universal; §Dependency Security).

## Relevant amendment history
- **2026-08-21-per-check-latency-measurement** (four entries: §Input Validation scenario-config row · §Anti-Patterns → Input · §Bootstrap phases · §Threat Model trust boundary) — registered `budget_ms` with garde `range(1, MAX_BUDGET_MS)` AND established the load-path `check_*()` arm as the sanctioned co-equal mechanism for sibling-reading rules, because garde 0.22.1 structurally cannot express them. This is the amendment that created the `budget_ms` ≤ own-`slo_tier`-deadline rule this chunk's re-tiers perturb; its own stated reason was that a reader would otherwise "'fix' it into an unbuildable validator". It also demonstrates the single-site-apply trap: the same rule was restated at four sites.
- **2026-08-22-operator-pause-and-checklist-live-firing** (§Input Validation scenario-config row · §Threat Model attack surface) — grew the sibling-spanning load-path enumeration from two to three (`check_checklist`) at both the §Input Validation site and its verbatim §Threat Model restatement; precedent that any new load-path check for tier honesty would owe the same two-site registration.
- **2026-08-19-connection-lifecycle-live-proof** (§Input Validation · §Threat Model · §Anti-Patterns → Input) — registered `[phases.fault]` with the closed `FaultKindSpec` enum (unknown kind = deserialize error, no inferred default) and `dive` never `skip`; the closed-discriminator precedent the fixed tier set follows.
- **2026-08-11-faithful-emission-dispatcher** (§Input Validation · §Anti-Patterns → Input) — widened the scenario-config boundary across 23 committed scenario TOMLs after finding `PhaseSpec.emission` marked `#[garde(skip)]`; the reason the `dive` never `skip` mandate is stated at three sites. Prior nearby evidence that a mass edit over committed scenario files is the moment the boundary enumeration goes stale.
