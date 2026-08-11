# security extract

## Relevance
Partial — no auth/PII/network boundary is crossed, but the chunk lands new log/artifact field emission (redaction boundary), touches the `runs.db` write path, edits a workspace manifest (dependency gate), and carries the `cargo audit` PREREQ.

## Constraints
- Every new span field must clear the sanitization boundary: no absolute host paths (canonicalized `CONDUCTOR_*` dirs, `ANDROMEDA_PULSE_DATA_DIR`), no internal seam-crate struct/field names — the mandated set (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `row_count`) is exactly the identity/verdict grain permitted (per security-plan §Error Handling → Run-report artifact sanitization; §Security Anti-Patterns → Logging).
- `report.generate`'s `state` must carry the `blocked` state and its named precondition string faithfully — never a silent downgrade to pass/fail/manual-check, and measurement fields stay `null` on a blocked run (per security-plan §Security Anti-Patterns → Universal, five-precondition bullet; §Error Handling).
- Instrumenting `RunsDb::insert` must not alter the write path off rusqlite **bound parameters** — no `format!`/concatenated SQL introduced while adding the `db.insert_run` span (per security-plan §Input Validation, `runs.db` writes row; §Security Anti-Patterns → Input).
- Adding `tracing` to `conductor-report`'s `[dependencies]` is a dependency-tree change: `cargo audit` + `cargo deny check` green and `Cargo.lock` committed/un-drifted before the release gate (per security-plan §Dependency Security → Pinning; §Security Anti-Patterns → Universal).
- The carried audit deferral is a **bounded wait only**: re-run, record, and verify the audit↔deny overlap actually ran green — never raise the floor (unexecutable for an advisory-DATABASE fault), never add a `deny.toml` ignore, never edit CI (per security-plan §Dependency Security, two-fault split).
- Self-observation stays `tracing` JSON to stdout/file; the span tree must not become an OTLP export or any new listener/egress — the only OTLP is the product fault stream to loopback `:4317` (per security-plan §Threat Model Summary → Attack surface; §Security Anti-Patterns → Universal, no-inbound-listener bullet).
- Any wall-clock stamp the new spans record on the journal/report path must come from `std::time::SystemTime`/`Instant`, not tokio's virtual clock (per security-plan §Security Anti-Patterns → Logging).

## Patterns to follow
- The shipped non-path log-value precedent: `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` are stamped into self-obs JSON *values*, serde_json-escaped, requiring no validation — the model for a new span attribute that is a label, not a path (per security-plan §Input Validation, non-path env-handle paragraph).
- Sink paths are already hardened: `logs/agent-latest.jsonl` derives from `CONDUCTOR_RUNS_DIR` through the shipped `resolve_under` traversal guard — consume it, do not re-derive a path for the span sink (per security-plan §Input Validation, non-path env-handle paragraph).
- The verdict/error wall: `Verdict` / `ReportState` are typed `Ok(...)` values, `Result::Err` is harness-fault only — `report.generate`'s `verdict`/`state` fields read from those typed values, never from a stringified error (per security-plan §Error Handling → Internal logging).
- `thiserror` inside the seam, type-erased `anyhow` only at the `conductor-cli` / `#[tauri::command]` edges — a span added inside `conductor-run`/`conductor-report` does not move the sanitization edge (per security-plan §Error Handling → Error format; §Bootstrap phases → error-sanitization-wire).

## Anti-patterns to avoid
- NEVER let the run-report artifacts, `runs.db` rows, JSONL journals, or the new self-obs span fields leak absolute host paths or internal struct/field names — they are agent-parseable ground truth shared across hosts (per security-plan §Security Anti-Patterns → Logging).
- NEVER resolve the audit deferral with a `deny.toml` ignore, a floor raise, or a CI edit — an advisory-DATABASE fault has no fixed release to pin to and suppression would fake compliance (per security-plan §Dependency Security).
- NEVER build `runs.db` SQL by string concatenation / `format!` while touching the insert path for instrumentation (per security-plan §Security Anti-Patterns → Input).

## Contract bindings
- **Redaction boundary ↔ obs:** security owns the "no host paths / no internal struct names" rule; obs-plan §6/§11 owns the tracing field-allowlist that mechanically enforces it — the new span attributes must be registered there, not exempted.
- **Field-assertion ↔ tests:** verification reuses the shipped agent-mode log-schema conformance scaffold over `agent-latest.jsonl`; that assertion doubles as the redaction check, so fixtures/goldens must carry no real host path (tests §CI Integration owns the harness, security owns the pass condition).
- **CI security gate ↔ tests §CI Integration:** the existing single `.github/workflows/ci.yml` already carries `cargo audit` / `cargo deny` / `npm audit --omit=dev`; this chunk records the audit result and must not add or modify a gate job.

## Acceptance criteria contributions
- (security) `cargo audit` re-run and its result recorded, with `cargo deny check advisories bans licenses sources` verified green as the overlapping signal — no floor raise, no `deny.toml` ignore, no CI edit (per security-plan §Dependency Security).
- (security) `Cargo.lock` committed and un-drifted after the `conductor-report` → `tracing` manifest edit (per security-plan §Dependency Security → Pinning; §Security Anti-Patterns → Universal).
- (security) the emitted span-field set on a real run's `agent-latest.jsonl` contains no absolute path, no `CONDUCTOR_*` / `ANDROMEDA_PULSE_DATA_DIR` value, and no internal seam-crate struct/field name — only `run_id` · `seed` · `scenario` · `p_ids` · `verdict` · `state` · `row_count` (per security-plan §Error Handling; §Security Anti-Patterns → Logging).
- (security) `RunsDb::insert` still uses rusqlite bound parameters after the `db.insert_run` span is added — grep shows no `format!`-built SQL (per security-plan §Input Validation, `runs.db` writes row).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§Input Validation) — the last chunk to add `tracing` emission: D-security-input fired on the new env reads and was resolved as a clarifying note (non-path labels stamped into JSON log values are not a validation boundary); D-security-deps cleared with `tracing`/`tracing-subscriber` audit + deny green. Precedent that new span/log *fields* are a redaction question, not a validation-boundary question.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§Input Validation) — established that `logs/agent-latest.jsonl` derives from `CONDUCTOR_RUNS_DIR` via the shipped `resolve_under` guard, and that consuming shipped-hardened infra downgrades the detector to a wording reconcile rather than a new boundary. Directly governs this chunk's sink.
- **2026-08-09-interpretation-correctness-posture** (§Dependency Security) — introduced the TOOL-fault vs advisory-DATABASE-fault split after `RUSTSEC-2026-0244` failed byte-identically on 0.22.1 and 0.22.2; this is the amendment the carried PREREQ's bounded wait rests on.
- **2026-06-15-dependency-audit-gate** (§Dependency Security, §Bootstrap phases) — reframed the audit-tool versions as minimum floors, not lockable pins; explains why "re-pin silently" is the ratified remedy here.
- **2026-06-23-line-oriented-output-rendering** (§Dependency Security) — recorded the two justified `deny.toml` exceptions; context for why the exception mechanism exists and why it is explicitly *not* the remedy for this chunk's advisory-DB fault.
