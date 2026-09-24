# obs extract

## Relevance
Partial. This chunk adds build-time CI gates and ships no runtime telemetry surface, so no spans, metrics or runtime logs are required. Obs still governs how the gates report hits, how they sit beside the existing §9 telemetry artifacts and gates, and how the plan lists its gate set.

## Constraints
- obs-plan §1 (instrumentation-scope table, `cargo build / CI/CD pipeline` row) classes build-time verification as Not-instrumentable. The secret scanner and the undeclared-`env`-key check are build-time only and fall in that class. They need no `tracing` spans, no self-obs JSONL lines and no service identity. Do not route them through `conductor-core` obs.
- obs-plan §2 (Agent-readable invariants) and §11 Universal require a machine-parseable result for every signal: a non-zero exit plus hit lines an agent can parse. It bans human-review-gated analysis. Each hit (file + line, per scope A.1; for the env check, the undeclared key per B.6) must be greppable from the job log and from the local run.
- obs-plan §9 (Pipeline integration, Lint / typecheck row, fmt-gate precedent) says gate diagnostics are read from the job log and must NOT be written into any telemetry artifact (`logs/agent-latest.jsonl`, `runs/a11y/<run_id>.jsonl`), because host paths break §9's log-conformance check. The scanner's output belongs in the job log only.
- obs-plan §9 (Log conformance check) and §11 Logs ("NEVER leak absolute host paths") require no absolute host paths (`X:\`, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, `.rustup`) in telemetry. Hit reports should name repo-relative paths. Together with scope A.1's rule never to echo a matched secret in full, the diagnostic must stay leak-free even if a later change pipes it into an artifact.
- obs-plan §9 (Telemetry artifact handling) and §11 CI ("NEVER lose telemetry artifacts") require the existing uploads (`logs/agent-latest.jsonl`, the a11y violation record, nextest JSON) and the log-conformance and zero-unlogged-panics gates to survive the `ci.yml` edits unchanged. Research should check whether a new early failing step changes which later upload or gate steps still run.
- obs-plan §9 (CI-specific resource attributes) takes `ci.run.id` / `git.commit.sha` from `${{ github.run_id }}` / `${{ github.sha }}`, which is the `github` context, not `env`. The B.6 check must match only `env.X` references so these obs tag sources never trip it. Whether the a11y job's `CONDUCTOR_RUNS_DIR=runs/a11y` is declared via a workflow/job/step `env:` block or via `GITHUB_ENV` is research's question. It bears directly on the open F12 measurement (B.8).
- obs-plan §3 (Log format JSON schema, the fingerprints / `run_id` fields) and §11 PII Scrubbing (`<redacted>` / `<host-path>` placeholders) define artifact content that looks secret-like on purpose: hex fingerprints, filesystem-safe run ids, redaction placeholders. The A.5 false-positive baseline must not force obs artifacts or the redaction layer's placeholder strings to change shape to pass the scanner. They should be admitted by allowlist entries with stated reasons. Whether such artifacts are tracked under `conductor-*` is research's question.

## Patterns to follow
- The fmt-gate precedent (obs-plan §9 Lint / typecheck row): an early dedicated step, diagnostics agent-readable from the job log, nothing written into telemetry artifacts.
- The log-conformance and zero-unlogged-panics gates (obs-plan §9): a machine check that fails the build on a violation and was proven by a dogfood PASS on the real input plus a FAIL on a seeded bad input. This matches scope B.7's need for a seeded known-bad input.
- The `journal_conformance` in-job assertion idiom (obs-plan §3 extension-point paragraph, §9 artifact table): the gate runs inside the job over the real tree, with no external service.

## Anti-patterns to avoid
- NEVER use unstructured, unparseable output or human-review-gated analysis as a gate's only result (obs-plan §11 Logs, §11 CI, §11 Universal). A hit must be named in a parseable form, not only shown as a red job.
- NEVER leak absolute host paths, or here the secret value itself, into anything that reaches a telemetry artifact (obs-plan §11 Logs, §11 PII Scrubbing).
- NEVER add an external or network-exporting consumer for gate results, such as a hosted scanner dashboard (obs-plan §11 Universal: dashboards-only obs, proprietary APM). This agrees with scope A.4's lean toward a stdlib-only scanner.

## Contract bindings
- obs ↔ security §Secret Management / §Logging & Monitoring: the no-echo rule and the no-host-path rule for hit output, and the allowlist entries covering obs redaction placeholders and fingerprint/run-id shapes. Security owns the pattern set and the tool choice. Obs owns the leak-free diagnostic shape and keeping it out of telemetry artifacts.
- obs ↔ tests §5 / CI: the new steps must not disturb the nextest JSON / `agent-latest.jsonl` artifact upload or the §9 obs gates that tests consume.

## Acceptance criteria contributions
- (obs) The secret-scan hit output (local and CI) names each hit by repo-relative path + line, contains no absolute host path and no full matched secret, and is not written into `logs/agent-latest.jsonl` or any `runs/**` telemetry artifact (per obs-plan §9 Lint / typecheck row + §11 Logs).
- (obs) After the `ci.yml` edits, every §9 telemetry artifact upload and the log-conformance and zero-unlogged-panics gates are still present and reachable in their jobs, measured by diffing the step lists before and after (per obs-plan §9 Telemetry artifact handling + §11 CI).
- (obs) The undeclared-`env`-key check ignores `${{ github.* }}` expressions, including the `ci.run.id` / `git.commit.sha` sources, and does not fire on the current `ci.yml` (per obs-plan §9 CI-specific resource attributes).
- (obs) Expected amendment (wrap): obs-plan §1's Not-instrumentable build-time verification list, §9's Pipeline integration table and §10's Build / deploy failure conditions should list the new secret-scan gate and env-key gate. If no detector proposes this, the orchestrator raises it under Validate check 5 (per obs-plan §1 / §9 / §10).

## Relevant amendment history
- 2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate: adding a build-time CI gate led to two edits. §1's Not-instrumentable row gained the gate, and §9's Lint row recorded the step placement and that its diff lines stay out of telemetry artifacts because of host paths. Why: it was the plan's own expected amendment, but no detector proposed it, so the orchestrator raised it under Validate check 5. This chunk's gates follow the same path.
- 2026-09-07-a11y-ci-gate: §9's artifact table registered a new CI artifact (`runs/a11y/<run_id>.jsonl`, uploaded with `CONDUCTOR_RUNS_DIR=runs/a11y`). This is relevant because that job's env declarations fall within scope of the B.6 env-key check and the artifact uploads must survive the edits.
- 2026-06-27-obs-ci-conformance-gate: the first time a §9 gate was put into operation, its spec wording was found stale and reconciled. A playbook rule was added so later reconciliations of gate wording on first operation are routine rather than escalated. That rule applies if this chunk finds §9 or §10 wording stale.
