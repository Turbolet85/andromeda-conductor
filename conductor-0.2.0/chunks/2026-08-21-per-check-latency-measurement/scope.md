# Scope — 2026-08-21-per-check-latency-measurement

**Working-route entry (verbatim intent):**
> Per-check latency measurement — sub-5s budgets beneath the closed `slo_tier` set, per-check
> `latency_ms` carried in the run-report envelope

**Version:** conductor-0.2.0 · **Epoch:** Epoch 4 — Lifecycle & delegated timing
**Capability:** `verification-matrix.json#v2-19` — *Sub-5s budget representation and per-check latency*
(`method: unit`; acceptance: "A scenario can declare a sub-5s budget and it round-trips through config,
verdict and the run-report envelope; each check carries its own `latency_ms` rather than sharing the run's
whole-loop value.")

---

## What this chunk builds

Two distinct properties, both about **timing representation**, neither about driving a live Pulse:

### 1. A sub-5s budget a scenario can declare

`SloTier` (`conductor-core/src/scenario.rs:50`) is a closed three-variant enum — `<5s` / `<20s` / `<90s`,
serde-renamed to those wire forms, with `deadline_ms()` returning `5_000` / `20_000` / `90_000`. The floor
of the ladder is therefore 5 seconds: **no scenario can today declare a deadline finer than `<5s`**, and the
four delegated-timing capabilities the NEXT route entry targets need 2s / 5s / 2s / 1s budgets.

This chunk gives a scenario a way to declare a budget **beneath** the tier it sits in, and carries that
declaration through the three surfaces the acceptance names: **config** (TOML → serde → garde), **verdict**
(the deadline the SLO comparison actually applies), and the **run-report envelope**.

- **VERIFIED (P3):** keep `SloTier` closed at three variants and add a *separate, optional* finer budget
  beneath it — NOT widen the enum. Basis: `architecture.md` §Data model conventions pins `slo_tier` as "a
  closed TEXT enum over exactly `<5s`/`<20s`/`<90s`"; the code-graph measures **150 refs across 18 files**
  behind `SloTier`/`deadline_ms`; security, design and layouts each independently forbid widening the
  tier's rendered value set.
- **VERIFIED (P3), with a placement correction:** budget ≤ its tier's `deadline_ms()` is a garde
  cross-field invariant — and it must sit on **`Scenario::expected`, one altitude up**, because
  `ExpectedCheck` carries no back-reference to the scenario's tier and garde 0.22.1's `custom` is
  field-level only. The shipped template is `fault_phases_are_silent` on `Scenario::phases`.

### 2. Per-check `latency_ms`

Today every check of a scenario is evaluated against the **same** pair of instants. In
`conductor-run/src/lib.rs`, `scenario.expected.iter().map(|check| evaluate_check(check, …, emitted_ms,
observed_ms))` passes ONE `emitted_ms` / `observed_ms` to every check, then `.max_by_key(severity_rank)`
collapses the outcomes to a single `CheckOutcome`, and `to_run_record` emits ONE `RunRecord` carrying ONE
`latency_ms` (`conductor-verify/src/record.rs:22-46`). That single value is the run's whole-loop span — it
is the same number no matter which check produced the row, which is precisely what v2-19 calls out.

This chunk makes each check carry its own measured latency into the envelope.

- **`[premise-corrected: one read-back pass per scenario means one observation instant, so per-check
  latency cannot mean per-check read-back instants]`** — `observe()` is called **once** per scenario
  (`conductor-run/src/lib.rs:371`) and composes **one** `Observation`; every check grades a *projection*
  of it (`conductor-verify/src/extract.rs:53-62` — `text` for `Exact`/`Contains`/`Absent`,
  `evidence_count` for `CountAtLeast`). Timing each check independently would require per-check read-back
  calls — a read-back-architecture change far outside a `method: unit` capability. **What IS achievable
  and genuinely different from today:** every check's outcome reaches the envelope with its own deadline
  comparison, instead of only the `max_by_key`-chosen worst check's (today the other checks' verdicts,
  observed values and SLO outcomes are discarded entirely at `conductor-run/src/lib.rs:414-437`). For the
  per-check half to be non-vacuous the **budget must be declarable per check**, since a shared instant
  pair makes per-check latencies equal unless the *deadline* differs — P4 resolves this (research.md
  Open question #1).

---

## Boundaries (what this chunk is NOT)

- **Not a live-Pulse leg.** v2-19's `method` is `unit`. No fresh-dir leg, no operator gate, no harvest
  binary. (The live proof of real budgets is the NEXT route entry, v2-20 — which carries a `BLOCKED-ON`
  annotation: three of its four capabilities have no timing observable on either side.)
- **Not a re-tiering of the catalog.** Existing scenarios' `slo_tier` values stay as they are; a sub-5s
  budget is an *addition* a scenario MAY declare, never a forced migration.
- **Not new read-back surface.** No new MCP tool, no change to what Pulse is asked.
- **Not the load-envelope rate-term fix** (`occurrences × samples`) — that CARRY belongs to the Epoch-6
  *Coverage completeness gate* entry, not here.

## Surfaces and contracts touched

| Surface | File | Nature of the touch |
|---|---|---|
| Scenario model + garde | `conductor-core/src/scenario.rs` | the budget declaration + its bounds |
| SLO evaluation | `conductor-verify/src/slo.rs` (`evaluate_slo` / `evaluate_check` / `SloOutcome` / `CheckOutcome`) | the deadline actually compared against |
| Record assembly | `conductor-verify/src/record.rs` (`to_run_record`) | per-check latency into the row |
| Run composition | `conductor-run/src/lib.rs` (`execute_scenario`, the `max_by_key` collapse, `manual_record`) | where the shared whole-loop pair is handed to every check |
| Envelope contract | `conductor-core/src/run_record.rs` + `architecture.md` §Standard Contracts | the serialized envelope shape |
| Storage | `conductor-report/src/db.rs` (`runs`, PK `(run_id, scenario)`) | see the open fork below |
| Render | `conductor-report/src/report.rs:147` · `conductor-cli/src/render.rs:185` | latency display |
| Redaction allowlist | `conductor-core/src/redact.rs:60-61` | any new field name must be admitted deliberately |

## Open forks for P4 (surfaced, not decided here)

1. **Where per-check latency lands in the envelope.** The `runs` table's PK is `(run_id, scenario)` — one
   row per scenario — while `architecture.md` §Occupied Resources calls `runs` "the per-check index". A
   genuine row-per-check changes that key; a per-check sub-structure inside one row does not. Arch also
   states column types are "fixed on first write (no migration framework to coerce later)". P4 must pick
   and cite; the envelope is a Standard Contract, so whichever lands must keep the documented shape honest.
2. **What the collapse means once checks differ.** `max_by_key(severity_rank)` picks the worst verdict; if
   checks now carry distinct latencies, the chosen row's latency must be attributable to a named check
   rather than silently inherited.
3. **VERIFIED (P3) — observability today is thin by construction:** only 11 of 35 catalog scenarios declare
   any `[[expected]]` check, and exactly ONE (`high-severity-log-capture`) declares two. So "checks carry
   *different* latencies" is demonstrable on a single catalog scenario — P4 should expect to prove the
   property on constructed fixtures at unit tier, not on catalog breadth.

## Folded annotation — PREREQ (from the working entry)

**Re-check `cargo audit` — 36th consecutive.** Standing deferral since
`2026-08-08-sut-capability-manifest`; ratified at the `2026-08-10-workspace-key-divergence-probe` wrap and
re-pins silently. Basis RE-VERIFIED at `2026-08-21-severity-lifecycle-live-proof`, which moved no
dependency at all (`Cargo.toml` and `Cargo.lock` byte-untouched, zero package admission), so the
advisory-DATABASE fault remains the sole cause and the audit↔deny overlap stays verified rather than
assumed; that wrap discharged the 35th in the PURE auto-satisfy form the 34th predicted — the first such
fire.

**PROBE-AUTO-SATISFY signature** = `cargo audit` true exit 1 whose first diagnostic line is
`duplicate advisory ID: RUSTSEC-2026-0244` AND `cargo deny check advisories bans licenses sources` true
exit 0. Reproduce it byte-identically → the pin is satisfied by the one-line record
`probe unchanged, 36th consecutive`, no basis re-authoring. ANY deviation (changed diagnostic, moved exit
code, overlap shift, or a dependency delta that ADMITS a package) restores the FULL form. Remedy stays the
bounded wait — no floor raise, no `deny.toml` ignore, no CI edit; close the moment it parses.
Full rationale: `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

**VERIFIED (P3) — zero dependency delta.** The probe was reproduced this session byte-identically:
`cargo audit` exit 1 with first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`, `cargo deny check
advisories bans licenses sources` exit 0, `Cargo.toml`/`Cargo.lock` byte-untouched. No design under
consideration admits a package, so the auto-satisfy basis holds; if implement admits one, the FULL form
is restored.

## Also in flight (recorded, not scoped)

A stale doc comment in `crates/conductor-run/tests/baseline_harvest.rs` ("under a FRESH data dir 0 is
correct by construction") was falsified at the last chunk and has no natural route owner. It is a one-line
fix for whichever chunk next touches the harvest tests. This chunk does not plan to touch them; if P3 finds
it does, the fix rides along.
