# arch extract

## Relevance
partial — the chunk is a regression and CARRY sweep, not new architecture. Arch governs where each fix lands (C1 in `conductor-emit`, C3 at the `conductor-cli` edge, C4 in the `a11y` job's registered handle), what the full gate set and the live-drive firing form are, and which resources the chunk must not widen.

## Constraints
- **Crate placement and seams.** C1's fix lives in `conductor-emit` (`exception.rs`, `pii.rs`) and C3's in `conductor-cli` (`paths.rs` `find_by_pid`). Neither may add a cross-seam `Cargo.toml` edge, and `conductor-emit` stays the one member that does not depend on `conductor-core` (per architecture §Established Decisions [Module Boundaries]; §Occupied Resources → Crate names).
- **C1 must not move the fingerprint derivation.** The five rustdoc-flagged items (`FINGERPRINT_BYTES`, `NORMALIZED_FRAMES`, `normalize_stacktrace`, `MAX_FRAMES`, `PiiCategory::index`) are next to or inside the derivation that arch requires to be Pulse's own derivation, byte for byte. Its narrowing (only the first `NORMALIZED_FRAMES` lines count) and its token-leading normalization are pinned by named tests. Either repair (`pub` or de-link) must leave those semantics and pins unchanged. Arch does not choose between widening the public API and de-linking; that is P4's call (per architecture §Established Decisions [Read-Back Dependency Posture], the "Conductor's fingerprint now IS Pulse's derivation" paragraph).
- **C3 needs deterministic P-ID resolution on the correct side of the verdict/error wall.** Arch states that a P-ID may be named by several scenarios and that the accepted set is data. So an ambiguous `conductor run <P-ID>` / `SCENARIO=<P-ID>` must either resolve by a deterministic rule or be refused. It must never depend on `read_dir` order (per architecture §Design Philosophy "Determinism under a seed"; §Infrastructure Patterns → Directory structure, the `scenarios/` line; §Established Decisions [Accepted Capability Set]). The refusal is raised before any scenario exists, so it is a harness or CLI-edge error (`anyhow` at `conductor-cli`) with a host-path-free reason. It is never a `Verdict`/`ReportState` (per architecture §Established Decisions [Error Handling]; §Conventions → Error handling). Whether `preconditions --for` already has a refusal shape to reuse is a question for research.
- **C4's comment must state the mechanism the registry records.** The `EDGEWEBDRIVER` entry records that a `GITHUB_ENV`-written key does resolve through `${{ env.* }}` (run 36006370951), so the shell read is a choice. The `RUNNER_TEMP` entry records that an image-set runner variable does not resolve. The reworded `ci.yml` comment has to agree with both entries. The step keeps its shape: no `continue-on-error`, no `if:`, and the handle-named `Test-Path` precondition (per architecture §Occupied Resources → Environment variables `EDGEWEBDRIVER` / `RUNNER_TEMP`; §Established Decisions [CI/CD]).
- **The full gate set as arch defines it:**
  - `cargo fmt --all --check`, build, nextest through the zero-retry `ci` profile plus `cargo test --doc` for the doctests nextest skips, and clippy.
  - The four named `rust`-job static gates: coverage-completeness, scenario-assertion audit, secret-scan and workflow env-context.
  - The `a11y` job's routine arm with `journal_conformance` over `runs/a11y`.
  - Both supply-chain runners over an un-drifted lock, with the advisory-db checkout's porcelain read before any red is classified.
  - The scope reads "both runners" as nextest AND a plain `cargo test`. Confirming that reading is the tests domain's job (test-plan §4).
  - Per architecture §Infrastructure Patterns → Build system / CI/CD approach and §Established Decisions [CI/CD].
- **Build ordering and the standalone sweep.**
  - Any cargo gate that compiles `conductor-tauri` needs `npm run build` first (`ensure_frontend`).
  - The `--e2e` binary is built with `--features tauri/custom-protocol`, because the profile alone does not embed the bundle.
  - Standalone per-seam buildability is measured per chunk, never gate-enforced: `--lib` for the seven libraries and `--bins` for the two bins, never `--all-targets` (per architecture §Infrastructure Patterns → Build system; §Established Decisions [Module Boundaries]).
- **Live-drive firing form.**
  - The live leg runs through `scripts/agent-run.{sh,ps1}` and stays a local operator gate, never CI.
  - Its firing form must carry a `PATH` prefix that resolves the sidecar. Without it, preflight returns `[BLOCKED]` in about 0 s, which looks the same as a SUT-side gate failure.
  - `ANDROMEDA_PULSE_DATA_DIR` must reach the sidecar.
  - The readiness gate's five named preconditions are selected by posture.
  - `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` is a boot-time posture, confirmed from Pulse's own log target.
  - A real-model-posture scenario runs headless only and is recorded `Blocked` under `conductor suite`.
  - Per architecture §Standard Contracts → Readiness gate / Liveness equivalent; §Occupied Resources → Service / process names, Environment variables; §Design Philosophy "Headless-drivable core"; §Infrastructure Patterns → CI/CD approach.

## Patterns to follow
- **A gate is the Rust test, and the step only invokes it.** A new or promoted gate, such as C1's `-D warnings` doc gate, is invoked as a presence-guarded named step. It is never a re-listed shell assertion that drifts from the test (per architecture §Established Decisions [CI/CD]).
- **Seeded-fixture subject for C5.** `runs/e2e-fixture` is the `CONDUCTOR_RUNS_DIR` of both the routine `--e2e` arm and `sr-empty`. It is seeded through the production writer by `crates/conductor-run/tests/envelope_fixture.rs` under `CONDUCTOR_E2E_SEED_DIR`, which is a no-op when unset. `runs.db`'s `run_envelope` table is the run-level standing that the `[ENVIRONMENT-SUSPECT]` caption reads, and `LoadEnvelope::classify` produces it. Whether `runs/e2e-fixture/runs.db` holds a `run_envelope` row is a question for research (per architecture §Occupied Resources → On-disk artifacts: `runs.db`, `logs/conductor-tauri.jsonl`, `contracts/pulse-load-envelope.toml`; §Occupied Resources → Environment variables `CONDUCTOR_E2E_SEED_DIR`).
- **Harness-owned artifact clearing.** Clearing is non-recursive and by pattern inside a known directory: `rm -f …/*.jsonl`, as the `cleanup` verb does. The harness ships no recursive delete (per architecture §Occupied Resources → On-disk artifacts, `runs/live-suite/{leg}.jsonl`).
- **Operator instruments stay operator-local.** The Epoch 5-moved `scripts/mutation-gate.py` (with its `selftest` verb over `scripts/fixtures/mutation-gate/`) and `scripts/arch-registry-check.py` are regressed the way they are registered: invoked by no CI step and wired into neither harness shell (per architecture §Stack and Technologies → Operator instruments).

## Anti-patterns to avoid
- **Minting a resource the registry does not hold.** This covers:
  - a new `CONDUCTOR_*` handle;
  - a sixth `agent-run` command, or a new `Commands` verb, as C3's fix;
  - a new listener beyond the dev-only harness-lifetime `4444`/`4445`;
  - a new non-loopback egress;
  - a second automation stack beside the one WebdriverIO + tauri-driver stack.
  - `CONDUCTOR_NVDA` stays per-invocation and is never logged or committed.
  - Per architecture §Occupied Resources → Ports / Environment variables; §Cross-cutting Patterns → Trust boundary, Scope law.
- **Using `--all-targets` for the standalone-build regression.** It re-unifies dev-dependencies and masks the exact defect the sweep exists to catch (per architecture §Established Decisions [Module Boundaries]).
- **Classifying a red supply-chain probe as external before reading the local advisory-db checkout's currency** (per architecture §Infrastructure Patterns → Build system, the closing clause on the local host-checkout artifact).

## Contract bindings
- **arch ↔ tests:** the full gate set, the nextest `ci` profile with zero retries, and `cargo test --doc` (§Infrastructure Patterns → Build system) bind to test-plan §4's two-runner determinism rule and its tool floors. `journal_conformance` is the one schema assertion for both the run journal and `runs/a11y` (§Occupied Resources → `runs/a11y/<run_id>.jsonl`).
- **arch ↔ security:** C4's comment binds to the workflow env-context gate and to the `EDGEWEBDRIVER` registration basis (§Occupied Resources → Environment variables). C3's refusal text binds to the host-path-free edge-sanitization rule (§Established Decisions [Error Handling]). The live leg's sidecar spawn binds to the fixed-name `PATH` resolution (§Occupied Resources → Service / process names).
- **arch ↔ a11y:** C5's E0-10 banner binds to `run_envelope` and `LoadEnvelope::classify`, and to the shared `runs/e2e-fixture` landing site for `--e2e` and `sr-empty` (§Occupied Resources → On-disk artifacts).
- **arch ↔ obs:** each suite's self-obs landing site is set by the `CONDUCTOR_RUNS_DIR` it chooses (§Occupied Resources → `logs/conductor-tauri.jsonl`).

## Acceptance criteria contributions
- (arch) C3: `conductor run <P-ID>` and `SCENARIO=<P-ID>` pass or fail this check.
  - Pass: for a P-ID that several committed scenarios name, the command either resolves to the same scenario on every invocation (independent of directory-listing order) or refuses at the `conductor-cli` edge with a named reason that contains no host path.
  - Fail if the fix adds a `Commands` verb or a sixth `agent-run` command.
  - Per architecture §Established Decisions [Accepted Capability Set], [Error Handling]; §Stack and Technologies → CLI argument parsing.
- (arch) C1: after the rustdoc repair, `conductor-emit`'s fingerprint pins `exception.rs::relative_paths_are_significant_at_every_depth` and `::token_leading_absolute_paths_normalize_to_the_same_empty_form` still pass. `conductor-emit`'s `Cargo.toml` gains no dependency edge (per architecture §Established Decisions [Read-Back Dependency Posture]; [Module Boundaries]).
- (arch) C4: the reworded `ci.yml` comment names the mechanism that §Occupied Resources → Environment variables `EDGEWEBDRIVER` / `RUNNER_TEMP` records. The workflow env-context gate is green, and the asserting and pin steps still carry no `continue-on-error` and no `if:` (per architecture §Established Decisions [CI/CD]).
- (arch) Standalone per-seam build sweep: the seven library members each pass `cargo build -p <member> --lib` with exit 0, and `conductor-cli` and `conductor-tauri` each pass `--bins` with exit 0, with `--all-targets` not used (per architecture §Established Decisions [Module Boundaries]).
