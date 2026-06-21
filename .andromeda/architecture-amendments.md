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

## 2026-06-16-seeded-phase-scheduler — seedable RNG (rand_chacha/rand_core) registered in §Stack + §Established Decisions
**Section:** §Stack and Technologies (new Determinism RNG row) · §Established Decisions (new [Determinism RNG] entry)
**Change:** added a "Determinism RNG | rand_chacha 0.9 (`ChaCha8Rng`) + rand_core 0.9 (`SeedableRng`)" Stack row and a [Determinism RNG] established decision locking `ChaCha8Rng` + `seed_from_u64` (platform/version-stable) as the timeline scheduler's sole non-determinism source.
**Why:** the seeded-phase-scheduler chunk added rand_chacha 0.9 / rand_core 0.9 to `[workspace.dependencies]` for deterministic per-gap jitter (report §Dependencies); arch §Stack listed no RNG and §Established Decisions did not record the RNG/seeding choice. Routine per playbook rule #5 (spec→sound-impl alignment) — arch §Cross-cutting already mandated a "seeded RNG"; this records the concrete algorithm-stable realization. Cascaded to `.claude/docs/stack.md`. D-arch-resources + D-arch-decisions resolved by this registration. D-security-deps escalate was a verified false positive (tauri already 2.10.3, untouched by this chunk; real new deps audit/deny-green) — dismissed with the user; a playbook rule was added for the misfire class.

## 2026-06-16-scenario-config-model — toml 0.9 registered in §Stack + [Scenario Config Format] decision
**Section:** §Stack and Technologies (new Scenario config (TOML) row) · §Established Decisions (new [Scenario Config Format] entry)
**Change:** added a "Scenario config (TOML) | toml 0.9" Stack row and a [Scenario Config Format] established decision locking declarative TOML scenario config (serde-deserialized + garde-validated via `Scenario::from_toml_str`), chosen over JSON for hand-author ergonomics + inline comments across the 60 per-P-ID files.
**Why:** the scenario-config-model chunk added `toml = "0.9"` to `[workspace.dependencies]` + conductor-core for the declarative per-phase emission spec (report §Dependencies + §Decisions — the P4 user format decision); arch §Stack listed only serde_json and §Established Decisions recorded no scenario config format. Routine per playbook rule #5 (spec→sound-impl alignment) + the serde_json / rand_chacha precedents (a new audit-green dep → §Stack row, a locked format choice → §Established Decisions). Cascaded to `.claude/docs/stack.md` (CLAUDE.md no delta — its overview Stack line is high-level). D-arch-decisions resolved by this registration; D-security-deps cleared by the security detector (toml audit/deny-green, Cargo.lock committed); the other 5 docs returned `proposals: []`.

## 2026-06-18-exception-events-fingerprint-control — fingerprint primitive placed in conductor-emit
**Section:** §Infrastructure Patterns (directory-tree crate comments) — cascaded to CLAUDE.md §Modules
**Change:** the per-exception fingerprint PRIMITIVE (`fingerprint()` + the exception-event builder, identical/path/line/type/frame variants) is recorded in `conductor-emit` (co-located with the exception content it derives from); `conductor-faults`' "fingerprint generation" is narrowed to the fingerprint-STORM fault (Epoch-7), which will depend on emit and compose this primitive.
**Why:** the exception-events-fingerprint-control chunk landed `fingerprint()` + `exception_trace_request()` in `conductor-emit` per the crate-seam the user ratified in /andromeda-phase (AskUserQuestion — the error-spans "builder-in-emit" precedent; faults is an empty Epoch-4 stub). arch's dir-tree comment had attributed all "fingerprints" to faults. Routine per playbook rule #5 (spec→sound-impl alignment) + the explicit prior user ratification — documentation alignment, no behavioral change. Cascaded to CLAUDE.md §Modules (stack.md has no crate-ownership mention — grep-confirmed; no delta). The other 6 docs returned `proposals: []`.

## 2026-06-21-run-report-envelope-serializer — ManualCheck widened + Verdict→ReportState default mapping
**Section:** §Read-Back Dependency Posture · §Probabilistic-Assertion Policy
**Change:** ManualCheck's definition broadened from "operator-checklist / no-programmatic-read-back only" to ALSO include an auto-measured model-interpretive (calibration-region) check; recorded the default `Verdict → ReportState` mapping (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`, via `Verdict::default_report_state`), with `verdict`/`state` kept independent and the run-report lamp chosen verdict-first (a calibration row renders HOLD, not Manual).
**Why:** the chunk shipped `Verdict::default_report_state` (P4 decision, user-approved) + `RunRecord::measured`; arch's narrow ManualCheck definition was stale. The arch drift-detector false-negatived (claimed arch already prescribed it — it did not); the orchestrator authored the amendment, cross-confirmed by the a11y detector. Verdict-first lamp precedence resolves the a11y §6 six-lamp conflict (escalated + user-confirmed 2026-06-21). Cascade no-op (CLAUDE.md / stack.md carry no ManualCheck-definition detail — grep-confirmed).
