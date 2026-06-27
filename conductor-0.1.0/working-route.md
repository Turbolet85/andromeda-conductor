# Working Route — conductor-0.1.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve then skips frozen lines); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation
[2026-06-14-cargo-workspace-scaffold] Cargo workspace scaffold — 8 crate-per-seam members, workspace manifest, rust-toolchain pin ≥1.94.1
   ↓
[2026-06-15-conductor-core-shared-types] conductor-core shared types — Verdict/ReportState enums, scenario model, verdict/error wall
   ↓
[2026-06-15-config-validation-surface] Config-validation surface — serde + garde range/cross-field rules, CONDUCTOR_* path-handle canonicalize
   ↓
[2026-06-15-dependency-audit-gate] Dependency-audit gate — cargo-audit + cargo-deny over the OTLP/gRPC/SQLite tree, committed Cargo.lock
   ↓
[2026-06-15-structured-logging-stack] Structured logging stack — tracing + tracing-subscriber JSON (no OTel SDK), service-identity fields, std::panic::set_hook capture
   ↓
[2026-06-15-log-error-boundary-redaction] Log + error-boundary redaction — tracing-subscriber field-allowlist + anyhow-edge sanitization (no host-paths/struct-names/stack-traces)
   ↓
[2026-06-15-design-token-typography-bundle] Design-token + typography bundle — Tailwind v4.1 @theme tokens, JetBrains Mono + IBM Plex Sans
   ↓
[2026-06-16-test-framework-fixtures-coverage-tooling] Test framework + fixtures + coverage tooling — cargo-nextest, rstest, proptest, insta, assert_cmd/fs, cargo-llvm-cov
   ↓
[2026-06-16-base-ci-agent-run-harness-skeleton] Base CI + agent-run harness skeleton — GitHub Actions build/nextest/clippy, agent-run.{sh,ps1} stub

### Epoch 2 — Timeline engine
[2026-06-16-seeded-phase-scheduler] Seeded phase scheduler — current_thread tokio::time deterministic phase sequencing
   ↓
[2026-06-16-scenario-config-model] Scenario-config model — declarative per-phase emission spec, serde + garde validated
   ↓
[2026-06-16-emission-journal-writer] Emission-journal writer — per-run JSONL, std::time wall-clock stamps, tests/obs-owned schema
   ↓
[2026-06-17-determinism-replay-harness] Determinism-replay harness — same scenario+seed yields identical stream shape via insta golden + proptest, tokio start_paused

### Epoch 3 — Emission primitives
[2026-06-17-raw-otlp-message-scaffold] Raw OTLP message scaffold — opentelemetry-proto structs over tonic/prost gRPC egress to :4317
   ↓
[2026-06-17-error-spans] Error spans — Status.Code=ERROR with root-vs-child placement (P-005, P-008)
   ↓
[2026-06-18-exception-events-fingerprint-control] Exception events + fingerprint control — identical/path/line variants, line-insensitive fingerprint (P-006, P-017, P-018)
   ↓
[2026-06-18-severity-logs] Severity logs — SeverityNumber across the 17-boundary (P-007)
   ↓
[2026-06-18-latency-shaping] Latency shaping — target p50/p95/p99 per operation (P-011, P-012)
   ↓
[2026-06-18-multi-service-topology] Multi-service topology — service.name virtual topology + W3C trace propagation (P-008, P-027)
   ↓
[2026-06-18-pii-payload-corpus] PII payload corpus — seven P-047 categories across spans/logs/exceptions (P-035, P-048)
   ↓
[2026-06-18-traffic-rate-ramps] Traffic-rate ramps — halo-breathing emission ramps (P-026)

### Epoch 4 — Fault helpers
[2026-06-19-port-occupier-fault] Port-occupier fault — sacrificial :4317 listener before Pulse starts (P-003 ReceiverFailed)
   ↓
[2026-06-19-emission-gap-resume] Emission gap/resume — exact gap lengths >20s with resume (P-015 restart detection)
   ↓
[2026-06-19-abrupt-silence-fault] Abrupt-silence fault — permanent emission stop (P-014)
   ↓
[2026-06-20-bursty-train-pattern] Bursty-train pattern — active 5min / quiet 10min repeating (P-013 activity-floor)

### Epoch 5 — Verification & read-back
[2026-06-21-mcp-read-back-client] MCP read-back client — rmcp over TokioChildProcess stdio, hardened fixed-path sidecar spawn (.env data-dir)
   ↓
[2026-06-21-preflight-readiness-gate] Preflight readiness gate — pinned 2024-11-05 + tool presence + data-dir canary, Blocked on mismatch
   ↓
[2026-06-21-otlp-egress-liveness-check] OTLP egress liveness check — loopback :4317 connectable, refused ⇒ harness Err
   ↓
[2026-06-21-verdict-assertion-policy-split] Verdict + assertion-policy split — hard Pass/Fail vs CalibrationRegion classification
   ↓
[2026-06-21-expected-outcome-slo-timing-model] Expected-outcome + SLO timing model — per-scenario expected blocks, tier-scaled tolerance <5s/<20s/<90s   PREREQ: feeds matched/observed/expected into the ClaimClass/classify→Assessment mechanism from verdict-assertion-policy-split (concrete comparison kinds deferred here)
   ↓
[2026-06-21-operator-pause-orchestration] Operator-pause orchestration — go/no-go holds + resume-on-confirm for non-Conductor actions

### Epoch 6 — Run report & persistence
[2026-06-21-run-report-envelope-serializer] Run-report envelope serializer — canonical shape shared by Markdown + runs.db + JSONL
   ↓
[2026-06-21-runs-db-index] runs.db index — rusqlite schema, bound-parameter writes, JSON1 fingerprint arrays
   ↓
[2026-06-21-markdown-run-report] Markdown run report — per-scenario Pass/Fail/ManualCheck/KnownResidual/Blocked render
   ↓
[2026-06-21-coverage-matrix-generator] Coverage-matrix generator — all 60 P-IDs classified auto/drive+observe/static-only

### Epoch 7 — Scenario catalog
[2026-06-21-connection-lifecycle-scenarios] Connection-lifecycle scenarios — Listening/Receiving/Idle/Stalled states with orthogonal port-occupier (P-001..P-004)
   ↓
[2026-06-21-hard-signals-scenarios] Hard-signals scenarios — ERROR/exception/severity-boundary/root-vs-deep checks (P-005..P-008)
   ↓
[2026-06-22-error-baseline-spike-latency-regression-scenarios] Error-baseline-spike + latency-regression scenarios — baseline convergence, ramp, candidate persistence (P-009..P-012)
   ↓
[2026-06-22-activity-floor-restart-suppression-scenarios] Activity-floor + restart-suppression scenarios — train/lunch/silence, restart gap, suppression/bypass triple (P-013..P-016, P-057)
   ↓
[2026-06-22-fingerprint-storm-scenarios] Fingerprint-storm scenarios — identity/path/line-variant fingerprints, storm cue thresholds (P-017, P-018)
   ↓
[2026-06-22-severity-lifecycle-scenarios] Severity-lifecycle scenarios — tiered inputs, auto-resolve, ack-retrigger, per-tier SLO (P-019..P-023, P-059, P-060)
   ↓
[2026-06-22-constellation-context-grounding-scenarios] Constellation + context-grounding scenarios — service dot/hue/stability, git commits/recurrence, P-032 known-residual (P-025..P-027, P-036)
   ↓
[2026-06-23-scrub-pipeline-degraded-report-surface-scenarios] Scrub/pipeline/degraded/report-surface scenarios — PII scrub, cadence/hot-reload, model-off, render-timing (P-035, P-037, P-045, P-047..P-056)

### Epoch 8 — CLI surface
[2026-06-23-conductor-run-suite-report-verbs] conductor run/suite/report verbs — clap CLI over current_thread bootstrap
   ↓
[2026-06-23-5-command-agent-run-harness] 5-command agent-run harness — boot=preflight, run=nextest+scenarios, status=runs.db/JSONL read, cleanup=idempotent, logs=journal (.sh + .ps1)
   ↓
[2026-06-23-line-oriented-output-rendering] Line-oriented output rendering — owo-colors/indicatif/comfy-table status lines + coverage table
   ↓
[2026-06-23-isatty-gated-operator-pause] isatty-gated operator-pause — inquire confirm + paused-count spinner mirror, headless never blocks
   ↓
[2026-06-24-sanitized-stderr-agent-mode-logging] Sanitized stderr + agent-mode logging — error:/hint: format, JSON-to-file journal   PREREQ: wire the new --agent-mode flag into `CliResolver::select()` (force the Headless resolver when set) — isatty-gated-operator-pause gated the operator-pause resolver on `IsTerminal` only

### Epoch 9 — Desktop control panel
[2026-06-24-frameless-window-shell] Frameless window shell — Tauri 2 decorations:false drag-region titlebar, deny-by-default capabilities ≥2.10.3   CARRY: wire the Tauri backend's `logs/conductor-tauri.jsonl` file sink via the new `conductor_core::ObsSink` enum (the agent-mode-logging chunk parameterized `init_observability(…, sink)`; `conductor-tauri/main.rs` currently passes `ObsSink::Stderr` — extend `ObsSink` or add a backend-file variant, obs-plan §3)
   ↓
[2026-06-24-paused-count-hold-point-signature] Paused-count hold-point signature — frozen heartbeat freeze/tint/resume in titlebar
   ↓
[2026-06-25-scenario-suite-picker-start-stop] Scenario/suite picker + start/stop — shadcn Command/Select with run controls
   ↓
[2026-06-26-live-counter-channel-stream] Live-counter Channel stream — Tauri Channel backend-to-frontend emission counters + target status   CARRY: retire the DEV-only run-state cycler (App `useState<RunState>` + `import.meta.env.DEV` backtick) and drive the EXISTING `Titlebar runState` prop from this Channel — paused-count-hold-point-signature shipped the prop contract + idle/live/hold/aborted heartbeat/freeze/tint rendering   CARRY: scenario-suite-picker-start-stop (ch3, option-A control scaffold) deferred real run EXECUTION + the pipeline-to-library extraction here — move `conductor-cli`'s bin-local `pipeline.rs` (preflight/execute_scenario/coarse_emit) into a library (conductor-core or a new conductor-run crate), generalize `&CliResolver` → a trait object, stand up a core-owned `current_thread` runtime under Tauri, so `start_run` drives a real `RunRecord` (Blocked w/o live Pulse) persisting runs.db/journal (CLI↔Tauri parity = test-plan Path 7); ch3 shipped the picker + `list_scenarios`/`start_run`/`stop_run` command surface + `RunPhase` lifecycle only. Also: ch3 wired start/stop → `setRunState` directly — reconcile that with the cycler-retire above
   ↓
[2026-06-26-component-primitives-library] Component primitives library — six status-lamp variants + dialog scaffold + operator-checklist primitive
   ↓
[2026-06-27-coverage-matrix-view] Coverage-matrix view — dense single-row-per-P-ID list with verdict/report-state lamps   CARRY: reuse the `StatusLamp` primitive (`ui/src/components/StatusLamp.tsx`, from component-primitives-library) for the per-P-ID lamps — mirror conductor-core `lamp.rs` spellings via `ui/src/lamp.ts` `LAMP_META`, don't re-spell; project each RunRecord → Lamp the verdict-first way `conductor-core::Lamp::for_record` does
   ↓
[2026-06-27-run-report-operator-checklist-views] Run-report + operator-checklist views — verdict lines + ManualCheck induced-state checklist   CARRY: reuse `StatusLamp` (verdict lines) + `OperatorChecklist` (controlled `items` + `onToggle`; native-checkbox induced-state rows) from component-primitives-library — don't rebuild them   CARRY: `lampForRecord` (verdict-first RunRecord→Lamp projection in `ui/src/lamp.ts`) + the read-only-`#[tauri::command]`-returns-the-`conductor-core`-type data-sourcing pattern (`#[derive(Serialize)]` on the core struct + `invoke<T[]>`, never re-author in TS) ship from coverage-matrix-view — reuse both to surface `RunRecord`s as verdict lines
   ↓
[2026-06-27-operator-pause-go-no-go-dialog] Operator-pause go/no-go dialog — AlertDialog gating each committed timeline step   CARRY: the `OperatorPauseDialog` scaffold (Radix AlertDialog — controlled `open`/`onOpenChange` + title/body/Proceed/Abort slots + `allowNoGo`, focus-trap/Escape/restore) SHIPS from component-primitives-library — ch8 WIRES it (drive `open`/`onProceed`/`onAbort` from a `HoldPoint` + the operator-pause command), doesn't rebuild it
   ↓
[2026-06-27-desktop-a11y-harness-setup] Desktop a11y harness setup — axe/Lighthouse/colorjs.io over tauri-driver, shadcn/Radix ARIA binding, token-pair contrast   CARRY: the deferred GUI-integration tests from 2026-06-26-live-counter-channel-stream land with this harness — `tauri::test` mock-runtime command/Channel-frame tests for `start_run`/`stop_run` + the hermetic cross-surface-parity leg (Path 7: Tauri mock-runtime vs CLI subprocess, identical runs.db envelope); deferred because the background-thread Channel stream isn't deterministically assertable in-process (zero-retry bar) and the run logic is already unit-covered in `conductor-run::drive_run` + the `cli_smoke` parity E2E   CARRY (from 2026-06-27-run-report-operator-checklist-views): the run-report + operator-checklist views' GUI/axe assertions also defer here — `tauri::test` for the read-only `run_report` command + tauri-driver axe/contrast/keyboard on `RunReport` (verdict lines, Blocked `—`/null never red) + `OperatorChecklistView` (checkbox Space-toggle, unticked-count `role=status`); both views are build-gated only this chunk   CARRY (from 2026-06-27-operator-pause-go-no-go-dialog): the operator-pause dialog's GUI/axe + the `resolve_operator_hold` command tests also defer here — `tauri::test` mock-runtime for `resolve_operator_hold` (a stored `oneshot` sender → the delivered `Decision`) + the hold `Channel<HoldPrompt>` frame, and tauri-driver axe/keyboard on the wired `OperatorPauseDialog` (alertdialog role, focus-trap, Escape→NoGo, focus-restore, Proceed/Abort label+color never color-alone, reduced-motion fade-drop); the bridge CORE (`HoldGate::arm`/`deliver` decision-capture + abort-default) is unit-covered this chunk, only the Channel/webview leg defers
   ↓
[2026-06-27-ci-quality-gate-config] CI quality-gate config — coverage threshold + flakiness budget + nextest JUnit / llvm-cov artifact upload   NOTE: pulled forward from Epoch 10 — fully Windows-doable (no live-Pulse / Linux+xvfb / webview dep) and it scaffolds the CI surface that "Obs CI conformance gate" + "A11y CI gate + violation JSON" build on, so it leads. Desktop a11y verification (below) + the GUI a11y CI gate stay deferred to a Linux+xvfb environment; the live-Pulse Epoch-10 chunks are NOT Linux-gated (corrected 2026-06-27 — live Pulse runs headless on Windows, see the Live-Pulse E2E NOTE).
   ↓
[2026-06-27-mcp-read-back-result-shape-adapter] MCP read-back result-shape adapter — adapt conductor-verify's read-back client to Pulse's hand-rolled `tools/call` results (raw tool JSON, NO MCP `{content:[...]}` envelope): replace rmcp's typed `call_tool` (returns `Call(UnexpectedResponse)` on every live call) with a raw JSON-RPC request parsed as `serde_json::Value`; make the `stub_pulse_mcp` test stub faithful to Pulse's raw shape; fix `run_preflight`'s `_ => CanaryOutcome::Failed` catch-all that masks call errors as the misleading "incident not found in corpus". PREREQUISITE for Live-Pulse E2E proof — read-back has NEVER worked against the live sidecar (discovered 2026-06-27 implementing the chunk below, against a live Pulse). Evidence: andromeda-pulse `crates/mcp-server/src/jsonrpc.rs` `success(id, result: Value)` (no content wrap) + `dispatch_query_incident_list` → `{items,total,next_cursor}`; conductor `verify::client::call_tool` → `Call(UnexpectedResponse)`. tools/list works (so tool-presence passes), only tools/call fails. Live env proven: pulse-app on :4317 · andromeda-pulse-mcp.exe at `andromeda-pulse/target/debug` · ANDROMEDA_PULSE_MCP_ENABLED=true · ANDROMEDA_PULSE_DATA_DIR=%APPDATA%\andromeda-pulse.
   ↓
[2026-06-27-live-pulse-e2e-proof] Live-Pulse E2E proof — error-baseline-spike/fingerprint-storm/restart-suppression/pii-scrub/connection-lifecycle MCP-verified   NOTE: PREREQUISITE NOW MET (2026-06-27) — the "MCP read-back result-shape adapter" above is COMPLETE; read-back works against the live sidecar (a real `query_incident_list` parses; `UnexpectedResponse` gone). Live env Windows-runnable (sidecar spawns, handshake 2024-11-05, all 4 tools present). REMAINING work for this chunk: the canary EMISSION (the emit→corpus→read-back→fidelity bridge that flips preflight `ready` — confirmed live, the gate now blocks ONLY because nothing emits the canary: precondition "incident not found in corpus", no longer a call error) + the 5-family faithful emit/extract. Canary design (once read-back works): Pulse incident titles are SCRUBBED/generated (not a verbatim marker echo), so fidelity comes via `retrieve_telemetry_slice` `fingerprint_refs` + a fingerprint-storm trigger (≥5 identical-fingerprint exceptions/30s, no baseline) using conductor-emit's Pulse-identical `fingerprint`. Prior planning retained in chunks/2026-06-27-live-pulse-e2e-proof/ (its plan's rmcp-typed-call assumption is superseded by the adapter — regenerate scope/plan on re-promote).
   ↓
Pulse LLM-in-the-loop verification posture — OPEN DECISION (cross-codebase, pending): how Conductor deterministically verifies Pulse's NON-DETERMINISTIC LLM incident pipeline (OTLP → L2 RetryStorm cue → L3 digest → L4 Llama-3.2-3B decides Dismiss/Severity → incident). Leading: a deterministic **test-L4 mode** in Pulse (canned `L4Output` reachable by the live app via env/flag) so the existing incident-readback path becomes deterministic. Alt: verify Pulse's deterministic cue/detection layer (needs a NEW Pulse read-back tool); alt: accept best-effort/tolerant verification. **PREREQ for ALL live incident-readback** — the canary live `ready:true` + every family's MCP-verified leg (verified live 2026-06-27: ingest works, but Pulse creates no incident deterministically). See `.andromeda/architecture-amendments.md` 2026-06-27-live-pulse-e2e-proof + `.claude/rules/verification-harness.md`.
   ↓
Live-Pulse E2E family verification — fingerprint-storm (P-017/P-018) + error-baseline-spike (P-009..P-012) [DEFERRED Part B of [2026-06-27-live-pulse-e2e-proof] — its canary BRIDGE shipped + CI-green; the faithful per-family emit/extract framework was NOT built] + restart-suppression (P-015/P-016/P-057) / pii-scrub (P-035/P-047/P-048) / connection-lifecycle (P-001..P-004 + port-occupier), all MCP-verified.   PREREQ: the LLM-in-the-loop verification-posture decision above. Reuse the shipped canary bridge; build the faithful emit/extract dispatcher + per-check observed extraction (replacing conductor-run's `coarse_emit` + the "incidents-listed" read-back) once the posture lands.
   ↓
Desktop a11y verification — axe violations + contrast + keyboard trap/focus-order on four accessible paths, NVDA/VoiceOver manual spec   CARRY (from 2026-06-27-desktop-a11y-harness-setup): the a11y harness + the authored real-webview specs SHIP from the setup chunk (`ui/wdio.conf.ts` + `ui/test/a11y/*.e2e.ts` — axe-core/colorjs.io/keyboard over @crabnebula/tauri-driver, DISPLAY-GATED) — RUN them on Linux+xvfb + a live Pulse and ASSERT (zero axe violations · token-pair contrast · keyboard-trap/focus-order across the four paths) + add the NVDA/VoiceOver manual spec; do NOT re-author the harness. The deterministic `tauri::test` mock-runtime command tests + the test-plan Path-7 cross-surface parity ALREADY LANDED at the setup chunk (`cargo nextest -p conductor-tauri`, 10/10) — don't re-do them; this chunk is the real-webview sweep + manual screen-reader spec only. The always-on a11y CI gate (violations → obs envelope) is the Epoch-10 "A11y CI gate + violation JSON" chunk, not here.

### Epoch 10 — Polish & ship
Severity-lifecycle full pass — auto-resolve + resolution-summary observed via read-back
   ↓
Cross-surface parity proof — CLI vs Tauri identical envelope for the same seed
   ↓
[2026-06-27-obs-ci-conformance-gate] Obs CI conformance gate — agent-latest.jsonl upload + log-schema conformance + zero-unlogged-panics check
   ↓
A11y CI gate + violation JSON — axe/contrast/keyboard PASS/FAIL into obs envelope, service-tagged   CARRY (from 2026-06-27-obs-ci-conformance-gate): reuse the concrete ci.yml obs-gate scaffold this chunk landed — the `shell: bash` gate-step idiom (jq/grep assertions + `::error::` annotation + non-zero exit) + the `actions/upload-artifact@v4` `if: always()` upload — to emit axe/contrast/keyboard PASS/FAIL into the obs envelope; the violation JSON is Conductor's OWN artifact, so the redaction boundary applies (unlike a third-party tool's output). NOTE: unlike the obs gate (Windows-doable), the a11y harness that PRODUCES the violations is display-gated (Linux+xvfb), so this is NOT a Windows-only pull-forward.
   ↓
Coverage-matrix completeness gate — 60 P-IDs zero-gap, all CI gates green (definition of done)
   ↓
Release build + Tauri bundle — cargo build --release + Tauri 2 bundle, final SLO verification
