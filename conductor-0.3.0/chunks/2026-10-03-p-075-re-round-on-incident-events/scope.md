# Scope — 2026-10-03-p-075-re-round-on-incident-events

**Working entry (working-route.md:83):** The P-075 re-round on incident events — the round re-driven against Pulse with
its incident lifecycle events read back through MCP and graded.

**Version:** conductor-0.3.0 · Epoch 5 — Polish & ship. This is the REPLAN of the promotion rolled back at P5 on
2026-10-03 (`.andromeda/runs/2026-10-02T22-27-37-phase/cancel-record.md`; its scope and research are kept there as
`cancelled-scope.md` / `cancelled-research.md`). Taken up on the operator's relay (overseer, measured 2026-10-04), which
cleared the head's BLOCKED-ON. The Setup HALT was answered "take it up" by that relay ("This clears the P-075 re-round
BLOCKED-ON") plus the phase invocation that followed it.

## What this chunk builds
ONE fresh, deterministic-posture P-075 round against the Pulse build at **S2**. It grades the SEVEN assertions that S2's
`round-request.md` names, each hard at its measured value:
- the six of the prior round, unchanged;
- a seventh, new: the round's incident lifecycle events read back through Pulse's new MCP tool
  `retrieve_incident_events`, graded around `mark_incident_resolved` over the four-kind vocabulary.

It also pins `retrieve_incident_events` in Conductor's `contracts/mcp-contract.toml` (the CARRY). The round, its
scenarios and its evidence are Conductor's. Pulse names only the binary and the assertion set.

## BLOCKED-ON (folded — the dependency, and how it cleared)
- As written: `BLOCKED-ON: Pulse "incident events readable through MCP" — clears when Pulse relays a sha whose MCP tool
  surface returns incident lifecycle events (at Pulse 83d4060 the incident_events table reached no MCP tool, arch
  §Established Decisions [Read-Back Dependency Posture])`.
- The cancel record states the annotation no longer names the real dependency: the tool existed at S, and the round
  waited on S2's vocabulary fix. The rewrite of that wording is route-resolve's, at this chunk's wrap (flip-compaction),
  never a hand edit here.
- Cleared by the relay naming S2. Re-verified at phase time (2026-10-03T22:2xZ) against the Pulse checkout, not taken
  from the relay's wording:
  - `git -C andromeda-pulse rev-parse HEAD` = S2 on `chore/migrate-pulse-to-v3`; the LOCAL tracking ref
    `origin/chore/migrate-pulse-to-v3` = S2 (no fetch at phase time; the relay states origin = S2);
  - S2's parent is S (`git log S..S2` = the one commit `cdb6c1e`);
  - at S2 `coerce_event_kind` admits exactly `triage::contract::incident_event_kinds()` = `created` / `active` /
    `acknowledged` / `resolved` (`crates/mcp-server/src/tools.rs`, `crates/triage/src/contract.rs`), and the producer
    writes `INCIDENT_EVENT_CREATED` from that same module (`pulse-app/src/inference_runtime.rs:922`). So the S-time
    finding — the producer's `created` read back as `unknown` — is fixed at S2 by construction, pinned by S2's
    `retrieve_incident_events_reads_the_producers_created_event_as_created` and
    `incident_events_vocabulary_is_created_plus_each_status_label`.

## The binary under test (folded from the relay + re-measured)
- **S2 = `cdb6c1ed572761ae384597a7ed437222e3a1d1fc`** on Pulse's `chore/migrate-pulse-to-v3`, parent `4a26ad8` (S).
  Subject: `chore(2026-10-02-incident-events-readable-through-mcp): operator pre-CI commit, for the run this chunk's
  verdict reads`.
- **A correction to the relay's wording:** the relay said `round-request.md` and `round-binary.md` are COMMITTED at S2.
  Both files are in S2's tree, and `round-request.md` at S2 is the S2 version. But `round-binary.md` AT S2 still records
  S (the Windows `.exe` artifacts). The S2 binary record — the Linux build and its two sha256 values — is an
  UNCOMMITTED working-copy change in Pulse's tree (`git status`: ` M …/evidence/round-binary.md`, the only change). So
  the S2 artifact facts below are read from Pulse's working copy, never from S2. This is the same posture the cancelled
  scope took for S.
- Build (Pulse's, already done): `cargo build --workspace --release --features mcp-server` on Omarchy Linux
  (`x86_64-unknown-linux-gnu`), after the native `npm` stage rebuilt `ui/dist` (the bundle `index-DmVOp_0t.js`), per that
  working-copy record.
- Artifact sha256 values re-measured at phase time (2026-10-03T22:21Z) — both match the working-copy record:
  - `target/release/pulse-app`: `23f6ef2bd0b854d9697a4d203b3aea1bf6f40b146420de362112556914976ada`
  - `target/release/andromeda-pulse-mcp`: `e64f3688ec14144910ab25d461ec4ea99c161fda2e1b8824a3d480f17021e02e`
- Corroborated by the overseer's CORRECTION relay (measured 2026-10-04): the committed `round-binary.md` at S2 is stale
  (names S and the `.exe` values); the working copy holds the right values, matched by the overseer's own `sha256sum` of
  the real binaries — the same two values above. Directive: plan the round against these, not the committed file.
  Pulse CI on S2: 11/12 green, supply-chain red external.
- **The binary under test is a LINUX build.** No Windows artifact exists for S2 (`target/release/*.exe` absent).
- The prior round's operator directives carry forward as the plan's default: re-measure both sha256 values
  immediately before launch (a mismatch halts, never rebuilds), and no Pulse rebuild. This is a posture, not a code
  premise. Its basis is the precedent at `2026-10-02-p-075-assert-round-against-pulse/evidence/round-ledger.md`. The
  operator confirms or replaces it at the P5 review.

## Host (operator directive, 2026-10-04 — a FOUNDER decision at P4)
- This session runs on a Linux host (Omarchy, `Linux 7.2.5`). The project's standing record says Windows-only (CLAUDE.md
  2026-08-22 learning; `.claude/rules/host-win32.md` is always loaded).
- **How the live round runs — on this Linux host, or on a CI runner — is the FOUNDER's decision, surfaced at P4 as a
  fork and never decided by this phase** (the operator's relay, verbatim intent: "surface it at P4, never decide it
  alone").
- **RULED at P4 (2026-10-04):** the FOUNDER's live word, relayed by the overseer: "the round runs on this Linux host
  against the relayed S2 binaries (pulse-app 23f6ef2b…6ada, andromeda-pulse-mcp e64f3688…e02e)". A missing UI sample
  grades UNGRADED and is never re-driven. The CI-runner and hold-for-Windows options were offered and not taken.
- A fact the fork needs: Conductor's CI is Windows-only (`ci.yml:18,245` `windows-latest`, `:289` `windows-2022` —
  re-read at P3), and the relayed binary is a Linux build. So a CI-runner leg would have to build its own Pulse at S2
  (different artifacts, different sha256) rather than run the relayed binary.
- The live leg's committed firing form names a Windows MSYS `PATH` (`crates/conductor-run/tests/p075_round_live.rs:13`).
  On Linux it becomes the Pulse checkout's `target/release`. `andromeda-pulse-mcp` is NOT on this host's inherited `PATH`
  (`which` → absent), so the round's `PATH` must resolve it to S2's sidecar. The shipped spawn and `PATH` resolution need
  no code change on Linux (`spawn.rs:67-78,99-100,144-159`). P3's inventory of the prior recipe's Windows-only elements
  is in research.md §Conventions to follow.
- The "a Windows teardown by `Stop-Process -Force` leaves no `app.exit` record" note is Windows-shaped. The Linux
  teardown form and whether it writes `app.exit` are UNMEASURED: no record shows `pulse-app` ever launched on Linux —
  Pulse's S2 operator pass on this host records "no pulse-app process was launched" (research.md §Pulse at S2). The same
  holds for every UI-timed leaf behind assertions 3–6.
- This checkout has no `target/`, no `crates/conductor-tauri/ui/{node_modules,dist}` and no `cargo-audit`: the first
  cargo build here is cold, every `--workspace` cargo gate needs the frontend bundle built first (`ensure_frontend`
  precedent), and the supply-chain gate needs `cargo-audit` installed (research.md §Host facts).

## Pulse CI on S2 (relay; not this repo's red)
- The relay states Pulse CI on S2 is red on supply-chain only: external, owned by its own Pulse entry. On S it was the
  three `wasmtime 48.0.3` advisories (RUSTSEC-2026-0325 / -0326 / -0327). S2's own CI record lives in Pulse's
  `operator-pass.md` §Gate 21 (S2). It is cited, not re-derived; it does not touch the assertions.
- Conductor's `Cargo.lock` carries no `wasmtime` package (`grep -c 'name = "wasmtime'` → 0, at phase time), so those
  advisories do not reach this repo's supply-chain gate.

## Mode (round-request §Mode)
- ONE round, deterministic: `ANDROMEDA_PULSE_L4_DETERMINISTIC` set on the app child, and a fresh data dir.
- `pulse-app` and `andromeda-pulse-mcp` come from S2's `target/release`. Conductor spawns the sidecar by fixed NAME through
  the inherited `PATH`, so the round's `PATH` must resolve `andromeda-pulse-mcp` to S2's sidecar.
- Ports 4317/4318 are shared with Pulse's tree, so the live leg runs only in the operator's slot.

## The seven graded assertions (round-request.md at S2, §The seven graded assertions)
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
   - read through `retrieve_incident_events` BEFORE `mark_incident_resolved`: the FIRST event is `created`, and the
     response carries NO `resolved` event;
   - read again AFTER it: the FIRST event is still `created`, the LAST event is `resolved`, and its
     `occurred_unix_nano` lies inside the resolve call's wall-clock window (request sent → response received);
   - every `event_kind` in both reads is one of `created` / `active` / `acknowledged` / `resolved`, and no event reads
     `unknown`.

   Between the first and the last event the sequence is not graded.

## The new tool, as Pulse states it (round-request §What is new at S2 — re-verified at P3 against S2's source)
- `tools/list` carries nine tools: the prior eight plus `retrieve_incident_events`.
- Input `{incident_id: integer}` — required; no other property accepted.
- Response `{"incident_id", "events": [{"event_kind", "occurred_unix_nano"}], "total", "truncated"}`, oldest first, at
  most 256 (`truncated` true when more exist).
- Vocabulary closed: `created` (written when the interpretation path opens the incident) then one row per status VALUE
  change; any other stored value reads `unknown`.
- An unknown id → JSON-RPC error -32603, message containing `incident not found`.
- The resolve stamp, RE-READ at S2 (`tools.rs:451-481,567-568`; `contract.rs:645-693`): the sidecar's
  `mark_incident_resolved` stamps `now` (host clock) INSIDE the call and, in ONE transaction, appends `resolved` at
  `occurred_unix_nano = now` only when the prior status differs; a stale guard declines with a JSON-RPC error. An
  incident already auto-resolved before the call gains no new event. A deterministic re-emission inside ≤ 60 s before
  the app's reconcile can append `active` AFTER `resolved` (an S-time reading; its files did not move S→S2), so the
  AFTER read follows the resolve immediately.

## Decisions carried from the cancelled P4 (cancel-record.md §The P4 answers that stand for the replan)
- **Q1, tool pin:** pin `retrieve_incident_events` as a fifth required MCP tool, in `READBACK_TOOLS`,
  `contracts/mcp-contract.toml` and the in-process stub's default list. This is the founder's LIVE ratification of the
  playbook's escalate pattern "Boundary widening" (a new input class on the sidecar-stdout boundary), per the overseer's
  correction message; the wrap that lands the pin records it as ratified on that word.
- **Q3, window stamp:** an additive timed sibling of `probe_resolve_lifecycle` in `crates/conductor-run/src/lifecycle.rs`,
  returning the observation plus the resolve call's `std::time` epoch-nanosecond request→response window. The existing
  probe delegates to it, and no `LifecycleObservation` struct-literal site threads.
- **Q2 (the predicted FAIL):** superseded by the hold; this round grades against S2.

## CONTEXT (folded)
- The founder ruling of 2026-10-02 (live, relayed by the overseer; route-archive.md:119): everything planned for 0.3.0
  is done now, properly; nothing is carried over. This entry is 2 of the 3 the ruling minted ahead of the version close.
- The first round graded six assertions PASS at Pulse S `03ec944` (`2026-10-02-p-075-assert-round-against-pulse`). This
  round is FRESH: P-075 is re-verified on this chunk's binary, not inherited from that one.

## CARRY (folded)
- From the `2026-10-02-captured-fingerprint-values-elided` wrap (overseer relay 2026-10-02): Pulse's round request
  carries 7 assertions and needs Pulse's new (9th, by the overseer's count) MCP tool `retrieve_incident_events` pinned in
  Conductor's `contracts/mcp-contract.toml`. `required_tools` holds 4 at phase time (re-read):
  `query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`.
- `required_tools` is what the preflight's missing-tool precondition gates on, so pinning the fifth name makes
  every preflight against a Pulse lacking the tool `Blocked` on that EXISTING precondition; the named-precondition count
  stays five (`preflight.rs:171-226`, re-read at P3).
- The four-name set lives in THREE places (re-derived at P3): the committed manifest; `READBACK_TOOLS: [&str; 4]`
  (`crates/conductor-verify/src/manifest.rs:17`, re-read at phase time — `validate()` requires the manifest to be its
  superset, and the child stub's `tools/list` is built from it, `stub_pulse_mcp.rs:7,52`); and the in-process stub's
  literal default list (`tests/common/mod.rs`), which `readback_shape_witness.rs` runs against the committed manifest. A
  manifest-only pin would turn that witness (and `preflight_spawn`) red on tool absence.

## What the round needs that is not shipped (hypotheses for P3)
- The prior round's machinery is reusable at S2: the `live-pulse`-gated `p075_round_live` leg; the
  `lifecycle_harvest` / `delegated_timing_harvest` graders (`RoundGrade` synthetic-arm builders, per-round evidence
  consts); `probe_resolve_lifecycle` + `attribute_by_liveness`; the `evidence_pin` module. Pulse's source moved
  between `03ec944` and S2 only in `crates/corpus`, `crates/mcp-server`, `crates/triage/src/contract.rs` (additive) and
  one const in `pulse-app/src/inference_runtime.rs`, with nothing under `ui/` and no telemetry leaf (`git diff --stat
  03ec944 S2 -- crates pulse-app ui`, 9 files) — so the readings behind assertions 1–6 and the P-025 contract carry
  (re-derived at P3).
- Assertion 7 is new at every tier: no code calls `retrieve_incident_events` (`grep -rn --include=*.rs
  retrieve_incident_events crates/` → 0, re-run at P3); `probe_resolve_lifecycle` stamps no time (`lifecycle.rs:114-129`).
- This round's evidence is its own: each grader pins a per-file constant (`lifecycle_harvest.rs:446`,
  `delegated_timing_harvest.rs:378`); the prior six ids stay bound to the prior chunk's files, and this round's ids are new.
- `ANDROMEDA_PULSE_DATA_DIR` reaches the leg only through `ReadbackClient::connect` (`p075_round_live.rs:102-105`), so
  the leg is not a `capture_paths` reader.

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
  This chunk links no capability (`matrix.py coverage` at session start: verified 10/11 · deferred 1 · unclaimed 0).
- The host decision is the founder's (§Host); this chunk does not re-scope the project's host record.

## CI verdict read at Setup (fold source 2)
- `6a9ff7c` (the last wrap's flip = HEAD): **green**, checks 3/3, wall 694 s, CI#37037859266. Nothing to fold as a red.
