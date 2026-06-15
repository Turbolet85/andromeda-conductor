# obs extract

## Relevance
Partial — chunk defines core type vocabulary (Verdict/ReportState/Scenario/error wall); obs contributes binding-schema alignment + telemetry-field cardinality constraints only, not instrumentation itself.

## Constraints

- Per obs-plan §1 Instrumentation Scope: `conductor-core` is instrumentable; all telemetry sinks controlled by caller (CLI or Tauri) — type definitions must be serde-serializable to match JSONL binding contract (obs-plan §6 Log Format JSON Schema).
- Per obs-plan §6 Log Coverage: Verdict enum must serialize to canonical PascalCase names (`"Pass"`, `"Fail"`, `"CalibrationRegion"`) matching the JSONL binding schema; ReportState enum must serialize to exact names (`"Pass"`, `"Fail"`, `"ManualCheck"`, `"KnownResidual"`, `"Blocked"`).
- Per obs-plan §3 Observability Harness Contract: scenario types must carry `seed` (u64) and `scenario` (string) fields for correlation by `run_id` envelope — no high-cardinality span attributes on types themselves.
- Per obs-plan §2 Telemetry Strategy: `SloTier` closed enum must be the only performance-budget cardinality point; serde rename to `<5s`/`<20s`/`<90s` literal strings (not Rust identifiers).
- Per obs-plan §1 Obs Scope Summary: no PII fields in `conductor-core` types — architecture prevention via synthetic test telemetry classification (Low).

## Patterns to follow

- Serde `#[serde(rename = "...")]` on enum variants to enforce canonical JSONL field names (e.g., `Verdict::Pass` serializes as `"Pass"`, never `"PASS"` or snake_case).
- Plain `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]` on both enum types — no custom `impl Serialize` (let serde-derive enforce conformance).
- Per obs-plan §2 Naming Conventions: struct/enum names follow `{PascalCase}` (Verdict, ReportState, SloTier, Scenario); field names follow `snake_case` in Rust, serde-renamed to wire form only where needed.

## Anti-patterns to avoid

- NEVER use high-cardinality fields (per-user-ID, per-path) in Scenario struct — only `p_ids` array (fixed set of 60 P-IDs per obs-plan §1 critical path 6).
- NEVER skip serde round-trip unit tests for enums — they are the binding contract with obs harness (Section 6 JSONL schema compliance is enforced at parse time).
- NEVER add optional verdict/state fields — both enums are closed (5 ReportState variants, 3 Verdict variants) and non-optional in the run-report envelope per obs-plan §3.

## Contract bindings

- **obs ↔ tests:** Type definitions must support the JSONL binding schema from tests excerpt §5 (obs-plan §6 verbatim); serde round-trip unit tests verify contract compliance.
- **obs ↔ log format:** `Verdict` and `ReportState` serialize to JSONL fields; SloTier renders as serde-renamed string; tests harness reads these fields from runs.db / emission journal (obs-plan §3 §6 binding).

## Acceptance criteria contributions

- (obs) Verdict and ReportState enums serialize to exact canonical PascalCase names matching JSONL schema (obs-plan §6); serde round-trip unit tests pass.
- (obs) Scenario struct carries required `seed` (u64) + `p_ids` array (cardinality-bounded) for run_id correlation; no PII fields.
- (obs) SloTier closed enum with serde-renamed values (`<5s`, `<20s`, `<90s`) matching perf-budget tiers (obs-plan §10); no open strings.

## Relevant amendment history

(none)
