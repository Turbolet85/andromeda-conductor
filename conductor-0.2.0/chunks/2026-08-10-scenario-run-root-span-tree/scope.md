# Scope — scenario.run root span tree

**Marker:** `2026-08-10-scenario-run-root-span-tree`
**Version:** conductor-0.2.0 · **Epoch 2 — Live-path enablement**
**Working entry:** _scenario.run root span tree — the must-trace self-observation spans beneath each scenario run_

## What this builds

The **root of Conductor's self-observation span chain** and the two spans that close it. `obs-plan.md` §4
Critical Path 1 mandates the chain `scenario.run` (root) → `timeline.execute` → `emit.batch` →
`verify.readback` → `report.generate` → `db.insert_run`. Three of the six exist; **three do not**, and the
missing ones are the two ends — the root that would parent the whole tree, and the terminal pair that closes it.

Verified in-tree this session:

| Span | State | Site |
|---|---|---|
| `scenario.run` (root) | **ABSENT** — no `scenario.run` / `scenario_run` anywhere in `crates/*/src/` | — |
| `timeline.execute` | present, `phase_count` | `conductor-timeline/src/scheduler.rs:34` |
| `emit.batch` | present, `emission_count` | `conductor-emit/src/client.rs:67` |
| `verify.readback.*` | present (connect · list_tools · call_tool · preflight) | `conductor-verify/src/client.rs:45,54,70,108,125` · `preflight.rs:133,233` |
| `report.generate` | **ABSENT** | — |
| `db.insert_run` | **ABSENT** | — |

Because the root is absent, the three existing spans are **orphans at the trace level**: they are emitted, but
nothing parents them into a per-run tree, so a run's self-observation cannot be reassembled from the log. This
chunk makes the tree real.

> **AMENDED at P5 (intent-incomplete — planning falsified a premise this scope stated).** The sentence above
> says the three existing spans "are emitted". **They are not.** `JsonObsLayer`
> (`crates/conductor-core/src/obs.rs:239-276`) implements only `on_event` — no `on_new_span`, no `on_close`,
> and the `Context` parameter is bound `_ctx`, unused — so **no span produces any output at all**. Verified
> empirically, not inferred: the committed `logs/agent-latest.jsonl` contains **0** occurrences of
> `timeline.execute` / `emit.batch` / `verify.readback`. The spans are not orphaned in the tree; they are
> absent from the artifact.
>
> Consequence: adding the three missing spans would change **nothing observable**, and none of this chunk's
> acceptance criteria would be verifiable. The enabling prerequisite — teaching the subscriber layer to emit
> span lifecycle records, plus widening the `conductor-core::redact` allowlist to the field names the emitted
> spans carry — is therefore **in scope**, confirmed by the operator at P4. See `plan.md` Steps 1-2.

## The CARRY this chunk absorbs

Folded from the working entry (assigned by the operator at the 2026-08-10 wrap):

- **The two orphaned `conductor-report` spans are this chunk's, not a future chunk's.** `crates/conductor-report/src/`
  emits **zero** `tracing::` calls across all five files (`coverage.rs` · `db.rs` · `journal.rs` · `lib.rs` ·
  `report.rs`) — confirmed by grep — while obs-plan §4 Critical Path 1 mandates `report.generate` → `db.insert_run`
  in the chain.
- `playbook.md` (2026-06-20/21) recorded both as route-sequenced to land "with the Epoch-8 cli/timeline caller".
  **Epoch 8 is complete and they never landed** — the deferral outlived its promised owner while being tracked
  only in a `note` field, which is not an owning channel.
- **Do NOT re-dismiss the `D-obs-instrumentation` detector on `conductor-report` as build-sequencing** — the
  sequencing already ran out. This entry's scope covers the report seam, not just the run root.

## The PREREQ this chunk carries

- **Re-check `cargo audit` — EIGHTH consecutive, deferred since `2026-08-08-sut-capability-manifest`.**
  Operator-**RATIFIED** at the 2026-08-10 wrap under the L5 age trigger: the bounded wait continues and
  **re-pins silently** — no further ratification HALT. Re-proven last chunk on **0.22.2** (the latest published):
  byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 — an advisory-**DATABASE** fault with
  nothing to raise a floor to.
- Remedy is the **bounded wait alone**: re-run it, record the result, and verify `cargo deny check` ran green as
  the overlapping signal. **Do NOT** raise the floor, **do NOT** add a `deny.toml` ignore, **do NOT** edit CI.
  Close the deferral the moment it parses.
  (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09/-08-10 · security-plan §Dependency Security.)

## Surfaces and contracts touched

- **`conductor-run`** — the composition root where a scenario run is actually composed; verified anchors
  `execute_scenario` (`src/lib.rs:253`), `persist` (`:388`), `drive_run` (`:459`). Shared by both bins, so one
  root span here yields CLI↔Tauri parity for free.
- **`conductor-report`** — the report seam; verified anchors `Report::write` (`report.rs:43`) and
  `RunsDb::insert` (`db.rs:77`).
- **Self-obs sinks** — `logs/agent-latest.jsonl` (agent mode) and `logs/conductor-tauri.jsonl` (GUI), via
  `conductor_core::ObsSink`. Unchanged as artifacts; they gain the span fields.
- **The redaction boundary** — obs-plan §6/§11 + security-plan §Error Handling: the tracing field-allowlist and
  the `anyhow` edge. Every new span field must clear it (no host paths, no internal struct names).

## Required span attributes (obs-plan §4 Critical Path 1)

- `scenario.run` — `run_id`, `seed`, `scenario`, `p_ids` (array)
- `report.generate` — `verdict`, `state`
- `db.insert_run` — `row_count` (1)

## Boundaries — what this chunk does NOT do

- **Not an OTel SDK, and never an OTLP export.** Self-observation is `tracing` JSON to stdout/file; the ONLY
  OTLP is the PRODUCT fault stream to Pulse `:4317`. This invariant is universal and non-negotiable here.
- **Does not re-author the three existing spans' identity.** `timeline.execute` / `emit.batch` /
  `verify.readback.*` keep their names and sites; this chunk parents them, it does not rebuild them.
- **Does not add the per-scenario span variants** obs-plan §4 lists for the other five must-trace scenarios
  (`timeline.execute_fingerprint_storm`, `verify.readback_auto_resolve`, `report.classify_known_residual`, …) —
  those ride their own live-proof chunks in Epochs 3–4.
- **No engine/verdict model change.** `Verdict` / `ReportState` / the run-report envelope / `runs.db` columns are
  untouched — this is instrumentation over the existing values.
- **Does not build the obs CI conformance gate** — it shipped in `2026-06-27-obs-ci-conformance-gate`.

## P4/P5 operator decisions (recorded)

- **Layer scope** — the chunk emits **span lifecycle records** (`on_new_span` + `on_close`), making spans
  first-class JSON lines with a parent link. Chosen over folding span context onto events, and over shipping
  the three annotations alone (which would emit nothing).
- **Root placement** — `scenario.run` is **per-scenario at the composition root** (`execute_scenario`), one
  site serving all three production paths. `report.generate` / `db.insert_run` are emitted inside `persist`
  as **siblings correlated by `run_id`**, because the call graph makes nesting impossible: `persist` is called
  by `execute_scenario`'s callers at all three sites, and under `suite`/`drive_run` one `persist` serves N
  scenarios whose spans have already closed. Chosen over wrapping both calls at the CLI edge (loses the
  instrument-once property) and over a new run-level parent span name (needs a bounded-set amendment).

## Inferred — not stated by the working entry; P4 re-verified these first

- **[inferred → CONFIRMED]** The root span's home is `conductor-run::execute_scenario` rather than either
  binary edge — chosen because it is the composition root both bins already call, so the tree is identical
  headless and under Tauri. **Confirmed by the code-graph:** all three production callers funnel through it
  (`run.rs:23`, `suite.rs:30`, `drive_run() lib.rs:481`), so no call site needs editing. The trailing guess
  that `persist` "may need to sit inside the same span" is **falsified** — it is a sibling and cannot.
- **[inferred → CONFIRMED]** `conductor-report` must gain a `tracing` dependency: its `Cargo.toml`
  `[dependencies]` is exactly `conductor-core` · `rusqlite` · `serde_json` · `thiserror` — no `tracing`.
  This is a workspace manifest edit, and the crate-per-seam rule makes dependency edges architectural, so it
  is called out rather than done silently. (`tracing` is an external crate, not a cross-seam edge — no seam
  boundary is crossed; it is already a `[workspace.dependencies]` member, so `Cargo.lock` should stay
  byte-unchanged.)
- **[inferred → OUT OF SCOPE]** `emit.batch` carries only `emission_count`; obs-plan §4 also names
  `p_id_count`. Left to the emission-dispatcher chunk — this chunk parents the shipped spans, it does not
  re-author their attribute sets.
- **[inferred → OUT OF SCOPE]** The `verify.readback.*` family is named with a dotted suffix
  (`verify.readback.call_tool`) while obs-plan §4 names the chain element `verify.readback` with an
  `mcp_method` attribute. Not reconciled: renaming shipped spans is a cost with no cause here, and a11y-plan
  §3 records the dotted family as the established convention its own recommended `focus.*` names assume.
- **[inferred → CONFIRMED]** Verification is unit/integration over the emitted JSON (assert the parent/child
  linkage and the required attributes on a real run's `agent-latest.jsonl`), reusing the shipped agent-mode
  log-schema conformance scaffold (`cli_smoke.rs::agent_mode_routes_self_obs_to_the_log_file_not_stderr`)
  rather than adding a harness.
- **[NEW, found at P3]** `row_count` — the attribute obs-plan §4 CP1 requires on `db.insert_run` — is **not**
  in `ALLOWLISTED_FIELDS` (`redact.rs:21-47`) and would be silently dropped at the processor stage. Nor are
  `phase_count` / `emission_count` / `record_count` / `mcp_tool`, carried by the three shipped spans that
  Step 1 makes visible. All five are allowlisted by this chunk.
