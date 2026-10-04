# CARRY measurement — does a lint warning fail `agent-run run` and CI's dogfood step?

The basis for the obs-plan §10 `cargo clippy` failure-condition amendment. Measured at /implement on 2026-10-04 (Linux
dev host, tree = base `dab66dc` + this chunk's edits), plus one recorded CI run. Host paths are elided throughout.

## 1. `scripts/agent-run.sh` — MEASURED: a lint warning fails the bundled default

The one-shot CARRY-sh form of `plan.md` §Test Commands, run once from the project root:

```
mkdir -p target/carry && f=crates/conductor-emit/src/identity.rs && cp "$f" target/carry/identity.rs.bak && trap 'cp target/carry/identity.rs.bak crates/conductor-emit/src/identity.rs' EXIT && printf '\n#[allow(dead_code)]\nfn planted_lint(v: &[u8]) -> bool {\n    v.len() == 0\n}\n' >> "$f" && { bash scripts/agent-run.sh run; test $? -eq 101; }
```

- **Exit of `bash scripts/agent-run.sh run`: 101**, read from the bare command by the form's own `test $? -eq 101`
  (the form exited 0, 2026-10-04T14:24:45Z → 14:25:01Z).
- The run reached the workspace clippy line and stopped there. The workspace nextest before it was green
  (`1204 tests run: 1204 passed, 0 skipped`) and so were the doctests; neither feature-gated clippy line ran, because
  the log ends at the workspace line's failure (`set -euo pipefail`, `agent-run.sh:20`).
- Clippy's printed error lines, verbatim:

```
error: length comparison to zero
   --> crates/conductor-emit/src/identity.rs:179:5
    = note: `-D clippy::len-zero` implied by `-D warnings`
error: could not compile `conductor-emit` (lib) due to 1 previous error
error: items after a test module
   --> crates/conductor-emit/src/identity.rs:54:1
    = note: `-D clippy::items-after-test-module` implied by `-D warnings`
error: could not compile `conductor-emit` (lib test) due to 2 previous errors
```

  The planted `len_zero` failed the lib target. The lib-test target ALSO raised `items_after_test_module`, because
  the plant was appended after the file's `mod tests`; the plan predicted `len_zero` alone. Both are clippy lints that
  rustc does not raise, so the red is still clippy's alone. The nextest and doctest lines compiled the same file
  green before clippy ran.
- Restoration: `identity.rs`'s sha256 was identical before and after (`ae7d2940…52f5`); the planted-lint guard
  (`grep -c planted_lint crates/conductor-emit/src/identity.rs`) reads 0.

## 2. CI's dogfood step — RECORDED witness: a native non-zero fails the step under CI's preference

CI#34689135760, sha `e3ff4e5f` (2026-09-12, conclusion `failure`), re-read at /implement with
`gh run view 34689135760 --log-failed`. The `rust` job's "Test + lint (dogfood agent-run)" step (`shell: pwsh`)
sets `$PSNativeCommandUseErrorActionPreference = $true` before `.\scripts\agent-run.ps1 run` (`.github/workflows/ci.yml`).
The step died with:

```
NativeCommandExitException: <runner path>\scripts\agent-run.ps1:280
Program "cargo.exe" ended with non-zero exit code: 100
##[error]Process completed with exit code 1.
```

(the runner's checkout path is elided). This was a **nextest** red (exit 100), not a clippy red. No CI run has ever
carried a clippy-only red (research.md §Files inspected: every other failure in the 40-run listing is the a11y job, or
the supply-chain step at `30156520449`), so CI's failure on a CLIPPY line is inferred from this mechanism, not
observed.

## 3. `scripts/agent-run.ps1` — FIXED in the script; the red path is UNMEASURED on this host

Before this chunk, the `''` arm checked no `$LASTEXITCODE` after its cargo lines, and the script never sets
`$PSNativeCommandUseErrorActionPreference`, so a red cargo line ended the bundled default non-zero ONLY when the CALLER
had set that preference (as CI does, §2). After this chunk the arm carries `if ($LASTEXITCODE -ne 0) { exit
$LASTEXITCODE }` immediately after EACH of its five cargo lines (workspace nextest · doctest · workspace clippy · the
two feature-gated clippy lines), the file's own idiom (`agent-run.ps1:212`). So the bundled default stops at the first
non-zero cargo line, with that line's exit, whoever calls it.

**That red path is UNMEASURED.** This Linux dev host has no `pwsh` (`command -v pwsh powershell` → nothing), and CI's
dogfood run proves only the GREEN path through the new checks. The claim rests on reading the script, not on a run.

## 4. The `--e2e` arm — no clippy line, so a lint warning cannot fail the `a11y` job

The `--e2e` arm runs no clippy line in either shell (`agent-run.sh:303-316` and `agent-run.ps1:351-370`: 0 `clippy`
occurrences each). It builds the release binary and runs the wdio leg, so a lint warning cannot fail the `a11y` job,
whose entry point it is.

## Summary for the obs §10 line

A `-D warnings` lint fails `agent-run run`'s bundled default:
- in `sh`, measured: exit 101 at the workspace clippy line, by `set -euo pipefail`;
- in `ps1`, by the per-line `$LASTEXITCODE` check this chunk adds. The red path is unmeasured on the Linux host.

It also fails CI's dogfood step, under the step's own `$PSNativeCommandUseErrorActionPreference`. The recorded witness
CI#34689135760 is a nextest red, not a clippy red. The `--e2e` arm runs no clippy line.
