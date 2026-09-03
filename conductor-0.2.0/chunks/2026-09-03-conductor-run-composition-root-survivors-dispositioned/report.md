# Report — 2026-09-03-conductor-run-composition-root-survivors-dispositioned

**Chunk:** conductor-run composition-root survivors dispositioned — every standing mutation survivor in the crate killed or classified accepted-deliberate against a cited rule
**Date:** 2026-09-03
**Commits:** none yet — this wrap authors the chunk's single commit

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-run/src/lib.rs` (test module only) · `crates/conductor-run/tests/dispatch_wire.rs` · `conductor-0.2.0/chunks/2026-09-03-conductor-run-composition-root-survivors-dispositioned/{scope,research,plan,report}.md` + `evidence/disposition-ledger.md` · `conductor-0.2.0/working-route.md` · `.andromeda/master-route.md`
- **Symbols / APIs:** **none — no production symbol added, changed, or removed.** Production source is byte-unchanged: both `lib.rs` hunks (`:959` test-module imports, `:1533+`) fall inside the `#[cfg(test)]` module opening at `:956`, and `tests/dispatch_wire.rs` is wholly a test target. 11 test functions and 4 test-only helpers were added; no public surface, no port, no socket, no env var.
- **Crates / modules:** none added, removed, or changed.
- **Dependencies:** **none.** `Cargo.lock` un-drifted at **564 packages**; no manifest edit. The plan's conditional `crates/conductor-run/Cargo.toml` touchpoint proved unnecessary — the in-process duplex resolved through `tokio`'s `io-util`, already enabled by `conductor-verify` as a normal dependency, so feature unification supplied it.
- **Schema / config:** none — no migration, config key, violation schema, or scrub/redaction shape moved.
- **Spec-master edits:** **test-plan.md** — §12 (mutation-instrument roster), §10 (roster reference), §500 (crate attribution). Applied via the P2 amendment flow, not by this chunk's code.
- **Counts / qualifiers moved:**
  - The **accepted-deliberate `declares` class: 6 → 4 members.** Stated in `test-plan.md` §12 and referenced in §10. Two former members are now killed and in `caught.txt`: `observe_run_contract:346` and `declares:358 → true`.
  - `conductor-run`'s mutation tally: **25 missed → 8 missed**, 65 → 82 caught, at a constant `Found 118 mutants`. No doc states these tallies as a literal.
- **Dev-tool versions:** none — cargo-mutants 27.1.0 unchanged (a floor, not a pin).
- **Harness / gate surface:** none — no `agent-run` script, xtask verb, CI step, or status/verdict shape changed. The mutation tier remains an operator/local instrument, never a CI stage.
- **Cross-project / external claims:** none. No Pulse-side fact was read or asserted this chunk; no live leg ran.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **`test-plan.md` §12's `declares` roster states SIX accepted-deliberate members; four is correct.** Measured by the tier: `observe_run_contract:346` and `declares:358 → true` are in `target/mutants-impl-2026-09-03/mutants.out/caught.txt`, killed by `observe_run_contract_names_the_term_this_environment_does_not_declare`. The env read lives in `declares` (`lib.rs:357`), not in its caller, so the cited env-at-the-caller rule never covered the enclosing function's mutant nor the always-true arm. Disposition: **applied** as a §12/§10 amendment (see Spec-master edits).
  2. **`test-plan.md:500` attributes the `declares` ×6 env-reading edge to `conductor-verify`; it is in `conductor-run`.** Measured at take-up: `grep -rn 'fn declares' crates/*/src/*.rs` returns a single hit, `crates/conductor-run/src/lib.rs:357`. Disposition: **applied** as a §500 amendment.
  3. **The code audit's "Suspected shape" — `lib.rs` "grows every epoch while its own test coverage does not follow" — is not supported.** Measured at P3 over 2026-08-16 → 2026-09-02: test lines +94% (297→575) against production's +72% (555→955); the ratio ROSE 0.535 → 0.602. The claim is hypothesis-marked in `.andromeda/runs/2026-09-02T15-49-17-code-audit/proposals.md` (a run artifact, not a spec master). Disposition: **recorded** in `scope.md`'s premise closure as `[premise-corrected]`; no spec master states it, so no amendment is owed.
- **Expected amendments (from plan):**
  - `test-plan.md` §12 roster gains `conductor-run`'s set — **carried**, with a corrected value: the plan predicted the `declares` class would shrink 6 → 5 if the `:346` kill landed; it shrank 6 → **4**. Fact in Changes → *Counts / qualifiers moved* and *Spec claims disproved* #1.
  - `test-plan.md` §10 roster reference tracks the same change — **carried**. Same fact.
  - `test-plan.md` §500 re-attribution `conductor-verify` → `conductor-run` — **carried**. Fact in Changes → *Spec claims disproved* #2.
- **Coverage of new surfaces:** none — this chunk introduces no external surface, hot-path operation, or UI element. Every added line is a test. Flags are `n/a` by construction.

## Deviations from intent

1. **`preflight:66:8` was classified accepted-deliberate, not killed.** The plan scoped it as a kill via a `tracing::subscriber::with_default` capture of the `preflight blocked: readiness gate not satisfied` line. Measured at implement: reaching `lib.rs:66` requires `preflight()` to hold a *connected* client, and `ReadbackClient::connect` spawns the fixed `andromeda-pulse-mcp` program name through the inherited `PATH` — absent on this host, so the function returns at `:61` and never reaches the guard; no test drives `preflight()` at all. **Justification:** a kill needs either the sidecar binary as a host precondition (host-dependent, against the zero-retry bar) or a new injectable seam in production code (a signature change, barred by the plan's §Constraints). The plan's own wording for the sibling `:346` case — "if no such assertion proves reachable, classify it accepted-deliberate against that same rule and record why — a sanctioned exit, not a failure" — is the exit taken.

2. **The `declares` class shrank 6 → 4, where the plan predicted 6 → 5.** The operator-ratified `:346` attempt landed as expected; `declares:358 → true` fell to the *same* test as an unpredicted side effect (an always-true `declares` reports an undeclared term as satisfied, which the assertion catches). **Justification:** strictly better than planned, and it shrinks a ledger of owed work under compulsion — the standing ethos. It moves the §12 amendment's value, which is dispositioned above.

3. **No manifest edit was needed.** The plan listed `crates/conductor-run/Cargo.toml` as a conditional touchpoint in case the in-process duplex required a `tokio` feature this crate does not enable. **Justification:** `io-util` is already enabled by `conductor-verify` as a normal dependency, so feature unification supplied it; `Cargo.lock` stayed un-drifted at 564 packages.

4. **A clause in the plan's step 3 (and in the first draft of the evidence ledger) was inaccurate and is corrected in the ledger.** Both said reaching `:518`/`:559` would collide "with this crate's own occupier guard test (`lib.rs:1176`)". Measured: `a_fault_phase_guard_binds_and_its_drop_releases` calls `phase_guard(&scenario, &window, 0, 0, …)` — `occupier_port = 0`, an ephemeral bind — and never touches `:4317`. **Justification:** operator-corrected at the wrap directive; the barrier stands unchanged on its two real legs (a fixed `:4317` bind contended between `--jobs 2` parallel mutant processes, test-plan §10; and any live Pulse on the dev host, architecture §Occupied Resources — Ports). The evidence ledger is corrected; `plan.md` retains the original clause as the record of what was planned, superseded by this report and the ledger.

## Decisions & corrections

- **Operator ruling at phase P4** — re-open `observe_run_contract:346` for a kill attempt rather than carrying all six `declares` members as-is, on the ground that the cited env-at-the-caller rule covers the env READ (in `declares`) and not its caller. Outcome: the kill landed, and took a second member with it.
- **Operator review at phase P5 (four notes)** — the plan's step-3 remedy for `:518`/`:559` named a test that never enters `execute_scenario`; the `warm_up_canary_service` home and driver were unnamed; the `:346` mechanism was described falsely as passing a declared set in; and `:66`'s capture mechanism was unnamed. All four verified true at HEAD and applied before approval. The same class of hand-wave was then found and fixed one step over, in step 2's `active_incident_ids` driver.
- **Operator wrap directive item 1** — the occupier-guard-collision clause is inaccurate; corrected in the ledger (deviation 4).
- **Operator wrap directive item 2** — classes B and C are accepted at the *tier*, but their observables are live-path facts a driven run can assert; lean toward a CARRY pinned to the entry owning live verification. Resolved at P5 (route-resolve).
- **Standing convention reaffirmed** — a mutation survivor's disposition is *killed* or *accepted-deliberate against a cited rule*; the score is a consequence, never the acceptance (test-plan §10).

## Outcome

Every acceptance criterion re-asserted against the **diff**, not the plan's text:

| criterion | verdict against the diff |
|---|---|
| Tier reports `Found 118 mutants` | **MET** — 118 both runs, so the two scores are comparable |
| `missed.txt` holds exactly the accepted set; every other survivor in `caught.txt` | **MET** — 8 missed / 8 accepted-deliberate, verified per-site against both files, not inferred from totals |
| Each of the 20 in-scope survivors ends killed or accepted against a cited rule | **MET** — 17 killed, 3 accepted (`:518`, `:559` ×2); plus `:66` re-classified (deviation 1) and 4 `declares` carried |
| §12 roster EXTENDED, not contradicted; no numeric threshold as acceptance | **MET** — extended with `conductor-run`'s classes B and C, and corrected 6 → 4; no `--fail-under` used |
| Both runners green, zero retries | **MET** — `nextest -p conductor-run` 167 passed; `cargo test -p conductor-run` exit 0 |
| CI coverage threshold untouched | **MET** — the diff contains no workflow or `.config` edit |
| Tests live in `crates/conductor-run`; no new member, no new cross-seam edge | **MET** — diff touches two files, both in that crate |
| No new `CONDUCTOR_*` / `ANDROMEDA_PULSE_*` handle; no test writes process env | **MET** — no env write in the diff; the `:346` kill turns on an env name being *unset* |
| Typed `Verdict`/`ReportState`/`ReadBack`/`LifecycleVerdict` asserted; `Err` for harness faults only | **MET** — `LifecycleVerdict` and `RunContractStatus` asserted as values |
| Only bounded span names and allowlisted attributes; no span removed or downgraded | **MET** — no instrumentation changed; `fault_span` asserted for presence, not by widening any set |
| `FaultKind::label` pins the literal `"silence"` / `"ramp"` | **MET** |
| `cargo deny` exit 0, read directly | **MET** — `advisories ok, bans ok, licenses ok, sources ok` |
| `cargo audit` re-checked as the 47th; signature or full-form deviation | **MET** — exit 1, `duplicate advisory ID: RUSTSEC-2026-0244`, byte-identical; no floor raise, no ignore, no CI edit |
| `Cargo.lock` un-drifted; any delta as a package count | **MET** — un-drifted, 564 packages |
| No committed record carries an absolute host path | **MET** — ledger scanned; the single pattern hit was `http://` matching the drive-letter alternation, read and dismissed |
| a11y fixture files absent from this chunk's diff | **MET** — `envelope_fixture.rs`, `lamps_fixture.rs`, `tests/fixtures/` untouched |

**Gates green** (exits read direct from the bare command, never through a pipe): `cargo nextest run -p conductor-run --profile ci` (167 passed) · `cargo test -p conductor-run` (0) · `cargo clippy --workspace --all-targets -- -D warnings` (0) · `cargo nextest run --workspace --profile ci` (792 passed) · `cargo mutants -f …lib.rs -f …dispatch.rs --test-tool=nextest --jobs 2` (118 tested; 8 missed / 82 caught / 28 unviable / 0 timeout) · `cargo deny check advisories bans licenses sources` (0) · `cargo audit` (1, pinned signature).

**Smoke:** skipped — no boot-path / UI-surface change. Both touchpoints are test code; production source is byte-unchanged and `conductor-run` is a library with no entry point.

**Process hygiene:** implement's P4 census recorded **zero stragglers**; re-measured at this wrap against the host process list by name (cargo · cargo-mutants · nextest · conductor · conductor-tauri · andromeda-pulse-mcp · pulse-app · msedgedriver · tauri-driver · nvda) — **none running**. No listener was opened; no Pulse, WebDriver or screen-reader process was involved.
