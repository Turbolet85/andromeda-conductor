# Scope — Canary fingerprint-feed capture

**Marker:** `2026-08-14-canary-fingerprint-feed-capture`
**Version:** conductor-0.2.0 · Epoch 2 — Live-path enablement
**Working entry:** _Canary fingerprint-feed capture — why Pulse's storm detector tracks zero fingerprints
while nine spans arrive and both sides agree on the attribute keys, settled by a capture taken while Pulse is
running_

---

## The question this chunk answers

The first live preflight leg (`2026-08-13-first-live-green-preflight`) produced a contradiction that no
existing artifact can resolve:

- Conductor emitted **9 `emit.batch` spans without error** (3 warm-up + 6 storm).
- Pulse logged **`span_count: 9`** — it received all of them.
- The attribute contract **matches exactly** on both sides:
  - `conductor-emit/src/exception.rs:156-160` emits an event named `exception` carrying
    `exception.type` / `exception.message` / `exception.stacktrace`;
  - Pulse's `crates/buffer/src/appender.rs:346-350` extracts exactly those three keys;
  - Pulse's `fingerprint.rs:71-79` returns `None` only when `exception_type` is absent or empty.
- And yet **`tracked_fingerprints_count` stayed 0.**

Zero — not a small number — was read as the load-bearing detail, ruling out **both** cheap explanations at
once: scattered fingerprints (which would show a non-zero count spread thin) and detector-missed-the-group
(which would show the fingerprints tracked but no cue).

**METRIC CORRECTION (P3 research, 2026-08-14) — the expectation was wrong, but the conclusion survives.**
`tracked_fingerprints_count` is a **windowed gauge sampled after eviction, not a latched counter**:
`run_one_storm_cycle` (`andromeda-pulse/crates/triage/src/pattern/storm.rs:370-415`) first drops every
fingerprint whose timestamps have aged past `DEFAULT_STORM_WINDOW_SECONDS = 60`, then reports
`detector.fingerprints.len()` — a count of **distinct** fingerprints. So a working six-occurrence
*identical*-fingerprint storm reads **1**, never 6, and the gauge alone proves nothing about a sample taken
after its window closed.

**But the gauge was never the only evidence, and the latched signals agree with it.** The two **cumulative**
fields on the same log line — `storms_detected_total` and `fingerprints_evicted_total` (`:387-393`) — are
immune to the window, and both were already recorded at zero:

- `two-launch-verdict.md:187-189` (committed): "`tracked_fingerprints_count: 0`, `fingerprint_count: 0`,
  `fingerprints_evicted_total: 0` **across every sample**. The fingerprint table is empty — not populated with
  scattered identities."
- The operator's saved **arm-3** log tail carries a tick line ~4 minutes *after* that arm's canary storm —
  postdating the whole canary window — with `storms_detected_total: 0` **and**
  `fingerprints_evicted_total: 0` beside the gauge's 0. Cumulative counters reset per process, so for arm 3
  specifically **nothing had entered the detector's feed by then**.

Zero evictions means nothing ever aged out, i.e. nothing was ever tracked. The eviction-window reading does
**not** rescue the observation.

Research also closed one branch outright: Pulse's `FingerprintObserver` is genuinely wired
(`pulse-app/src/main.rs:600-601`), not a null, and the appender reads the three keys from `event.attributes`
exactly as Conductor writes them (`buffer/src/appender.rs:345-350`) — the transcribed direction and arity are
correct.

**The discriminator therefore stands as written, with a raised prior on the arrival/intake half:** whether the
six exception spans reach Pulse's appender with their events intact. It still does not say WHICH side —
Conductor may not have put intact events on the wire, or Pulse may not have taken them in — so the settling
pair is unchanged: prove what Conductor sends (nothing proves this today), and witness what arrives.

## Why it must be captured live

The discriminator **cannot be settled post-mortem.** Pulse's `corpus.db` carries no span table and no
exception table — it holds `baseline_state` / `digest_archive` / `incident_events` / `incidents` /
`pipeline_metrics` / `service_registry` only. Received spans therefore never persist, and the in-process
buffer dies with the `pulse-app` process. There is no file to read after the fact.

So the capture must be taken **while Pulse is running**, and it needs `pulse-app` alive — **not a corpus and
not an incident**. This is the property that makes the chunk tractable right now: Pulse's 3,600s-per-service
`BootstrapState::Ready` gate (the falsification recorded at the last chunk) blocks incident formation, but it
does **not** block this question.

## The fork this chunk resolves

Exactly one of two things is true, and the capture decides which:

- **The spans do NOT arrive with their events intact** → the defect is **Conductor-side**, and this entry
  **owns the fix**.
- **The spans DO arrive intact** → the question moves to **Pulse**, and this entry **closes by handing it
  over** with the evidence that makes the hand-off actionable.

Both outcomes are successful completions of this chunk. The deliverable is a *settled* answer plus the
artifact that settles it — not a green fingerprint count.

## Boundaries / non-goals

- **No incident is required, and none is expected.** Nothing here waits on L2 cue → L3 digest → L4
  inference, and nothing here re-litigates the bootstrap gate.
- **No Pulse-side change.** If the answer lands on Pulse's side, this chunk records and hands over; it does
  not fix Pulse. (The Pulse-side fix-scope — bootstrap reachability, workspace-key alignment — stays
  separable and outside this entry.)
- **The workspace-key axis (F10) is not this chunk's question.** It needs an incident; this one does not.
- **Not a scenario, not a new capability claim.** No new P-ID is driven; the catalog is untouched. `[inferred]`
- **Scope law holds:** Conductor opens no inbound listener; the capture must not become one. `[inferred]`

## Surfaces and contracts this chunk touches

- **`conductor-emit`** — the exception-event + fingerprint primitive that produces the payload under
  question (`exception.rs`, the fingerprint primitive). The capture's subject.
- **The OTLP egress path to `127.0.0.1:4317`** — the wire between the two sides, and the only place where
  "what Conductor actually sent" is observable independently of both sides' claims. `[inferred]`
- **Self-observation (`tracing` JSON, `logs/agent-latest.jsonl`)** — the existing capture surface, already
  carrying `emit.batch` spans; any new capture must obey the redaction boundary and the field allowlist
  rather than route around them. `[inferred]`
- **`conductor-verify` read-back** — bounded by a hard constraint: only the **persistent-corpus** tools work
  cross-process from a Conductor-spawned sidecar. The four live-buffer tools (`query_traces` /
  `query_metrics` / `query_logs` / `generate_snapshot`) return empty because the sidecar shares no memory
  with `pulse-app`, so read-back is **not** available as the capture channel for in-flight spans.
  `[inferred — from architecture.md §Standard Contracts; the capture channel is P3/P4's to choose]`
- **The PII corpus / synthetic payloads** — whatever is captured is emitted telemetry, so the artifact-hygiene
  rule (no host paths, no internal struct names) applies to the capture as to any other artifact. `[inferred]`

## Carried obligations (folded from the working entry)

### CARRY (from `2026-08-13-first-live-green-preflight`)

This is the **one open Conductor-side piece** of the live-path fix-scope. The Pulse-side pieces (bootstrap
reachability, workspace-key alignment) are deliberately separable and are not this entry's work; piece 2 is
invisible until piece 1 lands.

**Live-leg operator recipe — FIVE items, not the four previously on record:**

1. The sidecar must be **built AND on PATH** — `cargo build -p mcp-server --bin andromeda-pulse-mcp
   --features mcp-server` in the Pulse repo; `spawn.rs` resolves the fixed program name from PATH, and an
   unreachable sidecar makes every arm measure the read-back-unreachable path — evidence that looks like a
   result but is not.
2. `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` **and** `ANDROMEDA_PULSE_DATA_DIR` exported in the **same shell**
   that launches both, since Conductor observes the L4 term only as a declaration in its own environment.
3. **`ANDROMEDA_PULSE_MCP_ENABLED=true` in CONDUCTOR's own environment** — `spawn.rs` passes only the data
   dir via `.env(...)`, so inheritance is the sole channel. Without it the sidecar starts and immediately
   exits, and every arm measures the read-back-unreachable path. (The first arm-1 attempt at the last chunk
   ran without it and was discarded.)
4. Poll budget at or above the contract floor — `agent-run boot` now derives it.
5. `pulse-app` launched from a cwd **outside this repo** (it writes Specta/TauRPC bindings relative to its own
   cwd), with **Conductor itself always run from its repo root** (all four `contracts/*.toml` resolve
   cwd-relative).

**Open design question for the operator, deliberately undecided:** `spawn.rs` could set
`ANDROMEDA_PULSE_MCP_ENABLED` itself the way it already sets the data dir, making the sidecar's own gate
unmissable — but that would have Conductor enable a Pulse-side feature flag on its behalf, which
`architecture.md` §Occupied Resources currently asserts it does not do. This chunk surfaces the decision; it
does not pre-empt it.

### PREREQ (from `2026-08-13-first-live-green-preflight`) — `cargo audit` re-check

**SIXTEENTH consecutive check**, deferred since `2026-08-08-sut-capability-manifest`, re-pinning **silently**
under the operator's L5 ratification at the 2026-08-10 wrap (no further ratification HALT). Last re-proven on
**0.22.2** (the latest published): byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 — an
advisory-**DATABASE** fault with nothing to raise a floor to.

The standing basis is **`audit SURFACE unchanged (no new [[package]]) + cargo deny check verified green`**,
and it must be **re-verified literally, never echoed**. Remedy is the **bounded wait alone**: re-run it,
record the result, verify `cargo deny` ran green as the overlapping signal. Do **NOT** raise the floor, do
**NOT** add a `deny.toml` ignore, do **NOT** edit CI. Close the deferral the moment it parses.
(`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09/-08-10 · security-plan
§Dependency Security.)

## What "done" looks like

The chunk is done when the discriminator is **decided by evidence**, not by reasoning from source: a capture
artifact exists that shows whether the six exception spans left Conductor — and reached Pulse's appender —
with their `exception` events intact, and the answer is recorded with its consequence (Conductor-side fix
owned here, or a Pulse-side hand-off with the evidence attached). `[inferred — the working entry states the
fork and the capture; the artifact form is P4's to fix]`
