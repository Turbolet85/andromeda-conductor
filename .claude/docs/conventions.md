# Project Conventions

_Extracted from `.andromeda/architecture.md` §Conventions + §Data model conventions. Detailed reference beyond CLAUDE.md's budget._

## File & directory naming
- snake_case Rust source files and identifiers (Rust convention).
- Crates named `conductor-<seam>` (kebab-case crate names, snake_case module paths).
- On-disk artifacts use kebab-case Markdown filenames (e.g. `coverage-matrix.md`, the run report) and per-run JSONL journals stemmed by `run_id`.
- Directory layout: `crates/` (**9** workspace members — 7 seam/library crates + the `conductor-cli` and `conductor-tauri` bins) · `scenarios/` (one config per P-ID) · `contracts/` (pinned MCP manifest + SUT capability manifest + SUT load envelope + SUT run contract) · `runs/` (journals + reports + `runs.db`) · `scripts/` (`agent-run.{sh,ps1}`). See architecture.md §Infrastructure Patterns for the full tree.

## Identifier & scenario naming
- Scenarios are keyed by Pulse capability P-ID, drawn from the SUT capability manifest's accepted set (`contracts/pulse-capabilities.toml`) — **"no scenario without a P-ID."**
- Crate prefix `conductor-*` is the reserved namespace.

## Data model conventions (SQLite / `runs.db`)
- Raw SQL, **no migration framework**; primary index keyed by `run_id` (with `seed`/`scenario`) — no synthetic UUID/serial PK.
- **Three tables**, each added additively under `CREATE TABLE IF NOT EXISTS`:
  - `runs` — the scenario-grained run index keyed `(run_id, scenario)` (run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps).
  - `run_envelope` — the run-level SUT-load-envelope standing keyed `run_id` (`classification` · `cause`).
  - `run_check` — the per-check index keyed `(run_id, scenario, check_index)` (`kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms`).

  Each qualifier is a separate table rather than a column on `runs`, on the same reasoning at a different grain: the envelope qualifier is run-scoped, the check qualifier is check-scoped — so `runs` keeps its eleven columns and `state` stays the closed five-variant set.
- Column types are fixed on first write (no migration framework to coerce later): on `runs`, `latency_ms` is INTEGER milliseconds (NULL for blocked rows); `journal_emitted_at`/`read_back_observed_at` are stored as **TEXT RFC-3339** (the JSONL envelope's wire form), while the integer-millisecond value the SLO math consumes is the **separate `latency_ms` INTEGER column**; `slo_tier` is a closed TEXT enum over exactly `<5s`/`<20s`/`<90s`; fingerprint arrays as a JSON1 TEXT array.
- **Nullability is qualified PER TABLE:** on `run_check` both `latency_ms` and `deadline_ms` are INTEGER **NOT NULL** — a blocked or declare-only check emits no row at all rather than a NULL one — and `budget_ms` is INTEGER NULL when the check inherits its scenario's tier.
- Timestamps: wall-clock journal stamps come from `std::time::SystemTime`/`Instant` — **never tokio's virtual clock** (would break journal-relative SLO math).

## Interface / contract conventions
- Three pinned interface surfaces, no network API of Conductor's own: outbound OTLP/gRPC to `127.0.0.1:4317`; inbound MCP read-back via a **hand-rolled line-delimited JSON-RPC client** over the sidecar's stdio (rmcp removed 2026-06-27 — Pulse's `tools/call` is non-MCP-compliant, returning the raw tool payload as the JSON-RPC `result`), version-pinned manifest + `initialize` preflight gate with the **blocked** state on mismatch; internal Tauri 2 `#[tauri::command]` + one `Channel`. REST/GraphQL/tRPC are N/A.
- Version negotiation reduces to reading the `initialize` result's `protocolVersion` — there is no client-library default to drift from; the contract manifest pins `2024-11-05`.
- `run_id` timestamp form: filesystem-safe hyphen-delimited `YYYY-MM-DDTHH-MM-SS-<suffix>` (it's the `runs.db` PK + artifact stem; colons are illegal on Windows). In-payload instants use colon-delimited RFC-3339.
- **Per-check record** (the second shared artifact shape): a graded check serializes as a nine-key `CheckRecord` — `run_id` · `scenario` · `check_index` · `kind` · `verdict` · `state` · `latency_ms` · `deadline_ms` · `budget_ms` — riding the per-run JSONL journal, the `run_check` table, and an indented per-check line in the Markdown report. The envelope's eleven-field shape is unchanged; a blocked or declare-only scenario emits ZERO check records.

## Error handling
- Typed `thiserror` enums per seam (`EmitError`, `VerifyError`, `ContractMismatch`, `ConfigError`); `anyhow` only at `conductor-cli` / `#[tauri::command]` edges.
- The verdict/error wall: `enum Verdict { Pass, Fail, CalibrationRegion }` and `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` are returned as `Ok(...)`; `Result::Err` means a harness fault (config parse, transport down, MCP unreachable). `tonic::Status` codes + MCP error responses are first-class verification inputs, never panics.
- Scenario load-error mapping is **three-way**: `CoreError::Config` on a TOML parse failure OR a non-garde load-path check failure (`check_budgets`, `check_capabilities`); `CoreError::Validation` only on a garde failure (it is pinned `#[from] garde::Report` and structurally cannot carry a hand-written message). All three sit on the `Result::Err` side of the wall.

## Config conventions
- Declarative scenario config (no DSL), serde-deserialized + garde-validated at load, co-located with the serde structs in their owning seam crate, **diving into nested spec structs rather than skipping them**. Durations non-negative; a bounded error fraction (shipped as an integer `error_percent` ∈ 0..=100); bounded per-phase `occurrences`; ramp factors sane; p50≤p95≤p99; severity-mix sums.
- A phase's emission shape is declared data — `EmissionSpec { signal, occurrences, shape }` with a `kind` discriminator selecting its emit-primitive family; `occurrences: 0` is a deliberate silence window, and the dispatcher paces a phase's emissions across that phase's own gap.
- A phase may also declare a FAULT for its window — the optional `[phases.fault]` table carrying the closed `FaultKindSpec::PortOccupier` kind (an unknown kind is a parse error, never an inferred default), under the invariant that a fault-declaring phase is silent (`fault_phases_are_silent`: fault ⇒ `occurrences = 0` — the occupier port is never a config field).
- An `[[expected]]` check may declare `budget_ms` — the optional sub-tier deadline it is graded against (garde `range(min = 1, max = MAX_BUDGET_MS)`, the ceiling DERIVED from `SloTier::Tier90s.deadline_ms()`); absent means it inherits its scenario's tier. Because the rule spans `expected` and its sibling `slo_tier`, it is checked at load by `Scenario::check_budgets()`, not by garde.
- A scenario may declare `[[checklist]]` — an array-of-tables of `ChecklistItem { induced, observation }` (`#[serde(default)]`, garde `dive`, both halves `length(min = 1, max = MAX_CHECKLIST_TEXT)` with `MAX_CHECKLIST_TEXT = 200`, a dialog row's display bound) carrying the operator-checklist rows a hold renders; `HoldPoint` carries the same vector so the hold projects them to its shell. Because the rule spans `checklist` and its sibling `expected`, it too is checked at load — by `Scenario::check_checklist()`, which rejects a `[[checklist]]` declared beside a non-empty `expected` (the checklist renders only on the operator-checklist path, which a checks-bearing scenario never takes). Exactly two committed scenarios declare one: `halo-hue-encoding` and `halo-breathing-encoding`.
- **The garde sibling boundary:** garde 0.22.1's field-level `custom` receives `(&field, &())` and can see no SIBLING field. An invariant contained WITHIN one field can lift one altitude up as a `custom` on that field (`fault_phases_are_silent` on `phases`); an invariant spanning a field and its sibling ships instead as a plain load-path `Scenario::check_*()` invoked from `from_toml_str` (`check_capabilities`, `check_budgets`, `check_checklist`).
- Conductor config handles live under the reserved `CONDUCTOR_*` env namespace; precedence is files over env defaults, CLI flags over env. The committed SUT-facing manifests under `contracts/` resolve from a fixed `default_path()` with deliberately NO `CONDUCTOR_*` override.

## Logging
See `.claude/rules/observability.md` for enforcement. Conventions: structured JSONL via `tracing-subscriber`, `run_id` on every line, no OTel SDK for self-obs.

## Testing
See `.claude/rules/testing.md` for enforcement. Conventions: snake_case `#[test]`/`#[tokio::test]`/`#[rstest]` fns; per-seam crate grouping; golden snapshots under `<crate>/tests/snapshots/`; `proptest-regressions/` committed.

## Cross-references
- Complete architecture → `.andromeda/architecture.md`
- Directory layout → `.andromeda/architecture.md` §Infrastructure Patterns
- Warnings & gotchas → `.claude/docs/gotchas.md`
