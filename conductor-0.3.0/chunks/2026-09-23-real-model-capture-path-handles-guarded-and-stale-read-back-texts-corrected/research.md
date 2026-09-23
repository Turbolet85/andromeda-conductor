# Codebase Research — 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected

## Scope
- **Depth:** moderate · **Reads:** 14 (bounded `sed` windows over 11 files) · **Globs/Greps:** 9 · **Graph queries:** 3 (rust plane)
- **Harness rules consulted:** none. The chunk has no live leg, and both captures are compiled and linted here, never fired.
- **Platform issues consulted:** none. No runner-only bullet was folded, and CI on `e799b9e0` was 3/3 green.

## Files inspected
- `crates/conductor-core/src/config_path.rs` (full). `resolve_under(base, candidate)`:
  - It canonicalizes `base`, so the base must exist.
  - It rejects an absolute `candidate` ("path handle must be relative to its base directory") and any `..` component ("path handle must not contain a `..` component"). Both are `CoreError::Config` and neither text carries a path.
  - A candidate that does not exist yet is fine: `joined.canonicalize().unwrap_or(joined)`.
  - It already has three unit tests (accept relative, reject `..`, reject absolute).
- `crates/conductor-core/src/lib.rs:39`: `pub use config_path::resolve_under;`. The function is on the crate's public surface.
- `crates/conductor-run/Cargo.toml`. `conductor-core.workspace = true` is a NORMAL dependency at `:9`, so the test binaries reach it with no manifest edit. `live-pulse = []` is at `:27`, and `serde_json` is dev-only.
- `crates/conductor-run/tests/real_model_live.rs`:
  - `:27` gates the file with `#![cfg(feature = "live-pulse")]`.
  - `:160-166` is `runs_dir()`: `root.join(handle)` with no guard.
  - `:670-687` is `pulse_log()`: `Path::new(&data_dir).join("logs")` with no canonicalize. A failure returns `None`, and the caller at `:521-523` emits the path-free "pulse-log: none (no readable agent-latest.jsonl.* under ANDROMEDA_PULSE_DATA_DIR logs)".
  - `:168-180` is `read_jsonl`, which already prints the file NAME plus the error, never the path.
  - `:292` passes `ANDROMEDA_PULSE_DATA_DIR` to the hardened `ReadbackClient::connect`, which this chunk does not touch.
- `crates/conductor-run/tests/live_suite.rs`:
  - `:45` gates the file with `#![cfg(feature = "live-pulse")]`.
  - `:58-64` is `runs_dir()`, the identical unguarded join.
  - `:66-70` is `capture()`, whose panic prints `path.display()`.
  - `:72-80` is `envelope()`, whose two panics print `journal.display()`.
  - `:88-95` is `journal_of()`, which builds that journal path from `runs_dir()`.
- `crates/conductor-run/tests/journal_conformance.rs:214-245`:
  - Not feature-gated: it runs in the default suite, and the CI journal-conformance step sets `CONDUCTOR_RUNS_DIR=runs/a11y`.
  - It is the same unguarded `root.join(&target)`.
  - Its per-file origin is path-free (`{target}/{name}`), but both panics interpolate `{target}`, the handle's raw VALUE. That value is a host path when it is absolute.
- `crates/conductor-run/tests/real_model_common/mod.rs` (`:1-40`). A `tests/` subdirectory module shared by `real_model_live.rs` (gated) and `real_model_harvest.rs` (default suite). It holds `rule_section` and `mask_host_paths`.
- `crates/conductor-run/tests/common/mod.rs:1-7` sets the precedent for a shared test module: `#![allow(dead_code)]`, compiled per binary.
- `crates/conductor-verify/src/preflight.rs`, windows `:1-20`, `:44-58`, `:195-210`, `:376-407`. `assert_canary` attributes by freshness alone: an empty list gives `NotYet(EmptyCorpus)`, any `opened_at > emitted_at_unix_nano` gives `Ok`, and anything else gives `NotYet(StaleCorpus)`.
- `crates/conductor-run/tests/storm_harvest.rs:1-12` and `contracts/pulse-real-model-leg-posture.md:51-66`.
- `contracts/pulse-run-contract.toml`, the `[[term]]` index (`:38-82`). Order: `l4-deterministic` (shell-declaration, deterministic) → `l4-real-model` (shell-absence, real-model) → `mcp-enabled` (shell-declaration, untagged) → `shared-data-dir` → `sidecar-built` → `incident-formation` → `load-envelope`.
- `.andromeda/architecture.md:93`, `:178` and `:197`, for the registered wording (read by `grep -n`, not in full).

## Graph impact (rust plane)
- **`runs_dir` / `pulse_log` / `capture` / `journal_of`**: the query `calls WHERE callee_name IN (...)` returned 12 rows. All of them are collisions: `runs_dir` in `conductor-tauri/src/commands.rs`, and `capture` inside `conductor-core/src/obs.rs` tests. The `symbol` probe by name (`kind='fn'`) returns only `conductor-tauri/src/commands.rs:45`. **The graph does not index either feature-gated test file**, so the zero hits on the live captures are an index gap, not "no callers". The basis for the test-local call sites is grep instead (`grep -n 'runs_dir()\|pulse_log()'` over the two files): `live_suite.rs:67` and `:94`, `real_model_live.rs:76`, `:226` and `:521`. Every caller is file-local, so the changes have no cross-file blast radius.
- **`resolve_under`**: 19 reference rows. The shipped readers are `conductor-cli/src/paths.rs` (`resolve`, `resolve_handle`, `load_scenario`), `conductor-tauri/src/{commands.rs,main.rs}` (including `tauri_log_path`) and `conductor-run/src/preconditions.rs` (`load_run_contract`). The chunk only CALLS it and never changes it, so no signature changes and no caller threading is owed. Shipped sink resolution is untouched (the obs constraint).

## Patterns detected
- **Path-free failure origin** (`journal_conformance.rs:231-237`): a failure names the handle and the file's own name, never `path.display()`. The inline comment explains that the gate's own output is subject to the rule it enforces.
- **Read fault = name + kind** (`real_model_live.rs:168-180`): an unreadable file prints its NAME and the error, never its path.
- **Shared test module** (`tests/common/mod.rs:7`, `tests/real_model_common/mod.rs:1-3`): a `tests/{name}/mod.rs` included with `mod {name};` by each target that needs it. `#![allow(dead_code)]` absorbs per-target non-use, and the module is never a test target of its own.
- **Env read at the caller, value passed in** (test-plan §10's rule, cited by the tests extract): a guard that takes `Option<&str>` / `&OsStr` is testable without process-env mutation.

## Conventions to follow
- **No `unsafe` env mutation in tests; pass the value in.** test-plan §11 → Integration.
- **Every gated file owes its own clippy:** `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` (test-plan §9; the `live-pulse` gate at `Cargo.toml:23-27`).
- **Comment corrections carry the SUT sha they describe** (the `83d4060` anchor used at `architecture.md:93`).

## New files to create
- `crates/conductor-run/tests/capture_paths/mod.rs`: the shared guards. `runs_dir_under(root, handle)` goes through `conductor_core::resolve_under`, and a data-dir guard canonicalizes and checks is-dir. Both return path-free error text. The module carries `#![allow(dead_code)]`, following the `tests/common` precedent.
- `crates/conductor-run/tests/capture_paths_guard.rs`: a DEFAULT-suite target (not feature-gated) that includes `mod capture_paths;` and holds the hermetic tests. It is needed because both capture files are `live-pulse`-gated, so tests placed only inside them would never run in `nextest`, `cargo test` or CI.

## Files to modify
- `crates/conductor-run/tests/real_model_live.rs`: `runs_dir()` → the shared guard, with callers `:76` and `:226`. `pulse_log()` → canonicalize and check is-dir before `join("logs")`, with caller `:521`. The scrub path (`emit` → `redact_value` → `mask_host_paths`) stays unchanged.
- `crates/conductor-run/tests/live_suite.rs`: `runs_dir()` → the shared guard, with callers `:67` and `:94`. The three panics at `:69`, `:76` and `:80` print the leg or journal FILE NAME plus the error kind, never `.display()`.
- `crates/conductor-verify/src/preflight.rs:202-205` (comment only). *Plus the claim-family sites below, if P4 widens.*
- `crates/conductor-run/tests/storm_harvest.rs:3-5` (header only).
- `contracts/pulse-real-model-leg-posture.md:56` (the L4 bullet only; see the closure below for `:61-62`).
- *Conditional on P4:* `crates/conductor-run/tests/journal_conformance.rs:216-245`.

**Companion sweep for the stale fingerprint claim.** Swept by what the claim SAYS, not by the three named sites: `grep -rnE` for the alternation *no MCP tool reads · reaches no read-back · pinned to [] (optionally backticked) · fixture pins · evidence_refs*, over `crates scenarios contracts scripts` (`*.rs *.toml *.md *.sh *.ps1 *.ts`) returned 19 hits. 9 of them state the retired "no read-back field varies with the emitted payload / the computed fingerprint reaches no read-back surface / `fingerprint_refs` is `[]` or payload-invariant" claim. At `83d4060` that claim is false: `architecture.md:93` records that `fingerprint_refs` carries the cue fingerprint and "varies with what Conductor emitted".
- `crates/conductor-verify/src/preflight.rs:11-14` (module doc): stale.
- `crates/conductor-verify/src/preflight.rs:49-54` (`CanaryMarker` doc): stale.
- `crates/conductor-verify/src/preflight.rs:381-385` (`assert_canary` doc): stale ("no read-back field varies with what Conductor emitted").
- `crates/conductor-verify/src/preflight.rs:202-205`: named by the CARRY. This one is stale in the OPPOSITE direction: it claims fingerprint attribution.
- `crates/conductor-run/src/canary.rs:233-234` (`emit_canary` doc): stale ("reaches no read-back surface … payload-invariant").
- `crates/conductor-verify/tests/common/mod.rs:75-77` (stub field doc): stale ("never from its own computed fingerprint").
- `crates/conductor-run/tests/storm_harvest.rs:3-5`: named by the CARRY.
- `crates/conductor-run/tests/lifecycle_harvest.rs:4-6`: stale in the present tense ("no read-back field varies with what Conductor emitted").
- `scenarios/fingerprint-storm.toml:73-74` (comment): stale ("Pulse exposes no read-back field that varies with what Conductor emitted").

The other 10 hits need no change:
- `scenario.rs:822`, `severity_harvest.rs:387`, `severity-tier-{curious,suggested}.toml:14` and `incident-auto-resolution.toml:26` say the fixture pins SEVERITY or a flag. That is a different, still-true claim.
- `baseline_harvest.rs:5`, `error-baseline-spike.toml:46`, `latency-regression.toml:98` and `pii-scrub.toml:16` are about `span_refs = evidence_refs.span_ids`. That is a different field and a still-true claim.
- `canary.rs:234`'s own line is counted above.
- `fingerprint-storm.toml:69` ("`degraded_mode` PERMANENTLY") is the `test-plan.md:335` claim class, which is out of scope per scope §Boundaries.

## Scope premise closure
- `live_suite.rs :76/:80` panics: **VERIFIED**. Both print `journal.display()`, and `journal` comes from `journal_of` → `runs_dir()` (`:88-95`).
- `journal_conformance.rs:216` as a fourth reader: **VERIFIED**, with one addition. Its panics print the handle VALUE (`{target}`), not only the joined path. Whether to cover it is a P4 fork.
- `resolve_under` is reachable and fits a not-yet-created runs dir: **VERIFIED**. It is at `lib.rs:39` as `pub use`, `conductor-core` is a normal dependency, the doc comment says "`candidate` need not" exist, and the base (the workspace root) exists.
- `pulse_log()` needs a non-`resolve_under` guard: **VERIFIED**. `architecture.md:197-198` registers the handle as the live Pulse's data dir, which is legitimately absolute.
- Hermetic proof suffices: **VERIFIED, with a placement constraint**. Both capture files are `live-pulse`-gated (`:27`, `:45`), so the tests must live in a default-suite target (see New files).
- `contracts/pulse-real-model-leg-posture.md:61-62` as a miscount: **FALSIFIED**. `mcp-enabled` IS the run contract's second `shell-declaration` in file order (`pulse-run-contract.toml:41` then `:57`), and `architecture.md:197` uses the same ordinal. Only `:56` is stale.

## Mechanism equalities the plan relies on (verified at HEAD)
- `resolve_under(workspace_root, Path::new("runs"))` returns `Ok(<root>/runs)`, even when `runs/` does not exist (`config_path.rs:36-38`).
- `resolve_under(root, "/abs")` returns `Err(Config("path handle must be relative to its base directory"))` (`:23-26`). On Windows, a drive-absolute `C:\x` is `is_absolute()`, which is the rejection the tests assert.
- `resolve_under(root, "a/../../x")` returns `Err(Config(...))` with the fixed text "path handle must not contain a `..` component" (`:28-31`). The `..` check runs before any join.
- Every `CoreError::Config` text above is a fixed literal. The only interpolated error is the base-canonicalize `io::Error` Display, which prints an OS message and not the path.
- `assert_canary` returns `Ok` iff some listed incident has `opened_at > canary.emitted_at_unix_nano` (`preflight.rs:397-407`). That decides the B1 wording: freshness, with no fingerprint term in the predicate.

## Open questions
- Does the chunk also guard `journal_conformance.rs:216`? It is the fourth unguarded reader of the same handle, in the default suite and CI-driven, and its panics print the raw handle value. → blocks: plan-decision.
- Does the stale-text correction cover the whole claim family (the 7 extra sites above), or only the three sites the CARRY names? → blocks: plan-decision.
