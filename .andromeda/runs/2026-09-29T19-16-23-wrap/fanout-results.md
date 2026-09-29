# Fan-out results — 2026-09-29-dual-license-mit-or-apache-2-0

Report: `conductor-0.3.0/chunks/2026-09-29-dual-license-mit-or-apache-2-0/report.md`. Seven Explore doc-agents, one
parallel batch, prompt sent verbatim (no `{contracts_line}`: `registry.py contracts` → `NOT MIGRATED` for
architecture / test-plan / obs-plan / a11y-plan, `n/a` for the other three). Entity probe: no `&lt;` `&gt;` `&amp;` in
any return.

## Verdicts
- **architecture** — `proposals: []`. Stripped: per-detector basis comments + a note that the two arch expected
  amendments (Directory structure; Build system) are uncovered by its four detectors — "missing information, not a
  false statement". Raw twin `.raw-fanout-architecture.md`.
- **security-plan** — `proposals: []`. Nothing stripped; no twin.
- **design-system** — `proposals: []`. Stripped: basis comments (no UI, no count moved, no platform verdict retired).
  Raw twin `.raw-fanout-design-system.md`.
- **layout-templates** — `proposals: []`. Stripped: basis comments (no surface; `:163`/`:334` license/deny mentions
  are Tauri capability guardrails, unrelated). Raw twin `.raw-fanout-layout-templates.md`.
- **test-plan** — `proposals: []`. Stripped: basis comments; its own sweep `private|unpublished|licen[cs]e|deny.toml`
  hit `:96 :216 :459 :481 :512 :572 :605`, each naming `cargo deny check` as a gate, none stating the removed
  exemption. Raw twin `.raw-fanout-test-plan.md`.
- **obs-plan** — `proposals: []`. Stripped: basis comments; the deny gate kind is named at §9 (`:521`) and §10
  (`:573`), its kind unchanged. Raw twin `.raw-fanout-obs-plan.md`.
- **a11y-plan** — `proposals: []`. Stripped: basis comments. Raw twin `.raw-fanout-a11y-plan.md`.

## Validate
- Detector proposals: 0 — checks 1-4 have nothing to act on.
- Check 3 (intent-consistency): the report matches the working-route entry (`:52`) and the plan's acceptance
  criteria; scope record none (`gate.py scope` clean, 0 recorded).
- Check 6 (disproved claims): the report's bullet is "none".
- Check 5 (expected amendments) — the plan lists three; no detector proposed any, so the orchestrator raises each:
  - **X1** security-plan §Dependency Security (Accepted exceptions) — the own `conductor-*` crates are license-checked
    (no `private` exemption), carry `MIT OR Apache-2.0` inherited from `[workspace.package]`; allow / ignore counts
    unchanged (16 + 9). Report: Changes → Schema / config. Site `security-plan.md:184`.
    **Disposition: apply** — routine, playbook `:308` (accurate this-chunk addition inside an existing section; every
    clause holds: the fact is in the report's Changes, the section exists). Not a widening (playbook `:124`): the
    change NARROWS an exemption.
  - **X2** architecture §Infrastructure Patterns → Directory structure — the tree gains the repo-root license file
    set. Report: Changes → Files. Site `architecture.md:219-246`. **Disposition: apply** — routine, playbook `:308`.
  - **X3** architecture §Infrastructure Patterns → Build system — `deny.toml`'s license policy covers the workspace's
    own crates. Report: Changes → Schema / config + Harness. Site `architecture.md:212` (window @c5163 read).
    **Disposition: apply** — routine, playbook `:308`.
- Escalations: 0.

## Apply
- X1 → `security-plan.md:184` (sentence appended) · X2 → `architecture.md:225` (tree row) · X3 → `architecture.md:212`
  (sentence after the Tauri-tree license narrative). Bodies re-read after the edit.
- Cascade: `cascade-dispositions.md` (sweep after all three); leaves re-derived — `.claude/docs/security-summary.md`,
  `.claude/rules/security.md` §Dependencies; one curation-home hit routed to P3
  (`.claude/docs/session-learnings.md:488-492`).
- Sidecars: `security-plan-amendments.md` +6 (line 451) · `architecture-amendments.md` +8 (line 1031), both
  `sidecar.py check` on-form, `splice.py append`, last line read back = this entry's `Ref`.
- D-arch-registry-size: `registries: within target` (ED 38 105 B · OR 37 955 B of 38 115 B), after apply.
- Drift = 0: 3 applied · 0 escalated · 0 open.
