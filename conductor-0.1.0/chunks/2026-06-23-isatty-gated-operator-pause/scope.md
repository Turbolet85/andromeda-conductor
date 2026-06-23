# Scope — isatty-gated operator-pause

**Marker:** `2026-06-23-isatty-gated-operator-pause`
**Version:** conductor-0.1.0 · **Epoch 8 (CLI surface) ch4/5**
**Working entry:** _isatty-gated operator-pause — inquire confirm + paused-count spinner mirror, headless never blocks_

## What it builds

The **CLI interactive resolver** — a `conductor-cli` implementation of the `conductor_core::PauseResolver`
trait that surfaces the runtime-agnostic hold/resume mechanism (built in Epoch 5,
`2026-06-21-operator-pause-orchestration`) to a human at the terminal. The core already supplies the typed
`HoldPoint` (scenario · `p_id` · `step` · `prompt` · `allow_no_go`), the closed `Decision { Go, NoGo }`,
`resolve_hold(resolver, hold) -> HoldResolution`, and the `HeadlessResolver` never-block default. This chunk
ships the **interactive** half the core deferred — and the tty gate that chooses between them. Three pieces,
all behind a thin seam in `conductor-cli` (no core/seam model change):

1. **isatty gate (the resolver selector)** — at the CLI edge, detect whether the session is an interactive
   terminal (`std::io::IsTerminal` on stdin/stdout, the same primitive ch3's `stdout_color()` already uses).
   - **TTY** → drive the new interactive `inquire` resolver (below).
   - **non-TTY** (piped / agent / redirected) → **NEVER block** — delegate to `HeadlessResolver::proceed()`
     so the source-of-truth agent path auto-resolves and records the `Decision` rather than hanging on a
     prompt no human will answer. This realizes the "headless never blocks" CLI discipline at the seam the
     core's doc comment named as a deferred consumer.

2. **`inquire` confirm prompt** — for each `HoldPoint`, the interactive resolver renders the hold's
   human-readable `prompt` (the non-Conductor action to perform/observe — e.g. *"Restart the Pulse process,
   then confirm"*) plus its scenario / `p_id` / `step` context, and presents a go/no-go **confirm** via the
   `inquire` crate, mapping the answer to `Decision::Go` / `Decision::NoGo`. `allow_no_go == false` renders a
   proceed-only acknowledgment (no decline branch offered). The resolver returns the `Decision`; the existing
   `resolve_hold` records the (redacted) `HoldResolution`.

3. **paused-count spinner mirror** — while a hold awaits operator input, the run/suite progress indicator
   (ch3's `indicatif` render seam) reflects the **paused** state — the CLI mirror of the Tauri paused-count
   titlebar hold-point signature that lands in Epoch 9. The hold's phase-line carries the `[HOLD]` ASCII
   prefix **always** (color is a tty-gated amber overlay, never the sole encoder), reusing ch3's `Lamp`→line
   mapping (verdict-first lamp precedence already established).

The interactive resolver is the **sibling** of the Epoch-9 Tauri AlertDialog go/no-go dialog — same core
`PauseResolver`/`Decision` vocabulary, different medium. It MUST NOT introduce a second hold model or a second
decision set; it implements the one trait the core defined.

## Surfaces / contracts touched

- **`conductor-cli`** (owner) — a new interactive `PauseResolver` impl (a new module, e.g. `pause.rs`, or a
  resolver seam alongside `render.rs`) + the tty-gated selector that picks interactive-vs-headless, wired into
  the run/suite execution path at the point a `HoldPoint` is encountered. The precise await-site (where in
  `pipeline.rs` / `commands/{run,suite}.rs` a hold is surfaced today, vs. introduced here) is the **open
  integration question** for planning (P4).
- **Reads, no model change** — `conductor-core::pause` (`HoldPoint`, `Decision`, `HoldResolution`,
  `PauseResolver`, `resolve_hold`, `HeadlessResolver`); ch3's render seam (`stdout_color()` tty-gate + the
  `indicatif` progress + the `Lamp`→`[HOLD]` line mapping) is reused, not reinvented.
- **New dependency** — `inquire` (terminal confirm prompt) enters `Cargo.lock`; `cargo audit` + `cargo deny`
  must stay green and the lock committed un-drifted (the supply-chain gate, parallel to ch3's three new deps).
- **Design / layout** — `design-system.md` amber/HOLD palette → terminal-color mapping for the hold line;
  `layout-templates.md` §cli operator-pause line/prompt layout (precedented by ch3's status-line amendment).
- **a11y** — the prompt + paused indicator obey "status is never color-alone": `[HOLD]` ASCII prefix (+
  label) is always present; color is the optional tty-gated overlay. The interactive prompt is keyboard-driven
  (`inquire` default).
- **Obs** — the hold await may emit a bounded `tracing` event for the paused/resumed transition (the
  `run_id`-tagged self-obs line); it MUST NOT stamp from the virtual clock and MUST NOT leak the raw prompt
  (redaction edge). Full agent-mode structured logging is ch5.

## Boundaries (explicitly OUT — deferred within/after Epoch 8)

- **Not the core hold mechanism** — `HoldPoint` / `Decision` / `resolve_hold` / `HeadlessResolver` already
  exist (Epoch 5). This chunk adds the **interactive resolver + tty gate + paused mirror** only; zero new
  core types or variants.
- **Not ch5 (sanitized stderr + agent-mode logging)** — the explicit `--agent-mode` `error:`/`hint:` stderr
  format and the JSON-to-file (`logs/agent-latest.jsonl`) journal are ch5. ch4 uses **`IsTerminal` detection**
  as the never-block gate (and honors an existing `--agent-mode`/non-tty signal if present), but does not
  build the structured agent-mode log format.
- **Not the Tauri go/no-go UI** — the Epoch-9 AlertDialog gating each committed step + the paused-count
  titlebar freeze/tint/resume signature are a separate surface; this is the CLI sibling only.
- **Not UI automation / Pulse process management** — stated project non-goals. The resolver *asks* the
  operator to perform the non-Conductor action; Conductor never performs it (e.g. it never restarts Pulse).
- **Zero verdict/report MODEL change** — no new `Verdict`/`ReportState`/`Decision` variants, no new
  emission/verify/scheduling logic (consistent with the zero-model-change chunk pattern). Reads existing
  `pause` types only.

## Acceptance intent (anchors P5 validation-1)

- **TTY path** — a `HoldPoint` presents an `inquire` confirm showing the (redacted) non-Conductor action +
  scenario/`p_id`; `Go` resumes the run, `No-Go` returns `Decision::NoGo` when `allow_no_go`; an
  `allow_no_go == false` hold offers proceed-only.
- **non-TTY path** — the resolver **never blocks**: piped/agent runs auto-resolve via the headless default and
  record the `Decision`; no interactive prompt is written to a pipe (no hang, no stray escape codes).
- **Status never color-alone** — the hold phase-line shows `[HOLD]` with color stripped (non-tty / `NO_COLOR`)
  and remains unambiguous; the paused indicator mirrors the hold state.
- **Determinism preserved** — same scenario + seed with auto-resolution ⇒ identical emission-stream shape as a
  run with no holds (the operator pause is wall-clock-only, outside the seeded virtual clock); tests drive the
  resolver non-interactively (auto-headless or an injected stub resolver), never a real TTY wait, honoring the
  zero-retry bar.
- **Supply chain + gates** — `inquire` audit/deny clean; `Cargo.lock` updated + committed; workspace tests
  green; clippy `-D warnings` clean; no host-path / internal-struct-name leak through any rendered prompt or
  recorded outcome.
