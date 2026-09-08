# Fan-out results — 2026-09-08-webview2-runtime-152-installed-in-job

7 Explore doc-agents, one batch. Entity-decode applied to the security-plan return (`&gt;-` → `>-`,
`&lt;`/`&gt;` → `<`/`>`, `&amp;` → `&`); post-decode `entities=0` on every saved body. Raw twins saved for
the two docs that carried proposals (`.raw-fanout-arch.md`, `.raw-fanout-security-plan.md`); the five
empty-and-clean returns are recorded here, which is their sanctioned audit artifact.

## Verdicts

| doc | detectors evaluated | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions · D-platform-claim | **5 proposals** |
| security-plan | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | **4 proposals** (all escalate) |
| design-system | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | `proposals: []` |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | `proposals: []` |

**D-platform-claim fired in none of the seven.** Every agent independently reached the same reading: the
report's one disproved claim (`cargo fmt --check` satisfiability) names no platform/runner/driver verdict,
and the install gate's own sufficiency is "unmeasured by construction". The report's explicit guard
("Detectors: do not propose retiring `test-plan.md:455`") was honored by all seven.

## Proposals (9)

### arch — D-arch-resources (primary + 1 dependent + 1 sibling primary)
1. **§Occupied Resources — Ports** — register the `a11y` job's outbound HTTPS GET to
   `go.microsoft.com/fwlink/p/?LinkId=2124703` as a **CI-JOB-SCOPED egress target** beside
   `127.0.0.1:4317`; reached only by the install gate, never by a shipped binary, binding no port.
   *basis:* report.md:15-18 · the registry demonstrably registers egress (`architecture.md:145`).
2. **§Cross-cutting Patterns — Trust boundary** *(dependent-of #1)* — the outbound-surface enumeration at
   `architecture.md:244` is a SECOND closed list (it already reaches past shipped binaries, registering the
   dev-only `4444`/`4445` driver binds), so a Ports-only edit leaves the claim standing there.
3. **§Occupied Resources — Environment variables** — record the install gate's own basis: it reads
   `RUNNER_TEMP` in the step SHELL, sets/reads no `WEBVIEW2_*` handle and claims no `CONDUCTOR_*` name, so
   the existing pair's registered lifetime ("that one `continue-on-error` diagnostic step") is unchanged.
   *basis:* report.md:83-90.

### arch — D-arch-decisions (primary + 1 dependent)
4. **§Established Decisions [CI/CD]** — record that the `a11y` job now provisions its own WebView2 runtime
   in-job: HTTPS fetch → Authenticode gate (`Valid` AND `O=Microsoft Corporation`, before execution) →
   fixed array-form `Start-Process … '/silent','/install'` → post-install major ≥ 152 assertion; no
   `continue-on-error`, no `if:`. A CI-time third-party binary class in no lockfile that no established
   supply-chain gate covers. *basis:* report.md:20-27.
5. **§Infrastructure Patterns — Build system** *(dependent-of #4)* — this paragraph is where arch asserts
   what BOUNDS the a11y job's third-party inputs (the pinned image + the lockfile-scanning runners).
   Qualify it: the `windows-2025` pin still fixes the Edge **driver** taken from the image, while the
   WebView2 **runtime** is now deliberately **floated to always-latest Evergreen**, admitted by Authenticode
   and invisible to `cargo audit` / `cargo deny` / `npm audit`. *basis:* architecture.md:202.

### security-plan — D-security-subprocess (escalate)
6. **§Security Anti-Patterns → Code Patterns rule (b)** — governed harness-spawn forms **5 → 6**, and the
   locus set gains `.github/workflows/ci.yml`. The new form is the **first whose PROGRAM is not
   repo-derived, repo-vendored, or a fixed OS/toolchain binary**; its admitting control is the
   pre-execution Authenticode check. *basis:* report.md:37-48, 161-162, 172.

### security-plan — D-security-deps (escalate; primary + 2 dependents)
7. **§Dependency Security** — record a **THIRD dependency class** beside the Rust crate graph and the npm
   tree: a CI-time-fetched third-party binary that appears in no lockfile and that `cargo audit` /
   `cargo deny` / `npm audit` **structurally cannot see**; sole admitting control is the Authenticode gate;
   and state the disposition on always-latest Evergreen versus pinning. *basis:* report.md:20-27, 126-129.
8. **§Threat Model Summary → Infrastructure → CI/CD** (`security-plan.md:87`) *(dependent-of #7)* — the
   clause "the supply-chain steps are unchanged by it" must stop implying the `a11y` job carries no
   supply-chain surface: the audit STEPS are unchanged, the audited SURFACE is not.
9. **§Dependency Security — the supply-chain-integrity SKIP note** (`security-plan.md:190-196`)
   *(dependent-of #7)* — "signed artifacts … **SKIP** — Standard + Hardened only" is the same claim the
   primary retires, restated as a tier-scoping. Bound the SKIP to artifacts Conductor **produces**;
   consumption-side signature verification is now an ACTIVE control.

## Validation (orchestrator)

- **Re-derivation tell:** none. Every `basis` cites the report or the agent's own document; no proposal
  reaches a manifest, lockfile or source location the report does not carry.
- **Check 1 — Playbook:** all 9 route to `playbook.md:124` — *"Boundary widening — a chunk WIDENS what
  crosses an already-hardened boundary (a read-only channel gains a write, a validated surface admits a new
  input class, a subprocess/IPC boundary gains a new crossing) and the proposal records it."* → **escalate**,
  with the note *"always a human's call — never mint a routine rule for this class, however often it
  recurs."* Corroborated by precedent: `2026-09-06-run-report-envelope-conformance-gate` widened rule (b)
  4 → 5 as an operator-ratified widening with **no routine rule minted**, so a sixth crossing escalates again.
- **Check 2 — Cross-contradiction:** none. arch and security-plan edits are complementary, not opposing.
- **Check 3 — Intent-consistency:** aligned. Proposals 1-5 are the plan's Expected amendment 2; 6-9 are
  Expected amendment 1, including the §Dependency Security half the operator added at P5 review.
- **Check 4 — Absence needs evidence:** each duplicate-site claim carries its line (`architecture.md:244`,
  `security-plan.md:87`, `:190-196`); the orchestrator re-derives the applied text from the invariant +
  the report's fact at apply time, never pastes the `change` line.
- **Check 5 — Expected-amendments reconciliation:** entries 1 and 2 proposed ✓. **Entry 3** (test-plan §6/§9
  + a11y-plan §1/§11) proposed by neither — both agents independently concluded *owed-but-not-yet-due*: the
  pair-set extension needs the probe's CI result, which follows this commit. Dispositioned to the follow-up
  entry + the handoff per wrap directive item 2, **not** raised as an amendment (writing it now would state
  a fact that does not exist).
- **Check 6 — Disproved-claims disposition:** the report's single entry (`cargo fmt --check`) is DISPOSED
  via the route channel per wrap directive item 1 — owner is a route entry (workspace formatting pass +
  `cargo fmt --check` as a CI gate), presented at P5 as trajectory. No master edit; `test-plan.md:455`
  stands as a target-state row.

**Outcome: 9 proposals, 0 auto-apply, all escalating to ONE operator decision → HALT.**
