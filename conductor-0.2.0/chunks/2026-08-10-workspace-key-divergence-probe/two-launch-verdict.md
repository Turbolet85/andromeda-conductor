# Two-launch verdict — the workspace-key divergence probe

Recorded 2026-08-10 by chunk `2026-08-10-workspace-key-divergence-probe` (the version intent's *first
action*, `intent.md` §Theme 3 F10; the acceptance half of `verification-matrix.json#v2-17`).

Arms are named by **role**, never by absolute path.

## Setup common to all arms

- `pulse-app` launched from a debug build, once per arm, with a different working directory.
- `ANDROMEDA_PULSE_DATA_DIR` shared by `pulse-app` and the sidecar Conductor spawns.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` — **confirmed active in all three arms** (Pulse logged
  "L4 deterministic mode active … canned output, no model" once per launch). The LLM is therefore
  removed as an explanation.
- `CONDUCTOR_PREFLIGHT_TIMEOUT=90` (90 one-second canary poll attempts).
- Instrument: `conductor preflight --json`.

## Results

| Arm | cwd role | `ready` | exit | `blocked_precondition` |
|---|---|---|---|---|
| 1 | the Conductor repo root | `false` | 1 | the workspace-key precondition |
| 2 | a freshly created marker-less temp dir | `false` | 1 | the workspace-key precondition |
| 3 | the data dir itself | `false` | 1 | the workspace-key precondition |

Identical in every arm: `negotiated_protocol_version: "2024-11-05"`, all four required tools
`present`, `canary_round_trip: "failed"`, and `data_dir: "<redacted>"` — the value scrub masked the
host path in the readiness envelope, as the hygiene invariant requires.

The precondition returned in all three arms was the new named one:

> pulse-app and the spawned MCP sidecar must resolve the same incident workspace key — the sidecar
> keys on ANDROMEDA_PULSE_DATA_DIR, pulse-app on its detected workspace root — or Pulse raised no
> incident for the canary

**No arm reached `ready:true`.**

## Verdict: the probe did NOT confirm the divergence — a prior blocker masks it

This is the finding, and it is not the one the intent anticipated.

Pulse's own telemetry during the probe window shows the canary **was ingested** (`services_tracked: 1`,
`operations_tracked: 1`) but that the L2 cue **never fired**: every `triage.cue.tick` heartbeat reports
`cues_emitted: 0` with `cues_evaluated: 3`, and the service sat in baseline bootstrap the whole time
(`services_in_bootstrap: 1`, `services_ready: 0`).

No cue means no L3 digest, which means L4 was never reached — so **no incident was created under any
workspace key, in any arm**. The zero rows `query_incident_list` returned are fully explained by the
*second* cause the precondition names. The workspace-key axis was never exercised, so the probe cannot
discriminate between the two causes and does not decide whether a Pulse-side fix gates the live legs.

That is the precondition working exactly as designed rather than failing: a diverged key and an empty
corpus are byte-identical on the wire, the string names both causes, and this run turned out to be the
no-incident one.

## What each arm did and did not settle

- **Arms 1 and 2 blocked, as predicted** — but for the no-incident reason, so they do not corroborate
  the key divergence.
- **Arm 3 leaves the `\\?\` question open.** The Windows verbatim-prefix hypothesis (Pulse canonicalizes
  its cwd while the sidecar uses the raw `data_dir` string) was the reason arm 3 might have blocked even
  with matching directories. It blocked, but for the no-incident reason, so the canonicalization question
  is **unresolved** and must not be reported as answered.
- **The intent's falsified premise stands, independently of this probe.** That a marker-less temp dir
  makes `pulse-app` fall back to `data_dir` is disproved by reading `workspace_detector::detect`
  (the marker and VCS are fields on the returned context, not success conditions; it errors only on a
  traversal component or a non-existent path). That conclusion comes from the source, not from these
  three runs, and is recorded in `verification-matrix.json#v2-17` `notes`.

## Consequence for the route

Deciding the key question needs an incident to exist first — Pulse must get past baseline bootstrap and
emit an L2 RetryStorm cue for the canary's fingerprint. Establishing and asserting those launch
conditions is the **Pulse run contract** entry (`intent.md` F11 / P-073), the next Epoch-2 chunk. The
divergence probe should be re-run once that entry can guarantee an incident forms; only then can arm 3
answer the canonicalization question, and only then can the two-launch check decide whether a Pulse-side
chunk gates the live legs.

Recorded honestly rather than resolved: the deliverable was to run the probe and record what it showed,
and what it showed is that a prerequisite blocker sits in front of the one being probed.

---

# Re-run — 2026-08-13

Recorded by chunk `2026-08-13-first-live-green-preflight`, the entry the section above named as its
condition of resolution ("re-run once that entry can guarantee an incident forms"). **It cannot.** The
mechanism meant to guarantee it is falsified below, with its cause identified.

Arms are named by **role**, never by absolute path.

## Setup common to all arms

- `pulse-app` launched from a debug build, once per arm, with a different working directory; killed
  between arms. Working directory confirmed per arm by the Specta/TauRPC binding tree it writes
  relative to its own cwd.
- The MCP sidecar built and resolvable on PATH — verified BEFORE arm 1 rather than assumed, since an
  unreachable sidecar makes every arm measure the read-back-unreachable path and yields evidence that
  looks like a result and is not (the failure the two preceding chunks each hit).
- `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` — **confirmed active**: Pulse logged "L4 deterministic mode
  active (ANDROMEDA_PULSE_L4_DETERMINISTIC); canned output, no model". The LLM is removed as an
  explanation, as in the 2026-08-10 run.
- `ANDROMEDA_PULSE_DATA_DIR` pinned explicitly to the platform default on BOTH sides, so the data-dir
  axis is fixed and only the workspace-key axis varies across arms.
- `ANDROMEDA_PULSE_MCP_ENABLED=true` — **a fifth recipe item the recorded operator recipe omits** (it
  lists four). Without it the sidecar starts and immediately exits with `MCP sidecar disabled (env var
  unset or not truthy)`, `feature_flag_enabled: true`, `env_var_mcp_enabled: false`; `spawn.rs` sets
  only the data dir via `.env(...)`, so this flag must be in Conductor's own environment to be
  inherited. The first attempt at arm 1 hit exactly this and was discarded. The readiness gate named
  the cause correctly in its precondition string, which is the precondition design working.
- Poll budget: the harness's derived `boot` budget (warm-up + poll floor + margin = 165s). **This
  re-run could not have happened without it** — the previous 30s wrapper killed the run before the 45s
  warm-up finished, let alone the 90s poll.
- Instrument: `conductor preflight --json`, run from the Conductor repo root in every arm (all four
  `contracts/*.toml` handles resolve relative to Conductor's cwd, so running Conductor itself from an
  arm's cwd is a harness fault at contract load, not a measured arm).

## Results

| Arm | cwd role | `ready` | exit | elapsed | `blocked_precondition` | which cause |
|---|---|---|---|---|---|---|
| 1 | the Conductor repo root | `false` | 1 | 135s | the workspace-key precondition | no incident |
| 2 | a marker-less temp dir | `false` | 1 | 135s | the workspace-key precondition | no incident |
| 3 | the data dir itself | `false` | 1 | 135s | the workspace-key precondition | no incident |

Identical in every arm: `negotiated_protocol_version: "2024-11-05"`, all four required tools
`present`, `canary_round_trip: "failed"`, `data_dir: "<redacted>"`.

**No arm reached `ready:true`. No arm exercised the workspace-key axis.**

## Verdict: the warm-up mechanism is falsified, and this time the cause is identified

The 2026-08-10 run recorded the symptom — `cues_emitted: 0` with the service in baseline bootstrap.
`[incident_formation]` was written to fix it. It does not, and cannot.

Per-arm Pulse telemetry, this run only: `cues_emitted: 0` across every sample (148 / 145 in arms 2 / 3),
`services_ready: 0` across every sample, `services_in_bootstrap: 1` on all but the two pre-ingest
samples, `services_tracked: 1` — the canary IS ingested and tracked. Zero incidents created in any arm.

The cause, from Pulse's source:

- `crates/triage/src/baseline/activity_floor.rs:33` — `BOOTSTRAP_WINDOW_SECONDS: u64 = 3_600`.
- `activity_floor.rs:173-183` — `bootstrap_state` is `now - first_observed_unix_nanos` versus that
  window, per service, derived purely from the internal clock.
- `crates/triage/src/cue/evaluate.rs:164` — `if snapshot.bootstrap_state != BootstrapState::Ready`
  the cue evaluator does not evaluate the service.
- The bootstrap anchor does not survive a restart. The `baseline_state` table exists in the corpus
  (`id, service_name, operation, snapshot_unix_nano, payload`) but holds **0 rows**, and
  `{data_dir}/triage/` is empty — so the mechanism is present and unused, and each `pulse-app` launch
  starts every service's `first_observed_unix_nanos` from zero.

The canary emits under service name `conductor`, which Pulse's `service_registry` has known since
2026-06-27 — but the registry is service-lifecycle state, not the ActivityFloor anchor, and that entry
is still `bootstrapping`. Because the anchor resets per launch, each run begins a fresh 3,600-second
window regardless of the service's age. The contract's `warmup_ms = 45000` is **45 seconds against a
3,600-second gate — short by a factor of 80**. This is not a mistuned parameter: no pre-roll measured
in seconds can clear an hour-long per-service gate, so the mechanism cannot deliver its stated goal by
any value of its own knobs.

**Correction to the chain, from the corpus rather than inference.** L3 digests are *not* blocked by the
missing cue: `digest_archive` holds 12,661 rows and grew by three during arm 3 alone, so the digest
cadence runs on its timer regardless. What is absent is the cue that would make a digest actionable,
and therefore the incident — not the digest itself.

**A stronger statement than the arms alone could make.** `incidents` and `incident_events` both hold
**0 rows in total** — not zero under some workspace key, zero absolutely. So the empty read-back in all
three arms is genuine emptiness, definitively *not* an artifact of workspace-key filtering. The key
divergence remains real (derived below) but is conclusively **not** the cause of what these three arms
observed.

**Consequence for the key question:** since no incident forms under any cwd, the workspace-key axis is
unexercised for the second time, and arm 3's canonicalization question is not answered by measurement.
The three arms do establish, experimentally, that the blocker is **cwd-invariant** — three distinct
working directories, one identical block reason — which places it upstream of the key axis rather than
inside it.

## `storms_detected_total: 0` — a SEPARATE fact with a separate cause

The hour-gate story above explains `cues_emitted: 0` and `services_ready: 0`. It does **not** explain
`storms_detected_total: 0`, and must not be read as covering it. The `RetryStormDetector` is fed at the
**buffer boundary** via the buffer-side `FingerprintObserver` hot path (`pulse-app/src/main.rs:530-564`),
and neither `crates/triage/src/cue/retry_storm.rs` nor `crates/buffer/` references `BootstrapState` —
so storm detection is reach-independent of the gate that stopped the cue evaluator.

What the evidence establishes:

- **The detector received nothing to count.** `tracked_fingerprints_count: 0`, `fingerprint_count: 0`,
  `fingerprints_evicted_total: 0` across every sample. The fingerprint table is empty — not populated
  with scattered identities.
- **The spans did arrive.** Conductor's own trail shows nine `emit.batch` spans closing without error
  (three warm-up + six storm, per `warmup_emissions = 3` and `CANARY_STORM_COUNT = 6`); a failed export
  would have returned `Err` into the canary-emission Blocked state instead. Pulse's log independently
  reports `span_count: 9`.
- **The attribute contract matches exactly.** Conductor emits an event named `exception` carrying
  `exception.type` / `exception.message` / `exception.stacktrace`
  (`conductor-emit/src/exception.rs:156-160`); Pulse extracts those three keys by name
  (`crates/buffer/src/appender.rs:346-350`) and `compute_exception_fingerprint` returns `None` only when
  `exception_type` is absent or empty (`crates/buffer/src/fingerprint.rs:71-79`).

**Two of the three candidate causes are therefore ruled out.** Fingerprints did not scatter across the
six occurrences, and the detector did not miss a qualifying group — it never received a fingerprint at
all. **No Conductor-side fingerprint-derivation defect is evidenced**: the architecture's claim that the
canary fingerprint is "computed to match Pulse's derivation" holds at the attribute-key level, which is
where a mismatch would have shown.

**What remains open, and why it could not be settled post-mortem.** The remaining discriminator is
whether the six exception spans reached the appender with their events intact. That read was expected to
come from the corpus, but **the received spans do not persist there**: `corpus.db` carries
`baseline_state`, `digest_archive`, `incident_events`, `incidents`, `pipeline_metrics` and
`service_registry` — no span or exception table at any point. Raw spans live in the in-process buffer,
which is why the four live-buffer MCP tools return empty cross-process (already recorded in
architecture §Standard Contracts), and which is gone once `pulse-app` stops. Settling this needs a live
capture while Pulse is still running — an instrumented re-run, not a post-mortem query.

## The `\\?\` question, answered from source rather than measured

Recorded with its epistemic status explicit: **derived, not observed.** No incident forms, so neither
key is ever written anywhere Conductor or this probe could read.

- `crates/workspace-detector/src/detect.rs:31` — `detect()` calls `candidate_root.canonicalize()`.
- `pulse-app/src/digest_runtime.rs:109-127` — `resolve_workspace_for_incidents` returns
  `ctx.root.to_string_lossy()` when detection succeeds: the app's key IS that canonicalized path.
- `crates/conductor-verify/src/spawn.rs:81` — the sidecar receives the **raw**
  `ANDROMEDA_PULSE_DATA_DIR` string, uncanonicalized.

`std::fs::canonicalize` on Windows returns the verbatim (`\\?\`) prefix. So even in arm 3 — the one cwd
position where the two keys could agree — the app's key is `\\?\<data-dir>` while the sidecar's is
`<data-dir>`. **They are not byte-equal, and no launch position makes them so on this platform.** The
byte-equality invariant asserted in `resolve_workspace_for_incidents`'s own doc comment holds between
the app's two halves, not between the app and the sidecar.

Independently re-confirmed live: Pulse logs no workspace field at all (no `workspace`/`_root`/`detected`
key in its telemetry across a full arm), and `pulse-app/src/` carries no logging call site for
`workspace_root` or `resolve_workspace_for_incidents` — so the app-side key remains structurally
unobservable, as `verification-matrix.json#v2-17` recorded from source reading alone.

## Read-back shape — the key-diff

First live witness of a Pulse read-back shape. Identical in all three arms:

> `read-back shape: query_incident_list returned keys [items, next_cursor, total]`

This matches the committed baseline in
`conductor-verify/tests/readback.rs::the_read_back_key_sets_are_pinned_as_the_live_diff_baseline`
**exactly**. The load-bearing reader `incident_ids` reads `items`, which is present — so the empty
result is a genuinely empty corpus, not a silent field-name mismatch, and the measured verdict can be
trusted. `retrieve_report` and `retrieve_telemetry_slice` were never reached (both require a non-empty
corpus), so their live shapes remain unwitnessed and their readers stub-proven only.

## Consequence for the route

Deciding the key question still needs an incident to exist, and now the requirement is quantified: the
canary's service must be under continuous observation for a full hour before a storm can raise a cue,
with no restart in between. The only path that satisfies this today is a single `pulse-app` kept
running for over an hour with the canary service emitting throughout — which permits **one arm per
hour**, since the next arm's cwd change requires a restart that resets the clock.

The fix-scope is now **three named Pulse-side pieces plus one open Conductor-side question**, kept
separable because the next route decision needs them costed independently:

1. **Pulse — bootstrap reachability.** Either a test-mode override of `BOOTSTRAP_WINDOW_SECONDS`, or
   populating the already-present `baseline_state` table so the anchor survives a restart. Either one
   alone makes a live incident reachable within a preflight budget.
2. **Pulse — workspace-key alignment.** The app canonicalizes (`\\?\` verbatim prefix on Windows) while
   the sidecar uses the raw data-dir string; they cannot agree from any launch position. Independent of
   (1), and invisible until (1) lands.
3. **Pulse — nothing else is assumed.** Ingest, the digest cadence, deterministic L4, the tool surface
   and protocol negotiation were all observed working; no other Pulse-side defect is implied by this
   run and none should be inferred from it.
4. **Conductor — open, pending the fingerprint answer.** Whether the six exception spans reach the
   appender with events intact is unresolved (see the storm section). If they do not, the defect is
   Conductor-side and is its own chunk; if they do, the question moves to Pulse. **No Conductor defect is
   evidenced today** — the attribute contract matches — so this is a question to settle, not a known bug.

**The operator recipe is five items, not four.** The recorded four (sidecar built and on PATH · both
env terms exported in the launching shell · poll budget at or above the contract floor · `pulse-app`
launched from a cwd outside the Conductor repo) omit **`ANDROMEDA_PULSE_MCP_ENABLED=true`, which must be
set in Conductor's own environment** so the spawned sidecar inherits it — `spawn.rs` passes only the data
dir via `.env(...)`. Without it every arm silently measures the read-back-unreachable path. This must
travel with the live-leg follow-up, not live only in this document. It also raises a design question for
the operator, deliberately left undecided here: `spawn.rs` could set that flag itself the way it already
sets the data dir, which would make the sidecar's own gate unmissable — but it would also mean Conductor
enabling a Pulse-side feature flag on its behalf, which the architecture currently asserts it does not do.

Recorded honestly rather than resolved, again: what the re-run showed is that the fix written for the
2026-08-10 blocker does not address it, and the reason is a constant three orders of magnitude away
from the one the contract assumed.
