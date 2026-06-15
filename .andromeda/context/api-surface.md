# API Surface

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's Standard Contracts. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-15T00:36:47Z
**Tooling:** `cargo public-api --simplified -p <crate>` per workspace member (cargo-public-api 0.51 has no `--workspace` flag — run per lib crate); auto-trait/blanket impls elided for readability
**Source:** arch.md §Standard Contracts / §Occupied Resources — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:api-surface start -->
## Public API per workspace member (`cargo public-api --simplified`)

- `conductor-core` → the shared type vocabulary (first real public surface; tool-reconciled this chunk):
  - `pub enum Verdict { Pass, Fail, CalibrationRegion }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub struct Scenario { name: String, p_ids: Vec<PId>, seed: u64, slo_tier: SloTier }`
  - `pub struct PId(pub String)` (serde-`transparent`)
  - `pub enum SloTier { Tier5s, Tier20s, Tier90s }` (serde-renamed `<5s`/`<20s`/`<90s`)
  - `pub enum CoreError { Config(String) }` (`#[non_exhaustive]`, `thiserror::Error`)
  - `pub type Result<T> = core::result::Result<T, CoreError>`
  - all data types derive `Debug, Clone (+ Copy on the fieldless enums), PartialEq, Eq, Serialize, Deserialize`
- `conductor-timeline` → `pub mod conductor_timeline` (no public items — placeholder)
- `conductor-emit` → `pub mod conductor_emit` (no public items — placeholder)
- `conductor-faults` → `pub mod conductor_faults` (no public items — placeholder)
- `conductor-verify` → `pub mod conductor_verify` (no public items — placeholder)
- `conductor-report` → `pub mod conductor_report` (no public items — placeholder)
- `conductor-cli`, `conductor-tauri` → binary crates (bin `conductor` / `conductor-tauri`); no public API surface.

The five seam libs remain placeholder surfaces (unchanged this chunk); they populate as their feature chunks land. No IPC methods, HTTP endpoints, or event topics yet.
<!-- LIVING:api-surface end -->
