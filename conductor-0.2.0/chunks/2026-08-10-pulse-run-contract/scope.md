# Scope — 2026-08-10-pulse-run-contract

**Working-route entry (Epoch 2 — Live-path enablement):**

> Pulse run contract — deterministic-L4 mode and shared data-dir asserted at preflight, unmet terms
> named (P-073)

**Claims:** `verification-matrix.json#v2-18` (Machine-checked Pulse run contract).
**Intent provenance:** `intent.md` §Theme 3 F11 (the run contract is nowhere recorded) + §Theme 1 F2
(P-073 — "a run contract that asserts it is active *is* that verification") + §Theme 1 F4 (the load
envelope bounds what the contract may prescribe).

**Provenance tags used below.** Bullets the working entry did NOT itself state are marked so P4
re-verifies them first: `[operator-directive]` = supplied at take-up by the overseer, pre-verified
against live artifacts; `[inferred]` = derived here from the specs / code / prior-chunk evidence.

## What this chunk builds

A **recorded, reproducible run contract** for a verifiable Pulse, plus the preflight assertion of it.
Two halves, and the second is what makes the first more than a README:

1. **The contract, recorded in-repo** — the launch conditions under which a live Pulse can actually be
   verified, in a committed artifact, following the established `contracts/` precedent
   (`pulse-capabilities.toml`, `pulse-load-envelope.toml`): read at a fixed path, bounds-checked at
   load, absent/malformed a harness fault, never a silent default.
2. **Preflight asserts the terms and names each unmet one individually** — extending the readiness gate
   that today carries four named preconditions, in its established shape: a distinct host-path-free
   `Blocked` precondition string, `Ok(Blocked)` not `Err`, never a silent downgrade.

### The contract's real content

The working entry names two terms; they are **necessary but not sufficient**, and the chunk that
proved it is the one immediately preceding this one.

- **Term A — deterministic L4.** `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` belongs on the **launched
  `pulse-app`** (the OTLP receiver that CREATES incidents), *not* on the MCP sidecar Conductor spawns
  (`intent.md` F11, verbatim). In deterministic mode the model prerequisites
  (`ANDROMEDA_PULSE_MODEL_PATH`, `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`) do not apply — the contract
  should say so rather than leave an operator provisioning a model it will not use.
- **Term B — shared data dir.** `pulse-app` and the spawned sidecar must resolve the same data dir.
- **Term C — incident-formation viability.** `[operator-directive]` A and B are not sufficient:
  `2026-08-10-workspace-key-divergence-probe` had **both**, with deterministic L4 confirmed active in
  all three arms, and still recorded `cues_emitted: 0` with the service in baseline bootstrap — so **no
  incident formed under any workspace key** and all three arms blocked for the no-incident cause
  (evidence: that chunk's `two-launch-verdict.md`). The contract must therefore also encode what makes
  an incident actually form:
  - ~~the **storm shape** — Pulse's L2 RetryStorm cue is ≥5 same-fingerprint occurrences within 30s~~
    **AMENDED 2026-08-10 at P5 (intent-incomplete, not a plan defect).** The cue rule is real, but it is
    **already satisfied**: `emit_canary` emits `CANARY_STORM_COUNT = 6` identical-fingerprint exceptions
    and cites that floor in its own doc comment (`conductor-run/src/lib.rs:105-107,146-150`). A contract
    term prescribing storm shape would restate what already holds. It stays recorded in the contract as a
    satisfied term, but it is not this chunk's lever;
  - the **bootstrap timing — the OPERATIVE lever** — L3 digest runs on a 20–60s cadence with L4 behind
    it, and the probe's telemetry shows the service sat in baseline bootstrap
    (`services_in_bootstrap: 1`, `services_ready: 0`) for the whole window `[inferred]`. Research
    established why: **the canary storm is the canary service's first-ever traffic**, so Pulse has no
    baseline for it and the cue evaluator never considers it. The contract's two real terms are therefore
    a **warm-up pre-roll** that carries the service out of bootstrap before the counted storm, and a
    **poll budget** covering the digest cadence (today's default is 30s — below the cadence's own 60s
    upper bound);
  - **Pulse's own boot smoke proves the pipeline works when driven right** `[operator-directive]`:
    "storm→10 incidents (deterministic L4)" at Pulse chunk
    `2026-07-10-incidents-floating-window-disclosure` (`andromeda-pulse/.claude/session-handoff.md`).
    The gap is drive-shape, not a broken SUT.
- **Term D — the sidecar must be BUILT.** `[operator-directive]` This was the probe chunk's *unlisted*
  precondition, disclosed in its P4 report: unbuilt, every arm would have measured the
  read-back-unreachable path instead of the one under test. The contract names it as a term rather than
  letting the next session rediscover it.
- **Term E — stay inside the load envelope.** The storm the contract prescribes must sit inside
  `contracts/pulse-load-envelope.toml`. Its documented ceiling is the DuckDB append stall under
  sustained storm: ingest keeps receiving while `duckdb.append` stops, `viz.query.traces` returns 0
  rows, and **no new incidents form until restart** — the same observable as Term C failing, from the
  opposite direction `[inferred]`.

## Why (the observed gap)

`intent.md` F11: nothing in this repo states how a verifiable Pulse must be launched.
`conductor-verify/src/spawn.rs` passes exactly one env var — a shipped test asserts
`envs.len() == 1, "only the data-dir env is set"` — and `ANDROMEDA_PULSE_L4_DETERMINISTIC` appears
**nowhere in Conductor's code**, only in prose (arch, residuals, prior scopes/plans). Meanwhile the
preceding chunk established that the live path is blocked by a *prerequisite* to the divergence it set
out to probe: an incident must exist before any workspace-key question can be asked.

## Boundaries

- **Not `ready:true`.** The later Epoch-2 entry *First live green preflight* owns the first green
  preflight against a real Pulse with journal + runs.db evidence. This chunk establishes and asserts
  the conditions; it does not claim the green run `[operator-directive]`.
- **Not Pulse process management.** Conductor's stated non-goal stands (arch §Cross-cutting Patterns
  scope law): Conductor does not launch, supervise, or configure `pulse-app`. The contract is a
  **recorded expectation the operator satisfies and preflight checks**, never Conductor spawning the
  SUT `[inferred]`.
- **Not v2-04.** `[operator-directive]` The in-lane-scenarios capability (P-067/P-072/P-079 each
  yielding a non-blocked live verdict) is **not** claimed here — its natural owner is the Epoch-6
  *Operator-gated live suite* entry. Recorded so later sessions stop re-deriving it.
- **Not the interpretation-correctness leg.** Deterministic L4 is the thing this contract asserts is
  ON; proving Pulse's interpretation with it OFF is a 0.3.0 residual with a named owner
  (`.andromeda/residuals.md`), and this chunk must not appear to close it.
- **Not a Pulse-side fix.** If a term proves unsatisfiable from Conductor's side, the deliverable is the
  named unmet term — not a change to Pulse.

## Surfaces and contracts touched

- `contracts/` — a new committed manifest for the run contract (precedent: `pulse-capabilities.toml` ·
  `pulse-load-envelope.toml`; both resolve via `default_path()` → `resolve_under` with deliberately NO
  `CONDUCTOR_*` override handle).
- `crates/conductor-verify` — the preflight readiness gate: `ReadyState` / `blocked_precondition`, whose
  named-precondition set is currently **four**. Every string stays host-path-free; `data_dir` stays
  redacted in the readiness envelope.
- `crates/conductor-core` — manifest load + bounds-check seam (where the two existing contract loaders
  live), and `UNBACKED_AUTO` if a scenario lands (below).
- Run-report envelope / `runs.db` — untouched unless a new blocked precondition needs surfacing;
  `ReportState` stays five, `LAMP_META` stays six.

## PREREQ folded in (from 2026-08-10-workspace-key-divergence-probe)

**`cargo audit` re-check — SEVENTH consecutive, deferred since `2026-08-08-sut-capability-manifest`.**
**RATIFIED** by the operator at the 2026-08-10 wrap under the L5 age trigger: the bounded wait continues
and **re-pins silently from here** — no further ratification HALT. It is an advisory-**DATABASE** fault
(`duplicate advisory ID: RUSTSEC-2026-0244`, reproduced byte-identically on 0.22.2, the latest published
— nothing to raise a floor to). Action at this chunk's gates: **run it, record the result, verify
`cargo deny check` ran green as the overlapping signal.** Do NOT raise the floor, do NOT add a
`deny.toml` ignore, do NOT edit CI. Close the deferral the moment it parses.

## CARRYs folded in

1. **The live workspace-key question is still unanswered, and this entry owns its precondition.**
   The three-arm probe could not exercise the key axis (no incident formed under any arm), so **arm 3
   — `cwd == the data dir` — leaves the Windows `\\?\` canonicalization question open**: `canonicalize`
   returns the verbatim prefix while the sidecar uses the raw `data_dir` string. This entry is what
   makes an incident actually form, so **re-run the three-arm probe once it does** and record which
   reason each arm blocks for. Evidence: `chunks/2026-08-10-workspace-key-divergence-probe/`
   `two-launch-verdict.md`; the falsified marker-less-temp-dir premise is in
   `verification-matrix.json#v2-17` `notes`.
2. **`P-073` sits in `conductor_core::UNBACKED_AUTO`** (10 entries after `P-079` retired), and that
   pin's own doc comment names **this entry** as P-073's owner: "The remaining six retire as their
   scenarios land. `P-073` is owned by the Pulse-run-contract entry." `check_scenario_backing` holds the
   pin at EXACT-SET equality in BOTH directions — a scenario naming `P-073` without shrinking the pin
   fails one way, shrinking without naming fails the other — so **the two halves must land in ONE
   commit**. Proven mechanic: the preceding in-lane chunk did exactly this for `P-079`, and the pin edit
   needs no renderer change (all 13 `UNBACKED_AUTO` consumers derive the count).

## Open questions for P3/P4 (not decided here)

- **How does preflight OBSERVE a term set on a process Conductor does not launch?** `[inferred]` The
  acceptance says preflight asserts `ANDROMEDA_PULSE_L4_DETERMINISTIC` "set on the launched pulse-app
  (not the sidecar)". Conductor cannot read another process's environment. Candidate paths for research:
  a read-back-observable signal across the 8-tool surface; an operator-declared term the contract file
  records and preflight reports; or Conductor's own inherited env as a same-shell proxy (how the probe
  actually ran — weak, and must not be dressed up as a measurement). The v2-17 precedent is binding
  here: when Conductor cannot measure a thing, the deliverable is a **named precondition stating the
  condition and its candidate causes, never a claimed measurement**.
- **Does P-073's backing take the form of a scenario TOML?** The pin doc says the six retire "as their
  scenarios land", which points at a scenario naming P-073 — but v2-18's acceptance is a preflight
  assertion, `method: integration`. P4 decides, and the answer determines whether the `UNBACKED_AUTO`
  edit is in scope for this commit.
- **Which storm shape does the canary emit today?** Whether `conductor-run::emit_canary` already
  satisfies ≥5 same-fingerprint/30s, and whether a warm-up pre-roll is needed to carry the service out
  of baseline bootstrap before the counted storm begins.
