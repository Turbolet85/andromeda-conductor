# Mutation tally — `conductor-tauri`, the tier's first score

## Firing form and cost

```
timeout 5400 cargo mutants -p conductor-tauri --test-tool=nextest --jobs 2 --output mutants.out
```

| fact | value |
|---|---|
| started / finished | 2026-09-03T05:11:45Z → 05:16:03Z |
| **elapsed** | **4 m 18 s** |
| budget | 5400 s (90 min) — the ceiling was never approached |
| process exit | 3 |

**Record the elapsed figure, not the budget.** The plan's 5400 s was an explicit budget derived from
the run's shape, never a measurement, and it over-estimated by ~20×. The next chunk to touch this
tier should budget from **~4.5 minutes**, not from the guess. (Exit 3 carries no verdict either way —
test-plan §4; a fully-accepted run has measured exit 3 before. The tallies below are the verdict.)

## The movement this chunk owns

| | mutants planned | mutants **tested** |
|---|---|---|
| before (audit, 2026-09-02) | 43 | **0** — exit 4, `cargo test failed in an unmutated tree` |
| after (this chunk) | 43 | **43** |

The unmutated-tree baseline passes, so the instrument runs. That is the chunk's route-entry
acceptance and it is met.

## Tallies (read from `mutants.out/mutants.out/`, this run)

```
43 mutants tested in 4m: 19 missed, 6 caught, 15 unviable, 3 timeouts
```

**A freshness trap sits directly on this path and nearly produced a false green.** `cargo mutants`
always creates a `mutants.out/` directory *inside* the `--output` path, so `--output mutants.out`
writes the real tallies to **`mutants.out/mutants.out/`**. The repo root already carried a
`mutants.out/` from a run dated **2026-08-21** (the verifier-self-hardening era), whose `missed.txt`
is empty and whose `caught.txt` holds 38 entries. Reading the outer files — the obvious path, and the
one the plan's own command line implies — reports **0 missed / 38 caught, a clean sweep**, when this
run in fact has 19 missed. The two were separated only by mtime. Verify the artifact, not the exit
code, and not the path you expected the tool to use.

### Caught (6) — what the shipped mock-runtime tier already kills

```
commands.rs:134  coverage_matrix -> Ok(vec![])
commands.rs:150  unbacked_auto   -> Ok(vec![])
commands.rs:150  unbacked_auto   -> Ok(vec![String::new()])
commands.rs:150  unbacked_auto   -> Ok(vec!["xyzzy".into()])
commands.rs:336  stop_run        -> Ok(())
pause.rs:58      HoldGate::arm   -> ()
```

The pattern is legible: every command whose test asserts on returned CONTENT is protected; the
`unbacked_auto` triple in particular shows a content assertion killing three distinct value mutants.

### Second measurement — the wrap light gate, fresh output dir

The wrap re-ran the tier with `--output` pointed at a **fresh** directory (applying this file's own
finding). Baseline passed again, 43 tested again — but the tally moved:

```
implement run : 43 tested — 19 missed,  6 caught, 15 unviable, 3 timeouts
light gate    : 43 tested — 18 missed,  6 caught, 16 unviable, 3 timeouts
```

Diffed, the delta is **one mutant over an identical tree**:
`pause.rs:85:9 replace <impl PauseResolver for TauriResolver>::kind -> &'static str with "xyzzy"` was
**missed** on the first run and **unviable** on the second. Its sibling (`… with ""`) was missed on
both, the timeout set is byte-identical, and `caught` is unchanged at 6. So cargo-mutants' viability
classification is not fully run-stable here, and the honest headline is **21 stable survivors, with a
22nd that appears when that mutant builds**. Treat `kind`'s two mutants as one disposition unit.

### Missed and Timeout — the standing survivors (19 + 3 as first measured; 18 + 3 on re-run)

| file | count | survivors |
|---|---|---|
| `commands.rs` | 14 | `resolve_handle:38` · `scenarios_dir:44` · `runs_dir:48` · `manifest_path:52` · `resolve_selection:81` (+ `== → !=` at `81:18`) · `load_all:89` · `list_scenarios:113` · `list_scenarios_impl:125` · `run_report:168` · `run_envelope:224` · `start_run:260` (+ `delete !` at `263:8`) · `run_thread:296` |
| `pause.rs` | 5 | `TauriResolver::kind:85` (`""`, `"xyzzy"`) · **timeout** `HoldGate::deliver:66` (`true`, `false`) · **timeout** `resolve_operator_hold:105` |
| `main.rs` | 3 | `main:18` · `tauri_log_path:44` (`None`, `Some(Default::default())`) |

### Domain classification — which may NOT be accepted-deliberate

The plan named three domains that admit no accepted-deliberate disposition. The survivor set hits
two of them squarely:

- **security-plan §Input Validation — the `#[tauri::command]` boundary-guard wiring.**
  `resolve_handle:38` IS the traversal guard: a mutant returning `Ok(Default::default())` bypasses
  `resolve_under` entirely, and `scenarios_dir:44` / `runs_dir:48` / `manifest_path:52` are its three
  callers. `run_report:168` and `run_envelope:224` are the `run_id`-class commands the same row
  governs. **Six survivors, all must be killed.**
- **layout-templates §desktop-webview Component — Primary content block 2.** `run_envelope:224`
  returning `Ok(None)` would let the load-envelope banner silently never render — the qualifier
  losing its always-rendered label. **Must be killed** (it double-counts with the security row).
- **obs-plan §3 Log file location.** `tauri_log_path:44` returning `None` would silently drop the
  Tauri backend's self-obs sink. Adjacent to the obs domain rather than inside its span-guard clause,
  but it is not a survivor anyone should accept quietly.

### Why they survive, and what killing them costs

The missed set is not a scattering of weak assertions — it is one coherent gap. Every survivor is a
function whose real behavior the mock-runtime tier never observes with a populated subject:

- the **path helpers** (`resolve_handle` and its three callers) resolve `CONDUCTOR_*` handles against
  the process CWD, which under test is the crate directory — so the genuine call and the
  `Ok(Default::default())` mutant are equally unobservable without a staged repo root;
- the **catalog commands** (`list_scenarios`, `list_scenarios_impl`, `load_all`, `resolve_selection`)
  read `scenarios/`, absent for the same reason — an empty return is indistinguishable from the
  mutant's empty return;
- the **run-data commands** (`run_report`, `run_envelope`) are tested exactly at their *empty* case
  (`run_envelope_command_returns_null_when_no_run_has_an_envelope`), which is precisely the value the
  mutants return — the existing tests assert the mutant's own answer;
- `start_run` / `run_thread` spawn a background thread whose frame sequence is the display-gated
  deferral (test-plan §5, `.claude/rules/testing.md` 2026-06-26).

Killing them therefore requires **seeded fixtures inside `conductor-tauri`'s test module** — a staged
repo root plus a populated runs dir — which is the same shape the webview lamps fixture already uses
(`.claude/rules/testing.md` 2026-09-02: committed fixture content + a production-reader round-trip).
That is a body of work, not a handful of assertions.

## Scope finding

The chunk's **route-entry acceptance is met in full** — the tier yields a score, and the
runner-portability gate proves the test on a fresh target dir.

The plan's *second* acceptance criterion — "every NAMED survivor ends killed or classified
accepted-deliberate against a cited standing rule" (test-plan §10) — was authored when the survivor
set was **unmeasurable by construction**: the tier had never produced a score, so nobody could know
whether it would surface two survivors or twenty-two. It surfaced 22, spread across three files, and
**`pause.rs` and `main.rs` are outside this chunk's touchpoint list** (8 of the 22 live there).

The route already carries the precedent for the remedy: Epoch 6a's **second entry** is
*"conductor-run composition-root survivors dispositioned — every standing mutation survivor gains a
killing test or a cited accepted-deliberate entry"*, scoped to one crate. Disposition is treated
there as a chunk-sized unit, per crate, with its own split along survivor clusters.

Nothing here is dispositioned by default. No survivor was quietly classified accepted-deliberate to
close the criterion, and no numeric threshold was introduced — either move would be the evasion
test-plan §10 exists to forbid.
