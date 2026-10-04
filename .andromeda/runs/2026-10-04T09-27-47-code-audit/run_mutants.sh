#!/usr/bin/env bash
# C1 runner: per-unit sequential invocations, 15-minute wall-clock cap each, outputs under the run dir.
# Resumes by artifact presence: a unit whose c-mutation-{unit}.json exists is skipped.
RD=.andromeda/runs/2026-10-04T09-27-47-code-audit
for u in conductor-faults conductor-timeline conductor-tauri conductor-report conductor-cli conductor-run conductor-verify conductor-core conductor-emit; do
  [ -f "$RD/c-mutation-$u.json" ] && continue
  shard=()
  case "$u" in conductor-core|conductor-emit) shard=(--shard 1/4) ;; esac
  echo "start $u $(date -u +%FT%TZ)" >> "$RD/_mutants.log"
  timeout 900 cargo mutants -p "$u" --test-tool=nextest --jobs 4 --output "$RD/mutants-$u" "${shard[@]}" > "$RD/_mutants-$u.log" 2>&1
  echo "end $u exit=$? $(date -u +%FT%TZ)" >> "$RD/_mutants.log"
done
echo "ALL-DONE $(date -u +%FT%TZ)" >> "$RD/_mutants.log"
