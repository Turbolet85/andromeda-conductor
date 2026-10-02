# Scope — 2026-10-02-p-075-assert-round-against-pulse

**Working entry (working-route.md:79):** The P-075 assert round against Pulse — one deterministic round against Pulse's
returned build, every delegated budget graded hard.

**Version:** conductor-0.3.0 · Epoch 5 — Polish & ship · taken up on the operator's relay, which cleared the head's
BLOCKED-ON (Setup HALT answered "take it up anyway" by the operator's word, 2026-10-02).

## What this chunk builds
ONE live, deterministic-posture round against the Pulse build at sha S. It grades the six assertions Pulse's
`round-request.md` names, each hard at its measured value. It also carries the span-landing subdir CARRY. The round,
its scenarios and its evidence are Conductor's. Pulse names only the binary and the assertion set.

## The binary under test (folded from the relay + re-measured)
- **S = `03ec94481b0d6c3ba574626e7acb39e33fd40141`**, on Pulse's `origin/chore/migrate-pulse-to-v3`, parent `a2addb3`.
  Re-verified at phase time: the Pulse checkout's `HEAD` and `origin/chore/migrate-pulse-to-v3` both resolve to S. The
  subject is `chore(2026-10-01-conductor-return): operator pre-CI commit`.
- Build (Pulse's, already done): `cargo build --workspace --release --features mcp-server`, per Pulse
  `evidence/round-binary.md` (gate 17, green). **No Pulse rebuild** (operator directive).
- Artifact sha256 values re-measured at phase time (2026-10-02T04:3xZ), matching the relayed prefixes:
  - `pulse-app.exe`: `8f2377e1e529b9b6c68a11b14879afc274e3fee579e15477bf9525c7efa761ab` (mtime 2026-10-02 06:14:07 local)
  - `andromeda-pulse-mcp.exe`: `d898a3b806098344cb22c814502cdfcd1d8f084f8704c1a8b2875ff66f85302a` (mtime 06:10:50 local)
- **Operator directive:** re-measure both sha256 values again immediately before launch. A mismatch is a halt, never
  a rebuild.
- Pulse's source-identity note says only evidence documents, run-dir trails and the ledger changed between the gate-17
  build and S. No `.rs`, manifest or lockfile changed, so "a rebuild at S is a no-op". This is a Pulse-side claim,
  cited and not re-derived.

## Mode (CONTEXT block, folded)
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` is set on the app child, and the data dir is fresh (CONTEXT; round-request §Mode).
- `pulse-app` and `andromeda-pulse-mcp` come from S's `target/release`. Conductor spawns the sidecar by fixed NAME
  through the inherited `PATH`, so the round's `PATH` must resolve `andromeda-pulse-mcp` to S's sidecar, not to any
  other copy on the host. (verified at P3 — research.md Conventions; checked by `which andromeda-pulse-mcp`)
- **The live run is the operator's slot plus `:4317`** (operator directive). Ports 4317/4318 are shared with Pulse's
  tree, so the live leg runs only inside that slot.

## The six graded assertions (round-request.md §The six graded assertions; CONTEXT restates them)
1. **Read-back content fidelity.** The incident read back through MCP `retrieve_telemetry_slice` carries, in
   `fingerprint_refs`, the full 32-hex fingerprint Conductor derives for the storm it emitted. The incident opened
   after the storm's emission instant. `retrieve_report` renders it with `degraded_mode: false`.
2. **Runtime-state fidelity.** `mark_incident_resolved` applies, and the incident leaves `query_incident_list`'s active
   set.
3. **P-025:** the worst `metric.constellation.hue_update_ms.duration_ms` in the leg window is ≤ 2000, graded per
   Conductor's `contracts/pulse-p025-measurement-contract.md` (the constellation dot hue, never the deferred Halo glow).
4. **P-027:** `metric.constellation.discovery_ms.duration_ms` (first-sighting anchored since Pulse
   `2026-09-30-p-027-discovery-bound`) is ≤ 5000.
5. **P-037:** `metric.report.render_ms.value` is ≤ 2000 (the Report window).
6. **P-045:** the worst `metric.findings.counter_refresh_ms.duration_ms` is ≤ 1000 (the Findings counter).

## Grading posture (round-request §Grading posture)
- Each assertion is a hard PASS or FAIL at the MEASURED value. An absent sample is UNGRADED and never counts as met.
- A FAIL is a Pulse finding: **no re-drive for a pass** (Conductor's v3-08 posture). Pulse surfaces it as its own wrap
  escalation.
- Pulse's `ref` cites Conductor's graded test ids and the evidence path at Conductor's commit, and is written only when
  all six read PASS. This chunk therefore produces stable test ids and a committed evidence path that Pulse can cite.
- Since S, a `Stop-Process -Force` teardown of `pulse-app` leaves no `app.exit` record. That is expected, not a finding
  (round-request §The binary under test).

## CARRY (folded)
- Give the span-landing witness inputs (`span-{a,b}.jsonl`, today a second, operator-local writer in the harness-owned
  `runs/live-suite/`, uncleared before drive A) their own harness-owned subdir, with a non-recursive clear before drive
  A, so a stale pair can never be graded. Origin: the overseer ruling at the
  `2026-10-01-per-run-span-identity-in-the-real-model-harness` wrap (the D-arch-collision escalation; arch §Occupied
  Resources `runs/live-suite/{leg}.jsonl`).
- Phase-time coordinates (hypotheses for P3): `crates/conductor-run/tests/span_landing_live.rs:38` reads
  `runs.join("live-suite").join("span-{leg}.jsonl")`. The harness capture dir is `scripts/agent-run.sh:69` /
  `scripts/agent-run.ps1:131`. Per security rules 2026-09-06, the clear is non-recursive BY PATTERN inside a known dir.
  (verified at P3 — the rule lives in `.claude/rules/security.md` Session Additions 2026-09-06, not the master)

## Boundaries
- No Pulse source, build or ledger is touched. Pulse's evidence is read-only from here and cited, never copied.
- The named gap stays the founder's, not the round's: `incident_events` reaches no MCP tool, so content fidelity is
  scoped to the fingerprint plus runtime state.
- One round. A FAIL is recorded, not re-driven.
- The fixed sidecar NAME and the no-listener boundary are unchanged.

## Premises closed at P3 (research.md)
- **Completion axis — VERIFIED, and it narrows the build.**
  - The `2026-08-31-p-075-assert-round` (Pulse `83d4060`) shipped assertion 2's whole mechanism:
    `probe_resolve_lifecycle`, `attribute_by_liveness`, the `live-pulse` feature, and `tests/lifecycle_live.rs` →
    `tests/lifecycle_harvest.rs`. Its verdict was PROVEN-BY-LIVENESS.
  - The four budget graders are shipped in `tests/delegated_timing_harvest.rs` (`bounds`/`grade`/`grade_in_window`).
  - What is NEW is assertion 1:
    - no shipped check compares the emitted fingerprint against `fingerprint_refs`;
    - no shipped live leg calls `retrieve_telemetry_slice`/`retrieve_report` for a named incident or stamps the
      storm's emission instant.
  - The round re-uses the graders against S and builds only that.
- `[premise-corrected: no scenario names P-075; P-037 fires from the Report webview's own fallback selection — corrected again at P5]` **The drivers:**
  - Scenarios:
    - P-025 `halo-hue-encoding`;
    - P-027 `service-constellation-discovery`;
    - P-037 `report-render-surface`;
    - P-045 `findings-counter-refresh`;
    - assertions 1–2 have no scenario (P-075 is `DriveObserve`, `coverage.rs:540-545`) and ride a feature-gated
      live leg, as in the prior round.
  - The P-025, P-027 and P-045 leaves fire with the compact widget, which is shown at boot (Pulse `main.rs:1134`).
  - **[premise-corrected at P5: `isOpen` is `effectiveId !== null` (`ReportWindow.tsx:23,32`), not visibility, so the
    hidden-at-boot Report webview fires on each newly formed incident with NO click, and a Findings-row click PINS the
    selection and suppresses samples. Overseer-ratified: nobody touches the desktop.]** The P3 reading follows, kept
    as the record. **`metric.report.render_ms` fires only from the in-app `get_report`, and only while Pulse's Report window is OPEN**
    (`Report.tsx:24-26`; the window is declared `visible: false`, `tauri.conf.json:49-61`). The MCP `retrieve_report`
    never emits it.
  - Conductor may not automate Pulse's UI, so an unopened Report window makes P-037 UNGRADED by construction. Who
    opens it is a P4 question for the operator.
- **Fingerprint representation and provenance — VERIFIED at S.**
  - `fingerprint_refs` = `incident.evidence_refs.fingerprint_hashes` (`tools.rs:439`), written ONLY at incident
    creation by `grounded_fingerprint_hashes` (`inference_runtime.rs:657-666`). That is the three `det-*` constants
    followed by the triggering cue's full 32-lowercase-hex fingerprint.
  - The derivation, `compute_exception_fingerprint` (`buffer/src/fingerprint.rs:79-96`), is equal to Conductor's
    `fingerprint()`: 0 commits since `83d4060`.
  - **Two conditions bind the assertion.**
    - The storm must CREATE the incident. A dedupe re-emission never updates `fingerprint_hashes`
      (`inference_runtime.rs:830-858`), so the P-075 leg must open on an EMPTY active set.
    - The check is MEMBERSHIP, never equality of the field.
- **`degraded_mode: false` — VERIFIED from source at S.** `degraded_mode = parsed_l4.is_none()` (`tools.rs:377-381`),
  and since `9d14166` the canned L4 output is scrubbed per leaf and stays parseable. The 2026-08-21
  `degraded_mode: true` samples predate that commit.
- **Matrix — VERIFIED.**
  - Conductor's matrix carries `v3-01`..`v3-11` (10 verified, `v3-09` deferred) and no P-075 capability.
  - Pulse's `verification-matrix.json#P-075` is Pulse's ledger.
  - This chunk links no capability.
- **New hazard from source (Q3):** the app learns of an MCP resolve only in its 60 s persist reconcile
  (`persistence.rs:250-283`). An app-side re-emission persist landing first can re-write the row `active`. The
  immediate post-resolve read is unaffected. A later re-read is a witness only, never the grade.
- **CARRY coordinates — VERIFIED, plus one finding.**
  - `span_landing_live.rs:38` reads `runs/live-suite/span-{leg}.jsonl`.
  - The live suite's own clear `rm -f "$LIVE_CAPTURE_DIR"/*.jsonl` (`agent-run.sh:106`, the ps1 twin) ALSO deletes
    the span pair. That is a cross-writer interaction in the shared dir, and the move removes it.
  - Nothing in the witness checks that the pair belongs to the CURRENT Pulse launch.
- **Sidecar on `PATH` — VERIFIED as a firing-form item** (verification-harness.md 2026-08-20, 2026-09-07). It is
  checked by `which andromeda-pulse-mcp`. The sha256 re-measure identifies the FILE, and `PATH` order decides which
  file is spawned. Both are recorded.

## CI verdict read at Setup (fold source 2)
- `e1092ce` (the last wrap's flip = HEAD): **green**, checks 3/3, wall 559 s, CI#36946764116. Nothing to fold as a red.
