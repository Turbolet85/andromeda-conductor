### Bootstrap phases (derive for route / setup-project)

_[ALL tiers — explicit derivation hint for downstream consumers per D26 chain.]_

- **test-runner-install:** install cargo-nextest 0.9.137 (via `taiki-e/install-action`) + `cargo test --doc` step + `[dev-dependencies]` assertion/fixture packages: rstest 0.26, assert_cmd 2 + predicates 3, assert_fs 1, insta 1 (`json` feature), proptest 1.9.
- **5-command-discipline-wire:** wire `scripts/agent-run.{sh,ps1}` with the 5 commands (boot=precondition probe + preflight gate / run / status / cleanup / logs) per 5-command implementation above. Binding contract — both shell variants must expose identical semantics, a property asserted of the SCRIPTS **together with the invoking environment**: a caller that preempts native-command exits (`$PSNativeCommandUseErrorActionPreference = $true`) diverges the `.ps1` leg with zero script delta, measured in CI 2026-09-07 (Windows/Linux dev-OS targets; the identical `.sh`/`.ps1` semantics include the webview E2E leg, which runs on Linux+`xvfb` AND headfully on the Windows dev host (`CONDUCTOR_MSEDGEDRIVER`-gated) — only macOS lacks a WebDriver).
- **status-endpoint-implement:** N/A as an endpoint — instead implement the Run-report envelope serializer (shared by `runs.db` row + JSONL journal + Markdown) per Status endpoint shape; polled by reading the `runs.db` row / JSONL journal.
- **log-format-bind-with-obs:** the `tracing-subscriber` JSON journal format is DEFINED here (see Log format above) — this plan is its source of truth and obs derives its product-side log envelope FROM this subsection (binding contract — harness greps the journal for assertions; format break = harness break). The field-allowlist / redaction layer (no host paths, no struct names) is the one piece owned by obs downstream; this plan asserts that boundary via a negative test.
- **pid-file-commitment-wire:** N/A — no daemon. `cleanup` removes run artifacts + `runs.db` rows + releases the `:4317` port-occupier instead.
- **test-data-bootstrap-wire:** wire rstest seeded fixtures + P-ID-keyed scenario config files + in-memory/`TempDir` `runs.db`; cleanup clears between runs.
- **coverage-tooling-install:** install cargo-llvm-cov 0.8.7; emit LCOV via `cargo llvm-cov nextest --lcov --output-path lcov.info` (+ `--cobertura coverage.xml` for CI) + `--fail-under-lines` gate.
- **quality-gate-config-emit:** emit GitHub Actions config enforcing coverage threshold (Section 10), zero-retry flakiness budget, `cargo audit` + `cargo deny check`, committed `Cargo.lock`, toolchain ≥ 1.94.1, `tauri` ≥ 2.10.3.

route uses this list to plan phase ordering (test-runner-install → 5-command-discipline-wire → status-endpoint-implement → log-format-bind-with-obs → pid-file-commitment-wire → test-data-bootstrap-wire → coverage-tooling-install → quality-gate-config-emit). setup-project materializes each phase's bootstrap script + dependency list + verification command (the log format + envelope shape are defined in this Section 3 and flow downstream to obs; obs derives its envelope from here, not the reverse).

---
