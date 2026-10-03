# P-075 re-round ledger — 2026-10-03

The one deterministic round against Pulse S2 = `cdb6c1ed572761ae384597a7ed437222e3a1d1fc` that grades the SEVEN
assertions of S2's `round-request.md`. It ran on this Linux host, on one `pulse-app` launch, with five legs and 150 s
quiet windows between them. The agent drove it in the overseer's slot.

## The seven verdicts

| # | Assertion | Verdict | Measured | Graded by |
|---|---|---|---|---|
| 1 | Read-back content fidelity | `[PASS]` | fingerprint in `fingerprint_refs` (4 refs, 3 `det-*`) · opened 30 ms after emission on an empty active set · `degraded_mode: false` | `lifecycle_harvest::p075_reround_assertion_1_read_back_content_fidelity` |
| 2 | Runtime-state fidelity | `[PASS]` | incident left the active set, idle 0 ms (sub-millisecond), `ProvenByLiveness` | `lifecycle_harvest::p075_reround_assertion_2_runtime_state_fidelity` |
| 3 | P-025 hue update ≤ 2000 ms | `[PASS]` | one in-window sample: a 128.45 ms rise (`autonomous`), anchored 0.55 ms from its incident's creation | `delegated_timing_harvest::tests::p075_reround_assertion_3_p025_hue_update` |
| 4 | P-027 discovery ≤ 5000 ms | `[PASS]` | worst 715.54 ms of 2 samples | `delegated_timing_harvest::tests::p075_reround_assertion_4_p027_discovery` |
| 5 | P-037 report render ≤ 2000 ms | `[PASS]` | 0 ms, 1 sample, `degraded_mode: false` | `delegated_timing_harvest::tests::p075_reround_assertion_5_p037_report_render` |
| 6 | P-045 counter refresh ≤ 1000 ms | `[PASS]` | worst 1.0 ms of 118 samples | `delegated_timing_harvest::tests::p075_reround_assertion_6_p045_counter_refresh` |
| 7 | Incident events read-back | `[PASS]` | before: `created` only, no `resolved` · after: `created,resolved` · `resolved` stamped 63 395 ns after the request was sent and 224 316 ns before the response arrived (window 287 711 ns) · 0 events outside the vocabulary · neither read truncated | `lifecycle_harvest::p075_reround_assertion_7_incident_events_read_back` |

None is UNGRADED. No leg was re-fired. The UI-timed leaves behind assertions 3–6 all fired under WebKitGTK on Wayland,
which was unmeasured on this platform before this round.

**For Pulse's `ref`:**
- The graded test ids are the seven above, in `crates/conductor-run/tests/`.
- The evidence path is `conductor-0.3.0/chunks/2026-10-03-p-075-re-round-on-incident-events/evidence/` at the
  Conductor commit that carries it.

## A Pulse finding to relay — the S2 Linux build does not stay up at its default posture here
- The first launch (23:02:14Z, data dir `/tmp/pulse-legs/reroundevents`, the plan's three handles only) booted to
  `ui-bridge.ready` and loaded deterministic L4, then died about 1.5 s in. Its last stderr line was
  `Gdk-Message: Error 71 (Protocol error) dispatching to Wayland display.`
- Host: NVIDIA GA102 (RTX 3090) · Hyprland · WebKitGTK 2.52.6 · native Wayland.
- No leg had fired. Nothing was left running, and `:4317`/`:4318` never opened a listener.
- The overseer ruled (founder-delegated, inside the founder's ruling to run on this Linux host): relaunch on a fresh
  data dir with `WEBKIT_DISABLE_DMABUF_RENDERER=1`, still on native Wayland, recorded as a launch-posture deviation
  beside the verdicts; if that also crashed, stop and ask, with no third posture.
- The relaunch stayed up for the whole round. The crashed launch's data dir and its stderr capture
  (`/tmp/pulse-legs/reroundevents-app.out`) were left in place, outside this repository.

## The binary under test and the slot
- **Slot:** granted by the overseer at about 23:01Z (binaries verified equal to S2, ports free). The founder was told
  hands-off at 23:03Z, before the first leg (no click; the Findings rows are not to be touched; the compact widget
  stays visible). No desktop input occurred.
- **Binaries:** sha256 re-measured at 23:02:07Z, immediately before the first launch. Both equal S2's:
  - `pulse-app` `23f6ef2bd0b854d9697a4d203b3aea1bf6f40b146420de362112556914976ada`
  - `andromeda-pulse-mcp` `e64f3688ec14144910ab25d461ec4ea99c161fda2e1b8824a3d480f17021e02e`

  The Pulse checkout HEAD read `cdb6c1ed572761ae384597a7ed437222e3a1d1fc` = S2. There was no rebuild.
- **Contract:** the sha256 of `contracts/pulse-p025-measurement-contract.md` is
  `05ca5d6c50a76ac13991893a779626f05d07039115105f5ce764ca689f9e95ea`, read at 23:02:07Z, BEFORE the round. It is
  the same value the prior round was graded by.
- **Launch** (the agent, on the slot), the round's launch:
  - S2's release `pulse-app`, launched by path from the cwd `/tmp/pulse-legs`, outside this repo;
  - a fresh data dir `/tmp/pulse-legs/reroundeventsretry`, absent before the launch;
  - `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_MCP_ENABLED=true`, `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` and
    `WEBKIT_DISABLE_DMABUF_RENDERER=1`, set in one block and echoed back;
  - PID 609008, started 23:03:25Z;
  - `:4317` and `:4318` accepting within a second.
- **Posture, from Pulse's own log** (`logs/agent-latest.jsonl.2026-10-03`, the file's name on Linux):
  - `interpretation.model.load`: "L4 deterministic mode active (ANDROMEDA_PULSE_L4_DETERMINISTIC); canned output, no
    model";
  - 0 `triage.baseline.bootstrap_window.override` lines, so the default window applied;
  - 0 ERROR lines at boot.

## The legs

Each leg was preceded by the non-priming `conductor preconditions` probe. All five read `[PRECONDITION] every
live-Pulse precondition is satisfied`, probe exit 0. There was no `boot`.

| Leg | Scenario | Pre-leg count | Fired → ended (Z) | Exit | Verdict line | Run id | Envelope |
|---|---|---|---|---|---|---|---|
| P-075 | `p075_round_live` | 645 | 23:04:04 → 23:04:14 | 0 | — (capture; end-check 1, events lines 2) | — | — |
| H | `halo-hue-encoding` | 3414 | 23:06:56 → 23:10:43 | 0 | `[MANUAL] halo-hue-encoding` | `2026-10-03T23-06-56-869` | `ManualCheck`, 180835 ms |
| D | `service-constellation-discovery` | 10232 | 23:13:29 → 23:14:22 | 0 | `[MANUAL] service-constellation-discovery` | `2026-10-03T23-13-29-947` | `ManualCheck`, 5957 ms |
| R | `report-render-surface` | 13530 | 23:17:02 → 23:17:54 | 0 | `[MANUAL] report-render-surface` | `2026-10-03T23-17-02-178` | `ManualCheck`, 6035 ms |
| F | `findings-counter-refresh` | 16783 | 23:20:33 → 23:21:25 | 0 | `[MANUAL] findings-counter-refresh` | `2026-10-03T23-20-33-277` | `ManualCheck`, 6074 ms |

- Every scenario leg's output `lacks [BLOCKED]` and contains its scenario name.
- Leg H's envelope reads `ManualCheck` here where the prior round's read `KnownResidual`. The envelope is not one of
  the seven assertions; it is recorded, not graded.
- The post-round count was 17786 at 23:21:31Z. Pulse wrote one log file, so no date split occurred.
- Quiet windows were 150 s each: 23:04:26→23:06:56, 23:10:59→23:13:29, 23:14:31→23:17:01 and 23:18:02→23:20:32.
- Leg H's self-obs was frozen to `h.jsonl` (790 lines) immediately after the leg.
- Status smoke (mint-then-read on leg F): `bash scripts/agent-run.sh status 2026-10-03T23-20-33-277` → exit 0,
  `"scenario": "findings-counter-refresh"`, the run id read back equal to the one minted.

## How each grade was derived
- **Assertions 1, 2 and 7:** from `p075-leg.txt`, the leg's stdout capture.
  - Membership is computed in-process, so only the boolean is committed.
  - The incident is the active one whose `opened_at_unix_nano` exceeds the `std::time` emission stamp taken before
    the storm; `active_at_open=0`, so the storm CREATED the incident.
  - Assertion 7's two `retrieve_incident_events` reads bracket `mark_incident_resolved`; the AFTER read follows the
    timed probe with no emission and no sleep in between. Its window is `std::time` epoch nanoseconds read
    immediately around the resolve call; only window-relative offsets leave the process.
- **Assertion 3:**
  - The window is `[timeline.execute new + 30 000 ms, scenario.run close]` from `h.jsonl`:
    1791068892902..=1791069043736.
  - One in-window sample in `pulse-h.jsonl`: the 128.45 ms rise. Its start instant (`timestamp − duration_ms`) lands
    0.55 ms from the `created=true` incident line at 23:08:18.087Z.
  - The leg's 930.82 ms fall (`severity_tier: none`) is stamped 23:10:56.849Z, 13.1 s after `scenario.run` closed at
    23:10:43.736Z, so the contract's window rule excludes it. It is under budget either way.
  - The leg's preflight canary rise is excluded, since its incident formed before phase-2 start.
- **Assertions 4–6:** the worst sample in the leg's own slice, read from each leaf's own field (`duration_ms`, with
  `value` for the report render).
- **Slices:** `pulse-{h,d,r,f}.jsonl` are Pulse's log lines in `(count_k, count_k+1]`, kept only where the target is
  one of the four metric leaves, `interpretation.incident.created`, or `triage.incident.auto_resolve.tick` with a
  numeric `resolved_count` ≥ 1 (none qualified in any slice). They were written by a byte-copying filter, as the
  prior round's were.

  | Slice | Window lines | Kept | sha256 (LF) |
  |---|---|---|---|
  | H | 6818 | 795 | `cdc2ecc3dc1170fa617878e309ebdcee9007609dca9ba7868719cb1b05e0f2a9` |
  | D | 3298 | 427 | `18e443b4f9f3a51b623788dd130de011abe5ce648e27a9ea30c21327949ce6e8` |
  | R | 3253 | 412 | `3f6c13c19f8d8100fdb137e2b8b8c2dbbe14c510a2f25e5cd6427147d86954d5` |
  | F | 1003 | 120 | `498496d70b80311c10ba742fe524a68cad61f5113e58c845b0ba8e7e8c5b5c03` |

  The other two pins are `p075-leg.txt` `aebc50ee41f3993929788bd424dcfae40cfaab26480efe29381ad93062aaafac` and
  `h.jsonl` `5eea3c19012ccb05b83775de6119b429f79b924f1ce80b473c9f76044fedf01d`.

## Process census and teardown
- **Before** (23:02:07Z): the census pattern `pulse-app|andromeda-pulse|conductor|WebKit` matched 0 processes; no
  listener on `:4317`/`:4318`; both data dirs absent.
- **The crashed first launch** (pulse-app PID 590369) had exited on its own within ~1.5 s; a census taken after it
  matched 0 processes and found no listener.
- **During** (23:21:31Z, after leg F): `pulse-app` 609008 and its five WebKitGTK children (one `WebKitNetworkProcess`,
  four `WebKitWebProcess`), all parented to 609008. No sidecar and no `conductor` process remained from the legs.
- **Teardown** (23:21:49Z): `kill -TERM 609008`; it exited within 2 s, so no `kill -KILL` was sent. The SIGTERM
  teardown WROTE an `app.exit` record (`WARN`, `process exit`) — recorded, not graded.
- **After** (23:21:50Z): the census pattern matched 0 processes, the five children included; no listener on
  `:4317`/`:4318`.

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` 590369 (first launch) | the agent, on the slot | exited on its own (the Wayland crash) |
| `pulse-app` 609008 + 5 WebKitGTK children | the agent, on the slot | terminated (SIGTERM) |
| `andromeda-pulse-mcp` (spawned by each leg's read-back client; the probes spawn none) | the legs | none present in the during- or after-census |
| `conductor` / `cargo` / test binaries | the legs | terminated |

## Deviations from the plan's entry text
- **Launch posture:** `WEBKIT_DISABLE_DMABUF_RENDERER=1` was added to the plan's three handles, on the overseer's
  ruling after the first launch crashed (above). The round's data dir is `/tmp/pulse-legs/reroundeventsretry`, not
  the plan's `/tmp/pulse-legs/reroundevents`, which the crashed launch had already written.
- **The round-sequencing helper:** each leg's probe → pre-count → leg sequence (→ freeze for H; → entries 13 and 14
  for P-075) ran from one scratch shell script, invoked once per leg, using the entries' own command text with each
  exit read from the bare command. The 150 s quiet windows ran as that script's leading `sleep 150`. The probe's
  output was captured whole, and its `[PRECONDITION]` line read from the capture.
- **Pre-built binaries:** the `conductor` CLI and the `p075_round_live` leg binary were built before the slot, so no
  compile time fell inside it. The entries' own `cargo run` / `cargo test` then found them current.
