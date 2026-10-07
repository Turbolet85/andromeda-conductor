# Conductor

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
Conductor is a desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through its claimed capabilities (the SUT capability manifest's accepted set) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims). Personal/local: solo developer, no cloud, runs beside a real Pulse on the dev host.

**Stack:** Rust 2024 workspace (tokio `current_thread`) · OTLP via opentelemetry-proto/tonic to `127.0.0.1:4317` · MCP read-back via a hand-rolled JSON-RPC client · rusqlite/`bundled` SQLite index · optional Tauri 2 + React 19 GUI · local-only, no network service of its own.

**Key directories:**
- `crates/` — the 9 crate-per-seam workspace members (core + timeline/emit/faults/verify/report + run + cli/tauri bins)
- `scenarios/` — declarative scenario config (serde + garde), each naming its Pulse P-IDs (a P-ID may be named by several)
- `contracts/` — pinned MCP contract manifest + the SUT capability manifest + the SUT load envelope + the SUT run contract + the scenario-assertion audit ledger + the P-025 measurement contract + the real-model leg posture (the two members no Rust code reads)
- `runs/` — per-run JSONL journal + Markdown report + `runs.db` SQLite index
- `scripts/` — `agent-run.{sh,ps1}` headless source-of-truth entrypoint
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`conductor-core`** — runtime-agnostic engine library the other members depend on, `conductor-emit` excepted (shared `Verdict`/`ReportState` types, scenario model, the eleven-key envelope name list).
- **`conductor-timeline`** — deterministic seeded phase scheduler on `tokio::time`.
- **`conductor-emit`** — OTLP raw-type emission primitives (opentelemetry-proto + tonic/prost), gRPC egress to `:4317`; exception events + the fingerprint primitive.
- **`conductor-faults`** — fault helpers: ramps, silence, port-occupier, fingerprint-storm fault (the per-exception fingerprint primitive lives in `conductor-emit`).
- **`conductor-verify`** — MCP read-back client (hand-rolled JSON-RPC over the sidecar's stdio), preflight gate, verdict logic.
- **`conductor-report`** — JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.
- **`conductor-run`** — run composition root library (preflight + scenario execution + `persist` + the live-counter `drive_run` + the fault-phase occupier guard; → `conductor-faults`) shared by both bins.
- **`conductor-cli`** — `agent-run` binary, headless source of truth + release gate.
- **`conductor-tauri`** — Tauri 2 GUI bin (commands + live-counter `Channel`; React 19 webview).
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- **Scope law + trust boundary:** every scenario carries a Pulse P-ID; Conductor's SHIPPED binaries open NO inbound listener — the `:4317` port-occupier is their sole deliberate bind (released on cleanup), and the dev-only webview a11y legs' driver ports (`4444`/`4445`) live and die with the harness, on the routine `--e2e` arm, the operator-local `a11y:driven` arm and the operator-local `sr*` screen-reader suites alike (three suite families over the ONE WebdriverIO + tauri-driver stack; the screen-reader leg additionally spawns the host NVDA named by `CONDUCTOR_NVDA`, a fixed-argv PowerShell window-activation script and, per key, the foreground-guarded fixed-argv key-send script `send-keys.ps1` — rule (b)'s eighth form — none opening a listener). "No UI automation" scopes to PULSE's UI; driving Conductor's OWN webview is in-scope. Never widen beyond the loopback gRPC/MCP-client model — the ONE registered exception is CI-only and reaches no shipped binary and no dev-host leg: the `a11y` job's HTTPS fetch of the version-pinned msedgedriver from `msedgedriver.microsoft.com`, admitted by a pre-execution Authenticode gate (registered 2026-09-17, operator-ratified; it REPLACED the WebView2 Evergreen bootstrapper fetch of 2026-09-08, so the count stays ONE, and its version derives from the image's own WebView2 runtime rather than floating always-latest).
- **Accepted capability set is DATA:** which P-IDs a scenario may name comes from `contracts/pulse-capabilities.toml`, never a compile-time constant — garde asserts the `P-NNN` shape, the manifest asserts membership. A malformed/absent manifest is a `CoreError` harness fault with a named reason: never `Blocked`, never a silent widen. The coverage *classification* stays code-native, held set-equal to the manifest by `check_sut_drift` — so re-aiming at a newer Pulse is a manifest edit **plus** a classification row, never a silent gap. A second gate sits on a different axis: `check_scenario_backing` holds `UNBACKED_AUTO` (the `Auto` claims no scenario names) to exact-set equality against the catalog, so classifying a capability `Auto` can no longer stand in for verifying it.
- **Verdict/error wall:** verification outcomes are typed VALUES (`Verdict`/`ReportState` as `Ok`); `Result::Err` is harness-faults only. Malformed child/transport input (`tonic::Status`, MCP errors) becomes a typed `Blocked`/`Fail`, never a panic.
- **Preflight integrity:** never silently downgrade a preflight that RUNS — `agent-run boot` skips the preflight entirely when its leading `conductor preconditions` arm reports an unmet subject (a non-mutating probe, non-zero exit, every subject named). That skip is CONDITIONAL and not a downgrade: each handle is graded by the KIND of value it carries — the PATH-valued `ANDROMEDA_PULSE_DATA_DIR` by presence, the two flags by an affirmative `"true"`/`"1"` (the deterministic posture; under a real-model scenario's posture — `conductor preconditions --for` — the L4 flag must instead be ABSENT or falsy) — so `handles-declared` is satisfiable and `boot` reaches the gate (measured 2026-09-04: probe exit 0, then `ReadyState` with `ready:true` in both shells). `conductor preflight --json` remains a valid direct entry point. For an invoked gate — each of the gate's FIVE named preconditions (protocol≠`2024-11-05` / missing tool / no incident opened after the canary storm / app-sidecar workspace-key agreement / unmet run-contract terms) surfaces the distinct `Blocked` state with its own host-path-free string, never the generic corpus-empty one. Which run-contract terms bind is selected by the run's L4 posture, and a scenario declaring a posture other than its gate's is `Blocked` at scenario level — never a sixth precondition. A contract term whose truth lives on the SUT's side is recorded `declared-not-observable` and never blocks — blocking on it would claim a measurement Conductor cannot make.
- **Supply chain:** keep `Cargo.lock` committed + un-drifted; never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green; hold toolchain ≥1.94.1 and `tauri` ≥2.10.3.
- **Subprocess hardening:** spawn `andromeda-pulse-mcp` as a fixed hard-coded program NAME resolved through the inherited `PATH` (never operator-chosen) — the same const also has a NON-SPAWNING sibling resolution (`sidecar_resolves_on_path`, a directory walk returning a boolean-grade fact, never the resolved path); pass `ANDROMEDA_PULSE_DATA_DIR` only via `.env(...)` after rejecting injection metacharacters — never into argv/shell.
- **Artifact hygiene:** never leak absolute host paths or internal struct names into logs / run-report / `runs.db` — sanitize at the `anyhow` edge + the tracing-subscriber field-allowlist.
- **Self-obs never exports OTLP:** the only OTLP is the PRODUCT fault stream to Pulse `:4317`; self-observation is `tracing` JSON to stderr/file (no OTel SDK), every line carrying `run_id`; zero unlogged panics (`std::panic::set_hook`).
- **Wall-clock from `std::time`:** journal/report stamps use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (scheduling-only) — journal-relative SLO math depends on it.
- **Status is never color-alone:** pair every Verdict/ReportState with its text label + glyph (desktop) or ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (cli); color encodes run state, never decoration.
- **Determinism is the bar:** same scenario+seed ⇒ same stream shape (span identity alone is salted per execution on the production path; the unsalted dispatcher tier stays seed-pure); loopback gRPC/MCP stubs only in tests (`start_paused` for scheduling); zero-flakiness — no nextest retries.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` |
| Directory tree · resource registry | `.andromeda/architecture.md` §Infrastructure Patterns (keyed: `.andromeda/registries/architecture-contracts.toml`, one file per key) / §Occupied Resources |
| Build plan / chunk route | `.andromeda/master-route.md` · the active version's `working-route.md` (master-route's last `## {project}-{version}` heading) |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/{plane}/tree.db` — one DB per plane (`rust` · `ts`); query via `scripts/code-graph.py query <run_dir> <marker> "<sql>" <plane>` (plane REQUIRED — two are detected); schema + templates in `scripts/code-graph-cookbook.md` |
| MCP read-back contract + preflight gate | `.andromeda/architecture.md` §Standard Contracts |
| Run-report envelope | `.andromeda/architecture.md` §Standard Contracts · `.andromeda/test-plan.md` §3 (keyed: `.andromeda/registries/test-plan-contracts.toml`) |
| Scenario config validation | `.andromeda/security-plan.md` §Input Validation |
| Dependency security / cargo-audit | `.andromeda/security-plan.md` §Dependency Security |
| Subprocess-spawn hardening | `.andromeda/security-plan.md` §Security Anti-Patterns |
| Design tokens / palette / typography | `.andromeda/design-system.md` §Color Palette / §Typography |
| Layout / surfaces (desktop + cli) | `.andromeda/layout-templates.md` |
| Test harness (5-command) | `.andromeda/test-plan.md` §3 (keyed: `.andromeda/registries/test-plan-contracts.toml`, one file per key) |
| CI integration / quality gates | `.andromeda/test-plan.md` §9 / §10 |
| Observability / log JSON schema | `.andromeda/obs-plan.md` §3 (keyed: `.andromeda/registries/obs-plan-contracts.toml`, one file per key) / §6 |
| WCAG / a11y harness | `.andromeda/a11y-plan.md` §3 (keyed: `.andromeda/registries/a11y-plan-contracts.toml`, one file per key) |
| Determinism discipline | `.andromeda/architecture.md` §Cross-cutting Patterns |
| Coverage matrix (manifest set) | `contracts/pulse-capabilities.toml` · `.andromeda/input.md` §Coverage classification |
| Agent harness commands | `scripts/agent-run.sh` · `.claude/rules/verification-harness.md` |
| Stack / versions | `.claude/docs/stack.md` |
| Conventions | `.claude/docs/conventions.md` |
| Commands | `.claude/docs/commands.md` |
| Gotchas | `.claude/docs/gotchas.md` |
<!-- GENERATED:setup:pointer-table end -->

## Workflow
<!-- GENERATED:setup:workflow start -->
**Key commands:**
- `scripts/agent-run.sh run` — the 5-command agent-driven harness (boot/run/status/cleanup/logs); source of truth
- `cargo nextest run --workspace --profile ci` — unit + integration tests (machine-parseable)
- `cargo clippy --workspace --all-targets -- -D warnings` — lint gate
- `cargo audit` — supply-chain gate (must be green before release)
- `cargo build --release` — local release binary (Tauri 2 bundle is convenience)

See `.claude/docs/commands.md` for the full reference.
<!-- GENERATED:setup:workflow end -->

## Architecture
<!-- GENERATED:setup:architecture start -->
Conductor is a modular monolith realized as a crate-per-seam Cargo workspace: the `Cargo.toml` dependency edges *are* the architecture, and a forbidden cross-seam dependency simply will not compile. All engine logic lives in runtime-agnostic library crates that both the headless `agent-run` path (the release gate) and the optional Tauri GUI call identically (for every deterministic-posture scenario; a real-model-posture scenario runs headless only) — headless-drivable core, thin shells.

Determinism is enforced in the runtime flavor: the timeline runs on a `current_thread` tokio runtime so the same scenario+seed always yields the same emission-stream shape. Verification outcomes are typed VALUES (`Verdict`/`ReportState`); `Result::Err` is reserved for Conductor's own harness faults, so the report classifies a model-backed SUT by matching types, not catching exceptions. Every SLO is measured journal-relative (`read_back_observed_at − journal_emitted_at`) against the on-disk JSONL journal — the agent-parseable ground truth.

**Primary source:** `.andromeda/architecture.md` (the pointer table's row — not imported; read explicitly where a step needs it).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

<!-- Maintainer note: The @ imports above MUST each be on their own line — Claude Code only recognizes standalone @path lines as import directives. Inline references like `See @path` or `- @path` are NOT expanded. The 200-line limit applies to CLAUDE.md itself, not to what it imports. Keep @ imports minimal — an import rides every turn of every session, so the block carries only what a session needs before it can ask: the handoff (the bridge). architecture.md and master-route.md are deliberately NOT imported: both grow every version, every skill that needs them reads them explicitly (the loop reads arch's directory tree and resource registry structurally where a plan creates files or mints a resource), and the pointer table names both. This comment is stripped from Claude's runtime context per Anthropic comment-stripping rule. See section-markers.md. -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `session-learnings.md` — curated by `/wrap-session`

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` · `testing.md` · `observability.md` · `a11y.md` · `frontend.md` · `verification-harness.md`
- `host-linux.md` — host recipes for the Bash tool on this host (unconditional; serves the host, not the stack)

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._
- Scope law: no scenario without a Pulse P-ID; Conductor opens no inbound listener of its own (the `:4317` port-occupier is the sole deliberate bind). [corrected 2026-09-01: true of the SHIPPED binaries — the dev-only `--e2e` webview leg's driver stack also binds loopback `4444`/`4445` for the harness's lifetime; and "no UI automation" scopes to PULSE's UI, since driving Conductor's own webview is in-scope self-verification]
- Verdict/error wall: verification outcomes are `Ok(Verdict/ReportState)`; `Result::Err` is harness-faults only — never panic on child/transport input.
- Journal stamps come from `std::time::SystemTime`/`Instant`, never tokio's virtual clock.
- Never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green and a committed, un-drifted `Cargo.lock`.
- Status is never color-alone — every Verdict/ReportState carries a text label + glyph (desktop) or `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` prefix (cli).
- 2026-06-14: Andromeda version builds run on a long-lived `build/conductor-<version>` branch; `main` fast-forwards only when the version is tagged complete — the branch unit is the version, not the task (specializes the global feature-branch rule). (confidence 0.8)
- 2026-08-08: The code-graph is authoritative on call sites — query by `callee_name`, disambiguate by `callee_kind` / `callee_file`, and read a 0-row result as "no callers" only after a `symbol` probe by NAME confirms the symbol is indexed, cross-checking with grep and saying which basis the answer rests on (a "0 callers" claim made before 2026-09-02 is re-measured before reuse). Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-08".
- 2026-08-09: Before asserting that document A says X, grep A — an anchor, an extract, a spec's target-state claim, a dictated citation or an "every"/"no" over a family is a hypothesis until the artifact that holds the fact confirms it, and a claim about an external system is checked on direction, representation, provenance, time, ownership, quantity and completion, never on existence alone. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-09".
- 2026-08-10: A version's `intent.md` and `requirements.md` are IMMUTABLE — no skill ever writes them and no detector or `Expected amendments` entry is aimed at either; a premise research falsifies goes to the LEDGER as a PREMISE-CORRECTION in `verification-matrix.json`'s `notes` (plus the chunk's `scope.md`), and an acceptance is re-worded only as an individually operator-ratified refinement. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-10".
- 2026-08-15: Dissolving every NAMED blocker does not establish that none remains — claim a capability on what a leg MEASURED, never on an argument that the known obstacles are gone, and when a chunk's own proof disproves its premise, un-claim it with the measurement recorded. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-15".
- 2026-08-21: A tool reporting success is not the same as the write landing — verify the artifact, not the exit code: read back what a generated or scripted write was meant to change, capture an exit status before any pipe, read a pattern's hits and ask what other wording, severity or content it admits or misses, validate a probe where the real path passes, and cite the layer that actually carries the row. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-21".
- 2026-08-22: Agent-driven GUI verification is the DEFAULT — the operator's eyes are for judgment items and review; a platform gate or a "manual" verdict is a claim to CHECK (a driver, or the tool's own log), and a `BLOCKED-ON` names the measurable EVENT that clears it, re-verified whenever its entry moves. [corrected 2026-10-07: the dev host is Linux, not "Windows-only"; CI runs on Windows images.] Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-08-22".
- 2026-09-02: The PostToolUse hook runs `rustfmt` ONLY — `cargo clippy --fix` must never run at write time, and a setup re-run must not silently restore it from the hooks-matrix default; `rustfmt` on a crate root reformats that crate's whole module tree, so partition such a chunk's file list into semantic vs formatting-only. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-09-02".
- 2026-09-04: A plan's steps can each be individually unambiguous and jointly contradictory, and no gate looks for that — when a plan predicts a MEASURABLE outcome, name the change that produces it, confirm no other step forecloses that change, and state the prediction as something to measure and record. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-09-04".
- 2026-09-15: A matrix acceptance concretized into clauses that all sit DOWNSTREAM of an unmeasured premise is weaker than the outcome it replaces, and P5's never-weaken card cannot show it — the measurement that would falsify the premise is work the same chunk performs later. Concretize over a population already measured, or leave the acceptance outcome-level until it is. (confidence 0.8)
- 2026-09-17: A token that names one ARM of a fork cannot probe the fork, and a search summary is a claim ABOUT a source, never the source — before reading a 0 as absence ask what wording the other arm produces, fetch the source a summary describes, never take a generic error string as a fact about its literal subject, and count occurrences on a multi-KB line with `grep -oE … | wc -l`, since `grep -c` counts lines. Full text: `.claude/docs/session-learnings.md`, "Tier-1 entry of 2026-09-17".
- 2026-10-01: The operator pass's pre-CI commit and guarded push stay the OPERATOR's acts when the agent performs them on the operator's explicit word — record them as made on that word, never as a skill bypass or a deviation, and never perform them without it. (confidence 0.7)
<!-- USER:session-learnings end -->
