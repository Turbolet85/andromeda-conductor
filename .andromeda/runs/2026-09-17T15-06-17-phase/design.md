# design extract

## Relevance
Partial — the chunk authors a requirement/ownership statement (plus a one-clause text repair), renders nothing, so token/typography/motion mandates are dormant; design still owns the *subjects* the attributed claims name (the operator-pause `AlertDialog`, the keyboard-first navigation stance, the focus-ring token).

## Constraints
- design-system §Navigation Pattern (desktop-webview) requires keyboard-first run control with first-class keybindings (start/stop/proceed/abort) exposed via Radix primitives — this is the design-side origin of the unattributed "per-surface keyboard shortcuts" claim (scope `:370`). Whether those shortcuts are implemented and asserted anywhere is research's question.
- design-system §Navigation Pattern requires a single-station console with no router, no breakpoints, no browser-style back/forward/URL nav — "the frameless window IS the surface". The scope's two *N/A by construction* rulings (skip links `:359`, focus restoration with no route-change surface `:367`) rest on that mandate, so the ownership statement should source them to it rather than assert the absence unattributed.
- design-system §Component Patterns 2 requires the operator-pause go/no-go dialog be `role="alertdialog"`, focus-trapped, with a visible `--color-focus` ring, gating every committed timeline step — it is the named subject of every hold-dependent trap/restoration claim (`:362-363`, `:366`). Whether the shipped dialog already traps and restores focus to the triggering control is research's question.
- design-system §Color Palette → Border Progression (Focus row) + §Depth Strategy require the focus ring be `--color-focus` (`#7DCFFF` dark / `#0969DA` light) rendered as a single `0 0 0 2px var(--color-focus)` ring — the only `box-shadow` the system permits. Any focus-visibility wording the statement adds must bind by token name.
- design-system §Component Patterns 7 requires Space to toggle an operator-checklist row (keyboard-first); per the 2026-09-02 amendment the primitive's only shipped mount is inside the operator-pause dialog — so that keyboard affordance sits behind the same live-Pulse HOLD gate the carve-out must cover, not behind the routine arm.
- design-system §Surface: cli → Navigation Pattern requires linear top-to-bottom stdout with no cursor manipulation and no full-screen redraw (ratatui deliberately omitted), and §cli Component Patterns 2 requires `inquire` prompts be `isatty`-gated and never block the headless path — a claim worded "per-surface" must not imply a cli focus-order owner exists.

## Patterns to follow
- The operator-pause dialog primitive (shadcn/ui `AlertDialog` over Radix) as the single named surface for hold-dependent trap + restoration claims, per design-system §Component Patterns 2 — attribute the claim to that primitive, not to a generic "modal".
- Radix-primitive-carried keyboard behavior in the picker and start/stop controls (shadcn `Command`/`Select`, keyboard-first buttons), per design-system §Component Patterns 5 — the webview's shortcut claims land on primitives, not bespoke handlers.
- Name-the-set-over-bake-a-literal when writing into a master body, per design-system-amendments §2026-08-09-current-sut-coverage-classification and §2026-09-10-release-build-and-bundle — the CARRY 1 stale-offset history in this very scope is the demonstration of what a baked coordinate costs.
- Record shipped state distinctly from designed intent when a spec sentence asserts behavior, per design-system-amendments §2026-09-02-screen-reader-manual-spec (the checklist's report-site mount is "designed, not built"); an ownership row for a claim asserted nowhere should say so rather than read as attributed.

## Anti-patterns to avoid
- design-system §Anti-Patterns → Per-Surface Bans (desktop-webview) bans hover-only interactions without keyboard alternatives — an ownership statement must not record a claim as owned in a way that implies a pointer-only path is acceptable.
- Same section bans `alert()` / `confirm()` / `prompt()` — the HOLD surface whose trap is being attributed is the styled Radix `AlertDialog`; naming a native dialog as the claim's subject would contradict the plan.
- design-system §Anti-Patterns → Universal Bans ("NEVER converge on common safe choices") plus the de-literalization precedents: do not introduce a fresh per-claim count or file-offset literal into the master body where naming the owning suite suffices.

## Contract bindings
- design §Border Progression (Focus) + §Depth Strategy focus-ring token ↔ a11y §Focus Visible (SC 2.4.7/2.4.11): design owns the ring's token and the box-shadow exemption, a11y owns the criterion the attributed claims are stated against.
- design §Component Patterns 2 (focus-trapped `role="alertdialog"`) ↔ a11y-plan §5 *Keyboard Navigation* `:362-363`/`:366` (SC 2.1.2, SC 2.4.3): design mandates the trap/restore behavior; this chunk assigns its test owner.
- design §Navigation Pattern (single-station, no router) ↔ a11y-plan `:359`/`:367` N/A-by-construction rulings, and ↔ layouts (layouts owns per-screen focus ORDER for `run-console-idle` / `-live` / `-HOLD` / `idle-with-report`; design owns only the keyboard-first stance and the ring).
- design §Motion `prefers-reduced-motion` ↔ a11y SC 2.3.3: not engaged — this chunk changes no animation.

## Acceptance criteria contributions
- (design) Ownership rows for the hold-dependent trap and restoration claims name the operator-pause `AlertDialog` primitive as the claim's subject, consistent with the focus-trapped `role="alertdialog"` + visible-ring mandate (per design-system §Component Patterns 2).
- (design) The two *N/A by construction* rulings (skip links; focus restoration with no route change) cite the single-station, no-router console as their basis rather than standing unsourced (per design-system §Navigation Pattern).
- (design) Any focus-visibility wording added to the requirement binds the ring by token name (`var(--color-focus)`), never a hex or a raw pixel value, and introduces no second `box-shadow` use (per design-system §Depth Strategy + §Color Palette Border Progression).
- (design) The statement names owning suites by name/as a set and adds no new line-offset or claim-count literal to the master body that would re-stale on the next edit (per design-system-amendments §2026-09-10-release-build-and-bundle de-literalization rule).

## Relevant amendment history
- **2026-09-02-screen-reader-manual-spec** — recorded that the operator-checklist primitive's only shipped mount is inside the operator-pause dialog (report site designed, not built — a route-owned gap) and retired baked strings/mounts the code does not produce. Directly adjacent: the keyboard affordance for that primitive is reachable only behind the HOLD gate this chunk must carve out, and the amendment is the precedent for stating shipped-vs-designed honestly in a master.
- **2026-09-01-desktop-a11y-sweep** — moved `--text-tertiary` / `--text-muted` values after an axe `color-contrast` (SC 1.4.3) failure; the nearest prior a11y↔design amendment. Establishes the pattern that a11y measurement drives design-token change, but it is contrast, not keyboard/focus — no token here needs to move for this chunk.
- **2026-09-04-sr-findings-remediation** — recorded the picker's filter-miss announced region and corrected the coverage-matrix empty prose; touches the same keyboard-first picker surface (§Component Patterns 5) whose shortcut claim is in this chunk's attribution population.
- **2026-08-09-current-sut-coverage-classification / 2026-09-10-release-build-and-bundle** — the de-literalization precedents (name the set, never substitute a fresh literal); cited above as the drafting rule for the ownership statement, given CARRY 1's stale-offset history.
