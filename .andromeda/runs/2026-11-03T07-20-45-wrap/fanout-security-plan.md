proposals:
  - detector: D-platform-claim
    severity: warning
    section: "§Security Anti-Patterns → Code Patterns"
    change: "Rule (a): the console-suppression flag VALUE was unit-held by the `#[cfg(windows)]` test `spawn.rs::the_sidecar_spawn_suppresses_the_child_console` only while the Windows CI runner ran it (retired 2026-11-03, Linux-only base CI); on the measured platform set (the Linux dev host and the `ubuntu-latest` CI leg) the `#[cfg(windows)]` arm and its test compile out, so no leg now witnesses the flag value or its application; the arm's disposition rides the entry «Spawned sidecar and shared-machine gate retired». The 'held by a `#[cfg(windows)]` unit test' clause takes the same past tense; the dated 2026-09-04 NVDA measurement stays as history."
    sidecar: "Code Patterns rule (a): the cfg(windows) console-suppression test is no longer witnessed on any leg after the Windows CI runner retired; the measured set is the Linux dev host + ubuntu-latest CI (2026-11-03-linux-only-base-ci)."
    rationale: "The stating sentence: 'The applied flag is not assertable at the unit tier — `std::process::Command` exposes no creation-flags getter — so the flag VALUE is unit-held and its APPLICATION was evidenced by the operator-local SR leg until that leg retired with the window'. The report's Harness / gate surface retires the Windows runner (the one job `rust` moves to `ubuntu-latest`) and states 'No leg witnesses the `#[cfg(windows)]` `spawn.rs` arms (compiled out)'; the P5 card routes those arms to «Spawned sidecar and shared-machine gate retired» (inputs#I5). A capability claim (the value is held by a running test), not a CI arrangement."
    basis: "security-plan fct-sbjbvw42; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab; doc:2026-11-03-linux-only-base-ci/report fct-h5jfta4u"
  - detector: D-retired
    severity: warning
    section: "§Input Validation"
    change: "Committed SUT-facing manifests row: `pulse-run-contract.toml`'s reader outside `conductor-core` is `scripts/agent-run.sh` alone, which parses `warmup_ms` + `min_canary_poll_seconds` to derive the `boot` wall-clock budget under the same never-defaulted rule (missing term = hard exit 2; a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` clamps up to the contract floor). Drop the `scripts/agent-run.{sh,ps1}` pair."
    sidecar: "Input Validation manifests row: the run-contract's harness reader is agent-run.sh alone; the PowerShell twin agent-run.ps1 retired (2026-11-03-linux-only-base-ci)."
    rationale: "The report's Retired bullet names the PowerShell harness twin `scripts/agent-run.ps1`; Counts moves harness scripts 2 → 1 (`agent-run.{sh,ps1}` → `agent-run.sh` alone); Expected amendments lists this row's harness-scripts text."
    basis: "security-plan fct-orv7vnhf; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7; doc:2026-11-03-linux-only-base-ci/report fct-qwvszocq"
  - detector: D-retired
    severity: warning
    section: "§Input Validation"
    change: "CLI arguments / stdin row: the harness's `run --live [real-model]` selector is a closed `case` allowlist in `scripts/agent-run.sh` (drop 'the `.ps1` binds the token by position count'); the `run_id` positional is rejected upstream by the harness's `valid_run_id` before the spawn (drop `Test-RunId` and 'the harness shells' plural)."
    sidecar: "Input Validation CLI-arguments row: the selector allowlist and the run_id rejection are agent-run.sh's (valid_run_id) alone; the .ps1 position binding and Test-RunId retired with the twin (2026-11-03-linux-only-base-ci)."
    rationale: "The Retired bullet names `scripts/agent-run.ps1` with `Test-RunId`; Counts records rule (b) form 5 `valid_run_id` / `Test-RunId` → `valid_run_id` alone; the Harness bullet states `agent-run.sh` is the one harness with `valid_run_id` ahead of `conductor cleanup` and the closed `--live [real-model]` refusal."
    basis: "security-plan fct-fy2a4ady; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Security Anti-Patterns → Code Patterns"
    change: "Rule (b): any HARNESS spawn on the ROUTINE `scripts/agent-run.sh` path; the TEARDOWN form reads 'the harness invokes the repo-built `conductor` binary … admitted only after its own run-id rejection (`valid_run_id`)' — drop `{sh,ps1}`, 'the shells' / 'each shell's' and `Test-RunId`. The retired forms' `send-keys.ps1` / `a11y-limited-token-launch.ps1` history stays."
    sidecar: "Code Patterns rule (b): the governed teardown form is agent-run.sh's alone, guarded by valid_run_id; Test-RunId retired with agent-run.ps1 (2026-11-03-linux-only-base-ci)."
    rationale: "The Retired bullet names `scripts/agent-run.ps1` (with `Test-RunId`); Counts records rule (b) form 5 `valid_run_id` / `Test-RunId` → `valid_run_id` alone; Expected amendments names Code Patterns rule (b)."
    basis: "security-plan fct-sbjbvw42; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-qwvszocq"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Security Anti-Patterns → Universal"
    change: "The preflight-scoping bullet's opening reads '`scripts/agent-run.sh` runs `conductor preconditions` as a leading arm and short-circuits on non-zero on the path SET `{boot, run --live}`'; the dated 2026-09-04 measurement in BOTH harness shells (bash · pwsh) stays as history."
    sidecar: "Universal preflight-scoping bullet: the preconditions leading arm is agent-run.sh's alone after the PowerShell twin retired (2026-11-03-linux-only-base-ci)."
    rationale: "The Retired bullet names the PowerShell harness twin `scripts/agent-run.ps1`; the Harness bullet makes `scripts/agent-run.sh` the one harness; this bullet states the `{sh,ps1}` pair as the standing invoker of the preflight."
    basis: "security-plan fct-gympmcsi; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Input Validation"
    change: "Real-model capture ingest row: the harvest walks every committed `rm-capture*.txt` under `crates/conductor-run/tests/evidence/*/` (one folder per source chunk, byte-equal copies, sha256 pins unchanged), workspace-root anchored from `CARGO_MANIFEST_DIR` with no handle; the population still pinned by count (26) beside an inverse control."
    sidecar: "Input Validation capture-ingest row: the harvest's committed-capture locus moved from conductor-0.3.0/chunks/*/evidence/ to crates/conductor-run/tests/evidence/*/ (2026-11-03-linux-only-base-ci)."
    rationale: "The report's Files bullet removes `conductor-0.3.0/chunks/` (46) in git and adds `crates/conductor-run/tests/evidence/` (46 byte-equal copies); `capture_population.rs`'s walk reads `crates/conductor-run/tests/evidence/*/rm-capture*.txt`, `COMMITTED_CAPTURES` stays 26; the Retired bullet names the chunk-file renders; Counts lists this site under capture-locus."
    basis: "security-plan fct-uxxrxviq; doc:2026-11-03-linux-only-base-ci/report fct-ysekx3t4; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7; crates/conductor-run/tests/real_model_grading/capture_population.rs"
  - detector: D-retired
    severity: warning
    section: "§Security Anti-Patterns → Data Protection"
    change: "The ratified exception: scrubbed corpus-rendered model text may enter a chunk's committed `evidence/` tree and its committed copy under `crates/conductor-run/tests/evidence/<marker>/` (extension ratified by the founder, 2026-11-03), only through the same scrub chain; the graded 2026-09-23 capture is an elided copy at `crates/conductor-run/tests/evidence/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/rm-capture-2026-09-22-elided.txt`. 'Outside the `evidence/` tree this exception names' names both loci."
    sidecar: "Data Protection exception: extended to crates/conductor-run/tests/evidence/ on the founder's word (inputs#I3); the elided 2026-09-23 copy's locus re-pointed off the retired conductor-0.3.0/chunks/ (2026-11-03-linux-only-base-ci)."
    rationale: "inputs#I3: the founder ratifies the capture move — the Data Protection exception extends to `crates/conductor-run/tests/evidence/`; the Files bullet removes `conductor-0.3.0/chunks/` (46) and adds the relocated byte-equal copies; Expected amendments names this exception and Counts lists both its locus phrases."
    basis: "security-plan fct-ppdqreyb; doc:2026-11-03-linux-only-base-ci/report fct-uh2y4bht; doc:2026-11-03-linux-only-base-ci/report fct-ysekx3t4; doc:2026-11-03-linux-only-base-ci/report fct-qwvszocq"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Security Anti-Patterns → Logging"
    change: "The host-path ban's committed-evidence subject reads 'committed evidence records (a chunk's `evidence/` tree and the committed capture copies under `crates/conductor-run/tests/evidence/`)', so the relocated committed captures stay inside the ban."
    sidecar: "Logging host-path ban: committed-evidence scope names the relocated crates/conductor-run/tests/evidence/ tree beside the per-chunk evidence/ tree (2026-11-03-linux-only-base-ci)."
    rationale: "The committed capture evidence now lives at `crates/conductor-run/tests/evidence/` (Files bullet; inputs#I3), while this ban names only 'the per-chunk `evidence/` tree' as the committed-evidence locus — the same locus claim the Data Protection proposal re-words."
    basis: "security-plan fct-okeb3fjj; doc:2026-11-03-linux-only-base-ci/report fct-ysekx3t4; doc:2026-11-03-linux-only-base-ci/report fct-uh2y4bht"
    dependent-of: D-retired
note: "D-security-deps clear: tokio 1.52.3 → 1.54.1 is lock-only (RUSTSEC-2026-0386), cargo audit + cargo deny green at CI#41595128664, and security-plan states no tokio version. D-security-input and D-security-subprocess clear: no new input surface; sidecar spawn, preflight and data-dir untouched. Left as dated history: fct-gympmcsi's 2026-09-04 bash+pwsh measurement, fct-sbjbvw42's send-keys.ps1 / a11y-limited-token-launch.ps1 retirements, fct-eorfa32l's %APPDATA% Pulse default (a true claim, per the report)."
