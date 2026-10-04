### Log format JSON schema

Schema (binding contract from upstream-context Section 5 Test Plan Excerpt → Test Harness Contract Summary):
```jsonl
{
  "journal_emitted_at": "ISO-8601 from std::time::SystemTime",
  "read_back_observed_at": "ISO-8601 from std::time::SystemTime (null until read-back; null for blocked rows)",
  "run_id": "YYYY-MM-DDTHH-MM-SS-<suffix> (filesystem-safe hyphen-delimited)",
  "seed": "u64",
  "scenario": "string",
  "p_ids": ["P-001", "P-002", ...],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": "integer or null (null for blocked rows)",
  "slo_tier": "<5s | <20s | <90s",
  "fingerprints": ["fingerprint1", "fingerprint2", ...] or empty array
}
```
Additional fields per scenario — an extension point of the envelope shape. Its only ever-named instance, `degraded_mode_response`, is RETIRED: measured 2026-09-06 with ZERO occurrences anywhere under `crates/` (`grep -c 'degraded_mode_response' crates/`), so it was never implemented and no scenario emits it. The extension point itself stands, and as of 2026-09-07 it IS exercised — by the a11y CI gate's violation record at `runs/a11y/<run_id>.jsonl`, which carries the eleven envelope keys PLUS the §9 resource tags `service.name` (`conductor-ui`) and `deployment.environment` (13 top-level keys measured locally, 15 under CI where `ci.run.id` + `git.commit.sha` join them), as measured at `conductor-0.2.0/chunks/2026-09-07-a11y-ci-gate/report.md`. Note what the extras are: resource TAGS on a harness artifact, not scenario-specific fields — the scenario record at `runs/<run_id>.jsonl` still carries the eleven alone. The superset is admitted deliberately: `conductor-run`'s `journal_conformance` asserts key PRESENCE (plus closed sets and host-path freedom), never key exclusivity, which is what lets one gate serve both shapes.
- Agent-parseable via `jq` and `serde_json`
- No absolute host paths, no internal struct names (redaction layer in Section 4 / Section 11)

**Two record shapes (clarified 2026-06-15-structured-logging-stack):** the schema block above is the **Run-report envelope** — the scenario-result record (emission journal `runs/<run_id>.jsonl` + the `runs.db` `runs` row), populated by the report seam (Epoch 6) on scenario-result events. **The report seam writes a SECOND line shape onto that same journal (2026-08-21):** the per-check `CheckRecord` (`run_id` · `scenario` · `check_index` · `kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms`), mirrored into the `run_check` table; it is a finer GRAIN beneath the envelope, never an extension of it — the envelope's eleven fields are unchanged, and a blocked or declare-only scenario emits no check line at all. Format owned by test-plan §3 (§3 Log format, Agent parsing) — a typed parse discriminates the two. Both are report-seam records, so the self-obs-line split below is unaffected, and neither goes through the span-attribute field allowlist (that governs the self-obs line only). The foundational **self-obs log line** (every `tracing` line; stderr / `logs/agent-latest.jsonl`) carries a smaller base set: `timestamp_ms` (epoch millis from `std::time::SystemTime` — the self-obs line stamp; the envelope's `journal_emitted_at` ISO-8601 remains the SLO-math field), `level`, `target`, the service-identity fields (`service.name` / `service.version` / `deployment.environment`), and `run_id`. Service-identity + `run_id` are on **every** line; the envelope/result fields appear only on the scenario-result record. The implementation uses a small custom `tracing-subscriber` layer (stock `fmt().json()` cannot emit constant identity fields flat at the top level). That layer emits the self-obs line in **two variants over the same base set** (format owned by test-plan §3): the **event line**, and the **span-lifecycle line** adding `span` (the bounded §4 span name), `span_event` (`new` | `close`), an optional `parent`, and the span's own allowlisted attributes on the `new` line — so a §4 span materializes as real lines rather than being implicit. Both are self-obs lines; the two-record-shapes split above is unaffected.
