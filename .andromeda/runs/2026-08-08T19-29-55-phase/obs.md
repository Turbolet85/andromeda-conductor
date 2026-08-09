# obs extract

## Relevance
Partial — no new instrumentation is added (lock-only, zero Rust source change expected), but this chunk re-proves two obs-owned CI gates and bumps `anyhow`, which is load-bearing obs error-edge infrastructure.

## Constraints
- The supply-chain audit result is an obs build-failure condition in its own right, not only security's: a red `cargo-audit` is listed among "build / deploy failure conditions" per obs-plan §10. Green audit+deny is therefore part of this chunk's obs exit condition.
- `cargo-audit` / `cargo-deny` output is a declared telemetry artifact — "inline in CI logs", agent-readable from stderr, per obs-plan §9 (Telemetry artifact handling). Whatever this chunk does to the gate must leave that output machine-parseable and captured; no new human-gated step.
- The no-OTel-SDK invariant is **behavioral**, per obs-plan §3 (OTel SDK init): dormant transitive `opentelemetry` + `opentelemetry_sdk` (pulled by `opentelemetry-proto` default features) are audit/deny-green and not a violation. A lock bump must not initialize an SDK or an exporter; equally, the recorded `default-features = false` follow-up on `conductor-emit`'s opentelemetry-proto dep must not be silently absorbed here as bump fallout.
- `anyhow` is obs infrastructure, not a neutral dep: the binary-edge bridge (`std::panic::set_hook()` → `tracing::error!` → `anyhow` edge) and `Display`-not-`Debug` at that edge are half the struct-name/host-path guard, per obs-plan §11 (Error Reporting · PII Scrubbing). 1.0.102 → 1.0.104 must preserve sanitized `Display` output with no absolute host paths.
- `scripts/agent-run.sh run` in the re-prove list re-exercises the §9 log-conformance gate and the §10 zero-unlogged-panics invariant; both must be asserted green post-bump, not assumed, per obs-plan §9 (Log conformance check) + §10 (Always-required SLO invariant).
- The gate reads the **self-obs base-line** record shape (`timestamp_ms`/`level`/`target`/`service.{name,version,environment}`/`run_id`) from `logs/agent-latest.jsonl`, NOT the run-report envelope, per obs-plan §3 ("two record shapes") + §9. Do not re-verify against envelope fields.
- No new span, metric, or log surface is licensed by this chunk: the bounded span-name set and the "no metrics backend" position stand unchanged per obs-plan §4 + §5 + §11 (Spans / Metrics).

## Patterns to follow
- Existing CI obs gate is already wired at `.github/workflows/ci.yml` (lines 112–160): a hermetic no-Pulse Blocked run produces `logs/agent-latest.jsonl`, three assertions (base schema via `jq`, host-path redaction anchors mirroring `conductor-core::redact`, `^thread.*panicked` grep across the file **and** captured producer stderr), then artifact upload. Re-prove through this job as-is — per obs-plan §9 it is the operationalized gate, no new step expected.
- Existing supply-chain steps at the same file (lines 74–78) are separate `cargo audit` + `cargo deny check` steps — the obs §9 "supply-chain audit report → inline in CI logs" pattern. Note CI invokes bare `cargo deny check` while the chunk's DoD names `advisories bans sources licenses`; reconcile deliberately rather than by drift.
- Dormant-transitive-dep handling: when a dep-tree change surfaces OTel crates, document as dormant + record a follow-up rather than escalate (obs-plan §3 transitive note; amendment 2026-06-17 installed the playbook rule).
- Dogfood verification: obs gates are proven against the real produced artifact, not a mock (obs-plan §9 conformance check; the CI job's `cargo run … --agent-mode` producer is the precedent).
- Agent mode is a **read-only** trigger (`flag || CONDUCTOR_AGENT_MODE` env, never written by Conductor) per obs-plan §3 (Logging stack); `scripts/agent-run.sh` exports it — preserve that direction when re-running.

## Anti-patterns to avoid
- NEVER resolve an advisory by introducing or initializing an OTel SDK/exporter for self-observation, and never let a bump convert a dormant transitive SDK crate into an initialized one (obs-plan §11 Telemetry Strategy + Universal).
- NEVER lose or de-structure telemetry artifacts to get the gate green — audit/deny output and `logs/agent-latest.jsonl` must remain uploaded and machine-parseable, 14-day retention (obs-plan §11 CI + §9).
- NEVER weaken the zero-unlogged-panics gate, add a retry, or accept an unsanitized panic/backtrace on stderr as fallout of the `anyhow` bump (obs-plan §11 SLO + Error Reporting; §10).

## Contract bindings
- **obs ↔ security** — the audit/deny gate is security's policy surface (`.claude/rules/security.md` §Dependencies, `deny.toml`); obs owns only its *build-failure-condition* status (§10) and its *artifact/agent-readability* surface (§9). Obs takes no position on lock-only vs. manifest-floor (chunk Decision 1) except that the floor must not name a version whose error edge regresses the redaction contract.
- **obs ↔ tests harness** — `cargo-nextest` JSON/junit output + `logs/agent-latest.jsonl` are the shared CI artifacts per obs-plan §9 (Pipeline integration); the 428-test baseline re-prove passes through this binding, and tests §3 owns the envelope schema obs aligns to.
- **obs ↔ error-handling seam** — `anyhow` version ↔ obs-plan §11 (Error Reporting / PII Scrubbing) `Display`-not-`Debug` edge; a bump here is an obs-relevant change even though it is a "lock-only" edit.

## Acceptance criteria contributions
- (obs) After the bumps, the CI obs-conformance job passes unchanged: every line of `logs/agent-latest.jsonl` carries the 7 base fields (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`) per obs-plan §3/§9.
- (obs) Redaction boundary holds post-bump: no absolute host path (drive-letter, `/home`, `/Users`, `%APPDATA%`, `~/`, `.cargo/`, `.rustup/`) in the self-obs artifact, and zero `^thread.*panicked` lines in the artifact or captured producer stderr — including in the `anyhow`-1.0.104 error/Blocked path (obs-plan §10, §11).
- (obs) No self-obs OTel SDK is initialized after the lock change: `opentelemetry` / `opentelemetry_sdk` remain dormant transitive deps with no init or exporter call site; any change to their presence/version is recorded against the §3 follow-up rather than absorbed silently (obs-plan §3, §11).
- (obs) Supply-chain telemetry stays agent-readable: `cargo audit` and `cargo deny check …` results remain in CI logs/stderr in parseable form via the existing steps, with the `obs-conformance` artifact still uploaded (obs-plan §9, §11 CI).

## Relevant amendment history
- **2026-06-17-raw-otlp-message-scaffold** (§3 OTel SDK init) — established that the no-SDK invariant is behavioral and that opentelemetry-proto's default features transitively pull `opentelemetry` + `opentelemetry_sdk` (dormant, audit/deny-green); a playbook rule was added so a dormant transitive SDK no longer re-escalates D-obs-stack. Directly governs this chunk: it is a dep-tree/lock change, and it must not re-litigate — or accidentally close — the recorded `default-features = false` follow-up.
- **2026-06-27-obs-ci-conformance-gate** (§9) — the gate this chunk re-proves asserts the §3 self-obs base schema, NOT the §6 run-report envelope; the two record shapes are distinct and the envelope's own gate is not yet built. Prevents re-verifying green against the wrong record shape.
- **2026-06-15-log-error-boundary-redaction** (§6 gate · §11 Logs/PII) — fixed the redaction model to host-FILE-path masking + field-name allowlist + `Display`-not-`Debug` at the `anyhow` edge, with the allowlisted `target` module path explicitly preserved (no `::`-token redaction). Relevant because `anyhow` is one of the three bumped crates and the CI grep anchors mirror `conductor-core::redact`.
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3 Logging stack) — agent mode is the read-only `flag || CONDUCTOR_AGENT_MODE` trigger, with the file sink reusing the unchanged processor-stage allowlist + `redact_value`. Relevant to re-running `scripts/agent-run.sh run` as gate evidence.
