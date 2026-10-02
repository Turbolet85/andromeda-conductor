# P-075 round ledger — 2026-10-02

The one deterministic round against Pulse S = `03ec94481b0d6c3ba574626e7acb39e33fd40141` that grades Pulse's
`round-request.md` six assertions. It ran on one `pulse-app` launch, with five legs and 150 s quiet windows between
them. It was driven by the agent in the overseer's slot and `:4317` grant.

## The six verdicts

| # | Assertion | Verdict | Measured | Graded by |
|---|---|---|---|---|
| 1 | Read-back content fidelity | `[PASS]` | fingerprint in `fingerprint_refs` (4 refs, 3 `det-*`) · opened 46 ms after emission · `degraded_mode: false` | `lifecycle_harvest::p075_round_assertion_1_read_back_content_fidelity` |
| 2 | Runtime-state fidelity | `[PASS]` | incident left the active set, idle 12 ms, `ProvenByLiveness` | `lifecycle_harvest::p075_round_assertion_2_runtime_state_fidelity` |
| 3 | P-025 hue update ≤ 2000 ms | `[PASS]` | worst 478.56 ms in window (rise 438.24 ms anchored 38.24 ms; fall 478.56 ms) | `delegated_timing_harvest::tests::p075_round_assertion_3_p025_hue_update` |
| 4 | P-027 discovery ≤ 5000 ms | `[PASS]` | worst 605.26 ms of 2 samples | `delegated_timing_harvest::tests::p075_round_assertion_4_p027_discovery` |
| 5 | P-037 report render ≤ 2000 ms | `[PASS]` | 0 ms, 1 sample, `degraded_mode: false` | `delegated_timing_harvest::tests::p075_round_assertion_5_p037_report_render` |
| 6 | P-045 counter refresh ≤ 1000 ms | `[PASS]` | worst 5.0 ms of 163 samples | `delegated_timing_harvest::tests::p075_round_assertion_6_p045_counter_refresh` |

None is UNGRADED. No leg was re-fired.

**For Pulse's `ref`:**
- The graded test ids are the six above, in `crates/conductor-run/tests/`.
- The evidence path is `conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/` at the
  Conductor commit that carries it.

## The binary under test and the slot
- **Slot:** granted by the overseer at about 05:22Z, with `:4317` and `:4318` measured free and no `pulse-app`
  running. The founder was told hands-off. The hands-off instruction went out as stated (no click; the Findings rows
  are not to be touched; the compact widget stays visible), and no desktop input occurred.
- **Binaries:** sha256 re-measured at 05:23:03Z, immediately before the launch. Both equal the relay:
  - `pulse-app.exe` `8f2377e1e529b9b6c68a11b14879afc274e3fee579e15477bf9525c7efa761ab`
  - `andromeda-pulse-mcp.exe` `d898a3b806098344cb22c814502cdfcd1d8f084f8704c1a8b2875ff66f85302a`

  The Pulse checkout HEAD read `03ec94481b0d6c3ba574626e7acb39e33fd40141` = S. There was no rebuild.
- **Launch** (the agent, on the slot):
  - S's release `pulse-app.exe`, launched by path from a cwd outside this repo;
  - a fresh data dir `%TEMP%/pulse-legs/assertround`, absent before the launch;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` and `ANDROMEDA_PULSE_MCP_ENABLED=true`;
  - PID 56832, created 05:23:13.96Z;
  - `:4317` accepting at once.
- **Posture, from Pulse's own log:**
  - `interpretation.model.load` with `inference_mode: deterministic` ("L4 deterministic mode active");
  - 0 `triage.baseline.bootstrap_window.override` lines, so the default window applied;
  - 0 ERROR lines at boot.
- **Contract:** the sha256 of `contracts/pulse-p025-measurement-contract.md` is
  `05ca5d6c50a76ac13991893a779626f05d07039115105f5ce764ca689f9e95ea`. It was read AFTER the round, not before leg H
  as the plan asked. Its last write was 05:01:15Z, before the 05:23:13Z launch, so this is the rule the round was
  graded by.

## The legs

Each leg was preceded by the non-priming `conductor preconditions` probe. All five read `[PRECONDITION] every
live-Pulse precondition is satisfied`, probe exit 0. There was no `boot`.

| Leg | Scenario | Pre-leg count | Fired → ended (Z) | Exit | Verdict line | Run id | Envelope |
|---|---|---|---|---|---|---|---|
| P-075 | `p075_round_live` | 7610 | 05:23:48 → 05:24:17 | 0 | — (capture; end-check 1) | — | — |
| H | `halo-hue-encoding` | 67061 | 05:27:11 → 05:31:02 | 0 | `[RESIDUAL] halo-hue-encoding` | `2026-10-02T05-27-11-079` | `KnownResidual`, latency 184032 ms |
| D | `service-constellation-discovery` | 179604 | 05:33:42 → 05:34:35 | 0 | `[MANUAL] service-constellation-discovery` | `2026-10-02T05-33-42-799` | `ManualCheck`, 5980 ms |
| R | `report-render-surface` | 240464 | 05:37:14 → 05:38:08 | 0 | `[MANUAL] report-render-surface` | `2026-10-02T05-37-15-029` | `ManualCheck`, 6062 ms |
| F | `findings-counter-refresh` | 302038 | 05:40:49 → 05:41:43 | 0 | `[MANUAL] findings-counter-refresh` | `2026-10-02T05-40-50-313` | `ManualCheck`, 6109 ms |

- Every scenario leg's output `lacks [BLOCKED]` and contains its scenario name.
- The post-round count was 318823 at 05:41:43Z. Pulse wrote one log file (`agent-latest.jsonl.2026-10-02`), so no
  date split occurred.
- Quiet windows were 150 s each: 05:24:39→05:27:09, 05:31:11→05:33:41, 05:34:43→05:37:13 and 05:38:17→05:40:48.
- Leg H's self-obs was frozen to `h.jsonl` (784 lines) immediately after the leg.
- Status smoke (mint-then-read on leg F): `bash scripts/agent-run.sh status 2026-10-02T05-40-50-313` → exit 0,
  `"scenario": "findings-counter-refresh"`.

## How each grade was derived
- **Assertions 1–2:** from `p075-leg.txt`, the leg's stdout capture.
  - Membership is computed in-process, so only the boolean is committed.
  - The incident is the active one whose `opened_at_unix_nano` exceeds the `std::time` emission stamp taken before
    the storm.
  - `active_at_open=0`, so the storm CREATED the incident, the case in which Pulse writes the cue fingerprint.
- **Assertion 3:**
  - The window is `[timeline.execute new + 30 000 ms, scenario.run close]` from `h.jsonl`:
    1790918907230..=1790919061263.
  - Two in-window samples in `pulse-h.jsonl`; the worst is 478.56 ms.
  - The leg's preflight canary rise is excluded, since its incident formed before phase-2 start.
- **Assertions 4–6:** the worst sample in the leg's own slice, read from each leaf's own field (`duration_ms`, with
  `value` for the report render).
- **Slices:** `pulse-{h,d,r,f}.jsonl` are Pulse's log lines in `(count_k, count_k+1]`, kept only where the target is
  one of the four metric leaves, `interpretation.incident.created`, or `triage.incident.auto_resolve.tick` with
  `resolved_count` ≥ 1. They were written by a byte-copying filter.

  | Slice | Lines | sha256 (LF) |
  |---|---|---|
  | H | 1187 | `62791cd31ff2cdc40e2c15dfe801365c120cf5c9c75bb9d0a619603750942851` |
  | D | 641 | `a5ffacd9df730a91d9853c620bbcf96587aade04e16c2ed22948cd7162f86c7f` |
  | R | 649 | `75e8de2942418d5bffb9ea4b806edc338fc185223ba164b88252f8feb2849576` |
  | F | 165 | `57776de49989020ec58d61eacb3eac680f2fac747f515b58dbc1945327275565` |

  The other two pins are `p075-leg.txt` `85773fb075f5bf7ac199613c9db5f0523f38a7f11a181595721515aace7a4e72` and
  `h.jsonl` `21c6520a5a0d10014d871a8a4a6120ab8054514d5d4097244d6909d7b3824332`.

## Process census and teardown
- **Before** (05:23:03Z): no `pulse-app`, `andromeda-pulse-mcp` or `conductor` process.
- **Teardown** (05:42Z):
  - `pulse-app` PID 56832 was matched by name and CreationDate, then stopped with `Stop-Process -Force`;
  - its children (conhost 33976, msedgewebview2 18908) were gone, with 0 processes left parented to 56832 or 18908;
  - no LISTENING socket on `:4317`/`:4318`.
  - A forced stop writes no `app.exit` record. That is expected and not a finding (round-request §The binary under
    test).
- **After** (05:42:11Z): no `pulse-app`, `andromeda-pulse-mcp` or `conductor` process. Each leg's sidecar and
  `conductor` process ended with its leg.

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` (PID 56832) + conhost + msedgewebview2 | the agent, on the slot | terminated |
| `andromeda-pulse-mcp` (spawned by each leg's read-back client; the probes spawn none, the count not taken) | the legs | none present in the after-census |
| `conductor` / `cargo` / test binaries | the legs | terminated |

## Deviations from the plan's entry text
- **The evidence dir** was created (`mkdir -p` of `evidence/`) before the P-075 leg's entry. The entry redirects
  stdout into it and does not create it.
- **The round-sequencing helper:** the probe → pre-count → leg → freeze sequence for legs H/D/R/F ran from one
  scratch shell script per leg. It used the entries' own command text, with each exit read from the bare command.
  The probe's output was filtered to its `[PRECONDITION]` line, and its exit was read from the pipeline's first
  stage.
- **The contract sha256 timing:** read after the round, not before leg H (above).
