# A11y validation — route draft

## No suggestions
Draft covers the a11y domain by retiring it. The plan's only assertable surface (a11y-plan §1 — desktop-webview) leaves in Epoch 1's first chunk, and the surviving command line is declared not-assertable (§1 entity `conductor-cli`; §11 Universal). Building any of the 7 Bootstrap items would add a capability this version removes (v4-01), so none appears as a build chunk and none is suggested. The draft differs from the second pass's only in the control-pairing wording of "Discrimination at description time" (Epoch 2) and of three Epoch 4 regression-run chunks, one of them split in two; none touches an a11y subject, so the verdict is unchanged.

- **Bootstrap items present:** none as build chunks; each is absent because its subject is the retired webview (a11y-plan §3 key file `bootstrap-phases-derive-for-route-setup-project.md`), and each is covered on the retirement side:
  - a11y-tooling-install → leaves with the window crate in "Conductor's window retired" (Epoch 1).
  - focus-management-library-install → the plan installs no library (the focus trap comes from the window's dialog component); leaves with the window crate, same chunk.
  - aria-component-library-install → the plan installs no library beyond the window's component set; leaves with the window crate, same chunk.
  - contrast-verification-harness-setup → "webview arms" in "Conductor's window retired" (Epoch 1).
  - screen-reader-test-spec-setup → "webview arms" and "window scripts" in the same chunk; the PowerShell side also falls under "Linux-only base CI" (Epoch 1).
  - a11y-ci-gate-wire → "frontend and a11y CI jobs" in "Conductor's window retired" (Epoch 1).
  - violation-json-emission-wire → the violation-JSON conformance step belongs to the `a11y` job (`/home/turbolet/dev/projects/conductor/.github/workflows/ci.yml:764-782`) and leaves with it; the `runs/a11y/` record directory leaves in "Panel-shaped types retired" (Epoch 1, per `1a-tree.md`).
- **Sequencing deps satisfied:**
  - Tooling / focus / ARIA installs before UI feature chunks → confirmed vacuously; no chunk in Epochs 1–8 builds a rendered surface.
  - Contrast harness before the full CI gate, and gate reachability in Foundation → not applicable; both are removed in Foundation's first chunk, not built.
  - Gate and surface leave together → confirmed; the `a11y` job and the window crate go in the same chunk, ahead of "Linux-only base CI", so the surface never ships ungated and the gate never runs against a deleted crate.
- **Coverage:**
  - Must-be-accessible paths (a11y-plan §1 Critical paths, §5): all four lose their surface. Scenario pick/start and run-report view leave in Epoch 1 ("Conductor's window retired", "Panel-shaped types retired"). The operator-pause and operator-checklist render sites leave in Epoch 1, and the hold and checklist themselves in "Window grading retired" (Epoch 5).
  - Six-state lamp and its label set (a11y-plan §6 State color tokens): the manual and degraded states and their lamps leave in "Model and manual vocabulary retired" (Epoch 5), which keeps the recessive non-lamp tint (§1 non-lamp reuse).
  - Per-surface verification chunk: none owed. §11 Universal bars treating the command line as an a11y verification path, so no Polish a11y chunk or SC-ID list is suggested for it.
  - Not-colour-alone on the command line (a11y-plan §1 `conductor-cli` — text-paired status as output discipline, not an assertion): carried by "Run read from the command line — … closed text-label set" (Epoch 3).
  - Standing a11y records: `1a-tree.md` names a11y-plan and its nine keys as retired by "Conductor's window retired", re-subjected at that chunk's wrap. "Records restated" and "No-survivor sweep" (Epoch 8) cover `/home/turbolet/dev/projects/conductor/.claude/rules/a11y.md` and `/home/turbolet/dev/projects/conductor/.claude/docs/a11y-summary.md`. The rule's path scope is the window crate plus `**/*.tsx` / `**/*.jsx`, so it stops loading once the first chunk lands.
  - Wrap note, not a route change: code that outlives the window still cites the plan, so the Epoch 1 re-subjecting should keep the §6 state-naming crosswalk reachable until Epochs 3 and 5. The sites are `crates/conductor-report/src/report.rs:327` and `crates/conductor-run/tests/journal_conformance.rs:168` (the crosswalk), and `crates/conductor-core/src/run_record.rs:14` and `crates/conductor-core/src/scenario.rs:81` (the violation record and §1 Critical paths). They stay live until "Run record for the one form" (Epoch 3) re-bases the conformance gate and Epoch 5 removes the states and the checklist.
  - Read basis: `a11y-plan.md` whole (§1–§6, §9–§12; it has no §7 or §8) and the Bootstrap key file; the other eight §3 key files were not opened.
