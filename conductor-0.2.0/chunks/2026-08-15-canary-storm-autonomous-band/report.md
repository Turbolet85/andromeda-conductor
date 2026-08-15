# Report — 2026-08-15-canary-storm-autonomous-band

**Chunk:** Canary storm inside Pulse's Autonomous band — the preflight canary raised past Pulse's Tier-1
acceptance threshold so an incident can form, plus the live ready:true preflight attempt (conductor-run, v2-10)
**Date:** 2026-08-15
**Commits:** none yet since `33ec6c8 chore(route): operator-requested adaptation — 0-pending wrap` (this wrap
is the chunk's first commit)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-run/src/lib.rs` (the only source edit) · `conductor-0.2.0/verification-matrix.json`
  (ledger) · `conductor-0.2.0/working-route.md` (freeze stamp, from phase) · `.andromeda/master-route.md`
  (promotion record, from phase) · `.andromeda/friction-log.ndjson` · NEW
  `conductor-0.2.0/chunks/2026-08-15-canary-storm-autonomous-band/` (scope · research · plan · report) · NEW
  `.andromeda/runs/2026-08-15T22-00-09-phase/` (7 extracts + graph trace). Counts from `git status`:
  **5 modified · 2 new**.
- **Symbols / APIs:** `conductor_run::CANARY_STORM_COUNT` — value `6` → `12`; type, visibility and name
  unchanged (`pub const … : u64`). Its doc comment was rewritten. **No** new/changed fn, IPC method, endpoint,
  export, port, socket or env var.
- **Crates / modules:** none added/removed. `conductor-run` changed (one constant + its doc comment).
- **Dependencies:** none added, none bumped. `Cargo.lock` moved **zero lines** (verified against HEAD).
- **Schema / config:** none. No migration, no config key, no violation schema. Notably **no** contract-manifest
  term was added — recording Pulse's threshold as contract data was considered and REJECTED at P4 (below).
- **Spec-master edits:** none applied by implement (implement never authors spec changes). Three are pinned as
  Expected amendments for THIS wrap — see §Spec claims disproved by measurement.
- **Counts / qualifiers moved:** **one** — the canary's occurrence count `6` → `12`, which
  `architecture.md:161` states in prose ("a working six-occurrence identical-fingerprint storm reads `1`,
  never 6") and whose span-count figure ("all nine spans", `1 → 2 → 3 → 9`) the 2026-08-15 leg supersedes
  with `15` = 3 warm-up + 12 storm. *(Corrected during this wrap's P2: originally filed "none" because the
  value sat only under Symbols/APIs — the arch detector found it there regardless and proposed both sites.)*
  Coverage stays **11/32**; `UNBACKED_AUTO`, the capability manifest and every roll-up are untouched.
- **Dev-tool versions:** none installed/upgraded. `cargo audit` re-run at its existing version.
- **Reverted / negative API facts:** `v2-10` was CLAIMED at phase P5 (`chunk = {marker}`) and then
  **deliberately un-claimed** by implement after the live leg measured `ready:false` — back to `chunk:null`,
  `status:planned`, acceptance text unchanged and unweakened. Nothing shipped that asserts `ready:true`.
- **Spec claims disproved by measurement:** THREE, all in one paragraph plus its rules-file twin.
  1. `.andromeda/architecture.md:161` — "Pulse gates cue evaluation on `BootstrapState::Ready`" and "without a
     baseline … the L2 cue evaluator never considers it" are true only of the **baseline-derived** families.
     Evidence: `crates/triage/src/cue/evaluate.rs:164` is the ONLY such gate in `crates/triage/` and sits
     inside `evaluate_service_went_silent`; the RetryStorm path consults no baseline
     (`crates/triage/src/pattern/storm.rs:245-285`); Pulse measured 2 incidents with `baseline_state` at 0 rows
     (`andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md`).
     This leg independently corroborates the scoping: `triage.baseline.service_went_silent.evaluate` fired 203
     times while no storm cue fired at all.
  2. `.andromeda/architecture.md:161` — "Reaching a live incident therefore requires a Pulse-side change (a
     test-mode bootstrap override, or populating the `baseline_state` table)" is measured false as stated: no
     such change exists, `baseline_state` still holds 0 rows, and Pulse forms incidents anyway for a sustained
     storm. The incident-formation story belongs on the **tier band** (`5 ≤ 6 < DEFAULT_AUTONOMOUS_THRESHOLD
     = 10`, Tier-1 Autonomous-only at `crates/triage/src/cadence/coordinator.rs:390`).
  3. `.andromeda/architecture.md:161` — the "second, independent Pulse-side gap … a region, not a named defect"
     is **NOT superseded**; the plan predicted it would be retired by the healthy-feed proof and this chunk's
     own live leg is the evidence it LIVES. It is now sharpened to **producer-dependent**: `inject_demo`'s
     spans fingerprint (Pulse tracked `c33df842`), Conductor's canary spans arrive and are counted but never
     fingerprint. See §Outcome for the discriminating trio.
  The same three corrections are owed to `.claude/rules/verification-harness.md:47` (the 2026-06-27 chain and
  its 2026-08-10 / 2026-08-13 extensions), which is a rules file — it rides **curation** as the chain's next
  in-place extension, never the amendment cascade.
- **Coverage of new surfaces:** no new external surface, hot-path op, or UI element. The one changed surface:
  - `conductor_run::CANARY_STORM_COUNT` (existing constant, new value) → validation n/a (a compile-time
    constant, no boundary input) · instrumentation ✓ (the existing `emit.batch` wire-shape witness tracks it —
    asserted at the new value by `canary_wire::the_wire_shape_witness_reaches_the_self_obs_artifact`, which
    reads a real self-obs artifact the test produced) · PII n/a · tests ✓ (4 existing `canary_wire` assertions
    follow it symbolically; 31/31 crate, 581/581 workspace) · a11y n/a (headless cli surface, a11y-plan §1
    not-assertable) · tokens n/a.

## Deviations from intent

1. **`crates/conductor-run/tests/canary_wire.rs` was NOT edited**, though the plan listed it under Files to
   modify. Justification: the plan predicted exactly this ("expected **no edit** … review only"). All four
   sites assert symbolically against `CANARY_STORM_COUNT` and the only literal (`spans=1`, spans-per-batch) is
   count-independent. Reviewed, confirmed, unchanged — the prediction held.
2. **`v2-10` was claimed at P5 and un-claimed at implement.** Justification: the claim rested on the premise
   that the tier band was the last blocker; the live leg disproved it. The operator's directive is explicit —
   a leg that does not reach `ready:true` is a recorded result feeding un-claim/decline, never stretched. The
   acceptance text was never weakened; the measurement is recorded in the cap's `notes`.
3. **The plan's Expected-amendments list under-enumerated the correction surface**, corrected at the P5 review
   before approval (one over-broad sentence → the full `:161` paragraph plus the rules-file chain), and again
   at this wrap (a THIRD correction: the second-gap region is un-retired). Justification: a proven premise
   correction implies a wider amendment surface than the sentence that states it most obviously.

## Decisions & corrections

- **Margin = 12, decided by the plan within two measured bounds**, per operator directive. Floor is 10
  (`count >= autonomous_threshold`, `storm.rs:245`) so 12 clears it by 2 and survives losing an occurrence;
  the unpaced burst lands ~250× inside the 30s detection sub-window and ~100× under the envelope's
  `max_sustained_rate_spans_per_s`. Leaned on `scenarios/fingerprint-storm.toml`, which already uses 12 for
  exactly this transition. The arithmetic is stated in the plan rather than assumed.
- **Recording Pulse's threshold as contract data — REJECTED (operator decision, P4).** It is arch's own
  pattern for SUT facts and would make the next threshold move fail loudly, but it costs a TOML term plus a
  `RunContract` field, bounds-check and tests — beyond the three items the visit named. Scope stayed tight;
  the doc comment carries the citation instead.
- **The doc comment is fix scope, not cleanup.** It was the artifact that encoded the wrong reasoning
  ("over Pulse's `>=5 in 30s` retry-storm floor … so the gate's incident is raised deterministically") by
  naming the cue floor and reasoning no further. Rewritten to state the RELATION and cite the SUT symbols,
  never a re-baked literal — the mechanism by which it went stale the first time.
- **Three dictated citations were corrected before they froze into a route annotation** (session-start visit):
  `crates/triage/src/**pattern**/storm.rs` (not bare `triage storm.rs`), `crates/triage/src/**cadence**/
  coordinator.rs` (not `crates/triage/src/coordinator.rs`), and the comparison is `>=` so **ten suffice, not
  eleven** — moving the margin floor by one. A third bound (`DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30`) was
  unstated in the brief entirely.
- **`storms_detected_total` discriminates storm PRESENCE, not TIER** — Pulse increments it on both the
  Suggested and Autonomous branches; `severity_hint` on `triage.pattern.storm.detected` names the tier. This
  refines the 2026-08-14 rule that promoted it as the window-immune discriminator.
- **Operator ruling carried in from the route wrap:** evolve-diagnose run dirs are audit-trail class and are
  absorbed into the wrap commit rather than halting the dirt-check.

## Outcome

**Acceptance criteria: met except the one requiring the live leg to go green.** Gates green in **1 iteration**,
no fixes needed: `cargo nextest run -p conductor-run` 31/31 · `cargo nextest run --workspace --profile ci`
**581/581** zero retries · `cargo test --workspace --doc` 3 passed across 7 suites · `cargo clippy --workspace
--all-targets -- -D warnings` exit 0. Smoke ✓ — `bash scripts/agent-run.sh run` exit 0.

**Supply chain (the 18th pin, recorded not remediated):** `cargo audit` exit 1, byte-identical
`duplicate advisory ID: RUSTSEC-2026-0244` — an advisory-DATABASE fault with nothing to raise a floor to.
Basis re-verified rather than echoed: `Cargo.lock` **zero lines** vs HEAD, and the named overlap
`cargo deny check` **true exit 0** across advisories/bans/licenses/sources. No floor raise, no `deny.toml`
entry, no CI edit.

**The live leg ran in full and returned `ready:false`.** Conditions: BOTH Pulse binaries rebuilt at HEAD
`d090314` (`mcp-server --bin andromeda-pulse-mcp --features mcp-server`, then `pulse-app`), fresh data dir,
`pulse-app` launched from a cwd outside both repos, all six recipe preconditions met (sidecar on PATH · L4
deterministic · `ANDROMEDA_PULSE_MCP_ENABLED` in Conductor's own env · contract-derived poll budget · paired
`RUST_LOG=info,conductor_emit=debug`). Clean shutdown: `:4317` released, zero orphans.

*What worked:* `span_count: 15` at OTLP ingest — exactly the predicted 3 warm-up + 12 storm, so the raised
count reaches Pulse intact. Sidecar decrypted and dispatched (`tool dispatch ok`), all four required tools
present, protocol `2024-11-05` negotiated, and the app published its workspace key under the data dir
(`run/workspace-key`). `service_registry: 1` — the canary service registered.

*What did not:* `incidents: 0`, `canary_round_trip: "failed"`, blocked on the workspace-key/no-incident
precondition with `query_incident_list` returning 0 rows throughout the poll. No storm was detected at any
tier: no `severity_hint` line, and `storms_detected_total` / `fingerprints_evicted_total` /
`tracked_fingerprints_count` all 0 across 13 storm-detector ticks.

**The discriminating trio (`buffer.tick`, 15 ticks, every one identical):**

```
span_events_seen:      0     observer_invocations:  0     fingerprints_computed: 0
rows_ingested:         1     retention_window_active: false     memory_bytes: 0
```

This names the failure class. `observer_invocations: 0` means the fingerprint observer was **never invoked**
— not invoked-and-empty — so the gap is upstream of fingerprinting altogether. `span_events_seen: 0` says the
buffer never enumerated a single span event, and `rows_ingested: 1` against `span_count: 15` says spans are
counted at OTLP ingest but do not land as buffer rows. The gap therefore sits **between OTLP ingest receipt
and the buffer's span-event enumeration**, and it is **producer-dependent**: `inject_demo`'s spans traverse
the same path and fingerprint (`c33df842`), Conductor's canary spans do not. That is a materially sharper
localization than the 2026-08-14 "region, not a named defect", and it is the remaining preflight-green
blocker.

**Net:** the constant fix is correct and complete, and it was **necessary but not sufficient**. The chunk's
central premise — that the tier band was the LAST blocker — is disproved by its own leg. `v2-10` returns to
the pool unclaimed with the measurement recorded, so the coverage gate is a clean no-op rather than a HALT.
