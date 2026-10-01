# Fan-out results — 2026-09-30-full-gate-regression-over-the-moved-surfaces

Seven Explore doc-agents, one parallel batch, the verbatim amendment-flow prompt, report
`conductor-0.3.0/chunks/2026-09-30-full-gate-regression-over-the-moved-surfaces/report.md`. No keyed-contract render
(`registry.py contracts`: architecture / test-plan / obs-plan / a11y-plan `NOT MIGRATED`; the other three carry none).
Every return was YAML with `#` comment lines only; stripping removed commentary, no proposal content. No `<` `>` `&`
in any proposal value (entity probe: 0).

## Verdicts
- architecture — `proposals: []` (C3's retired claim absent; `:244` "a P-ID may be named by several" still true).
- security-plan — 1 proposal.
- design-system — `proposals: []`.
- layout-templates — `proposals: []` (`:188` "a P-ID can name several scenarios" still true).
- test-plan — 2 proposals (one primary, one `dependent-of`).
- obs-plan — `proposals: []`.
- a11y-plan — `proposals: []`.

## Proposals and dispositions

### S1 — security-plan · D-security-input · escalate
- section: §Input Validation → CLI arguments / stdin (`security-plan.md:123`)
- change: name the `conductor run <target>` positional; a P-ID-shaped `run` target named by more than one scenario is
  REFUSED before any scenario load (an `anyhow` harness fault naming the P-ID, the count and the stems; exit 1;
  `error:` + `hint:`; stems only, never a host path); a single-owner P-ID still resolves; the harness's
  `SCENARIO=<P-ID>` inherits it; beside the existing `preconditions --for` parse refusal (exit 2).
- basis: `security-plan.md:123`; report Symbols / APIs C3, Coverage of new surfaces.
- **disposition: APPLY — routine.** Check 1 (playbook): `Accurate this-chunk addition` (`playbook.md:308`) — the
  named refusal is this chunk's (report C3), landing in an existing row, invariant intact. `Boundary widening`
  (`:124`) fails its precondition — the change NARROWS what the boundary admits (an input previously accepted is now
  refused), no new crossing — so it is not a participant. The CLI-flag rule (`:272`) fails its precondition (no flag
  added). Plan implementation notes record the overseer's P5 direction that C3 narrows behaviour and needs no founder
  word. Checks 2–6: no conflict; in intent (plan Expected amendments, entry 2); sweep cited by the agent
  (`first scenario|directory order|unsorted|read_dir|several scenarios|SCENARIO=` → `:123` only).

### T1 — test-plan · D-tests-coverage · warning
- section: §3 → 5-command implementation → `run` (`test-plan.md:151`, offsets 689 and 1013 of a 1 452-char line)
- change: retire "take the first scenario naming the P-ID in unsorted directory order … making `run` do the same is
  route-owned (CARRY)"; `run` (and `SCENARIO=`) REFUSES a P-ID several scenarios name, before any load, naming their
  stems, `hint: pass one of the named scenarios instead of the P-ID`, exit 1, covered by
  `cli_smoke::run_refuses_a_p_id_named_by_several_scenarios`.
- **disposition: APPLY — routine** (`playbook.md:308`; plan Expected amendments, entry 1).

### T2 — test-plan · D-tests-coverage · warning · dependent-of D-tests-coverage
- section: §3 → Test selection (`test-plan.md:154`, offset 235 of a 306-char line)
- change: "a P-ID target selects determinately only where a single scenario names it" → resolves only where a single
  scenario names it; a P-ID several scenarios name is refused, naming them.
- **disposition: APPLY — routine**, atomically with T1.

## Validate summary
- Check 3 (intent): scope record none (`gate.py scope` clean); the SR stamp fix sits in a listed file inside the
  chunk's SR-regression intent.
- Check 5 (expected amendments): test-plan `:151` → T1 · security-plan `:123` → S1 · the leaf cascade
  (`verification-harness.md:19`, `testing.md:38`, `docs/commands.md:18`) → cascade step 3 · `frontend.md:54` → P3.
- Check 6 (disproved claims): `frontend.md:54` (a `## Session Additions` entry, preserve-verbatim) → P3 curation as
  an in-place extension · route CARRY "twelve" (`working-route.md:73`, frozen) → already corrected in `scope.md`
  (premise-corrected at P3) · parser comment (`parse-nvda-log.ts:506`, code) → remedied in code by the stamp.
- Escalations: 0.
