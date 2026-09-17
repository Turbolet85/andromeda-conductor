# Fan-out results — 2026-09-17-keyboard-and-focus-order-coverage-ownership

Seven Explore doc-agents, one per spec source, one parallel batch. Six returned `proposals: []`; one
returned a single warning-severity proposal. No return needed stripping and none carried entity escapes, so
no raw twins were warranted — this file is the sanctioned audit artifact for the clean returns.

| doc | verdict | basis the agent gave |
|---|---|---|
| architecture | `proposals: []` | — |
| security-plan | `proposals: []` | no new external-input surface (no IPC/endpoint/port/socket/env var); sidecar spawn, preflight and data-dir untouched; Dependencies none, `tsx` already a devDependency enumerated at `security-plan.md:190`; no platform verdict retired |
| design-system | `proposals: []` | all four Coverage rows carry `tokens n/a`, none `hardcoded✗`, no rendered UI; `Counts / qualifiers moved` is `none — verified`, so the palette rows, the ANSI map at `:291-310` and the token block at `:204-249` have nothing to check |
| layout-templates | `proposals: []` | no user-facing region added; the Outcome bullets affirm the doc (run-state keys match `:28`; the no-router basis matches §Primary navigation `:112-114`); `:108`/`:328`/`:186`/`:190` name platforms and entry points without stating a verdict |
| test-plan | **1 proposal** — `D-tests-framework` | §4 `:247` closes the ui/ proving set at the wdio legs; this chunk ships a fourth executing leg (`tsx`) |
| obs-plan | `proposals: []` | new symbols are TS data + a static checker, zero `.rs` delta; every Coverage row instrumentation `n/a`; Dependencies none; the only new writer prints suite and claim names, never a path |
| a11y-plan | `proposals: []` | no interactive element added; Schema/config `none`, the envelope at `:94-109`/`:232-246` still matches obs §6; the `:115` dittography and the §11 `:516` sentence are Expected amendments, not detector drift |

## D-platform-claim — no hit in any of the seven

All seven evaluated it and all seven declined, on the same two grounds:

- The report's `Spec claims disproved by measurement` entry retires an **orphan-hygiene defect** framing, not
  a capability verdict about where the harness can run. Its subject is `working-route.md:39`, and the
  report's own `grep -rlF "msedgewebview2" .andromeda/*.md` → **0 hits** was independently re-confirmed by
  three of the agents.
- The `Harness / gate surface` entry only ADDS a driver-free verb and states that nothing about CI, the
  harness shells or what runs where changed.

Platform-bearing lines the agents quoted and correctly declined as non-verdict: enumerations of surfaces and
hosts, entry-point pointers, and a11y-plan `:457`'s TARGET-state `ubuntu-latest` + `xvfb` arrangement — which
the detector's own text excludes as "an unimplemented plan, not a capability claim".

## Orchestrator verification of the one proposal

Verified before accepting, rather than taken on the agent's word:

- **`test-plan.md:247` really does close the set.** Read in full: "the e2e harness config and its members are
  proven only by EXECUTING the wdio leg that LOADS them, never by the build gate — `--e2e` … `npm run
  a11y:driven` … `npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error` … and a `wdio.conf.ts` member reachable
  from NONE of them is as unproven as it was under the build gate." This chunk's two new TS members are
  reachable from none of those legs.
- **The §12 invariant survives.** `:603` reads "No JS/TS **unit-test runner** researched … (e.g., vitest)" —
  a component/unit test framework. `tsx` is a TypeScript executor with no test framework, no assertion
  library and no test discovery, so §12's decision and §1 `:87`'s restatement of it both stay true. This is
  the qualifying clause of playbook rule `:28`, checked rather than assumed.
- **The sweep claim holds.** The retired claim has exactly one site. Resolved BY OFFSET on the file's longest
  line (`:307`, 10 647 chars, which names the same suite families and could plausibly restate it):
  `proven only by` → 0 hits, `EXECUTING` → 0 hits, `unit runner` → 0 hits. `:307` names the arms and their
  subjects but makes no proving-set claim.

**Playbook:** rule `:28` (a spec's illustrative mechanism reconciled to the sound implementation shipped,
where the report demonstrates the invariant still holds) → **routine**. The fix names the SET of executing
legs rather than re-closing the list, per the de-literalization discipline at rule `:127`.
