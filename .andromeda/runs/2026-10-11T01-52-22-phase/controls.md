# P5 known-positive controls — 2026-10-11-conductors-window-retired

Each green baseline guard was run, in the gate tool's shell (`/usr/bin/bash -o pipefail -c`), against an input it
must fail on, minted under `controls/` in this run dir and removed after the reading (the two copies of repository
files would otherwise ride the wrap's commit). Run before 2026-10-11T07:56:36Z, the clock read after the controls.

| entry | guard | control input | must read | read |
|---|---|---|---|---|
| 3 | lock prune-only (`comm -13 … \| wc -l`, asserts 0) | `Cargo.lock` with `anyhow` 1.0.104 → 1.0.999 | ≠ 0 | 1 |
| 9 | the eleven rust-job steps by name (asserts 11) | `ci.yml` without the `Secret-scan gate` line | ≠ 11 | 10 |
| 12 | `cli.rs` byte-identical to the base (asserts exit 0) | base `b2f60c88` (before `edc0af8` changed `cli.rs`) | exit ≠ 0 | exit 1 |
| 13 | `bash -n scripts/agent-run.sh` (asserts exit 0) | `if then fi (` | exit ≠ 0 | exit 2 |
| 14 | PowerShell parse error count (asserts 0) | `function f {` + `$x = (1 +` | ≠ 0 | 3 |
