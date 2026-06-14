## Output Protocol

Rules for this iteration:

1. **Output patches, not the document.** Emit a list of patches (old → new) plus a changelog. NEVER reproduce the full document or whole sections.
2. **Patch format.** Each patch is:
   - `### Patch N: <short description>`
   - `**Old:**` followed by the exact verbatim text currently in the draft (enough to locate it unambiguously, no more).
   - `**New:**` followed by the replacement text.
3. **Changelog.** After the patches, list one line per patch: `[Iteration N] [substantive|cosmetic] <description>`. Tag a change `substantive` only if it alters meaning, anchoring, tokens, or a downstream contract; tag wording/formatting-only fixes `cosmetic`.
4. **Patch budget (priority-gated).** Spend patches on `[priority: high]` dimensions first. Patch a `[priority: medium]` dimension only if budget remains after every high-priority finding is addressed, and only if the issue is real (anchored, not stylistic). Never burn the budget on cosmetic rewrites of already-correct content — a `cosmetic` patch is justified only when it removes a genuine ambiguity a downstream skill would otherwise have to guess through.
5. **Stay in lane.** Introduce only design-tokens / patterns / anti-patterns changes. PROHIBITED: full-document reproduction; restructuring sections without a stated reason; relabeling cosmetic edits as substantive; proposing backend / database / frontend-framework / CSS-tooling / component-library / mobile-framework changes (Phase 1 + architecture settled tooling); writing content owned by tests / observability / accessibility / security specialists (see the Analysis Protocol out-of-scope step before patching anything that touches their territory).
6. **If no issues found for a dimension:** that is a valid outcome — do not invent one. If NO dimension yields a real patch, output the literal line `No patches` and a single cosmetic changelog entry stating the plan passed this dimension review.

## Analysis Protocol

Do not scan-and-patch. Reason in this order, then patch:

1. **Read** the full draft once, top to bottom, noting nothing. Build a mental model of the whole instrument before judging any part.
2. **Cross-reference** sections against each other — most real defects in this document are inconsistencies between two places that are individually plausible:
   - Brand Identity personality ("calm and resolute under load … no alarm") ↔ Color Palette / Typography / Motion / Iconography rationales — does the personality actually echo in each, or was a rationale written in isolation?
   - Brand Identity expression `0.3` ↔ Motion Duration scale row + Hard-limits block ↔ every Surface Component Pattern — does any pattern smuggle in motion above the declared ceiling?
   - Signature (Paused-count hold-point) ↔ its three required realizations: Motion High-impact moments, the desktop-webview Frameless-titlebar pattern, the cli `indicatif` pattern. Abstract mention ≠ concrete freeze behavior.
   - **Color Palette / Surface Scale / Text Hierarchy / Border Progression tables ↔ the desktop-webview `@theme` block** — every token named in a table must exist in `@theme` (and in BOTH the dark default AND the `prefers-color-scheme: light` override). Every token referenced in a Component Pattern (e.g. `radius-full`, `--border-emphasis`) must be defined. This table-to-implementation fidelity check is the highest-yield pass for this document — run it explicitly.
   - desktop-webview status hexes ↔ cli ANSI 256 mappings ↔ the Decisions Log status list — the four-state triad must agree across all three.
   - Anti-Patterns subsections ↔ the per-section rationales that reference them (Color / Motion / Depth / Iconography) — bans should be cross-referenced, not silently re-stated and drifting.
3. **Check each dimension** below with its anchor example in mind. The anchor shows the SHAPE of a real issue here; find that shape (the same defect or its sibling), not a paraphrase of the anchor.
4. **Out-of-scope discipline (specialist boundaries).** Before patching anything near another specialist's territory, STOP and apply this test. Design defines visual / motion / surface scope; other specialists own their cases. Do NOT patch — and do NOT *add* — content that is:
   - **tests' domain** — concrete test cases (assertions written in words / test-runner call syntax). Design names the visual / motion / surface scope; tests writes the cases.
   - **observability's domain** — observability instrumentation schemas (spans / metrics / traces). Design names the visual error / loading / empty / Blocked states; observability instruments them. Do not name a concrete observability or error-reporting platform.
   - **accessibility's domain** — ARIA attributes / accessibility attribute names, or WCAG conformance claims / specific contrast-ratio numbers. Design provides the *inputs* a11y consumes — Text-Hierarchy hex values, the focus-ring color, the motion budget, the icon-label policy — and a11y derives the attributes and conformance itself. (This draft already leaks a few literal accessibility attribute names and an accessibility-success-criterion citation into design rationales; treat the presence of the *visual input* as correct and in-lane, and only flag a leak if the design rationale tries to OWN the attribute or the conformance number rather than expose the visual requirement behind it.)
   - **security's domain** — threat-model / tier / compliance content. The settled Tauri guardrails may be *referenced* as constraints; do not author new security policy.
   For any such finding: do not patch the foreign content in. Instead verify the document exposes the boundary requirement — the *what-must-hold* (the visual state, the contrast input, the motion budget), not the *how-it's-wired*. Patch only if that boundary requirement is itself unstated.
5. **Prioritize** by blast radius before emitting: (a) defects that would stop a downstream skill from deriving its plan or would ship a broken artifact (e.g. a light-mode token that resolves to a dark value, an undefined token a pattern references) outrank (b) defects that would produce a generic / off-brand implementation, which outrank (c) cosmetic rationale drift. Patch in that order; let the patch budget fall on (a) first.

## Analysis Dimensions

### 1. Token-Table ↔ Implementation Fidelity [priority: high]

Every token named in the Color Palette / Surface Scale / Text Hierarchy / Border Progression / Spacing / Border-Radius tables must be defined in the desktop-webview `@theme` block, in BOTH the dark default AND the `@media (prefers-color-scheme: light)` override; and every token a Component Pattern references must exist. Verify the light override is complete (a token present in the dark `@theme` but absent from the light override silently keeps its dark value under light scheme — a broken light mode). Verify no Component Pattern references a custom property that `@theme` never declares.

**Anchor example:** Surface: desktop-webview → Tokens `@theme` light override block

> "  @theme {
>     --color-base: #F4F5F8;          --color-raised-1: #FFFFFF;
>     --color-inset: #E9EBF2;
>     --count-nominal: #1A7F37;       --count-hold: #9A6700;
>     --status-fail: #CF222E;         --count-blocked: #6E7491;
>     --color-id-cyan: #0969DA;       --color-focus: #0969DA;
>     --text-primary: #1A1B26;        --text-secondary: #343B58;
>     --text-tertiary: #565F89;       --text-muted: #6E7491;
>     --border-subtle: #D8DBE6;       --border-standard: #C4C8D8;
>   }"

**Issue:** The dark `@theme` declares `--color-raised-2`, `--color-raised-3`, and `--border-emphasis`, but this light override omits all three — searched the light override for `raised-2`, `raised-3`, and `border-emphasis`: none present. The Surface Scale table DOES list light variants for them (`Raised-2 #FBFBFD`, `Raised-3 #FFFFFF`) and Border Progression lists `Emphasis … #9AA0BE`, so the values exist but were never wired into the light block. Under `prefers-color-scheme: light`, `--color-raised-2`/`--color-raised-3` resolve to their dark fills (`#24273A`/`#2A2D42`) and `--border-emphasis` to `#565F89` — popovers, the operator-pause dialog, and the selected coverage-matrix row render dark-on-light. Separately, Component Pattern 4 (Verdict lamp) references `radius-full` but `@theme` defines only `--radius-sm/md/lg` — searched `@theme` for `radius-full`: absent.

**Why this matters:** `setup-project` scaffolds the `@theme` token system verbatim from this block; `route` vendors it as the `design-tokens-bundle-init`. A token that resolves to the wrong scheme's value, or a pattern that references an undefined custom property, ships a visibly broken light mode and an unstyled lamp — and no downstream test or observability hook can police a token the design plan never declared. This is the cheapest defect to fix here and the most expensive to discover at implementation time.

**Adversarial:** If `--border-emphasis` silently keeps its dark value under light scheme, the selected coverage-matrix row's left edge becomes a mid-slate line on a near-white panel — does the Squint Test in the Self-Validation Protocol catch a token that is *present and plausible in the dark block* but *missing from the light block*, or does a reviewer who only reads the dark `@theme` and the prose tables conclude the system is complete? Which downstream skill is the first to fail, and does it fail loudly (build error) or silently (wrong color)?

### 2. Downstream Readiness (per-consumer) [priority: high]

Walk each downstream consumer separately and confirm it can derive its plan from THIS document alone, without the document crossing into the consumer's authoring lane: `route` (the Fontsource WOFF2 packages to vendor, the Tailwind v4.1 `@theme` system to scaffold, shadcn/ui as settled component lib); `setup-project` (design-tokens / no-banned-fonts / expression-budget rules materializable from Anti-Patterns + Self-Validation + Decisions Log); `tests` (visual-regression targets per Surface, motion budget at `0.3`, banned-font lint target — SCOPE only, never the cases); `observability` (the visual error / loading / empty / Blocked states per Surface — never instrumentation schemas); `accessibility` (Text-Hierarchy hex values as contrast inputs, focus-ring color, motion budget, icon-label policy — never the attributes themselves). Flag any place a consumer would have to GUESS, and any place the document over-specifies into a consumer's lane.

**Anchor example:** Surface: cli → Component Patterns → pattern 2 (Operator-pause prompt)

> "**The headless path is NEVER blocked on an interactive prompt** — when stdin is not a TTY (agent-driven `agent-run.sh`), the prompt is skipped per the configured non-interactive policy and the decision is recorded to the artifact; an interactive operator gets the colored confirm."

**Issue:** This is the correct shape for downstream readiness — it states the *visual / behavioral* boundary (skip-not-block, record the decision) without authoring the test case that proves it or the observability span that records it. Use it as the calibration reference: a downstream-ready statement names what-must-hold and stops. Flag the inverse — any Surface or States note that either (a) leaves a state undefined so `observability` must invent a design token to render it, or (b) reaches past the visual boundary into a consumer's authoring lane.

**Why this matters:** If a consumer cannot derive its plan, it either stalls or guesses — and a guessed design decision re-introduces exactly the AI-default convergence the Anti-Patterns section exists to prevent. If the document over-specifies into a consumer's lane (e.g. authoring an accessibility attribute as a design rule), two plans now own the same decision and they will drift.

**Adversarial:** If `observability` reads "**Empty:** 'No scenarios loaded' prose, not a gray placeholder" for the coverage matrix but finds no corresponding empty/loading state for the run-report view or the cli `comfy-table` summary, does it invent a placeholder token (out of lane) or ship an undefined empty state? Which is worse for brand coherence — and does the document give it enough to avoid both?

### 3. Brand Identity & Signature Coherence [priority: high]

The signature (Paused-count hold-point — the frozen heartbeat at the operator-pause) must appear as concrete, specified freeze behavior in at least three distinct places: Motion High-impact moments, the desktop-webview Frameless-titlebar pattern, and the cli `indicatif` pattern. An abstract mention ("the signature is the frozen count") does not count — each must specify the *frozen-value* behavior (halts at the exact hold value, tints green→amber, resumes on proceed / dims slate-violet on abort; cli stops-not-hides). Confirm the personality ("calm and resolute under load … no alarm") echoes in the Color, Typography, Motion, AND Iconography rationales, and that the Domain anchors stay named concretely (never softened to "modern / clean / professional / user-friendly").

**Anchor example:** Brand Identity → Signature element

> "when an operator-pause go/no-go prompt fires before a committed timeline step, the count does NOT blank and does NOT keep running — it **freezes at the exact hold value**, the phase line beside it flips to "HOLD — operator pause", and the count tints green → hold-amber for the duration of the hold."

**Issue:** This is the canonical specification of the signature. Verify the three realizations match it verbatim in intent — desktop-webview pattern 1 ("count interval halts, value frozen, color → `--count-hold` over 150ms"), Motion High-impact moments ("the count's tick interval HALTS in place"), and cli pattern 1 ("spinner **STOPS in place (not hides)** at the exact value"). Flag any realization that weakens the freeze into a hide, a blank, an animate-to-100%, or that drops the green→amber tint or the slate-violet abort.

**Why this matters:** The signature is the one thing that makes a Conductor screen recognizably Conductor and not a generic dev tool; the Signature Test in the Self-Validation Protocol requires it in 3+ places. If any realization weakens the freeze semantics, `/implement` produces an inconsistent signature on that surface — and the absence-of-motion concept is fragile precisely because "do nothing" is the easiest thing for an implementer to get wrong (they add a spinner, a fade, a progress fill).

**Adversarial:** The signature is "choreographed by the absence of motion." If an implementer reads only the desktop-webview pattern and not the Motion Hard-limits ban on "progress bar that animates-to-100% or hides," will they reach for a determinate progress bar at the operator-pause (the universal reflex) — and does any single section, read in isolation, fail to forbid it loudly enough to stop them?

### 4. Library-Shortlist Faithfulness [priority: high]

The Color Palette, Typography pairing, and Design Direction must each trace to the exploration's shortlist (the "k9s cluster watch on a Warp terminal" Color World; pairing #1 JetBrains Mono + IBM Plex Sans self-hosted via Fontsource; the "Precision & Density" / instrument-panel register). Confirm no banned primary face (Inter / Roboto / Arial / Helvetica / system-ui) has re-entered as anything other than a fallback-stack tail, and that the developer-tool industry rules (data-density, reserved mono status tier, keyboard-first run control) are reflected as concrete Anti-Pattern entries, not stated once and dropped.

**Anchor example:** Typography → Rationale

> "**IBM Plex Sans** carries the phase line and all prose — the sanctioned humanist-grotesque replacement for Linear's banned Inter (similar metrics, self-hostable). … Inter / Roboto / Arial / Helvetica / system-ui are banned as primary faces."

**Issue:** This is faithful — IBM Plex Sans is named as the replacement for the banned Inter, with no banned face as a primary. Use it as the reference and verify the fallback stacks do not promote a banned face: the Loading note gives `"IBM Plex Sans", ui-sans-serif, sans-serif` and `"JetBrains Mono", ui-monospace, monospace` — confirm `system-ui` / `Inter` / `Arial` appear nowhere as a fallback tail (searched the Typography section and the `@theme` `--font-sans` / `--font-mono` for `system-ui`, `Inter`, `Arial`: none present). Do NOT claim the Q3/Q5 overrides are unrecorded — the Decisions Log records the Q3 palette override and Q5 signature override explicitly; flag only a NEW non-default choice that the Log omits.

**Why this matters:** The shortlist is the pre-validated foundation; a banned font slipping into a fallback tail means `/implement` can legitimately render the generic face when the primary fails to load, reintroducing AI-slop sameness. `tests` lints for banned fonts using the names the design plan enumerates — a banned font that is never named cannot be policed.

**Adversarial:** If a fallback stack quietly ended `…, system-ui` (a banned primary face), is that a violation or an acceptable last-resort tail — and does the Universal Bans line "generic font families: Inter, Roboto, Arial, Helvetica, system-ui default" draw the primary-vs-fallback line sharply enough that `tests` and `/implement` reach the same verdict, or could each interpret it oppositely?

### 5. Multi-Surface Consistency [trigger: detected surfaces > 1] [priority: high]

Both the desktop-webview and cli surfaces must derive from the same Color Palette / Typography status-tier / Motion tokens rather than drifting into two mini-design-systems. Verify the four-state status triad maps consistently across the dark hexes and the cli ANSI codes, and that the mono ID-cyan tier is the same concept on both. Confirm each surface manifests the signature through a surface-appropriate channel (CSS color freeze on desktop-webview; `indicatif` stop-not-hide on cli) without either weakening the freeze. Confirm "Platform-Specific Notes" are used as adaptation, not as an escape hatch that breaks cohesion.

**Anchor example:** Surface: cli → Tokens (ANSI mapping block)

> "Nominal green  #7EE787  → ANSI 114      (Pass / on-timeline heartbeat)
> Hold amber     #E3B341  → ANSI 179      (HOLD / CalibrationRegion)
> Fail red       #F85149  → ANSI 203      (Fail — no blink)
> Blocked violet #565F89  → ANSI 60       (Blocked — never measured)
> Mono ID cyan   #7DCFFF  → ANSI 117      (P-001..P-060 / run_id / SLO timings / fingerprints)"

**Issue:** Cross-check each hex against the desktop-webview `@theme` and the Color Palette tables: `--count-nominal #7EE787`, `--count-hold #E3B341`, `--status-fail #F85149`, `--count-blocked #565F89`, `--color-id-cyan #7DCFFF` — all five match 1:1, so the triad is consistent today. Use this as the reference and flag any future drift where a surface renames or re-values a status color, OR where the same hex is reused for two different meanings such that one surface's mapping becomes ambiguous (note `#565F89` already serves Blocked AND `--text-muted` / disabled in the palette — confirm the cli never colors disabled text ANSI 60 in a way that reads as Blocked).

**Why this matters:** An operator who runs the cli and the desktop console in the same session must read "one Conductor." If the status semantics diverge (a different green, a Blocked that reads as muted), the brand fractures into "two tools" and the Squint/Sameness tests fail across surfaces — re-skin work later and erosion of the single-instrument identity.

**Adversarial:** `#565F89` is simultaneously Blocked status (ANSI 60) and `--text-muted` / disabled-label color. If the cli ever dims a disabled row to ANSI 60, an operator cannot tell "disabled" from "Blocked — never measured" — does any section forbid that collision, or does the shared hex make it inevitable that one surface conflates two states the Color Palette spent a whole note distinguishing?

### 6. CLI Surface Completeness [trigger: detected surfaces include cli OR tui] [priority: medium]

The cli Tokens block must map the Color World hexes to ANSI 256 codes (not just hex) and note degradation (`NO_COLOR` / `TERM=dumb` / pipe-stripping via `anstream`). The output structure must be fully and *self-consistently* documented — ASCII status prefixes, glyph prefixes (TTY-only), header formatting, per-line density, stdout-data-vs-stderr-message separation, and any stated column/field counts must match their enumerations. The signature must use a terminal-native channel (the `indicatif` stop-in-place), not a skeuomorphic shadow/rounded-corner attempt. Spacing/Border-Radius must defer to terminal-controlled fixed width (`comfy-table` + `indicatif`, width detected dynamically).

**Anchor example:** Surface: cli → Component Patterns → pattern 3 (Coverage-matrix / SLO table)

> "`comfy-table` 7 columns: P-ID (ANSI 117 cyan) · scenario · `state` (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` colored prefix) · `slo_tier` (`<5s`/`<20s`/`<90s`) · `latency_ms` (cyan, right-aligned) · fingerprints."

**Issue:** The text says "7 columns" but enumerates exactly six: P-ID, scenario, `state`, `slo_tier`, `latency_ms`, fingerprints. Either the count is wrong (should read "6 columns") or a seventh column is missing from the enumeration (the desktop-webview coverage-matrix row carries a `label` between status and P-ID — "status-lamp glyph … + label + P-ID" — which would make the intended count 7 and the cli list is the one missing the `scenario`-vs-`label` distinction). Resolve to one self-consistent count.

**Why this matters:** `comfy-table` is constructed column-by-column from this spec; an off-by-one between the stated count and the column list forces the implementer to guess whether to add a column or fix the number — and if they guess "add a column," they invent a field the data model may not have. A countable contradiction in a table spec is the kind of defect that silently propagates into a wrong header row.

**Adversarial:** If an implementer trusts "7 columns" over the six-item list and pads a seventh empty column, the cli coverage table gains a blank trailing column that the desktop-webview matrix does not have — does that break the cross-surface Sameness read, and would any other section catch the divergence, or does the contradiction live entirely inside this one line?

### 7. Expression / Anti-Pattern Alignment [trigger: expression level <= 0.3 OR >= 0.7] [priority: medium]

At expression `0.3` (low end), the Motion Hard-limits sub-block must be aggressive and explicit (no animation library / spring / parallax / scroll-driven / staggered entrances / 3D / canvas-WebGL / pulse-blink-glow / flashing-red-on-Fail / animate-to-100%-progress) and should be *longer* than the active-animations list. Verify no Surface Component Pattern, Iconography behavior, or Decisions-Log entry contradicts the ceiling by smuggling in a stagger / spring / parallax / pulse. Confirm `prefers-reduced-motion: reduce` handling is present and binding, dropping even the count-tint transition on a held count.

**Anchor example:** Motion → Hard limits for this expression level

> "At `0.3`, the following are BANNED: no `framer-motion` / animation libraries (zero animation library — CSS transitions only); no spring physics; no parallax; no scroll-driven animation; no staggered entrances beyond initial paint; no 3D transforms; no canvas/WebGL; no pulse / blink / glow on any status … no flashing red on `Fail` … no progress bar that animates-to-100% or hides on the operator-pause (it must freeze)."

**Issue:** This Hard-limits list is correctly longer and more specific than the active-animations list (hover 150ms, dialog 200ms fade, in-place color transition) — the healthy signal at expression `0.3`. To confirm nothing contradicts it, the active motion vocabulary must stay inside this ceiling everywhere: searched every Surface Component Pattern, the Iconography section, and the Decisions Log for `stagger`, `spring`, and `parallax` — none present outside this Motion ban block and the descriptive higher-expression rows of the Duration scale. Flag any future pattern that re-introduces one as active behavior.

**Why this matters:** Expression `0.3` is the single source of truth for the downstream motion budget. If a Surface pattern contradicted the ceiling, `/implement` would face two instructions and likely pick the more animated one (the default reflex), over-engineering motion for a "calm under load, no alarm" brand. The Hard-limits block is the load-bearing guardrail; its precision is what keeps motion noise out of downstream.

**Adversarial:** The `@media (prefers-reduced-motion: reduce)` rule in `@theme` is a blanket `* { animation: none; transition: none; }`. The signature's count-tint (green→amber freeze) is a `transition`, so reduced-motion removes it — leaving the freeze legible only via the phase-line text flip and the `aria-live` announcement. Is that the intended graceful degradation (text + announcement carry the hold when color-transition is suppressed), or does suppressing the tint quietly delete part of the signature for reduced-motion users — and does any section state which it is?
