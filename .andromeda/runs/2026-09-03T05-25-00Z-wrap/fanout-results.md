# Fan-out results — 2026-09-02-mutation-tier-restored-for-conductor-tauri

7 doc-agents, one per spec source. Detectors scoped from `drift-base.md`; each agent additionally
carried `D-platform-claim` (cross-doc).

| doc | verdict | notes |
|---|---|---|
| architecture | `proposals: []` | Read all 263 lines. No IPC/port/env-var/crate landed; workspace stays 9 members; dependency delta is removal-only. Confirmed the relocation *satisfies* §Established Decisions [Module Boundaries] (no bin↔bin edge; `conductor-tauri` keeps two outbound / zero inbound). Zero occurrences of `mock` / `parity` / `tauri::test` / `cargo_bin` in arch. |
| security-plan | `proposals: []` | Removal-only deps, 564 → 564, deny green over the new lock, toolchain pin untouched. Verified the §Input Validation `#[tauri::command]` row still holds: the six mock-runtime IPC tests **remain** in `conductor-tauri`, so the `resolve_under` / `sanitize_error` proof did not move. Spawn form keeps array-form argv + child env map per rule (b). |
| design-system | `proposals: []` | One new surface, a test target, `tokens n/a`. Independently re-greped every palette row / ANSI-map entry / token label / reuse tally for the three moved tallies — zero hits (the `13`/`12` hits are type sizes). |
| layout-templates | `proposals: []` | No user-facing surface added. Re-greped each moved tally — the only near-collision (`43 auto`) is the manifest-derived capability count, semantically unrelated to the mutant tally. Correctly did **not** propose the `run_envelope:224` survivor as doc drift: production code is byte-unchanged, so the always-rendered-qualifier invariant still describes shipped behaviour — a test-coverage gap, not doc drift. |
| test-plan | **4 proposals** | See `.raw-fanout-test-plan.md`. Sites `:287` (primary, D-platform-claim) · `:288` (dependent) · `:370` (dependent) · `:264` (D-tests-framework, the `cargo_bin` → `env!` driver fact). |
| obs-plan | `proposals: []` | No production symbol changed; no telemetry dep; no logging or artifact-write path added. obs never restates the retired first-arm characterization — its parity entries describe the PRODUCTION path (`tauri.command.start_run` → `scenario.run`), which is byte-unchanged. |
| a11y-plan | `proposals: []` | No interactive UI element; violation schema untouched. Its one `mock` occurrence (`:333`) is the load-envelope banner's non-DOM coverage — a *different* test, and the report affirms the mock runtime remains in use by the six `#[test]` fns that stayed. |

## Orchestrator's own mechanism sweep (cascade step 2 — what the claim SAYS, not what it is NAMED)

The retired claim is *"the parity leg's first arm is a `tauri::test` mock-runtime run."* A token sweep
(`mock-runtime` / `tauri::test` / `get_ipc_response`) finds `:287`, `:288`, `:370` — the three the
detector proposed. A **mechanism** sweep finds two more, which carry none of those tokens:

- **`test-plan.md:80`** — Critical Path 7: *"a scenario **launched via the Tauri start command** yields
  the same `runs.db` envelope … as the headless `conductor run`"*
- **`test-plan.md:371`** — §6 Both-surface parity: *"**the Tauri-launched run's** `runs.db` envelope …
  is identical to the headless `conductor run` envelope"*

Same mechanism, different words. These are **escalated, not applied** — see the escalation below.

## Escalation (open at time of writing)

`test-plan:80` / `:371` state Critical Path 7's **Verification signal**, and `:80` names its source as
the **Creator Brief Must-Work** (*"Every catalog scenario runs from the control panel AND headless"*).
Playbook rule *"spec-illustration → sound-impl alignment"* (`playbook.md:28-30`) is **NO MATCH**: its
qualifying clause is *"contract preserved; only the form or mechanism differs"*, and amending these two
lines to the measured in-process mechanism would not preserve the contract — it would retire a
brief-sourced requirement to match what the test does. Per `amendment-flow.md` §Validate, a subject
match with a failed qualifier takes the no-match branch → escalate.

## Adjacent finding (surfaced, not applied — not this chunk's drift)

The design-system agent noted `design-system.md:406` (decisions log) still says *"Tailwind v4.1
`@theme`"*, which the §Tokens note at `:201` explicitly retires in favour of plain `:root`. That is
surviving residue of the **2026-06-15** amendment which playbook rule `:28-30` cites as its own
founding precedent — a duplicate the original single-site apply left standing. Pre-existing, outside
this chunk's Changes, so it is reported rather than folded in.
