# Session Handoff

**Last Updated:** 2026-08-10T20:05:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **5 ahead** after this chunk commit — read at write time)
**Status:** clean
**Last Commit:** 2026-08-10-workspace-key-divergence-probe — an app/sidecar workspace-key mismatch now blocks with a named precondition, and the live probe's verdict is recorded

## Position
- Done: **2026-08-10-workspace-key-divergence-probe** — the opaque `canary round-trip failed: incident not found in corpus` is replaced by a **named** precondition on the zero-incident branch, naming the app/sidecar workspace-key agreement AND "Pulse raised no incident" as the two candidate causes. The gate's named-precondition set is now **four**. Private to `conductor-verify`: a typed `NotFound{EmptyCorpus,FingerprintAbsent}` selects the string inside the existing cascade — no `ReadyState` field, no `CanaryOutcome` variant, `ReportState` still five, six lamps, zero caller threading.
- Next: **Pulse run contract** (Epoch 2 — deterministic-L4 + shared data-dir asserted at preflight, unmet terms named, P-073) — `/andromeda-phase` to promote + plan. It carries a **PREREQ** (`cargo audit` 7th, now ratified) and a **CARRY** (re-run the three-arm probe once incidents actually form).

## Work done
2 files in `conductor-verify` (+1 new chunk-evidence artifact), **zero dependency delta**, both lockfiles un-drifted. Gates green in **1 fix-loop iteration, no fixes needed**: nextest `-p conductor-verify` **68/68**, workspace `--profile ci` **495/495** (+1) zero retries, doctest ok, `clippy -D warnings` clean, `cargo deny check` all four classes ok. Smoke ✓ (`agent-run.sh run` exit 0). Beyond the harness: three live `conductor preflight --json` runs against a real Pulse, each returning the new precondition with `data_dir` scrubbed to `<redacted>`.

## Drift resolved
2 detectors fired → **2 amendments**, **0 escalations**, 0 open. arch §Standard Contracts (precondition set 3→4 + the two-cause rule + host-path-free) · security-plan §Security Anti-Patterns (the never-downgrade ban restated over four preconditions; **the stale `keychain read-while-write` cause removed** — a cascade step-2 fix, since that bullet verbatim-cites the amended arch passage and had been stale since 2026-06-27). Cascade: 3 leaves edited (CLAUDE.md · rules/security.md · docs/security-summary.md), 2 checked-and-correctly-unchanged, 4 masters checked for stale citations and correctly left alone ("empty canary ⇒ blocked" is still true). Full record: `.andromeda/runs/2026-08-10T19-49-03-wrap/fanout-results.md`.

## Notes
- **The live probe did NOT settle the workspace-key question — and that is the recorded finding.** Under deterministic L4 (confirmed active in all three arms) with a shared data-dir, Pulse ingested the canary but `cues_emitted` stayed **0** with the service in baseline bootstrap, so **no incident formed under any key**. All three arms blocked for the *no-incident* reason, which means the key axis was never exercised and arm 3 leaves the Windows `\\?\` canonicalization question open. Evidence: `chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md`. Pinned as a CARRY on the Pulse-run-contract entry, which owns "an incident actually forms".
- **The intent's interim-unblock premise is falsified.** `workspace_detector::detect` records the `.andromeda` marker and VCS as **fields**, not success conditions, so detection succeeds for virtually any real cwd and a marker-less temp dir keys to *that temp dir*, never `data_dir`. Recorded via **C20** in `verification-matrix.json#v2-17` `notes` (a PREMISE-CORRECTION narrative) + `scope.md` — **not** as an amendment: `intent.md` and `requirements.md` are immutable and no skill writes them.
- **`cargo audit` — SIXTH red, now RATIFIED.** Byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 on 0.22.2 (the latest — nothing to raise a floor to). The L5 age trigger fired; the operator ratified the bounded wait, so pin #7 carries `deferred since 2026-08-08-sut-capability-manifest` and **re-pins silently from here** until the DB heals. `cargo deny check` observed green as the overlapping signal.
- **Two long-lived gaps got owners this wrap.** (1) The orphaned `conductor-report` spans — that crate emits **zero** `tracing::` calls while obs-plan §4 CP-1 mandates `report.generate` + `db.insert_run`; the Epoch-8 chunk that was to add them is complete, and the deferral had survived only in a playbook `note`, which is not an owning channel. Now a CARRY on the Epoch-2 *scenario.run root span tree* entry (operator-assigned; that entry's scope now covers the report seam). (2) `/ui/` — Pulse emits Specta/TauRPC bindings relative to *its* cwd, so arm 1 wrote a foreign `ui/src/bindings/` tree into this repo; gitignored rather than deleted (files preserved for inspection, and recurrence-proof for every future arm).
- **Deviations:** the plan's step-5 stub knob was **not** added (a shipped test already drives that cause via a mismatched `CanaryMarker`); two "untouched" shipped legs were edited because they pinned the exact string being replaced; the Pulse MCP sidecar had to be **built** (unlisted precondition — unbuilt, every arm would have measured the read-back-unreachable path instead).
- **Curation:** T1 1 (intent/requirements immutable → the C20 ledger channel) · T2 2 (verification-harness: live-probe host preconditions · security: an in-place extension adding the bounded-wait end-condition) · T3 0. Filtered 3 (2 duplicate, 1 confidence). No conflicts.
- **Verification matrix:** `v2-17` **verified**. Coverage **6/32**.
- **Last failed command:** none.

## Deferred learnings
1 learning analyzed but not applied (max-3 cap):
- The two-cause honesty pattern: a diverged workspace key and an empty corpus are byte-identical on the wire, and read-back exposes no second key (`query_incident_list` takes no arguments), so the precondition names both causes rather than claiming a measurement Conductor cannot make (confidence 0.6).
Review with `/andromeda-wrap-session --review` if it should be applied.
