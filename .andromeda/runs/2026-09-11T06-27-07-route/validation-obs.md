# Obs validation — route draft

## Rewrite

- `Hosted-runner endpoint cause probed`: "diagnose-only, the reading recorded whatever it says" → "diagnose-only, the reading recorded whatever it says, host paths and runner identity out of the telemetry artifacts"
  Reason: per obs-plan §9 the log-conformance gate fails any artifact line carrying an absolute host path, and the fmt-gate precedent keeps such readings in the job log rather than in `agent-latest.jsonl` — a probe naming module paths and session identity lands squarely on that line.

- `P-025 measurement contract for Pulse`: "which observable, at what resolution, over what window" → "which Pulse-emitted observable, at what resolution, over what window"
  Reason: per obs-plan §4 Delegated-timing family the delegated budgets grade at the harvest tier over Pulse's own tracing leaves and never through `budget_ms`/`effective_deadline_ms`, which bound Conductor's journal-relative emission window, so a contract written against the envelope would be unimplementable.

- `Scenario tier honesty across the corpus`: "every declared tier fits its own phase duration or states why not" → "every declared tier fits its own phase duration inside the closed `slo_tier` enum, or states why not"
  Reason: obs-plan §3 Log format schema and §6 Log Coverage fix `slo_tier` as a closed enum on every envelope record, so the scenarios measured beyond every tier cannot become honest by minting a fourth one.

- `A11y CI gate at an honest terminal`: "or a ratified exclusion on a measured cause with a named owner" → "or a ratified exclusion on a measured cause with a named owner, naming which arm still produces the a11y violation record"
  Reason: obs-plan §9's artifact table registers `runs/a11y/<run_id>.jsonl` as a CI artifact with conformance asserted in-job, so closing the CI half as excluded would silently drop that job's only telemetry artifact.

- `Diagnostic-quality cluster off the drift pin`: "backed by an exercised path rather than `UNBACKED_AUTO`" → "backed by an exercised path, the coverage roll-up's unbacked count and committed matrix moving with them"
  Reason: per obs-plan §4 Coverage-matrix completeness gate that path mints no span — its one boundary line's derived unbacked qualifier is the only observable, and the gate asserts the committed artifact.

- `Full-gate regression over the moved surfaces`: "scenario corpus, the `a11y` job and the live-drive path" → "scenario corpus, the `a11y` job and the live-drive path, with the log-conformance and zero-unlogged-panics gates"
  Reason: obs-plan §10 makes zero-unlogged-panics the tier's always-required SLO invariant and §9 wires it plus log conformance as CI gates; Epoch 4's operator-gated live legs are the paths most able to raise an uncaptured panic.
