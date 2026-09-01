# Scope — 2026-08-31-p-075-assert-round

**Working-route entry (verbatim title):** _P-075 assert round — the read-back fidelity Pulse's last 0.3.0
capability can honestly claim, and the payload-level premise measured false_

**Epoch:** 5 — Verification surfaces (head entry) · **Version:** conductor-0.2.0

---

## What this chunk is

The round Pulse's P-075 needs, shaped by what turned out to be true rather than by what was asked for. It has
three parts, in descending order of how much code they need:

1. **Prove the read-back lifecycle that IS runtime-real** — `mark_incident_resolved`'s applied/declined
   round-trip and `query_incident_list` active-set membership, measured against a live deterministic-L4 Pulse.
   This is runtime-STATE fidelity: it depends on no L4-authored field, which is exactly why it survives where
   payload fidelity cannot.
2. **Record the second premise disproof** — payload-level content fidelity stays unattainable, now on a second
   independent axis (`incident_events` reaches no MCP tool), with the measurement written where the ledger and
   the specs will carry it forward.
3. **Close P-075's Pulse-side claim by citation** — quote `v2-20`'s already-shipped fixtures as the external
   evidence reference, which that entry's own acceptance text anticipated. Nothing is re-asserted.

## What it builds

- A live leg driving a deterministic-L4 Pulse to a known incident, then exercising the lifecycle write:
  `mark_incident_resolved` → re-read `query_incident_list` → assert the incident left the ACTIVE set.
  Pulse's own unit test (`crates/mcp-server/src/tools.rs:934` `mark_incident_resolved_writes_resolved_status`)
  asserts exactly this pairing in-process; this chunk is the first time Conductor asserts it across the wire.
- Whatever run-seam wiring that leg needs to reach `ReadbackClient::mark_incident_resolved` — which today has
  a client method and **no production caller** (see §Measured premises).
- The typed handling of the DECLINED arm, which arrives as a JSON-RPC **error**, not an `Ok` value — so it must
  become a typed `Verdict`/`ReportState`, never a panic and never a harness `Err` (verdict/error wall).
- The recorded disproof: the `incident_events` measurement written into the artifacts that own it, and the
  P-075 evidence citation.

## Boundaries — what this chunk does NOT do

- **Does not re-assert P-027 ≤5s / P-037 ≤2s / P-045 ≤1s.** `v2-20` is already `status: verified`
  (chunk `2026-08-21-delegated-timing-budgets-proven`), its `ref` naming three tests confirmed present at
  `crates/conductor-run/tests/delegated_timing_harvest.rs:270`, `:291`, `:305`. Re-certifying verified work
  from a weaker basis is the explicit anti-goal; the round QUOTES those fixtures.
- **Does not touch P-025's ≤2s hue clause.** Excluded from v2-20 by the 2026-08-21 operator ruling after
  measurement disproved its premise (staleness, not latency); owned by Epoch 6's _Halo hue budget re-driven_.
- **Does not build an `incident_events` read path.** No MCP tool surfaces the table; that work is recorded as a
  **Pulse 0.4.0 residual candidate** for the operator, deliberately not Conductor's.
- **Does not attempt payload-level content fidelity.** The 2026-08-16 posture stands: under deterministic L4
  every L4-authored field is a fixture constant and freshness remains the canary's carrier.
- **Claims no verification-matrix capability by default.** None of the 11 unclaimed ids covers read-back
  fidelity. P5's matrix step re-checks this; if the lifecycle leg turns out to make a pooled cap fully
  verifiable, that is a claim to make deliberately at P5, not an assumption here.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `conductor-verify` | `ReadbackClient::mark_incident_resolved` (`client.rs:148`) gains its first caller; the declined-arm error mapping lands in the `VerifyError` fan-out |
| `conductor-run` | the live leg + its harvest-tier assertions (the shape seven families already ship) |
| `scenarios/` | possibly one scenario, if the leg needs a declared driver rather than a harvest test |
| architecture §Read-Back Dependency Posture / §Standard Contracts | the second-axis disproof + what runtime-state fidelity now proves — a wrap amendment, not a phase edit |
| `verification-matrix.json` | at most a `notes` narrative; a claim only if P5 finds one genuinely satisfied |

## Folded annotations (from the working entry)

**PREREQ — 40th `cargo audit` re-check.** Standing deferral since `2026-08-08-sut-capability-manifest`,
ratified at the `2026-08-10-workspace-key-divergence-probe` wrap, PROBE-AUTO-SATISFY form. This chunk's wrap
re-runs the probe and records `probe unchanged, 40th consecutive` if the signature reproduces byte-identically
(`cargo audit` true exit 1 / `duplicate advisory ID: RUSTSEC-2026-0244`; `cargo deny check advisories bans
licenses sources` true exit 0), capturing the exit code **before any pipe**. Any deviation — including a
dependency delta that ADMITS a package — restores the full form. Origin and ratification marker carry forward.

**CONTEXT — the four operator rulings** (2026-08-31 relay, ratified after this session's re-derivation). Folded
into §What this chunk is and §Boundaries above rather than restated here.

## Measured premises (verified first-hand at promotion, not inherited)

- Pulse HEAD `83d4060`. **ZERO** `incident_events` references in `crates/mcp-server`; hits live only in
  `crates/corpus` (contract.rs, disposition.rs, schema.rs), `crates/triage/src/incident/persistence.rs`, and one
  pulse-app test. The eight wire tools are pinned at `crates/mcp-server/src/tools.rs:44-51`.
- `crates/mcp-server/src/tools.rs:445` `dispatch_mark_incident_resolved` returns
  `Ok({"resolved": true, "incident_id": <id>})` on the **applied** path, and a JSON-RPC **error**
  (`ToolDispatchFailed`, reason `"incident changed concurrently; resolution not applied"`) when
  `corpus::contract::IncidentWriteOutcome::DeclinedStale` comes back from `update_incident_status`.
- `ReadbackClient::mark_incident_resolved` exists at `crates/conductor-verify/src/client.rs:148` with **no
  production call site** — the only other references are the const, a doc line, the stub's tool list, and a
  preflight test that drops the tool. The capability is wired and has never fired.
- `query_incident_list` is the live workhorse (`conductor-verify/src/extract.rs:77`, `preflight.rs:377`).

## Premises — CLOSED at P3 (see `research.md` §Scope premise closure)

- **`[premise-corrected: the guard is a monotonic-timestamp compare, not a race]`** The **declined** arm is
  unreachable through the MCP surface by any ordinary means — not because a concurrent modification is hard
  to stage, but because `update_incident_status` guards on `WHERE … AND updated_unix_nano <= ?2` and
  `dispatch_mark_incident_resolved` stamps `now` fresh on every call, so the predicate can fail only against
  a future-stamped row. The honest split is therefore **applied-arm-live + declined-arm-by-stub**, recorded
  with that reason. This also dissolves the security extract's open question about a test-only corpus
  stager: no corpus staging is needed or wanted, so the `corpus.db` ban is never approached.
- **VERIFIED** The leg's home is a harvest-tier test (`crates/conductor-run/tests/*_harvest.rs`), not a
  scenario TOML — eleven siblings ship the pattern, and it is plain `#[test]`s over verbatim pinned leg
  captures. Consequence: **no new P-ID binding, no `check_scenario_backing` movement, no `UNBACKED_AUTO`
  change.**
- **VERIFIED (real, mechanism named)** Resolving an incident perturbs anything reading back *after* it in the
  same run: `query_incident_list` is active-only, so the resolve empties the active set and `route_read_back`
  then routes a declare-only scenario to `AutoResolved` and a checks-bearing one to `Blocked`. Preflight runs
  before and is unaffected. test-plan §6 already requires this exact mis-pairing be pinned as a negative test.
- **VERIFIED, and the arch site count is TWO** The payload-fidelity/freshness-carrier posture is stated at
  BOTH §Established Decisions [Read-Back Dependency Posture] and §Standard Contracts (readiness gate); the
  duplicate-occurrence precedent requires both to move together, plus the `residuals.md` pin.
- **VERIFIED: it does not** `mark_incident_resolved` is in `contracts/mcp-contract.toml`'s `required_tools`
  and preflight asserts **presence only**, so firing it changes no preflight assumption.

## Measured at P3 — facts that shape the build

- `ReadbackClient::mark_incident_resolved` has **zero references workspace-wide** on the code-graph `refs`
  view, with the sibling `query_incident_list` returning 5 on the same view as the working control. Four
  independent bases agree (grep · graph · test-plan §5 · no harvest file references it).
- **No new cross-seam edge is needed** — `conductor-run → conductor-verify` already exists.
- **No new instrumentation is needed** — all four tool methods delegate to `call_tool`, which already carries
  the bounded `verify.readback.call_tool` span and the allowlisted `mcp_tool` field.
- `VerifyError` already separates `JsonRpc { code, message }` (tool-level) from `Transport` (io), and
  `JsonRpc`'s `Display` prints only the code, so it is artifact-safe by default.
- **`incident_id` is NOT in `ALLOWLISTED_FIELDS`** — it cannot ride a span attribute and must go on `message`.
