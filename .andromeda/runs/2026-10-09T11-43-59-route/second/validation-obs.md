# Obs validation — route draft

## Insert
- Between `Harness verbs on the one form` and `Accepted capability set re-based`: **"Own-log gates on the one form — an engine-less blocked run feeds CI's own-log conformance and unlogged-panic gates; uploaded log kept"** (epoch: `Epoch 3 — The reaction on the timeline`)
  Reason: Per obs-plan §9 Log conformance check and Zero-unlogged-panics gate (§3 Bootstrap phases `obs-ci-gate-wire`), both gates grade a log whose only producer is a single-list run forced Blocked by the shared-machine environment precondition (`/home/turbolet/dev/projects/conductor/.github/workflows/ci.yml:185-193` — `run error-baseline-spike` under `ANDROMEDA_PULSE_DATA_DIR: 'pulse;injection'`). Epoch 5's `Single-list scenario form retired` and `Spawned sidecar and shared-machine gate retired` remove that scenario file, run path and precondition, so the gates lose their subject unless the one form supplies one first; the draft's "conformance gate re-based" in `Run record for the one form` names the run record's gate only.

## Reorder
- Move `Own log over a run of hours` before `Living background`
  Reason: Per obs-plan §3 Heartbeat ticks ("CLI: N/A", premised on 5–120 s runs) and §3 Logging stack's one-writer sink (opened truncating, `/home/turbolet/dev/projects/conductor/crates/conductor-core/src/obs.rs:108-111`), the first hour-long runs — `Living background`'s graded healthy hour and `Four event families`, v4-15's "first long run" — would leave an own log with no liveness line that any second agent-mode command erases. The tick and the intact-log clause depend on neither of those chunks nor on `Run written as it goes`: truncation happens per command and is already reachable through the `status` and `logs` verbs.

## Rewrite
- `Linux-only base CI`: "keeping the supply-chain, secret-scan and static gates" → "keeping the supply-chain, secret-scan, static, own-log conformance and unlogged-panic gates"
  Reason: Per obs-plan §9 (Log conformance check, Zero-unlogged-panics gate, the uploaded `logs/agent-latest.jsonl`) and §10 Build / deploy failure conditions, these gates sit in the `rust` job on `windows-latest` that this entry replaces (`/home/turbolet/dev/projects/conductor/.github/workflows/ci.yml:18`, `:185-241`). They are not static gates — each grades a log a run has just produced — so the drafted kept-list does not carry them to the Linux job (§11 CI: never skip the zero-unlogged-panics gate).
