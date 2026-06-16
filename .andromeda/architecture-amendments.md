# Architecture — Amendments

_Append-only changelog of amendments to `architecture.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-14-cargo-workspace-scaffold — MSRV raised 1.88.0 → 1.94.1
**Section:** §Stack and Technologies (+ §Infrastructure Patterns Build system · §Inherited Defaults · directory-tree comment)
**Change:** MSRV pinned to 1.94.1 (was 1.88.0) across all occurrences.
**Why:** the cargo-workspace-scaffold chunk implemented the security-plan §Dependency Security required bump (≥1.94.1, tar-rs CVE-2026-33056); arch's stated MSRV 1.88.0 was superseded. The workspace pins build toolchain 1.95.0 with `rust-version = "1.94.1"` as the MSRV floor.

## 2026-06-14-cargo-workspace-scaffold — self-observation stack row added
**Section:** §Stack and Technologies
**Change:** added a "Self-observation | tracing 0.1.44 + tracing-subscriber 0.3.23" row.
**Why:** the chunk pinned `tracing` + `tracing-subscriber` in `[workspace.dependencies]` (the obs-plan §3 self-obs stack); arch's Stack table did not list them. NOT an OTel SDK — OTLP remains the PRODUCT emission.

## 2026-06-15-conductor-core-shared-types — serde_json 1.0 registered in §Stack
**Section:** §Stack and Technologies
**Change:** added a "Serialization (JSON) | serde_json 1.0" row.
**Why:** the conductor-core-shared-types chunk added `serde_json = "1.0"` to `[workspace.dependencies]` (canonical-name round-trip tests now; the run-report envelope + per-run JSONL journal consume it at runtime in Epoch 6); arch's §Stack registry did not list it. Cleared by the security drift-detector (audit-green, allowed by §Dependency Security). Cascaded to `.claude/docs/stack.md`. The design proposal (record an implementation note in the Decisions Log) was rejected as non-drift — design-system.md §cli already prescribes the implemented prefixes.

## 2026-06-15-config-validation-surface — garde pinned 0.23.0 → 0.22.1
**Section:** §Stack and Technologies (Validation row) · §Established Decisions [Validation Library] · §Inherited Defaults (Validation)
**Change:** garde version 0.23.0 → 0.22.1 across all three; §Established Decisions now records that garde 0.22.1's `#[garde(custom)]` is field-level only (no container-level custom) — cross-field invariants spanning distinct fields use garde's `Context` pattern.
**Why:** the config-validation-surface chunk needed garde's `derive` feature, but `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + `derive` is unbuildable; user authorized the downgrade to 0.22.1. The no-duplicate-P-IDs validator consequently landed field-level (not struct-level). Cascaded to `.claude/docs/stack.md`. Cleared by the security + arch drift-detectors (§Dependency Security carried no garde pin).

## 2026-06-15-structured-logging-stack — CONDUCTOR_SERVICE_NAME / CONDUCTOR_ENV registered
**Section:** §Occupied Resources (Environment variables)
**Change:** added `CONDUCTOR_SERVICE_NAME` (self-obs `service.name` override) and `CONDUCTOR_ENV` (self-obs `deployment.environment`, default `local`) to the reserved `CONDUCTOR_*` env-var list.
**Why:** the structured-logging-stack chunk's `init_observability` reads both (obs-plan §3) to populate `ServiceIdentity`; the arch env-var inventory listed only RUNS_DIR/SCENARIOS_DIR/CONTRACT_MANIFEST/SEED. No cascade — CLAUDE.md / stack.md do not enumerate env vars (grep-confirmed). D-arch-decisions cleared (tracing/tracing-subscriber already in §Stack; no OTel SDK introduced).

## 2026-06-15-design-token-typography-bundle — frontend stack + ui/ asset subtree registered
**Section:** §Stack and Technologies (new Desktop-frontend row) · §Occupied Resources (Frontend asset subtree) · §Inherited Defaults (Frontend bullet)
**Change:** recorded the optional GUI's realized frontend toolchain — React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide via `@tailwindcss/vite`) + Fontsource WOFF2, package manager npm (committed `package-lock.json` + `npm audit` gate); registered the `crates/conductor-tauri/ui/` asset subtree (not a Cargo member; `node_modules/` + `dist/` git-ignored) and the npm/frontend default.
**Why:** the design-token-typography-bundle chunk landed the first frontend code under `crates/conductor-tauri/ui/`; arch §Stack pinned no Vite major (Vite 8.0.16 was the npm-audit floor clearing esbuild GHSA-gv7w-rqvm-qjhr), and §Occupied Resources / §Inherited Defaults did not record the frontend asset layer. User-approved at the wrap escalation (all-4 spec recordings). Cascaded to `.claude/docs/stack.md`. D-arch-resources + D-arch-decisions cleared (React 19 / Tailwind 4.1 / Fontsource already allowed by §Stack; Vite major was unpinned).

## 2026-06-16-test-framework-fixtures-coverage-tooling — test/coverage toolchain registered in Build system
**Section:** §Infrastructure Patterns — Build system
**Change:** the test clause now names cargo-nextest as the pinned runner (zero-retry `ci` profile in `.config/nextest.toml`) + `cargo test --doc`, the dev-test stack (rstest/proptest/insta/assert_cmd/assert_fs/predicates), and `cargo-llvm-cov` line coverage requiring the `llvm-tools-preview` toolchain component; exact versions deferred to test-plan §4.
**Why:** the test-framework chunk landed exactly this toolchain (report Changes + Outcome — 46/46 nextest, 91.97% llvm-cov, all gates green); arch §Infrastructure named only `cargo test` / cargo-nextest loosely. Routine per playbook rule #5 (spec→sound-impl alignment). Cascaded to `.claude/docs/stack.md`. D-arch-resources cleared (no new crate/port/env var); D-arch-decisions resolved by this registration.
