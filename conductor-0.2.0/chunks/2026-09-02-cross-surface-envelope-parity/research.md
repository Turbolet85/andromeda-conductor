# Codebase Research — 2026-09-02-cross-surface-envelope-parity

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 16 · **Graph queries:** 4 (rust plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read IN FULL incl. all 21 `## Session
  Additions` (the live-leg firing form accumulates there; the 2026-08-16 entry warns that a route entry is a
  point-in-time citation while this file accrues). Also auto-loaded and read: `.claude/rules/testing.md`,
  `.claude/rules/observability.md` (both cover `crates/**/src/**/*.rs` + `crates/**/tests/**`).
  **Retrieval caveat confirmed:** this rule's `paths:` do NOT cover `crates/conductor-tauri/ui/test/`
  (singular `test`), so the webview half of this chunk would not auto-load it — the read is deliberate here.

## Files inspected
- `crates/conductor-core/src/load_envelope.rs` (§160-190 `phase_breach` / `phase_rate_exceeds`, §190-250
  `classify`, §300-330 `check_load_envelope`) — the breach arithmetic and the two envelope surfaces.
- `crates/conductor-core/src/phase_spec.rs` (:35-50, :103-105) — `gap_ms` bounded only by
  `range(max = MAX_GAP_MS)` (no minimum, so `0` is expressible); `occurrences` bounded
  `range(max = MAX_OCCURRENCES)` with `MAX_OCCURRENCES = 10_000` (:29).
- `crates/conductor-run/src/lib.rs` (:824-861 `persist` / `read_envelope`, :863-878 `classify_run`,
  :905-949 `drive_run`) — the composition root both shells call.
- `crates/conductor-cli/src/commands/run.rs` (full) — the CLI verb's ordering.
- `crates/conductor-tauri/src/commands.rs` (:222-269, :346, :471) — the `run_envelope` command, `classify_run`
  in `start_run`, and the two existing cross-surface-parity comments.
- `crates/conductor-tauri/ui/wdio.conf.ts` (:66-98 fixture + suite table, :196-260 `onPrepare`) — the ONE
  tauri-driver spawn site and its per-suite `CONDUCTOR_RUNS_DIR`.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (:238-292) — the banner spec, already written.
- `crates/conductor-run/tests/lamps_fixture.rs` (:1-25) — the committed-fixture round-trip precedent.
- `contracts/pulse-load-envelope.toml` (full) — the pinned terms and the empty `[[exempt]]` ledger.
- `crates/conductor-verify/src/lib.rs` (:1-14) — the false rmcp doc claim.
- `contracts/mcp-contract.toml` (:1-12) — the CARRY-named stale header comment.

## Graph impact (rust plane, `db_state: fresh`)
- **`read_envelope`** — `symbol` view HAS it (`conductor-run 0.1.0 read_envelope().`, `lib.rs:851`), yet the
  `calls` view returns **rows=0**. Both cookbook preconditions for a real "no callers" reading are met (plane
  built, symbol indexed) — but grep finds a genuine caller at `conductor-tauri/src/commands.rs:243`, inside a
  `#[tauri::command]` body. **Basis stated: the graph is INCOMPLETE here (macro-expanded call context), not
  evidence of a leaf.** The cookbook's own corollary covers this — zero callers on an IPC-facing symbol is not
  dead-code evidence. Caller enumeration for this chunk therefore rests on grep, not the graph.
- **`get_envelope`** — one definition (`conductor-report db/impl#[RunsDb]get_envelope()`, `db.rs:221`).
- **`crate_edges`** (12 rows) — `conductor-tauri → {conductor-core, conductor-run}` only; **no
  `conductor-tauri → conductor-report` edge**, which is why `read_envelope` lives in `conductor-run`.
  `conductor-run → {conductor-core, conductor-faults, conductor-report, conductor-timeline, conductor-verify}`.
- **`check_load_envelope`** — grep across all `crates/**/*.rs`: referenced ONLY by its own unit tests
  (`load_envelope.rs:480,556,613,622,631,640,647`) and the `conductor-core/src/lib.rs:44` re-export. **Zero
  production callers.** The gate is enforced as a unit test over the committed catalog (`:556`).

## Patterns detected
- **Envelope judged before driving, persisted unconditionally** (`run.rs:20-27`; `lib.rs:937,946`): both shells
  compute `classify_run` from scenario shape + the pinned contract — a pure function, no SUT involvement — and
  `persist` ends with `db.insert_envelope` on every path including abort. This is the mechanism that decides
  the CARRY-2 fork.
- **Exact-integer breach math** (`load_envelope.rs:163-168`): `occurrences * 1000 > max_rate * gap_ms`, and
  `gap_ms == 0` returns `true` unconditionally ("a zero-length window declaring emissions is an unbounded
  rate"). At the pinned `10000`, the breach condition reduces to **`occurrences > 10 × gap_ms`**.
- **Committed-fixture seeding** (`wdio.conf.ts:78-86` + `lamps_fixture.rs`): the harness re-creates
  `runs/e2e-fixture` clean and copies a committed JSONL to `<FIXTURE_RUN_ID>.jsonl`; a Rust test round-trips
  the same file through the production reader and asserts its SEMANTICS. **The journal-only seed cannot create
  a `run_envelope` row — that is a `runs.db` table, which is exactly why the banner spec context-skips.**
- **Per-suite subject env at ONE spawn site** (`wdio.conf.ts:241-260`): `CONDUCTOR_RUNS_DIR` is chosen per
  invoked suite (`runs/e2e-fixture` routine · `runs/driven/runs` driven · `runs/sr-leg/runs` SR), all
  repo-relative because `resolve_under` rejects absolute handles.
- **The banner spec already exists** (`accessibility.e2e.ts:270-280`): it expects
  `report__envelope-label` to read `ENVIRONMENT-SUSPECT` and `this.skip()`s with a stated
  subject-absent reason. The work is to supply the SUBJECT, not to author the spec.

## Conventions to follow
- **Live-leg firing form** (assembled from `verification-harness.md` Session Additions, not improvised):
  fresh data dir per leg under `%TEMP%/pulse-legs/<ts>` (2026-08-18); **never `boot` before a leg that fires
  its own preflight** — including a wdio driven leg (2026-08-19a, extended 2026-09-01: two 12-minute runs lost
  to exactly this); a **quiet window of ≥120s Pulse idle + a 30s resolver tick** after any preceding canary;
  `PATH` in POSIX form resolving `andromeda-pulse-mcp`, verified with `which` before the leg (2026-08-20 — a
  `[BLOCKED]` in ~0s is a sidecar-resolution failure, not a SUT verdict); `ANDROMEDA_PULSE_MCP_ENABLED=true` +
  `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` in Conductor's OWN env; the whole env block set in ONE paste before
  launch (2026-08-22c — a partial set fails silently); and a **process census before and after** with each
  survivor's stop form named (2026-09-02, operator directive).
- **Fixture rule, three parts** (`testing.md` 2026-09-02): seed from the harness, keep record CONTENT in a
  committed file under `crates/`, and pin its MEANING with a production-reader round-trip — "a parse failure is
  loud, a semantic drift is not". Build it so the assertion CAN fail.
- **Assert on rendered findings, never a bare count** (`testing.md` 2026-09-01): a gate's failure must say what
  it observed.
- `tauri::test` mock-runtime shape (`testing.md` 2026-06-27): `mock_builder()` + `mock_context(noop_assets())`
  + `get_ipc_response`, request `url` MUST be `"http://tauri.localhost"`; the in-process Blocked spine is forced
  with `ANDROMEDA_PULSE_DATA_DIR="pulse;injection"` — the same lever `cli_smoke` uses in-child.
- Manual `tracing::info_span!("tauri.command.*").entered()` in every command; **no `db.*` read span** for the
  envelope read (`observability.md`; obs-plan §6 operator ruling).

## New files to create
- `crates/conductor-run/tests/fixtures/<over-envelope scenario>.toml` — the variant scenario whose emitting
  phase breaches the rate term (`gap_ms` small, `occurrences > 10 × gap_ms`), committed as reviewable content.
- A parity integration test (crate + exact path is P4's; `crates/conductor-tauri/` has **no `tests/` dir**
  today, and the mock-runtime tier lives inline in `commands.rs` under `#[cfg(test)]` because the crate is a
  bin with no lib — so an out-of-crate `tests/` file is not available to it).
- An envelope-subject fixture pin, in the `lamps_fixture.rs` shape.

## Files to modify
- `contracts/mcp-contract.toml` (:5) — the CARRY-named stale rmcp header clause.
- `crates/conductor-verify/src/lib.rs` (:3) — "the rmcp client/transport foundation", false at HEAD.
- `crates/conductor-tauri/ui/wdio.conf.ts` (:78-86 `seedFixtureRuns`, and/or the `onPrepare` env block) — to
  seed the `run_envelope` subject the journal copy cannot create.
- `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` (:270-280) — only if the subject's arrival needs
  the skip-guard re-pointed; the assertion itself already exists.
- **Caller threading:** `read_envelope`'s only caller is `conductor-tauri/src/commands.rs:243` (grep basis —
  the graph cannot see it). `classify_run`'s callers are `conductor-cli/src/commands/run.rs:21`,
  `conductor-cli/src/commands/suite.rs:25`, `conductor-tauri/src/commands.rs:269`. `persist`'s are
  `conductor-cli/src/commands/run.rs:27` and `conductor-run/src/lib.rs:937,946`. **No signature change is
  planned**, so these are read-context, not a threading set.
- **Seam facts:** no new dependency is implied. `conductor-tauri` has no `conductor-report` edge and must not
  gain one (the projection stays through `conductor-run`). Any test-only dep would be `[dev-dependencies]`,
  which cannot back a shipped signature — none is needed here.
- **Spec masters are NOT touchpoints:** `.andromeda/test-plan.md`'s 24 rmcp lines are an **Expected amendment**
  routed to wrap, never an edit by phase or implement.

## Open questions
1. Does the operator want the live-Pulse GUI leg given that the banner subject provably does not require it
   (a production-writer seed is deterministic, CI-reachable and satisfies test-plan §7)? → blocks:
   **plan-decision** — P4 asks, per the directive's "plan the live leg, then ask".
2. Where does the v2-25 parity test live, given `conductor-tauri` is a bin with no lib and no `tests/` dir, so
   the mock-runtime tier is inline `#[cfg(test)]` while the `assert_cmd` CLI leg needs a subprocess? →
   blocks: **implementation-scope** — the file list above is provisional on it.
3. Which of `test-plan.md`'s 24 rmcp lines describe the CURRENT client (amend) vs the vulnerability CLASS or
   rejection rationale (leave standing)? → blocks: **implementation-scope** — the per-site pass runs over the
   count, not over either partial enumeration, and its output is the wrap amendment's payload.
