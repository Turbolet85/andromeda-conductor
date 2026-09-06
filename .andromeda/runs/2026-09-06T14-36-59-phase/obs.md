# obs extract

## Relevance
**Relevant** — this chunk builds obs-plan §4's Critical Path 6 (Coverage-matrix completeness gate), the one must-trace path whose named observables scope §1.1 measured absent from the tree.

## Constraints

- **The span chain and its attributes are already specified — this chunk is where they land or where §4 is re-based.** obs-plan §4 (Scenario: Coverage-matrix completeness gate) requires `report.coverage_matrix_generate` → `db.query_all_p_ids` → `report.validate_coverage` with `p_id_count_found` / `p_id_count_expected` on the DB span and `coverage_percent` / `missing_count` on the validate span, plus log fields `p_ids` / `missing_p_ids` / `coverage_percent` / `journal_emitted_at` (also stated in the §1 must-trace table). Scope §1.1 measured all seven tokens at zero hits, so either the gate emits them or §4 moves; there is no third option.

- **§11's bounded span-name set does not contain any of the three names, and `db.*` is deliberately closed.** obs-plan §11 (Spans / Traces) enumerates the permitted set as `scenario.run · timeline.execute* · emit.batch · emit.logs_batch · verify.readback* · report.generate · db.insert_run · fault.* · tauri.command.*`. `report.coverage_matrix_generate`, `report.validate_coverage` and `db.query_all_p_ids` are outside it, and obs-plan §4 (auto-instrumentation table, conductor-report row) states the set stays `db.insert_run` alone "with no `db.*` widening". §4 and §11 therefore disagree on this path; reconciling them is an amendment-class decision for P4, not an implementation choice.

- **Any span attribute outside `conductor-core::redact::ALLOWLISTED_FIELDS` emits nothing.** obs-plan §4 (Known-residual path, Required-span-attributes heading) states the processor stage drops non-allowlisted field names. Whether `p_id_count_found`, `p_id_count_expected`, `coverage_percent`, `missing_count` are already allowlisted is **research's question** — if they are not, the mandated attributes are uninstrumentable as written (the same disqualifier that retired `degraded_mode_requested`).

- **Denominator semantics are load-bearing and pre-bound.** obs-plan §4 (:359) requires `p_id_count_expected` = the FULL manifest capability count while `coverage_percent` is computed over the **in-scope** count (manifest set minus out-of-remit rows), both manifest-derived and never literals, matching the shipped roll-up so the gate and the roll-up cannot silently disagree. The same passage requires the auto term's derived `(N unbacked)` qualifier to stay a qualifier — a gate reading these fields must keep the four per-mode counts summing to the row total.

- **A gate placed on the CLI path can raise no obs signal without a new dependency edge.** obs-plan §4 (auto-instrumentation table, cli row) and §6 (per-module levels, conductor-cli row) require that `conductor-cli` declares no `tracing` dependency and carries no `tracing::` call site, so every span and log on the CLI path comes from the libraries it drives. This bears directly on scope §4 fork 2 (Rust test vs. non-zero exit on `conductor coverage`): the `conductor coverage` placement would require adding that edge before §4's observables can exist.

- **The gate's record shape is unresolved between the two named shapes.** obs-plan §3 (Log format JSON schema — two record shapes) fixes the self-obs line (base set + allowlisted span attributes) and the report-seam records (eleven-field envelope + per-check `CheckRecord`). §4's four coverage log fields match neither. Which shape carries `missing_p_ids` / `coverage_percent` is research's question, and it is owned laterally by test-plan §3.

- **Redaction applies to whatever artifact CARRY 2 resolves to.** obs-plan §11 (Logs) bans absolute host paths and internal struct names in logs, run-report artifacts and `runs.db`; a committed or generated `coverage-matrix.md` is a run-report-class artifact under that ban.

## Patterns to follow

- **`{module}.{operation}` span naming** with low-cardinality names (obs-plan §2 Naming conventions / §4 Span naming convention) — the three mandated names already conform in shape; only their membership in §11's set is at issue.
- **Non-allowlisted data rides the allowlisted `message` field.** obs-plan §6 (Boundary-call wrappers) uses this for the MCP key-set witness and the `mark_incident_resolved` incident id, precisely because a dedicated attribute would emit nothing. This is the escape hatch if the coverage attributes are not allowlisted.
- **Read-path convention: a DB read logs, it does not span.** obs-plan §6 (DB READ entry) and §4 (conductor-report row) require a rusqlite READ to emit a boundary-call `info!` carrying `run_id` plus outcome on the `message` field, inside the caller's span, minting no `db.*` read span. `db.query_all_p_ids` is a read, so this convention and §4's mandated span are in direct tension.
- **Span carries the count; the log carries the array.** obs-plan §4 puts `missing_count` on `report.validate_coverage` and `missing_p_ids` in the log fields — the cardinality discipline §11 (Spans / Traces) requires.
- **Manual `#[tracing::instrument]` on the library handlers the CLI drives, never on `fn main()`** (obs-plan §4, cli row).

## Anti-patterns to avoid

- **Never widen the bounded span-name set implicitly** (obs-plan §11 Spans / Traces) — adding `db.query_all_p_ids` or a `db.*` wildcard as a side effect of building the gate contradicts a set the plan records as a deliberate contract.
- **Never turn `coverage_percent` into a metric instrument or histogram** (obs-plan §5 + §11 Metrics) — Minimal tier has no metrics backend; the performance/gate quantity is a JSON field assertion only.
- **Never emit the gate's failure as unstructured stderr text** (obs-plan §11 Logs / Universal) — a red gate must be agent-parseable JSON or sanitized stderr with machine-parseable hints, and must not leak absolute host paths in the diff it prints.

## Contract bindings

- **obs ↔ tests (primary):** obs-plan §4/§1 Critical Path 6 is the obs-side twin of test-plan §1 Critical Path 6 (scope :80, :250). The two-sided `D-tests-obs-harness` bind means an obs-side re-base of this chain requires the test-plan side to move with it.
- **obs ↔ tests §3 (record shape):** test-plan §3 owns the journal line formats (obs-plan §3 states obs reproduces, does not author). Where the gate's four fields land is the owner's call.
- **obs ↔ CI (§9):** a new CI step must not disturb the two shipped obs CI gates — the log-conformance gate asserting the §3 self-obs base schema over `logs/agent-latest.jsonl`, and the zero-unlogged-panics gate — and obs-plan §11 (CI) bans losing telemetry artifacts. Scope §1.1's "every existing CI gate stays green" is the same requirement from the entry's side.
- **obs ↔ security (CARRY 5):** obs-plan §11 (PII Scrubbing) rests on "Conductor owns no PII — only synthetic test telemetry." An eighth `provider_key` category adds credential-SHAPED literals (`sk_live_…`, `ghp_…`, `AKIA…`) to `conductor-emit`'s corpus; these are PRODUCT payload, but §11 requires they never reach a self-obs line or a run-report artifact. Whether any code path logs corpus values is research's question.
- **obs ↔ CARRY 4 (`ramp_factor`):** obs-plan §4 (Fault-injection spans) pins `ramp_factor` to the signed slope over `EmissionShape::Ramp`'s `from_rate`/`to_rate`. Whether the load-envelope rate-term correction shares a basis with that attribute — and so whether it moves it — is research's question.

## Acceptance criteria contributions

- The gate emits §4's mandated span chain and attributes, or §4's chain is amended to the shipped shape with §11's bounded set reconciled in the same pass — a gate that silently satisfies neither fails (per obs-plan §4 Coverage-matrix completeness gate + §11 Spans / Traces).
- Every attribute the gate records appears in the emitted JSON lines — an attribute name outside `conductor-core::redact::ALLOWLISTED_FIELDS` is dropped at the processor stage and must instead ride the allowlisted `message` field (per obs-plan §4 Required-span-attributes constraint + §6 Boundary-call wrappers).
- `p_id_count_expected` reports the full manifest capability count while `coverage_percent` is computed over the in-scope count, both manifest-derived (never a literal), with the four per-mode counts still summing to the row total and `(N unbacked)` not treated as a fifth mode (per obs-plan §4 Denominator semantics, :359).
- The added CI step leaves the §9 log-conformance gate and the zero-unlogged-panics gate green, emits machine-parseable output on failure, and its telemetry artifact is retained (per obs-plan §9 CI Integration + §11 CI).

## Relevant amendment history

- **`2026-08-08-sut-capability-manifest`** — de-hardcoded this exact gate's span attributes: `p_id_count_expected` and the `p_ids` log field name the manifest's count/set rather than the literal 60. *Why it matters here:* the gate must read the manifest, and scope §1.2's re-verified 82-id manifest is the direct consequence of that reversal.
- **`2026-08-09-out-of-scope-classification-treatment`** — pinned `coverage_percent`'s denominator to the in-scope count while `p_id_count_expected` stays the full manifest count, explicitly as a **pre-binding for this not-yet-built Epoch-6 gate**, to stop the 82-row expectation and the shipped in-scope denominator diverging unnoticed. This is CARRY 3's origin.
- **`2026-08-09-interpretation-correctness-posture`** — recorded the roll-up auto term's derived `(N unbacked)` qualifier as a qualifier, not a fifth summand, so a gate reading these fields cannot mis-sum. This is CARRY 1's roll-up constraint.
- **`2026-09-01-live-per-p-id-verdict-lamps`** — the operator ruled the bounded span-name set is a deliberate contract and is **not** widened with a `db.*` wildcard; a rusqlite READ logs a boundary `info!` inside the caller's span and mints no read span, with a `db.*` read span left as a design option for the entry that next touches `conductor-report`. This chunk touches `conductor-report`, and `db.query_all_p_ids` is that read span.
- **`2026-09-06-run-report-envelope-conformance-gate`** (immediately prior chunk, the precedent scope §4 fork 2 cites) — recorded that `conductor-cli` raises no spans of its own (no `tracing` dep), that a deliberately uninstrumented seam is a legitimate outcome, and that §11's set stays `db.insert_run` alone. Directly constrains gate placement.
- **`2026-08-13-per-check-read-back-extraction`** — established the disqualifier precedent: attributes not in `ALLOWLISTED_FIELDS` were retired because a built attribute "would emit nothing." The same test applies to the four coverage attributes.
- **`2026-08-16-fault-application-spans`** and **`2026-08-16-canary-fingerprint-derivation-aligned`** — two further precedents that an attribute uncomputable at the moment the layer reads attributes is retired rather than shipped; `2026-08-16-fault-application-spans` also fixes `ramp_factor`'s signed-slope definition that CARRY 4 sits adjacent to.
- **`2026-06-27-obs-ci-conformance-gate`** — the first operationalization of a §9 gate revealed the spec had named the wrong record shape for the artifact it read. Same failure mode is live here: this chunk first operationalizes Critical Path 6, and §4's four log fields match neither named record shape.
