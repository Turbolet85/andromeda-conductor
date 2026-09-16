if (-not $env:EDGEWEBDRIVER) {
  Write-Output '[precondition] EDGEWEBDRIVER: NOT SET by the runner image'
  exit 1
}
$driver = Join-Path $env:EDGEWEBDRIVER 'msedgedriver.exe'
if (-not (Test-Path -LiteralPath $driver -PathType Leaf)) {
  Write-Output '[precondition] CONDUCTOR_MSEDGEDRIVER: NOT RESOLVED'
  exit 1
}
Write-Output '[precondition] CONDUCTOR_MSEDGEDRIVER: resolved'
$env:CONDUCTOR_MSEDGEDRIVER = $driver
# No $PSNativeCommandUseErrorActionPreference here. It makes the harness's non-zero a
# TERMINATING error at its `npm run a11y` call, before agent-run.ps1 prints the captured wdio
# output the gate's verdict is read from — the job log then carries one
# NativeCommandExitException line and none of wdio's own (measured 2026-09-07). The harness
# already reads $LASTEXITCODE from the bare command and exits with it, so the preference adds
# no detection and only destroys the diagnostic.
#
# The leg runs under a LIMITED token. A hosted step is elevated by construction, and elevation
# is the established cause of the missing DevTools endpoint — varied with a control on both
# sides on a known-good host (2026-09-12), IsElevatedAdmin True again on this runner in run
# 35079315258 with every rival cause measured away in the same run. The drop is placed HERE
# rather than at the wdio spawn site because msedgedriver launches the app three levels below
# it, so only the step covers every descendant.
& .\scripts\a11y-limited-token-launch.ps1 `
  -Program (Get-Command pwsh).Source `
  -ArgumentList '-NoProfile','-File','scripts/agent-run.ps1','run','--e2e' `
  -WorkingDirectory $PWD.Path `
  -ForwardEnv 'CONDUCTOR_A11Y_STRICT','CONDUCTOR_ENV','CONDUCTOR_MSEDGEDRIVER','EDGEWEBDRIVER'
$legExit = $LASTEXITCODE

# The scheduler does not capture the task's console, so the harness's own captured wdio output
# is the evidence channel. Printing it here keeps the PRINTED verdict readable in the job log;
# the verdict itself decided inside the leg and travels as the exit code.
$a11yLog = Join-Path $PWD 'runs/a11y-e2e.log'
if (Test-Path -LiteralPath $a11yLog) {
  Write-Output '--- captured wdio output (runs/a11y-e2e.log) ---'
  Get-Content -LiteralPath $a11yLog
} else {
  Write-Output '[diag] A11Y_LEG_LOG: absent — the leg did not reach its wdio call'
}
exit $legExit
