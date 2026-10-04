
## 2026-10-04-real-model-test-surface-corrective — the secret-scan gate's skip line in the hygiene row
**Section:** §9 Pipeline integration → Repository-hygiene gates row
**Change:** the row's output cell adds the secret-scan gate's one path-free skip line, `secret-scan gate: skipped — no git repository at the workspace root`, printed (and the gate passing) only where the workspace root holds no `.git` entry; a listing failure inside a repository stays red. Like the hit lines, it reaches the job log only.
**Why:** the gate gained a skip path for VCS-less copies; the row enumerates what the gate prints.
**Ref:** .andromeda/runs/2026-10-04T12-32-24-wrap/
