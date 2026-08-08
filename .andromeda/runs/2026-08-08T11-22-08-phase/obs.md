# obs extract

## Relevance
Partial — no new must-trace path, span, or metric; obs applies to the manifest **load boundary's** log line, error edge, and redaction/CI-gate conformance.

## Constraints
- **Field-allowlist gate.** Any field logged at manifest load must be a member of `conductor-core::redact::ALLOWLISTED_FIELDS`, or the subscriber's processor stage silently DROPS it (per obs-plan §11 PII Scrubbing, §6). Today's bounded self-obs fields are `message` / `phase` / `count` / `panic` / `location` plus the 7 identity fields — a `manifest_path`, `manifest_version` or `capability_count` field would vanish from the JSONL. Fold the datum into `message`/`count`, or amend the allowlist deliberately in the same change (§11: single-location ownership in `conductor-core::redact`).
- **No host path may escape.** The manifest is a path-valued on-disk input, so its failure text will name a file. Per obs-plan §11 (Logs · Error Reporting) the log value scrub (`redact_value` → `<redacted>`) covers the JSONL and `sanitize_error` (Display-not-Debug) covers the operator/stderr edge; §9 conformance FAILS the build on a drive-letter / `/home` / `/Users` / `%APPDATA%` path in any field.
- **Never panic on malformed input.** Whichever side of the verdict/error wall the plan picks (scope open question 1), the malformed/absent/unparseable case is one structured `error!`/`warn!` event, never a panic — obs-plan §10 zero-unlogged-panics invariant + §9 gate (greps `agent-latest.jsonl` + stderr for `^thread.*panicked`) + §11 Error Reporting.
- **Bounded span-name set is closed.** Manifest load is not one of the 7 §4 must-trace paths; prefer a boundary `info!` event over a new span. A new name must follow `{module}.{operation}`, stay low-cardinality (never per-path / per-capability), and land as an obs-plan amendment — obs-plan §11 (Spans / Traces).
- **Level mapping.** Load success = `info` boundary-call summary (once-per-process, not hot path); unrecoverable = `error`, degraded/recoverable read = `warn` — obs-plan §6 log-levels + boundary-call wrappers. No `info` inside the per-capability validation loop (§11 Logs: "NEVER log in hot path at `info`").
- **No new telemetry mechanism.** No metrics instrument for capability counts (§5 N/A at Minimal tier; §11 Metrics), no OTel SDK / exporter (§3 OTel SDK init — behavioral invariant), no background watcher on the artifact (§11 Telemetry Strategy; determinism).

## Patterns to follow
- `D:/dev/projects/conductor/crates/conductor-core/src/redact.rs` — `is_allowlisted` + `redact_value` at the processor stage, `sanitize_error` at the operator edge; consuming this unchanged infra is not a new boundary (per obs-plan §11 PII Scrubbing).
- `D:/dev/projects/conductor/crates/conductor-core/src/obs.rs` — the custom `tracing-subscriber` layer injects identity + `run_id` on every line, so new call sites just use `tracing::info!` with no per-callsite plumbing; manifest-load lines are **self-obs base lines**, not Run-report envelope records (obs-plan §3 "two record shapes").
- `D:/dev/projects/conductor/crates/conductor-core/src/config_path.rs` (`resolve_under` → `CoreError::Config`) — the existing `CONDUCTOR_*` path-handle canonicalize + bounds-check precedent for scope open question 3; note its error strings interpolate `{e}` (io/canonicalize text), so they must reach the operator only via the sanitize edge.
- `D:/dev/projects/conductor/crates/conductor-verify/src/manifest.rs` — the "reported `Blocked` + named precondition" precedent the route line favours; it currently carries **no** `tracing` instrumentation, so choose the load-boundary line explicitly rather than copying its silence (obs-plan §6 boundary-call wrappers: one structured line per boundary — call + result + error).

## Anti-patterns to avoid
- No `eprintln!` / unstructured stderr for the malformed-manifest message, and no multi-line or `Debug`-dumped parse error (obs-plan §11 Logs + Error Reporting).
- No high-cardinality or off-convention span for the load (per-manifest-path, per-capability) — obs-plan §11 Spans / Traces bounded set.
- No metrics instrument, meter provider, or OTel SDK to track capability count / manifest version (obs-plan §11 Metrics, §3).

## Contract bindings
- **obs ↔ tests (CI gate):** §9 log-conformance gate reads `logs/agent-latest.jsonl` and asserts the §3 self-obs base schema + no-host-path on **every** line, including the new manifest-load lines; a missing base field or leaked path is a CI FAIL.
- **obs ↔ tests (envelope ownership):** the Run-report envelope schema is owned by test-plan §3, not obs — this chunk emits **no** envelope fields at load time. If a malformed manifest lands as `Blocked`, writing `state` / `p_ids` into the envelope stays the report seam's (obs-plan §3 two record shapes).
- **obs ↔ security:** the manifest path's redaction at logger config + the `anyhow` edge is the shared boundary (obs-plan §11 PII Scrubbing ↔ security §Logging & Monitoring); security also owns the `CONDUCTOR_*` canonicalize/bounds treatment referenced in scope open question 3.
- **obs ↔ arch (verdict/error wall):** obs takes no position on scope open question 1; it requires only that either outcome be a single structured, redacted, non-panicking event.

## Acceptance criteria contributions
- (obs) A malformed / absent / unparseable manifest emits exactly one structured JSON event at `error`/`warn`; no `^thread.*panicked` line appears in `logs/agent-latest.jsonl` or stderr (obs-plan §10 + §9 zero-unlogged-panics gate).
- (obs) No absolute host path in any field of any manifest-load log line or in the operator-facing error text — proven with a temp-dir/drive-letter manifest-path fixture asserting `<redacted>` and absence of `X:\` / `/home` / `/Users` (obs-plan §9 + §11 Logs).
- (obs) Every manifest-load line carries the §3 self-obs base schema (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`) and no run-report envelope fields (obs-plan §3 + §9).
- (obs) The diff introduces no span name outside the §11 bounded set, and every logged field is in `ALLOWLISTED_FIELDS` (or the allowlist is extended in the same change with the obs-plan amendment) — obs-plan §11 Spans / PII Scrubbing.

## Relevant amendment history
- **2026-06-15-log-error-boundary-redaction** — fixed the redaction model this chunk's load-error path must use: value scrub anchors on absolute host-FILE paths → `<redacted>` (NOT `::` tokens), struct names kept out by the field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge, allowlisted `target` preserved. Directly governs "the message names the manifest, not a host path".
- **2026-06-27-obs-ci-conformance-gate** — the `agent-latest.jsonl` gate asserts the §3 **base** schema, not the §6 envelope; the new load lines are graded by this gate, and a missing base field FAILs CI. Explains why load-time lines carry no `verdict`/`latency_ms`.
- **2026-06-15-structured-logging-stack** — established the two record shapes and the custom subscriber layer (stock `fmt().json()` can't emit flat identity fields); the reason manifest-load lines are base lines with identity + `run_id` only.
- **2026-06-18-severity-logs** — the precedent procedure for ADDING to the bounded span-name set (`emit.logs_batch`): conforming `{module}.{operation}` name + a recorded amendment; follow it only if a load span proves warranted.
