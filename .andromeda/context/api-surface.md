# API Surface

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's Standard Contracts. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout each wrap (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** null
**Tooling:** `cargo public-api --simplified --workspace`
**Source:** arch.md §Standard Contracts / §Occupied Resources — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:api-surface start -->
## Planned surfaces (initial seed — replaced by tooling output on first reconcile)

### IPC methods
Tauri 2 `#[tauri::command]` request/response (internal, backend↔bundled webview only — not network-exposed):
- **`start`** (webview → backend) — start a scenario/suite run
- **`stop`** (webview → backend) — stop the active run
- **`pick`** (webview → backend) — scenario/suite picker selection
- **`run_report`** (webview → backend) — fetch the run-report view
- **`operator_pause`** (webview → backend) — go/no-go proceed/abort at a committed step
- **live-counter `Channel`** (backend → webview) — streams live emission counters + target status

MCP tools CONSUMED (rmcp client → Pulse's `andromeda-pulse-mcp`, protocol `2024-11-05`):
- **`query_incident_list`** · **`retrieve_report`** (with `degraded_mode`) · **`retrieve_telemetry_slice`** · **`mark_incident_resolved`**

### HTTP endpoints
No HTTP endpoints — Conductor exposes no HTTP/network service of its own. Outbound only: OTLP/gRPC egress (client) to `127.0.0.1:4317` (`:4318` unused). The `:4317` port-occupier fault is the sole deliberate inbound bind.

### Event topics / message schemas
No message broker / event topics. The only streaming surface is the Tauri `Channel` (live counters, backend → frontend).

### Schema files / public exports
- **Run-report envelope** — the canonical per-check shape shared by the Markdown report + `runs.db` row + JSONL journal (arch §Standard Contracts).
- **Pinned MCP contract manifest** (`contracts/`) — expected protocol `2024-11-05` + required-tool set.
- **`conductor-core` public exports** — `Verdict { Pass, Fail, CalibrationRegion }`, `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`, the scenario model (re-exported to every seam).

---

_(LIVING block content above is the cold-start seed. On the first wrap-session P4 reconcile after code ships, this entire LIVING block is replaced with tooling output. METADATA `Last reconciled` is updated to mark the transition.)_
<!-- LIVING:api-surface end -->
