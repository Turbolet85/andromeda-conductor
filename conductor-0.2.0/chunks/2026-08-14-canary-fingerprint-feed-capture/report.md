# Report — 2026-08-14-canary-fingerprint-feed-capture

**Chunk:** Canary fingerprint-feed capture — a live capture settling whether the six exception spans reach Pulse's appender with their events intact, the one open Conductor-side piece of the live-path fix-scope (conductor-emit/run)
**Date:** 2026-08-14
**Commits:** none since `last_wrap` (`cb9e285 feat(2026-08-13-first-live-green-preflight)` was the last)

## Changes (structured — detectors read this)

- **Files:** counts from `git status` — **7 modified · 3 new** (plus 2 process run-dirs).
  - modified code: `crates/conductor-emit/src/client.rs` (+39/−0) · `crates/conductor-run/src/lib.rs` (+32/−11)
  - new code: `crates/conductor-run/tests/canary_wire.rs`
  - new chunk artifacts: `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/{scope,research,plan,fingerprint-feed-verdict}.md`
  - route/ledger/bookkeeping: `.andromeda/master-route.md` (+1/−0, the promotion append) · `conductor-0.2.0/working-route.md` (+1/−1, the freeze stamp) · `conductor-0.2.0/verification-matrix.json` (+2/−1, a `notes` line on `v2-11`) · `.claude/session-handoff.md` · `.andromeda/friction-log.ndjson`
- **Symbols / APIs:**
  - `conductor_run::CANARY_STORM_COUNT` — visibility widened private const → **`pub const`** (value unchanged at 6)
  - `conductor_run::canary_spec(&str) -> ExceptionSpec` — **new pub fn** (the canary's synthetic exception, extracted verbatim from `emit_canary`)
  - `conductor_run::emit_canary_storm(&mut TraceEmitter, &ExceptionSpec, u64) -> anyhow::Result<()>` — **new pub fn**; the storm's emission loop with the transport injected instead of fixed at `DEFAULT_OTLP_ENDPOINT`
  - `conductor_emit::client::wire_shape(&ExportTraceServiceRequest) -> String` — **new private fn** beside the existing `count_spans`
  - `emit_canary` keeps identical observable behavior (same occurrence count, same `base.wrapping_add(i)` seeding, same fingerprint)
  - **No new ports, sockets, IPC methods, endpoints, or `CONDUCTOR_*` env vars.**
- **Crates / modules:** none added, removed, or re-scoped.
- **Dependencies:** **none added, none bumped.** `Cargo.lock` un-drifted — **0 lines**, 0 new `[[package]]`. Both touched crates' `Cargo.toml` unchanged (the new test's dev-deps — `opentelemetry-proto`, `tonic`, `tokio-stream`, `assert_fs` — were already present for `dispatch_wire.rs`).
- **Schema / config:** none. The witness rides the **already-allowlisted `message`** field — no new `conductor-core::redact` allowlist entry, no new self-obs field, no envelope change.
- **Spec-master edits:** none (implement is read-only on the seven masters).
- **Counts / qualifiers moved:** workspace test count **577 → 581** (+4). Stated in no spec master — its only occurrence is `.claude/session-handoff.md`, which P6 rewrites. `CANARY_STORM_COUNT` value unchanged (6) and named in no doc.
- **Dev-tool versions:** none. `cargo-audit` remains 0.22.2 (the latest published) — re-run, not upgraded.
- **Reverted / negative API facts:** `conductor_run::canary_storm_requests(&ExceptionSpec, u64) -> Vec<ExportTraceServiceRequest>` was written and then **removed before it shipped**. It cannot compile from `conductor-run`: `opentelemetry-proto` is a dev-dependency only there, and `conductor-emit` re-exports no proto request type, so the return type is unnameable. Naming it would have required either a new dependency edge (drifting the lockfile the audit deferral's basis rests on) or a re-export in `conductor-emit/src/lib.rs`, outside the chunk's modify set. Replaced by transport injection (`emit_canary_storm`), which also proves more — the test drives the real emission loop rather than a builder.
- **Coverage of new surfaces:**
  - `emit.batch` wire-shape witness (new self-obs line on the existing bounded span) → validation n/a · instrumentation **✓** (`tracing::debug!` inside the existing `#[instrument]` span; `debug` per obs-plan §11's hot-path ban on `info`) · PII **redacted✓** (event and attribute **key names only**, never values; processor-stage redaction unchanged) · tests **✓** (integration — read back out of a real self-obs artifact, not a double) · a11y n/a · tokens n/a
  - `conductor_run::{canary_spec, emit_canary_storm}` (new pub fns) → validation n/a (no external input; the marker is Conductor-generated) · instrumentation **✓** (inherits `emit.batch` per export) · PII n/a · tests **✓** (4 wire-tier integration tests against an ephemeral `127.0.0.1:0` stub) · a11y n/a · tokens n/a
  - Live-leg witness capture against a running Pulse → tests **unrunnable-here** (no live `pulse-app`; MCP sidecar not on PATH). Measured, not assumed — see Outcome.

## Deviations from intent

1. **Plan step 3's seam shape replaced.** The plan prescribed a function *returning* the storm's requests; that is not compilable from `conductor-run` (see Reverted API facts). Injected the transport instead. Justification: meets the step's stated purpose (assertable at an ephemeral stub without the fixed endpoint), stays inside the chunk's modify set, avoids a lockfile-drifting dependency edge, and yields a stronger proof.
2. **Plan step 7 recipe item 6 is wrong as written; corrected in the deliverables.** `RUST_LOG=conductor_emit=debug` — a bare target directive — filters **every other target out**, silencing the rest of the self-obs stream. Measured across three forms: absent → the CLI's `agent_mode_routes_self_obs_to_the_log_file_not_stderr` passes; bare → **fails**; `info,conductor_emit=debug` → passes. The working form is now in the test and in `fingerprint-feed-verdict.md` §5. `plan.md` is read-only to implement, so the wrong form still stands there.
3. **A test was added beyond the plan's list** (inside the plan-created `canary_wire.rs`): the witness is read back out of a real self-obs artifact. Justification: the plan's own implementation note requires exactly this ("the witness must not become the second silent zero… verify the line appears in the real artifact, not in a test double"); without it the witness would ship unverified.
4. **Plan step 6 was first reported blocked, then completed.** The per-arm Pulse logs were reported unavailable after a repo-only search. On operator correction they were found on disk (a date-suffixed rotation in the Pulse data dir) together with the prior session's recorded per-arm byte offsets and pre-sliced windows. The transcription then completed in one pass and produced the chunk's strongest evidence.

## Decisions & corrections

- **Operator correction (phase P5) — a premise the plan had inverted.** The windowed-gauge finding fixes an *expectation* (a working six-occurrence identical-fingerprint storm reads **1**, never 6) but does **not** soften the observed zero: `two-launch-verdict.md:187-189` already recorded `fingerprints_evicted_total: 0` across every sample — the latched signal — and my draft had drawn the opposite inference without checking it. Corrected in `scope.md`, `research.md` and `plan.md` before approval.
- **Operator directive (implement) — step 6 is not blocked.** Pulse's log rotates by date suffix and the arms' file is frozen on disk; the probe harness recorded each arm's byte offset. Verified byte-identical (md5) to the frozen log read at those offsets before transcribing.
- **Correction found by running the smoke:** the bare-target `RUST_LOG` form silences every other target (deviation 2). The lesson generalizes past this chunk: a bare `target=level` directive is not additive.
- **Correction found by reading SUT source rather than inferring from shape:** the ingest heartbeat's `span_count` is a **cumulative** counter (`fetch_add` on receipt), not a per-tick delta. That is what makes `1 → 2 → 3 → 9 → 9 …` mean *all nine spans arrived and were counted*, from the very same log lines that show the fingerprint table empty.
- **Diagnostic lesson that closed the fork:** a **windowed gauge cannot witness an event's absence** — discriminate with the **cumulative** counters beside it, and verify a counter's semantics in source before inferring from its shape.
- **Sharpened operator recipe item 1:** an unreachable sidecar means the canary **never emits at all**, so a missing witness line is evidence the leg never ran, not evidence about the wire.
- **`v2-11` partial-advance provenance stands** — pooled, `chunk: null`; this chunk removes the fingerprint-feed blocker without satisfying that acceptance (which needs an incident).

## Outcome

**Acceptance criteria** — met, with one explicitly unproven and recorded:

- tests / clippy ✓ · wire-tier assertions on server-captured requests ✓ · obs key-names-only + bounded span set ✓ · security artifact hygiene ✓ (grep-verified: no host paths, no internal struct names, no corpus content) · arch no-new-bind / no-new-env-var ✓ · design+layouts n/a (no cli surface added) · a11y ✓ (no claim attached, no CI step added).
- **Unproven, not silently passed:** the criterion naming `logs/agent-latest.jsonl`. Measured why: a scenario run produced a **fresh** 16-line self-obs artifact with **0** `emit.batch` spans, stopping at `preflight blocked: MCP read-back path unreachable` — earlier than the canary emission. No stale residue was read.

**Gates (commands run):** `cargo nextest run -p conductor-emit -p conductor-run` · `cargo nextest run --workspace --profile ci` → **581/581, zero retries** · `cargo clippy --workspace --all-targets -- -D warnings` → clean · `cargo test --workspace --doc` → 3 passed across 7 suites · `cargo deny check` → advisories/bans/licenses/sources **ok, exit 0**.

**`cargo audit` — SIXTEENTH consecutive red**, re-pinned silently under the operator's L5 ratification (origin `2026-08-08-sut-capability-manifest`). Byte-identical `error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1, on 0.22.2 — the latest published, so an advisory-**DATABASE** fault with nothing to raise a floor to. Basis **re-verified literally, not echoed**: `Cargo.lock` drift 0 lines, 0 new `[[package]]`, `cargo deny` true exit 0 as the overlapping signal. No floor raise, no `deny.toml` ignore, no CI edit.

**Smoke (boot-path changed):** `bash scripts/agent-run.sh run` → exit 0. The `SCENARIO=`-gated leg additionally fired, producing `[BLOCKED] fingerprint-storm` at exit 0 (Blocked is not a hard Fail) with a fresh artifact.

**The chunk's question is settled and handed off.** Conductor's storm leaves the process with its exception events intact — asserted on spans a collector actually received, the canary path's first wire-tier test. Pulse received all nine spans and counted them, stably, in every arm; its fingerprint table stayed empty across **twelve in-window samples** spanning three arms, with both cumulative counters at zero across **31 tick lines**. Spans arrive; nothing reaches the fingerprint observer. The gap lies inside Pulse, between OTLP ingest receipt and the per-span-event fingerprint hook — a **region, not a named defect**, since Conductor cannot see into that path. Full evidence: `fingerprint-feed-verdict.md`.
