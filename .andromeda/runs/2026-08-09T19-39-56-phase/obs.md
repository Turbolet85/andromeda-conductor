# obs extract

## Relevance
Partial — the chunk authors declarative catalog data, not instrumentation, but it touches three obs-owned contract surfaces: the `slo_tier` closed enum, the Run-report envelope's `p_ids`/`state` fields, and the recorded coverage-roll-up denominator semantics including the derived `(N unbacked)` qualifier.

## Constraints
- Each new `scenarios/*.toml` must carry a `slo_tier` drawn from the closed enum `<5s | <20s | <90s` — no fourth tier, no free-form string; the tier becomes a hard report-time deadline assertion (`latency_ms <= threshold`, journal-relative wall clock) per obs-plan.md §5 Metric Coverage + §10 SLO Invariants.
- The scenario's identity fields land in the Run-report envelope: `p_ids` (array), `verdict` ∈ {Pass, Fail, CalibrationRegion}, `state` ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}. The two `DriveObserve` scenarios route to the existing `ManualCheck` state — no new enum member is permitted, per obs-plan.md §6 Log Coverage (and §3 Log format JSON schema, which is a binding contract obs derives from tests).
- Coverage-gate denominator semantics are load-bearing and already recorded: `p_id_count_expected` is the FULL manifest capability count while `coverage_percent` is computed over the **in-scope** count (manifest set minus out-of-remit rows); both stay manifest-derived, never literals — per obs-plan.md §4 (Scenario: Coverage-matrix completeness gate).
- The `(N unbacked)` term is a **derived qualifier on the auto count, not a fifth summand** — the four per-mode counts must keep summing to the in-scope row total after the pin shrinks 11 → 10, per obs-plan.md §4 denominator semantics.
- No new span names: the span-name set is bounded to `scenario.run`, `timeline.execute*`, `emit.batch`, `emit.logs_batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `fault.*`, `tauri.command.*` — per obs-plan.md §11 Spans/Traces. Scenario-config load and catalog listing are not must-trace operations (§4 must-trace paths enumerate 7 scenarios, none of which is catalog authoring).
- Scenario TOML content (names, comments, phase labels) flows into run-report artifacts, so it must contain no absolute host paths and no internal struct names — per obs-plan.md §11 Logs + §11 PII Scrubbing (redaction is owned single-source in `conductor-core::redact`; do not add a second scrub site).
- garde validation on scenario-config deserialization is mandatory — the three new TOMLs must load through the validating path, never a bypass loader, per obs-plan.md §11 Project-specific bans.

## Patterns to follow
- `D:\dev\projects\conductor\scenarios\service-constellation-discovery.toml` — the shipped `DriveObserve` shape: P-ID header comment, `p_ids`, `seed`, `slo_tier` with an inline justification of the tier choice, `[[phases]]`, and an explicit trailing comment recording *why* there is no `[[expected]]` block (the ManualCheck/operator-checklist path).
- All three roll-up surfaces already single-source the pin dynamically rather than by literal: `D:\dev\projects\conductor\crates\conductor-report\src\coverage.rs:82` (`summary_line(rows, unbacked)`), `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs:210`, `D:\dev\projects\conductor\crates\conductor-tauri\src\commands.rs:138`. Shrinking `UNBACKED_AUTO` should propagate with no per-surface arithmetic edit — matching obs-plan.md §4's "manifest-derived, never a literal" rule.
- `D:\dev\projects\conductor\crates\conductor-report\src\coverage.rs:187` (`the_unbacked_qualifier_is_not_a_fifth_summand`) asserts against `UNBACKED_AUTO.len()`, not a hardcoded count — keep that assertion shape when updating tests so the 11 → 10 shrink needs no test-literal churn.
- The webview read of the pin is already instrumented as `tracing::info_span!("tauri.command.unbacked_auto")` (`commands.rs:139`) — a conforming `tauri.command.*` member of the bounded set (obs-plan.md §11); reuse it rather than adding a new span if that path is touched.
- `SloTier` deadline math lives in `D:\dev\projects\conductor\crates\conductor-verify\src\slo.rs` with the tier matrix under test — the `Auto` (P-079) scenario's `[[expected]]` block plugs into this existing evaluation path per obs-plan.md §5 SLO assertion.

## Anti-patterns to avoid
- Do NOT instrument the catalog: no span around TOML load, `list_scenarios`, or the pin edit — obs-plan.md §11 Telemetry Strategy ("NEVER over-instrument"; spans are reserved for the 7 must-trace paths) + §11 bounded span-name set.
- Do NOT hardcode `10`, `11`, or any capability/row count as a literal in a roll-up surface, gate, or assertion — obs-plan.md §4 denominator semantics (both counts manifest/`UNBACKED_AUTO`-derived).
- Do NOT emit or embed absolute host paths (drive-letter, `%APPDATA%`, `~/.cargo`) in scenario TOMLs, comments, or roll-up output — obs-plan.md §11 Logs + PII Scrubbing.

## Contract bindings
- **obs ↔ tests (envelope schema):** the Run-report envelope (`p_ids`, `verdict`, `state`, `slo_tier`, `latency_ms`) is OWNED by test-plan §3; obs-plan.md §3/§6 derive from it. New P-IDs entering the catalog become envelope `p_ids` values — obs adds no fields here, and the `DriveObserve` scenarios must resolve to the existing `ManualCheck` state rather than a new one.
- **obs ↔ arch/core (coverage gate source):** `conductor_core::UNBACKED_AUTO` (`crates/conductor-core/src/drift.rs`) is the data the obs-plan.md §4 coverage-gate attributes and the derived `(N unbacked)` qualifier read; the exact-set `check_scenario_backing` gate is arch/core's, but its rendered output is obs's §4 contract.
- **obs ↔ CI (§9):** the log-conformance gate (`logs/agent-latest.jsonl`, §3 self-obs base schema) and zero-unlogged-panics gate remain unchanged by this chunk but must stay green; this chunk adds no new CI obs gate.

## Acceptance criteria contributions
- (obs) Each of the three new scenario TOMLs declares `slo_tier` from the closed enum `<5s | <20s | <90s` (obs-plan §5/§6) — no new tier value, no missing tier.
- (obs) After the pin shrinks to 10, all three coverage roll-up surfaces (Markdown report, CLI table, webview) render the unbacked count derived from `UNBACKED_AUTO.len()` with no literal, and the four per-mode summands still sum exactly to the in-scope row count — the qualifier is not a fifth mode (obs-plan §4 denominator semantics).
- (obs) The bounded span-name set is unchanged: this chunk introduces no new span name and no instrumentation on the config-load / catalog path (obs-plan §11 Spans/Traces).
- (obs) No absolute host paths or internal struct names appear in the new scenario TOMLs or in the coverage roll-up output on any surface (obs-plan §11 Logs / PII Scrubbing).

## Relevant amendment history
- **2026-08-09-interpretation-correctness-posture** (§4, coverage-gate denominator semantics) — recorded that the roll-up's auto term carries a derived `(N unbacked)` qualifier read from `conductor_core::UNBACKED_AUTO`, qualifying the auto count rather than joining the breakdown, so per-mode summands still sum to the row count and a gate must not treat it as a fifth mode. This chunk is the first to *change* that number (11 → 10), so the invariant is directly under test here.
- **2026-08-09-out-of-scope-classification-treatment** (§4) — pinned `coverage_percent`'s denominator to the in-scope count (manifest set minus out-of-remit rows) while `p_id_count_expected` stays the full manifest count; recorded to stop the not-yet-built Epoch-6 gate from diverging from the shipped roll-up. Any roll-up edit in this chunk must preserve that split.
- **2026-08-08-sut-capability-manifest** (§4 + §1 must-trace table) — de-hardcoded the coverage-gate span attributes so `p_id_count_expected` / `p_ids` name the manifest's capability count/set rather than the literal 60, after an SUT advance invalidated the fixed number. Same failure mode this chunk must avoid when touching the unbacked count.
