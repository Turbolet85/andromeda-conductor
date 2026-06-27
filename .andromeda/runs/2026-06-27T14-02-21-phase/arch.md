# arch extract

## Relevance
Partial — infrastructure pattern (CI/CD) and existing occupied resources; zero new seams or core changes

## Constraints
1. CI gate placement per §Infrastructure Patterns §CI/CD approach (existing GitHub Actions build+test layer, no new inbound service)
2. No new stack dependencies for conformance script per §Infrastructure Patterns (presentational tooling, zero engine/seam modification)
3. Produce artifact `logs/agent-latest.jsonl` per §Occupied Resources (registered; self-obs `tracing` stream in agent mode, sibling of runs dir)
4. Consume env var `CONDUCTOR_AGENT_MODE` per §Occupied Resources (registered; read-only trigger; main never writes it)
5. Validate the agent-driven headless path per §Cross-cutting Patterns (Development Style: agent-driven; core must remain headless-drivable without the Tauri shell)
6. Self-obs stack is `tracing 0.1.44 + tracing-subscriber 0.3.23` per §Stack and Technologies + §Inherited Defaults (NOT an OTel SDK; OTLP remains product emission)

## Patterns to follow
1. Gate-step scaffold from prior CI chunk `2026-06-27-ci-quality-gate-config` (shell: bash multi-command, if: always())
2. Agent-mode invocation via existing `scripts/agent-run.sh run --agent-mode` or CLI equivalents (no new harness)
3. Self-obs JSON line validation mirrors obs-plan §3 base-field set (timestamp_ms, level, target, service identity, run_id) per existing recording

## Anti-patterns to avoid
1. Do not add workspace crates, seams, or modify core crate boundaries (scope: zero engine/seam diff)
2. Do not introduce live Pulse dependency or require Epoch-gated setup (scope: Windows-doable, no `ANDROMEDA_PULSE_MCP_ENABLED` required for this gate)
3. Do not hard-code absolute host paths in gate logic (redaction boundary applies to `agent-latest.jsonl` per obs-plan §6/§11)

## Contract bindings
obs-plan §3 (log JSON schema + two-record-shape; base-field set for `agent-latest.jsonl`) · obs-plan §6/§9/§11 (CI conformance gate spec + redaction boundary) · test-plan §9/§10 (CI integration conventions) · security-plan (redaction enforcement on Conductor's own artifact)

## Acceptance criteria contributions
- "(arch) CI gate step conforms to §Infrastructure Patterns §CI/CD approach (existing GitHub Actions layer); zero new crates, seams, or external service."
- "(arch) Artifact per §Occupied Resources: produces `logs/agent-latest.jsonl` via `CONDUCTOR_AGENT_MODE` env var (registered 2026-06-24); stored per §Data model conventions (TEXT RFC-3339 if timestamps, JSON-per-line)."
- "(arch) Gate validates the agent-driven headless path per §Cross-cutting Patterns; agent-mode run is source of truth for self-obs conformance."
- "(arch) Conformance checker is presentational/tooling (scripts/* or #[test]; zero modification to conductor-core seams or established-decisions)."

## Relevant amendment history
1. **2026-06-24-sanitized-stderr-agent-mode-logging** (§Occupied Resources): Registered `CONDUCTOR_AGENT_MODE` env var (read-only agent-mode trigger; main never writes) + `logs/agent-latest.jsonl` on-disk artifact (the self-obs `tracing` JSON stream in agent mode, separate schema from emission journal, sibling of runs dir moving with `CONDUCTOR_RUNS_DIR`). This chunk validates that this artifact is produced + conforms to obs-plan §3 base-field contract.