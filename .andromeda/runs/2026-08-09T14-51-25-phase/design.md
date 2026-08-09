# design extract

## Relevance
Relevant — this chunk is purely a visual/token treatment on two rendered surfaces (desktop coverage matrix + CLI `comfy-table`), which is design's core authority.

## Constraints
1. **Never color-alone, both surfaces.** The out-of-scope treatment must pair color with the existing text label (desktop) and an ASCII prefix/label that survives `NO_COLOR`, `TERM=dumb`, and piping (cli) — per design-system.md §Iconography (Rule) + §Surface: cli / Tokens (status prefixes) + §Anti-Patterns / Per-Surface Bans (cli).
2. **Reuse shipped tokens by name; mint nothing.** No new palette row, no new CSS var, no new ANSI code — the treatment must compose from the 34-token `:root` contract and the existing cli ANSI map, adapted-not-forked (CSS var ↔ ANSI code by name), per §Surface: desktop-webview / Tokens and §Surface: cli / Tokens. This is a hard precedent (see amendment history below).
3. **Do not borrow a status-tier token, and do not collide with one visually.** `--status-fail` and `--count-blocked` are banned by scope; note also that `--count-blocked` and `--text-muted` are the *same dark hex* (`#565F89`), and `--status-residual` owns the *dashed* treatment — so a recessive treatment built on `--text-muted` or on dashes would read as `Blocked`/`KnownResidual`. The six-treatment "none silently reads as another" discipline in §Color Palette (Verdict-vs-ReportState note) applies to this seventh case.
4. **No sixth lamp, no new glyph tier.** Out-of-scope is a coverage *mode*, orthogonal to the verdict axis; §Component Patterns (desktop-webview) #4 enumerates exactly six lamp treatments and scope forbids extending it. The differentiator belongs in the Mode cell + row recessiveness, not the lamp column.
5. **Borders-only depth, dense list.** Row differentiation happens via fill/seam/text tier (`--color-raised-1`, `--border-subtle`, `--space-md` padding) — never a shadow, `backdrop-filter`, tile, card, or a separate "excluded" section, per §Depth Strategy + §Component Patterns (desktop-webview) #3.
6. **Motion: none added.** Expression `0.3` allows only in-place 150ms color transitions and no entrance animation; a static classification row gets no transition at all, and the global `prefers-reduced-motion` block in `tokens.css` already covers what exists (§Motion — This project's values / Hard limits).
7. **Typographic tiers hold.** P-ID stays mono `--color-id-cyan` (Data role); the Mode cell stays IBM Plex Sans Label/Body — mono-everywhere is a Rejected Default (§Typography + §Anti-Patterns / Rejected Defaults).

## Patterns to follow
1. **§Component Patterns (desktop-webview) #3 — Coverage matrix row anatomy.** Extend the existing row (lamp + label + P-ID + tier + latency, `--space-md` padding, `--border-subtle` dividers, hover `--color-raised-1`, selected `--border-emphasis`); the Mode cell already exists in `CoverageMatrix.tsx/.css` and consumes tokens by name.
2. **§Anti-Patterns / Rejected Defaults — "Blocked is present-but-greyed, never a red error."** This is the shipped model for "a distinct non-failure state": recessive, carrying its own explanatory string, distinct token. Out-of-scope should follow the *shape* of that treatment while being visually separable from it.
3. **§Surface: cli / Component Patterns #3 — Coverage/SLO table.** comfy-table with a colored text-prefix column, dynamic terminal width, and the precedent that a non-measured row renders measurement columns as `—`/null rather than an error — the honest-null pattern the footer roll-up should mirror.
4. **§Surface: cli / Tokens (ANSI map) ↔ §Surface: desktop-webview / Tokens.** The by-name adaptation table is the contract for item 3 of the scope; pick a pair that already exists on *both* sides so the cli side needs no new entry (the stderr `error:`/`hint:` reuse is the worked example).
5. **§Color Palette (Verdict-vs-ReportState note) — orthogonality is stated in the render, not only in the model.** The rendered row must let a reader tell remit from outcome without cross-referencing (scope Intent), the same way `ManualCheck` is deliberately placed outside the green/amber/red triad.

## Anti-patterns to avoid
1. **Minting a new color for the new classification.** §Anti-Patterns / Universal Bans ("never use color purely for decoration" — every color must map to an existing meaning tier); reuse or compose, never add a palette row.
2. **Color-alone or emoji-carried signalling.** §Anti-Patterns / Per-Surface Bans (cli): never rely on color alone, never emoji in machine-parseable piped output; desktop equivalent is the label+glyph pairing rule.
3. **Breaking the dense list into a card/tile/section for the 16 rows.** §Anti-Patterns / Universal Bans (cookie-cutter card grids; same layout for different information types) + Rejected Default (KPI tile wall).

## Contract bindings
- **a11y §Contrast (SC 1.4.3):** whichever recessive token carries the out-of-scope row must clear 4.5:1 against `--color-raised-1` and `--color-base` in **both** dark and light blocks — `--text-tertiary` / `--text-muted` are the near-threshold tiers; a11y verifies, design supplies the token name.
- **a11y §Use of Color (SC 1.4.1):** the text label + glyph/ASCII-prefix pairing satisfies this; design owns the token, a11y owns the criterion.
- **a11y §Animation (SC 2.3.3):** no new transition is introduced, so the existing global reduce-motion block remains the only obligation.
- **Layouts (`layout-templates.md` §Component block 1, `:126`):** the Mode cell placement/column order is layouts' authority; design supplies only the treatment (token names + label/glyph rule) that gets appended to that entry at wrap.
- **Tests:** the negative-guarantee assertion (scope item 4) should be written against design's *token names* — absence of `--status-fail`/`--count-blocked` in the rendered desktop row and absence of ANSI 203/60 and `[FAIL]`/`[BLOCKED]` in the cli row — so the design invariant is what the test pins.

## Acceptance criteria contributions
1. **(design) Tokens only, adapted not forked.** The out-of-scope treatment uses shipped tokens by name on both surfaces (no raw hex/px/ms, no new `:root` var, no new ANSI code), and the desktop var ↔ cli ANSI choice is a documented name-pair.
2. **(design) Never reads as `Fail` or `Blocked`, and is distinguishable from `KnownResidual`.** The row carries neither `--status-fail`/`--count-blocked` (desktop) nor ANSI 203/60 or `[FAIL]`/`[BLOCKED]` (cli), and is visually separable from the `Blocked` slate-violet and the `KnownResidual` dashed treatment in **both** dark and light.
3. **(design) Not color-alone.** The mode label renders on every surface with color stripped (`NO_COLOR` / piped / light mode), ASCII-only in piped output, and no new lamp glyph was minted.
4. **(design) Flat and motionless.** No shadow/`backdrop-filter`/card wrapper introduced, no new transition or entrance animation, row stays inside the dense one-row-per-P-ID list.

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output) — the governing precedent for this chunk: a new cli surface needed a treatment, the drift detector proposed adding a *new palette row* ("Hint grey"), and that was **corrected at validation** — ANSI 246 was an existing token, so the edge reused two shipped tokens and no color was added. Apply the same discipline to the out-of-scope treatment.
- **2026-08-09-current-sut-coverage-classification** (§Brand Identity Domain anchors · §Component Patterns #3) — the immediately prior chunk in this exact area; it de-hardcoded "all 60 capabilities"/"full 60-row wall" to name the set rather than substitute the new literal. The carried nit (`CoverageMatrix.css:1` still says "all 60 capabilities") must be swept the same way — **name the set, do not write "82"**.
- **2026-08-08-sut-capability-manifest** (§Color Palette Primary · §Typography Data · cli ANSI map) — same de-hardcoding sweep for P-ID ranges in token-usage examples; relevant if the Mode-cell treatment is illustrated with example rows.
- **2026-06-15-design-token-typography-bundle** (§Surface: desktop-webview / Tokens) — any token work lands on plain `:root` in `tokens.css`, never `@theme`, and must be mirrored in the `prefers-color-scheme: light` block (the 34-token contract).
- **2026-06-26-component-primitives-library** (§Motion / §Component Patterns #2) — establishes that spec-illustration → sound-implementation reconciliation is routine and lands as a wrap-time amendment; the same flow applies to scope item 5's `layout-templates.md` extension (planned here, applied at wrap).
