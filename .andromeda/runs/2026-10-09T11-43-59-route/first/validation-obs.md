# Obs validation — route draft

## Insert
- Between `Run written as it goes` and `Engine memory and database growth measured`: **"Own log over a run of hours — periodic tick line while the run lives; size bounded; a second command leaves the running log intact"** (epoch: `Epoch 7 — The living world and the long run`)
  Reason: Per obs-plan §3 Heartbeat ticks ("CLI: N/A") and §6 Sink configuration ("File rotation: N/A"), both premised on 5–120 s runs, the own log has no tick and no size bound, and §3 Logging stack's one-writer sink is reopened truncating by every agent-mode invocation (`crates/conductor-core/src/obs.rs:111`, `crates/conductor-cli/src/main.rs:28`), so following progress from a second command would erase the running run's log. The draft's `Run written as it goes` names the record and journal only, so nothing ahead of the first hours-long run covers the own log.

## Reorder
- Move `Credential hygiene` before `Two-host path reachable`
  Reason: Per obs-plan §11 PII Scrubbing (redaction at processor stage, never sink-only) and §3 Bootstrap phases `pii-scrubbing-wire`, the shipped value scrub masks host paths only (`crates/conductor-core/src/redact.rs:123-142`) while the allowlisted `message` field carries boundary error text. As drafted, `Two-host path reachable` and `Channel refusals` would hold a token and door credential with nothing keeping them out of Conductor's own log, journal and report.
