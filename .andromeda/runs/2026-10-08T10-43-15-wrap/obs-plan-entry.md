
## 2026-10-08-version-close-on-measured-evidence — §10's clippy line drops the stale `pwsh` reason
**Section:** §10 SLO Invariants & Telemetry Budgets → Build / deploy failure conditions, the `cargo clippy … -- -D warnings` line
**Change:** the parenthesis was "(its red path unmeasured — the Linux dev host has no `pwsh`)"; it now reads "(its red path unmeasured — no leg has driven it; …)", followed by the reading: `pwsh` 7.6.6 on the Linux dev host since 2026-10-06, as measured at this chunk's `evidence/version-close.md`, where the script was parsed and never executed. The rest of the line stands.
**Why:** the same sentence as test-plan §3's `run` contract, retired by the same measurement in the same pass, so the two plans agree. The limit stands: no leg has driven the PowerShell red path.
**Ref:** .andromeda/runs/2026-10-08T10-43-15-wrap/
