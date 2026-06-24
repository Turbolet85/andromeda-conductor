# Scope — Sanitized stderr + agent-mode logging

**Marker:** `2026-06-24-sanitized-stderr-agent-mode-logging`
**Version:** conductor-0.1.0 · **Epoch 8 (CLI surface) ch5/5** (closes the epoch)
**Crate:** `conductor-cli` (over the existing `conductor-core`/obs tracing layer) — **zero core/seam model change expected**

## What it builds

The CLI's machine-facing output discipline — the final Epoch-8 chunk that makes `conductor-cli` cleanly
agent-drivable and pipe-safe. Three tied pieces over the layer that the `2026-06-15-structured-logging-stack`
+ `…-log-error-boundary-redaction` chunks already built (tracing-subscriber JSON, service-identity + `run_id`,
`std::panic::set_hook`, field-allowlist redaction):

1. **`--agent-mode` global flag** (clap, on `conductor-cli`). When set it:
   - **triggers agent mode via the `CONDUCTOR_AGENT_MODE` env handle** (obs-plan §3). Resolved at P4 (D4):
     the env is a **read-only trigger** — `agent_mode = flag || env-set` — and main does NOT write it (avoids
     edition-2024 `unsafe std::env::set_var`); `scripts/agent-run.{sh,ps1}` may export the env OR pass the flag.
     Same observable mode as "sets …=1 internally", no `unsafe`;
   - **forces the self-obs sink to JSON-only to file** `logs/agent-latest.jsonl` (relative to
     `CONDUCTOR_RUNS_DIR`) — no pretty-print to stderr (obs-plan §3 dual-sink; lines 82/205/490);
   - **(folded PREREQ)** forces the **Headless** resolver in `CliResolver::select()` so the operator-pause
     **never blocks regardless of TTY** — `2026-06-23-isatty-gated-operator-pause` gated `select()` on
     `IsTerminal` only; agent-mode must override the isatty gate (the release-gate-never-blocks invariant).

2. **Sanitized stderr error format** at the `conductor-cli` anyhow edge — the two-part
   `error: <short>` + contextual detail + `hint: <fix>` shape (design-system §cli "Error output";
   layout-templates §263), **sanitized**: no absolute host paths / internal struct names / stack traces
   (security-plan §Error Handling §334). Never colorized when piped; **stack traces only under `--debug`/`-v`**.

3. **Dual-sink self-obs wiring** — the `tracing` self-observation stream's sink is selected by mode:
   stderr pretty-print in dev mode, file `logs/agent-latest.jsonl` in `--agent-mode` (obs-plan §3, §202).
   Redaction stays at the **processor (subscriber-layer)** stage, never only at the sink (obs-plan §606),
   so an upstream failure cannot leak host paths via stderr.

## Boundaries (what it does NOT touch)

- **NOT the per-run emission journal.** `logs/agent-latest.jsonl` (self-obs `tracing` stream: per-line base
  fields `timestamp_ms`/`level`/`target`/service-identity/`run_id`) is a **SEPARATE artifact** from
  `runs/<run_id>.jsonl` (the SLO ground-truth Run-report envelope). The two schemas must not be conflated
  (test-plan-amendments §7; test-plan §188). This chunk wires the self-obs sink only — it does not alter the
  emission-journal writer.
- **No core/timeline/emit/verify/report MODEL change** — `Verdict`/`ReportState`, the journal schema, the
  scenario model, runs.db are all unchanged. (Clarified at P4:) the one core touch is a mechanical **API
  extension** — the shared `conductor_core::init_observability` entry gains a `sink: ObsSink` param so both
  shells select stderr-vs-file identically; this ripples to the `conductor-tauri` caller, updated to pass
  `ObsSink::Stderr` to keep compiling (its own `logs/conductor-tauri.jsonl` file sink is Epoch-9). Neither is a
  verdict/journal model change. The resolver work reuses the existing `CliResolver` enum.
- **Not Tauri.** `logs/conductor-tauri.jsonl` + the Tauri error-dialog edge are Epoch 9 — out of scope here.
- **Not a new redaction policy.** The field-allowlist + anyhow-edge sanitization already exist; this chunk
  *applies* them at the cli error edge + the agent-mode sink, it does not redesign redaction.
- **stdout stays data-only.** Raw artifact data (results table, run-report path) on stdout; all human/error
  messages on stderr — never mixed (design-system §352; layout-templates §263).

## Surfaces / contracts touched

- **obs-plan §3** — dual sink (CLI), `--agent-mode` flag + `CONDUCTOR_AGENT_MODE=1`, `logs/agent-latest.jsonl`
  path (relative to `CONDUCTOR_RUNS_DIR`), redaction-at-processor, self-obs base-field set.
- **security-plan §Error Handling** — sanitize at the `anyhow` edge: no stack traces / absolute host paths /
  internal struct or field names to cli stderr or run-report artifacts.
- **design-system §cli "Error output"** + **layout-templates §263/§268** — `error:`/`hint:` format, never
  colorized when piped, ANSI auto-stripped, `NO_COLOR`/`TERM=dumb` honored, stdout/stderr separation.
- **test-plan §3** — `assert_cmd` captures cli stderr (`.get_output().stderr`); self-obs stream is asserted as
  a distinct artifact from the emission journal.
- **`scripts/agent-run.{sh,ps1}`** — the headless harness invokes the cli; `--agent-mode` (or the env)
  is its release-gate path (the never-block proof). Confirm the boot/run/logs verbs surface the sink.

## Acceptance intent (full criteria synthesized in plan.md)

- `--agent-mode` present on the cli; sets `CONDUCTOR_AGENT_MODE=1`; routes self-obs JSON to
  `logs/agent-latest.jsonl` (no pretty stderr); forces Headless resolver (never blocks off **or** on a TTY).
- A harness-fault error renders as sanitized `error:`/`hint:` on stderr with zero host-path/struct-name/
  stack-trace leakage; `--debug`/`-v` is the only path that surfaces a trace.
- The self-obs sink and the emission journal remain distinct files with distinct schemas.
- Gates green: `conductor-cli` nextest + workspace nextest, doctest, clippy `-D warnings`,
  `cargo audit` + `cargo deny check`; `agent-run run` completes without hanging.
