# Codebase Research — 2026-08-13-first-live-green-preflight

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 6 · **Graph queries:** 4 (`tree-query-2026-08-13-first-live-green-preflight.json`)

## Files inspected
- `crates/conductor-run/src/lib.rs` (`:30-230`) — the whole live preflight composition: `preflight` / `readiness` → `canary_gate` → `observe_run_contract` → `emit_canary` (+ `warm_up_canary_service`) → `run_preflight` with `canary_poll`. Everything the live leg exercises is here and shipped.
- `crates/conductor-verify/src/preflight.rs` (`:120-340`) — the five-arm precondition cascade, `poll_canary`, `assert_canary`, `preflight_boot`. Confirms arm ORDER: version → tools-unverifiable → tools-absent → **run-contract** → canary-call-error → canary-not-found, and that an unsatisfied contract SKIPS the poll (`:183-187`).
- `crates/conductor-verify/src/extract.rs` (full) — the key-diff target. Expected keys are exactly `items[].{id|incident_id,status,severity,title}` · `markdown` · `degraded_mode` · `span_refs` · `fingerprint_refs`. Every reader returns `unwrap_or_default()` on a shape miss.
- `crates/conductor-verify/src/spawn.rs` (full) — fixed program `andromeda-pulse-mcp` resolved from PATH, `.env(ANDROMEDA_PULSE_DATA_DIR, …)` only, injection-reject, platform default when unset.
- `crates/conductor-verify/src/client.rs` (signatures) — the four corpus wrappers all take `Option<Value>` and return raw `Value`; `call_tool` is public.
- `crates/conductor-cli/src/commands/preflight.rs` (full) + `crates/conductor-cli/src/cli.rs` (`:43-48`) — the verb has exactly one flag (`--json`); it serializes `ReadyState` and exits 0/1.
- `crates/conductor-cli/src/paths.rs` (`:27-33`, `:86-94`) — every contract/manifest handle resolves through `resolve_under(std::env::current_dir(), …)`.
- `crates/conductor-core/src/config_path.rs` (`:17-44`) + `run_contract.rs` (`:129-135`) — `default_path()` is the RELATIVE `contracts/pulse-run-contract.toml`; `resolve_under` joins it onto the **current working directory**.
- `contracts/pulse-run-contract.toml` (full) — `warmup_ms = 45000`, `warmup_emissions = 3`, `min_canary_poll_seconds = 90`; 5 terms, of which exactly one (`l4-deterministic`) is `shell-declaration` and therefore the only one that can block.
- `scripts/agent-run.sh` (`:1-80`) + `scripts/agent-run.ps1` (`:20-60`) — the `boot` verb on both shells.
- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` (full) — the prior probe's recorded method and results.

## Graph impact (`rows` from the trace, not a clipped view)
- **`run_preflight`** — 7 call sites: production `crates/conductor-run/src/lib.rs:135` (inside `canary_gate`) + the `conductor-verify` re-export at `src/lib.rs:50`; the rest are tests (`tests/preflight.rs:12,48,177`). Single production caller ⇒ any change to the gate has a one-site blast radius.
- **`observe`** — 38 rows on a deliberately broad `%observe%` predicate (it also matches `observed`/`observe_run_contract`). Production sites that matter: `conductor-run/src/lib.rs:122` (`observe_run_contract` inside `canary_gate`), `:152` (its definition), `:301`/`:346` (`execute_scenario`'s read-back + per-check fold).
- **`crate_edges` for `conductor-verify`** — 3 rows: `conductor-cli → conductor-verify`, `conductor-run → conductor-verify`, `conductor-verify → conductor-core`. No new edge is needed for anything in scope.
- **`%canary%` / `%warm%` / `%incident_formation%`** — 18 rows; the warm-up + poll machinery lives at `conductor-run/src/lib.rs:174` (`emit_canary`), `:199` (`warm_up_canary_service`), `:221` (`canary_poll`), with the contract fields on `IncidentFormation` at `conductor-core/src/run_contract.rs:49-53`.

## Patterns detected
- **The gate is a pure function of injected inputs** (`preflight.rs:135-142`): client, manifest, `RunContractStatus`, canary marker, data-dir string, poll budget. The env read happens at the caller (`conductor-run/src/lib.rs:151-168`), per the 2026-08-10 rule. Any live diagnostic must not reach for `std::env` inside the gate.
- **Two `Blocked`-not-found causes share one wire shape** (`preflight.rs:269-286`): `NotFound::EmptyCorpus` → the workspace-key precondition, `NotFound::FingerprintAbsent` → the canary-fingerprint precondition. The 2026-08-10 probe's three arms all landed on the first.
- **Every read-back reader degrades to empty, never errors** (`extract.rs:147-171`): `incident_ids`, `fingerprint_refs`, `string_array`, `list_text` all end in `unwrap_or_default()`. This is the silent live failure mode the CARRY names, and it is by design (the verdict/error wall), not a defect to "fix".
- **`assert_canary` and `observe` read the SAME shapes independently** (`preflight.rs:325-339` vs `extract.rs:76-118`) — both call `incident_ids` + `fingerprint_refs`. So a live key mismatch on `items[].id` breaks the canary AND the per-check observation identically; a mismatch on `markdown`/`degraded_mode`/`span_refs` breaks only the observation, silently.
- **Warm-up is gated on contract satisfaction** (`conductor-run/src/lib.rs:126`, `emit_canary(&contract, status.is_satisfied())`): when the run contract is unmet, the storm still emits but the 45s pre-roll is skipped. So an arm that blocks on the L4 declaration never exercises `[incident_formation]` at all.

## Conventions to follow
- **`Ok(Blocked)` for every readiness outcome, `Err` only for harness faults** — `run_preflight` returns `Result<ReadyState, VerifyError>` but the `Err` arm is unreachable from the cascade (`preflight.rs:218`); `canary_gate` turns an emission failure into a Blocked state (`conductor-run/src/lib.rs:126-135`).
- **`redact_value` at every value boundary before it enters an artifact** — `preflight.rs:144, 171, 249, 260`; `extract.rs:175-180`.
- **Arms are named by ROLE in the evidence file** — `two-launch-verdict.md:6` states it explicitly ("Arms are named by **role**, never by absolute path") and the shipped table follows it.
- **A live-run finding that must survive as a test becomes a stub case** — `crates/conductor-verify/tests/common/mod.rs` (`StubConfig`/`serve_stub`) is the existing mechanism; `tests/preflight.rs` holds one snake_case test per named precondition.

## New files to create
- (none required by research) — the chunk's outputs are an append to an existing evidence file plus whatever P4 decides about a dump affordance and a key-shape pin.

## Files to modify
- `conductor-0.2.0/chunks/2026-08-10-workspace-key-divergence-probe/two-launch-verdict.md` — append the re-run's three arms (the CARRY names this file explicitly).
- `scripts/agent-run.sh` — the `boot` timeout (see Open question 1); `scripts/agent-run.ps1` — its missing counterpart. Both are the documented live entrypoint and the 5-command discipline requires identical semantics.
- `conductor-0.2.0/verification-matrix.json` — `v2-10` claim or a `notes` line.
- *Provisional, contingent on the key-diff result:* `crates/conductor-verify/src/extract.rs` + a test in `crates/conductor-verify/tests/readback.rs` (only if a live shape mismatch is found — the caller set is the two production sites at `conductor-run/src/lib.rs:301,346` plus the canary's independent readers at `preflight.rs:330,337`).

## Open questions
1. **The `boot` timeout is now shorter than the run it wraps, and the two shells disagree.** → blocks: **plan-decision**.
   `scripts/agent-run.sh:24` sets `PREFLIGHT_TIMEOUT_SEC="${CONDUCTOR_PREFLIGHT_TIMEOUT:-30}"` and `:53` wraps the CLI in `timeout "$PREFLIGHT_TIMEOUT_SEC"`. The in-process budget is now warm-up (`warmup_ms = 45000` → 45s of `sleep`) + the storm + `canary_poll` = `max(env, 90).max(1)` seconds ⇒ **≈135s minimum**. So the default kills the live leg at 30s, and following the operator recipe literally (`CONDUCTOR_PREFLIGHT_TIMEOUT=90`) still kills it at 90s — mid-poll, because the shell timer counts the warm-up the poll floor does not. `scripts/agent-run.ps1:57` has **no timeout wrapper at all**, so the same leg completes on PowerShell and is killed on bash — a live-evidence divergence between two shells the harness contract requires to be identical. Decide: raise/derive the shell budget from the contract (warm-up + floor + margin), drop the wrapper, or document an explicit export — and land the `.sh`/`.ps1` parity either way.
2. **The three arms vary `pulse-app`'s cwd, not Conductor's — and Conductor CANNOT run from arms 2 or 3.** → blocks: **plan-decision**.
   The route entry says "Re-run `conductor preflight --json` from the three cwd roles". Taken literally that is wrong twice. (a) The key under test is `pulse-app`'s `resolve_workspace_for_incidents` detected root; Conductor's cwd never reaches it (the sidecar keys on `ANDROMEDA_PULSE_DATA_DIR`). `two-launch-verdict.md:10` records the actual prior method — "`pulse-app` launched … once per arm, with a different working directory". (b) Conductor resolves `contracts/pulse-run-contract.toml`, `contracts/mcp-contract.toml`, `contracts/pulse-capabilities.toml` and `contracts/pulse-load-envelope.toml` relative to `std::env::current_dir()` (`paths.rs:27-33`, `conductor-run/src/lib.rs:144-146`, `config_path.rs:33`), so launching Conductor from a temp dir or the data dir yields a **harness `Err` at contract load** — not a measured Blocked arm. Confirm the arm variable is pulse-app's cwd and state that Conductor always runs from the repo root.
3. **There is no affordance to dump a raw read-back result, and the CARRY's key-diff requires one.** → blocks: **implementation-scope**.
   `preflight --json` serializes `ReadyState` only (`commands/preflight.rs:18-19`); `ReadyState` carries no tool payload, and `observe` composes rather than surfacing raw. `ReadbackClient::call_tool` is public (`client.rs:126`) so a mechanism is cheap, but which one — a CLI flag, a feature-gated bin, a `#[cfg(test)]`-only path, or an operator-run one-off — is unstated, and it must land inside the redaction boundary (obs: "never an unstructured stdout blob").
