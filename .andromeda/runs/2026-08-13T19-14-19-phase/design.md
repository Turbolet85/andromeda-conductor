# design extract

## Relevance
Partial — no new UI is built, but this chunk becomes a *producer* of `ReportState::KnownResidual` / `Blocked` / real `verdict` + `latency_ms` values that the already-shipped lamp, cli table, and run-report treatments render, so the domain's state-presentation contract governs what may be emitted.

## Constraints
- A `degraded_mode` read-back must reach every surface as the pre-accepted-residual treatment (muted dashed lamp / `[RESIDUAL]` / `~`), **never** as `Fail` red — the 5-valued `ReportState` exists precisely so `ManualCheck` and `KnownResidual` never silently collapse into `Fail` (per design-system §Color Palette → Verdict-vs-ReportState note).
- Zero new design vocabulary: the residual tier is the *existing* `--status-residual` ↔ ANSI 246 by-name pair, and the lamp/prefix set is closed (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`). No palette row, no ANSI code, no seventh lamp, no sixth `ReportState` may be added for this routing (per design-system §Surface: cli / Tokens + §Iconography).
- A malformed/absent/errored read-back emitted as a typed `Blocked` value must carry its named precondition string with measurement columns rendered `—`/null — "present-but-greyed, never measured", never a red error row (per design-system §Surface: cli / Component Patterns #3 + §Color Palette Info/Blocked row).
- Never color-alone: any state this chunk emits must be consumable as text — the ASCII bracket prefix / label is the signal, the tint only de-emphasizes (per design-system §Surface: cli / Tokens + §Anti-Patterns Per-Surface Bans cli).
- Values that become real (`latency_ms`, and `fingerprints` if the open question resolves to "include") belong to the reserved mono ID-cyan status tier (ANSI 117 ↔ `--color-id-cyan`, tabular-nums) — they must be emitted as plain data for that tier, never pre-formatted with a raw hex/ANSI literal (per design-system §Typography Data row).
- Newly-real `Fail` verdicts resolve in place, motionless — no blink/pulse/flash may be introduced anywhere on the newly-measured path (per design-system §Motion Hard limits + §Anti-Patterns Rejected Defaults).

## Patterns to follow
- **Single lamp projection** — `Lamp::for_record` (`D:\dev\projects\conductor\crates\conductor-core\src\lamp.rs`) already gives `Blocked`/`KnownResidual` state-driven precedence over the verdict; assign the `ReportState` and let the shipped projection decide the treatment rather than branching at any render site (per design-system §Color Palette Verdict-vs-ReportState + §Surface: cli / Component Patterns #4).
- **Results / SLO table blocked-row rule** — 6 columns, cyan P-ID and right-aligned cyan `latency_ms`, blocked rows em-dashing never-measured cells; this is a *separate* table from the 4-column `conductor coverage` matrix, which carries no verdict column (per design-system §Surface: cli / Component Patterns #3).
- **Residual-tier reuse precedent** — `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs` already reuses ANSI 246 for three non-lamp cases (`hint:`, out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]`) with zero new palette entries; follow the same "reuse the shipped pair by name" move if this chunk needs any residual-adjacent tint (per design-system §Surface: cli / Tokens).
- **Run-report residual row** — a `KnownResidual` row is the dashed muted lamp plus the "expected to fail until {named fix}" note (P-032's `recent_commits` stub → v0.3.0 is the named first instance), so the mapping should preserve enough identity for that note; the note is a render-side affordance, not a new envelope field (per design-system §Surface: desktop-webview / Component Patterns #6).

## Anti-patterns to avoid
- Adding a new color, ANSI code, lamp glyph or bracket label to express "degraded" — the treatment already exists (per design-system §Anti-Patterns Universal Bans; reinforced by three prior amendments).
- Downgrading a degraded/unmeasurable read-back into `Fail` red, or into an undifferentiated gray "no result" placeholder — the explicit Rejected Default "conflating *no result yet* with *failed*" (per design-system §Anti-Patterns Rejected Defaults).
- Introducing any alarm affordance (flash/pulse/toast) now that Fail verdicts become real — "calm under load, no alarm"; Conductor also must never emit a native OS toast (per design-system §Anti-Patterns Rejected Defaults + §Motion Hard limits).

## Contract bindings
- **State + label pairing** binds a11y §Use of Color (SC 1.4.1) — every emitted `ReportState`/lamp travels with its ASCII prefix and label, which is also the NO_COLOR / piped-output contract.
- **`--status-residual` ↔ ANSI 246** is a cross-surface by-name pair: any change on one side binds the webview token block (§Surface: desktop-webview / Tokens, the 34-token contract) and the cli ANSI map simultaneously.
- **Lamp projection ↔ arch run-report envelope** — the design's 6-way lamp is a *projection* of the envelope's independent `verdict` + `state` fields; this chunk must keep both populated (never flattened) for the projection to hold.
- **Redacted observed text ↔ report prose tier** — redaction is security-owned, but the redacted string lands in the run-report body (`--text-secondary` / journal-grey prose), so the redaction placeholder must be readable prose, not a struct dump.

## Acceptance criteria contributions
- (design) A `degraded_mode` read-back renders `[RESIDUAL]` + label in the residual-mute tier (ANSI 246 ↔ `var(--status-residual)`), and never `[FAIL]` / ANSI 203 (per design-system §Color Palette Verdict-vs-ReportState note).
- (design) The change adds no palette row, no ANSI code, no lamp state and no bracket prefix — the closed 6-lamp set and the 34-token `:root` contract are byte-unchanged (per design-system §Surface: desktop-webview / Tokens + §Iconography).
- (design) A `Blocked` row produced by an unusable read-back carries its named precondition string and em-dashes/nulls its measurement cells, never a red error row (per design-system §Surface: cli / Component Patterns #3).
- (design) Real `latency_ms` (and `fingerprints`, if populated) surface through the reserved mono ID-cyan data tier with no hardcoded hex/ANSI literal at the new call sites (per design-system §Typography Data row).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** — §Color Palette: the ANSI 246 ↔ `var(--status-residual)` entry now names the recessive tier's non-lamp uses as a *set* rather than a count. Why it matters here: this chunk is the next place tempted to invent a "degraded" color; the standing resolution is reuse-the-pair, and if a new non-lamp use appears, extend the set without a fresh literal.
- **2026-08-09-out-of-scope-classification-treatment** — §cli Tokens + Component Patterns #3: recorded the residual tier's non-lamp reuses, named `var(--status-residual)` as the webview half of the by-name pair, and disambiguated the 6-column results/SLO table from the 4-column coverage matrix. Why it matters: this chunk writes into the *results* table's verdict/state/latency columns, not the coverage matrix — do not re-conflate them.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — §cli Error output: the detector proposed a NEW "hint grey" palette row and was **corrected** at validation because ANSI 246 was already the shipped Residual-mute token. Directly precedential: a new state-adjacent surface reuses shipped tokens; a proposed new palette entry is the failure mode, not the fix.
- **2026-06-15 / 2026-06-24 / 2026-06-26 reconciliations** — establish the standing "spec-illustration → sound-impl reconciliation" routine: when the shipped implementation proves the invariant, amend the plan's illustration rather than bending the implementation. Applies if the residual note or lamp wording needs a wording-only correction after this chunk.
