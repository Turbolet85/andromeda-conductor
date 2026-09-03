# Codebase Research — 2026-09-03-conductor-run-composition-root-survivors-dispositioned

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 6 · **Graph queries:** 1 (43 rows, plane `rust`, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/testing.md` — indexed structurally (78 lines / 54 KB, multi-KB
  single-line entries), then `:72`, `:77`, `:78` read in full; `:67` read at take-up. 33 Session Additions
  present. No live leg in this chunk, so `.claude/rules/verification-harness.md` was not consulted.

## The decisive measurement (taken at P3, not deferred)

The tier was **re-measured at HEAD** rather than planned against the 2026-09-02 audit table:

```
cargo mutants -f crates/conductor-run/src/lib.rs -f crates/conductor-run/src/dispatch.rs \
  --test-tool=nextest --jobs 2 --output target/mutants-run-2026-09-03
→ 118 mutants tested in 12m: 25 missed, 65 caught, 28 unviable, 0 timeout   (cargo-mutants exit 2)
```

The measured 25-row survivor set is **composition-identical** to the audit's table — same sites, same mutation
texts, 24 in `lib.rs` + 1 in `dispatch.rs`. The plan therefore rests on measurement, not on a month-old
hypothesis. Read-out followed `.claude/rules/testing.md` 2026-09-03: a **fresh** `--output` dir, tallies taken
from the **nested** `mutants.out/`, verdict from tallies not the exit code.

**Two facts the audit did not have.** (1) **0 timeouts** — every survivor here is a MISSED, so the remedy class
is uniformly *strengthen the assertion*, never the sibling's *bound the await*. (2) The tier needs **no
build-graph repair**: 118 mutants enumerate, baseline is 156/156 exit 0, `ui/dist` present.

## Files inspected
- `crates/conductor-run/src/lib.rs` (1530 lines; read at `:60-70`, `:190-200`, `:245-255`, `:344-362`,
  `:395-412`, `:512-522`, `:553-565`, `:705-760`, and the `#[cfg(test)]` module opening at `:956`) — the
  composition root holding 24 of 25 survivors. Production `:1-955`, test module `:956-1530`.
- `crates/conductor-run/src/dispatch.rs` (`:158-170`) — the 25th survivor: the P-008 root-vs-deep error
  placement branch `if d == 0 { ErrorPlacement::Root } else { DeepChild { depth: d } }`.
- `crates/conductor-run/Cargo.toml` (full) — **no `conductor-tauri` edge**; dev-deps already carry `tokio`
  (`macros`/`rt`/`net`/`test-util`), `assert_fs`, `opentelemetry-proto`, `tonic`, `tokio-stream`, `insta`,
  `serde_json`. A `live-pulse` feature declares no dependency.
- `crates/conductor-run/tests/` (17 files) + `tests/fixtures/` (`lamps-journal.jsonl`, `over-envelope.toml`).
- `.claude/rules/testing.md` — see Harness rules above.

## Graph impact (`.andromeda/runs/2026-09-03T10-31-07-phase/tree-query-…json`, 43 rows, `probe_hits: null`)
- **`declares`** · **`observe_run_contract`** · **`warm_up_canary_service`** · **`fault_span`** — 1 call site
  each, all inside `lib.rs`; **crate-internal, zero cross-crate blast radius**. Killing tests can live in the
  in-file `#[cfg(test)]` module without widening any public API.
- **`active_incident_ids`** — 2 sites, `lib.rs` only. Same containment.
- **`attribute_by_liveness`** — 4 sites, and **every caller is a test** (`tests/lifecycle_harvest.rs`,
  `tests/lifecycle_live.rs`). Its boundary is already test-driven; only the `<`/`<=` edge is unasserted.
- **`execute_scenario`** — 7 sites / 3 files, crossing into `conductor-cli` (`commands/run.rs`,
  `commands/suite.rs`). The only survivor-bearing symbol with cross-crate callers; an assertion-only change
  threads nothing, but a signature change would (none is planned).
- Independently reproduces the corrected figures in CLAUDE.md's session learnings (`execute_scenario` 7 sites,
  `persist` 10) — the retired path-anchor artifact stays retired.

## Patterns detected
- **Assert the mutant's difference on the surface you observe** (`.claude/rules/testing.md:72`): a mutant
  survives wherever the harness answers it as agreeably as correct code. Two shapes named there — an ECHO stub
  blind to the value it echoes, and a one-shot guard needing THREE observations to pin.
- **Crate-private mechanism, unit-pinned arms** (`lib.rs:512`, `route_read_back` → `ReadBack{Graded,
  AutoResolved, Blocked}`): the shipped shape for reaching a composition-root branch from inside the crate.
  Its three arms have tests at `lib.rs:977`, `:989`, `:1009` — but **those exercise `route_read_back` itself,
  the classifier, not `execute_scenario`'s arms.** `a_declare_only_empty_read_back_routes_to_the_auto_resolve_residual`
  (`:989`) calls `route_read_back` and then `manual_record` **directly**, passing its own literal
  `Observation { degraded: true, .. }` and `3_000`; it never enters `execute_scenario`. **Corrected at the P5
  review:** `:518`/`:559` therefore survive not because an existing test under-asserts, but because
  **nothing drives `execute_scenario` past its egress gates at all** — `probe_egress` (`:488`) and
  `Dispatcher::connect` (`:493`) both take the fixed `DEFAULT_OTLP_ENDPOINT`
  (`conductor-emit/src/client.rs:23`), the two in-file drivers (`:1369`, `:1384`) stop at the Blocked arm via
  `blocked_preflight()`, and no `tests/*.rs` can drive it at all because `Preflight`'s fields are crate-private
  (recorded at `tests/operator_pause_harvest.rs:20`).
- **Fault classification is tested, its labels are not**: `a_zero_occurrence_phase_classifies_as_silence_whatever_its_shape` (`lib.rs:1446`) and `only_the_two_run_path_faults_classify` (`lib.rs:1456`) exercise
  `classify_fault`, yet `FaultKind::label` survives — the tests assert the *variant*, never the emitted
  `fault_type` string. obs-plan §4 pins those literals (`"silence"` / `"ramp"`), so the assertion is available.
- **Boundary constants are named, not inlined** (`lib.rs:197`, `AUTO_RESOLVE_IDLE_SECONDS`): a `<` → `<=`
  mutant is killed by one case sitting exactly on the constant.
- **`std::time` wall-clock for journal-relative arithmetic** (`lib.rs:559`, `observed_ms - emitted_ms`;
  `:753`, `now_ms().saturating_sub(emitted_ms)`) — never the virtual clock, per arch §Determinism discipline.

## Conventions to follow
- **In-file `#[cfg(test)] mod tests`** at `lib.rs:956` is the crate's unit home (~20 `#[test]`/`#[tokio::test]`
  attributes, 30+ helper/test fns); crate-local `tests/*.rs` is the integration home. New kills for the
  crate-internal survivors belong in the in-file module (test-plan §4 Conventions).
- **Read-out gate** (test-plan §4, as reconciled by the sibling): `missed.txt` holds **exactly** the run's
  accepted-deliberate survivors; every other named survivor appears in `caught.txt`. Never the exit code.
- **Fresh `--output` dir per run; read the nested `mutants.out/`** (`.claude/rules/testing.md:77` — a stale
  pre-existing dir once produced a confident, wholly false clean sweep).
- **Env is read at the caller and passed in as a typed value** (`.claude/rules/testing.md:67`); no
  `unsafe { set_var }` in a shared-process test module.
- **Both runners green**: `cargo nextest run -p conductor-run` **and** `cargo test -p conductor-run`.

## New files to create
- (none anticipated) — every survivor is reachable from the existing in-file test module or the existing
  `tests/` files. No new fixture is required by the measured set.

## Files to modify
- `crates/conductor-run/src/lib.rs` — **`#[cfg(test)]` module only** for the 18 undispositioned `lib.rs`
  survivors; production code expected byte-unchanged (the sibling's shape, and arch's
  `2026-08-21-severity-lifecycle-live-proof` precedent that dispositioning an arm need not move production
  source).
- `crates/conductor-run/src/dispatch.rs` or `crates/conductor-run/tests/dispatch_wire.rs` — the
  `dispatch.rs:163` root-vs-deep placement kill. `dispatch_wire.rs` already captures against the loopback OTLP
  stub, so the placement assertion has a home; which of the two hosts it is an /implement judgment.
- **No manifest edit anticipated.** Every dev-dependency a kill would need is already declared. Should that
  prove wrong, the `Cargo.lock` move is reported as a **package-count delta**, never as "byte-unchanged"
  (`.claude/rules/security.md` 2026-09-02), with `cargo deny` verified green over the new lock.
- **Caller threading: not applicable.** No survivor's remedy changes a signature, so no caller set is
  enumerated. `execute_scenario`'s 3 cross-crate caller files are recorded above as the boundary that *would*
  be in play if that changed.

## Survivor map — the 25, by disposition status

**Already ratified accepted-deliberate (6), carried forward, re-asserted not re-litigated:**
`observe_run_contract:346` + `declares:358`×2 + `declares:360`×3 — the env-at-the-caller class, ratified
2026-08-21 against `.claude/rules/testing.md` (procedural anchor `:19`; substantive ground `:67`).

**Undispositioned (19), with the observable each turns on:**
| # | site | the difference to observe |
|---|---|---|
| 1 | `lib.rs:66:8` delete `!` in `preflight` | the `preflight blocked: readiness gate not satisfied` log line fires on the wrong branch |
| 2 | `lib.rs:197:21` `<`→`<=` in `attribute_by_liveness` | a case sitting exactly on `AUTO_RESOLVE_IDLE_SECONDS` |
| 3-6 | `lib.rs:250:5` `active_incident_ids` → `Ok(vec![])`/`[0]`/`[1]`/`[-1]` | a stub returning known ids the assertion pins |
| 13-18 | `lib.rs:401/402/405` `warm_up_canary_service` | the early-return guard arms, and the `warmup_ms / warmup_emissions` gap arithmetic |
| 19 | `lib.rs:518:27` delete `degraded` field | **unreachable** — inside `execute_scenario` past the fixed-endpoint egress gates; accepted-deliberate |
| 20-21 | `lib.rs:559:25` `-`→`+`/`/` in `execute_scenario` | **unreachable** — same gate; accepted-deliberate |
| 22-23 | `lib.rs:715:9` `FaultKind::label` → `""`/`"xyzzy"` | the literal obs-plan §4 `fault_type` values |
| 24 | `lib.rs:753:5` `fault_span` → `None` | the `fault.*` span is emitted at all |
| 25 | `dispatch.rs:163:26` `==`→`!=` | `error_depth: Some(0)` ⇒ Root vs `Some(n>0)` ⇒ DeepChild |

## Open questions
- Whether any of the 19 resists a killing assertion and must instead be classified accepted-deliberate against
  a cited rule → blocks: **implementation-scope**. The measured set makes each one's observable explicit
  (table above), so the file list is firm; only the per-site verdict is open, and both exits are sanctioned by
  test-plan §10.
- Whether the `dispatch.rs:163` kill belongs in `dispatch.rs`'s own test module or `tests/dispatch_wire.rs` →
  blocks: **implementation-scope**. Both are existing, sanctioned homes.
