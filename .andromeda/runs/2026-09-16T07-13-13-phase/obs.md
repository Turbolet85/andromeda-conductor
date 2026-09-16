# obs extract

## Relevance
**partial** — the chunk is corpus hygiene, but obs OWNS the `slo_tier` closed enum + its ms thresholds that the tier half grades against, and owns the CI-gate registration/artifact surface if the check registers as a build gate.

## Constraints
- The tier half's closed set is obs-owned and must be read from the single definition, not re-authored: `slo_tier` is the closed enum `<5s | <20s | <90s` mapped to 5000 / 20000 / 90000 ms (per obs-plan §5 Per-surface/per-path metrics + §10 Performance budgets). Whether the code already exposes this enum as a code-native closed set the check can consume (vs. a literal list in a script) is research's question.
- A static check over declared phase durations is a DIFFERENT quantity from the runtime SLO measurement obs mandates: §10 requires `latency_ms = read_back_observed_at − journal_emitted_at` (wall-clock), asserted at report-generation time against the tier threshold (per obs-plan §10 Performance budgets + §5). The summed-phase-duration proxy may bound the tier statically but must not be stated as, or substituted for, the §10 runtime assertion.
- `slo_tier` is one of the eleven required Run-report envelope keys, and "required" means KEY PRESENCE, not non-null (per obs-plan §6 Required fields). A corpus-side closed-set probe and the envelope-side conformance gate must agree on the same enum; divergence would split one contract in two.
- If the registration is a CI gate, §9 requires it to state an explicit machine-evaluable FAIL condition that gates the build, in the shape the two existing obs gates use (per obs-plan §9 Log conformance check + Zero-unlogged-panics gate).
- Any output the check produces must be agent-readable without a dashboard or human review — machine-parseable text/JSON, consumable from the job log or an uploaded artifact (per obs-plan §2 Agent-readable invariants; §11 Universal).
- Absolute host paths are the known hazard for a file-walking gate: §9's log-conformance check REJECTS drive-letter / `/home` / `/Users` / `%APPDATA%` / `~/.cargo` / `.rustup` paths in any telemetry artifact, so a check that prints scenario file paths keeps them in the job log and out of `logs/agent-latest.jsonl` and `runs/**.jsonl` (per obs-plan §9 Pipeline integration + §11 Logs).
- The chunk's scope law (no listener, no sidecar, committed files only) sits inside obs's standing bans anyway — no OTel SDK, no metric instrument, no network OTLP for this check (per obs-plan §11 Telemetry Strategy / Metrics / Universal).

## Patterns to follow
- The two shipped obs CI gates in §9 — the log-conformance check and the zero-unlogged-panics gate — are the in-repo shape for "a gate that fails the build" with an agent-readable verdict and an explicit violation condition (per obs-plan §9).
- The fmt gate's §9 row: a gate shipped as its OWN early step in the `rust` job (index 2, not colocated with clippy), whose diff is agent-readable from the job log and is deliberately NOT routed into any telemetry artifact because it carries absolute host paths (per obs-plan §9 Pipeline integration, Lint/typecheck row).
- The a11y CI gate's §9 artifact row: a gate emitting a JSONL record, uploaded via `actions/upload-artifact@v4` with `if-no-files-found: warn`, conformance asserted IN-JOB — the pattern to follow if this check emits a machine-readable record rather than plain stdout (per obs-plan §9 Telemetry artifact handling).
- §5's tier→threshold table is the one place the mapping is stated; §10 restates it as the enforcement contract. Both are the citation target for the probe's expected set (per obs-plan §5 + §10).
- Naming, if the check is expressed in Rust inside an instrumented crate: `{module}.{operation}` and the closed span-name set — no new name is minted for a static check (per obs-plan §2 Naming conventions + §11 Spans/Traces).

## Anti-patterns to avoid
- NEVER define a soft SLO budget with no enforcement, and never let the static corpus check stand in for the report-generation-time `latency_ms <= threshold` assertion (per obs-plan §11 SLO).
- NEVER leak absolute host paths (or internal struct names) from the check into logs, run-report artifacts, or `runs.db` (per obs-plan §11 Logs + §9 Log conformance check).
- NEVER add a metric instrument, meter provider, or OTel SDK to express the tier tally — Minimal tier's performance budget is a JSON field assertion, not a histogram (per obs-plan §11 Metrics + §5).

## Contract bindings
- **obs ↔ tests harness:** the `slo_tier` enum and the eleven-field envelope are a binding contract owned by test-plan §3/§5 that obs §3/§6 reproduces ("obs aligns to tests, not vice versa"). A corpus-side closed-set probe asserts the same enum the harness's journal-conformance gate asserts; changing one side alone breaks the bind (per obs-plan §3 Log format JSON schema + §6).
- **obs ↔ CI:** registering the check as a build gate adds a §9 Pipeline-integration row (and a §9 Telemetry-artifact row if it emits a record) — the fmt gate and a11y gate each landed exactly this way (per obs-plan §9).
- **obs ↔ security:** the host-path-freedom rule on any emitted artifact is the shared redaction invariant (per obs-plan §11 PII Scrubbing).

## Acceptance criteria contributions
- The tier probe's expected set is exactly `<5s | <20s | <90s` with thresholds 5000 / 20000 / 90000 ms, sourced from the obs-owned definition rather than re-declared in the check (per obs-plan §5 Per-surface/per-path metrics + §10 Performance budgets).
- The check's verdict and per-scenario findings are machine-parseable end to end — readable from the CI job log or an uploaded artifact by an agent, with no dashboard and no human-review step (per obs-plan §2 Agent-readable invariants + §11 Universal).
- No absolute host path emitted by the check reaches `logs/agent-latest.jsonl` or any `runs/**.jsonl` telemetry artifact; scenario paths stay job-log-only, as the fmt gate does (per obs-plan §9 Pipeline integration + §11 Logs).
- The check mints no span name outside §11's bounded set and creates no metric instrument or OTel SDK/exporter (per obs-plan §11 Spans/Traces + §11 Metrics).

## Relevant amendment history
- **2026-09-10-live-pulse-in-lane-scenario-round** (§4 delegated-timing note) — retired the gloss of `read_back_observed_at − journal_emitted_at` as "Conductor's MCP round-trip"; it is the journal-relative span over the scenario's WHOLE emission window (legs of 18s/35s/30s declared phases measured `latency_ms` 18169/35120/30212, while sidecar tool calls logged `duration_ms` 0–22). Directly underwrites this chunk's tier half: the amendment's own downstream note records the round-trip gloss as a plausible origin of `constellation-severity-live-wiring`'s unattainable `<20s` tier — i.e. exactly the "tier vs. own summed phase duration" defect this check mechanizes.
- **2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate** (§1 build-time set, §9 Lint/typecheck row) — the last new CI gate to join §9; established that a gate's path-bearing output stays job-log-readable and is NOT written into a telemetry artifact, because §9's log-conformance check rejects absolute host paths. Nearest precedent for this chunk's registration and for its output-routing rule.
- **2026-09-07-a11y-ci-gate** (§3 extension point, §9 artifact table) — a CI gate whose JSONL record joined §9's artifact table with in-job conformance under `CONDUCTOR_RUNS_DIR`; also recorded that `journal_conformance` asserts key PRESENCE, closed sets and host-path freedom, never key exclusivity — which is what lets one gate serve two record shapes. The precedent if this check emits a record.
- **2026-09-06-run-report-envelope-conformance-gate** (§6 Required fields) — restored the envelope list to ELEVEN and redefined "required" as key presence rather than non-null. Fixes the exact semantics of the `slo_tier` key this chunk's tier half grades.
- **2026-06-27-obs-ci-conformance-gate** (§9 Log conformance check) — the two record shapes are distinct: the `agent-latest.jsonl` gate asserts the §3 self-obs BASE schema, not the §6 envelope; the envelope gets its own gate. Relevant so a new check's output is not accidentally validated against, or routed into, the wrong record shape.
