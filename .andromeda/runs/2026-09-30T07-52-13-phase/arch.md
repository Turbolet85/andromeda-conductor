# arch extract

## Relevance
partial — the chunk touches only a host-Python operator instrument and its roster. Architecture governs where that instrument sits, what may invoke it, and how any gate of its own would reach CI. It says nothing about the grading logic.

## Constraints
- The mutation-tally gate (`scripts/mutation-gate.py` + `scripts/mutation-roster.toml`) is registered as an operator-local instrument that runs on host Python 3, with no lockfile and no toolchain pin. Arch requires that no CI step invokes it and that it adds no sixth `agent-run.{sh,ps1}` command, so the 5-command harness surface stays unchanged (per architecture §Stack and Technologies, "Operator instruments (host runtime)" row). Any test harness this chunk adds for the gate must preserve both halves. A Rust test binary that shells out to the script, run under the CI nextest stage, would make a CI step invoke the instrument. That contradicts the row as written, so it would need a wrap amendment, not a silent widening.
- Every CI gate follows one shape: a named `rust`-job step with no `continue-on-error` and no `if:`, a presence guard that emits `::error::` and exits 1, then an invocation of a specific nextest target. "The gate IS the Rust test", and re-listing its assertion in shell is a drifting second copy (per architecture §Established Decisions [CI/CD]; §Infrastructure Patterns → CI/CD approach). If P4 routes the gate's own regression test into CI, it must take this shape. A python-test CI step would be a new gate shape that arch does not register. Whether any such step is wanted at all is P3/P4's question.
- Tests run under `cargo nextest` through the zero-retry `ci` profile in `.config/nextest.toml`, where no retries mask flakes (per architecture §Infrastructure Patterns → Build system; §Cross-cutting Patterns → Determinism discipline). A fixture-driven gate test must therefore be deterministic across repeated runs, whichever harness hosts it. Grading `unviable` against an exact roster would break this.
- Crate-per-seam boundaries: a Rust-hosted test lives in one existing member's `tests/`, and the chunk scope forbids production Rust change (per architecture §Established Decisions [Module Boundaries]; §Occupied Resources → Crate names). Arch names no member as the owner of mutation-instrument tests. If the Rust route is chosen, research picks the host crate. A new workspace member is out of bounds for this chunk (§Project Intent → How new functionality is added).
- The committed directory tree lists `scripts/` members, each with a qualifier (for example "neither harness shell, no 6th command"). `mutation-gate.py` and `mutation-roster.toml` appear in the Operator-instruments row but not in the tree (per architecture §Infrastructure Patterns → Directory structure). A new committed fixture tree or test file for the gate falls under the same registry-at-wrap duty. Whether the tree must list it is a wrap-amendment question, not a phase edit.
- Host Python is resolved by no lockfile. Arch states "stdlib only" for `arch-registry-check.py` but NOT for the mutation gate (per architecture §Stack and Technologies, "Operator instruments" row). The chunk must add no new third-party Python dependency for the gate or its test without registering it. Whether the gate already reads its TOML roster through the stdlib is research's question.

## Patterns to follow
- A presence-guarded static gate over a committed subject, which fails loudly on an absent subject rather than passing vacuously (per architecture §Established Decisions [CI/CD]). This is the model for the unit-closed failure when a rostered timeout class has no measured tally.
- A committed artifact graded against the source it derives from, as in the coverage-completeness and scenario-assertion audit gates (per architecture §Infrastructure Patterns → CI/CD approach). The fixture `mutants.out/` trees graded by the gate's own logic are the same "committed data, graded by the real grader" shape.
- An operator-local instrument that stays outside the harness verbs: it is registered in the Stack row and never wired into `agent-run` (per architecture §Stack and Technologies, "Operator instruments" row; §Cross-cutting Patterns → Development Style).

## Anti-patterns to avoid
- Wiring the mutation RUN, or the gate script itself, into a CI step or into an `agent-run` verb (per architecture §Stack and Technologies, "Operator instruments" row).
- Duplicating a gate's assertion in workflow shell instead of invoking the test that owns it (per architecture §Established Decisions [CI/CD]).
- Adding a workspace member, or any production-crate edge, to host an instrument's test (per architecture §Established Decisions [Module Boundaries]).

## Contract bindings
- arch ↔ tests: arch's Operator-instruments row ("none is invoked by any CI step") binds the test plan's statement that the mutation gate is "never a CI stage" (the scope cites test-plan §4 and §9). If the gate's OWN regression test enters CI, both masters must stay consistent. That makes it a wrap amendment across architecture §Stack and Technologies and the test plan, not a one-sided change.
- arch ↔ tests: a new roster class (timeout) is ratified in test-plan §12, per the scope. On arch's side, the only duty is the registry: the Operator-instruments row names the roster file, and that naming stays accurate.

## Acceptance criteria contributions
- No `.github/workflows/` step invokes `scripts/mutation-gate.py` or `cargo mutants`. Check it with a positive probe of `git diff --numstat f33d6b7 -- .github/`, which should show no delta or a delta containing neither token (per architecture §Stack and Technologies, "Operator instruments" row).
- `scripts/agent-run.sh` and `scripts/agent-run.ps1` are unchanged against `f33d6b7`, so no sixth harness command is added (per architecture §Stack and Technologies, "Operator instruments" row).
- No production source changes under `crates/*/src/` and no new workspace member appear in `Cargo.toml`, both measured against `f33d6b7` (per architecture §Established Decisions [Module Boundaries]).
- If a Rust-hosted test drives the gate, it runs under the nextest `ci` profile with zero retries and passes on repeated runs (per architecture §Infrastructure Patterns → Build system).
