# obs extract

## Relevance
Partial — this chunk establishes the CI artifact-upload pattern and gates that a downstream obs-conformance chunk will reuse; itself does not implement obs conformance.

## Constraints
1. **Artifact upload contract (obs-plan §9):** `logs/agent-latest.jsonl` and cargo-nextest JSON outputs must be uploaded as CI artifacts via `actions/upload-artifact`, establishing the reusable pattern for downstream obs gates.
2. **Agent-mode flag behavior (obs-plan §3 / amendment 2026-06-24):** agent mode is a **read-only trigger** (`CONDUCTOR_AGENT_MODE` env var OR `--agent-mode` CLI flag) that Conductor reads, never writes; logs to `logs/agent-latest.jsonl` in JSON-only format (no pretty-print) when triggered.
3. **JSONL schema binding (obs-plan §6 / §3):** any logs uploaded must conform to the 11-field Run-report envelope schema (journal_emitted_at, read_back_observed_at, run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, fingerprints) — a contract from upstream test-plan, required for downstream conformance gate.
4. **Zero-unlogged-panics invariant (obs-plan §9):** CI conformance gate (separate downstream chunk) will grep logs for unstructured panic backtraces; this chunk's test infrastructure must ensure all panics are captured via `std::panic::set_hook()` and logged as structured `tracing::error!` events.
5. **Artifact retention alignment (obs-plan §9):** GitHub Actions default (14 days) matches upstream test-plan retention (§5 "Artifact retention: 14-90 days").

## Patterns to follow
1. **Artifact-upload seam:** CI workflow steps that upload coverage report + JUnit XML must follow the same `actions/upload-artifact` pattern and naming convention (`coverage-report`, `test-results`); downstream obs-gate will reuse this step structure.
2. **Agent-parseable output:** cargo-nextest JUnit XML and llvm-cov JSON/HTML reports must be machine-parseable (no ANSI color codes in uploaded artifacts); align with obs-plan §2 "agent-readable invariants" (no human-only dashboards).

## Anti-patterns to avoid
1. **Absolute host paths in artifacts:** CI logs must not leak drive-letter paths (`X:\`, `C:\`), user home paths (`%APPDATA%`, `~/.cargo`), or backtrace file paths; redaction layer (per obs-plan §6 anti-pattern "Leaking absolute host paths in run-report artifacts") applies to all CLI output piped to artifacts.
2. **Unstructured panic output in CI logs:** raw panic backtraces (matching `^thread.*panicked`) are a conformance violation; zero-unlogged-panics gate (downstream) will fail if found.

## Contract bindings
- **Tests harness**: cargo-nextest `ci` profile (zero-retry + JUnit emission), CI job configuration (artifact upload steps, coverage gate threshold — pulled from test-plan §10).
- **Obs CI conformance gate** (downstream chunk): reuses artifact-upload pattern + log JSONL schema binding (§6) + zero-unlogged-panics enforcement. This chunk does NOT implement conformance check; that chunk adds the `jq`-based validation + stderr grep for panics.

## Acceptance criteria contributions
1. _(CI quality gate)_ Coverage report and JUnit XML artifacts are uploaded on every CI run and accessible via GitHub Actions artifact API.
2. _(CI determinism)_ The `ci` nextest profile enforces zero retries (fail-fast on any flake); no nextest retry configuration contradicts this.
3. _(Obs binding ready)_ All logs written to `logs/agent-latest.jsonl` (in agent mode) conform to the 11-field JSONL envelope schema (§6); downstream conformance gate can parse them via `jq` without schema violations.

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging:** agent-mode flag reworded to clarify `CONDUCTOR_AGENT_MODE` is read-only (flag OR env var; Conductor never writes it). Relevant because this chunk routes test logs through agent-mode to `logs/agent-latest.jsonl` and must respect the read-only trigger contract (harness/operator exports it; Conductor consumes it).
