# Fan-out results — 2026-08-21-severity-lifecycle-live-proof

7 Explore doc-agents, one per spec source, 18 detectors scoped. Report is the single source each read.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **2 proposals** |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | clean (`proposals: []`) |
| design-system | D-design-tokens · D-design-derived-count | clean (`proposals: []`) |
| layout-templates | D-layout-surface · D-layout-derived-count | clean (`proposals: []`) — see escalation |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count | **3 proposals** (1 primary + 2 `dependent-of`) |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | **4 proposals** (1 primary + 3 `dependent-of`) |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | clean (`proposals: []`) |

All seven returned parseable YAML with no preamble to strip, so no `.raw-fanout-{doc}.md` twin was
warranted; this file is the audit artifact for the four clean returns.

## Validate — the five checks

1. **Playbook** — all 9 detector proposals classify `routine`: two arch factual status/registry updates the
   report's Symbols/APIs + Counts bullets substantiate directly, and the test-plan/obs-plan sets are the
   established family re-base class (five precedents: fingerprint-storm, error-baseline-spike,
   restart-suppression, pii-scrub, connection). No proposal reversed a locked decision or moved an invariant;
   nothing read as structural or surprising, so no "no rule but uneasy" escalation.
2. **Cross-contradiction** — none. The three docs are disjoint, and within each the primary + its
   `dependent-of` group say the same thing at each restating site (the duplicate-occurrence rule working).
3. **Intent-consistency** — the report matches the chunk's working-route entry + plan acceptance criteria.
   Its four deviations each carry a justification; none is unjustified, so no intent amendment was needed.
4. **Absence needs evidence** — the obs/test proposals rest on "never built" and "unreachable" claims, each
   verified directly rather than accepted: all four retired span names plus `auto_resolve_triggered` /
   `summary_received` / `severity_level` / `lifecycle_phase` / `severity_choice_calibrated` return **zero**
   occurrences across `crates/`; the shipped chain is `scenario.run` → `timeline.execute` → `emit.batch` →
   `verify.readback.observe` → `report.generate`; and the resolution-summary absence was grepped to zero
   across all five committed leg slices.
5. **Expected-amendments reconciliation** — the plan queued four; all four ended disposed:
   - test-plan §6 + §1 CP4 twin → **matched** (test-plan proposals 1–2). The plan's extra clause (retire the
     stale `conductor run severity-lifecycle --seed <s>` step) was NOT in the proposal text — **raised by the
     orchestrator** and folded into the §6 amendment: no scenario carries that name, and §3's seed precedence
     already supersedes the flag form.
   - obs-plan §4 CP4 + §1 row + §6 extras → **matched** (obs proposals 1–3), plus a fourth site the plan did
     not predict (§3's additional-fields example).
   - arch §Standard Contracts declare-only list + §Read-Back Posture AutoResolved status → **matched**
     (arch proposals 1–2).
   - layout-templates P-022 sample-row label → **not proposed** (D-layout-surface fires only on NEW surfaces;
     D-layout-derived-count only on moved counts — a known blind class). **Raised by the orchestrator and
     ESCALATED**, because the chunk report does not substantiate it.
6. **Disproved-claims disposition** — the report lists five; all five disposed:
   1. tick counters `"<redacted>"` → obs §4 amendment + SUT intake.
   2. read-back cannot witness absence (leg A 2 vs in-app 1) → obs §4 amendment + the v2-16 refinement + SUT intake.
   3. resolution summary unreachable → test-plan §6/§1, obs §4/§1, v2-16 `notes`, SUT intake.
   4. fresh dir ≠ 0 boot lines → falsifies a doc comment in `baseline_harvest.rs` (a TEST file, not a spec
      master, and not this chunk's code) → routed to **P3 curation** as a Tier-2 testing rule.
   5. `CountAtLeast` mis-aimed for this family → recorded in the scenario TOML headers at implement; no spec
      asserts `span_ids` are populated, so no spec body drifted. No amendment needed.

## Escalation (1) — resolved with the operator

**layout-templates sample-row P-ID labels.** Verification against `conductor-core::coverage_matrix` + the
committed catalog found the mismatch systemic, not the single site the plan predicted: P-001
`span-status-error` (actually Receiver Lifecycle State), P-002 `baseline-error-rate` (Last-Span-Ago
Tracking), P-003 `fingerprint-identity` (Receiver Failure Surface), P-014 `restart-suppression` (Service Went
Silent — `restart-suppression.toml` names P-015/P-016/P-057), P-022 `port-occupier` (Auto-Resolution and
Lifecycle — the port-occupier is P-003's `receiver-failed-port-conflict`). P-009 / P-032 / P-035 were correct.

Presented with a marked recommendation; **operator chose the full sweep** over the plan's literal one-site
scope. Applied to 13 sites across the wireframes, cli transcripts, the results table and the per-P-ID prose.
Labels only — no wireframe, state, token or lamp changed. No new playbook rule proposed: the escalation was
a scope question about a pre-existing doc error, not a recurring validation pattern.

## Applied

**10 amendments across 4 docs** — arch 2 · test-plan 3 · obs-plan 4 · layout-templates 1 (13 sites).
Sidecars appended to all four (`{doc}-amendments.md`), each written after re-reading the edited body.

## Cascade

- **Cross-master citation grep** (all seven masters, both curation homes, both judgment bases): the retired
  span/field names appear ONLY inside obs-plan's own retirement sentence (intentional, the restart-suppression
  form). `a11y-plan` never carried `lifecycle_phase`, so the a11y↔obs schema bind needed no move; the
  tests↔obs harness bind is untouched (no harness command, status-read or log-format change).
- **Leaves re-derived (2):** `.claude/docs/tests-summary.md` E2E CP4 line (from the amended test-plan §1);
  `.claude/rules/observability.md` scenario-extras example (from the amended obs-plan §3). `stack.md`,
  `obs-summary.md`, `design-summary.md` and CLAUDE.md's `GENERATED:setup:*` carried no claim the amendments
  moved.
- **Preserve-verbatim hit routed, not edited:** `.claude/rules/testing.md` `## Session Additions` line 58
  (the 2026-06-22 "first MIXED-class family … `any(Hard) && any(CalibrationRegion)`" entry) is stale now that
  the family carries zero checks. The cascade must never edit a curation home, so it routes to **P3 curation**
  as an in-place extension.

**Drift = 0:** every proposal applied or escalated-and-resolved; zero open.
