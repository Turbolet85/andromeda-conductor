# Session Handoff

**Last Updated:** 2026-08-16T12:08:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **17 ahead** after this chunk commit)
**Status:** clean
**Last Commit:** 2026-08-16-fault-application-spans — the three fault spans open where the faults actually
are, and the spec said conductor-faults

## Position
- Done: **2026-08-16-fault-application-spans** — `fault.silence` / `fault.ramp` / `fault.port_occupier` now
  emit; the obvious reading of obs-plan §4 turned out to be unreachable and the spec moved to measured truth.
- Next: **fingerprint-storm live proof** — identity triple, storm cue thresholds and exactly-one-incident
  coalescing (P-017, P-018, P-074). `/andromeda-phase` to promote + plan.

## Work done
8 source/config files + `Cargo.lock`, 0 new source files, +436/−17. `conductor-timeline` gained a per-phase
observer (`run_timeline_observed` + `PhaseWindow`); `run_timeline_with` keeps its signature and delegates,
with `timeline.execute` moved onto the new fn so exactly one span is emitted. `conductor-run` classifies a
phase as silence/ramp and builds the span (`classify_fault` · `ramp_factor` · `fault_span`), created but
NEVER entered so `emit.batch` keeps `timeline.execute` as its parent. `conductor-faults` took its first
`tracing` dep and holds `fault.port_occupier` across `occupy`→`release`. Five names appended to
`ALLOWLISTED_FIELDS` (31 → 36) — without them every attribute drops silently. Gates green in 2 iterations:
workspace `--profile ci` **597/597** zero retries (588 before, +9), doctest 0, `clippy -D warnings` clean,
`cargo deny` true exit 0 across all four classes, goldens byte-unchanged.

## Drift resolved
7 doc-agents / 18 detectors, **6 proposals · 6 applied · 0 rejected · 0 escalations** — the tightest wrap of
the version. All six are obs-plan (§4 ×3 + §1 ×2 + §6 ×1), two of them `dependent-of` duplicates the fan-out
caught restating the same claim in §1's tables. Cascade: `rules/observability.md` gained a fault-span
placement + level bullet; `obs-summary.md` recomputed with **no delta**; a grep of all seven masters and the
three preserve-verbatim curation homes for the retired wording returned **zero hits**. Full record:
`.andromeda/runs/2026-08-16T11-53-35-wrap/fanout-results.md`.

## Notes
- **The chunk's spec was wrong about where its own work goes, and the graph proved it.** obs-plan §4 said
  each fault span "wraps the fault application phase in `conductor-faults`" — but that crate is an ISLAND:
  0 crate edges in either direction, 0 references to `PortOccupier`/`AbruptSilence`/`EmissionGap`/
  `BurstyTrain` from outside itself. Silence and ramp are declarative phase data (`occurrences: 0` /
  `EmissionShape::Ramp`), so the spans had to open from the run path. Instrumenting where the spec said
  would have put them on a path no scenario run reaches.
- **`ramp_factor` had no computable source** — spec said float 0.0–1.0, the shape carries three integers.
  Shipped as the normalized SIGNED slope `(to−from)/max(from,to)`, −1.0..1.0, so a rise and a fall are
  distinguishable (operator decision at P4). obs-plan §4 amended.
- **`fault.port_occupier` ships parentless, deliberately.** No run path applies the occupier, so its
  `timeline.execute` parentage is recorded CONDITIONAL rather than claimed. It carries `fault_type` + `port`
  only (nothing else is knowable at bind time); the realized hold is witnessed on `message` at `debug`.
- **A fourth spec claim was surfaced by /implement and dispositioned here:** obs-plan §6 assigned "fault
  application" to `debug`, but the spans ship at `info` — every sibling span is `info`, and at `debug` they
  are invisible under the default filter, so the chunk's own acceptance could not hold. Table amended with
  the reasoning recorded beneath it.
- **No capability claimed.** `v2-15` was considered and declined; its `notes` now record that no
  port-occupier driver exists anywhere, so its claiming chunk must WRITE one. That finding is also pinned as
  a CARRY on the `connection-lifecycle live proof` entry.
- **`cargo audit` — 23rd red**, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1.
  Re-pinned COMPACT with the basis re-verified: `Cargo.lock` moved (+2 lines) but both were dependency EDGES
  inside `conductor-faults`' existing entry with **zero new `[[package]]`**, so the audit surface is
  unchanged; `cargo deny` verified green as the overlap.
- **Curation: T1 0 · T2 1 · T3 0** (3 filtered). The operator's freshness candidate scored over Filter 1's
  bar against an entry the previous chunk extended the same morning, so it landed as an in-place additive
  extension carrying the two genuinely new facets: a bare `status` passes every phase-P5 mechanical check
  when the plan asserts at the test tier, and MINT-THEN-READ is the positive procedure.
- **Heads-up (not acted on):** `playbook.md`'s 2026-06-20 deferred-span rule names `fault.silence` /
  `fault.ramp` / `fault.port_occupier` as deferred build-sequencing. The rule is not false — its fault-span
  half is simply spent now that the spans emit — but a future wrap could misapply it to dismiss a genuine
  fault-span proposal. Playbook is an operated artifact, not a spec master, so it was flagged rather than
  edited.
- **Honest limit on the smoke:** a Pulse-free run emits NO fault span, because `execute_scenario` returns
  Blocked before the timeline. The smoke's artifact confirms it — zero `fault.*` AND no `timeline.execute`.
  The artifact-level sighting belongs to the live-proof entries.
- **Last failed command:** none.
