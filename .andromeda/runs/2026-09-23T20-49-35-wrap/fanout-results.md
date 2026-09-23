# Fan-out results — 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected

Seven Explore doc-agents, one batch. Returns carried no preamble and no HTML entities (`entities=0`), so no stripping was needed. The two docs with proposals have verbatim raw twins, `.raw-fanout-security-plan.md` and `.raw-fanout-test-plan.md`; the condensed lists below summarize them. **Written LATE:** this file came after the apply + cascade, and both twins came later still, on operator correction, from the returns held in the wrap conversation. There was no re-fan.

| Doc | Verdict | Proposals |
|---|---|---|
| architecture | `proposals: []` | 0 — D-arch-resources: nothing registered-new (test-only helpers, no crate/env/port); D-arch-decisions: no dependency (rstest deliberately not added); D-platform-claim: no verdict retired; `:93` already states the corrected claim |
| security-plan | 5 proposals | D-security-input ×5 (1 primary + 4 `dependent-of`), severity escalate |
| design-system | `proposals: []` | 0 — no UI; derived-count hits `:63`/`:263` are the still-true "programmatic read-back" ManualCheck wording |
| layout-templates | `proposals: []` | 0 — no surface; "read-back" hits `:201`/`:232`/`:278` are generic |
| test-plan | 1 proposal | D-tests-coverage, severity warning (additive Vector-1 class) |
| obs-plan | `proposals: []` | 0 — no instrumentation, dependency or artifact write; `:131`/`:315`/`:316` already carry the corrected fingerprint claim |
| a11y-plan | `proposals: []` | 0 — no UI element, no schema change |

## security-plan (raw return, condensed to fields)

1. D-security-input · escalate · §Input Validation, env-var path-handles row · basis `security-plan.md:115`. The three `CONDUCTOR_RUNS_DIR` test readers are guarded through `tests/capture_paths` → `resolve_under`, so the residual and its route-owned CARRY are closed.
2. D-security-input · escalate · real-model capture ingest row · `:121` · dependent-of. The data-dir value is canonicalized and required to be a directory, so the no-canonicalize residual is retired.
3. D-security-input · escalate · data-dir spawn row · `:122` · dependent-of. The parenthetical's residual is retired, and the "neither relaxes nor satisfies" clause is kept.
4. D-security-input · escalate · §Bootstrap → input-validation-library-install · `:221` · dependent-of. The three readers drop from the residual list; the E2E_SEED_DIR residual is kept.
5. D-security-input · escalate · §Security Anti-Patterns → Input · `:325` · dependent-of. Same retirement; "E2E_SEED_DIR is again the only unguarded Rust test-binary reader".

## test-plan (raw return, condensed)

1. D-tests-coverage · warning · §1 Coverage triggers → Vector 1 (`:87`). Adds a FIFTH class: the test-binary `CONDUCTOR_RUNS_DIR` / data-dir readers and their negative test `capture_paths_guard`.

## Validate (6 checks)

1. **Playbook.**
   - security 1–5: rule `a chunk SHIPS the fix that a spec master's own body already names as route-owned-not-shipped` (playbook `:134`) governs the routed readers (`real_model_live.rs`, `live_suite.rs`, data-dir reader). Its clauses hold: the master records the defect, names the 2026-09-23 CARRY, and cites the measurement. Rule `Boundary widening` checked first: nothing new crosses; the guard narrows. → routine.
   - For `journal_conformance.rs` the route-entry clause FAILS (the master recorded it as a residual but never routed it) → no-match branch. The operator's recorded P4 ruling ("guard journal_conformance.rs too") settles the amendment → applied, and a rule extension is proposed at the wrap card.
   - test-plan 1: plan Expected amendments entry, report-substantiated, additive → routine.
2. **Cross-contradiction:** none. The security and test-plan edits touch different sections in the same direction.
3. **Intent-consistency:** the report diverges from the working entry, adding a fourth reader, ten claim sites instead of three, and `:61-62` left unchanged. Every divergence is justified by the operator's P4 rulings or the P3 premise closure, and scope.md was amended at P3/P4 → intent-incomplete, already amended.
4. **Absence needs evidence:** proposal 5's "E2E_SEED_DIR is again the only unguarded Rust test-binary reader" is a universal with no sweep behind it → NOT applied as written. The applied text says only that the three named readers are guarded and E2E_SEED_DIR's residual is unchanged. All other proposal claims rest on report bullets.
5. **Expected-amendments reconciliation:** all five plan entries are matched. security `:115` → proposal 1; `:121`/`:122` → 2/3; `:221`/`:325` → 4/5; test-plan Vector 1 → test-plan 1.
6. **Disproved-claims disposition:**
   - (1) retired read-back claim family: corrected in code this chunk. Multi-line sweep over the seven masters + playbook + drift-base returned 0 hits, so no master states it. Curation-home hits: CLAUDE.md `:128` and `docs/session-learnings.md:228` are already bracket-corrected; `.claude/rules/verification-harness.md:47` at offset ~30101 is UNCORRECTED → routed to P3 curation (in-place extension).
   - (2) posture-doc `shell-declaration`: corrected in `contracts/`. `grep -rn 'defining term, and a'` over masters/leaves returned 0, so no master states it.
   - (3) entry premise on `:61-62`: a scope correction, already written in scope.md at P3.

Applied: 6 amendments (security-plan ×5, test-plan ×1). Escalations resolved: 0 open; the escalate-severity proposals were governed routine by playbook `:134` plus the operator's P4 direction. Cascade: 2 leaves re-derived (`.claude/rules/security.md:18`, `.claude/docs/security-summary.md:11`); 1 curation-home hit routed to P3.
