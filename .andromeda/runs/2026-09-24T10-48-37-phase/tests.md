# tests extract

## Relevance
partial. The chunk adds CI gate steps and a local check form, but no product seam, envelope or critical path. The test-plan rules that apply are the static-gate, harness, CI, fixture and flakiness rules.

## Constraints
- **Agent-runnable, no manual step.** The plan preamble's agent-driven invariant and test-plan §11 → Universal ("NEVER include a manual smoke step") both apply. The secret scanner, the undeclared-env-key check and the `settings.json` hook-guard verification must each give an exit code and output a machine can parse. They must not rely on anyone reading the result by eye. This applies to scope item 11's octal-payload smoke too.
- **Five commands only.** test-plan §3 (5-command implementation, `run` → CI stage selectors) and §3 Bootstrap phases (`5-command-discipline-wire`) cap the harness at five commands. The existing precedents add a flag or a sibling `case` branch instead: `--live`, and the committed `mutation-gate.py`, which "adds no sixth `agent-run.{sh,ps1}` command". The plan also requires `.sh` and `.ps1` to behave identically, including under the invoking environment. So the local entry (scope A.2) cannot become a sixth command, and it must behave the same in both shells.
- **The gate must be able to fail, and prove it.** test-plan §6 (Coverage-matrix completeness gate, both static-gate legs) sets the pattern:
  - a loud presence guard, then the check;
  - `continue-on-error: false`;
  - in-suite negative arms that seed a known-bad input and assert the error names the offender.

  The same applies to scope B.7: with 0 live `${{ env.* }}` references, the env-key step only proves it works through a seeded known-bad input.
- **No literal counts.** test-plan §6 (the static-gate Verification signal and the scenario-backing leg) bans literal-count predicates. It requires exact-set grading in both directions: unpinned, rotted and lost-subject. The allowlist and the "0 true hits" baseline (scope A.5) should be graded as exact sets. A hit count or allowlist length must not be the pass condition.
- **Gate steps have no escape hatch.** test-plan §9 (Pipeline E2E row) says "the GATE step itself never carries `continue-on-error`". test-plan §11 → Quality says "NEVER skip quality gates 'just this once'". test-plan §10 (Zero-flakiness budget) bans retries. The new scanning steps are gates, not diagnostics.
- **Fixtures.** test-plan §7 (Self-bootstrapping requirement) allows committed fixtures only in crate-local `tests/fixtures/` trees. Each committed fixture must carry its meaning under test through a round-trip, not just parse. test-plan §11 → Test Data bans production data.
  - Known-bad secret-shaped inputs must be synthetic.
  - They must live where that provenance set sanctions them, or be generated at test time.
  - They must not turn the scanner's own tracked-tree baseline red.
- **Runner and tooling.** test-plan §4 (Framework; runner-portability gate) and §12 (Open questions: no JS/TS runner adopted) register no Python test runner. The only committed non-Rust instrument, `scripts/mutation-gate.py`, is operator-local and never a CI step (§4, §9 Scoped mutation audit). A stdlib Python scanner enforced in CI, with tests under `scripts/`, would have no registered runner and no precedent. The alternative is a crate-local Rust `tests/` static gate like `coverage_gate` or `scenario_audit_gate`. This is a P4 fork. Whether either form fits the existing CI job set is for research to answer.

## Patterns to follow
- **Static gates over committed data (test-plan §6 and §4, Scenario catalog bullet).** Each is a named step after the existing gates, with a presence guard, then the target, red on violation. Grading is exact-set in both directions. Can-fail arms mutate the real committed input. The "set of static gates" is defined as whatever targets assert a committed artifact against its source, never as a count.
- **Stage-table rows and amendments.** New CI gate steps get no §9 stage-table row. This follows the `2026-09-06-coverage-completeness-gate` operator resolution, reaffirmed by `2026-09-16-scenario-assertion-audit-gate`. Any §9/§10 build-failure-condition wording goes through wrap's amendment flow, not an edit during the phase.
- **Sweep controls (test-plan §6, scenario-audit leg).** A sweep control proves that the detector fires on a known positive. Scope A.5's pattern set should have a known-positive control, so that a baseline of "0 hits" is trustworthy and not a silent no-op. This matches the `Found 0 mutants` = no-op rule in test-plan §4.
- **Stale-bound CI claims (test-plan §9, Matrix builds).** Refer to CI jobs and steps by name, never by `ci.yml` line coordinates, which move. This affects how the new steps and the CARRY's `ci.yml:61-71` anchor are recorded.

## Anti-patterns to avoid
- A gate that can report success by skipping or erroring (test-plan §9, E2E row rule). That includes `continue-on-error`, a missing-input path that exits 0, or a zero-hit pass with no positive control.
- A literal hit, allowlist or row count as the acceptance predicate (test-plan §6, static-gate Verification signal).
- A manual or eyeball verification step, including for the hook-guard fix, or retry-once handling of a flaky scan (test-plan §11 → Universal; test-plan §11 → Quality).

## Contract bindings
- **tests ↔ security.** The secret-scanning step lands in the same CI workflow as the `cargo audit` / `cargo deny check` supply-chain gate (test-plan §9 stage table, §3 `quality-gate-config-emit`). Its allowlist-with-reasons idiom, the tool-vetting rule and the one-member CI-fetch class are owned by security-plan §Secret Management / §Dependency Security, not by this plan.
- **tests ↔ obs.** If a harness entry is added, test-plan §3 requires that §3 ↔ obs-plan §3 still agree: no new envelope/JSONL shape, and the `status` non-recursive `runs/*.jsonl` glob untouched. Any scanner output artifact must sit outside that glob, as `runs/a11y/` does.

## Acceptance criteria contributions
- Each new check (secret scanner and undeclared-env-key check) goes red on a seeded known-bad input, naming file + line or the key, and never echoes the matched secret in full. It goes green on the tracked tree, with a known-positive control showing the pattern set fires. (per test-plan §6, Coverage-matrix completeness gate: static-gate legs)
- The allowlist and baseline are graded by exact-set equality in both directions: a new unallowlisted hit fails, and an allowlist entry that no longer matches anything fails as rot. No literal-count predicate. (per test-plan §6, static-gate Verification signal)
- The new CI steps are gate steps with a presence guard and no `continue-on-error`. The local form runs through the existing five-command harness at identical `.sh`/`.ps1` semantics, with no sixth command. (per test-plan §9, Pipeline E2E row; test-plan §3, 5-command implementation)
- Existing gates stay green: `cargo nextest run --workspace --profile ci`, the `cargo test -p <crate>` runner-portability run, and `--fail-under-lines 60`, unlowered. Any new Rust test target must pass under both runners. (per test-plan §4, Framework; test-plan §10, Coverage thresholds)

## Relevant amendment history
- **2026-09-06-coverage-completeness-gate:** by operator resolution, a CI gate step over committed data takes no §9 stage-table row. The proposal's rationale misquoted §9 as claiming the table is the complete inventory. This is the precedent the new scanning steps follow.
- **2026-09-16-scenario-assertion-audit-gate:** the second static gate over committed data was recorded as a sibling leg, not a new scenario, again with no §9 row. The wrap sweep caught a §7 fixture-enumeration site the body edits had missed. If this chunk adds a committed fixture tree, §7's tree set and its round-trip enumeration both need amending at wrap.
- **2026-09-13-audit-debt-retired-before-epoch-1-closes:** `scripts/mutation-gate.py` was registered as a committed, operator-local, non-CI instrument. The operator dismissed a proposed §4 clause stating a general verification duty for committed non-Rust instruments, as policy that chunk never decided. A Python scanner in CI would reopen that question, and it belongs to P4 or escalation, not an assumption.
- **2026-09-05-audit-corrective:** the supply-chain gate form was corrected from a flag form measured red by construction to the bare `cargo audit` CI actually runs, and the flag was swept across six sites. Lesson for this chunk: measure the gate command's real exit behaviour against the adjudicated allow-set before registering it, and sweep claims on the flag, not the command name.
- **2026-09-07-sr-findings-fixed:** the gate step never carries `continue-on-error`. Harness identical-semantics is asserted together with the invoking environment: PowerShell `$PSNativeCommandUseErrorActionPreference` preempted the capture-then-print. This applies to any `.ps1` local entry for the scanner.
