# Tests validation — route draft

## Insert
- Between `Base CI + agent-run harness skeleton` and `Seeded phase scheduler`: **"Test framework + coverage tooling — cargo-nextest, proptest, insta, assert_cmd/assert_fs, cargo-llvm-cov"** (epoch: `Foundation`)
  Reason: Per test-plan §3 bootstrap phases (test-runner-install / coverage-tooling-install), the framework must precede all feature development.
- Between `Base CI + agent-run harness skeleton` and `Seeded phase scheduler`: **"Test data bootstrap — rstest fixtures, scenario config one-per-P-ID, in-memory/TempDir runs.db isolation"** (epoch: `Foundation`)
  Reason: Per test-plan §3 test-data-bootstrap-wire, fixtures must be in place before any timeline/emission chunk is tested.
- Between `Coverage-matrix generator` and `Artifact redaction layer`: **"CI test-job config — nextest JUnit XML, llvm-cov LCOV/Cobertura, test-reporter, per-OS matrix"** (epoch: `Run report & persistence`)
  Reason: Per test-plan §3 + §9 CI Integration, the CI test job + artifact upload + parallelization must be explicitly wired.
- Between `Coverage-matrix generator` and `Artifact redaction layer`: **"Quality gate config — coverage threshold, zero-flakiness budget, cargo-audit/deny, Tauri ≥2.10.3, toolchain ≥1.94.1"** (epoch: `Run report & persistence`)
  Reason: Per test-plan §3 quality-gate-config-emit + §10 Quality Gates, gates must be configured alongside coverage tooling.

## Reorder
- Move `Coverage-matrix completeness gate` (Polish & ship) before `Artifact redaction layer` (Run report & persistence)
  Reason: Per test-plan §3 status-endpoint-implement, the coverage-matrix artifact is a data contract belonging after the persistence layer, not deferred to Polish.

## Rewrite
- `Determinism-replay harness`: "same-seed identical stream-shape property/golden assertion" → "same scenario+seed ⇒ identical stream shape via insta golden + proptest regressions, tokio start_paused"
  Reason: Per test-plan §7 Fixture pattern + §2 Agent-runnable invariants, the wording should name the determinism machinery.
- `5-command agent-run harness`: "boot/run/status/cleanup/logs across .sh and .ps1" → "boot=preflight gate, run=nextest+scenarios, status=runs.db/JSONL read, cleanup=idempotent, logs=JSONL journal"
  Reason: Per test-plan §3 Test Harness Contract, the command semantics are specific (boot=preflight, status=disk-read, no daemon PID).
