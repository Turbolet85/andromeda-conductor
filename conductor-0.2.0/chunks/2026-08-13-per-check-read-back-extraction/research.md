# Codebase Research — 2026-08-13-per-check-read-back-extraction

## Scope
- **Depth:** deep · **Reads:** 7 · **Globs/Greps:** 13 · **Code-graph queries:** 7 (all non-empty except one, which is itself the finding)

## Files inspected
- `crates/conductor-run/src/lib.rs` (255-400) — `execute_scenario`: the `scenario.run` root span, the OTLP egress probe, the dispatcher drive, **the placeholder read-back at `:301-305`**, the empty-`expected` → `manual_record` branch at `:309-326`, and the per-check fold at `:328-342` that maps every check over ONE `&observed` and keeps the max-severity outcome.
- `crates/conductor-verify/src/slo.rs` (1-130) — `compare` (the four `ComparisonKind` arms), `evaluate_slo`, `evaluate_check`. Infallible, `&str`-in. `CountAtLeast` parses `observed.trim()` as `i64`; an unmet floor is softened to `CalibrationRegion` regardless of declared class.
- `crates/conductor-verify/src/preflight.rs` (290-379) — **`assert_canary` is the existing extraction precedent**: it calls `query_incident_list`, pulls ids via `incident_ids()`, then `retrieve_telemetry_slice` per id and matches `fingerprint_refs()`. Both helpers are private field-tolerant `Value` readers.
- `crates/conductor-core/src/expected.rs` (35-64) — `ComparisonKind {Exact, Contains, Absent, CountAtLeast}` and `ExpectedCheck {kind, class, expected}` — **three fields, no tool or field selector**.
- `crates/conductor-verify/tests/common/mod.rs` (11-110) — `StubConfig` (6 knobs) and the raw result shapes the stub emits.
- `scenarios/degraded-mode-report.toml` (full) — the P-053 scenario, empty `expected`, whose header names this exact routing as "the Epoch-8 evaluator's job".
- `D:\dev\projects\andromeda-pulse\crates\mcp-server\src\tools.rs` (336-437) — **the live SUT's real dispatch shapes** (premise verification, see below).

## Graph impact (trace: `.andromeda/runs/2026-08-13T19-14-19-phase/tree-query-2026-08-13-per-check-read-back-extraction.json`)
- **`execute_scenario`** — 7 rows: 3 production callers (`conductor-cli::commands::run` @ `commands/run.rs:23`, `conductor-cli::commands::suite` @ `commands/suite.rs:30`, `conductor-run::drive_run` @ `lib.rs:477`) + 4 in-crate test sites. This is exactly why the matrix pins signature preservation: a signature change threads three production call sites across two crates.
- **`evaluate_check`** — 17 rows, but **exactly ONE production caller**: `execute_scenario()` @ `conductor-run/src/lib.rs:330`. Everything else is `crate/` import lines and tests (`slo.rs`, `record.rs`, `tests/expected_slo.rs`). The comparison layer has a single consumer, so feeding it per-check values is a local change.
- **`to_run_record`** — 2 rows: `execute_scenario()` @ `lib.rs:333` + one test. Same single-consumer shape.
- **`query_incident_list`** — 3 rows: `execute_scenario()` @ `lib.rs:301` (the placeholder), `preflight::assert_canary()` @ `preflight.rs:324`, one test.
- **`retrieve_telemetry_slice`** — 1 row: `preflight::assert_canary()` @ `preflight.rs:333`. The extraction path would be its **second** consumer.
- **`retrieve_report`** — **0 rows.** Consulted-and-no-match is the finding: the tool is pinned in `contracts/mcp-contract.toml`, asserted present by the preflight gate, and **never called anywhere in the workspace**. The `degraded_mode` surface is entirely unbuilt.
- **`crate_edges` for `conductor-verify`** — 3 rows: `conductor-verify → conductor-core`; `conductor-run → conductor-verify`; `conductor-cli → conductor-verify`. Adding to `conductor-verify` is additive with **two** inbound consumers (cli as well as run), and needs no new edge.

## Premise verification against the live SUT (`D:\dev\projects\andromeda-pulse`)
The scope flagged "where `degraded_mode` is observable" as a premise to verify rather than plan on. Verified first-hand in Pulse's `crates/mcp-server/src/tools.rs`:

| tool | arguments | result shape |
|---|---|---|
| `query_incident_list` (`:336`) | **none** | `{items: [{incident_id, status, severity, title, opened_at_unix_nano}], total, next_cursor}` |
| `retrieve_report` (`:360`) | `{incident_id}` | `{markdown, degraded_mode: bool}` |
| `retrieve_telemetry_slice` (`:416`) | `{incident_id}` | `{incident_id, span_refs[], fingerprint_refs[], timestamps_unix_nano[]}` |
| `mark_incident_resolved` (`:439`) | `{incident_id}` | mutating — not a read-back source |

**Two premises came back different from the spec's phrasing:**

1. **`degraded_mode` is a RESPONSE FIELD, not a request argument.** `architecture.md` §Occupied Resources writes "`retrieve_report` (with `degraded_mode`)" and `scenarios/degraded-mode-report.toml:17` writes "`retrieve_report(degraded_mode=true)`" — both read as an argument Conductor passes. Pulse's `dispatch_retrieve_report` parses `IncidentIdArgs` only (`{incident_id}`) and **computes** `let degraded_mode = parsed_l4.is_none();` (`tools.rs:372`), returning it in the result (`:380`). So Conductor cannot *request* degraded mode; it can only *observe* that a report came back degraded. The routing is therefore read-side, and no argument plumbing exists to build.
2. **`query_incident_list` takes no arguments at all** — consistent with the workspace-key precondition string's existing claim, and it means the item fields (`status`, `severity`, `title`) are the only per-incident signal available without a second call.

Field-key note: Pulse emits `incident_id` as the item key; Conductor's `incident_ids()` (`preflight.rs:364`) already tries `id` then `incident_id`, so it is tolerant of both — the stub uses one, the SUT the other.

## Patterns detected
- **Field-tolerant `Value` reader helpers** (`preflight.rs:357` `incident_ids`, `:372` `fingerprint_refs`): `.get(key).and_then(as_array).map(…).unwrap_or_default()` — never `unwrap`, an absent/malformed shape degrades to empty rather than erroring. This is the shipped shape for reading a raw Pulse payload and the obvious template to generalize.
- **Two-step id-then-detail read-back** (`preflight.rs:324-344`): list incidents → loop ids → per-id detail call → match. The extraction path needs the same two steps because `retrieve_report` / `retrieve_telemetry_slice` both require an `incident_id`.
- **Max-severity fold over checks** (`lib.rs:328-333`): `severity_rank` orders `Pass(0) < CalibrationRegion(1) < Fail(2)` and `max_by_key` keeps the worst — one `RunRecord` per scenario, not per check.
- **Producer-assigned `ReportState`** (`lib.rs:347-367` `manual_record`): the run seam constructs a `RunRecord` directly with a chosen `state` and `verdict: None`. This is the shipped precedent for assigning a context-specific state, and the natural home for a `KnownResidual` sibling.
- **Read-back spans already exist** (`client.rs:45,54,70,108,125`): `verify.readback.connect*`, `.list_tools`, `.call_tool` — the last carrying a `mcp_tool` field. `preflight.rs:133,233` add `.preflight`/`.preflight_boot`. Every tool call is already instrumented at the client, so the extraction inherits spans without adding any.

## Conventions to follow
- **Raw-shape stub fidelity**: `tests/common/mod.rs:76-110` emits Pulse's unenveloped shapes (`{items,total,next_cursor}` / `{incident_id, span_refs, fingerprint_refs}` / `{ok:true}` for everything else) — never an MCP `{content:[…]}` envelope. A new leg is a `StubConfig` knob (`:11-22`), matching the existing six.
- **`redact_value` before an observed string reaches an artifact**: applied today inside `classify` (`verdict.rs`), asserted by `tests/expected_slo.rs:76` `observed_with_host_path_is_redacted_in_assessment`.
- **Bound-parameter SQL only** for `runs.db`; the envelope's 11 fields and encodings are fixed with no migration framework.
- **Catalog shape** (35 scenarios): **26 carry `[[expected]]` checks** (37 blocks total — `Contains` 22, `Absent` 8, `CountAtLeast` 7; `Exact` is unused across the whole catalog), and **9 declare an empty `expected`**. Of those 9, **two declare a KNOWN-RESIDUAL** (`degraded-mode-report` P-053, `project-context-grounding` P-032) and seven are operator-checklist / declare-only. Any routing change on the empty-`expected` branch must not sweep all nine.
- **The 30 distinct expected tokens** map onto the verified result shapes: severity words (`Autonomous`/`Suggested`/`Curious`) and lifecycle words (`Resolved`) live in `query_incident_list` item fields; candidate/detector tokens (`RetryStorm`, `ErrorRateSpike`, `LatencyRegression`, `RestartEvent`, `ServiceWentSilent`, `Previously seen`, `resolution summary`) and the PII affixes (`sk_live_`, `@example.com`, `Bearer `, `password=`, `user.email`) live in the `retrieve_report` markdown; the numeric floors (`1`,`2`,`3`,`10`,`50`) are counts.

## New files to create
- Likely `crates/conductor-verify/src/extract.rs` — the per-check observation reader (tool result `Value` → observed string(s) + a degraded flag), promoting `incident_ids`/`fingerprint_refs` from `preflight.rs`'s private helpers into a shared home. *(Shape is P4's call; the graph shows the name is free.)*

## Files to modify
- `crates/conductor-run/src/lib.rs` — replace the `:301-305` placeholder with the real extraction; feed per-check observed values into the `:328-333` fold; route a degraded observation to `KnownResidual`.
- `crates/conductor-verify/src/lib.rs` — re-export the new extraction surface (the crate's `pub use` block is `:36-47`).
- `crates/conductor-verify/src/preflight.rs` — if `incident_ids`/`fingerprint_refs` move to the shared module, update their two call sites (`:329`, `:336`); otherwise untouched.
- `crates/conductor-verify/tests/common/mod.rs` — a `StubConfig` knob for the degraded/malformed/empty legs.
- `crates/conductor-verify/tests/readback.rs` — the new stub-driven legs.
- **Caller threading (from the graph, not memory):** `execute_scenario`'s public signature is preserved, so its three production callers — `conductor-cli/src/commands/run.rs:23`, `conductor-cli/src/commands/suite.rs:30`, `conductor-run/src/lib.rs:477` (`drive_run`) — need **no** edit. That is a consequence of the constraint, and the graph is what proves it; if the signature moved, these three are the threading set.

## Open questions
- **Which observation source serves which check, given `ExpectedCheck` carries no selector?** → blocks: **plan-decision**. The verified shapes make a composed observation (list fields + report markdown, per incident) the leading candidate over a per-`ComparisonKind` tool switch, but the choice changes what a `CountAtLeast` observed value even means. P4 must resolve before synthesis.
- **Does `degraded_mode` route the whole scenario record, or only checks read from that report?** → blocks: **plan-decision**. `degraded-mode-report.toml` has an EMPTY `expected`, so today it reaches `manual_record` (ManualCheck) and never touches `evaluate_check` — the routing must decide whether it is a run/record-level state assignment on the empty-expected branch, a per-check override, or both.
- **Does `fingerprints` get populated from `fingerprint_refs` in this chunk?** → blocks: **implementation-scope**. The source is proven and one call away, but neither the working entry nor `v2-09` states it; leaving it empty keeps the file list smaller, filling it makes the envelope honest.
