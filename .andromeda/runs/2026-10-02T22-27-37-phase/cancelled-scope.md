# Scope — 2026-10-02-p-075-re-round-on-incident-events

**Working entry (working-route.md:83):** The P-075 re-round on incident events — the round re-driven against Pulse with
its incident lifecycle events read back through MCP and graded.

**Version:** conductor-0.3.0 · Epoch 5 — Polish & ship · taken up on the operator's relay (overseer, measured
2026-10-03), which cleared the head's BLOCKED-ON. The Setup HALT was answered "take it up" by that relay plus the phase
invocation that followed it.

## What this chunk builds
ONE fresh, deterministic-posture P-075 round against the Pulse build at sha S. It grades the SEVEN assertions Pulse's
`round-request.md` names, each hard at its measured value:
- the six of the prior round, unchanged;
- a seventh, new: the round's incident lifecycle events read back through Pulse's new MCP tool
  `retrieve_incident_events`, graded around `mark_incident_resolved`.

It also pins `retrieve_incident_events` in Conductor's `contracts/mcp-contract.toml` (the CARRY). The round, its
scenarios and its evidence are Conductor's. Pulse names only the binary and the assertion set.

## BLOCKED-ON (folded — the dependency, and how it cleared)
- As written: `BLOCKED-ON: Pulse "incident events readable through MCP" — clears when Pulse relays a sha whose MCP tool
  surface returns incident lifecycle events (at Pulse 83d4060 the incident_events table reached no MCP tool, arch
  §Established Decisions [Read-Back Dependency Posture])`.
- Cleared by the relay naming S. Re-verified at phase time against the Pulse checkout, not taken from the relay's
  wording:
  - `git -C andromeda-pulse rev-parse HEAD` and `origin/chore/migrate-pulse-to-v3` both resolve to S;
  - at S, `crates/mcp-server/src/tools.rs:52` declares `TOOL_RETRIEVE_INCIDENT_EVENTS = "retrieve_incident_events"`,
    `:180` dispatches it and `crates/mcp-server/src/jsonrpc.rs:224` lists it in `tools/list`.
- The origin-ref reading is the LOCAL tracking ref (no fetch at phase time); the relay states origin = S.

## The binary under test (folded from the relay + re-measured)
- **S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`** on Pulse's `chore/migrate-pulse-to-v3`, parent `a69030a`. Subject:
  `chore(2026-10-02-incident-events-readable-through-mcp): operator pre-CI commit, for the run this chunk's verdict reads`.
- Build (Pulse's, already done): `cargo build --workspace --release --features mcp-server`, per Pulse
  `evidence/round-binary.md` (plan gate 16, green). That file is UNCOMMITTED in the Pulse tree (`git status`: `??`), so
  it is read from Pulse's working copy, never from S.
- Artifact sha256 values re-measured at phase time (2026-10-02T22:2xZ) — both match `round-binary.md`:
  - `pulse-app.exe`: `2141d524bd4d0d2f2e5378e95b9acad45c32ef91b55b6964b1886277a724ac35`
  - `andromeda-pulse-mcp.exe`: `ba8d5c3be3de88dbe8d95eedf83ca988fee09b5e8dff8db965b99cac9d30cd44`
- The prior round's operator directives carry forward as the plan's default: re-measure both sha256 values
  immediately before launch (a mismatch halts, never rebuilds), and no Pulse rebuild. This is a posture, not a code
  premise. Its basis is the precedent at `2026-10-02-p-075-assert-round-against-pulse/evidence/round-ledger.md`, and
  the operator confirms or replaces it at the P5 review.
- Pulse's source-identity note (two debug-only mutation checks restored, source equal to S) is a Pulse-side claim,
  cited and not re-derived.

## Pulse CI on S (relay; not this repo's red)
- Pulse `ci#37069724167` on S: 11/12 jobs success, red only on `supply-chain`: RUSTSEC-2026-0325 / -0326 / -0327 against
  `wasmtime 48.0.3`, with `Cargo.lock` identical to `a69030a`. External decay, owner pending on Pulse's side. It does not
  touch the assertions.
- Conductor's `Cargo.lock` carries no `wasmtime` package (`grep -c 'name = "wasmtime'` → 0, at phase time), so the three
  advisories do not reach this repo's supply-chain gate.
- The shipped Pulse binaries link `wasmtime 48.0.3` (the plugin host), per `round-binary.md`. The round runs those
  binaries as delivered.

## Mode (round-request §Mode)
- ONE round, deterministic: `ANDROMEDA_PULSE_L4_DETERMINISTIC` set on the app child, and a fresh data dir.
- `pulse-app` and `andromeda-pulse-mcp` come from S's `target/release`. Conductor spawns the sidecar by fixed NAME through
  the inherited `PATH`, so the round's `PATH` must resolve `andromeda-pulse-mcp` to S's sidecar.
- Ports 4317/4318 are shared with Pulse's tree, so the live leg runs only in the operator's slot.
- A Windows teardown by `Stop-Process -Force` leaves no `app.exit` record. That is expected, not a finding.

## The seven graded assertions (round-request.md §The seven graded assertions, at S)
1. **Read-back content fidelity.** The incident read back through MCP `retrieve_telemetry_slice` carries, in
   `fingerprint_refs`, the full 32-hex fingerprint Conductor derives for the storm it emitted. The incident opened after
   the storm's emission instant. `retrieve_report` renders it with `degraded_mode: false`.
2. **Runtime-state fidelity.** `mark_incident_resolved` applies, and the incident leaves `query_incident_list`'s active
   set.
3. **P-025:** the worst `metric.constellation.hue_update_ms.duration_ms` in the leg window ≤ 2000, graded per Conductor's
   `contracts/pulse-p025-measurement-contract.md` (the constellation dot hue, never the deferred Halo glow).
4. **P-027:** `metric.constellation.discovery_ms.duration_ms` (first-sighting anchored) ≤ 5000.
5. **P-037:** `metric.report.render_ms.value` ≤ 2000 (the Report window).
6. **P-045:** the worst `metric.findings.counter_refresh_ms.duration_ms` ≤ 1000 (the Findings counter).
7. **Incident events read-back (NEW).** For the round's incident:
   - read through `retrieve_incident_events` BEFORE `mark_incident_resolved`: the response carries NO `resolved` event;
   - read again AFTER it: the LAST event is `resolved`, and its `occurred_unix_nano` lies inside the resolve call's
     wall-clock window (request sent → response received);
   - every `event_kind` in both reads is one of `active` / `acknowledged` / `resolved`.

   The assertion is over the LAST event and the absence of `resolved` before the call, never an exact sequence.

## The new tool, as Pulse states it (round-request §What is new at S — to be re-verified at P3 against S's source)
- `tools/list` carries nine tools: the prior eight plus `retrieve_incident_events`.
- Input `{incident_id: integer}` — required; no other property accepted.
- Response `{"incident_id", "events": [{"event_kind", "occurred_unix_nano"}], "total", "truncated"}`. `events` are the
  incident's status transitions, oldest first, at most 256 (`truncated` true when more exist).
- `event_kind` ∈ `active` / `acknowledged` / `resolved`; any other stored value reads `unknown`.
- `[premise-corrected: the deterministic L4 creation path writes a "created" event (pulse-app/src/inference_runtime.rs:922 into incident_events, crates/corpus/src/contract.rs:939-951; pinned by Pulse's unit_incident_producer.rs:218-221), and the sidecar returns it as "unknown" (crates/mcp-server/src/tools.rs:487-501) — re-verified by hand at S]`
  The round-request said "Creation records NO event". At S, an incident formed by the deterministic L4 path reads back
  as `events: [{event_kind: "unknown", …}]` before its first status change, never `events: []`. As written, assertion
  7's third bullet ("every `event_kind` in both reads is one of active / acknowledged / resolved") therefore FAILS BY
  CONSTRUCTION. Its first two bullets are unaffected. The claim's corpus-level half is true: `save_incident` alone
  inserts no event (`contract.rs:611-640`). The app adds the event separately.
- An unknown id returns a JSON-RPC error, code -32603, message containing `incident not found`. VERIFIED at S
  (`tools.rs:548-551`, `bin/andromeda-pulse-mcp.rs:293-295`). A missing or ill-typed id is -32602.
- The direction is Pulse-supplies-the-value. VERIFIED at S:
  - the schema is `{incident_id: integer}`, required, with `additionalProperties: false` (`jsonrpc.rs:223-234`);
  - the response keys are `incident_id`, `events[].event_kind`, `events[].occurred_unix_nano`, `total` and
    `truncated`, ordered `ORDER BY id ASC`; `total` counts the events returned (`tools.rs:503-535`).
- **The resolve event's time** (research.md §Pulse at S):
  - The sidecar's `mark_incident_resolved` stamps `now` (epoch nanos, the host clock) INSIDE the call. It appends a
    `resolved` event at `now` in the same transaction, but only when the prior status differs (`tools.rs:455-480`,
    `contract.rs:653-694`).
  - The window comparison is therefore inclusive on both ends.
  - An incident already auto-resolved before the call gains no new event.
  - A deterministic re-emission inside the ≤ 60 s before the app's reconcile can append `active` AFTER `resolved`.
    So the AFTER read follows the resolve immediately.

## CONTEXT (folded)
- The founder ruling of 2026-10-02 (live, relayed by the overseer; route-archive.md:119): everything planned for 0.3.0
  is done now, properly; nothing is carried over. This entry is 2 of the 3 the ruling minted ahead of the version close.
- The first round graded six assertions PASS at Pulse S `03ec944` (`2026-10-02-p-075-assert-round-against-pulse`). This
  round is FRESH: P-075 is re-verified on this chunk's binary, not inherited from that one.

## CARRY (folded)
- From the `2026-10-02-captured-fingerprint-values-elided` wrap (overseer relay 2026-10-02): Pulse's round request
  carries 7 assertions and needs Pulse's new (9th, by the overseer's count) MCP tool `retrieve_incident_events` pinned in
  Conductor's `contracts/mcp-contract.toml`. `required_tools` holds 4 today: re-read at phase time —
  `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`.
- `required_tools` is what the preflight's missing-tool precondition gates on. VERIFIED: `run_preflight` maps every
  manifest name against `tools/list`, and any absent name gives "required tool(s) absent: …" ahead of the
  run-contract and canary arms (`conductor-verify/src/preflight.rs:171-226`). Pinning the fifth name therefore makes
  every preflight against a Pulse LACKING the tool (any build before S) `Blocked` on that EXISTING precondition. The
  named-precondition count stays five.
- The four-name set lives in THREE places, not one (research.md §Files inspected):
  - the committed manifest;
  - the code constant `READBACK_TOOLS: [&str; 4]` (`manifest.rs:17-22`). `validate()` requires the manifest to be
    its superset, and the CHILD stub's `tools/list` is built from it (`stub_pulse_mcp.rs:7,52`);
  - the IN-PROCESS stub's literal default list (`tests/common/mod.rs:113-121`), which
    `tests/readback_shape_witness.rs` runs against the committed manifest.

  A manifest-only pin would turn that witness test (and `preflight_spawn`) red on tool absence.

## What the round needs that is not shipped (hypotheses for P3)
- The prior round's machinery is re-usable at S. VERIFIED:
  - the `live-pulse`-gated `p075_round_live` leg;
  - the `lifecycle_harvest` / `delegated_timing_harvest` graders, which carry `RoundGrade` synthetic-arm builders
    and per-round evidence consts;
  - `probe_resolve_lifecycle` + `attribute_by_liveness`;
  - the `evidence_pin` module.

  Pulse's source moved only in `crates/corpus` and `crates/mcp-server` between `03ec944` and S (7 files), so the
  prior round's source readings for assertions 1–6 carry to S, and the P-025 contract needs no re-pin.
- Assertion 7 is new at every tier. VERIFIED:
  - no code calls `retrieve_incident_events` (`grep -rn retrieve_incident_events crates/` → 0 at phase time);
  - `probe_resolve_lifecycle` stamps NO time and exposes no request→response window (`lifecycle.rs:114-129`);
  - it has no production caller (only the two `live-pulse` legs).
- This round's evidence is its own. VERIFIED: each grader pins a per-file constant (`lifecycle_harvest.rs:412-419`,
  `delegated_timing_harvest.rs:378-382`). The prior six ids stay bound to the prior chunk's files, and this round's
  ids are new.
- `ANDROMEDA_PULSE_DATA_DIR` reaches the leg only through `ReadbackClient::connect` (`p075_round_live.rs:102-105`).
  The leg is therefore not a `capture_paths` reader and reads no `CONDUCTOR_RUNS_DIR`. The tests extract's "the leg
  reads `CONDUCTOR_RUNS_DIR`" is overridden by this finding.

## Grading posture (round-request §Grading posture)
- Each assertion is a hard PASS or FAIL at the MEASURED value. An absent sample is UNGRADED and never counts as met.
- A FAIL is a Pulse finding: **no re-drive for a pass** (Conductor's v3-08 posture). Pulse surfaces it as its own wrap
  escalation.
- Pulse records the round in its own `evidence/round-result.md`: Conductor's commit, CI run, the graded test ids and the
  evidence path for all seven, each verdict read from Conductor's files. This chunk therefore produces stable test ids
  and a committed evidence path Pulse can cite.

## Boundaries
- No Pulse source, build or ledger is touched. Pulse's evidence is read-only from here and cited, never copied.
- One round. A FAIL is recorded, not re-driven.
- The fixed sidecar NAME and the no-listener boundary are unchanged. Pulse's UI is not automated — nobody touches the
  desktop during the round (the prior round's ratified hands-off posture for P-037).
- Conductor's matrix (`v3-01`..`v3-11`) carries no P-075 capability; `verification-matrix.json#P-075` is Pulse's ledger.
  This chunk links no capability. VERIFIED: the matrix carries no P-075 id, and its unclaimed pool is empty
  (`matrix.py coverage`: verified 10/11 · deferred 1 · unclaimed 0).

## CI verdict read at Setup (fold source 2)
- `6a9ff7c` (the last wrap's flip = HEAD): **green**, checks 3/3, wall 694 s, CI#37037859266. Nothing to fold as a red.
