# Session Handoff

**Last Updated:** 2026-06-21T07:35:23Z
**Branch:** build/conductor-0.1.0
**Status:** clean
**Last Commit:** 2026-06-20-bursty-train-pattern — feat: bursty-train pattern (conductor-faults, P-013)

## Position
- Done: **2026-06-20-bursty-train-pattern** — `conductor-faults/src/train.rs` (`BurstyTrain`: a *repeating* active/quiet duty-cycle descriptor, canonically 5min/10min; `new(active,quiet) -> Result<Self, FaultError>` validated, infallible `canonical()`/`Default`, `active/quiet/period` + `*_ms`, `is_active_at` phase query; `FaultError` +3 variants `ActiveZero`/`QuietZero`/`WindowTooLong`). The P-013 activity-floor false-positive guard. **Epoch 4 (Fault helpers) COMPLETE — 4/4.**
- Next: **Epoch 5 (Verification & read-back), chunk 1 — "MCP read-back client"** (rmcp over `TokioChildProcess` stdio, hardened fixed-path sidecar spawn, `.env` data-dir) → `/andromeda-phase` to promote + plan.

## Work done
New `conductor-faults/src/train.rs` (`BurstyTrain` + 3 consts + `new`/`canonical`/`Default` + accessors + `is_active_at` + 8 unit tests + 1 doctest); `error.rs` +3 `FaultError` variants + 3 Display tests; `lib.rs` re-export + crate-doc → 4/4. No new deps; `Cargo.lock` un-drifted. Gates green: faults 29/29 · workspace 175/175 (+11) · clippy `-D` · doctest 3/3 · llvm-cov 98.22% (train.rs 100% lines). 0 fix-loop iterations (clean first pass). Smoke skipped — pure library primitive (timeline/cleanup wiring is Epoch 7/8).

## Drift resolved
none — all 7 fan-out detectors returned `proposals: []` (0 amendments, 0 escalations, 0 spec-body edits, no cascade). The report's Coverage section pre-stated the deferred posture (the `fault.*` span route-sequenced to Epoch 7/8; garde wiring Epoch 7, citing the existing playbook rules), so D-obs-instrumentation / D-security-input / D-arch-resources did NOT over-fire — cleaner than the prior abrupt-silence wrap (which needed 1 escalation→dismiss + a new playbook rule).

## Notes
- **Key decisions:** P4 "Helper shape" (AskUserQuestion) → **parameterized + validated (`Result`)**. `BurstyTrain::new` joins `EmissionGap`/`PortOccupier` on the `Result`/`FaultError` branch (`AbruptSilence` = infallible; the `Option` anonymous-bound branch is still unused) — the fault-constructor idiom now spans all 4 Epoch-4 helpers. `MAX_WINDOW` = 1h **inclusive**, sized so the *same* helper expresses the Epoch-7 "lunch" duty cycle (60min quiet) — a fixed marker would have forced a second helper. `is_active_at` half-open `[0,active)`/`[active,period)` makes the *repeating* contract testable (the P-013 distinction from P-014 death / P-015 restart).
- **Curation:** none this session — filtered 3 (1 dup: the three-branch idiom is already in `session-learnings.md` from the abrupt-silence wrap · 1 confidence<0.6: "wrap report pre-stating deferred posture pre-empts over-fires" — a dogfood candidate if it recurs · 1 task-specific: the `MAX_WINDOW`/lunch sizing, captured in the chunk plan/report).
- **Follow-up (tracked, not route chunks):** (a) `fault.*` bursty-train obs span + timeline/cleanup wiring — Epoch 7/8 (route-sequenced; Epoch 7 "Activity-floor + restart-suppression scenarios — train/lunch/silence, P-013..P-016" carries the scenario wiring). (b) scenario-config garde wiring for all 4 fault helpers (`PortOccupier`/`EmissionGap`/`AbruptSilence`/`BurstyTrain`) — Epoch 7. (c) `opentelemetry-proto default-features=false` (drop dormant `opentelemetry_sdk`) — still open. (d) obs `fault.silence` null/sentinel `fault_duration_ms` for permanent silence — Epoch 7/8.
- **Last failed command:** none.
