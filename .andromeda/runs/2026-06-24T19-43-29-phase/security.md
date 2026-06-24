# security extract

## Relevance
relevant — touches error sanitization (cli stderr edge) and agent-mode self-obs logging output discipline

## Constraints
1. All external responses (conductor-cli stderr + run-report artifacts) must be sanitized: no stack traces, absolute filesystem paths, internal struct names, or library versions (security-plan §Error Handling §253).
2. Error format must never expose paths derived from canonicalized `CONDUCTOR_*` env-var handles or `ANDROMEDA_PULSE_DATA_DIR` to stderr or run-report artifacts (security-plan §Error Handling §269-275).
3. Field-allowlist redaction operates at subscriber-layer (processor stage), never only at the sink, so upstream failures cannot leak host paths via stderr (security-plan §Error Handling; obs-plan §606).
4. Stack traces surface only under `--debug`/`-v` verbosity flags; normal execution hides them (security-plan §Error Handling; design-system §cli "Error output").
5. `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path string labels (no validation boundary); they are JSON-escaped by serde_json at the value level only (security-plan §Input Validation final note per 2026-06-15 amendment).

## Patterns to follow
1. `thiserror` typed enums (EmitError, VerifyError, ContractMismatch, ConfigError) collapse to `anyhow` only at the conductor-cli / #[tauri::command] edges (security-plan §Error Handling §259-260).
2. Run-report artifacts (logs/agent-latest.jsonl self-obs stream, runs.db rows, per-run reports) record only verdict/state/identity fields (run_id, seed, scenario, p_ids, slo_tier, latency_ms, fingerprints) without leaking canonicalized paths or seam-crate struct names (security-plan §Error Handling §269-275).
3. Dual-sink self-obs: processor-layer redaction redacts fields, then sink is selected by mode (stderr pretty-print dev / file JSON in --agent-mode) so failures cannot leak via either path (obs-plan §3, §202, §606).

## Anti-patterns to avoid
1. NEVER expose stack traces, absolute file paths, or internal struct/field names to the operator via conductor-cli stderr or #[tauri::command] returns — sanitize at the anyhow edge (security-plan §Anti-Patterns §Logging §334).
2. NEVER let run-report artifacts (logs/agent-latest.jsonl, runs.db rows, JSONL journals) leak absolute host paths (canonicalized CONDUCTOR_* directories, ANDROMEDA_PULSE_DATA_DIR) or internal seam-crate struct names (security-plan §Anti-Patterns §Logging §335).
3. NEVER use the journal/report wall-clock stamps from tokio's virtual clock — use std::time::SystemTime/Instant so journal-relative SLO math stays correct (security-plan §Anti-Patterns §Logging §336).

## Contract bindings
obs-plan ↔ self-obs sink selection (CLI mode dual sink §3, §202); design-system ↔ CLI error format (error:/hint: shape, colorization, NO_COLOR/TERM=dumb §cli, §263/§268); scripts/agent-run.{sh,ps1} ↔ --agent-mode flag wiring (release-gate never-blocks proof); test-plan ↔ assert_cmd stderr capture vs. emission-journal artifact distinction (§3, §188)

## Acceptance criteria contributions
1. (security) `conductor-cli` stderr errors render sanitized with no absolute host paths, internal struct names, or stack traces under normal execution; `--debug`/`-v` only path surfaces traces.
2. (security) `logs/agent-latest.jsonl` (self-obs stream) field redaction is at subscriber-layer; canonicalized path values cannot leak via stderr or the JSON sink.
3. (security) `--agent-mode` flag forces Headless resolver (never blocks on TTY) and routes self-obs to file-only JSON (no pretty stderr), proving release-gate never-block invariant.
4. (security) `cargo audit` + `cargo deny check` pass (presentation deps from 2026-06-23 chunk: owo-colors/indicatif/comfy-table; deny.toml exceptions recorded: RUSTSEC-2025-0119 ignore + Zlib license allow per 2026-06-23 amendment).

## Relevant amendment history
**2026-06-15-structured-logging-stack:** obs identity env-handles `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path string labels (JSON-escaped by serde_json; no path/SQL/argv exposure); require **no** validation, distinct from `CONDUCTOR_*` *path* handles that canonicalize + bounds-check (clarifies Input Validation boundary scope for agent-mode logging).

**2026-06-23-line-oriented-output-rendering:** deny.toml exceptions recorded for presentation deps (owo-colors/indicatif/comfy-table): accept RUSTSEC-2025-0119 (unmaintained-but-non-vulnerable `number_prefix` via `indicatif`; no upstream fix) + allow Zlib license (`foldhash` via `rusqlite`→`hashbrown`; OSI+FSF permissive). Audit + deny green; no unresolved CVE.
