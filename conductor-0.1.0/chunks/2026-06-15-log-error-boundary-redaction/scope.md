# Scope — Log + error-boundary redaction

**Marker:** 2026-06-15-log-error-boundary-redaction
**Version:** conductor-0.1.0 · **Epoch:** 1 — Foundation (chunk 6/9)
**Working-route line:** _Log + error-boundary redaction — tracing-subscriber field-allowlist + anyhow-edge sanitization (no host-paths/struct-names/stack-traces)_

## What it builds
The artifact-hygiene **redaction layer** that composes on top of the structured-logging stack
(built in `2026-06-15-structured-logging-stack`) and at Conductor's error edges — the obs-plan's
`pii-scrubbing-wire` bootstrap phase, deferred to this chunk by the prior session's user-confirmed
sequencing (playbook rule, 2026-06-15). Two complementary mechanisms realize the universal
**artifact-hygiene invariant** ("never leak absolute host paths or internal struct names into
logs / run-report / `runs.db` — sanitize at the `anyhow` edge + the tracing-subscriber
field-allowlist"):

- **tracing-subscriber field-allowlist** — a redaction layer over the custom JSON subscriber so
  only an explicit allowlist of self-obs field names reaches the sink; non-allowlisted fields (and
  values matching host-path / internal-struct-name shapes) are dropped or masked before any line is
  written to stderr/file. This keeps absolute host paths and internal struct/field names out of the
  self-obs JSON log stream — including the `std::panic::set_hook` line (a panic payload must not
  leak a host path either).
- **anyhow-edge sanitization** — at the `conductor-cli` / `#[tauri::command]` binary edges (where
  typed `thiserror` per-seam enums collapse to `anyhow`), a sanitizing boundary so no stack traces,
  absolute host paths, or internal struct names reach the operator (cli stderr / Tauri command
  returns) — only the operator-facing `error:` / `hint:` shape. **(Lean scope — P4 decision: this
  chunk builds the reusable `sanitize_error` primitive, kept anyhow-free in `conductor-core`;
  rewiring the cli/tauri `main()`s to call it is deferred to the Epoch 8 CLI surface that owns error
  rendering — the 4-line `main` stubs have no error path yet.)**

The redaction primitive must be **reusable** by the (later) run-report artifact writers
(`<run_id>.md` / `runs.db` / JSONL journal, Epoch 6): this chunk builds the shared scrubber + wires
it into the surfaces that exist today (the obs log path + the cli/tauri error edges), designed so
the Epoch 6 artifact seams attach to the same primitive.

## Boundaries / non-goals
- **Self-obs + operator-facing artifact hygiene ONLY — not product-stream PII.** This redacts
  Conductor's OWN self-observation logs and operator-facing errors. It is NOT the product OTLP
  payload PII work (the seven P-047 categories / P-035 scrub across spans/logs/exceptions), which is
  a separate concern in Epoch 3 — Emission primitives (PII payload corpus) about what Conductor
  EMITS to Pulse `:4317`. Different stream, different chunk.
- **Run-report artifact writers are Epoch 6.** `runs.db` (rusqlite) / the Markdown run report / the
  JSONL emission journal do not exist yet. This chunk builds the redaction primitive and wires the
  EXISTING surfaces (self-obs logs + cli/tauri error edges); the artifact-writer wiring lands when
  those seams are built (Epoch 6), reusing this primitive — do not stand up the report seam here.
- **Does not rebuild the obs stack or the panic hook.** The subscriber, service-identity fields,
  `run_id`, and `std::panic::set_hook` already exist (prior chunk); this composes a redaction layer
  onto that writer, it does not re-stand-up the stack.
- **cli/tauri error-edge wiring deferred (lean scope — P4 decision).** The cli/tauri `main()`s are
  4-line stubs with no error path until clap + commands land in Epoch 8; this chunk builds + tests the
  reusable `sanitize_error` primitive and wires the **log** redaction path now, and the Epoch 8 CLI
  surface wires the primitive into the `error:` / `hint:` reporter. No `anyhow` dependency is added to
  any crate this chunk (the core sanitizer takes `&dyn std::error::Error` / `impl Display`).
- **No OTel SDK, no network sink.** Unchanged from the obs invariant — self-obs stays `tracing` JSON
  to stderr/file, local-only.

## Surfaces / contracts touched
- **Artifact-hygiene invariant** (CLAUDE.md Critical Warnings) — this chunk is its primary
  realization: "sanitize at the `anyhow` edge + the tracing-subscriber field-allowlist."
- **obs-plan §3 / §6** — the log JSON schema field-set is the source of the allowlist; obs-plan §3
  sequences this as the `pii-scrubbing-wire` bootstrap phase.
- **security-plan §Error Handling / §Security Anti-Patterns** — "Never leak stack traces, absolute
  host paths, or internal struct/field names to the operator (cli stderr / Tauri returns) or into
  run-report artifacts — sanitize at the edge + field-allowlist redaction." The verdict/error wall
  stays intact (`tonic::Status` / MCP errors remain typed verification inputs, never panics).
- **`conductor-core::obs`** — the subscriber init surface (candidate home for the field-allowlist
  layer); plus the `anyhow` edges in `conductor-cli` and `conductor-tauri`. Exact module path fixed
  by P3 codebase research.
- **Regression gate** — `cargo nextest run --workspace` (default profile), clippy, and audit/deny
  stay green with `Cargo.lock` un-drifted (any new dep passes the supply-chain gate; prefer
  std/existing deps where possible).

## Definition of done (intent level)
A self-obs log line carrying a non-allowlisted field or a value containing an absolute host path /
internal struct name is emitted **redacted** (dropped or masked), never raw; an error surfaced at
the cli/tauri edge shows the sanitized `error:` / `hint:` shape with no stack trace, host path, or
struct name; the redaction primitive is reusable by the future run-report writers; the regression
gate (nextest, clippy, audit/deny) stays green with the lock un-drifted.
