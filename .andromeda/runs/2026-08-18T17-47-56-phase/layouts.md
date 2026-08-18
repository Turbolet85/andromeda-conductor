# layouts extract

## Relevance
Partial — cli surface only (`conductor run` / `agent-run.sh` live-leg output, results/SLO table, verdict lines); no desktop-webview component is created or moved by this chunk.

## Constraints
- The cli output structure is a parse contract for downstream agents — adding a column to the results/SLO table without a `--format` flag is a breaking change (per layout-templates §Surface: cli → IA notes → Command model). The re-calibration clause may move cell *values*, never the table shape.
- The results/SLO table is exactly 6 columns (P-ID · scenario · state · slo_tier · latency_ms · fingerprints), terminal-width-detected, and is a **different** table from the 4-column `conductor coverage` matrix (per §cli Component — Primary content block 1). Any `slo_tier` re-check lands in that one column, whose value set is closed at `<5s` / `<20s` / `<90s`.
- The per-P-ID verdict / report-state bracket-label set is **closed at six** (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`); a run-level qualifier may ride outside the lamp column but is neither a seventh lamp nor a sixth `ReportState` (per §cli Component — Primary content block 2). A check retiring to declare-only must map into that closed set, not extend it.
- `Blocked` rows render measurement columns as `—` / null with a named precondition string and are **never** a red error; `KnownResidual` is likewise never red (per §cli Component — Primary content block 2). This chunk's env-recipe failures (sidecar/PATH, `ANDROMEDA_PULSE_MCP_ENABLED`, data-dir match) are `Blocked` preconditions, not Fails.
- Headless invariant: `inquire` operator-pause prompts are `isatty`-gated and the `scripts/agent-run.sh` source-of-truth path is never blocked on a prompt (per §cli IA notes → Headless invariant; §cli Component — Hero / signature output). Operator-gated live legs run the TTY path; the headless gate must stay unblocked.
- Pipe discipline: raw artifact data on stdout, human messages on stderr, ANSI auto-stripped when piped, `NO_COLOR`/`TERM=dumb` honored, every status color paired with its ASCII prefix (per §cli IA notes → Pipe discipline; §cli Signature placement note). Harvested leg evidence must remain readable with color stripped.
- Same tokens by name across surfaces, adapted not forked — desktop CSS vars ↔ cli ANSI ↔ Markdown artifact (per §cli IA notes → Multi-surface coordination). A moved `slo_tier` or verdict must read identically in the cli table, the `<run_id>.md` artifact, and the webview matrix row.

## Patterns to follow
- The fixed `conductor run <scenario>` transcript shape: header (`CONDUCTOR run <scenario> seed <n>`) → preflight readiness line (protocol / tools / canary, `[OK]`|`[BLOCKED]` prefix) → `→ phase` line → `indicatif` heartbeat → `[HOLD]` stop + confirm → per-P-ID verdict lines → `comfy-table` summary caption → `run report → runs/<run_id>.md` terminator (per §cli Output structure — `conductor run <scenario>`).
- The preflight readiness gate printed before any scenario trusts read-back, exiting non-zero on a Blocked precondition (per §cli Primary screens → `conductor preflight [--json]`) — the natural surface for this chunk's five-item operator recipe preconditions.
- The `Blocked` precondition-string form already templated with this chunk's exact env triple (`precondition: mcp-server feature + ANDROMEDA_PULSE_MCP_ENABLED + matching data-dir`) (per §cli Component — Primary content block 2).
- The `[RESIDUAL]` line + `residual: … until {named fix}` detail as the shape for a measured, pre-accepted gap (per §cli Component — Primary content block 2) — the rendering precedent if a token check's surface cannot carry it. Whether the shipped cli already emits these six shapes is research's question.
- The run-level `[ENVIRONMENT-SUSPECT]` caption printed once per run **above** the verdict lines, naming the breaching emitting phase and the sustained term it left, omitted when in-envelope (per §cli Output structure — `conductor run`); a sustained-emission live leg can trip it.

## Anti-patterns to avoid
- Do not introduce a seventh bracket label, a new lamp state, or an extra state column to express a declare-only retirement, an ungradeable token, or a "not carried by this surface" outcome — closed set, qualifier rides outside the lamp column (per §cli Component — Primary content block 2; the `not-conductors` Mode-cell precedent in §Primary content block 1).
- Do not conflate the 6-column results/SLO table with the 4-column `conductor coverage` matrix, and do not widen either (per §cli Component — Primary content block 1).
- Do not emit emoji or un-stripped ANSI into machine-parseable / piped leg evidence, and do not mix stdout data with stderr messages (per §cli IA notes → Pipe discipline; §cli Component — Footer / terminator + error output).

## Contract bindings
- **layouts ↔ obs/evidence:** the terminator line `run report → runs/<run_id>.md` (per §cli Component — Footer / terminator) names the same artifact this chunk's evidence expectations retain (`runs/<run_id>.jsonl` + `runs.db`); the layout's artifact-path terminator and the journal path must denote the same run.
- **layouts ↔ a11y / NO_COLOR:** the "color never alone, always an ASCII prefix" pairing is stated as a layout requirement from which a11y / `NO_COLOR` conformance derives (per §cli Signature placement note; §Notes).
- **layouts ↔ design:** cli status colors are the by-name token mappings (ANSI 114 / 179 / 203 / 60 / 117 / 246), owned by design and mirrored here (per §cli IA notes → Multi-surface coordination).

## Acceptance criteria contributions
- (layouts) The live-leg transcript follows the documented `conductor run` structure — header + preflight readiness line, phase line, heartbeat, per-P-ID verdict lines, 6-column summary caption, artifact-path terminator (per layout-templates §Surface: cli — Output structure `conductor run <scenario>`).
- (layouts) Any re-calibrated `slo_tier` renders as one of `<5s`/`<20s`/`<90s` in the results/SLO table's slo_tier column, with no column added or removed (per layout-templates §Surface: cli — Component Primary content block 1; §IA notes Command model).
- (layouts) The declare-only / ungradeable-token disposition renders inside the closed six-label per-P-ID set (or as a run-level qualifier outside the lamp column) — no seventh label, no red `Blocked`/`Residual` (per layout-templates §Surface: cli — Component Primary content block 2).
- (layouts) Leg evidence captured from cli stdout stays legible with ANSI stripped: every state cell readable from its ASCII bracket prefix alone (per layout-templates §Surface: cli — IA notes Pipe discipline).

## Relevant amendment history
- **2026-08-16-fingerprint-storm-live-proof — coverage roll-up caption DE-LITERALIZED** (§cli Primary screens, `conductor coverage`): the sample caption now reads `43 auto (N unbacked)` after the same literal went stale three times (11→10→9→8) as successive chunks named P-IDs in scenario TOMLs. Directly relevant: if this chunk's scenario TOMLs name additional auto-classified P-IDs, `UNBACKED_AUTO` moves again — but the placeholder form means **no layout amendment is owed** for a count move; only a rule change would be.
- **2026-08-09-sut-load-envelope — cli run-level load-envelope caption** and **2026-08-13-dispatcher-determinism-goldens — caption re-based**: established `[ENVIRONMENT-SUSPECT]` (ANSI 246) as a run-level, non-lamp qualifier printed once above the verdict lines, naming the breaching emitting phase and the sustained term left (storm window / sustained rate), and re-affirmed that the lamp set stays six. Relevant because this family's sustained baseline emission is exactly the class that can trip the classifier during a live leg.
- **2026-08-09-out-of-scope-classification-treatment** (§cli Primary content block 1): recorded the results/SLO-vs-coverage table split (6 vs 4 columns) after the two were conflated — the reason the constraint above is stated explicitly.
