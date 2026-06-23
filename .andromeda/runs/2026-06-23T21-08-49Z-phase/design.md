# design extract

## Relevance
Partial — the chunk is a CLI surface implementation of an existing core trait; design applies to the interactive terminal UX (hold prompt display + paused-count mirroring), not to core logic.

## Constraints
- Per design-system §Brand Identity: Expression level `0.3` governs motion/complexity — at CLI surface, the paused-count spinner must **STOP in place (not hide)** when a hold fires; the absence of motion is the signature (design-system §Signature element, §Motion hard limits).
- Per design-system §Color Palette §Semantic Colors: `[HOLD]` state renders with hold-amber `#E3B341` (ANSI 179) when TTY; status must be **never color-alone** — always paired with `[HOLD]` ASCII prefix (design-system §Anti-Patterns Universal Bans, a11y binding).
- Per design-system §Surface: cli §Component Patterns §1: The paused-count hold-point mirrors the Tauri titlebar signature — spinner stops at the exact hold value, phase-line prints above the `inquire` prompt with amber `HOLD — operator pause` prefix (TTY-gated color overlay, never sole encoder) (design-system §Component Patterns §1, §Motion).
- Per design-system §Surface: cli §Platform-Specific Notes: `isatty()` gating is mandatory — interactive `inquire` prompts NEVER block the headless agent-driven path; piped/non-TTY runs must auto-resolve via `HeadlessResolver::proceed()` and never hang on a prompt (design-system §CLI Navigation, "headless never blocks" discipline).
- Per design-system §Surface: cli §Platform-Specific Notes: ANSI stripping on piped stdout, honoring `NO_COLOR` and `TERM=dumb`; emoji never in machine-parseable output — ASCII prefixes only (design-system §cli Tokens, §Anti-Patterns §cli).
- Per design-system §Motion: All transitions MUST drop under `@media (prefers-reduced-motion: reduce)` — the scope's note on `tracing` event emission confirms motion-clock observation exists; respect the accessibility override (design-system §Motion hard limits, binding with a11y SC 2.3.3).

## Patterns to follow
- Paused-count spinner **stop-not-hide** (design-system §Component Patterns §1, §Brand Identity §Signature element): when hold fires, `indicatif` spinner halts at the exact count value; this embodies "paused count shows where it stopped" — the frozen heartbeat is the CLI mirror of the Tauri titlebar hold signature.
- `[HOLD]` phase-line prefix + amber overlay (design-system §Surface: cli §Component Patterns §1, §Tokens ANSI mapping): colored amber ANSI 179 for TTY, stripped for non-TTY/pipes; never the sole encoder; paired with text label (a11y rule).
- `inquire` confirm prompt + scenario/p_id/step context (design-system §Surface: cli §Component Patterns §2): render the hold's human-readable action prompt + context, keyboard-first (inquire default), mapping go/no-go to `Decision::Go` / `Decision::NoGo`; `allow_no_go == false` offers proceed-only acknowledgment.
- Headless auto-resolve path (design-system §Anti-Patterns §cli, "headless never blocks"): non-TTY runs delegate to `HeadlessResolver::proceed()` — no prompt written to pipes, preserving source-of-truth agent discipline.

## Anti-patterns to avoid
- NEVER use interactive prompts without `isatty` check — the headless path (agent/piped) must auto-resolve and never hang (design-system §Anti-Patterns §cli).
- NEVER hide the spinner on the operator-pause or animate-to-100% — it must STOP in place to be the signature (design-system §Anti-Patterns Universal Bans, §Motion hard limits).
- NEVER use color as the sole status encoder for `[HOLD]` — always pair with `[HOLD]` ASCII prefix + label; honor `NO_COLOR` and pipe context (design-system §Anti-Patterns Universal Bans, §cli).

## Contract bindings
- **a11y ↔ design-system §Motion**: `prefers-reduced-motion: reduce` must suppress the spinner halt/tint (any animation — CLI has no visual animation, but the tracing event clock must respect the override per binding).
- **a11y ↔ design-system §Color Palette**: Status is never color-alone — `[HOLD]` ASCII prefix + label is always present; color is optional TTY-gated overlay; `NO_COLOR` environment variable must be honored.
- **core ↔ CLI seam**: The interactive resolver is a sibling of Epoch-9 Tauri AlertDialog — same `PauseResolver` trait + `Decision` vocabulary, different medium; zero new core types (design-system §Component Patterns §2, scope §Boundaries).

## Acceptance criteria contributions
- **(design / interactive UX)** The hold phase-line shows `[HOLD]` ASCII prefix at all times (color stripped for non-TTY / `NO_COLOR`); paused-count spinner stops in place, never hides or animates to 100%.
- **(design / headless discipline)** The non-TTY path (piped/agent) auto-resolves without blocking; no interactive prompt is written to a pipe or to stderr when color is unavailable.
- **(design / a11y)** The hold state remains unambiguous under `NO_COLOR`, `TERM=dumb`, and piped output; spinner halt respects `prefers-reduced-motion: reduce` if tracing observes it.
- **(design / status lamp)** The paused-count mirror uses the `[HOLD]` phase-line + amber ANSI 179 (TTY) mapping from design-system §Tokens ANSI; verdict lamp precedence (ch3's `Lamp`→line mapping) is preserved, never overwritten.

## Relevant amendment history
**(none)** — design-system-amendments.md contains only the 2026-06-15 token CSS structure change (`:root` vs `@theme`), which does not touch CLI motion / hold UI / ANSI token mapping. This chunk's hold-point / paused-spinner / `inquire` confirm patterns are defined in the current design-system.md body and have no prior amendment entry.
