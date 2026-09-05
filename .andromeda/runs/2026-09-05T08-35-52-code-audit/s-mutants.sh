#!/usr/bin/env bash
# Tier C runner — sequential per-unit cargo-mutants under the operator-ratified budget (15-min cap, --jobs 2).
# Each unit writes incrementally to {run_dir}/mutants-{unit}/mutants.out/; a stopped run resumes by artifact presence.
# core runs as --shard 1/4 (126 of 504 mutants): the portion that fits the cap without killing cargo's child tree.
set -u
RUN="$1"
cd /d/dev/projects/conductor || exit 2
run_unit() {
  local unit="$1"; shift
  local extra="$*"
  if [ -f "$RUN/_done-$unit" ]; then echo "skip $unit (done marker present)"; return; fi
  echo "=== $unit start $(date -u +%FT%TZ) extra=[$extra] ==="
  local t0=$(date +%s)
  cargo mutants -p "$unit" --test-tool=nextest --jobs 2 $extra --output "$RUN/mutants-$unit" > "$RUN/_mutants-$unit.log" 2>&1
  local rc=$?
  local t1=$(date +%s)
  echo "exit=$rc seconds=$((t1-t0))" > "$RUN/_done-$unit"
  echo "=== $unit exit=$rc seconds=$((t1-t0)) $(date -u +%FT%TZ) ==="
}
run_unit conductor-tauri
run_unit conductor-run
run_unit conductor-verify
run_unit conductor-cli
run_unit conductor-core --shard 1/4
date -u +%FT%TZ > "$RUN/_mutants-all-done"
echo "ALL DONE"
