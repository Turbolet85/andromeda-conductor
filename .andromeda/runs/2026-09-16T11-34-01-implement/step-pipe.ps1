$app = Join-Path $PWD 'target\release\conductor-tauri.exe'
if (-not (Test-Path -LiteralPath $app)) { Write-Output '[diag] pipe-route app binary: NOT BUILT'; exit 0 }
$udf = Join-Path $env:RUNNER_TEMP 'wv2-pipe'
Remove-Item -LiteralPath $udf -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $udf | Out-Null
$env:WEBVIEW2_USER_DATA_FOLDER = $udf
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = '--remote-debugging-pipe'
$p = Start-Process -FilePath $app -WorkingDirectory $PWD -PassThru
try {
  $seen = -1
  foreach ($s in 1..60) {
    Start-Sleep -Seconds 1
    if (Test-Path -LiteralPath (Join-Path $udf 'EBWebView')) { $seen = $s; break }
  }
  $label = if ($seen -ge 0) { "${seen}s" } else { 'never within 60s' }
  Write-Output "[diag] pipe-route EBWebView first seen: $label"
  Write-Output "[diag] pipe-route DevToolsActivePort present: $(Test-Path -LiteralPath (Join-Path $udf 'EBWebView\DevToolsActivePort'))"
  Write-Output "[diag] pipe-route app alive: $(-not $p.HasExited)"
  # The question the route turns on: does the loader ACCEPT the pipe switch, or reject the argument
  # and fail to bring the webview up at all? A live app with its profile created is acceptance;
  # an exited app is rejection, and says the route is closed without any token work.
} finally {
  $env:WEBVIEW2_USER_DATA_FOLDER = $null
  $env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = $null
  Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
}
