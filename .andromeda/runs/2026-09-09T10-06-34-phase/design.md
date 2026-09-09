# design extract

## Relevance
Partial — this chunk builds no UI, but design's cli surface is *source-resident* (ANSI codes, `[PASS]`-class prefix literals, the TTY gate all live in Rust inside the 8 reformatted crates), so a workspace-wide rustfmt pass is the one non-UI operation with reach into design-owned surface.

## Constraints
- The cli token map is a closed set of ANSI 256 codes (114 / 179 / 203 / 60 / 146 / 246 / 117) bound to the Color World hexes; a formatting-only pass must leave every one byte-identical, and must introduce no new code or hex (per design-system §Surface: cli / Tokens). Whether the 60-file diff actually perturbs any such literal is research's question.
- The per-P-ID status-prefix set (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`) plus the run-level non-lamp captions (`[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) are a closed set; the pass must not alter, merge, or re-wrap them such that any status could read as color-alone (per design-system §Anti-Patterns → Per-Surface Bans → cli).
- Colorization is gated by `owo-colors` + `std::io::IsTerminal` decided **independently for stdout and stderr**, each requiring a TTY AND `NO_COLOR` unset AND `TERM != dumb`; design-system §Surface: cli / Tokens (preamble) + §Platform-Specific Notes require that structure to survive the reformat unchanged in meaning.
- The webview's 34-token `:root` block is a binding names+values contract and its declaration shape is load-bearing (`@theme` tree-shakes 11 of 34); design-system §Surface: desktop-webview / Tokens therefore requires that nothing in this chunk reflow or re-scope `tokens.css` — consistent with the chunk's own "Rust only, no prettier / `ui/` tree" boundary.
- Color in Conductor is functional-only (green=`Pass`, amber=HOLD/`CalibrationRegion`, red=`Fail`, slate-violet=`Blocked`, ID-cyan=mono status tier); a formatting chunk creates no surface, so any *new* color value appearing in the diff is a scope violation rather than a palette addition (per design-system §Anti-Patterns → Universal Bans).

## Patterns to follow
- Design's cli tokens are integer ANSI constants and ASCII string literals in the Rust source, not stylesheet assets (design-system §Surface: cli / Tokens) — the `conductor-cli` (7 sites / 1 file), `conductor-emit` (51 / 16) and `conductor-run` (64 / 8) hunks are the ones to read with a design eye; `conductor-report` is the crate the scope reports clean.
- By-name token *reuse* over new tokens: `error:` reuses Fail red (203) and `hint:` the Residual mute (246), under a stderr-specific gate (design-system §Surface: cli / Component Pattern 5) — a diff that appears to introduce a "new" color here should be read as a reuse or as leaked semantic change.
- The recessive ANSI-246 tier is used by a named *set* of non-lamp sites (`hint:` label, out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`), always paired with an always-rendered text label (design-system §Surface: cli / Tokens) — pairing is carried by the literal, so literal integrity is the whole guard.
- `comfy-table` widths are detected from terminal width and never hardcoded (design-system §Surface: cli / Component Pattern 3 + §Platform-Specific Notes) — relevant to the scope's open premise about `#[rustfmt::skip]`: a skip must not be used to freeze hand-aligned table/ANSI blocks into anything that reintroduces fixed widths.

## Anti-patterns to avoid
- Never introduce or alter a hex / ANSI value under cover of a formatting diff — color must remain functional and token-bound (design-system §Anti-Patterns → Universal Bans).
- Never let a re-wrap leave a status signalled by color alone; the ASCII prefix must always stand, including under `NO_COLOR` and when piped (design-system §Anti-Patterns → Per-Surface Bans → cli).
- Never widen the pass into `crates/conductor-tauri/ui/` — reflowing the `:root` token block risks the exact class of regression the `@theme` → `:root` history records (design-system §Surface: desktop-webview / Tokens).

## Contract bindings
- **design ↔ a11y:** the never-color-alone prefix set is design-owned but satisfies a11y SC 1.4.1; a re-wrap that damages a prefix literal is simultaneously an a11y regression (design-system §Anti-Patterns → cli; a11y §Use of Color).
- **design ↔ tests/CI:** the fmt gate itself is a CI/test-plan contract (test-plan §9 Lint row) — design contributes no gate step, only the invariants the pass must not break; design's checks below are diff-review checks, not new CI jobs.

## Acceptance criteria contributions
- The formatting diff contains zero changes to ANSI code values, hex values, or status/caption prefix string literals across all 60 files (per design-system §Surface: cli / Tokens).
- Every status line in the reformatted cli code still emits its ASCII prefix alongside color — no color-alone signal is created by re-wrapping (per design-system §Anti-Patterns → Per-Surface Bans → cli).
- The stdout and stderr `IsTerminal` / `NO_COLOR` / `TERM != dumb` gate conditions are semantically unchanged after the pass (per design-system §Surface: cli / Platform-Specific Notes).
- No file under `crates/conductor-tauri/ui/` — in particular the 34-token `:root` block in `tokens.css` — appears in this chunk's diff (per design-system §Surface: desktop-webview / Tokens).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** (§Surface: cli — Toolkit / Tokens / Platform-Specific Notes): retired `anstream`/`anstyle` as the named gate; the shipped mechanism is `owo-colors` + `std::io::IsTerminal`, decided per-stream. Relevant because that gate lives in the exact `crates/conductor-cli/src/` code this pass reformats, and the amendment's point was that the *behaviour* is mandated while the named mechanism moved — the reformat must not disturb either.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Surface: cli / Error output): recorded `error:`/`hint:` as a *reuse* of ANSI 203/246 after a detector wrongly proposed a new "Hint grey" palette row. Relevant as the review lens for this chunk: an apparent new color in a formatting diff is either a reuse or a semantic change that has slipped past the formatting-only boundary — never a palette addition.
- **2026-06-15-design-token-typography-bundle** (§Surface: desktop-webview / Tokens): `@theme` → `:root` because `@theme` tree-shook 11 of the 34 tokens. Relevant only as the reason the `ui/` tree must stay outside any widening of "format the workspace" — the token block's declaration shape is load-bearing, not cosmetic.
