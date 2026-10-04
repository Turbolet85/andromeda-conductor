
## 2026-10-04-real-model-test-surface-corrective — the secret-scan gate skips only where no `.git` exists
**Section:** §Secret Management → Secret-scan gate shape
**Change:** the gate now skips with one path-free stderr line and passes where the workspace root holds no `.git` entry at all (a VCS-less copy, such as cargo-mutants' default copy). The check is `Path::exists` before any spawn, so a `.git` FILE counts as a repository and no spawn, argv element or env var is added. A `git ls-files` failure inside a repository still fails the gate; CI always runs over a checkout, behind the unchanged presence guard.
**Why:** the gate panicked in a VCS-less copy and left `conductor-core` unmeasurable by mutation (the Epoch 5 code audit). The overseer ruled that the skip fires only with no VCS at all and that a test pins both arms.
**Ref:** .andromeda/runs/2026-10-04T12-32-24-wrap/
