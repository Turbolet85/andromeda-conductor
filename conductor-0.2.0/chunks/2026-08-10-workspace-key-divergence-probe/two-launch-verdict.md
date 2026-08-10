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
