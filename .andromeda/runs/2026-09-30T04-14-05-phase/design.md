# design extract

## Relevance
partial — no scope item builds or restyles a rendered surface. Items 1-3 (digest pins, workspace-key mask, storm-prefix elision) are test-source and evidence work with no design surface. Design touches only item 4 (the routine a11y arm's axe gate asserts over the token-rendered Minimal-tier baseline, contrast included) and item 5 (the `agent-run.sh run --live real-model` cli output the series drives through).

## Constraints
- The item-4 fix repairs the harness's frame-readiness race only. It may not change any token value to make the axe gate pass. The Text Hierarchy tertiary and muted tokens were each moved so they clear 4.5:1 on every surface they render on, including `--color-raised-1`, `--color-raised-2` and `--color-inset`, and the axe `color-contrast` rule is what measures that. Whether the current tokens still hold 4.5:1 once the gate actually runs to completion is research's question. (per design-system §Color Palette → Text Hierarchy)
- The desktop-webview token block is the binding contract for names and values: tokens are declared on plain `:root` (not `@theme`), dark is the default, and light comes from `@media (prefers-color-scheme: light)`. The a11y baseline the routine arm scans is rendered from this block. A fix that stubs, injects or overrides styles in order to reach frame readiness would put a page under the scan that is not the shipped one. (per design-system §Surface: desktop-webview → Tokens (platform-specific))
- `@media (prefers-reduced-motion: reduce)` drops every transition and animation, and that override is mandatory at expression `0.3`. If P3 finds that a transition affects frame readiness, the fix must keep this override and the `--motion-micro` / `--ease-quiet` tokens intact. It must not add an animation or a wait keyed to one. (per design-system §Motion → Hard limits for this expression level)
- Status in cli output is never shown by color alone. Per-P-ID lines carry the closed ASCII prefix set `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`. Run-level captions (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) are outside the lamp column and are neither a lamp state nor a `ReportState`. Any new line item 5's series adds to the run output (for example a real-model grade) must follow this discipline. Whether it can reuse an existing prefix or needs a new caption is a plan decision: design adds no seventh lamp and no sixth `ReportState`. (per design-system §Surface: cli → Tokens (platform-specific); §Component Patterns 4)
- Styling is gated by TTY: ANSI color only when the stream is a terminal, `NO_COLOR` is unset and `TERM != dumb`, decided separately for stdout and stderr. The headless agent path, which is how the series is driven, never blocks on a prompt, and its piped output carries no ANSI and no emoji. (per design-system §Surface: cli → Platform-Specific Notes; §Per-Surface Bans → cli)
- Error output goes to stderr, sanitized: no absolute host paths, internal struct names or stack traces outside `--debug`, in the form `error:` + `hint:`. This is the surface counterpart of item 2's never-printed workspace-key guarantee. It does not replace the capture-side mask. (per design-system §Surface: cli → Component Patterns 5)

## Patterns to follow
- A `Blocked` row carries its named precondition string and renders measurement columns as `—`/null, never as a red error. A drive that item 5's gates prevent from running is "no result yet", not `Fail`. (per design-system §Surface: cli → Component Patterns 3; §Rejected Defaults "Conflating no result yet with failed")
- Markdown has no color channel, so its counterpart to a color tier is emphasis or the bracket label in a blockquote. Any status the series writes into a Markdown artifact should use that pattern. (per design-system §Surface: cli → Tokens (platform-specific), Residual mute note)
- Mono ID-cyan is reserved for P-IDs, run_id, SLO timings and fingerprints. Fingerprint-shaped tokens that still render after elision stay in the mono tier. (per design-system §Color Palette → Core Colors)

## Anti-patterns to avoid
- Never use color purely for decoration, and never rely on color alone for status. (per design-system §Anti-Patterns → Universal Bans; §Per-Surface Bans → cli)
- Never lower a token's contrast, or suppress a scanned node's styling, to get an axe run green. The fix belongs to the harness's readiness path, not to the palette. (per design-system §Color Palette → Text Hierarchy; §Self-Validation Protocol → 4. Token Test)

## Contract bindings
- design ↔ a11y: token contrast is measured by the routine arm's axe `color-contrast` rule (a11y §Contrast, SC 1.4.3). The item-4 witness that "cannot pass vacuously" must show that axe actually ran its rules to completion over the rendered baseline. An inject that never reached the page must not read as zero violations.
- design ↔ a11y: the reduce-motion override binds to a11y SC 2.3.3, and the status-plus-label rule binds to a11y SC 1.4.1.
- design ↔ security: the cli sanitized-error surface binds to security-plan §Error Handling (no host paths in operator-facing output).

## Acceptance criteria contributions
- (design) The item-4 fix changes no value in the `:root` token block or its light and reduced-motion overrides. Check: `git diff --numstat 9785405` over the UI stylesheet shows no token-line change, or else a named, justified one. (per design-system §Surface: desktop-webview → Tokens (platform-specific))
- (design) The routine arm reports zero axe violations with `color-contrast` included and executed, not skipped or un-injected, on the Minimal-tier baseline after the fix. (per design-system §Color Palette → Text Hierarchy)
- (design) Any new status line the series adds to the cli output carries an ASCII bracket prefix or caption, not color alone, and emits no ANSI and no emoji when piped. (per design-system §Surface: cli → Tokens (platform-specific); §Per-Surface Bans → cli)
