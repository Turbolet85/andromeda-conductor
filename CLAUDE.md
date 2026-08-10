# Conductor

<!-- GENERATED:setup start -->

## Overview
<!-- GENERATED:setup:overview start -->
Conductor is a desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection + verification harness that drives a live Pulse instance through its claimed capabilities (the SUT capability manifest's accepted set) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims). Personal/local: solo developer, no cloud, runs beside a real Pulse on the dev host.

**Stack:** Rust 2024 workspace (tokio `current_thread`) · OTLP via opentelemetry-proto/tonic to `127.0.0.1:4317` · MCP read-back via a hand-rolled JSON-RPC client · rusqlite/`bundled` SQLite index · optional Tauri 2 + React 19 GUI · local-only, no network service of its own.

**Key directories:**
- `crates/` — the 9 crate-per-seam workspace members (core + timeline/emit/faults/verify/report + run + cli/tauri bins)
- `scenarios/` — declarative scenario config (serde + garde), one per Pulse P-ID
- `contracts/` — pinned MCP contract manifest + the SUT capability manifest + the SUT load envelope
- `runs/` — per-run JSONL journal + Markdown report + `runs.db` SQLite index
- `scripts/` — `agent-run.{sh,ps1}` headless source-of-truth entrypoint
<!-- GENERATED:setup:overview end -->

## Modules
<!-- GENERATED:setup:modules start -->
- **`conductor-core`** — runtime-agnostic engine library every other crate depends on (shared `Verdict`/`ReportState` types, scenario model).
- **`conductor-timeline`** — deterministic seeded phase scheduler on `tokio::time`.
- **`conductor-emit`** — OTLP raw-type emission primitives (opentelemetry-proto + tonic/prost), gRPC egress to `:4317`; exception events + the fingerprint primitive.
- **`conductor-faults`** — fault helpers: ramps, silence, port-occupier, fingerprint-storm fault (the per-exception fingerprint primitive lives in `conductor-emit`).
- **`conductor-verify`** — MCP read-back client (hand-rolled JSON-RPC over the sidecar's stdio), preflight gate, verdict logic.
- **`conductor-report`** — JSONL emission journal + Markdown run report + `runs.db` (rusqlite) storage seam.
- **`conductor-run`** — run composition root library (preflight + scenario execution + `persist` + the live-counter `drive_run`) shared by both bins.
- **`conductor-cli`** — `agent-run` binary, headless source of truth + release gate.
- **`conductor-tauri`** — Tauri 2 GUI bin (commands + live-counter `Channel`; React 19 webview).
<!-- GENERATED:setup:modules end -->

## Critical Warnings (universal invariants)
<!-- GENERATED:setup:warnings start -->
- **Scope law + trust boundary:** every scenario carries a Pulse P-ID; Conductor opens NO inbound listener of its own — the `:4317` port-occupier is the sole deliberate bind (released on cleanup). Never widen beyond the loopback gRPC/MCP-client model.
- **Accepted capability set is DATA:** which P-IDs a scenario may name comes from `contracts/pulse-capabilities.toml`, never a compile-time constant — garde asserts the `P-NNN` shape, the manifest asserts membership. A malformed/absent manifest is a `CoreError` harness fault with a named reason: never `Blocked`, never a silent widen. The coverage *classification* stays code-native, held set-equal to the manifest by `check_sut_drift` — so re-aiming at a newer Pulse is a manifest edit **plus** a classification row, never a silent gap. A second gate sits on a different axis: `check_scenario_backing` holds `UNBACKED_AUTO` (the `Auto` claims no scenario names) to exact-set equality against the catalog, so classifying a capability `Auto` can no longer stand in for verifying it.
- **Verdict/error wall:** verification outcomes are typed VALUES (`Verdict`/`ReportState` as `Ok`); `Result::Err` is harness-faults only. Malformed child/transport input (`tonic::Status`, MCP errors) becomes a typed `Blocked`/`Fail`, never a panic.
- **Preflight integrity:** never silently downgrade a failed MCP preflight — each of the gate's FOUR named preconditions (protocol≠`2024-11-05` / missing tool / canary fingerprint absent / app-sidecar workspace-key agreement) surfaces the distinct `Blocked` state with its own host-path-free string, never the generic corpus-empty one.
- **Supply chain:** keep `Cargo.lock` committed + un-drifted; never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green; hold toolchain ≥1.94.1 and `tauri` ≥2.10.3.
- **Subprocess hardening:** spawn `andromeda-pulse-mcp` from a fixed hard-coded path; pass `ANDROMEDA_PULSE_DATA_DIR` only via `.env(...)` after rejecting injection metacharacters — never into argv/shell.
- **Artifact hygiene:** never leak absolute host paths or internal struct names into logs / run-report / `runs.db` — sanitize at the `anyhow` edge + the tracing-subscriber field-allowlist.
- **Self-obs never exports OTLP:** the only OTLP is the PRODUCT fault stream to Pulse `:4317`; self-observation is `tracing` JSON to stdout/file (no OTel SDK), every line carrying `run_id`; zero unlogged panics (`std::panic::set_hook`).
- **Wall-clock from `std::time`:** journal/report stamps use `std::time::SystemTime`/`Instant`, never tokio's virtual clock (scheduling-only) — journal-relative SLO math depends on it.
- **Status is never color-alone:** pair every Verdict/ReportState with its text label + glyph (desktop) or ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (cli); color encodes run state, never decoration.
- **Determinism is the bar:** same scenario+seed ⇒ same stream shape; loopback gRPC/MCP stubs only in tests (`start_paused` for scheduling); zero-flakiness — no nextest retries.
<!-- GENERATED:setup:warnings end -->

## Where to Look
<!-- GENERATED:setup:pointer-table start -->
| Topic | Source |
|---|---|
| Architecture decisions | `.andromeda/architecture.md` |
| Build plan / chunk route | `.andromeda/master-route.md` · the active version's `working-route.md` (master-route's last `## {project}-{version}` heading) |
| Code map / impact (symbols · callers · crate deps) | `.andromeda/cache/tree.db` — query via `scripts/code-graph.py query`; schema + templates in `scripts/code-graph-cookbook.md` |
| MCP read-back contract + preflight gate | `.andromeda/architecture.md` §Standard Contracts |
| Run-report envelope | `.andromeda/architecture.md` §Standard Contracts · `.andromeda/test-plan.md` §3 |
| Scenario config validation | `.andromeda/security-plan.md` §Input Validation |
| Dependency security / cargo-audit | `.andromeda/security-plan.md` §Dependency Security |
| Subprocess-spawn hardening | `.andromeda/security-plan.md` §Security Anti-Patterns |
| Design tokens / palette / typography | `.andromeda/design-system.md` §Color Palette / §Typography |
| Layout / surfaces (desktop + cli) | `.andromeda/layout-templates.md` |
| Test harness (5-command) | `.andromeda/test-plan.md` §3 |
| CI integration / quality gates | `.andromeda/test-plan.md` §9 / §10 |
| Observability / log JSON schema | `.andromeda/obs-plan.md` §3 / §6 |
| WCAG / a11y harness | `.andromeda/a11y-plan.md` §3 |
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
Conductor is a modular monolith realized as a crate-per-seam Cargo workspace: the `Cargo.toml` dependency edges *are* the architecture, and a forbidden cross-seam dependency simply will not compile. All engine logic lives in runtime-agnostic library crates that both the headless `agent-run` path (the release gate) and the optional Tauri GUI call identically — headless-drivable core, thin shells.

Determinism is enforced in the runtime flavor: the timeline runs on a `current_thread` tokio runtime so the same scenario+seed always yields the same emission-stream shape. Verification outcomes are typed VALUES (`Verdict`/`ReportState`); `Result::Err` is reserved for Conductor's own harness faults, so the report classifies a model-backed SUT by matching types, not catching exceptions. Every SLO is measured journal-relative (`read_back_observed_at − journal_emitted_at`) against the on-disk JSONL journal — the agent-parseable ground truth.

**Primary source:** architecture.md (imported below).
<!-- GENERATED:setup:architecture end -->

<!-- GENERATED:setup:imports start -->
@.andromeda/architecture.md
@.andromeda/master-route.md
@.claude/session-handoff.md
<!-- GENERATED:setup:imports end -->

## Deeper Topics
<!-- GENERATED:setup:deeper-topics start -->
On-demand references in `.claude/docs/` (Claude reads when relevant):
- Specialist summaries: `security-summary.md` / `design-summary.md` / `tests-summary.md` / `obs-summary.md` / `a11y-summary.md`
- Core: `stack.md` / `conventions.md` / `commands.md` / `gotchas.md` / `workflow.md`
- `session-learnings.md` — curated by `/wrap-session`

Path-scoped rules in `.claude/rules/` (auto-load when matching files touched):
- `security.md` · `testing.md` · `observability.md` · `a11y.md` · `frontend.md` · `verification-harness.md`

For complete Andromeda documentation: `/andromeda-help`
<!-- GENERATED:setup:deeper-topics end -->

<!-- GENERATED:setup end -->

<!-- USER:session-learnings start -->
## Session Learnings
_This section is curated by `/wrap-session`. It accumulates universal (Tier 1) rules captured from work sessions — rules that apply to every file and every task. Do not edit manually during wrap-session runs — changes are preserved but wrap-session appends new entries here._
- Scope law: no scenario without a Pulse P-ID; Conductor opens no inbound listener of its own (the `:4317` port-occupier is the sole deliberate bind).
- Verdict/error wall: verification outcomes are `Ok(Verdict/ReportState)`; `Result::Err` is harness-faults only — never panic on child/transport input.
- Journal stamps come from `std::time::SystemTime`/`Instant`, never tokio's virtual clock.
- Never `cargo build --release` or merge without `cargo-audit` (+ `cargo-deny`) green and a committed, un-drifted `Cargo.lock`.
- Status is never color-alone — every Verdict/ReportState carries a text label + glyph (desktop) or `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` prefix (cli).
- 2026-06-14: Andromeda version builds run on a long-lived `build/conductor-<version>` branch; `main` fast-forwards only when the version is tagged complete — the branch unit is the version, not the task (specializes the global feature-branch rule). (confidence 0.8)
- 2026-08-08: The code-graph is authoritative on call sites — read it correctly rather than distrusting it. SCIP matches by DESCRIPTOR (`%from_toml_str%`, never `Type::method`, which matches nothing); a row count belongs to the query that produced it; and a `LIMIT`-ed or `head`-ed view is never the result — the run-dir trace's `rows` field is. Grep OVER-counts call sites (it also matches the definition, test-fn names and doc comments); graph lines are 0-indexed, grep's are 1-indexed. (confidence 0.9)
- 2026-08-09: Before asserting that document A says X, grep A. A claim about a document outside the asserting agent's remit is unverified by construction — each /andromeda-phase distiller reads only its OWN source, so its statements about another plan are speculation to check, not findings to propagate. And point every citation at the artifact that actually holds the fact: a COUNT cites the file it was counted from, a RULE cites the spec that states it. Under drift=0 a mis-aimed citation costs a real escalation — it either targets a document that does not carry the fact, or invites authoring content into a spec that was already correct. (confidence 0.6)
- 2026-08-10: A version's `intent.md` and `requirements.md` are IMMUTABLE — no skill ever writes them (intent is the human record of what was ASKED; requirements' ids are contractual). Wrap's amendment flow owns the seven `.andromeda/` masters ONLY, so a detector or an `Expected amendments` entry aimed at either has no sanctioned apply path and must never be raised. When research falsifies a premise those files state, the correction goes to the LEDGER instead: a PREMISE-CORRECTION narrative in `verification-matrix.json`'s `notes` on the affected capability, plus the chunk's `scope.md`. The intent records the ask; the ledger records what turned out true. (confidence 0.9)
<!-- USER:session-learnings end -->
