# Report — 2026-08-16-canary-fingerprint-derivation-aligned

**Chunk:** Canary fingerprint derivation aligned — Conductor adopts Pulse's own derivation over the stacktrace Pulse actually hashes, retiring the FNV-1a-over-frame-functions invention, with the comparison surface settled by research and the P-017 identity semantics held
**Date:** 2026-08-16T09:59:45Z
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:** `Cargo.toml` · `Cargo.lock` · `deny.toml` · `crates/conductor-emit/Cargo.toml` ·
  `crates/conductor-emit/src/exception.rs` · `crates/conductor-verify/src/preflight.rs` ·
  `crates/conductor-verify/src/extract.rs` · `crates/conductor-verify/tests/common/mod.rs` ·
  `crates/conductor-verify/tests/preflight.rs` · `crates/conductor-run/src/lib.rs` ·
  `conductor-0.2.0/verification-matrix.json` · `conductor-0.2.0/working-route.md` ·
  `.andromeda/master-route.md` · the chunk folder (scope/research/plan/report/evidence)

- **Symbols / APIs:**
  - CHANGED `conductor_emit::exception::fingerprint(&ExceptionSpec) -> String` — signature unchanged;
    derivation replaced (FNV-1a 64-bit over `exception_type` + frame `function`s → **blake3 over
    `exception_type` + `b"\0"` + `normalize_stacktrace(render_stacktrace(spec))`, first 16 bytes**).
  - NEW pub consts `conductor_emit::exception::NORMALIZED_FRAMES = 3` · `FINGERPRINT_BYTES = 16`.
  - NEW private fns in `exception.rs`: `normalize_stacktrace` · `normalize_frame` · `is_hex_address_start` ·
    `skip_hex_address` · `is_absolute_path_start` · `skip_absolute_path` · `is_path_char` ·
    `skip_line_number_suffix` (semantics transcribed from `andromeda-pulse crates/buffer/src/fingerprint.rs`).
  - REMOVED private `struct Fnv1a` + impl (zero references outside `exception.rs`, per the code-graph query).
  - CHANGED `conductor_verify::CanaryMarker` — NEW pub field `emitted_at_unix_nano: i64`;
    `CanaryMarker::new` arity **2 → 3**.
  - NEW `conductor_verify::extract::opened_at_unix_nanos(&Value) -> Vec<i64>` (`pub(crate)`).
  - CHANGED `assert_canary` — no longer calls `retrieve_telemetry_slice`; asserts incident freshness.
    **Scoped to the canary only:** per-check read-back extraction (`extract.rs`) still calls
    `retrieve_telemetry_slice` (and `retrieve_report` / `query_incident_list` / `mark_incident_resolved`),
    so Conductor's MCP client tool set is UNCHANGED at four and the pinned contract manifest is untouched.
  - RENAMED internal `NotFound::FingerprintAbsent` → `NotFound::StaleCorpus`; the precondition string it
    surfaces changed (see Counts/qualifiers).
  - REMOVED `ShapeWitness::slice` (dead once the canary stopped calling that tool).
  - NEW private `conductor_run::now_unix_nanos() -> i64` (`std::time::SystemTime`, never the virtual clock).
  - **No new** env var · port · socket · IPC method · endpoint · crate.

- **Crates / modules:** none added · none removed. Changed: `conductor-emit` (derivation),
  `conductor-verify` (canary gate + reader), `conductor-run` (emission stamp + doc twins).

- **Dependencies:** **ADDED `blake3 = "1"`** — pinned in root `[workspace.dependencies]`, a **normal
  (non-dev)** dependency of `conductor-emit`; resolves to **1.8.6**. Pin matches the SUT's own
  (`andromeda-pulse Cargo.toml:154`) per the Pulse-consistency mandate. Four transitive packages entered
  `Cargo.lock`: `arrayref` · `arrayvec` · `constant_time_eq` · `cpufeatures` (+45/−3 lines). None bumped.
  **`.andromeda/architecture.md` §Stack and Technologies carries NO hashing/digest row today.**

- **Schema / config:** `deny.toml` `[licenses] allow` **+= `"BSD-2-Clause"`** with a justifying comment
  naming the crate and its path (`arrayref`, via `conductor-emit→blake3`; OSI-approved + FSF Free/Libre).
  No migration, no config key, no violation-schema change. The Run-report envelope is byte-unchanged
  (11 fields); `fingerprints[]` is still fed from read-back `Observation#fingerprints`, never from
  `exception::fingerprint()` — the two never meet.

- **Spec-master edits:** none (implement authored none; this wrap's P2 owns any).

- **Counts / qualifiers moved:**
  - Fingerprint **rendered width 16 hex chars → 32 hex chars** (64-bit FNV → 16-byte blake3). Stated in
    `.andromeda/architecture.md` §Established Decisions [Read-Back Dependency Posture] and §Standard
    Contracts; the cli `Fingerprints` column and `RunReport.tsx` render a value/count, not a literal.
  - Fingerprint **frame contribution `MAX_FRAMES = 64` → `NORMALIZED_FRAMES = 3`** (rendered stacktrace
    still bounded at 64).
  - Canary precondition STRING: `"canary fingerprint not found in telemetry slice"` → `"no incident opened
    after the canary storm was emitted — every incident in the corpus predates it, so Pulse did not raise
    one for this run"`. The gate's precondition COUNT is **unchanged at FIVE**.
  - Workspace test count **584 → 588**.
  - `deny.toml` `[licenses] allow` entries **8 → 9**.

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:** none — nothing was written then withdrawn.

- **Spec claims disproved by measurement:**
  1. **The canary's fidelity carrier.** `.andromeda/architecture.md` §Standard Contracts (Readiness gate)
     and §Established Decisions [Read-Back Dependency Posture] both state the gate asserts the emitted
     fingerprint reads back via `retrieve_telemetry_slice.fingerprint_refs`. **Measured false.** That field
     is populated from the L4 model's `evidence_refs`
     (`andromeda-pulse pulse-app/src/inference_runtime.rs:684-701`), which the deterministic-L4 fixture pins
     to `[]` (`pulse-app/src/deterministic_inference.rs:35`); the chain is
     `deterministic_inference.rs:35 → interpretation/markdown.rs:286,:310-318 → mcp-server/tools.rs:436`.
     Pulse's computed fingerprint lands in `span_events.fingerprint`, and **`span_events` has zero reads in
     `mcp-server`** — no read-back surface at any width. Confirmed live: `retrieve_telemetry_slice` returned
     `result_count: 0` on a healthy leg and the envelope carries `fingerprints: []`.
  2. **P-017 amended clause (c) — path-insensitivity.** **Verified at wrap: the claim appears in NONE of the
     seven spec masters** (`grep -n 'P-017'` across all seven returns zero hits; the only `insensitiv` hits are
     `.andromeda/input.md:58`, which asserts LINE-insensitivity — still true — and a `master-route.md` chunk
     description, both outside the amendment surface). Its live statements are
     `crates/conductor-emit/src/exception.rs` doc comments (corrected in this chunk) and
     `scenarios/fingerprint-storm.toml`'s header prose (source/config — not wrap's to edit; carried at
     route-resolve). **Measured false, in TWO independent narrowings:**
     (a) only the **first 3 normalized lines** contribute, so a frame past that bound is invisible to
     identity; (b) `normalize_frame` strips **absolute paths only**, and Conductor's frames are relative by
     construction (guarded by the shipped `stacktrace_carries_no_absolute_host_path`), so **a relative-path
     change IS identity-significant**. Evidence: `PathVariant` → `bbefab94922dba433da0ae1e82f7d379` vs base
     `cbe26ad382329351acf14d822346a0d4`.
  3. **The warm-up's stated purpose** (already recorded as disproved in arch) — the source doc twin in
     `conductor-run/src/lib.rs` that still taught it is corrected in this chunk.

- **Coverage of new surfaces:**
  - `conductor_emit::exception::fingerprint` (changed derivation) → validation n/a (pure fn over an
    already-validated in-process spec) · instrumentation n/a (obs-plan §11 names fingerprint generation a hot
    path that must be profiled BEFORE gaining a span — none added) · PII n/a (preimage is `exception_type` +
    normalized stacktrace; `exception.message` never enters it, and normalization strips absolute paths) ·
    tests unit (5 in-crate: line-insensitive variants · relative-path significance · frame-bound ·
    divergent variants · width/lowercase-hex) · a11y n/a · tokens n/a
  - `conductor_verify::extract::opened_at_unix_nanos` (new read-back reader) → validation ✓ (absent /
    unparseable stamp contributes nothing, so it degrades to NOT-fresh — never reads as satisfied) ·
    instrumentation ✓ (rides the existing `verify.readback.preflight` span + the one-shot
    `query_incident_list` key-set witness; no new span name) · PII n/a (reads an integer stamp) ·
    tests integ (stub legs: fresh · stale · stamp-absent) · a11y n/a · tokens n/a
  - `conductor_verify::CanaryMarker.emitted_at_unix_nano` (new pub field) → validation n/a (Conductor-
    produced `std::time` value) · instrumentation n/a · PII n/a · tests integ (the three stub legs above +
    the live leg) · a11y n/a · tokens n/a
  - **No new** external surface, UI element, or hot-path op was added.

## Deviations from intent

1. **The chunk's central premise was falsified at P3 and the mechanism changed** (goal kept). Planned:
   align the derivation *so the round-trip's last precondition can pass*. Measured: the precondition could
   never pass at any width, because `fingerprint_refs` carries no Pulse-computed fingerprint. Resolution,
   operator-ratified at P4: keep the goal (preflight reaches `ready:true`), re-aim the precondition at
   **incident freshness** (`opened_at_unix_nano` > emission stamp), and adopt the derivation anyway on its
   own merit (it fixes a real divergence in how Conductor predicts Pulse's exception grouping).
   Justification: the standing rule that a falsified mechanism does not falsify the goal.

2. **`deny.toml` edited though absent from research's Files-to-modify.** blake3 pulls `arrayref` under
   BSD-2-Clause, outside the allow-set, failing `cargo deny check licenses`. Judged in-scope rather than an
   out-of-scope soft-exit: the plan's own acceptance criterion mandates "any new exception justified inline",
   and security-plan §Dependency Security names this exact mechanism for a permissive OSI+FSF license. One
   justified entry naming the crate and its dependency path — not a blanket widening.

3. **Plan step 5 under-predicted the P-017 narrowing.** It anticipated only the frame-count bound and
   asserted the identity triple would still pass. The gates caught the second narrowing (relative paths).
   Tests were re-pinned to measured behavior and the divergence surfaced rather than silently absorbed.

4. **A second live arm was run beyond the plan's boot leg.** `conductor preflight` is a gate, not a run —
   the boot leg wrote no journal and no `runs.db` row, while `v2-10`'s acceptance requires both. Rather than
   set `implemented` against Aug-14 residue, a scenario leg (`SCENARIO=span-status-error-detection
   agent-run.sh run`) was run through the now-green preflight to produce this-session artifacts.

## Decisions & corrections

- **Operator decision (P4, recommended-first):** the canary asserts **incident freshness by
  `opened_at_unix_nano`**, over the alternatives of a corpus-count increase or keeping the fingerprint
  assertion and re-scoping the chunk.
- **Operator decision (P4):** **adopt Pulse's derivation now** rather than correcting the docs only —
  accepted the first hashing dependency while `cargo audit` cannot parse its database, covered by the
  `cargo deny` overlap.
- **Operator-supplied ground truth** (verified at Pulse HEAD `d090314`): the falsified-premise citation
  chain and `opened_at_unix_nano`'s provenance from `row.created_unix_nano` (`tools.rs:412`) — cited, not
  re-derived.
- **Width lean recorded, not asked:** the canonical fingerprint is the **full 16 bytes** (32 hex). Pulse's
  8-char prefix is a logging convenience (`fingerprint.rs:98-108`), not a wire identity, and with no
  comparison surface there is nothing to truncate for.
- **A truncated grep nearly misled the setup:** `grep … | head -6` suggested only the sidecar read
  `ANDROMEDA_PULSE_DATA_DIR`; reading `resolve_data_dir` showed `pulse-app` reads it FIRST
  (`main.rs:181-183`). Had the truncated view stood, the leg would have run with divergent data dirs and
  measured nothing.
- **Honesty register held:** the adopted derivation was never compared against a Pulse-computed
  fingerprint (none is observable); its correctness rests on transcription + unit tests.

## Outcome

**Acceptance criteria: met**, with one criterion's mechanism changed by operator decision (above).

Gates — all re-run green at implement:
- `cargo nextest run -p conductor-emit` · `-p conductor-verify` · `-p conductor-run` — green
- `cargo nextest run --workspace --profile ci` — **588/588**, zero retries (584 before)
- `cargo test --workspace --doc` — exit 0
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo deny check advisories bans licenses sources` — **true exit 0** across all four classes (after the
  justified BSD-2-Clause entry)
- `cargo audit` — **RED, 22nd consecutive**, byte-identical `error loading advisory database: parse error:
  duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1. External advisory-DATABASE fault; no floor to
  raise. **Basis CHANGED this chunk:** `Cargo.lock` moved (blake3 + 4 transitive), so the deferral can no
  longer rest on "no dependency delta" — the new dependency is covered by the `cargo deny` overlap alone.

**Smoke (boot-path changed):** ✓. Two arms, live Pulse at HEAD `d090314`, fresh data dir
`D:/dev/tmp/pulse-leg-0946`, deterministic L4 confirmed active in Pulse's own log.
- Arm 1 `agent-run.sh boot` → **`ready:true`, exit 0** — the first green preflight in the project's history.
  `canary_round_trip: "ok"`, `blocked_precondition: null`, `data_dir: "<redacted>"`.
  The re-aimed precondition fired visibly: `query_incident_list` `result_count: 0` (09:52:26.939, retry) →
  `1` (09:52:27.952, Ok), with Pulse's incident-persist line at `09:52:27.074Z`, after the storm emitted at
  `09:52:26.9`.
- Arm 2 `SCENARIO=span-status-error-detection agent-run.sh run` → `[RESIDUAL]`, exit 0, producing
  `runs/2026-08-16T09-54-12-950.jsonl` + the `runs.db` row: **`verdict: "Pass"`, `state: "KnownResidual"`,
  `latency_ms: 2153`, `slo_tier: "<5s"`** — the first run record with a non-null verdict and a real
  journal-relative SLO measurement (the prior row is `verdict: null` / `state: Blocked` / all-null
  measurements under the Blocked-row rule).
- Pulse-side telemetry harvested post-leg from `{data_dir}/logs/agent-latest.jsonl.2026-08-16`, sliced by
  pre-leg line count (10,759 → 32,663): **27 `duckdb.append` lines, ZERO `reject_reason`**;
  `severity_hint: "autonomous"` ×3 at `occurrence_count: 10`; incident formed. Console tee remains 0 bytes
  (GUI-subsystem binary).

Full leg record: `evidence/leg-verdict.md`.

**Honest limits (recorded, not designed away):** freshness proves causation-in-time, not payload identity —
a concurrent unrelated incident inside the poll window would satisfy the gate, and under deterministic L4 no
stronger claim exists because every L4-authored field is a fixture constant.
