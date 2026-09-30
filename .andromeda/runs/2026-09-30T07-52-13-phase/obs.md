# obs extract

## Relevance
partial — obs-plan never names the mutation gate, `scripts/mutation-gate.py` or `mutation-roster.toml` (a case-insensitive grep for `mutation|mutants|.py|python` finds 0 hits). No instrumentation obligation follows. What applies is the plan's output-hygiene and agent-readability discipline, carried over to a gate whose printed verdict an agent reads.

## Constraints
- The gate sits outside obs-plan's instrumentation scope. §1 Instrumentation scope lists only the workspace crates, the runtimes and the build/CI pipeline, and it rates the pipeline "Not-instrumentable". So this chunk owes no span, metric or `tracing` record. §11 Spans / Traces fixes a bounded set of span names, so this chunk must add no new span name to it (per obs-plan §1 Instrumentation scope; §11 Spans / Traces).
- An agent must be able to read every signal the gate produces without a human reviewing it. Its verdict should come as its exit code plus machine-parseable lines, never as prose that someone has to interpret (per obs-plan §11 Universal (agent-driven specific); §11 CI).
- §11 Logs bans unstructured output that an agent cannot parse. It is written for Conductor's own self-observation, but applied to this gate it means one failure per line, with the class (`missed`/`timeout`), the direction (unexpected/absent) and the coordinate-free `(file, mutation)` key as fields a reader can separate (per obs-plan §11 Logs).
- No absolute host path may appear in anything the gate or its test prints. That includes the tally directory's resolved location and any fixture `mutants.out/` path, so file keys stay repo-relative (per obs-plan §11 Logs "NEVER leak absolute host paths"; §9 Log conformance check host-path list).
- Gate and test output belongs in the job log or on the terminal only. It must never be written into a telemetry artifact (`logs/agent-latest.jsonl`, `runs/**`). §9 already applies this rule to the fmt diff row and the repository-hygiene rows (per obs-plan §9 Pipeline integration).
- The fixture proof must be deterministic and must not retry. The zero-flakiness invariant rejects retry-once policies because they hide real failures. This matches the scope's refusal to grade `unviable.txt` as an exact roster (per obs-plan §10 Always-required SLO invariant; §11 SLO).

## Patterns to follow
- The §9 repository-hygiene gate row: each hit is one line naming a repo-relative location and its rule, and the gate prints its own verdict line. This is the closest existing model for how the gate should print a failure (per obs-plan §9 Pipeline integration, Repository-hygiene gates row).
- If the gate's own regression test ends up as a Rust test binary under nextest (P3 decides), its results reach the agent through nextest's machine-readable output. The plan treats that as the agent-read path for unit tests (per obs-plan §9 Pipeline integration, Unit tests row).
- Keep the gate's output apart from Conductor's two record shapes. §9 separates the §3 self-observation baseline from the §6 run-report envelope. Gate output is neither, and it must not be emitted in either shape (per obs-plan §9 Log conformance check).

## Anti-patterns to avoid
- A human-reviewed reading as the proof. The scope's own item 4 (a committed, re-runnable test rather than a one-off manual reading) is this ban applied (per obs-plan §11 Universal; §11 CI "NEVER use human-review-gated log analysis without machine-parseable export").
- Printing a resolved absolute tally or fixture path in a failure line (per obs-plan §11 Logs).
- Retrying a graded run, or any tolerance that lets a host-dependent count pass on a second try (per obs-plan §11 SLO; §10).

## Contract bindings
- obs ↔ tests: tests' §4/§9 decide whether the gate's own test runs in CI. The mutation run is never a CI stage. If the test does run in CI, it falls under obs §9's rule that output is agent-readable in the job log and never lands in a telemetry artifact. Whether any existing CI step already collects a python test's output is P3's question (per obs-plan §9 Pipeline integration).
- obs ↔ security: host-path hygiene in the gate's printed lines. The host-path list in obs §9 matches security's artifact-hygiene rule. The security plan owns what counts as a leak (per obs-plan §9 Log conformance check; §11 Logs).

## Acceptance criteria contributions
- Every failure line the gate prints on each fixture arm is a single line. Each one names its tally class (`missed` or `timeout`), its direction (unexpected or absent) and a repo-relative `(file, mutation)` key. A drive-letter / `/home` / `/Users` / `%APPDATA%` sweep of the captured output finds nothing that is a real host path (per obs-plan §11 Logs; §9 Log conformance check).
- The gate's verdict can be read from its exit code: 0 on the pass fixture, non-zero on the unexpected-timeout, absent-timeout and `missed` fixtures. Each is asserted by the committed test, never by a manual reading (per obs-plan §11 Universal).
- Running the gate's test writes no file under `logs/` or `runs/` (per obs-plan §9 Pipeline integration).
- The gate's test contains no retry or re-run path, and it grades the same on every run over the same fixtures (per obs-plan §10 Always-required SLO invariant; §11 SLO).
