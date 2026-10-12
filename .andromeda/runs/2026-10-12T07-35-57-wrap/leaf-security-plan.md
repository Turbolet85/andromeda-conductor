group: security-plan
read: .claude/docs/security-summary.md · 49 · «**Full plan:** `.andromeda/security-plan.md`. Path-scoped ru»
read: .claude/rules/security.md · 57 · «- 2026-10-11: A silent `cargo deny check licenses` is no evi»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/security-summary.md:11   class: retired
  leaf-text: «so none carries a residual. `CONDUCTOR_E2E_SEED_DIR` is TEST-read, never read by a shipped binary, with no setter since 2026-10-11 — an unset no-op. The real-model»
  source: .andromeda/security-plan.md:51 «`CONDUCTOR_*` env namespace (`CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, and `CONDUCTOR_SEED`)»
  replace-with: «so none carries a residual. The real-model»

- leaf: .claude/docs/security-summary.md:12   class: omission
  leaf-text: «Scenario config files → garde `range` + `#[garde(custom)]` validation at load (the trust boundary);»
  source: .andromeda/security-plan.md:317 «garde's `range` + `#[garde(custom)]` rules AND the load-path `Scenario::check_*()` cross-field checks called from `from_toml_str` TOGETHER are the trust boundary»
  replace-with: «Scenario config files → garde `range` + `#[garde(custom)]` validation at load AND the load-path `Scenario::check_*()` sibling-spanning checks called from `from_toml_str` (`check_capabilities` · `check_budgets` · `check_checklist`, each a `CoreError::Config`) — together the trust boundary;»

- leaf: .claude/docs/security-summary.md:14   class: stale
  leaf-text: «and its two test-binary readers (`coverage_gate`, `scenario_audit_gate`) resolve from `CARGO_MANIFEST_DIR`»
  source: .andromeda/security-plan.md:109 «**`scenario-audit-ledger.toml` is the SECOND artifact of that test-binary-reader shape, and the first with NO shipped reader at all (2026-09-16):** `conductor-core --test scenario_audit_gate` loads it»
  replace-with: «and its one test-binary reader (`scenario_audit_gate`) resolves from `CARGO_MANIFEST_DIR`, as does `coverage_gate`, the capability manifest's test-binary reader»

- leaf: .claude/docs/security-summary.md:44   class: omission
  leaf-text: «Prove which by re-running the latest published tool.»
  source: .andromeda/security-plan.md:171 «**So the porcelain check on the DB checkout PRECEDES classifying any parse failure as external** (a fetch cannot clear residue; a CI run's clean checkout, or a fresh clone under `--db`, is the second witness).»
  replace-with: «Before classifying any parse failure as external, read the local advisory-db checkout's `git status --porcelain` (a fetch never clears untracked residue — the 2026-08-09 deferral proved a local cache fault, closed 2026-09-05); then prove which by re-running the latest published tool.»

- leaf: .claude/docs/security-summary.md:44   class: omission
  leaf-text: «**A new dependency MAY land during that wait, but only on»
  source: .andromeda/security-plan.md:172 «regime CLOSED 2026-09-05 — the audit measures green, so a delta now lands under a green `cargo audit` over the committed lock, and this conditional re-arms only if a future audit goes red on a fault verified external»
  replace-with: «**The wait regime CLOSED 2026-09-05 (the audit measures green); it re-arms only if a future audit goes red on a fault the porcelain check verifies external — then a new dependency MAY land during that wait, but only on»

- leaf: .claude/docs/security-summary.md:34   class: stale
  leaf-text: «Subprocess hardening: fixed sidecar path + `.env(...)` data-dir, never argv/shell interpolation.»
  source: .andromeda/security-plan.md:315 «spawn `andromeda-pulse-mcp` as a fixed, hard-coded program NAME resolved through the inherited `PATH` (never a config-derived or operator-supplied command string)»
  replace-with: «Subprocess hardening: the sidecar spawned as a fixed hard-coded program NAME resolved through the inherited `PATH` + `.env(...)` data-dir, never argv/shell interpolation.»

- leaf: .claude/rules/security.md:18   class: retired
  leaf-text: «`CONDUCTOR_E2E_SEED_DIR` is READ by a `conductor-run` TEST binary, never by a shipped one, and has had no setter since its setter `wdio.conf.ts` left with the window (2026-10-11) — its value would be a harness-owned repo-relative constant, so the canonicalize rule does not apply, and the seeding test runs as an unset no-op. `CONDUCTOR_MSEDGEDRIVER`»
  source: .andromeda/security-plan.md:111 «Env-var path handles (`CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_CONTRACT_MANIFEST`)»
  replace-with: «`CONDUCTOR_MSEDGEDRIVER`»

- leaf: .claude/rules/security.md:16   class: stale
  leaf-text: «so BOTH test-binary readers are recorded — `conductor-report --test coverage_gate` and `conductor-core --test scenario_audit_gate`, each resolving»
  source: .andromeda/security-plan.md:109 «**`pulse-capabilities.toml` additionally has a TEST-BINARY reader outside `conductor-core` (2026-09-06):** `conductor-report --test coverage_gate` loads it through the same bounds-checking»
  replace-with: «so each test-binary reader is recorded — `conductor-report --test coverage_gate` (of `pulse-capabilities.toml`) and `conductor-core --test scenario_audit_gate` (the ledger's only reader), each resolving»

- leaf: .claude/rules/security.md:40   class: omission
  leaf-text: «or into run-report artifacts (`<run_id>.md`, `runs.db`, JSONL) — sanitize»
  source: .andromeda/security-plan.md:350 «NEVER let run-report artifacts (`<run_id>.md`, `runs.db` rows, JSONL journals) or committed evidence records (the per-chunk `evidence/` tree) leak absolute host paths»
  replace-with: «or into run-report artifacts (`<run_id>.md`, `runs.db`, JSONL) or committed evidence records (the per-chunk `evidence/` tree) — sanitize»

- leaf: CLAUDE.md:39   class: omission
  leaf-text: «never leak absolute host paths or internal struct names into logs / run-report / `runs.db`»
  source: .andromeda/security-plan.md:350 «NEVER let run-report artifacts (`<run_id>.md`, `runs.db` rows, JSONL journals) or committed evidence records (the per-chunk `evidence/` tree) leak absolute host paths»
  replace-with: «never leak absolute host paths or internal struct names into logs / run-report / `runs.db` / committed `evidence/`»
