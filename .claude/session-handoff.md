# Session Handoff

**Last Updated:** 2026-08-16T10:12:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **16 ahead** after this chunk commit)
**Status:** clean
**Last Commit:** 2026-08-16-canary-fingerprint-derivation-aligned — the derivation aligned, the canary
re-aimed onto incident freshness, and preflight reached `ready:true` for the first time

## Position
- Done: **2026-08-16-canary-fingerprint-derivation-aligned** — the chunk's own premise was falsified at
  research, the goal kept and the mechanism changed, and the live leg measured the gate green.
- Next: **Fault-application spans** — the first markerless entry in Epoch 3 (silence / ramp / port-occupier
  phases observable beneath the timeline span). `/andromeda-phase` to promote + plan.

## Work done
9 source/config files + `Cargo.lock`, 0 new files. `exception.rs` swapped FNV-1a-over-frame-functions for
**Pulse's own derivation** (blake3 over `exception_type` + `\0` + the normalized stacktrace, first 16 bytes →
32 hex), porting Pulse's `normalize_stacktrace`/`normalize_frame` semantics and deleting `Fnv1a`.
`preflight.rs` re-aimed `assert_canary` onto **incident freshness**, `extract.rs` gained
`opened_at_unix_nanos`, `conductor-run` gained the `std::time` emission stamp and lost both stale doc twins.
`blake3 = "1"` landed as a NORMAL dep; `deny.toml` gained one justified `BSD-2-Clause` allow.
Gates green in 2 iterations: workspace `--profile ci` **588/588** zero retries (584 before, +4), doctest 0,
`clippy -D warnings` clean, `cargo deny` true exit 0 across all four classes.

## Drift resolved
7 doc-agents / 18 detectors, **16 proposals · 14 applied · 2 rejected · 2 escalations resolved**.
Amendments: `architecture.md` ×5 (Stack hashing row · [Read-Back Dependency Posture] rewritten ·
§Standard Contracts prose · its FIVE-precondition enumeration · Build-system deny.toml note),
`security-plan.md` ×5, `test-plan.md` ×3, `obs-plan.md` ×1. Both escalations were operator-ratified: the
locked-decision reversal (playbook line 97) and a NEW playbook rule for admitting a dependency under a red
audit. **Two obs proposals were REJECTED** — they would have removed `retrieve_telemetry_slice` from
Conductor's client tool lists, but only the canary stopped calling it while per-check extraction still does;
the `dependent-of` pair was rejected atomically and the report gap that misled them was closed. Cascade:
CLAUDE.md warnings + `stack.md` + `rules/security.md` + `security-summary.md` re-derived; preserve-verbatim
homes routed to curation. Full record: `.andromeda/runs/2026-08-16T09-59-45-wrap/fanout-results.md`.

## Notes
- **`ready:true` — the first green preflight in the project's history.** Live leg against Pulse at HEAD
  `d090314`, fresh data dir, deterministic L4 confirmed active. The re-aimed precondition fired visibly:
  `query_incident_list` `result_count: 0` → `1` across one second, with Pulse's incident-persist line at
  `09:52:27.074Z` after the storm emitted at `09:52:26.9`. Pulse-side: 27 `duckdb.append`, **zero**
  `reject_reason`, `severity_hint: "autonomous"` at `occurrence_count: 10`.
- **`v2-10` CLAIMED and verified — coverage 11/32 → 12/32.** It needed two arms: `preflight` is a gate and
  writes no artifacts, so a scenario leg produced the evidence — `runs/2026-08-16T09-54-12-950.jsonl` +
  the `runs.db` row with **`verdict: "Pass"`, `state: "KnownResidual"`, `latency_ms: 2153`**, the first run
  record carrying a non-null verdict and a real journal-relative SLO measurement.
- **The chunk's premise was falsified and the goal kept.** `fingerprint_refs` carries the L4 model's
  `evidence_refs` (`[]` under deterministic L4) and `span_events` has zero reads in `mcp-server`, so
  Conductor's fingerprint has **no read-back surface at any width** — no derivation could have opened that
  precondition. Aligning still shipped, on its own merit: Conductor's expectation now predicts how Pulse
  actually GROUPS exceptions.
- **P-017 clause (c) is falsified on TWO axes** — only the first 3 normalized lines contribute, and
  `normalize_frame` strips ABSOLUTE paths only, so a relative-path change is identity-significant. Verified
  at wrap that **no spec master states the claim**; the live statements were source (fixed) and
  `scenarios/fingerprint-storm.toml`'s header prose, which is CARRIED on the `fingerprint-storm live proof`
  entry — that entry also now owns re-deriving the identity triple, since `PathVariant` no longer matches.
- **`cargo audit` — 22nd red**, byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1.
  **Re-pinned in FULL form: the basis CHANGED** — `Cargo.lock` moved, so "no dependency delta" is no longer
  available and `cargo deny` is the sole coverage for the added packages. security-plan and a new playbook
  rule now state the admission condition.
- **Honesty register held:** freshness proves causation-in-time, not payload identity (a concurrent incident
  in the poll window would satisfy the gate), and the adopted derivation was never compared against a
  Pulse-computed fingerprint — none is observable. Both recorded in `evidence/leg-verdict.md` and arch.
- **Curation:** T1 1 · T2 2 · T3 0 (filtered 1), all in-place extensions. T1 gained a **PROVENANCE** axis
  (who produces the value) beside direction/arity/representation. Plus **3 cascade-routed corrections** of
  entries the measurement made false: `verification-harness.md`, `rules/testing.md` ("a stable hash makes it
  match the SUT's" — false), `docs/session-learnings.md` ("fidelity rides the fingerprint" — false).
- **A truncated grep nearly invalidated the leg:** `grep … | head -6` suggested only the sidecar reads
  `ANDROMEDA_PULSE_DATA_DIR`; `resolve_data_dir` shows `pulse-app` reads it FIRST. Had it stood, the two
  processes would have used different data dirs and the leg would have measured nothing.
- **Instance note for the next Pulse visit** (recorded, no cross-repo edit): deterministic-L4 zeroing
  `evidence_refs` (`deterministic_inference.rs:35`) is a Pulse-side improvement candidate — a representative
  canned `evidence_refs` would restore payload-identity assertions for any verifier. File alongside the
  PK-coupling observation.
- **Last failed command:** none.
