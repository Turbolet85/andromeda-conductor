# Codebase Research — 2026-08-31-p-075-assert-round

## Scope
- **Depth:** moderate · **Reads:** 9 (6 Conductor, 3 Pulse) · **Globs/Greps:** 7 · **Graph queries:** 5 (rust plane, `db_state: fresh`)

## Graph impact (rust plane)

- **`ReadbackClient::mark_incident_resolved`** — indexed as `conductor-verify 0.1.0 client/impl#[ReadbackClient]mark_incident_resolved().`; `calls` **0 rows**, `refs` **0 rows**. Genuinely zero references anywhere in the workspace, tests included. This is *consulted-but-no-match*, not an index gap — both cookbook preconditions hold (plane built `fresh`; the symbol is confirmed present via the bare-name probe), and the control below proves the view itself works.
- **Control — `ReadbackClient::query_incident_list`** — same view, **5 refs**: `extract.rs:76` and `preflight.rs:376` (production), `tests/jsonrpc_correlation.rs:24`, `tests/readback.rs:45`, `tests/readback.rs:131`. Graph lines are 0-indexed and match grep's 77/377. Because the sibling method resolves on the identical view, `mark_incident_resolved`'s zero is a measurement, not silence.
- **The `calls` view is NOT usable here.** It returned 0 rows for *both* methods, though `query_incident_list` demonstrably has callers — it under-resolves these `&self` async method calls. Use `refs` for this seam; a `calls`-based impact claim on this crate would be false-negative by construction.
- **`crate_edges`** — `conductor-run → conductor-verify` **already exists** (alongside `→ conductor-core/-emit/-faults/-report/-timeline`; `conductor-tauri → conductor-run`). A leg in `conductor-run` calling the verify client adds **no new cross-seam edge**, satisfying arch §Occupied Resources without a `Cargo.toml` change.

**Four independent bases now agree the method has never been called:** grep (5 hits — const, doc, definition, stub tool-list, a preflight test that *drops* the tool), the code-graph `refs` view (0), test-plan §5's own amendment text ("no production call site and no live exercise"), and the absence of any `_harvest.rs` referencing it.

## Files inspected

- `crates/conductor-verify/src/client.rs` (115–150) — **all four tool methods are one-line delegations to `call_tool`**, which carries `#[tracing::instrument(name = "verify.readback.call_tool", skip(self, arguments), fields(mcp_tool = %name))]`. The obs mandate (bounded `verify.readback*` span name + `mcp_tool` attribute) is therefore **already satisfied for `mark_incident_resolved` with zero new instrumentation** — this answers the obs extract's research question.
- `crates/conductor-verify/src/error.rs` (13–37) — `VerifyError` already **separates** the tool-level application error from the transport fault: `JsonRpc { code, message }` vs `Transport(io::Error)`. This answers arch's research question. `JsonRpc`'s `Display` prints **only the code**, so it is artifact-safe by default and the server's text stays in `message` for deliberate, sanitized surfacing.
- `contracts/mcp-contract.toml` — `mark_incident_resolved` **is** in `required_tools`. Preflight asserts **presence only**, so firing the tool changes no preflight assumption. (Note, not this chunk's: the file's header comment still says "the rmcp client negotiates DOWN to it" — stale rmcp wording, owned by `v2-28` under the Epoch-5 _Cross-surface envelope parity_ entry. Surfaced, deliberately not fixed here.)
- `crates/conductor-run/src/lib.rs` (502–518) — `route_read_back(ReadBackOutcome, declare_only) -> ReadBack`: `Observed → Graded`, `EmptyCorpus && declare_only → AutoResolved`, `EmptyCorpus | CallFailed(_) → Blocked`. `state_for` overrides state to `KnownResidual` when `observation.degraded`, leaving `verdict` independent.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — the harvest-tier shape: pure grading helpers (`harvest_since` / `is_target` / `num_field` / `observations` / `grade`) plus plain `#[test]`s asserting over **verbatim pinned leg captures** (`leg_b_discovery_line()` etc.). **No `#[ignore]`, no live-process gating** — the live leg runs once by hand, its lines are pinned into the test, and the test is CI-safe thereafter. Eleven sibling `*_harvest.rs` files ship this pattern.
- `crates/conductor-core/src/redact.rs` — `ALLOWLISTED_FIELDS` contains `mcp_tool`, `count`, `message`, `latency_ms`, `verdict`, `state` — but **not `incident_id`**. An incident identifier cannot ride a span attribute; it must go on the allowlisted `message` field or not be emitted.
- **Pulse** `crates/mcp-server/src/tools.rs` (445–475) — `dispatch_mark_incident_resolved`: applied ⇒ `Ok({"resolved": true, "incident_id": <id>})`; `DeclinedStale` ⇒ JSON-RPC **error** `ToolDispatchFailed`, reason `"incident changed concurrently; resolution not applied"`.
- **Pulse** `crates/corpus/src/contract.rs` (622–665) — the decisive arithmetic, below.
- **Pulse** `crates/mcp-server/src/tools.rs` (44–51, 934) — the eight wire tools; `mark_incident_resolved_writes_resolved_status` asserts in-process exactly the pairing this chunk asserts across the wire ("Re-read: status is now resolved, no longer in the active list").

## Patterns detected

- **Delegation-carries-instrumentation** (`client.rs:124`): a single `call_tool` wrapper owns the span and the `mcp_tool` field; new tool calls inherit observability by construction rather than by repetition.
- **Harvest-tier pin-then-grade** (`delegated_timing_harvest.rs:265-310`): the live measurement is captured once and frozen as a fixture; the committed test grades the *logic* against that fixture. This is what lets a live claim be CI-safe without faking the SUT.
- **State override, not verdict override** (`lib.rs:516`): degradation changes what a row MEANS (`state`), never what it measured (`verdict`) — the precedent for how a lifecycle outcome should be expressed without touching the closed enums.

## Conventions to follow

- Live claims grade at the harvest tier in `crates/conductor-run/tests/*_harvest.rs`, never as in-scenario `[[expected]]` checks (test-plan §6; eleven shipped examples).
- Live-leg firing form: `SCENARIO=<name|P-ID> [SEED=<n>] bash scripts/agent-run.sh run` — `--seed` rides **only** an explicitly set `SEED` (the TOML seed governs otherwise). `boot` writes **no** run artifacts, so a criterion needing a journal/`runs.db` row is satisfied only by a scenario leg.
- The live environment block must be set in ONE paste before launch — `ANDROMEDA_PULSE_MCP_ENABLED`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_L4_DETERMINISTIC` — because a partial set fails silently as `[BLOCKED]` in ~0s.

## New files to create

- `crates/conductor-run/tests/lifecycle_harvest.rs` — the applied-arm live capture pinned + graded, following `delegated_timing_harvest.rs`.

## Files to modify

- `crates/conductor-verify/src/client.rs` — *possibly none*. The method already exists and is already instrumented; the chunk may add only a caller. Listed because the declined-arm typing may want a small typed helper beside it.
- `crates/conductor-verify/tests/readback.rs` — the stub-side declined-arm assertion (the CI-runnable half). This file already holds two `query_incident_list` refs, so it is the established home.
- `crates/conductor-verify/tests/common/mod.rs` — the shared stub already lists `mark_incident_resolved` in its tool set (`:95`); a declined-arm response mode joins the existing `StubConfig`-style switches (`:54` is the precedent for an error-returning mode).
- **Caller threading:** none. The graph's `refs` set for `mark_incident_resolved` is empty, so nothing needs re-plumbing; the crate edge already exists; no `lib.rs` re-export is required (the method is already `pub` on a `pub` type reachable from `conductor-run`).

## Scope premise closure

1. ~~`[inferred]` the declined arm may not be inducible live (needs a concurrent modification)~~ → **`[premise-corrected: the guard is a monotonic-timestamp compare, not a race]`**. `update_incident_status` runs `UPDATE … WHERE id = ?5 AND updated_unix_nano <= ?2` and returns `DeclinedStale` when `rows == 0` (`corpus/src/contract.rs:652-659`); Pulse's own tests name it `update_incident_status_declines_a_write_older_than_the_stored_row`, with a sibling proving an **equal** timestamp *applies*. Since `dispatch_mark_incident_resolved` stamps `now = current_unix_nanos()` fresh on every call, the predicate can fail only if the stored row's `updated_unix_nano` lies in the **future** relative to the sidecar's clock. The declined arm is therefore **unreachable through the MCP surface by any ordinary means** — stub-only, for a sharper reason than "hard to race". This also **dissolves** the security extract's open question about a test-only corpus stager: no corpus staging is needed or wanted.
2. ~~`[inferred]` the leg's home is a harvest-tier test rather than a scenario TOML~~ → **VERIFIED**. Eleven `*_harvest.rs` files ship the pattern; arch and test-plan both require harvest tier for live claims; the tests are plain `#[test]`s over pinned captures, so no scenario TOML — and therefore no new P-ID binding, no `check_scenario_backing` movement, no `UNBACKED_AUTO` change.
3. ~~`[inferred]` resolving the canary's incident may perturb later preflight or scenario expectations~~ → **VERIFIED as real, with the mechanism named**. `query_incident_list` returns the ACTIVE set only, so a resolve empties it; `route_read_back` then sends a declare-only scenario to `AutoResolved` and a checks-bearing one to `Blocked`. Preflight runs *before* and is unaffected; the exposure is to anything reading back *after* the resolve in the same run. test-plan §6 already requires the creation↔first-active-set-drop mis-pairing be pinned as a negative test — the same hazard.
4. ~~`[inferred]` the disproof may touch architecture §Read-Back Dependency Posture and `residuals.md`~~ → **VERIFIED, and the site count is TWO in arch, not one**. The payload-fidelity/freshness-carrier posture is stated at both §Established Decisions [Read-Back Dependency Posture] and §Standard Contracts (readiness gate); the duplicate-occurrence precedent requires both to move together or the retired reading survives in the twin.
5. ~~`[inferred]` whether firing `mark_incident_resolved` changes a preflight assumption~~ → **VERIFIED: it does not**. The tool is in `contracts/mcp-contract.toml`'s `required_tools`, and preflight asserts *presence* only.

## Open questions

- Where is the applied arm's live evidence captured from — Conductor's own `logs/agent-latest.jsonl` (the `verify.readback.call_tool` line plus the raw `{resolved, incident_id}` result) or Pulse's `{data_dir}/logs/agent-latest.jsonl.<date>`? The delegated-timing precedent harvests *Pulse's* stream, but this chunk's evidence is Conductor-side, and `incident_id` is not allowlisted as a span attribute. → blocks: **plan-decision**.
- Does the leg resolve the **preflight canary's own** incident (no new emission needed, but it consumes the run's only incident) or drive a fresh one first? → blocks: **plan-decision**; interacts directly with closure 3.
