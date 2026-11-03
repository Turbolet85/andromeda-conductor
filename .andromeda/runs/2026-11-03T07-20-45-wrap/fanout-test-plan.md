proposals:
  - detector: D-platform-claim
    severity: warning
    section: "§1 Test Scope Summary → Untestable zones"
    change: "Retire the stated verdict 'which runs on the Linux dev host only until `Linux-only base CI` makes CI Linux': the `#[cfg(unix)]` first-interrupt test runs on every unix leg the suite runs on, the Linux dev host and CI's `rust` job on its Linux runner (green at CI#41595128664, 2026-11-03); the zone's label narrows from 'Untestable (in CI, and in-process)' to 'Untestable (in-process)'; the second Ctrl-C's exit(130) stays unwitnessed in-process, unchanged."
    sidecar: "§1 Untestable zones: the first-interrupt test's CI half discharged; it runs on the Linux CI leg as well as the dev host (2026-11-03-linux-only-base-ci)."
    rationale: "Report Harness / gate surface: first on a Linux CI leg, the #[cfg(unix)] the_first_interrupt_sets_the_stop_flag_and_the_process_lives, all green (CI#41595128664); Counts moved: first-interrupt test CI half pending → discharged (test-plan:47 ×2, changed)."
    basis: "test-plan fct-esm7gxmw; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
  - detector: D-retired
    severity: warning
    section: "§9 CI Integration → Matrix builds"
    change: "OS bullet: today `rust` alone, on `ubuntu-latest` (the Windows runner retired 2026-11-03; the dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash`); the job count still reads 1 and no OS matrix exists; the rest of the bullet unchanged."
    sidecar: "§9 Matrix builds: the one job `rust` on `ubuntu-latest`; the Windows runner retired (2026-11-03-linux-only-base-ci)."
    rationale: "Report Retired: the Windows CI runner (runs-on: windows-latest); Harness / gate surface: the one job rust moves from windows-latest to ubuntu-latest; Counts moved: runner-os test-plan:386 changed; Expected amendments: test-plan §9 Matrix builds and the OS bullet."
    basis: "test-plan fct-m6asvzjt; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab; .github/workflows/ci.yml"
  - detector: D-retired
    severity: warning
    section: "§1 Test Scope Summary → Untestable zones"
    change: "Windows creation-flag zone: the flag VALUE test `spawn.rs::the_sidecar_spawn_suppresses_the_child_console` is `#[cfg(windows)]` and compiles out on every standing leg (the Linux dev host and the Linux CI job) since the Windows runner retired 2026-11-03, so no run witnesses the value either; the arms go with the entry «Spawned sidecar and shared-machine gate retired»; the rest of the zone unchanged."
    sidecar: "§1 Untestable zones: the Windows creation-flag value test is compiled out on every standing leg since the Windows runner retired; owner named (2026-11-03-linux-only-base-ci)."
    rationale: "Report Retired names the Windows CI runner; Harness / gate surface: no leg witnesses the #[cfg(windows)] spawn.rs arms (compiled out); Decisions & corrections (inputs#I5 row): spawn.rs's Windows arms → «Spawned sidecar and shared-machine gate retired»."
    basis: "test-plan fct-7j7mxml4; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab; doc:2026-11-03-linux-only-base-ci/report fct-h5jfta4u"
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`run` command body: the bundled default stops at its FIRST non-zero cargo line in `agent-run.sh` by `set -euo pipefail` (the 2026-10-04 measurement kept), and CI's dogfood step runs `bash scripts/agent-run.sh run` under `shell: bash` (`bash -e`), so a red line ends the step; drop 'identical in both shells', the `agent-run.ps1` `$LASTEXITCODE` clauses, the `$PSNativeCommandUseErrorActionPreference` caller clause and the ps1 green/red-path and `pwsh` host sentences (the twin retired 2026-11-03); the harness scenario leg reads `SCENARIO=<name|P-ID> [SEED=<n>] scripts/agent-run.sh run` and drops 'identical `.sh`/`.ps1` semantics'."
    sidecar: "§3 5-command implementation run: one harness, agent-run.sh; the ps1 twin's exit-check and native-error clauses retired with it (2026-11-03-linux-only-base-ci)."
    rationale: "Report Retired: the PowerShell harness twin scripts/agent-run.ps1 with its $LASTEXITCODE per-line checks, and the pwsh dogfood step with its $PSNativeCommandUseErrorActionPreference line; Counts moved: harness scripts 2 → 1 and the dogfood red path → set -euo pipefail under bash -e (key 5-command-implementation :11)."
    basis: "test-plan fct-4txlbtvf; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7; doc:2026-11-03-linux-only-base-ci/report fct-ukiudoab"
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`boot` leading arm: 'Identical semantics in `.sh` and `.ps1`' → the short-circuit semantics stated of `agent-run.sh`, the one harness; the dated 2026-09-04 / 2026-09-03 both-shell readings stay as history, marked as taken while the since-retired ps1 twin shipped."
    sidecar: "§3 5-command implementation boot: the parity clause leaves with the ps1 twin."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, key 5-command-implementation :4 changed."
    basis: "test-plan fct-vwyevac2; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`boot` command body: 'measured in the full shipped shell SET' → measured in `scripts/agent-run.sh` 2026-09-04T17:07:17→17:08:04Z (the same day's `agent-run.ps1` reading is history; that twin retired 2026-11-03); the rest unchanged."
    sidecar: "§3 5-command implementation boot: the shipped shell set is agent-run.sh alone."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, key 5-command-implementation :5 changed."
    basis: "test-plan fct-bslufraa; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "Timeout (b): the wall-clock wrapper `scripts/agent-run.sh` `boot` applies around `conductor preflight`, derived per invocation from the same contract as `warmup_ms/1000 + poll + margin`; 'both shells' and `{sh,ps1}` dropped."
    sidecar: "§3 5-command implementation boot timeout: one harness derives the wrapper."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, key 5-command-implementation :8 changed."
    basis: "test-plan fct-fuxx7zeo; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "CI stage selectors: `--live` is a sibling `case` branch of `agent-run.sh`'s `run` (drop 'both shells ship it at identical semantics'); the leg order is read from `live_leg_order` in `scripts/agent-run.sh`; the unknown selector and the retired `--e2e` token fall to `usage:` exit 2, 'in both shells' dropped at both sites."
    sidecar: "§3 5-command implementation run selectors: stated of agent-run.sh alone."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, key 5-command-implementation :15 changed."
    basis: "test-plan fct-oinj3vis; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`status` command body: `agent-run.sh` selects the newest envelope line by the envelope-only `seed` key (`jq -s` filtering on `seed`); the PowerShell `Where-Object` form leaves with the twin."
    sidecar: "§3 5-command implementation status: the ps1 envelope-line selector retired."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, key 5-command-implementation :18 changed."
    basis: "test-plan fct-oo4n5l6h; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → 5-command implementation"
    change: "`cleanup` command body: 'in the shells' / 'the shells issue no SQL' → in `agent-run.sh` / the harness script issues no SQL; the rest unchanged."
    sidecar: "§3 5-command implementation cleanup: one harness script."
    rationale: "Report Counts moved: harness scripts 2 → 1; the plural restates the two-script harness without the ps1 token."
    basis: "test-plan fct-lpebvclu; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 → Bootstrap phases (derive for route / setup-project)"
    change: "5-command-discipline-wire: wire `scripts/agent-run.sh` with the 5 commands per 5-command implementation; the two-shell parity binding with its `$PSNativeCommandUseErrorActionPreference` clause and 2026-09-07 Windows/Linux CI measurement retires with the ps1 twin (2026-11-03), one shell owing no cross-variant identity."
    sidecar: "§3 Bootstrap phases: 5-command-discipline-wire names agent-run.sh alone; the parity binding retired."
    rationale: "Report Retired: scripts/agent-run.ps1 and the pwsh dogfood step; Counts moved: ps1-twin and dogfood-pwsh, key bootstrap-phases-derive-for-route-setup-project :6 changed."
    basis: "test-plan fct-uxjgyjwg; doc:2026-11-03-linux-only-base-ci/report fct-vb7246br; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§1 Test Scope Summary → Test harness requirements"
    change: "5-command requirements `run`: 'which stops the run at the first such line in both shells' → 'which stops the run at the first such line (`agent-run.sh`, `set -euo pipefail`)'; the rest unchanged."
    sidecar: "§1 harness requirements run: one shell."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:61 changed."
    basis: "test-plan fct-jd3veh5r; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§3 Test Harness Contract"
    change: "Section note: consumed by setup-project to materialize `scripts/agent-run.sh` (not `scripts/agent-run.{sh,ps1}`)."
    sidecar: "§3 header: setup-project materializes agent-run.sh alone."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:133 changed."
    basis: "test-plan fct-iqllmv72; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§4 Unit Test Strategy"
    change: "Mutation instrument: 'it adds no sixth `agent-run.{sh,ps1}` command' → 'it adds no sixth `agent-run.sh` command'; the rest unchanged."
    sidecar: "§4 mutation instrument: harness named as agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:145 changed."
    basis: "test-plan fct-pzb6nngw; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Headless deterministic scenario run with MCP read-back"
    change: "Step 1: reached through `scripts/agent-run.sh boot` (not `scripts/agent-run.{sh,ps1} boot`)."
    sidecar: "§6 error-baseline-spike steps: boot through agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:233 changed."
    basis: "test-plan fct-fikilkyv; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Fingerprint-storm"
    change: "Steps: preflight via `scripts/agent-run.sh boot` (not `scripts/agent-run.{sh,ps1} boot`)."
    sidecar: "§6 fingerprint-storm steps: boot through agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:248 changed."
    basis: "test-plan fct-btwjiwiz; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Restart-suppression incl. one bypass case"
    change: "Step 1: preflight via `scripts/agent-run.sh boot` (not `scripts/agent-run.{sh,ps1} boot`)."
    sidecar: "§6 restart-suppression steps: boot through agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:256 changed."
    basis: "test-plan fct-php6ab3n; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Severity-lifecycle full pass"
    change: "Steps: preflight via `scripts/agent-run.sh boot`, then `SCENARIO=<name> scripts/agent-run.sh run` for each of the five family scenarios (both `{sh,ps1}` forms dropped)."
    sidecar: "§6 severity-lifecycle steps: boot and run through agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:263 changed."
    basis: "test-plan fct-cmep33bg; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§6 E2E Test Strategy → Scenario: Known-residual classification (P-032)"
    change: "Step 1: preflight via `scripts/agent-run.sh boot` (not `scripts/agent-run.{sh,ps1} boot`)."
    sidecar: "§6 P-032 steps: boot through agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:270 changed."
    basis: "test-plan fct-imnnp6ty; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§9 CI Integration → Pipeline structure"
    change: "Unit stage row: `agent-run.sh run --unit` → `cargo nextest run --workspace --profile ci` (not `agent-run.{sh,ps1}`)."
    sidecar: "§9 stage table Unit row: agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:373 changed."
    basis: "test-plan fct-ep2ez4nz; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§9 CI Integration → Pipeline structure"
    change: "Integration stage row: `agent-run.sh run --integration` → the hand-rolled MCP stub + in-memory/`TempDir` `runs.db` (not `agent-run.{sh,ps1}`)."
    sidecar: "§9 stage table Integration row: agent-run.sh."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:375 changed."
    basis: "test-plan fct-cfw4k5hz; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§9 CI Integration → Live-Pulse scenarios"
    change: "'both owed lines run in `agent-run run`'s bundled default in both shells' → 'both owed lines run in `agent-run.sh run`'s bundled default'; the rest unchanged."
    sidecar: "§9 Live-Pulse: the feature-gated lint lines run in agent-run.sh's bundled default."
    rationale: "Report Retired: scripts/agent-run.ps1; Counts moved: ps1-twin, test-plan:379 changed."
    basis: "test-plan fct-yroyo3j2; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7"
    dependent-of: D-retired
  - detector: D-tests-derived-count
    severity: warning
    section: "§8 Mocking & Stubbing Discipline → Mocking libraries"
    change: "Time-mocking row: 'tokio 1.52.3 `test-util`' → 'tokio `test-util` (the `Cargo.lock`-resolved version, per §4 Tool-version policy)'; the patch literal dropped rather than replaced, so a lock bump cannot re-stale it."
    sidecar: "§8 Time-mocking row: tokio named without a patch literal (the lock moved 1.52.3 → 1.54.1, RUSTSEC-2026-0386)."
    rationale: "Report Dependencies: tokio 1.52.3 → 1.54.1 (lock-only, RUSTSEC-2026-0386), pattern tokio naming test-plan :351 time-mocking row changed; §4 Tool-version policy already holds crate exact versions as Cargo.lock-authoritative."
    basis: "test-plan fct-odwlbdq3; test-plan fct-jhtwefv4; doc:2026-11-03-linux-only-base-ci/report fct-w4e767kf"
  - detector: D-tests-derived-count
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope"
    change: "conductor-timeline entity: the arch Stack quote drops the 'tokio 1.52.3' literal (the lock now resolves 1.54.1), quoting arch §Stack's amended row as it words tokio's `current_thread` flavor without a patch literal; the rest unchanged."
    sidecar: "§1 Coverage scope conductor-timeline: tokio quoted without a stale patch literal."
    rationale: "Report Dependencies: tokio 1.52.3 → 1.54.1; pattern tokio names test-plan:30 changed."
    basis: "test-plan fct-efo3vjjh; doc:2026-11-03-linux-only-base-ci/report fct-w4e767kf"
    dependent-of: D-tests-derived-count
  - detector: D-tests-derived-count
    severity: warning
    section: "§7 Test Data & Fixtures → Self-bootstrapping requirement"
    change: "The real-model harvest reads committed live-leg capture EVIDENCE under `crates/conductor-run/tests/evidence/<marker>/` (one folder per source chunk, byte-equal copies of that chunk's captures, file names and sha256 pins unchanged; moved 2026-11-03 under the founder's ratified Data Protection exception, inputs#I3), read-only, `CARGO_MANIFEST_DIR`-anchored, each file checked against its digest pin before any grade reads it; 'a capture is evidence, never a fixture copied elsewhere' → 'a capture is evidence held byte-equal under its pin, never a fixture: never edited, never graded unpinned'."
    sidecar: "§7 Self-bootstrapping: capture evidence locus → crates/conductor-run/tests/evidence/<marker>/, byte-equal copies under unchanged pins (I3)."
    rationale: "Report Files: crates/conductor-run/tests/evidence/ NEW, 46 files in 11 per-source-chunk folders, byte-equal copies; the prefix conductor-0.3.0/chunks/<marker>/evidence becomes crates/conductor-run/tests/evidence/<marker>; Counts moved: capture-locus test-plan:317 changed; inputs#I3: test-plan §7 amended to match."
    basis: "test-plan fct-goccvdfd; doc:2026-11-03-linux-only-base-ci/report fct-ysekx3t4; doc:2026-11-03-linux-only-base-ci/report fct-v2c3a5a7; doc:2026-11-03-linux-only-base-ci/report fct-uh2y4bht"
note: "D-tests-obs-harness: the ps1 twin's retirement edits test-plan §3 (above) and obs-plan §9/§10 (report obs-plan:472 ×2), so apply both sides together; D-tests-coverage and D-tests-framework found no drift (test-only symbols each with a can-fail arm, runner unchanged, 1218/1218); §10's citation-home prose names no citation_home value; the dated chunk-evidence citations the report marks no-change are left alone."
