#!/usr/bin/env bash
# Sequential cargo-mutants runner for this audit run. Run from the repo root.
R=.andromeda/runs/2026-09-23T16-37-53-code-audit
run() {
  unit=$1; shift
  echo "START $unit $(date -u +%FT%TZ)" >> $R/_mutants-progress.log
  cargo mutants -p "$unit" --test-tool=nextest --jobs 2 --output "$R/mutants-$unit" "$@" > "$R/_mutants-$unit.log" 2>&1
  echo "END $unit exit=$? $(date -u +%FT%TZ)" >> $R/_mutants-progress.log
}
run conductor-timeline
run conductor-cli
run conductor-run
run conductor-core --shard 1/4
run conductor-emit --shard 1/4
run conductor-tauri
echo "ALL DONE $(date -u +%FT%TZ)" >> $R/_mutants-progress.log
