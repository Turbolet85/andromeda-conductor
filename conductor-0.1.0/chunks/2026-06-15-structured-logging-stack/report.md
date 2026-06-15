# Report — 2026-06-15-structured-logging-stack

**Chunk:** Structured logging stack — tracing + tracing-subscriber JSON (no OTel SDK), service-identity + run_id fields, std::panic::set_hook capture
**Date:** 2026-06-15T17:46:44Z
**Commits:** (none yet — this wrap creates the chunk commit)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-core/src/obs.rs` (new) · `crates/conductor-core/src/lib.rs` · `crates/conductor-core/Cargo.toml` · `Cargo.toml` (workspace) · `crates/conductor-cli/src/main.rs` · `crates/conductor-tauri/src/main.rs` · `Cargo.lock`
- **Symbols / APIs:** new public in `conductor-core`:
  - `fn init_observability(default_service_name: &str, run_id: Option<String>) -> ServiceIdentity` — installs the global `tracing` JSON subscriber (stderr) + the panic hook; called by both bins.
  - `fn mint_run_id() -> String` — filesystem-safe `YYYY-MM-DDTHH-MM-SS-mmm` from `std::time::SystemTime`.
  - `struct ServiceIdentity { service_name, service_version, deployment_environment, run_id }` (pub fields).
  - **No** IPC methods / HTTP endpoints / ports / sockets added (no inbound listener — trust boundary intact).
  - **Env vars READ:** `CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV` (both obs-plan §3; within the reserved `CONDUCTOR_*` namespace but NOT individually listed in arch §Occupied Resources env-var list → see Deviations / detector D-arch-resources).
- **Crates / modules:** added module `conductor-core::obs` (private `mod` + flat `pub use`); no new workspace crate.
- **Dependencies:** activated `tracing 0.1.44` + `tracing-subscriber 0.3.23` (features `json` + **`env-filter`** added) into `conductor-core`; `serde_json` promoted dev→runtime in `conductor-core`. Both `tracing` deps were already declared in `[workspace.dependencies]` at the scaffold chunk. Transitive additions in `Cargo.lock`: `tracing-core/-log/-serde/-attributes`, `nu-ansi-term`, `matchers`, `regex-automata`, `regex-syntax`, `sharded-slab`, `thread_local`, `once_cell` (50 locked deps total). **No `opentelemetry-*` / OTel SDK** pulled in.
- **Schema / config:** self-obs JSON **log-line** shape (flat, one object/line): `timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`, + event fields. This is the self-obs line schema only — NOT the run-report envelope (`verdict`/`state`/`latency_ms`/`slo_tier`/`fingerprints` — owned by test-plan §3, emitted by later timeline/verify/report chunks).
- **Coverage of new surfaces:**
  - `conductor-core::obs` self-obs log stream → validation n/a (no external input; env reads have safe defaults) · instrumentation ✓ (this IS the self-obs stack; `run_id` on every line; `std::time` stamps) · PII n/a (synthetic-only; field-allowlist **redaction is the next chunk** — seam built, not wired) · tests unit ✓ (5) · a11y n/a (no UI) · tokens n/a (no UI)
  - `init_observability` (new public entry-point, both bins) → validation n/a · instrumentation ✓ · tests unit ✓ + smoke ✓ · a11y n/a · tokens n/a

## Deviations from intent
1. **Custom JSON `Layer` instead of stock `fmt().json().flatten_event(true)`.** Justified: stock fmt nests span/constant fields under `"span"`/`"spans"` and cannot emit constant `service.*` + `run_id` as **top-level flat fields on every line** (obs-plan §3 "flat fields on every JSONL line" + acceptance). `JsonObsLayer` builds each line's JSON directly. Anticipated by research Open Q2. No OTel SDK, no batch tasks (determinism preserved); serde_json-parseable.
2. **`mint_run_id` uses inline Howard-Hinnant civil-date math (no `chrono`/`time` crate).** Justified: honors the planned `YYYY-MM-DDTHH-MM-SS-mmm` format with zero new date deps (minimal audit surface). Verified correct end-to-end (smoke run_id `2026-06-15T17-44-03-234`; unit test on epoch 0 + 1e9).
3. **Self-obs line timestamp is `timestamp_ms` (epoch millis), not ISO-8601.** Justified: ISO `journal_emitted_at` is the *envelope* field (report/journal seam, later); the self-obs line timestamp format isn't schema-pinned. Epoch millis from `SystemTime` satisfies "std::time, not tokio virtual clock."
4. **`init_observability` emits the startup line itself** (rather than bins calling `tracing::info!`). Justified: keeps the bins' `Cargo.toml` untouched (no direct `tracing` dep — matches the plan's touchpoints) and guarantees one observable line for the smoke.
5. **Public name `init_observability` via flat re-export** (research wording: `obs::init`). Justified: follows the existing `lib.rs` convention (private `mod` + flat `pub use`).
6. **No `deny.toml` change needed** — the new `tracing` tree's licenses passed the existing allowlist (no out-of-scope edit).
7. **`CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` not in arch §Occupied Resources** — defined in obs-plan §3 and within the reserved `CONDUCTOR_*` namespace, but the arch env-var enumeration lists only RUNS_DIR/SCENARIOS_DIR/CONTRACT_MANIFEST/SEED. Candidate arch amendment (register the two obs env handles). Flagged for P2 D-arch-resources.

## Decisions & corrections
- **P4 scope decision (user-approved via AskUserQuestion):** lean + parameterized seam — JSON to stderr now; the file sink (`logs/agent-latest.jsonl`) + `--agent-mode` selection deferred to the Epoch 8 agent-run harness chunk (it owns `CONDUCTOR_RUNS_DIR` + the clap flag). `scope.md` was amended to record this (val-1 intent-incomplete resolution).
- **Research Open Qs resolved:** Q2 → custom JSON layer (flat top-level fields); Q3 → include `env-filter`/`RUST_LOG` (default `INFO`); Q1 → `run_id` mint kept in core, flagged for possible later relocation to the report seam (Epoch 6, runs.db PK).

## Outcome
- **Acceptance criteria: all met.** JSON subscriber (one object/line) ✓ · every line carries `service.name` (override-honored, unit-tested) + `service.version` (`0.1.0`) + `deployment.environment` (`local`) + `run_id` ✓ · panic hook → one ERROR line (unit-tested) ✓ · init in `conductor-core`, both bins call identically (smoke) ✓ · no OTel SDK/exporter/traceparent (audit/deny + dep list) ✓ · `std::time` timestamps ✓ · audit+deny green, lock un-drifted ✓ · `-p conductor-core` + `--workspace` green, clippy clean ✓.
- **Gates (commands run):** `cargo check --workspace --all-targets` ✓ · `cargo nextest run --workspace` ✓ **25/25** (20 prior + 5 new) · `cargo clippy --workspace --all-targets -- -D warnings` ✓ (1 fix: `redundant_closure`) · `cargo audit` ✓ exit 0 (50 deps) · `cargo deny check advisories bans sources licenses` ✓ all ok.
- **Smoke (boot-path changed — both bins):** ✓ each emits the flat startup JSON line (`conductor` / `conductor-tauri`, correct calendar `run_id`).
