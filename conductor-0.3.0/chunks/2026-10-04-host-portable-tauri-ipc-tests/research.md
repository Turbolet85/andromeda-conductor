# Codebase Research — 2026-10-04-host-portable-tauri-ipc-tests

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** none — no live leg in this chunk (test-only Rust change; the legs are `cargo nextest` /
  `cargo test` on this host and the Windows CI push). `.claude/rules/testing.md:65` (the 2026-06-27 mock-runtime
  entry, as corrected 2026-10-03) was read by offset because it states the origin rule this chunk changes.
- **Platform issues consulted:** none — no runner-only bullet (the Setup 5a verdict was `in progress`, not a red), and
  the mechanism was read in the framework's own source, `tauri` 2.11.3 from the local cargo registry (the version
  `Cargo.lock` pins: `Cargo.lock:4116-4117`).
- **Code-graph:** skipped — `derived-without-graph` on the `rust` plane (python `duckdb`/`protobuf` not importable on
  this host, health check 11). Call sites were enumerated by grep over `crates/conductor-tauri/src/` (all three source
  files), which is the crate's whole source tree.

## Files inspected
- `crates/conductor-tauri/src/commands.rs` (370-430, 495-560, 655-740) — the `#[cfg(test)] mod tests`: `test_app()`,
  `main_window()` (`WebviewWindowBuilder::new(app, "main", WebviewUrl::default())`), the shared `request()` helper with
  the literal at `:417` and the doc comment at `:409-411`, `invoke()` (panics on any error) and
  `invoke_expecting_error()` (`:512`, returns the rendered error, the caller asserts only `!err.is_empty()`).
- `crates/conductor-tauri/src/pause.rs` (115-142, 225-260) — its own `#[cfg(test)] mod tests`, building its own mock app
  and window (`:225-233`) and an inline `InvokeRequest` with the literal at `:243`.
- `crates/conductor-tauri/src/main.rs` (module list) — `mod commands; mod pause;` plus its own `#[cfg(test)] mod tests`
  (2 tests, no IPC dispatch: `grep -n 'get_ipc_response\|InvokeRequest' crates/conductor-tauri/src/main.rs` → 0).
- `crates/conductor-tauri/Cargo.toml` — `tauri` with `features = ["test"]` in `[dev-dependencies]` (`:24`); no
  `custom-protocol` feature declared, so the test build compiles with tauri's `dev` cfg.
- `crates/conductor-tauri/capabilities/default.json` — `"windows": ["main"]`, three `core:window:*` permissions; no
  `remote`/URL scope. Untouched by this chunk.
- `crates/conductor-tauri/tauri.conf.json` (`:7-8`) — `frontendDist: ui/dist`, `devUrl: http://localhost:5173`. NOT
  read by the mock runtime (see the mechanism below: `mock_context` builds its own `Config` with `build: Default`).
- `.config/nextest.toml` (`:8`, `:17`) — `retries = 0` in both profiles.
- `.claude/rules/testing.md:65` — states (corrected 2026-10-03) that the literal is right on Windows only, and still
  claims "any other value (e.g. `tauri://localhost`) fails dispatch" — false on Linux, where that value IS the origin.
  Wrap curation, not this chunk's code.
- tauri 2.11.3 source (registry): `src/manager/mod.rs:337-365` · `src/manager/webview.rs:43, 430-475` ·
  `src/webview/mod.rs:1698-1730, 1742-1800` · `src/test/mod.rs:205-235` · `src/test/mock_runtime.rs:210, 609-611` ·
  `src/ipc/authority.rs:409` · tauri-utils 2.9.3 `src/config.rs:122-126`.

## The mechanism (re-derived at HEAD; the freight's causal claim, closed)
The dispatch decision is ONE predicate, `Webview::is_local_url(&request.url)` (`webview/mod.rs:1744`), which picks
`Origin::Local` or `Origin::Remote { url }` for the ACL lookup (`:1786-1792`). Its first arm compares the request
URL's scheme + domain against `AppManager::tauri_protocol_url(https)` (`:1702-1706`), and that function is
`cfg!(windows) || cfg!(target_os = "android")` → `http://tauri.localhost`, else `tauri://localhost`
(`manager/mod.rs:339-346`). A non-local origin resolves to a Remote context; no capability admits a remote origin for
an app command, so the dispatch fails with `"{cmd} not allowed. Plugin not found"` (`ipc/authority.rs:409`).

**The equality the fix needs, verified at source for THESE inputs:** for the mock app these tests build
(`mock_context(noop_assets())` — `build: Default::default()`, so no `devUrl` and no `frontendDist` URL,
`test/mod.rs:111-126`) and a window built at `WebviewUrl::default()` (= `App("index.html")`, tauri-utils
`config.rs:122-126`), `prepare_webview` sets the webview URL to `get_app_url(false)` (`manager/webview.rs:443-458`;
`PROXY_DEV_SERVER = cfg!(all(dev, mobile))` is false on desktop, `:43`), and `get_app_url` with no dev/dist URL falls
through to `tauri_protocol_url(false)` (`manager/mod.rs:353-365`). The mock stores that URL verbatim and returns it from
`url()` (`mock_runtime.rs:210`, `:609-611`). So **`window.url()` yields exactly the origin `is_local_url` accepts, on
every host**: `tauri://localhost` on Linux/macOS, `http://tauri.localhost` on Windows. Deriving the request URL from
the window it is dispatched to is host-correct BY CONSTRUCTION, with no per-OS literal in Conductor's tree.
Tauri's own doc example for `assert_ipc_response` uses the other form — a `cfg!(any(windows, target_os = "android"))`
literal pair (`test/mod.rs:216-223`) — which is correct today but restates the framework's rule by hand.

**Measured on this host (Linux, HEAD `f5076ad`):** `cargo nextest run -p conductor-tauri --profile ci` → exit 100,
`26 tests run: 20 passed, 6 failed, 0 skipped`; each failure's text is `"<cmd> not allowed. Plugin not found"`
(log: the session scratchpad's `nextest-tauri-base.log`). The six, by name:
1. `commands::tests::coverage_matrix_command_returns_every_classified_pid`
2. `commands::tests::unbacked_auto_command_returns_the_core_ledger`
3. `commands::tests::run_report_command_returns_a_well_formed_record_list`
4. `commands::tests::run_envelope_command_returns_null_when_no_run_has_an_envelope`
5. `commands::tests::stop_run_command_sets_the_cooperative_abort_flag`
6. `pause::tests::resolve_operator_hold_command_delivers_the_decision`

## A finding the freight did not name: three tests pass VACUOUSLY on Linux
`invoke_expecting_error` (`commands.rs:512`) dispatches through the same `request()` and its three callers assert only
`!err.is_empty()`:
- `run_report_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir` (`:666`)
- `run_envelope_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir` (`:681`)
- `list_scenarios_errors_when_the_capability_manifest_is_unreachable` (`:694`)
(re-derived: `grep -c 'invoke_expecting_error(&\|invoke_expecting_error($' crates/conductor-tauri/src/commands.rs` → 3.)
On Linux the ACL denial is itself a non-empty error, so all three PASS without the command ever running — they are
among the 20 "passed" above. Their comments name them as mutation killers (`Ok(vec![])` / `Ok(None)` mutants); on a
Linux run they would "kill" every mutant without executing the handler. Fixing the origin alone makes them execute
again, but nothing would stop the same vacuous pass recurring under any future origin drift. This is the same
mechanism and the same helper, so it is in scope: the error-expecting path must reject an ACL denial as its error.
`start_run_errors_before_spawning_a_run_thread` (`:705`) calls `start_run` directly, not over IPC — unaffected.

## Graph impact
- **`request()`** (`commands.rs:412`) — derived-without-graph; callers by grep: `invoke()` (`:425`) and
  `invoke_expecting_error()` (`:513`), both in the same test module. Its signature changes (it needs the window to read
  the origin from) — both callers are in this file.
- **`invoke()`** — 6 dispatching tests in `commands.rs` (5 failing ones + `mock_app_registers…` does not dispatch).
- **pause.rs `:237-247`** — the one inline construction; its window (`:230-232`) is in scope at the site.
- No production symbol changes. No caller outside `#[cfg(test)]`.

## Patterns detected
- **Error-is-the-assertion helper** (`commands.rs:510-518`): `invoke_expecting_error` exists because `invoke` panics on
  an error. The hardening extends that helper, so its three callers inherit the guard without edits.
- **Mock app + window per test** (`commands.rs:385-406`, `pause.rs:225-233`): every test builds its own app and its
  own `"main"` window at `WebviewUrl::default()`, so `window.url()` is available at every dispatch site.
- **Doc comment as the in-tree record of the invariant** (`commands.rs:409-411`), citing `.claude/rules/testing.md`.

## Conventions to follow
- **No `unwrap` without a stated reason in test code**: the test modules use `.expect("…")` with a message
  (`commands.rs:400, 406`; `pause.rs:229, 232`); the existing literal's `.parse().unwrap()` is the exception, not the
  convention.
- **Zero retries** (`.config/nextest.toml:8, :17`); **runner portability**: `cargo test -p conductor-tauri` must
  also be green (test-plan §4, 2026-08-20 entry); **formatter-clean** (`cargo fmt --all --check`, CI `rust` job).

## New files to create
- none

## Files to modify
- `crates/conductor-tauri/src/commands.rs` — `request()` derives its URL from the dispatching window; the doc comment
  states the host-dependent truth; `invoke_expecting_error()` rejects an ACL denial as its error; one new negative
  test: a foreign origin is refused on every host.
- `crates/conductor-tauri/src/pause.rs` — the inline `InvokeRequest` takes its URL from its own window.

## Open questions
- none — the form (derive from `window.url()` vs a `cfg!` literal pair) is decided by the artifacts: arch extract
  (prefer the framework's own mechanism over a hand-maintained per-OS table) and tests extract (test-plan §8, read the
  value the runtime provides); recorded as a decisive lean in plan.md.
