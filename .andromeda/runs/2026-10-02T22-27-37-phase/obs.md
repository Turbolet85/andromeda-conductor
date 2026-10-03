# obs extract

## Relevance
partial — the chunk is a test-tier live round plus a contract pin; obs binds only where the new `retrieve_incident_events` call and the resolve-call wall-clock window touch the MCP boundary-call, clock and envelope rules.

## Constraints
- Every MCP tool call on the verify seam, the lifecycle WRITE `mark_incident_resolved` included, must ride the bounded `verify.readback.call_tool` span with the allowlisted `mcp_tool` attribute. Its boundary log must carry the tool name, `latency_ms`, any error and the observed key-set witness (key NAMES only, never values) on the allowlisted `message` field (per obs-plan §6 Log Coverage → Boundary-call wrappers; §1 Obs Scope Summary, `conductor-verify` row). If `retrieve_incident_events` is called through a shipped `ReadbackClient` method, the same rule applies to it. Research decides whether the call goes through that client or is made raw from the test leg.
- An incident ID passed to an MCP tool must ride the allowlisted `message` field, never a dedicated `incident_id` attribute. `incident_id` is outside `conductor-core::redact::ALLOWLISTED_FIELDS`, so the processor stage would drop it (per obs-plan §6 Log Coverage → Boundary-call wrappers, MCP lifecycle WRITE bullet).
- The span-name set is closed (`verify.readback*` among others). A new tool adds no span name and no allowlist entry; any attribute outside the allowlist emits nothing (per obs-plan §11 Obs Anti-Patterns → Spans / Traces; §4 Span / Trace Coverage, Known-residual "Required span attributes" preamble).
- The resolve call's window (request sent → response received) must be stamped from `std::time`, never tokio's virtual clock (per obs-plan §11 Obs Anti-Patterns → Project-specific bans; §5 Metric Coverage measurement basis). The window is compared with Pulse's `occurred_unix_nano`, which counts from the epoch. So the window needs an epoch-anchored `SystemTime` reading. An `Instant` alone has no epoch to compare against. Research decides whether `probe_resolve_lifecycle` already exposes such a window.
- The run-report envelope stays the eleven keys, and the per-check `CheckRecord` stays its nine. A Pulse-side quantity is graded at the harvest/test tier, never through the envelope or through `latency_ms`/`budget_ms`/`effective_deadline_ms`, which bound Conductor's journal-relative span (per obs-plan §3 Observability Harness Contract → Log format JSON schema, "Two record shapes"; §4 Span / Trace Coverage → Delegated-timing family).
- Assertions 3–6 reuse each leaf's own field, and the names differ: `duration_ms` on the hue, discovery and counter-refresh leaves, `value` on `metric.report.render_ms`. Each is graded by the in-window worst-observation rule, with an absent sample UNGRADED (per obs-plan §4 Span / Trace Coverage → Delegated-timing family).
- The `mark_incident_resolved` declined arm is stub-proven only, permanently. Pulse's own dispatch cannot trip `DeclinedStale`, so a live leg cannot exercise it (per obs-plan §6 Log Coverage → Boundary-call wrappers, MCP lifecycle WRITE bullet).

## Patterns to follow
- Key-set shape witness on `message` for each raw tool result. A reader that degrades to empty on an unrecognized shape needs it, or a field-name divergence is indistinguishable from an empty result (per obs-plan §6 Log Coverage → Boundary-call wrappers, MCP readback bullet). `events: []` before the first transition is exactly that ambiguity.
- Prior-round grading lives in-process in the `live-pulse`-gated `p075_round_live` leg, through test-tier harvest graders over a digest-pinned capture, never through a span or the envelope. `fingerprints_read_back_count` stays a COUNT (per obs-plan §4 Span / Trace Coverage → Fingerprint-storm scenario, `verify.readback_fingerprints` attribute note).
- Delegated-timing grades are held by stable per-assertion test ids. Prior-round ids stay bound to their own evidence (per obs-plan §4 Span / Trace Coverage → Delegated-timing family, the "Re-graded 2026-10-02" clause).
- Correlation is by `run_id` only, with no trace context (per obs-plan §3 Observability Harness Contract → Correlation).

## Anti-patterns to avoid
- A new span name or attribute for the incident-events read, the resolve window or the event kinds (per obs-plan §11 Obs Anti-Patterns → Spans / Traces). Use the existing `verify.readback.call_tool` + `mcp_tool` and the `message` field.
- `tokio::time::Instant` (or any virtual-clock reading) for the resolve-call window (per obs-plan §11 Obs Anti-Patterns → Project-specific bans).
- Leaking absolute host paths into the round's self-obs lines, journal or committed evidence (per obs-plan §11 Obs Anti-Patterns → Logs; §9 CI Integration → Log conformance check).

## Contract bindings
- obs ↔ arch / contracts: obs-plan §1 Obs Scope Summary (the "Pulse MCP server" row) and §6 Log Coverage → Boundary-call wrappers enumerate exactly four MCP tools. Pinning `retrieve_incident_events` in `contracts/mcp-contract.toml` `required_tools` would leave both obs enumerations naming four. That is a master-drift candidate for wrap, owned by obs-plan, not fixed by this extract.
- obs ↔ tests: the run-report envelope and `CheckRecord` shapes are owned by test-plan §3 and held by `journal_conformance` (key presence, closed sets, host-path freedom). The round must not extend either shape (per obs-plan §3 Observability Harness Contract → Log format JSON schema).
- obs ↔ security: span attributes pass `conductor-core::redact::ALLOWLISTED_FIELDS`, and the host-path value scrub applies (per obs-plan §11 Obs Anti-Patterns → PII Scrubbing). Committed evidence hygiene binds to the security-plan scrub chain.

## Acceptance criteria contributions
- Any shipped call of `retrieve_incident_events` emits a `verify.readback.call_tool` span whose `mcp_tool` value names the tool, plus a boundary line carrying the observed key set and the incident ID on `message`. The bounded span-name set and the field allowlist are unchanged. If the call is test-only, the plan states that and asserts no span (per obs-plan §6 Log Coverage → Boundary-call wrappers; §11 Obs Anti-Patterns → Spans / Traces).
- The resolve-call window used by assertion 7 is two `std::time::SystemTime` epoch readings taken around the `mark_incident_resolved` request/response, and `grep` for `tokio::time` in the stamping code returns nothing (per obs-plan §11 Obs Anti-Patterns → Project-specific bans).
- The round's `runs/<run_id>.jsonl` envelope lines carry exactly the eleven keys and pass `journal_conformance`. Assertion 7 is graded at the test tier, never through `latency_ms`/`budget_ms` (per obs-plan §3 Observability Harness Contract → Log format JSON schema; §4 Span / Trace Coverage → Delegated-timing family).
- The round's self-obs stream and committed evidence contain zero absolute host paths (per obs-plan §9 CI Integration → Log conformance check).
