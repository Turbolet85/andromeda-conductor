# API Surface

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's Standard Contracts. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout each wrap (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-14T23:46:47Z
**Tooling:** `cargo public-api --simplified -p <crate>` per workspace member (cargo-public-api 0.51 has no `--workspace` flag — run per lib crate)
**Source:** arch.md §Standard Contracts / §Occupied Resources — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:api-surface start -->
## Public API per workspace member (`cargo public-api --simplified`)

- `conductor-core` → `pub mod conductor_core` (no public items yet — placeholder; `Verdict`/`ReportState` + scenario model land in the next chunk)
- `conductor-timeline` → `pub mod conductor_timeline` (no public items)
- `conductor-emit` → `pub mod conductor_emit` (no public items)
- `conductor-faults` → `pub mod conductor_faults` (no public items)
- `conductor-verify` → `pub mod conductor_verify` (no public items)
- `conductor-report` → `pub mod conductor_report` (no public items)
- `conductor-cli`, `conductor-tauri` → binary crates (bin `conductor` / `conductor-tauri`); no public API surface.

No IPC methods, HTTP endpoints, event topics, or public exports yet — the scaffold is structural only. These populate as their feature chunks land.
<!-- LIVING:api-surface end -->
