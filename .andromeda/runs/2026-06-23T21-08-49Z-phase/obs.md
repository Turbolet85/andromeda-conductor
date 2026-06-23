# obs extract

## Relevance
Partial — the chunk's paused-state UI mirror is a telemetry signal (hold-point tracing event + `[HOLD]` status line); full agent-mode logging deferred.

## Constraints
- Per §3 Observability Harness Contract / Logging stack, all tracing spans (including hold-point events) must emit wall-clock `std::time::SystemTime` timestamps, never tokio virtual clock (§11 Project-specific bans: "NEVER use tokio's virtual clock")
- Per §6 Log Coverage / Required fields, hold-point self-obs lines must carry `run_id` correlation key on every JSON line (no W3C `traceparent`); no PII/redactable content in span attributes or prompt text
- Per §11 Anti-Patterns / Logs, hold-point prompts MUST NOT leak absolute host paths or internal struct names; use the `conductor-core::redact` allowlist + Display-not-Debug edge when capturing operator context
- Per §6 Log Coverage / Boundary-call wrappers, hold-point await transitions (paused/resumed) should emit bounded `tracing` info-level events with span attributes `hold_point_id`, `decision` (Go/NoGo), `allow_no_go` (boolean), avoiding per-user cardinality
- Per §11 Anti-Patterns / Spans, the hold-point resolver span name must be bounded and low-cardinality — suggest `pause.await` or `pause.decision` (follows `{module}.{operation}` pattern §2) — NOT per-p_id or per-prompt variants
- Per §3 Service identity, hold-point events inherit the command-handler's service context (`service.name`=`conductor`, `service.version`, `deployment.environment`); no new resource attributes needed

## Patterns to follow
- Reuse the CLI's existing `IsTerminal` gate (from ch3 `stdout_color()` primitive) to choose TTY vs non-TTY resolver paths; non-TTY never blocks (headless-never-blocks discipline)
- Mirror the Tauri paused-count titlebar design (Epoch 9) in the CLI via ch3's `indicatif` progress + `Lamp` mapping; `[HOLD]` ASCII prefix always present, color is optional tty-gated overlay (status-never-color-alone pattern from §1 design section)
- Emit hold-point state transitions as bounded `tracing::info!()` lines within the command-handler span context (plain `tracing`, no OTel SDK); structured JSON via `tracing-subscriber` JSON formatter reuses the self-obs infrastructure §3

## Anti-patterns to avoid
- NEVER introduce a second hold-point model or decision set — implement the one `PauseResolver` trait the core defined; no new `Decision` variants or verdict impacts
- NEVER log the raw, unsanitized prompt text — apply redaction via the allowlist + Display edge before emission (§11: "NEVER forward unsanitized panic/input")
- NEVER use per-p_id or per-prompt span names (would exceed bounded span-name set §11); keep hold-point observation low-cardinality with a fixed span name like `pause.await`

## Contract bindings
obs ↔ tests harness (per focus guide): tests consume structured JSON logs + status endpoint shape; the hold-await transition must be agent-parseable (JSON lines, no unstructured prompts leaked to stderr). obs ↔ design-system (per scope): `[HOLD]` status line + amber color gating obey design-system palette + layout-templates precedent from ch3.

## Acceptance criteria contributions
- "(obs) Hold-point await emits bounded `tracing` event with `run_id` + `hold_point_id` + `decision` (Go/NoGo) + `allow_no_go` (boolean); no raw prompt text in structured output."
- "(obs) Non-TTY hold resolver never blocks — auto-resolves via headless default; no interactive prompt written to pipe (no hang, no stray escape codes in agent logs)."
- "(obs) Status line `[HOLD]` prefix always present; color is tty-gated overlay — remains unambiguous under `NO_COLOR` or when piped."

## Relevant amendment history
(none) — isatty-gated operator-pause is outside the scope of amendments 2026-06-15 through 2026-06-18 (those covered structured-logging-stack foundations, redaction-model reconciliation, Run-report envelope schema alignment, and OTel SDK clarification). No prior hold-point instrumentation was amended.
