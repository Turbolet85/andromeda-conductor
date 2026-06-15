# Codebase Research — 2026-06-15-structured-logging-stack

## Scope
- **Depth:** moderate (early chunk, small codebase; but the obs-plan §3 authority + every insertion point read directly) · **Reads:** 12 · **Globs/Greps:** 2

## Files inspected
- `crates/conductor-core/src/lib.rs` (full) — 5 modules (`config_path`/`error`/`report_state`/`scenario`/`verdict`) + flat re-exports; **no obs module**. Doc comment asserts "Plain data types — no async, and no I/O beyond the single sync `std::fs` path-handle guard" — this chunk adds a documented exception (a side-effecting subscriber init).
- `crates/conductor-core/Cargo.toml` (full) — deps `serde`/`thiserror`/`garde`; dev-dep `serde_json`. **No `tracing` yet.**
- `crates/conductor-cli/src/main.rs` (full) — `fn main() {}` (empty stub; no tokio/clap/anyhow yet — those are Epoch 8).
- `crates/conductor-cli/Cargo.toml` (full) — depends only on `conductor-core`.
- `crates/conductor-tauri/src/main.rs` (full) — `fn main() {}` (stub; full Tauri wiring is Epoch 9).
- `crates/conductor-tauri/Cargo.toml` (full) — depends only on `conductor-core`.
- `Cargo.toml` (workspace, full) — **`tracing = "0.1.44"` + `tracing-subscriber = { version = "0.3.23", features = ["json"] }` already declared** in `[workspace.dependencies]` (also `anyhow`/`tokio`/`serde_json` available). `[workspace.package] publish = false`.
- `.andromeda/obs-plan.md` §3 (179–262) — **the authority** (see Conventions below).
- `.andromeda/context/dependency-tree.md` — core's real tree today is garde/serde/thiserror only (tracing not yet pulled in).
- `.andromeda/context/api-surface.md` — core's current public surface (Verdict/ReportState/Scenario/PId/SloTier/CoreError/resolve_under/Result).
- `.claude/docs/obs-summary.md` + loaded rules `.claude/rules/observability.md` + `.claude/rules/verification-harness.md` — distilled contract + the 5-command harness `logs`/`run` semantics.

## Patterns detected
- **core re-export idiom** (`lib.rs:10-20`): `mod x;` then `pub use x::{…}`. A new `obs` module follows it (`mod obs; pub use obs::{…}`).
- **workspace-dep inheritance** (`crates/*/Cargo.toml`): every dep is `name.workspace = true`. Add tracing via `tracing.workspace = true` / `tracing-subscriber.workspace = true` — no version literal in the crate manifest.
- **bins are empty `fn main(){}` stubs**: wiring the init call is purely additive; no existing bootstrap to refactor.
- **`publish = false` inheritance** (`[workspace.package]`): keeps cargo-deny green; new deps don't change that.

## Conventions to follow
- **Init pattern (obs-plan §3 "OTel SDK init" + "Logging stack"):** `tracing-subscriber::fmt().json().flatten_event(true)` installed at `main` / Tauri-backend startup **before any scenario logic**. NO OTel SDK / exporter / `tracing-opentelemetry` (creator-explicit ban — recursion + determinism guard).
- **Service identity (obs-plan §3 "Service identity"; observability.md):** `service.name` hardcoded `"conductor"` (CLI) / `"conductor-tauri"` (Tauri), override `$CONDUCTOR_SERVICE_NAME`; `service.version = env!("CARGO_PKG_VERSION")`; `deployment.environment = std::env::var("CONDUCTOR_ENV").unwrap_or("local")`. Emitted as **flat per-line JSON fields**, NOT OTel resource attributes.
- **`run_id` on every line (obs-plan §3 "Correlation"):** the sole correlation key — no `trace_id`/`traceparent`. `run_id` format is the filesystem-safe `YYYY-MM-DDTHH-MM-SS-<suffix>` stamp from `std::time::SystemTime` (never tokio's virtual clock).
- **Panic capture (obs-plan §10 SLO + observability.md):** `std::panic::set_hook()` → `tracing::error!(panic=…)` one-line JSON (message + location, no multi-line backtrace spill) → zero unlogged panics. NEVER a retry-once policy.
- **Sinks (obs-plan §3 "Sink"):** CLI dual — stderr (dev) / file `logs/agent-latest.jsonl` (agent mode, under `CONDUCTOR_RUNS_DIR`); Tauri backend file `logs/conductor-tauri.jsonl` + stderr. `--agent-mode` sets `CONDUCTOR_AGENT_MODE=1` (the env var, not the clap flag, is this chunk's mode signal — clap is Epoch 8).
- **Regression gate (this version):** `cargo nextest run --workspace` (**default profile** — `--profile ci` errors until the Test-framework chunk lands `.config/nextest.toml`) + `cargo clippy --workspace --all-targets -- -D warnings` + `cargo audit` / `cargo deny check`. Keep `Cargo.lock` committed + un-drifted.

## New files to create
- `crates/conductor-core/src/obs.rs` — the self-observation init surface: a public `fn` (config: service name + sink mode + `run_id`) that builds the JSON subscriber, stamps service-identity + `run_id` as flat fields on every line, and installs the panic hook; plus a small `std::time`-based filesystem-safe `run_id` mint helper. (Test-framework deps like `assert_cmd` are not available yet → the panic-capture test uses an in-process captured `MakeWriter` + `std::panic::catch_unwind`, not a subprocess.)

## Files to modify
- `crates/conductor-core/src/lib.rs` — add `mod obs;` + `pub use obs::{…}`; update the module doc to note core now owns the (side-effecting) obs init surface — a documented exception to "plain data types".
- `crates/conductor-core/Cargo.toml` — add `tracing.workspace = true` + `tracing-subscriber.workspace = true`; promote `serde_json` to a runtime dep (line serialization / tests).
- `Cargo.toml` (workspace) — add the `env-filter` feature to `tracing-subscriber` (`["json","env-filter"]`) so `RUST_LOG` gating works (obs §6 / observability.md). *(Only if RUST_LOG support stays in scope — see Open Q3.)*
- `crates/conductor-cli/src/main.rs` — call the core obs init at the top of `main` (service.name `"conductor"`).
- `crates/conductor-tauri/src/main.rs` — call the core obs init (service.name `"conductor-tauri"`).
- `Cargo.lock` — will gain the `tracing` subtree; commit it (audit/deny re-run).

## Open questions
1. **`run_id` minting home** — a `std::time`-based, filesystem-safe `run_id` generator is pragmatic in core/obs now, but `run_id` is ultimately the `runs.db` primary key (report seam, Epoch 6). Keep the mint helper in core for reuse, or have obs *accept* a caller-supplied `run_id` only? — lean: accept-or-mint (helper in core, reusable later); flag for possible relocation. Not a blocker.
2. **Flat-field mechanism** — `tracing-subscriber`'s JSON `fmt` nests span fields under `"span"`/`"spans"`; getting `service.name`/`run_id` as **top-level** flat fields likely needs a tiny custom `Layer` (or equivalent field-injection). Implementation detail; acceptance test asserts top-level presence. Not a blocker.
3. **`env-filter` / RUST_LOG scope** — include `RUST_LOG` gating now (adds the `env-filter` feature), or defer to a later obs chunk? Lean: include (cheap, the stack is being stood up). Confirm at review.
