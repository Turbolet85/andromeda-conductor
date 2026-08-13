# design extract

## Relevance
Partial — the chunk is test/golden work (no rendering surface), but its CARRY re-scopes `check_load_envelope`, which decides when the already-shipped `[ENVIRONMENT-SUSPECT]` run-level caption fires and reconciles the contract/arch wording that names its tier.

## Constraints
- The envelope breach is a **run-level qualifier**, not a verdict: it must never become a seventh lamp, a sixth `ReportState`, or render as Fail red — re-scoping *when* it fires must not change *what tier* it is (per design-system §Color Palette · the "Verdict (3) vs ReportState (5)" note; §Surface: cli / Component Patterns #4).
- Re-scoping to emitting-phase duration adds **zero new palette rows and zero new ANSI entries** — the caption stays on the existing Residual-mute pair (ANSI 246 ↔ `var(--status-residual)`), which the plan already records as the shared recessive tier for every non-lamp use (per design-system §Surface: cli / Tokens · Residual-mute entry).
- The status-prefix set is **closed**: `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` plus the run-level `[ENVIRONMENT-SUSPECT]` outside the lamp column. A retired exemption or a newly-breaching scenario reuses that label; it does not mint one (per design-system §Surface: cli / Tokens).
- Never color-alone: the ASCII bracket label always renders and carries the signal, the 246 tint only de-emphasizes — output must stay readable under `NO_COLOR`, `TERM=dumb`, and pipe-stripping (per design-system §Anti-Patterns · Per-Surface Bans · cli; §Iconography Rule).
- Anything the goldens capture from the CLI must be ASCII-prefix output with no emoji and no TTY affordances — the `indicatif` spinner is TTY-gated precisely "so agent-captured artifacts stay clean" (per design-system §Surface: cli / Component Patterns #1; §Anti-Patterns · Per-Surface Bans · cli).
- Prose/doc reconciliation (c) that names the tier should keep naming the **set** of non-lamp reuses rather than a fresh literal count, and color-free surfaces (Markdown contract/arch text) express the tier via emphasis or the bracket label, never a color word (per design-system §Surface: cli / Tokens · Residual-mute entry).

## Patterns to follow
- Reuse-by-name of the Residual-mute tier: `crates/conductor-cli/src/render.rs` already carries `ENVELOPE_SUSPECT_MUTE = 246` with a doc comment citing the design-system anchor and explaining why it is deliberately not fail (203) or blocked (60) — the re-scope inherits this, no render change needed (design-system §Surface: cli / Tokens).
- Caption shape `[ENVIRONMENT-SUSPECT] {cause}` printed **once above** the per-P-ID result lines because it qualifies every row beneath it — the re-scoped check must keep producing a **named human-readable cause string**, the same "a reason is mandatory" discipline the `[[exempt]]` entries carry (design-system §Surface: cli / Component Patterns #4).
- Gate-failure output (e.g. ledger rot when (b) does not land with (a)) uses the sanitized stderr edge — `error: <short>` + detail + `hint: <fix>`, Fail red 203 / Residual mute 246, no absolute host paths or struct names (design-system §Surface: cli / Component Patterns #5).
- Precedent for the reconciliation itself: a new use-site of an existing token is documented as a **use-site note on the existing entry**, never a duplicated palette row (design-system §Surface: cli / Tokens; amendment 2026-06-24).

## Anti-patterns to avoid
- Do NOT introduce a new color/ANSI entry, bracket label, or lamp state for the re-scoped emitting-phase term (design-system §Anti-Patterns · Universal Bans · "color purely for decoration"; §Surface: cli / Tokens closed set).
- Do NOT let a breach or a retired exemption read as a red `Fail` or as a grey "no result" — the plan's Rejected Default is exactly this conflation of no-result-yet / not-measured with failed (design-system §Anti-Patterns · Rejected Defaults).
- Do NOT put emoji or color-only signalling into any output a golden or piped artifact captures (design-system §Anti-Patterns · Per-Surface Bans · cli).

## Contract bindings
- **Residual-mute pair** binds cli ↔ desktop-webview by name: ANSI 246 ↔ `var(--status-residual)`; the webview half is untouched by this chunk, so the pairing must not drift (design-system §Surface: cli / Tokens).
- **Bracket-label set** binds design ↔ tests harness: `.claude/rules/testing.md` names `[ENVIRONMENT-SUSPECT]` among the E2E brand anchors, so any golden/E2E assertion on envelope output anchors on the label text, never on color (design-system §Surface: cli / Component Patterns #4).
- **Never-color-alone** binds a11y §Use of Color SC 1.4.1 — the always-present ASCII label is the a11y-visible carrier (design-system §Iconography Rule).
- Motion/typography/spacing/radius bindings: **none** — this chunk renders no webview surface.

## Acceptance criteria contributions
- (design) The load-envelope re-scope adds no new palette row, ANSI code, bracket label, or lamp state — the breach continues to render as `[ENVIRONMENT-SUSPECT]` on the existing Residual-mute tier (per design-system §Surface: cli / Tokens).
- (design) A breach and a retired exemption never render as `Fail` red or collapse into a sixth `ReportState`; the caption stays a run-level qualifier above the per-P-ID lines (per design-system §Color Palette · Verdict-vs-ReportState note).
- (design) The `[ENVIRONMENT-SUSPECT]` ASCII label and its cause string remain present under `NO_COLOR` / piped stdout — signal survives with color stripped (per design-system §Anti-Patterns · Per-Surface Bans · cli).
- (design) Any CLI text captured into a committed golden is ASCII-prefixed, emoji-free, and spinner-free (TTY-gated), so the artifact stays clean and byte-stable (per design-system §Surface: cli / Component Patterns #1).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** — §Color Palette Residual entry: the ANSI 246 ↔ `var(--status-residual)` entry recorded the `[ENVIRONMENT-SUSPECT]` caption as the tier's **third** non-lamp reuse and switched to naming the reuses as a SET rather than a literal count (so the body does not re-stale on the fourth). Directly this chunk's caption; keep the set-naming when reconciling contract/arch wording.
- **2026-08-09-out-of-scope-classification-treatment** — §Surface: cli / Tokens + Component Patterns #3: established the non-lamp-reuse convention (existing pair, zero new tokens; the always-rendered label carries the signal, tint only de-emphasizes; Markdown uses emphasis as the color-free counterpart) and disambiguated the **Results / SLO table** (6 cols, has a state column) from the separate 4-column `conductor coverage` table — relevant if any golden captures either table.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — §Surface: cli Error output: the precedent that a new edge **reuses** shipped tokens (Fail red 203 / Residual mute 246) and gets a use-site note, with the proposed new palette row explicitly CORRECTED away at validation. This is the template if the re-scope tempts a "new envelope color".
