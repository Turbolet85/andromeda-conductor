# A11y validation — route draft

## Insert
- Between `Operator-pause go/no-go dialog` and `Desktop a11y assertion harness`: **"A11y tooling install — axe-core + @axe-core/webdriverio + Lighthouse + colorjs.io over tests' tauri-driver"** (epoch: `Desktop control panel`)
  Reason: Per a11y-plan §3 Bootstrap (a11y-tooling-install), tooling is a primary bootstrap item that must precede any verification chunk; currently embedded in the aggregated harness chunk.
- Between `Operator-pause go/no-go dialog` and `Desktop a11y assertion harness`: **"Focus + ARIA library binding — shadcn/Radix AlertDialog trap/restore + pattern semantics (reuse, no install)"** (epoch: `Desktop control panel`)
  Reason: Per a11y-plan §3 Bootstrap (focus-management / aria-component-library), these reuse the in-stack shadcn/Radix but must be named as distinct binding items.
- Between `Operator-pause go/no-go dialog` and `Desktop a11y assertion harness`: **"Contrast-verification harness — colorjs.io token-pair ratios for SC 1.4.3 + SC 1.4.11"** (epoch: `Desktop control panel`)
  Reason: Per a11y-plan §3 Bootstrap (contrast-verification-harness-setup), it is a dedicated phase that must run before the CI gate.
- Between `Desktop a11y assertion harness` and `Coverage-matrix view`: **"Screen-reader test-spec — NVDA/VoiceOver/Orca per-state manual pass specs for the must-announce items"** (epoch: `Desktop control panel`)
  Reason: Per a11y-plan §3 Bootstrap (screen-reader-test-spec-setup), it is an explicit (supplemental) bootstrap item currently omitted.
- Between `Cross-surface parity proof` and `Coverage-matrix completeness gate`: **"A11y CI gate + violation JSON — axe/contrast/keyboard PASS/FAIL folded into obs envelope, service-tagged"** (epoch: `Polish & ship`)
  Reason: Per a11y-plan §3 Bootstrap (a11y-ci-gate-wire / violation-json-emission-wire), these final two items are Polish/CI-gated, not subsumed in one harness chunk.

## Rewrite
- `Desktop a11y assertion harness`: "axe via tauri-driver + colorjs.io contrast + ARIA patterns" → "axe violations + contrast ratios + keyboard trap/focus-order on the four must-be-accessible paths"
  Reason: Per a11y-plan §4 critical paths, the verification chunk should name the four accessible paths it asserts rather than re-listing the tool picks (now their own bootstrap chunks).
