# design extract

## Relevance
Partial — a11y/test-harness setup chunk, not a visual surface implementation. Tests design artifacts (tokens, contrast, ARIA bindings, motion) and verifies design invariants via tooling.

## Constraints
1. **Token-only color/spacing/motion/radius in test code** — per §Surface: desktop-webview / Tokens, all values in harness assertions must reference CSS custom properties (`--color-*`, `--space-*`, `--motion-*`, `--radius-*`, `--ease-*`); zero hardcoded hex, RGB, px, or ms literals. The colorjs.io contrast checker reads `:root` declarations on `ui/src/styles/tokens.css` (34-token binding contract per amendment 2026-06-15).
2. **WCAG AA token-pair contrast required** — per §Color Palette, every foreground/background color pair must meet SC 1.4.3 (4.5:1 normal text, 3:1 UI components). Harness must verify pairs from `tokens.css` `:root` (e.g., `--text-primary` on `--color-base`, `--count-nominal` on `--color-raised-1`) and report violations by token pair name.
3. **State never color-alone** — per §Anti-Patterns (Universal Bans) + §Iconography, verdict/report-state signals (`--count-nominal` / `--count-hold` / `--status-fail` / `--count-blocked` / `--status-manual` / `--status-residual`) must be paired with text label or icon (axe SC 1.4.1 rule must pass); harness must verify no color-only signals in Blocked / Fail / ManualCheck rows.
4. **Motion respects prefers-reduced-motion** — per §Motion (Hard limits at expression 0.3), all transitions MUST drop under `@media (prefers-reduced-motion: reduce)` override (SC 2.3.3); harness must verify operator-pause dialog fade (`--motion-micro` 150ms) is fully suppressed, never persists under the override.
5. **Operator-pause dialog fade = var(--motion-micro) (150ms)** — per amendment 2026-06-26, the fade is bound to the token, not a raw 200ms literal. Harness must verify the fade uses `--motion-micro` token reference, never an inline duration.
6. **ARIA bindings on shadcn/Radix primitives assertable** — per §Component Patterns (§2 Operator-pause dialog, §3 Coverage matrix, §7 Operator-checklist), harness must assert ARIA roles (alertdialog on pause dialog, role=status on unticked-count), focus trap + restore, visible focus ring fade via `--color-focus`.
7. **Tokens 34/34 emit in dark + light modes** — per amendment 2026-06-15, `:root` declares all 34 tokens (no Tailwind v4 `@theme` tree-shake loss); harness must verify light-mode overrides under `@media (prefers-color-scheme: light)` for all named tokens (surfaces, status, text, border, motion).

## Patterns to follow
1. **Token-pair contrast verification via colorjs.io** — read `:root` declarations, compute contrast ratio for each foreground/background pair (§Color Palette tables: Semantic Colors, Text Hierarchy, Surface Scale), verify against WCAG AA thresholds, output violations by token pair name + computed ratio.
2. **ARIA assertion via tauri-driver on keyboard interaction** — verify shadcn/Radix components carry correct roles (alertdialog + focus-trap + Escape→NoGo on pause dialog; role=status + Space-toggle on checklist); tauri-driver tests must assert focus restoration after dialog close and visible focus ring via `--color-focus` fade-in.
3. **Reduced-motion CSS rule verification** — harness must verify `@media (prefers-reduced-motion: reduce)` CSS rule exists and suppresses transitions; test with prefers-reduced-motion override + observe that operator-pause fade drops completely, no motion persists.
4. **Color + label pairing assertion** — per test-plan §3 (a11y harness), every verdict/report-state row must carry text label (`Pass` / `CalibrationRegion` / `Fail` / `Blocked` / `Manual` / `Residual`) + colored lamp; harness must assert both present before signaling SC 1.4.1 pass.

## Anti-patterns to avoid
1. **Color-only state signals in assertions** — never verify state by color token alone; Proceed/Abort buttons, verdict/report-state rows, and ManualCheck items must always carry text labels paired with color. Blocked must never render red (verify `--count-blocked` or slate-violet, never `--status-fail`).
2. **Animation library motion in test harness** — banned at expression 0.3; harness must NOT test framer-motion, spring physics, pulse, blink, or glow effects on status lamps. CSS transitions only via `--motion-*` tokens.
3. **Literal duration/easing values in assertions** — every transition must cite the token (e.g., `var(--motion-micro)`, `var(--ease-quiet)`), never inline `150ms` or `cubic-bezier(0,0,0.2,1)` (enforces token binding per amendment 2026-06-26 spec-illustration → sound-impl reconciliation).

## Contract bindings
- **Token contrast ↔ a11y §Contrast** — colorjs.io checker verifies `--color-*` pairs meet WCAG SC 1.4.3; design-system §Color Palette is source of truth for token values and semantics (Success / Warning / Error / Info / Manual / Residual).
- **Motion tokens ↔ a11y §Animation SC 2.3.3** — operator-pause fade (`--motion-micro` 150ms) and heartbeat breath (`--motion-heartbeat` 1600ms — amendment 2026-06-24) must be dropped under `prefers-reduced-motion: reduce` override; harness verifies CSS rule presence and behavioral suppression.
- **Color + label ↔ a11y §Use of Color SC 1.4.1** — every verdict/report-state signal (Pass / CalibrationRegion / Fail / Blocked / ManualCheck / KnownResidual) must carry text label + icon/color; axe SC 1.4.1 rule enforces pairing.
- **ARIA bindings ↔ test-plan §Path 7 (cross-surface parity)** — tauri::test mock-runtime + tauri-driver assertions verify desktop-webview ARIA layer mirrors CLI observable behavior; design-system §Component Patterns defines the ARIA contract.

## Acceptance criteria contributions
1. "(design) All color/spacing/motion/radius/easing values in the a11y harness reference design tokens; zero hardcoded hex, RGB, px, or ms literals in test code or assertions (tokens-by-name invariant per amendment 2026-06-15)."
2. "(design) Token-pair contrast check (colorjs.io) verifies all 34 `:root` tokens (dark + light modes) per §Color Palette Semantic Colors / Text Hierarchy / Surface Scale; report lists any pair with ratio < 4.5:1 (normal text) or < 3:1 (UI components); zero violations on green approval."
3. "(design) Verdict/report-state colors (--count-nominal / --count-hold / --status-fail / --count-blocked / --status-manual / --status-residual) verified SC 1.4.1 (color + label, never color-alone); axe rule passes, every row carries text label + icon."
4. "(design) Operator-pause dialog fade uses var(--motion-micro) (150ms) and respects prefers-reduced-motion: reduce override; tauri-driver verifies transition is suppressed under override, no motion persists (SC 2.3.3)."
5. "(design) ARIA bindings on shadcn/Radix components asserted: OperatorPauseDialog carries alertdialog role + focus trap + Escape→NoGo; OperatorChecklistView carries role=status on unticked-count + Space-toggle; CoverageMatrix restores focus after interaction; focus ring visible via --color-focus fade-in per --motion-micro."

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** — Tokens declared on `:root` (not Tailwind v4 `@theme`); all 34/34 emit with no tree-shake loss. Harness must read `:root` declarations from `tokens.css` with explicit verification that all 34 tokens are present in both dark and light modes.
- **2026-06-24-paused-count-hold-point-signature** — `--motion-heartbeat: 1600ms` registered as a token (the signature heartbeat breath period). Harness must verify this token is declared and drives the live-counter breath duration, not a raw 1600ms literal.
- **2026-06-26-component-primitives-library** — Operator-pause dialog fade reconciled from spec's aspirational 200ms → `var(--motion-micro)` (150ms, the actual shipped token). Harness must verify the fade uses the token value, never a 200ms literal (token-binding invariant per spec-illustration → sound-impl reconciliation routine).