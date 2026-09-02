# Fan-out results — 2026-09-01-live-per-p-id-verdict-lamps

7 Explore doc-agents, one batch. **27 proposals across 6 docs**; `design-system` returned clean.
Raw twins: none warranted — every return was well-formed YAML needing no stripping, and no return
carried entity escapes (`entities=0` probe clean on all 7).

## Per-doc verdicts

| doc | proposals | disposition |
|---|---|---|
| architecture | 5 | 2 apply · 3 dismissed (playbook rules 37 + 73) |
| security-plan | 3 | 2 dismissed (rule 94) · **1 escalated** |
| design-system | 0 | clean — verified, see below |
| layout-templates | 4 | 4 apply |
| test-plan | 5 | 5 apply |
| obs-plan | 6 | 6 apply (3 on the operator's recorded direction) |
| a11y-plan | 4 | 4 apply |

## Applied

**architecture (2)** — both from `D-platform-claim`, and both MEASURED on disk by the orchestrator
before applying, not merely derived:
- §Occupied Resources → On-disk artifacts, `logs/conductor-tauri.jsonl`: the self-obs sink is now
  PER-ARM. `tauri_log_path()` resolves the sink from `runs_dir.parent()/logs`, so the routine `--e2e`
  arm's `CONDUCTOR_RUNS_DIR=runs/e2e-fixture` moves it to `runs/logs/conductor-tauri.jsonl`; the
  operator-local `a11y:driven` arm leaves the handle unset and still lands at the workspace root.
  **Evidence:** `runs/logs/conductor-tauri.jsonl` 4741 B at 07:31 (the leg's own timestamp, matching
  `runs/e2e-fixture/runs.db`) while root `logs/conductor-tauri.jsonl` stayed stale at 00:55.
- §Occupied Resources → Frontend asset subtree parenthetical (`dependent-of`): same verdict, second site.

**layout-templates (4)** — `D-layout-surface`, one primary + 3 duplicate-occurrence sites. The run-level
load-envelope banner is a genuinely new user-facing region: desktop §Primary content block 2 (anatomy),
§Wireframe — Run report (terminal), §Primary screens (content enumeration), and the cli §Primary content
block 2 sentence that framed the `[ENVIRONMENT-SUSPECT]` qualifier as cli-stdout-only.

**test-plan (5)** — the `--e2e` leg now seeds its own subject, which the doc's canonical chain omitted:
§3 CI stage selectors + §9 E2E row (the seed step + `CONDUCTOR_RUNS_DIR`), §6 desktop-webview ROUTINE arm
(re-stated from "subject-absent" to fixture-seeded, naming the banner as the one path with no e2e proof),
§7 seed strategies (the committed-journal fixture family + its production-reader round-trip pin), and §5
(the read-only-command mock-runtime tier's "against the `conductor-core` source" qualifier, falsified —
`run_envelope` single-sources `conductor-report`'s `RunsDb::get_envelope` via `conductor_run::read_envelope`).

**obs-plan (6)** — three subjects:
- §1 desktop-webview row + §4 auto-instrumentation row: handler count **7 → 8**, `run_envelope()` into the
  enumeration (the report's Counts bullet).
- §1 Notes: deleted the trailing `instrument via #[tracing::instrument]` clause that contradicted the same
  sentence's own (correct) statement that the attribute does not stack with `#[tauri::command]`. This is
  the report's one `Spec claims disproved by measurement` entry — disposed. §4 line 283's
  `#[tracing::instrument]` is the **cli** row and is untouched (playbook rule 109's boundary).
- §6 Boundary-call wrappers + §4 conductor-report row + §1 rusqlite row: the read-path convention —
  a rusqlite READ inside a `tauri.command.*` span logs a boundary `info!` carrying `run_id` and the
  outcome on the allowlisted `message` field, and mints **no** `db.*` read span. Applied on the operator's
  recorded direction (WRAP directive item 2: do not widen the bounded span-name set with a wildcard;
  a `db.*` read span is a design option for the entry that next touches `conductor-report`). The two
  further sites are the same retired wording ("every rusqlite query boundary is a span") restated, which
  the cascade's duplicate-occurrence rule requires fixing in the same pass.

**a11y-plan (4)** — `--status-residual` carries TEXT on both its non-lamp surfaces, so it owes 4.5:1
(SC 1.4.3), not the 3:1 a non-text indicator owes: §6 pair table (+2 rows, 9 → 11 enumerated), §3
Source-of-truth tokens, §1 Contrast verification harness (the same closed enumeration restated twice), and
§4 pattern catalog (a row for the banner, recording its rendered-DOM axe as unrunnable in the routine arm).

## Dismissed, with the rule that dismissed them

- **arch A1/A2** (register `run_envelope` in §Occupied Resources Tauri commands + the §Conventions
  core↔UI restatement) — **playbook rule 73** (line 73): arch registers the Tauri command surface at
  CATEGORY grain; concrete handler names are the realization arch deliberately omits. `run_envelope`
  falls inside the already-registered "run-report view" category, and the chunk added no port, socket,
  endpoint, env var or crate.
- **arch A3** (register `conductor_run::read_envelope` on the §Crate names conductor-run entry) —
  **playbook rule 37**: arch registry sections do not track per-crate public API surface. *Flagged as the
  one borderline dismissal:* that bullet already names `persist` by hand, so omitting its literal read
  counterpart leaves the bullet describing a write-only composition root. Dismissed on the twice-confirmed
  rule; surfaced for the operator to overrule.
- **security-plan S2/S3** (add `run_envelope` to the §Threat Model Tauri-IPC attack-surface enumeration
  and the §Anti-Patterns capabilities ban) — **playbook rule 94**: that enumeration is category-grain and
  illustrative, not an allowlist (it already omits `coverage_matrix` and never named
  `resolve_operator_hold`), and app-defined `#[tauri::command]`s are not capability-ACL-gated. Their
  subject differs from their nominal primary S1 (an illustrative enumeration vs a missing mandate row),
  so they dismiss on their own merits regardless of S1's outcome.

## design-system — clean, and the check that established it

Returned `proposals: []` after grepping all three moved counts. Two findings worth recording because both
are near-misses that a shallower pass would have mis-fired on:
- The literal `11` at design-system.md:201 ("11 of 34") is a Tailwind `@theme` tree-shaking **token**
  tally, a different quantity that coincidentally shares the literal with the a11y pair count 11 → 13.
  The chunk added no token, so it still matches reality — substituting 13 would have been a false positive.
- The residual-tier enumeration (`:298-306`) already names the run-level `[ENVIRONMENT-SUSPECT]`
  load-envelope caption AND states "webview binds the same pair by name as `var(--status-residual)`".
  The shipped banner IS that webview binding, so the tally is made *more* true, not stale.

## Expected-amendments reconciliation (Validate check 5)

The chunk's `plan.md` listed 5 expected amendments — the coverage floor:

| # | entry | outcome |
|---|---|---|
| 1 | arch §Occupied Resources — eighth Tauri command | proposed (A1), **dismissed** per rule 73 — the plan's expectation was over-cautious |
| 2 | a11y-plan §6 contrast pairs — `--status-residual` | proposed (Y1) → applied |
| 3 | layout-templates §Primary content block 2 — webview banner | proposed (L1) → applied |
| 4 | layout-templates §Primary content block 1 — coverage row anatomy | **NOT PROPOSED → ESCALATED** |
| 5 | design-system §Residual-mute — extend the by-name set | **NOT PROPOSED**; orchestrator raised and found not-drift (the doc already names the caption and the webview binding — see above) |

## Escalations

1. **Coverage row anatomy** (expected-amendment #4). No detector proposed it because the **report omitted
   the fact** — detectors read the report alone. This is the same failure shape the previous wrap recorded.
   Remedy: the report's Changes section is repaired in this pass, and the amendment's DIRECTION is put to
   the operator (the divergence is pre-existing and not introduced by this chunk).
2. **security-plan S1** — §Input Validation's boundary table has no row for `#[tauri::command]` IPC
   arguments, while the shipped command's own doc comment cites that section for its `resolve_under`
   guard. Escalate-severity; playbook rule 58 only partially matches (the proposal documents an existing
   correct practice rather than demanding re-confirmation of a hardening invariant), and the boundary
   class is pre-existing — `run_report(run_id: Option<String>)` shipped the same shape earlier.

## Checks 2, 4, 6

- **Cross-contradiction (2):** none. The multi-proposal docs edit disjoint sites — obs-plan §1 is touched
  at three independent points (count, Notes clause, rusqlite row), obs-plan §4 at two different rows,
  arch §Occupied Resources at four different bullets. No pair opposes.
- **Absence needs evidence (4):** every "no hit" claim cited the search that established it, with line
  numbers (design-system :201/:274/:298-306; test-plan :181/:304; a11y :115/:299/:452/:560/:589;
  layout-templates :158/:183). None inferred from a partial view.
- **Disproved-claims disposition (6):** the report's single entry (obs-plan §1's self-contradicting
  `#[tracing::instrument]` clause) is disposed by the applied O3 amendment.
