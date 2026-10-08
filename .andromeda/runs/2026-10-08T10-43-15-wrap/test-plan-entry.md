
## 2026-10-08-version-close-on-measured-evidence — the PowerShell red path's stale reason retired
**Section:** §3 → 5-command implementation (the `run` label, Command body)
**Change:** was "its red path is read from the script, not measured (the Linux dev host has no `pwsh`)"; now "its red path is read from the script, not measured — no leg has driven it", with the reading beside it: the Linux dev host has carried `pwsh` 7.6.6 since 2026-10-06, as measured at this chunk's `evidence/version-close.md`, where the script was parsed and never executed, so a missing shell is no longer the reason. The green-path sentence (CI#37209452847) is unchanged.
**Why:** the reason was measured false at this chunk's implement. The limit itself stands: nobody drove the red path at the version close, on the operator's word that no unplanned leg runs there. A host statement in a master is a dated reading, so a chunk that leans on "this host lacks a tool" re-measures it first.
**Kept:** the red path stays unmeasured, and no claim is made that the PowerShell harness runs on the Linux host: a parse is not a run.
**Ref:** .andromeda/runs/2026-10-08T10-43-15-wrap/
