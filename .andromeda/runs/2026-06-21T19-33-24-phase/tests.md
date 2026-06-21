# tests extract

## Relevance
relevant — the chunk delivers a persistent-store consumer (Markdown report writer) that serializes the run-report envelope shape across all three artifact surfaces (JSONL journal, `runs.db`, Markdown report); test strategy must ensure both determinism and artifact hygiene.

## Constraints
1. Per test-plan §1 Scope Summary, the Report envelope and its serialization are testable via cargo-nextest and insta (golden-locked); the trio of surfaces (JSONL/database/Markdown) must share identical canonical `RunRecord` shape with zero struct-name leakage (security anti-pattern bans "exposing internal error details").
2. Per §4 Unit Test Strategy (conductor-report bullet), the canonical line shape is locked via exact-string `assert_eq!` at unit level, matching `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern; insta is reserved for E2E journal-golden with redaction.
3. Per §3 Test Harness Contract / Status endpoint shape, the Run-report envelope JSON contract is normative for all three surfaces; the Markdown writer is a pure consumer/renderer of that shape (envelope defined by envelope-serializer chunk, not this chunk).
4. Per scope.md § Boundaries, the Markdown render module establishes the shared **verdict-first lamp helper** (verdict when present ∈ {Pass|Fail|CalibrationRegion}, else state) that is reused by coverage-matrix (ch4), cli (Epoch 8), and desktop (Epoch 9); the lamp helper owns the precedence logic, not report-local rendering.
5. Per §3 Test Harness Contract / Log format, artifact sanitization (no absolute host paths, no internal struct names) is asserted by negative test on the journal output; Markdown render extends this invariant to the `.md` artifact via field redaction at the edge.
6. Per test-plan §1 Testable entities (conductor-report), the Run-report envelope serialization + JSONL journal shape + `runs.db` row mapping are golden-testable via cargo-nextest (both exact-assert unit + insta E2E with redaction); JSONL/database/Markdown share the same source shape, so determinism follows from the envelope-serializer's determinism.

## Patterns to follow
1. Exact-string `assert_eq!` unit goldens for canonical serialization shape (matching conductor-core's established pattern for verdict.rs/report_state.rs/scenario.rs), locked at unit level before E2E insta redaction.
2. rstest `#[fixture]` + `#[rstest]` table-driven valid/invalid matrix over the five report states (Pass/Fail/ManualCheck/KnownResidual/Blocked) to exercise envelope-to-Markdown rendering path.
3. Per test-plan §2 Deterministic invariant, same record set ⇒ identical Markdown (wall-clock `generated_at` field must be excluded from golden or injected at assertion); verification via `assert_fs` artifact comparison (string equal, not parse-tree equal).
4. The verdict-first lamp helper is a separate, reusable module (e.g. `conductor-report::lamp` or shared in conductor-core) with owned unit tests for the three precedence cases (verdict→lamp, CalibrationRegion→[HOLD], state-fallback).

## Anti-patterns to avoid
1. No struct names (e.g. `RunRecord`, `Verdict`) or absolute host paths in the rendered Markdown — test asserts via negative-pattern grep on the artifact.
2. No call to insta at unit level (unit uses exact-assert; insta is E2E-only for redacted journal).
3. Do not overwrite an existing `runs/<run_id>.md` on repeat; unit tests verify idempotency via file-existence check + a `never-overwrites-if-present` test case (scope.md intent).

## Contract bindings
- **envelope-serializer (Epoch 6, ch1)** ← tests must consume the canonical `RunRecord` shape once it is defined; this chunk's unit tests depend on that interface.
- **verdict/report logic (conductor-verify)** ← the Markdown render is a pure consumer of verdict/state enums; verdict mapping tests live in conductor-verify, not here.
- **runs/ artifact directory + `CONDUCTOR_RUNS_DIR` canonicalization (cli edge, Epoch 8)** ← test fixture provides a `TempDir` to simulate the runs directory; real canonicalization is cli-owned.
- **obs-plan §3** ← artifact sanitization (no struct names/paths) is a joint test gate: tests asserts the Markdown carries no leak; obs observes that the downstream JSONL journal also carries no leak (two independent test boundaries).

## Acceptance criteria contributions
1. "(tests) `cargo nextest run -p conductor-report` passes; the exact-string unit goldens for Markdown render match the specified envelope schema." — verifies the canonical Markdown shape is deterministic.
2. "(tests) A CalibrationRegion verdict renders as [HOLD], never [MANUAL]; a Blocked row renders state as em-dash; all five states produce correct lamps." — verifies verdict-first precedence and the blocked-row null rule (scope.md intent).
3. "(tests) The Markdown artifact contains no absolute host paths or internal struct names; negative-pattern grep confirms." — security anti-pattern from test-plan §1.
4. "(tests) Deterministic output — same `RunRecord` set ⇒ identical Markdown; insta E2E journal goldens with redaction (wall-clock excluded) pass." — verifies determinism per §2.

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** (test-plan §4): frontend code carries no Rust unit tests (build-gated only); this chunk (conductor-report render) is Rust, so unaffected — the amendment clarifies that E2E frontend testing is deferred to Epoch 9, not a concern for this Markdown-report chunk.
- **2026-06-16-emission-journal-writer** (test-plan §4, conductor-report bullet): exact-string `assert_eq!` at unit level (matching conductor-core's pattern for verdict/report serialization goldens); insta reserved for E2E with redaction — directly applies to this chunk's unit-test strategy.
- **2026-06-17-raw-otlp-message-scaffold** (test-plan §2 Integration): tokio-stream gRPC stub added to integration mechanisms; Markdown render uses only the in-memory/TempDir file surface, not gRPC, so unaffected — the amendment is orthogonal to this chunk's test surfaces.