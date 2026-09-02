# Fan-out results — 2026-09-02-screen-reader-manual-spec (wrap 2026-09-02T11-47-51Z)

Seven Explore doc-agents, one per master, each given the report + its scoped detectors (drift-base.md) and
the verbatim doc-agent prompt (amendment-flow.md). Every return carried proposals, so every doc has a raw twin
(`.raw-fanout-{doc}.md`, the return as received — the obs / security / test returns arrived with HTML-escaped
`&lt;` `&gt;` `&amp;`, decoded before application: `<CONDUCTOR_RUNS_DIR>`, `<host-path>`, `&gt;-` = a YAML
folded scalar, `&amp;` = `&`).

| Doc | Proposals | Detectors fired | Clean detectors |
|---|---|---|---|
| architecture | 7 (3 primary + 4 dependent) | D-arch-resources ×2 primary (+1 dep) · D-arch-decisions ×1 primary (+3 dep) | D-platform-claim |
| security-plan | 10 (4 primary + 6 dependent) | D-security-input ×2 primary (+6 dep, escalate class) · D-security-subprocess ×1 (escalate class) · D-platform-claim ×1 (+1 dep) | D-security-deps |
| design-system | 3 (3 primary) | D-design-derived-count ×3 | D-design-tokens · D-platform-claim |
| layout-templates | 17 (6 primary + 11 dependent) | D-layout-derived-count ×4 primary (+7 dep) · D-layout-surface ×2 primary (+4 dep) | D-platform-claim |
| test-plan | 6 (4 primary + 2 dependent) | D-tests-derived-count ×1 (+2 dep) · D-tests-coverage ×2 · D-platform-claim ×1 | D-tests-framework · D-tests-obs-harness |
| obs-plan | 9 (2 primary + 7 dependent) | D-obs-instrumentation ×1 (+6 dep) · D-obs-redaction ×1 (+1 dep, escalate class) | D-obs-stack · D-platform-claim |
| a11y-plan | 6 (1 primary + 5 dependent) | D-platform-claim ×1 (+5 dep) | D-a11y-surface · D-a11y-obs-schema |
| **total** | **58** | | |

## Validation (amendment-flow §Validate)

1. **Playbook.** Count / qualifier / sample moves (arch · design · layout · test derived-count, the obs sink
   qualification, the a11y and security "two arms" enumerations) → the 2026-06-15 reconcile-wording-to-shipped
   rule (playbook line 28) — routine. The `CONDUCTOR_NVDA` §Input Validation row + its five restatements →
   named on the chunk's P5-approved `Expected amendments (wrap)` list and analogous to the 2026-08-22
   exhaustive-enumeration rule (line 112) — routine. The SR speech-log ingest row → same exhaustive-enumeration
   reasoning (a genuinely new external-input surface earns its row) — routine. The harness-spawn rule (b)
   reconciliation → the operator's consolidated relay (2026-09-02) named the exact NVDA CLI form and the P5
   card approved the array-form spawn under the `nativeDriver()` guard shape — routine (line 28, invariant
   preserved: never a shell string, validated before use, separate argv element). **Escalate-class items
   (D-obs-redaction ×2, D-security-subprocess ×1 — the sidecar console-pane host-path disclosure):** no
   playbook rule matches; resolved on the operator's WRAP directive item 1 ("the sidecar console pane stealing
   the OS foreground at run start is a Conductor spawn DEFECT (CREATE_NO_WINDOW class), a route candidate of
   its own") — applied as a MEASURED gap with the fix route-owned, never as a shipped obligation; a playbook
   rule is PROPOSED to the operator in the wrap report (not appended — needs approval).
2. **Cross-contradiction.** None. design #3 / layout #6 / #8 retire `Run in progress` in the same direction;
   a11y #4 leaves line 269's strings to the orchestrator-raised prose amendment (folded into one edit).
3. **Intent-consistency.** All proposals record shipped truth or register the leg — consistent with the
   entry ("a per-state NVDA must-announce pass spec … with one operator pass recorded against it") and the
   plan's acceptance (a recorded pass, findings never absorbed as fixes).
4. **Absence needs evidence.** layout D-layout-surface (footer / report-site checklist absent) cites the
   ui src grep + the shipped mount (`OperatorPauseDialog.tsx:57`) + the SR rows; a11y #6 cites the four
   already-retired sites by line; arch D-platform-claim's "no capability verdict" cites its grep set.
   The obs primary's "never the project-root `logs/`" OVER-generalized: with the handle unset the sink IS the
   project-root `logs/` (measured 2026-09-01 by the driven arm before this chunk's env change) — applied with
   the correct scope (unset ⇒ CWD-relative root; every a11y suite now sets the handle).
5. **Expected-amendments reconciliation** (plan.md `Expected amendments (wrap)`): every entry matched a
   proposal EXCEPT (a) a11y-plan §1 / §3 / §5 four-state naming + the must-announce prose as shipped, (b)
   a11y-plan §4 checklist second render site + `contentinfo` unshipped, (c) test-plan §1 Untestable zones
   (browse-mode reading), (d) design-system §Component Patterns 7 (the two-context checklist) — raised by the
   orchestrator as routine (the report substantiates each; items 2–4 of `Spec claims disproved`).
6. **Disproved-claims disposition.** #1 → a11y ×6; #2 → layout #1–#3 + orchestrator (a11y §1/§3/§5);
   #3 → design ×3 + layout #4–#9 + orchestrator (a11y §1/§3); #4 → layout #12–#17 + orchestrator (a11y §4,
   design pattern 7); #5 → layout #10–#11; #6 (chunk-internal plan claims) → the chunk's own artifacts (spec
   md + rows.ts already corrected; recorded in report/leg-verdict; nothing in a master states them);
   #7 → arch #3 + obs ×7; #8 → test #6 (msedgedriver major) + the README CARRY (route-resolve, not an amendment).

## Dispositions

- APPLIED: all 58 (the obs primary with the unset-handle scope correction; the a11y line-269 edit merged with
  the orchestrator-raised prose/state amendment).
- RAISED + APPLIED (orchestrator, check 5): a11y-plan §1 line 66 · §1 line 135 · §3 line 269 (states +
  strings) · §5 line 356 (`run-report-terminal` → idle-with-report; footer roll-up → the checklist view's own
  status line) · §4 line 320 (`contentinfo` designed, unshipped) · §9 line 465 (`CONDUCTOR_NVDA` + the third
  family); test-plan §1 Untestable zones (browse-mode reading, agent-unreachable today); design-system
  §Component Patterns 7 (only the dialog context ships).
- ESCALATIONS: 3 escalate-class proposals (obs ×2, security ×1), resolved on the operator's directive item 1
  (recorded above); 0 open.
- CASCADE (amendment-flow §Cascade, one pass): the retired-wording grep over all seven masters found three more
  a11y-plan sites no proposal named — §1 Notes (the four-state set), §1 CI integration ("both arms driven there"),
  §11 the skeleton ban's prose samples — fixed in the same pass; leaves re-derived: CLAUDE.md warnings line
  (three suite families + `CONDUCTOR_NVDA`), `.claude/rules/a11y.md` (landmarks · status prose · Testing),
  `.claude/rules/security.md` (the handle set + the speech-log boundary + the sidecar window-suppression duty),
  `.claude/rules/observability.md` (the sink), `.claude/docs/{a11y,tests,security,design,obs}-summary.md`,
  `.claude/docs/commands.md` (the operator-local siblings), `.claude/docs/gotchas.md` (the listener qualification,
  stale since the driven arm shipped). `conventions.md` / `stack.md` derive from unamended sections — untouched.
  Judgment bases (`playbook.md`, `drift-base.md`) and the three preserve-verbatim curation homes carry none of
  the retired wording (grep).
- CARRIED (not amendments): the ui test README's Linux-only paragraph (route-resolve CARRY); the WebView2
  driver refresh (operator host task before the next leg).
