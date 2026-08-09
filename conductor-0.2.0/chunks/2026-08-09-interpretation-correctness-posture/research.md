# Codebase Research — 2026-08-09-interpretation-correctness-posture

## Scope
- **Depth:** moderate · **Reads:** 8 files (+2 rule files) · **Globs/Greps:** 9 · **Graph queries:** 1

## Files inspected
- `crates/conductor-run/src/lib.rs` (150–260) — `execute_scenario`. **The decisive finding is line 216**: the observed read-back value is the literal placeholder `"incidents-listed"` (`Ok(_) => "incidents-listed".to_string()`), with the in-code comment "the faithful per-check observed-extraction is the Epoch-10 bridge". Every `ExpectedCheck` is graded against that constant.
- `crates/conductor-core/src/expected.rs` (full) — `ClaimClass::{Hard, CalibrationRegion}` + `ComparisonKind::{Exact, Contains, Absent, CountAtLeast}` + `ExpectedCheck`. The class is **declared up front in the TOML, never guessed at runtime**. This is the shipped vehicle for a model-interpretive claim; the doc comment already names "hypothesis quality" as a `CalibrationRegion` example (`expected.rs:19`).
- `crates/conductor-core/src/verdict.rs` (1–60) — `Verdict::CalibrationRegion` labels `"HOLD"` / `[HOLD]`, and `default_report_state()` maps it to `ReportState::ManualCheck`. Both the enum and the mapping already exist; nothing new is needed to express "measured but model-interpretive".
- `crates/conductor-core/src/coverage.rs` (1–80 + the row table) — `CoverageMode` (4 variants) + `CapabilityRow` (`&'static str`-backed, "no allocation and no runtime IO") + `coverage_matrix()`. **`coverage.rs:121` classifies P-033 "Ranked Hypothesis Generation" (Diagnostic Quality) as `CoverageMode::Auto`.**
- `crates/conductor-core/src/drift.rs` (46) — `check_sut_drift` compares the code-native classification against the manifest's accepted set. It asserts **set-equality of P-IDs only**; it does not and cannot observe whether an `Auto` row has a verifying scenario.
- `scenarios/severity-tier-suggested.toml` (full) — the closest authoring precedent: a `class = "CalibrationRegion"` + `kind = "Contains"` check on an inferred model token (`"Suggested"`), with a header comment block that names its declare-only legs and its "Epoch-8 calibration points".
- `crates/conductor-report/src/coverage.rs` (24/46/76) — `render()` / `write(out_path)` / `summary_line(rows)`; the Markdown coverage surface and its roll-up.
- `crates/conductor-cli/src/render.rs` (85/92) — `coverage_table()` / `coverage_summary()`; the CLI mirror.
- `.claude/rules/verification-harness.md` (:47) — the standing **"OPEN posture decision (pending, cross-codebase)"** entry from `2026-06-27-live-pulse-e2e-proof`, which this chunk closes. Its "leading" option (a deterministic test-L4 mode in Pulse) has since landed Pulse-side.
- `crates/conductor-core/src/redact.rs` (21/50) — `ALLOWLISTED_FIELDS` + `is_allowlisted`; a non-allowlisted field name is **dropped at the processor stage**, silently.

## Graph impact
Query (`tree-query-2026-08-09-interpretation-correctness-posture.json`): symbols matching `coverage_matrix` / `CoverageMode` / `check_sut_drift` / `CalibrationRegion`.
- **`coverage_matrix()`** — defined `conductor-core/src/coverage.rs`; consumed by `conductor-tauri/src/commands.rs:121` (the `coverage_matrix` command, tested at `:316`), plus the report/cli render sites above. A classification edit is a 3-surface change.
- **`check_sut_drift()`** — `conductor-core/src/drift.rs:46`; the classification↔manifest gate.
- **`Verdict::CalibrationRegion`** — `conductor-core/src/verdict.rs:22`; **`ClaimClass::CalibrationRegion`** — `conductor-core/src/expected.rs:24`. Two distinct enums (declared claim class vs measured verdict) — do not conflate them in the plan.

## Patterns detected
- **Declare-only legs are an established authoring device** (`scenarios/severity-tier-suggested.toml:13-16`, `restart-suppression`, `project-context-grounding`): a scenario TOML documents, in its header comment, the parts of a capability it deliberately does *not* yet assert plus which epoch owns them. This is the in-repo precedent for recording a boundary next to the thing it bounds.
- **`KnownResidual` is producer-assigned, never config-declared** (`.claude/rules/testing.md`, 2026-06-22): the `Scenario` model has no `state` field, so a scenario cannot declare itself a residual — the report seam sets it. A Branch-B deferral therefore cannot be expressed as scenario config alone.
- **An empty `expected` list routes to `ManualCheck`** (`conductor-run/src/lib.rs:222-239`): a scenario with no `[[expected]]` block takes the operator-checklist path and produces a `manual_record`. This is the shipped "awaits a human" path — and `v2-05`'s own method is `manual`.
- **Coverage counts are always derived, never literal** (`conductor-report/src/coverage.rs:76` `summary_line`, tested for exactness and for summing to the row count) — the 2026-08-08/08-09 de-hardcoding rule.

## Conventions to follow
- **Claim class is declared in TOML, verdict is computed** — `expected.rs:58-60`; a P-033 check would carry `class = "CalibrationRegion"`, and `Verdict::default_report_state` (`verdict.rs:52`) does the rest.
- **Header-comment provenance in scenario files** — every scenario TOML opens with its P-ID(s), the spec section, declare-only legs and calibration points (`severity-tier-suggested.toml:1-18`).
- **Any new self-obs field must be added to `ALLOWLISTED_FIELDS`** (`redact.rs:21`) or it vanishes silently.
- **Three coverage surfaces move together** — `conductor-report/src/coverage.rs`, `conductor-cli/src/render.rs`, `conductor-tauri/src/commands.rs:121` (+ the webview).

## Measured facts this chunk must reckon with
Computed over `coverage.rs` (82 rows: 43 Auto / 16 DriveObserve / 7 StaticOnly / 16 NotConductors — matching the shipped roll-up) versus the 40 distinct P-IDs claimed across `scenarios/*.toml`:

**11 `Auto`-classified capabilities have no scenario at all:**
`P-031` Report Structure · **`P-033` Ranked Hypothesis Generation** · `P-034` Suggested Investigation Steps · `P-039` MCP Delivery When Configured · `P-041` Persistent Incident Corpus · `P-042` Cross-Session Continuity · `P-043` Project-Scoped Memory · `P-044` Retrieval-Augmented Interpretation · `P-073` Deterministic env-gated L4 mode · `P-074` Tier1 incident-path reliability under load · `P-079` Constellation severity live-wiring.

So P-033 is **not a singular anomaly** — it is one of eleven `Auto` claims with nothing behind them, and no gate in the repo can see this (`check_sut_drift` compares classification↔manifest, not classification↔scenarios). Three of the eleven (`P-073`/`P-074`/`P-079`) are already routed to named future entries; the interpretation cluster (`P-031`/`P-033`/`P-034`/`P-044`) is precisely the diagnostic-quality set F3 says deterministic L4 bypasses.

## Files to modify
Depends on the branch chosen at P4. Candidates, none yet committed:
- `scenarios/` — a new P-033 TOML (Branch A, or a declare-only Branch-B artifact).
- `.claude/rules/verification-harness.md` — the `:47` open-decision entry, which this chunk closes either way (a wrap-owned Session-Additions edit).
- `crates/conductor-core/src/coverage.rs` — only if the posture reclassifies P-033 (a `check_sut_drift`-governed edit).
- `crates/conductor-core/src/drift.rs` — only if the posture becomes a live gate over the Auto-without-scenario set.

## Open questions
1. **Branch A is not buildable inside this chunk's boundaries.** A real-model leg must *assert the top hypothesis*, but `execute_scenario` grades every check against the literal `"incidents-listed"` (`conductor-run/src/lib.rs:216`); real per-check extraction is `v2-09` in Epoch 2, which this chunk's scope explicitly excludes. Building it here means pulling Epoch-2 work forward. → P4 decision.
2. **Should the posture be a live gate or a recorded decision?** The research surfaces a concrete gate candidate (Auto-classified rows with no verifying scenario — currently 11, invisible to every existing check), which would make the posture load-bearing in the way `check_sut_drift` is. → P4 decision.
3. **Does the deferral name P-033 alone or the whole interpretation cluster?** The honest scope is `P-031`/`P-033`/`P-034`/`P-044`. → P4 decision.
