# obs extract

## Relevance
Partial — the modify-set is a build-time verification gate, which obs-plan §1 classes Not-instrumentable; obs contributes CI-gate and no-new-telemetry constraints only, and mandates no instrumentation for this chunk.

## Constraints
- The surface this chunk touches is build-time verification (the `cargo build / CI/CD pipeline` row whose enumerated set includes cargo-nextest), which obs-plan §1 classifies **Not-instrumentable** — so obs-plan mandates no span, no log line and no metric from this chunk (per obs-plan §1 Obs Scope Summary).
- obs-plan §11 (CI) requires every CI run to pass `cargo-nextest` alongside log conformance and panic-hook verification — the standing red workspace gate is therefore an obs CI-gate failure as well as a tests failure, and the chunk's headline green is a shared pass condition (per obs-plan §11 CI).
- obs-plan §4's must-trace set is a fixed seven paths on which §1's Minimal-tier justification rests; the only gate named there is the coverage-matrix completeness gate over `coverage-matrix.md`, a different artifact. This chunk must not add an eighth must-trace row for the ledger gate (per obs-plan §1 Critical paths, §4).
- obs-plan §11 (Spans / Traces) fixes a bounded span-name set; no member names a ledger/report gate, and the set was deliberately closed against a `db.*` wildcard — any span minted for this gate would fall outside it (per obs-plan §11 Spans / Traces).
- obs-plan §9's log-conformance gate rejects absolute host-file paths in any field of `logs/agent-latest.jsonl`, and §9's own Lint row sets the precedent that a build-time gate's path-bearing output stays agent-readable in the CI **job log** and is never routed into a telemetry artifact. If the re-worded `:153` / `:208` assertion messages name a scan-resolved version directory, that precedent binds (per obs-plan §9 CI Integration).
- obs-plan §2/§9 require the gate's outcome to be machine-consumable with no human-review step; for unit tests that path is the uploaded cargo-nextest JSON artifact (per obs-plan §2 agent-readable invariants, §9 artifact table).
- obs-plan §9/§10's zero-unlogged-panics gate greps stderr plus `agent-latest.jsonl` for `^thread.*panicked`, which is exactly the shape a failing Rust assertion emits. Whether that gate's stderr scope reaches the nextest job — i.e. whether the standing red is already visible to it — is research's question (per obs-plan §9, §10).

## Patterns to follow
- The sibling gate in the same crate, §4's coverage-matrix completeness gate, asserts through a CI-wired test target with **no span minted** and the assertion carried by no log field at all — the ledger gate should keep the identical shape (per obs-plan §4 Coverage-matrix completeness gate).
- Derived-over-baked: §4's gate quantities were twice de-hardcoded so a count/set rides its live source rather than a literal. Generalising the id-space predicate is the same move one axis over from the directory resolution (per obs-plan §4 denominator semantics).
- Where a gate does need an observable, §4's precedent renders derived values into the already-allowlisted `message` value rather than introducing new field names, because the §11 field allowlist drops non-allowlisted names at the processor stage (per obs-plan §4, §11 PII Scrubbing).
- Build-time gate diagnostics are consumed by an agent from the job log, with no telemetry-artifact write (per obs-plan §9 Pipeline integration, Lint / typecheck row).

## Anti-patterns to avoid
- Do not add `tracing` / `tracing-subscriber` to the gate test, or emit self-obs lines to diagnose a ledger mismatch — §1 classes this surface Not-instrumentable and §4's sibling gate keeps its assertion out of the log stream (per obs-plan §1, §4).
- Do not mint a span for the ledger gate; any name would sit outside §11's bounded span-name set, which was closed rather than widened (per obs-plan §11 Spans / Traces).
- Do not route path-bearing assertion output (scan-resolved version directories) into `logs/agent-latest.jsonl` or any telemetry artifact — §9's conformance check rejects absolute host-file paths (per obs-plan §9, §11 Logs).

## Contract bindings
- **obs ↔ tests harness:** §9's Unit-tests row makes `cargo-nextest` JSON (`--message-format libtest-json`) the uploaded CI artifact through which this gate's pass/fail reaches an agent; format ownership sits with test-plan, obs owns the artifact-upload and agent-readability side (per obs-plan §9).
- **obs ↔ CI:** §11's CI ban makes the green workspace nextest run an obs requirement, not only a tests one — the same acceptance serves both domains.
- No obs ↔ security or obs ↔ a11y binding fires: the chunk handles no user data, emits no violation record, and per scope introduces no dependency delta touching §3's behavioral no-SDK invariant.

## Acceptance criteria contributions
- `cargo nextest run --workspace --profile ci` returns green — obs-plan's CI ban requires all CI runs pass cargo-nextest, so the red gate fails an obs gate too (per obs-plan §11 CI).
- The chunk mints no new span name and adds no must-trace path: §11's bounded span-name set and §4's seven-path critical-path table are byte-unchanged by this work (per obs-plan §4, §11 Spans / Traces).
- No new self-obs emission from the gate: the crate's test target gains no `tracing` dependency and no log line, matching the Not-instrumentable classification of build-time verification (per obs-plan §1).
- Any re-worded assertion message that names a scan-resolved directory stays confined to the nextest job log and is not written into `logs/agent-latest.jsonl` or another telemetry artifact, whose conformance check rejects absolute host-file paths (per obs-plan §9).

## Relevant amendment history
- **2026-09-06-coverage-completeness-gate** — nearest precedent: same crate (`conductor-report`), same "gate over a committed ledger artifact" shape. Re-based Critical Path 6 onto what shipped — the gate mints NO span, its derived counts ride the allowlisted `message`, and the retired span chain was unbuildable because its names sat outside §11's bounded set and outside `ALLOWLISTED_FIELDS`. Establishes that a ledger-class gate carries no telemetry obligation of its own.
- **2026-08-08-sut-capability-manifest** and **2026-08-09-out-of-scope-classification-treatment** — de-hardcoded the coverage gate's span attributes so counts/sets stay manifest-derived and never literals ("the span spec asserted a fixed 60 the accepted set no longer defines"). Same defect class as this chunk's baked `v2-` id space: a literal that a version/SUT advance silently invalidates.
- **2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate** — added the fmt gate to §1's build-time verification enumeration (confirming build-time gates remain Not-instrumentable) and recorded that its path-bearing output is job-log-only, never written into a telemetry artifact because §9's log-conformance check would reject it. Directly governs the `:153` / `:208` message re-wording.
- **2026-08-21-delegated-timing-budgets-proven** — deliberately declined to add an eighth row to §4's critical-path table, recording the seven-path set as a tier-justification input. The direct precedent for not growing the must-trace set from this chunk.
- **2026-09-06-run-report-envelope-conformance-gate** — narrowed `conductor-report`'s manual-span rule and recorded a write deliberately left uninstrumented because it is off the must-trace paths, holding §11's bounded set at `db.insert_run` with no `db.*` widening. Same crate; supports the no-new-span position here.
