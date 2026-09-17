# Session Handoff

**Last Updated:** 2026-09-17T08:00:22Z
**Branch:** `build/conductor-0.3.0` · **1 ahead of `origin/build/conductor-0.3.0` at Setup**; this wrap's
commit leaves it **2**. The operator pushes.
**Status:** clean — drift 0, 20 amendments applied, **four escalations UNRESOLVED and carried** (the code
they would have ratified was removed instead).
**Last Commit:** `feat(2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm)` — see below.

## Position
- Done: **`2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm`** — the terminal is NOT reached, and
  the chunk's own deliverable is why. The remedy it was built to add is, on the working configuration, the
  thing that blocks green — and finding that is what closed six days.
- Next: **`The a11y routine arm's terminal, on the measured configuration`** — `working-route.md:37`, minted
  at this wrap's P5, head of the markerless tail, Epoch 3. It owns `v3-02`.
- Coverage **5/11 verified · 6 unclaimed** — `v3-02` un-claimed, returned to the pool for the successor.

## The finding

**The arm RUNS and is one assertion from green.** Run 35192876641 — hosted `windows-2022`, native WebView2
runtime 131.0.2903.86, msedgedriver pinned to 131.0.2903.86, **High integrity, no launcher** — created the
first hosted-runner WebView2 session in this project's history (`DevToolsActivePort` in 1 s, banner
`[webview2 131.0.2903.86 windows]`) and ran the routine arm at **11 passing / 1 failing / 2 skipped**.
SC 2.4.3 (`:397`) PASSES on 131.

The single red `:384` (SC 2.1.1) is a **counting-basis defect in the assertion**, not an a11y defect:
expected and received bracket lists are identical to the character — same six controls, same order, same one
wrap — and only the prefix differs, expected `6 reached` vs received `12 reached`. The expectation counts
DISTINCT controls, the label counts VISITS. **Record it as "12 visits / 6 distinct", never a bare count.**

**Integrity's SIGN is configuration-bound.** Medium helped at runtime 153 on the dev host; High is REQUIRED
at 131 on windows-2022. The 2026-09-16 legs A/B/C are BOUNDED by this, never retired.

## Work done
`scripts/a11y-limited-token-launch.ps1` (mechanism `runas` → `DuplicateTokenEx` + `SetTokenInformation` +
`CreateProcessAsUser`, P/Invoke, no dependency) · `scripts/a11y-token-witness.ps1` (modes, transports, refusal
exits 96/97, TEMP write probe, driver-log readback, stale scheduled-task naming retired) ·
`.github/workflows/ci.yml` (three driver-alone diagnostics, leg-console print, stale naming) · **3 evidence
records**. No Rust/TS surface, no dependency, no lockfile delta. **Eleven CI probes** driven inside the
fix-loop under the ci-probe directive — HEAD `139bbb1` throughout, every probe ref deleted (verified 0 on
origin).

## Drift resolved
**20 corrections: 13 in master bodies** (test-plan 4 · a11y-plan 4 · architecture 4 · residuals 1) **+ 7 in
the distillation tier** (a11y-summary 4 · tests-summary 2 · rules/a11y 1), from a 7-agent fan-out that
returned **29 proposals**; design-system, layout-templates and obs-plan clean. 3 sidecar entries.
The fan-out found far more than the directive's sweep set named — five restatements in test-plan alone, and
two egress-COUNT claims in architecture plus one in security-plan that the directive did not list.

## FOUR ESCALATIONS — UNRESOLVED, carried on the successor entry
No operator ruling arrived, so the directive's own fallback was taken: **the probe-scoped code came out**
rather than shipping unratified. All four therefore resolved for THIS commit without a ruling, and all four
remain owed:
1. **The `≥152` floor is falsified as a posture** — not necessary (coherent 131/131 works), not sufficient
   (coherent 152/152 fails, run 34654076633), and it DESTROYED the working configuration (run 35185153012,
   fetching 153 over a native 131). Measured subject is driver/runtime major **COHERENCE**.
2. **The float's stated exit condition names a mechanism that does not exist** — the Standalone Installer is
   Evergreen and takes no version; only Fixed Version is versioned (>250 MB, unobtainable at 131).
3. **The seventh governed spawn form's disposition** if the launcher leaves the asserting step.
4. **A second non-loopback egress** (`msedgedriver.microsoft.com`) if the driver pin ships.

## Notes
- **Removed from the tree before commit, not shipped:** the `windows-2022` label, the `≥152` floor bypass,
  the driver-pin step with its second egress, and the launcher's removal from the asserting step. The shipped
  arrangement is unchanged and no arrangement row moved in any master.
- **The exclusion arm is DEAD on measurement** — the endpoint demonstrably opens on a hosted runner, so any
  future permanent exclusion would have been ratified on a false basis. `v3-02`'s acceptance was NOT refined
  downward; it is correct as written.
- **Still open from prior sessions:** the n=1 deferred escalation class; the audit-debt chunk's discarded wrap
  `gates` evolve record; the `quantile` 14-vs-11 correction for `code-metrics.ndjson`; CARRY 3's orphaned
  `msedgewebview2` hygiene defect (6 in-window survivors predate this session, `StartTime 18:33:31`).
- **`v3-08` stays BLOCKED** — unchanged.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-09-17 10:19:22
