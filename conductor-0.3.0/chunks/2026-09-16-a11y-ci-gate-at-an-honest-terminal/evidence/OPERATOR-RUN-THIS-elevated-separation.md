# Operator: two legs from ONE elevated PowerShell

**Purpose.** Separate *administrator role* from *integrity level* as the blocker for WebView2 session
creation. Probe run 35111618735 showed `runas /trustlevel:0x20000` drops the role (`IsElevatedAdmin`
False) but **not** the integrity level (still High) — so the two have never been varied independently,
and the runner cannot do it. This host can, in minutes, with one consent.

**The binary question is session creation, NOT the suite verdict.** Two assertions (`:384`, `:397`) are
independently broken by WebView2 153 on this host and will fail in BOTH legs; ignore them. The only
thing being read is whether a driven session is created at all.

## Before you start

Open **one elevated PowerShell** (Run as administrator) and `cd` to the repo. Everything below runs in
that one session. Nothing here writes to the repo except gitignored files under `runs/`.

## Leg A — direct, elevated: True + High

```powershell
$env:CONDUCTOR_MSEDGEDRIVER = [Environment]::GetEnvironmentVariable('CONDUCTOR_MSEDGEDRIVER','User')
Remove-Item runs\a11y-token-witness.log,runs\a11y-leg-console.log,runs\a11y-leg-exit.txt -Force -EA SilentlyContinue
.\scripts\a11y-token-witness.ps1
Copy-Item runs\a11y-token-witness.log runs\LEG-A-witness.log -Force
Copy-Item runs\a11y-leg-console.log   runs\LEG-A-console.log -Force
```

## Leg B — under runas /trustlevel, elevated caller: False + High

```powershell
Remove-Item runs\a11y-token-witness.log,runs\a11y-leg-console.log,runs\a11y-leg-exit.txt -Force -EA SilentlyContinue
.\scripts\a11y-limited-token-launch.ps1 `
  -Program (Get-Command pwsh).Source `
  -ArgumentList '-NoProfile','-File','scripts/a11y-token-witness.ps1' `
  -WorkingDirectory $PWD.Path `
  -ForwardEnv 'CONDUCTOR_MSEDGEDRIVER' `
  -TimeoutSeconds 900
Copy-Item runs\a11y-token-witness.log runs\LEG-B-witness.log -Force
Copy-Item runs\a11y-leg-console.log   runs\LEG-B-console.log -Force
```

## Then paste back these four lines

```powershell
Select-String -Path runs\LEG-A-witness.log,runs\LEG-B-witness.log -Pattern 'IsElevatedAdmin|IntegrityLevel:|WindowStation|UserInteractive|SessionName'
Select-String -Path runs\LEG-A-console.log,runs\LEG-B-console.log -Pattern 'RUNNING in wry|webview2 .* windows|DevToolsActivePort|Spec Files:'
```

## How each outcome reads

The discriminator is the **`[webview2 <version> windows]` banner** (session created) versus
**`DevToolsActivePort file doesn't exist`** (not created).

| Leg A (True+High) | Leg B (False+High) | Conclusion |
|---|---|---|
| no session | no session | **Role exonerated, HIGH INTEGRITY implicated.** Option (b) — a `CreateRestrictedToken` + `SetTokenInformation` launch that lowers the label — gets a measured premise before any P/Invoke is written. |
| no session | **session created** | **The ROLE was the discriminator.** The runner's False+High red then contradicts it, relocating the cause to the runner (window station is the next suspect) and killing option (b) before it is written. |
| session created | session created | 2026-09-12's elevated red does not reproduce on this host today — the control has moved again, and neither variable is currently the blocker here. |

Any row is a result. The third is not a failure of the test; it would mean the host changed under us
again, which is worth knowing before spending a CI probe.

## Why the window-station readings are in both legs

Both legs also print `WindowStation`, `UserInteractive` and `SessionName` from inside the leg. That axis
has never been varied and is currently the last candidate standing *by elimination rather than by
measurement*. This is the cheapest place it will ever be observed beside a known-good, so it is recorded
even though these two legs do not vary it.

## Safety notes

- No repository file is written; `runs/` is gitignored.
- The launcher restores any User-scope handle it forwards, in a `finally`.
- Leg B leaves no scheduled task — it uses `runas`, not the scheduler.
- If either leg leaves processes behind:
  `Get-Process conductor-tauri,msedgedriver,tauri-driver,node -EA SilentlyContinue | Stop-Process -Force`
