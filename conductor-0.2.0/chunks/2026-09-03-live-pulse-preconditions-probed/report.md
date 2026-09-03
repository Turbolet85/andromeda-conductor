# Report — 2026-09-03-live-pulse-preconditions-probed

**Chunk:** Live-Pulse preconditions probed before a leg is scheduled — a non-mutating probe over `:4317` reachability, `andromeda-pulse-mcp` on the inherited `PATH` and the three `ANDROMEDA_PULSE_*` handles, reporting a stated precondition at scheduling time instead of each chunk absorbing the SUT's absence as its own deferral
**Date:** 2026-09-03
**Commits:** none since `last_wrap` (`a176696` is the prior chunk's); this chunk's work is uncommitted and rides this wrap

## Changes (structured — detectors read this)

- **Files:**
  - New: `crates/conductor-core/src/preconditions.rs` (383) · `crates/conductor-cli/src/commands/preconditions.rs` (38) · `crates/conductor-run/tests/run_contract_pin.rs` (104)
  - Modified: `crates/conductor-core/src/lib.rs` · `crates/conductor-core/src/run_contract.rs` · `crates/conductor-verify/src/lib.rs` · `crates/conductor-verify/src/spawn.rs` · `crates/conductor-run/src/lib.rs` · `crates/conductor-cli/src/cli.rs` · `crates/conductor-cli/src/main.rs` · `crates/conductor-cli/src/commands/mod.rs` · `crates/conductor-cli/src/render.rs` · `crates/conductor-cli/tests/cli_smoke.rs` · `contracts/pulse-run-contract.toml` · `scripts/agent-run.sh` · `scripts/agent-run.ps1`

- **Symbols / APIs:**
  - **New CLI verb** `conductor preconditions [--json]` — `Commands::Preconditions { json }` (`cli.rs`), dispatch arm (`main.rs`), handler `commands::preconditions` (`commands/preconditions.rs`, registered in `commands/mod.rs`). Exits **0 iff every subject is satisfied**, non-zero otherwise — the documented `conductor preflight` go/no-go exception, NOT the scenario-verdict rule; it deliberately does not use `commands/mod.rs`'s shared `exit_code(&[RunRecord])`.
  - **New public core surface** (`conductor-core`, exported from `lib.rs`): `Preconditions::evaluate(&PreconditionObservation) -> PreconditionsStatus` · `PreconditionObservation` · `PreconditionsStatus` · `PreconditionSubject` · `UnmetPrecondition` · `const OBSERVED_HANDLES: [&str; 3]`. `evaluate` is PURE — no env read, no IO, no socket; every input arrives as a value (the `RunContract::evaluate` shape, per testing.md's env-at-the-caller rule).
  - **New public fn** `conductor_verify::sidecar_resolves_on_path() -> bool` (`spawn.rs`) — resolves the fixed program name over `std::env::split_paths(PATH)` + `PATHEXT`; **never spawns**, returns a boolean-grade fact and never the resolved path. `PULSE_MCP_PROGRAM` stays `pub(crate)` (unchanged visibility). Its pure half `resolves_on(path, path_ext, program)` is crate-private.
  - **New public async fn** `conductor_run::observe_preconditions() -> PreconditionsStatus` — the observation site; reuses `probe_egress(DEFAULT_OTLP_ENDPOINT)` and the existing crate-private `declares()`. **Not routed through `canary_gate`**, so it fires no canary storm.
  - **New render fn** `conductor_cli::render::preconditions_caption(&PreconditionsStatus) -> Option<String>` + crate-private `preconditions_caption_styled`; new consts `PRECONDITION_MUTE = 246` and `PRECONDITION_LABEL = "PRECONDITION"`.
  - **Remaining-caller facts:** `probe_egress` gains a SECOND production caller (`observe_preconditions`); its existing caller `execute_scenario` (`conductor-run/src/lib.rs`) is unchanged — no signature change, so no caller threading was owed. `declares()` gains a second caller beside `observe_run_contract`; `observe_run_contract` and its sole production caller `canary_gate` are unchanged. `RunContract::{evaluate, observed_env}` unchanged in signature; `observed_env()` now RETURNS two names instead of one (a data change, not an API change).
  - **Ports / sockets:** none opened. `:4317` is contacted as a **client only** (connect-and-drop via the shipped `probe_egress`); no bind, no listener, `:4318` untouched.
  - **Env vars:** none added. The probe READS three existing Pulse-side handles (`ANDROMEDA_PULSE_DATA_DIR` · `ANDROMEDA_PULSE_L4_DETERMINISTIC` · `ANDROMEDA_PULSE_MCP_ENABLED`) and writes none. No new `CONDUCTOR_*` handle.

- **Crates / modules:** none added or removed. Changed: `conductor-core` (new `preconditions` module) · `conductor-verify` · `conductor-run` · `conductor-cli`. No new cross-crate dependency edge — `conductor-run` already depended on core, emit and verify.

- **Dependencies:** **none added, none bumped.** `Cargo.lock` byte-unchanged; package count **564 → 564**. The `which` crate was explicitly rejected (~15 lines of `std::env::split_paths` answers it) to avoid a dependency delta under the standing red-audit deferral.

- **Schema / config:** `contracts/pulse-run-contract.toml` gains ONE `[[term]]` — `id = "mcp-enabled"`, `check = "shell-declaration"`, `env = "ANDROMEDA_PULSE_MCP_ENABLED"`, with a `causes` string. Same class as the shipped `l4-deterministic`; needs no new machinery (`evaluate`/`observed_env` already filter on `CheckKind::ShellDeclaration`). No migration. No violation-schema or scrub-shape change.

- **Spec-master edits:** none — this chunk edited no `.andromeda/` master; the three Expected amendments below are for this wrap's P2 flow.

- **Counts / qualifiers moved:**
  - **CLI verb surface 5 → 6** (`Run`/`Suite`/`Report`/`Preflight`/`Coverage` + `Preconditions`). Note `architecture.md:33` and `layout-templates.md:251`/`:309` state a **three**-verb literal (`conductor run|suite|report`) — already stale before this chunk, now further behind.
  - **Run-contract `[[term]]` count 5 → 6**; **observable (`shell-declaration`) env set 1 → 2** (`ANDROMEDA_PULSE_L4_DETERMINISTIC` + `ANDROMEDA_PULSE_MCP_ENABLED`).
  - **cli run-level non-lamp bracket labels 1 → 2** — `[ENVIRONMENT-SUSPECT]` joined by `[PRECONDITION]`. `design-system.md:311` enumerates the closed per-P-ID six PLUS the non-lamp caption by name; that enumeration now omits one.
  - **Residual-mute (ANSI 246) non-lamp reuse 3rd → 4th.** Not a stale literal: design-system deliberately names these as a SET (`2026-08-09-sut-load-envelope` amendment), so no count needs substituting.
  - **Harness command count UNCHANGED at 5** (`boot`/`run`/`status`/`cleanup`/`logs`) — `boot`'s composition changed, not the set.

- **Dev-tool versions:** none installed or upgraded.

- **Harness / gate surface:** `scripts/agent-run.sh` and `scripts/agent-run.ps1` — `boot` gains a **leading probe arm** that runs `conductor preconditions` first and short-circuits on non-zero, **skipping the preflight invocation rather than paying it** (the arch ordering rationale for the fifth named precondition, applied one rung earlier). Identical semantics in both shells; no 6th command, so no test-plan amendment is owed for the command count. Verdict/status shapes unchanged: the probe mints no `Verdict`, no `ReportState`, and no per-P-ID row.

- **Cross-project / external claims:** the `mcp-enabled` term's `causes` and the probe's sidecar `causes` string name Pulse's own build command (`cargo build -p mcp-server --bin andromeda-pulse-mcp --features mcp-server`) and the inheritance channel. Basis: transcribed from `security-plan.md` §Anti-Patterns and `verification-harness.md`'s accumulated recipe — **not measured against the Pulse repo this chunk**. No Pulse-side file was read or changed.

- **Reverted / negative API facts:** none.

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **`anstream` is named as the cli TTY-gating mechanism and is absent.** Stated at `design-system.md:284`/`:288`/`:331` and `layout-templates.md:170`. Measured: `anstream` appears in neither `Cargo.toml` nor `crates/conductor-cli/Cargo.toml` nor anywhere in `crates/conductor-cli/src/`; the shipped gate is `owo-colors` + `std::io::IsTerminal`, two independent decisions at `render.rs:133` (stdout) and `:141` (stderr), each requiring a terminal AND `NO_COLOR` unset AND `TERM != dumb`. `architecture.md`'s stack table already records owo-colors correctly, so one master is right and two are wrong. The mandated BEHAVIOUR is fully shipped — only the named mechanism diverges.
  2. **`contracts/pulse-run-contract.toml`'s `sidecar-built` term is `check = "asserted"` on a rationale measurement contradicts.** Its `causes` reads "an unbuilt sidecar fails connect and surfaces the read-back-unreachable precondition upstream of this gate, so reaching term evaluation at all proves it" — but a `PATH` miss short-circuits to `[BLOCKED]` in ~0s *upstream* of term evaluation (`verification-harness.md:54`, measured 2026-08-20), so the term is satisfied-by-construction exactly when it is false. Same failure shape arch already records for `warmup_ms`. Deliberately NOT re-classified this chunk (operator-selected at P4): re-classifying moves what the preflight gate can block on.

- **Expected amendments (from plan):**
  1. **arch §Occupied Resources — register the new verb + the `boot` leading arm** → carried; its fact is in *Symbols / APIs* (the verb) and *Harness / gate surface* (the arm).
  2. **arch §Occupied Resources — `pulse-run-contract.toml` gains `mcp-enabled`; `sidecar-built`'s `asserted` rationale recorded as measurably false** → carried; facts in *Schema / config* and *Spec claims disproved* #2.
  3. **test-plan §3 — record `boot`'s new leading arm in the 5-command discipline (COUNT unchanged; only `boot`'s composition moves)** → carried; fact in *Harness / gate surface* and *Counts moved*.
  4. **design-system §Surface: cli + layout-templates §Surface: cli — the `anstream` divergence** → carried; fact in *Spec claims disproved* #1.

- **Coverage of new surfaces:**
  - `conductor preconditions [--json]` (cli verb) → validation n/a (no operator-supplied input; the verb takes only a bool flag) · instrumentation ✓ (one `info!`/`warn!` on the allowlisted `message` field inside the caller's span, **no new span name** — the bounded set is unwidened per the 2026-09-01 ruling) · PII redacted✓ (negative test asserts no absolute host path, no `PATH` contents, no struct name, in both the rendered and JSON forms; drive-letter token word-anchored so the probe's own `127.0.0.1:4317` cannot false-positive) · tests unit+integ ✓ (`preconditions.rs` `#[cfg(test)]` 8 rows incl. rstest per-subject matrix; `cli_smoke.rs` 3 edge tests) · a11y n/a (cli surface is declared not-assertable, a11y-plan §1) · tokens ✓ (`[PRECONDITION]` reuses the existing Residual-mute ANSI 246; zero new palette rows, no seventh lamp, no sixth `ReportState`)
  - `conductor_verify::sidecar_resolves_on_path` (PATH read) → validation n/a (reads the process `PATH`, supplies no operator value anywhere; the fixed program NAME is a compile-time const) · instrumentation n/a (a pure predicate; its outcome is logged by the caller) · PII redacted✓ (returns `bool`; the resolved path is never returned, logged or rendered) · tests unit ✓ (8 rows: absent PATH, empty PATH, dir-without, present, later-entry, directory-named-like-the-program, different-name, PATHEXT normalisation) · a11y n/a · tokens n/a
  - `conductor_run::observe_preconditions` (egress connect + env reads) → validation n/a · instrumentation ✓ (one boundary line, no new span) · PII redacted✓ (no field carries a path or an env VALUE — only handle NAMES) · tests integ ✓ (exercised end-to-end through the CLI edge tests; the pure evaluator carries the arm matrix) · a11y n/a · tokens n/a
  - `agent-run boot` leading arm (harness) → validation n/a · instrumentation ✓ (inherits the probe's line) · PII redacted✓ (the short-circuit message names no path) · tests ✓ (driven live in both shells this session) · a11y n/a · tokens ✓ (ASCII label always rendered)

## Deviations from intent

1. **The CLI-edge test sets no `CONDUCTOR_RUNS_DIR`**, where plan step 10 prescribed one → `assert_fs::TempDir`. **Justification:** that handle is repo-relative by design and `resolve_under` rejects an absolute value *before* the verb dispatches, so the probe printed nothing and both new tests failed on empty stdout; the probe writes no artifact and needs no runs dir. The prescription was a generic CLI-edge pattern inherited from the tests extract that does not fit a verb writing nothing.
2. **`crates/conductor-core/src/run_contract.rs`'s `loads_and_bounds_checks_the_committed_contract` was updated** to the new two-handle observable set and to assert the `mcp-enabled` term. **Justification:** the contract gaining that term is the chunk's own operator-ratified design (P4 fork 1), the assertion pinned the pre-change value, and `run_contract.rs` is in the plan's Files-to-modify.
3. **Probe artifacts redirect under `target/`** rather than the repo root. **Justification:** operator review note — `git check-ignore` returns nothing for `probe-out.json`/`probe-err.jsonl` at the root, so `git add -A` would commit them; `.gitignore:11` covers `/target/`.

## Decisions & corrections

- **Operator P4 fork 1 — contract scope.** Selected *split*: the probe owns subjects 1+2; the contract gains exactly ONE `shell-declaration` term (`mcp-enabled`); `sidecar-built`'s false `asserted` rationale is **surfaced, not silently re-classified**, because re-classifying moves what the preflight gate can block on. The rejected alternative (a new `CheckKind` unifying all three into the contract) is recorded in the plan's rejected-approaches.
- **Operator P4 fork 2 — harness wiring.** Selected *both*: a standalone verb AND `boot`'s leading arm, keeping the command count at five so no test-plan amendment is required.
- **Operator review, three corrections applied before approval.** (a) The standing `cargo audit`/`cargo deny` PREREQ was absent from Test Commands — added with its disposition plus two security criteria, because wrap's light gate re-runs Test Commands only and could not otherwise reproduce the signature. (b) Probe redirects moved under `target/`. (c) The negative test's drive-letter token **word-anchored** (`\b[A-Za-z]:[\\/]`) — the unanchored form matches the `p:/` inside the probe's own `http://127.0.0.1:4317` payload.
- **Correction to my own earlier claim:** I reported the `Commands` enum as carrying four verbs from a `head`-truncated grep; it carries **five** (`Coverage` exists). The layouts distiller flagged the discrepancy and research resolved it against the source.
- **Wrap directive — two surfaced findings assigned owners at P5** (see Outcome).

## Outcome

**Acceptance criteria — re-asserted against the diff:**

| Criterion | Verdict against the diff |
|---|---|
| (arch) binds no port, emits no OTLP, `:4318` untouched, no inbound listener | **MET** — only `probe_egress` (connect-and-drop) is called; no `bind` anywhere in the diff |
| (arch) no `Verdict`/`ReportState`/`Blocked` row; unanswerable subject is a harness fact | **MET** — `PreconditionsStatus` is its own type; the verb bypasses the `RunRecord` exit helper |
| (arch) no env handle outside `CONDUCTOR_*`; the `ANDROMEDA_PULSE_*` trio read never written | **MET** — diff adds no env handle; `declares()` is read-only |
| (security) no host path, `PATH` contents, struct name or stack trace in output or JSON | **MET** — `preconditions_output_leaks_no_host_path_or_env_value` asserts both forms |
| (security) fixed program NAME resolved through inherited `PATH`, **never spawns** | **MET** — `resolves_on` does a directory walk; no `Command` in the new code |
| (security) `cargo audit` 48th re-pin, exit captured before any pipe | **MET** — exit **1**, `duplicate advisory ID: RUSTSEC-2026-0244` |
| (security) `cargo deny` exits 0; delta stated as package COUNT | **MET** — exit **0**; count **564**, unchanged |
| (tests) both runners pass, zero `retries` added | **MET** — nextest 691/691 and `cargo test` green over the touched crates |
| (tests) no test binds `127.0.0.1:4317` | **MET** — the connectability arm reuses the existing ephemeral-port stub pattern; no new bind |
| (tests) `.sh`/`.ps1` identical exit-code-readable `boot`; command count still five | **MET** — both drove to exit 1 with preflight skipped; `case`/`switch` arms still five |
| (obs) single-line JSON self-obs; no new span name; no field outside the allowlist | **MET** — one `warn!` on `message`; `run_id` minted by `ServiceIdentity::resolve` |
| (obs) zero unlogged panics | **MET** — no `^thread.*panicked` in the probe's stderr |
| (design) zero new colors, no seventh lamp, no sixth `ReportState`; caption omitted when N/A | **MET** — `PRECONDITION_MUTE = 246` reuses the shipped tier; `preconditions_caption` returns `None` when satisfied |
| (design) ASCII label always rendered; legible with color stripped | **MET** — asserted under `NO_COLOR` + a pipe |
| (layouts) flat verb-noun, one level; stderr `error:`/`hint:`; stdout parseable, ANSI-stripped when piped | **MET** |
| (a11y) no WCAG claim; probe gates only the driven/SR arms, cannot relax the routine arm | **MET** — no a11y file touched |

**Gates green** (commands run): `cargo nextest run -p conductor-core -p conductor-verify -p conductor-run -p conductor-cli -p conductor-emit --profile ci` **691/691** · `cargo test` over the same crates (runner portability) **green** · `cargo clippy --workspace --all-targets -- -D warnings` **clean** · `bash scripts/agent-run.sh run` **824/824** + doctests + clippy · `cargo audit` **exit 1 (pinned signature, 48th)** / `cargo deny check advisories bans licenses sources` **exit 0**.

**Smoke** (boot-path changed): MINT-THEN-READ, not a bare `status`. Pre-leg newest journal `2026-09-01T22-14-57-257`; the scenario leg minted `2026-09-03T19-12-17-540`; `status` read that exact id back at exit 0 with `state: "Blocked"`, `verdict: null`, null measurement fields — the correct no-live-Pulse spine. `boot` drove to exit 1 in **both** shells with all three subjects named and **no `ReadyState` JSON emitted**, proving the preflight was skipped rather than merely failed.

**Two findings surfaced for ownership at P5** (both re-verified first-hand by the overseer on this host):
1. **Panic-hook race** — `conductor-core/src/obs.rs:491-496` and `:532-537` each `take_hook`/`set_hook` the process-global panic hook; under `cargo test` they share one process and race. Observed red exactly once, on the first run after this chunk added 14 core tests and shifted scheduling; passes 4/4 since, and did not reproduce at HEAD in 3 forced-parallel runs. **Pre-existing latent defect UNMASKED, not caused** — the mechanism is in HEAD's source. Not fixable in-scope: both tests use crate-private helpers, so relocating them to their own binary would widen the public API for tests alone.
2. **rustfmt edition mismatch** — `.claude/settings.json:36` runs bare `rustfmt "$f"` with no `--edition` and no `rustfmt.toml` exists, so an edition-2024 workspace gets 2015-style import sorting. This chunk's `render.rs` 222+/43−, `spawn.rs` 182+/6−, `cli_smoke.rs` 221+/37− are largely re-wraps of pre-existing lines, and it defeated one anchored Edit mid-burst. `cargo fmt --check` is **not a gate here** (absent from CI and `agent-run.sh`) and already flags **745 spots workspace-wide** including untouched files, so the non-canonical state predates this chunk entirely.

**Process hygiene** — re-measured against the host process list at wrap (readable from this session):

| Process | Started by | Final state |
|---|---|---|
| `cargo` / `conductor.exe` | this run (gates, probe, scenario leg, both `boot` arms) | `terminated` — 0 live |
| `andromeda-pulse-mcp` | never spawned — the probe is a `PATH` lookup by design | n/a |
| `pulse-app` · `tauri-driver` · `msedgedriver` · `nvda` | not involved in this chunk | n/a |

Census across 8 process names: **zero stragglers**; no LISTENING socket on `:4317`.
