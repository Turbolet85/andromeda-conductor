#!/usr/bin/env bash
# Tier C — mutation, touched units only, baseline's pinned firing form.
# Sequential (memory), incremental per unit, resumable by artifact presence.
set -u
RD=".andromeda/runs/2026-09-02T15-49-17-code-audit"
for unit in conductor-run conductor-verify conductor-tauri; do
  out="$RD/mutants-$unit"
  if [ -f "$RD/done-$unit" ]; then
    echo "[$unit] already done — skipping (resume)"
    continue
  fi
  echo "=== [$unit] start $(date -u +%FT%TZ) ==="
  timeout 2400 cargo mutants -p "$unit" --test-tool=nextest --jobs 2 --output "$out" \
    > "$RD/_mutants-$unit.log" 2>&1
  rc=$?
  echo "$rc" > "$RD/done-$unit"
  echo "=== [$unit] exit $rc  $(date -u +%FT%TZ) ==="
  tail -6 "$RD/_mutants-$unit.log"
done
echo "ALL UNITS COMPLETE $(date -u +%FT%TZ)"
