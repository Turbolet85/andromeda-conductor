# Project Conventions

_Extracted from `.andromeda/architecture.md` §Conventions + §Data model conventions. Detailed reference beyond CLAUDE.md's budget._

## File & directory naming
- snake_case Rust source files and identifiers (Rust convention).
- Crates named `conductor-<seam>` (kebab-case crate names, snake_case module paths).
- On-disk artifacts use kebab-case Markdown filenames (e.g. `coverage-matrix.md`, the run report) and per-run JSONL journals stemmed by `run_id`.
- Directory layout: `crates/` (8 seam crates) · `scenarios/` (one config per P-ID) · `contracts/` (pinned MCP manifest + SUT capability manifest + SUT load envelope) · `runs/` (journals + reports + `runs.db`) · `scripts/` (`agent-run.{sh,ps1}`). See architecture.md §Infrastructure Patterns for the full tree.

## Identifier & scenario naming
- Scenarios are keyed by Pulse capability P-ID, drawn from the SUT capability manifest's accepted set (`contracts/pulse-capabilities.toml`) — **"no scenario without a P-ID."**
- Crate prefix `conductor-*` is the reserved namespace.

## Data model conventions (SQLite / `runs.db`)
- Raw SQL, **no migration framework**; primary index keyed by `run_id` (with `seed`/`scenario`) — no synthetic UUID/serial PK.
- Column types are fixed on first write (no migration to coerce later): `latency_ms` INTEGER ms (NULL for blocked rows); `journal_emitted_at`/`read_back_observed_at` stored as the same integer-millisecond journal offsets the SLO math consumes (NOT ISO strings); `slo_tier` a closed TEXT enum over exactly `<5s`/`<20s`/`<90s`; fingerprint arrays as a JSON1 TEXT array.
- Timestamps: wall-clock journal stamps come from `std::time::SystemTime`/`Instant` — **never tokio's virtual clock** (would break journal-relative SLO math).

## Interface / contract conventions
- Three pinned interface surfaces, no network API of Conductor's own: outbound OTLP/gRPC to `127.0.0.1:4317`; inbound MCP read-back (rmcp, version-pinned manifest + preflight); internal Tauri 2 `#[tauri::command]` + one `Channel`. REST/GraphQL/tRPC are N/A.
- `run_id` timestamp form: filesystem-safe hyphen-delimited `YYYY-MM-DDTHH-MM-SS-<suffix>` (it's the `runs.db` PK + artifact stem; colons are illegal on Windows). In-payload instants use colon-delimited RFC-3339.

## Error handling
- Typed `thiserror` enums per seam (`EmitError`, `VerifyError`, `ContractMismatch`, `ConfigError`); `anyhow` only at `conductor-cli` / `#[tauri::command]` edges.
- The verdict/error wall: `enum Verdict { Pass, Fail, CalibrationRegion }` and `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` are returned as `Ok(...)`; `Result::Err` means a harness fault (config parse, transport down, MCP unreachable). `tonic::Status` codes + MCP error responses are first-class verification inputs, never panics.

## Config conventions
- Declarative scenario config (no DSL), serde-deserialized + garde-validated at load, co-located with the serde structs in their owning seam crate. Durations non-negative; error fractions ∈ [0,1]; ramp factors sane; p50≤p95≤p99; severity-mix sums.
- Conductor config handles live under the reserved `CONDUCTOR_*` env namespace; precedence is files over env defaults, CLI flags over env.

## Logging
See `.claude/rules/observability.md` for enforcement. Conventions: structured JSONL via `tracing-subscriber`, `run_id` on every line, no OTel SDK for self-obs.

## Testing
See `.claude/rules/testing.md` for enforcement. Conventions: snake_case `#[test]`/`#[tokio::test]`/`#[rstest]` fns; per-seam crate grouping; golden snapshots under `<crate>/tests/snapshots/`; `proptest-regressions/` committed.

## Cross-references
- Complete architecture → `.andromeda/architecture.md`
- Directory layout → `.andromeda/architecture.md` §Infrastructure Patterns
- Warnings & gotchas → `.claude/docs/gotchas.md`
