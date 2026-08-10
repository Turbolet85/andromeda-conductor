# Report — 2026-08-10-pulse-run-contract

**Chunk:** Pulse run contract — the launch conditions a verifiable Pulse must meet, recorded in-repo and asserted at preflight with each unmet term named (conductor-verify/core, P-073, v2-18)
**Date:** 2026-08-10
**Commits:** none since `last_wrap` — this chunk's commit is P7's (prior HEAD `2c02b60`, the workspace-key-divergence probe)

## Changes (structured — detectors read this)

- **Files:**
  - new — `contracts/pulse-run-contract.toml` · `crates/conductor-core/src/run_contract.rs` · `scenarios/pulse-run-contract.toml`
  - modified — `crates/conductor-core/src/lib.rs` · `crates/conductor-core/src/drift.rs` · `crates/conductor-verify/src/preflight.rs` · `crates/conductor-verify/tests/preflight.rs` · `crates/conductor-verify/tests/preflight_spawn.rs` · `crates/conductor-run/src/lib.rs` · `crates/conductor-run/Cargo.toml`

- **Symbols / APIs:**
  - new public (`conductor-core`): `RunContract` (`default_path`/`load`/`evaluate`/`observed_env`/`preflight_budget_ms`/`check_within_envelope`) · `RunContractStatus` (`satisfied`/`from_unmet`/`is_satisfied`/`unmet`) · `ContractTerm` · `UnmetTerm` · `CheckKind{ShellDeclaration,Asserted,DeclaredNotObservable}` · `IncidentFormation`
  - **changed signatures** (`conductor-verify`, +1 param `&RunContractStatus`): `run_preflight` · `preflight_boot` — 6 call sites threaded, exactly the set the code-graph enumerated
  - new private (`conductor-run`): `load_run_contract` · `observe_run_contract` · `declares` · `warm_up_canary_service`; `emit_canary` and `canary_poll` gained parameters
  - **env vars:** `ANDROMEDA_PULSE_L4_DETERMINISTIC` is now READ by Conductor (observed as a declaration in its own environment; Pulse-side var, NOT a new `CONDUCTOR_*` handle). `CONDUCTOR_PREFLIGHT_TIMEOUT` keeps its 30s default but gains a contract-derived effective FLOOR (90s) it can no longer sit below.
  - no new IPC method, endpoint, socket, or port

- **Crates / modules:** `conductor-core` gains the `run_contract` module (+6 re-exports). No crate added or removed. No new cross-crate edge — `conductor-verify → conductor-core` already existed.

- **Dependencies:** `tokio` moved from `[dev-dependencies]` to `[dependencies]` on `conductor-run` (`features = ["time"]`, workspace version) so the contract's warm-up window can pace its pre-roll. **No new package; `Cargo.lock` is byte-unchanged** (already a dev-dep of that crate at the same workspace version). `cargo deny check` green (advisories · bans · licenses · sources).

- **Schema / config:**
  - `contracts/pulse-run-contract.toml` — a fourth committed `contracts/` manifest: `sut_version` · `captured_at` · `provenance` · `[incident_formation]{warmup_ms=45000, warmup_emissions=3, min_canary_poll_seconds=90}` · 5 × `[[term]]{id, statement, check, env?, causes}`
  - `scenarios/pulse-run-contract.toml` — new scenario, `p_ids = ["P-073"]`, 3 phases (120s total), one Hard `CountAtLeast "1"` check
  - no `runs.db` migration; the run-report envelope is unchanged (11 fields), `ReportState` stays five, `LAMP_META` stays six

- **Spec-master edits:** none — `/implement` authored no amendment (5 are recorded as Expected, below).

- **Counts / qualifiers moved:**
  - `conductor_core::UNBACKED_AUTO` **10 → 9** (`P-073` retired) ⇒ the coverage roll-up caption `(N unbacked)` now renders **`(9 unbacked)`** — verified live: `43 auto (9 unbacked) · 16 drive+observe · 7 static-only · 16 not-conductors`. Docs stating the literal: `layout-templates.md` · `design-system.md` · `obs-plan.md`.
  - the preflight readiness gate's **named-precondition set 4 → 5** (a run-contract-terms arm). Docs stating the count: `architecture.md` §Standard Contracts · `security-plan.md` §Security Anti-Patterns.
  - **`contracts/` manifest count 3 → 4.** Docs stating it: `architecture.md` §Occupied Resources + its directory tree.

- **Dev-tool versions:** none — `cargo-audit` remains 0.22.2 (installed at a prior chunk; nothing to upgrade to).

- **Reverted / negative API facts:**
  - **No `ReadyState` field was added**, deliberately. A structured-terms field would have forced edits to four full struct literals across two crates (`preflight.rs:191-198`, `:222-234`; `conductor-run/src/lib.rs:89-101`, `:168-183`); a single composed string satisfies "names each unmet term individually" at zero threading cost.
  - The plan's "private evaluator in `preflight.rs`" was **not** built there: reading env inside the gate would make every pre-existing preflight leg block on the new term (the var is absent under test) and setting env in tests is `unsafe` in edition 2024. Pure evaluation lives in `conductor-core`; `preflight.rs` keeps the cascade arm + string composition.
  - The contract records `shared-data-dir` as `check = "declared-not-observable"` rather than asserting it — Conductor cannot observe `pulse-app`'s key (`query_incident_list` takes no arguments; the v2-17 limit).

- **Coverage of new surfaces:**
  - `contracts/pulse-run-contract.toml` (new external-input boundary) → validation `explicit validate()✓` (the two-manifest precedent; garde governs scenario config, not committed manifests) · instrumentation `tracing::info!(count) boundary line✓` · PII `e.kind()-only read error + sanitize_error on parse✓` · tests `unit 11✓` · a11y n/a · tokens n/a
  - `run_preflight` run-contract cascade arm (changed API surface) → validation n/a (input is a typed value, not external text) · instrumentation `existing info! Blocked line✓` · PII `host-path-free asserted by test✓` · tests `integration 3✓` · a11y n/a · tokens n/a
  - `warm_up_canary_service` (new OTLP-egress hot-path op) → validation n/a · instrumentation `tracing::info!(count)✓` · PII n/a · tests `unrunnable-here` (needs a live OTLP receiver; the operator-gated live leg is its harness — the existing `emit_canary` path is likewise only refused-transport-tested) · a11y n/a · tokens n/a
  - `scenarios/pulse-run-contract.toml` (new scenario config) → validation `garde✓` (loaded through the catalog's `from_toml_str_with` + capability-manifest membership) · instrumentation n/a · PII n/a · tests `catalog + backing-gate✓` · a11y n/a · tokens n/a

## Deviations from intent

1. **`crates/conductor-run/Cargo.toml` edited — outside the plan's touchpoints.** Plan step 5 mandates a warm-up *wait* in `conductor-run/src/lib.rs`, but `tokio` was a dev-dependency only, so no timer was reachable from library code. Gray-area judgment went to in-scope: soft-exiting on a one-line manifest edit that the step itself requires would have stalled the chunk on its central deliverable. `Cargo.lock` unchanged.
2. **Contract path resolved inside `canary_gate`, not at the binary edge.** Step 3 cited `load_envelope.rs`'s never-this-module rule, but `conductor-run`'s `preflight()`/`readiness()` take no contract path and supplying one would edit `conductor-cli/paths.rs`, `conductor-cli/commands/preflight.rs` and `conductor-tauri/commands.rs` — none in scope. Resolved in the composition root, mirroring `paths.rs`'s exact `resolve_under(&base, &X::default_path())` shape.
3. **Term evaluation lives in `conductor-core`, not `preflight.rs`** (see Reverted / negative API facts for the reasoning).
4. **The unmet-term arm skips the canary poll** instead of polling first and reporting the downstream symptom. Not in the plan text but implied by its own cascade-ordering rationale: with the floor raised to 90s, polling first would spend the full budget to report a condition already known.
5. **Plan step 7 (the live three-arm probe) did not run** — `andromeda-pulse-mcp` is not reachable on PATH in this session, which is precondition (1) of that probe. Per `.claude/rules/verification-harness.md` (2026-08-10) an unreachable sidecar makes every arm measure the read-back-unreachable path and yield evidence that looks like a result but isn't. Deliberately not faked; re-pinned as a CARRY.

## Decisions & corrections

- **Operator decision (P4, question 1):** the deterministic-L4 term is asserted through a **shell-declaration proxy with a named fallback** — Conductor reads its OWN environment (the same shell that launches `pulse-app`) and, when the declaration is absent, surfaces a precondition naming the condition and both candidate causes. Never presented as a measurement of `pulse-app`.
- **Operator decision (P4, question 2):** P-073 is backed by a scenario and retired from `UNBACKED_AUTO` **in this commit** — both halves together, as `check_scenario_backing`'s exact-set equality requires in both directions.
- **Research correction, applied to scope at P5 (intent-incomplete, not a plan defect):** the scope named the `>=5 same-fingerprint in 30s` storm shape as the contract's missing content. `emit_canary` already emits 6 and cites that floor. The operative lever is that **the canary storm is the canary service's first-ever traffic**, so Pulse holds no baseline and the L2 cue evaluator never considers it — hence the warm-up pre-roll. `scope.md` Term C was amended in place with the strike-through recorded.
- **A term Conductor cannot measure is recorded, not asserted.** `shared-data-dir` ships as `declared-not-observable` with its causes — the v2-17 precedent applied to the contract artifact itself.
- **`cargo audit` — 7th consecutive red, silent re-pin per the standing ratification.** True exit 1 on 0.22.2 (the latest published), byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`. Advisory-DATABASE fault with nothing to raise a floor to. No floor raise, no `deny.toml` ignore, no CI edit; `cargo deny check` verified green as the overlapping signal.

## Outcome

**Acceptance criteria: met**, with one nuance routed to the ledger (below).

Gates — green in **1 fix-loop iteration, zero fixes needed**:
- `cargo nextest run -p conductor-core -p conductor-verify` → **305/305**
- `cargo nextest run --workspace --profile ci` → **509/509** (+14 = 11 loader unit tests + 3 preflight legs), zero retries
- `cargo test --workspace --doc` → ok
- `cargo clippy --workspace --all-targets -- -D warnings` → clean
- `cargo deny check advisories bans licenses sources` → all four ok
- `cargo audit` → red (external advisory-DB fault; ratified bounded wait, re-pinned)

Smoke (boot-path changed): `bash scripts/agent-run.sh run` → **exit 0**. Beyond the harness: `conductor preflight --json` exercised the real CLI gate (exit 1, `ready:false`, `canary_round_trip: "skipped"`, envelope host-path-free), and `conductor coverage` confirmed the roll-up moved to `43 auto (9 unbacked)` with the per-mode counts still summing to 82. The new cascade arm was **not reachable through the binary here** — the sidecar is absent from PATH, so the read-back-unreachable precondition fires upstream of it by design; the arm is proven at the stub tier by 3 integration legs.

**Ledger nuance (v2-18):** the acceptance prose asserts "both sides resolving the same data dir", which is structurally unobservable from Conductor's side — the exact limit v2-17's notes established. No sanctioned acceptance-refinement class covers this, so it is recorded as a PREMISE-CORRECTION in the matrix `notes`; the acceptance text stays as the record of what was asked, and the coverage flip stands on refs.

**Expected amendments (wrap):**
1. `architecture.md` §Standard Contracts — the readiness gate's named-precondition set **4 → 5**.
2. `architecture.md` §Occupied Resources — register `contracts/pulse-run-contract.toml`; the directory tree moves from three `contracts/` manifests to four.
3. `architecture.md` §Occupied Resources (Environment variables) — `CONDUCTOR_PREFLIGHT_TIMEOUT` gains a contract-derived effective floor beneath its default of 30.
4. `security-plan.md` §Input Validation — extend the committed-SUT-facing-manifest boundary row to the third such manifest; §Security Anti-Patterns — the never-downgrade bullet's precondition enumeration **4 → 5**.
5. `layout-templates.md` / `design-system.md` / `obs-plan.md` — the `(N unbacked)` documented sample **10 → 9** wherever each states the literal.
