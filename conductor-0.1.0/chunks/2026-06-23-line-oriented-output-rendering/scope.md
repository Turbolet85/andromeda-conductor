# Scope — Line-oriented output rendering

**Marker:** `2026-06-23-line-oriented-output-rendering`
**Version:** conductor-0.1.0 · **Epoch 8 (CLI surface) ch3/5**
**Working entry:** _Line-oriented output rendering — owo-colors/indicatif/comfy-table status lines + coverage table_

## What it builds
A presentation layer for the `conductor` CLI that renders the ALREADY-typed outcomes
(`Verdict`/`ReportState`/`Lamp`, `RunRecord`, `ReadyState`, the 60-P-ID coverage matrix) as human-readable
terminal output. Three new dev-stack crates land here, each behind a thin render seam in `conductor-cli`:

1. **owo-colors** — colorized **status lines**. Color is terminal-gated (`if_supports_color` / tty
   auto-detection) and is ALWAYS paired with the ASCII status prefix — the line carries its meaning with color
   stripped. The pinned prefixes are `[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]` (the universal "status is
   never color-alone" invariant; color encodes run state, never decoration).
2. **indicatif** — non-interactive **progress feedback** (spinner / counter) during `conductor run` and
   `conductor suite` execution, so a live run shows per-scenario advance rather than a silent pause.
3. **comfy-table** — TWO structurally-distinct tables (planning split what this scope first conflated): (a) the
   **per-run results table** for `conductor suite` / `conductor report` — one row per `RunRecord` (P-IDs ·
   scenario · verdict/report-state lamp · slo_tier · latency_ms · fingerprints), blocked rows em-dashing the
   never-measured cells; and (b) the **static coverage matrix** — all 60 P-IDs (P-001..P-060) by coverage mode
   (auto / drive-observe / static-only), the definition-of-done classification, which carries NO per-run lamp.
   The static matrix gets a new **`conductor coverage`** verb (P4 decision D1) that also regenerates
   `coverage-matrix.md` at the repo root (a layout-templates amendment, precedented by ch2's `preflight` verb).
   `conductor report` switches its stdout from raw Markdown to the colored results table (P4 decision D2); the
   on-disk `<run_id>.md` stays Markdown.

The CLI line surface is the **sibling of the existing Markdown run report** — same canonical states, different
medium. It MUST mirror the existing state→lamp mapping (verdict-first lamp precedence: a CalibrationRegion row
renders `[HOLD]`, not Manual) already realized in `conductor-report`'s Markdown renderer; it does not invent a
second classification.

## Surfaces / contracts touched
- **`conductor-cli`** (owner) — a new render/output module wired into the output paths of the four existing
  verbs: `run` · `suite` · `report` · `preflight`. The verbs already exist (ch1/ch2); this chunk gives them a
  rendered surface in place of whatever minimal/None output they emit today.
- **Reads, no model change** — `conductor-core` (`Verdict`/`ReportState`/`Lamp` + coverage-matrix types),
  `conductor-report` (`RunRecord` + the Markdown-report row shape / lamp mapping to mirror), `conductor-verify`
  (`ReadyState` for the `preflight` render).
- **Design / layout** — `design-system.md` palette → terminal-color mapping for the lamp colors;
  `layout-templates.md` §cli Primary screens (the per-line layout; recently amended to add `conductor
  preflight` + agent-run stage flags).
- **a11y** — the "never color-alone" rule is the a11y contract on this surface: ASCII prefix (+ glyph/label as
  applicable) is always present; color is an optional, tty-gated overlay over the prefix.
- **Supply chain** — three new crates enter `Cargo.lock`; `cargo audit` + `cargo deny` must stay green and the
  lock committed un-drifted.

## Boundaries (explicitly OUT — deferred within Epoch 8)
- **ch4 (isatty-gated operator-pause)** — the INTERACTIVE pause prompt (`inquire` confirm + paused-count
  spinner mirror, headless-never-blocks) is ch4. ch3 builds only the non-interactive run-progress spinner; it
  does not add an interactive prompt.
- **ch5 (sanitized stderr + agent-mode logging)** — the explicit agent-mode `error:`/`hint:` stderr format and
  the JSON-to-file logging path are ch5. ch3 relies on owo-colors' built-in tty auto-detection to suppress
  color in piped/non-tty output (so agent/piped output is not corrupted with escape codes) but does not build
  the structured agent-mode log format.
- **Zero verdict/report MODEL change** — no new variants, no new emission/verify/scheduling logic (consistent
  with the recent zero-model-change chunk pattern). Rendering reads existing types only.
- **Desktop (Tauri) rendering** — Epoch 9, a separate surface; not touched here.

## Acceptance intent (anchors P5 validation-1)
- Status lines render with the ASCII prefix ALWAYS present; with color stripped (non-tty / `NO_COLOR`) the line
  is still unambiguous — color is never the sole encoder.
- `conductor coverage` renders all 60 P-IDs (zero gaps) with coverage mode (the static classification — no
  per-run lamp); the per-run results table carries the lamp/state/latency per `RunRecord`.
- `run`/`suite` show live per-scenario progress feedback; `report` renders the per-run summary table;
  `preflight` renders the readiness result as lines.
- Piped / non-tty output contains no raw escape sequences (owo-colors auto-detection); full agent-mode is ch5.
- New deps audit/deny clean; `Cargo.lock` updated + committed; workspace tests green; clippy `-D warnings` clean.
