
## 2026-10-02-p-075-assert-round-against-pulse — the span pair in its own dir; the read-back posture re-measured at S
**Section:** §Occupied Resources → `runs/live-suite/{leg}.jsonl` · §Occupied Resources → `runs/span-landing/span-{a,b}.jsonl` (new) · §Infrastructure Patterns → Directory structure (`live-suite/`, `span-landing/`) · §Established Decisions → [Read-Back Dependency Posture] (the canary-carrier passage)
**Change:**
- `runs/span-landing/span-{a,b}.jsonl` is registered. It holds the span-landing pass's drive journals, has no `CONDUCTOR_*` handle and is git-ignored. Its two named files are cleared by `rm -f` before drive A. `span_landing_live` reads it via `capture_paths`, and a stale pair is refused. The tree gains a `span-landing/` line.
- The `live-suite/` entry and its tree comment lose their second, operator-local writer. That writer was registered in the 2026-10-01 per-run span-identity entry's third bullet, the only part of that entry this retires. The suite's `rm -f …/*.jsonl` no longer reaches the pair.
- To hold §Occupied Resources within target, the entry's wording was compressed without dropping a fact: the "else a suite read sees only the last leg" gloss went, along with "(not `run_id`-stemmed)" and a shorter stale-file clause.
- The posture passage now reads that from `83d4060`, `fingerprint_refs` are the model's refs ∪ the cue fingerprint. That set is written at creation only, and a dedupe never updates it. At Pulse S `03ec944`, Conductor's emitted fingerprint is a member (4 refs, 3 `det-*`), and that incident's `retrieve_report` reads `degraded_mode: false`. The evidence is cited as measured at the chunk's `evidence/round-ledger.md`.
- The passage was "as measured at" the 2026-09-10 `leg1` envelope, quoting its 32-hex value, and named `inference_runtime.rs`. Both citations moved here, as history, to hold §Established Decisions within target.
- Bytes: §Established Decisions 38097 B and §Occupied Resources 38111 B, both ≤ 38115 B.
**Why:** this chunk moved the span pair out of `live-suite/`, which discharges the route-owned move. The P-075 round re-measured read-back content fidelity at a newer Pulse HEAD. §Established Decisions already states `degraded` per-read-back, so no contrary arch claim remained.
**Kept:** `:70`'s KnownResidual example "a `retrieve_report` result returned under `degraded_mode`" and the per-read-back clause, both true. The `incident_events` paragraph is unchanged: no MCP tool reads it.
**Ref:** .andromeda/runs/2026-10-02T12-53-46-wrap/
