# Tests validation — route draft

## Insert
- Between `A11y CI gate at an honest terminal` and `Keyboard and focus-order coverage ownership`: **"Control-panel-launched parity at a terminal — Critical Path 7's owed half proven on the webview leg or deferred with a named owner"** (epoch: `Epoch 3`)
  Reason: test-plan §1 Critical Path 7 / §6 Scenario 7 record the control-panel-LAUNCHED half as OWED and assign it to the tauri-driver leg, whose fate is exactly what this epoch decides.

## Rewrite
- `Diagnostic-quality cluster off the drift pin`: "P-031, P-033, P-034 and P-044 backed by an exercised path rather than `UNBACKED_AUTO`" → "P-031, P-033, P-034 and P-044 backed by an exercised path, gate pin and committed matrix moved together"
  Reason: test-plan §6's coverage gate asserts the `UNBACKED_AUTO` pin by exact-set equality in both directions AND the committed `coverage-matrix.md` byte-equal to its render (whose summary line carries the unbacked count), so dropping four ids reds both arms unless the pin and artifact move in the same change.

- `Real-model leg posture and grading rule`: "deterministic mode off, operator-gated, never a CI gate, grading fixed before the first drive" → "deterministic mode off, operator-gated, never a CI gate, grading and per-leg quiet window fixed before the first drive"
  Reason: test-plan §9 Live-Pulse records these legs as NOT run-stable — every `conductor run` fires its own preflight canary against Pulse's 120 s idle + 30 s tick dedupe — so a posture without the per-leg quiet window lets an unchanged tree grade differently by SUT state.

- `Scenario-assertion audit gate`: "one mechanical re-runnable check a fresh reader can use to establish both outcomes, per test-plan §10" → "one mechanical re-runnable check establishing both outcomes, named as a CI gate or an operator instrument"
  Reason: test-plan §9 declares its stage table CI's complete gate inventory with the mutation audit as the named operator-instrument exception, so a check placed in neither is enforced by nothing.

- `Full-gate regression over the moved surfaces`: "scenario corpus, the `a11y` job and the live-drive path, per test-plan §10 quality gates" → "scenario corpus, the `a11y` job and the live-drive path, green under both test runners, per test-plan §10"
  Reason: Epoch 2 edits pinning tests including an inline `#[cfg(test)]` module under `src/`, and test-plan §4 (§12, 2026-08-20) makes `cargo test -p <crate>` a standing runner-portability gate beside nextest. Bootstrap is otherwise absent by design — harness §3, fixtures §7, CI §9 and the coverage/flakiness gates §10 all ship in-tree from 0.2.0 (`scripts/agent-run.{sh,ps1}`, `.github/workflows/ci.yml`, `.config/nextest.toml`), so no bootstrap chunk is owed and the harness-before-features sequencing rule is satisfied by construction.
