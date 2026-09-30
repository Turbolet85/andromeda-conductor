# Curation — 2026-09-30-the-sr-cause-isolated-on-this-host wrap

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + verification-harness.md: "An admission probe gates EXECUTION on its signature verdict inside the same command" (confidence 0.8)
      Proof: plan entry 11 ran `"$DRIVER_153" --version` after printing `Get-AuthenticodeSignature` status, unconditioned on it; the implement session split it (signature + SHA-256 alone first); operator relay `conductor-wrap-srcause-2026-09-30.md` §1 named it a real plan defect to carry into the harness curation. Its security half ("a binary is never executed before its signature verdict is read") landed through P2 S1 in security-plan rule (b) and its leaf.
    + verification-harness.md: "Pass the portable NVDA ABSOLUTE -c / -f paths" (confidence 0.8)
      Proof: arm S start 1 (2026-09-30T13:36:39Z) — NVDA never logged `NVDA initialized` into `runs/sr-control/nvda-speech.synthetic.log`; a config tree dated 15:36-15:37 local appeared under the NVDA install folder; start 3 with absolute paths read `Config dir:` = the repo copy and the leg ran.
    + host-win32.md: "Windows PowerShell 5 has no [ushort] / [uint] type accelerators" (confidence 0.8)
      Proof: arm S start 2 (13:38:29Z) died `Unable to find type [ushort]` after Edge opened; the scratch parse check had read `parse-errors=0` on the same file; `[uint16]` fixed it and the keytest dry run passed.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (the security-class half of C1: rules/security.md body, this wrap's cascade) · 1 below threshold (attach_observed sweep hazard, 0.4) · 0 conflict · 2 deferred (→ handoff: Edge background relaunch on window close; the Write tool decoding a JSON \u escape into a raw character)
  Load-bearing: "Pass the portable NVDA ABSOLUTE -c / -f paths" → The SR pass regrades on the OS input path
  No-other-home: "Windows PowerShell 5 has no [ushort] / [uint] type accelerators"
  Recurrence: recurrence-despite-learning — host-win32.md 2026-09-23 backslash-pair transport correction (two more commands this wrap written with doubled backslashes, both blocked by the guard) → handoff Deferred learnings.
