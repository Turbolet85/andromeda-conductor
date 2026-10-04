### Status endpoint shape

```json
{
  "run_id": "string", "seed": 0, "scenario": "string", "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0, "slo_tier": "string",
  "journal_emitted_at": "ISO-8601", "read_back_observed_at": "ISO-8601",
  "fingerprints": []
}
```

No HTTP/IPC status *endpoint* exists (no listener). This is the Run-report envelope (arch Standard Contracts), serialized identically into the Markdown report, the `runs` row, and the envelope JSONL journal line. Per-check detail is a separate finer grain, never an extension of the envelope (whose eleven fields are unchanged): the `run_check` table keyed `(run_id, scenario, check_index)` (read via `RunsDb::checks_for`) plus its own journal lines and an indented per-check line in the Markdown report. The agent polls it by reading the `runs.db` row (bound-parameter SQL) or the JSONL journal during/after each scenario. Artifacts MUST NOT leak absolute host paths or internal seam-crate struct names (security anti-pattern) — enforced by the insta golden snapshot + a tracing field-allowlist redaction layer.
