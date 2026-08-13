# security extract

## Relevance
Relevant — the chunk consumes the MCP read-back child-stdout trust boundary and turns placeholder values into real ones written to `runs.db` / journal / report, hitting Input Validation, Error Handling, Data Protection, and the standing dependency-audit gate.

## Constraints
- The read-back child's stdout stays a **bounded-decode** boundary: extraction must consume the values the existing line-delimited JSON-RPC client already parsed (soft per-line size bound + serde_json recursion limit); no new unbounded read, no second decode path (per security-plan §Input Validation, MCP read-back child stdout row).
- A malformed, absent, or errored read-back becomes a **typed value** (`Blocked` / `Fail`) routed through the verdict/error wall — never a panic, and `Result::Err` reserved for harness faults (transport down) only (per security-plan §Error Handling; §Security Anti-Patterns → Universal).
- An empty result set / empty canary round-trip MUST become `blocked`, never a false pass-as-empty — this applies per-check now that each `ExpectedCheck` reads its own observed value (per security-plan §Security Anti-Patterns → Input).
- Corpus content read back from Pulse is **read to prove wiring only** — never persisted, copied, or exfiltrated into Conductor artifacts; `corpus.db` is SUT-owned plaintext SQLite out of Conductor's scope (per security-plan §Data Protection; §Security Anti-Patterns → Data Protection).
- The now-real observed / verdict / `latency_ms` values written to `<run_id>.md`, `runs.db` rows, and the JSONL journal MUST carry verdict/state/identity fields only — no absolute host paths (`CONDUCTOR_*`, `ANDROMEDA_PULSE_DATA_DIR`), no internal seam-crate struct/field names (per security-plan §Error Handling, Run-report artifact sanitization; §Security Anti-Patterns → Logging).
- `degraded_mode` → `KnownResidual` is a producer-assigned residual and MUST NOT become a silent downgrade of any of the preflight gate's five named `blocked` preconditions — each keeps its own distinct precondition string (per security-plan §Security Anti-Patterns → Universal, never-silently-downgrade).
- The absorbed `cargo audit` deferral (twelfth consecutive) is an **advisory-DATABASE** fault: remedy is the bounded wait alone — re-run, record, and verify `cargo deny check advisories bans licenses sources` green against *this chunk's own* lockfile delta. No floor raise, no `deny.toml` ignore, no CI edit (per security-plan §Dependency Security, two-fault split).

## Patterns to follow
- `D:\dev\projects\conductor\crates\conductor-verify\src\jsonrpc.rs` — the shipped bounded line decode (`MAX_LINE_BYTES`) mapping faults to `VerifyError::{JsonRpc,Decode,Transport}`; the extraction seam should consume its output rather than re-reading stdout (per security-plan §Input Validation).
- `D:\dev\projects\conductor\crates\conductor-core\src\redact.rs` (`redact_value`, idempotent per the toolchain smoke property test) — the shipped redaction already applied inside `classify`; any newly extracted observed string must reach artifacts through it (per security-plan §Error Handling).
- `D:\dev\projects\conductor\crates\conductor-verify\src\record.rs` — producer-assigned `ReportState` at the run/verify seam is the existing home for a `degraded_mode` `KnownResidual`, consistent with keeping verification outcomes as typed `Ok(...)` values (per security-plan §Error Handling, verdict/error wall).
- `D:\dev\projects\conductor\crates\conductor-report\src\db.rs` — rusqlite bound parameters for every `runs.db` write; real observed values change the content, not the parameterization discipline (per security-plan §Security Anti-Patterns → Input).

## Anti-patterns to avoid
- Never add spawn code, an operator-chosen command, or config-into-argv on the read-back path — the fixed-program-path + `.env(...)` sidecar spawn stays untouched (per security-plan §Security Anti-Patterns → Code Patterns).
- Never let a malformed/hostile child response panic or unwrap its way out of the wall, and never persist raw corpus payloads (report markdown, incident bodies) into `runs.db` / journal / report (per security-plan §Security Anti-Patterns → Universal, → Data Protection).
- Never build SQL for the newly real fields by `format!`/concatenation (per security-plan §Security Anti-Patterns → Input).

## Contract bindings
- **Redaction/scrubbing** binds to obs §PII Scrubbing and to tests: the `stub_pulse_mcp` fixtures must carry only synthetic shapes — no real corpus content, no host-specific absolute paths — so the artifact-hygiene assertion is meaningful (security focus guide: PII scrubbing cross-domain binding).
- **Dependency gate** binds to tests §CI Integration: the audit/deny re-verification is a recorded gate result in the existing single workflow, not a new or edited CI job (per security-plan §Dependency Security, CI integration).

## Acceptance criteria contributions
- `cargo audit` re-run and its result recorded, with `cargo deny check advisories bans licenses sources` verified green against this chunk's own lockfile delta; no floor raise, no `deny.toml` ignore, no CI edit (per security-plan §Dependency Security).
- Against the JSON-RPC stub: a malformed, absent, error-response, and empty read-back each yield a typed `Blocked`/`Fail` value with no panic and no `Result::Err` other than transport (per security-plan §Error Handling, §Security Anti-Patterns → Universal/Input).
- Inspection of a produced `<run_id>.md`, `runs.db` row, and JSONL journal shows the newly real observed/verdict values contain no absolute host path, no internal struct/field name, and no verbatim corpus payload — extracted strings pass through `redact_value` (per security-plan §Error Handling, Run-report artifact sanitization).
- A `degraded_mode`-served read-back maps to `KnownResidual` without overwriting or masking any preflight `blocked` precondition string (per security-plan §Security Anti-Patterns → Universal).

## Relevant amendment history
- `2026-06-27-mcp-read-back-result-shape-adapter` — the read-back boundary was reconciled from rmcp to the hand-rolled JSON-RPC client: bounded per-line decoding + serde_json recursion limit, faults typed as `VerifyError::{JsonRpc,Decode,Transport}` through the verdict/error wall. This is exactly the boundary this chunk extracts from; the invariant is "consume the bounded client", not re-open the stream.
- `2026-06-27-live-pulse-e2e-proof` — `corpus.db` corrected to plaintext SQLite (P-049 encryption assumption disproved); the preserved posture is corpus access via MCP read-back ONLY, never persisting its content into Conductor artifacts. Directly governs what extracted values may be written.
- `2026-08-09-interpretation-correctness-posture` — established the TOOL-fault vs advisory-DATABASE-fault split and its remedies; this is the basis for the absorbed `cargo audit` deferral's bounded-wait handling.
- `2026-08-10-workspace-key-divergence-probe` + `2026-08-10-pulse-run-contract` — the never-silently-downgrade ban was restated over five named preconditions, each owing its own `blocked` string, with a "never claim a measurement Conductor cannot make" clause. Relevant because this chunk introduces a new state-routing arm (`degraded_mode` → `KnownResidual`) adjacent to that ban.
