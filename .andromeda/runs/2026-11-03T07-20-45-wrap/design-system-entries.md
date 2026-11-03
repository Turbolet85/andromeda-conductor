## 2026-11-03-linux-only-base-ci — the cli surface's Platform line names Linux alone
**Section:** §Surface: cli (body: the **Platform:** line)
**Change:**
- The cli surface's Platform reads `Linux terminal` (was `Windows / macOS / Linux terminal`); the rest of the line stands: headless `conductor-cli` (`scripts/agent-run.sh`), the source of truth + release gate, line-oriented (ratatui full-TUI deliberately omitted).
- macOS leaves with Windows: the line names the measured set only, not the plan's narrower "drops Windows".
**Why:** the report's Harness / gate surface retires the Windows CI runner and the PowerShell twin `scripts/agent-run.ps1`, leaving `scripts/agent-run.sh` the one harness, measured green on the Linux dev host and the one `ubuntu-latest` CI job; no leg witnesses a Windows or macOS terminal. Its Counts moves the cli Platform line from `Windows / macOS / Linux terminal` to Linux.
**Ref:** .andromeda/runs/2026-11-03T07-20-45-wrap/
