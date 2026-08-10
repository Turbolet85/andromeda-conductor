# obs extract

## Relevance
Partial — half B (`v2-07` environment-suspect classification on the run-report envelope / `runs.db` / report seam) is squarely obs; half A (`v2-06` catalog assertion) touches obs only via the derived-count discipline and the load-profile bound.

## Constraints
- Minimal tier, no metrics backend: the envelope's rate/duration terms must be JSON-field assertions evaluated at report-generation time, never an instrument, histogram or meter provider (per obs-plan §1 Obs Scope Summary, §5 Metric Coverage). This is the obs-side echo of the chunk's "not a load-tester" boundary.
- The Run-report envelope is a **derived** contract, not obs-authored: any new classification field or `state` value is a schema change that must be reconciled with the owner (tests plan §3) before it lands in `runs/<run_id>.jsonl` + `runs.db` (per obs-plan §3 Log format JSON schema, §6 Log Coverage; precedent in the 2026-06-16 amendment below).
- Two record shapes are distinct: the per-line self-obs stream (`logs/agent-latest.jsonl`) vs the scenario-result envelope (`runs/<run_id>.jsonl`). The environment-suspect classification belongs on the **envelope/result** record; adding it must not perturb the §9 conformance gate, which validates the self-obs base schema only (per obs-plan §3 "two record shapes" note, §9 Log conformance check).
- Any new envelope/log field is invisible unless registered: the `conductor-core::redact` field-name allowlist **drops** non-allowlisted field names, and the value scrub masks absolute host-FILE paths (per obs-plan §11 PII Scrubbing, §11 Logs). A breach message naming a `scenarios/*.toml` file must not carry an absolute host path.
- New spans stay inside the bounded, low-cardinality span-name set and follow `{module}.{operation}`; no scenario-name-in-span-name (per obs-plan §4 Span naming convention, §11 Spans / Traces).
- Realized-profile timing (the dynamic `v2-07` read) must use wall-clock `std::time::SystemTime`/`Instant`, never tokio's virtual clock — journal-relative SLO math depends on it (per obs-plan §11 Project-specific bans, §1 conductor-timeline row).
- Obs already bounds load instrumentation to "typical/high" profiles and explicitly excludes saturation (per obs-plan §4 Fault-injection spans preamble) — the envelope artifact should be consistent with that existing bound rather than restating a competing one.
- SLO enforcement may flip `state` Pass→Fail on a `latency_ms` breach at report-generation time (per obs-plan §10 SLO Invariants). An over-envelope run must be classified before/around that rule so a zero-row read-back is not silently flattened into `Fail`.

## Patterns to follow
- **Known-residual classification path** (obs-plan §4, Scenario: Known-residual classification path) — the existing precedent for a non-Fail classification: a `report.classify_*` span carrying `classification_result`, `db.insert_run` carrying `state_written`, and one extra envelope field naming the cause. This is the closest template for environment-suspect.
- **Per-scenario additional envelope fields** (obs-plan §6 Log Coverage, additional fields list; §3 "Additional fields per scenario") — `bypass_triggered` / `lifecycle_phase` / `degraded_mode_response` show the established shape for an optional qualifier on the envelope without inventing a new record shape; relevant to Open question 1's "run-level qualifier" fork.
- **Derived-count / named-set discipline** (obs-plan §4 coverage-matrix gate, Denominator semantics) — counts and sets are read from their source (`UNBACKED_AUTO`, the manifest), never written as literals, and a qualifier is documented as *qualifying* a term rather than joining the closed breakdown. Applies to the catalog count and to any envelope-terms roll-up this chunk renders.
- **Boundary-call must-log wrappers** (obs-plan §6 Boundary-call wrappers) — `report.generate` logs final verdict + state; the envelope-breach cause should ride that same must-log event rather than a side channel.
- **Single-location redaction ownership** (obs-plan §11 PII Scrubbing) — extend `conductor-core::redact` at the processor stage; do not add a second scrub at the new sink.

## Anti-patterns to avoid
- NEVER add a metrics instrument, histogram or OTel meter to measure rate/duration for the envelope check (obs-plan §11 Metrics, §5) — the budget is a JSON field assertion.
- NEVER emit the breach as unstructured stderr text, and NEVER leak an absolute host path or internal struct name into the log, run report, or `runs.db` (obs-plan §11 Logs).
- NEVER introduce a high-cardinality span name (per-scenario / per-profile) or leave the classification span dangling past its phase boundary (obs-plan §11 Spans / Traces).

## Contract bindings
- **Envelope / `state` shape ↔ tests plan §3** — tests owns the harness-contract schema; obs derives. A sixth `ReportState` variant or a new envelope field requires the owner's schema to move first (obs-plan §3, §6).
- **`ReportState` closed set ↔ architecture.md §Standard Contracts** — obs reproduces the `state` enum in §3 and §6; if the chunk lands a sixth variant, both obs reproductions amend in lockstep with the CLI prefixes / Markdown lamp / `runs.db` / webview `LAMP_META` mirrors.
- **Redaction allowlist ↔ security plan §Logging & Monitoring** — the "no absolute host paths in run-report artifacts" anti-pattern is the binding; a new field passes through the same allowlist (obs-plan §11 PII Scrubbing).
- **CI artifacts ↔ obs §9** — the `logs/agent-latest.jsonl` conformance gate asserts the self-obs base schema and must stay green; the envelope-side gate is still unbuilt, so this chunk's new field gets no automatic CI conformance coverage (obs-plan §9 Log conformance check).

## Acceptance criteria contributions
- (obs) Any new run-report envelope field for the classification / breach cause survives `conductor-core::redact` — i.e. it is registered in the field-name allowlist and a test proves it is present in the emitted `runs/<run_id>.jsonl`, not silently dropped (§11 PII Scrubbing).
- (obs) The over-envelope run's report names the breach cause in **structured** fields with no absolute host path (no drive-letter / `/home` / `/Users` / `%APPDATA%` scenario-file path in the message), and logs at `warn` per the level mapping (§6 Log levels, §11 Logs).
- (obs) No metric instrument, histogram, meter provider, or OTel SDK init is added for rate/duration measurement; the envelope terms are asserted as JSON fields at report-generation time (§5, §11 Metrics).
- (obs) Any new span conforms to `{module}.{operation}` and is added explicitly to the bounded span-name set; no scenario/profile value appears in a span *name* (§4, §11 Spans / Traces).

## Relevant amendment history
- **2026-08-09-interpretation-correctness-posture** (§4 denominator semantics) — recorded that a derived qualifier can ride *alongside* a closed set (the `(N unbacked)` term qualifies the auto count rather than becoming a fifth mode, so the summands still sum). Directly precedent-bearing for this chunk's Open question 1: a run-level qualifier orthogonal to the closed `ReportState` set has an accepted shape here; a sixth member does not.
- **2026-08-09-out-of-scope-classification-treatment** (§4 coverage gate) — pinned `coverage_percent`'s denominator to the in-scope count (manifest minus out-of-remit rows) after a classification split was rendered on three surfaces. Same failure mode this chunk risks: a new classification that changes what a roll-up counts, recorded so a later gate cannot silently disagree.
- **2026-08-08-sut-capability-manifest** (§4 / §1 must-trace) — de-hardcoded the coverage-gate span attributes when a SUT advance invalidated a literal count. Same root cause as this chunk (SUT-advance reversal); reinforces "name the SET, never a literal" for the catalog and the envelope terms.
- **2026-06-16-emission-journal-writer** (§3 + §6, all four envelope reproductions) — `read_back_observed_at` added after obs-plan was found to be the LONE doc diverging from the schema OWNER (tests §3). The lesson binding this chunk: envelope changes must be made in the owner first and then reproduced in *every* obs copy, or the reproductions drift.
- **2026-06-15-log-error-boundary-redaction** (§6 gate + §11) — established that the field-name allowlist DROPS non-allowlisted fields and that the value scrub targets absolute host-FILE paths, not `::` tokens. This is why an unregistered environment-suspect field would vanish silently.
- **2026-06-27-obs-ci-conformance-gate** (§9) — separated the self-obs base-line gate from the (still unbuilt) envelope gate. Relevant because the new field lands on the envelope record, which currently has no CI conformance gate to catch a regression.
