# Scope — Dispatcher determinism goldens

**Marker:** `2026-08-13-dispatcher-determinism-goldens`
**Version:** conductor-0.2.0 · **Epoch 2** — Live-path enablement
**Working entry:** _Dispatcher determinism goldens — same scenario and seed yielding an identical emission
stream shape over the new per-phase dispatcher_

---

## What this chunk builds

The determinism bar — "same scenario + seed ⇒ same stream shape" (arch §Design Philosophy) — is currently
frozen over the **phase-transition** stream only. `2026-08-11-faithful-emission-dispatcher` introduced the
per-phase emission dispatcher (`EmissionSpec { signal, occurrences, shape }` + a caller-supplied hook the
scheduler fires *inside* each phase's own window), and both committed goldens came back **byte-identical**
— correctly, because the pacing change preserved the transition stream exactly. That is the gap: **the
emission stream the dispatcher now produces is not frozen anywhere**, so an RNG/ordering/shape regression in
the dispatcher would pass every gate.

This chunk closes that gap: extend the determinism-replay harness so the **emission** stream — the sequence
of dispatched emissions with their per-phase identity and ordering — is captured as a committed golden under
the same seed discipline the transition golden already uses, and generalize the replay property across the
seed space so determinism is asserted, not assumed.

## Boundaries

- **In:** the determinism/replay test surface over the shipped dispatcher; the load-envelope close-out
  (below); the `cargo audit` re-check (below).
- **Out:** changing the dispatcher's behavior or the emit primitives (that chunk shipped and its gates are
  green) — this chunk *freezes* the behavior, it does not redesign it. Out: live-Pulse legs (Epoch 2's
  later entries own those). Out: new scenario families or new `EmissionShape` variants.
- A golden that must be regenerated because the dispatcher is genuinely wrong is a **finding**, not a
  rubber-stamp: record it rather than accepting the new bytes silently.

## Surfaces / contracts touched

- `crates/conductor-timeline/tests/replay.rs` + `tests/snapshots/` — the existing insta goldens
  (`replay__fixture_seed_424242.snap`, `replay__fixture_seed_7.snap`), which freeze `Vec<PhaseTransition>`
  through the full `Scenario::from_toml_str → PhaseTimeline::from → run_timeline` pipeline. `[inferred]`
- `crates/conductor-timeline/tests/pacing.rs` — the shipped `run_timeline_with` hook contract and its
  `EmissionPoint` stamping helper, the natural capture point for an emission-stream golden. `[inferred]`
- `crates/conductor-core/src/load_envelope.rs` — `check_load_envelope` (the CARRY below).
- `contracts/pulse-load-envelope.toml` — the `[envelope]` terms + the two `[[exempt]]` entries.
- Determinism discipline: virtual clock (`start_paused`) only; never a `std::time` stamp inside a golden
  (arch §Cross-cutting Patterns · the journal-stamp invariant). `[inferred]`

---

## CARRY (from `2026-08-11-faithful-emission-dispatcher`) — the load-envelope close-out

**This entry owns it, and the contract itself names it.** `contracts/pulse-load-envelope.toml:40-42` states
that both `[[exempt]]` entries AND the switch from total-duration to **emitting-phase** duration "retire at
the Epoch-2 Faithful emission dispatcher entry". That chunk deliberately declined it (plan step 8, ratified
at its P5 review) to stop the version's largest chunk widening further, and shipped only the derivability
re-wording.

What is now true: `EmissionSpec::occurrences` exists, so a per-phase rate **is** computable, and both
`crates/conductor-core/src/load_envelope.rs` and arch §Occupied Resources record the terms as *derivable but
not yet asserted*.

### PREMISE-CORRECTION (2026-08-13, this chunk's P3 research)

**The CARRY's stated mechanism is falsified; its stated GOAL is reachable by a different term.** The CARRY
and `contracts/pulse-load-envelope.toml:40-42` both assert that summing the gaps of emitting phases lets both
exemptions retire "on their own merits". Computed over all 35 committed scenarios against the 600 000 ms
ceiling:

| scenario | total | SUM(emitting) | MAX(single emitting phase) | peak rate |
|---|---|---|---|---|
| `activity-floor` | 3 900 000 | **900 000 — OVER** | 300 000 — inside | 0.20/s |
| `incident-auto-resolution` | 731 000 | **610 000 — OVER** | 600 000 — inside (at the boundary) | 1.60/s |

So the summed re-scope is a **no-op on the gate's verdict** (the same two scenarios breach before and after)
and retiring the exemptions under it turns the gate **red**. The reason is visible in `activity-floor`: its
three `train-active-*` phases are 300 000 ms each and genuinely emit, but they are separated by 10-minute
quiets — **summing disjoint bursts is not "sustained"**, which is precisely what its exemption reason said.

**Resolved at P5 review (operator-approved):** assert the envelope's own prose instead — a **per-phase
sustained-storm** term. Catalog-wide the longest single emitting phase is 600 000 ms (exactly at the ceiling,
not over) and the highest per-phase rate is 4.00 spans/s against the 10 000 spans/s term. **Zero phases
breach, so the ledger genuinely retires to `[]`** and the contract's recorded end state becomes true.

### The three linked pieces (as corrected)

- **(a)** Re-scope the asserted term from total `gap_ms` to a **per-phase joint bound**: no single phase with
  `occurrences > 0` may exceed `max_sustained_storm_ms`, and no such phase's rate
  (`occurrences ÷ gap_ms/1000`) may exceed `max_sustained_rate_spans_per_s`. **Both** consumers move together
  — `check_load_envelope` @ `load_envelope.rs:227` AND `LoadEnvelope::classify` @ `load_envelope.rs:155`
  (→ `conductor-run::classify_run` @ `lib.rs:412` → the `[ENVIRONMENT-SUSPECT]` caption) — so the static gate
  and the per-run judgment never measure different things. `max_scenario_duration_ms` inverts to
  recorded-but-unasserted, the standing the rate terms are vacating.
- **(b)** Retire **both** `[[exempt]]` entries; the ledger becomes `[]`. `exempt` is `#[serde(default)]`, so
  removing the tables needs no schema change.
- **(c)** Reconcile the contract's own sentence + the `load_envelope.rs` module doc, recording that the term
  landed as per-phase sustained storm rather than summed emitting duration, **with the falsifying evidence**.

**The ledger is EXACT-SET in both directions** (`check_load_envelope` fails on `unpinned`, `rotted`, AND
`lost_subject`), so **(b) must land in the same commit as (a)** or the gate fails as ledger rot.

## PREREQ (from `2026-08-11-faithful-emission-dispatcher`) — `cargo audit` re-check

**ELEVENTH consecutive**, deferred since `2026-08-08-sut-capability-manifest`, re-pinning **silently** under
the operator's L5 ratification at the 2026-08-10 wrap (no further ratification HALT). Re-proven each time on
**0.22.2** (the latest published): byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 —
an advisory-**DATABASE** fault with nothing to raise a floor to.

The standing basis held **literally** at the last check: audit *surface* unchanged (`Cargo.lock` moved 4
lines, all edges inside `conductor-run`'s existing package entry, **zero new `[[package]]`**) and
`cargo deny check` verified green (advisories/bans/licenses/sources, exit 0) as the overlapping signal.

Remedy is the **bounded wait alone**: re-run it, record the result, verify `cargo deny` ran green. Do **NOT**
raise the floor, do **NOT** add a `deny.toml` ignore, do **NOT** edit CI. Close the deferral the moment it
parses. (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09/-08-10 · security-plan
§Dependency Security.)

**Note for this chunk specifically:** a test-only chunk plus a `check_load_envelope` re-scope is expected to
move `Cargo.lock` not at all — so the standing basis should hold trivially. Verify rather than assume.
`[inferred]`
