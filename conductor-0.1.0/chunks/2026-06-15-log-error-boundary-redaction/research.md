# Codebase Research — 2026-06-15-log-error-boundary-redaction

## Scope
- **Depth:** moderate · **Reads:** 9 (obs.rs full, error.rs, lib.rs, cli/main.rs, tauri/main.rs, 4× Cargo.toml) · **Globs/Greps:** 3
- The redaction layer composes onto an existing, tested obs stack — no greenfield. The exact processor-stage seam is already ours to edit (`JsonObsLayer`/`JsonVisitor`).

## Files inspected
- `crates/conductor-core/src/obs.rs` (full, 335 lines) — the self-obs stack. `JsonObsLayer<W>::on_event` (`obs.rs:154-183`) builds the flat JSON map: 7 constant identity fields, then `event.record(&mut JsonVisitor(&mut map))` adds event fields, then serialize + newline + `make_writer.write_all`. `JsonVisitor` (`obs.rs:185-211`) inserts each field by `field.name()`. `log_panic` (`obs.rs:86-92`) emits `panic = %payload` + `location = file:line`. Tests use a `SharedBuf` MakeWriter to capture+parse lines (`obs.rs:213-334`).
- `crates/conductor-core/src/error.rs` (full) — `CoreError` (`thiserror`, `#[non_exhaustive]`): `Config(String)` → `"scenario config error: {0}"`, `Validation(#[from] garde::Report)` → `"config validation failed: {0}"`. The typed seam error the anyhow edge will format.
- `crates/conductor-core/src/lib.rs` (full) — `mod obs;` private; re-exports `init_observability, mint_run_id, ServiceIdentity`. A new redaction primitive is added under `obs` and (if public) re-exported here.
- `crates/conductor-cli/src/main.rs` (full, 5 lines) — `fn main() { conductor_core::init_observability("conductor", None); }`. No error path, no `anyhow`, returns `()`.
- `crates/conductor-tauri/src/main.rs` (full, 5 lines) — identical shape, service.name `conductor-tauri`.
- `Cargo.toml` (workspace) — `anyhow = "1.0.102"` declared in `[workspace.dependencies]` but **not consumed by any crate**. `tracing-subscriber` has `json` + `env-filter`. **No `regex` crate** anywhere (only transitive `regex-automata` via tracing-subscriber's EnvFilter).
- `crates/conductor-core/Cargo.toml`, `crates/conductor-cli/Cargo.toml`, `crates/conductor-tauri/Cargo.toml` — cli/tauri depend on `conductor-core` ONLY (no `anyhow` yet); core has serde/serde_json/thiserror/garde/tracing/tracing-subscriber.

## Patterns detected
- **Custom processor-stage layer** (`obs.rs:149-183`): redaction must live INSIDE `JsonObsLayer` (the field-insert path), not as a separate wrapping layer — tracing layers each format independently, so a wrapper can't intercept another layer's fields. This matches obs-plan §11 "apply at the processor, not just the sink."
- **Panic auto-covered** (`obs.rs:86-92` → `on_event`): `log_panic` routes through `tracing::error!` → `on_event` → `JsonVisitor`, so wiring redaction into the visitor automatically scrubs the `panic`/`location` fields — no separate panic-path work.
- **MakeWriter capture test harness** (`obs.rs:218-263`): `SharedBuf` + `build_subscriber(identity, buf.clone(), EnvFilter::new("info"))` + `with_default` is the established pattern for asserting on emitted JSON lines. Redaction tests reuse it verbatim.
- **Injectable env closure** (`obs.rs:37-49`): `resolve_with(.., get_env)` keeps env reads testable (Rust-2024 `set_var` is `unsafe`). Any redaction config that reads env should follow this — but arch extract says redaction config is hardcoded allowlists, not env-driven, so likely no env read at all.
- **Dependency-free std utilities** (`obs.rs:124-142`, `civil_from_unix`): the codebase prefers hand-rolled std over pulling a crate. The scrubber should follow suit (no `regex`).

## Conventions to follow
- **`conductor-<seam>` / snake_case modules** (arch §Naming): primitive lives in `conductor-core::obs` (inline submodule or sibling `redact`), re-exported from `lib.rs` only if cli/tauri need it directly.
- **No new direct deps if avoidable** (security supply-chain invariant): scrub host-paths/`module::` shapes with std string ops, not `regex`; `anyhow` is already workspace-declared (free to wire — not a new lockfile entry, already audited).
- **Test on the public seam / observable behavior** (testing.md): assert on emitted JSON lines (`SharedBuf`) + error-edge stderr, never private fields.
- **Status never color-alone** (design extract): any cli error/redacted output pairs a text prefix (`error:`, `[FAIL]`, `<redacted>`) — no color-only.

## New files to create
- `crates/conductor-core/src/obs/redact.rs` (or inline `mod redact` in `obs.rs`) — the reusable redaction primitive: `redact_value(&str) -> Cow<str>` masking absolute host-path + `module::`/struct-name + backtrace-path substrings → `<redacted>`, plus the field-name allowlist predicate. (Submodule-vs-inline is a plan call; if `obs.rs` becomes `obs/mod.rs`, keep the existing tests.)
- Possibly `crates/conductor-cli/src/` error-reporter (or a `conductor-core` shared `sanitize_error`) — depends on the anyhow-edge depth decision (see Open questions).

## Files to modify
- `crates/conductor-core/src/obs.rs` — wire `redact_value` + the name-allowlist into `JsonObsLayer::on_event` (constant `target`) and `JsonVisitor::record_*` (event values); add redaction unit/golden tests in the existing `#[cfg(test)] mod tests`.
- `crates/conductor-core/src/lib.rs` — re-export the primitive if it must be callable from the binaries / future Epoch-6 writers.
- `crates/conductor-cli/Cargo.toml` + `crates/conductor-cli/src/main.rs` — add `anyhow.workspace = true`; rewire `main` to a sanitizing error path **(scope-fork — see Open questions)**.
- `crates/conductor-tauri/Cargo.toml` + `crates/conductor-tauri/src/main.rs` — same, if the anyhow edge is wired now rather than deferred.

## Open questions
1. **anyhow-edge depth (the scope fork).** The cli/tauri `main`s are 4-line stubs with NO error path yet (clap + commands are Epoch 8). Build the reusable sanitizer primitive now (testable) and **defer** the actual `main() -> Result` rewiring to Epoch 8 (which owns the cli error rendering), mirroring the prior chunk's user-approved lean split — OR rewire both mains to a sanitizing reporter now. → AskUserQuestion at P4.
2. **Field-name allowlist membership + drop-vs-mask.** obs-plan §6 names the 7-field identity set; §11 says "preserve ONLY verdict/state/identity/count fields" — but existing lines legitimately carry `message`/`phase`/`count`. Resolve whether redaction is primarily **value-scrubbing** (mask host-path/`module::` in any value; allow the bounded name set) with the name-allowlist only dropping unexpected names, and whether a masked field shows `<redacted>` (design extract) vs is dropped. Center the plan on value-scrubbing; confirm allowlist membership at P4.
