# tests extract

## Relevance
relevant — the chunk changes the committed executable form of the §4 mutation instrument and its roster, which test-plan §4, §9, §10 and §12 govern directly.

## Constraints
- test-plan §4 (Mutation instrument) requires the gate to grade on tallies read out of the output tree, never on cargo-mutants' exit code, and to compare a unit's tally against `scripts/mutation-roster.toml` as a MULTISET over `(file, mutation)`. The new timeout comparison must use the same keying and the same multiset semantics as the `missed` comparison. Whether `timeout.txt` lines have the same `{path}:{line}:{col}: {mutation}` shape as `missed.txt` is research's question: the plan states the shape for `missed.txt` only (per test-plan §12, the roster member IDENTITY entry).
- test-plan §12 (roster member IDENTITY entry, 2026-09-13/-14) requires three things. Identity is the mutation DESCRIPTION, never line or column. REGISTRATION is the `[[unit]]` declaration, never the row count, so a registered unit with zero rows PASSES on an empty tally. A run producing 0 mutants is an explicit FAIL. The timeout class must keep all three rules: a registered unit with no timeout rows passes on an empty `timeout.txt`, and adding the timeout class must not weaken the 0-mutant FAIL.
- test-plan §9 (Scoped mutation audit) and §4 require the mutation run to stay an operator/local instrument. No CI step may invoke it, and it adds no sixth `agent-run.{sh,ps1}` command. The plan is silent on whether a regression test OF the gate script may run in CI. That question is research's and a P4 fork, separate from the mutation run's footing.
- test-plan §10 (Mutation-survivor disposition) and §12 require the roster's two forms to stay joined on the `member` key without restating each other. §12 holds the reasoning and citation, and the roster holds only what a tally comparison needs. So any timeout rows the roster gains must join to the `conductor-emit` 2026-09-14 entry in §12 rather than carry disposition prose. That entry's sentence saying the fifteen timeouts are "rostered nowhere, because no gate can hold them" goes stale when this chunk lands. Correcting it is a wrap-time amendment, not a phase edit.
- test-plan §10 (Zero-flakiness budget) and §12 (the `conductor-verify` 2026-09-04 member's rejected agreement test) require a roster to be an enumerated set that means the same thing on every host. Rostering timeouts is only sound if the rostered timeouts are host-stable. The plan cannot attest that the fifteen `conductor-emit` timeouts are host-stable (scope cites `unviable` moving on link contention alone). This belongs to P3/P4 and to the next audit's measurement.
- test-plan §4 (Framework) requires any Rust test binary to be green under both `cargo nextest run -p <crate>` and `cargo test -p <crate>`. This binds only if the gate's regression harness is a Rust test that shells out to the script.
- test-plan §7 (Self-bootstrapping requirement) requires every committed fixture to carry its MEANING under a test that exercises it. The sanctioned committed-fixture family is the SET of crate-local `tests/fixtures/` trees. Where fixture `mutants.out/` tallies live, and whether they are committed or generated per test, is research's question. A committed tally outside that SET would extend the family, and §7 would need a wrap amendment.

## Patterns to follow
- A dependency-injected seam over a runtime patch (test-plan §8 Anti-monkey-patching). The grading should take an existing tally directory as its input, so fixtures reach it without patching the unconditional `cargo mutants` subprocess call.
- The committed-fixture round-trip that asserts BOTH arms (test-plan §7, `scenario_audit_gate::the_sweep_sees_a_wrapped_gloss_the_plain_form_misses`). Each fixture proves the arm it exists for. Pass, unexpected-timeout FAIL, rostered-timeout-absent FAIL and the existing `missed` arms each get a separate fixture, and each asserts the verdict it drives.
- Grade the gate's OWN verdict (its exit plus its printed failure lines) as the observable, just as §4 grades tallies rather than the tool's exit (test-plan §4; §11 Unit, on testing observable behavior rather than internals).
- The multiset precedent (test-plan §12 identity entry). Two members may legitimately share a `(file, mutation)` pair, as the two `replace == with != in stub_result` entries do. Timeout fixtures should include a duplicate-pair case so the multiset semantics carry over to the new class.

## Anti-patterns to avoid
- Invoking `cargo mutants` from any test command, fixture or CI step. test-plan §11 CI and §9 keep operator/local instruments out of CI, and the founder ruling in scope forbids any invocation in this chunk.
- Monkey-patching `subprocess.run` (or any production path) at test time instead of adding a seam (test-plan §11 Mocking, "NEVER monkey-patch production code at runtime").
- Rostering a tally class whose membership moves with host conditions, such as an exact `unviable` set, or adding a numeric threshold in place of an enumerated set (test-plan §10 Zero-flakiness budget and Mutation-survivor disposition: "deliberately NOT a numeric `--fail-under` threshold").

## Contract bindings
- tests ↔ CI stage table (test-plan §9 Pipeline structure). Every CI stage is a cargo invocation, and no stage runs a Python test runner.
  - A Rust test binary that shells out to the script lands in the Unit stage (`cargo nextest run --workspace --profile ci`). It would then run in CI and need `python` on the Windows runner. Whether the runner has it is research's question.
  - A Python test beside the script would run in no CI stage.
  - Either way the §9 non-CI footing of the MUTATION RUN must hold.
- tests ↔ 5-command harness (test-plan §4, §9). The gate adds no sixth `agent-run.{sh,ps1}` command, and its regression proof must not add one either.
- test-plan §12 citation home ↔ `scripts/mutation-roster.toml` executable form. They join on the `member` key, per test-plan §10 Mutation-survivor disposition.

## Acceptance criteria contributions
- Driven against fixture tally directories with no `cargo mutants` invocation, the gate FAILs (non-zero, with a named failure line) on an unexpected `timeout.txt` entry and on a rostered timeout that is absent. It PASSes when `timeout.txt` equals the unit's rostered timeout multiset, and the existing `missed` unexpected/absent arms still FAIL as before (per test-plan §4 Mutation instrument; §12 roster member IDENTITY entry).
- A registered `[[unit]]` with zero timeout rows PASSes on an empty `timeout.txt`, and a 0-mutant fixture still FAILs (per test-plan §12 roster member IDENTITY entry, REGISTRATION clause).
- The gate's regression proof is a committed, re-runnable test whose fixtures each assert the verdict they exist for. If it is a Rust test, it is green under both `cargo nextest run -p <crate>` and `cargo test -p <crate>` (per test-plan §7 Self-bootstrapping requirement; §4 Framework).
- `.github/` holds no step that invokes `mutation-gate.py`'s mutation run or `cargo mutants`, and `scripts/agent-run.{sh,ps1}` gains no sixth command (per test-plan §9 Scoped mutation audit).
