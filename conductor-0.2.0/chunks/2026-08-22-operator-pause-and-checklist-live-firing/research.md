# Codebase Research — 2026-08-22-operator-pause-and-checklist-live-firing

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 11 · **Code-graph queries:** 2 (plane `rust`, `db_state: fresh`)

The extracts raised four research questions and the directive left three open. All seven are answered below;
none remain open at plan time.

## Files inspected

- `crates/conductor-core/src/pause.rs` (symbols + `resolve_hold`) — `Decision` (:29) · `HoldPoint` (:55) ·
  `HoldResolution` (:78) · `PauseResolver` (:95) · `HeadlessResolver` (:105) · `resolve_hold` (:137). The
  whole hold model; nothing here needs to change for parts (1)+(2).
- `crates/conductor-core/tests/operator_pause.rs` (full) — 6 tests, all `HeadlessResolver`-driven. Confirms
  the never-fired-attended gap: no test exercises `PromptResolver`.
- `crates/conductor-run/src/lib.rs` (:355-412, :518-539) — the ONE live firing site and `manual_record`.
  **The two decisive findings live here** (below).
- `crates/conductor-cli/src/pause.rs` (:23-66, :67-96, :109-155) — `PromptResolver { spinner }`,
  `bar.suspend(render_and_prompt)`, `confirm()`'s decision mapping, `CliResolver::select`, `resolve_kind`.
- `crates/conductor-cli/src/commands/suite.rs` (full) — the part-(2) vehicle.
- `crates/conductor-cli/src/commands/run.rs` (:15-35) — `CliResolver::select(None, agent_mode)`.
- `crates/conductor-cli/src/render.rs` (:79-84, :110-132, :155-168) — `hold_line`, `spinner`, `hold_line_styled`.
- `crates/conductor-cli/src/paths.rs` (:62-79) — `load_all_scenarios`, the `--filter` semantics.
- `crates/conductor-core/src/obs.rs` (:63-135, :79-80, :148-159) — `ObsSink`/`ObsWriter`, the default
  `EnvFilter`, `build_subscriber`.
- `crates/conductor-core/src/lamp.rs` (:40-48, :98-106) — `(ManualCheck, None) => Manual`; verdict-first.
- `crates/conductor-core/src/scenario_catalog.rs` (:1-80) — `scenario_files`, `list_scenarios`, `SUITE_SELECTION`.
- `crates/conductor-tauri/src/pause.rs` (symbol scan) — `HoldPrompt` (:23) · `HoldGate` (:47) ·
  `TauriResolver` (:68) · `resolve_operator_hold` (:92).
- `crates/conductor-tauri/ui/src/components/OperatorChecklist.tsx` (full) + `OperatorChecklistView.tsx` (full)
  — the `induced`/`observation` shape and its only render site.
- `scripts/agent-run.sh` (:93-100) — the harness's only run form.

## Graph impact (code-graph, plane `rust`, `db_state: fresh`)

Query 1 (existence probe, `rows: 22`) confirms every pause symbol IS indexed — so query 2's result is a real
caller set, not an index gap (the two-precondition rule, cookbook §Reading the result).

Query 2 (`callee LIKE '%/resolve_hold().%' OR '%[CliResolver]select().%' OR '%/spinner().%'`, `rows: 20`):

- **`resolve_hold`** — 1 production caller: `execute_scenario()` @ `conductor-run/src/lib.rs:401` (graph
  0-indexed = source :402). All other callers are the 6 tests in `conductor-core/tests/operator_pause.rs`.
  **A single production firing site** — the change surface for parts (1)+(3) is exactly one function.
- **`CliResolver::select`** — 2 production callers: `commands/run/run()` @ `run.rs:22` (source :23, passes
  `None`) and `commands/suite/suite()` @ `suite.rs:27` (source :28, passes `Some(progress)`), plus 5 tests
  in `conductor-cli/src/pause.rs`.
- **`render::spinner`** — 1 caller: `suite()` @ `suite.rs:26` (source :27).
- Use-statement edges at `conductor-core/src/lib.rs:47`, `conductor-run/src/lib.rs:26`,
  `operator_pause.rs:2` — the re-export surface, unchanged by an additive design.

**Caller threading:** any signature change to `resolve_hold` or to `execute_scenario`'s resolver parameter
threads through `run.rs:23` **and** `suite.rs:28` (both bins' call sites) plus `conductor-tauri`'s
`TauriResolver` impl, and the 11 test call sites above pin the current shapes.

## The decisive findings

### F1 — the operator's Decision is discarded (parts (1) + (3))
`execute_scenario` binds `let resolution = resolve_hold(resolver, &hold).await;` (`lib.rs:402`) and then uses
`resolution` **only** in the `tracing::debug!` on the next line. `manual_record` (`lib.rs:518-539`) is called
without it and builds `verdict: None` / `state: state_for(observation, ReportState::ManualCheck)`. So today
**a NoGo/Abort produces a byte-identical `RunRecord` to a Go** — the activation reaches no artifact.

### F2 — that one witness is invisible by default, and its wording is false
The line is `tracing::debug!`, and the default filter is `LevelFilter::INFO`
(`obs.rs:79-80`, `EnvFilter::builder().with_default_directive(LevelFilter::INFO.into())`). Under a normal
attended invocation it does not print at all. When it does print (`RUST_LOG=…=debug`) it reads
`"operator-checklist hold resolved headless: {}"` — unconditional, so it is *false* the moment an attended
resolver answers. Acceptance part (1) therefore has **no correct, default-visible witness by either route**.

### F3 — the freeze mechanism is real and already correct
`PromptResolver::resolve` (`cli/pause.rs:30-40`) does `bar.suspend(render_and_prompt)` when a spinner is
present. `suite.rs` builds the bar (`render::spinner`, template `"{spinner} {pos}/{len} scenarios"`,
`enable_steady_tick(120ms)`, stderr-TTY-gated) and calls `progress.inc(1)` **after** `execute_scenario`
returns — so at the hold the count reads its exact pre-increment value, suspends in place, and resumes.
Nothing needs building for part (2); it needs **exercising and observing**.

### F4 — no Rust-side checklist item exists; the webview one is gallery-only
`induced`/`observation` appear exactly once in the tree: `ChecklistItem` in
`ui/src/components/OperatorChecklist.tsx`. `OperatorChecklistView` is rendered from **`Gallery.tsx:199`
alone**, fed `useState<ChecklistItem[]>([...])` hardcoded placeholders. No Rust type carries the pair; the
only Rust checklist surface is the single generic sentence at `lib.rs:399`. The per-scenario wording exists
as TOML **comment prose** ("hue shifted toward burgundy under error pressure?"), labelled *Epoch-8/10
calibration points*.

### F5 — the self-obs sink emits JSON on every path (answers obs's research question)
`build_subscriber` (`obs.rs:148-159`) installs exactly ONE layer, `JsonObsLayer`, over the writer;
`ObsSink::Stderr → ObsWriter::Stderr`. There is **no pretty-print branch anywhere**. The attended
(non-agent) leg therefore emits parseable JSON lines to stderr, so it *can* produce gradeable self-obs
evidence. obs-plan §3/§6's "stderr pretty-print in dev" wording is confirmed stale — an Expected amendment.

### F6 — an attended hold does NOT inflate `latency_ms` (answers obs's second research question)
`observed_ms = now_ms()` and `read_back_observed_at = now_rfc3339()` are stamped at `lib.rs:391-392`,
**before** the `if scenario.expected.is_empty()` block at :394. `manual_record` receives the pre-hold
`observed_ms - emitted_ms`. Unbounded human wait sits outside the measured window; the tier assertion is safe.

### F7 — the attended vehicle is `conductor suite --filter halo`
`load_all_scenarios` filters by **substring on the file stem** (`paths.rs:73`
`filter.is_some_and(|f| !stem.contains(f))`). `--filter halo` selects exactly `halo-breathing-encoding` +
`halo-hue-encoding` — both declare-only, so both hit the checklist hold, over a real `{pos}/{len}` count of 2.
`agent-run.sh` has **no `suite` verb** (only `conductor run <scenario> --agent-mode`, :98/:100), confirming
the attended leg is necessarily a direct binary invocation outside the harness.

## Patterns detected
- **Enum-dispatched resolver** (`cli/pause.rs:63-66`): `PauseResolver::resolve` returns `impl Future` and is
  not object-safe, so `CliResolver` is an enum matched by hand rather than a `dyn` trait object. Any new
  resolver variant follows this shape.
- **Spinner-suspend as the freeze** (`cli/pause.rs:35-38`): `match &self.spinner { Some(bar) => bar.suspend(f), None => f() }` — the `None` arm is why `conductor run` shows no freeze.
- **`state_for(observation, default)`** (`lib.rs:534`): the degraded read-back is what turns a `ManualCheck`
  default into `KnownResidual`; under deterministic L4 every read-back is degraded, so these legs land
  `KnownResidual` by construction.
- **Declare-only routing keys on `scenario.expected.is_empty()`** (`lib.rs:394`) — the same predicate feeds
  `route_read_back(…, scenario.expected.is_empty())` at :373. 24 catalog scenarios satisfy it.

## Conventions to follow
- **Witness on the allowlisted `message` field** rather than minting a span attribute — obs-plan §6's
  established route (`verify.readback` / `emit.batch` precedents); a non-allowlisted attribute emits nothing.
- **`info` for state transitions, `debug` for internal control flow** (obs-plan §6) — the hold's resolution is
  a state transition, which is why `debug` is the wrong level for it (`lib.rs:403`).
- **Indented detail line, never a new column/label/token** (`layout-templates` §cli Primary content block 2;
  the `2026-08-21-per-check-latency-measurement` precedent) — the lamp set stays closed at six.
- **Live claims graded at the harvest tier** — a `conductor-run` integration test pinning this-run capture
  artifacts verbatim, the six-precedent shape (`*_harvest.rs`, most recently
  `delegated_timing_harvest.rs`).

## New files to create
- `crates/conductor-run/tests/operator_pause_harvest.rs` — the harvest-tier test pinning the attended legs'
  captured artifacts (record shape, resolution witness, frozen-count capture).

## Files to modify
- `crates/conductor-run/src/lib.rs` — the firing site (:394-411): carry the operator's `Decision` into the
  record path and replace the false/invisible `debug` witness with a correct one at the right level; source
  the hold's `prompt` from per-scenario checklist data instead of the generic sentence.
- `crates/conductor-core/src/pause.rs` — if the checklist item becomes a core type, it lands here or beside
  the scenario model (arch: engine data is core-shaped).
- `crates/conductor-core/src/scenario.rs` — the `[[checklist]]` / induced+observation declaration if part (3)
  lands as scenario config (serde + garde `dive`, closed shape).
- `scenarios/halo-hue-encoding.toml` · `scenarios/halo-breathing-encoding.toml` — the two attended-leg
  vehicles gain their declared checklist item(s) (wording already present as comment prose).
- `crates/conductor-cli/src/render.rs` — the `ManualCheck` detail line carrying induced/observation, if the
  cli is the chosen render surface.
- **Caller threading (from the graph):** `crates/conductor-cli/src/commands/run.rs:23` and
  `crates/conductor-cli/src/commands/suite.rs:28` if the resolver/`execute_scenario` signature moves;
  `crates/conductor-tauri/src/pause.rs` (the third `PauseResolver` impl) likewise.
- **Test companions that pin changed data/shape:** `crates/conductor-core/tests/operator_pause.rs` (6 tests),
  `crates/conductor-cli/src/pause.rs` tests (5), and `crates/conductor-core/src/scenario.rs`'s catalog guards
  (`constellation_and_p032_are_operator_checklist_declare_only` and siblings) — a catalog data change trips
  these even though no signature moved.
- `crates/conductor-core/src/lib.rs:47` — re-export line, if a new pub type must be visible outside the core.

## Open questions
- **Which render surface carries the induced/observation pair — cli detail line, or the webview checklist
  card?** → blocks: **plan-decision**. Both are spec-sanctioned (design §cli Component Patterns 4 has a
  `ManualCheck` line shape that already carries an observation question; §desktop-webview Component
  Patterns 7 has the card). The cli is the release-gate surface and the only one the attended leg touches;
  the webview primitive already exists but is gallery-only and its a11y verification is display-gated to
  Linux+xvfb (the standing Epoch-5 CARRY). **P4 resolves this before synthesis.**
- **Does the JSON self-obs stream on stderr visibly corrupt the 120ms-steady-tick spinner during an attended
  suite leg?** → blocks: **implementation-scope**. Both occupy stderr and `route_read_back` emits `info`
  lines mid-run (`lib.rs:373`, :379); `indicatif` only suspends around its own `suspend` calls, not around
  arbitrary `tracing` writes. Mitigation is a leg-time `RUST_LOG` choice, not a code change — but it decides
  whether the frozen-count observation is legible enough to grade.
