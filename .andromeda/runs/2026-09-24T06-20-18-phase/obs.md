# obs extract

## Relevance
Partial and thin. The chunk only edits documentation (two `architecture.md` registry sections) and adds no instrumentation. Obs has a stake for two reasons: obs-owned handles are registered in arch §Occupied Resources, and obs-plan points into §Established Decisions by anchor.

## Constraints
- Every obs-owned handle registered in §Occupied Resources must still be registered after compaction. That covers the env vars `CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`, `CONDUCTOR_AGENT_MODE` and `CONDUCTOR_RUNS_DIR`, plus `RUST_LOG`. Per obs-plan §3 Service identity, §3 Logging stack and §11 Logs (the `RUST_LOG=info,{crate}=debug` directive form). Whether the current arch body states each handle today, and in which row, is a question for P3 research.
- The self-obs sink paths must stay registered as current truth. They are `logs/agent-latest.jsonl` (CLI agent mode) and `<runs_dir.parent()>/logs/conductor-tauri.jsonl` (Tauri backend, directory rides `CONDUCTOR_RUNS_DIR`). Per obs-plan §3 Log file location and §12 Obs Decisions Log (Exporter).
- The a11y violation artifact `runs/a11y/<run_id>.jsonl` must stay registered. It is the only exercised instance of the envelope extension point. Per obs-plan §3 Log format JSON schema and §9 CI Integration (artifact table).
- Port status must survive as stated fact: `:4317` is the product fault stream only, and `:4318` is unused/dead. obs-plan says "`:4318` is unused per arch" (§1 row at `:49` and §3 Correlation) and "unused/dead per arch" (§11 Universal), and depends on arch still saying so. Per obs-plan §1, §3 Correlation, §11 Universal.
- The decision entry named `[Read-Back Dependency Posture]` must keep that bracketed name in §Established Decisions. It must also keep, as current truth, the fact obs-plan cites it for. The `verify.readback_fingerprints` attribute line (`:315`) reads today, verbatim: "At HEAD `83d4060` `fingerprint_refs` also carries each incident's triggering-cue fingerprint (the grounded union — measured 2026-09-10, arch §Established Decisions [Read-Back Dependency Posture]), so an emitted-vs-read-back match IS now computable; no span attribute or shipped check computes it". Per obs-plan §4 Scenario: Fingerprint-storm.

## Patterns to follow
- Cite by section anchor plus bracketed decision name, never by `architecture.md:{line}`. obs-plan §4 already does this (`:315`), so its link survives line renumbering. Any new cross-reference written during compaction should do the same.
- Amendment sidecar entries use the Section / Change / Why shape and record a sweep. The sweep lists its grep patterns, a known-positive control and per-file hit counts, and confirms zero hits in the other masters when no lateral fix is owed. See obs-plan-amendments 2026-09-15-structurally-dead-assertion-class-retired and 2026-09-13-p-025-measurement-contract-for-pulse. History moved out of the arch body should land in `architecture-amendments.md` in this shape.
- Evidence behind a moved fact goes to a measurement pointer (a chunk `evidence/…` path) rather than a copy of the narrative in the body. obs-plan §4 Delegated-timing family keeps its measurement citations inline as pointers.

## Anti-patterns to avoid
- Do not let an obs handle drop out of the registry as a side effect of compaction. A missing `CONDUCTOR_SERVICE_NAME` or `CONDUCTOR_ENV` row would leave the "NEVER hardcode service identity" ban without a registered override handle. Per obs-plan §11 Universal.
- If the durable size measurement becomes a CI step, its output must not carry absolute host paths into any telemetry artifact (`agent-latest.jsonl`), because §9's log-conformance check rejects them. Per obs-plan §9 CI Integration and §11 Logs (host-path ban). This matches the fmt-gate precedent: the job log is agent-readable, but the output is not routed into telemetry.

## Contract bindings
- obs ↔ arch §Occupied Resources: obs-plan §3 (identity env vars, sink paths, `CONDUCTOR_AGENT_MODE` read-only trigger) and §9 (a11y artifact) depend on arch registering these handles. The detector proposed at wrap (`D-arch-collision`, second-owner check) would include these obs handles.
- obs ↔ arch §Established Decisions: obs-plan §4 Fingerprint-storm depends on the `[Read-Back Dependency Posture]` anchor.
- obs ↔ a11y: the `runs/a11y/<run_id>.jsonl` artifact registration is jointly owned (obs §3/§9 schema, a11y emits it). Both sides must stay consistent after the row is compacted.

## Acceptance criteria contributions
- (obs) After compaction, grepping the §Occupied Resources body finds each of `CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`, `CONDUCTOR_AGENT_MODE`, `CONDUCTOR_RUNS_DIR`, `RUST_LOG`, `agent-latest.jsonl`, `conductor-tauri.jsonl`, `runs/a11y` and `:4318` (as unused) at least once (per obs-plan §3 Service identity / Logging stack / Log file location; §11 Universal).
- (obs) `grep -F '[Read-Back Dependency Posture]'` over `architecture.md` returns exactly 1 hit inside §Established Decisions. That entry's body still states that `fingerprint_refs` carries each incident's triggering-cue fingerprint (the grounded union), so obs-plan's citation still resolves to the fact it cites (per obs-plan §4 Scenario: Fingerprint-storm).
- (obs) A sweep of `obs-plan.md` for `architecture.md:[0-9]` returns 0 live-body hits, so no obs body citation goes stale when line numbers shift. Such citations in `obs-plan-amendments.md` are history and are left as-is (per obs-plan §12 Obs Decisions Log and sidecar append-only convention).
- (obs) If the size measurement ships as a CI step, its output contains no absolute host path and is not written to `logs/agent-latest.jsonl` (per obs-plan §9 CI Integration).

## Relevant amendment history
- **2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate:** the cross-master sweep's control fired at `architecture.md:37`. This entry is precedent for line-number citations to arch inside obs history, and compaction will make them stale. History is append-only and is not rewritten; P3 should only note it under premise 4 (readers keyed on line numbers).
- **2026-09-10-live-pulse-in-lane-scenario-round:** the sweep cited `architecture.md:7/:60`. `:60` is the 12 099 B [CI/CD] line this chunk targets. It is the same kind of stale-after-compaction line citation in history. The entry also shows the latency-formula statement living in arch `:7/:60`, which must survive as current truth if that line is compacted.
- **2026-09-15-structurally-dead-assertion-class-retired:** the sweep for that marker was recorded once in `architecture-amendments.md` and referenced from obs history. This is the cross-sidecar pattern for history about several masters. It is a model for where moved arch history lands, and it confirms that moving history into the sidecar is sanctioned.
- **2026-09-22-interpretation-proven-live:** A17 cascade, escalated, operator "Amend now, as measured". It re-based §1 Fingerprint-storm row `:131` and §4 `:315`/`:316` onto the grounded-union statement citing "architecture [Read-Back Dependency Posture]". This is the most recent obs amendment that depends on that arch entry, so compacting that entry must not drop the grounded-union fact.
- **2026-09-07-a11y-ci-gate:** registered the a11y violation artifact (`runs/a11y/<run_id>.jsonl`, 13 local / 15 CI top-level keys) on the obs side. The matching arch §Occupied Resources row must survive compaction.
