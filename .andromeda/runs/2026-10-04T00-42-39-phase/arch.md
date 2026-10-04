# arch extract

## Relevance
partial — test-only fix inside the `conductor-tauri` bin crate; arch contributes workspace placement, the Tauri IPC surface's identity, the origin-selection mechanism it records, and the zero-retry determinism bar; no new contract, resource or dependency.

## Constraints
- The change stays inside the `conductor-tauri` member (the Tauri 2 GUI bin, which has no lib target); the crate-per-seam boundary forbids reaching into another seam to obtain the origin, and no cross-seam edge may be added for a test fixture (per architecture §Established Decisions → [Module Boundaries]; §Inherited Defaults → Module boundaries).
- The Tauri command set is a closed, pinned internal IPC surface (start/stop · picker · run-report view · operator-pause + one live-counter `Channel`) and is the complete route-equivalent set; the fix must neither add nor reshape a command, and the origin it supplies is a test-dispatch input, not a new surface (per architecture §Conventions → Interface surfaces; §Occupied Resources, the Tauri-commands and Route-prefixes rows).
- The plan records that the webview origin is decided by the `custom-protocol` feature (`tauri` build.rs `dev = !custom_protocol`, read back through `DEP_TAURI_DEV`), measured 2026-09-01 on the Windows host only — `http://tauri.localhost/` with the feature, `devUrl` without it. So the origin is a function of build cfg AS WELL AS host OS; which inputs the MOCK runtime's origin actually turns on (OS family, the `dev` cfg, both) is research's question, and the plan states no Linux value (per architecture §Infrastructure Patterns → Build system).
- `generate_context!` resolves `build.frontendDist` (`ui/dist`) at compile time, so `npm run build` must precede any workspace `nextest` that compiles `conductor-tauri` — the Linux gate-27 reproduction inherits this precondition (per architecture §Infrastructure Patterns → Build system).
- Tests run under the zero-retry nextest `ci` profile — no retry may mask a host-dependent result, and the fix must make the outcome deterministic per host rather than tolerated (per architecture §Infrastructure Patterns → Build system; §Cross-cutting Patterns → Determinism discipline).
- CI remains Windows-only (`windows-latest` for the original jobs); the plan registers no Linux runner, so the Linux leg is a dev-host measurement, never a new job (per architecture §Infrastructure Patterns → CI/CD approach; §Established Decisions → [CI/CD]).

## Patterns to follow
- Host-conditional behaviour resolved at compile time by `cfg`, the shape the plan records for the sidecar spawn's `#[cfg(windows)]` `console_suppressing_flags()` — a single named source selected per target rather than a literal duplicated at call sites (per architecture §Occupied Resources, the sidecar-spawn row).
- Prefer deriving the origin from the same mechanism that decides it in the framework (the `custom-protocol`/`dev` axis the plan measured) over a hand-maintained per-OS literal table; whether tauri exposes such a source to test code is research's question (per architecture §Infrastructure Patterns → Build system).
- Standalone-build discipline: a member must compile on its own targets (`--bins` for `conductor-tauri`); any test-only helper must not lean on a feature declared only via workspace unification (per architecture §Established Decisions → [Module Boundaries]).

## Anti-patterns to avoid
- Widening the deny-by-default capability posture or the origin check to admit any origin so dispatch passes regardless — the IPC surface stays the pinned set and the test must still fail on a wrong origin (per architecture §Conventions → Interface surfaces; §Occupied Resources, the Tauri-commands row).
- Adding a dependency, a Linux CI job, or a new env handle to carry the origin — the plan's resource registry and CI matrix admit none for this (per architecture §Occupied Resources; §Infrastructure Patterns → CI/CD approach).

## Contract bindings
- arch ↔ tests: the Build-system record of the origin/`custom-protocol` relation binds the test plan's mock-runtime dispatch fixture; the zero-retry `ci` profile binds the test plan's gate-27 run (per architecture §Infrastructure Patterns → Build system).
- arch ↔ security: the IPC surface's origin is the axis the capability ACL is evaluated against; keeping it host-correct (never widened) is a security-plan concern carried through arch's pinned surface (per architecture §Conventions → Interface surfaces).
- Master-staleness note (surface, do not fix here): the plan frames the dev host as Windows throughout (e.g. "core.autocrlf=true is the norm on the Windows dev host", and the Build-system origin measurement is Windows-only), while this chunk's leg runs on a Linux dev host; any Linux origin fact the chunk measures is a wrap-amendment candidate for §Infrastructure Patterns → Build system, not this chunk's code (per architecture §Infrastructure Patterns → Build system).

## Acceptance criteria contributions
- (arch) All edits land in `crates/conductor-tauri/src/` `#[cfg(test)]` code; `git diff --stat` touches no other member, no `Cargo.toml`/`Cargo.lock`, no `capabilities/*.json`, no `tauri.conf.json` (per architecture §Established Decisions → [Module Boundaries]; §Occupied Resources).
- (arch) `cargo build -p conductor-tauri --bins` stays exit 0 standalone after the change (per architecture §Established Decisions → [Module Boundaries]).
- (arch) The six mock-runtime tests pass under the zero-retry `ci` profile on the Linux dev host with zero retries recorded and none skipped/`#[ignore]`d/`cfg`-excluded (per architecture §Infrastructure Patterns → Build system; §Cross-cutting Patterns → Determinism discipline).
