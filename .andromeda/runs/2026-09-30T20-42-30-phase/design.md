# design extract

## Relevance
partial. This is a regression-gate chunk with no new surface. Design binds only where a red gate or a CARRY forces a change on a moved webview surface (the coverage row, the `h1` phase line, the `contentinfo` footer strip, the visually-hidden count), on the knip dispositions in `ScenarioPicker.tsx` / `CoverageMatrix.tsx` (C2), on the E0-10 banner regrade (C5), or on the cli refusal text of an ambiguous P-ID (C3).

## Constraints
- Any fix to a moved webview surface binds by `var(--…)` name to the token bundle declared on plain `:root` (not `@theme`), with its light-scheme overrides and the `prefers-reduced-motion: reduce` block. Per design-system §Surface: desktop-webview §Tokens (platform-specific). Whether the moved surfaces already use only tokens is research's question.
- The `[ENVIRONMENT-SUSPECT]` caption is a run-level qualifier in the shared recessive tier: ANSI 246 on the cli, `var(--status-residual)` in the webview. Its always-rendered text label carries the signal, and the tint only de-emphasizes. It is never a lamp state and never a `ReportState`. Per design-system §Surface: cli §Tokens (platform-specific) and §Surface: cli §Component Patterns 4. For C5, E0-10's expected content is the label text, never the tint.
- The `contentinfo` footer strip is a single line: seed · run state · a count per non-zero lamp. It does NOT carry the unticked-checklist count, so the operator-checklist's own `role=status` roll-up stays the only surface for that count. Per design-system §Surface: desktop-webview §Component Patterns 7. A regression fix must not move the unticked count into the strip.
- The coverage matrix must keep all of the following:
  - One row per manifest capability, with no virtualization.
  - One tab stop, with a roving current row moved by ArrowUp/ArrowDown and Home/End.
  - `aria-current` plus a 2px `--border-emphasis` left edge on the current row's P-ID cell. Every P-ID cell reserves a transparent 2px edge, and this edge is distinct from the `--color-focus` ring.
  - The `No coverage data.` empty prose.
  Per design-system §Surface: desktop-webview §Component Patterns 3. The C2 disposition of `CoverageMatrix.tsx` exports must leave this pattern intact.
- The three empty/miss strings stay distinct and must not merge under a C2 cleanup of `ScenarioPicker.tsx`: `No scenarios found.` for an empty catalog, `No scenarios match.` for a filter miss (rendered from a persistently-mounted announced region), and `No coverage data.` for an empty matrix. Per design-system §Surface: desktop-webview §Component Patterns 5.
- The C3 refusal of an ambiguous P-ID is cli error output:
  - It goes to stderr and is sanitized, with no host paths, internal struct names or stack traces.
  - It uses the labels `error: <short>` (Fail red, ANSI 203) and `hint: <fix>` (Residual mute, ANSI 246).
  - Color is TTY-gated on stderr's own `IsTerminal`, and the ASCII labels always stand.
  Per design-system §Surface: cli §Component Patterns 5 and §Surface: cli §Platform-Specific Notes.
- Focus is drawn as a `2px solid var(--color-focus)` `outline` under `:focus-visible`, with no `box-shadow` and no transition. Per design-system §Depth Strategy and §Motion (calibrated to expression level `0.3`).

## Patterns to follow
- Status is never color alone. Every lamp, prefix and caption pairs color with a text label or glyph. Per design-system §Iconography and §Anti-Patterns (NEVER do these) §Universal Bans.
- The Tertiary and Muted text tokens were moved so that they clear 4.5:1 on every surface they render on. A fix reuses these tokens rather than minting a new hex. Per design-system §Color Palette §Text Hierarchy.
- The picker's filter-miss prose reuses the list's existing empty styling, with "no new token". A regression fix follows the same reuse-first rule. Per design-system §Surface: desktop-webview §Component Patterns 5.
- Motion is functional only, at expression `0.3`: 150ms `--motion-micro` with `--ease-quiet`, no entrance animation, and every transition dropped under reduced motion. Per design-system §Motion (calibrated to expression level `0.3`).

## Anti-patterns to avoid
- No hardcoded hex, px or ms values, and no new token, in a regression fix. Per design-system §Self-Validation Protocol §4 Token Test.
- No shadows, no `backdrop-filter` and no flashing or pulsing status. Per design-system §Anti-Patterns (NEVER do these) §Per-Surface Bans and §Motion (calibrated to expression level `0.3`) (Hard limits).
- Never collapse `Blocked`, "no result yet" or the ENVIRONMENT-SUSPECT caption into a red `Fail`. Per design-system §Anti-Patterns (NEVER do these) §Rejected Defaults.

## Contract bindings
- Token contrast (Tertiary/Muted on `--color-raised-1`, `--color-raised-2` and `--color-inset`) ↔ a11y contrast (SC 1.4.3), asserted by the `a11y` job's routine axe/contrast arm. Per design-system §Color Palette §Text Hierarchy.
- The focus ring's `outlineStyle` / `outlineColor` ↔ the routine a11y arm, which asserts them through computed style. Per design-system §Depth Strategy.
- The E0-10 banner label as text ↔ a11y not-color-alone (SC 1.4.1) and the SR harness's `rows.ts` expected-content token. Per design-system §Surface: cli §Tokens (platform-specific).
- The reduced-motion block ↔ a11y SC 2.3.3. Per design-system §Motion (calibrated to expression level `0.3`).
- C2 knip dispositions in `ScenarioPicker.tsx` / `CoverageMatrix.tsx` ↔ the component patterns above. A removed export must not remove a shipped state string or behavior. Per design-system §Surface: desktop-webview §Component Patterns 3 and 5.

## Acceptance criteria contributions
- (design) Every webview file this chunk touches introduces no raw hex, px or ms value, and no new token. Each value binds a `var(--…)` from the `:root` bundle. Per design-system §Surface: desktop-webview §Tokens (platform-specific).
- (design) SR row E0-10 grades against the ENVIRONMENT-SUSPECT banner's text label, never its tint. The caption stays outside the lamp column. Per design-system §Surface: cli §Tokens (platform-specific).
- (design) The C3 ambiguous-P-ID refusal prints `error:` + `hint:` ASCII labels on stderr, with no host path and color only when stderr is a TTY. Per design-system §Surface: cli §Component Patterns 5.
- (design) After the C2 dispositions, the three strings `No scenarios found.`, `No scenarios match.` and `No coverage data.` still render at their own sites. The coverage matrix keeps its single roving tab stop. Per design-system §Surface: desktop-webview §Component Patterns 3 and 5.
