# Report — 2026-10-01-per-run-span-identity-in-the-real-model-harness

**Chunk:** per-run span identity — two same-seed real-model drives inside one Pulse buffer window both land
**Date:** 2026-10-01T23:58Z
**Commits:** `163e0f6 chore(2026-10-01-per-run-span-identity-in-the-real-model-harness): operator pre-CI commit, for
the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-01T20:56:09Z; base `2c97d3b`)

## Changes (structured — detectors read this)
- **Files:** basis `git diff --stat 2c97d3b HEAD -- crates/`, 6 files, +538/−44.
  - New: `crates/conductor-emit/src/identity.rs` (+165), `crates/conductor-run/tests/span_landing_live.rs` (+202).
  - Modified: `crates/conductor-emit/src/lib.rs` (+4), `crates/conductor-run/src/dispatch.rs` (95 lines),
    `crates/conductor-run/src/execute.rs` (10 lines), `crates/conductor-run/tests/dispatch_wire.rs` (+106).
  - Chunk folder: `evidence/revert-red.md`, `evidence/witness-ledger.md`, this report.
- **Symbols / APIs:**
  - NEW `conductor_emit::rekey_trace_identity(request: &mut ExportTraceServiceRequest, salt: u64)` (public re-export,
    `identity.rs`).
    - One `ChaCha8Rng::seed_from_u64(salt)` draws a 16-byte trace mask, THEN an 8-byte span mask (the order is
      contractual).
    - It XORs every NON-EMPTY `trace_id` with the trace mask, and every `span_id`, `parent_span_id` and link
      `trace_id`/`span_id` with the matching mask. An empty id stays empty.
    - It is a bijection per salt, preserves linkage, and moves no content byte.
    - It is a pure function: no clock, no I/O, no `tracing`.
  - NEW public re-export `conductor_emit::ExportTraceServiceRequest` (= `opentelemetry_proto::…::v1::ExportTraceServiceRequest`).
    It names the type every `conductor-emit` builder already returned, for a caller that holds one between build and
    export. This was needed because `conductor-run` carries `opentelemetry-proto` as a DEV-dependency only.
  - CHANGED `conductor_run::Dispatcher::connect(scenario, endpoint)` → `connect(scenario, endpoint, identity_salt:
    Option<u64>)`. The salt is a required parameter, so omitting it is a compile error.
    - `None` keeps identity seed-pure: the tier the `dispatch_wire__*` goldens pin.
    - `Some(salt)` re-keys every TRACE export through one private helper `Dispatcher::export_traces`. That covers
      Plain/Traces, Error, Exception, Latency, Pii/Traces, Ramp, Breathing and Topology: all 8 `self.traces.export`
      sites, basis `re.subn` count 8.
    - The logs path is untouched.
    - Remaining callers, basis `grep -rn "Dispatcher::connect" crates/`: the production site `execute.rs` (passes
      `Some(emitted_ms as u64)`) and the test macro `dispatch_wire.rs` `drive_scenario!` (passes `None`, plus a new
      `salt = …` arm passing `Some`). There are no others.
  - `execute_scenario`: signature unchanged. It now feeds the dispatcher the `std::time` `emitted_ms` it already read,
    the same `now_ms()` base the canary uses (`canary.rs`).
  - `emission_seed` and every builder's seed input are UNCHANGED, so content (PII corpus, latency, rate, fingerprints)
    stays a pure function of seed.
  - No port, socket, env var, artifact path or process was added. The live witness reads the existing
    `CONDUCTOR_RUNS_DIR` / `ANDROMEDA_PULSE_DATA_DIR` handles through `capture_paths`.
- **Crates / modules:** module `identity` added to `conductor-emit`. Test binary `span_landing_live` added to
  `conductor-run`, gated `#![cfg(feature = "live-pulse")]` and auto-discovered, so there is no Cargo.toml edit. No
  crate added or removed, and there is no new cross-crate edge (`grep -c "conductor-core" crates/conductor-emit/Cargo.toml`
  → 0).
- **Dependencies:** none added or bumped. The package count is 562 (`grep -c "^name = " Cargo.lock`), and there is no
  Cargo.toml or Cargo.lock diff (base-diff probe → no output).
- **Schema / config:** none. No scenario, contract or manifest changed.
- **Spec-master edits:** none (implement wrote no master).
- **Counts / qualifiers moved:**
  - `dispatch_wire` test count 15 → 16. Basis: plan baseline "15 tests run: 15 passed" vs gate entry 3's `16 tests
    run: 16 passed`.
  - `conductor-emit` gained 5 unit tests.
  - Workspace nextest reads 1153 passed. Basis: gate entry 6 `1153 tests run: 1153 passed, 0 skipped`. No prior
    workspace total is stated in this chunk's plan.
- **Dev-tool versions:** none. cargo-audit and cargo-deny were re-read and unchanged.
- **Harness / gate surface:**
  - NEW operator-gated live witness `cargo test -q -p conductor-run --features live-pulse --test span_landing_live --
    --nocapture`. It is never a CI gate and mints no `agent-run` verb or selector (`scripts/` diff → none).
  - It prints exactly one summary line, `span-landing: PASS run_ids=N delta_ms=N retention_ms=N reject_lines=N
    append_rejections=N spans_after_b=N`, and panics with `span-landing: FAIL …` otherwise.
  - Its inputs are frozen journals `runs/live-suite/span-a.jsonl` / `span-b.jsonl` (gitignored) plus every
    `agent-latest.jsonl*` under the live Pulse data dir's `logs/`.
  - Its inter-drive quiet window was 180 s (`sleep 180` in the operator pass): 120 s idle + a 30 s resolver tick +
    margin. That makes it a new member of the SUT-side quiet-window class.
  - No CI step was added or removed.
- **Cross-project / external claims:**
  - Pulse `a2addb3` source, read at P3 (research.md): `spans` `PRIMARY KEY (trace_id, span_id)` (`buffer/src/schema.rs:36`).
    A replayed identity rejects the whole batch at flush (`appender.rs:996` Pulse test), logged as `duckdb.append`
    `reject_reason=append_failed` plus `buffer.tick` `append_rejections`. Retention defaults to 600 s with a 100 s sweep.
  - Measured LIVE against the `a2addb3` release binaries (sha256 `6476568e…e281` pulse-app, `29f35540…a6da` sidecar,
    re-measured 23:31:36Z) in deterministic posture:
    - two same-seed `exception-event-capture` drives landed **253 507 ms** apart, inside Pulse's own logged
      `retention_window_seconds` 600;
    - `reject_lines=0`, `append_rejections=0`, `spans_after_b=4` (`evidence/witness-ledger.md`).
  - CI: **CI#36942272745**, push, on sha `163e0f67b8b7` (the pre-CI commit): `verdict: green · checks 3/3 · wall 649 s`,
    completed/success, overseer-verified. This wrap's commit adds on top of that sha.
  - Advisory-db (external, a local copy of RustSec's repo): an untracked residue
    `crates/matrix-sdk-crypto/RUSTSEC-0000-0000.md` (a placeholder-id advisory for a crate neither project uses) was
    MOVED by the overseer, an overseer act, into the overseer's scratchpad. It was kept, not deleted. After the move,
    porcelain was empty and HEAD == FETCH_HEAD (`6de44551`).
- **Reverted / negative API facts:** the revert-red control.
  - The re-key call was temporarily replaced (`let _ = (salt, rekey_trace_identity);`). The two-drive test read RED,
    union 13 vs |A|+|B| 26.
  - The call was then restored from a byte copy and read GREEN (`evidence/revert-red.md`). Nothing from the mutated
    state shipped.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **The plan's step 5 (ii) wording.** It said "the same exception fingerprint attribute values in the same order"
     (plan.md Implementation Steps 5). No fingerprint ATTRIBUTE is on the wire: Pulse derives the fingerprint from
     `exception.type` + the normalized `exception.stacktrace` (`exception.rs` `exception_event`, three attributes:
     type/message/stacktrace). The test compares that preimage sequence instead (10 values = 5 events × 2). It is a
     plan wording, and no master states it. Disposition: no master edit owed.
  2. **The plan's timing prediction.** It said "Δ is predicted at roughly 300-400 s" (plan.md Implementation notes,
     worded "a prediction to measure, never restate"). Measured Δ was 253.5 s, because drive A ran 57 s end to end
     (23:32:47→23:33:44Z), not the predicted 2-3 min. It is a plan-only prediction. Disposition: recorded, no master
     edit.
- **Expected amendments (from plan):**
  - **architecture §Cross-cutting Patterns → Determinism discipline + the §Occupied Resources
    `contracts/pulse-run-contract.toml` identity passage — CARRIED.** Fact: the Symbols/APIs bullets above (the
    production path's span identity = f(seed, the per-execution `std::time` salt from `execute_scenario`), applied
    as a content-preserving re-key in `conductor-emit`; the unsalted dispatcher tier stays seed-pure and is pinned by
    the `dispatch_wire__*` goldens; stream SHAPE is unchanged). Sites:
    - `grep -cF "Determinism discipline" .andromeda/architecture.md` → 1 (`:259`);
    - `grep -cF "(trace_id, span_id)" .andromeda/architecture.md` → 1 (`:178`, the pulse-run-contract entry);
    - `grep -cF "Determinism discipline" .andromeda/security-plan.md` → 1 (a citation, for the detector to read);
    - the other five masters → 0.
    The pattern was answered NOT a Boundary widening at P4 by the overseer (playbook `:124`).
  - **test-plan §6 (Real-model interpretation leg) and §2 (operator-local real-wall-clock list) — CARRIED.** Facts:
    - the d2 same-seed replay is repaid by per-execution identity, held by
      `two_same_seed_drives_inside_one_window_share_no_span_identity`;
    - the operator-gated `span_landing_live` witness is registered (Harness bullet), with its 180 s inter-drive quiet
      window as a quiet-window class member.
    Sites:
    - `grep -cF "span identity" .andromeda/test-plan.md` → 1 (`:336`, §6 real-model leg);
    - `grep -cF "quiet window" .andromeda/test-plan.md` → 4;
    - `grep -cF "real_model_live" .andromeda/test-plan.md` → 3;
    - `quiet window` also in architecture 1 and a11y-plan 1 (for the detector to read).
  - **test-plan §8 / §4 — CARRIED.** Fact: the two-drive identity property sits at the dispatcher integration tier
    (loopback stub), and the re-key primitive at the `conductor-emit` unit tier (5 tests). Sites: `grep -cF
    "dispatch_wire" .andromeda/test-plan.md` → 1 (`:400`, §7 Golden artifacts row); §4 `## 4. Unit Test Strategy`
    `:222`, §8 `## 8. Mocking & Stubbing Discipline` `:416`.
- **Coverage of new surfaces:**
  - `rekey_trace_identity` → validation n/a (no external input; any `u64` is a valid salt) · instrumentation n/a by
    design (obs-plan §11 bounded span set; probe `grep -c "tracing::" identity.rs` → 0) · PII n/a (moves ids only) ·
    tests unit (5) + integ (dispatch_wire two-drive) · a11y n/a · tokens n/a
  - `Dispatcher::connect` identity_salt → validation n/a (typed `Option<u64>`) · instrumentation unchanged (`emit.batch`
    on the existing export path) · PII n/a · tests integ (16 dispatch_wire) + live witness · a11y n/a · tokens n/a
  - `span_landing_live` witness (operator-gated reader of untrusted SUT log text) → validation ✓ (`capture_paths` guards:
    `runs_dir_from` → `resolve_under`, `pulse_logs_dir_from` canonicalize + is-dir; bounded `serde_json` per line;
    read errors carry `e.kind()` only) · instrumentation n/a (test binary) · PII ✓ (prints integers only, never a
    Pulse line, path, key or fingerprint) · tests e2e (operator leg, PASS) · a11y n/a · tokens n/a

## Deviations from intent
- **`ExportTraceServiceRequest` re-exported from `conductor-emit`** (plan step 2 listed only `rekey_trace_identity`).
  Justification: step 3's single helper must name the request type, and `conductor-run` has `opentelemetry-proto` only
  as a dev-dependency, while `crates/conductor-run/Cargo.toml` is frozen by the base-diff probe. The re-export names a
  type already in `conductor-emit`'s public builder signatures. It is in a listed file, adds no edge, and the
  dependency count is unchanged.
- **Unit test (d)** compares the two messages with blanked ids by the message's derived `PartialEq`, not by encoded
  bytes. `prost` is not a `conductor-emit` dependency, and adding one is a plan-listed rejection. Field equality
  implies byte-equal encoding.
- **Two-drive test (ii)** compares the fingerprint PREIMAGE sequence (`exception.type` + `exception.stacktrace`).
  Spec claim 1 above gives the reason.
- **The salted twin is a macro arm** (`drive_scenario!(s, salt = N)`), one of the plan's two named options.
- **`emitted_ms as u64`** carries a justifying comment (the global no-unjustified-`as` rule).
- **Witness extras within intent:** it reads every `agent-latest.jsonl*` across files sorted by stamp, and asserts
  exactly ONE run_id and ONE `timeline.execute` start per journal (a path-free FAIL otherwise).
- scope record: none — `gate.py scope` clean, 0 recorded (changed 6 · listed 6 · excluded 37), at implement P4 and
  again at this wrap's P1.

## Decisions & corrections
- **Operator (overseer) rulings this chunk:**
  - "the live witness slot and the 4317 grant are mine: STOP and ask with the length". The ask named ~20 min; the
    grant was "Grant 20 min now".
  - "the Pulse release binaries … are held for you, so re-measure their sha256 right before launch": done at 23:31:36Z,
    both equal.
  - "Answer a hold with the hold option": the ask offered Hold.
- **Overseer act:** moved the advisory-db's untracked placeholder `RUSTSEC-0000-0000.md` (matrix-sdk-crypto) into the
  overseer scratchpad, kept and not deleted. Entry 15 was then re-run green.
- **Pre-CI commit, push and CI read** were performed by the agent ON THE OPERATOR'S EXPLICIT WORD ("the OPERATOR PASS
  on my word"). They are recorded as the operator's acts made on that word, not as a skill bypass.
- **Host contention lesson:** the workspace nextest gate (entry 6) hit its 1800 s bound while `link.exe` linked the
  `conductor-tauri` test binary (exit 143 = the bound's SIGTERM, not a link defect). Another session's `cargo nextest
  -p pulse-app` build was loading the host CPU at the time. A lone `--only 6` re-run was green in 18 s of test time.
  Read a link `exit code: 143` as the bound's kill before suspecting the toolchain.
- **Advisory-db residue shape:** a `RUSTSEC-0000-0000.md` file is an upstream placeholder that later receives a real
  id. Its residue sat beside the assigned `RUSTSEC-2026-0318.md` (same 749 B size). The local `cargo audit` loaded
  1279 advisories with or without it, identical to a fresh clone (`cargo audit --db target/audit-fresh-db-20261002`
  → 1279 / 562 / 7 allowed / exit 0).
- **Sweep hazard (measured):** `grep '"timeline.execute"'` over a Conductor self-obs journal ALSO matches every child
  line carrying `"parent":"timeline.execute"`. Key on `span == "timeline.execute" && span_event == "new"`, which the
  witness does.
- **Hook transports measured this session:**
  - a `cat > file <<` heredoc form is blocked by the PreToolUse guard;
  - a command carrying a doubled backslash is blocked (bash-guard);
  - both were handled with Write-tool scratchpad files run by path.

## Outcome
Acceptance criteria re-asserted against the diff:
- (tests/arch) `rekey_trace_identity` is a per-salt bijection that preserves linkage and moves no content byte.
  **MET**: 5 unit tests; `cargo nextest run -p conductor-emit --profile ci` and `cargo test -p conductor-emit` are green.
- (tests) Two same-seed drives with distinct salts share no `(trace_id, span_id)` pair. **MET**: union 26 = 13+13,
  equal counts, equal preimage sequence, linkage intact, same-salt reproducible. RED with the re-key removed
  (`evidence/revert-red.md`).
- (tests/arch) The dispatcher tier stays seed-pure. **MET**: `the_same_seed_reproduces_the_same_stream` and the three
  goldens are green, and the snapshots diff vs `2c97d3b` prints nothing.
- (arch/security) The change stays in its seam. **MET**:
  - no emit→core edge (0);
  - package count 562;
  - audit and deny green;
  - base-diff probe empty (no builder, `client.rs`, Cargo, scenario, contract, script or workflow change).
  The new `ExportTraceServiceRequest` re-export is a public NAME for an existing builder return type, not a new verb,
  selector, scenario, contract, workflow or dependency, so the criterion's list is not contradicted.
- (obs) The primitive raises no span, attribute or log line. **MET**: the `tracing::` count is 0, and no identity
  value reaches a self-obs line (no new `tracing` call in any touched file).
- (tests/live) Two drives without `[BLOCKED]`, witness PASS, `delta_ms` < retention. **MET**: `[MANUAL]
  exception-event-capture` ×2; `span-landing: PASS run_ids=2 delta_ms=253507 retention_ms=600000 reject_lines=0
  append_rejections=0 spans_after_b=4`.
- (tests) Workspace nextest green with zero retries, doc tests, workspace clippy and fmt. **MET** (entry 6 green on its
  re-run: 1153/1153).
- (security) Evidence carries no host path and hygiene is clean. **MET**: the evidence probe count is 0 at exit 1, and
  `hygiene: clean`.
- (ci) **MET**: CI#36942272745 `verdict: green` on `163e0f6`.
- No matrix capability claimed. **MET**: `matrix.py show --chunk` reads claimed 0, pool unclaimed 0; `v3-09` untouched.

Gates (implement P2 run in `.andromeda/runs/2026-10-01T21-38-30-implement`, plus targeted re-runs; outcome basis
below):
- `cargo nextest run -p conductor-emit --profile ci` — green, exit 0
- `cargo test -p conductor-emit` — green, exit 0
- `cargo nextest run -p conductor-run --test dispatch_wire --profile ci` — green, 16 passed
- `cargo test -p conductor-run --test dispatch_wire` — green
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings` — green
- `cargo nextest run --workspace --profile ci` — **timeout** at 1800 s on the first run. The survivor was the
  `conductor-tauri` test-binary link (exit 143), killed by the bound, under host contention from another session's
  Pulse build. `--only 6` re-run: green, `1153 tests run: 1153 passed, 0 skipped`.
- `cargo test --workspace --doc` — green
- `cargo clippy --workspace --all-targets -- -D warnings` — green
- `cargo fmt --all --check` — green
- `git diff --numstat 2c97d3b -- …/snapshots/` — green, no output
- `git diff --numstat 2c97d3b -- client.rs exception.rs span_tree.rs Cargo.* crates/… scripts/ scenarios/
  contracts/ .github/` — green, no output
- `grep -c "conductor-core" crates/conductor-emit/Cargo.toml` — green, exit 1, 0
- `grep -c "tracing::" crates/conductor-emit/src/identity.rs` — green, exit 1, 0
- `grep -c "^name = " Cargo.lock` — green, 562
- `git -C "$CARGO_HOME/advisory-db" status --porcelain` — **red · no output ✗** on first run (the residue file). After
  the overseer's move, the re-run was **green**, no output, HEAD == FETCH_HEAD.
- `cargo audit` — green, exit 0: 1279 advisories · 562 crates · 7 allowed. Equal to a fresh-clone reading, and the
  copy is verified current after the move.
- `cargo deny check advisories bans licenses sources` — green: advisories ok, bans ok, licenses ok, sources ok
- Operator-leg entries, the census, sha256, preconditions ×2, drives A/B, freezes ×2, `sleep 180`, the witness,
  `status <id>`, hygiene, the push and the CI read: fired once in the operator pass, all atoms holding. Treatment
  recorded in `evidence/witness-ledger.md`.
- evidence host-path probe (`ls …witness-ledger.md && cat …/evidence/* | grep -cE …`) — red (exit 2, file absent)
  before the operator pass, then **green** (0, exit 1) after it.

Watches: none folded.

Outcome basis: the operator pass ran.
- Setup 4's commit list is `163e0f6` (the pre-CI commit, parent `2c97d3b`).
- The final-HEAD CI run is CI#36942272745, green 3/3 on `163e0f67b8b7`.
- Implement's P4 report is the basis for the gate figures. The overseer directive after implement (the advisory-db
  move, then the operator pass on the operator's word) changed entry 15 from red to green.
- Post-implement artifacts: `evidence/witness-ledger.md` (its operator entries are completed in this wrap's commit, per
  directive).

Process hygiene, re-measured at this wrap (2026-10-01T23:5xZ):
- `pulse-app` PID 52128: started by this chunk on the operator's word, stopped at 23:38:42Z by PID + CreationDate;
  4317/4318 listeners 0; census empty.
- `conductor` drives, the witness and the gate, build and audit tasks: exited.
- The Pulse `cargo nextest -p pulse-app` tree was another session's and was left to it.
