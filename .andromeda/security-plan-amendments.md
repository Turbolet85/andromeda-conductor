# Security Plan — Amendments

_Append-only changelog of amendments to `security-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Input Validation (scenario-config boundary row) · §Bootstrap phases (input-validation-library-install)
**Change:** garde version 0.23.0 → 0.22.1 in both spots. The validation contract is otherwise unchanged (garde `#[derive(Validate)]` + `#[garde(custom)]` at load; the `CONDUCTOR_*` path handles canonicalize-and-bounds-check at the edge, OUTSIDE garde).
**Why:** mirrors the arch §Stack downgrade — `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable; user authorized 0.22.1 this session. §Dependency Security carried no garde version pin, so it was untouched.
