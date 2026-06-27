# Codebase Research — 2026-06-27-live-pulse-e2e-proof

## Scope
- **Depth:** deep (the canary/preflight area) · **Reads:** 7 files · **Globs/Greps:** 3 globs + 1 grep + 2 code-graph queries

## Files inspected
- `crates/conductor-verify/src/preflight.rs` (full) — the readiness gate. `run_preflight` (l.86) asserts version-pin + tool-presence + a canary read-back: `query_incident_list(None)` → `body.contains(&canary.marker)` → `CanaryOutcome::Ok`, else `Failed`→`Blocked`. **The doc comment (l.5-7) is the gap: "The canary's *emission* is the caller's job (the Epoch-8 cli suite runner)" — that emitter was never built.** The check is a crude substring `contains`, not content-fidelity. `preflight_boot` (l.180) is the transport-injected entry the spawn test drives.
- `crates/conductor-run/src/lib.rs` (full) — the composition root. `preflight()` (l.47) connects the client, builds `CanaryMarker::new("conductor-canary")` (l.60, **static string**), calls `run_preflight` — but **never emits a canary first**, so against live Pulse the corpus lacks the marker → `Blocked`. `readiness()` (l.74) duplicates this for the `preflight` verb. `execute_scenario()` (l.112) is explicitly **coarse**: `coarse_emit` (one signal per phase, l.182) + a coarse read-back (`query_incident_list(None)` → the literal `"incidents-listed"`, l.138-141); l.137 + l.9-10 name the faithful path as "the Epoch-10 bridge" — i.e. THIS chunk. `persist()` (l.247) writes JSONL+runs.db+Markdown. Imports **both** `conductor_emit` (l.27) and `conductor_verify` (l.33).
- `crates/conductor-verify/src/client.rs` (full) — `ReadbackClient`: `connect(data_dir)` (l.48, hardened spawn → `resolve_data_dir` rejects injection + `.env`), `query_incident_list`/`retrieve_report`/`retrieve_telemetry_slice`/`mark_incident_resolved` (l.98-127), `negotiated_protocol_version` (l.72). Pinned to `ProtocolVersion::V_2024_11_05` (l.42).
- `crates/conductor-emit/src/lib.rs` (full) — the emission surface available for the canary + faithful per-family emit: `exception_trace_request` / `error_trace_request` / `trace_request` / `latency_trace_request` / `rate_trace_request` / `service_topology_request` / `pii_trace_request` / `pii_logs_request` / `severity_logs_request`, the `fingerprint` primitive, `TraceEmitter`/`LogsEmitter`, `probe_egress`.
- `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` (full) — the canary's read-back **shape**: `query_incident_list` returns `[{"incident":"<CANARY>"}]` (l.47-48); the live marker must likewise be carried in the incident listing Pulse echoes. Test canary marker is `"conductor-canary-7f3a"` (l.20), matched by `tests/preflight_spawn.rs`.
- `crates/conductor-cli/src/commands/preflight.rs` (full) — the `preflight` verb: serialize-and-exit over `pipeline::readiness`; exit 0 iff `ready` (the go/no-go gate).
- `crates/conductor-verify/src/lib.rs` (full) — public surface (`run_preflight`/`preflight_boot`/`CanaryMarker`/`CanaryOutcome`/`ReadyState`/`evaluate_check`/`classify`/`compare`/`CheckOutcome`).

## Graph impact (from the code-graph query)
- **preflight / readiness / execute_scenario / drive_run** — callers are `conductor-cli` `commands/{run,suite,preflight}.rs` + (prior knowledge) `conductor-tauri` `start_run`. **Implication:** keeping the public signatures of `preflight()`/`readiness()`/`execute_scenario()` stable confines the canary-bridge change to their *bodies* + `run_preflight` — zero caller churn, zero cross-crate blast.
- **conductor-run crate edges** — already `→ conductor-emit` and `→ conductor-verify` (the two seams the bridge needs). **The canary bridge adds NO new crate edge** — it composes existing seams in the existing composition root. Putting the emit inside `conductor-verify` instead WOULD add a forbidden-ish `verify → emit` edge; avoid it.

## Patterns detected
- **Verdict/error wall** (`preflight.rs:163`, `:198`): `run_preflight`/`preflight_boot` return `Ok(ReadyState)` even when Blocked; only a genuine harness fault is `Err`. The canary bridge must preserve this — a failed emit/poll/fidelity ⇒ `CanaryOutcome::Failed`→Blocked, never a panic or `Err`.
- **Hardened sidecar spawn** (`client.rs:48-52`): `resolve_data_dir` (injection-reject) + `.env`; the bridge reuses `ReadbackClient::connect`, adds no spawn code.
- **Coarse-emit shape** (`lib.rs:182-206`): `TraceEmitter`/`LogsEmitter` per-phase signal — the template the faithful per-family emit replaces/extends.
- **Composition root owns both seams** (`lib.rs:27-36`): the canary emit (conductor-emit) + read-back (conductor-verify) orchestration belongs in `conductor-run::preflight`/`readiness`.
- **Redaction at the edge** (`preflight.rs:93,:120`, `redact_value`): data-dir + error strings scrubbed before they enter `ReadyState`.

## Conventions to follow
- **Marker-in-incident, echoed by `query_incident_list`** — stub shape `[{"incident":"<marker>"}]` (`stub_pulse_mcp.rs:48`); the live emit must place the marker where Pulse surfaces it in the listing.
- **`std::time` stamps** — `now_ms` / `now_rfc3339` (`lib.rs:240`, `conductor_core::now_rfc3339`), never tokio's virtual clock.
- **Bounded `tracing` span names** — `verify.readback.*` (`preflight.rs:85`), `emit.batch` (obs rule); instrument the canary round-trip + bounded poll on existing names.
- **`CanaryMarker` is the seam type** (`preflight.rs:41`) — the bridge passes a marker into it; making it unique-per-preflight is a constructor-site change in `conductor-run`, not a type change.

## New files to create
- `crates/conductor-emit/src/canary.rs` (or a `canary_trace_request(marker)` in an existing module) — a known canary-incident emission helper (an error/exception trace stamped with the unique marker), `pub use`d from `conductor-emit` — IF a dedicated helper reads cleaner than reusing `exception_trace_request`/`error_trace_request` directly. (Implement-phase call; reuse is acceptable.)
- Possibly `crates/conductor-run/src/canary.rs` — the emit→bounded-poll→fidelity orchestration, if `lib.rs` grows unwieldy (currently 417 lines).

## Files to modify
- `crates/conductor-run/src/lib.rs` — `preflight()` + `readiness()`: emit the (unique) canary incident via conductor-emit, bounded-poll until it reads back (or timeout), THEN classify ready; de-dup the two via a shared helper. `execute_scenario()`: replace `coarse_emit` + the `"incidents-listed"` coarse read-back with scenario-faithful emission + per-check observed extraction (scope-dependent — see Open questions).
- `crates/conductor-verify/src/preflight.rs` — upgrade the canary check from `body.contains` to content-fidelity (parse the listed incident, assert it is the emitted one) + a bounded ingest retry; correct the now-stale "emission is the caller's job (unbuilt)" doc comment.
- `scenarios/{error-baseline-spike,fingerprint-storm,restart-suppression,pii-scrub,receiver-lifecycle-state,receiver-failed-port-conflict}.toml` — refine `expected`/`slo_tier` to the live Pulse's observed tokens where the authored values were inferred (the "Epoch-8/10 calibration point" markers in testing.md). Connection-lifecycle = `receiver-lifecycle-state.toml` + `receiver-failed-port-conflict.toml` (P-001..P-004).
- `crates/conductor-verify/tests/preflight.rs` (+ `preflight_spawn.rs` / `stub_pulse_mcp.rs`) — extend the stub + fixtures for the content-fidelity + emit-then-read shape.

## Open questions
1. **Scope of the 5-family faithful verification** — (a) canary bridge + ALL 5 families faithfully emit+extract, (b) canary bridge + the faithful pattern proven on 1–2 families with the rest a sibling chunk, or (c) canary-bridge-only (flip `ready`) with all 5 families' faithful path next. → P4 AskUserQuestion (the user framed the canary bridge as "this chunk's core work").
2. **Canary marker uniqueness** — static `"conductor-canary"` risks a stale-corpus false-positive against a persistent live corpus; make it unique-per-preflight (run_id/seed/timestamp, per the security extract). Recommend yes; resolve in plan.
3. **Which Pulse incident field carries + echoes the marker** (service.name vs exception type/message vs incident title) — confirm against the live Pulse at implement time (the user has run it live).
