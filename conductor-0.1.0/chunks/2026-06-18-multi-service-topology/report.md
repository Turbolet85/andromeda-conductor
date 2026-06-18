# Report — 2026-06-18-multi-service-topology

**Chunk:** Multi-service topology — multiple `service.name` ResourceSpans + shared `trace_id` W3C cross-service propagation/parent-linkage (conductor-emit, P-008/P-027)
**Date:** 2026-06-18
**Commits:** (uncommitted; chunk commit authored in P7) — prior `HEAD` is `ecdba99` code-graph chore (#15)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-emit/src/topology.rs` (builder + validated type + 11 unit tests)
  - NEW `crates/conductor-emit/tests/multi_service_topology.rs` (2 loopback-stub integration tests)
  - MOD `crates/conductor-emit/src/lib.rs` (`mod topology;` + re-export + doc-list line)
  - (wrap/promotion artifacts, non-code: `master-route.md`, `working-route.md`, `chunks/2026-06-18-multi-service-topology/`, `session-handoff.md`)
- **Symbols / APIs:**
  - NEW pub `ServiceTopology` (struct; `::new(&[&str]) -> Option<Self>` self-validating ≥2 distinct services; `.services() -> &[String]`) — re-exported at crate root.
  - NEW pub fn `service_topology_request(&ServiceTopology, seed: u64, Option<ErrorPlacement>, message: &str) -> ExportTraceServiceRequest` — re-exported at crate root.
  - Reuses existing pub `ErrorPlacement` (unchanged) + `pub(crate)` `service_resource`/`gen_id`/`span`/`ok_status`/`error_status` (unchanged).
  - No new IPC method · no endpoint · no port/socket · no env var · no workspace crate.
- **Crates / modules:** `conductor-emit` gains internal `mod topology`. No crate added/removed/renamed.
- **Dependencies:** NONE added · NONE bumped (no `Cargo.toml` / `Cargo.lock` change).
- **Schema / config:** none — no scenario-config/garde wiring (deferred to the scenario epoch, mirroring latency/severity); no `runs.db` schema; no violation schema.
- **Coverage of new surfaces:**
  - `ServiceTopology` / `service_topology_request` (OTLP trace-emission primitive — an emit-side builder, NOT an external-input boundary) → validation {garde n/a — self-validates in-crate via `ServiceTopology::new -> Option`, mirroring `LatencyProfile`/`Severity`; `conductor-core` garde is the later scenario-wiring validator} · instrumentation {existing `emit.batch` span on `TraceEmitter::export` covers the multi-`ResourceSpans` request; NO new self-obs span — bounded span-name set unchanged ✓} · PII {n/a — caller-supplied service names + fixed `root`/`child-N` span names; no host paths / struct names redacted✓} · tests {unit 11 + integ 2 ✓} · a11y {n/a — renders nothing} · tokens {n/a — renders nothing}

## Deviations from intent
- **`ServiceTopology` owns `Vec<String>` (Clone, not `Copy` / not a borrow).** Plan implied a thin wrapper + listed a `len()` accessor; an owning type avoids a stored-borrow temporary-lifetime trap (a `&[&str]` stored in a `let` would drop-while-borrowed). Faithful to the validated-wrapper intent; mirrors `LatencyProfile`-owns-its-data (`Copy` there only because u64s are cheap; `String` isn't, so `Clone`).
- **No separate `len()` accessor** (plan listed one); exposed only `services()` (`services().len()` covers it) — a public `len()` without `is_empty()` trips clippy `len_without_is_empty` under `-D warnings`.
- **Error depth read by matching `ErrorPlacement`'s public variants, not `ErrorPlacement::depth()`** — `depth()` is private to the `span_tree` module and `span_tree.rs` is OUT of this chunk's Files-to-modify scope (lib.rs only). Scope boundary held; no cross-module edit.
- **Out-of-range `DeepChild{depth}` saturates to the deepest service** (`depth.min(len-1)`) — the recommended resolution from research §Open questions / plan step 4; implemented + tested.
- **lib.rs doc line updated** to list `service_topology_request` — doc-accuracy nicety on the re-export hub, not a structural change.
- All deviations are minor in-scope HOW refinements; none alter the chunk's WHAT or any acceptance criterion.

## Decisions & corrections
- **One-span-per-service linear chain** is the intentional minimal model realizing P-027 (≥2 distinct `service.name`) + P-008 (cross-service root-vs-deep error) + W3C propagation (shared `trace_id` + cross-service `parent_span_id`). Richer topologies (fan-out, multi-span services) are out of scope, deferred.
- **`gen_id` draw order is part of the determinism contract** (trace_id once, then per-service span_id in chain order) — locked by the shape golden; reordering draws would change the seeded bytes.
- **Distinct builder, not an extension** of `error_trace_request` (its chain length is driven by error depth; topology's is the #services) — left the single-service P-005/P-008 path untouched.
- **Deferred (tracked, not this chunk):** (a) `opentelemetry-proto` `default-features = false` to drop the dormant transitive `opentelemetry_sdk` — still open; no `Cargo.toml` change this chunk, so again not the moment. (b) scenario-config garde wiring for topology — scenario epoch.

## Outcome
- **All 11 plan acceptance criteria met:** ≥2 distinct `service.name` ResourceSpans · shared `trace_id` + cross-service parent linkage · Root/DeepChild/None error placement · both-direction seed determinism (shape projection excludes wall-clock) · refused transport ⇒ `EmitError::Transport` · no host-path/struct-name leak · no new self-obs span.
- **Gates green (first run, 0 fix iterations):** `cargo nextest run -p conductor-emit` 47/47 (+13) · `cargo nextest run --workspace --profile ci` 128/128 (+13) · `cargo test -p conductor-emit --doc` 0 (none in emit) · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo llvm-cov nextest -p conductor-emit --fail-under-lines 60` → topology.rs **99.50%** lines, total 96.69%.
- **Supply chain:** no `Cargo.toml`/`Cargo.lock` change ⇒ `cargo audit`/`cargo deny` unaffected (green by construction; full gate is P7's).
- **Smoke:** skipped — no boot-path change (pure `conductor-emit` library primitive; no binary/entry-point; CLI is Epoch 8). Bounded `bash scripts/agent-run.sh status` confirmed the harness reads cleanly (usage + exit 0).
