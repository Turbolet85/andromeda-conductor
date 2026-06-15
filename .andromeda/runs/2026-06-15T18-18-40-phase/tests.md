# tests extract

## Relevance
partial — the chunk's log-redaction surface is in-scope (applies to self-obs logs + error edges); run-report artifact redaction reuses the primitive but lands in Epoch 6.

## Constraints
- Per test-plan §1, Minimal tier with property/security/determinism triggers; artifact-hygiene invariant is a hard constraint, not optional coverage.
- Per test-plan §3 Log format, self-obs JSONL must carry ONLY allowlisted fields; non-allowlisted fields redacted before write.
- Per test-plan §3 Status endpoint shape, run-report artifacts (Epoch 6) must NOT leak absolute host paths or internal struct names — enforced via golden-snapshot + field-allowlist assertion; this chunk builds the primitive.
- Per test-plan §5 Integration, the sanitization boundary at the `anyhow` edge (stack traces / host paths / struct names → operator-facing error) is a contract assertion surface.
- Per test-plan §11 Test Anti-Patterns / Logging, the ban "Logging stack traces, absolute host paths, or internal struct/field names" applies directly to this chunk's implementation scope.
- Per test-plan §3 Bootstrap phases / log-format-bind-with-obs, the field-allowlist + redaction layer is SOURCE OF TRUTH for the harness contract; obs derives its envelope FROM this subsection (binding contract — harness greps the journal).

## Patterns to follow
- Unit tests for the redaction primitive: valid-input (allowlisted fields pass through) and invalid-input (non-allowlisted / path-shaped values masked or dropped) fixtures using rstest.
- Integration golden test (insta) of a self-obs log line containing a non-allowlisted field or host-path value, asserting redacted output against a committed snapshot (no host paths / struct names in the golden).
- Error-edge sanitization test: craft an `anyhow::Error` with a stack trace + internal struct name, surface it at the cli/tauri edge, assert stderr/command-return shows only the operator-facing shape (`error:` / `hint:`) with no trace/struct-name.
- Reusability contract: the redaction fn signature must be callable from both the subscriber init (self-obs logs) and future run-report writers (Epoch 6), parameterized over field allowlist + patterns.

## Anti-patterns to avoid
- Logging raw stack traces, absolute host paths (`C:\Users\...` / `/home/...`), or internal struct/field names into the self-obs JSON stream.
- Hardcoding redaction logic at each edge (cli / tauri error handling); centralize the primitive in `conductor-core::obs` and call from both.
- Validating redaction via ad-hoc log inspection; instead commit golden snapshots and assert the redacted line matches the golden (insta).

## Contract bindings
- **tests ↔ obs**: obs §3 Log JSON schema field-set drives the allowlist (harness parses allowlisted fields; non-allowlisted filtered before write).
- **tests ↔ security**: security anti-patterns explicit ban on stack traces / paths / struct names at the edge is the assertion target.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes new redaction-primitive unit tests (valid/invalid fixtures per rstest, exit 0).
- (tests) Integration golden test (insta): a self-obs JSONL line with a non-allowlisted field or host-path value is redacted; the golden snapshot contains NO absolute paths / struct names.
- (tests) CLI error edge: a simulated anyhow error at the CLI boundary shows only sanitized `error:` / `hint:` output (no stack trace, no host path); assert via assert_cmd stdout/stderr predicates.
- (tests) Redaction primitive callable from both self-obs subscriber and (future) run-report writers — tested via a reusability contract fn-signature assertion in a unit/doc test.

## Relevant amendment history
Per test-plan-amendments.md: 2026-06-15-structured-logging-stack amended §3 Log format to clarify that the self-obs stream (stderr / `logs/agent-latest.jsonl`, per-line base fields incl. service-identity + `run_id`) is SEPARATE from the emission-journal envelope (`runs/<run_id>.jsonl`). This chunk's redaction primitive applies to the self-obs stream; the run-report envelope redaction lands in Epoch 6 but reuses the same primitive. The amendment does NOT change the field allowlist itself — derived from obs-plan §3, source of truth for both streams.
