# Fan-out results — 2026-06-22-fingerprint-storm-scenarios wrap

| doc | verdict |
|---|---|
| arch | **1 proposal** — D-arch-resources (warning): register `RetryStorm` read-back token in §Occupied Resources |
| security-plan | `proposals: []` (garde validates the TOMLs; no sidecar/data-dir touch; no deps) |
| design-system | `proposals: []` (config, no UI rendered) |
| layout-templates | `proposals: []` (no user-facing surface) |
| test-plan | `proposals: []` (rstest + cargo-nextest match §2/§4; harness + envelope intact) |
| obs-plan | `proposals: []` (D-obs-instrumentation did NOT fire — scenario-config → Epoch-8-driver deferral, per playbook line 55; no deps/redaction surface) |
| a11y-plan | `proposals: []` (no interactive UI; no schema change) |

## arch proposal (raw)
```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "Occupied Resources / Interface routes / surfaces (no HTTP routes)"
    change: "MCP tools consumed: query_incident_list, retrieve_report (degraded_mode), retrieve_telemetry_slice, mark_incident_resolved; read-back tokens recognized: RetryStorm (inferred fingerprint pattern, P-017/P-018 storm detection)."
    sidecar: "Registered RetryStorm token as a new read-back symbol in the MCP interface contract."
    rationale: "Report notes RetryStorm as an inferred token / Epoch-8 calibration point in both fingerprint TOMLs; argued it is a new symbolic resource in the read-back contract that must be inventoried alongside the four tool names."
```

## Orchestrator validation
- **Recommendation: DISMISS (over-reach).** `RetryStorm` is a read-back / detector-output TOKEN declared as scenario-config CONTENT inside `scenarios/fingerprint-storm.toml` — "other per-item content within an already-registered directory" (playbook rule line 49). arch tracks the read-back contract at the **4 MCP tool names** (§Occupied Resources) + the **run-report envelope shape** (§Standard Contracts), NOT the open-ended detector-output token vocabulary. 11 prior tokens (`ErrorRateSpike`, `ServiceWentSilent`, `RestartEvent`, `LatencyRegression`, `ReceiverFailed`, `Idle`, `Stalled`, `Receiving`, `ERROR`, `WARN`, `exception`) were never registered; `RetryStorm` is identical. The chunk added no new port/socket/endpoint/IPC/event/env-var/crate (report Changes §Symbols = none).
- **New flavor → escalate to confirm + add pre-emption rule.** Rule 49 covers it in spirit ("other per-item content"), but this is the first time a detector argued the token-as-interface-element angle, and it WILL recur on every remaining Epoch-7 scenario chunk (severity-lifecycle adds `Autonomous`/`Resolved`/`Curious`; constellation; scrub/pipeline). Per project discipline (rules 49/52/55 were each added WITH the user to pre-empt re-fire), escalate to confirm dismiss + broaden rule 49 to name read-back tokens explicitly.
