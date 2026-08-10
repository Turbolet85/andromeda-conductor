# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/andromeda-setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-08-09 — A baked count inside an illustrative sample is the same stale-derived-fact class as one in prose

When a chunk invalidates a derived fact (here: the coverage classification widening from 60 to 82 capabilities), the de-hardcoding sweep must reach **sample output and wireframe captions**, not just prose. Illustrative status is not an exemption — a reader takes a number from a sample exactly as readily as from a sentence, and leaving one behind recreates the very self-contradiction the sweep exists to remove. `layout-templates.md` had already been half-swept on 2026-08-08: its coverage header strip (`:37`) read "82 loaded (manifest set)" while the component prose (`:121`) still said "the full 60-row wall" — the document contradicted itself for a full version cycle because the earlier pass treated the two as different classes.

The sharper trap is **coupled facts inside one sample**. The suite-run caption baked two numbers that had to agree: a per-state tally (`55 Pass · 1 Calib · 1 Fail · 1 Manual · 1 Residual · 1 Blocked`) and a step denominator (`step 60/60`), the first summing to the second. Fixing only the denominator would have left the sample internally inconsistent — strictly worse than the stale total it replaced, because an inconsistent example teaches nothing and reads as a typo. Fix such a caption as a unit and verify the arithmetic afterward (`77+1+1+1+1+1 = 82 = denominator`), and prefer the form a sibling passage already established (`(manifest set) 82 loaded`) so one convention governs the file.

Practical sweep note: grep the *class*, not one phrasing — `60-P-ID`, `all 60`, `60-row`, `P-001\.\.P-060`, `60/60` each found different sites, and the eventual inventory (11 code sites + 6 spec sites) was roughly triple what the initial reading suggested. Guard the false positives explicitly: `Lamp::Blocked => 60` is an ANSI color code and `--fail-under-lines 60` is a coverage percentage, neither a capability count.

**Extended 2026-08-09 (in-lane-sut-scenarios) — the class reached recurrence #3 and is now machine-detected.** Three consecutive chunks staled a documented derived value with no drift-base invariant covering it (out-of-scope treatment ×2, then `UNBACKED_AUTO` 11 → 10 staling `layout-templates:178`'s caption *and* `test-plan:306`'s `P-001..P-060` selector range). Three things generalize from closing it:

**A detector needs a fact to bind to.** The class was invisible because the report had no bullet for it — the structural families (Files / Symbols / Crates / Dependencies / Schema) describe *what was added*, never *which documented derived value moved*. Adding a `Counts / qualifiers this chunk moved` bullet to the report is what let `D-layout-surface` fire; its proposal rationale cites that exact bullet. The report↔detector contract is load-bearing in both directions: extend the report first, then the detector has something to read. Two detectors now cover it (`D-layout-derived-count`, `D-tests-derived-count`) — drift-base scopes one detector to one doc, so a cross-doc invariant costs one entry per doc.

**The fix is to name the SET, never substitute a fresh literal.** `P-001..P-060` → "the manifest's accepted set", not → `P-001..P-082`. A new literal is the same bug with a later expiry date, and it re-stales on the next SUT release. This is why the two leaf distillations needed *no* edit this time: `design-summary.md` and `tests-summary.md` had already been written to name the mechanism (`(N unbacked)`, "the `UNBACKED_AUTO` pin") rather than the value, so re-derivation produced identical text. Prose naming the set is correct and must not be "fixed".

**A doc-agent can see a finding and decline to raise it.** The test-plan detector *found* the stale `P-001..P-060` range and wrote, in prose after its `proposals: []`, that it was "not what any of my three invariants guard". It was right — and the finding survived only because that trailing prose was read before the strip step discarded it. When a detector returns clean but explains itself, read the explanation: a scoped agent correctly refusing to exceed its remit is reporting a gap in the *detector set*, not an absence of drift.

---

## 2026-08-09 — Trace fidelity is not just the count: honoring `rows` while reconstructing composition from a truncated view

The Tier-1 code-graph rule already says a `head`-ed or `LIMIT`-ed view is never the result — the run-dir trace's `rows` field is. There is a subtler way to break it that satisfies the letter of that rule: take the **count** from the trace, then reconstruct the **composition** from the truncated console output you happened to see. The count is right, the claim built on it is wrong, and nothing about the output looks suspicious.

Concretely: a `crate_edges` query with the canonical bidirectional predicate (`WHERE to_crate = 'X' OR from_crate = 'X'`) returned 6 rows. The visible tail showed five inbound edges, so the sixth was inferred to be outbound — plausible, since the query asks for both directions. In fact all six were inbound and the sixth was simply above the window; the crate imports no workspace crate at all. **A bidirectional `OR` predicate does not imply both directions are populated.**

Read the trace's `result` array for composition, not only its `rows` scalar for arity — and when a claim rests on a direction, a subset, or a grouping, derive it from the result set, never from what scrolled past. The stakes are not cosmetic: a plan is `/andromeda-implement`'s input and the wrap report inherits whatever number stands in it, so a wrong split propagates into the permanent record. Corollary for writing it down: state the split explicitly ("6 rows, all inbound, zero outbound") rather than a bare count with an illustrative list, which invites the next reader to re-derive the same wrong inference.

---

## 2026-06-27 — Live-Pulse E2E reference: the Pulse MCP read-back surface + the run recipe

Verified live against the running Pulse this session (operator findings + direct probes):

- **Incident creation is LLM-in-the-loop + non-deterministic:** OTLP → L1 → L2 RetryStorm cue (deterministic, ≥5 same fingerprint/30s) → L3 digest (cadence 20-60s) → L4 llama.cpp Llama-3.2-3B inference (Dismiss/Severity) → incident. The verification-posture consequence (live incident-readback can't be deterministic; open decision) is in `verification-harness.md` Session Additions.
- **The MCP read-back surface is 8 tools, not 4:** 4 live-buffer (`query_traces`/`query_metrics`/`query_logs`/`generate_snapshot`) + the 4 persistent-corpus tools Conductor consumes (`query_incident_list`/`retrieve_report`/`retrieve_telemetry_slice`/`mark_incident_resolved`). **Only the persistent-corpus tools work cross-process from a Conductor-spawned sidecar** — the in-memory-buffer tools return EMPTY (the spawned sidecar shares no memory with pulse-app, which owns the live OTLP buffer). Shapes: `query_incident_list` → `{items:[{id,status,severity,title,opened_at}],total,next_cursor}` (titles are SCRUBBED/generated, NOT a marker echo — so fidelity rides the fingerprint); `retrieve_telemetry_slice(incident_id)` → `{incident_id,span_refs,fingerprint_refs,timestamps_unix_nano}`. Incidents are filtered by the `workspace` column = the sidecar's data-dir.
- **`corpus.db` is plaintext SQLite** (header "SQLite format 3") at `{data_dir}/corpus/corpus.db` — NOT encrypted-at-rest (the arch P-049 "encrypted / OS keychain" assumption was wrong). Tables: `incidents` (`id,workspace,status,created/updated/resolved/read_unix_nano,payload`), `service_registry`, `baseline_state`, `pipeline_metrics`, `digest_archive`. Conductor's production code reads the corpus via MCP read-back ONLY (the direct `sqlite3` reads this session were one-off operator-sanctioned diagnosis).
- **Pulse run recipe (future live pass):** launch pulse-app with `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`; the build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (a triage/build.rs bug truncates the 9 MB tokenizer → L4 breaks). Sidecar double-gate = `ANDROMEDA_PULSE_MCP_ENABLED=true` + the compiled feature flag. Conductor resolves the sidecar from PATH (`andromeda-pulse-mcp`); data-dir defaults to `%APPDATA%\andromeda-pulse`. RetryStorm thresholds ≥5/≥10 same fingerprint/30s; L4 ≈4 s/inference.

---

## 2026-06-27 — Obs-artifact CI conformance gate: the no-Pulse producer + grep stderr for panics

To assert Conductor's self-observation artifact in CI without a live Pulse (the obs-plan §9 obs CI gate), PRODUCE `logs/agent-latest.jsonl` with a hermetic Blocked agent-mode run: `cargo run -p conductor-cli --bin conductor -- run error-baseline-spike --seed 424242 --agent-mode` under `ANDROMEDA_PULSE_DATA_DIR=pulse;injection`. The injection metacharacter is rejected by `conductor-verify` BEFORE any sidecar spawn → a Blocked envelope (exit 0, not a hard Fail) — but `init_observability` still opens the file sink and writes every self-obs line (the `cli_smoke.rs` `agent_mode_routes_self_obs_to_the_log_file_not_stderr` precedent; reuses the 2026-06-23 verification-harness Blocked lever, now applied as a CI PRODUCER rather than a test). `cargo run -p conductor-cli` does NOT compile `conductor-tauri`, so no frontend bundle is needed. A `shell: bash` gate then asserts the §3 self-obs BASE schema per line (`jq`: `timestamp_ms`/`level`/`target`/`service.name`/`service.version`/`deployment.environment`/`run_id` — NOT the §6 run-report envelope, which lives in `runs/<run_id>.jsonl`), scans for leaked absolute host paths (anchors MIRRORING `conductor-core::redact::is_host_path_token` — drive-letter `[A-Za-z]:[\\/]`, `/home/`, `/Users/`, `%APPDATA%`, `.cargo/`, `.rustup/`… — so `::` module paths and repo-relative `crates/…` source paths are NOT flagged), and scans `^thread.*panicked`.

GOTCHA (caught by the real-smoke dogfood, not the typed tests): the zero-unlogged-panics scan MUST grep BOTH `agent-latest.jsonl` AND the captured producer stderr. A CAPTURED panic (`std::panic::set_hook`) is a structured `tracing::error!(panic=…)` JSON line in the FILE; but an UNSTRUCTURED panic (a bypassed hook — the exact regression the gate guards) is written by Rust's default handler to STDERR, never the agent-mode file sink — so a file-only grep would silently miss it. Capture it: make the producer step `shell: bash`, `mkdir -p logs` (the `2>` redirect target dir must exist first), `… --agent-mode 2> logs/producer-stderr.log`, then `grep -nE '^thread.*panicked' "$log" logs/producer-stderr.log`. obs-plan §9 already specified "greps agent-latest.jsonl + stderr" — the spec was right; the plan's softer "if captured" wording got concretized here. This was an in-scope `ci.yml` strengthening, not a spec change.

Verify locally before trusting CI: produce the artifact under a relative scratch `CONDUCTOR_RUNS_DIR` (e.g. `.obs-smoke/runs`; `resolve_under` requires a path under cwd, so an absolute temp dir is rejected — relative-under-repo, then move it OUT of the tree rather than `rm -rf`, which the sandbox blocks), run the gate logic against it (expect PASS), and against crafted negative fixtures with VALID JSON (a `/home/…` path for the leak check; a panic line on a stderr file for the panic check — a non-JSON line fails the per-line `jq` field check FIRST, so use valid JSON to exercise the leak/panic checks for the right reason). The Epoch-10 A11y CI gate + Obs gates reuse this producer + `shell: bash` gate-step + `if: always()` upload scaffold.

---

## 2026-06-27 — CI coverage-gate mechanics (cargo-llvm-cov + nextest in ci.yml): promote the pre-staged step, and the gotchas

The CI quality-gate chunk promoted `ci.yml`'s pre-staged `Coverage (measure only)` step (`--summary-only`, whose own comment flagged it Epoch-10) into a real gate: collect once with `cargo llvm-cov nextest --workspace --profile ci --no-report`, then `cargo llvm-cov report --lcov --output-path lcov.info` + `--cobertura coverage.xml`, then a final `cargo llvm-cov report --fail-under-lines 60` (the binding line floor, test-plan §10), then `actions/upload-artifact@v4` for the coverage reports + `target/nextest/ci/junit.xml` (both `if: always()` so a failing gate still publishes). `.config/nextest.toml` needed NO edit — `[profile.ci.junit]` + `retries = 0` (both profiles) were already there from the test-framework chunk; a separate `shell: bash` grep step asserts `retries = 0` at the CI level (the flakiness budget), catching `retries = N` and the `retries = {…}` backoff-map form.

cargo-llvm-cov behaviors worth knowing for the downstream Obs/A11y CI-gate chunks: (1) it reports `src/` only — so the test-plan §10 "exclude rstest/insta fixtures" `--ignore-filename-regex '[\\/]tests[\\/]'` is currently a NO-OP (integration `tests/` files aren't in the denominator; the measured 89.89% already reflects product code) — kept for intent + future-proofing, not because it changes the number. (2) LCOV (`lcov.info`) carries ABSOLUTE source paths by default (`SF:D:\dev\…`), while Cobertura keeps only the `<source>`-root absolute (per-file paths relative) and nextest JUnit is path-clean — so the only absolute-path surface is LCOV, and in CI those are ephemeral runner paths (see the security.md redaction-boundary rule — outside Conductor's redaction scope). (3) Emit multiple formats from ONE instrumented run via `--no-report` collect + per-format `cargo llvm-cov report` calls — don't re-run nextest per format.

Windows-runner shell gotcha (recurs on every multi-command CI gate step): a SINGLE-command `run:` step propagates its exit code (GitHub appends a `$LASTEXITCODE` check for the default pwsh shell), but a MULTI-command pwsh block does NOT fail-fast by default — a failing `cargo llvm-cov report --fail-under-lines` mid-block would pass silently. Use `shell: bash` (GitHub's bash runs `set -eo pipefail`) for multi-command report/gate steps, or keep the binding gate as its own single-command step (the shipped shape). Verified locally on Windows before gating (the critical safety check — the prior step only *measured*, so adding `--fail-under-lines` blind could have turned CI red): real coverage 89.89%, gate exit 0 at 60 / exit 1 at 95 (both directions), `cargo audit`+`cargo deny` green, `ci.yml` valid YAML, `agent-run status` smoke exit 0. The full `agent-run run` was not re-run — a ci.yml-only change has zero Rust delta and the nextest leg was already green via the coverage collect.

---

## 2026-06-27 — Tauri operator-pause resolver bridge: an async core seam ↔ webview across the background run thread

The GUI is the third `conductor_core::PauseResolver` shell (after the agent `HeadlessResolver` + the CLI `inquire` `PromptResolver` — the 2026-06-23 "third arm, not a parallel mechanism" entry anticipated this): `conductor_tauri::pause::TauriResolver`. The run drives on a background `std::thread` (a core-owned `current_thread` runtime), while the go/no-go answer comes from the webview — so the resolver bridges thread ↔ webview WITHOUT blocking Tauri's event loop. `resolve(hold)` `arm`s a shared managed `HoldGate` (`Arc<Mutex<Option<oneshot::Sender<Decision>>>>`) with a fresh `oneshot::Sender`, pushes a `HoldPrompt` projection (P-ID/step/prompt + `allow_no_go`) over a **2nd IPC `Channel<HoldPrompt>`** passed to `start_run` alongside the live-counter `Channel<RunEvent>` (a `Channel`, NOT a Tauri `emit`/`listen` event — that would need the `core:event:allow-listen` ACL grant; a Channel is a command ARG, ungated), then awaits the receiver. The `resolve_operator_hold` `#[tauri::command]` `deliver`s the operator's `Decision` by taking the sender from the gate and sending it. A dropped/unanswered sender ⇒ `Decision::NoGo` — the abort-safe default, mirroring the CLI cancel.

`conductor_run::drive_run` was generalized `<R: PauseResolver, E, A>` (was a hardcoded `HeadlessResolver::proceed()`) so the GUI injects the `TauriResolver`; its ONLY non-test caller is the Tauri run thread (the CLI calls `execute_scenario` directly, already generic), so the blast radius was the Tauri caller + 2 unit tests — the CLI release gate (`cli_smoke`) stayed green. The hold only fires on the live-Pulse path (empty-`expected` operator-checklist scenarios P-025/026/027/P-032; every scenario is `Blocked` BEFORE the hold without a live Pulse), so this chunk unit-tests the bridge core (the DI-free `HoldGate` methods) + gallery-demonstrates the dialog, with the live run firing in Epoch-10. `execute_scenario` currently records the decision + returns a `ManualCheck` record UNCONDITIONALLY — acting on a `NoGo` to halt the run is also Epoch-10 semantics, not this chunk. `capabilities/default.json` stayed unchanged (app command + Channel are not ACL-gated — the 2026-06-26 rule); `serde` was added to conductor-tauri's manifest (already a workspace dep — no new `Cargo.lock` package) for the `HoldPrompt` `Serialize`, and tokio's `sync` feature for `oneshot`.

---

## 2026-06-27 — Two RunRecord data-source paths: per-run = the JSONL journal (read in conductor-core); cross-run / per-P-ID = a runs.db query (Epoch-10)

The run-report view (ch7) sources a run's per-scenario `RunRecord`s by reading its `runs/<run_id>.jsonl` journal — **NOT** `runs.db`. The CLI `report` verb already did this (`latest_run_id` → `read_journal` → `serde_json::from_str::<RunRecord>` per line); this chunk **lifted those two fns to `conductor-core`** (new `run_journal` module — `latest_run_id` / `read_run_journal`, typed `CoreError::Config` on IO/parse per the verdict/error wall, the IO-in-core sibling of `scenario_files` / `list_scenarios`) so the CLI `report` verb and the Tauri `run_report` command share ONE reader (DRY). Consequence: the run-report view went **live now with zero new cross-crate edge** (`RunRecord ∈ conductor-core`, already a `conductor-tauri` dep — no `conductor-tauri → conductor-report`) and **zero new `RunsDb` query** — materially cheaper than ch6's deferred coverage join, which is why the /andromeda-phase P4 AskUserQuestion chose "live journal read now" over the ch6 "defer live join" precedent.

The distinction worth remembering for Epoch 10: a query for **all records of ONE run** is a journal read (cheap, no DB, no aggregation); a query for the **latest record per P-ID ACROSS runs** (the coverage matrix's per-P-ID lamp join) is a cross-run aggregate that DOES need a new `conductor-report::RunsDb` accessor (it exposes only `open` / `insert` / `get(run_id, scenario)` today — JSON1 over `p_ids`), still deferred to Epoch 10. Don't conflate them: surfacing a run's report ≠ the cross-run per-P-ID rollup. Two smaller carries from the same chunk: a per-run `RunRecord` already derives `Serialize`, so the Tauri command returns it directly with **no `#[derive]` add** (unlike `CapabilityRow` in ch6); and the operator-checklist **view** shipped presentational / DEV-gallery-only because the induced/observation pairs have **no structured `conductor-core` model** yet (declare-only TOML comments — a future scenario-model field, not this chunk).

---

## 2026-06-26 — Extracting the shared run composition root to a new crate (above the seams) + driving it under Tauri on a background current_thread runtime

ch3's option-A scaffold deferred real GUI execution; ch4 made `start_run` drive the real pipeline by EXTRACTING `conductor-cli`'s bin-local `pipeline.rs` (the composition root) into a NEW library crate `conductor-run`, consumed by BOTH bins. The placement is forced by the crate-per-seam DAG: the composition root imports every seam (`conductor-timeline`/`-emit`/`-verify`/`-report`), and those all depend on `conductor-core` (the base) — so a core-hosted composition root would invert every seam edge into a cycle and won't compile. It must sit ABOVE the seams and BELOW the bins (`{cli,tauri} → conductor-run → {core,timeline,emit,verify,report}`, zero cycles). This refines the 2026-06-23 "second composition root" entry: the two bins SHARE one extracted library (DRY — the CLI's `run`/`suite`/`preflight` re-point to `conductor_run`, `persist` moves there too), they don't each re-compose. `execute_scenario` generalized its resolver from the bin-local `&CliResolver` to a generic `<R: PauseResolver>` (NOT a trait object — RPITIT-async, not object-safe; the 2026-06-21 generic-over-dyn rule) so the CLI passes its `CliResolver` and Tauri the core `HeadlessResolver`. The existing `cli_smoke` E2E staying green is the parity proof the extraction was behaviour-preserving (test-plan Path 7 headless leg).

Driving the engine under Tauri needs a core-owned `current_thread` runtime OFF Tauri's multi_thread shell (the determinism flavor, arch §Async Runtime Flavor). The shipped pattern: the sync `#[tauri::command] start_run` spawns a `std::thread`, which builds `tokio::runtime::Builder::new_current_thread().enable_all().build()` and `block_on`s the pipeline; it returns IMMEDIATELY (non-blocking UI). Progress streams over a Tauri 2 IPC `Channel<RunEvent>` passed as a command ARG (frontend `new Channel()` → `channel.onmessage`), which needs NO capability entry (app-command Channels aren't ACL-gated — the deny-by-default window allowlist is unchanged). `stop_run` sets an `Arc<AtomicBool>` the run loop polls between scenarios (cooperative abort). Without a live Pulse every run resolves Blocked (the preflight gate) and emits nothing, so the live `count` is scenarios-completed (0→N), not per-emission — honest + observable now, with the faithful per-emission counter the Epoch-10 bridge. The `tauri.command.*` manual span + `sanitize_error` edge (obs rule) carry over. Reusable for the remaining Epoch-9 Tauri command chunks (ch5 component primitives, ch8 operator-pause dialog).

---

## 2026-06-24 — A full debug build of the workspace + Tauri tree needs ~33–37 GB of disk

Standing up `conductor-tauri` as a real Tauri 2 app pulls in the full Tauri/wry/tao/webview2-com/windows-* tree (hundreds of crates). A from-scratch `cargo nextest run --workspace` (all test binaries, `debuginfo=2`) drives `target/` to ~33–37 GB — `target/debug/incremental` alone reached ~14 GB. On a near-full disk this surfaces mid-compile as `rustc-LLVM ERROR: IO failure on output stream: no space on device` / `os error 112` / `STATUS_ACCESS_VIOLATION` (rustc crashing as it fails to write), NOT a code error.

Recovery: `cargo clean` frees the whole `target/` (then a full recompile); or delete the regenerable `target/debug/incremental`. The destructive-command guard blocks a raw `rm -rf target/...`, so `cargo clean` is the blessed, non-`rm` way to reclaim the space. The individual gates (per-crate nextest, targeted clippy, `cargo build -p conductor-tauri`, deny/audit) each fit — it's the *combined* full-workspace test compile that exhausts the disk. Budget the headroom before a from-scratch Tauri build.

---

## 2026-06-24 — Edition-2024 makes `std::env::set_var` unsafe; read env handles as triggers instead

Rust **edition 2024** marks `std::env::set_var` / `remove_var` as `unsafe` (mutating the process environment is not thread-safe). So a spec that says a CLI flag "sets `CONDUCTOR_AGENT_MODE=1` internally" (obs-plan §3) should NOT be implemented by writing the env from `main`. The shipped pattern (agent-mode logging chunk, decision D4): treat the env var as a **read-only trigger** — `let agent_mode = cli.agent_mode || std::env::var_os("CONDUCTOR_AGENT_MODE").is_some();` — and thread the resolved `bool` to the consumers (the obs sink selection + `CliResolver::select(…, agent_mode)`). The harness/operator exports the env OR passes the flag; `main` never writes it. Observable mode is identical, no `unsafe`, and the bool is unit-testable via a pure `resolve_kind(agent_mode, stdin_tty, stdout_tty)` (the `render::*_styled(color)` testable-core pattern). When a spec says a flag "sets" an env var, prefer this read-and-thread shape and reconcile the spec wording at wrap.

A related obs gotcha from the same chunk: `tracing_subscriber`'s `build_subscriber<W: MakeWriter>` is generic, but `set_global_default` takes ONE concrete subscriber — you cannot `if agent { build(file) } else { build(stderr) }` (the two `W` types differ). Unify the sinks behind an `ObsWriter { Stderr, File(Arc<Mutex<File>>) }` enum that impls `MakeWriter`, dispatching per-line (the 2026-06-23 `CliResolver` enum-dispatch theme applied to writer-type unification). Open the agent file with create+truncate (the `-latest` name) and fall back to `Stderr` on open failure so startup logging never blocks.

---

## 2026-06-23 — CLI operator-pause resolver: enum dispatch over a non-object-safe trait

`conductor_core::PauseResolver::resolve` returns `-> impl Future<Output = Decision>` (RPITIT), which is NOT object-safe — a `&dyn PauseResolver` will not compile. So the CLI dispatches a fixed `CliResolver { Interactive(PromptResolver), Headless(HeadlessResolver) }` enum (in `conductor-cli::pause`) that impls `PauseResolver` by `match`-ing each arm to its inner resolver's `.resolve(hold).await`. `CliResolver::select(Option<ProgressBar>)` is the isatty gate — interactive only when BOTH `std::io::stdin()` and `stdout()` are `.is_terminal()` (the same `IsTerminal` primitive `render::stdout_color()` uses); off-tty it returns `Headless(HeadlessResolver::proceed())` so the agent path is never gated on a prompt.

The `Interactive(PromptResolver)` variant carries the live `Option<ProgressBar>` (the suite spinner; `run` passes `None`) and wraps the `inquire::Confirm` in `ProgressBar::suspend(…)` so the heartbeat freezes at its current count during the prompt, then resumes. The `[HOLD]` phase-line (`render::hold_line`, amber `lamp_code(Lamp::Hold)=179`, prefix always present) + the confirm both emit to **stderr**, keeping STDOUT the machine-parseable results table. `resolve` is infallible, so an `inquire` cancel/interrupt collapses to `Decision::NoGo` (when `allow_no_go`, else `Go`) via a catch-all arm. The Epoch-9 Tauri go/no-go dialog will extend the SAME enum + `Decision` vocabulary — a third arm, not a parallel mechanism.

---

## 2026-06-23 — conductor-cli line-oriented render seam (owo-colors + indicatif + comfy-table)

The `conductor-cli::render` seam colorizes the existing `Lamp` projection (`Lamp::for_record` / `status_prefix` / `label` from conductor-core — the single status-truth source, never reclassified) and renders comfy-tables. Status is never color-alone: the ASCII `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefix is always present; color is a tty-gated overlay.

Techniques for future CLI rendering (Epoch-8 ch4/ch5 will reuse this seam): (1) ONE `stdout_color()` gate (`std::io::IsTerminal` on stdout + `NO_COLOR` unset + `TERM != dumb`) drives BOTH the owo-colors lines AND the comfy-table cells — comfy-table can't use owo-colors' `if_supports_color`, so a single shared bool keeps lines + tables consistent and guarantees piped/agent output carries zero ANSI escape bytes (this is why the planned owo-colors `supports-colors` feature was dropped — unneeded). (2) Public render fns delegate to private `*_styled(…, color: bool)` cores so tests assert BOTH the plain and colored paths deterministically, independent of how the harness wires stdout — relying on test-capture's tty status is fragile (`cargo test` keeps fd 1 a terminal via thread-local capture; nextest pipes it). (3) `comfy_table::ContentArrangement::Disabled` (content-sized, no wrap/truncate) keeps scenario names + status tokens contiguous so substring E2E assertions hold — the right call for a machine-parseable agent tool over width-detected `Dynamic` (which wraps mid-token). (4) The indicatif spinner draws to STDERR via `ProgressDrawTarget::stderr()`, explicitly swapped to `::hidden()` when `!std::io::stderr().is_terminal()`, so piped/agent runs aren't corrupted (the operator-pause spinner-freeze is ch4). (5) Exact xterm-256 colors: `owo_colors::XtermColors::from(u8)` (lines) + `comfy_table::Color::AnsiValue(u8)` (cells) both take the raw 256-index, matching the design-system §Surface: cli ANSI map (Pass 114 · Fail 203 · Hold 179 · Manual 146 · Residual 246 · Blocked 60 · ID-cyan 117).

---

## 2026-06-23 — `conductor preflight` verb: shape, the hermetic-test manifest fixture, and `.ps1` exit-code gotchas

The `agent-run boot` command shells out to `conductor preflight --json` — the verb ch1 never shipped (`boot` called it from the skeleton, unrecognized, until now). It is a thin serialize-and-exit over a new `pipeline::readiness(manifest_path) -> anyhow::Result<ReadyState>` that REUSES the hardened `ReadbackClient::connect` + `run_preflight` UNCHANGED (no new spawn boundary — the consume-shipped-hardened-infra rule held: the security/obs drift-detectors returned clean without escalating). On an unreachable read-back path it synthesizes the Blocked `ReadyState` inline, mirroring `conductor_verify::preflight_boot`'s Err arm; the `UNREACHABLE_PRECONDITION` string is DUPLICATED in `pipeline.rs` because verify's const + the `spawn::*` helpers are `pub(crate)` (widening them was out of this chunk's scope). `pipeline::preflight` (the run/suite path) was left UNTOUCHED — it retains the `ReadbackClient` for scenario reuse while `readiness()` discards it, so they can't merge without double-spawning the sidecar. Deferred DRY follow-up: expose a `conductor_verify::readiness(data_dir, manifest, canary) -> ReadyState` real-sidecar sibling of `preflight_boot` (its doc already names the `conductor preflight` verb as a driver) to retire the duplication.

Testing gotcha — the hermetic `conductor preflight` test needs MORE than the ch1 forced-unreachable pattern: it must ALSO copy `contracts/mcp-contract.toml` into the TempDir, because `readiness()` loads the manifest BEFORE connecting, so without it the verb errors with manifest-not-found instead of emitting `ready:false`. This is asymmetric with the `conductor run` unreachable leg (`pipeline::preflight`), which early-returns Blocked WITHOUT loading the manifest — so `run`/`suite`/`report` hermetic tests need no manifest fixture, but any `preflight`/`boot` one does.

`.ps1` harness-authoring gotchas (the `.sh`⇄`.ps1` parity surface): use `[Console]::Error.WriteLine(msg); exit N` (NOT `Write-Error`) for deterministic exit codes — under `$ErrorActionPreference='Stop'`, `Write-Error` throws a terminating error so the intended `exit N` becomes dead code (the script exits 1, not N). And never name a PowerShell variable `$env` (it collides with the `$env:` provider) — use `$envelope` etc. The `.sh` `boot` wrapper's `timeout 30 …` works under Git Bash on Windows because `which timeout` resolves to coreutils `/usr/bin/timeout`, not Windows' `timeout.exe`.

---

## 2026-06-23 — The Epoch-8 CLI is the sole composition root; the engine seams are mutually independent, so two content bridges are live-only (Epoch-10)

`conductor-cli`'s `run`/`suite`/`report` verbs are the FIRST place the full pipeline (timeline → emit → verify → report) composes end-to-end. The code-graph `crate_edges` show why: every seam (`conductor-timeline`/`-emit`/`-verify`/`-report`) depends ONLY on `conductor-core`, never on each other — so nothing wired them together before the CLI. The CLI bin may depend on all library seams (it is the composition root); the forbidden edges are library→library. Consequence for Epoch-9: the Tauri GUI is the second composition root and reuses the same wiring shape over the same `conductor-core` types (`Scenario`/`RunRecord`/`Lamp`/`init_observability`).

Two content bridges are a direct consequence of the seam independence and are UNBUILT + live-Pulse-only — they belong to the Epoch-10 "Live-Pulse E2E proof", not the CLI surface: (1) **faithful per-scenario emission** — `EmissionSpec` carries only a coarse `Signal` class (`convert.rs` drops the emission descriptor; the Epoch-3 emit builders were never wired to it), so ch1 coarse-emits one generic span/log per phase by `Signal`, not each scenario's specific incident shape; (2) **per-check read-back observed-extraction** — nothing maps an `ExpectedCheck` to which MCP tool to call + how to slice the observed string, so ch1 feeds a coarse marker into `evaluate_check`. The P4 decision ("coarse live + full Blocked path") wired the full orchestration with these two coarse, deferring fidelity to Epoch-10.

Practical reality for ch1: a run produces a **Blocked** envelope in BOTH CI (the sidecar spawn fails — no `andromeda-pulse-mcp`) AND a default live Pulse (the preflight data-dir **canary** round-trip requires a faithfully-emitted canary incident, which is Epoch-10). So the coarse measured path (emit→read-back→classify) is wired + compiles but is gated behind `ready` and effectively dormant until Epoch-10 lands faithful canary emission. The CI-tested spine is the no-Pulse→Blocked path + the verbs + persist/report; the live measured leg stays operator-gated as always. Empty-`expected` (operator-checklist / declare-only) scenarios produce a verdict-less `ManualCheck` record via direct `RunRecord` construction — no core change, since `Lamp::for_record` already maps `(ManualCheck, None) → Manual`. The CLI-level root `scenario.run` obs span (obs §4 must-trace) was omitted this chunk — a carried code follow-up (obs §4 stays target-state; the seam child-spans + the `run_id`-on-every-line correlation invariant via `init_observability` hold regardless).

---

## 2026-06-21 — Scenario-catalog chunks (Epoch 7): declarative config + `expected` model carrier; runtime realization is deferred to the Epoch-8 driver

A scenario-catalog chunk (the first was connection-lifecycle P-001..P-004) authors **declarative `scenarios/*.toml`** plus the model field needed to express them, and **defers all runtime realization to the Epoch-8 CLI driver**. In scope: the TOML identity (`name`/`p_ids`/`seed`/`slo_tier`), the `[[phases]]` timing structure (legible names + `gap_ms` encoding the lifecycle windows), and the `[[expected]]` read-back targets. Out of scope (the driver wires these later over existing machinery): the emit-on/off + silence realization (Listening = no spans, Idle/Stalled = stopped — `EmissionSpec` is `#[non_exhaustive]`, the extension point if in-config silence is ever wanted), the P-003 `:4317` port-occupier bind (the `conductor-faults` helper already exists; never bind in config/tests), and the live MCP read-back/verify. So the chunk's proof is purely a committed-fixture round-trip (`#[rstest]` over each TOML through `Scenario::from_toml_str` → valid `PhaseTimeline`) + determinism under `start_paused` — no live Pulse, no boot-path change (the `agent-run.sh status` smoke needs a `run_id`, so it is skipped). The per-P-ID parameters (state thresholds, SLO windows) come from `refs/pulse-capability-spec.md` §"Conductor verification" clauses, not from input.md alone.

The `expected` read-back targets wire into the model as an **additive `Scenario.expected: Vec<ExpectedCheck>`** — `#[serde(default)]` (omission stays valid; existing scenarios unaffected) + `#[garde(dive)]` (each check validates at load), reusing the pre-existing `conductor-core::ExpectedCheck`/`ClaimClass`/`ComparisonKind` (already exported) rather than a new type; the `impl From<&Scenario> for PhaseTimeline` reads only `phases`+`jitter_ms`, so the new field is non-breaking (only the in-tree `Scenario { .. }` test struct-literals need the field appended). This closes the carried "Scenario.expected/holds TOML wiring" follow-up; `holds` stayed deferred (drive+observe timing claims need no operator go/no-go — the visual badge is a `ManualCheck` report-state, not a `HoldPoint`). One model limitation to remember for the latency (P-011/P-012) and tracker (P-002) families: **`ComparisonKind` {Exact, Contains, Absent, CountAtLeast} has no tolerance-window kind** — a "±1s" or "p95-within-band" claim is expressed with the closest existing kind and the numeric tolerance left to the Epoch-8 evaluator; resist extending the comparison vocabulary in a config-authoring chunk (keep the model change additive-minimal). The chunk shape (4 files, one P-ID each; wire-expected-now) was a /andromeda-phase P4 AskUserQuestion decision.

---

## 2026-06-21 — Coverage classification (`CoverageMode`) is a third, orthogonal axis; the Epoch-6 matrix is classify-only

The coverage-matrix generator introduced `conductor-core::CoverageMode` (`auto` / `drive+observe` / `static-only`) — Conductor's **third classification axis**, distinct from `Verdict` (Pass/Fail/CalibrationRegion — a per-check *outcome*) and `ClaimClass` (hard/calibration — the assertion *policy*). `CoverageMode` answers a different question: *how* Conductor verifies a capability (drive+assert via MCP read-back · induce+operator-confirm the visual · no-telemetry-dimension/Pulse-owned). Don't conflate them — the coverage matrix classifies by `CoverageMode`, the run report lamps by `Verdict`, the assertion split routes by `ClaimClass`.

The Epoch-6 `coverage-matrix.md` is **classify-only** (user-confirmed at /andromeda-phase): the committed 60-row mode table + a pure Markdown render, with **no `runs.db` read and no `Lamp`**. Although `lamp.rs:9` names the coverage matrix a future `Lamp` consumer, the *status overlay* (latest verdict per P-ID, lamp-rendered) belongs to the **consuming surface that has run data** — the Epoch-8 cli table + Epoch-9 desktop view — not the classification model (no scenarios/runs exist until Epoch 7, so a status column would be all em-dashes). The classification is **code-native**: a committed Rust `static` table (the source of truth), seeded at authoring time from `.andromeda/refs/capability-verification-matrix.json` + input.md §Coverage classification — NOT read from disk at runtime (keeps the render pure, adds no input boundary). Mind the **lens shift**: the refs JSON carries *Pulse's* modes (`automated-nextest`/`by-construction`/…); re-map each P-ID to Conductor's three (the JSON's "Dynamic-verification (Conductor)" notes mark where Conductor owns the timing bound but the visual is operator-observed → `drive+observe`). Completeness — all 60, zero gaps, each exactly one mode — is the definition-of-done gate, asserted by test.

---

## 2026-06-21 — Artifact write lifecycle: regenerated singletons overwrite atomically; per-run artifacts use `create_new`

Two artifact write lifecycles now coexist in `conductor-report` and take **opposite** overwrite semantics — pick by whether the artifact is per-run-immutable or a regenerated singleton. **Per-run artifacts** (`<run_id>.md` report, `<run_id>.jsonl` journal) are run_id-stemmed and immutable, so they open `OpenOptions::create_new` — a repeat write is a loud `Err`, never a silent clobber. **A regenerated definition-of-done singleton** (`coverage-matrix.md`) is the opposite: one canonical file re-derived as the classification evolves, so regeneration MUST succeed — it uses an **atomic deterministic overwrite** (`.tmp` → `fs::rename`, which replaces the destination on both Unix and Windows), and the test asserts a *second* write overwrites cleanly (the inverse of the run report's never-overwrite test).

Watch for borrowed assumptions: this chunk's spec extracts both said the coverage write should be "loud-never-overwrite consistent with the run-report seam" — a wrong carry-over from the run-report lifecycle. A singleton that can't be regenerated is a bug, not a safety feature. The discriminator is artifact identity: run_id-stemmed + immutable → `create_new`; single canonical name + regenerated → atomic overwrite. Both stay "loud" on a real IO fault via the typed `ReportError` (the verdict/error wall).

---

## 2026-06-21 — Verdict-first lamp precedence is centralized in `conductor-core::Lamp`; the render seam is a pure (clock-free) function

The run-report "lamp" (the `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` status) is now a single shared type — `conductor-core::Lamp` with `Lamp::for_record(&RunRecord)` — so the four render surfaces (the Markdown report this chunk, the coverage-matrix Epoch 6 ch4, the cli Epoch 8, the desktop Epoch 9) **reuse one resolver and never re-derive the precedence**. A `RunRecord` carries `verdict` and `state` independently; the lamp is chosen *verdict-first* — but only for the Pass/Fail/Hold trichotomy (`Some(CalibrationRegion)` → `[HOLD]`, the rule that a calibration row reads HOLD, not Manual). The subtlety worth remembering: **`KnownResidual` and `Blocked` are state-driven and MUST be checked BEFORE the verdict arms**, because `RunRecord::measured()` always supplies a `Verdict`, so a `KnownResidual` row carries `Some(verdict)` — a naive "verdict-first else state" would mis-render an accepted residual as `[FAIL]`/`[PASS]`, defeating the state's whole purpose (distinguishing an accepted residual from a real Fail). `Lamp::for_record` matches `(state, verdict)` with `Blocked`/`KnownResidual` first, then the three verdict arms, then a verdict-less state fallback (`ManualCheck` without a verdict → `[MANUAL]`, the operator-checklist case).

The Markdown renderer (`conductor-report::RunReport`) is a **pure function of `(run_id, &[RunRecord])` — it reads no wall-clock**, so its output is byte-deterministic and lockable with an exact-string `assert_eq!` golden (the crate's convention, not `insta`). The temporal anchor is the `run_id` (which already embeds the run timestamp) plus each record's own RFC-3339 instants; a human-facing "generated at" line, if ever wanted, is injected by the cli edge, never read inside the seam — the same "no clock in the seam" discipline as the std::time-vs-tokio journal rule, applied to a one-shot document. The writer `RunReport::write` uses `OpenOptions::create_new` so a repeat `run_id` is a loud `Err`, never a silent clobber (mirrors `RunsDb`'s loud duplicate-key + `JournalWriter`'s never-truncate). The blocked-row null rule renders the five never-measured fields as an em-dash `—`, never the literal `null` or a struct name (artifact hygiene); a measured-but-empty fingerprint set (`Some([])`) renders `(none)`, distinct from a never-measured `None` (`—`).

---

## 2026-06-21 — Mapping a typed envelope onto rusqlite (runs.db): u64 bit-cast · serde-wire-form enums · parse OUTSIDE the row closure

Persisting a `conductor-core::RunRecord` as a `runs.db` row (the new `conductor-report::RunsDb` seam) surfaced three rusqlite mapping gotchas worth reusing for the Epoch-7 P-036 recurrence query + the Epoch-8 `status` read. (a) **`u64` overflows the SQLite signed-`INTEGER` (i64) column** — rusqlite's `ToSql for u64` *errors* when the value exceeds `i64::MAX`, so a `seed: u64` must be stored via an `as i64` bit-cast on write and read back `as u64` (lossless; tested against `u64::MAX`), never bound directly. (b) **Store serde enums as their wire string, not a hand `match`** — `Verdict`/`ReportState`/`SloTier` go into TEXT columns via `serde_json::to_value(v)?.as_str()` and come back via `serde_json::from_value(Value::String(s))`, so the stored spelling stays identical to the JSONL `#[serde(rename)]` (`<5s`, `Pass`, …) with the rename as the single source of truth — a parallel match would silently drift if a variant is renamed.

(c) **You cannot parse serde/JSON inside the `query_row` mapping closure** — that closure must return `rusqlite::Error`, but `serde_json::from_str`/`from_value` (the JSON1 array columns `p_ids`/`fingerprints`, plus the enum wire forms) produce your own error type. Read the raw column values into a private `RawRow` struct inside the closure (all `row.get(i)?`), then convert `RawRow -> RunRecord` OUTSIDE the closure where your `Result<_, RunsDbError>` + `?` compose. The `Blocked`-row NULL rule then falls out for free: the envelope's five measurement fields are already `Option`, so `None` binds to SQL `NULL` and a NULL column reads back as `None` — no special-casing.

The `RunsDb` type mirrors its sibling `JournalWriter`: `open(runs_dir: &Path)` takes the already-resolved dir (the cli edge owns `CONDUCTOR_RUNS_DIR`; the seam never reads env), `CREATE TABLE IF NOT EXISTS` bootstrap (no migration framework), bound parameters only, `#[non_exhaustive]` thiserror `RunsDbError` (`Io`/`Sqlite`/`Json`). Note: rusqlite 0.38.0's first real `bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite **3.50.4** (not the 0.38.0/3.51.1 the specs had assumed — arch §Stack + stack.md corrected this wrap); JSON1 (`json_array_length`) is present and the `≥3.38` floor holds.

---

## 2026-06-21 — clippy `too_many_arguments` counts `&self`: a 7-arg method still trips `8/7`

Under the `cargo clippy --all-targets -- -D warnings` gate, `clippy::too_many_arguments` counts the `self`/`&self` receiver toward its 7-argument threshold — a method with seven non-self parameters fires `8/7` and fails the gate, exactly like an eight-parameter free function. So a faithful many-field constructor needs `#[allow(clippy::too_many_arguments)]` whether it is an associated fn (`RunRecord::measured`, eleven params) OR a method (`CheckOutcome::to_run_record`, `&self` + seven). Do not assume the receiver is exempt. Pair the allow with a one-line justification (here: the run-report envelope is eleven fields by contract — arch §Standard Contracts). (run-report-envelope-serializer chunk, conductor-core + conductor-verify)

---

## 2026-06-21 — A public async trait under the `-D warnings` clippy gate: declare RPITIT, not `async fn`

A public trait method written as `async fn` trips the `async_fn_in_trait` lint (callers can't add a `Send`/lifetime bound on the returned future), which fails the `cargo clippy --all-targets -- -D warnings` gate. Declare it with return-position `impl Trait` instead — `fn resolve(&self, hold: &HoldPoint) -> impl Future<Output = Decision>;` — while implementors may still write `async fn` (an `async fn` in an impl satisfies an `-> impl Future` trait method, stable since Rust 1.75). Keep the orchestration **generic** over the trait (`fn resolve_hold<R: PauseResolver>(…)`) rather than `dyn Trait`: this sidesteps the dyn-incompatibility of RPITIT async methods AND avoids pulling in the `async-trait` crate, so a new async abstraction can land in an otherwise sync/dependency-light crate (here `conductor-core`'s `PauseResolver` — the crate's first async surface) with **zero new runtime dependencies** (only a `tokio` dev-dep for the test executor). (operator-pause-orchestration chunk, conductor-core)

---

## 2026-06-20 — conductor-faults: an *infallible* constructor when a helper has NO failure mode — the third branch of the fault-constructor idiom

The 2026-06-19 entry below split faults-helper construction two ways: a *named domain constraint* → `Result<Self, FaultError>`; an *anonymous pure-value bound* → `Option`. **P-014 `AbruptSilence` is the third branch: no failure mode at all → an infallible `new() -> Self`, with `FaultError` left untouched** — the crate's first infallible helper. Unlike `EmissionGap` (a 20s floor to validate) or `PortOccupier` (a socket to acquire), a *permanent* emission stop has no bound and no resource, so there is nothing to fail on; permanence is modeled as the structural *absence* of a `Duration`, and a positive `resumes() -> bool { false }` accessor makes the no-resume contract testable against `EmissionGap`.

Heuristic completing the prior entry: pick constructor fallibility by whether a *real* failure mode exists — `Result` + a named `FaultError` variant for a threshold/resource failure, `Option` for an anonymous shape bound, and an **infallible `Self`** when there is neither. Do NOT force a `Result`/`Option` "for symmetry" with the siblings — a constructor that is structurally always-`Ok` is the computed-but-never-applied anti-pattern (the same reasoning that kept the seed off the exact-gap helper). User-ratified via /andromeda-phase AskUserQuestion (Helper shape → "Infallible marker").

---

## 2026-06-19 — conductor-faults helpers fail a *named domain constraint* with `Result<_, FaultError>`; emit's pure-value bounds use `Option`

The two seam crates split their constructor-validation idiom by the KIND of invalid input. A **`conductor-faults`** helper whose construction can violate a *named domain constraint* returns `Result<Self, FaultError>`, extending the `#[non_exhaustive] FaultError` enum with a descriptive variant: `PortOccupier::occupy` → `Bind` (a refused loopback bind), and now `EmissionGap::new` → `GapTooShort`/`GapTooLong` (a gap at/below the 20s P-015 restart threshold, or above the 1h ceiling). A **`conductor-emit`** generator validating a *pure-value bound* uses an `Option`-returning constructor instead — `RateCurve::ramp`/`breathing` (`windows>0`, `amplitude<center`), `Severity::new`, `LatencyProfile::new` (the sibling 2026-06-18 emit entry below).

Heuristic for the upcoming faults chunks (P-014 abrupt-silence, P-013 bursty-train): reach for a typed `FaultError` variant when the failure has a *named cause worth surfacing* (a threshold breached, a resource denied) — the name aids the operator and the verdict/error wall; reserve `Option` for anonymous "these numbers don't form a valid shape" bounds. Both honor the rule that invalid input is a value, never a panic. Note the gap is **seed-independent** — an exact `Duration`, deterministic by construction (no seed param to thread), the same seed-only-governs-what-it-drives principle as the 2026-06-18 fingerprint entry in `.claude/rules/testing.md`.

---

## 2026-06-18 — Emission primitives self-validate their typed input in-crate (constructor); core garde is the *later* scenario-wiring validator

Each `conductor-emit` primitive owns and validates its typed input **inside the emit crate** via an `Option`-returning constructor that enforces the invariant — NOT by deferring to `conductor-core`'s garde layer. `Severity::new` (rejects outside `1..=24`, severity-logs) and now `LatencyProfile::new` (rejects unless `p50 ≤ p95 ≤ p99`, latency-shaping) are the pattern. This follows the emission-seam comment in `phase_spec`: the concrete OTLP taxonomy (severity boundaries, fingerprint identity, latency targets, ramps) lands in the Epoch-3 emission seam, not the scenario model.

Consequence for the scenario-wiring epoch: `conductor-core`'s garde is the authoritative validator only *later*, when these targets wire into scenario config by extending the `#[non_exhaustive] EmissionSpec`. The `scenario.rs` note that the p50≤p95≤p99 invariant "joins when the Epoch-3 latency spec lands" refers to that future garde wiring — distinct from the primitive's own constructor check, which is this chunk. So a new emission primitive (topology, PII, ramps) defaults to an emit-local, constructor-validated typed input; don't reach into core's scenario garde for it this early. Complements the placement heuristic in the sibling 2026-06-18 entry (primitive → producing seam crate) with the validation-location dimension.

---

## 2026-06-18 — Emission/compute *primitives* live in their producing seam crate; the *fault* that composes them lives in conductor-faults

The build route places a low-level emission/compute **primitive** in the crate that produces its raw material, even when the module-map one-liner nominally attributes the broader concern to another crate. The per-exception **fingerprint primitive** (`fingerprint()` + the exception-event builder) landed in `conductor-emit` — co-located with the exception content it derives from — NOT in `conductor-faults`, despite arch / CLAUDE.md §Modules listing "fingerprint generation" under faults. That attribution is now narrowed: `conductor-faults` owns the higher-level **fingerprint-storm FAULT** (Epoch-7), which will depend on `conductor-emit` and *compose* this primitive. This recurs from error-spans (its multi-span builder also landed in emit, not faults).

Heuristic for future phase/placement calls: a PRIMITIVE goes in its producing seam crate (`conductor-emit` owns OTLP-message construction + anything derived directly from it, like the content fingerprint); a FAULT that orchestrates/composes primitives goes in `conductor-faults`, built later (Epoch-4+). When the module-map blurb seems to conflict, prefer co-location with the data + the dependency direction (faults → emit), then reconcile the doc (arch §Modules amended this chunk). User-ratified via /andromeda-phase AskUserQuestion.

---

## 2026-06-17 — OTLP emission scaffolding (conductor-emit) notes

The Epoch-3 emission seam builds raw OTLP messages from `opentelemetry-proto` 0.32.0 directly (not the SDK exporter). Two facts for the upcoming emission chunks (error-spans, exception-events, severity-logs, latency, topology, PII, ramps):

- **No `build.rs` / `tonic-prost-build`.** opentelemetry-proto's `gen-tonic` feature ships the generated `TraceServiceClient` (+ server stub) and the message structs (`ResourceSpans` / `Span` / `Status` / …); there is no local `.proto` to compile, so the workspace needs no build script.
- **Build proto structs with `..Default::default()`.** opentelemetry-proto 0.32.0's `KeyValue` carries a third field (`key_strindex`, a newer OTLP string-table index), and other messages gain fields across proto versions. Set only the fields you control and spread `..Default::default()` for the rest — an exhaustive struct literal breaks when a proto-version bump adds a field.

---

## 2026-06-15 — `--profile ci` not defined until the test-framework chunk (regression-gate workaround)

The documented test command `cargo nextest run --workspace --profile ci` (CLAUDE.md §Workflow · `.claude/docs/commands.md` · `.claude/rules/verification-harness.md`) **fails** with `error: profile 'ci' not found (known profiles: default, default-miri)` — the `.config/nextest.toml` that defines the `ci` profile is created by the later Epoch-1 chunk "Test framework + fixtures + coverage tooling" and does not exist yet. Until that chunk lands, every implement chunk's regression gate hits this.

Workaround: run `cargo nextest run --workspace` (the default profile runs the identical test set — only the run-config differs: retries / JUnit / output — and tests are profile-independent). Do NOT create `.config/nextest.toml` ad hoc in an unrelated chunk; that profile is the test-framework chunk's deliverable. Same Foundation-sequencing class as the `playbook.md` rule about interim `cargo test`.

---

## 2026-06-15 — cargo-deny over an unpublished workspace needs `publish = false`

`cargo deny check` treats every workspace member as a *publishable* crate unless it is marked `publish = false`. For the `conductor-*` crates (no `license` field, internal `path` deps) that produced two error classes at once: `error[unlicensed]` (a public crate must declare a license) and `error[wildcard]` ("allow-wildcard-paths is enabled, but does not apply to public crates as crates.io disallows path dependencies"). Both vanish once the crates are `publish = false` — then `[licenses].private.ignore = true` skips their license check and `[bans].allow-wildcard-paths = true` covers their internal `path` deps.

The fix is `publish = false` in `[workspace.package]` + `publish.workspace = true` per crate (matching the existing version/edition/rust-version inheritance). Correct for a local-only, no-cloud tool that never publishes to crates.io — and it strengthens supply-chain posture rather than weakening the gate. Don't reach for `wildcards = "allow"` or dropping the license check to dodge it.

---

## 2026-06-15 — garde 0.22.1 API gotchas (config validation)

Conductor pins **garde 0.22.1**, not the arch's original 0.23.0: `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable here. When wiring garde into a seam crate:
- `derive` is **not** a default feature — the edge must be `garde = { workspace = true, features = ["derive"] }`, or `#[derive(Validate)]` / the `#[garde(...)]` helper attribute won't resolve ("cannot find derive macro `Validate`").
- `Validate::validate(&self)` takes **no** context argument (returns `Result<(), garde::Report>`); call `.validate()`, not `.validate(&())`, for the default `()` context. The error type is `garde::Report`, bridged into `CoreError` via `#[from]`.
- `#[garde(custom(fn))]` is **field-level only** — there is no container/struct-level `custom` in 0.22.1 (it errors "unrecognized attribute"). A whole-list invariant rides on the one field it concerns (e.g. no-duplicate-P-IDs on `p_ids`); invariants spanning *distinct* fields (p50≤p95≤p99, severity-mix sums — the Epoch-2 emission spec) need garde's `Context` pattern or a manual `Validate` impl.

Applies to every future garde validation surface (Epoch-2 `Scenario-config model` especially). See arch §Established Decisions [Validation Library] for the pinned-version decision.

---

---

## 2026-08-10 — A deferral is only real once it has an owned channel

A deferral recorded as report prose is not owned by anything: nothing re-reads a chunk report, so the
obligation evaporates at the next wrap. The owned channels are (a) a `CARRY:` pin appended to the
markerless working-route entry that owns the surface the work belongs to, or (b) a `.andromeda/residuals.md`
append when no in-version entry owns it — check the markerless tail FIRST, because a residual entry is the
cross-version escape hatch, not the default. The pin travels with the entry, so `/andromeda-phase` folds it
into `scope.md` at promotion and it cannot be silently skipped.

Two live confirmations of the cost of getting this wrong. The `sut-load-envelope` chunk deferred a webview
banner with the consequence "a GUI user sees no envelope signal" — real user-visible behavior that would
have been invisible to every later chunk had it stayed in the report. And dismissing a drift proposal in the
same wrap surfaced that the playbook's own deferred-span entries promised `db.insert_run` / `report.generate`
spans would land "with the Epoch-8 cli/timeline caller" — Epoch 8 completed and they never did, because that
deferral lived in a playbook `note` rather than on a route entry. A `note` explains a dismissal; it does not
own the work.

## Entry format

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` (critical, short).
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` (loaded when matching files touched).
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read).
