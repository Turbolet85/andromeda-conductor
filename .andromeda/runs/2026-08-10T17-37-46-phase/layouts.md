# layouts extract

## Relevance
Partial — the chunk's engine/detection work is out of domain, but the named `Blocked` precondition it produces renders on two governed surfaces (cli verdict lines + `conductor preflight` line; desktop-webview run-report row), so layout rules bind the *presentation* only.

## Constraints
- A `Blocked` row/line renders as the hollow-ring / `[BLOCKED]` state with its **named precondition string on a following detail line**, and its measurement columns render `—` / null — never a red error, because it was never measured (per layout-templates §Surface: cli — Component — Primary content block 2, and §Surface: desktop-webview — Component — Primary content block 2).
- The per-P-ID bracket-label / lamp set is **closed at six**; a new precondition is a new *string under the existing `[BLOCKED]` label*, not a seventh label or lamp state (per layout-templates §Surface: cli — Component — Primary content block 2).
- Color is always paired with its ASCII prefix and state text, so the signal survives `NO_COLOR` / piping / monochrome; `count-blocked` ↔ ANSI 60 is adapted, never forked, across surfaces (per layout-templates §Surface: cli — IA notes / Multi-surface coordination).
- Operator-facing strings that land in the report/logs follow the sanitized-output rule — no absolute host paths, no internal struct names, no stack traces; stdout stays reserved for raw artifact data with human messages on stderr (per layout-templates §Surface: cli — Component — Footer / terminator + error output, and §IA notes Pipe discipline).
- `conductor preflight [--json]` is the readiness-gate screen: it emits the `ReadyState` JSON or a `[PASS]` / `[BLOCKED]` line and exits non-zero on a Blocked precondition — the new value flows through that existing shape, adding no screen (per layout-templates §Surface: cli — Primary screens).
- Distinct non-result states are **never downgraded to Fail**; the footer roll-up carries the `Blocked` count tinted `count-blocked` and paired with its label so the tally is never color-only (per layout-templates §Surface: desktop-webview — Component — Primary content block 2, and §Component — Footer (status strip)).

## Patterns to follow
- The **P-022 `port-occupier` precondition shape** is the direct template: `[BLOCKED]` state cell + named precondition carried on the row's indented detail line (cli) / `Body`, `text-tertiary` line beneath the verdict line (webview) — reuse this shape verbatim for the divergence precondition (per §cli Primary content block 2, §desktop-webview Primary content block 2).
- Run-level qualifiers that are **not** a check state ride *outside* the lamp column with an always-printed ASCII bracket label and the recessive muted tint (the `[ENVIRONMENT-SUSPECT]` / `not-conductors` Mode-cell precedent) — the fallback pattern if the divergence is ever surfaced run-level rather than per-P-ID (per §cli Component — Primary content block 2).
- Token mirroring by name, not by fork: whatever renders on cli in ANSI 60 renders on the webview in `count-blocked` (per §cli IA notes — Multi-surface coordination).
- Preflight readiness line format — protocol/tool/canary identifiers in the ID-cyan mapping with the `[OK]` / `[BLOCKED]` prefix carrying the gate without color (per §cli Component — Header / banner).

## Anti-patterns to avoid
- Do **not** render the divergence in `status-fail` / ANSI 203, and do not let it flash or blink — `Blocked` ≠ red (per §cli + §desktop-webview Primary content block 2).
- Do **not** introduce a new lamp state, a seventh bracket label, or a new color/ANSI code for this precondition (per §cli Component — Primary content block 2).
- Do **not** emit the condition through `alert()`/`confirm()`, a native OS toast, or an un-prefixed color-only cue — the styled `AlertDialog` is the only modal and Conductor never emits OS toasts (per §desktop-webview IA notes, §Component — Footer (status strip)).

## Contract bindings
- **layouts ↔ architecture:** the readiness result's `blocked_precondition` field (architecture §Standard Contracts, per this chunk's scope) is the data behind the cli `[BLOCKED]` preflight line and `--json` `ReadyState` payload — layout fixes the rendered shape, architecture fixes the field.
- **layouts ↔ a11y:** "status never color-alone" + "verdict/state change is announced" are stated here as behavioral requirements; a11y derives the live-region attributes and accessible names (per §Notes).
- **layouts ↔ design:** `count-blocked` ↔ ANSI 60 mapping and the note that `count-blocked` shares its hex with `text-muted` (so a plain dim reads as `Blocked`) are design-owned values referenced by name only (per §Notes, §cli Multi-surface coordination).
- **layouts ↔ security:** the no-absolute-host-path / sanitized-message rule for operator-facing strings is honored at the layout level and owned by `.claude/rules/security.md` §Error handling (per §IA notes — security guardrails, §cli Footer / terminator + error output).

## Acceptance criteria contributions
- (layouts) The divergence renders under the existing `[BLOCKED]` label / hollow-ring `count-blocked` lamp with its named precondition on the detail line, measurement columns `—` / null, and never in `status-fail` red (per layout-templates §cli Component — Primary content block 2 · §desktop-webview Component — Primary content block 2).
- (layouts) No seventh lamp state, bracket label, token, or ANSI code is introduced — the per-P-ID label set still enumerates exactly six (per layout-templates §cli Component — Primary content block 2).
- (layouts) The precondition string prints its ASCII bracket prefix and full text under `NO_COLOR` / when piped, and contains no absolute host path, struct name, or stack trace (per layout-templates §cli IA notes — Pipe discipline · §Component — Footer / terminator + error output).
- (layouts) `conductor preflight` still emits the `[PASS]`/`[BLOCKED]` line (or `--json` `ReadyState`) and exits non-zero on this Blocked precondition — no new screen or verb added (per layout-templates §cli Primary screens).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** (§cli Output structure `conductor run` · Component — verdict/report-state lines) — the nearest prior change to the exact block this chunk writes into: it added the run-level `[ENVIRONMENT-SUSPECT]` caption *outside* the lamp column and, in doing so, explicitly declared the six per-P-ID labels a closed set with `ReportState` staying five. Why it matters here: it is the standing precedent for adding new operator-facing signal without growing the lamp set — a named precondition must ride inside `[BLOCKED]`, not beside it.
- **2026-08-09-out-of-scope-classification-treatment** (§Mode cell) — recorded that `--count-blocked` shares its hex with `--text-muted`, so a plain "dim it" treatment reads as `Blocked`; relevant as a caution against inventing a new muted tint for this precondition.
- **2026-06-23-5-command-agent-run-harness** (§cli Primary screens) — registered the `conductor preflight [--json]` verb and its exit-code contract; that is the surface this chunk's `blocked_precondition` value flows through, so the verb line stays as-is and only its Blocked payload changes.
