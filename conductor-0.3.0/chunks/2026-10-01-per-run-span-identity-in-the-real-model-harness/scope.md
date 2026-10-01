# Scope — Per-run span identity in the real-model harness

**Marker:** `2026-10-01-per-run-span-identity-in-the-real-model-harness` · **Version:** conductor-0.3.0 ·
**Epoch:** Epoch 5 — Polish & ship · **Base:** `2c97d3b` (W182)

## The working entry (verbatim title + scope hint)

Per-run span identity in the real-model harness — two same-seed drives inside one Pulse buffer window both land,
held by a test

## What this chunk builds

- Make the real-model harness's scenario emission land when two drives of the SAME scenario + seed are fired inside
  one Pulse buffer window — the d2 gap of the 2026-10-01 series.
- The fix takes ONE of the two shapes the entry's CONTEXT names (the choice is a P4 fork): **(a)** each drive's span
  identity (`trace_id` / `span_id`) is unique per run, or **(b)** the harness enforces a measured minimum gap between
  same-seed drives.
- A test that holds it: two same-seed drives inside one buffer window both land (the entry's own acceptance).
- (val-1, intent-incomplete, amended at P5) An operator-attended LIVE witness, which the operator chose at P4
  ("measured over inferred"). It runs two `conductor run` drives of one deterministic exception scenario at the same
  seed. They run against one deterministic-posture `pulse-app` on the sha256-verified `a2addb3` binaries, inside
  Pulse's retention window, graded on Pulse's own log. This is not a real-model series and not a `v3-09` drive. It is
  the only proof of the production `execute_scenario` wiring, which CI cannot reach because the ready path emits to
  the fixed `:4317`.

## Folded freight (from the entry — hypotheses, re-verified at fold time where a coordinate is named)

- **EVIDENCE (verbatim):** measured at `2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix` d2 —
  Conductor's exception span identity is a pure function of the scenario seed
  (`crates/conductor-emit/src/exception.rs:177-187`, seed 4317033); d2's scenario emission came ~7 min after d1's
  while Pulse still buffered d1's rows, and all 36 of its appends were refused (`duckdb.append`
  `reject_reason=append_failed`, `buffer.tick` `append_rejections` 36); Pulse evicted d1's rows 19:48-19:51Z and d3
  landed (that chunk's `evidence/attempt-ledger.md`).
  - Coordinates re-verified at fold: `exception_trace_request` sits at `exception.rs:176-187` at `2c97d3b` and seeds
    `ChaCha8Rng::seed_from_u64(seed)` for `trace_id`/`span_id`; `seed = 4317033` is
    `scenarios/real-model-interpretation.toml:42`; the d2 row of `attempt-ledger.md:85` carries the 36 refusals and
    the eviction window.
- **Mechanism claim (measured-marked in the freight, kept verbatim; VERIFIED at P3):** "the scenario's span identity is a
  deterministic function of its seed (`crates/conductor-emit/src/exception.rs` `exception_trace_request`), so d2
  re-sent d1's `(trace_id, span_id)` while Pulse's buffer still held d1's rows (evicted 19:48-19:51Z)". Re-derived:
  Conductor side `dispatch.rs:78` `emission_seed(scenario.seed, phase, occurrence)` → `exception.rs:185-187`;
  Pulse `a2addb3` side `spans` `PRIMARY KEY (trace_id, span_id)` (`buffer/src/schema.rs:36`), a replayed identity
  rejects the whole batch at flush (Pulse's own test `appender.rs:996`), logged as `duckdb.append`
  `reject_reason=append_failed` + `append_rejections` (`consumer.rs:78-84,272`); retention 600 s default with a 100 s
  sweep (`retention.rs:41-51`, `contract.rs:109`) explains the 19:48-19:51Z eviction (research.md).
- **CONTEXT (verbatim):** the fix makes each drive's identity unique, or enforces a measured minimum gap, with a test
  that two same-seed drives inside one buffer window both land (operator relay
  `conductor-wrap-v309third-2026-10-01` §2, entry 1).

## Boundaries

- The emission's SHAPE stays a function of scenario + seed (the "determinism is the bar" invariant: same
  scenario+seed ⇒ same stream shape). VERIFIED with its open half resolved: the architecture's prose names timing
  as the seeded shape, but committed tests pin span IDENTITY at the dispatcher tier —
  `dispatch_wire.rs:437-472` asserts identical `(trace_id, span_id)` across two same-seed drives and the three
  `dispatch_wire__*` goldens carry identity hex (`dispatch_wire.rs:66-90`). So a dispatcher driven without a
  run-scoped input must keep its identity bytes; any per-run identity enters only on the production path.
- The d2 canary spans surfaced while the scenario's were refused. VERIFIED: the canary's identity base is the wall
  clock — `base = now_ms()` (`canary.rs:294`), marker `ConductorCanary_{now_ms()}` (`canary.rs:285`); the live
  lifecycle test does the same (`lifecycle_live.rs:80`). That is the in-tree precedent for shape (a).
- [premise-corrected: identity replay is not exception-only, and content shares the seed — `gen_id` 17 sites over 7
  emit modules; `pii_harvest.rs:104-119` re-derives the PII corpus from the same per-slot seed] The replay applies to
  EVERY span the dispatcher emits (`trace_request` · `error_trace_request` · `exception_trace_request` · latency ·
  pii · rate · topology), all drawn from `emission_seed`; and that one seed also draws CONTENT (the PII corpus a
  live harvest re-computes, latency durations, rate counts). So the fix changes span identity only — never the seed
  content is drawn from — and reaches every scenario on the production path (`execute_scenario`, which both live
  legs and the GUI funnel through), deterministic and real-model alike. Logs and metrics are untouched: Pulse keys
  them on a timestamp + `seq` (`schema.rs:66,82`), so they cannot replay this way.
- No Pulse change — Pulse is the SUT. Reading Pulse's source on disk is in scope; building it is not without asking.
- No new real-model series and no `v3-09` re-grade in this chunk — `v3-09`'s next step awaits the founder (handoff).

## Operator directives for this take-up (2026-10-01, verbatim intent)

- Founder rulings: evolve-diagnose waits; nothing deferred.
- A change to span identity is Conductor emit behaviour: if the playbook names it a Boundary widening, ask as one for
  the founder's live word.
- Any Pulse launch / model run is the operator's slot and needs the operator's `:4317` grant.
- The Pulse tree is pulse-builder-busy: ask before any Pulse build.

## CI (Setup 5a)

- `2c97d3b` (the last wrap, the only sha since the last flip): **verdict not yet available** — CI#36926075951
  `in_progress`, checks 3/3 started, the oldest running (A11y gate) at 157 s when read 2026-10-01 ~21:07Z. Not a red,
  not folded as green; re-read at P5.
- Re-read at P5: `2c97d3b` **verdict: green**, checks 3/3, wall 697 s, CI#36926075951 completed/success. Nothing to
  fold.
