# Codebase Research — 2026-10-02-p-075-assert-round-against-pulse

## Scope
- **Depth:** deep (a mature codebase). The six assertions each rest on a shipped grader or a shipped read-back path.
  - **Reads:** 19.
  - **Globs/Greps:** 22.
  - **Graph queries:** 1.
  - **Pulse-side source pass at S:** one Explore agent. I spot-checked its four load-bearing claims myself
    (below).
- **Harness rules consulted:**
  - `.claude/rules/verification-harness.md`: read in full, its 32 Session Additions included. It auto-loaded on
    `crates/**/tests/**`.
  - `.claude/rules/testing.md`: read in full, 49 Session Additions.
  - `.claude/rules/host-win32.md` and `.claude/rules/security.md`: always loaded.
  - Applied below:
    - the firing-form env block;
    - `PATH` resolution of the sidecar;
    - no `boot` before a preflight-firing leg;
    - fresh dir plus quiet windows;
    - the pre-leg line count;
    - freeze per leg;
    - the census and stop form;
    - printed-verdict atoms;
    - the digest pin.
- **Platform issues consulted:** none. No runner-only bullet: Setup 5a read CI#36946764116 green 3/3 on `e1092ce`.

## Files inspected
- `conductor-0.2.0/chunks/2026-08-31-p-075-assert-round/report.md` (full) — the COMPLETION-axis precedent.
  - The prior round, at Pulse `83d4060`, shipped `probe_resolve_lifecycle`, `attribute_by_liveness`, the
    `live-pulse` feature and `tests/lifecycle_live.rs`. Its verdict was PROVEN-BY-LIVENESS (incident 6, idle
    0.0 s).
  - It also recorded the `incident_events` gap and the one-active-incident dedupe.
  - Assertion 2 therefore re-uses shipped code against a new binary. It is not new grading logic.
- `crates/conductor-run/tests/lifecycle_live.rs` (full) — the live leg shape:
  - the firing-form env block (`:10-16`);
  - storm via `canary_spec` + `emit_canary_storm` (`:123-127`);
  - polling the active set (`:88-109`);
  - a liveness keep-alive then `probe_resolve_lifecycle` (`:151-162`);
  - printing rather than asserting (`:31-32`).

  It computes no fingerprint, never calls `retrieve_telemetry_slice` or `retrieve_report`, and stamps no
  emission instant.
- `crates/conductor-run/tests/lifecycle_harvest.rs` (`:1-70`) — grades the frozen capture. It pins the live
  item key `incident_id` (`the_live_item_key_is_incident_id`) and its module doc records the `83d4060`
  coordinates.
- `crates/conductor-run/src/lifecycle.rs` (`:14-130`) — `LifecycleObservation`, `LifecycleVerdict`,
  `attribute_by_liveness` (`:72-90`) and `probe_resolve_lifecycle` (`:114-130`). The incident id rides
  `message` (`:121`).
- `crates/conductor-run/tests/delegated_timing_harvest.rs`:
  - `bounds()` (`:63-90`): the field per leaf, `value` for `metric.report.render_ms`;
  - `grade` (`:123-145`): the worst observation, with absence returned as `Err` UNGRADED;
  - `grade_in_window` (`:244-258`), `tiered_hue_samples_in_window` (`:260-275`) and
    `incident_anchor_error_ms` (`:277-288`);
  - the 2026-09-29 live pin (`:595-699`), which pins Pulse metric lines VERBATIM in test source.
  - The graders are private to this test binary. Re-use means adding the round's tests to THIS file.
- `crates/conductor-run/tests/span_landing_live.rs` (full) — the CARRY's subject:
  - `drive()` reads `runs.join("live-suite").join("span-{leg}.jsonl")` (`:38`);
  - `pulse_lines()` reads every `agent-latest.jsonl*` under the guarded data dir (`:79-114`);
  - the witness prints one integers-only line (`:198-201`).
  - Nothing checks that the two journals belong to the CURRENT Pulse launch.
- `crates/conductor-run/tests/capture_paths/mod.rs` (`:1-40`) — the `runs_dir_from` → `resolve_under` and
  `pulse_logs_dir_from` guards, which return path-free reasons.
- `crates/conductor-run/tests/live_suite.rs` (`:60-80`) — reads `runs/live-suite/{leg}.jsonl`.
- `scripts/agent-run.sh` (`:63-155`), `scripts/agent-run.ps1` (`:125-225`) — the `--live` suite:
  - `LIVE_CAPTURE_DIR="$RUNS_DIR/live-suite"`;
  - the suite clears `rm -f "$LIVE_CAPTURE_DIR"/*.jsonl` (`agent-run.sh:106`). That is a NON-recursive glob, and it
    also deletes `span-a.jsonl`/`span-b.jsonl`, the second writer's files;
  - the real-model arm clears only its three named files (`:179`);
  - the per-leg freeze is `cp "$LIVE_LOG" …/${label}.jsonl` (`:84-86`).
- `crates/conductor-run/src/canary.rs` (`:120-215`):
  - `CANARY_STORM_COUNT = 12` (`:132`);
  - `CANARY_SERVICE_NAME = "conductor-canary"`;
  - `canary_storm_seed` (`:142`) and `canary_spec(marker)` (`:157-166`);
  - `emit_canary_storm(traces, spec, base)` (`:203-215`);
  - the deterministic posture emits one storm (`:183-193`).
- `crates/conductor-emit/src/exception.rs` (`:151-175`) — `fingerprint(spec)`: blake3 over type + `\0` + the
  normalized stacktrace, the first 16 bytes rendered as 32 lowercase hex. The doc says it was transcribed at
  `efabe8e` and re-verified at `83d4060`.
- `crates/conductor-verify/src/client.rs` (`:139-190`) — `query_incident_list`,
  `retrieve_report(Option<Value>)`, `retrieve_telemetry_slice(Option<Value>)`, `resolve_incident(i64)`.
- `crates/conductor-verify/src/extract.rs` (`:80-200`):
  - the production read-back passes `{"incident_id": id}` to both report and slice (`:95`);
  - degraded is read from `report.degraded_mode` (`:103-106`);
  - `fingerprint_refs` comes from `string_array` (`:176-179`);
  - `incident_ids` accepts `id` or `incident_id` (`:160-174`);
  - `opened_at_unix_nanos` (`:181-197`).
  - All are `pub(crate)`, so a test binary re-implements the small extraction it needs (the `lifecycle_live`
    precedent, `:51-72`).
- `scenarios/{halo-hue-encoding,service-constellation-discovery,report-render-surface,findings-counter-refresh,fingerprint-storm}.toml`
  (headers) — the P-ID each one names:

  | Scenario | P-IDs | Seed | Tier | Phases |
  |---|---|---|---|---|
  | `halo-hue-encoding` | P-025 | 4317025 | `<90s` | 30 s baseline, then 150 s storm |
  | `service-constellation-discovery` | P-027 | 4317027 | `<20s` | two topology phases |
  | `report-render-surface` | P-037 | 4317037 | `<20s` | — |
  | `findings-counter-refresh` | P-045 | 4317045 | `<20s` | — |
  | `fingerprint-storm` | P-017/P-018/P-074 | — | — | — |

  NO scenario names P-075 (`grep -ln '"P-075"' scenarios/*.toml` → 0). P-075 is classified `DriveObserve` at
  `conductor-core/src/coverage.rs:540-545`.
- `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/plan.md` (`:145-260`) — the recipe for the last
  graded P-025 leg:
  - the compact widget visible;
  - a non-priming `preconditions` probe;
  - the pre-leg count;
  - one direct `conductor run halo-hue-encoding --agent-mode`;
  - freeze to `evidence/h.jsonl`;
  - a Pulse slice filtered BY TARGET into `evidence/pulse-hue-lines.jsonl`;
  - a verdict note;
  - censuses.
- `conductor-0.3.0/chunks/2026-10-01-per-run-span-identity-in-the-real-model-harness/plan.md` (`:102-379`) — the
  span pair was written by plan `[[gate]]` entries (`cp logs/agent-latest.jsonl runs/live-suite/span-a.jsonl`). It
  is not a harness verb.
- `contracts/pulse-p025-measurement-contract.md` (index) — every coordinate is pinned at `226554a`
  (`pinned_at`, `:14`), and the clauses say they expire when HEAD moves.
- `.gitignore:17` — `/runs/` is ignored as a whole, so any `runs/<subdir>/` is ignored with no new entry.

## Pulse at S (03ec944) — read by an Explore agent, four claims re-verified by hand
Paths are relative to the Pulse repo. The agent's report is condensed below; lines marked **[re-verified]** I read
myself at S.
- **Q1 `fingerprint_refs`:**
  - The source is `incident.evidence_refs.fingerprint_hashes` (`crates/mcp-server/src/tools.rs:439`)
    **[re-verified]**.
  - Its only writer is incident CREATION. `grounded_fingerprint_hashes` (`pulse-app/src/inference_runtime.rs:657-666`)
    **[re-verified]** produces an order-preserving union: the model's refs, then the triggering cue's full-hex
    fingerprint.
  - Under deterministic L4 the refs are the three `det-*` constants (`deterministic_inference.rs:68-72`), so the
    expected shape is `[det-span-…, det-template-0007, det-fingerprint-…, <cue fp>]`.
  - Width: 32 lowercase hex (`triage/src/pattern/storm.rs:323-326`, `contract.rs:165-171`).
  - Derivation: `compute_exception_fingerprint` (`crates/buffer/src/fingerprint.rs:79-96`) **[re-verified]**,
    equal to Conductor's `fingerprint()`. `git log 83d4060..03ec944 -- crates/buffer/src/fingerprint.rs` → 0 commits.
  - **A dedupe re-emission does NOT update `fingerprint_hashes`** (`inference_runtime.rs:830-858`). A storm that
    dedupes into an already-open incident therefore never reaches `fingerprint_refs`.
- **Q2 `degraded_mode`:**
  - `degraded_mode = parsed_l4.is_none()` over `resolution_summary_text` (`tools.rs:377-381`) **[re-verified]**.
    Arity is one `incident_id`.
  - At S, `resolution_summary_text = scrubbed_l4_json(parsed)` (`inference_runtime.rs:906`). Since `9d14166` it
    masks per LEAF (`:675-688`), so the canned L4 output stays parseable, which gives **`degraded_mode: false` under
    deterministic L4**.
  - Before `9d14166`, a whole-string scrub could collapse it, which gave degraded `true`. This is why every
    2026-08-21 render sample reads `degraded_mode: true`.
- **Q3 lifecycle:**
  - Applied response: `{"resolved":true,"incident_id":<id>}` (`tools.rs:477`).
  - Items key: `incident_id`. `query_incident_list` takes no arguments and returns `status != 'resolved'`
    (corpus `contract.rs:692-698`).
  - Auto-resolve: 120 s idle, a 30 s resolver tick that skips its first tick.
  - Dedupe tuple: `(kind, scope, scope_id)`, with the fingerprint excluded.
  - Unchanged since `83d4060`.
  - **New hazard named by source:** the app's in-memory registry learns of an MCP resolve only in the 60 s
    persist-cycle reconcile (`triage/src/incident/persistence.rs:250-283`). An app-side re-emission persist landing
    first can re-write the row `active`.
  - The immediate post-resolve read (what `probe_resolve_lifecycle` does) is unaffected. A LATER re-read could see
    the incident back.
- **Q4 the four leaves:**

  | Leaf | Field | Emit site | Allowlist |
  |---|---|---|---|
  | hue | `duration_ms` | `ui-bridge/src/telemetry.rs:310-324` | `observability.rs:992-995` |
  | discovery | `duration_ms` | `:326-341` | `:996-1002` |
  | counter | `duration_ms` | `:343-356` | `:1003-1006` |
  | report render | `value` | `pulse-app/src/incidents_router.rs:563-569` | `:2430-2435` |

  - **hue:** fires only on a WITNESSED tier change of a dot, mounted in the compact widget, which is shown at boot
    (`main.rs:1134`).
  - **discovery:** fires per batch of newly visible dots. It measures backend first-sighting → first render (since
    `fb93fca`/`87fe658`).
  - **counter:** fires on each `list_active` poll (1000 ms) in the compact widget, Findings and Report webviews.
  - **render:** fires ONLY from the in-app `get_report` resolver, never from MCP `retrieve_report`. The Report
    window is declared `visible: false` at boot (`tauri.conf.json:49-61`), and `Report.tsx:24-26` passes
    `useReport(isOpen ? id : null)`. So **no render sample is produced unless Pulse's Report window is OPEN**.
    **[corrected at P5: `isOpen` is `effectiveId !== null` with
    `effectiveId = clickedId ?? findings.rows[0]?.id` (`pulse-app/ui/src/widget/ReportWindow.tsx:23,32` at S), so the
    hidden Report webview fires on each newly formed incident with no click. A Findings-row click
    (`FindingsWindow.tsx:189-191,222`) pins the selection until the Report is closed (`:26`).]**
  - Whether a hidden WebView2 runs the effect is "not measurable from source".
- **Q5 P-025 coordinates, `226554a` → S:**
  - The formula `duration_ms = max(0, paint − tier_effective_at/1e6)`, the one-sample-per-witnessed-change rule
    and the 60 000 clamp all hold.
  - Moved:
    - the hue emit, `telemetry.rs:278-282` → **`:316-320`**;
    - the allowlist, `observability.rs:989-992` → **`:992-995`**;
    - `ServiceListItem`, `lifecycle/registry.rs:54-69` → **`:56-71`** (field at `:70`).
  - Unchanged: `frame-metrics.ts`, `constellation-types.ts`, `tier_effective.rs`, `ConstellationCanvas.tsx`,
    `xtask/src/hue_shift.rs:41`.
- **Q6 `app.exit`:**
  - New at S (`03ec944`; `observability.rs:2771-2807`).
  - On Windows only the `event_loop` class writes, through tray Quit. `taskkill /F` / `Stop-Process -Force` writes
    none, as the round-request says.
- **Q7 bootstrap override:**
  - `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is still present (`triage/src/cue/thresholds.rs:12,17`). The
    default is 3600, accepted values are 0 < v < 86400, and the WARN target is
    `triage.baseline.bootstrap_window.override`.
  - It is inert for this round. Every assertion rides storm-path incidents or UI leaves, never a baseline cue (no
    absence-based arm).

## Graph impact
- **`probe_resolve_lifecycle` / `attribute_by_liveness` / `emit_canary_storm` / `canary_spec`** — 33 rows (trace
  `tree-query-2026-10-02-p-075-assert-round-against-pulse.json`).
  - The production callers are `canary.rs:188-189,205` (deterministic/real-model canary). The others are `lib.rs`
    re-exports (`:28-29,36-37`) and test binaries (`canary_wire.rs`, `canary_obs_witness.rs`,
    `lifecycle_harvest.rs`, `lifecycle_live.rs`, `composition_root.rs:21`).
  - `lifecycle_live.rs`'s `probe_resolve_lifecycle(…)` call at `:155` is absent from the rows. It is an async fn,
    the index-gap class the cookbook names. Grep confirms the call site.
  - **This chunk calls these symbols and changes none of their signatures**, so there are no callers to thread and
    no cross-crate blast.

## Patterns detected
- **The live leg prints and the harvest grades** (`lifecycle_live.rs:31-32`; `delegated_timing_harvest.rs:595-699`).
  The feature-gated leg is the producer, and a default-suite harvest asserts over committed evidence.
- **Absence is UNGRADED, never met** (`delegated_timing_harvest.rs:123-145`, `:451-470`). This matches the
  round-request posture exactly.
- **The windowed P-025 grade with incident anchoring** (`grade_in_window` + `incident_anchor_error_ms` ≤ 1 000 ms,
  `:244-288`). The window comes from the frozen self-obs `timeline.execute` `new` + phase-1 `gap_ms`, through
  `scenario.run` close.
- **A filtered-by-target Pulse slice as committed evidence** (2026-09-29 `evidence/pulse-hue-lines.jsonl`). These
  are closed-field lines: no corpus text, fingerprint or host path.
- **Digest-pinned evidence graded from file** (`real_model_harvest.rs:5-6`, the 2026-09-30 regime): sha256 over LF
  content, a tamper arm, and no capture text in source.
- **Raw-wire dump before trusting an extraction** (`lifecycle_live.rs:56-58`; testing.md 2026-09-01).

## Conventions to follow
- **Test-binary readers of `CONDUCTOR_RUNS_DIR` / `ANDROMEDA_PULSE_DATA_DIR`** go through `capture_paths` and fail
  with path-free reasons (`capture_paths/mod.rs:6-9`).
- **The `live-pulse` gated target owes its own clippy line**:
  `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`.
- **Capture output from a `-q … -- --nocapture` test is buffered and written ONCE**, with a leading newline
  (testing.md 2026-09-23; `span_landing_live.rs:199-201`).
- **Read the live item key `incident_id`, accepting `id` too** (`lifecycle_live.rs:65-67`; `extract.rs:160-174`).
- **Name a scenario by its stem.** Never pass a shared P-ID to `run`/`SCENARIO=`. P-025/027/037/045 are each named
  by more than one scenario (the grep above lists `halo-breathing-encoding` and `constellation-severity-live-wiring`
  among the P-ID-bearing files).
- **The firing env block, pasted whole:**
  - `PATH` with Pulse's `target/release` first, in POSIX form (`/d/dev/projects/andromeda-pulse/target/release`);
  - `ANDROMEDA_PULSE_DATA_DIR` set to the LIVE app's dir;
  - `ANDROMEDA_PULSE_MCP_ENABLED=true`;
  - `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`;
  - checked with `which andromeda-pulse-mcp` before the leg (verification-harness.md 2026-08-20, 2026-08-22 (c)).
- **No `boot` before a preflight-firing leg**, and a 150 s quiet window between any two canaries or storms on one
  launch (verification-harness.md 2026-08-19 (a), 2026-09-04 extension).
- **Fresh data dir under `%TEMP%/pulse-legs/<letters-only leaf>`**, with `pulse-app` launched BY PATH from a cwd
  outside this repo (verification-harness.md 2026-08-18 [refined 2026-09-30]; 2026-08-10 (2)).
- **The census is taken before and after**, stopping only what the agent started, by PID + parent + CreationDate
  (host-win32.md 2026-09-17; memory "Stop everything you start").
- **The Pulse pre-leg line count is taken after readiness and before the first dispatch** (~2.2 k boot lines;
  verification-harness.md 2026-08-21 (a)).

## New files to create
- `crates/conductor-run/tests/p075_round_live.rs` — the round's P-075 live leg (`#![cfg(feature = "live-pulse")]`).
  - It stamps the `std::time` emission instant, then emits a `canary_spec(marker)` storm through `emit_canary_storm`
    and computes `fingerprint(&spec)`.
  - It polls the active set for an incident opened after that stamp, then calls `retrieve_telemetry_slice` and
    `retrieve_report` for that incident.
  - It does a liveness keep-alive, then runs `probe_resolve_lifecycle`.
  - It prints ONE buffered capture of integers, booleans and ids only (fingerprint membership as a boolean, never
    the hex).
- `conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/` — the round's committed evidence,
  each file digest-pinned:
  - the leg capture;
  - the frozen self-obs of each scenario leg;
  - the target-filtered Pulse slice;
  - the verdict note with the binary sha256 values and the censuses.

## Files to modify
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — the round's P-025 (windowed) and P-027/P-037/P-045
  grades over the committed slice, using the existing `bounds`/`grade`/`grade_in_window`.
- `crates/conductor-run/tests/lifecycle_harvest.rs` — the round's assertion-1/2 grades over the committed leg
  capture.
- `crates/conductor-run/tests/span_landing_live.rs` — the CARRY: the input subdir moves off `runs/live-suite/`, a
  firing-form clear runs before drive A, and a stale pair is refused.
- `contracts/pulse-p025-measurement-contract.md` — an add-only re-pin of the three moved coordinates at S.

## Open questions
- **[resolved at P5: nobody. See the Q4 correction. The overseer ratified hands-off.]** **Who opens Pulse's Report window?** No `metric.report.render_ms` sample exists unless it is OPEN
  (`Report.tsx:24-26`, `tauri.conf.json:49-61`). The scope law bars Conductor from automating Pulse's UI. An unopened
  window makes P-037 UNGRADED by construction. → blocks: plan-decision.
- **The round's leg set:**
  - Option 1: the P-075 leg + all four delegated scenarios (each P-ID backed by a drive of its own scenario).
  - Option 2: the P-075 leg + `halo-hue-encoding` only, grading P-027/037/045 as the worst over the whole round
    capture.
  - Option 1 is ~25 min of slot, Option 2 ~10. → blocks: plan-decision.
- **How the CARRY is proven:**
  - Hermetically: the gated target's pure arms plus clippy.
  - Or also by a live re-witness of span-landing inside the round: two same-seed drives of a short scenario 180 s
    apart, ~4 min more. → blocks: plan-decision.

## Scope premise closure
Each `[inferred]` bullet of `scope.md` is closed against the findings above. scope.md is amended in place.
