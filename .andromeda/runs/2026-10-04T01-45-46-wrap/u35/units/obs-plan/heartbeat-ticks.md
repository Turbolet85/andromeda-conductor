### Heartbeat ticks

- **CLI:** N/A (short-lived per-scenario, 5-120s typical execution; no long-running server)
- **Tauri backend:** Optional every 30s `conductor.tick` event (active scenario count + emission counter) emitted via Tauri `Channel` to frontend UI state; NOT a telemetry span (unstructured counter data for UI rendering)
- **Tauri frontend:** Per-scenario emission counter tick via `Channel` (UI state update, not telemetry)
