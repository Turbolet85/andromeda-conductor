# Disposition ledger — `conductor-run` mutation survivors

**Chunk:** 2026-09-03-conductor-run-composition-root-survivors-dispositioned
**Instrument:** cargo-mutants 27.1.0 · `-f crates/conductor-run/src/lib.rs -f crates/conductor-run/src/dispatch.rs --test-tool=nextest --jobs 2`
**Read-out:** nested `mutants.out/` tallies, per `.claude/rules/testing.md` 2026-09-03. Never the exit code.

| | pre-state (P3, before kills) | post-state (after kills) |
|---|---|---|
| mutants found | **118** | **118** — the same count, which is what makes the two scores comparable |
| missed | 25 | **8** |
| caught | 65 | **82** |
| unviable | 28 | 28 |
| timeout | 0 | 0 |
| wall clock | 12m | 12m |

**17 killed · 8 accepted-deliberate.** `missed.txt` holds exactly the accepted set; every other named
survivor appears in `caught.txt` (verified per-site, not inferred from the totals).

---

## Killed (17)

| site | mutation | killed by |
|---|---|---|
| `lib.rs:197:21` | `<` → `<=` in `attribute_by_liveness` | `liveness_attribution_turns_on_the_idle_threshold_exactly` |
| `lib.rs:250:5` ×4 | `active_incident_ids` → `Ok(vec![])` / `[0]` / `[1]` / `[-1]` | `active_incident_ids_reads_every_id_the_corpus_returns` · `active_incident_ids_is_empty_only_when_the_corpus_is` |
| `lib.rs:346:5` | `observe_run_contract` → `Default::default()` | `observe_run_contract_names_the_term_this_environment_does_not_declare` |
| `lib.rs:358:5` | `declares` → `true` | same test — an always-true `declares` reports the undeclared term as satisfied |
| `lib.rs:401:5` | `warm_up_canary_service` → `Ok(())` | `the_warm_up_emits_its_declared_count_paced_by_the_declared_gap` |
| `lib.rs:402:24` | `==` → `!=` (`warmup_ms == 0`) | same |
| `lib.rs:402:55` | `==` → `!=` (`warmup_emissions == 0`) | same |
| `lib.rs:402:29` | `\|\|` → `&&` | `a_zero_length_warm_up_window_emits_nothing` |
| `lib.rs:405:64` ×2 | `/` → `%` / `*` in the gap arithmetic | `the_warm_up_emits_its_declared_count_paced_by_the_declared_gap` (1000/4 = 250ms ×4 = 1000ms; `%` → 0ms, `*` → 16000ms) |
| `lib.rs:715:9` ×2 | `FaultKind::label` → `""` / `"xyzzy"` | `the_fault_label_is_the_bounded_obs_plan_token` |
| `lib.rs:753:5` | `fault_span` → `None` | `a_fault_declaring_phase_opens_a_span_and_a_plain_phase_does_not` |
| `dispatch.rs:163:26` | `==` → `!=` in `Dispatcher::dispatch` | `a_zero_error_depth_places_the_error_at_the_root_and_a_deeper_one_does_not` |

## Accepted-deliberate (8), each against a cited standing rule

### A. Env read at the caller — 4 survivors (was 6)

`lib.rs:358:5` (`declares` → `false`) · `lib.rs:360:11` · `lib.rs:360:21` · `lib.rs:360:26`

**Cited rule:** `.claude/rules/testing.md` 2026-08-10 — env is read at the caller and passed in as a typed
value, so the edge stays thin and both branches test with no `unsafe` env mutation. Killing these requires
`std::env::var` to return a *set* value, i.e. `unsafe { set_var }` in a shared-process test module — the exact
hazard the 2026-08-20 chunk removed. Ratified 2026-08-21; procedural anchor `.claude/rules/testing.md:19`.

**The class SHRANK 6 → 4.** Two of its former members proved killable with no env mutation at all, because
the env read lives in `declares` and not in its caller: `observe_run_contract:346` (the P4 operator-ratified
attempt) and `declares:358 → true`. Both are now in `caught.txt`. The roster entry in test-plan §12 should
record four members, not six.

### B. Unreachable past a fixed-endpoint egress gate — 3 survivors

`lib.rs:518:27` (delete field `degraded`) · `lib.rs:559:25` ×2 (`-` → `+` / `/`)

**Cited rules:** architecture.md §Occupied Resources — Ports and §Cross-cutting Patterns — Trust boundary;
test-plan §10 zero-flakiness.

These sit inside `execute_scenario` past `probe_egress(DEFAULT_OTLP_ENDPOINT)` (`lib.rs:488`) and
`Dispatcher::connect(scenario, DEFAULT_OTLP_ENDPOINT)` (`:493`), where `DEFAULT_OTLP_ENDPOINT` is the **fixed**
`http://127.0.0.1:4317` (`conductor-emit/src/client.rs:23`), not an injectable parameter. Reaching them
requires binding Pulse's own ingest port inside a unit test — a second deliberate inbound bind architecture
does not sanction — and the barrier stands on two legs: the tier runs mutant processes in parallel
(`--jobs 2`), so a FIXED port is contended between them and flaky by construction against the zero-retry bar
(test-plan §10), and any live Pulse on the dev host already holds `:4317` (architecture §Occupied Resources —
Ports). No integration test can drive it either: `Preflight`'s fields are crate-private, as
`tests/operator_pause_harvest.rs:20` records, and no `tests/*.rs` calls `execute_scenario`. Making the
endpoint injectable would be a production signature change, barred by the plan's §Constraints.

### C. Unreachable without the sidecar binary — 1 survivor

`lib.rs:66:8` (delete `!` in `preflight`)

**Cited rules:** architecture.md §Occupied Resources — Service / process names (the sidecar is a fixed program
NAME resolved through the inherited `PATH`); test-plan §10 zero-flakiness.

**This one is a deviation from the plan**, which had scoped it as a kill via a `tracing` capture. Measured at
implement time: reaching `lib.rs:66` requires `preflight()` to hold a *connected* client, and
`ReadbackClient::connect` spawns `andromeda-pulse-mcp` — absent on PATH in a hermetic run, so the function
returns at `:61` and never reaches the guard. No test drives `preflight()` at all. A kill would require either
the sidecar binary as a test precondition (host-dependent, against the zero-retry bar) or a new injectable
seam in production code (a signature change, barred by §Constraints). The plan's own wording for the sibling
`:346` case — "if no such assertion proves reachable, classify it accepted-deliberate against that same rule
and record why — a sanctioned exit, not a failure" — is the exit taken here.

---

## Standing PREREQ — the 47th `cargo audit` re-check

**Signature reproduced byte-identically.** `cargo audit` exit **1**, first diagnostic
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
`cargo deny check advisories bans licenses sources` exit **0** (`advisories ok, bans ok, licenses ok,
sources ok`). Both exits read directly from the bare command, never through a pipe. No deviation: the
deferral re-pins as a database fault, with no floor raise, no `deny.toml` ignore and no CI edit.

`Cargo.lock` un-drifted at **564 packages**; no manifest edit was needed (the in-process duplex resolved
through `tokio`'s `io-util`, already enabled by conductor-verify as a normal dependency).

## Production-source delta

**Byte-unchanged.** Both `lib.rs` hunks (`:959` imports, `:1533+`) fall inside the `#[cfg(test)]` module that
opens at `:956`; `tests/dispatch_wire.rs` is wholly a test target.
