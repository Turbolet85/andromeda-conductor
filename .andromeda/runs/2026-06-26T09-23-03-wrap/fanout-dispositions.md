# P2 fan-out dispositions — 2026-06-25-scenario-suite-picker-start-stop

7 doc-agents · 5 proposals (all warning) · 0 escalate-severity fired.

| doc | proposals | disposition |
|---|---|---|
| security-plan | [] | clean — new IPC input validated (`validate_selection` + `resolve_under`); `cmdk`/`tracing` audit-green |
| design-system | [] | clean — both new UI elements tokens-by-name + not-color-alone |
| test-plan | [] | clean — core unit tests + frontend build-gate per §4; harness/envelope untouched |
| obs-plan | [] | clean — 3 commands instrumented `tauri.command.*`; tracing (no OTel SDK); `sanitize_error` edge |
| a11y-plan | [] | clean — picker keyboard combobox + not-color-alone; no schema change |
| arch | 4 (D-arch-resources ×3, D-arch-decisions ×1) | **over-reach → dismiss** (new flavor → playbook rule) |
| layout-templates | 1 (D-layout-surface) | **dismiss** — invariant met (surface already in §Wireframe + §Component Primary navigation) |

## arch dispositions (all dismiss — over-reach family)
1. D-arch-resources → register `list_scenarios`/`start_run`/`stop_run` names in §Occupied Resources. DISMISS: arch §Occupied Resources already registers the Tauri command surface at CATEGORY grain ("start/stop, scenario/suite picker, run-report view, operator-pause prompt; one Channel"); the concrete handler names are the realization arch omits (Tauri-command-name flavor of the CLI-verb / Tauri-capability-perm / library-symbol over-reach).
2. D-arch-resources → itemize `conductor-tauri::commands` / `conductor-core::scenario_catalog` modules (+ a fabricated "state" module) in §Crate names. DISMISS: §Crate names lists workspace MEMBERS, not internal modules (content within already-registered crates) — covered by the library-symbol over-reach rule.
3. D-arch-resources → add `cmdk` to §Stack table. DISMISS: arch §Stack summarizes the frontend stack (React/Vite/Tailwind) and never enumerated shadcn/Radix/Lucide; the component-library decision lives in design-system §Component Patterns §5 ("shadcn Command/Select (Radix-driven)") — cmdk is its engine, recorded in package.json + gated by npm audit.
4. D-arch-decisions → add a "[Frontend Component Pattern] cmdk + token styling" Established Decision. DISMISS: frontend-component-styling is design-system/frontend's altitude, not an arch Established Decision (which locks language/runtime/DB/module-boundaries). The cmdk-direct realization → P3 curation (frontend.md).

## layouts disposition
D-layout-surface → add a dedicated ScenarioPicker/RunControls component section. DISMISS: the agent's own reading confirms the picker+controls are in §Wireframe (Run console idle control row: `[Scenario/suite v] [Start] [Stop]`) + §Component Primary navigation ("shadcn Command/Select picker", "Start/stop keybindings") — the invariant ("described in the wireframes") is MET; a dedicated subsection exceeds the detector's mandate (nicety, not drift).

## Curation routing (P3)
- cmdk-direct over Radix + token `.css` (not vendored shadcn-cli utility files; no Lucide) → frontend.md Tier 2.
- Tauri 2 app commands not ACL-gated → security.md Tier 2 (or frontend).
- `#[tracing::instrument]` doesn't stack under `#[tauri::command]` → use `info_span!().entered()` → gotchas/frontend Tier 2.
- name-collision (command fn == imported fn) breaks `generate_handler!` → gotcha Tier 2/3.
