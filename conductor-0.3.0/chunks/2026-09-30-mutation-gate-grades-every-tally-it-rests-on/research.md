# Codebase Research — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/testing.md:19` (the mutation bullet, whole line — its last sentence
  states the gate never reads `timeout.txt`); `host-win32.md` (always loaded — `-X utf8`, no `| tail` on a verdict).
  No live leg in this chunk: `verification-harness.md` is not a source for any gate here.
- **Platform issues consulted:** none — no runner-only bullet folded (CI `f33d6b7` was in progress, not red) and no
  CI-reading entry.

## Files inspected
- `scripts/mutation-gate.py` (full, 157 lines @ `f33d6b7`) — the whole instrument. The grading sits inline in `main()`
  (`:137-153`) AFTER an unconditional `subprocess.run(cmd, cwd=ROOT)` (`:122`), so no input reaches the grading
  without a real `cargo mutants` run. `parse_tally` (`:43-57`) is class-agnostic and reusable as is. `expected_for`
  (`:60-65`) reads the module constant `ROSTER` (`:37`), so a fixture roster cannot be substituted either.
- `scripts/mutation-roster.toml` (full, 155 lines @ `f33d6b7`) — 3 `[[unit]]` declarations, 13 `[[entry]]` rows, all
  of them `missed`-class by construction (the file has no field naming a tally class; re-derived: `grep -c '^\[\[entry\]\]'
  scripts/mutation-roster.toml` → 13). Row keys: `unit · file · mutation · class · member? · rule · citation_home`.
- `conductor-0.3.0/chunks/2026-09-14-emit-scrubber-and-percentile-math-under-test/evidence/disposition-ledger.md`
  (full) — the fifteen timeouts, by helper and shape only (`:97-102`); run 1 had sixteen, the sixteenth moved to
  `unviable` in run 2 on Windows link contention (`:104-106`, `:39-49`).
- `crates/conductor-emit/src/exception.rs` — `+=` sites only (`grep -nE 'fn (skip_…|normalize_frame)|\+='`): four
  helpers at `:258` / `:291` / `:320` / `:340`, with 1 / 2 / 3 / 4 `+=` advances in scope respectively.
- `mutants.out.old/` (repo root, git-ignored `.gitignore:63`, cargo-mutants 27.1.0, conductor-run, 2026-08-20) — the
  only surviving real cargo-mutants output on the host. Its `outcomes.json` and tallies are the format witness below.
- `scripts/arch-registry-check.py` `:1-30`, `:470-484` — the in-repo precedent for proving an operator instrument.
- `conductor-0.3.0/chunks/2026-09-24-architecture-registries-compacted-under-the-read-cap/plan.md:111-127`, `:233-239` —
  how that precedent's `selftest` was specified and gated.
- `.andromeda/test-plan.md:222-258` (§4), `:465-513` (§9-§10), `:619` (§12 `conductor-emit` member).
- `.gitignore:11`, `:59-63`.

## Graph impact
- **graph not applicable** (the change surface lies outside every indexed plane) — the modify-set is Python + TOML + data files. The code-graph indexes `rust` and `ts`
  only (health check 11: "planes rust, ts"), and no Rust or TS symbol is added, changed or called. No query was run.

## Patterns detected
- **The tally format is uniform across classes** (`mutants.out.old/`): `caught.txt`, `missed.txt` and `unviable.txt`
  lines are all `{path}:{line}:{col}: {mutation}` (e.g. `crates/conductor-run/src/lib.rs:127:5: replace
  canary_storm_seed -> u64 with 0`), and `timeout.txt` exists even when empty (0 bytes there). So `parse_tally`'s
  `SURVIVOR` regex (`:40`) applies to `timeout.txt` unchanged. The sample holds no non-empty `timeout.txt`, so that
  file's line shape is inferred from the tool writing one format for every class, not directly observed. The fixture
  proof below is independent of that inference.
- **`outcomes.json` carries its own per-class counts.** Top-level keys at 27.1.0: `total_mutants · missed · caught ·
  timeout · unviable · success · start_time · end_time · cargo_mutants_version` (read with `json.load` over
  `mutants.out.old/outcomes.json`). Measured equality on that artifact: each tally file's non-blank line count equals
  its count (missed 21 = 21 · caught 29 = 29 · timeout 0 = 0 · unviable 18 = 18, from `wc -l mutants.out.old/*.txt`),
  and `missed + caught + timeout + unviable = 21 + 29 + 0 + 18 = 68 = total_mutants`. (`success` is the unmutated
  baseline and sits outside the sum.) So the gate can check CONSERVATION against the tool's own count with no
  roster, which catches a truncated or missing tally file. It uses no host-flaky `unviable` roster; unviable counts
  only toward the sum.
- **An in-script `selftest` verb is the project's way to prove a Python operator instrument.** `arch-registry-check.py`
  ships `selftest`, which mutates real inputs in memory once per arm and asserts each arm reports. Its gate is
  `python -X utf8 scripts/arch-registry-check.py selftest …` with `expect = ['exit 0', 'last line selftest: every arm
  detected']` (`2026-09-24…/plan.md:234-238`). It is operator-local, stdlib-only, and needs no CI step, no Rust test
  binary and no test framework. No Python test file exists anywhere in `scripts/` (`ls scripts/`), and
  `.github/workflows/ci.yml` has no `python` token (`grep -nE 'python|setup-python'` → 0 hits).
- **Timeout identity is coordinate-free and COLLIDES within a helper.** cargo-mutants names a `+=` mutation
  `replace += with -= in {fn}`, which is identical for every `+=` site in that function. `skip_absolute_path` has three
  advances (`:326`, `:328`, `:331`), but the ledger says its six timeouts are whole-body `0`/`1` plus `-=`/`*=` "on the
  two advances". Which two of the three is not recorded, and the multiplicity of each description is not recoverable
  from the ledger (`skip_line_number_suffix` has four advances against the ledger's "the advances"). A roster row set
  reconstructed from the ledger is a HYPOTHESIS the next audit run confirms or refutes. It is not a copy of a
  measured tally.
- **Timeout membership is contention-sensitive in the same way `missed` already is.** The sixteenth timeout of run 1
  became `unviable` in run 2 through LNK1104 link contention (ledger `:104-106`). An exact timeout multiset therefore
  inherits the residual the `missed` comparison already has: a rostered mutant lost to contention reads as ABSENT.
  This is not a new class of host-flakiness, only the existing one extended to a second tally.

## Conventions to follow
- **Grade the tally, print one failure per line, verdict last**: `  SURVIVED, not in roster (x{n}): {f}: {m}` /
  `  ROSTERED, did not survive (x{n}): …` then `MUTATION GATE {unit}: FAIL|PASS` (`mutation-gate.py:145-152`). A
  timeout arm follows this form with its own direction words.
- **Paths printed repo-relative**: `output.relative_to(ROOT).as_posix()` (`:116`). One site breaks this: `complete()`
  returns `f"no outcomes.json under {out}"` (`:72`) with `out` absolute, so the not-readable FAIL path prints a host
  path today. The fixture arm that exercises it would put that leak in committed evidence. FINDING, in scope: the
  line is on the grading path this chunk re-shapes.
- **stdlib only** (`tomllib`, `json`, `re`, `subprocess`, `collections`) — `mutation-gate.py:25-34`. The host Python
  is 3.14.3 (`python --version`), above `tomllib`'s 3.11 floor.
- **Array-form spawn**: `subprocess.run(cmd, cwd=ROOT)` with `cmd` a list (`:118-122`), with no `shell=True` (re-derived:
  `grep -n 'shell=' scripts/mutation-gate.py` → 0 hits).
- **Fixture placement is constrained by `.gitignore:62`**: `mutants.out/` carries no leading slash, so it ignores a
  directory of that name at ANY depth. `git check-ignore -v --no-index
  scripts/fixtures/mutation-gate/pass/mutants.out/missed.txt` → matched by `.gitignore:62`. The same path without the
  `mutants.out` component → not ignored. So a committed fixture tally dir must NOT be named `mutants.out`, and the
  grading seam must take the tally directory itself, not `{output}/mutants.out`.

## New files to create
- `scripts/fixtures/mutation-gate/`
- `conductor-0.3.0/chunks/2026-09-30-mutation-gate-grades-every-tally-it-rests-on/evidence/`

## Files to modify
- `scripts/mutation-gate.py` — the grading extracted into a function that takes a tally directory and a roster path;
  the timeout comparison; the conservation check against `outcomes.json`; the `:72` host path made repo-relative; a
  `selftest` verb over the committed fixtures.
- `scripts/mutation-roster.toml` — the header documents the timeout tally class, and rows carry the class field the
  plan fixes. Which conductor-emit timeout rows land, if any, is the fork below.

## Open questions
- How are the fifteen `conductor-emit` timeouts rostered under the no-cargo-mutants ruling? (a) Reconstruct rows from
  the ledger's shape table as a hypothesis the next audit's run confirms or refutes. (b) Roster none now: the unit
  fails closed at the next audit, whose measured `timeout.txt` then seeds the rows. (c) Enumerate candidates with a
  non-testing `cargo mutants --list`, which still cannot say which ones time out. → blocks: plan-decision.
