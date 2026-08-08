# Tests validation — route draft

## Insert
- Between `Faithful emission dispatcher` and `Per-check read-back extraction`: **"Dispatcher determinism goldens — same scenario and seed, identical emission stream shape across runs over the new per-phase dispatcher"** (epoch: `Epoch 2`)
  Reason: per test-plan §1 Coverage triggers (property-test, determinism discipline) + §7 Golden artifacts, replacing `coarse_emit` changes the journal stream the committed goldens lock, and §10's zero-retry flakiness budget rests on that invariant holding.

- Between `Operator-pause and checklist live firing` and `Desktop a11y sweep`: **"Webview E2E harness leg — agent-run's --e2e stage covering the headless tauri-driver run on Linux+xvfb, both shell variants"** (epoch: `Epoch 5`)
  Reason: test-plan §9 defines the E2E (webview) stage as `agent-run run --e2e` → tauri-driver under xvfb and §3 binds `.sh`/`.ps1` to identical semantics, yet Epoch 5's sweep and CI gate assume a leg that the 0.1.0 harness never covered (`--e2e` is the CLI-crate leg only).

- Between `Coverage completeness gate` and `Dependency polish`: **"Operator-gated live suite — re-runnable live-Pulse proof invocation carrying its evidence, never a CI gate"** (epoch: `Epoch 6`)
  Reason: test-plan §9 Live-Pulse scenarios requires the live leg be an explicit operator gate (`workflow_dispatch` or `scripts/agent-run.sh`), otherwise Epochs 2–4's live proofs are one-shot evidence with no agent-runnable re-run path.

## Rewrite
- `Per-check read-back extraction`: "observed values from the corpus tools feeding the unchanged evaluate/classify path" → "observed values from the corpus tools feeding the unchanged evaluate/classify path, including degraded_mode ⇒ KnownResidual"
  Reason: test-plan §5 read-back boundary list and §6 Scenario "Known-residual classification" make `degraded_mode ⇒ KnownResidual` Critical Path 5, which no chunk in the draft names.
