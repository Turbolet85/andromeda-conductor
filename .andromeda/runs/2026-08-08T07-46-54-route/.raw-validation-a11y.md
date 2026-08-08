# A11y validation — route draft

## Insert
- Between `Operator-pause and checklist live firing` and `Desktop a11y sweep`: **"A11y harness install — axe-core and Lighthouse injected into the tests' existing tauri-driver/WebdriverIO webview session, no second automation stack"** (epoch: `Epoch 5`)
  Reason: a11y-plan §3 Bootstrap phases (a11y-tooling-install) has no chunk although the Epoch 5 sweep and gate consume that tooling; focus-management and aria-component installs correctly need no chunk since Radix/shadcn already ship the trap and roles.

- Immediately after the harness-install chunk, before `Desktop a11y sweep`: **"Token-pair contrast harness — named design-token foreground/background pairs asserted per-pair with machine-readable ratio output (SC 1.4.3, SC 1.4.11)"** (epoch: `Epoch 5`)
  Reason: a11y-plan §3 Bootstrap phases (contrast-verification-harness-setup) + §6 Color contrast pairs require a per-pair ratio harness wired to the nine named token pairs, which must exist before the a11y gate but appears nowhere in the draft.

- Between `Desktop a11y sweep` and `Screen-reader manual spec`: **"Six-state lamp and motion assertions — every state text-labelled plus glyph, all transitions dropped under reduced-motion (SC 1.4.1, SC 2.3.3)"** (epoch: `Epoch 5`)
  Reason: a11y-plan §1 A11y triggers records both token-driven triggers (visual-discrimination, motion-sensitive) and §10 makes not-color-alone and reduced-motion failures gate-failing conditions, yet neither assertion appears in any chunk.

## Reorder
- Move `Live per-P-ID verdict lamps` before `Desktop a11y sweep`
  Reason: the sweep's lamp contrast (SC 1.4.11) and the six-state not-color-alone assertion (a11y-plan §6 State color tokens) can only run once all six verdict states actually render in the coverage view.

## Rewrite
- `Desktop a11y sweep`: "keyboard/focus order across the four accessible paths on Linux+xvfb" → "keyboard order, HOLD-dialog trap escape and focus restoration across the four accessible paths on Linux+xvfb (SC 2.1.1/2.1.2/2.4.3/2.4.7/1.4.3)"
  Reason: a11y-plan §5 Focus trap/restoration and §10 SLO invariants name SC 2.1.2 No Keyboard Trap and SC 2.4.7 as zero-violation invariants that "focus order" alone does not assert, and verification chunks must name their SC IDs in scope.

- `A11y CI gate`: "A11y CI gate — violation JSON into the obs envelope, service-tagged, failing the build on any violation" → "A11y operator/local gate — violation JSON into the obs envelope, service-tagged, failing the gate on any violation with no retries"
  Reason: a11y-plan §9 CI Integration binds the desktop-webview a11y step as an operator/local gate riding tests' `wdio run` webview job (not a build-failing CI gate, since dynamic proof needs a live Pulse), and §10 bans any retry-once policy.
