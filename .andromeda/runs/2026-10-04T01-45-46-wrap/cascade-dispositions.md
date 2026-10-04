# Cascade dispositions — 2026-10-04T01-45-46-wrap (0-pending: U35 citation re-points)

Sweep: `cascade-sweep.txt` (patterns `sec12` = `§\s?12\b`, `declog` = `Decisions Log`; baseline 07c8f113).
Edits made before the sweep (the sweep then reads `new 0` for both patterns over the masters):

- a11y-plan.md — §6 Target size tokens "needs a Decisions Log entry" → "a recorded decision — an `a11y-plan-amendments.md` entry";
  §6 Motion tokens "see the Decisions Log" → "see §3 Configuration"; §11 Strategy "explicit Decisions Log entry" →
  "a recorded decision — an `a11y-plan-amendments.md` entry"; §11 CI "(§6 Motion tokens · Decisions Log)" → "(§6 Motion
  tokens)" and "§3 Configuration, §6 and §12" → "§3 Configuration and §6".
- registries/contracts/a11y-plan/a11y-testing-tool-pick.md — "Section 6 Motion + Decisions Log" → "Section 6 Motion ·
  §3 Configuration".
- security-plan.md — §Bootstrap phases `secret-scanning-ci-gate` and §Secret Management "Secret scanning in CI": both
  "(Security Decisions Log, 2026-09-24)" re-pointed to §Secret Management → Secret-scan gate shape (the lifted text).
- test-plan.md — §1 Vector 1 trigger "(§4, §12)" → "(§4 Frontend unit runner)"; §4 Frontend unit runner lift's "the decision
  this plan's `§12` citations name" clause dropped; §4 conductor-tauri/ui "§12's decision … §12 Decisions Log" → §4's
  Frontend unit runner decision; §10 Mutation-survivor disposition "§12's mutation-instrument entry is the CITATION
  HOME" / "§12 carries the reasoning" → the accepted-deliberate roster below / the citation home.
- Leaves: `.claude/rules/testing.md:19` and `.claude/docs/tests-summary.md:12` "test-plan §4/§9/§10/§12" → "§4/§9/§10",
  tests-summary "test-plan §12 is the citation home" → "§10"; `.claude/docs/security-summary.md:46` "(Decisions Log
  2026-09-24)" → "(security-plan §Secret Management → Secret-scan gate shape)".
- Formatting only (no claim changed): a blank line restored above four lifted paragraphs (a11y-plan Token-name drift,
  test-plan Supply-chain audit form, layout-templates ×2) and above security-plan Secret-scan gate shape and test-plan's
  Accepted-deliberate roster, matching their sections' blank-separated paragraph style.

Row dispositions:

| row | disposition |
|---|---|
| security-plan.md:390 `## Security Decisions Log` | true claim — the U35 stub heading, kept by registry-contract §The map |
| design-system.md:402 `## Design Decisions Log` | true claim — U35 stub heading |
| layout-templates.md:310 `## Decisions Log` | true claim — U35 stub heading |
| test-plan.md:526 `## 12. Test Decisions Log` | true claim — U35 stub heading |
| obs-plan.md:570 `## 12. Obs Decisions Log` | true claim — U35 stub heading |
| a11y-plan.md:494 `## 12. A11y Decisions Log` | true claim — U35 stub heading |
| CLAUDE.md:131 (curation, `USER:session-learnings`) | quotation of a past state — the 2026-09-05 learning describes where the hyphenated form sat then; USER region preserved verbatim |
| playbook.md:129 (base) | quotation of a past state — a rule's historical note naming where a literal sat; the playbook grows by approved appends only, never reworded |
