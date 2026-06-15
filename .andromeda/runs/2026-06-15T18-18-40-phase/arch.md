# arch extract

## Relevance
Partial — this chunk is the primary realization of Conductor's artifact-hygiene invariant (redaction layer on logs + error edges); it is foundational and touches cross-cutting surfaces, but does not involve workspace placement or external IPC contracts.

## Constraints
- (per §Design Philosophy) "Outcomes are values, errors are harness faults" — `Result::Err` is reserved for Conductor's own failures; verification verdicts/report states are typed return values. The redaction layer must preserve this wall: `tonic::Status` and MCP errors remain first-class verification inputs, never panics or stack-trace leaks.
- (per §Stack and Technologies) Observability stack is pinned `tracing 0.1.44 + tracing-subscriber 0.3.23`, not an OTel SDK. Redaction composes onto this existing structured-logging foundation without rebuilding it.
- (per §Conventions / Error handling) Typed `thiserror` enums in seam crates; `anyhow 1.0.102` only at `conductor-cli` / `#[tauri::command]` edges. Redaction surfaces at these edges and at the structured-log subscriber (not at the seam-internal enum definitions).
- (per §Occupied Resources / Environment variables) No new env vars beyond the reserved `CONDUCTOR_*` namespace. Redaction configuration must be hard-coded allowlists, not env-driven.
- (per §Cross-cutting Patterns / Determinism discipline) Wall-clock journal stamps use `std::time::SystemTime`/`Instant`, never tokio's virtual clock. Redaction must not corrupt timestamp fields in logs.
- (per §Conventions / Naming patterns) Crate names are `conductor-<seam>` (kebab-case); modules use snake_case. The redaction primitive lives in `conductor-core::obs`.

## Patterns to follow
- **Structured-logging JSON subscriber** — the custom JSON subscriber already writes to stderr/file; redaction composes as a visitor/layer in the tracing-subscriber chain (intercepting at field-visit before sink) to enforce the field-allowlist and mask/drop non-allowlisted content.
- **Allowlist-first redaction** — only an explicit list of allowlisted field names reach the sink; any unrecognized field is dropped, and any value matching host-path / `module::` shapes is masked before write.
- **Error-edge sanitization shape** — `conductor-cli` and `conductor-tauri` `#[tauri::command]` handlers catch `anyhow::Error`, format as `error: <short message>` + `hint: <suggestion>` (no Display chain, no `.backtrace()`, no `.source()` chain), surfacing only that shape.
- **Reusable redaction primitive** — a shared `conductor-core::obs` redaction module with a scrub function that Epoch 6 run-report writers will call to sanitize JSONL journal / Markdown report / `runs.db` string columns.

## Anti-patterns to avoid
- **Do not leak stack traces, struct names, or absolute paths** — any occurrence of `struct <name>::<field>`, `at <path>:line:col`, or `C:\Users\...` / `/home/<user>/...` must be redacted before the operator sees it (cli stderr, Tauri error payload, or log JSON sink).
- **Do not introduce new OTel SDK machinery or network sinks** — self-obs remains local tracing→JSON.
- **Do not re-stand-up the panic hook or subscriber** — already initialized in the prior chunk; this layer composes on top.

## Contract bindings
- **arch ↔ obs**: the allowlist of JSON field names defined in obs-plan §3/§6 drives the tracing-subscriber field-allowlist here.
- **arch ↔ security**: the "no stack traces / host paths / struct names" mandate in security-plan §Error Handling is the spec this layer implements.
- **arch ↔ Epoch-6 artifact writers**: the scrub primitive is reused by `runs.db` / JSONL journal / Markdown report writers, not redefined per writer.

## Acceptance criteria contributions
- (arch) Redaction primitive in `conductor-core::obs` sanitizes strings against host-path and struct-name patterns; reusable by Epoch 6 artifact writers.
- (arch) tracing-subscriber field-allowlist (obs-plan §3 names) is enforced in the JSON subscriber chain; non-allowlisted fields dropped before sink.
- (arch) Error edges format `anyhow::Error` as `error:` + `hint:` with zero stack-trace / path / struct-name leakage.
- (arch) Regression gate (`cargo nextest run --workspace`, clippy, audit) stays green; no new env vars beyond reserved `CONDUCTOR_*`.

## Relevant amendment history
(none) — architecture-amendments.md records only prior amendments (2026-06-14, 2026-06-15); none touch error handling, logging redaction, or artifact hygiene. This chunk is the first redaction work and adds no architecture decision change.
