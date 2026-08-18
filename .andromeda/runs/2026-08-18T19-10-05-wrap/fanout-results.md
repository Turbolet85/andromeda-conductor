# Fan-out results — 2026-08-18-error-baseline-spike-live-proof wrap

7 doc-agents / 18 detectors. **16 proposals (7 primaries + 9 dependents) · 16 applied · 0 escalations · 0 rejected.**
All matched playbook rule "reconciles a spec's illustrative mechanism or wording to the sound implementation
actually shipped, invariant preserved" (+ the derived-count sample rules) → routine.

| doc | verdict | proposals | applied |
|---|---|---|---|
| arch | drift | 3 (D-arch-decisions ×2 [RBDP evidence_refs pin + §Standard Contracts restatement, dependent] + D-arch-resources [envelope sample `<5s`→`<90s` + declare-only note]) | 3 ✓ |
| security-plan | clean | 0 | — |
| design-system | drift | 1 (D-design-derived-count — Pattern 4 sample tier → `<slo_tier>` placeholder) | 1 ✓ |
| layout-templates | drift | 5 (D-layout-derived-count — P-009 sample `<5s`→`<90s` at 5 sites, 1 primary + 4 dependents) | 5 ✓ |
| test-plan | drift | 4 (D-tests-derived-count ×2 primaries [§6 signal re-base; §6 fingerprints-`[]` retire] + §1 dependents ×2; the CP2 tail "NOT via a populated field" refined in the same apply) | 4 ✓ |
| obs-plan | drift | 3 (D-obs-instrumentation — §4 required-fields + §1 row + §4 `verify.readback_fingerprints` bullet, 1 primary + 2 dependents; the third site was NOT on the report's own doc list — the per-occurrence sweep caught it) | 3 ✓ |
| a11y-plan | clean | 0 (both reproductions read "or empty array" — type declarations already admitting populated) | — |

Validation: playbook ✓ (all routine) · cross-contradiction none (design chose placeholder, layouts/arch
updated samples — both sanctioned forms) · intent-consistency ✓ (deviations justified in the report) ·
absence-evidence ✓ (every no-hit cites its grep) · expected-amendments floor MET (obs-plan §4/§1 ✓,
test-plan §6/§1 two-site ✓) · disproved-claims all DISPOSED (obs+tests via proposals; the v2-12 latency
half via the P7 coverage-gate escalation per the operator's wrap directive).

Cascade: all 7 masters grep CLEAN of every retired wording post-apply ("pinned/pins `[]`", "MAY BE EMPTY",
"empty under deterministic L4", "1840ms <5s", "verdict=Pass" family signal). Two out-of-cascade hits, each
routed to its owning channel: `.claude/docs/session-learnings.md:83` (preserve-verbatim curation home →
P3 extends the 2026-08-16 entry in place) and `crates/conductor-run/src/lib.rs:223` (a stale SOURCE doc
comment — wrap edits no source; rides the next conductor-run-touching chunk, noted at route-resolve).
Raw twins: `.raw-fanout-{arch,design-system,layout-templates,test-plan,obs-plan}.md` (proposal-bearing docs);
security-plan + a11y-plan returned `proposals: []` clean — this file is their sanctioned audit record.
