# Report — 2026-08-21-severity-lifecycle-live-proof

**Chunk:** severity-lifecycle live proof — auto-resolve and resolution summary observed through read-back (P-019..P-023, P-059, P-060)
**Date:** 2026-08-21
**Commits:** none since `last_wrap` (2026-08-20T23:21:35Z) — this wrap authors the chunk commit.

## Changes (structured — detectors read this)

- **Files:**
  - `scenarios/incident-auto-resolution.toml` · `scenarios/severity-tier-autonomous.toml` ·
    `scenarios/severity-tier-suggested.toml` · `scenarios/severity-tier-curious.toml` ·
    `scenarios/ack-cooldown.toml` — re-shaped stimulus + retired declare-only with measurements.
  - `crates/conductor-core/src/scenario.rs` — **`#[cfg(test)]` module ONLY** (no production line touched).
  - `crates/conductor-run/tests/severity_harvest.rs` — NEW test binary (16 tests).
  - `conductor-0.2.0/chunks/2026-08-21-severity-lifecycle-live-proof/` — scope · research · plan · report ·
    `evidence/leg-verdict.md` + `evidence/leg-{a..e}/` (502 KB total).
  - `conductor-0.2.0/verification-matrix.json` — v2-16 claim (phase P5) + this wrap's refinement.
- **Symbols / APIs:** **none added, none changed, none removed.** Zero production-source delta: no public fn,
  no IPC method, no endpoint, no export, no port, no socket, no env var. The shipped
  `route_read_back` / `ReadBack {Graded, AutoResolved, Blocked}` (crate-private, `conductor-run`) was
  EXERCISED, not modified — its `AutoResolved` arm fired live for the first time. Test-only helpers added
  inside the new harvest binary are private to it and have no callers outside it. `conductor-core`'s test
  module gained one private helper (`severity_fixture`) with callers only inside that module.
- **Crates / modules:** none added · none removed · none changed.
- **Dependencies:** **none added, none bumped.** `Cargo.toml` and `Cargo.lock` are byte-untouched
  (`git diff --stat Cargo.lock Cargo.toml` empty) — zero package admission. The new harvest binary uses
  `conductor_run`'s existing surface plus `std` only.
- **Schema / config:** no migration, no config key, no violation-schema change. The five scenario TOMLs are
  declarative phase data; every one keeps its `name` / `p_ids` / `seed` / `slo_tier` / `jitter_ms` unchanged.
  All five now carry ZERO `[[expected]]` blocks (declare-only). Phase shapes changed as follows:
  - `incident-auto-resolution` — trigger/retrigger 8 → **12** identical exceptions (clears Pulse's
    Autonomous storm threshold of 10; 8 sat in the 5..10 dead band and formed nothing); NEW
    `dilute-error-rate` phase (150 plain OK spans / 150000ms); `sustain-10min` (600000ms/60) DROPPED;
    resolve wait 120000 → **165000ms** (120s window + 30s observer tick + margin).
  - `severity-tier-autonomous` — NEW 90-sample `converge-baseline` (plain) + spike 8 → 30 errors @100%.
  - `severity-tier-suggested` — NEW 70-sample `short-baseline` + spike 12 errors @40% → @100%.
  - `severity-tier-curious` — NEW 20-sample `minimal-baseline` + spike 10@10% → 6 errors @100%.
  - `ack-cooldown` — phases KEPT VERBATIM (drive documentation); only its `[[expected]]` block removed.
- **Spec-master edits:** none at authoring time (P2 owns any).
- **Counts / qualifiers moved:**
  - workspace nextest **673 → 693** (+20 = 16 new `severity_harvest` tests + 4 net in `conductor-core`).
  - `conductor-core` **256 → 260**; `conductor-run` **95 → 111**.
  - Declare-only scenario families **6 → 7** (the severity-lifecycle family joins fingerprint-storm,
    error-baseline-spike, latency-regression, restart-suppression, pii-scrub, connection).
  - `UNBACKED_AUTO` **unchanged** (P-031/033/034/039/041-044); every family P-ID stays `Auto` with a
    scenario naming it, so `check_sut_drift` and `check_scenario_backing` are both undisturbed.
  - Load-envelope `[[exempt]]` ledger **stays empty** — longest emitting phase is now 150000ms
    (`dilute-error-rate`), peak dispatch rate 1/s, both far inside `max_sustained_storm_ms` 600000 /
    `max_sustained_rate_spans_per_s` 10000. Dropping `sustain-10min` removed the one phase sitting exactly
    ON the storm ceiling.
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** the plan's Leg-A addendum — driving `mark_incident_resolved` to force an
  empty active list — was **written into the plan and deliberately not executed**: leg E reached an empty
  list naturally, which is what the addendum existed to force. `mark_incident_resolved` therefore still has
  **no production call site** and remains unexercised live.
- **Spec claims disproved by measurement:**
  1. **`triage.incident.auto_resolve.tick` cannot witness a resolution.** Its `resolved_count`,
     `evaluated_count` and `duration_ms` render as the literal string `"<redacted>"` on the wire (Pulse's
     observer field allowlist predates them). The tick proves the observer RAN, never that it resolved
     anything. The plan and the v2-16 acceptance both named it as the hard witness. Evidence:
     `evidence/leg-a/pulse-events.jsonl` (every tick line); pinned by
     `severity_harvest.rs::the_auto_resolve_tick_counters_are_redacted_on_the_wire`.
  2. **Read-back cannot witness the resolved incident's ABSENCE on leg A.** At read-back (09:06:13) the
     sidecar's `query_incident_list` returned **2** incidents while pulse-app's own
     `incidents.list_active` reported **1** (09:06:10). They agreed pre-resolution (both 1 at 09:00:07) and
     diverged only after the two auto-resolutions. Leg E DID corroborate emptiness through read-back
     (`result_count: 0`). Evidence: `evidence/leg-a/conductor.stderr` + `evidence/leg-{a,e}/pulse-events.jsonl`.
  3. **P-059's resolution summary is unreachable under deterministic L4** (research-established, now
     measured): ZERO lines naming `resolution_summary` / `ResolutionSummary` across all five legs. It
     attaches only on `DigestKind::ResolutionSummary` — which nothing at HEAD `efabe8e` constructs — or on
     an L4 output whose `is_resolution_summary` is true, which the canned fixture pins false.
  4. **A "fresh" data dir does NOT start at zero log lines.** `pulse-app` writes its own boot output before
     the OTLP receiver opens: **2234** lines on leg A (2264–2275 on B–E). `baseline_harvest.rs`'s
     `harvest_since` doc comment states "under a FRESH data dir 0 is correct by construction" — false. The
     pre-leg line count is load-bearing; a harvest assuming 0 mixes boot output into the leg window.
  5. **`CountAtLeast` was a mis-aimed instrument for this family.** It grades `evidence_count` = summed
     `retrieve_telemetry_slice.span_refs`, and Pulse's incident producer writes
     `evidence_refs.span_ids = Vec::new()`. Measured live: `retrieve_telemetry_slice` returned
     `result_count: 0` on every call, so the observed value was always "0" whatever the lifecycle did.
- **Coverage of new surfaces:** this chunk adds **no external surface, no hot-path op, and no UI element** —
  zero production code. The five scenario TOMLs are existing declarative config re-shaped:
  - `scenarios/*.toml` (config, not a surface) → validation **garde✓** (unchanged `#[garde(dive)]` path;
    all five re-validated by `severity_lifecycle_fixtures_load_and_validate`; no `#[garde(skip)]`
    introduced) · instrumentation **n/a** · PII **n/a** · tests **unit✓** (15 catalog guards) · a11y **n/a**
    · tokens **n/a**.
  - `crates/conductor-run/tests/severity_harvest.rs` (test-only) → validation **n/a** · instrumentation
    **n/a** · PII **redacted✓** (pinned lines are Pulse's own already-scrubbed output; committed evidence
    greps clean for drive-letter / `/c/Users` / `.cargo` / `.rustup`) · tests **unit✓ (16)** · a11y **n/a** ·
    tokens **n/a**.

## Deviations from intent

1. **Edited `crates/conductor-core/src/scenario.rs`, which research's Files-to-modify did not list.**
   Justification: it holds five catalog guard tests that DATA-PIN the very `[[expected]]` blocks plan steps
   2–4 retire (`assert!(!s.expected.is_empty())`, `Contains "Resolved"` Hard, `CountAtLeast` Hard, the
   mixed-class suite guard). Retiring the checks without moving the guards leaves an incoherent tree that
   cannot go green under either runner. `codebase-research.md`'s caller-threading note names this class
   ("tests that PIN the changed artifact's DATA") as a boundary member; the research list omitted it. The
   edit is confined to the `#[cfg(test)]` module — no production line — and the replacements are stronger,
   not weaker: they pin the declare-only state AND the drive shapes that make the legs reachable (baseline
   sample counts as tier selectors, ≥12 storm occurrences, the dilution tail, the ≥150s wait), so a future
   "simplification" that silently un-reaches the SUT fails a test.
2. **Ran the bundled `agent-run.sh run` gate once rather than once per leg.** The plan's Test Commands list
   the harness form per leg; the harness's no-flag `run` bundles nextest + doctest + clippy BEFORE the
   scenario, so five legs would have re-run the whole suite five times. The four remaining legs were driven
   through the same underlying binary invocation the harness executes
   (`conductor run <scenario> --agent-mode` with the identical env prefixes). Gate coverage is unchanged.
3. **The plan's Leg-A `mark_incident_resolved` addendum was not executed.** It existed to force an empty
   active list because the plan judged the `AutoResolved` arm structurally unreachable. Leg E reached the
   empty list naturally, proving the arm live without the intervention, so the addendum was unnecessary —
   and skipping it keeps `mark_incident_resolved` free of any production call site.
4. **`sustain-10min` was dropped from `incident-auto-resolution`** (the plan authorized the re-shape but
   named the phase only implicitly). Its P-059 interpretation-continuity target is ungradeable under
   deterministic L4, and it was the single catalog phase sitting exactly ON the load-envelope storm ceiling.

## Decisions & corrections

- **PREREQ discharged in PURE auto-satisfy form — the first such fire.** `cargo audit` true exit 1 with
  first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans licenses
  sources` true exit 0; `Cargo.lock` un-drifted and byte-untouched. Record: **probe unchanged, 35th
  consecutive.** Remedy stays the bounded wait — no floor raise, no `deny.toml` ignore, no CI edit.
- **The three tier bands are separated by CONFIDENCE, not magnitude — and that is now measured, not
  argued.** Legs B/C/D produced an IDENTICAL magnitude progression (3.330 → 6.549 → 9.661 …) because the
  spike shape and the EWMA math are the same; only the baseline sample count differed, and the band
  followed it. `confidence == persistence_seconds / 100` held exactly on every row. The baseline phase IS
  the tier selector, which is what the three files were re-shaped to be.
- **A harvest assertion initially graded the wrong incident, and the test caught it.** Pairing the
  scenario's creation with the FIRST active-count drop measures the preflight canary's own resolution
  (107629ms), not the scenario's (137625ms) — the count series carries no incident identity, and a leg
  always has the canary's incident open beside the scenario's. Fixed by asserting against the instant the
  set EMPTIES (attributable: every incident open before it is resolved by then), with the mis-pairing
  pinned as its own negative test.
- **Cue lines carry no service identity**, so cues are attributed by `persistence_seconds` (the emitting
  service's sample count). The canary's is pinned at 15 (3 warm-up + 12 storm) and, because the EWMA is
  sample-driven and the canary stops emitting, its cue FREEZES at exactly
  `magnitude 8.511516379456504 / confidence 0.15 / persistence 15` and repeats every second — byte-identical
  on legs A and B, which is what makes it a reliable discriminator.
- **The `AutoResolved` arm's live firing closes a prior chunk's honest limit by measurement.** 2026-08-20
  shipped it unit-pinned and unexercised because the canary's cues kept its incident alive; leg E shows the
  canary's incident auto-resolving on schedule (its cues are `curious`, and the Tier-1 coordinator accepts
  Autonomous alone, so nothing refreshed it), leaving 4 minutes of empty active list before read-back.
- **Evidence had to be filtered to be committable.** Leg A's raw Pulse slice was 138,065 lines / 48 MB, of
  which 128,820 were `metric.webgpu.frame_duration_ms`. Committed evidence is the six load-bearing targets
  only: 502 KB across five legs, zero host-path matches.

## SUT intake (Pulse-side — recorded, NOT scoped into Conductor)

Three items for the Pulse intake queue. Each is a measurement this chunk made about the SUT; none is
Conductor work, and none opens a Conductor route entry.

1. **NEW — the corpus-backed active set and the in-app ledger diverge after auto-resolve.** At leg A's
   read-back (09:06:13) the sidecar's `query_incident_list` returned **2** incidents while pulse-app's own
   `incidents.list_active` reported **1** at 09:06:10. They AGREED before the resolutions (both 1 at
   09:00:07) and diverged only after the two auto-resolutions, so at least one resolved incident's corpus
   row still reads active to the sidecar. Cause not established from logs alone — identifying it needs a
   direct `corpus.db` read, which is out of bounds for Conductor. Consequence for anyone building on
   read-back: **absence is not assertable through `query_incident_list` after an auto-resolve.**
   Evidence: `evidence/leg-a/conductor.stderr` + `evidence/leg-a/pulse-events.jsonl`.
2. **EXTENDS intake #7 (the obs allowlist predates its counters) — same class, new sites.**
   `triage.incident.auto_resolve.tick` renders ALL THREE fields — `resolved_count`, `evaluated_count`,
   `duration_ms` — as the literal string `"<redacted>"`. The tick therefore proves the observer RAN and can
   never prove it RESOLVED anything, which is what forced this chunk's auto-resolve witness onto
   `incidents.list_active.request` `item_count`. Same shape as the `triage.cue.tick`
   `cues_suppressed`/`bypass_triggered` finding recorded 2026-08-18.
   Evidence: `evidence/leg-a/pulse-events.jsonl`; pinned by
   `severity_harvest.rs::the_auto_resolve_tick_counters_are_redacted_on_the_wire`.
3. **NEW (ratified at phase P5) — the resolution summary is structurally unreachable under deterministic
   L4.** `attach_resolution_summary_to_incident` has exactly two triggers: `DigestKind::ResolutionSummary`,
   which NOTHING at HEAD `efabe8e` constructs (`triage/contract.rs:275` is the enum variant's sole
   declaration; every other reference is a match arm), and an L4 output whose `is_resolution_summary` is
   true, which the canned fixture pins false. Measured: ZERO lines naming `resolution_summary` /
   `ResolutionSummary` across all five legs. Sibling of intake #2's degraded-report half — the mechanism
   exists SUT-side and is unreachable in the mode every verifiable run uses.

## Outcome

**Acceptance criteria: met**, with two v2-16 sub-clauses refined at this wrap (see Spec claims disproved
1–2) under the ratified refine-with-evidence channel — the OUTCOME is proven strictly stronger than the
original wording.

Gates run (all green, zero retries, zero deferrals — every changed surface gated):
- `cargo nextest run --workspace --profile ci` → **693 passed**, 0 skipped
- `cargo test -p conductor-run` → 11 test binaries, all ok (runner-portability gate)
- `cargo clippy --workspace --all-targets -- -D warnings` → exit 0
- `cargo test --workspace --doc` → ok
- `bash scripts/agent-run.sh run` → the bundled release-gate path, exit 0
- `cargo audit` / `cargo deny check advisories bans licenses sources` → the PREREQ probe above

Smoke (boot-path/harness): `bash scripts/agent-run.sh status 2026-08-21T08-59-21-737` → returned THIS run's
envelope (mint-then-read, never a bare status verb), exit 0.

**Five live legs**, fresh data dir each, SUT HEAD `efabe8e`, all `conductor run` exits 0, all rows
`verdict: null` / `state: KnownResidual`:

| leg | scenario | latency_ms | slo_tier |
|---|---|---|---|
| A | incident-auto-resolution | 365827 | `<90s` |
| B | severity-tier-autonomous | 121307 | `<5s` |
| C | severity-tier-suggested | 82923 | `<20s` |
| D | severity-tier-curious | 26334 | `<90s` |
| E | ack-cooldown | 370001 | `<20s` |

Proven live: **P-022 auto-resolve** (scenario incident created 09:00:28.046, active set emptied
09:02:45.671 — **137.6s**, inside the 120s window + 30s tick; the canary's resolved at 128.6s on its own
schedule) · **new-not-reopen** (retrigger on the SAME fingerprint `12dcd67b` → `created=true, deduped=false`;
Pulse dedups against the active set only) · **lossless ingestion** (cumulative `span_count` **189** =
15 canary + 12 trigger + 150 dilution + 12 retrigger) · **the tier ladder** (autonomous / suggested /
curious on B / C / D) · **P-059's measured absence** · **the `AutoResolved` residual arm** (leg E).

Full evidence: `evidence/leg-verdict.md`.
