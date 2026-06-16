# Test Plan — Amendments

_Append-only changelog of amendments to `test-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — self-obs stream noted as distinct from the emission journal
**Section:** §3 Test Harness Contract / Log format
**Change:** added a note that the `tracing` self-observation stream (stderr / `logs/agent-latest.jsonl`; per-line base fields incl service-identity + `run_id`, obs-plan §3) is a SEPARATE artifact from the per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth + Run-report envelope) — the two schemas must not be conflated.
**Why:** D-tests-obs-harness fired on the chunk's new flat `service.*` self-obs fields. Validated (main) that those belong to the self-obs stream, NOT the emission-journal envelope test-plan §3 owns — so the bound §3 ↔ obs-plan §3 envelope is unchanged; the amendment is a clarifying cross-reference, not a field addition to the envelope.

## 2026-06-15-design-token-typography-bundle — frontend (ui/) tests build-gated, unit deferred to Epoch 9
**Section:** §4 Unit Test Strategy (What unit tests cover)
**Change:** added a `conductor-tauri/ui` bullet — the React/Tailwind token bundle carries no Rust/nextest unit tests; it is build-gated (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke), and frontend coverage (webview E2E via tauri-driver) is deferred to Epoch 9.
**Why:** D-tests-coverage (warning) fired on the new frontend code paths having no unit tests; routine per playbook (a Foundation chunk defers a downstream-sequenced concern — §12 Decisions Log already records no JS/TS unit runner adopted, GUI convenience-only). No envelope/harness change; D-tests-framework + D-tests-obs-harness clear. No distillation cascade (tests-summary/testing.md describe the unchanged Rust test strategy).

## 2026-06-16-test-framework-fixtures-coverage-tooling — external-CLI tool versions reframed as floors
**Section:** §4 Unit Test Strategy
**Change:** added a Tool-version policy paragraph — the external-CLI tools (cargo-nextest, cargo-llvm-cov) named in §4 are reference floors (outside `Cargo.lock`; any green-running install satisfies the gate, per the cargo-audit/deny precedent); the crate dev-deps are caret-resolved with `Cargo.lock` authoritative.
**Why:** D-tests-framework (warning) fired — the chunk resolved cargo-nextest 0.9.133 · cargo-llvm-cov 0.8.5 · proptest 1.11.0 · insta 1.48.0 · assert_cmd 2.2.2 · assert_fs 1.1.4 · predicates 3.1.4 · rstest 0.26.1, differing from §4's named pins (0.9.137 / 0.8.7 / 1.9.0 / 2.1.2) while all gates ran green (46/46, 91.97%). Routine per playbook rules #2 (dev-CLI floors) + #5 (spec→sound-impl); reframing to floors stops the detector recurring. predicates was already named in §4 (test-runner-install). Cascaded to `.claude/docs/{stack,tests-summary}.md`.

## 2026-06-16-emission-journal-writer — unit serialization goldens use exact-assert; insta stays the E2E mechanism
**Section:** §4 Unit Test Strategy (conductor-report bullet)
**Change:** clarified the conductor-report golden — the canonical line shape is locked via exact-string `assert_eq!` at unit level (matching the `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern), with insta reserved as the E2E journal-golden mechanism (`run_id`/timestamp redaction, §6/§7).
**Why:** D-tests-framework (warning). The chunk's unit goldens used exact-string `assert_eq!` (dropping insta from `conductor-report` dev-deps), matching the established canonical-serialization golden pattern already in `conductor-core`; insta remains the E2E mechanism (real runs, redaction) and stays in the workspace dev-deps. Routine per playbook rule #5 (spec-illustration → sound-impl alignment; report proves the schema is golden-locked). No distillation cascade (testing.md / tests-summary.md describe insta for CI/E2E goldens, unchanged).
