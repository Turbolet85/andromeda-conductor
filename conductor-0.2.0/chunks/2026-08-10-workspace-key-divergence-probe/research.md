# Codebase Research — 2026-08-10-workspace-key-divergence-probe

## Scope
- **Depth:** deep · **Reads:** 8 (6 Conductor, 2 Pulse) · **Globs/Greps:** 6 · **Graph queries:** 2

Two bodies of code matter here: Conductor's preflight seam (where the precondition lands) and Pulse's
workspace-key derivation (the thing being probed). Intent F10 explicitly marks its Pulse claims as
*code read, verify live before designing around it* — so the Pulse half was re-verified first-hand, and
that verification changed the chunk's shape.

## Files inspected

**Conductor**
- `crates/conductor-verify/src/preflight.rs` (full, 326 lines) — the readiness gate. The precondition
  cascade is a single `if/else if` chain at `preflight.rs:153-175` producing `Option<String>`, with
  `ready = blocked_precondition.is_none()` at `:177` and one boundary log at `:179`. Four existing
  branches: version mismatch · tools unverifiable · tools absent · canary call-error · canary not-Ok.
  The generic string this chunk replaces is at `:171` / `:251` / `:277`
  (`"canary round-trip failed: incident not found in corpus"` — three literal copies).
- `crates/conductor-verify/src/spawn.rs` (full) — `resolve_data_dir` / `platform_default` /
  `build_command`. `DATA_DIR_ENV` (`:18`) and the `.env(...)`-only hardening are intact; this is the
  single place Conductor knows the sidecar's key.
- `crates/conductor-run/src/lib.rs:1-200` — the composition root. `preflight()` (`:48`) and
  `readiness()` (`:70`) each read `ANDROMEDA_PULSE_DATA_DIR` from env (`:49`, `:71`) and hand the
  string down; `canary_gate()` (`:112`) calls `run_preflight` (`:128`).
- `crates/conductor-cli/src/commands/preflight.rs` (full, 33 lines) — prints
  `blocked_precondition` verbatim after the `[BLOCKED]` prefix (`:26-30`) and exits non-zero when
  `!ready` (`:32`). Nothing here needs to change for a new precondition VALUE.
- `crates/conductor-verify/tests/common/mod.rs` (full) — `StubConfig`, one knob per readiness leg.
- `crates/conductor-verify/tests/preflight.rs:1-60` — `drive_with(config, manifest) -> ReadyState`
  (`:22`), which passes the literal `"/test/data-dir"` as the data-dir (`:27`).

**Pulse (read-only verification of intent F10)**
- `crates/corpus/src/contract.rs:628` (+ `:675`, `:795`) — `WHERE workspace = ?1`. **CONFIRMED.**
- `pulse-app/src/main.rs:688-693` → `pulse-app/src/digest_runtime.rs:109-127` —
  `resolve_workspace_for_incidents(detected, data_dir)` returns the detected root's
  `ctx.root.to_string_lossy()` when detection succeeds, `data_dir` only when it fails. **CONFIRMED.**
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:74` — `let workspace_root =
  data_dir.to_string_lossy().to_string();`, above a comment still claiming parity with
  `main.rs`. **CONFIRMED, and the comment is confirmed stale.**
- `crates/workspace-detector/src/detect.rs:21-57` — the detection rule itself (see the third finding).

## Graph impact (`.andromeda/runs/2026-08-10T17-37-46-phase/tree-query-2026-08-10-workspace-key-divergence-probe.json`)

- **`ReadyState`** — 80 reference rows across 5 files (`rows: 80`, `db_state: fresh`). Construction sites are only three:
  `conductor-verify/src/preflight.rs:181` (the gate's own), `preflight.rs:213` (`preflight_boot`'s
  unreachable arm), and `conductor-run/src/lib.rs:88` + `:167` (`unreachable_state` /
  `canary_blocked_state`). Every other row is a field read. **A new precondition VALUE therefore
  threads nowhere** — it is a string assigned inside an existing construction, so the caller set is
  not impacted. This is the difference between adding a value and adding a field.
- **`run_preflight`** — 3 production callers: `conductor-run/src/lib.rs:127` (`canary_gate`),
  `conductor-verify/src/preflight.rs:206` (`preflight_boot`), and the re-export at
  `conductor-verify/src/lib.rs:43`; plus `tests/preflight.rs:26`. `preflight_boot` has one caller,
  `tests/preflight_spawn.rs:8`.
- **No inbound crate edge is added.** `conductor-run → conductor-verify` already exists
  (`lib.rs:34-37` imports `run_preflight`/`ReadyState`/`ToolPresence`/`CanaryOutcome`), so anything
  this chunk exports from `conductor-verify` reaches the composition root over an existing edge.
- **`run_preflight`** row count: 9 (`db_state: fresh`). Both queries copy-adapted cookbook shape 1.

## Patterns detected

- **Named-precondition const** (`preflight.rs:99`): `UNREACHABLE_PRECONDITION` is a private `const
  &str` with a doc comment citing arch §Standard Contracts. **It is duplicated verbatim** as a second
  private const at `conductor-run/src/lib.rs:87-88` — a copy, not a shared export. Any new
  precondition needed on both sides inherits that choice unless it is exported.
- **Redact-before-precondition** (`preflight.rs:295` `call_error_reason`, and `:114`
  `redact_value(data_dir)`): every path-bearing value passes `conductor_core::redact_value` before it
  can become a precondition or a `ReadyState` field. `data_dir` is *already* redacted into the
  readiness result at `:114`, so the existing field is not a leak vector.
- **One boundary log, not per-branch** (`preflight.rs:179`): a single
  `tracing::info!(state = …, "preflight blocked: {precondition}")` covers every branch. A new branch
  gets its log coverage for free; a second log statement would double-count the boundary event.
- **Stub knob per leg** (`tests/common/mod.rs:11-22`): each readiness leg is one `StubConfig` bool
  (`canary_in_corpus`, `query_errors`), consumed by a single `match` arm in `serve_stub`.
- **The distinct-cause precedent** (`canary_blocked_state`, `conductor-run/src/lib.rs:167`): when a
  failure has a nameable cause, it gets its own `ReadyState` builder and its own precondition string
  rather than falling through to the generic one. That is exactly the move this chunk makes.

## Conventions to follow

- **Blocked is a value, not an error**: every branch returns `Ok(ReadyState)`
  (`preflight.rs:182`, `:214`); the only `Err` is a harness fault (a missing contract manifest,
  `conductor-run/src/lib.rs:117`).
- **Preconditions name env/feature/condition, never paths** (`preflight.rs:99-100`): the shipped
  string enumerates `mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED` + the data-dir equality *as a
  condition*, carrying no path value.
- **Test the leg through `drive_with`** (`tests/preflight.rs:22`), asserting `report_state()` +
  `blocked_precondition`, never the private cascade.
- **The data-dir is read from env at the composition root, not in the seam** (`lib.rs:49`, `:71`) —
  `conductor-verify` receives it as a `&str` parameter.

## New files to create
- (none expected) — the deliverable is a precondition value, a detection branch, and test legs, all
  in existing files. A live-probe verdict record is an artifact question, see Open question 2.

## Files to modify
- `crates/conductor-verify/src/preflight.rs` — the new named precondition const + its branch in the
  `:153-175` cascade.
- `crates/conductor-verify/src/lib.rs` — re-export only if the const must be shared with
  `conductor-run` (the `UNREACHABLE_PRECONDITION` duplication precedent says the current answer is
  "not necessarily"; see Open question 3).
- `crates/conductor-verify/tests/common/mod.rs` — one `StubConfig` knob for the divergence leg.
- `crates/conductor-verify/tests/preflight.rs` — the new leg + the no-host-path negative assertion.
- `crates/conductor-run/src/lib.rs` — **only if** detection needs the composition root's knowledge of
  the resolved data-dir / a candidate root (Open question 1). Its two Blocked-`ReadyState` builders
  (`:86`, `:167`) are the sites that would carry the value.
- Caller threading: **none**. Per the graph, `ReadyState` gains no field, so its 84 reference rows and
  the 3 `run_preflight` callers are untouched.

## Open questions

1. **How does Conductor DETECT the divergence rather than merely predict it?** → blocks:
   **plan-decision**. Three facts close off the obvious routes:
   (a) `dispatch_query_incident_list(ctx)` takes **no arguments** (`mcp-server/src/tools.rs:174`,
   `:336`) and filters on `ctx.workspace_root` alone — so Conductor **cannot probe a second workspace
   key** through read-back; (b) Pulse **never logs** `incident_workspace_key` (its only uses are
   `main.rs:688`, `:701`, `:728`, `:738`, `:1017`) — so it cannot be read from a log either; (c) the
   corpus is out of bounds by rule. What remains is inference on Conductor's side: replicate Pulse's
   rule over a candidate root and compare against the data-dir the sidecar is keyed to. That is
   honest only if the precondition states its own conditionality, and it couples Conductor to a Pulse
   algorithm. P4 must choose between that, a re-worded precondition on the zero-incident path, and an
   explicitly-declared-expected-key assertion.
2. **The intent's interim unblock is falsified — what do the two launches actually test?** → blocks:
   **plan-decision**. `workspace_detector::detect` (`detect.rs:21-57`) canonicalizes the candidate and
   then records `has_andromeda_marker` and `vcs` as **fields on the returned context**, never as
   success conditions; it returns `Err` only on a traversal component or a non-existent / IO-failing
   path. So detection succeeds for **virtually any real cwd**, `pulse-app` keys incidents to its
   *canonicalized cwd* almost always, and the `data_dir` fallback is effectively unreachable from a
   normal launch. Consequently a "marker-less temp dir" launch keys to the temp dir — **not** to
   `data_dir` — and both arms of the specified two-launch check should come back blocked. The only
   candidate agreeing position is `cwd == data_dir`, and even that is doubtful on Windows, where
   `canonicalize` returns the `\\?\` verbatim prefix while the sidecar uses the raw `data_dir` string.
   P4 must decide whether the probe keeps its two specified arms, gains the `cwd == data_dir` third
   arm, and where the verdict is recorded.
3. **Does the new precondition need to be shared with `conductor-run`, or stay private to the gate?**
   → blocks: **implementation-scope**. `UNREACHABLE_PRECONDITION` set the precedent of duplicating the
   literal across both crates rather than exporting it. If detection lives wholly inside
   `run_preflight`, nothing is shared and `conductor-run` is untouched; if the composition root must
   emit it (because it holds the candidate root), the const has to be exported or duplicated — and
   duplicating a *second* precondition would make the pattern a standing liability rather than a
   one-off.

## Feasibility notes (for the live leg)
- `D:\dev\projects\andromeda-pulse\target\debug\pulse-app.exe` exists — the two-launch probe is
  runnable on this host without a fresh release build.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` is live Pulse-side (`pulse-app/src/deterministic_inference.rs:18`,
  logged at `main.rs:465`), so the probe can be run with the LLM removed as an explanation. Asserting
  that flag as a *contract* belongs to the next Epoch-2 entry (F11 / P-073), not here.
