# A11y validation — route draft

## No suggestions
Draft covers the a11y domain by retiring it: the plan's only assertable surface (a11y-plan §1 — desktop-webview) leaves in Epoch 1, and the surviving command line is declared not-assertable (§1 entity `conductor-cli`; §11 Universal). Inserting any of the 7 Bootstrap items would add a capability this version removes (v4-01), so 0 of 7 appear as build chunks and none is suggested.

- **Bootstrap items present:** none as build chunks, each absent because its subject is the retired webview (a11y-plan §3 key file `bootstrap-phases-derive-for-route-setup-project.md`); each is covered on the retirement side instead:
  - a11y-tooling-install → leaves with the frontend in "Conductor's window retired" (Epoch 1).
  - focus-management-library-install → the plan installs no library (focus trap comes from the window's dialog component); leaves with the window crate, same chunk.
  - aria-component-library-install → the plan installs no library beyond the window's component set; leaves with the window crate, same chunk.
  - contrast-verification-harness-setup → "webview harness arms" in "Conductor's window retired" (Epoch 1).
  - screen-reader-test-spec-setup → "webview harness arms" in the same chunk; its PowerShell helpers also fall under "Linux-only base CI and harness" (Epoch 1).
  - a11y-ci-gate-wire → "frontend and a11y CI jobs" in "Conductor's window retired" (Epoch 1).
  - violation-json-emission-wire → the violation record directory and webview fixtures leave in "Panel-shaped types retired" (Epoch 1); the CI step that asserts it leaves with the a11y job.
- **Sequencing deps satisfied:**
  - Tooling / focus / ARIA installs before UI feature chunks → confirmed vacuously; no chunk in Epochs 1–8 builds a rendered surface.
  - Contrast harness before the full CI gate, and gate reachability in Foundation → not applicable; both are removed in Foundation's first chunk rather than built.
  - Retirement before dependents → confirmed; the window leaves first, so no later chunk has to keep an a11y gate green against a surface that is going away.
- **Coverage:**
  - Must-be-accessible paths (a11y-plan §1 Critical paths, §5): all four lose their surface. Scenario pick/start and run-report view leave in Epoch 1 ("Conductor's window retired", "Panel-shaped types retired"). The operator-pause and operator-checklist render sites leave in Epoch 1, and the hold and checklist themselves in "Window grading retired" (Epoch 5).
  - Six-state lamp and its label set (a11y-plan §6 State colour tokens): leaves in "Old verdict vocabulary retired" (Epoch 5).
  - Per-surface verification chunk: none owed. The plan bars treating the command line as an a11y verification path (§11 Universal), so no Polish a11y chunk is suggested for it.
  - Not-colour-alone on the command line (a11y-plan §1 `conductor-cli` entity — text-paired status as output discipline): carried by "Run read from the command line — … status always a text label" (Epoch 3).
  - Standing a11y records and rules: "Records restated" and "No-survivor sweep" (Epoch 8) cover them. The path-scoped a11y rule matches only window-crate and `.tsx` paths, so it stops loading once Epoch 1's first chunk lands and gives no wrong instruction in between.
  - Answer to the open question in `plans-not-opened.md`: `a11y-plan.md` has no command-line part to build on. Its only command-line content is the not-assertable verdict (§1, §4, §5, §11), so the "window is its whole subject" premise holds for this plan.
  - One dependency for the wrap of "Conductor's window retired", not a route change: a11y-plan §6's state-naming crosswalk is cited by code that outlives the window — `crates/conductor-report/src/report.rs:327` and `crates/conductor-run/tests/journal_conformance.rs:168`. Those citations stay live until "Run record for the one form" (Epoch 3) re-bases the conformance gate and "Old verdict vocabulary retired" (Epoch 5) removes the states. Re-subjecting the plan at the Epoch 1 wrap should keep that crosswalk reachable until then.
  - Read basis: `a11y-plan.md` whole (§1–§6, §9–§12) and the Bootstrap key file; the other eight §3 key files were not opened.
