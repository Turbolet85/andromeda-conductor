# layouts extract

## Relevance — relevant

This chunk is the CLI interactive resolver for operator pauses (isatty-gated confirm + paused-count spinner mirror), which is named in §cli Component — Hero / signature output as a layout placement.

## Constraints — domain rules that apply

1. **Signature placement requirement** — the paused-count spinner MUST reflect the held state as the CLI mirror of the desktop titlebar freeze (per layout-templates §Surface: cli Signature placement); the hold is placed in **exactly 2 distinct components**: (a) the `indicatif` heartbeat spinner stops in place (primary), (b) a bold hold-amber `HOLD — operator pause` line prints above the `inquire` prompt carrying the frozen count as text (placement §2).
2. **Status never color-alone** — every hold state must pair its color with the `[HOLD]` ASCII prefix (per layout-templates §cli Component — Primary content block 1/2 + Component — Header / banner); color is tty-gated optional overlay, prefix is required for `NO_COLOR` / piping compliance.
3. **Interactive resolver belongs to the `inquire` confirm pattern** — the hold's human-readable prompt + scenario / p_id / step context presented via `inquire` confirm (per layout-templates §cli Component — Hero / signature output); mapping answer to `Decision::Go` / `Decision::NoGo` when `allow_no_go == true`; proceed-only acknowledgment when `allow_no_go == false` (scope §2).
3. **Status never color-alone** — every hold state must pair its color with the `[HOLD]` ASCII prefix (per layout-templates §cli Component — Primary content block 1/2 + Component — Header / banner); color is tty-gated optional overlay, prefix is required for `NO_COLOR` / piping compliance.
4. **Non-TTY path never blocks** — piped/agent runs auto-resolve via `HeadlessResolver::proceed()` (per layout-templates §cli Primary screens + §IA notes Headless invariant); no interactive prompt written to a pipe; the source-of-truth agent path is never gated on a prompt (scope §1.1 + acceptance intent).
5. **Paused indicator reuses ch3's `Lamp`→line mapping** — the hold phase-line uses the existing verdict-first lamp precedence established by ch3's render seam; the `[HOLD]` prefix always present, color is tty-gated overlay (scope §3 + layout-templates §cli Component — Header / banner).
6. **Determinism preserved** — the operator pause is wall-clock-only, outside the seeded virtual clock (scope Acceptance intent, anchor P5 validation-1); tests drive the resolver non-interactively (auto-headless or stub resolver), never a real TTY wait.

## Patterns to follow

1. **Signature hold-moment phrase** — match the exact phrasing `HOLD — operator pause` (with context detail like `step 14 · 00:01:47`) per layout-templates Example outputs (§cli Output structure — `conductor run <scenario>` line 190; §§desktop-webview Wireframe — Run console (HOLD) line 63).
2. **`indicatif` honest progress** — the spinner is a status indicator that appears only after ~200ms and reflects actual progress; on hold, it **STOPS in place** (not hide, not animate-to-100%), the stop is the signature (layout-templates §cli Expression level + Component — Hero / signature output).
3. **Redaction in rendered prompts** — the hold's non-Conductor action is presented to the operator; no host paths, no internal struct names, no raw types leak through (scope § Surfaces/contracts + a11y section).
4. **Keyboard-driven resolve** — `inquire` default is keyboard-first; the interactive prompt supports both TTY and non-TTY gracefully, with the TTY path being the human-facing colorized resolve (layout-templates §cli Primary navigation + scope §2).

## Anti-patterns to avoid

1. **Never emit a prompt to a pipe** — an interactive prompt or color-only status in non-TTY context hangs the agent path or corrupts parseable output (layout-templates §cli IA notes Headless invariant + Pipe discipline); always check `isatty` before interactive branches.
2. **Never let the verdict-only lamp variant stand alone** — every verdict/hold state cell must pair color with the ASCII bracket prefix (`[HOLD]`, `[PASS]`, etc.) so the signal survives `NO_COLOR` / piping (layout-templates §cli Component — Header / banner + scope Acceptance intent).
3. **Never introduce a second hold model or decision set** — the resolver implements the single `PauseResolver` trait + `Decision { Go, NoGo }` vocabulary defined in Epoch 5; zero new verdict/report model variants (scope §Boundaries explicitly OUT).

## Contract bindings — where your domain ties into another

- **a11y ↔ layouts** — the hold prompt status-never-color-alone rule (ASCII `[HOLD]` prefix paired with color) binds to a11y §Status convention (layout-templates §cli Component — Header / banner); keyboard-driven resolve binds to a11y §Keyboard-first interaction (no mouse-only affordances).
- **core (pause) ↔ layouts** — the interactive resolver implements `conductor_core::PauseResolver` trait and `Decision { Go, NoGo }` enum; the hold phase-line mirrors the desktop titlebar hold-point signature via the same `HoldPoint` vocabulary (layout-templates §Signature placement strategy, both surfaces) (scope §What it builds + layout-templates §§Surface: cli Signature placement + §desktop-webview Signature placement).
- **ch3 render seam ↔ layouts** — the paused indicator reuses the existing `Lamp`→line mapping and `stdout_color()` tty-gate primitive already established (scope § Surfaces/contracts Reads); the amber `HOLD` color derives from design-system (scope § Design/layout).

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. **(layouts) Hold phase-line on TTY** — when `isatty(stdin)` is true, a `HoldPoint` renders a bold amber `HOLD — operator pause` line with the frozen count as text, paired with `[HOLD]` ASCII prefix (layout-templates §cli Component — Hero / signature output placement §2).
2. **(layouts) Paused spinner stops in place** — the `indicatif` heartbeat halts at the exact hold value (not hide, not animate-to-100%); the stop is the signature mirror of the desktop titlebar freeze (layout-templates §cli Expression level + §cli Signature placement primary).
3. **(layouts) Status never color-alone on non-TTY** — on piped / `NO_COLOR` path, the `[HOLD]` ASCII prefix is present and unambiguous; color is stripped; the hold state remains legible (layout-templates §cli IA notes Pipe discipline + scope Acceptance intent).
4. **(layouts) Non-TTY path never blocks** — when `isatty(stdin)` is false, the resolver does not emit an interactive prompt; the hold auto-resolves via `HeadlessResolver::proceed()` and records the decision (layout-templates §cli IA notes Headless invariant + scope acceptance intent).

## Relevant amendment history

(none)
