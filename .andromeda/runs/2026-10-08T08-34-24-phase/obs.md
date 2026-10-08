# obs extract

## Relevance
partial — the chunk touches test-tier capture code and a contract note only (no shipped crate, so no span, log line or envelope field is in play), but obs-plan §4 carries a dated statement about the third `canary:` line that this chunk's change makes stale for captures taken after it.

## Constraints
- Obs tier is Minimal: no instrumentation depth is owed for a test-tier pairing change, and none may be added to justify it (per obs-plan §1 Obs Scope Summary).
- The real-model posture rides the headless critical path with NO critical path, span name or span attribute of its own, and its canary storms log only the existing message-borne line; the plan requires that a real-model chunk add no span name, span attribute, log line or allowlist entry. Whether this chunk's diff stays inside `crates/conductor-run/tests/` and so cannot touch any of them is research's question (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- A canary's outcome is read from Pulse's own log, never from the capture's printed `canary:` token. After the fix the token is still a rendering; the new tests therefore grade the printed token against what the recorded Pulse lines bear out, and no check elsewhere may start treating the token as the authority (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- The capture's `canary:` count is not the storm count: the plan records two lines where the third digest's tick falls after the emission instant. A d3-shaped capture keeping two lines is consistent with the plan and must not be "recovered" into a third (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- The real-model interpretation is graded at the harvest tier over the scrubbed capture, never through the envelope; the canary classification must not become an envelope key or a `CheckRecord` field, and the eleven-key envelope stays as it is (per obs-plan §3 Log format JSON schema).
- Pulse is boundary-only: Conductor observes Pulse's responses and its own-log lines, never Pulse internals. The pairing may read only what Pulse's recorded lines carry; where a line carries no digest identity the attribution rests on another recorded property, stated explicitly (per obs-plan §1 Obs Scope Summary, Instrumentation scope row "Pulse MCP server").
- The emission instant the selection compares against is a journal stamp taken from `std::time`, never tokio's virtual clock. Whether the capture's `emitted` value is that journal stamp, and whether a fixture built from ledger stamps preserves that basis, is research's question (per obs-plan §11 Obs Anti-Patterns, Project-specific bans).

## Patterns to follow
- Harvest-tier evidence over PULSE's own tracing lines, computed in test code and never through a span or the envelope: each quantity is read from its own exact line, and where the line lacks an identity field the attribution is by another recorded field, named as such (per obs-plan §4 Scenario: Severity-lifecycle full pass observing auto-resolve + resolution summary).
- Pulse fields outside its own allowlist read `"<redacted>"` live, so a witness is taken from an unredacted line. Whether every field the widened pairing reads (the inference outcome, the parse status, the dedupe/surface outcome) is unredacted in the recorded d1/d2/d3 lines is research's question (per obs-plan §4 Scenario: Restart-suppression scenario incl. bypass case).
- Dated measurement records in the plan stand as that date's reading and are superseded by a later dated reading beside them, never rewritten: the sixth series' three-line / `pipeline-fault` reading stays as measured at its chunk's `evidence/attempt-ledger.md` (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- A test-tier check that computes a fact no span carries is recorded in the plan as a test-tier check, with the span attribute left untouched (per obs-plan §4 Scenario: Fingerprint-storm scenario (high-cardinality emission)).

## Anti-patterns to avoid
- Never widen the bounded span-name set or mint a span to witness the pairing; the set is closed and the pairing is test-tier (per obs-plan §11 Obs Anti-Patterns, Spans / Traces).
- Never let an absolute host path or an internal struct name reach a committed artifact; a fixture or a test's printed output built from recorded Pulse lines carries none, and the committed captures are not re-rendered to make it so (per obs-plan §11 Obs Anti-Patterns, Logs).
- Never assume the redaction layer's three application sites bound every channel: a new test fixture is a channel of its own, outside `conductor-core::redact` (per obs-plan §11 Obs Anti-Patterns, PII Scrubbing).

## Contract bindings
- obs ↔ tests: the envelope format and its two record shapes are owned by test-plan §3; the canary classification stays outside both. The new default-suite tests run under the `rust` job's nextest stage and produce no telemetry artifact of their own (per obs-plan §9 CI Integration, Pipeline integration).
- obs ↔ security: recorded SUT output is untrusted text; security-plan owns the scrub chain and the rule on capture text in test source. Obs binds only the host-path-free outcome (per obs-plan §11 Obs Anti-Patterns, Logs).
- obs ↔ `contracts/pulse-real-model-leg-posture.md`: the contract's "The canary classification" clause and obs-plan §4's real-model paragraph describe the same pairing behaviour. The chunk's second dated correction in the contract and an obs-plan amendment must say the same thing, with the same date (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).

## Acceptance criteria contributions
- (obs) The chunk's diff adds no span name, span attribute, self-obs log line or `ALLOWLISTED_FIELDS` entry, and touches no shipped crate (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- (obs) The run-report envelope keeps its eleven keys and gains no canary field; the journal conformance gate stays green unchanged (per obs-plan §3 Log format JSON schema).
- (obs) The chunk's expected amendments name obs-plan §4's real-model paragraph: its statement that a tick just before the instant yields a third line reading `pipeline-fault` is true of the sixth series as measured and stops being true of captures taken after this change, so a dated note is owed beside it, with the "read from Pulse's own log, never from the printed token" rule left standing (per obs-plan §4 Scenario: Headless deterministic scenario run with MCP read-back verification, Real-model posture).
- (obs) No fixture, test output or contract note written by this chunk carries an absolute host path (per obs-plan §11 Obs Anti-Patterns, Logs).
