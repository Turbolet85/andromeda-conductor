# obs extract

## Relevance
partial — the chunk is test-surface and CI-gate work that adds no runtime instrumentation, but it touches the harvest tier obs-plan §4 names by file path, a §9/§10 repository-hygiene gate, and a new CI stage whose output §9 governs.

## Constraints
- obs-plan §4 (Real-model posture, 2026-09-22) requires the real-model interpretation to be graded at the harvest tier over the scrubbed capture and never through the run-report envelope, and states that this scenario adds no critical path, span name or span attribute. The split and the lift into `real_model_common` must keep that boundary. A behaviour-preserving split must not move any grading into the envelope or into a span (per obs-plan §4 Real-model posture).
- obs-plan §4 names the grading site by path (`conductor-run/tests/real_model_harvest.rs`). Splitting the file by series makes that citation stale, and the owning master (obs-plan) must then be corrected through the wrap's amendment flow, never left pointing at a retired target. Whether the original file name survives as one of the split targets is P4's decision (per obs-plan §4 Real-model posture).
- obs-plan §9 (Pipeline integration, the Repository-hygiene gates row) requires every `Secret-scan gate` hit to be one line naming a repo-relative `path:line` and its rule, never the matched text, and requires that output to stay in the job log, never written into a telemetry artifact (`logs/agent-latest.jsonl`, `runs/**`). The new no-`.git` skip arm sits on that same output channel, so its reason line must follow the same discipline: no absolute host path, and no workspace or temp-copy path (per obs-plan §9 Pipeline integration).
- obs-plan §10 (Build / deploy failure conditions) lists a red repository-hygiene gate (`Secret-scan gate`) as a build failure. The skip must not turn that condition into a vacuous green wherever the gate's subject exists, which includes CI's checkout. Whether the code's current panic site (`secret_scan_gate.rs:185-189`) can tell "no repository" apart from "`git ls-files` failed inside one" is research's question (per obs-plan §10 Build / deploy failure conditions).
- obs-plan §9 (Lint / typecheck row) sets the discipline for compiler output: rustc and cargo diagnostics stay agent-readable in the job log and are never written into a telemetry artifact, because they carry absolute host paths that §9's log-conformance check rejects. The new `stub-server` `cargo check --tests` step's output falls under that discipline (per obs-plan §9 Pipeline integration; §9 Log conformance check).
- obs-plan §11 (Spans / Traces) requires a bounded span-name set. Test-target reorganization and a lifted grading helper add no name to it. Any `tracing` use in the lifted `real_model_common` code must not mint a span name outside the set (per obs-plan §11 Spans / Traces).

## Patterns to follow
- Use the existing repository-hygiene gate output shape for any line the secret-scan skip prints: one line, repo-relative or path-free, rule- or reason-named, and kept in the job log only (per obs-plan §9 Pipeline integration, Repository-hygiene gates row).
- Use the fmt-step precedent for a new compile-only CI step: run it as its own `rust`-job step, let its diagnostics stay in the job log, and add no telemetry artifact (per obs-plan §9 Pipeline integration, Lint / typecheck row).
- Keep harvest-tier grading as test-binary work over committed, scrubbed captures, with each grade held by a named test fn. This is the shape §4 cites for `severity_harvest.rs`, `delegated_timing_harvest.rs` and `real_model_harvest.rs` alike (per obs-plan §4 Severity-lifecycle and Real-model posture).

## Anti-patterns to avoid
- NEVER leak absolute host paths in logs or artifacts. A skip reason or an assertion message that echoes the missing repository's path, or the path of a cargo-mutants temp copy, is such a leak (per obs-plan §11 Logs).
- NEVER use human-review-gated analysis without machine-parseable export. The `stub-server` gate must fail the job through a non-zero exit, never a warning that someone has to read to notice (per obs-plan §11 CI).
- NEVER assume the named redaction sites bound every host-path channel. A new test-output channel, such as the skip line or a lifted helper's panic message over capture text, counts as a channel too (per obs-plan §11 PII Scrubbing).

## Contract bindings
- obs ↔ tests: the harvest-tier grading that obs-plan §4 delegates to `real_model_harvest.rs` is test-plan territory. Renamed nextest targets must still be run by the same CI and harness legs, or the §4 grading site stops being exercised (per obs-plan §4 Real-model posture; §9 Pipeline integration, Unit tests row).
- obs ↔ security: the real-model grading reads captures scrubbed by the security chain and held by sha256 digest pins. The obs side requires grading over the scrubbed capture only, never through the envelope (per obs-plan §4 Real-model posture; §11 PII Scrubbing).
- obs ↔ CI: the `Secret-scan gate` belongs to the §10 build-failure set and the §9 Repository-hygiene output row. The new `stub-server` check step joins the §9 pipeline as a compile-only stage (per obs-plan §9; §10).
- The §3 keyed contracts (logging stack, log format, heartbeat and the rest) are not touched by this chunk and were not consulted.

## Acceptance criteria contributions
- The secret-scan gate's no-`.git` skip arm emits at most one line. The line names its reason, carries no absolute host path and no matched text, and is written to no telemetry artifact (`logs/agent-latest.jsonl`, `runs/**`) (per obs-plan §9 Pipeline integration, Repository-hygiene gates row; §11 Logs).
- In a tree that has a git repository, the secret-scan gate still fails the build on a secret-shaped hit, and a `git ls-files` failure inside a repository still fails rather than skips (per obs-plan §10 Build / deploy failure conditions).
- The split and the lift add zero span names, zero span attributes and zero envelope fields. The real-model interpretation stays graded at the harvest tier over the scrubbed capture (per obs-plan §4 Real-model posture; §11 Spans / Traces).
- The new `stub-server` `cargo check --tests` step fails CI with a non-zero exit when a gated target does not compile, and its diagnostics stay in the job log only (per obs-plan §9 Pipeline integration, Lint / typecheck row; §11 CI).
