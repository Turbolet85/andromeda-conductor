# Scope — Config-validation surface

**Marker:** 2026-06-15-config-validation-surface
**Epoch:** 1 — Foundation
**Working entry:** "Config-validation surface — serde + garde range/cross-field rules, CONDUCTOR_* path-handle canonicalize"

## What this chunk builds
The validation seam for Conductor's declarative config — attaching `garde` to the `conductor-core` scenario identity types the previous chunk left serde-only, wiring garde's failure type into the verdict/error wall as a harness fault, and adding the `CONDUCTOR_*` path-handle canonicalize + bounds-check guard. This is the chunk the prior scope explicitly deferred ("NO garde validation … the NEXT Foundation chunk ('Config-validation surface')" + "NO `CONDUCTOR_*` path-handle canonicalization"). It establishes the validation PATTERN/surface; pure load-time validation + a path guard — no async, no scenario-file I/O beyond `canonicalize`.

- **garde `#[derive(Validate)]` on the scenario identity model** — attach declarative bounds to the existing `Scenario`/`PId`/`SloTier` types: non-empty `p_ids` (the "no scenario without a P-ID" law as a `length(min=1)` rule, not merely a non-optional field), P-ID format/range validation (the `P-NNN` shape over 001..060), and seed/name sanity. Validation co-located with the serde structs in their owning seam crate, per arch convention.
- **The cross-field `#[garde(custom = …)]` machinery** — establish the struct-level custom-validator pattern garde uses for cross-field invariants — the home the Epoch-2 emission-spec rules (error fraction ∈ [0,1], p50≤p95≤p99 ordering, severity-mix sums) will extend. Any cross-field invariant expressible over the CURRENT identity fields lands here as the worked example; the emission-spec field rules themselves wait for those fields (Epoch 2).
- **garde `Report` → `CoreError` bridge (the verdict/error wall extension point)** — add the `CoreError` variant that `#[from]`-converts garde's validation `Report` into a harness fault (the `ConfigError` class), filling the `#[non_exhaustive]` extension point the prior chunk explicitly left for it. A validation failure is `Result::Err` (harness fault: Conductor was handed bad config), never a `Verdict`/`ReportState` — the wall holds.
- **`CONDUCTOR_*` path-handle canonicalize + bounds-check** — the path-traversal guard for the `CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_CONTRACT_MANIFEST` env handles: `std::fs::canonicalize` + bounds-check, sitting OUTSIDE garde (security rule: path handles are validated at the edge, not via garde rules). The reusable guard/helper lands here; its wiring into the binary edge is consumed when the actual runs.db / journal / manifest path-uses exist.

## Boundaries (NOT in this chunk)
- NO per-phase emission-spec model / declarative timeline-phase fields — Epoch 2 ("Scenario-config model"). The emission-spec field rules (error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99, severity-mix sums) attach to THOSE fields when they land; this chunk builds the surface/pattern + the rules expressible over the current identity model.
- NO scenario-file loading/deserialization pipeline (reading `scenarios/*` off disk) — the loader is later; this chunk validates the in-memory deserialized struct and provides the path-canonicalize guard.
- NO `runs.db` / journal writes or manifest reads that CONSUME the canonicalized paths — Epoch 6 (report seam) + the CLI edge. This chunk provides the guard; the consumers wire it in later.
- NO new seam crates and no behavior in timeline/emit/faults/verify/report — validation lives co-located in `conductor-core` (+ the path guard at the core/cli edge; exact crate is a plan-time placement decision).
- NO clap/CLI-arg parsing — `CONDUCTOR_*` are env handles; CLI-flag precedence over env is an Epoch-8 CLI chunk.

## Surfaces / contracts touched
- arch §Established Decisions [Validation Library] serde 1.0.x + garde 0.23.0 — `range` rules + `#[garde(custom)]` cross-field, co-located with the serde structs in their owning seam crates.
- arch §Conventions "Config conventions" — durations non-negative, error fractions ∈ [0,1], p50≤p95≤p99 ordering, severity-mix sums (the rule catalog this surface enforces as fields arrive).
- arch §Established Decisions [Error Handling] + §Conventions "Error handling" — garde's validation `Report` becomes a `ConfigError` harness-failure class via `#[from]`; the verdict/error wall (validation failure = `Err`, never a verdict).
- security-plan §Input Validation — validate ALL scenario config at load with garde; `CONDUCTOR_*` path handles sit OUTSIDE garde (`std::fs::canonicalize` + bounds-check at the edge before any runs.db/journal/manifest path use); the path-traversal guard class.
- arch §Occupied Resources "Environment variables" — `CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_CONTRACT_MANIFEST` / `CONDUCTOR_SEED` reserved namespace; precedence files-over-env, CLI-over-env.
- Extends the prior chunk's `CoreError` `#[non_exhaustive]` enum — the garde `Report` `#[from]` was explicitly left for here.

## Acceptance hints (refined into criteria in plan.md)
- `garde` 0.23.0 wired into `conductor-core` via the workspace edge (`garde.workspace = true`) — already pinned in `[workspace.dependencies]` by the scaffold chunk, so only the per-crate edge is added; `Cargo.lock` stays un-drifted. _(P5 amendment: research found garde already workspace-pinned.)_
- `Scenario`/`PId` derive `garde::Validate`; non-empty `p_ids` + P-ID format/range rules enforced; `.validate(&())` rejects an empty-`p_ids` / malformed-P-ID scenario and accepts a well-formed one.
- The cross-field `#[garde(custom)]` struct-level validator pattern is established (a worked example over the current identity fields).
- `CoreError` gains a `#[from] garde::Report` variant (the ConfigError class); a validation failure surfaces as `Result::Err`, never a `Verdict`/`ReportState`.
- A `CONDUCTOR_*` path-handle resolver canonicalizes + bounds-checks (rejects traversal / out-of-bounds), unit-tested with a traversal attempt.
- `cargo build -p conductor-core` + `cargo test -p conductor-core` green (incl. validation-reject + path-guard unit tests); `cargo clippy` clean.
