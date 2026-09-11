# Design validation — route draft

## Insert
- Between `Scenario-assertion audit gate` and `A11y CI gate at an honest terminal`: **"Console footer status strip — the designed seed · phase · verdict roll-up line, including the unticked-ManualCheck count, present on the desktop-webview surface"** (epoch: `Epoch 3`)
  Reason: layout-templates §Component — Footer (status strip) marks it "DESIGNED, NOT SHIPPED … route-owned gap" and it is the only per-surface layout primitive (header / content / footer) with no shipped scaffold, while the tokens bundle, titlebar header, coverage/report content blocks and component primitives all ship from 0.1.0–0.2.0; anchored ahead of the Epoch-3 chunks that measure the console DOM so the surface stops moving before its terminal is fixed.

- Between `Console footer status strip` and `A11y CI gate at an honest terminal`: **"Operator-checklist at the run-report site — the shipped primitive's second designed mount, ManualCheck rows carrying induced state and expected observation"** (epoch: `Epoch 3`)
  Reason: layout-templates §Component — Operator-checklist and design-system §Component Patterns 7 both record the report-site render as unbuilt and route-owned (SR pass row S2-08), and `crates/conductor-tauri/ui/src/components/RunReport.tsx` confirms only the `StatusLamp` line renders there today while `OperatorChecklistView` mounts solely in `OperatorPauseDialog.tsx`; sequenced directly after the footer chunk so both console-surface moves land together.
