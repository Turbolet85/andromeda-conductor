
## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — `wdio.conf.ts` sets `CONDUCTOR_SCENARIOS_DIR` for the driven suite too
**Section:** §Occupied Resources → Environment variables → `CONDUCTOR_SCENARIOS_DIR`
**Change:** the entry names its harness setter — `wdio.conf.ts` sets it for the `driven` / `sr-empty` / `sr-error` suites (was: the override alone). The driven suite's value is `runs/driven/scenarios` (repo-relative, harness-owned, git-ignored, re-created per run with exactly the two `[[checklist]]` scenarios); no new handle, no second owner — the app stays its sole reader, the one tauri-driver spawn site its setter.
**Why:** the driven arm now seeds its own two-scenario catalog so one run reaches two holds behind one canary. The line was kept to one clause because §Occupied Resources stood at 38 028 / 38 115 B (87 B of headroom) before this pass; after it, 38 087 B, within target.
**Kept:** §Established Decisions untouched (38 111 B, 4 B of headroom); `runs/driven/scenarios/` not registered as its own artifact line (per-item content inside the already-registered gitignored `runs/` tree).
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/
