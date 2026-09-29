# Design System — Amendments

_Append-only changelog of amendments to `design-system.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-design-token-typography-bundle — §Tokens illustrative CSS `@theme` → `:root`
**Section:** §Surface: desktop-webview / Tokens
**Change:** the token block is declared on plain `:root` (was `@theme`), with `@import "tailwindcss"` retained for the engine; the light-mode override is `@media (prefers-color-scheme: light) { :root { … } }` (was a nested `@theme`). Token NAMES + VALUES unchanged (the 34-token binding contract). A Tailwind-v4 note states the reason.
**Why:** Tailwind v4 `@theme`/`@theme static` tree-shakes non-namespace tokens (only 23/34 emitted — all `--space-*`, three `--radius-*`, `--motion-micro` and `--ease-quiet` dropped) and forbids `@theme` nested inside `@media`; the shipped `tokens.css` declares all 34 on `:root` and all 34 emit (Vite's minifier preserves author custom props). The operator chose the `:root` replacement at escalation. Sets a standing `playbook.md` rule: a spec illustration reconciled to a sound implementation is routine when the report proves the invariant holds.
**Ref:** .andromeda/runs/2026-06-15T22-05-00-wrap/

## 2026-06-24-sanitized-stderr-agent-mode-logging — §cli "Error output" documents the error:/hint: token reuse
**Section:** §Surface: cli / Output structure (5. Error output)
**Change:** the `error:` label reuses Fail red (ANSI 203) and `hint:` the Residual mute (ANSI 246), tty-gated on a stderr-specific `IsTerminal` gate distinct from the stdout gate; the ASCII labels are always present (never color-alone). Recorded as a use-site note on the existing "Error output" entry — no palette entry added.
**Why:** ANSI 246 is the EXISTING "Residual mute" token, not a new color, so a proposed "Hint grey" palette row was rejected; the error edge reuses two shipped tokens (203/246) and the token-based, never-color-alone invariant holds. Routine spec-illustration → sound-impl reconciliation.
**Ref:** .andromeda/runs/2026-06-24T20-38-37-wrap/

## 2026-06-24-paused-count-hold-point-signature — §Motion gains a Motion-tokens table (registers --motion-heartbeat)
**Section:** §Motion (calibrated to expression level 0.3)
**Change:** a Motion-tokens table (mirroring the §Spacing / §Border-Radius token tables) registers `--motion-micro` (150ms), `--motion-heartbeat` (1600ms) and `--ease-quiet`; the heartbeat-period row ties the live count breath to the signature (value-ticking on the count arrives with the Epoch-9 live-counter `Channel`). Was: motion documented as prose + a duration-scale expression table with no named-token inventory. Other token NAMES/VALUES unchanged.
**Why:** the chunk added `--motion-heartbeat: 1600ms` to `tokens.css` as the heartbeat period — `--motion-micro` 150ms is a flicker, and tokens-by-name forbids a raw ms — user-approved at phase; with no named-token inventory the new token was unregistered.
**Ref:** .andromeda/runs/2026-06-24T23-44-29-wrap/

## 2026-06-26-component-primitives-library — operator-pause dialog fade reconciled 200ms → `--motion-micro` (150ms)
**Section:** §Motion (This project's values) + §Component Patterns §2 (Operator-pause go/no-go dialog)
**Change:** the operator-pause dialog fade is `--motion-micro` (150ms), was a literal 200ms — the §Motion "This project's values" transitions line and the §Component Patterns §2 dialog entry both read "150ms fade (`--motion-micro`)".
**Why:** the shipped `OperatorPauseDialog` (Radix AlertDialog) animates with `var(--motion-micro)` because no 200ms token exists in `tokens.css`, and tokens-by-name forbids a raw 200ms literal, so the spec's 200ms was unrealizable as written. Routine spec-illustration → sound-impl reconciliation; the never-color-alone and token-bound invariants hold.
**Kept:** the generic expression-scale ceiling — §Motion "200ms fades at most" and the `0.3-0.4 → 200ms fade` reference row; 150ms satisfies "at most 200ms".
**Ref:** .andromeda/runs/2026-06-26T23-47-21-wrap/

## 2026-08-08-sut-capability-manifest — De-hardcoded P-ID range in token-usage examples
**Section:** §Color Palette (Primary) · §Typography (Data row) · §Surface: cli ANSI map · §Brand Identity
**Change:** the Mono status-tier / ANSI-117 usage examples name "P-IDs" (was the fixed `P-001..P-060` range). No token, hex or type-role changed.
**Why:** the illustrative examples carried the superseded range.
**Ref:** .andromeda/runs/2026-08-08T16-05-00-wrap/

## 2026-08-09-current-sut-coverage-classification — De-hardcoded the last two literal-60 prose counts
**Section:** §Brand Identity (Domain anchors) · §Surface: desktop-webview → Component Patterns 3 (Coverage matrix)
**Change:** "the dense single-row-per-P-ID wall of all 60 capabilities" now reads "over the manifest's accepted capability set"; "Virtual-scroll for the full 60-row wall" now reads "for the full wall (one row per manifest capability)". Wording only — no token, hex or type-role changed.
**Why:** the coverage classification widened from 60 to 82 rows, so both counts were stale; these were the two prose sites the 2026-08-08 de-hardcoding missed. Standing treatment: de-hardcode, don't substitute — name the set, never the new literal.
**Ref:** .andromeda/runs/2026-08-09T14-17-38-wrap/

## 2026-08-09-out-of-scope-classification-treatment — ANSI 246 / Residual-mute records its second non-lamp reuse; results-vs-coverage table disambiguated
**Section:** §Surface: cli / Tokens (the ANSI 256 map, Residual-mute entry) · §Surface: cli / Component Patterns #3
**Change:** the Residual-mute → ANSI 246 entry records the tier's two NON-lamp uses (the `hint:` stderr label and the coverage-matrix out-of-scope Mode cell), names `var(--status-residual)` as the webview half of the same by-name pair, and states that the always-rendered label carries the signal while Markdown uses emphasis as its color-free counterpart. Component Pattern #3 is retitled **Results / SLO table** (was "coverage-matrix / SLO table") and states that the coverage matrix is a separate 4-column table with no verdict/state column.
**Why:** the chunk tinted the out-of-scope Mode cell with the existing pair (zero new tokens, zero new ANSI entries), so the map no longer accounted for where the tier is used. The retitle fixes a pre-existing conflation, resolved with the operator: the block enumerated the results table's 6 columns while `conductor coverage` renders 4.
**Ref:** .andromeda/runs/2026-08-09T15-56-31-wrap/

## 2026-08-09-sut-load-envelope — Residual-mute tier records its third non-lamp reuse
**Section:** §Color Palette (Residual-mute entry)
**Change:** the ANSI 246 ↔ `var(--status-residual)` entry names the recessive tier's non-lamp uses as a SET — the `hint:` stderr label, the out-of-scope Mode cell and the run-level `[ENVIRONMENT-SUSPECT]` load-envelope caption — rather than a literal count (was "two NON-lamp uses"), and records the Markdown counterpart for the new one.
**Why:** the chunk added a third non-lamp reuse; naming the set rather than a fresh literal keeps it from re-staling on the fourth.
**Ref:** .andromeda/runs/2026-08-10T15-43-07-wrap/

## 2026-08-18-error-baseline-spike-live-proof — cli Pattern 4 sample tier de-literalized
**Section:** Surface: cli / Component Patterns #4
**Change:** the `P-009 … 1840ms <5s` sample renders `<slo_tier>` drawn from the scenario's declared value (closed `<5s`/`<20s`/`<90s` set) — never a baked per-scenario literal (was `<5s`).
**Why:** error-baseline-spike re-declared `<5s` → `<90s`; placeholder-over-literal per the derived-count rule so the sample cannot re-stale.
**Ref:** .andromeda/runs/2026-08-18T19-10-05-wrap/

## 2026-09-01-desktop-a11y-sweep — text-tier token values moved for contrast
**Section:** §Color Palette → Text Hierarchy (Tertiary + Muted rows) · §Surface: desktop-webview → Tokens (both theme blocks)
**Change:** `--text-tertiary` dark `#717AA0` → `#838EBA`; `--text-muted` dark `#565F89` → `#727EB5` and light `#6E7491` → `#636882`. The Muted row's `(blocked slate-violet)` descriptor retired — Muted no longer shares a hex with `--count-blocked`.
**Why:** axe `color-contrast` (wcag2aa / SC 1.4.3) flagged 18 nodes, all `--text-tertiary`, at 3.50:1 on `--color-raised-2` and 3.78:1 on `--color-raised-1` against a required 4.5:1. Computing a11y-plan §6's nine pairs showed the node list was a floor, not the scope: `--text-tertiary`/`--color-base` failed latently at 4.06:1 and `--text-muted`/`--color-inset` failed in BOTH themes (2.91:1 dark, 3.86:1 light), needing corrections in opposite directions. Values were solved against all nine pairs in both themes plus the text-ramp ordering and a token-collision guard.
**Kept:** light `--text-tertiary` (already passed); `--count-blocked` / `--status-manual` / `--status-residual` / `--border-emphasis` untouched.
**Ref:** .andromeda/runs/2026-09-01T22-22-12Z-wrap/

## 2026-09-02-screen-reader-manual-spec — empty / in-progress samples corrected to shipped; the checklist's single shipped mount
**Section:** §Component Patterns 3 (Coverage matrix Empty) · 6 (Run-report view Empty / In-progress) · 7 (Operator-checklist)
**Change:** Pattern 3's empty sample is the shipped `No scenarios found.` (was `No scenarios loaded`); Pattern 6's is the shipped `No run yet` (was `No run yet — pick a scenario/suite to begin`), and its "Run in progress" report-area prose is retired — no such prose ships; the titlebar phase line + count and the matrix carry the live state. Pattern 7 records that the primitive mounts only inside the operator-pause dialog as shipped (the report site designed, not built — route-owned) and that the unticked roll-up is the checklist view's own `role=status` line (the footer strip unbuilt).
**Why:** the screen-reader pass heard, and the shipped UI source confirmed, that the doc baked strings and a mount the code does not produce. The design intents stand; only the shipped state is recorded.
**Ref:** .andromeda/runs/2026-09-02T11-47-51Z-wrap/

## 2026-09-03-conductor-tauri-survivors-dispositioned — decisions-log `@theme` retired to `:root`
**Section:** §Design Decisions Log — the `2026-06-14: Initial design system generated by /andromeda-design (Phase 4)` entry, `Surfaces:` bullet (`:406`)
**Change:** the desktop-webview stack reads "React 19 + Tailwind v4.1 design tokens on `:root` + shadcn/ui" (was "Tailwind v4.1 `@theme`").
**Why:** §Tokens retired `@theme` for plain `:root` on 2026-06-15 (`@theme` / `@theme static` tree-shake non-namespace tokens, dropping `--space-*`) and the shipped `crates/conductor-tauri/ui/src/styles/tokens.css` declares on `:root` with no `@theme` rule; this bullet was the duplicate the 2026-06-15 single-site apply left standing, so the doc contradicted itself. Raised on the operator's directive; not this chunk's own drift.
**Kept:** §Tokens (the authority, correct); architecture's mentions, which already state "design tokens on `:root`" and name `@theme` only as the reason not to use it; a11y-plan's statement that NAMESPACE tokens survive tree-shaking — none stale.
**Ref:** .andromeda/runs/2026-09-03T09-10-06-wrap/

## 2026-09-03-live-pulse-preconditions-probed — `anstream` retired as the named cli gate; `[PRECONDITION]` joins the non-lamp set
**Section:** §Surface: cli — Toolkit / Framework · Tokens (ANSI-map preamble · Residual-mute row · status-prefix line) · Platform-Specific Notes
**Change:**
- `anstream`/`anstyle` retired from the named cli styling stack, the ANSI-application path and the Platform-Specific Notes bullet; the named gate is now `owo-colors` + `std::io::IsTerminal`, decided independently for stdout and stderr, each requiring a terminal AND `NO_COLOR` unset AND `TERM != dumb`. The mandated BEHAVIOUR is unchanged; only the named mechanism moved.
- The status-prefix line names the run-level non-lamp caption SET (`[ENVIRONMENT-SUSPECT]` + `[PRECONDITION]`), was one member; the Residual-mute (246) reuse list gains `[PRECONDITION]` — set framing kept, no count introduced, no new palette row, lamp set still closed at six.
**Why:** `anstream` was measured absent from both manifests and from all of the cli crate's source; spec illustration → sound impl with the invariant holding governs.
**Kept:** no library version touched — the proposal's `indicatif 0.18` / `inquire 0.7` were outside its detector's scope and disagree with the manifest (`0.17.11` / `0.9.4`); the standing `indicatif 0.18` is pre-existing and owned by the route's *Dependency polish* entry.
**Ref:** .andromeda/runs/2026-09-03T19-20-00-wrap/
## 2026-09-04-sr-findings-remediation — coverage-matrix Empty corrected; the picker's filter-miss state recorded
**Section:** §Component Patterns 3 (Coverage matrix) · §Component Patterns 5 (Scenario/suite picker)
**Change:** Pattern 3's **Empty** names the shipped `No coverage data.` (was `No scenarios found.`) and states that `No scenarios found.` is the SCENARIO-section empty-catalog prose, not this component's. Pattern 5 gains the picker's **Filter miss** state — the shipped `No scenarios match.` in the list's existing recessive styling (no new token), rendered from a persistently-mounted announced region, noting that a region mounting together with its text announces nothing.
**Why:** Pattern 5's state is this chunk's new surface (the `ScenarioPicker` `FilterMiss` region, announced as expected on the screen-reader pass); routine spec → impl. The Pattern 3 mis-attribution predates this chunk (2026-09-02); the corrected value rests on a direct read of the shipped app source, and was resolved with the operator as an escalation.
**Ref:** .andromeda/runs/2026-09-04T07-33-12Z-wrap/

## 2026-09-07-dependency-polish — cli Toolkit/Framework inquire version reconciled
**Section:** §Surface: cli — Toolkit / Framework
**Change:** `inquire` 0.7 → 0.9. The `indicatif` 0.18 value on the same line was already the target state; this chunk's bump made it true.
**Why:** design-system's detectors key on tokens, derived counts and platform verdicts, so a stated library version in the Toolkit/Framework line falls inside none of them and draws no proposal — such a site has to be raised by the orchestrator, as here.
**Ref:** .andromeda/runs/2026-09-07T12-43-31-wrap/
## 2026-09-10-release-build-and-bundle — the ~3 MB artifact literal de-literalized at both sites
**Section:** Depth Strategy (Rationale) · Anti-Patterns — Per-Surface Bans (desktop-webview)
**Change:** the `~3 MB Tauri artifact` literal is retired at both sites — measured false (nsis 4.21 MB · msi 5.87 MB, 2026-09-10). §Depth Strategy cites "the Tauri installer-artifact + performance discipline — the msi + nsis set the bundler produces, whose measured sizes are recorded against `verification-matrix.json#v2-27`"; the desktop-webview ban reads "GPU + installer-artifact size discipline". The DESIGN argument is untouched: borders-only depth, no `backdrop-filter`, no GPU-heavy shadow compositing.
**Why:** a derived count is named as its SET rather than substituted by a fresh literal that would re-stale on the next bundle.
**Kept:** the `Tauri ≥ 2.10.3` mentions — they are the security-plan CRATE floor, deliberately not touched.
**Ref:** .andromeda/runs/2026-09-10T20-36-29-wrap/
