# Codebase Research — 2026-10-02-p-075-re-round-on-incident-events

## Scope
- **Depth:** deep (a mature codebase; the round re-uses five shipped graders and adds one new read-back surface).
  - **Reads:** 21.
  - **Globs/Greps:** 14.
  - **Graph queries:** 1 (rust plane; trace `tree-query-2026-10-02-p-075-re-round-on-incident-events.json`).
  - **Pulse-side source pass at S:** one Explore agent. I re-read its five load-bearing claims myself (marked
    **[re-verified]** below).
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full (it loaded with this phase's reads of
    `crates/**/tests/**`), its 33 Session Additions included.
  - `.claude/rules/testing.md`: read in full, 49 Session Additions.
  - `.claude/rules/observability.md`: read in full.
  - `.claude/rules/host-win32.md` and `.claude/rules/security.md`: always loaded.
  - Applied below:
    - the firing-form env block, with `PATH` resolving S's sidecar;
    - no `boot` before a preflight-firing leg;
    - a fresh data dir under `%TEMP%/pulse-legs/<letters-only leaf>` and 150 s quiet windows;
    - the pre-leg line count;
    - a freeze per leg;
    - the census and stop form;
    - printed-verdict atoms;
    - the digest pin;
    - the stub item-key fidelity rule.
- **Platform issues consulted:** none. There is no runner-only bullet: Setup 5a read CI#37037859266 green 3/3 on
  `6a9ff7c`.

## Files inspected
- `contracts/mcp-contract.toml` (full) — `required_tools` holds 4 names (re-read at phase time).
- `crates/conductor-verify/src/manifest.rs` (full):
  - `READBACK_TOOLS: [&str; 4]` (`:17-22`);
  - `validate()` requires `required_tools ⊇ READBACK_TOOLS` (`:58-64`), so a manifest may carry MORE names than the
    constant, never fewer;
  - unit tests at `:79-113` (`loads_and_bounds_checks_the_committed_manifest` reads the committed file through
    `CARGO_MANIFEST_DIR`; `rejects_a_dropped_pinned_tool`).
- `crates/conductor-verify/src/preflight.rs` (`:150-299`):
  - `run_preflight` maps EVERY manifest name to `Present`/`Absent` against `tools/list` (`:171-197`);
  - a single absent name gives `blocked_precondition = "required tool(s) absent: {names}"` (`:220-226`), ahead
    of the run-contract and canary arms. Pinning a fifth name therefore blocks any sidecar lacking it on the
    EXISTING tool-absence arm. The count of named preconditions stays five.
- `crates/conductor-verify/src/client.rs` (full):
  - four tool-name consts (`:22-26`);
  - `call_tool` is `#[tracing::instrument(name = "verify.readback.call_tool", fields(mcp_tool = %name))]`
    (`:138-150`), so any new typed method that routes through it inherits the bounded span and the allowlisted
    attribute with no new span name;
  - `resolve_incident(i64)` (`:186-189`) is the one-field-object convenience precedent.
- `crates/conductor-verify/src/lib.rs` (`:43-49`) — re-exports the four consts and `READBACK_TOOLS`.
- `crates/conductor-verify/src/jsonrpc.rs` (`:18`, `:100`) — `MAX_LINE_BYTES = 16 MiB`. A worst-case 256-event
  `retrieve_incident_events` response is ~18 KB (Pulse Q6), far inside the bound.
- `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` (`:7`, `:51-56`) — the CHILD stub's `tools/list` is built
  from `READBACK_TOOLS`, so it advertises whatever the constant holds. An unknown `tools/call` falls through to a
  generic result.
- `crates/conductor-verify/tests/common/mod.rs` (`:90-135`, `:150-290`) — the IN-PROCESS stub:
  - `StubConfig::default().tools` is a LITERAL four-name list (`:113-121`), not `READBACK_TOOLS`;
  - its `tools/call` dispatch has per-tool arms (`:198-284`) and a `resolve_declines` JSON-RPC-error knob
    (`:211-219`), which is the precedent for an error-arm knob;
  - an unrecognized tool returns `{"ok": true}` (`:282`).
- `crates/conductor-verify/tests/readback_shape_witness.rs` (`:43-74`) — runs `run_preflight` against
  `StubConfig::default()` with the COMMITTED manifest loaded from `CARGO_MANIFEST_DIR`. With a fifth pinned name and
  the four-name literal default, this test would block on tool absence. It is a data-pin companion of the manifest.
- `crates/conductor-verify/tests/preflight_spawn.rs` (`:20-26`) — the committed manifest against the CHILD stub. It
  holds whenever `READBACK_TOOLS` and the manifest agree.
- `crates/conductor-cli/tests/cli_smoke.rs` (`:23-28`) and `crates/conductor-cli/tests/cross_surface_parity.rs`
  (`:34-35`, `:99`) copy the committed manifest into a temp tree. `cli_smoke` runs the no-Pulse Blocked path;
  `cross_surface_parity` reaches the child stub, so it holds under the same agreement as `preflight_spawn`.
- `crates/conductor-run/tests/composition_root.rs` (`:50-56`) — calls `preflight`/`readiness` on the relative path.
  It discards the result.
- `crates/conductor-tauri/src/commands.rs` (`:74`, `:323`) — the GUI resolves the manifest at runtime. No literal.
- `crates/conductor-run/src/lifecycle.rs` (`:1-135`):
  - `LifecycleObservation { before, resolved, after }` with pub fields (`:13-18`);
  - `probe_resolve_lifecycle` reads the active set, logs the id on `message` (`:121`), calls
    `client.resolve_incident`, then reads it again (`:114-129`). It **stamps no time**: the resolve call's
    request→response window is not exposed anywhere.
- `crates/conductor-run/src/lib.rs` (`:35-38`) — re-exports the lifecycle API.
- `crates/conductor-run/tests/p075_round_live.rs` (full) — the prior round's leg:
  - the firing-form doc (`:8-23`);
  - the `std::time` emission stamp (`:43-48`, `:116`);
  - the storm and form poll (`:116-135`);
  - slice and report reads (`:153-193`);
  - keep-alive then `probe_resolve_lifecycle` (`:195-217`);
  - one buffered `p075-round:` block (`:110`, `:220`);
  - `ANDROMEDA_PULSE_DATA_DIR` passed to `ReadbackClient::connect` only (`:102-105`): NOT a `capture_paths` reader,
    and it reads no `CONDUCTOR_RUNS_DIR`.
- `crates/conductor-run/tests/lifecycle_harvest.rs` (`:280-458`):
  - the synthetic-capture builder and `round_fields` parser (`:290-313`);
  - `grade_assertion_1`/`grade_assertion_2` with `RoundGrade::{Pass, Fail, Ungraded}`;
  - synthetic arms per failing shape (`:316-407`);
  - the pinned capture consts (`:412-419`);
  - the graded ids `p075_round_assertion_{1,2}_*` (`:425-441`);
  - the tamper arms (`:443-458`).
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (`:340-470`):
  - `ROUND_EVIDENCE` and `round_lines` (`:378-382`);
  - `p075_round_assertion_{3..6}_*` (`:393-453`);
  - the tamper arms (`:455-470`).
- `crates/conductor-run/tests/evidence_pin/mod.rs` (full) — `sha256_hex`, `check_digest`, `committed` (LF-normalized,
  workspace-anchored) and `pinned`.
- `conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/`:
  - `scope.md`, `research.md` and `report.md` (full);
  - `evidence/p075-leg.txt`;
  - `evidence/round-ledger.md` (head);
  - the `[[gate]]` index of `plan.md` (`:280-507`).

  This is the recipe this round re-runs: the P-075 leg first on a fresh launch, then H/D/R/F with 150 s quiet
  windows, a freeze per leg, censuses, the hygiene and hex probes, the guarded push and the CI read.

## Pulse at S (`4a26ad8`) — read by an Explore agent, five claims re-verified by hand
Paths are relative to the Pulse repo; HEAD = S (confirmed).
- **The tool surface:**
  - `tools/list` advertises nine tools, `retrieve_incident_events` last (`crates/mcp-server/src/jsonrpc.rs:127-234`).
  - Its schema is `{incident_id: integer}`, required, with `additionalProperties: false` (`:223-234`).
  - Dispatch is `dispatch_retrieve_incident_events` (`tools.rs:503-535`) **[re-verified]**.
  - Response `{incident_id, events: [{event_kind, occurred_unix_nano}], total, truncated}`. `total` counts the events
    RETURNED (≤ 256). Order is `ORDER BY id ASC` (`crates/corpus/src/contract.rs:862-863`).
  - An unknown id gives `-32603` "tool dispatch failed for `retrieve_incident_events`: incident not found"
    (`tools.rs:548-551`, `bin/andromeda-pulse-mcp.rs:293-295`).
  - A missing or ill-typed id gives `-32602` (`tools.rs:576-580`, `bin:287`).
  - **Direction is VERIFIED:** Conductor passes `incident_id` only; every field is returned.
- **`event_kind` coercion** `coerce_event_kind` (`tools.rs:487-501`) **[re-verified]**: only the three
  `incident_status_label`s (active / acknowledged / resolved) pass; ANY other stored value leaves as `unknown`.
- **Creation DOES record an event** **[re-verified]**:
  - `create_incident_from_l4_output` calls `persistence.save_incident_event(id, "created", now_unix_nano)` right
    after the incident is saved (`pulse-app/src/inference_runtime.rs:922`);
  - `save_incident_event` inserts into `incident_events` (`crates/corpus/src/contract.rs:939-951`), the table the
    tool reads (`:862`);
  - Pulse's own `pulse-app/tests/unit_incident_producer.rs:218-221` pins it ("a `created` audit event is recorded").
  - So on the deterministic L4 path, the first `retrieve_incident_events` read of a freshly formed incident returns
    `[{event_kind: "unknown", …}]`, not `events: []`.
  - **This falsifies the round-request's "Creation records NO event"** (`round-request.md` §What is new at S) and
    makes assertion 7's third bullet ("every `event_kind` in both reads is one of active/acknowledged/resolved") FAIL
    BY CONSTRUCTION. The first two bullets are unaffected: no `resolved` before; LAST `resolved` after.
- **The resolve stamp** (`tools.rs:455-480`) **[re-verified]**:
  - the sidecar's `mark_incident_resolved` takes `let now = current_unix_nanos()` inside the call;
  - it writes status `resolved` with `updated_unix_nano = now` through `update_incident_status`, which appends a
    `resolved` event at `now` in the same transaction ONLY when the prior status differs (`contract.rs:653-694`).
  - `now` is `chrono::Utc::now()` epoch nanoseconds (`tools.rs:572-574`), the same host clock Conductor's
    `SystemTime` reads. It therefore lies between Conductor's request-sent and response-received stamps.
  - Equality at the clock's tick is possible, so the window comparison is INCLUSIVE.
  - Already-resolved incident → no new event (same status). Then the LAST event carries the EARLIER resolve's time
    and falls outside the window. This is the auto-resolve hazard below.
- **Automatic transitions** (agent-read; consistent with the prior round's Q3 at `03ec944`):
  - **No auto-acknowledge.** The only caller is the UI router (`incidents_router.rs:234`).
  - **Auto-resolve** writes `resolved` at its tick for an incident idle ≥ 120 s (`incident_observer.rs:24,67,74`;
    `state_machine.rs:49-51`). The leg's liveness keep-alive (`p075_round_live.rs:197`) is what keeps this away
    before the resolve.
  - **Post-resolve re-emission:** inside the ≤ 60 s before the app's persist reconcile, a deterministic re-emission
    on the same dedupe tuple re-writes the row `active` and appends an `active` event AFTER `resolved`
    (`inference_runtime.rs:836-848`; reconcile `triage/src/incident/persistence.rs:250-283` writes no event itself).
    The AFTER read must therefore follow the resolve immediately. A later re-read is a witness only, never the grade.
- **Scope of Pulse's change since the last round:** `git diff --stat 03ec944 4a26ad8` over source = 7 files, all in
  `crates/corpus` and `crates/mcp-server` (+654/−17). No UI, telemetry, triage or inference-runtime file moved.
  - The prior round's source readings for assertions 1–6 (fingerprint union, `degraded_mode`, the four delegated
    leaves, auto-resolve, `app.exit`) therefore carry to S unchanged.
  - The P-025 contract's `03ec944` coordinates need no re-pin.
- **Pulse CI:** red on supply-chain only (`wasmtime 48.0.3`), per the relay. Conductor's `Cargo.lock` has no
  `wasmtime` (`grep -c 'name = "wasmtime' Cargo.lock` → 0).

## Graph impact
Trace `tree-query-2026-10-02-p-075-re-round-on-incident-events.json`, `refs` keyed on `callee_name`, rust plane,
58 rows (the trace's `rows` field). Graph lines are 0-indexed; the citations below are +1.
- **`READBACK_TOOLS`** — 8 refs:
  - `stub_pulse_mcp.rs:7,52` (the child stub's import and `tools/list`);
  - `lib.rs:50` (re-export);
  - `manifest.rs:58,83,101` (`validate` and two unit tests);
  - `tests/preflight.rs:12,103`.

  Growing it to five propagates to the child stub automatically. Every use is iteration or `.iter()`, so the array
  type change `[&str; 4]` → `[&str; 5]` breaks no call site.
- **`ContractManifest`** — 23 refs:
  - production loads at `conductor-run/src/canary.rs` (`:22,100,108,243,354`) and
    `conductor-verify/src/preflight.rs` (`:27,158,269`);
  - the committed-manifest test readers `tests/preflight_spawn.rs:13`, `tests/readback_shape_witness.rs:20,60` and
    `tests/preflight.rs`.

  No caller depends on the tool COUNT.
- **`LifecycleObservation`** — 15 refs. Struct-literal sites: `lifecycle.rs:124`, `src/testkit.rs:125`,
  `tests/lifecycle_harvest.rs:83,94`. Also `tests/composition_root.rs:24,41` (a type ref).
  - Adding a field would thread through every literal site.
  - An additive sibling function threads through none.
- **`probe_resolve_lifecycle`** — 3 refs:
  - the re-export `lib.rs:38`;
  - imports in `lifecycle_live.rs:40` and `p075_round_live.rs:33`.

  It is an async fn, the index-gap class, so grep supplies the call sites: `lifecycle_live.rs:155` and
  `p075_round_live.rs:199`. There is NO production caller (grep `probe_resolve_lifecycle` over `crates/*/src` → the
  definition and the re-export only).
- **`resolve_incident`** — `lifecycle.rs:122`, `tests/readback.rs:291`.
- **`MARK_INCIDENT_RESOLVED`** — `client.rs:179`, `lib.rs:44`, `manifest.rs:11,21`, `tests/preflight.rs:11,131,137`.
  These are the sibling-const precedent for the new `RETRIEVE_INCIDENT_EVENTS`.

## Patterns detected
- **The live leg prints and the harvest grades** (`p075_round_live.rs:20-23`; `lifecycle_harvest.rs:409-441`).
- **Absence is UNGRADED, never met**, as `RoundGrade::Ungraded` (`lifecycle_harvest.rs:346-371`).
- **One synthetic arm per failing shape** with key=value overrides on a canonical capture (`lifecycle_harvest.rs:290-407`).
- **Digest-pinned evidence graded from file**, with a tamper arm (`evidence_pin/mod.rs`; `lifecycle_harvest.rs:443-458`).
- **A JSON-RPC-error knob on the stub** for a declined arm (`common/mod.rs:211-219`, `resolve_declines`).
- **A one-field typed convenience method** over the raw-`Value` call (`client.rs:186-189`).
- **Stdout is integers, booleans and closed words only.** The prior hex probe `grep -cE "[0-9a-f]{16,}"` over the leg
  capture reads 0. A raw 19-digit `occurred_unix_nano` would MATCH that pattern (digits are hex), so the leg prints
  window-RELATIVE offsets, never raw epoch nanos.

## Conventions to follow
- A new tool name joins the client as a `pub const` beside the four (`client.rs:22-26`), is re-exported from
  `lib.rs`, and enters `READBACK_TOOLS` so the manifest's superset check and the child stub agree.
- The incident id rides `message`, never an attribute (`lifecycle.rs:119-121`; obs-plan §6).
- Readers accept `incident_id` and `id` (testing.md 2026-09-01). The new tool's response key is `incident_id`, read
  from S's source. The stub must emit it, plus `events[].event_kind`, `events[].occurred_unix_nano`, `total` and
  `truncated` (test-plan §5 STUB ITEM-KEY FIDELITY).
- Window stamps are `std::time::SystemTime` epoch nanos (obs-plan §11), taken immediately around the
  `resolve_incident` await, never tokio time.
- A `live-pulse`-gated target owes `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`.
- Capture output is buffered and written ONCE, leading with a newline (testing.md 2026-09-23).
- Name every scenario by its stem (P-025/027/037/045 are each named by several scenarios).
- The firing env block, pasted whole:
  - `PATH` with S's `target/release` first, in POSIX form;
  - `ANDROMEDA_PULSE_DATA_DIR` = the live app's dir;
  - `ANDROMEDA_PULSE_MCP_ENABLED=true`;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`;
  - `which andromeda-pulse-mcp` before the first leg (verification-harness.md 2026-08-20, 2026-09-07).
- No `boot` before a preflight-firing leg, and 150 s quiet windows between legs on one launch.
- A fresh data dir `%TEMP%/pulse-legs/<letters-only leaf>`, with `pulse-app` launched BY PATH from a cwd outside this
  repo.
- Censuses before and after by `Win32_Process` PID + parent + CreationDate. Stop only what the agent started.
- Evidence file names stay outside the `rm-capture*.txt` population glob (security-history 2026-10-02).

## New files to create
- `conductor-0.3.0/chunks/2026-10-02-p-075-re-round-on-incident-events/evidence/` — the round's committed evidence: the P-075 leg capture, each delegated leg's frozen self-obs and target-filtered Pulse slice, and the round ledger with the binary sha256 values and the censuses

## Files to modify
- `contracts/mcp-contract.toml` — `required_tools` gains `retrieve_incident_events`
- `crates/conductor-verify/src/client.rs` — the `RETRIEVE_INCIDENT_EVENTS` const and a typed `retrieve_incident_events` read by id through `call_tool`
- `crates/conductor-verify/src/lib.rs` — re-export the new const
- `crates/conductor-verify/src/manifest.rs` — `READBACK_TOOLS` grows to five, its doc and a unit arm for the fifth name
- `crates/conductor-verify/tests/common/mod.rs` — the default tool list gains the fifth name and the stub answers `retrieve_incident_events` with Pulse's raw shape plus an unknown-id error knob
- `crates/conductor-verify/tests/readback.rs` — stub-tier arms for the new read: populated, empty and the unknown-id JSON-RPC error
- `crates/conductor-verify/tests/preflight.rs` — an arm where a sidecar lacking the fifth tool blocks on the tool-absence precondition
- `crates/conductor-run/src/lifecycle.rs` — the resolve call's request-to-response window, stamped from std time
- `crates/conductor-run/src/lib.rs` — re-export the new lifecycle item
- `crates/conductor-run/tests/p075_round_live.rs` — the leg reads incident events before and after the resolve and prints window-relative offsets and closed words
- `crates/conductor-run/tests/lifecycle_harvest.rs` — the re-round's assertion 1, 2 and 7 grades over its pinned capture with synthetic arms per failing shape
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — the re-round's assertion 3 to 6 grades over its pinned slices

## Open questions
- **Assertion 7's third bullet is predicted to FAIL from source.** The deterministic creation path writes a
  `created` event that the sidecar returns as `unknown`. Do we drive the round on S as written, with the predicted
  FAIL pre-registered and relayed as a Pulse finding? Or do we hold the drive until Pulse amends either the request
  or the egress coercion? → blocks: plan-decision.
- **Where the resolve window is stamped.**
  - (a) An additive `conductor-run` sibling of `probe_resolve_lifecycle` that returns the observation plus the
    window, so assertion 2 keeps grading through the shipped probe path.
  - (b) A new field on `LifecycleObservation`, which threads through four struct-literal sites.
  - (c) A leg-local re-implementation of the probe, which leaves `src/` untouched but moves assertion 2 off the
    shipped probe.

  → blocks: plan-decision.

## Scope premise closure
Each `[inferred]` bullet of `scope.md`, and the folded tool-description bullet the Pulse read falsified, is closed
against the findings above. scope.md is amended in place.
