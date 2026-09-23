# arch extract

## Relevance
Relevant. The arch plan owns the deferral this chunk discharges (§Established Decisions [Read-Back Dependency Posture]) and the `UNBACKED_AUTO` exact-set gate that naming a cluster P-ID moves ([Accepted Capability Set]). It also owns the closed tier, verdict and state sets the grading has to fit, and the readiness-gate contract that a launch with deterministic L4 off runs into.

## Constraints
- **A model miss is a value, never `Fail` or `Err`.** architecture §Established Decisions [Probabilistic-Assertion Policy] puts hypothesis quality (named explicitly) on the calibration-region track, never hard-failed on exact values, using the default `Verdict::default_report_state` mapping. §Design Philosophy ("Outcomes are values…") and §Cross-cutting Patterns — Verdict/error wall keep `Result::Err` for harness faults only. §Standard Contracts — Run report envelope keeps the verdict triad and the five-state set closed.
  - The policy splits claims into two tracks: deterministic claims get hard pass/fail, model-interpretive claims get calibration-region. The posture's table gives a hit `Pass` and sends only a miss to `CalibrationRegion`.
  - Whether that one-sided hard `Pass` fits the policy as written, or needs an arch-body record at wrap, is P4's call. The posture itself cannot be re-decided either way.
- **No tier can hold this leg, and none may be added.** §Established Decisions [Timing-Tolerance Model] and §Conventions — Data model conventions require `slo_tier` to stay the closed three-value set. §Conventions — Config conventions caps `budget_ms` at the Tier90s deadline (`Scenario::check_budgets`). A scenario with `[[expected]]` checks would be graded against ≤90 000 ms, so declare-only is the only shape the arch plan allows for a ~110 s formation.
- **Naming a pinned P-ID moves the backing gate in the same change.** §Established Decisions [Accepted Capability Set] requires:
  - manifest membership (`check_capabilities` via `from_toml_str_with`);
  - `UNBACKED_AUTO` held to exact-set equality by `check_scenario_backing`. A pin that a scenario now covers is a `CoreError::UnbackedCoverage`, so a scenario naming P-033 (or any other cluster P-ID) cannot land while the pin edit waits for the next route entry.

  Whether `CoverageMatrix::render()` reflects backing, which would also move the committed `coverage-matrix.md` byte-compare (§Infrastructure Patterns — Build system), is research's question.
- **The other CI catalog gates also grade the new scenario.** §Occupied Resources — On-disk artifacts requires every catalog scenario to:
  - pass `check_load_envelope` unaided: terms judged per emitting phase, the `[[exempt]]` ledger empty and exact-set, the rate counted as wire records per `EmissionShape` arm through `max_spans_per_dispatch()` (a new arm needs its own count);
  - be graded in both directions by `check_scenario_audit` (`contracts/scenario-audit-ledger.toml`, raising `CoreError::ScenarioAudit`).

  Whether this scenario needs a `[[live_assertion]]` or `[[over_tier]]` pin is research's question.
- **The cause goes in as declared, seeded emission data over raw OTLP.**
  - §Established Decisions [OTLP Emission Strategy] and §Conventions — Interface surfaces require raw opentelemetry-proto types over gRPC to `127.0.0.1:4317`, with no SDK exporter.
  - §Conventions — Config conventions, [Validation Library] and [Scenario Config Format] require the profile to be TOML `EmissionSpec` data, with garde `dive` and never `skip`.
  - §Design Philosophy "Determinism under a seed" still binds Conductor's side, even though the SUT's model is not deterministic.
  - §Standard Contracts — Readiness gate ("Corpus access") treats `corpus.db` as SUT-owned and out of scope.
  - What counts as "one identity" for a storm follows the Pulse fingerprint semantics recorded in [Read-Back Dependency Posture]: the first 3 normalized frames, relative paths significant at every depth, line and hex addresses ignored.
  - §Occupied Resources — Service / process names registers the emitted `service.name` pair `conductor` / `conductor-canary`. A cause that needs another identity adds a registration.
- **As the plan specifies it, the readiness gate blocks a launch with deterministic L4 off.**
  - §Occupied Resources — Environment variables and its `pulse-run-contract.toml` row make `ANDROMEDA_PULSE_L4_DETERMINISTIC` a `shell-declaration` term, the only kind that can block; an absent declaration counts as an unmet term.
  - §Standard Contracts — Liveness equivalent grades that handle through the truthy gate in `conductor preconditions` `handles-declared`.
  - §Standard Contracts — Readiness gate runs the unmet-terms arm before the canary.
  - The posture requires the handle to be absent or falsy. Whether the code or contract already allows that is research's question.
  - Changing what preflight blocks on is a §Standard Contracts change. The run-contract row records `sidecar-built` being left unreclassified for exactly that reason.
  - With real L4, the canary itself only succeeds if the LLM surfaces an incident ([Read-Back Dependency Posture]). The poll budget can only be raised through the existing `CONDUCTOR_PREFLIGHT_TIMEOUT` (which overrides upward from the `min_canary_poll_seconds` floor), never through a new handle.
- **Read-back stays within the four consumed tools.** §Occupied Resources — Interface routes / surfaces names four MCP tools as the complete set. The observable comes from `retrieve_report` (with `degraded_mode`) as a raw `serde_json::Value` ([MCP Read-Back Client]).
  - [Read-Back Dependency Posture] says the real-model proof needs two things: real per-check read-back extraction and a non-deterministic live leg.
  - Whether the shipped `extract.rs` composition already meets the first need (the scope's `[inferred]` claim (3), "No new machinery") is research's question.

## Patterns to follow
- **A declare-only row plus a harvest-tier grade.** §Standard Contracts — Run report envelope: declare-only families ship zero `[[expected]]` checks, land `verdict: null` / `KnownResidual`, and are graded at the harvest tier. §Standard Contracts — Per-check record: they write zero `CheckRecord` / `run_check` rows. The precedent harvest test (`conductor-run/tests/lifecycle_harvest.rs`) is cited in [Read-Back Dependency Posture].
- **Attribute by liveness and runtime state, not payload.** §Established Decisions [Read-Back Dependency Posture] gives the Conductor-side observables for "reached the live-writer branch, not a second incident":
  - one active incident per dedupe tuple `(kind, scope, scope_id)`, measured at `inference_runtime.rs:811`;
  - freshness against Conductor's `std::time` emission instant;
  - membership in the `query_incident_list` active set.

  Two things decide whether the incident stays active across the second digest and whether silence cues refill the active set: the same section's auto-resolve lifecycle (120 s idle / 30 s ticks), and the bootstrap window fixed once at Pulse's boot (`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`, §Occupied Resources — Environment variables).
- **Fire it from the operator-gated arm on the existing harness surface.**
  - §Established Decisions [CI/CD]: proof that needs a live Pulse is a local operator gate.
  - The §Stack and Technologies Operator-instruments row keeps the 5-command harness surface. The posture's §Occupied Resources row adds no verb, flag or arm.
  - §Occupied Resources — Service / process names: a `PATH` prefix that resolves the sidecar is part of any operator-driven leg's firing form.
  - §Occupied Resources — On-disk artifacts: `runs/live-suite/{leg}.jsonl` is the harness-owned per-leg freeze site if the leg rides `run --live`.
  - Operator pauses are isatty-gated and never block headless runs (§Stack Terminal output rendering row; `CONDUCTOR_AGENT_MODE`).
- **Keep code in its seam.** §Established Decisions [Module Boundaries] and §Infrastructure Patterns — Directory structure:
  - scenario model and validation go in `conductor-core`;
  - read-back extraction and verdict logic go in `conductor-verify`;
  - state routing (`route_read_back`, `state_for`) and execution go in `conductor-run`.

  If scoping the check to the first `## Hypotheses` entry needs production code, it belongs in `conductor-verify`. Declare any new dependency feature in `[dependencies]`, never only in `[dev-dependencies]`.

## Anti-patterns to avoid
- **Giving the posture file a Rust reader.** §Occupied Resources — On-disk artifacts registers `contracts/pulse-real-model-leg-posture.md` as having no reader: no `default_path()`, no `resolve_under`, no bounds check, no `CONDUCTOR_*` handle. The committed cause and its identifying text belong in scenario data or in the harvest test.
- **Widening a closed surface to fit the leg.** That means no fourth tier, no `budget_ms` above Tier90s, no new verdict or state variant or envelope key, no sixth command or new arm, and no unregistered env handle named in a shipped artifact. The posture row names only registered handles and mints none. (Per §Established Decisions [Timing-Tolerance Model]; §Standard Contracts — Run report envelope; §Occupied Resources — Environment variables.)
- **Crossing the trust or scope boundary.** No CI path to the leg, no Conductor starting or stopping `pulse-app`, no staged corpus rows or writes into `andromeda-pulse`, and no new network reach or listener. (Per §Cross-cutting Patterns — Trust boundary and Scope law "no Pulse process management"; §Established Decisions [CI/CD].)

## Contract bindings
- **arch ↔ tests (run tier vs harvest tier).** [Read-Back Dependency Posture] sends a degraded read-back to `KnownResidual`. It sends a declare-only scenario's empty active list after a green preflight to `KnownResidual` as well. Read-back call failures always land `Blocked`. The posture's harvest-tier table calls "no hypotheses rendered" `Blocked`.
  - So the run row reads `KnownResidual` both when the leg has something to grade and when it doesn't.
  - The harvest test must classify from the captured report content, never from the row's `state`.
  - No report surface carries the interpretation verdict, because declare-only rows write zero check records.
- **arch ↔ tests/CI (static gates over committed data).** The scenario never runs in CI, but the `rust` job still grades it through the backing, drift, load-envelope, scenario-audit and coverage-completeness gates ([CI/CD]). `/runs/` is git-ignored (§Occupied Resources, `runs/a11y` row), so a committed harvest test cannot read its subject from `runs/`.
- **arch ↔ security.** Precondition and readiness output stays host-path-free with `data_dir` redacted, and handle reporting is name-only (§Standard Contracts — Readiness gate; §Occupied Resources `ANDROMEDA_PULSE_MCP_ENABLED`). Verbatim captures committed as harvest evidence inherit these rules.
- **arch ↔ residuals/matrix.** The leg discharges the deferral that [Read-Back Dependency Posture] pins in `.andromeda/residuals.md` (`absorbed:v3-09`). The arch passage "Conductor green ≠ interpretation trustworthy" and `UNBACKED_AUTO` must stay consistent with what ships.

## Acceptance criteria contributions
- (arch) A model miss grades `CalibrationRegion` → `ManualCheck` through the default mapping and is never `Fail` or `Err`. A read-back call failure lands `Blocked` with a named precondition. No new verdict or state variant, envelope key or `slo_tier` value is added. (per architecture §Established Decisions [Probabilistic-Assertion Policy])
- (arch) With the scenario committed, CI stays green on `check_scenario_backing` (with the `UNBACKED_AUTO` pin moved in the same change if a pinned P-ID is named), `check_sut_drift`, `check_load_envelope` (with `[[exempt]]` still empty), `check_scenario_audit`, and the coverage-completeness byte-compare. (per architecture §Established Decisions [Accepted Capability Set])
- (arch) The leg can only be reached from the existing operator-gated arm: no sixth command, verb, flag or arm, no CI job, and no env handle minted or named in a shipped artifact without registration. (per architecture §Occupied Resources — On-disk artifacts, `pulse-real-model-leg-posture.md` row)
- (arch) The confirmed formation figure is measured from the journal's `std::time` emission instant, never wall-clock from test start, and the leg adds no outbound target or listener. (per architecture §Design Philosophy — Journal-relative ground truth, and §Cross-cutting Patterns — Trust boundary)

## Relevant amendment history
- **2026-06-27-live-pulse-e2e-proof.** Recorded incident creation as LLM-in-the-loop, because the live SUT disproved the deterministic canary premise. It also recorded a real-model run recipe:
  - `pulse-app` needs `ANDROMEDA_PULSE_MODEL_PATH` and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`;
  - the build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH`, and a tokenizer-truncation bug breaks L4;
  - L4 takes about 4 s per inference.

  None of these three handles is registered in the body. If the drive's shipped firing form names them, registering them is owed under the 2026-09-06-halo-hue-budget-re-driven rule (register only handles a shipped artifact names).
- **2026-08-09-interpretation-correctness-posture.** Added `check_scenario_backing` / `UNBACKED_AUTO` and recorded the deferral this chunk discharges. That passage (and possibly the pin) moves at wrap.
- **2026-08-10-pulse-run-contract (three entries).** Made `ANDROMEDA_PULSE_L4_DETERMINISTIC` a `shell-declaration` proxy that Conductor reads, placed the unmet-terms arm before the canary, and set the `CONDUCTOR_PREFLIGHT_TIMEOUT` floor. The reason given: an unmet launch condition explains a failed canary. This is the direct collision with an L4-off leg.
- **2026-08-15-canary-storm-autonomous-band.** Tier-1 accepts only Autonomous cues (≥10 of the same fingerprint in 30 s); ≥5 raises only a Suggested cue and forms no incident; the canary count went from 6 to 12. This sets each storm's count in the two-digest profile.
- **2026-08-31-p-075-assert-round, then 2026-09-01-webview-self-verify-windows-host.** Dedupe was re-scoped from per-workspace to per `(kind, scope, scope_id)` tuple, because the wider generalisation was unlicensed. This governs telling the `:823` refresh apart from a second incident.
- **2026-08-16-canary-fingerprint-derivation-aligned and 2026-08-17-fingerprint-semantics-token-leading.** Made freshness the carrier, with the honest limit (it proves causation in time, not payload identity) resting on L4-authored fields being fixture constants. With real L4 the hypothesis is L4-authored and can vary with the emission, so a passing leg may owe a qualification of that limit.
- **2026-08-20-latency-regression-re-proof, 2026-08-21-delegated-timing-budgets-proven and 2026-09-06-operator-gated-live-suite.** Established two routes to `KnownResidual` (the degraded one is not gated on declare-only; the auto-resolve one applies to declare-only scenarios only), made `degraded` a per-read-back property, and registered `runs/live-suite/{leg}.jsonl`.
- **2026-09-06 0-pending adaptation and 2026-09-06-halo-hue-budget-re-driven.** Auto-resolve reachability is bounded by the bootstrap window fixed at boot; past it, silence cues form incidents mid-silence. This is the arch basis for the posture's suite-wide window and per-leg quiet window.
- **2026-08-21-per-check-latency-measurement.** Made the tier deadline a ceiling and capped `budget_ms` at Tier90s. This is why the leg cannot be tier-graded.
- **2026-09-15 (two entries) and 2026-09-16-scenario-assertion-audit-gate.** Anchored the declare-only registry on its complement (2 of 36 scenarios still carry `[[expected]]` checks, dated) and registered the audit ledger and its gate. The new scenario enters both.
- **2026-09-18-real-model-leg-posture-and-grading-rule.** Registered the posture file as the second `contracts/` member with no Rust reader, with provenance recorded per clause and the ~110 s figure marked to be confirmed at the first drive. Confirming or correcting that figure moves this row's provenance wording at wrap.
