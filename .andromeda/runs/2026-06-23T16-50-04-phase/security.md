# security extract

## Relevance
Relevant — chunk opens the CLI surface and wires existing engine seams (timeline, emit, verify, report) behind run/suite/report verbs, crossing multiple security boundaries (input validation on CLI args/env-handles, OTLP egress, MCP read-back, JSONL/database persistence).

## Constraints
1. Canonicalize + bounds-check `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED` env-handles at the `conductor-cli` binary edge with `std::fs::canonicalize` before any read/write (security plan §Input Validation — path traversal defense; env handles sit OUTSIDE garde's struct validation).
2. All scenario-config deserialization via `Scenario::from_toml_str` MUST validate with garde `#[derive(Validate)]` + `#[garde(custom)]` cross-field rules (error fraction ∈ [0,1], p50≤p95≤p99, non-negative durations, sane ramp factors) before scenario execution — failed validation → `ConfigError` harness fault, never silent bypass (security plan §Input Validation).
3. `conductor-cli` binary edge MUST sanitize all error output — no stack traces, absolute host paths (canonicalized `CONDUCTOR_*` directories), internal struct/field names, or library versions exposed to operator stderr (security plan §Error Handling — external surface sanitization).
4. Run-report artifacts (`<run_id>.jsonl`, `<run_id>.md`, `runs.db` rows) MUST NOT leak absolute host paths or internal seam-crate struct names; record only verdict/state/identity fields (`run_id`, `seed`, `scenario`, `p_ids`, slo_tier, latency_ms, fingerprints) and populate only identity fields (with precondition string) when `Blocked` (security plan §Error Handling — artifact sanitization).
5. Parameterized SQL for `runs.db` writes — use rusqlite 0.38.0 bound parameters for `run_id`, `seed`, fingerprint-JSON1 values; never string-formatted SQL, despite self-generated synthetic data (security plan §Input Validation — defense-in-depth rule).
6. MCP read-back stdout decoding MUST use bounded protobuf recursion + treat `tonic::Status` and MCP error responses as first-class typed verification inputs, never panics; empty canary round-trip → `Blocked` state, never false pass (security plan §Input Validation, Threat Model Summary §Attack surface — read-back vector; prevents protobuf-decode DoS, RUSTSEC-2024-0437).
7. `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` (obs identity labels) are non-path string values stamped into self-observation JSON; require NO validation (distinct from the `CONDUCTOR_*` path handles above) (security plan §Input Validation amendment 2026-06-15).

## Patterns to follow
1. Env-handle canonicalization at the CLI edge (conductor-cli main.rs): `std::fs::canonicalize` + explicit type/existence check for directory/file, reject before any downstream read/write or manifest parse (security plan §Input Validation table, Input Validation anti-patterns entry 1).
2. Scenario config validation at deserialization time: garde `#[derive(Validate)]` on the Scenario struct (already wired in the scenario seam via prior chunks), rejected parse → `ConfigError` harness fault routed through verdict/error wall (security plan §Input Validation table).
3. Error edge type erasure: internal `thiserror` enums (`ConfigError`, `VerifyError`, `ContractMismatch`) collapse to `anyhow` ONLY at the `conductor-cli` / `#[tauri::command]` boundary; seam-crate logic stays typed (security plan §Error Handling — Conventions: Error handling).
4. Artifact redaction: run reports exclude absolute paths and internal names; run_id/seed/scenario/p_ids/slo_tier/latency/fingerprints only; when `Blocked`, identity fields + precondition string (security plan §Error Handling — Run-report artifact sanitization).

## Anti-patterns to avoid
1. NEVER interpolate `CONDUCTOR_*` env path handles or any operator-supplied value into argv / shell commands — pass via builders (`.env(...)` for child spawn), reject metacharacters first (security plan §Security Anti-Patterns § Input entry 2; rmcp STDIO command/argument-injection, CVE-2026-30623).
2. NEVER expose internal error details (stack traces, file paths, internal struct names, library versions) in operator-facing stderr or run-report artifacts (security plan §Security Anti-Patterns § Logging + API entries).
3. NEVER let malformed child/transport input panic — `tonic::Status` and MCP error responses are typed verification inputs for the verdict wall; panics corrupt run classification (security plan §Security Anti-Patterns § Universal entry 8).

## Contract bindings
- **Input Validation ↔ Scenario Seam:** scenario-config boundary depends on garde `#[derive(Validate)]` being co-located with serde structs in the scenario seam crate (per security plan §Input Validation table, bootstrap phase input-validation-library-install).
- **Error Handling ↔ Report Seam:** artifact sanitization depends on `conductor-report` seam never writing absolute paths or internal struct names to `runs.db`/JSONL/Markdown (security plan §Error Handling — artifact sanitization).
- **Env-handle Validation ↔ CLI Edge:** path canonicalization/bounds-check occur at `conductor-cli` main entry, before any seam-crate code receives paths (security plan §Input Validation anti-patterns entry 1 — outside garde struct validation).

## Acceptance criteria contributions
1. (security) All `CONDUCTOR_*` path env-handles validated at CLI edge: `std::fs::canonicalize` + type/existence check in `conductor-cli` main.rs before any read/write (guards path traversal per security plan §Input Validation).
2. (security) Scenario config parsed via garde-validated `Scenario::from_toml_str` (no unvalidated serde deserialize); failed validation → `ConfigError` harness fault, never silent (guards config-boundary injection per security plan §Input Validation).
3. (security) Binary error output (conductor-cli stderr) sanitized: no absolute paths, internal struct/field names, or library versions; `anyhow` Display only (guards error-disclosure per security plan §Error Handling).
4. (security) Run-report artifacts (`runs.db`, JSONL, Markdown) record verdict/state/identity fields only; no absolute paths or internal seam-crate struct names (guards artifact-leakage per security plan §Error Handling).
5. (security) `runs.db` writes use rusqlite bound parameters for `run_id`, `seed`, fingerprint-JSON1 — no string-formatted SQL (defense-in-depth per security plan §Input Validation).
6. (security) MCP read-back decoding treats `tonic::Status` and tool-call results as typed verification inputs; empty canary → `Blocked`, never false pass (guards protobuf-decode DoS per security plan §Input Validation).

## Relevant amendment history
- **2026-06-15-config-validation-surface** — garde pinned 0.22.1 (not 0.23.0); validation contract unchanged. Affects: §Bootstrap phases input-validation-library-install (constraint 2 above).
- **2026-06-15-structured-logging-stack** — `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are non-path labels requiring NO validation (distinct from `CONDUCTOR_*` path handles). Affects: §Input Validation clarification (constraint 7 above).
- **2026-06-21-runs-db-index** — bundled SQLite 3.50.4 verified (via libsqlite3-sys 0.36.0); `Cargo.lock` committed; audit-green. Affects: CLI's downstream `runs.db` writes via conductor-report (all constraints, acceptance criteria 1 & 5).