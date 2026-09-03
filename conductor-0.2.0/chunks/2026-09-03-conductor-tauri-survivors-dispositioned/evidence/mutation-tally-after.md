# Mutation tally — `conductor-tauri`, after dispositioning

## Firing form and cost

```
cargo mutants -p conductor-tauri --test-tool=nextest --jobs 2 --output target/mutants-2026-09-03
```

| fact | value |
|---|---|
| started / finished | 2026-09-03T08:30:32Z → 2026-09-03T08:34:27Z |
| **elapsed** | **3 m 55 s** |
| budget basis | ~4 m 18 s (the previous chunk's measurement), not the 5400 s the plan before it guessed |
| process exit | 2 |
| output dir | **fresh** — `target/mutants-2026-09-03/` did not exist before this run |

The output directory was verified absent before the run. A pre-existing `mutants.out/` from the
2026-08-21 era still sits at the repo root; it is **not** this run's artifact and was not read.
`cargo mutants` always creates `mutants.out/` *inside* `--output`, so the tallies below come from
`target/mutants-2026-09-03/mutants.out/` (`.claude/rules/testing.md` 2026-09-03).

Exit 2 carries no verdict in either direction (test-plan §4; a fully-accepted run has measured exit 3
before). The tallies are the signal.

`-p conductor-tauri` was kept rather than `-f`: the package holds exactly three `.rs` sources and
`build.rs` is not mutated, so `-p` already scopes to what `-f` would name, and keeping it preserves
comparability with the 43-mutant basis. **`Found 43 mutants to test` — the same 43**, which is what
makes the two runs directly comparable: production code is byte-unchanged.

## The movement this chunk owns

| | missed | caught | unviable | timeout | tested |
|---|---|---|---|---|---|
| before — implement run (2026-09-03T05:11Z) | 19 | 6 | 15 | 3 | 43 |
| before — wrap light gate (same tree) | 18 | 6 | 16 | 3 | 43 |
| **after — this run** | **3** | **25** | **15** | **0** | **43** |

**22 standing survivors → 3, and all three are the dispositioned accepted-deliberate set.**
Timeouts went to **zero**: the three that hung were an unbounded `rx.await`, not an unobserved
return, and bounding two existing awaits converted them to caught.

The previously-flip-flopping 22nd mutant (`pause.rs:85 … kind -> "xyzzy"`, which classified missed on
one prior run and unviable on another over an identical tree) was **viable and caught** on this run,
so `kind`'s two mutants are both killed and the disposition unit held.

## `missed.txt` — exactly the three accepted-deliberate

```
crates/conductor-tauri/src/main.rs:18:5: replace main with ()
crates/conductor-tauri/src/commands.rs:263:8: delete ! in start_run
crates/conductor-tauri/src/commands.rs:296:5: replace run_thread with ()
```

`timeout.txt` is empty.

## `caught.txt` — the 19 newly killed (plus the 6 already caught)

```
main.rs:44        tauri_log_path -> None
main.rs:44        tauri_log_path -> Some(Default::default())
commands.rs:38    resolve_handle -> Ok(Default::default())
commands.rs:44    scenarios_dir  -> Ok(Default::default())
commands.rs:48    runs_dir       -> Ok(Default::default())
commands.rs:52    manifest_path  -> Ok(Default::default())
commands.rs:81    resolve_selection -> Ok(vec![])
commands.rs:81:18 replace == with != in resolve_selection
commands.rs:89    load_all           -> Ok(vec![])
commands.rs:113   list_scenarios     -> Ok(vec![])
commands.rs:125   list_scenarios_impl-> Ok(vec![])
commands.rs:168   run_report         -> Ok(vec![])
commands.rs:224   run_envelope       -> Ok(None)
commands.rs:260   start_run          -> Ok(())
pause.rs:66       HoldGate::deliver  -> true
pause.rs:66       HoldGate::deliver  -> false
pause.rs:85       TauriResolver::kind -> ""
pause.rs:85       TauriResolver::kind -> "xyzzy"
pause.rs:105      resolve_operator_hold -> Ok(())
```

Already caught before this chunk and still caught: `commands.rs:134` `coverage_matrix`,
`commands.rs:150` `unbacked_auto` (×3), `commands.rs:336` `stop_run`, `pause.rs:58` `HoldGate::arm`.
6 + 19 = 25.

## Gates green alongside

| gate | result |
|---|---|
| `cargo nextest run -p conductor-tauri` | 26/26 (was 12) |
| `cargo test -p conductor-tauri` | 26/26 — runner portability, no pin, no retries |
| `cargo nextest run --workspace --profile ci` | 781/781, exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `bash scripts/agent-run.sh status` | exit 0, well-formed envelope |
| host-path hygiene grep | no matches (exit 1) |
| `cargo audit` | exit 1 — signature reproduced, the 46th re-pin |
| `cargo deny check advisories bans licenses sources` | exit 0 — advisories ok, bans ok, licenses ok, sources ok |

Dependency delta: `tokio`'s **dev**-dependency gained the `time` feature. `Cargo.lock` holds at
**564 → 564 packages** and is in fact byte-unchanged, because a feature set is not recorded in the
lockfile. The basis stated is the package COUNT plus `cargo deny` green, never byte-identity
(security-plan §Dependency Security, 2026-09-02).
