# Report — 2026-08-13-per-check-read-back-extraction

**Chunk:** Per-check read-back extraction — real observed values from the corpus tools feeding the unchanged
evaluate/classify path, with a degraded_mode read-back as KnownResidual (conductor-run/verify, v2-09)
**Date:** 2026-08-13T20:00:00Z
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** 6 source/test — new `crates/conductor-verify/src/extract.rs`; modified
  `crates/conductor-run/src/lib.rs`, `crates/conductor-verify/src/lib.rs`,
  `crates/conductor-verify/src/preflight.rs`, `crates/conductor-verify/tests/common/mod.rs`,
  `crates/conductor-verify/tests/readback.rs`. Plus `conductor-0.2.0/verification-matrix.json` (ledger) and
  bookkeeping (master-route promotion, working-route stamp, friction-log, handoff, the chunk + phase run dirs).

- **Symbols / APIs:**
  - NEW public (`conductor-verify`): `Observation` (struct: `text`, `evidence_count`, `degraded`,
    `fingerprints`), `Observation::observed_for(ComparisonKind) -> Cow<str>`,
    `ReadBackOutcome` (enum re-export of `extract::Outcome`: `Observed` | `EmptyCorpus` | `CallFailed(String)`),
    `observe(&ReadbackClient) -> ReadBackOutcome`.
  - MOVED (no signature change, `pub(crate)`): `call_error_reason`, `incident_ids`, `fingerprint_refs` —
    `preflight.rs` → `extract.rs`, one definition now shared by the canary and the per-check extraction.
  - NEW private (`conductor-run`): `state_for(&Observation, ReportState) -> ReportState`.
  - CHANGED private (`conductor-run`): `manual_record` gained an `&Observation` parameter.
  - UNCHANGED public: `execute_scenario`'s signature is byte-identical (verified: `git diff` on
    `crates/conductor-cli/` and `crates/conductor-tauri/` is empty — all three production call sites untouched).
  - UNCHANGED: `evaluate_check`, `compare`, `evaluate_slo`, `classify` — `git diff` on
    `conductor-verify/src/slo.rs` and `verdict.rs` is empty.
  - **New span name:** `verify.readback.observe` — conforms to obs-plan §11's bounded set (`verify.readback*`);
    no new span-name family.
  - No new IPC method, endpoint, socket, port, or env var.

- **Crates / modules:** added module `conductor-verify::extract`. No workspace crate added/removed. No new
  cross-crate edge (`conductor-run → conductor-verify` and `conductor-cli → conductor-verify` already existed).

- **Dependencies:** **none added, none bumped.** `Cargo.lock` moved **zero lines** this chunk (un-drifted);
  zero new `[[package]]`.

- **Schema / config:** none. The run-report envelope stays **11 fields** with unchanged names/semantics; no
  `runs.db` column added; `ReportState` stays the closed 5-variant set and `Verdict` the closed 3. Test-only:
  4 new `StubConfig` knobs (`report_markdown`, `report_degraded`, `span_ref_count`, `malformed_results`) plus a
  `retrieve_report` arm the stub previously lacked.

- **Spec-master edits:** none. `/implement` authored no spec change (two gaps were SURFACED for this wrap — see
  Deviations).

- **Counts / qualifiers moved:**
  - Workspace test count **558 → 574** (+16). Checked: no master states a test-count literal (`test-plan.md`
    carries none), so this moves no documented value.
  - The envelope's `fingerprints` field is now **populated from `retrieve_telemetry_slice.fingerprint_refs`**
    where it was previously always `Vec::new()`. obs-plan §6 / a11y-plan §3 describe it as "may be empty",
    which stays true (a Blocked row is `null`, a measured row may legitimately be empty) — no drift.
  - `retrieve_report` gained its **first workspace call sites** (code-graph showed 0 before this chunk),
    despite being pinned in `contracts/mcp-contract.toml` and preflight-asserted throughout.

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:** the `verify.readback_degraded_mode` span and its
  `degraded_mode_requested` / `response_received` attributes — named by obs-plan §4 — were deliberately **NOT
  built**. Two independently disqualifying reasons, both artifact-verified: `degraded_mode` is computed and
  RETURNED by Pulse (`parsed_l4.is_none()`, `andromeda-pulse/crates/mcp-server/src/tools.rs:372,380`), so
  `degraded_mode_requested` describes a call Conductor cannot make; and neither attribute name is in
  `conductor-core::redact::ALLOWLISTED_FIELDS`, so a built attribute would be dropped at the processor stage
  and emit nothing. The degraded signal ships on the allowlisted `message` field via a `warn` line instead.

- **Coverage of new surfaces:**
  - `conductor_verify::observe` (new consumer of the MCP child-stdout boundary) → validation **✓** (consumes the
    shipped bounded line-delimited decode; adds no second decode path) · instrumentation **✓**
    (`verify.readback.observe` span + `info` completion line + `warn` on degraded, all carrying `run_id`) ·
    PII **redacted✓** (observed text reaches artifacts only through `classify`'s `redact_value`; a leak scan
    over this run's journal + report + `agent-latest.jsonl` found zero host paths / struct names / corpus
    content, with every emitted field name allowlisted) · tests **unit + integration** (6 stub-driven legs
    + 6 unit) · a11y **n/a** (no UI) · tokens **n/a** (no UI).
  - `Observation::observed_for` (pure per-kind resolver) → validation **n/a** · instrumentation **n/a** ·
    PII **n/a** · tests **unit ✓** · a11y **n/a** · tokens **n/a**.
  - `conductor_run::state_for` (pure state override) → validation **n/a** · instrumentation **n/a** ·
    PII **n/a** · tests **unit ✓** (both branches, all three measured states) · a11y **n/a** · tokens **n/a**.
  - `fingerprints` now written to journal / `runs.db` / report → validation **n/a** · instrumentation **n/a** ·
    PII **redacted✓** (leak scan clean; entries are Pulse's hex fingerprint refs) · tests **unit ✓** ·
    a11y **n/a** · tokens **n/a**.

## Deviations from intent

1. **obs-plan §4's degraded-path span attributes were not built as worded.** Justification: two artifact-verified
   disqualifiers (see *Reverted / negative API facts*) — `degraded_mode` is response-only, and the attribute
   names are not allowlisted so they would emit nothing. The plan's acceptance criterion says "its **allowlisted**
   attributes", which the shipped `warn`-line approach satisfies. `conductor-core/src/redact.rs` was outside the
   plan's Files-to-modify, so widening the allowlist was not taken unilaterally.

2. **`state_for` was extracted rather than the override inlined at both call sites.** Justification: two call
   sites need identical logic; one named function prevents drift between the empty-`expected` branch and the
   measured branch, and makes the acceptance's "measured verdict preserved" clause directly testable. Two
   divergent copies of a state rule is the failure this avoids.

3. **One plan acceptance criterion was over-strong and was not implemented literally.** The plan said "only the
   two KNOWN-RESIDUAL declarers can reach `KnownResidual`". Justification: Pulse computes `degraded_mode` per
   *incident report*, so degradation is a property of the SUT's response, not of the scenario — implementing the
   clause literally would hard-code a scenario allowlist into the run seam. `architecture.md:60` already scopes
   `KnownResidual` response-side ("a `retrieve_report` result returned under `degraded_mode`"), so the shipped
   `state_for` matches the master and **no amendment is owed**; the over-strong clause existed only in `plan.md`
   and this record is its complete trail. What is tested instead is the real regression risk: under a
   non-degraded read-back the declare-only rows still render `ManualCheck` — the routing does not sweep them.
   The `v2-09` matrix acceptance contains no such clause and is cleanly satisfied.

## Decisions & corrections

- **P4 scope decisions (operator, recommended-first dialogue):** per-check resolution DERIVES from
  `ComparisonKind` rather than a scenario-model change (a single composed observation would have failed
  `parse::<i64>()` on all 7 catalog floor-checks and silently softened them to `CalibrationRegion`);
  `degraded_mode` routes at RECORD level (per-check-only would never reach P-053, whose `expected` is empty);
  `fingerprints` IS populated.
- **Premise correction (research, verified against the live SUT):** the specs phrase `degraded_mode` as a
  requestable argument; Pulse returns it. `query_incident_list` takes no arguments at all. Recorded in
  `verification-matrix.json#v2-09` `notes`.
- **Operator wrap directives (this session):** (1) counts come from `git status` as standing practice;
  (2) the obs-plan amendment covers the full THREE-wording set in one pass — `degraded_mode_requested`,
  `response_received` (allowlist), and `mcp_method`→`mcp_tool` — rewording the CP5 span line to
  response-observed semantics and stating the allowlist constraint; (3) deviation 3 needs no master amendment
  and a detector proposing a scenario-allowlist for `KnownResidual` must be REJECTED as contradicting arch:60;
  (4) `cargo audit` re-pins silently under the L5 ratification — thirteenth pin, origin unchanged.
- **Test-expectation correction (self-caught):** the new `malformed_results` stub knob's match arm sat after the
  per-tool query arm, so it did not mean what its doc said. The product readers had degraded correctly; the
  stub was fixed, not the assertion — the never-change-a-test-to-match-buggy-code check ran first.

## Outcome

**Acceptance criteria: met.** `v2-09` satisfied at the stub tier with its three clauses individually proven, two
of them verified literally against the diff (`slo.rs`/`verdict.rs` zero-diff for "both functions unmodified";
`conductor-cli`/`conductor-tauri` zero-diff for "three production call sites untouched").

**Gates green** (2 fix-loop iterations): `cargo nextest run -p conductor-verify -p conductor-run -p conductor-cli`
146/146 · `cargo nextest run --workspace --profile ci` **574/574**, zero retries · `cargo test --workspace --doc`
7 suites ok · `cargo clippy --workspace --all-targets -- -D warnings` clean ·
`cargo deny check advisories bans licenses sources` all four ok (true exit 0). No gate deferrals — the Rust
delta is non-zero, so every gate ran.

**`cargo audit` — THIRTEENTH consecutive red, re-pinned silently** under the operator's 2026-08-10 L5
ratification (origin `2026-08-08-sut-capability-manifest`). Byte-identical advisory-DATABASE fault
(`duplicate advisory ID: RUSTSEC-2026-0244`), true exit 1. The standing basis held **stronger than at any prior
pin** and was re-verified literally, not echoed: `Cargo.lock` moved **zero lines** (audit surface unchanged,
zero new `[[package]]`) and `cargo deny check` was observed green as the overlapping signal.

**Smoke ✓** — `SCENARIO=degraded-mode-report bash scripts/agent-run.sh run` → `[BLOCKED]` exit 0;
`agent-run.sh status <run_id>` returned a truthful envelope. Artifact freshness confirmed against a pre-run UTC
marker (artifact 19:51:36Z > marker 19:51:08Z, `run_id` matching), Blocked-row null rule held across journal /
report / status. **Honest limit:** the run blocked at the preflight gate (no live Pulse), so
`verify.readback.observe` never executed on that path — the smoke proves the boot-path and artifact hygiene,
not the extraction, which is proven at the stub tier. Proving it live is `v2-10`'s job, the next route entry.
