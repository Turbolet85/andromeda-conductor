# API Surface

_Living artifact. Seeded by `/andromeda-setup-project` Phase 6 from arch's Standard Contracts. Reconciled by `/andromeda-wrap-session` P4 — the LIVING block is replaced wholesale with fresh tooling stdout (per `integrity-protocol.md`)._

<!-- METADATA start -->
**Last reconciled:** 2026-06-16T20:48:47Z
**Tooling:** `cargo public-api --simplified -p <crate>` per workspace member (cargo-public-api 0.51 has no `--workspace` flag — run per lib crate); auto-trait/blanket impls elided for readability
**Source:** arch.md §Standard Contracts / §Occupied Resources — seed; actual code via tooling — reconcile
**Maintenance:** wrap-session P4 (living-docs reconcile)
<!-- METADATA end -->

<!-- LIVING:api-surface start -->
## Public API per workspace member (`cargo public-api --simplified`)

- `conductor-core` → the shared type vocabulary + config-validation surface + the self-observation init surface + the artifact-hygiene redaction primitive (includes the `redact` module):
  - `pub enum Verdict { Pass, Fail, CalibrationRegion }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` — `fn label(&self) -> &'static str`, `fn status_prefix(&self) -> &'static str`
  - `pub struct Scenario { name: String, p_ids: Vec<PId>, seed: u64, slo_tier: SloTier, phases: Vec<PhaseSpec>, jitter_ms: u64 }` — `impl garde::Validate`; `fn from_toml_str(&str) -> Result<Scenario>` (serde-deserialize TOML → garde-validate; parse err → `CoreError::Config`, validation err → `CoreError::Validation`)
  - `pub struct PId(pub String)` (serde-`transparent`) — `impl garde::Validate`
  - `pub enum SloTier { Tier5s, Tier20s, Tier90s }` (serde-renamed `<5s`/`<20s`/`<90s`)
  - `pub struct PhaseSpec { name: String, gap_ms: u64, emission: EmissionSpec }` — `impl garde::Validate` (name 1..=40 chars, `gap_ms` bounded; `emission` serde-default) — the declarative per-phase emission spec
  - `pub struct EmissionSpec { signal: Signal }` (`#[non_exhaustive]`; `fn new(Signal) -> Self`, `Default`) — forward-compatible emission descriptor (the Epoch-3 emission seam extends it)
  - `pub enum Signal { Traces, Metrics, Logs }` (serde snake_case, `Default = Traces`) — the OTLP signal class a phase emits
  - `pub enum CoreError { Config(String), Validation(garde::Report) }` (`#[non_exhaustive]`, `thiserror::Error`; `From<garde::Report>` via `#[from]`)
  - `pub fn resolve_under(base: &Path, candidate: &Path) -> Result<PathBuf>` — the `CONDUCTOR_*` path-handle guard (`std::fs::canonicalize` + bounds-check)
  - `pub fn init_observability(default_service_name: &str, run_id: Option<String>) -> ServiceIdentity` — installs the global `tracing` JSON subscriber (stderr) + `std::panic` hook, emits a startup line; called once at binary startup
  - `pub fn mint_run_id() -> String` — filesystem-safe `YYYY-MM-DDTHH-MM-SS-mmm` run-id from `std::time::SystemTime`
  - `pub struct ServiceIdentity { service_name: String, service_version: String, deployment_environment: String, run_id: String }` (pub fields; derives `Debug, Clone`); resolved from `$CONDUCTOR_SERVICE_NAME` / `env!("CARGO_PKG_VERSION")` / `$CONDUCTOR_ENV`
  - `pub fn redact_value(value: &str) -> Cow<'_, str>` — masks absolute host-file paths → `<redacted>` (std-only; `Cow::Borrowed` on the clean path); the artifact-hygiene value scrubber, reusable by the Epoch-6 run-report writers
  - `pub fn sanitize_error(err: &dyn std::error::Error) -> String` — single-line scrubbed `Display` text (no `Debug`/backtrace), anyhow-free; the reusable error-edge sanitizer (cli/tauri edge wires it in Epoch 8)
  - `pub type Result<T> = core::result::Result<T, CoreError>`
  - the data types derive `Debug, Clone (+ Copy on the fieldless enums), PartialEq, Eq, Serialize, Deserialize` (`ServiceIdentity` is `Debug, Clone` only — not a serde/verdict type)
- `conductor-timeline` → the deterministic seeded phase scheduler (the timeline engine's first real surface):
  - `pub struct Phase { name: String, gap: Duration }` — `fn new(name: impl Into<String>, gap: Duration) -> Self`
  - `pub struct PhaseTimeline { phases: Vec<Phase>, jitter: Duration }` — `fn new(phases: Vec<Phase>, jitter: Duration) -> Self` (ordered phase list + symmetric per-gap jitter bound)
  - `impl From<&conductor_core::Scenario> for PhaseTimeline` — total, deterministic, order-preserving conversion of a validated scenario's phase sequence (`gap_ms`/`jitter_ms` → `Duration`) into the runtime timeline the scheduler consumes
  - `pub struct PhaseTransition { index: usize, name: String, elapsed_ms: u128 }` — a surfaced phase boundary (virtual-ms elapsed)
  - `pub async fn run_timeline(timeline: &PhaseTimeline, seed: u64) -> Result<Vec<PhaseTransition>, TimelineError>` — sequences the timeline on `tokio::time` under a seeded `ChaCha8Rng`; `#[tracing::instrument(name = "timeline.execute")]`
  - `pub enum TimelineError { EmptyTimeline }` (`thiserror::Error`, `#[non_exhaustive]`) — harness fault only (verdict/error wall)
  - the data types derive `Debug, Clone, PartialEq, Eq`
- `conductor-emit` → `pub mod conductor_emit` (no public items — placeholder)
- `conductor-faults` → `pub mod conductor_faults` (no public items — placeholder)
- `conductor-verify` → `pub mod conductor_verify` (no public items — placeholder)
- `conductor-report` → `pub mod conductor_report` (no public items — placeholder)
- `conductor-cli`, `conductor-tauri` → binary crates (bin `conductor` / `conductor-tauri`); no public API surface. Both now call `conductor_core::init_observability` at `main` startup (service.name `conductor` / `conductor-tauri`).

The remaining four seam libs (`conductor-emit` · `conductor-faults` · `conductor-verify` · `conductor-report`) stay placeholder surfaces; they populate as their feature chunks land. No IPC methods, HTTP endpoints, or event topics yet.
<!-- LIVING:api-surface end -->
