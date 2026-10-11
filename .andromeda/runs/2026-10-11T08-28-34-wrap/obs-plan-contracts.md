# obs-plan.md ## 3. Observability Harness Contract — the keyed contracts, one row per key (U35): key · file · bytes · labels. Read one file whole when the chunk turns on its key; never the set.

- OTel SDK init · .andromeda/registries/contracts/obs-plan/otel-sdk-init.md · 1995 B · labels: SDK packages, Init order, Logging init instead
- Service identity · .andromeda/registries/contracts/obs-plan/service-identity.md · 570 B · labels: service.name, service.version, deployment.environment, Resource attributes
- Logging stack · .andromeda/registries/contracts/obs-plan/logging-stack.md · 1220 B · labels: Library, Format, Sink (CLI), Sink (Tauri backend), Sink (Tauri frontend), Agent-mode flag
- Log format JSON schema · .andromeda/registries/contracts/obs-plan/log-format-json-schema.md · 4226 B · labels: Two record shapes (clarified 2026-06-15-structured-logging-stack)
- Log file location · .andromeda/registries/contracts/obs-plan/log-file-location.md · 1051 B · labels: CLI, Tauri backend, Tauri frontend, Rotation, Paste-to-AI workflow
- Snapshot / paste-to-AI integration · .andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md · 354 B · labels: Snapshot path, Snapshot trigger, Paste-to-AI surface
- Correlation (no distributed tracing) · .andromeda/registries/contracts/obs-plan/correlation-no-distributed-tracing.md · 1023 B · labels: No W3C trace context anywhere, HTTP, gRPC outbound (conductor-emit → Pulse), IPC (Tauri command → conductor-core), Internal async (tokio `current_thread`)
- Heartbeat ticks · .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md · 439 B · labels: CLI, Tauri backend, Tauri frontend
- Bootstrap phases (for downstream skills) · .andromeda/registries/contracts/obs-plan/bootstrap-phases-for-downstream-skills.md · 756 B · labels: logger-stack-install, service-identity-wire, log-format-schema-emit, run-id-correlation-wire, pii-scrubbing-wire, obs-ci-gate-wire
