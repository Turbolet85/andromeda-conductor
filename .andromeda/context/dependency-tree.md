# Dependency Tree

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's planned module structure. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout each wrap (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** null
**Tooling:** `cargo modules generate tree` (fallback `cargo tree`)
**Source:** arch.md §Inherited Defaults Workspace crates / §Directory structure — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:dep-tree start -->
## Planned modules (initial seed — replaced by tooling output on first reconcile)

- **`conductor-core`** — runtime-agnostic engine library every other crate depends on (shared `Verdict`/`ReportState` types, scenario model)
- **`conductor-timeline`** — deterministic seeded phase scheduler on `tokio::time`
- **`conductor-emit`** — OTLP raw-type emission primitives (opentelemetry-proto + tonic/prost), gRPC egress to `:4317`
- **`conductor-faults`** — fault helpers (ramps, silence, port-occupier, fingerprint generation)
- **`conductor-verify`** — MCP read-back client (rmcp over `TokioChildProcess` stdio), preflight gate, verdict logic
- **`conductor-report`** — JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam
- **`conductor-cli`** — `agent-run` binary, headless source of truth + release gate
- **`conductor-tauri`** — Tauri 2 GUI bin (commands + live-counter `Channel`; React 19 webview)

## Planned dependency edges

- `conductor-core` is the base library; `conductor-timeline` / `-emit` / `-faults` / `-verify` / `-report` each depend on `conductor-core`.
- `conductor-cli` (bin) and `conductor-tauri` (bin) compose the seam crates into a single process — a forbidden cross-seam dependency simply will not compile (the `Cargo.toml` edges ARE the architecture).

---

_(LIVING block content above is the cold-start seed. On the first wrap-session P4 reconcile after code ships, this entire LIVING block is replaced with tooling output. METADATA `Last reconciled` is updated to mark the transition.)_
<!-- LIVING:dep-tree end -->
