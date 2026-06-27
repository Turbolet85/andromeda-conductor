# Codebase Research — 2026-06-27-obs-ci-conformance-gate

## Scope
- **Depth:** moderate · **Reads:** 8 (obs.rs, redact.rs, ci.yml, agent-run.sh, agent-run.ps1, main.rs, paths.rs, cli_smoke.rs) · **Globs/Greps:** 3 · **Graph queries:** 1
- The chunk's change surface is `.github/workflows/ci.yml` (+ optionally a `scripts/` checker) — **not a Rust symbol**. The obs sink it asserts is read-only.

## Files inspected
- `crates/conductor-core/src/obs.rs` (full) — **THE self-obs schema authority.** `JsonObsLayer::on_event` (lines 244–275) inserts on **every** line, flat: `timestamp_ms` (u64, `unix_millis`), `level` (`INFO`/`ERROR`/…), `target` (redacted module path), `service.name`, `service.version`, `deployment.environment`, `run_id` — then merges allowlisted event fields. The panic hook (`log_panic`, 163–169) emits a **structured** `tracing::error!(panic=…, location=…, "panic")` line (level `ERROR`), so a captured panic is JSON, never a `^thread.*panicked` backtrace. `init_observability` (71–86) installs the global subscriber + panic hook; a `File` sink that can't open **falls back to stderr** (best-effort).
- `crates/conductor-core/src/redact.rs` (full) — the gate's leak-scan must mirror `is_host_path_token` (104–124): flags drive-letter `X:\`/`X:/`, `/home/`, `/Users/`, `\Users\`, `/root/`, `%APPDATA%`/`%USERPROFILE%`/`%LOCALAPPDATA%`, `$HOME`, `~/`, `.cargo/`, `.rustup/`. It PRESERVES module paths (`conductor_core::obs`) and repo-relative source paths (`crates/…/obs.rs`) — the gate must NOT flag those (`module_and_type_names_are_not_over_redacted` test, 169–176). `ALLOWLISTED_FIELDS` (21–47) = the 7 identity fields + `message`/`phase`/`count`/`panic`/`location` + reserved envelope fields; a non-allowlisted field name is DROPPED at the subscriber (so a `secret` field can never reach the artifact).
- `.github/workflows/ci.yml` (full) — the `rust` job on `windows-latest`. Existing gate-step idioms to reuse: the **flakiness budget** step (`shell: bash` + `grep`, 56–62), the **coverage** steps (`shell: bash` multi-command + `--fail-under-lines`, 84–94), and **two `actions/upload-artifact@v4` steps with `if: always()`** (96–110). The **`Test + lint (dogfood agent-run)`** step (68–72) runs `agent-run.ps1 run` with **no `SCENARIO`**, so `logs/agent-latest.jsonl` is **NOT produced in CI today** — the producer step is genuinely new.
- `scripts/agent-run.sh` (69–71) + `scripts/agent-run.ps1` (70–74) — **both** already carry the optional leg `conductor run $SCENARIO --seed $SEED --agent-mode` when `$SCENARIO` is set (default seed `424242`). So a producer can be either `SCENARIO=… agent-run … run` or a direct `cargo run … -- run … --agent-mode`.
- `crates/conductor-cli/src/main.rs` (full) — `--agent-mode` (or `CONDUCTOR_AGENT_MODE` env) → `obs_sink(true)` = `ObsSink::File(logs/agent-latest.jsonl)` (44–50); init runs BEFORE dispatch, so the file is populated even when the run is Blocked. `agent_mode` is read, never written (edition-2024 `set_var` avoided).
- `crates/conductor-cli/src/paths.rs` (62–70) — `agent_log_path()` resolves `logs/agent-latest.jsonl` as a sibling of the runs dir (moves with `CONDUCTOR_RUNS_DIR`) through the `resolve_under` traversal guard.
- `crates/conductor-cli/tests/cli_smoke.rs` (109–136) — **`agent_mode_routes_self_obs_to_the_log_file_not_stderr`**: the exact producer pattern already exists as an E2E. Runs `conductor run error-baseline-spike --agent-mode` with `ANDROMEDA_PULSE_DATA_DIR=pulse;injection` → asserts `logs/agent-latest.jsonl` exists, the first line carries `run_id`, the file contains `"observability initialized"`, stderr is clean, and the Blocked envelope persists. This is the deterministic, host-independent, no-live-Pulse producer.

## Graph impact (code-graph query — `tree-query-…json`)
- **`init_observability`** — 3 consumers: `conductor-cli main()` (`crates/conductor-cli/src/main.rs:27`), `conductor-tauri main()` (`crates/conductor-tauri/src/main.rs:17`), re-export `conductor-core/src/lib.rs:36`. Bounded + stable; the chunk ASSERTS its output artifact and modifies none of it ⇒ **zero cross-crate blast radius**.

## Patterns detected
- **Reusable CI gate-step idiom** (`ci.yml:56-62, 84-94`): `shell: bash` step + `grep`/tool with non-zero exit → `::error file=…::` annotation → `exit 1`. The conformance + zero-panics gates copy this shape.
- **`if: always()` artifact upload** (`ci.yml:96-110`): two `actions/upload-artifact@v4` steps already upload coverage + JUnit regardless of prior failure; the `agent-latest.jsonl` upload is a third instance.
- **Hermetic no-Pulse Blocked spine** (`cli_smoke.rs:29`, verification-harness.md 2026-06-23): `ANDROMEDA_PULSE_DATA_DIR=pulse;injection` forces `Blocked` before any sidecar spawn → exit 0 → self-obs file still written.
- **`jq -e` on a JSONL line is the established machine-assert** (`agent-run.sh:86` status verb): the conformance gate's field check copies it.

## Conventions to follow
- **Self-obs base schema is the gate's contract, NOT the envelope** (obs.rs:247-268; observability.md "two record shapes"; tests extract): assert `timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id` per line. `verdict`/`state`/`latency_ms`/`fingerprints` belong to `runs/<run_id>.jsonl` (out of scope this chunk).
- **Leak-scan anchors mirror `redact.rs::is_host_path_token`** — flag absolute host markers ONLY; never flag `::` module paths or repo-relative paths (redact.rs:169-176).
- **Reuse the `shell: bash` multi-command + `if: always()` upload scaffold** from `ci-quality-gate-config` (security.md / session-learnings note: Windows multi-command CI steps use `shell: bash`).
- **Redaction boundary DOES apply here** — `agent-latest.jsonl` is Conductor's own artifact (security.md Session Addition 2026-06-27, contrasting `lcov.info`).

## New files to create
- _(mechanism-dependent — resolved at P4)_ Possibly `scripts/obs-conformance-check.sh` — a reusable bash+jq+grep checker invoked by `ci.yml` (and locally), IF the "reusable script" option is chosen over inline `ci.yml` steps.

## Files to modify
- `.github/workflows/ci.yml` — add to the `rust` job: (1) a **producer** step (`conductor run error-baseline-spike --seed 424242 --agent-mode` with `ANDROMEDA_PULSE_DATA_DIR: pulse;injection`) writing `logs/agent-latest.jsonl`; (2) a **conformance + zero-panics gate** (`shell: bash` — per-line required-field `jq` assert + host-path leak `grep` + `^thread.*panicked` grep); (3) an **`actions/upload-artifact@v4`** (`if: always()`) for `logs/agent-latest.jsonl`. No engine/seam change.

## Open questions
1. **Gate mechanism** (P4 AskUserQuestion): (A) inline `shell: bash` jq/grep steps in `ci.yml` (mirrors existing gates, lowest footprint, zero new files) vs (B) a reusable `scripts/obs-conformance-check.sh` invoked by CI + locally (DRY, testable, one more file) vs (C) a Rust `#[test]`/xtask checker (typed serde parse, but ordering-awkward — the artifact must exist first — and heavier). Extracts lean A/B; A matches the all-bash CI precedent.
2. **Producer invocation**: a dedicated `cargo run … -- run error-baseline-spike --seed 424242 --agent-mode` step (explicit, fast) vs `SCENARIO=error-baseline-spike` on the existing dogfood step (reuses the harness leg, but re-runs nextest/clippy). Lean: dedicated step. _(Resolved-leaning; confirm in plan.)_
3. **§3↔§3 record-shape** (carried from scope): **RESOLVED by research** — gate asserts the self-obs base set against `agent-latest.jsonl`; the envelope schema (`runs/<run_id>.jsonl`) is a separate, out-of-scope gate. No user question needed.
