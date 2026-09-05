# Disposition ledger — the boundary-#4 code audit's named survivors

**Chunk:** 2026-09-05-audit-corrective
**Instrument:** cargo-mutants 27.1.0 · `--test-tool=nextest --jobs 2`, one fresh `--output` dir per run under the
gitignored `target/`
**Read-out:** the NESTED `mutants.out/` tallies, per `.claude/rules/testing.md` 2026-09-03. Never the exit code —
every run below exited non-zero while meeting its acceptance (a surviving accepted-deliberate mutant is a
non-zero CLASS, not a failure).
**Basis:** `.andromeda/runs/2026-09-05T08-35-52-code-audit/` (`c-mutation-conductor-{cli,core,verify,tauri,run}.json`)
at HEAD `59d5b7c`.

## Scores

| unit | before (audit) | after | mutants | caught | missed | timeout | unviable |
|---|---|---|---|---|---|---|---|
| `conductor-cli` | **58.59** (41 missed) | **97.85** | 111 | 91 | **2** | 0 | 18 |
| `conductor-verify` | 88.33 (14 missed, **2 timeouts**) | 88.62 | 148 | 109 | 14 | **0** | 25 |
| `conductor-core` (`obs.rs`-scoped) | 6 `civil_from_unix` missed | **0 in `civil_from_unix`** | 203 | 190 | 7 | 0 | 6 |
| `conductor-tauri` | 89.29 (3 accepted) | 90.32 | 46 | 28 | 3 | 0 | 15 |
| `conductor-run` | 94.44 (5 accepted) | 94.32 | 118 | 83 | 5 | 0 | 30 |

`missed.txt` holds EXACTLY that unit's accepted-deliberate set in every case; every other named survivor is in
`caught.txt` (verified per site, not inferred from the totals).

---

## W1 — `conductor-cli`: 41 named survivors → 39 killed · 2 accepted-deliberate

The unit's mutant population moved 117 → 111 because `color_enabled` replaced two copies of the
three-condition tty rule with one — six operator mutants became three.

### Killed (39)

| site (audit coordinates) | mutation | killed by |
|---|---|---|
| `render.rs:62:5` ×2 | `lamp_code` → `0` / `1` | `render::tests::each_lamp_carries_its_own_palette_code` |
| `render.rs:75:5` ×2 | `paint` → `""` / `"xyzzy"` | `render::tests::the_public_renders_carry_their_ascii_content` · `cli_smoke::report_renders_the_latest_run` (the run_id through the report header) |
| `render.rs:95:5` ×2 | `hold_line` → `""` / `"xyzzy"` | `the_public_renders_carry_their_ascii_content` |
| `render.rs:104:5` ×3 | `envelope_caption` → `None` / `Some("")` / `Some("xyzzy")` | `the_public_renders_carry_their_ascii_content` |
| `render.rs:132:5` ×2 | `coverage_summary` → `""` / `"xyzzy"` | `the_public_renders_carry_their_ascii_content` |
| `render.rs:156:53` | `!=` → `==` in `stdout_color` | `color_is_enabled_only_when_every_condition_allows_it` (the `TERM=dumb` row) |
| `render.rs:162:5` | `stderr_color` → `true` | `cli_smoke::run_with_unknown_target_is_a_sanitized_error_with_a_hint` (piped stderr carries no escape byte) |
| `render.rs:163:9` · `:164:9` | `&&` → `\|\|` in `stderr_color` | `color_is_enabled_only_when_every_condition_allows_it` |
| `render.rs:164:53` | `!=` → `==` in `stderr_color` | same |
| `render.rs:270:30` | `==` → `!=` in `coverage_table_styled` | `only_the_out_of_scope_mode_cell_carries_the_residual_tint` |
| `render.rs:291:25` · `:293:12` · `:299:18` · `:299:40` · `:299:43` | the roll-up's mode filter / auto-term guard | `the_coverage_roll_up_is_derived_from_the_classification` |
| `render.rs:329:5` ×2 | `wire` → `""` / `"xyzzy"` | `the_results_table_renders_every_measurement_cell` |
| `render.rs:338:5` ×2 | `latency` → `""` / `"xyzzy"` | same |
| `render.rs:343:5` ×2 · `:345:20` ×2 | `fingerprints` → `""` / `"xyzzy"`, match guard → `true` / `false` | same (a populated, a measured-empty and a blocked row in one table) |
| `main.rs:55:5` ×2 | `hint_for` → `""` / `"xyzzy"` | `cli_smoke::run_with_unknown_target_is_a_sanitized_error_with_a_hint` (the hint TEXT, and the generic fallback asserted absent) |
| `commands/mod.rs:29:5` | `exit_code` → `Default::default()` | `commands::tests::only_a_hard_fail_exits_non_zero` |
| `paths.rs:73:39` | `delete !` in `Paths::load_all_scenarios` | `cli_smoke::suite_filter_selects_the_matching_scenario_and_no_other` |
| `paths.rs:130:5` | `env_seed` → `None` | `cli_smoke::the_seed_handle_reaches_the_envelope_and_the_flag_beats_it` |
| `pause.rs:31:9` ×2 · `:104:9` ×2 | `kind` → `""` / `"xyzzy"` | `pause::tests::each_resolver_reports_its_own_kind` · `the_resolved_hold_records_the_answering_arm` |

Both **security-plan §Input Validation** sites (`paths.rs:73`, `paths.rs:130`) end KILLED, as that domain requires.

### Accepted-deliberate (2)

| site | mutation | rule |
|---|---|---|
| `render.rs:161:5` | `stdout_color` → `false` | test-plan §12 (2026-09-04, `sidecar_resolves_on_path`): the arm is a read of the HOST's own state — a process whose stdout is a real terminal. Every runner captures it, so the only kill is host-dependent, and a roster must mean the same thing on every host (§10 zero-flakiness). The three-condition LOGIC is killed through the pure `color_enabled` seam; what survives is the wrapper's `is_terminal()` read alone. |
| `render.rs:171:5` | `stderr_color` → `false` | same, for stderr. Its `-> true` twin IS killed (a piped subprocess must carry no escape byte), which is what shows the acceptance is scoped to the unobservable direction rather than to the function. |

**Rejected, and recorded as part of the disposition:** an env override (a `CONDUCTOR_COLOR`-class handle) would make
both arms reachable — refused, because a new `CONDUCTOR_*` handle is a new occupied resource (architecture.md
§Occupied Resources) and the plan lists it under rejected approaches. `unsafe { set_var }` is likewise refused
(process-global, edition-2024 unsafe).

---

## W2 — `civil_from_unix`: 2 removed with their dead branch · 4 killed

### Removed (2)

| site | mutation | why it is a removal, not an acceptance |
|---|---|---|
| `obs.rs:224:41` ×2 | `z - 146_096` → `+` / `/` | The expression sat in the `else` arm of `if z >= 0`. `civil_from_unix` takes `secs: u64`, so `z = days + 719_468 ≥ 719_468` and the negative-era branch is unreachable for every possible input. Deleted (`let era = z / 146_097;`) with the domain stated in a comment — code that can be removed is never accepted. |

### Killed (4)

| site | mutation | killed by |
|---|---|---|
| `obs.rs:226:33` ×2 | `+ doe / 36_524` → `-` / `*` | `civil_from_unix_holds_at_the_era_boundaries::case_6_century_day` (`2100-03-01`, `doe = 36_524` — the first day the 100-year term is non-zero) and `case_5_before_the_century_day` |
| `obs.rs:226:48` ×2 | `- doe / 146_096` → `+` / `/` | `case_3_era_last_day` (`2000-02-29`, `doe = 146_096` — the only day of an era on which the 400-year term is non-zero) and `case_7_next_era_last_day` |

The case set was derived rather than guessed: each correction term is non-zero on exactly one class of date, so an
ordinary timestamp exercises the expression while separating nothing. `caught.txt` carries 172 `civil_from_unix`
mutants; `missed.txt` carries none.

**Out of scope, surfaced not fixed:** the file-scoped run covers all 203 `obs.rs` mutants where the audit's
`--shard 1/4` reported 126, so it surfaced **7 survivors the audit never named** — `install_panic_hook`,
`ObsWriterGuard::flush`, `unix_millis` ×2 and three `JsonVisitor::record_*` arms. None is in `civil_from_unix`;
all belong to the untested `conductor-core` shards 2–4 that this chunk's scope explicitly leaves for a future
audit. Recorded here so the next boundary inherits them rather than rediscovering them.

---

## W3 — `jsonrpc.rs`: 2 timeouts → 2 killed

| site | mutation | killed by |
|---|---|---|
| `jsonrpc.rs:49:30` | `!=` → `==` in `JsonRpcSession::request` | the bound, through every duplex test |
| `jsonrpc.rs:70:9` | `write_message` → `Ok(())` | same |

Both were HANGS, not unobserved returns: under `==` the response loop skips the matching reply and blocks in
`read_line`; under `Ok(())` nothing reaches the stub, so no reply ever arrives. A hang makes the runner report the
whole binary as a timeout, and the assertion that would have failed never speaks
(`.claude/rules/testing.md` 2026-09-03). `tests/common::bounded` wraps each read-back await at **5 s** — chosen to
fire well inside cargo-mutants' own per-test timeout (auto = 5× the baseline, floored at 20 s), because a bound AT
that floor races the harness and the kill is not credited.

**A file the plan had excluded turned out to be load-bearing:** step 2 judged `tests/jsonrpc_line_bound.rs` to
"terminate on a decode error and need no bound". False under both mutants — its stub answers only after reading a
request line, so `write_message → Ok(())` leaves it silent and `!= → ==` makes the client skip the one reply it
gets. The first verify re-run still reported 2 timeouts; bounding that file's two awaits took them to 0.

The 14 standing survivors are unchanged in count and kind (8 in the `stub_pulse_mcp` test-stub binary,
`client.rs:157`, `extract.rs` ×2, `manifest.rs:35`, and the 2 accepted `spawn.rs:116` arms).

---

## W4 — the three clone families

Measured with jscpd 5.0.16 in the audit's own form (`jscpd crates --format rust --reporters json --output <dir>
--silent`); full pair lists in `jscpd-before-pairs.md` / `jscpd-after-pairs.md`. Total pairs **90 → 84**.

| family | before | after | what landed |
|---|---|---|---|
| (a) `conductor-tauri/src/commands.rs` self-clones | 2 pairs (13 lines at 173/232, 12 at 409/508) | **0** | `resolve_run_target` hoists the `run_report` / `run_envelope` prologue; a test-side `request(cmd, body)` hoists the shared `InvokeRequest`. Each handler keeps its own `tauri.command.<name>` span guard and its own no-run log line. |
| (b) `conductor-cli/src/render.rs` ↔ `conductor-report/src/coverage.rs` | 3 pairs (9 + 8 + 8) | 2 pairs (8 + 8) | The 9-line PRODUCTION roll-up is gone: `conductor_report::coverage_rollup` counts the terms once and both renderers format them. The two remaining pairs are TEST-side — each crate asserting the per-mode counts sum to the row count over its OWN surface. Deduplicating those would make one crate's test depend on the other's; the parity is the point. |
| (c) `conductor-run/tests/{dispatch_wire,canary_wire,canary_obs_witness}.rs` | 6 pairs (21 + 13 + 8 + 7 + 9 + 7) | 2 pairs (9 + 7) | `tests/common/mod.rs` owns `Capture` / `start_stub` / `start_trace_stub`. Both remaining pairs are INTRA-file repetitions inside one binary's own assertions, not the cross-file scaffolding. `canary_obs_witness.rs` stays its own test binary (test-plan §11 — the process-global subscriber). |

`conductor-tauri`'s three accepted-deliberate survivors survive the prologue extraction unchanged in kind, at
`main.rs:18:5`, `commands.rs:273:8` (`delete !` in `start_run`) and `commands.rs:308:5` (`run_thread`); 0 new.

---

## W5 — the `conductor-run` split

`src/lib.rs` 1944 lines → **34** (crate docs + `mod` + `pub use`), over six siblings plus a `#[cfg(test)]`
`testkit.rs` holding the fixtures several modules share.

| module | owns |
|---|---|
| `canary.rs` | `Preflight` · `preflight` · `readiness` · `unreachable_state` · `canary_blocked_state` · `canary_gate` · `emit_canary` · `warm_up_canary_service` · `canary_poll` · `emit_canary_storm` · `canary_*_seed` · `canary_spec` · `CANARY_*` |
| `preconditions.rs` | `load_run_contract` · `observe_run_contract` · `declares` · `observe_preconditions` |
| `lifecycle.rs` | `LifecycleObservation` · `LifecycleVerdict` · `select_resolve_target` · `AUTO_RESOLVE_IDLE_SECONDS` · `attribute_by_liveness` · `evaluate_lifecycle` · `probe_resolve_lifecycle` · `active_incident_ids` |
| `execute.rs` | `execute_scenario` · `ScenarioOutcome` · `route_read_back` · `state_for` · `manual_record` · `severity_rank` · `now_ms` · `now_unix_nanos` · `classify_fault` · `ramp_factor` · `fault_span` · `phase_guard` |
| `envelope.rs` | `persist` · `read_envelope` · `classify_run` |
| `drive.rs` | `RunEvent` · `RunStage` · `drive_run` |

**Public API byte-stable.** `public-api-{before,after}.txt` (extracted from rustdoc's `all.html`) diff **identical**
at 27 items — nothing removed, nothing added — and `tests/composition_root.rs::public_api_paths_are_stable`
references every one by its `conductor_run::` path from outside the crate, so a future rename fails to compile
rather than silently falsifying architecture.md's prose citation of `conductor_run::observe_preconditions`.

**The accepted-deliberate roster moved, and identity was re-measured by function + column** (§12's own method),
never by line arithmetic:

| test-plan §12 (pre-split) | post-split | the mutant |
|---|---|---|
| `lib.rs:68:8` (class C) | `canary.rs:58:8` | `delete !` in `preflight` |
| `lib.rs:559:27` (class B) | `execute.rs:95:27` | `delete field degraded from Observation` |
| `lib.rs:600:25` ×2 (class B) | `execute.rs:136:25` ×2 | `observed_ms - emitted_ms` → `+` / `/` |
| `lib.rs:362:5` (`declares`) | `preconditions.rs:50:5` | `declares` → `false` |

The tier found **118 mutants both before and after**, which is what makes the two scores comparable; 5 missed = the
accepted set exactly, 0 new, 0 timeouts.

**Test partition.** The inline module's 52 items became 43 tests plus 9 shared fixtures. Every test whose subject is
a crate-private item stayed a unit test inside the module that now owns the item; only genuinely public behaviour
moved to `tests/composition_root.rs`. That is fewer relocations than the plan predicted — `blocked_preflight()`
constructs `Preflight`'s crate-private fields, so the `execute_scenario` / `drive_run` / envelope tests cannot move
to `tests/` without widening the public API, which the split exists to hold fixed.

**Obs identity unchanged.** `target-values-{before,after}.txt`: the three `conductor_run` lines became
`conductor_run::canary` (1) and `conductor_run::execute` (2) — `target` is the emitting module path (obs-plan §3),
so it moves with the code by design. Span NAMES and topology are byte-identical: `scenario.run` opens at
`execute_scenario`, `report.generate` and `db.insert_run` remain run-scoped siblings correlated by `run_id`, and
every name stays inside the bounded set. `tests/severity_harvest.rs:454` pinned the old literal and was relaxed to
the crate prefix, which is the half that carries the claim.

---

## W6 — the standing cargo-audit deferral

The pin's own two-step probe, run in the form the plan lists:

```
DB="${CARGO_HOME:-$HOME/.cargo}/advisory-db"   →  <cargo-home>/advisory-db
git -C "$DB" rev-parse HEAD                    →  5a0ebedfe8bdd2e295b171f4162f8c977bcad9a5
git -C "$DB" status --porcelain                →  (empty — no untracked residue)
cargo audit                                    →  exit 0 · 564 crate dependencies · 18 allowed warnings
cargo deny check advisories bans licenses sources → exit 0 (advisories ok, bans ok, licenses ok, sources ok)
```

Exit codes read from the bare commands, output redirected into the gitignored `target/`. This is the reading the
migrated pin predicted, and the deferral's closure conditions are met — the wrap closes it (route-resolve
§Deferred-gate closure). `Cargo.lock` is untouched at 564 packages and no crate manifest changed.
