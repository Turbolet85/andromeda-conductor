# security extract

## Relevance
Partial — the chunk generates a static classification/artifact, not a verification surface; error sanitization and artifact generation are in scope; threat model updates are out of scope.

## Constraints

1. Coverage-matrix output (`coverage-matrix.md`) MUST NOT leak absolute paths or internal struct names (security plan §Error Handling, run-report artifact sanitization); classification data + render are agent-parseable, treated as ground-truth artifacts like `<run_id>.md` reports.

2. All 60 P-IDs (P-001..P-060) MUST be classified with zero gaps or unclassified entries — completeness enforced by test (security plan §Threat Model Summary § Attack surface, vector coverage law: "coverage-matrix.md definition-of-done artifact: every P-ID enumerated").

3. Artifact write MUST be loud-never-overwrite-safe per the run-report seam pattern — same path-canonicalization + safety checks as the `<run_id>.md` report write (security plan §Error Handling, run-report artifact sanitization).

4. Mode classification sourced from `.andromeda/input.md` §Coverage classification + `capability-verification-matrix.json` audit reference — no operator/scenario-config input in this chunk; input validation OUT OF SCOPE (no new CLI flags / env vars / file deserialization introducing untrusted boundaries).

5. Lamp precedence reuse, not re-derivation (security plan §Conventions: Error handling + Inbound verification, preflight discipline) — the `Lamp::for_record` reuse constraint applies if status-column wired; classification-only render has no verification inputs, hence no validation burden.

## Patterns to follow

- Pure/clock-free render + exact-string golden test matching (security plan §Error Handling, artifact sanitization + §Conventions: Determinism discipline — no tokio virtual clock for timestamps).
- Artifact write path safety consistent with `RunReport::render` (security plan §Error Handling, run-report artifacts + §Anti-Patterns: Logging, leaking paths/struct names).
- Reuse existing crate split patterns (e.g., `conductor-core` model / `conductor-report` render) rather than introducing new cross-crate dependencies (security plan §Dependency Security, no new deps without justification).

## Anti-patterns to avoid

- NEVER expose absolute paths (`canonicalized` `CONDUCTOR_RUNS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) or internal struct/field names in the `coverage-matrix.md` artifact (security plan §Anti-Patterns: Logging).
- NEVER leave P-IDs unclassified or missing — a gap is a defect, not a silent omission (security plan §Threat Model Summary, coverage law).
- NEVER re-derive `Lamp` precedence if status-column is wired; reuse `Lamp::for_record` directly (security plan §Conventions: Error handling, preflight discipline).

## Contract bindings

Tests harness (golden test for classification-only render; completeness assertion for 60-row enumeration); obs (no direct input boundary, but artifact will be read by obs/dashboard in Epoch 8/9 — ensure no secrets/paths leak into JSON output if observability later cross-cuts the artifact).

## Acceptance criteria contributions

- (security) `coverage-matrix.md` rendered + written with no path leaks or internal struct names (grep verifies artifact for `/home/`, `/root/`, `C:/Users/`, struct names from scope + security plan).
- (security) all 60 P-IDs (P-001..P-060) enumerated and classified, zero unclassified/missing, asserted by test.
- (security) artifact write using canonicalized + loud-never-overwrite-safe pattern (matches run-report write safety).

## Relevant amendment history

- **2026-06-21-runs-db-index** — bundled SQLite 3.50.4 (§Infrastructure: Database, Cargo.lock committed); no impact on coverage-matrix output or validation rules (dependency audited, not in this chunk's data path).
- **2026-06-15-dependency-audit-gate** — cargo-audit/cargo-deny floor discipline established (not applicable to this chunk directly; no new deps added unless justified per scope definition-of-done).
- **2026-06-15-structured-logging-stack** — identity env-handles (`CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV`) noted as non-path labels, no validation needed; if coverage-matrix ever reads env, confirm same non-path discipline (currently out-of-scope per chunk boundaries).
