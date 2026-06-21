# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/andromeda-setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-06-21 — Mapping a typed envelope onto rusqlite (runs.db): u64 bit-cast · serde-wire-form enums · parse OUTSIDE the row closure

Persisting a `conductor-core::RunRecord` as a `runs.db` row (the new `conductor-report::RunsDb` seam) surfaced three rusqlite mapping gotchas worth reusing for the Epoch-7 P-036 recurrence query + the Epoch-8 `status` read. (a) **`u64` overflows the SQLite signed-`INTEGER` (i64) column** — rusqlite's `ToSql for u64` *errors* when the value exceeds `i64::MAX`, so a `seed: u64` must be stored via an `as i64` bit-cast on write and read back `as u64` (lossless; tested against `u64::MAX`), never bound directly. (b) **Store serde enums as their wire string, not a hand `match`** — `Verdict`/`ReportState`/`SloTier` go into TEXT columns via `serde_json::to_value(v)?.as_str()` and come back via `serde_json::from_value(Value::String(s))`, so the stored spelling stays identical to the JSONL `#[serde(rename)]` (`<5s`, `Pass`, …) with the rename as the single source of truth — a parallel match would silently drift if a variant is renamed.

(c) **You cannot parse serde/JSON inside the `query_row` mapping closure** — that closure must return `rusqlite::Error`, but `serde_json::from_str`/`from_value` (the JSON1 array columns `p_ids`/`fingerprints`, plus the enum wire forms) produce your own error type. Read the raw column values into a private `RawRow` struct inside the closure (all `row.get(i)?`), then convert `RawRow -> RunRecord` OUTSIDE the closure where your `Result<_, RunsDbError>` + `?` compose. The `Blocked`-row NULL rule then falls out for free: the envelope's five measurement fields are already `Option`, so `None` binds to SQL `NULL` and a NULL column reads back as `None` — no special-casing.

The `RunsDb` type mirrors its sibling `JournalWriter`: `open(runs_dir: &Path)` takes the already-resolved dir (the cli edge owns `CONDUCTOR_RUNS_DIR`; the seam never reads env), `CREATE TABLE IF NOT EXISTS` bootstrap (no migration framework), bound parameters only, `#[non_exhaustive]` thiserror `RunsDbError` (`Io`/`Sqlite`/`Json`). Note: rusqlite 0.38.0's first real `bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite **3.50.4** (not the 0.38.0/3.51.1 the specs had assumed — arch §Stack + stack.md corrected this wrap); JSON1 (`json_array_length`) is present and the `≥3.38` floor holds.

---

## 2026-06-21 — clippy `too_many_arguments` counts `&self`: a 7-arg method still trips `8/7`

Under the `cargo clippy --all-targets -- -D warnings` gate, `clippy::too_many_arguments` counts the `self`/`&self` receiver toward its 7-argument threshold — a method with seven non-self parameters fires `8/7` and fails the gate, exactly like an eight-parameter free function. So a faithful many-field constructor needs `#[allow(clippy::too_many_arguments)]` whether it is an associated fn (`RunRecord::measured`, eleven params) OR a method (`CheckOutcome::to_run_record`, `&self` + seven). Do not assume the receiver is exempt. Pair the allow with a one-line justification (here: the run-report envelope is eleven fields by contract — arch §Standard Contracts). (run-report-envelope-serializer chunk, conductor-core + conductor-verify)

---

## 2026-06-21 — A public async trait under the `-D warnings` clippy gate: declare RPITIT, not `async fn`

A public trait method written as `async fn` trips the `async_fn_in_trait` lint (callers can't add a `Send`/lifetime bound on the returned future), which fails the `cargo clippy --all-targets -- -D warnings` gate. Declare it with return-position `impl Trait` instead — `fn resolve(&self, hold: &HoldPoint) -> impl Future<Output = Decision>;` — while implementors may still write `async fn` (an `async fn` in an impl satisfies an `-> impl Future` trait method, stable since Rust 1.75). Keep the orchestration **generic** over the trait (`fn resolve_hold<R: PauseResolver>(…)`) rather than `dyn Trait`: this sidesteps the dyn-incompatibility of RPITIT async methods AND avoids pulling in the `async-trait` crate, so a new async abstraction can land in an otherwise sync/dependency-light crate (here `conductor-core`'s `PauseResolver` — the crate's first async surface) with **zero new runtime dependencies** (only a `tokio` dev-dep for the test executor). (operator-pause-orchestration chunk, conductor-core)

---

## 2026-06-20 — conductor-faults: an *infallible* constructor when a helper has NO failure mode — the third branch of the fault-constructor idiom

The 2026-06-19 entry below split faults-helper construction two ways: a *named domain constraint* → `Result<Self, FaultError>`; an *anonymous pure-value bound* → `Option`. **P-014 `AbruptSilence` is the third branch: no failure mode at all → an infallible `new() -> Self`, with `FaultError` left untouched** — the crate's first infallible helper. Unlike `EmissionGap` (a 20s floor to validate) or `PortOccupier` (a socket to acquire), a *permanent* emission stop has no bound and no resource, so there is nothing to fail on; permanence is modeled as the structural *absence* of a `Duration`, and a positive `resumes() -> bool { false }` accessor makes the no-resume contract testable against `EmissionGap`.

Heuristic completing the prior entry: pick constructor fallibility by whether a *real* failure mode exists — `Result` + a named `FaultError` variant for a threshold/resource failure, `Option` for an anonymous shape bound, and an **infallible `Self`** when there is neither. Do NOT force a `Result`/`Option` "for symmetry" with the siblings — a constructor that is structurally always-`Ok` is the computed-but-never-applied anti-pattern (the same reasoning that kept the seed off the exact-gap helper). User-ratified via /andromeda-phase AskUserQuestion (Helper shape → "Infallible marker").

---

## 2026-06-19 — conductor-faults helpers fail a *named domain constraint* with `Result<_, FaultError>`; emit's pure-value bounds use `Option`

The two seam crates split their constructor-validation idiom by the KIND of invalid input. A **`conductor-faults`** helper whose construction can violate a *named domain constraint* returns `Result<Self, FaultError>`, extending the `#[non_exhaustive] FaultError` enum with a descriptive variant: `PortOccupier::occupy` → `Bind` (a refused loopback bind), and now `EmissionGap::new` → `GapTooShort`/`GapTooLong` (a gap at/below the 20s P-015 restart threshold, or above the 1h ceiling). A **`conductor-emit`** generator validating a *pure-value bound* uses an `Option`-returning constructor instead — `RateCurve::ramp`/`breathing` (`windows>0`, `amplitude<center`), `Severity::new`, `LatencyProfile::new` (the sibling 2026-06-18 emit entry below).

Heuristic for the upcoming faults chunks (P-014 abrupt-silence, P-013 bursty-train): reach for a typed `FaultError` variant when the failure has a *named cause worth surfacing* (a threshold breached, a resource denied) — the name aids the operator and the verdict/error wall; reserve `Option` for anonymous "these numbers don't form a valid shape" bounds. Both honor the rule that invalid input is a value, never a panic. Note the gap is **seed-independent** — an exact `Duration`, deterministic by construction (no seed param to thread), the same seed-only-governs-what-it-drives principle as the 2026-06-18 fingerprint entry in `.claude/rules/testing.md`.

---

## 2026-06-18 — Emission primitives self-validate their typed input in-crate (constructor); core garde is the *later* scenario-wiring validator

Each `conductor-emit` primitive owns and validates its typed input **inside the emit crate** via an `Option`-returning constructor that enforces the invariant — NOT by deferring to `conductor-core`'s garde layer. `Severity::new` (rejects outside `1..=24`, severity-logs) and now `LatencyProfile::new` (rejects unless `p50 ≤ p95 ≤ p99`, latency-shaping) are the pattern. This follows the emission-seam comment in `phase_spec`: the concrete OTLP taxonomy (severity boundaries, fingerprint identity, latency targets, ramps) lands in the Epoch-3 emission seam, not the scenario model.

Consequence for the scenario-wiring epoch: `conductor-core`'s garde is the authoritative validator only *later*, when these targets wire into scenario config by extending the `#[non_exhaustive] EmissionSpec`. The `scenario.rs` note that the p50≤p95≤p99 invariant "joins when the Epoch-3 latency spec lands" refers to that future garde wiring — distinct from the primitive's own constructor check, which is this chunk. So a new emission primitive (topology, PII, ramps) defaults to an emit-local, constructor-validated typed input; don't reach into core's scenario garde for it this early. Complements the placement heuristic in the sibling 2026-06-18 entry (primitive → producing seam crate) with the validation-location dimension.

---

## 2026-06-18 — Emission/compute *primitives* live in their producing seam crate; the *fault* that composes them lives in conductor-faults

The build route places a low-level emission/compute **primitive** in the crate that produces its raw material, even when the module-map one-liner nominally attributes the broader concern to another crate. The per-exception **fingerprint primitive** (`fingerprint()` + the exception-event builder) landed in `conductor-emit` — co-located with the exception content it derives from — NOT in `conductor-faults`, despite arch / CLAUDE.md §Modules listing "fingerprint generation" under faults. That attribution is now narrowed: `conductor-faults` owns the higher-level **fingerprint-storm FAULT** (Epoch-7), which will depend on `conductor-emit` and *compose* this primitive. This recurs from error-spans (its multi-span builder also landed in emit, not faults).

Heuristic for future phase/placement calls: a PRIMITIVE goes in its producing seam crate (`conductor-emit` owns OTLP-message construction + anything derived directly from it, like the content fingerprint); a FAULT that orchestrates/composes primitives goes in `conductor-faults`, built later (Epoch-4+). When the module-map blurb seems to conflict, prefer co-location with the data + the dependency direction (faults → emit), then reconcile the doc (arch §Modules amended this chunk). User-ratified via /andromeda-phase AskUserQuestion.

---

## 2026-06-17 — OTLP emission scaffolding (conductor-emit) notes

The Epoch-3 emission seam builds raw OTLP messages from `opentelemetry-proto` 0.32.0 directly (not the SDK exporter). Two facts for the upcoming emission chunks (error-spans, exception-events, severity-logs, latency, topology, PII, ramps):

- **No `build.rs` / `tonic-prost-build`.** opentelemetry-proto's `gen-tonic` feature ships the generated `TraceServiceClient` (+ server stub) and the message structs (`ResourceSpans` / `Span` / `Status` / …); there is no local `.proto` to compile, so the workspace needs no build script.
- **Build proto structs with `..Default::default()`.** opentelemetry-proto 0.32.0's `KeyValue` carries a third field (`key_strindex`, a newer OTLP string-table index), and other messages gain fields across proto versions. Set only the fields you control and spread `..Default::default()` for the rest — an exhaustive struct literal breaks when a proto-version bump adds a field.

---

## 2026-06-15 — `--profile ci` not defined until the test-framework chunk (regression-gate workaround)

The documented test command `cargo nextest run --workspace --profile ci` (CLAUDE.md §Workflow · `.claude/docs/commands.md` · `.claude/rules/verification-harness.md`) **fails** with `error: profile 'ci' not found (known profiles: default, default-miri)` — the `.config/nextest.toml` that defines the `ci` profile is created by the later Epoch-1 chunk "Test framework + fixtures + coverage tooling" and does not exist yet. Until that chunk lands, every implement chunk's regression gate hits this.

Workaround: run `cargo nextest run --workspace` (the default profile runs the identical test set — only the run-config differs: retries / JUnit / output — and tests are profile-independent). Do NOT create `.config/nextest.toml` ad hoc in an unrelated chunk; that profile is the test-framework chunk's deliverable. Same Foundation-sequencing class as the `playbook.md` rule about interim `cargo test`.

---

## 2026-06-15 — cargo-deny over an unpublished workspace needs `publish = false`

`cargo deny check` treats every workspace member as a *publishable* crate unless it is marked `publish = false`. For the `conductor-*` crates (no `license` field, internal `path` deps) that produced two error classes at once: `error[unlicensed]` (a public crate must declare a license) and `error[wildcard]` ("allow-wildcard-paths is enabled, but does not apply to public crates as crates.io disallows path dependencies"). Both vanish once the crates are `publish = false` — then `[licenses].private.ignore = true` skips their license check and `[bans].allow-wildcard-paths = true` covers their internal `path` deps.

The fix is `publish = false` in `[workspace.package]` + `publish.workspace = true` per crate (matching the existing version/edition/rust-version inheritance). Correct for a local-only, no-cloud tool that never publishes to crates.io — and it strengthens supply-chain posture rather than weakening the gate. Don't reach for `wildcards = "allow"` or dropping the license check to dodge it.

---

## 2026-06-15 — garde 0.22.1 API gotchas (config validation)

Conductor pins **garde 0.22.1**, not the arch's original 0.23.0: `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable here. When wiring garde into a seam crate:
- `derive` is **not** a default feature — the edge must be `garde = { workspace = true, features = ["derive"] }`, or `#[derive(Validate)]` / the `#[garde(...)]` helper attribute won't resolve ("cannot find derive macro `Validate`").
- `Validate::validate(&self)` takes **no** context argument (returns `Result<(), garde::Report>`); call `.validate()`, not `.validate(&())`, for the default `()` context. The error type is `garde::Report`, bridged into `CoreError` via `#[from]`.
- `#[garde(custom(fn))]` is **field-level only** — there is no container/struct-level `custom` in 0.22.1 (it errors "unrecognized attribute"). A whole-list invariant rides on the one field it concerns (e.g. no-duplicate-P-IDs on `p_ids`); invariants spanning *distinct* fields (p50≤p95≤p99, severity-mix sums — the Epoch-2 emission spec) need garde's `Context` pattern or a manual `Validate` impl.

Applies to every future garde validation surface (Epoch-2 `Scenario-config model` especially). See arch §Established Decisions [Validation Library] for the pinned-version decision.

---

---

## Entry format

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` (critical, short).
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` (loaded when matching files touched).
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read).
