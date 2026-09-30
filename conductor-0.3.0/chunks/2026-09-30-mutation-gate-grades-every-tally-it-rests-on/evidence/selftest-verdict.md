# Selftest verdict — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

Measured at /implement P2 on 2026-09-30, by the gate tool (`gate v1.5`) over the plan's `## Test Commands`, from the
repo root, in Git Bash. Each reading below is the command's log verbatim, with the exit the tool read from the process.
The logs sit in the gate tool's per-run log dir outside the tree (run dir `.andromeda/runs/2026-09-30T08-15-24-implement/`).
No `cargo mutants` ran: no entry names it, and the no-spawn witness below reads both of its output paths absent.

## The selftest — `python -X utf8 scripts/mutation-gate.py selftest` → exit 0

```
  pass: detected
  empty-unit-pass: detected
  missed-unexpected: detected
  missed-absent: detected
  timeout-unexpected: detected
  timeout-absent: detected
  timeout-multiset: detected
  count-mismatch: detected
  not-conserved: detected
  tally-missing: detected
  unparsed-line: detected
  zero-mutants: detected
  no-outcomes: detected
  incomplete: detected
  roster-tally-missing: detected
  real roster: 13 rows, schema ok
selftest: every arm detected
```

Each `detected` means four things held for that arm of `scripts/fixtures/mutation-gate/arms.toml`: the verdict
matched, a printed line contained the arm's named finding, the count of two-space-indented lines equalled its
`findings`, and no printed line carried the absolute repo path in either slash form.

## The known-bad control — `python -X utf8 scripts/mutation-gate.py selftest --fixtures conductor-0.3.0/chunks/2026-09-30-mutation-gate-grades-every-tally-it-rests-on/evidence/selftest-control` → exit 1

```
  pass-flipped: NOT detected — verdict PASS, expected FAIL; no line contains 'TIMED OUT, not in roster'; 0 findings, expected 1
  real roster: 13 rows, schema ok
selftest: ARM NOT detected — pass-flipped
```

The control arm's tallies pass, and its manifest expects a FAIL. So the harness is shown able to fail, not only the
grader, and it names all three of the conditions that failed.

## The `--fixtures` guard — `python -X utf8 scripts/mutation-gate.py selftest --fixtures ../outside` → exit 2

```
selftest: --fixtures must be repo-relative (absolute or .. rejected)
```

The value is never echoed.

## Usage — `python -X utf8 scripts/mutation-gate.py` → exit 2

```
usage: mutation-gate.py <unit> | selftest [--fixtures DIR]
```

## Unregistered unit — `python -X utf8 scripts/mutation-gate.py no-such-unit` → exit 1

```
MUTATION GATE no-such-unit: FAIL — unit is not declared in mutation-roster.toml and has no roster rows
```

The real roster loads and validates under the new `tally` schema, and the unit path still fails before the output dir
or the spawn.

## No-spawn witness — `test ! -e target/mutation-gate && test ! -e mutants.out` → exit 0, no output

This ran after every gate-script entry above. The unit path creates `target/mutation-gate/` before it spawns, and a
bare `cargo mutants` writes `mutants.out/` at the root. Both are absent.
