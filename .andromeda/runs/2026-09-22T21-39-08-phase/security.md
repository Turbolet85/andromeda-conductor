# security extract

## Relevance
Relevant. The drive touches several surfaces the security plan lists, all at once: the scenario-config boundary, the `run --live` preconditions and preflight gate (where the posture's L4-absent launch conflicts with the plan as written), MCP child-stdout read-back of untrusted model text, cause injection over loopback only, and hygiene of committed artifacts. It also raises one unsettled §Data Protection question. No auth, secret or dependency-delta surface is involved.

## Constraints
- **Cause injection route.** The known cause may reach Pulse only through Conductor's loopback OTLP/gRPC egress to `127.0.0.1:4317`. The chunk adds no inbound listener to a shipped binary and no non-loopback reach (per security-plan §Threat Model Summary → Attack surface, OTLP/gRPC egress vector; §Security Anti-Patterns → Universal). Staging a corpus row, or opening or copying `corpus.db`, to plant the cause or force a hypotheses render is the route the plan names as banned. Corpus contact goes through the MCP tool surface only (per §Security Anti-Patterns → Data Protection).
- **Scenario boundary.** The leg's scenario must name a Pulse P-ID, and its scenario key must map to a known one (per §Input Validation, CLI-arguments row; §Security Anti-Patterns → Universal).
  - It must load through garde `range`/`custom` plus the `from_toml_str` `Scenario::check_*()` arm. A failure there is a harness fault at load, never a verdict.
  - Every `[phases.emission]` block needs its `kind`, and `occurrences` must be ≤ `MAX_OCCURRENCES` (per §Input Validation, scenario-config row).
  - The emission profile needs a second cue-bearing digest on one identity. If the scenario model lacks a field to express that, the new nested field must `dive` (never `#[garde(skip)]`), carry bounds, and be added to the scenario-config row, which the plan keeps exhaustive (per §Security Anti-Patterns → Input).
  - If `contracts/pulse-capabilities.toml` is touched, its committed-manifest bounds hold: `P-NNN` shape, no duplicates, never silently widened.
  - Whether the shipped model already expresses the profile is research's question.
- **The L4 conflict: never a silent downgrade.** Two specified behaviours clash with the posture's launch, where the L4 handle is absent or falsy:
  - §Input Validation's declaration-only READ SET row requires an absent `ANDROMEDA_PULSE_L4_DETERMINISTIC` declaration to surface as an unmet run-contract term and/or a named `[PRECONDITION]` subject.
  - The committed `contracts/pulse-run-contract.toml` states `l4-deterministic` as a `shell-declaration` that requires `=true`.
  - So, as specified, the posture's launch runs into both the leading probe on `run --live` (`handles-declared`) and the preflight's run-contract arm. Whether the shipped code refuses or blocks such a leg is research's question.

  Whatever firing form P4 picks must meet all of the following (per §Input Validation, READ SET row; §Security Anti-Patterns → Universal, both preflight bullets):
  - Each unmet term or subject still surfaces individually with its causes. None is suppressed, defaulted or re-graded as met.
  - The deterministic legs' gating stays unchanged.
  - The inverted check stays declaration-only, showing the handle NAME only. The READ SET is read, never written: the harness may not set or unset a handle to create the posture.
  - The disposition is recorded as an observation of Conductor's own environment, never as a measurement of `pulse-app`.

  A new or changed run-contract term takes the committed-manifest bounds: a unique id, a statement AND its causes, and a `shell-declaration` that names its env var. It is never defaulted and never silently widened. If the precondition count moves, the §Universal five-precondition list has to be restated (per §Input Validation, committed-manifests row).
- **Harness-side inputs.**
  - Real-model per-leg budgets must derive from the validated `warmup_ms` / `min_canary_poll_seconds` under the shell readers' never-defaulted rule. A missing term is a hard exit 2, and a lowered `CONDUCTOR_PREFLIGHT_TIMEOUT` is clamped UP to the contract floor, never down (per §Input Validation, committed-manifests row).
  - Any new `CONDUCTOR_*` handle, flag or `conductor-cli` argument the firing form adds is a new input surface and needs its own row. A flag follows the `CONDUCTOR_A11Y_STRICT` shape: it only tests whether the value is set to true, and the value never becomes a path, an argv element or a log value.
  - Any Conductor-side reader of a further Pulse handle needs a row per READER. `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` has no row today because the posture gives Conductor no reader of it (per §Input Validation; §Security Anti-Patterns → Input).
- **Grading goes through the verdict/error wall.** (per §Input Validation, MCP child stdout row; §Error Handling; §Security Anti-Patterns → Universal)
  - The hypotheses are untrusted SUT output on MCP child stdout. They are read under bounded JSON-RPC decoding that never panics and raises typed `VerifyError` faults.
  - A failed read-back, a degraded report, or an absent or unparseable `## Hypotheses` section (or its first entry, if the assertion is scoped to rank 1) contributes nothing. It must land as `blocked` with its named precondition, never as a false `Pass`.
  - A graded miss is a typed value. `Result::Err` stays reserved for harness faults.
  - Whatever confirms that the live-writer branch was reached must be something Conductor reads from its own side. A truth that lives inside the SUT is `declared-not-observable`.
- **Artifact hygiene.** The run report, `runs.db` rows, JSONL journal, committed `evidence/` and the harvest test's embedded captures carry verdict, state and identity fields only (per §Error Handling, run-report artifact sanitization; §Security Anti-Patterns → Logging).
  - They contain no absolute host path: not the operator's `ANDROMEDA_PULSE_DATA_DIR` value, not the canonicalized `CONDUCTOR_*` directories, not the path of the Pulse log the capture came from.
  - They contain no seam-crate struct name.
  - A `blocked` row carries the identity fields and the named precondition, with measurements as JSON `null`.
  - If the harvest test READS its captures through a `CONDUCTOR_*` handle instead of embedding them, the per-READER duty applies. Its failure message names the handle and the file name, never a resolved path (per §Input Validation, env-var path handles row).
- **Open for P4: committing text rendered from the corpus.**
  - The plan forbids Conductor from persisting corpus content into its artifacts (per §Security Anti-Patterns → Data Protection; §Threat Model Summary, corpus.db note).
  - The `## Hypotheses` markdown is rendered from the incident's resolution-summary text, which is incident-corpus content that Pulse owns.
  - The plan already sanctions persisting read-back identity fields (`fingerprints[]`). Beyond those, a harvest test that embeds the hypotheses text verbatim is, on a plain reading, persistence of corpus content.
  - Whether any committed harvest already carries report text rendered from the corpus (rather than Pulse's own ledger lines) is research's question.
  - P4 must settle this explicitly, with the speech-log ingest row as the nearest precedent, or take a §Data Protection amendment to the operator. It must never land silently.

## Patterns to follow
- **Declaration-only observation.** Use `conductor_core::{flag_declared, handle_declared}`, called per handle by `conductor_run::observe_preconditions`, which renders handle NAMES only. An L4-absent-or-falsy term is the complement of the value-only `flag_declared` and keeps its non-widening witness (per security-plan §Input Validation, READ SET row).
- **The leading `conductor preconditions` arm on `{boot, run --live}`.** An unmet subject means a refusal at exit 1 that names each unsatisfied subject, with no leg fired and no sidecar spawned. The real-model leg should inherit this arm, not bypass it (per §Security Anti-Patterns → Universal).
- **The canary-freshness pattern for missing data.** Absent or unparseable means it contributes nothing, reads as not satisfied, and ends as `blocked`. Use this as the template for asserting that the rank-1 hypothesis identifies the cause (per §Input Validation, MCP child stdout row).
- **The screen-reader speech-log ingest pattern** for untrusted third-party text entering a committed record: bounded length, closed sets, host paths scrubbed to `<host-path>` with a `security_finding`, and the raw capture left under gitignored `runs/` (per §Input Validation, speech-log ingest row).
- **Spawn rules.** (per §Security Anti-Patterns → Code Patterns)
  - Anything the drive adds to `scripts/agent-run.{sh,ps1}` (the process census, a stop form, a verb call) follows rule (b)'s shapes: a fixed or resolved program, array-form argv, and operator values only as separate words after rejection checks. The nearest precedent is `conductor cleanup <run_id>` behind `valid_run_id` / `Test-RunId`.
  - The sidecar's rule (a) stays untouched: a fixed NAME resolved through the inherited `PATH`, the data dir passed via `.env(...)` only, and the console window suppressed.

## Anti-patterns to avoid
- Planting the cause, or getting hypotheses to render, by a side route: a staged corpus row or any direct `corpus.db` open or copy. The ban is on FILE access (per security-plan §Security Anti-Patterns → Data Protection).
- Getting the real-model leg through the gate by downgrading it (per §Security Anti-Patterns → Universal; §Input Validation, READ SET + MCP child stdout rows):
  - suppressing, defaulting or re-grading an unmet term or subject (`l4-deterministic`, `handles-declared`);
  - setting or unsetting an `ANDROMEDA_PULSE_*` handle from the harness;
  - reading an absent or degraded `## Hypotheses` as `Pass`.
- Widening the boundary to reach the leg (per §Threat Model Summary → Infrastructure, CI/CD; §Security Anti-Patterns → Code Patterns rule (b); §Security Anti-Patterns → Universal):
  - a CI job that runs it (dynamic end-to-end proof is an operator/local gate);
  - a new governed harness-spawn form without operator ratification;
  - a shell-string or `eval` spawn;
  - an inbound listener, or non-loopback egress.

## Contract bindings
- **obs ↔ security.** obs owns the field schema of the report, journal and `runs.db`, including the declare-only row's null verdict and known-residual state. Security owns the redaction rule and the shape of a `blocked` row: identity fields plus the named precondition, with measurements as JSON `null` (per §Error Handling).
- **tests ↔ security.** The harvest-tier test's embedded verbatim captures carry the host-path scrub and the open corpus-content question. The leg stays operator-local while the supply-chain gate stays a CI job (tests §CI Integration).
- **arch ↔ security.** The run-contract term set (including any L4 term specific to the posture), the P-ID cluster pin (`UNBACKED_AUTO` / `check_scenario_backing`) and the firing form are arch/tests decisions. Security constrains them only through three rules: never downgrade, never a scenario without a P-ID, and committed-manifest validation.

## Acceptance criteria contributions
- A grep of every file the chunk commits (report, `evidence/`, the harvest test's embedded captures) finds zero absolute host paths (drive-letter, `%APPDATA%`, `/Users` or `/home` tokens), zero occurrences of the operator's `ANDROMEDA_PULSE_DATA_DIR` value, and zero seam-crate struct names (per security-plan §Security Anti-Patterns → Logging; §Error Handling).
- On the real-model arm, an unmet precondition (for example `ANDROMEDA_PULSE_MCP_ENABLED` unset, or the sidecar unresolvable) gives a refusal at exit 1 that names each unsatisfied subject, with no leg fired and no sidecar spawned. A failed read-back, a degraded report or an absent `## Hypotheses` grades `Blocked` with its named precondition, never `Pass` (per security-plan §Security Anti-Patterns → Universal; §Input Validation, MCP child stdout row).
- The L4 disposition appears only as a handle-NAME observation of Conductor's own environment, with no L4 value and no claim about `pulse-app`. No harness code writes or unsets an `ANDROMEDA_PULSE_*` handle. The deterministic `--live` legs' gating is unchanged: a deterministic run with L4 undeclared is still refused or blocked (per security-plan §Input Validation, READ SET row; §Security Anti-Patterns → Universal).
- Boundary and supply-chain checks (per security-plan §Security Anti-Patterns → Input / Code Patterns rule (b) / Universal; §Dependency Security):
  - a grep finds no new `#[garde(skip)]` on a scenario-model field;
  - the leg's scenario names an admitted P-ID;
  - no CI job invokes the leg;
  - no inbound listener or non-loopback egress is added;
  - no governed spawn form is added without operator ratification;
  - `cargo audit` and `cargo deny check advisories bans licenses sources` run green over an unchanged `Cargo.lock`.

## Relevant amendment history
- **2026-08-10-pulse-run-contract** (two entries). Registered the run contract as a committed input boundary, together with the `ANDROMEDA_PULSE_L4_DETERMINISTIC` declaration-only row. Restated the never-downgrade ban over FIVE preconditions and added the `declared-not-observable` clause. That bullet lists its set word for word, so it goes stale whenever the count moves. This chunk's posture inverts exactly that term.
- **2026-08-13-first-live-green-preflight.** Recorded where `ANDROMEDA_PULSE_MCP_ENABLED` lives (exported by the operator, inherited by the sidecar, never set by Conductor). Also recorded the shell-side `warmup_ms` / `min_canary_poll_seconds` readers and the clamp-up rule. Why: a live arm run without the flag measured the unreachable path and was thrown away. Real-model budgets derive from these terms.
- **2026-08-16-canary-fingerprint-derivation-aligned** (child-stdout row). Made the direction for missing data explicit: an absent or unparseable stamp reads as not fresh and ends as `blocked`, never a pass. The same direction governs an absent hypotheses section.
- **2026-08-31-p-075-assert-round.** Established that corpus access is the MCP tool surface (read-back plus `mark_incident_resolved`), that the ban is on FILE access, and that staging a corpus row to force a SUT state is the banned route. The scope's "never a staged corpus row" is this ban, and its "never persist its content" clause is what the verbatim-capture question depends on.
- **2026-08-11-faithful-emission-dispatcher.** Extended the scenario-config boundary to `[phases.emission]` and required `dive`, never `skip`, after `PhaseSpec.emission` was found skipped. The injected cause and its second digest travel in emission blocks.
- **2026-09-03-live-pulse-preconditions-probed**, then the **2026-09-04** pair (a withdrawal, then its retirement). Generalised the L4 row into the READ SET (read, never written; names only). `handles-declared` was then re-adjudicated twice against measurement before the non-widening witness was settled. The real-model leg needs L4 absent on exactly that subject, where a claim about how handles are graded has already been wrong once.
  - Alongside it, **2026-08-20-latency-regression-re-proof** recorded that a `PATH` miss produces a ~0 s `[BLOCKED]` row that looks the same as a SUT-side failure. This is the posture's sidecar-resolvable term.
- **2026-09-06-operator-gated-live-suite.** Extended the short-circuit to `{boot, run --live}` (refusal at exit 1, no leg fired). The operator chose to FIX a harness recursive delete on a path derived from `CONDUCTOR_RUNS_DIR`, replacing it with non-recursive `*.jsonl` clearing, rather than widen two bans. `run --live` is this chunk's firing surface, so any real-model capture directory should keep that idiom.
- **2026-09-06-run-report-envelope-conformance-gate**, plus the rule (b) changes on **2026-09-16** and **2026-09-17**.
  - The canonicalize duty now binds per READER. The trigger was a test whose failure message leaked a host path via `path.display()` until it was fixed to name the handle and file name.
  - Rule (b) has grown one form at a time to its current list. Each new spawn crossing was ratified by the operator and no routine rule was minted.
  - The new harvest test is the same kind of failure-message surface, and any spawn the drive adds will need operator ratification again.
- **2026-09-02-screen-reader-manual-spec.** Added the speech-log ingest row (untrusted third-party text entering a committed record: bounded, scrubbed, raw capture left in gitignored `runs/`) and extended the Logging ban to committed `evidence/`. This is the nearest precedent for committing verbatim model output. **2026-09-07-a11y-ci-gate** is the precedent for the shape of any new flag-valued handle.
- The posture half, **2026-09-18-real-model-leg-posture-and-grading-rule**, made no security-plan amendment. The absorbed residual's own amendment (**2026-08-09-interpretation-correctness-posture**, which separated a tool fault from an advisory-DB fault) was re-adjudicated as a local fault at **2026-09-05-audit-corrective**. It bears only on running the gates, not on the leg.
