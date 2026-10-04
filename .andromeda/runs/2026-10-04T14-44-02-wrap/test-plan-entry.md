
## 2026-10-04-second-test-surface-corrective — the bundled default stops at its first red line in both shells
**Section:** §3 → 5-command implementation (`run` command body; exit code semantics); §1 → 5-command requirements (`run`)
**Change:**
- `run` body: the bundled default stops at its FIRST non-zero cargo line in both shells — `agent-run.sh` by `set -euo pipefail` (as measured: a planted clippy-only `-D warnings` lint ended `run` at exit 101 on the workspace clippy line, after green nextest and doctest lines); `agent-run.ps1` by an explicit `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` after each of its five cargo lines (since 2026-10-04). It no longer depends on its caller setting `$PSNativeCommandUseErrorActionPreference`, and the script never sets it. A caller that does set it (CI's dogfood step) still sees the red line throw first (step exit 1, CI#34689135760); one that does not gets that line's own cargo exit. The ps1 green path is proven by CI#37209452847; its red path is read from the script, unmeasured on the Linux dev host (no `pwsh`).
- Exit code semantics (key file and §1): was "non-zero = at least one hard `Fail`"; now also a red build, doctest or `-D warnings` lint line in the bundled default, which ends the run at that line (measured exit 101 with every test green). The reported-states clause is unchanged.
**Why:** before this chunk the ps1 bundled default blocked on a red cargo line only when its caller set the native-command preference; the measured sh behaviour already contradicted the hard-`Fail`-only exit wording.
**Kept:** the `--e2e` capture-then-print caveat (a caller setting the preference preempts the printed verdict) stands — the `--e2e` arm is unchanged.
**Ref:** .andromeda/runs/2026-10-04T14-44-02-wrap/
