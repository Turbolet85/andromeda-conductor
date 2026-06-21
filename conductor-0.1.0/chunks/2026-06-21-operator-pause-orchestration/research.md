# Codebase Research — 2026-06-21-operator-pause-orchestration

## Scope
- **Depth:** deep · **Reads:** 12 files · **Globs/Greps:** 4 (code-graph query + 3 rg)

## Files inspected
- `crates/conductor-timeline/src/scheduler.rs` (full) — `run_timeline(&PhaseTimeline, seed) -> Result<Vec<PhaseTransition>, TimelineError>`; async, sleeps each gap on the **virtual** clock, emits nothing, "runtime-agnostic (the caller owns the runtime)". `#[tracing::instrument(name = "timeline.execute", …)]`. `TimelineError` is `#[non_exhaustive]` "because the emission/fault seams extend the timeline-fault surface in later chunks." The candidate integration host if a hold pauses the timeline.
- `crates/conductor-timeline/src/lib.rs` (full) — `pub use phase::{Phase, PhaseTimeline, PhaseTransition}; pub use scheduler::{run_timeline, TimelineError};` — 3-module crate (convert/phase/scheduler).
- `crates/conductor-core/src/lib.rs` (full) — the dep root. Doc states core owns the outcome vocabulary + scenario model + harness-fault `CoreError`, and "**no async and no I/O** beyond the single sync `std::fs` path-handle guard." Re-export list is the public surface (add `pause` exports here).
- `crates/conductor-core/src/expected.rs` (full) — **the type-definition precedent to mirror**: closed enums (`ClaimClass`, `ComparisonKind`) `#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]` serializing to canonical PascalCase; `ExpectedCheck` struct with `#[derive(… Validate)]` + `#[garde(length(min=1))]`; tests assert canonical wire names + JSON round-trip + garde valid/invalid.
- `crates/conductor-verify/src/verdict.rs` (full) — **the evaluator precedent**: `classify(class, matched, observed, expected) -> Assessment` is **infallible** (no `Result`), redacts `observed`/`expected` via `redact_value` before capture, returns a `#[derive(Serialize)]` value struct "the Epoch-6 run-report writers consume … never a `Result::Err` — the verdict/error wall." Tests: determinism + redaction.
- `crates/conductor-core/src/redact.rs` (full) — `redact_value(&str) -> Cow<str>` masks absolute host-path tokens (std-only, no `regex`); `sanitize_error`. `is_allowlisted` (pub(crate)) gates self-obs field names. Any hold prompt/outcome string must pass through `redact_value` before capture (the artifact-hygiene edge).
- `crates/conductor-core/src/report_state.rs` (full) — `ReportState::ManualCheck` **already exists**: "Visual/operator-checklist claim with no programmatic read-back — terminal, awaits a **human go/no-go**. `[MANUAL]`." `label()`/`status_prefix()` pairing (status-never-color-alone). This is the report linkage for a go/no-go hold.
- `crates/conductor-core/src/verdict.rs` (full) — `Verdict::CalibrationRegion` already labels **"HOLD"** / `[HOLD]` (amber). Naming caution below.
- `crates/conductor-core/Cargo.toml` — deps: serde, serde_json, toml, thiserror, garde, tracing, tracing-subscriber. **No tokio / async-trait / futures** — core is strictly sync today.
- `crates/conductor-timeline/Cargo.toml` — deps: conductor-core, tokio(`time`), tracing, thiserror, rand_chacha, rand_core. Async lives here.
- `crates/conductor-verify/Cargo.toml` — deps: conductor-core, rmcp, tokio(`process,io-util,rt,macros`), … Async lives here too; has the feature-gated `stub_pulse_mcp` test-bin precedent.

## Graph impact (code-graph query → `tree-query-2026-06-21-operator-pause-orchestration.json`)
- **`run_timeline`** — callers are **only tests** (`convert.rs` tests at L33/72/73/84/85, lib.rs doctest at L14). `rg` for `run_scenario`/`run_suite`/`async fn run` → **no higher-level run orchestrator exists**. ⇒ This chunk builds the operator-pause mechanism **in isolation** (CI-testable with a stub resolver); threading it into a full preflight→emit→timeline→verify→report run is **Epoch 8** (CLI bootstrap, carried follow-up (c)). The plan must NOT invent that orchestrator now.

## Patterns detected
- **Closed-enum + canonical serde + garde struct** (`expected.rs:20-64`): the exact shape for `HoldPoint` / `Decision`.
- **Infallible classifier → serializable redacted value struct** (`verify/verdict.rs:46-74`): the exact shape for a hold-resolution evaluator/recorder — a value, never `Err`.
- **`redact_value` applied before capture** (`verify/verdict.rs:54-55`): hold prompt/observed strings get redacted at construction.
- **`label()` + `status_prefix()` pairing** (`report_state.rs:27-50`, `verdict.rs:24-44`): status never color-alone; reuse for any new hold glyph (note: `[HOLD]` is taken — see Open questions).
- **Bounded `#[tracing::instrument(name="{module}.{operation}")]`** (`scheduler.rs:34`): if a hold span is added it must be a bounded name (obs flagged `hold.wait_resolve`).
- **`#[non_exhaustive]` thiserror enum for forward-compat** (`scheduler.rs:21-27`): if a hold harness-fault variant is ever needed it follows this; but resolution outcomes are values, not errors.

## Conventions to follow
- **Verdict/error wall** — resolution is a value (`Decision`/resolution struct in `Ok`); `Result::Err` only for a genuine harness fault (e.g. a garde-invalid `HoldPoint` at load → `CoreError::Validation`). The evaluator/headless-resolve path is infallible (`verify/verdict.rs` precedent).
- **Star topology** — `conductor-core` is the dep root; seam crates import only core, never each other. The resolver abstraction's home must not force a seam→seam edge (drives the Open question).
- **std-only in core** — `redact.rs`/`obs.rs` are hand-rolled std (no `regex`); prefer native `async fn in trait` over adding an `async-trait` dep if the trait lands in core.
- **Test style** — `serializes_to_canonical_names` + `round_trips_through_json` + both-directions determinism (`testing.md` 2026-06-16 learning); drive any async under `#[tokio::test(flavor="current_thread", start_paused=true)]`; assert shape, never wall-clock duration.
- **Headless invariant** (`verification-harness.md`) — the headless path is NEVER blocked on a prompt; when non-interactive the decision is recorded to the artifact. The `HeadlessResolver` realizes this.

## New files to create (final paths pinned at P4 by the seam decision)
- `crates/conductor-core/src/pause.rs` — `HoldPoint` (serde + garde, redaction-safe prompt) + `Decision { Go, NoGo }` (closed, canonical serde) + the recorded hold-resolution value struct. **Always in core** (data model = expected.rs precedent).
- Resolver trait + `HeadlessResolver` never-block default + the await-resolve primitive — **home is the Open question** (core async-trait vs `conductor-timeline`).
- Unit tests in-module + a `tests/` integration file (`operator_pause.rs`) exercising go / no-go / headless-never-block / determinism-under-`start_paused`.

## Files to modify
- `crates/conductor-core/src/lib.rs` — `mod pause;` + re-export the new types (and trait/resolver if core-homed).
- The chosen seam's `lib.rs` (`conductor-timeline` or none) — `mod pause;` + exports, if the trait/orchestration is seam-homed.
- Possibly a `Cargo.toml` (only if a new dep is chosen — avoidable via native async-fn-in-trait + generics, or a sync trait).

## Open questions (resolve at P4 — AskUserQuestion)
1. **Resolver trait home + async shape** (the decision). (a) **core, async, generic** — `trait PauseResolver { async fn resolve(&self,&HoldPoint)->Decision }` via native async-fn-in-trait (no dep, no `dyn`), orchestration generic over `R`; core gains its first async surface but stays dep-free, star topology unconditionally safe, future-proofs the Tauri async dialog. (b) **timeline, async** — keep core strictly sync (data only); trait + `HeadlessResolver` + await live in `conductor-timeline`; risk: if any other seam ever awaits a hold it's a forbidden seam→seam edge. (c) **core, sync trait now** — `fn resolve(&self,&HoldPoint)->Decision`; smallest, zero async; the headless default + CLI `inquire` (blocking) fit, but the Tauri async go/no-go (Epoch 9) may force an async variant later. Recommendation: **(a)**.
2. **`Decision` → `ReportState` mapping** — define `Decision`/resolution here; fold a no-go/confirm into `ReportState::ManualCheck` at the **Epoch-6** report writer (out of scope now). Lean: yes, defer the mapping.
3. **Introduce the `hold.wait_resolve` tracing span now?** — only the orchestration that actually awaits would log it, and adding a bounded span name needs an obs-plan §11 amendment (a wrap concern). Lean: instrument minimally / defer the span until the hold is wired into a run (Epoch 8), unless (a) makes the primitive a natural span host.
