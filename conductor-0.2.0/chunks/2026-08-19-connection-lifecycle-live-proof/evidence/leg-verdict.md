# Leg verdict — connection-lifecycle live proof

**Runs (six, fresh data dir each under `%TEMP%/pulse-legs/`):**
leg A1 `2026-08-19T22-38-54-528` (old walk TOML, check intact — the retirement's measurement leg) ·
leg A2 `2026-08-19T22-42-10-570` (new port-conflict TOML, Pulse left up — the ratified Err-path leg;
**no run row by design**) · leg B1 `2026-08-19T22-44-56-113` (shipped walk) · leg B2
`2026-08-19T22-47-17-426` (shipped last-span-ago) · leg B3 `2026-08-19T22-48-30-247` (shipped
orthogonal) · leg B4 `2026-08-19T22-50-47-826` (shipped port-conflict, the choreographed leg).
**SUT:** andromeda-pulse HEAD `efabe8e` (unchanged — the TIME-axis obligation discharged); the ratified
binary pair (`pulse-app.exe` 2026-08-17 23:04 · `andromeda-pulse-mcp.exe` 2026-08-17 22:27).
**Recipe:** the six-item set — `ANDROMEDA_PULSE_DATA_DIR` (fresh per leg, Windows form, pulse-app cwd =
data dir) · `ANDROMEDA_PULSE_MCP_ENABLED=1` · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` · sidecar via
`PATH` (debug dir) · NO `SEED` (every envelope recorded its TOML seed: 4317001/4317002/4317003/4317004)
· NO `RUST_LOG`. Conductor invoked DIRECTLY (`target/debug/conductor.exe run <name> --agent-mode`) —
the plan's `boot && run` pairing was dropped as a plan defect: `boot` fires its own canary, whose OPEN
incident would dedupe-block the run's canary on the same dir (the 2026-08-16 back-to-back rule).

## Leg A1 — the as-shipped grading measured (then retired)

| Prediction (research, source-proven) | Measured |
|---|---|
| Hard `Contains "Stalled"` fails STRUCTURALLY (no read-back surface carries connection state) | failed → verdict **`Fail`** |
| Degraded read-back overrides state (det-L4 permanent `degraded_mode`) | `state: KnownResidual`, renders `[RESIDUAL]`, **exit 0** |
| Fingerprints payload-invariant | the constant `det-*` triple |
| `<90s` tier | HELD: `latency_ms` 69169 (journal-relative; ≈ the 69s phase sum) |

The walk's one check graded nothing it claimed to grade (the structural-fail class the fingerprint-storm
/ baseline / pii retirements measured), so all four family TOMLs retired declare-only with the
measurement in their headers. The other three members' checks (`Receiving` / `Idle`+`error` /
`ReceiverFailed`) share the mechanism source-proven (no MCP tool carries FSM state; `Observation.text`
composes incident-list + report text only) — the class was measured on the flagship, not re-measured
per member.

## Leg A2 — the ratified occupy-failure policy, live

New port-conflict TOML, Pulse deliberately left holding `:4317`. Preflight green → all three phases ran
(fault.silence spans at journal offsets 0 / 20014 / 60017, each `parent: timeline.execute`) → at the
port-held open the occupy failed AddrInUse → the ERROR line `port occupier could not bind: the port is
already held` → after the timeline: `error: the fault-declared phase could not apply its port occupier`,
**exit 1, NO run row written** (the newest journal remained A1's). No `fault.port_occupier` span (the
bind never succeeded), zero panics. Exactly the P4-ratified shape.

## Legs B1–B4 — the shipped declare-only shape: every witness landed

All four rows: `verdict: null` · `state: KnownResidual` (ManualCheck overridden by the degraded
read-back — the DriveObserve family's sibling landing, non-Blocked) · TOML seed · exit 0:

| leg | scenario | latency_ms | tier | key witnesses (pinned in `connection_harvest.rs`) |
|---|---|---|---|---|
| B1 | receiver-lifecycle-state | 69184 | `<90s` HELD | walk tail `Receiving -> Idle` lag 10790 → `Idle -> Stalled` lag 60788 (severity `warning`); warm-up oscillation (spans ~15s apart straddle the 10s threshold) |
| B2 | last-span-ago-tracking | 6079 | `<20s` (re-tiered from `<5s`, measured) | fresh-span lags 539–588 ms; Idle crossings 10496–10589 — the ±1s tracker claim at the crossings |
| B3 | orthogonal-health-domains | 18188 | `<90s` (kept; 9% margin under `<20s` too thin) | 100%-error burst raises NO alert state (zero ReceiverFailed); clean-stop `Receiving -> Idle` lag 10392 |
| B4 | receiver-failed-port-conflict | 75074 | `<90s` (re-tiered from `<5s`) | see below |

**B4 choreography (first mid-scenario operator choreography; cues off Conductor's own self-obs):**
cue1 pre-conflict open at t=46s → pulse-app STOPPED · cue2 `fault.port_occupier` span `new` at +20s —
**the occupier BOUND `:4317`** → pulse-app STARTED into the conflict · cue3 span `close` at +38s (the
phase-boundary RAII release) → pulse-app RESTARTED. Pulse's ledger:
- instance 2: ERROR `bind failed`, reason `OTLP receiver bind failed: Only one usage of each socket
  address … (os error 10048)`, `bind_address 127.0.0.1:4317` at 22:51:56.246 — INSIDE the hold window;
- the FSM transition `Listening -> ReceiverFailed`, `trigger_reason: "receiver bind failed"`, severity
  `critical`, level ERROR at 22:51:56.254;
- instance 3: `OTLP gRPC receiver bound` at 22:52:36.445 — AFTER the release: **the release proven by
  the SUT's own successful rebind.**

Conductor's own self-obs on the same leg (run `2026-08-19T22-50-47-826`) carries the obs-plan §4
witness — the occupier span BENEATH the timeline, exactly two attributes, closing at the boundary
40,013 ms later (verbatim):

```json
{"deployment.environment":"local","fault_type":"port_occupier","level":"INFO","parent":"timeline.execute","port":4317,"run_id":"2026-08-19T22-50-47-826","service.name":"conductor","service.version":"0.1.0","span":"fault.port_occupier","span_event":"new","target":"conductor_faults::port_occupier","timestamp_ms":1787179913956}
{"deployment.environment":"local","level":"INFO","run_id":"2026-08-19T22-50-47-826","service.name":"conductor","service.version":"0.1.0","span":"fault.port_occupier","span_event":"close","target":"conductor_faults::port_occupier","timestamp_ms":1787179953969}
```

obs-plan §4's CONDITIONAL `timeline.execute` parentage is therefore measured REAL on the driven leg —
the wrap-owned §4 unconditional rewording has its evidence here.

**One acceptance sub-clause measured differently than worded (surfaced for wrap):** the concretized
v2-15 acceptance says the relaunched pulse-app logs the rebind "with a recovery transition". Recovery is
a PROCESS REPLACEMENT — bind status is per-process and a fresh instance's FSM starts at Listening — so
no FSM transition OUT of ReceiverFailed exists to log. The proof mechanism the same sentence names (the
rebind) is measured and pinned; the sub-clause needs a wrap-owned notes clarification, not a code change.

## Gates

nextest **655/655** zero-retry ci profile (648 post-model + 7 harvest) · doctests · clippy `-D warnings`
green (3 first-pass `rfind` lints in the new harvest test, fixed) · full `agent-run.sh run` bundle
exit 0 (twice: pre-legs and post-harvest) · `check_load_envelope` green with `[[exempt]]` still exactly
empty (every re-based phase is a silence window or a 10-span/3s burst, orders inside both sustained
terms).

**PREREQ (31st consecutive `cargo audit` re-check — RESTATED basis):** `cargo audit` standalone TRUE
exit 1, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244` — the external advisory-DB fault
persists; `cargo deny check advisories bans licenses sources` TRUE exit 0 over the NEW `Cargo.lock` —
the overlap VERIFIED, not assumed. The basis is NO LONGER zero-delta: this chunk adds the
`conductor-run -> conductor-faults` edge (one member-dep line in `Cargo.lock`, ZERO new external
crates) — the 2026-08-16 admitting-a-dependency precedent applied (deferred since
`2026-08-08-sut-capability-manifest`). No floor raise, no `deny.toml` ignore, no CI edit.

## Hygiene

- Conductor artifacts (all six legs' journals/reports/`runs.db` rows + `logs/agent-latest.jsonl`):
  zero absolute host paths, zero internal struct names (the leg dirs live under `%TEMP%` and never
  enter any artifact — grep-verified on the B-leg artifacts).
- The pinned harvest lines carry no host path and no Conductor internal
  (`no_pinned_line_carries_a_host_path_or_conductor_internal`).
- Zero panics across all legs including the deliberate AddrInUse leg (A2).

## New measurements (this chunk) — standing intake candidates

1. The preflight warm-up (3 spans across 45s) paces spans past the 10s Idle threshold, so every
   preflighted leg's ledger carries a Receiving <-> Idle oscillation before phases — expected noise a
   harvest predicate must tolerate (pinned as `the_warmup_oscillation_straddles_the_idle_threshold_by_construction`).
2. `trigger_reason` on the ReceiverFailed transition is the human string `"receiver bind failed"`,
   not the enum-ish `bind_failed` the plan predicted — pinned verbatim.
3. Recovery from ReceiverFailed is a process replacement (no rebind loop, no FSM exit transition) —
   confirms research; the visible recovery witness is the fresh instance's bound line.
