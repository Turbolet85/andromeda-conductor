# tests extract

## Relevance
Relevant — the chunk ships a CI gate that test-plan §9's Lint row already mandates, and 30 of the 60 reformatted files sit under crate-local `tests/`, so the domain owns both the gate's shape and the suite's post-pass greenness.

## Constraints
- The fmt check belongs to the **Lint stage** the pipeline table already describes, which requires `cargo fmt --check` run beside `cargo clippy --workspace --all-targets -- -D warnings` under a `~/.cargo` + `target/` cache keyed on `Cargo.lock` (per test-plan §9 Pipeline structure, Lint row `:455`). Whether the shipped `ci.yml` has a job/step that already hosts the clippy half is research's question — the plan states target state, not what CI does today.
- A fmt error is a **build-failure condition**, stated at TWO sites — §9 Build failure conditions `:477` and §10 Build failure conditions `:507` (both read "Lint / clippy `-D warnings` / fmt error/failure"). A gate that merely warns satisfies neither (per test-plan §9 + §10 Build failure conditions).
- CI jobs must be identified **by NAME**, never by `ci.yml` line coordinates, "which move as the workflow grows" (per test-plan §9 Matrix builds).
- A **gate step never carries `continue-on-error`** — only diagnostic steps may, because "a gate that can report success by skipping or erroring is the thing the strict arm exists to forbid" (per test-plan §9 Pipeline structure, E2E webview row).
- The formatting pass must leave the binding gates green: `cargo nextest run --workspace --profile ci` (§9 Unit row), `cargo clippy --workspace --all-targets -- -D warnings` (§9 Lint row) and `--fail-under-lines 60` (§10 Coverage thresholds) — §11 Quality forbids skipping any of them "just this once."
- **Runner portability is a standing gate**: the suite must be green under `cargo test -p <crate>` as well as nextest, since only the shared-process runner exposes a test relying on per-test-process isolation (per test-plan §4 Framework; restated §12 Runner-portability bullet). Reformatting 30 `tests/` files owes both runners.
- Committed goldens are content-locked: canonical line shapes are pinned by exact-string `assert_eq!` at unit tier and by `<crate>/tests/snapshots/` insta files in assert-mode only (per test-plan §7 Golden artifacts + §4 conductor-report bullet). Any site where rustfmt's output would move asserted literal layout is where the chunk's `#[rustfmt::skip]` premise resolves.

## Patterns to follow
- **No new stage selector, no new `agent-run` verb.** §3 CI stage selectors keeps `cargo clippy` and `cargo test --doc` as their own §9 stages explicitly NOT folded into `--unit`; the command count stays five. A fmt gate is a §9 Lint-stage concern, not a sixth command or a new `run --*` flag (per test-plan §3 CI stage selectors).
- **Gate-ships-without-a-stage-table-row precedent.** The 2026-09-06 coverage-completeness gate shipped a CI step and took no §9 stage-table row on operator ruling; the Lint row already names `cargo fmt --check`, so the row is the mandate, not a thing to duplicate (per test-plan §9 Pipeline structure + the amendment below).
- **The a11y job's step shape** is the in-repo model for a Windows-runner gate step: named gate step with no `continue-on-error`, diagnostics separated and `if: always()` uploads kept out of the gate's verdict (per test-plan §9 Pipeline structure, E2E webview row).
- **Cache discipline** — the Lint stage's cache is specified as `~/.cargo` + `target/` keyed on `Cargo.lock`; §11 CI bans skipping dependency caching because it slows the agent feedback loop (per test-plan §9 Pipeline structure + §11 CI).
- **De-literalize moved counts.** The plan's own derived-count discipline requires naming the SET rather than substituting a fresh literal that re-stales — relevant to how this chunk records the 282-site / 60-file measurements (per test-plan §9 Live-Pulse scenarios de-baking precedent; amendments 2026-09-07-dependency-polish, 2026-09-07-sr-findings-fixed).

## Anti-patterns to avoid
- NEVER make the fmt gate advisory, `continue-on-error`, or scoped to a subset of the tree — §11 Quality bans skipping a quality gate "just this once" and bans lowering a bar to pass the build; §9/§10 name fmt a build-FAILURE condition.
- NEVER re-record a golden to absorb a formatting-induced diff — `cargo insta review`'s interactive accept UI is banned outright; insta runs CI/assert mode only (per test-plan §11 Universal + §2 Agent-runnable invariants).
- NEVER drop the Lint stage's dependency cache while editing the workflow (per test-plan §11 CI).

## Contract bindings
- **tests §9 ↔ security §Dependency Security** — the fmt step lands in the same `.github/workflows/ci.yml` that carries the supply-chain stage (`cargo audit` + `cargo deny check`, bare-`cargo audit` form per the 2026-09-05 amendment); adding a step must not perturb that stage's ordering or greenness (per test-plan §9 Pipeline structure, Supply-chain row).
- **tests §9 ↔ a11y CI gate** — the same workflow hosts the `a11y` job whose WebView2 thread the chunk scope explicitly excludes; the fmt gate must not alter that job's steps or conditions (per test-plan §9 Pipeline structure, E2E webview row).
- **tests §3 ↔ obs-plan §3** — a formatting-only pass plus one workflow step changes no envelope field, no `status` disk read and no JSONL line shape, so the §3 ↔ obs-plan §3 agreement must be re-affirmed unchanged (per test-plan §3 Test Harness Contract).

## Acceptance criteria contributions
- (tests) CI's named Rust lint job runs `cargo fmt --check` alongside `cargo clippy --workspace --all-targets -- -D warnings`, with no `continue-on-error`, and turns the job red on any violation (per test-plan §9 Pipeline structure Lint row `:455` + §9/§10 Build failure conditions `:477`/`:507`).
- (tests) `cargo nextest run --workspace --profile ci` is green after the pass, with zero retries configured and no test quarantined (per test-plan §9 Unit row + §10 Zero-flakiness budget).
- (tests) Each reformatted crate is additionally green under `cargo test -p <crate>` — the runner-portability gate — since the pass touches 30 crate-local `tests/` files (per test-plan §4 Framework + §12 Runner-portability bullet).
- (tests) `cargo clippy --workspace --all-targets -- -D warnings` and the `--fail-under-lines 60` coverage gate remain green after the pass; no gate is relaxed to accommodate it (per test-plan §10 Coverage thresholds + §11 Quality).

## Relevant amendment history
- **2026-09-06-coverage-completeness-gate** (§9 Matrix builds; §1/§4/§6 Path 6) — the closest precedent: a chunk that FIRST BUILDS a spec'd CI gate reconciles that spec's own stale description, and the operator ruled NARROW — retire only false PRESENT-TENSE claims, leave TARGET-state descriptions standing. Its proposed §9 stage-table row for the new gate was explicitly NOT applied ("no row, follow the precedent"). This is the direct authority behind the chunk's boundary that `test-plan.md:455` is left standing, not retired.
- **2026-09-04-sidecar-spawn-without-a-console-window** (escalation E2, §9 Matrix builds) — a detector's proposal to retire §9 target-state CI claims against the measured two-job `ci.yml` was dismissed as an UNIMPLEMENTED PLAN rather than drift. Same shape as `:455`: an unbuilt gate is a plan, not a falsehood.
- **2026-09-05-audit-corrective** (§9 stage table + build-failure conditions, §10, §11 CI, §12 — SIX sites) — the last chunk to amend the CI gate wording. Its lesson is the sweep discipline: a claim about a FLAG must be swept on the flag, not the command name, and the §9/§10 build-failure-condition pair must move together. Both `:477` and `:507` carry the fmt clause today.
- **2026-09-07-a11y-ci-gate** (§9 Matrix builds, Pipeline E2E row) — established "identify CI jobs BY NAME rather than by `ci.yml` line coordinates," and shipped the second Windows CI job, which is why the workflow's job/step structure moved since earlier plan text.
- **2026-09-07-sr-findings-fixed** (§9 Pipeline E2E row) — opened the a11y job's step set to `continue-on-error` diagnostics while pinning the rule that the GATE step itself never carries it; also de-literalized a baked pass count per the derived-count discipline.
- **2026-08-20-verifier-self-hardening** (§9 Scoped mutation audit; §4 Framework) — recorded that the §9 stage table is CI's complete inventory of enforced gates and NOT the complete inventory of checks a chunk may owe, and minted the `cargo test -p <crate>` runner-portability gate that the `tests/`-file half of this pass triggers.
