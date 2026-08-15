# Route adaptation — 0-pending wrap (operator-requested)

**Date:** 2026-08-15 · **Version:** conductor-0.2.0 · **Branch:** build/conductor-0.2.0
**Path:** Setup §6 no-op (0 pending) + P5 route-resolve §Operator-requested adaptation
**Phases run:** P3 curation · P5 route-resolve · P6 state+handoff · commit. No P1/P2/P4/P7 (no chunk).

## Trigger

Operator session-start directive: Pulse is settled and proven across four chunks since the switch
(fingerprint feed healthy with permanent counters · F10 observed then fixed · corpus key custody restored ·
incident path measured healthy end to end). The one remaining preflight-green blocker is Conductor's own
canary constant. Request: insert ONE entry, then phase it.

## Premise verification (before the edit)

Every dictated premise was checked against the artifact that holds it; both repos are on disk.

| Premise | Verdict | Source |
|---|---|---|
| `CANARY_STORM_COUNT = 6` | confirmed | `crates/conductor-run/src/lib.rs:111` |
| Suggested 5 / Autonomous 10 | confirmed, **path corrected** | `crates/triage/src/pattern/storm.rs:73`, `:78` (dictated as bare `triage storm.rs`) |
| Tier-1 accepts Autonomous only | confirmed, **path corrected** | `crates/triage/src/cadence/coordinator.rs:390` (dictated as `crates/triage/src/coordinator.rs`); Pulse's own test at `:861-864` |
| threshold comparison | **sharpened** — `count >= autonomous_threshold`, so 10 suffices, not 11 | `pattern/storm.rs:245` |
| detection window | **added** — `DEFAULT_DETECTION_SUB_WINDOW_SECONDS = 30`, unstated in the brief | `pattern/storm.rs:69` |
| fingerprint-storm scenario two-phase 6 -> 12, does not move | confirmed | `scenarios/fingerprint-storm.toml` |
| `architecture.md:174` false workspace-column belief | confirmed at exactly that line | `.andromeda/architecture.md:174` |
| constant's doc comment carries the insufficient rationale | confirmed (a fix-scope addition) | `crates/conductor-run/src/lib.rs:109-110` |

Three corrections reached the annotation before it froze: two module paths, one comparison operator
(which moved the plan's margin floor by one), and one bounding constraint the brief never mentioned.

## Edit applied (markerless tail only)

1. **Inserted** as the FIRST markerless entry under `### Epoch 3 — Live proof: the five families`:
   `Canary storm inside Pulse's Autonomous band — the preflight canary forms a Tier-1 incident and
   preflight reaches ready:true against a live Pulse` (22 words, WHAT-not-HOW, no implementation verbs),
   carrying one CARRY (measured Pulse-side facts with corrected citations; fix shape with the margin left
   to the plan; the scenario-does-not-move guard; the `architecture.md:174` expected amendment; the two
   verdict-doc closures; `v2-10` claimability) and the PREREQ below.
2. **Moved** the audit PREREQ off `Fault-application spans` onto the new first entry, per
   §Operator-requested adaptation (an insertion ahead of the previous first markerless entry re-pins its
   next-entry PREREQs, origins preserved). Landed in the §Deferred-gate closure COMPACT standing form —
   18th pin, origin `2026-08-08-sut-capability-manifest`, ratified at the
   `2026-08-10-workspace-key-divergence-probe` wrap.
   **Basis re-verified at this wrap rather than echoed:** `cargo audit` re-run -> byte-identical
   `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 (advisory-DATABASE fault, nothing to raise a
   floor to); `Cargo.lock` zero lines vs HEAD; named overlap `cargo deny check` true exit 0
   (advisories/bans/licenses/sources all ok). No basis change -> compact form stands.
3. `Fault-application spans` keeps its `scheduler.rs:69` doc-comment CARRY unchanged.

**No BLOCKED-ON annotation** anywhere (operator directive: the dissolved block was deliberately never
pinned, avoiding write-then-clear churn). **No frozen `[{marker}]` line touched** — verified by diff.

## Gradient

Trajectory (new chunk ahead) -> HALT + dialogue -> operator approved as proposed, including placement,
title, CARRY content and the PREREQ move. Setup dirt-check also halted once on an untracked
evolve-diagnose run dir outside the named bookkeeping set; operator ruled it audit-trail class, absorbed
into this commit (recorded as a Tier-3 learning so the next 0-pending wrap does not re-ask).

## Not done here

Master-route untouched (0 pending — nothing to flip; promotion is `/andromeda-phase`'s).
Verification matrix untouched — `v2-10` stays pooled at `chunk:null`; its claim decision belongs to the
new entry's phase P5, never to this wrap.
