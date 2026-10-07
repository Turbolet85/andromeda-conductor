# Codebase Research — 2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive

## Scope
- **Depth:** moderate · **Reads:** 21 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole, Session Additions included
  (it loaded on the first read under `crates/conductor-run/tests/`); 9 additions applied: 2026-08-19 (a) no `boot`
  before a drive; 2026-08-20 a `[BLOCKED]` in about 0 s is sidecar resolution; 2026-09-02 the census taken twice
  with a stop form beside the firing form; 2026-09-10 a live leg's `expect` atoms come from a printed line;
  2026-09-23 as corrected 2026-10-01 a no-incident outcome is read from the skip line; 2026-09-30 one re-fire of a
  harness defect on the overseer's word; 2026-10-03 as corrected 2026-10-06 the 10 s liveness check and the one
  recorded relaunch; 2026-10-06 a binary proof needs a two-sided control (not owed here: no build); 2026-10-07 the
  shared growing data dir confounds a drive's ordinal with what accumulates there, and the capture keeps its
  `creating digest corpus retrieval rows:` line. `.claude/rules/testing.md` loaded the same way; applied:
  2026-09-07 a nextest selector that selects nothing, 2026-10-04 child modules under `tests/<name>/`.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the phase relay: the founder picked the operator's wrapper; the leg discloses and reads nothing
    under the capture folder.
  - `inputs#I2` — the invocation directive: vehicle (A); the go before the launch; the sitting ends in daylight.
  - `inputs#I3` — the wrap relay: three drives, one fresh dir, the fifth series' design, a capture and no verdict.
  - `inputs#I6` — Pulse `pulse-app/src/llamacli_inference.rs` at its committed state (pointer; `claimed f70be92:
    same`).
  - `inputs#I8` — the overseer's P5 review answer: the daylight bound; the three operator files are not
    committed; the pass-through restores the caller's umask before `exec`.
  - Not snapshotted, on that answer: the pass-through and the two env files. They were read in place and are
    recorded by sha256 and byte count in §The operator's files. Their first snapshots (ids I4, I5 and I7) were
    removed from `inputs/` before any commit; those ids are not reused.

## Files inspected
- `contracts/pulse-real-model-leg-posture.md` (`:572-721`, and the heading index) — the fifth series' section is
  the form the new record follows; the next heading after it is `## The quiet window and serialization` (`:687`),
  so the new section lands between the two. The contract names 6 distinct handles (`grep -oE
  '(ANDROMEDA|CONDUCTOR|PC)_[A-Z0-9_]+' … | sort -u`): the data dir, the L4 flag, the MCP flag, the bootstrap
  override and the two model handles. It names neither `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` nor any `PC_` name.
- `crates/conductor-run/tests/real_model_grading/mod.rs` (full) — `Series` (`:29`) is the one shape a dated run
  grades through; `graded()` (`:124`) and `v3_09_met` (`:139`) are separate, so a module can use every `Series`
  method and never state a verdict. `contract_section` (`:144`) and `pre_registered` (`:153`) take the heading as
  an argument.
- `crates/conductor-run/tests/real_model_grading/series_2026_10_07.rs` (full) — the module to mirror, minus its
  last test (`v3_09_is_not_met_by_the_2026_10_07_series`, `:111`) and its `v3_09_met` import (`:6`).
- `crates/conductor-run/tests/real_model_series/mod.rs` (full) — pins are labels, file names and sha256 digests;
  the fifth series' names are `EVIDENCE_2026_10_07` and `SERIES_2026_10_07` (`:124`, `:128`), so the capture run
  needs names that do not collide with them.
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` (full) — `COMMITTED_CAPTURES = 20` (`:9`),
  an exact count over a walk of every chunk's `evidence/` for `rm-capture*.txt`.
- `crates/conductor-run/tests/real_model_harvest.rs` (`:1236-1415`, and a grep of its registration sites) — `use`
  lines at `:66` and `:69-70`; `GRADING_MODULES: [(&str, &str); 12]` at `:1255`, held equal to the directory's
  `.rs` files by `the_capture_text_arm_scans_every_grading_module` (`:1382`); `graded_captures` (`:1304`) with one
  loop per run; the rule markers at `:73` and `:461`.
- `scripts/agent-run.sh` (grep of the real-model arm, `:157-209`) — the arm clears `rm.jsonl`, `rm-capture.txt`
  and `rm-capture.err` at its head (`:179`) and prints `[live] leg rm: froze …` (`:86`) and the closing line
  (`:209`); nothing in it reads a model handle.
- `runs/live-suite/rm.jsonl` (the fifth series' d3 stream, local and git-ignored; first and last line) — 271
  lines; first `timestamp_ms` 1791360282989, last 1791360644458; each line holds exactly one `"timestamp_ms":`
  key with no space before its value.
- The fifth series' chunk folder: `plan.md` (full), `scope.md` (full), `evidence/attempt-ledger.md` (full),
  `evidence/round-074555Z.txt` (full), `evidence/rm-capture-d1.txt` (`:396-536`) — the plan's step order and gate
  fence, the ledger's row form, and the line classes a capture prints.
- `.andromeda/runs/2026-10-07T11-44-55-wrap/adaptation-record.md` (full) — the entry's origin and the escalation
  that left the vehicle open.
- Pulse `pulse-app/src/llamacli_inference.rs` at HEAD (inputs#I6; `:262-312`, `:448-515`, `:852-866`, and a grep) —
  see §The pass-through hypothesis.
- The operator's three files, read whole in place (§The operator's files); the pass-through read again after the
  overseer changed it.

## Graph impact (from the code-graph query; rust plane, 25 rows)
- **pre_registered** — 4 calling tests: `series_2026_09_30.rs:23`, `series_2026_10_01.rs:25`,
  `series_2026_10_06.rs:25`, `series_2026_10_07.rs:25` (the other four rows are the modules' `use` lines). The
  capture run's digest test is a fifth caller. No signature changes.
- **v3_09_met** — 5 calling tests, one per series module (`series_2026_09_29.rs:124`, `series_2026_09_30.rs:119`,
  `series_2026_10_01.rs:145`, `series_2026_10_06.rs:131`, `series_2026_10_07.rs:131`). The capture run's module
  adds none: it does not import the function.
- **graded_captures** — 2 callers, `real_model_harvest.rs:1374` and `:1401` (the capture-text arm and its inverse
  control). A new loop in it brings the run's captures under both.
- **committed_captures** — 1 caller, `capture_population.rs:53`. The walk finds the new files by name; only the
  count constant moves.
- **capture_2026_10_07** — 2 callers (`series_2026_10_07.rs:94`, `real_model_harvest.rs:1327`) and the `use` at
  `real_model_harvest.rs:66`: the registration shape the new accessor repeats.

## Patterns detected
- **A dated run is one child module plus four registration sites** (`real_model_grading/mod.rs:13-17`;
  `real_model_harvest.rs:66`, `:69-70`, `:1255-1301`, `:1304-1331`): a `pub(crate) mod` line, a `use` of its
  capture accessor, its two constants in the `use real_model_series` list, a `GRADING_MODULES` row, a
  `graded_captures` loop.
- **The verdict is one test, apart from the grading** (`series_2026_10_07.rs:111-134`): the six tests above it
  hold the pre-registration, the pins, the recorded rule, the scrub, the measured table and the envelopes; only
  the seventh calls `v3_09_met`.
- **The measured table is written red before green** (the fifth series' ledger `:202-207`): each value is read
  from the grading arm's own failure output, never judged.
- **A round is the plan's `round` and `live` entries fired per drive** (the fifth series' `round-074555Z.txt`):
  the tool fires the round steps since the previous leg, then the drive; its entry lines and summary are kept in
  `evidence/`, the outputs in the run dir's trail.

## Conventions to follow
- **The ledger records names, never values**: handle NAMES, the dir's leaf, digests, counts, booleans (the fifth
  series' ledger `:5-7`).
- **Pulse probes run inside a subshell `cd`** with bare file names, so no entry prints a host path (the fifth
  series' plan, Test Commands).
- **The launch is a scratchpad script, not a committed one**: `setsid nohup`, cwd the data dir, one env block
  echoed back by name (the fifth series' ledger `:132-142`). A committed launch script would be a ninth governed
  spawn form.
- **Editor lines, not graph lines**: every `file:line` above is the graph's 0-indexed line plus one.

## The pass-through hypothesis (re-derived, scope item 8)
- Pulse's side (inputs#I6): `build_llama_cli_args` ends `--grammar-file`, the grammar path, `-p`, the prompt
  (`:452-494`); the readiness check admits a binary that `is_file()` (`:285`) and logs two
  `interpretation.model.load` lines with `tier`, `load_status` and, on the second, `model_identity` from the model
  file's stem (`:278-306`); the spawn is `Command::new(binary)` with `args`, `kill_on_drop(true)`, stdin null and
  stdout and stderr piped (`:858-863`). `grep -n -E '\.env\(|env_clear|env_remove|current_dir'` over the file: 0
  hits. `git diff --quiet f70be92 HEAD -- pulse-app/src/llamacli_inference.rs`: exit 0.
- Both hardware routes resolve to one binary: `binary_target_for_profile` (`:503-512`) picks the CUDA or the CPU
  handle, the plain env sets both to the same `llama-cli` build, and the capture env sets both to the
  pass-through, whose `exec` target is that build (P3 probe, repeated after the change: the script's target line
  equals the operator's real-binary variable, count 1).
- The pass-through's side: `/bin/sh` is `bash` on this host; the script records before it runs `exec "$REAL"
  "$@"` and discards every write error. As first read it set `umask 077` and left it; as re-read at 14:20 local
  it saves the caller's umask, sets 077 for its own writes and restores the saved value on the line before
  `exec` (inputs#I8).

## The operator's files (read in place; not committed, inputs#I8)
Paths are relative to the overseer's own project folder. Read 2026-10-07 14:20:24 local (12:20:24Z), after the
overseer's change to the pass-through; `sha256sum` and `stat -c %s`.

| File | sha256 | Bytes | Modified (local) |
|---|---|---|---|
| `l4-shim/llama-cli` (mode `rwxr-xr-x`) | `66dc9cc47922021899e7c747db9fb24e1547155348dc7095f75baf1acb40c742` | 1230 | 2026-10-07 14:19:27 |
| `l4-env-capture.sh` | `59d167a3385b8f14126edada3176605ff164d08ae9cc4fcf5efae8b5b80cb2c7` | 1016 | 2026-10-07 13:49:53 |
| `l4-env.sh` | `0bc0706d2e27f3137a402a523e7f1fa7b8c7cad736011143ef7f45ff9cabbe2b` | 1080 | 2026-10-05 18:23:40 |

- The pass-through as first read at P3 (before the change): sha256 `d6ea53ec…b310`, 1192 B.
- `l4-env.sh` hashes equal to the copy the fifth series' chunk holds in its own `inputs/`.
- Both pre-leg handle entries were re-fired after the change: exit 0 under the capture env; the posture entry
  exits 1 under the plain env.
- The equality the plan needs, stated at its grain: for THIS run the bytes of `pulse-app` equal the fifth series'
  (`sha256sum`: `df1676477222776d3e95edae7d219a4d421f2311ea8f17863233630c1ed8ba4a`, the fifth ledger's `:61`), so
  the argv builder, the grammar and the prompt code are identical; what differs is one hop in the spawn. A
  generation that parses under the grammar with `prompt_version` `v2.6` shows the real binary ran behind that hop
  and answered in form. It does not show the argv arrived byte for byte; nothing on Conductor's side can.

## The binaries and the host (measured at P3, 2026-10-07 14:05 local)
- `pulse-app` sha256 `df167647…ba4a`, built 2026-10-07 09:40:59 +02:00; `andromeda-pulse-mcp` sha256
  `6175fc36…d2a9`, built 09:41:15 +02:00. Both equal the fifth ledger's rebuilt digests.
- Pulse: `merge-base --is-ancestor f70be92 HEAD` exit 0; `status --porcelain -- crates pulse-app Cargo.toml
  Cargo.lock` prints one line, the modified example. So the fifth series' clean-build-inputs probe would read red
  today; it guarded a build, and none is planned.
- The model file: 4 977 171 584 B, as the fifth series recorded. Its sha256 is read at the pre-leg check.
- NVIDIA: `kernel=610 userspace=610`.
- Census: 0 matching processes, 0 listeners on `:4317` or `:4318`.
- `~/.cache/pulse-legs/` holds `rm-fifth-series` and `rm-trigger-series`.
- Counts at the chunk base: 9 `.rs` files carry `andromeda:walks-tree` (`grep -rl … crates | wc -l`); 562 packages
  (`grep -c "^name = " Cargo.lock`).

## New files to create
- `conductor-0.3.0/chunks/2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive/evidence/` — the attempt ledger, the three captures and the round listings
- `crates/conductor-run/tests/real_model_grading/capture_run_2026_10_07.rs` — the run's grading module: the pre-registration digest, the pins, the recorded rule, the scrub, the measured table, the envelopes; no verdict test

## Files to modify
- `contracts/pulse-real-model-leg-posture.md` — one add-only section before the quiet-window section
- `crates/conductor-run/tests/real_model_series/mod.rs` — the run's evidence path and its three pins
- `crates/conductor-run/tests/real_model_grading/mod.rs` — one module line
- `crates/conductor-run/tests/real_model_harvest.rs` — the four registration sites, nothing between the rule markers
- `crates/conductor-run/tests/real_model_grading/capture_population.rs` — the population count

## Open questions
- none. (Where "daylight" ends for this sitting was open at P3; the overseer ruled it at the P5 review: the
  sitting ends by 19:00 local, no go request after 18:25 — inputs#I8.)
