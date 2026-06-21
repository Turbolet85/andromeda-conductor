# security extract

## Relevance
Partial — Markdown run report is a reporting/serialization surface; security touches artifact hygiene, data protection, error handling, and dependency supply chain only.

## Constraints
1. All `RunRecord` serialization to the Markdown artifact (`runs/<run_id>.md`) MUST NOT leak absolute host paths, internal struct/field names, or stack traces (security plan §Error Handling, artifact hygiene).
2. Blocked-row measurement fields render as em-dash/absent markers, never the literal `null` or struct names (security plan §Error Handling, blocked-row null rule).
3. Render logic MUST use only self-generated synthetic telemetry and run-metadata from `RunRecord` — no user-derived PII or credentials ever appear in the output (security plan §Data Protection; conductor owns no sensitive data).
4. All file writes to `runs/` directory MUST use canonicalized `runs_dir` path (resolved + bounds-checked by cli edge per security plan §Input Validation, Env-var path handles row) — render module receives the validated path, never constructs it.

## Patterns to follow
1. Deterministic Markdown output (same record set ⇒ identical document) — use insta golden tests; exclude wall-clock `generated_at` from content-hash, inject at render time only (security plan §Error Handling + conductor architecture §Determinism discipline).
2. Verdict-first lamp precedence helper (shared, reusable) — prefer verdict when present, else state; return typed `LampStatus` enum (`Pass`/`Fail`/`Hold`/`Blocked`/etc.) for render (scope.md §Verdict-first lamp precedence; reused by coverage-matrix, cli, desktop).
3. Status ASCII prefix encoding (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`/`[RESIDUAL]`/`[MANUAL]`) — Markdown has no color, so text prefix is the status encoding (status-never-color-alone invariant from CLAUDE.md).

## Anti-patterns to avoid
1. NEVER interpolate or log the canonicalized `CONDUCTOR_RUNS_DIR` path, `ANDROMEDA_PULSE_DATA_DIR`, or internal struct names (e.g. `ReportState::Blocked`) into the `.md` artifact (security plan §Error Handling §Logging).
2. NEVER render a blocked row's measurement fields (`latency_ms`, `fingerprints`, etc.) as literal `null` or struct names — use em-dash/absent marker only (security plan §Error Handling, blocked-row null rule).

## Contract bindings
- **Run report envelope** ↔ envelope-serializer (chunks 1) — `RunRecord` shape and verdict/state definitions are shared; Markdown render is a pure consumer.
- **Artifact hygiene & error handling** ↔ CLAUDE.md universal invariant — no host paths, struct names, or traces in output.
- **Dependency supply chain** ↔ CI security gate (Epoch 8, cli) — `cargo-audit`/`cargo-deny` green is prerequisite; no new deps added by this chunk (render uses only stdlib + existing serde).

## Acceptance criteria contributions
1. (security) No absolute host path (e.g. `C:\Users\...`, `$HOME/...`) or internal Rust struct name appears in rendered `runs/<run_id>.md` — grep verify against the artifact.
2. (security) A blocked row's measurement fields render as em-dash (`—`) or absent, never `null` or `ReportState::Blocked` — inspect golden test output.
3. (security) `runs/<run_id>.md` is written once per run_id and never overwritten; file-write uses canonicalized `runs_dir` only (no path construction in render module).
4. (security) All dependencies remain audit-green (`cargo audit` exit 0 / `cargo deny check advisories bans` pass); no new external crates introduced.

## Relevant amendment history
- **2026-06-21-runs-db-index** — bundled SQLite version corrected to 3.50.4 (via libsqlite3-sys 0.36.0); Cargo.lock committed; audit-green. Relevant because report render shares the Minimal-tier residual-risk control (dependency supply chain, security plan §Dependency Security) — audit closure is prerequisite.