# Canary fingerprint-feed — capture verdict

**Question.** Pulse received all nine of the canary run's spans (`span_count: 9`) and tracked zero
fingerprints. Do the six exception spans reach Pulse's appender with their events intact?

**Status: settled, and handed off.** Conductor's storm leaves this side with its exception events intact,
and Pulse received all nine spans in every arm while its fingerprint table stayed empty across twelve
in-window samples. Spans arrive; nothing reaches the fingerprint observer. The remaining question is
Pulse's, and §4 says where it lives. One optional live leg (§5) would put both halves in a single run.

Every claim below carries its provenance, because the three kinds are not equally strong:

| Label | Meaning |
|---|---|
| **measurement** | Conductor observed it, in a run whose artifact is named here |
| **transcribed SUT record** | quoted from Pulse's own output; authoritative only as far as that record is |
| **read from SUT source** | derived by reading Pulse's code, never observed running — a hypothesis about behavior |

The third label is new here. The `contracts/*.toml` manifests distinguish a Conductor measurement from a
transcribed SUT record; this chunk needed a third, because reading a constant out of the SUT's source is
weaker than either — it says what the code *should* do, not what the process *did*.

---

## 1. The reading was wrong before the number was

*(read from SUT source — `crates/triage/src/pattern/storm.rs`, `pulse-app/src/main.rs`)*

`tracked_fingerprints_count` is **a windowed gauge sampled after eviction, not a latched counter**.
`run_one_storm_cycle` first drops every fingerprint whose timestamps have aged past the retention window,
then reports the number of **distinct** fingerprints still held. Consequences, each of which invalidates a
reading someone could reasonably have made:

- **A working six-occurrence storm reads `1`, never `6`.** The canary deliberately emits six *identical*
  fingerprints; the gauge counts distinct ones. Anyone expecting 6 would call a healthy run broken.
- **A zero sampled after the retention window proves nothing on its own.** The storm lands behind the
  warm-up pre-roll, and the canary poll keeps the process alive well past the window's close — so a late
  sample legitimately reads 0 on a fully working path.
- **The latched discriminators are `storms_detected_total` and `fingerprints_evicted_total`**, both
  cumulative and immune to the window. `fingerprints_evicted_total ≥ 1` proves a fingerprint was tracked and
  later aged out, whatever the gauge says.
- **Detection does not wait for the tick.** Occurrences are recorded inline as they arrive, so a cue at the
  Suggested threshold fires on the occurrence that crosses it, independent of tick cadence. Tick timing is
  therefore not an explanation for a missing cue.

Two branches closed by the same reading: Pulse's fingerprint observer **is** wired (it is constructed and
passed, not left as the no-op), and the appender reads the three keys from the **event's** attributes — the
same place Conductor writes them. The direction and arity previously transcribed are correct.

## 2. Conductor's half — the storm leaves intact

*(measurement — `crates/conductor-run/tests/canary_wire.rs`, run under `cargo nextest run -p conductor-run`)*

The canary path had **no wire-tier test** before this chunk: every prior assertion about it was a statement
about the builder, or about source. It now has one. The real emission loop (`emit_canary_storm`) is driven
against a loopback collector stub on an ephemeral port, and the assertions run on the spans the **stub
received**, not on what Conductor intended to send:

- all `CANARY_STORM_COUNT` occurrences arrive;
- every span carries a non-empty `trace_id` **and** `span_id` — a span missing either is skipped outright by
  the receiver, so this is a precondition, not a formality;
- every span carries an `exception` event with a non-empty `exception.type`, `exception.message` and
  `exception.stacktrace` — an absent or empty type yields no fingerprint at all;
- span identity is **distinct** across the six while `exception.type` and the stacktrace are **identical** —
  which is what makes the storm one recurring fault rather than six unrelated ones;
- the resource carries the canary `service.name` — a row with an empty service is dropped before it reaches
  the detector's window.

A wire-shape witness also now rides the `emit.batch` boundary, recording per batch the span count, spans a
receiver would skip for a missing id, and the distinct event / event-attribute **key names**. Names only,
never values. It is the emitting-side twin of the read-back boundary's key-set witness, and exists for the
same reason: a receiver that degrades to empty makes a shape divergence indistinguishable from emptiness.

The witness is **read back out of a real self-observation artifact**, not asserted in isolation — because a
witness dropped by the field allowlist or filtered out by level would reproduce the exact failure under
investigation one layer down. Driving the storm with the self-obs layer installed yields one line per
exported batch, each carrying the full self-obs base field set and this message:

```
wire shape: spans=1 spans_missing_ids=0 event_names=[exception] event_attr_keys=[exception.message,exception.stacktrace,exception.type]
```

Every key Pulse's appender extracts is named, no span is missing an id, and no value appears.

**Conclusion for this half:** what Conductor puts on the wire satisfies every documented precondition of
Pulse's intake path. If fingerprints are not being tracked, the cause is not a malformed emission.

## 3. Pulse's half — what the prior arms recorded

*(transcribed SUT record)*

- Across the three-arm probe: `tracked_fingerprints_count: 0`, `fingerprint_count: 0`,
  `fingerprints_evicted_total: 0` **across every sample** — the fingerprint table empty rather than
  populated with scattered identities.
- **Arm 3** additionally carries a tick line roughly four minutes *after* that arm's canary storm —
  postdating the entire canary window — with `storms_detected_total: 0` **and**
  `fingerprints_evicted_total: 0` beside the gauge's 0.

Cumulative counters reset per process, so for arm 3 specifically **nothing had entered the detector's feed by
then**. Zero evictions means nothing ever aged out, which means nothing was ever tracked. The
eviction-window explanation does not rescue the observation — the latched signals agree with the gauge.

### The full per-arm tick series — all three arms *(transcribed SUT record)*

The arms' Pulse log was never lost: it is frozen on disk as the date-suffixed rotation
`{data_dir}/logs/agent-latest.jsonl.2026-08-13`, and the probe harness recorded each arm's byte offset
before starting it. The saved per-arm slices were verified **byte-identical** (md5) to the frozen log read
at those offsets, so what follows is the original record, not a re-derivation.

Offsets relative to that arm's storm burst. Every arm shows `window_seconds = 60`.

**Arm 1 — repo root** (11 ticks, −46s → +104s)
**Arm 2 — marker-less temp dir** (10 ticks, −46s → +89s)
**Arm 3 — the data dir itself** (10 ticks, −46s → +89s)

| offset | arm 1 | arm 2 | arm 3 |
|---|---|---|---|
| −46s / −31s / −16s / −1s | 0 · 0 · 0 | 0 · 0 · 0 | 0 · 0 · 0 |
| **+14s / +29s / +44s / +59s** | **0 · 0 · 0** | **0 · 0 · 0** | **0 · 0 · 0** |
| +74s / +89s (/ +104s, arm 1) | 0 · 0 · 0 | 0 · 0 · 0 | 0 · 0 · 0 |

Each cell is `tracked_fingerprints_count · storms_detected_total · fingerprints_evicted_total`. **Thirty-one
tick lines across three arms, every field zero.**

**This closes the eviction-window question outright.** The retention window is 60s, and each arm has **four
samples inside it** (+14s, +29s, +44s, +59s) — twelve in-window samples in total. They read zero. A
fingerprint that had been tracked and later aged out would have to pass through those samples on its way to
being evicted, and would leave `fingerprints_evicted_total` non-zero besides. Neither happened, in any arm.
The zero is not a sampling artifact.

### What the same logs show about arrival *(transcribed SUT record + read from SUT source)*

The ingest heartbeat's `span_count` is a **cumulative counter** — `fetch_add` on receipt, snapshotted per
heartbeat (`crates/ingest/src/state.rs`, `crates/ingest/src/contract.rs`), not a per-tick delta. Read that
way, every arm records an identical and complete arrival:

```
span_count:  1 → 2 → 3 → 9 → 9 → 9 → 9 → 9 → 9 → 9
timestamps:  −46s  −31s  −16s   +0s  (then steady)
```

- The three warm-up spans arrive one per heartbeat at −46s, −31s and −16s — 15s apart, exactly the
  contract's warm-up window divided by its emission count.
- The counter then jumps **3 → 9 in a single heartbeat**: six spans in one burst, which is the storm.
- It holds at 9 for the rest of every arm — nothing further arrives, and nothing is lost.

So **Pulse received all nine spans, counted them, and the count is stable** — in all three arms, at every cwd.

### PENDING — a new live leg
Not run: no live `pulse-app` was reachable and the MCP sidecar is not on PATH. See §5. Note this is now the
only open item — the three arms above are complete and need no re-run.

**Why no witness lines exist yet, measured rather than assumed** *(measurement)*: a scenario run was driven
to completion against this environment. It produced a fresh self-observation artifact carrying sixteen lines,
of which **zero** were `emit.batch` spans and **zero** were witness lines. The run stops at
`preflight blocked: MCP read-back path unreachable` — earlier than the canary emission, because the read-back
session is established *before* the canary gate runs. The scenario reported `[BLOCKED]` and the harness
exited 0, which is correct: a blocked state is not a hard failure.

The consequence sharpens recipe item 1 below. **An unreachable sidecar means the canary never emits at all** —
so a live leg run without it produces no witness lines whatsoever, not merely a missing incident. The absence
of a witness line is therefore not evidence about the wire; it is evidence that the leg never ran.

## 4. Where the fork stands

The chunk's fork was: events do not arrive intact ⇒ Conductor-side defect, owned here; events arrive intact
⇒ hand the question to Pulse.

**The second branch is taken. This is a hand-off to Pulse, and the evidence supports it from both ends:**

- Conductor's storm leaves the process carrying `exception` events with all three attribute keys, non-empty
  ids, distinct span identity and one shared fingerprint — asserted on spans a collector actually received
  (§2).
- Pulse received all nine spans and counted them, stably, in every arm (§3).
- Pulse's fingerprint table stayed empty across twelve in-window samples spanning three arms, with both
  cumulative counters at zero throughout (§3).

Spans arrive; nothing reaches the fingerprint observer. The gap therefore lies **inside Pulse, between OTLP
ingest receipt and the per-span-event fingerprint hook** — downstream of the ingest counter that proves
arrival, upstream of the observer whose invocation would have moved `fingerprints_evicted_total` off zero.
That is the region a Pulse-side investigation should start in.

**Two limits stated plainly, because the hand-off should not claim more than it has.** First, no single run
has yet observed the wire witness and the storm-tick counters together — the two halves come from different
runs of the same build, and §5 exists to close that. Second, localizing the gap *inside* Pulse names a
region, not a defect: Conductor cannot see into that path, and a claim about which component drops the events
would be a measurement Conductor has not made.

## 5. Running the live leg

No incident is required — this is what makes the leg tractable while Pulse's per-service bootstrap gate
stands. The storm tick fires on its own cadence regardless, and occurrences are recorded inline. A
`ready:false` preflight is an **expected, non-blocking** outcome: the evidence here does not depend on
read-back at all.

1. Sidecar built **and on PATH** — build the MCP server binary in the Pulse repo with its feature enabled.
   Unreachable, every arm silently measures the read-back-unreachable path and yields evidence that looks
   like a result but is not.
2. `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` and `ANDROMEDA_PULSE_DATA_DIR` exported in the **same shell** that
   launches both sides.
3. `ANDROMEDA_PULSE_MCP_ENABLED=true` in **Conductor's own** environment — the sidecar inherits it, and that
   inheritance is the only channel.
4. Poll budget at or above the run contract's floor (the harness derives it).
5. `pulse-app` launched from a working directory **outside** this repository; Conductor always run from its
   repository root.
6. `RUST_LOG=info,conductor_emit=debug`, so the wire-shape witness reaches the self-observation
   artifact. **The leading `info` is load-bearing** — a bare `conductor_emit=debug` directive filters every
   other target out, silencing the rest of the self-obs stream and leaving the artifact carrying witness
   lines and nothing else. Measured, not assumed: the bare form fails the CLI's own agent-mode self-obs
   test, and the two-directive form passes it. Do not run the harness's `run` verb with either form set —
   the env reaches nextest's child processes; the witness belongs on the `boot` leg.

**Capture, from the one run:**

- **Conductor side** — the `emit.batch` witness lines from the self-observation artifact: one per exported
  batch, showing the span count, spans missing an id, and the event / attribute key names actually sent.
  Confirm the artifact is fresh for this run before reading it.
- **Pulse side** — every storm-tick line spanning the storm's live window, each with its offset from the
  storm, the gauge, **and both cumulative fields**. Roughly four ticks fall inside the window; read the whole
  span, not one line.

**A non-zero `fingerprints_evicted_total` on this run would be news** — it would contradict the three prior
arms and mean something changed between legs. Record it prominently rather than treating the tick capture as
a formality.
