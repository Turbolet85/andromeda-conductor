# Codebase Research — 2026-08-10-scenario-run-root-span-tree

## Scope
- **Depth:** deep · **Reads:** 5 · **Globs/Greps:** 10 · **Graph queries:** 4 (trace: `.andromeda/runs/2026-08-10T22-02-24-phase/tree-query-2026-08-10-scenario-run-root-span-tree.json`)

## Headline finding — spans currently emit NOTHING

`JsonObsLayer` (`crates/conductor-core/src/obs.rs:239-276`) implements **exactly one** `Layer` trait
method: `on_event`. There is no `on_new_span`, no `on_close`, no `on_enter`, and the `Context`
parameter is bound as `_ctx` — **unused**. The layer never walks the span stack, so:

1. **No span produces a log line.** The three shipped spans (`timeline.execute`, `emit.batch`,
   `verify.readback.*`) are invisible in the artifact.
2. **Span fields never reach any line.** Events inside a span do not inherit its attributes, because
   nothing looks the span context up.

**Verified empirically, not inferred:** the committed `logs/agent-latest.jsonl` (5 lines) contains
**0** occurrences of `timeline.execute` / `emit.batch` / `verify.readback`. Every line is a flat
`on_event` record: 2× `conductor_verify::client`, 1× `conductor_core::obs`, 1× `conductor_core::capability_manifest`, 1× `conductor_run`.

**Consequence for this chunk:** adding `#[tracing::instrument(name = "scenario.run", …)]` changes
**nothing observable**. obs-plan §4 CP1's acceptance — "`scenario.run` is an ancestor of
`timeline.execute`…" — is unreachable until the subscriber layer learns spans. The chunk's real work
is therefore **layer capability + three spans**, not three spans alone.

The foundation is present: `build_subscriber` (`obs.rs:146-157`) already stacks on
`Registry::default()`, which stores span data — only the layer declines to read it.

## Graph impact (from the code-graph query)

- **`execute_scenario`** — 3 production callers + 4 test callers:
  `conductor-cli commands/run/run()` @ `crates/conductor-cli/src/commands/run.rs:23` ·
  `conductor-cli commands/suite/suite()` @ `crates/conductor-cli/src/commands/suite.rs:30` ·
  `conductor-run drive_run()` @ `crates/conductor-run/src/lib.rs:481` (the Tauri path).
  **All three production paths funnel through it** — one span here does cover CLI `run`, CLI `suite`,
  and the GUI.
- **`persist`** — 5 production call sites: `run.rs:25` · `suite.rs:36` · `drive_run()` @ `lib.rs:477`
  (abort path) and `lib.rs:484` (normal path), plus the `commands/mod.rs:17` / `run.rs:9` / `suite.rs:10`
  import rows. **`persist` is a SIBLING of `execute_scenario`, never nested inside it.**
- **`drive_run`** — 1 production caller: `conductor-tauri commands/run_thread()` @
  `crates/conductor-tauri/src/commands.rs:260` (+ 4 test callers).
- **`crate_edges` for `conductor-report`** — outbound `conductor-report → conductor-core`; inbound
  from `conductor-cli` and `conductor-run`. Not a leaf, but adding `tracing` is internal and additive:
  **zero public-API blast radius**.

## The structural problem the graph exposes

obs-plan §4 CP1 requires the root to **close on `db.insert_run` completion**. But `db.insert_run`
lives inside `persist`, and `persist` is called by `execute_scenario`'s *callers*:

```
run.rs:24   records = execute_scenario(...)   <- scenario.run would live here
run.rs:26   persist(...)                      <- report.generate + db.insert_run live here
```

A root span wrapping only `execute_scenario` **excludes both terminal spans** — the chain breaks at
exactly the point obs-plan says it must close.

`suite` makes it structurally worse: `execute_scenario` runs in a **loop** (`suite.rs:30-34`) and
`persist` fires **once** afterwards (`suite.rs:37`) for all N records. So for N>1 there is no single
`scenario.run` that can contain `db.insert_run` — one persist serves N scenario spans that have
already closed. `drive_run` has the same shape (loop at `lib.rs:476-484`, persist at `:485`), plus a
second persist on the abort path (`:478`).

**This is the chunk's central design decision and it needs a P4 resolution.**

## Files inspected
- `crates/conductor-core/src/obs.rs` (146-157, 230-305) — `build_subscriber` layer stack; `JsonObsLayer` with only `on_event`; `JsonVisitor` gating every field on `is_allowlisted`.
- `crates/conductor-core/src/redact.rs` (1-60) — the 21-entry `ALLOWLISTED_FIELDS` const and `is_allowlisted`.
- `crates/conductor-run/src/lib.rs` (380-510) — `persist` (`:388`, the only site touching both report writers), `classify_run`, `drive_run` (`:459`) with its two persist calls.
- `crates/conductor-cli/src/commands/run.rs` (full) — single-scenario path: `execute_scenario` `:24` then `persist` `:26`.
- `crates/conductor-cli/src/commands/suite.rs` (full) — N-scenario loop `:30-34`, one `persist` `:37`.

## Allowlist blocker — `row_count` would be silently dropped

`ALLOWLISTED_FIELDS` (`redact.rs:21-47`) carries 21 names. Of the attributes obs-plan §4 CP1 requires:

| Attribute | Allowlisted? |
|---|---|
| `run_id` `seed` `scenario` `p_ids` `verdict` `state` | **yes** (`redact.rs:29,39-43`) |
| `row_count` | **NO** |

A non-allowlisted field name is **dropped at the processor stage** (`JsonVisitor`, `obs.rs:280-305`) —
silently, with no error. So `db.insert_run`'s `row_count` would vanish. Two ways out: add `row_count`
to the allowlist, or emit the already-allowlisted `count` (`redact.rs:33`). P4 decides; the allowlist
edit is the more faithful reading of obs-plan, and `count` is already the name the Tauri-command
precedent uses.

## Patterns detected
- **Manual `info_span!(...).entered()` for Tauri commands** (`crates/conductor-tauri/src/commands.rs:113,134,150,168,205,281`; `pause.rs:97`) — mandated by `.claude/rules/observability.md` (2026-06-26): the `#[tracing::instrument]` *attribute* does not stack cleanly with `#[tauri::command]`.
- **`#[tracing::instrument]` attribute at seam fns** (`conductor-timeline/src/scheduler.rs:34`; `conductor-emit/src/client.rs:67,104`; `conductor-verify/src/client.rs:45-125`) — the in-crate convention for ordinary async/sync fns.
- **`persist` is the single composition point** (`conductor-run/src/lib.rs:388-403`) reaching all three artifacts: `JournalWriter::create`, `RunsDb::open`/`insert`/`insert_envelope`, `RunReport::write`. Both terminal spans belong inside it or inside the two report-seam fns it calls.
- **Env read at the caller, never inside the gate** (`.claude/rules/testing.md`, 2026-08-10) — the shipped precedent for keeping instrumented logic a pure function of its inputs.

## Conventions to follow
- **Bounded span-name set** (`.claude/rules/observability.md`): `scenario.run`, `timeline.execute*`, `emit.batch`, `emit.logs_batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `fault.*`, `tauri.command.*`. All three names this chunk adds are already members — **a new name (e.g. a run-level parent) would need an obs-plan amendment.**
- **Two record shapes** (`.claude/rules/observability.md`): the self-obs base line vs the Run-report envelope. Span records join the former; the envelope is untouched.
- **`run_id` is the correlation key on every line** — already injected flat by the layer from `ServiceIdentity` (`obs.rs:265-268`), so a `run_id` span attribute would duplicate a field the layer already sets.
- **rusqlite bound parameters only** on the `RunsDb::insert` path (`.claude/rules/security.md`).

## New files to create
- (none) — all work is edits to existing files.

## Files to modify
- `crates/conductor-core/src/obs.rs` — teach `JsonObsLayer` to emit span records and/or attach span context to events (the enabling change; without it nothing else is observable).
- `crates/conductor-core/src/redact.rs` — allowlist `row_count` (unless P4 picks `count`).
- `crates/conductor-run/src/lib.rs` — the `scenario.run` root at/around `execute_scenario` (`:253`) and whatever the P4 decision requires around `persist` (`:388`).
- `crates/conductor-report/src/report.rs` — `report.generate` at `RunReport::write` (`:43`).
- `crates/conductor-report/src/db.rs` — `db.insert_run` at `RunsDb::insert` (`:77`).
- `crates/conductor-report/Cargo.toml` — add `tracing.workspace = true` (current `[dependencies]`: `conductor-core`, `rusqlite`, `serde_json`, `thiserror`).
- **Caller threading (from the graph, not memory)** — if the root span must enclose `persist`, these three production sites each need the span applied: `crates/conductor-cli/src/commands/run.rs:24-26`, `crates/conductor-cli/src/commands/suite.rs:30-37`, `crates/conductor-run/src/lib.rs:476-485`. `crates/conductor-tauri/src/commands.rs:260` calls `drive_run` and inherits whatever it does.

## Open questions
- **The root cannot enclose `db.insert_run` under the current call structure** — `persist` is a sibling of `execute_scenario`, and under `suite`/`drive_run` one `persist` serves N scenarios. Does `scenario.run` stay per-scenario (chain not contiguous, obs-plan CP1 "closes on db.insert_run" unmet as written), or does the chunk restructure so a run-scoped span encloses both? → blocks: **plan-decision**.
- **Does this chunk own the subscriber-layer span capability?** It is a prerequisite for any observable span tree, but it is a `conductor-core` obs-infrastructure change the working entry never named. → blocks: **plan-decision**.
- **`row_count` allowlist edit vs reusing the allowlisted `count`.** → blocks: **implementation-scope**.
