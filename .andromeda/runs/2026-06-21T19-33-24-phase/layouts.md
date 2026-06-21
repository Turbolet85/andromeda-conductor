# layouts extract

## Relevance
Partial — Markdown report is a surface, but the chunk focuses on text rendering, not layout structure.

## Constraints
- Layout templates §Surface: desktop-webview | cli — this chunk targets a *third* surface (Markdown artifact) not covered by layout-templates.md
- Report-state lamp convention must pair ASCII prefix with status (layout-templates §Component — Primary content block 2 / §Verdict-first lamp precedence) per status-never-color-alone invariant
- The Markdown surface inherits the five report states (`Pass` / `Fail` / `CalibrationRegion` → `[HOLD]` / `ManualCheck` / `KnownResidual` / `Blocked`) from the verdict-first lamp rule (layout-templates §Signature placement & Decisions Log)

## Patterns to follow
- Verdict-first lamp precedence (desktop/cli both use: verdict when present, else state) — the Markdown surface establishes this as a *shared reusable helper* (per scope intent: "precedence logic should be a shared, reusable helper")
- ASCII `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefix encoding (no color in Markdown, so the prefix IS the status) — mirrors desktop Display role + cli ANSI mapping
- Blocked-row null rule (layout-templates §Component — Primary content block 2): measurement columns render as absent marker (em-dash), never `null` or struct name

## Anti-patterns to avoid
- Never emit a CalibrationRegion row as `[MANUAL]` — it is `[HOLD]` under verdict-first precedence
- Never colorize the Markdown artifact (no color available; relay status exclusively through the ASCII prefix)
- Never include absolute host paths, internal struct names, or stack traces in the `.md` artifact

## Contract bindings
- Report envelope shape (`RunRecord`) ↔ envelope-serializer chunk (already defined, not owned by this chunk)
- Verdict-first lamp helper ↔ follow-up coverage-matrix (ch4), cli (Epoch 8), desktop (Epoch 9) consumers
- Artifact hygiene (no host-path / struct-name leak) ↔ CLAUDE.md universal invariant

## Acceptance criteria contributions
- (layouts) Verdict-first lamp precedence helper is reusable across Markdown / coverage-matrix / cli / desktop surfaces; CalibrationRegion always renders as `[HOLD]`, never `[MANUAL]`.
- (layouts) Every verdict/state lamp is paired with ASCII `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefix; status is never encoding-color-alone in the Markdown.
- (layouts) Blocked rows render measurement columns as em-dash / absent marker; no `null` string or struct name leaks into the artifact.

## Relevant amendment history
(none)