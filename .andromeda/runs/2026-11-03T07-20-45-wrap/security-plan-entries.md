## 2026-11-03-linux-only-base-ci — the harness is agent-run.sh alone
**Section:** §Input Validation (the committed SUT-facing manifests row; the CLI arguments / stdin row) · §Security Anti-Patterns → Code Patterns (rule (b), harness spawns) · §Security Anti-Patterns → Universal (the preflight-scoping bullet)
**Change:**
- Manifests row: `pulse-run-contract.toml` has one reader outside `conductor-core`, `scripts/agent-run.sh`, which parses `warmup_ms` + `min_canary_poll_seconds` to derive the `boot` wall-clock budget under the same never-defaulted rule (was `scripts/agent-run.{sh,ps1}`, "readers").
- CLI arguments row: the `run --live [real-model]` selector is a closed `case` allowlist in `scripts/agent-run.sh` (was "the `.ps1` binds the token by position count"); the harness rejects the `run_id` positional upstream with `valid_run_id` before the spawn (was "the harness shells", `valid_run_id` / `Test-RunId`).
- Code Patterns rule (b): the ROUTINE path is `scripts/agent-run.sh`; its one governed form, the TEARDOWN spawn `conductor cleanup <run_id>`, is invoked by the harness and admitted only after its own `valid_run_id` rejection (was `agent-run.{sh,ps1}`, "the shells", "each shell's own run-id rejection (`valid_run_id` / `Test-RunId`)").
- Universal: `scripts/agent-run.sh` runs `conductor preconditions` as the leading arm on the path set `{boot, run --live}` (was `scripts/agent-run.{sh,ps1}`).
**Why:** the chunk retires the PowerShell harness twin `scripts/agent-run.ps1` with its `Test-RunId`; `scripts/agent-run.sh` is the one harness, byte-unchanged since `8e450f3`, keeping its five verbs, `valid_run_id` ahead of `conductor cleanup` and the closed `--live [real-model]` refusal.
**Kept:** the Universal bullet's dated 2026-09-04 measurement in both harness shells (bash · pwsh) and rule (b)'s retired `send-keys.ps1` / `a11y-limited-token-launch.ps1` forms — dated history and retired files, true as written; the `.ps1` token there is not the twin.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — the committed captures' new locus, inside the Data Protection exception
**Section:** §Input Validation (the real-model capture ingest row) · §Security Anti-Patterns → Data Protection (the ratified corpus-text exception) · §Security Anti-Patterns → Logging (the host-path ban)
**Change:**
- Capture ingest row: the harvest walks every committed `rm-capture*.txt` under `crates/conductor-run/tests/evidence/*/` (one folder per source chunk), workspace-root anchored from `CARGO_MANIFEST_DIR` with no handle (was under `conductor-0.3.0/chunks/*/evidence/`).
- Data Protection: scrubbed corpus-rendered model text may enter a chunk's committed `evidence/` tree and its committed copy under `crates/conductor-run/tests/evidence/{marker}/`, only through `mask_workspace_key` + `redact_value` + `mask_host_paths` + `elide_fingerprints` (was the `evidence/` tree alone); the remedied 2026-09-29 breach now reads "outside both `evidence/` trees"; the graded 2026-09-23 capture is the elided copy at `crates/conductor-run/tests/evidence/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/rm-capture-2026-09-22-elided.txt` (was under `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/`).
- Logging: the ban's committed evidence records are a chunk's `evidence/` tree and the committed capture copies under `crates/conductor-run/tests/evidence/` (was the per-chunk `evidence/` tree).
**Why:** the chunk moves the committed evidence to 46 byte-equal copies in 11 per-source-chunk folders under `crates/conductor-run/tests/evidence/`, file names and sha256 pins unchanged and the walk still counting 26 captures, and removes `conductor-0.3.0/chunks/`. The exception's extension is the founder's own word, 2026-11-03, by dialog in the pc overseer's session, relayed verbatim (inputs#I3); the overseer, founder-delegated (PROVISIONAL), confirmed at this wrap that this amendment lies inside it. The Logging scope follows the relocated captures — a narrowing of what escapes the ban, never a widening.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/

## 2026-11-03-linux-only-base-ci — no measured leg witnesses the sidecar's console-suppression flag
**Section:** §Security Anti-Patterns → Code Patterns (rule (a), the MCP sidecar spawn)
**Change:**
- `build_command`'s `#[cfg(windows)]` creation flag stands "with its `#[cfg(windows)]` unit test (`spawn.rs::the_sidecar_spawn_suppresses_the_child_console`)" (was "held by" it).
- The flag VALUE is unit-testable only on a Windows build (was "unit-held"); its APPLICATION was evidenced by the operator-local SR leg until that leg retired (2026-10-11).
- Added: no measured leg — the Linux dev host and the `ubuntu-latest` CI job — compiles the `#[cfg(windows)]` arm or its test, so none witnesses the flag value or its application; the arm's disposition is owned by the route entry «Spawned sidecar and shared-machine gate retired».
**Why:** the chunk retires the Windows CI runner — the one job `rust` runs on `ubuntu-latest` — and no leg witnesses the `#[cfg(windows)]` `spawn.rs` arms, compiled out. The owner is the P5 card's word, the overseer's, founder-delegated, PROVISIONAL (inputs#I5).
**Kept:** the 2026-09-04 NVDA measurement and its supersession of the 2026-09-02 reading, dated history.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
