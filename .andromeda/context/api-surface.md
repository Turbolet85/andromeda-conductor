# API Surface

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's Standard Contracts. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-16T16:46:23Z
**Tooling:** `cargo public-api --simplified -p <crate>` per workspace member (cargo-public-api 0.51 has no `--workspace` flag — run per lib crate); auto-trait/blanket impls elided for readability
**Source:** arch.md §Standard Contracts / §Occupied Resources — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:api-surface start -->
## Public API per workspace member (`cargo public-api --simplified`)

- `conductor-core` → the shared type vocabulary + config-validation surface + the self-observation init surface + the artifact-hygiene redaction primitive (gained the `redact` module this chunk):
  - `pub enum Verdict { Pass, Fail, CalibrationRegion }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub struct Scenario { name: String, p_ids: Vec<PId>, seed: u64, slo_tier: SloTier }` — `impl garde::Validate`
  - `pub struct PId(pub String)` (serde-`transparent`) — `impl garde::Validate`
  - `pub enum SloTier { Tier5s, Tier20s, Tier90s }` (serde-renamed `<5s`/`<20s`/`<90s`)
  - `pub enum CoreError { Config(String), Validation(garde::Report) }` (`#[non_exhaustive]`, `thiserror::Error`; `From<garde::Report>` via `#[from]`)
  - `pub fn resolve_under(base: &Path, candidate: &Path) -> Result<PathBuf>` — the `CONDUCTOR_*` path-handle guard (`std::fs::canonicalize` + bounds-check)
  - `pub fn init_observability(default_service_name: &str, run_id: Option<String>) -> ServiceIdentity` — installs the global `tracing` JSON subscriber (stderr) + `std::panic` hook, emits a startup line; called once at binary startup
  - `pub fn mint_run_id() -> String` — filesystem-safe `YYYY-MM-DDTHH-MM-SS-mmm` run-id from `std::time::SystemTime`
  - `pub struct ServiceIdentity { service_name: String, service_version: String, deployment_environment: String, run_id: String }` (pub fields; derives `Debug, Clone`); resolved from `$CONDUCTOR_SERVICE_NAME` / `env!("CARGO_PKG_VERSION")` / `$CONDUCTOR_ENV`
  - `pub fn redact_value(value: &str) -> Cow<'_, str>` — **NEW** this chunk: masks absolute host-file paths → `<redacted>` (std-only; `Cow::Borrowed` on the clean path); the artifact-hygiene value scrubber, reusable by the Epoch-6 run-report writers
  - `pub fn sanitize_error(err: &dyn std::error::Error) -> String` — **NEW** this chunk: single-line scrubbed `Display` text (no `Debug`/backtrace), anyhow-free; the reusable error-edge sanitizer (cli/tauri edge wires it in Epoch 8)
  - `pub type Result<T> = core::result::Result<T, CoreError>`
  - the data types derive `Debug, Clone (+ Copy on the fieldless enums), PartialEq, Eq, Serialize, Deserialize` (`ServiceIdentity` is `Debug, Clone` only — not a serde/verdict type)
- `conductor-timeline` → `pub mod conductor_timeline` (no public items — placeholder)
- `conductor-emit` → `pub mod conductor_emit` (no public items — placeholder)
- `conductor-faults` → `pub mod conductor_faults` (no public items — placeholder)
- `conductor-verify` → `pub mod conductor_verify` (no public items — placeholder)
- `conductor-report` → `pub mod conductor_report` (no public items — placeholder)
- `conductor-cli`, `conductor-tauri` → binary crates (bin `conductor` / `conductor-tauri`); no public API surface. Both now call `conductor_core::init_observability` at `main` startup (service.name `conductor` / `conductor-tauri`).

The five seam libs remain placeholder surfaces (unchanged this chunk); they populate as their feature chunks land. No IPC methods, HTTP endpoints, or event topics yet.
<!-- LIVING:api-surface end -->
