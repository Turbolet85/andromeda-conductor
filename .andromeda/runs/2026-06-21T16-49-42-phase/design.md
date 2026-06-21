# design extract

## Relevance
Partial — the chunk builds core orchestration, not UI surfaces; design applies only to the hold-point value representation and the two thin-shell contract points (CLI + Tauri resolvers deferred to later epochs).

## Constraints
- design-system.md §Brand Identity / Signature element: The paused-count hold-point is Conductor's signature — hold-awaiting MUST freeze the count display in place at the exact hold value (no hiding, no animation-to-100%); CLI: `indicatif` spinner stops-not-hides; desktop: titlebar count tints green→amber over 150ms as phase line flips to "HOLD — operator pause". The **orchestration this chunk defines provides the core signal** that shells later consume — the hold must carry wall-clock-aware timing information so shells can correctly render the frozen moment.
- design-system.md §Motion: All motion is functional only, expression level 0.3. The hold's entry signal is a 150ms CSS color transition (green→amber) paired with a phase-line text swap; no spring physics, no staggered entrance, no hide-then-show. The hold duration itself is *wall-clock time outside the seeded virtual clock* (determinism preserved) — the pause gap is real-time, not simulated.
- design-system.md §Color Palette / Semantic Colors: Hold-point state (operator-pause HOLD + CalibrationRegion verdict) renders as `--count-hold` amber (`#E3B341` dark / `#9A6700` light). Hold-related messages must pair color with text label (`[HOLD]` prefix, never color-only). CLI ANSI mapping: ANSI 179 (hold amber).
- design-system.md §Typography / Data tier: Any hold-related artifact labels (scenario name, P-ID, hold reason) that are serialized or transmitted MUST use the `JetBrains Mono` `--color-id-cyan` status tier (`#7DCFFF` dark / `#0969DA` light) in desktop contexts; CLI: ANSI 117 cyan for P-IDs, ANSI 179 for `[HOLD]` prefix.
- design-system.md §Anti-Patterns / Rejected Defaults: NEVER hide or animate-to-100% the progress spinner / heartbeat during operator-pause (this is the signature — "paused count shows where it stopped"); NEVER use flashing/pulsing visual; NEVER converge on generic "pause" iconography (the signature is the *frozen* count, not a pause-button icon).
- design-system.md §Component Patterns (desktop-webview #2 Operator-pause go/no-go dialog, #1 Paused-count titlebar): The hold-point orchestration provides the **state transition** that these components consume. Desktop shells must receive from this core: (a) hold-entered signal + hold metadata (prompt, verdict tier, P-ID), (b) go/no-go result from the operator, (c) exact timing of the hold so the frozen count is synchronized to the wall-clock moment the hold was triggered.
- design-system.md §Surface: cli / Component Patterns #1 & #2: CLI shells receive the hold signal and must render `indicatif` spinner-stop + bold ANSI 179 `[HOLD — operator pause]` phase line above an `inquire` prompt. This chunk provides the core **hold value + resolver abstraction** that CLI's `inquire` integration later consumes (deferred to Epoch 8).

## Patterns to follow
- Hold-point model uses the same serializable-value discipline as config (design-system.md: "no host paths or internal struct names leaking through the redaction edge") — any artifact-round-tripped hold carries semantic labels (scenario name, P-ID, human-readable prompt), never Rust `Debug` output or internal debug strings.
- Go/no-go resolution follows the "no-go is an operator decision, not a harness fault" pattern — `no-go` is a typed value (`Decision::NoGo`), never a `Result::Err` or panic. The report and shells treat both outcomes deterministically.
- Headless resolver mirrors the "headless never blocks" CLI discipline (design-system.md §Surface: cli / Platform-Specific Notes) — in agent-driven runs (non-interactive TTY), holds auto-resolve to their configured default outcome rather than waiting; the auto-resolution is recorded so the outcome is reproducible under the same seed.
- Hold timing is wall-clock only, outside the seeded virtual clock (design-system.md §Determinism preservation) — same scenario + seed + headless resolver ⇒ identical emission-stream shape (no seeded stream shape perturbation).

## Anti-patterns to avoid
- NEVER serialize a hold with Rust `Debug` output or internal struct names — violates the "no redaction-edge leaks" rule (design-system.md §Anti-Patterns Per-Surface Bans / desktop-webview: "NEVER ship Tauri commands without sanitization").
- NEVER make a hold outcome a `Result::Err` — a `no-go` decision is not a harness fault; always return a typed `Decision` value.
- NEVER hardcode a hold's auto-resolution outcome in the headless resolver — the resolution must be configurable per scenario so tests / CI can verify both go and no-go paths deterministically.

## Contract bindings
- **shells ↔ core orchestration:** The CLI resolver (Epoch 8) and Tauri resolver (Epoch 9) are both consumers of the `Resolver` trait abstraction this chunk defines in `conductor-core`, ensuring both shells drive the same core logic (star-topology: seam crates import only core, not each other).
- **verification (conductor-verify) ↔ hold-point:** The `ManualCheck` operator-checklist pattern (design-system.md §Component Patterns #7) is a hold-awaiting-operator-confirmation. The verification seam must integrate hold outcomes so the checklist rows resolve to their `Pass`/`Fail` verdict once an operator confirms.
- **timeline (conductor-timeline) ↔ hold resumption:** The timeline phase scheduler (which owns `tokio::time` scheduling) must pause and resume from hold points; the choice of await mechanism (trait object vs channel vs callback) is deferred but the contract is: pause at the hold → consume the resolver → await the decision → resume from the frozen state deterministically.
- **design/motion tokens ↔ shell rendering:** The hold entry signal must carry timing metadata so desktop/CLI shells can synchronize the visual state-change (titlebar count freeze + tint, spinner stop) to the exact wall-clock moment the hold was triggered, honoring the 150ms motion budget.

## Acceptance criteria contributions
- (design) Hold-point value carries only semantic labels (scenario name, P-ID, human-readable prompt) — no Rust `Debug` output, no internal struct names, no host paths. Redaction-edge clean per design-system.md.
- (design) Hold UI representation uses `--count-hold` amber (`#E3B341` dark / `#9A6700` light) paired with text label (`[HOLD]` prefix, `--status-manual` lavender for `ManualCheck` holds awaiting operator); never color-only signal (design-system.md §Color Palette / Semantic Colors + §Anti-Patterns).
- (design) Same scenario + seed + headless auto-resolving holder ⇒ identical emission-stream shape as a run with no holds; hold-timing is wall-clock-only, outside the seeded virtual clock (determinism preserved per scope.md Acceptance anchor).
- (design) In headless/non-interactive mode, a hold never blocks — it auto-resolves to its configured outcome and is recorded for reproducibility; interactive shells (CLI `inquire`, Tauri dialog) consume the same core `Resolver` trait, ensuring both paths drive identical core logic.

## Relevant amendment history
(none) — The design-system.md is fresh (2026-06-14 generated by /andromeda-design Phase 4); the one amendment (2026-06-15 §Tokens) concerns Tailwind v4 `:root` CSS declaration mechanics, not hold-point or operator-pause orchestration. The hold-point signature and motion tokens were finalized in the original system and are stable.