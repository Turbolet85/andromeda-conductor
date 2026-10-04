### Bootstrap phases (for downstream skills)

Downstream skills (route, setup-project) derive:
- **logger-stack-install:** `tracing 0.1.44` + `tracing-subscriber 0.3.23` + JSON formatter + file sink integration
- **service-identity-wire:** `service.name` compile-time via `env!("CARGO_PKG_NAME")`; `deployment.environment` runtime via `$CONDUCTOR_ENV` env var
- **log-format-schema-emit:** Emit JSON schema file (tests harness binding contract)
- **run-id-correlation-wire:** Tauri IPC envelope carries `run_id` (correlation; no W3C `traceparent`)
- **pii-scrubbing-wire:** Redaction layer on journal write + report generation (Section 4 / Section 11)
- **obs-ci-gate-wire:** cargo-nextest JSON output + CI artifact upload (`logs/agent-latest.jsonl`)

---
