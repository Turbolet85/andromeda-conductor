# Fan-out results — 2026-09-07-sr-findings-fixed

7 Explore doc-agents, one per spec source, one parallel batch. Returns stripped + entity-decoded
(`&gt;-` → `>-`, `&lt;` → `<`, `&amp;` → `&` appeared in the arch, a11y and obs returns); `entities=0`
verified on every parsed body. **21 proposals across 5 docs; 2 docs clean.**

| doc | verdict | proposals |
|---|---|---|
| architecture | drift | 3 — `D-platform-claim` (primary + 1 dependent), `D-arch-resources` |
| security-plan | drift | 4 — `D-security-input` (primary + 3 dependents) |
| design-system | clean | `proposals: []` |
| layout-templates | drift | 1 — `D-layout-surface` |
| test-plan | drift | 5 — `D-tests-obs-harness` (primary + 1 dependent), `D-tests-derived-count` ×3 |
| obs-plan | clean | `proposals: []` |
| a11y-plan | drift | 8 — `D-a11y-surface` (primary + 7 dependents) |

## Proposals as returned (decoded, condensed to detector · section · claim)

### architecture
1. `D-platform-claim` · §Established Decisions [CI/CD] (`:59`) — retire "The a11y job's own first GitHub
   run is pending the operator's push, so the wiring is recorded here, never a green run"; qualify
   "which is exactly why it is gateable" as Pulse-freedom being necessary, not sufficient.
2. `D-platform-claim` (dependent) · §Infrastructure Patterns — CI/CD approach (`:234`) — the three
   enumerated a11y stages are the job's wiring; two have never executed on a hosted runner.
3. `D-arch-resources` · §Occupied Resources — Environment variables (`:187`) — register `EDGEWEBDRIVER`
   as the CI-side value source for `CONDUCTOR_MSEDGEDRIVER`.

### security-plan (all `D-security-input`)
1. §Input Validation, `CONDUCTOR_MSEDGEDRIVER` row (`:116`) — record a second, CI-only reader.
2. (dependent) §Threat Model Summary → Attack surface (`:51`).
3. (dependent) §Security Anti-Patterns → Input, canonicalize carve-out (`:315`).
4. (dependent) §Bootstrap phases → input-validation-library-install (`:211`).

### layout-templates
1. `D-layout-surface` · §Surface: desktop-webview → Primary content block 1, **States** (`:138`) — record
   the console load-error state as a third App.tsx-owned state, with the visible copy outside the
   announced region.

### test-plan
1. `D-tests-obs-harness` · §3 `run` → `--e2e` CI-stage selector (`:155`) — the printed-verdict contract
   holds only where the caller does not preempt capture-then-print.
2. (dependent) §3 Bootstrap phases → `5-command-discipline-wire` (`:210`) — parity is of the scripts
   PLUS the invoking environment.
3. `D-tests-derived-count` · §6 Drivers per surface, desktop-webview row (`:307`) — the baked
   `10 passing / 2 skipped` sample.
4. `D-tests-derived-count` · §9 Pipeline structure, E2E (webview) row (`:460`) — the a11y job's step set
   is no longer closed (two `continue-on-error` diagnostics + the `a11y-session-diag` upload).
5. `D-tests-derived-count` · §9 Matrix builds, OS bullet (`:469`) — retire "whose own first
   push-triggered run is still pending".

### a11y-plan (all `D-a11y-surface`)
1. §11 Anti-Patterns → Strategy, the CARVE-OUT (`:516`) — narrow to the hold-dependent half.
2. (dependent) §5 Focus order per layout, run-console-idle (`:353`).
3. (dependent) §1 Scope Summary → CI integration (`:115`).
4. (dependent) §3 Harness Contract → CI integration → Command (`:281`).
5. (dependent) §3 → Focus management test harness (Driver) (`:252`).
6. (dependent) §9 CI Integration → E2E row (`:465`).
7. (dependent) §9 → Pipeline integration (`:471`).
8. (dependent) §10 SLO Invariants → per-run budget (`:497`).

## Orchestrator verification of cited sites (before validation)

Every cited line was read at HEAD. All 21 sites exist and carry the claim named.

Two findings from that read, both corrections to the ORCHESTRATOR's own report rather than to a proposal:

1. **The report's `Counts / qualifiers moved` bullet carried a false negative.** It claimed a site search
   across the seven masters returned 0 hits for the routine-arm spec count, "so no doc bakes the old
   value". The probe was `grep -rn '10 spec\|ten spec\|12 spec' .andromeda/*.md` — 0 hits, re-confirmed.
   But `test-plan.md:307` spells the value `10 passing / 2 skipped`; `grep -rn '10 passing'
   .andromeda/*.md` returns that one hit. The probe keyed on a PROXY for the claim (the word "spec")
   rather than on what the doc actually says. `D-tests-derived-count` found it by reading the report's
   fact and searching the doc for the claim's own wording. Report corrected.
2. **The report's disposition of disproved-claim (1) was wrong.** It said the sequential-focus premise is
   route-CARRY freight that "no master states". `a11y-plan.md:516` states it inside the carve-out
   sentence ("the sequential-focus start-point defect is open and its holder unmeasured"). Caught by the
   a11y detector; the claim is now dispositioned through that amendment. Report corrected.

Neither is a proposal defect — both are the fan-out catching the orchestrator, which is the check working.
