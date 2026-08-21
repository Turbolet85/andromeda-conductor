# Scope — 2026-08-21-severity-lifecycle-live-proof

## Working-entry intent (verbatim surface)
severity-lifecycle live proof — auto-resolve and resolution summary observed through read-back
(P-019..P-023, P-059, P-060)

## What this chunk does
Live-prove the severity-lifecycle scenario family against a live deterministic-L4 Pulse, in the shape the
five prior family live proofs established: measure which of the family's claims MCP read-back actually
carries under deterministic L4, grade what is gradeable at the harvest tier (Pulse's own corpus/ledger
evidence), retire structurally-ungradeable checks to declare-only WITH the measurement recorded (never a
vacuous green), and land the evidence trail (per-leg journal + runs.db rows + leg-verdict.md under this
chunk's evidence/).

The family's two headline observables, per the working entry:
1. **Auto-resolve observed through read-back** — Pulse's own lifecycle (120s idle / 30s resolver ticks)
   empties/flips the incident; the run must observe that transition via read-back rather than assume it.
2. **Resolution summary observed through read-back** — whatever resolved-incident summary surface the
   corpus tools actually expose, observed live.

- VERIFIED: the family on disk is FIVE catalog TOMLs: `severity-tier-autonomous.toml`
  (P-019, P-020, P-060) · `severity-tier-suggested.toml` (P-019..P-021, P-060) · `severity-tier-curious.toml`
  (P-019..P-021, P-060) · `incident-auto-resolution.toml` (P-022, P-059) · `ack-cooldown.toml` (P-023).
- Target capability: **v2-16** ("severity-lifecycle full pass") — standing acceptance: a full pass observes
  auto-resolve firing and a resolution summary through MCP read-back; lifecycle timing asserted hard
  pass/fail; severity choice asserted CalibrationRegion, never hard-failed on exact values.

## Boundaries
- No Pulse-side changes: anything the SUT would need to build/fix is recorded as SUT intake, not patched.
- Scope law + trust boundary unchanged: no new inbound listener; loopback gRPC/MCP-client model only.
- Live legs are operator/local-gated, never CI; determinism discipline holds for everything CI-visible.
- No un-verifying of prior ledger entries; premise corrections (if research disproves an acceptance
  premise) take the ratified PREMISE-CORRECTION channel with operator ratification at P4/P5.

## Surfaces / contracts it touches
- `scenarios/` — the five family TOMLs; expected blocks retire declare-only per measurement (the
  research-measured retirement grounds are in `research.md`; the leg confirms them live).
- VERIFIED: `conductor-run` — the `route_read_back` `AutoResolved` arm (`lib.rs:372`/`444-463`,
  unit-pinned on all three arms, unexercised live) is squarely this family's subject. No production
  change expected — the chunk's Conductor-side delta is scenarios + tests + evidence.
- VERIFIED with one correction: `conductor-verify` read-back is a ONE-SHOT `observe()` over the
  ACTIVE incident list (`extract.rs:76`), composing list text + each active incident's report
  markdown + telemetry slice. [premise-corrected: `mark_incident_resolved` is a RESOLVE write
  (`client.rs:147`, Pulse `tools.rs:445-467` sets status=resolved), NOT an ack — the ack-cooldown
  TOML's own Q2 note already records that no ack tool exists in the four-tool contract.]
- `conductor-0.2.0/verification-matrix.json` — v2-16 claim link (+ any operator-ratified acceptance
  concretization at claim time).

## Premises — CLOSED by P3 research (details + citations in `research.md`)
- VERIFIED + sharpened: severity is fixture-pinned under det-L4 (`"severity": "autonomous"` →
  `Severity::Error` → list string `"error"`), and the list/report vocabulary (`info/warn/error/critical`,
  lowercase status labels) is DISJOINT from the tier tokens — the three tier `Contains` checks are
  structurally ungradeable through read-back on two independent grounds.
- VERIFIED + corrected: auto-resolve is real SUT mechanism (120s window, 30s `AutoResolveObserver`
  ticks, corpus persisted on transition) and `query_incident_list` returns ACTIVE ONLY
  (`load_active_incidents`; Pulse's own test `query_incident_list_returns_active_incidents`) — so
  auto-resolve is observable as ABSENCE-from-list + the retriggered NEW incident, never as a
  "Resolved" token. [premise-corrected: the RESOLUTION SUMMARY is UNREACHABLE under deterministic L4
  at SUT HEAD `efabe8e` — nothing constructs `DigestKind::ResolutionSummary` (the enum's only refs are
  match arms) and the canned fixture pins `is_resolution_summary: false`, the only two triggers of
  `attach_resolution_summary_to_incident`. The v2-16 acceptance clause naming it needs an
  operator-ratified premise correction at claim.]
- VERIFIED + sharpened: Tier-1 digests fire on AUTONOMOUS cues alone (`coordinator.rs:390`); Tier-2
  acceleration is DISABLED on CpuPrimary hosts (`run_one_coordinator_cycle` early-return); Tier-3
  baseline digests DO carry recent attention cues into incident creation. The shipped tier-scenario
  shapes (8@100% / 12@40% / 10@10% error) sit under the sample floors and confidence bands
  (conf = samples/100; autonomous needs ≥5x + conf ≥0.9) — re-shape onto the proven
  converged-baseline+spike recipe. The shipped `incident-auto-resolution` trigger (8 identical
  exceptions) sits in 5≤8<10: cue floor cleared, Autonomous band missed — NO incident forms as shipped
  (the canary's own 2026-08-15 lesson); re-shape ≥12.
- VERIFIED with mechanism: the 2026-08-20 leg measured 375 `error_rate_spike` cues from the canary's
  FROZEN short-term error EWMA (30s EWMA, updates per sample — no samples ⇒ frozen elevated) refreshing
  `updated_at` indefinitely; each cue-bearing digest also refreshes via `observe_reemission`. So the
  scenario's OWN incident resolves only if its service's cues STOP: storm cues stop when the 60s count
  window empties; error-rate cues stop only if the short EWMA ends BELOW threshold — the stimulus needs
  an OK-span dilution tail between storm and silence. The CANARY incident (isolated identity, frozen
  elevated EWMA) will NOT idle out on its own, so an EMPTY active list — the `AutoResolved` arm's live
  firing — is reachable only if the canary incident is cleared (`mark_incident_resolved`, with the
  persist-loop resurrect hazard measured) or stays an honest limit as on 2026-08-20.
- NEW (research-surfaced): `CountAtLeast` grades against `evidence_count` = summed `span_refs`, and
  Pulse's incident producer writes `span_ids: Vec::new()` — observed is ALWAYS "0", so both family
  `CountAtLeast "2"` checks hard-fail regardless of lifecycle truth (a mis-aimed instrument, never a
  vacuous pass). New-not-reopen itself HOLDS at source (`list_active`-scoped dedup ⇒ a resolved
  incident is not found ⇒ a NEW one is created).
- NEW (research-surfaced): `ack-cooldown`'s five phases declare NO emission tables, and
  `PhaseSpec.emission` DEFAULTS to one plain trace — the scenario emits 5 OK spans, forms nothing, and
  its Hard check cannot pass as shipped; no ack mechanism exists in the four-tool contract (the TOML's
  own Q2 note). Retirement with measurement.
- NEW (research-surfaced): a workspace-global `ReflectionTrend` incident (dedup one-per-workspace,
  refreshed per Reflection digest) can appear on long legs and hold the active list non-empty — its
  cadence (`reflection_seconds`) is config-sourced; the leg measures whether it fires inside the
  family's windows.

## PREREQ folded (from the working entry; hypothesis re-verified)
**35th consecutive `cargo audit` re-check**, standing deferral since `2026-08-08-sut-capability-manifest`,
operator-ratified at the `2026-08-10-workspace-key-divergence-probe` wrap (re-pins silently); basis
re-derived at `2026-08-20-read-back-seam-survivors-closed` (edge-only `assert_fs` lock delta, ZERO package
admission, deny observed exit 0). **PROBE-AUTO-SATISFY form**: run `cargo audit` + `cargo deny check
advisories bans licenses sources`; signature = audit true exit 1 with first diagnostic line
`duplicate advisory ID: RUSTSEC-2026-0244` AND deny true exit 0 → the pin is satisfied by the one-line
record `probe unchanged, 35th consecutive`, no basis re-authoring. ANY deviation (changed diagnostic,
moved exit code, overlap shift, a dependency delta that ADMITS a package) restores the FULL form. Remedy
stays the bounded wait — no floor raise, no `deny.toml` ignore, no CI edit; close the moment it parses.
Named rationale artifact verified present:
`conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md` (11,202 bytes, 2026-08-14).
