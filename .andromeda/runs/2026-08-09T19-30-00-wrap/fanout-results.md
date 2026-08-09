# Fan-out results — 2026-08-09-interpretation-correctness-posture

15 detectors across 7 spec sources → **6 proposals** · 4 applied · 2 dismissed · 2 self-raised · 1 escalation resolved.

## Per-doc verdicts

| Doc | Detectors | Verdict |
|---|---|---|
| arch | D-arch-resources · D-arch-decisions | 2 proposals — 1 dismissed, 1 applied |
| security-plan | D-security-input · D-security-subprocess · D-security-deps | 2 proposals — 1 dismissed (escalated first), 1 applied |
| design-system | D-design-tokens | `proposals: []` — clean |
| layout-templates | D-layout-surface | 1 proposal — applied |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | 1 proposal — applied |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction | `proposals: []` — clean |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | `proposals: []` — clean |

All 7 returned parseable YAML with no preamble; no raw twin was warranted (the three empty returns are recorded here, per `amendment-flow.md`).

## Applied (4 detector-proposed)

1. **arch §Established Decisions [Accepted Capability Set] + [Read-Back Dependency Posture]** (D-arch-decisions) — recorded the second integrity gate (`check_scenario_backing` / `UNBACKED_AUTO`, exact-set, `Auto`-only) and closed the OPEN posture decision as a deferral naming its conductor-0.3.0 owner with the "Conductor green ≠ interpretation trustworthy" statement.
2. **security-plan §Dependency Security** (D-security-deps, escalate → resolved by operator directive) — installed cargo-audit recorded as 0.22.2; added the TOOL-fault vs advisory-DATABASE-fault split with their distinct remedies.
3. **layout-templates §Surface: cli → Primary screens** (D-layout-surface) — the roll-up caption literal gains `(11 unbacked)` qualifying the auto term, with the never-a-fifth-summand rule and the webview's prop-sourcing path.
4. **test-plan §5** (D-tests-coverage) — the Epoch-9 deferral narrowed to the `Channel`-frame + GUI-parity leg; mock-runtime command-dispatch is now the standing tier for read-only commands.

## Self-raised (2 — the plan's Expected-amendments list is the coverage floor)

Validation check 5 found two Expected amendments **no detector proposed**:

5. **test-plan §6 Coverage-matrix completeness gate** — added the scenario-backing leg (second axis). The test-plan detector proposed §5 instead and did not reach §6.
6. **obs-plan §4 denominator semantics** — recorded the derived `(N unbacked)` qualifier on the auto term. The obs detector returned entirely clean.

Both substantiated by the report → routine. **This is the second consecutive chunk where the detectors under-ran the plan's Expected list** (the prior chunk missed 2 of 4). Cause is detector *scope*, not report content: no invariant covers "an existing documented element gains a qualifier". A candidate detector — *a roll-up/caption/count the chunk changes must be re-checked in every doc that documents it* — is worth adding if it recurs a third time.

## Dismissed (2 — both the command-name over-reach family)

- **D-arch-resources** proposed registering the `unbacked_auto` command in arch §Occupied Resources. Dismissed per the 2026-06-26 playbook rule: arch registers the Tauri command surface at CATEGORY grain; concrete handler names are the realization it omits (`coverage_matrix` / `run_report` / `resolve_operator_hold` were never registered either).
- **D-security-input** (escalate) proposed adding it to security-plan §Threat Model's Tauri-IPC enumeration. **Escalated to the operator** — the over-reach family was scoped to arch, not security-plan. Resolved: dismiss, and extend the rule. Evidence: that enumeration already omits `coverage_matrix` (ch6) and never tracked `resolve_operator_hold`; the "capabilities author must allow only these" argument is factually wrong for app commands (security.md 2026-06-26 — app commands are not ACL-gated); and the command is nullary/read-only with no input boundary.

## Playbook

- **New rule** (operator-approved): the security-plan flavor of the command-name over-reach dismissal, so a future read-only Tauri command does not re-escalate.
- **Amended rule** (external supply-chain decay): the remedy now FORKS on tool-fault vs advisory-database-fault, with the empirical proof method — install the latest published tool and re-run; identical failure means the data, not the tool.

## Cascade (single pass, closed)

- Cross-master citation grep of every amended passage: no stale citation in the six other masters. Remaining `0.22.1` hits are **garde** (unrelated library); the old caption literal survives only in `session-handoff.md`, which P6 overwrites.
- `architecture.md` → `CLAUDE.md` §Critical Warnings (capability-set warning gains the second gate) + the pointer table.
- `security-plan.md` → `docs/security-summary.md`. `test-plan.md` → `docs/tests-summary.md`. `layout-templates.md` → `docs/design-summary.md` (its standing leaf — layout-templates has no summary doc of its own).
- `obs-plan.md` → `docs/obs-summary.md` carries no coverage-roll-up content, so no leaf edit was warranted.
- `docs/stack.md` unchanged — arch's §Stack table was not amended.
- `.claude/rules/security.md` + `rules/verification-harness.md` entries are `## Session Additions` content → owned by P3 curation, not the cascade.

**Pointer-table re-compute:** `CLAUDE.md:52` still named `conductor-0.1.0/working-route.md` while the active version is 0.2.0 — the baked-version-path staleness the re-compute rule anticipates (it carries no amended wording, so a wording-scan would miss it). Fixed **version-agnostically** ("the active version's `working-route.md`") so it cannot rot at the next version transition.
