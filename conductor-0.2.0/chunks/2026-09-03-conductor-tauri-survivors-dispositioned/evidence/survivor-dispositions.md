# Survivor dispositions — `conductor-tauri`

The exact set of standing survivors from the tier's first score
(`2026-09-02-mutation-tier-restored-for-conductor-tauri/evidence/mutation-tally.md`), each ending
**killed** or **accepted-deliberate against a cited standing rule** (test-plan §10). No numeric
`--fail-under` and no CI gate was introduced; the score moved as a consequence.

This is an **exact-set ledger in both directions** (`.claude/rules/testing.md` 2026-08-09): a survivor
outside it, or a listed row that no longer survives, is a defect in the ledger. It can only shrink
under compulsion.

**22 rows** — the 21 stable survivors plus the `kind -> "xyzzy"` mutant that classified missed on one
prior run and unviable on another over an identical tree. Both `kind` mutants are one disposition
unit; on this chunk's verifying run `"xyzzy"` was viable and caught.

## Killed — 19

Each row states **what its test actually pins**, not what a domain criterion hopes it pins.

| # | survivor | killed by | what the test actually pins |
|---|---|---|---|
| 1 | `commands.rs:38` `resolve_handle -> Ok(Default::default())` | `commands::tests::resolve_handle_rejects_a_traversal_default_naming_the_traversal` + `…_resolves_an_in_scope_default_under_the_current_dir` | **Traversal rejection proper** — the ONLY row in this ledger that does. Its `base` is the process CWD, which exists, so `resolve_under` canonicalizes it and reaches the `..` guard. The test asserts the error names the traversal (`component`) and is **not** the absent-base error. |
| 2 | `commands.rs:44` `scenarios_dir` | `commands::tests::the_artifact_handles_resolve_absolute_paths_under_the_current_dir` | The handle returns an **absolute** path **starting with the canonicalized CWD**. Env-robust: it holds for any value `CONDUCTOR_SCENARIOS_DIR` may carry. The mutant's empty `PathBuf` is neither. |
| 3 | `commands.rs:48` `runs_dir` | same test | as above, for `CONDUCTOR_RUNS_DIR` |
| 4 | `commands.rs:52` `manifest_path` | same test | as above, for `CONDUCTOR_CONTRACT_MANIFEST` |
| 5 | `commands.rs:81` `resolve_selection -> Ok(vec![])` | `…::resolve_selection_loads_the_whole_suite_for_the_sentinel` | The sentinel selects **every** fixture scenario (2), which an empty vec cannot be. |
| 6 | `commands.rs:81:18` `replace == with !=` | the sentinel test **paired with** `…::resolve_selection_loads_exactly_one_for_a_named_scenario` | The two branches differ in **cardinality** (2 vs 1). One test alone could not discriminate the swap; the pair can. |
| 7 | `commands.rs:89` `load_all -> Ok(vec![])` | `…::load_all_reads_every_committed_fixture_scenario` | The whole fixture directory loads (2 scenarios). |
| 8 | `commands.rs:113` `list_scenarios -> Ok(vec![])` | `…::list_scenarios_errors_when_the_capability_manifest_is_unreachable` | The command **errors** under the crate-dir CWD, because `capabilities()` cannot find `contracts/pulse-capabilities.toml`. Not a catalog assertion — an error-vs-success one. `scenarios_dir()` itself resolves fine. |
| 9 | `commands.rs:125` `list_scenarios_impl -> Ok(vec![])` | `…::the_committed_scenarios_fixture_stays_loadable` | The fixture directory yields exactly 2 summaries carrying both fixture identities. Doubles as the fixture's **meaning pinner** (test-plan §7). |
| 10 | `commands.rs:168` `run_report -> Ok(vec![])` | `…::run_report_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir` | **NOT the traversal guard.** `runs_dir()` succeeds (its candidate need not exist) and returns `<crate>/runs`, which does not exist — so the inner `resolve_under` fails at `base.canonicalize()` *before* the `..` check. What is pinned: *a supplied `run_id` against an unresolvable runs dir errors*. The mutant's `Ok(vec![])` cannot error. |
| 11 | `commands.rs:224` `run_envelope -> Ok(None)` | `…::run_envelope_errors_on_a_supplied_run_id_with_an_unresolvable_runs_dir` | Same mechanism as row 10. `Ok(None)` is a **success**, and is also the *correct* render for an in-envelope run — which is why the kill had to come from the error arm, not from asserting the banner is present. |
| 12 | `commands.rs:260` `start_run -> Ok(())` | `…::start_run_errors_before_spawning_a_run_thread` | The command errors at `capabilities()` — an **early return**, so no background thread is spawned and the abort flag stays untouched. Called directly as a plain fn (the two `Channel` args constructed in-test), which exercises the same mutated body without IPC channel deserialization. |
| 13-14 | `pause.rs:66` `HoldGate::deliver -> true` / `-> false` | `pause::tests::gate_delivers_the_decision_to_the_awaiting_hold` (now bounded) + `…::deliver_is_a_no_op_without_a_pending_hold` | `-> true` fails the no-pending-hold assertion; `-> false` fails `assert!(gate.deliver(...))`. **Both were TIMEOUTS before**: the round-trip test's `rx.await` blocked forever when the mutant stopped the oneshot being sent, timing out the binary before any assertion could report. A 5 s bound turns the hang into a named failure. |
| 15 | `pause.rs:105` `resolve_operator_hold -> Ok(())` | `pause::tests::resolve_operator_hold_command_delivers_the_decision` (now bounded) | Same conversion: the dispatched command that never delivers now fails fast instead of hanging. |
| 16-17 | `pause.rs:85` `TauriResolver::kind -> ""` / `-> "xyzzy"` | `pause::tests::the_tauri_resolver_identifies_itself_as_the_dialog_resolver` | The resolver's identity string is `"tauri-dialog"`. Nothing asserted it before, which is why both mutants survived. One disposition unit. |
| 18-19 | `main.rs:44` `tauri_log_path -> None` / `-> Some(Default::default())` | `tests::the_self_obs_sink_resolves_beside_the_runs_dir` + `tests::the_obs_sink_is_a_file_sink_whenever_the_path_resolves` | The sink path is absolute and ends (component-wise) with `logs/conductor-tauri.jsonl`, and `obs_sink()` yields `ObsSink::File`, never the stderr fallback — obs-plan §3's **unconditional** file sink. `main.rs` carried no test module at all before this chunk. |

## Accepted-deliberate — 3

Each cites a standing rule that **prescribes** the untested shape. None is a deferral.

| survivor | cited rule | why the rule prescribes it |
|---|---|---|
| `main.rs:18` `replace main with ()` | `architecture.md` §Design Philosophy — "Headless-drivable core, thin shells" | `main` may hold no engine logic; it wires the app and enters the Tauri event loop. No `main` that launches that loop can be called from a test, and **no refactor changes this** — an extraction would move the testable part out and leave `main` a thinner unkillable shell. The same `generate_handler!` list is already asserted to build by `commands::tests::mock_app_registers_the_full_command_surface`. **Operator-ratified at this chunk's P4.** _Known limit: `test_app` duplicates `main`'s handler list, so the two can drift — the parallel assertion is a mirror, not a proof of identity._ |
| `commands.rs:296` `replace run_thread with ()` | `.claude/rules/testing.md` 2026-06-26 | Names this exact shape — a Tauri command driving the async engine on a background thread — as not deterministically testable in-process against the zero-retry bar, and prescribes testing the library fn `conductor_run::drive_run` instead, which `conductor-run` does. |
| `commands.rs:263:8` `delete ! in start_run` | `.claude/rules/testing.md` 2026-08-10 | Reaching the inverted guard needs `capabilities()` **and** `list_scenarios_impl()` to succeed first, and both resolve against the process CWD — the crate dir under `cargo test`, where neither `contracts/` nor `scenarios/` exists. The rule's remedy is to read the environment at the caller and pass it in as a typed value; that is a production refactor this chunk's scope bans, and it is the same env/CWD-reading-edge shape test-plan §10 already cites as the shipped accepted-deliberate case (`declares` ×6). |

## Adjacent finding — recorded, not fixed

The doc comments at `commands.rs:164` and `:221` say an absent runs dir yields an empty list / `None`
and "never an error". That holds for **`run_id = None`** — `latest_run_id` returns `Ok(None)` when
`read_dir` fails (`conductor-core/src/run_journal.rs:14-15`) — but **not** for a supplied id, which
reaches `resolve_under` against a base that does not exist and errors. The two commands behave
correctly; the comments read wider than they are. Production code is unchanged this chunk, so this
is carried forward rather than fixed.
