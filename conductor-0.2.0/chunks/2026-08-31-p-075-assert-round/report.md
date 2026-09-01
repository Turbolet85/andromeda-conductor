# Report — 2026-08-31-p-075-assert-round

**Chunk:** P-075 assert round — the read-back fidelity Conductor can honestly assert
(`mark_incident_resolved` applied/declined + `query_incident_list` active-set membership, runtime-state not
payload), the payload-fidelity premise recorded disproved on a second axis (`incident_events` reaches no MCP
tool), and v2-20's shipped fixtures quoted as P-075's external evidence rather than re-asserted
**Date:** 2026-09-01
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:** `crates/conductor-run/src/lib.rs` · `crates/conductor-run/Cargo.toml` ·
  `crates/conductor-verify/src/client.rs` · `crates/conductor-verify/tests/common/mod.rs` ·
  `crates/conductor-verify/tests/readback.rs` · NEW `crates/conductor-run/tests/lifecycle_harvest.rs` ·
  NEW `crates/conductor-run/tests/lifecycle_live.rs` · chunk folder (scope/research/plan/report/leg-capture)

- **Symbols / APIs:**
  - NEW `conductor_verify::ReadbackClient::resolve_incident(i64)` — a typed wrapper over the existing
    `mark_incident_resolved(Option<Value>)`, so a caller need not take a `serde_json` dependency to build the
    one-field argument. The raw-`Value` method is UNCHANGED and keeps its existing callers (none in
    production before this chunk; the stub tool-list and a preflight test reference the tool NAME only).
  - NEW `conductor_run::probe_resolve_lifecycle(&ReadbackClient, i64)` — the FIRST production caller of
    `mark_incident_resolved` in the workspace. Not wired into `execute_scenario` or any run path.
  - NEW `conductor_run::{LifecycleObservation, LifecycleVerdict, select_resolve_target,
    evaluate_lifecycle, attribute_by_liveness, AUTO_RESOLVE_IDLE_SECONDS}` — pure grading surface.
  - `LifecycleVerdict` derives `PartialEq` but deliberately NOT `Eq` (the `ProvenByLiveness` arm carries an
    `f64`).
  - No IPC method, endpoint, port, socket, or env var added. No export removed.

- **Crates / modules:** none added or removed. `conductor-run` and `conductor-verify` changed; the existing
  `conductor-run → conductor-verify` edge already covered the new call (code-graph `crate_edges`), so no new
  cross-seam edge.

- **Dependencies:** **none added, none bumped.** `Cargo.lock` byte-unchanged (verified by `git diff --stat`).
  `conductor-run/Cargo.toml` gains a `[features] live-pulse = []` declaration — an empty feature with NO
  dependency, so it adds zero package nodes; it exists to keep the live-Pulse test out of default
  `nextest`/`clippy`/release builds (the `stub-server` precedent, test-plan §5 Drivers).

- **Schema / config:** none. No migration, no config key, no violation schema, no scrub/redaction shape
  changed. `conductor-core::redact::ALLOWLISTED_FIELDS` is UNCHANGED — the incident id rides the already
  allowlisted `message` field precisely because `incident_id` is not in the allowlist.

- **Spec-master edits:** none (this chunk authored no spec change; the expected amendments are listed under
  Deviations/Outcome for P2 to dispose).

- **Counts / qualifiers moved:** **none — verified.** No documented derived count changed: the consumed MCP
  tool set stays four, `ReportState` stays five, the lamp set stays six, the scenario catalog is untouched,
  `UNBACKED_AUTO` is untouched, and the coverage classification is untouched (this chunk adds no scenario and
  claims no capability).

- **Dev-tool versions:** none installed or upgraded.

- **Harness / gate surface:** no change to `scripts/agent-run.{sh,ps1}`, no xtask verb, no CI step, no
  status/verdict shape. The live leg is a feature-gated test invoked directly, deliberately NOT a harness verb
  and NOT a CI gate (test-plan §9).

- **Cross-project / external claims** (ground truth OUTSIDE this repo — `D:\dev\projects\andromeda-pulse`,
  HEAD `83d4060`, read first-hand):
  - `incident_events` has **ZERO** references in `crates/mcp-server`; it is written by
    `crates/triage/src/incident/persistence.rs::save_incident_event` and read only corpus-side
    (`crates/corpus/src/disposition.rs` + an `event_kinds` test helper in `contract.rs`). No MCP tool surfaces
    it at any width. Basis: repo-wide grep + reading the sites.
  - The eight wire tools are pinned at `crates/mcp-server/src/tools.rs:44-51`.
  - `dispatch_mark_incident_resolved` (`tools.rs:445-475`) returns `Ok({"resolved":true,"incident_id":<id>})`
    on the applied path; the DECLINED path is a JSON-RPC **error** (`ToolDispatchFailed`, reason
    `"incident changed concurrently; resolution not applied"`) driven by `IncidentWriteOutcome::DeclinedStale`.
  - `DeclinedStale` is a MONOTONIC-TIMESTAMP guard, not a race:
    `UPDATE … WHERE id = ?5 AND updated_unix_nano <= ?2`, `rows == 0 ⇒ DeclinedStale`
    (`crates/corpus/src/contract.rs:652-659`); Pulse's own tests name the older-write case and prove an EQUAL
    timestamp applies. Since the dispatch stamps `now` fresh per call, the arm is unreachable through MCP.
  - The corpus is FIVE tables at `user_version 2` (`service_registry`, `pipeline_metrics`, `incidents`,
    `incident_events`, `digest_archive`); `baseline_state` is GONE. Read directly from a live corpus DB.
  - The sidecar now reads a PUBLISHED workspace key (`workspace_detector::contract::read_published_workspace_key`,
    `{data_dir}/run/workspace-key`), falling back to `data_dir`. Measured: the published key matched the
    incidents' stamped `workspace` exactly, so the app/sidecar key divergence did NOT occur on this leg.

- **Reverted / negative API facts:** the **two-incident control** design (operator-selected at P4) was built,
  driven live, and REMOVED — Pulse's incident producer dedupes a new incident against any OPEN incident
  regardless of fingerprint, so at most one can be active per workspace and a spared control cannot exist.
  `LifecycleVerdict::Proven` is retained but documented unreachable; `attribute_by_liveness` replaced it as
  the attribution the single-incident reality supports.

- **Spec claims disproved by measurement:**
  1. **test-plan §5** states `mark_incident_resolved` has "no production call site and no live exercise".
     Both halves are now false: `conductor_run::probe_resolve_lifecycle` is a production caller, and the
     2026-09-01 live leg exercised it against a real Pulse. Evidence: `leg-capture.txt`; graph `refs` view.
  2. **The plan's own two-incident control premise** is disproved (see Reverted facts). Evidence:
     `interpretation.incident.created` `created=false deduped=true` at `2026-09-01T16:33:00.239Z`; every
     `created=true` in the leg occurred with the active set empty; incident 7 formed the same second incident
     6 was resolved.
  3. **The shared read-back stub was unfaithful on the incident item key** — it emitted `id` where the live
     sidecar emits `incident_id`. Not a spec-master claim, but it is the reason a stub-green reader read an
     empty list off a populated live corpus. Fixed in the stub; pinned by
     `lifecycle_harvest::the_live_item_key_is_incident_id`.
  4. **`contracts/mcp-contract.toml`'s header comment** still says "the rmcp client negotiates DOWN to it";
     rmcp was removed 2026-06-27. SURFACED only — owned by `v2-28` under the Epoch-5 *Cross-surface envelope
     parity* entry, deliberately not fixed here.

- **Coverage of new surfaces:**
  - `ReadbackClient::resolve_incident` → validation n/a (typed `i64`, no external input) · instrumentation
    span✓ (inherits the bounded `verify.readback.call_tool` span + allowlisted `mcp_tool` field via the shared
    `call_tool`) · PII n/a (no payload) · tests unit/integ✓ (`readback.rs` applied + declined arms) · a11y n/a ·
    tokens n/a
  - `conductor_run::probe_resolve_lifecycle` → validation n/a (no external input; the `i64` comes from a
    read-back result) · instrumentation span✓ + log✓ (`message`-borne incident id, since `incident_id` is not
    allowlisted) · PII redacted✓ (no host path, no struct name; `VerifyError::JsonRpc` Display renders the code
    only) · tests integ✓ (live leg, pinned capture) · a11y n/a · tokens n/a
  - `conductor_run::{select_resolve_target, evaluate_lifecycle, attribute_by_liveness}` → validation n/a ·
    instrumentation n/a (pure) · PII n/a · tests unit✓ (11 in `lifecycle_harvest.rs`) · a11y n/a · tokens n/a
  - No UI element, no external listener, no hot-path op added.

## Deviations from intent

1. **The two-incident control leg (plan step 5, operator-selected at P4) was not achievable and was
   replaced.** Justification: measured live — Pulse dedupes against the open incident regardless of
   fingerprint, so a second concurrent incident cannot be formed. The substitute, liveness attribution, is
   strictly weaker in form but sound: Pulse auto-resolves only an incident idle ≥120s, so resolving a
   freshly-refreshed one and observing it leave the active set excludes the resolver by construction. The leg
   measured `idle_seconds_at_resolve = 0.0` and Pulse's corpus independently records incident 6 active
   16:42:38→16:43:23 (45s against a 120s threshold).
2. **A `live-pulse` cargo feature and `tests/lifecycle_live.rs` were added beyond the plan's touchpoints.**
   Justification: the plan named a live leg as its own producer but provided no invocation path for it; a
   feature-gated test is the established in-repo convention (`preflight_spawn.rs` under `stub-server`). The
   feature declares no dependency, so `Cargo.lock` is byte-unchanged.
3. **`conductor-verify/src/client.rs` was edited** where the plan said "possibly none". Justification: the
   plan explicitly listed it "because the declined-arm typing may want a small typed helper beside it"; the
   helper avoided adding `serde_json` to `conductor-run`'s manifest, which would have been a dependency delta
   changing the standing audit deferral's stated basis.
4. **The declined arm is stub-proven, not live-proven.** Justification: unreachable through the MCP surface
   (monotonic-timestamp guard, above). Recorded as stub-proven per test-plan §11, never as a live claim.

## Decisions & corrections

- **Operator decision (P4):** resolve a second incident and keep one as control, rather than resolving the
  canary's own. Superseded by measurement during implement (deviation 1) — the *reason* for the choice
  (auto-resolve is indistinguishable from a write on a single-incident set) survived and drove the
  liveness substitute.
- **Correction to my own P2/P3 implement report:** I reported the smoke as "skipped — no live Pulse". The
  operator supplied warm release binaries; the leg then ran and passed. The earlier statement is superseded.
- **Diagnostic discipline that resolved the leg:** four legs read an empty active set while Pulse's corpus
  held incidents active for 131–142s each. Only dumping the RAW wire value separated "Pulse never exposed it"
  from "our reader extracted nothing" — the second was true, twice (library helper, then the test's own local
  copy of the same helper).
- **Pulse's `agent-run boot` pre-builds a DEBUG profile.** With release binaries already warm, that cold
  compile consumed the whole budget and was killed mid-crate; spawning `target/release/pulse-app.exe`
  directly is what its boot ultimately does anyway.

## Outcome

**Acceptance criteria met**, with one refined by measurement (the control criterion — see Deviations 1).

Gates run, all green, exits read direct (never through a pipe):
- `cargo nextest run -p conductor-verify` → 0 (101 passed)
- `cargo nextest run -p conductor-run` → 0 (149 passed)
- `cargo test -p conductor-verify` → 0 · `cargo test -p conductor-run` → 0 (runner-portability)
- `cargo clippy --workspace --all-targets -- -D warnings` → 0
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` → 0
- `cargo test --workspace --doc` → 0

**Live leg (operator/local, not a CI gate):** ran against Pulse HEAD `83d4060`, release binaries,
deterministic L4, fresh data dir under `%TEMP%/pulse-legs/`. Outcome **PROVEN-BY-LIVENESS**: incident 6
resolved through `mark_incident_resolved`, left the active set, `idle_seconds_at_resolve = 0.0`. Pipeline
witnesses on the way: 12 spans ingested, ZERO `reject_reason`, `span_events_seen: 12`,
`fingerprints_computed: 12`, `storms_detected_total` advancing, `severity_hint: "autonomous"`. Capture archived
at `leg-capture.txt` and pinned in `lifecycle_harvest.rs`. Teardown clean — `pulse-app` stopped, `:4317`
released.

**Smoke:** the live leg IS the boot-path exercise; it ran and passed.

**Expected amendments (wrap):**
- `architecture.md` §Established Decisions [Read-Back Dependency Posture] — the second-axis disproof
  (`incident_events` reaches no MCP tool at HEAD `83d4060`), what runtime-STATE fidelity now proves, the
  one-active-incident dedupe constraint, and the published-workspace-key mechanism now on the SUT side.
- `architecture.md` §Standard Contracts (readiness gate) — the TWIN site of the payload-fidelity posture; the
  duplicate-occurrence precedent requires both move together.
- `test-plan.md` §5 — the `mark_incident_resolved` bullet ("no production call site and no live exercise") is
  falsified on both halves; check §1/§6 for a twin under the two-site rule. Also: the stub's item-key
  infidelity is a mocking-discipline fact belonging here.
- `obs-plan.md` §6 — whether the boundary-call must-log set should name the lifecycle write explicitly.
- **Cross-version residual (route-resolve's channel, NOT an amendment):** `incident_events`-through-MCP as a
  Pulse 0.4.0 candidate.
