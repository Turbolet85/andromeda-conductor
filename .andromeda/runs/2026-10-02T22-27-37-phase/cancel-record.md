# Cancelled at P5 — 2026-10-02-p-075-re-round-on-incident-events

The promotion was rolled back on the operator's direction (overseer relay, 2026-10-03). No `plan.md` was written. The
chunk folder held only `scope.md` and `research.md`; both are kept byte-identical here as `cancelled-scope.md` and
`cancelled-research.md` (`cmp` clean), because the replan needs them.

## Why
- **The P3 finding.** At Pulse S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`, the deterministic L4 creation path
  writes a `created` incident event, and the sidecar returns it as `unknown`:
  - `pulse-app/src/inference_runtime.rs:922` writes it, into `incident_events` (`crates/corpus/src/contract.rs:939-951`);
  - `crates/mcp-server/src/tools.rs:487-501` coerces any non-status kind to `unknown`;
  - Pulse's own `pulse-app/tests/unit_incident_producer.rs:218-221` pins the event.

  As written, the round request's assertion 7, third bullet ("every `event_kind` in both reads is one of active /
  acknowledged / resolved"), therefore fails by construction. Its premise "Creation records NO event" is false. The
  finding was re-verified by hand in this phase and independently by the overseer at S.
- **The founder's live word, 2026-10-03, on Q2: HOLD the drive for a Pulse fix.** Pulse fixes it in its unwrapped
  P1 chunk and relays S2, and there is one round only. The P4 menu had recorded "drive on S"; the overseer corrected
  it.

## The P4 answers that stand for the replan
- **Q1, tool pin:** pin `retrieve_incident_events` as a fifth required MCP tool, in `READBACK_TOOLS`,
  `contracts/mcp-contract.toml` and the in-process stub's default list.
  - This is the founder's LIVE ratification of the playbook's escalate pattern "Boundary widening" (a new input class
    on the sidecar-stdout boundary), per the overseer's correction message.
  - The wrap that lands the pin records it as ratified on that word.
- **Q3, window stamp:** an additive timed sibling of `probe_resolve_lifecycle` in `crates/conductor-run/src/lifecycle.rs`.
  - It returns the observation plus the resolve call's `std::time` epoch-nanosecond request→response window.
  - The existing probe delegates to it, and no `LifecycleObservation` struct-literal site threads.
- **Q2, the predicted FAIL:** superseded by the hold. The replan grades against S2.

## For the replan (re-verify against S2; nothing below is carried as measured)
- The research's Conductor-side findings are tree facts and should still hold, but re-derive them at HEAD:
  - the four-name set lives in three places;
  - `readback_shape_witness.rs` and `preflight_spawn.rs` are the data-pin companions;
  - `probe_resolve_lifecycle` stamps no time;
  - the leg prints window-relative offsets to stay under the 16-hex probe.
- Every Pulse-side reading in `cancelled-research.md` is at S. S2 changes at least the creation event or its egress
  coercion, so re-read the tool's dispatch, the creation path and the resolve stamp at S2.
- **The working-route entry is unstamped and markerless again, with its original
  `BLOCKED-ON: Pulse "incident events readable through MCP"`.** That annotation no longer names the real dependency:
  the tool now exists, and the round waits on S2's fix. The annotation is route-resolve's to rewrite, at a 0-pending
  wrap adaptation, never by hand here.
