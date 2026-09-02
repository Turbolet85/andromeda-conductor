# v2-28 — per-site rmcp classification of `.andromeda/test-plan.md`

**Produced by** `/andromeda-implement`, chunk `2026-09-02-cross-surface-envelope-parity`, 2026-09-02.
**This file is the wrap amendment's payload.** `test-plan.md` is a spec master — phase and implement are
read-only on it, so nothing here has been applied; wrap's amendment flow owns the edit.

## Basis
`grep -c -i rmcp .andromeda/test-plan.md` = **24** lines. The count is the basis, not any enumeration: the
originating CARRY (`2026-08-31-p-075-assert-round`) named **one** remaining site — `contracts/mcp-contract.toml`,
which is not even one of v2-28's three named documents — and the phase-time tests distiller claimed **26** with a
partial list. Both were re-derived against the artifact.

## The target wording (architecture.md §Established Decisions [MCP Read-Back Client] is the authority)
The shipped client is a **hand-rolled line-delimited JSON-RPC client over the sidecar's stdio**
(`conductor-verify/src/jsonrpc.rs` + `client.rs`) — `initialize` / `tools/list` / `tools/call` returning the RAW
`serde_json::Value`; version handling is **reading** `protocolVersion` back from the `initialize` result, not
negotiating down from a client default. The test stub (`stub_pulse_mcp` + `tests/common/mod.rs`) is likewise
hand-rolled, emitting Pulse's raw shapes. rmcp was **removed 2026-06-27** and is absent from `Cargo.lock`.

## Classification

### AMEND — 21 sites describing the CURRENT client or the CURRENT stub
| Line | § | What is stale |
|---|---|---|
| 33 | §1 Entity | "`conductor-verify` (MCP read-back client via rmcp…)" + the quoted arch Stack "rmcp 1.7.0 … `serve_client()` over `TokioChildProcess` stdio" |
| 53 | §2 Surfaces | "via rmcp over `TokioChildProcess` stdio" + "a stub/mock MCP server (rmcp) over stdio" |
| 114 | §3 Tier table | "rmcp stub stdio" as the integration-tier IPC driver |
| 143 | §3 `boot` | "over `rmcp::serve_client()` on `TokioChildProcess`" + "runs against the rmcp in-process/`TokioChildProcess` stub server" |
| 153 | §3 stage selectors | "`--integration` ⇒ the rmcp-stub + …" |
| 243 | §4 conductor-verify | "against an rmcp in-process stub" |
| 254 | §4 External services | "mocked via rmcp stub at unit/integration" |
| 264 | §5 Driver(s) | "rmcp 1.7.0 in-process duplex / `TokioChildProcess` stub server" |
| 272 | §5 boundary table | "rmcp stub MCP server over stdio …" and the tool cell "rmcp 1.7.0 (`client` feature)" |
| 275 | §5 lifecycle | "rmcp stub spun per test" |
| 303 | §6 surface table | "rmcp 1.7.0 stub server over stdio (CI)" |
| 317 | §6 step 1 | "(rmcp stub in CI; live Pulse on local gate)" |
| 416 | §8 Mocking | "mock with an rmcp 1.7.0 stub server" |
| 431 | §8 mock table | "rmcp 1.7.0 (`#[tool]` stub)" — `#[tool]` is an rmcp macro with no counterpart in the hand-rolled stub |
| 437 | §8 Anti-monkey-patching | "transport injection for rmcp" as the current DI mechanism |
| 457 | §9 pipeline | "rmcp-stub + tauri::test mock-runtime" |
| 494 | §10 coverage | "exclude … rmcp/tauri stubs … from coverage" |
| 522 | §11 stack-specific | "the rmcp stub canary proves MCP wiring only" |
| 554 | §11 Mocking | "wrap the live Pulse behind the rmcp transport" |
| 556 | §11 Mocking | "DI (rmcp transport injection, …)" |
| 596 | §12 E2E drivers | "MCP read-back → rmcp 1.7.0 stub server over stdio" |

### LEAVE STANDING — 2 sites naming the vulnerability CLASS
| Line | § | Why it stands |
|---|---|---|
| 88 | §2 trigger | "security plan Vector 4 + anti-patterns (rmcp STDIO command/argument-injection CVE-2026-30623…)" — the injection class is named after rmcp's STDIO transport and keeps that name regardless of which client ships. The shipped code says the same (`conductor-verify/src/spawn.rs:5`, `lib.rs:6`). |
| 558 | §11 stack-specific | "(rmcp STDIO injection class CVE-2026-30623)" — same class name, same reasoning. |

### MIXED — 1 site, amend one clause only
| Line | § | Disposition |
|---|---|---|
| 91 | §2 trigger | Two claims in one line. **LEAVE** the quoted anti-pattern title "Pinning the rmcp client to a strict newer protocol default" — it is the recorded name of a rejected design. **AMEND** the mechanism clause "the preflight gate negotiates DOWN to `2024-11-05` (not strict-newer)": the hand-rolled client does not negotiate down from a default, it READS the version from the `initialize` result and the manifest pin is what the gate asserts against. |

## Sites OUTSIDE test-plan.md, for the record
- **Fixed by this chunk** (in-repo, not spec masters): `contracts/mcp-contract.toml:5` (the CARRY's one named
  site) and `crates/conductor-verify/src/lib.rs:3` — "the rmcp client/transport foundation", a present-tense
  claim about the shipped client that neither the CARRY nor the distiller named. The same doc comment's
  "negotiates the protocol version *down*" clause was corrected with it, same defect class.
- **Verified already correct, no action**: `.claude/docs/tests-summary.md:46`, `.claude/rules/verification-harness.md:18,44`,
  `.claude/docs/stack.md:15`, `.claude/docs/conventions.md:28`, `.claude/docs/gotchas.md:12,17`,
  `.claude/rules/testing.md:35`, `.claude/agents/code-reviewer.md:47` — each records the 2026-06-27 removal.
  v2-28's other two named documents (`tests-summary.md`, `verification-harness.md`) are therefore already
  satisfied; `test-plan.md` is the only one outstanding.
- **Historical records, leave standing**: `.claude/rules/verification-harness.md:40,41` — dated 2026-06-21
  `## Session Additions` entries superseded in place by the 2026-06-27 entry at `:44`.
- **Correct as written** (rejection rationale, not a current-client claim):
  `crates/conductor-verify/src/client.rs:7`, `jsonrpc.rs:5`, `tests/common/mod.rs:3` — each records WHY rmcp was
  rejected (its typed `call_tool` rejects Pulse's raw result).
- **Surfaced, not owned by v2-28**: `.andromeda/obs-plan.md` §4 carries two rmcp mentions the CARRY never
  enumerated (the span-kinds line and the auto-instrumentation table's conductor-verify row). Same defect class,
  outside v2-28's three named documents — an amendment-flow candidate at this wrap.
