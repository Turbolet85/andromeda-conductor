# Scope — conductor-core shared types

**Marker:** 2026-06-15-conductor-core-shared-types
**Epoch:** 1 — Foundation
**Working entry:** "conductor-core shared types — Verdict/ReportState enums, scenario model, verdict/error wall"

## What this chunk builds
The foundational type vocabulary inside `conductor-core` — replacing the scaffold's placeholder `lib.rs` with the real shared types every seam crate matches on. This is the chunk the scaffold's scope explicitly deferred ("`conductor-core`'s `Verdict`/`ReportState`/scenario model are the NEXT chunk"). Pure runtime-agnostic data types + the typed-error scaffolding; no async, no I/O, no behavior.

- **`Verdict` enum** — `{ Pass, Fail, CalibrationRegion }`: the two-state probabilistic-assertion split (deterministic claims hard pass/fail; model-interpretive claims route to `CalibrationRegion`, never hard-failed on exact values). Returned as an `Ok` value, never an error.
- **`ReportState` enum** — `{ Pass, Fail, ManualCheck, KnownResidual, Blocked }`: the five canonical run-report states. Each carries its distinct meaning (ManualCheck = operator-checklist terminal; KnownResidual = pre-accepted residual; Blocked = failed preflight precondition).
- **Status-rendering contract on both enums** — a text label + ASCII status-prefix accessor (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` for cli; label+glyph slot for desktop) so the "status is never color-alone" invariant is satisfied at the type, not the call site.
- **Serde representation** — both enums `Serialize`/`Deserialize` to their canonical PascalCase names exactly as the run-report envelope shows (`"verdict": "Pass"`, `"state": "Pass"`), plus the usual `Debug, Clone, Copy, PartialEq, Eq` derives.
- **The verdict/error wall (as a typed shape)** — `conductor-core`'s own `thiserror` error enum for harness faults (config-parse class and the seam-shared core error variants), establishing the compile-enforced rule: verification outcomes are `Ok(Verdict)` / `Ok(ReportState)`; `Result::Err` is reserved for Conductor's own failures. The pattern + the core error type land here; per-seam error enums (`EmitError`, `VerifyError`, …) arrive with their own seams.
- **The scenario model (core skeleton)** — the shared `Scenario` identity vocabulary: scenario name, the required Pulse P-ID(s) (the "no scenario without a P-ID" law expressed as a non-optional field), seed, and the `SloTier` closed enum (`<5s`/`<20s`/`<90s`, serde-renamed since the wire form is not a Rust identifier). serde-derive only — the struct shape the validation chunk will then attach garde rules to.

## Boundaries (NOT in this chunk)
- NO garde validation — `#[derive(Validate)]`, `range` rules, and the cross-field invariants (error fraction ∈ [0,1], p50≤p95≤p99, severity-mix sums) are the NEXT Foundation chunk ("Config-validation surface"). Here the scenario structs derive serde only.
- NO `CONDUCTOR_*` path-handle canonicalization / bounds-checking (also the Config-validation chunk).
- NO per-phase emission spec / declarative timeline-phase model — that is Epoch 2 ("Scenario-config model" + the seeded phase scheduler). This chunk owns scenario *identity*, not its per-phase emission body.
- NO run-report envelope struct / serializer, no `runs.db` row types — Epoch 6. This chunk only defines the `Verdict`/`ReportState` enums that envelope will embed.
- NO verdict-computation logic (latency/SLO comparison, calibration bucketing) — that is `conductor-verify` (Epoch 5). Types only, no decisions.
- NO MCP/contract types, no OTLP/emission types, no tokio/async — `conductor-core` is runtime-agnostic and these are plain data.

## Surfaces / contracts touched
- arch §Established Decisions [Error Handling] + §Cross-cutting Patterns "Verdict/error wall" — outcomes are values, `Err` is harness-only.
- arch §Standard Contracts "Run report envelope" — `verdict ∈ {Pass, Fail, CalibrationRegion}`, `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`; canonical serialized names.
- arch §Probabilistic-Assertion Policy (two-state split → `Verdict`) + §Read-Back Dependency Posture (the five `ReportState` meanings).
- arch §Conventions "Config conventions" + §Data model conventions — `SloTier` closed enum `<5s`/`<20s`/`<90s`; "no scenario without a P-ID".
- CLAUDE.md universal invariant "Status is never color-alone" → the label + ASCII-prefix accessors.
- Establishes the vocabulary that every seam crate (`timeline`/`emit`/`faults`/`verify`/`report`/`cli`/`tauri`) imports from `conductor-core` — the dependency root of the workspace.

## Acceptance hints (refined into criteria in plan.md)
- `Verdict { Pass, Fail, CalibrationRegion }` and `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` defined in `conductor-core` with exact variants + `Debug/Clone/Copy/PartialEq/Eq` + serde.
- Both serialize to canonical PascalCase names matching the run-report envelope; serde round-trips.
- Each exposes a text label + ASCII status-prefix accessor (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`) — the never-color-alone cli contract.
- A `conductor-core` `thiserror` error enum exists for harness faults; `Verdict`/`ReportState` are NOT error variants (verdict/error wall holds by construction).
- `Scenario` core struct carries a required P-ID (non-optional), seed, name, and `SloTier` (`<5s`/`<20s`/`<90s`, serde-renamed); serde-serializable.
- `cargo build -p conductor-core` + `cargo test -p conductor-core` green (incl. serde round-trip unit tests).
