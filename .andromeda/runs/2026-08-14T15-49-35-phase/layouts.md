# layouts extract

## Relevance
Partial — no desktop-webview work (no new P-ID, catalog untouched, no matrix/report change); the cli surface rules bind only insofar as the capture is driven, reported, or terminated through `conductor`/`agent-run.sh` stdout.

## Constraints
- Any capture status printed to cli stdout must pair its color with an ASCII bracket prefix so the signal survives `NO_COLOR` / piping / redirection — color is never the sole channel (per layout-templates §Surface: cli — Signature placement, closing paragraph).
- Raw artifact data stays on stdout, human messages on stderr; error text is sanitized (no absolute host paths, no internal struct names, no stack traces outside `--debug`) — the surface half of this chunk's artifact-hygiene obligation (per layout-templates §Surface: cli — Component — Footer / terminator + error output).
- The capture artifact is announced by a single closing terminator line naming its destination, dimmed (ANSI 146 mapping) and left un-colorized when piped — no persistent footer, no banner (per layout-templates §Surface: cli — Component — Footer / terminator + error output).
- cli output structure is a parsed contract: adding a column to the 6-column results/SLO table (or conflating it with the 4-column `coverage` table) without a `--format` flag is a breaking change (per layout-templates §Surface: cli — IA notes → Command model; §Component — Primary content block 1).
- The headless/agent-driven path is never gated on an interactive prompt — an `inquire` gate on the capture path would silently break the release gate; `isatty` is checked first (per layout-templates §Surface: cli — IA notes → Headless invariant).
- The per-P-ID bracket-label set is closed at six; a run-level capture qualifier rides *outside* the lamp column (the `[ENVIRONMENT-SUSPECT]` / `not-conductors` Mode-cell precedent) and is neither a seventh lamp nor a sixth `ReportState` (per layout-templates §Surface: cli — Component — Primary content block 2).
- Layout-level security guardrail: the only backend→frontend live channel is the single Tauri `Channel` — no polling/SSE/URL surface — so a capture channel must not introduce an inbound/served surface on the webview side (per layout-templates §Surface: desktop-webview — IA notes → Live channel / Security guardrails).

## Patterns to follow
- Preflight readiness line shape — `preflight protocol 2024-11-05 tools 4/4 canary ok` with negotiated identifiers in the ID-cyan mapping and an `[OK]`/`[BLOCKED]` prefix; the existing home for a canary result on cli (per layout-templates §Surface: cli — Component — Header / banner; §Primary screens `conductor preflight [--json]`).
- Run-level non-lamp caption — printed once per run, above the verdict lines it qualifies, ANSI 246 reuse (no new token/ANSI entry), always-printed ASCII bracket label, omitted entirely when not applicable; the template for any capture-provenance qualifier (per layout-templates §Surface: cli — Output structure `conductor run <scenario>`).
- `[BLOCKED]` / non-result rendering — named precondition string carried in the row detail, measurement columns render `—` / null, never a red error; the shape for "capture unavailable / read-back unreachable" outcomes (per layout-templates §Surface: cli — Component — Primary content block 1 / block 2).
- Verb-noun, flat one level, help verb-prefixed — if the capture needs an entrypoint it is a registered verb or a flag on an existing one, not a nested subcommand (per layout-templates §Surface: cli — Component — Primary navigation (verb structure)).

## Anti-patterns to avoid
- Do not introduce a new bracket label / lamp state for the capture outcome (lamp set closed at six, `ReportState` at five) — use the run-level-qualifier lane instead (per layout-templates §cli Component — Primary content block 2).
- Do not bake a fresh literal (count, duration, path, fingerprint) into a documented sample caption — name the term set; a replacement literal simply re-stales (per layout-templates amendment `2026-08-13-dispatcher-determinism-goldens`).
- Do not mix diagnostic prose into stdout or emit emoji on the machine-parseable path — ASCII prefixes only there (per layout-templates §cli IA notes → Pipe discipline).

## Contract bindings
- **layouts ↔ obs/self-observation:** the cli stderr-sanitization rule (no host paths / internal struct names) is the surface expression of this chunk's redaction-boundary + field-allowlist obligation on the `logs/agent-latest.jsonl` capture — same rule, two channels (layout-templates §cli Footer / terminator + error output).
- **layouts ↔ tests/agent-run harness:** the "never block the headless path" invariant binds the capture to `scripts/agent-run.sh`'s 5-command `boot`/`run`/`status`/`cleanup`/`logs` shape (layout-templates §cli Primary screens; amendment `2026-06-23-5-command-agent-run-harness`).
- **layouts ↔ a11y:** not engaged this chunk (no new focusable elements, no dialog, no live region).

## Acceptance criteria contributions
- (layouts) Every capture status line on cli prints its ASCII bracket label alongside the color mapping and renders identically under `NO_COLOR` / piping (per layout-templates §Surface: cli — Signature placement).
- (layouts) The capture artifact's location is announced by one dimmed terminator line, un-colorized when piped, with raw data on stdout and human messages on stderr (per layout-templates §Surface: cli — Component — Footer / terminator + error output).
- (layouts) No seventh lamp/bracket state and no new column on the 6-column results/SLO table ship; any run-level capture qualifier rides outside the lamp column (per layout-templates §Surface: cli — Component — Primary content block 2 · §IA notes Command model).
- (layouts) The `conductor coverage` roll-up caption literal is unchanged — the catalog is untouched this chunk, so the `(N unbacked)` qualifier must not move (per layout-templates §Surface: cli — Primary screens `conductor coverage [--write]`).

## Relevant amendment history
- `2026-08-13-dispatcher-determinism-goldens` — re-based the `[ENVIRONMENT-SUSPECT]` sample caption to name the breaching phase + the sustained term SET rather than a duration literal. Why it matters here: this is the most recent precedent for a run-level, non-lamp cli qualifier and it establishes the standing rule that any capture-related caption names its term set instead of baking a literal that re-stales.
- `2026-08-09-sut-load-envelope` — introduced that run-level `[ENVIRONMENT-SUSPECT]` caption and pinned the enumeration of six per-P-ID labels as the *closed* set while stdout carries the qualifier outside it. Why: directly constrains where a capture/provenance signal may be rendered on cli.
- `2026-08-10-pulse-run-contract` (and the 11→10, 10→9 chain before it) — moved only the `(N unbacked)` roll-up qualifier when a scenario newly named a P-ID, and noted sibling docs that name the SET rather than the literal were correctly untouched. Why: this chunk drives no scenario and names no P-ID, so the mirrored inverse applies — the roll-up caption must NOT move, and a detector firing on it here would be a false positive.
- `2026-06-23-5-command-agent-run-harness` — registered `conductor preflight [--json]` and the `agent-run.sh` 5-command set in §cli Primary screens. Why: the operator recipe this chunk carries runs through `agent-run boot` / preflight, and the precedent is that any genuinely-new cli entrypoint gets registered in §cli Primary screens at wrap.
