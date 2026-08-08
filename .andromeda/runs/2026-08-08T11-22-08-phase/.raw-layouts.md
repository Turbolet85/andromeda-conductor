# layouts extract

## Relevance
Partial — the chunk builds no surface, but its failure path (route line: "malformed reporting blocked") and the P-ID range labels it touches are both already fixed by the layout plan.

## Constraints
1. **A `Blocked` outcome renders per the fixed contract on both surfaces** — cli: `•` glyph + `[BLOCKED]` ASCII prefix + ANSI 60 with the **named precondition string on its detail line**, measurement columns `—`/null; desktop: hollow-ring lamp + precondition string below, measurement cols `—`/null. Never a red error — `Blocked` was never measured (per layout-templates §Surface: cli — Primary content block 2 (verdict / report-state lines); §Surface: desktop-webview — Primary content block 2 (run-report view + verdict lamp)).
2. **The status tier is closed** — the machine-verdict triad plus the three non-verdict ReportStates (`ManualCheck` / `KnownResidual` / `Blocked`) map 1:1 to typed run outcomes on both surfaces; a malformed-manifest state must land in an existing one, not add a seventh (per layout-templates §Decisions Log — Cross-surface IA decisions).
3. **Status is never color-alone** — every state cell pairs its color with the bracket prefix so it survives `NO_COLOR`, piping and screen readers (per layout-templates §Surface: cli — Signature placement; §Surface: cli — IA notes, Pipe discipline).
4. **If the wall decision instead lands on harness fault**, the message shape is fixed: **stderr**, `error: <short>` + contextual detail + `hint: <fix>`, sanitized (no absolute host paths / internal struct names / stack traces), un-colorized when piped, stack traces only under `--debug`/`-v`; stdout stays reserved for parseable artifact data (per layout-templates §Surface: cli — Footer / terminator + error output).
5. **cli output structure is a parsed contract** — downstream agents parse it, so changing the `comfy-table` column set or the state-cell vocabulary without a `--format` flag is a breaking change (per layout-templates §Surface: cli — IA notes, Command model).
6. **Headless invariant** — a manifest load failure must never introduce an interactive gate; `inquire` prompts are `isatty`-checked and the agent-driven path is never blocked on a prompt (per layout-templates §Surface: cli — IA notes, Headless invariant).
7. **No surface change is authorized by this chunk** — desktop stays the four run states of one frameless window (no router / no routes / no new modal), the cli verb list stays run|suite|report|preflight|coverage; responsive breakpoints are N/A (per layout-templates §Surface: desktop-webview — Primary screens + IA notes Global model; §Decisions Log — Responsive breakpoints).

## Patterns to follow
- `D:\dev\projects\conductor\crates\conductor-cli\src\commands\preflight.rs:26-28` — the in-repo realization of the `[BLOCKED]` + named-precondition line (`blocked_precondition` → `[BLOCKED] preflight — {precondition}`); the manifest precondition should read the same way, not invent a second phrasing.
- `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs:42` — the lamp→ANSI mapping (`Lamp::Blocked => 60`); reuse the existing lamp, do not add a code or a variant.
- `D:\dev\projects\conductor\crates\conductor-cli\src\main.rs:59` — the existing harness-fault `hint:` string for a missing contract manifest; the shape to mirror if open question 1 resolves to `CoreError::Config` rather than reported `Blocked`.
- `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs:79,140,156-161` + `D:\dev\projects\conductor\crates\conductor-cli\tests\cli_smoke.rs:186` — the coverage-table renderer and the smoke assertion that stdout contains `P-001` and `P-060`; the header/caption range label is layout-visible, so it must not name a span the rendered row set does not contain.

## Anti-patterns to avoid
- **A new lamp/status state, or red, for a malformed manifest** — `Blocked` is never red and no state ever collapses into `Fail` (per layout-templates §Primary content block 2; §Decisions Log — Motion trigger placement).
- **`alert()`/`confirm()`/a native OS toast/a new dialog for the load failure** — the styled `AlertDialog` is the only modal, and Conductor never emits a native OS toast (per layout-templates §Surface: desktop-webview — IA notes Global model; §Component — Footer (status strip)).
- **Emoji or bare color in machine-parseable (piped) output** — ASCII prefixes only there (per layout-templates §Surface: cli — IA notes, Pipe discipline).

## Contract bindings
- **Verdict/error wall (scope open question 1) ↔ arch/errors:** the decision selects which layouts component renders the failure — reported `Blocked` goes through §cli Primary content block 2 + the desktop hollow-ring lamp + the footer roll-up's `Blocked` count token; harness fault goes through §cli Footer / terminator + error output (stderr `error:`/`hint:`). Layouts is neutral on which, but both shapes are already fixed and one must be picked, not blended.
- **ASCII-prefix pairing ↔ a11y:** layouts states "never color-alone"; a11y derives the `NO_COLOR` / screen-reader conformance from it (per layout-templates §Surface: cli — Signature placement; §Notes).
- **Token names ↔ design:** layouts names `count-blocked` ↔ ANSI 60; design owns the value (per layout-templates §Surface: cli — IA notes, Multi-surface coordination).
- **Coverage-table output shape ↔ tests:** `cli_smoke.rs:186` pins `P-001`/`P-060` in stdout; any range-label change is a test-visible layout change (per layout-templates §Surface: cli — IA notes, Command model).

## Acceptance criteria contributions
1. (layouts) If a malformed/absent manifest reports `Blocked`, it renders as `•` + `[BLOCKED]` + ANSI 60 with the named precondition on its detail line and measurement columns `—`/null, never red and with no new lamp state (layout-templates §cli Primary content block 2; §desktop Primary content block 2).
2. (layouts) If it instead resolves as a harness fault, the message lands on **stderr** as `error: <short>` + `hint: <fix>`, sanitized and un-colorized when piped, with stdout left parseable (layout-templates §cli Footer / terminator + error output).
3. (layouts) The chunk adds no verb, screen, dialog or focusable control — cli verbs stay run|suite|report|preflight|coverage and desktop stays one frameless window in four run states; any addition requires a layout-templates amendment (layout-templates §cli Primary screens; §desktop Primary screens).
4. (layouts) No rendered range label (`COVERAGE P-001..P-060`, `CONDUCTOR suite P-001..P-060`, the coverage-table caption) may name a span wider than the rows actually rendered — scope open question 2's `coverage.rs` boundary decision must keep label and row set in agreement (layout-templates §desktop Wireframe — Run console (idle); §cli Output structure — `conductor suite`).

## Relevant amendment history
- **`2026-06-23-5-command-agent-run-harness`** (§cli Primary screens) — registered the `conductor preflight [--json]` verb as the readiness gate that exits 0 iff `ready:true` and non-zero on a **Blocked precondition**. Relevant because that verb is the existing precedent for "a named precondition, surfaced as `[BLOCKED]`, with a non-zero exit" — the exact shape this chunk's malformed-manifest path is being asked to follow.
- **`2026-06-23-line-oriented-output-rendering`** (§cli Primary screens · §cli tooling context) — registered `conductor coverage [--write]` (renders the static 60-P-ID matrix as a `comfy-table`; `--write` regenerates `coverage-matrix.md` at repo root). Relevant because that verb is the surface that renders the P-ID universe this chunk re-sources; the `--write` artifact is where a manifest-sourced set would eventually become visible, and it is `v2-03`'s boundary, not this chunk's.
- Both amendments establish the standing precedent: **a new cli verb is a layout-surface amendment (D-layout-surface routine)** — so if this chunk needs any new verb or flag to surface the manifest, that is an amendment, not a silent addition.
- (Not relevant: `2026-06-24-frameless-window-shell` titlebar-height reconcile — webview chrome only, untouched by this chunk.)
