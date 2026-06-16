# arch extract

## Relevance
Relevant — the chunk builds the config-model bridge between conductor-core scenarios and the timeline scheduler; foundational schema + validation design.

## Constraints
- Per §Design Philosophy, this bridge participates in the "outcomes are values, errors are harness faults" discipline — scenario parsing/garde validation failures surface as `ConfigError` harness faults (`Result::Err`), never verdicts, never panics. (arch §Outcomes and §Established Decisions [Error Handling])
- Per §Established Decisions [Validation Library], use serde 1.0.x + garde 0.22.1 for bounds validation co-located with the declarative serde structs; garde 0.22.1's `#[garde(custom)]` is field-level only — cross-field invariants (phase ordering, bounds spanning distinct fields) use garde's `Context` pattern. (arch §Stack · §Conventions [Config conventions])
- Per §Conventions [Naming patterns], phases are keyed by Pulse capability P-ID (P-001..P-060, "no scenario without a P-ID"); declarative scenario config uses kebab-case filenames on disk. (arch §Project Intent, §Cross-cutting)
- The conversion `Scenario → PhaseTimeline` is total, deterministic, and order-preserving; same scenario + seed always produces the same phase sequence shape — no wall-clock reads, no work-stealing, no non-deterministic library calls beyond the seedable `ChaCha8Rng` (rand_chacha 0.9) owned by the timeline scheduler. (arch §Design Philosophy [Determinism], §Established Decisions [Async Runtime Flavor] + [Determinism RNG])
- Per §Module Boundaries [Crate-per-seam], the scenario model likely lives in `conductor-core` (runtime-agnostic library), and the conversion target `PhaseTimeline` lives in `conductor-timeline` (depends on core, never reverse); the conversion ownership is resolved by the seam dependency direction. (arch §Inherited Defaults [Module boundaries])
- Per §Stack, declare any new dependencies (serde/garde/serde_json/tokio/rand_chacha variants) against the pinned `[workspace.dependencies]` — no per-crate version overrides. (arch §Stack, §Infrastructure Patterns [Build system] — no duplicate pins across crates)

## Patterns to follow
- **Scenario fixtures** — declarative `.toml`/`.yaml` files under `scenarios/` with one per P-ID; each exercises a phase sequence with serde round-trip + garde validation end-to-end (config → validated model → `PhaseTimeline`). Naming: kebab-case stem (e.g. `error-baseline-spike.toml` for P-009/P-010). See amendment history entry "2026-06-15-config-validation-surface" (garde validated serde schema, reuses garde `Report`→`CoreError` bridge). (arch §Conventions, scope §In scope — "at least one representative scenario fixture")
- **Determinism assertion** — basic determinism checks (same seed → same phase count / timing shape) land in unit tests; full golden + proptest sweep deferred to Epoch 2 chunk 4. (scope §Out of scope — "Epoch 2 chunk 4 (dev-deps already wired; basic determinism asserts here are fine)")
- **Error wall integration** — `ConfigError` (typed thiserror enum) for parse + garde validation failures; co-located in the scenario-model owning crate; `#[from] garde::Report` bridge. (arch §Conventions [Error handling], §Established Decisions [Error Handling])

## Anti-patterns to avoid
- Do NOT introduce HTTP/network surfaces, async-runtime spawning, or random/non-deterministic logic in the `Scenario → PhaseTimeline` conversion; tokio multi-threaded work-stealing is forbidden. (arch §Design Philosophy [Determinism], §Established Decisions [Async Runtime Flavor])
- Do NOT create a DSL or imperative phase builder; the config is declarative serde only. (arch §Cross-cutting [Config management])
- Do NOT emit OTLP, MCP calls, or SLO verdict logic from the config layer; those are Epoch 3+ (conductor-emit, conductor-verify, conductor-report). This chunk is data-model + validation + conversion only. (scope §Out of scope)

## Contract bindings
- **conductor-core scenario model** ↔ **this chunk**: extends the existing scenario model (do not duplicate); scenario serde structs + garde validators co-locate here or in core (scope open question — "read the current model first"). (scope §Open questions, arch §Module Boundaries)
- **conductor-timeline `Phase`/`PhaseTimeline`** ↔ **this chunk**: the conversion target; conductor-timeline depends on conductor-core, never reverse. Conversion is total + deterministic. (scope §In scope, arch §Established Decisions [Module Boundaries])
- **verdict/error wall** ↔ **tests + obs + security**: config parse/garde failures are harness `Err`, not verdicts; verdict logic (Pass/Fail/CalibrationRegion) and report states (Pass/Fail/ManualCheck/KnownResidual/Blocked) live downstream (Epoch 3+). (arch §Established Decisions [Error Handling], §Conventions [Error handling])

## Acceptance criteria contributions
- (arch) Scenario config lives under `scenarios/` as declarative `.toml`/`.yaml` files, one per P-ID, keyed by kebab-case stem (arch §Occupied Resources, §Conventions [Naming patterns]).
- (arch) Per-phase emission spec schema (serde + garde) co-located with the scenario model in `conductor-core` (or clarified by scope P4 research); no DSL or imperative builders (arch §Module Boundaries, §Cross-cutting [Config management]).
- (arch) `Scenario → PhaseTimeline` conversion is deterministic, total, and order-preserving; same scenario + seed reproducibly shapes the same phase sequence (no wall-clock reads, no RNG beyond `ChaCha8Rng` owned by scheduler) (arch §Design Philosophy [Determinism], §Established Decisions [Determinism RNG]).
- (arch) Config-parse + garde-validation failures surface as typed `ConfigError` harness faults (`Result::Err`), never verdicts or panics; invalid configs are rejected at the gates (arch §Established Decisions [Error Handling], §Verdict/error wall).

## Relevant amendment history
- **2026-06-15-config-validation-surface** — garde pinned 0.23.0 → 0.22.1 (no `garde_derive 0.23.0` on registry); cross-field validators use `Context` pattern (field-level `#[garde(custom)]` only); reuses garde `Report`→`CoreError` bridge. *Affects this chunk:* validation for per-phase fields + cross-field phase-ordering invariants must use this pinned 0.22.1 + `Context` pattern.
- **2026-06-16-seeded-phase-scheduler** — rand_chacha 0.9 (`ChaCha8Rng`) + rand_core 0.9 registered as the timeline scheduler's sole seedable RNG source (platform/version-stable via `seed_from_u64`). *Affects this chunk:* the conversion must NOT introduce parallel randomness; seed-driven jitter is owned by the timeline scheduler, not the config model.
