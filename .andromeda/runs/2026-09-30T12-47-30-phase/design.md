# design extract

## Relevance
partial — the chunk is a screen-reader cause-isolation diagnosis with no planned change to any rendered Conductor surface; design's stake is guarding the focus-ring and webview-posture contracts the controls read against, not authoring anything new.

## Constraints
- The keyboard focus indicator of Conductor's webview is specified as a `2px solid var(--color-focus)` `outline` under `:focus-visible`, with no `box-shadow`, asserted by the routine a11y arm through computed `outlineStyle`/`outlineColor` (per design-system §Depth Strategy). That contract is visual; whether it has any bearing on UIA focus-event delivery to NVDA is research's question, and nothing in the design plan claims it does. The chunk must not alter it as a diagnostic step.
- The focus ring appears with NO transition, so there is no focus fade to remove under reduced motion (per design-system §Motion "This project's values"). A control that varies Conductor's posture must not introduce a focus transition, because a new transition would bring in the reduce-motion obligation.
- Focus-ring colour is the reserved ID-cyan token `--color-focus` (dark `#7DCFFF` / light `#0969DA`), bound by `var(--…)` name (per design-system §Color Palette → Border Progression "Focus"; §Surface: desktop-webview → Tokens). Any evidence or fixture that describes Conductor's focus styling refers to the token by name, never by a hex literal.
- The desktop webview must suppress Chromium/WebView2 artifacts (context menu, devtools, text-selection on non-text elements) (per design-system §Surface: desktop-webview → Platform-Specific Notes; §Anti-Patterns → Per-Surface Bans, desktop-webview). The named-but-unbuilt product lever (Tauri `additionalBrowserArgs` / the loader's browser-arguments variable) sits on exactly this seam. That is one more reason it stays unbuilt here, and any future chunk that builds it owes a check that suppression still holds.
- Status is never colour-alone: every verdict/state carries a text label (desktop) or an ASCII prefix (cli) (per design-system §Iconography "Rule"; §Anti-Patterns → Per-Surface Bans, cli). This applies to any regrade or routing of the three owned reds that reaches a rendered or cli surface, as opposed to the evidence record alone.

## Patterns to follow
- A diagnostic control that stands a plain focus page beside Conductor is NOT a Conductor surface. The design plan governs the `desktop-webview` and `cli` surfaces only (per design-system §Surface: desktop-webview; §Surface: cli), so a no-boundary control page needs none of Conductor's tokens and should not borrow them. Keeping it unstyled keeps the control honest as "the same Chromium without Conductor's posture."
- When the evidence record describes Conductor's side of a control, it cites the focus contract by its plan anchor (outline, `--color-focus`, untransitioned) rather than restating values (per design-system §Depth Strategy; §Motion).

## Anti-patterns to avoid
- NEVER add `backdrop-filter`/drop shadows, including a `box-shadow` focus ring swapped in as an experiment (per design-system §Anti-Patterns → Per-Surface Bans, desktop-webview; §Depth Strategy "Drop shadows: none").
- NEVER ship with visible Chromium/WebView2 artifacts. A browser-arguments change, if ever tried, must not re-expose devtools or the context menu (per design-system §Anti-Patterns → Per-Surface Bans, desktop-webview).

## Contract bindings
- design ↔ a11y: the focus-ring contract (§Depth Strategy, §Motion) is the visual half of the a11y plan's focus-visible and screen-reader rows. The routine a11y arm asserts the ring's computed style; the SR leg grades the announcement. This chunk's controls touch only the SR half, so the ring contract should read unchanged through the strict `--e2e` arm.
- design ↔ security: the webview artifact-suppression note shares §Platform-Specific Notes with the security plan's Tauri guardrails. The unbuilt browser-arguments lever touches both.

## Acceptance criteria contributions
- (design) No change to `--color-focus`, to the `:focus-visible` outline rule, or to any focus transition in `crates/conductor-tauri/ui` against the chunk base `4460307`. Pass means `git diff --numstat 4460307` shows no styling delta to the focus contract (per design-system §Depth Strategy; §Motion).
- (design) Any fixture or evidence text that names Conductor's focus styling refers to `--color-focus`/the outline contract by token or anchor, never by a hex or pixel literal (per design-system §Color Palette; §Self-Validation Protocol → Token Test).
- (design) If the three owned reds are routed or regraded onto any rendered or cli status surface, each state carries its text label or ASCII prefix, not colour alone (per design-system §Iconography; §Anti-Patterns → Per-Surface Bans, cli).
