
## 2026-10-04-real-model-test-surface-corrective — the feature-gated lint lines bundled into `run`, and the secret-scan skip where no `.git` exists
**Section:** §3 → 5-command implementation (`run` command body) · §9 CI Integration → Lint row · §9 → Live-Pulse scenarios · §6 E2E Test Strategy → Repository-hygiene legs · §4 Unit Test Strategy → Mutation instrument
**Change:**
- §3 → 5-command implementation: the bundled `run` body was nextest → doctest → workspace clippy; it adds `cargo clippy -p conductor-verify --features stub-server --all-targets -- -D warnings` and `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` after the workspace clippy, in both shells (compile + lint only, never run; CI reaches them through the dogfood step).
- §9 Lint row names the two feature lines beside the workspace clippy.
- §9 Live-Pulse scenarios: "out of default `nextest` / `clippy` / release builds" and "invisible to the default lint pass … `preflight_spawn.rs` is the in-repo precedent" now read the workspace `clippy` pass; both owed lines run in the bundled default (as measured at CI#37201730301), the gated targets are still never RUN in CI, and a new feature owes its own line in the bundle.
- §6 Repository-hygiene legs (secret-scan leg): the gate skips with one path-free line where the workspace root holds no `.git` entry, decided by `Path::exists` before any spawn; a listing failure inside a repository still fails it; both arms named.
- §4 Mutation instrument (an addition): `conductor-core`'s tier no longer needs `--copy-vcs true` — measured by a `.git`-less copy run, never a mutation run; the next epoch-boundary audit confirms it.
**Why:** the CARRY (Epoch 5 diagnosis P14(b)), widened by the overseer to both feature sets, puts the gated targets under a gate CI reaches; the secret-scan gate panicked in cargo-mutants' default copy and left `conductor-core` unmeasurable (the Epoch 5 code audit).
**Ref:** .andromeda/runs/2026-10-04T12-32-24-wrap/
