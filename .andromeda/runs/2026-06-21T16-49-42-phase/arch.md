# arch extract

## Relevance
Partial — the chunk defines hold/resume orchestration in `conductor-core` with integration into `conductor-timeline` or `conductor-verify`, touching the cross-cutting verdict/error wall but not OTLP emission or MCP read-back infrastructure.

## Constraints
- The hold-point model + go/no-go decision type + resolver trait MUST live in `conductor-core` (architecture.md §Inherited Defaults: runtime-agnostic shared vocabulary, both bins import core only) — per the "headless-drivable core, thin shells" design philosophy (§Design Philosophy).
- Hold resolution is a typed VALUE (`Ok(...)` outcome), never `Result::Err` — verdicts and decisions are return values per the verdict/error wall (§Established Decisions: "verification outcomes are VALUES…; `Result::Err` is reserved strictly for Conductor's own failures").
- Hold/resume orchestration MUST preserve determinism: same scenario + seed ⇒ identical emission-stream shape (§Design Philosophy: "Determinism under a seed"); wall-clock pause outside the seeded virtual clock, journal-relative timing only (§Established Decisions: "wall-clock-from-test-start" is forbidden; timestamps from `std::time::SystemTime`/`Instant`, never tokio's virtual clock).
- Headless resolver SHALL NOT block (§Cross-cutting Patterns: "headless never blocks" CLI discipline); auto-resolve to a deterministic outcome in non-interactive runs (§Conventions: crate-per-seam module boundaries enforce this separation).
- No host paths or internal struct names leak through the redaction edge into recorded prompts or outcomes (§Conventions: naming/interface/data-model conventions apply).
- Integration point (await + resume in `conductor-timeline` vs `conductor-verify`) resolves in planning (open seam question); the async-resolver shape is a trait object vs mpsc/oneshot vs async callback to be confirmed (scope.md: "to be confirmed against the architecture").

## Patterns to follow
- Resolver trait pattern: decouple *where the hold is awaited* from *how it is answered* (scope.md §2 + §3) — same core orchestration driven identically by headless and interactive shells; the architecture enables this via the star-topology invariant (all seam crates import `conductor-core` only) (§Infrastructure Patterns: crate-per-seam Cargo workspace).
- Config/types in core, logic in seam: hold model + types + trait in `conductor-core`, integration (await + resume) in the owning seam — mirrors precedent from scenario config (`Scenario` struct in core, load + validation in `conductor-verify` / file I/O in `conductor-report`) (§Established Decisions: Module Boundaries — "crate-per-seam … forbidden cross-seam dependency simply will not compile").
- Test-harness pattern: CI tests resolve holds via immediate auto-resolving stub under `tokio::time` `start_paused`; human pause is wall-clock-only, outside seeded clock — preserves determinism (scope.md §4 — determinism preservation).

## Anti-patterns to avoid
- NO `Result::Err` on hold resolution — `no-go` is a typed value, never a panic or error (§Established Decisions: verdict/error wall).
- NO tokio's virtual clock in hold timestamps — journal-relative from `std::time::SystemTime`/`Instant` only (§Design Philosophy: "wall-clock-from-test-start" forbidden; determinism RNG owns the timeline scheduler's sole non-determinism source per §Established Decisions: [Determinism RNG]).
- NO cross-seam dependency bypass (e.g., `conductor-verify` imports `conductor-timeline` to await a hold defined there) — forbidden by compiler; hold types live in core, seam logic stays walled (§Infrastructure Patterns: crate-per-seam Cargo workspace).

## Contract bindings
obs ↔ holds: structured logging (tracing 0.1.44, per §Stack and Technologies + §Occupied Resources amendment 2026-06-15) records hold lifecycle (e.g., auto-resolve decision, operator decision when CLI interactive); tests ↔ holds: CI test harness auto-resolves holds via stub under `tokio::time` `start_paused` (scope.md §4 — determinism preservation); CLI resolver (Epoch 8) ↔ holds: receives resolver trait + core types, surfaces interactive inquire prompt (deferred consumer, out of scope here); Tauri resolver (Epoch 9) ↔ holds: receives resolver trait + core types, surfaces AlertDialog + paused-count Channel signal (deferred consumer, out of scope here); report (Epoch 6) ↔ holds: surfaces hold outcome (ManualCheck confirmation, no-go decision) in Markdown/runs.db (deferred consumer, out of scope here).

## Acceptance criteria contributions
- (arch) Hold-point model + go/no-go decision type + resolver trait defined in `conductor-core` per workspace boundary rules (§Inherited Defaults: crate-per-seam, star-topology core import).
- (arch) Hold resolution is a typed value, never `Result::Err` — verdict/error wall preserved (§Established Decisions: outcomes are values, errors harness-only).
- (arch) Headless default resolver auto-resolves to defined deterministic outcome, never blocks in non-interactive runs — "headless never blocks" discipline honored (§Cross-cutting Patterns).
- (arch) Same scenario + seed + headless resolver ⇒ identical emission-stream shape + identical recorded hold outcome (determinism preserved per §Design Philosophy: "same scenario + seed always produces same emission stream shape").

## Relevant amendment history
2026-06-15-structured-logging-stack: `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` registered in §Occupied Resources (env-var namespace) — tracing 0.1.44 + tracing-subscriber 0.3.23 already in §Stack per amendment 2026-06-14-cargo-workspace-scaffold; holds' auto-resolve decision will be structured-logged via tracing. 2026-06-14-cargo-workspace-scaffold: self-observation stack (tracing + tracing-subscriber) registered; no impact on holds' core logic, but structured logging of hold lifecycle (auto-resolve, operator-confirm) is enabled. All prior amendments (2026-06-14 through 2026-06-18) touched other seams (core-shared-types, config-validation, exception-events); no prior hold/resume logic touched.