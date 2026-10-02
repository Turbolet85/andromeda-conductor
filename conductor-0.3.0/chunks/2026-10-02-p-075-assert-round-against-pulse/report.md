# Report — 2026-10-02-p-075-assert-round-against-pulse

**Chunk:** P-075 assert round against Pulse S 03ec944 — six assertions graded hard, deterministic
**Date:** 2026-10-02
**Commits:** `2a49480 chore(2026-10-02-p-075-assert-round-against-pulse): operator pre-CI commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-02T00:16Z; parent `e1092ce`)

## Changes (structured — detectors read this)
- **Files** (basis: `git diff --stat e1092ce` + untracked):
  - NEW `crates/conductor-run/tests/p075_round_live.rs`;
  - NEW `crates/conductor-run/tests/evidence_pin/mod.rs`;
  - `crates/conductor-run/tests/lifecycle_harvest.rs`;
  - `crates/conductor-run/tests/delegated_timing_harvest.rs`;
  - `crates/conductor-run/tests/span_landing_live.rs`;
  - `contracts/pulse-p025-measurement-contract.md`;
  - the chunk folder: scope / research / plan / scope-record / report;
  - `evidence/`: `p075-leg.txt` · `h.jsonl` · `pulse-{h,d,r,f}.jsonl` · `round-ledger.md` · `operator-pass.md`.
- **Symbols / APIs:** none in any `src/` (`git diff --numstat e1092ce -- crates/*/src` prints nothing). These are
  test-binary symbols only.
  - **`p075_round_live` (NEW, `live-pulse`-gated test binary):**
    - It reads `ANDROMEDA_PULSE_DATA_DIR` from its own env and passes it to `ReadbackClient::connect`, the shipped
      hardened spawn: fixed sidecar NAME, `.env(...)`, metacharacter rejection. This is the `lifecycle_live.rs`
      precedent.
    - It joins NO filesystem path on that value, so it is NOT a `capture_paths` reader. It reads no
      `CONDUCTOR_RUNS_DIR`.
    - It calls `query_incident_list`, `retrieve_telemetry_slice`, `retrieve_report` and, through the shipped
      `probe_resolve_lifecycle`, `mark_incident_resolved`. Those are the four registered tools; no new one.
    - It emits a `canary_spec` storm on `CANARY_SERVICE_NAME` and compares `conductor_emit::fingerprint` IN-PROCESS.
    - Stdout carries integers, booleans and closed words only; raw wire values go to stderr only.
  - **`evidence_pin` (NEW `tests/` subdirectory module)** holds `sha256_hex` / `check_digest` / `committed` /
    `pinned`, transcribed from `real_model_harvest.rs:1530-1567`, which is unchanged.
  - **`span_landing_live`:**
    - NEW `span_dir()` → `{runs}/span-landing`;
    - NEW `pair_is_current()` stale-pair refusal (drive A ≥ the live Pulse log's first line, drive B ≤ its last,
      A < B), called before grading;
    - the input path moved from `runs/live-suite/span-{a,b}.jsonl` to `runs/span-landing/span-{a,b}.jsonl`;
    - the firing-form doc now clears the two NAMED files with a non-recursive `rm -f` before drive A;
    - the reader is still resolved through `capture_paths::runs_dir_from`.
  - No port, socket, IPC method, endpoint, `CONDUCTOR_*` handle or env var was added.
- **Crates / modules:** none added or removed. The test modules above join `conductor-run`'s `tests/`.
- **Dependencies:** none added or bumped. `grep -c "^name = " Cargo.lock` reads 562, unchanged. `sha2` and
  `serde_json` were already `conductor-run` dev-deps.
- **Schema / config:** none. No scenario, manifest, run-contract or capability-manifest change (the
  `git diff --numstat e1092ce …` probe printed nothing). `contracts/pulse-p025-measurement-contract.md` (no Rust
  reader) gained three add-only `[at S 03ec944]` coordinate notes plus a re-read header bracket:
  - the hue emit `telemetry.rs:278-282` → `:316-320`;
  - the allowlist `observability.rs:989-992` → `:992-995`;
  - `ServiceListItem` `registry.rs:54-69` → `:56-71`, field `:70`.

  §The grading rule is byte-unchanged.
- **Spec-master edits:** none (wrap P2 owns them).
- **Counts / qualifiers moved:**
  - The `live-pulse`-gated test-target SET went from 4 to 5 (`p075_round_live` added). Basis:
    `grep -l 'cfg(feature = "live-pulse")' crates/conductor-run/tests/*.rs` at `e1092ce` and at the working tree.
  - The `CONDUCTOR_RUNS_DIR` test-binary reader count is unchanged at four (the same `span_landing_live`, new subdir).
  - Harvest test counts:
    - `lifecycle_harvest` 11 → 26;
    - `delegated_timing_harvest` 23 → 31 (P5 baselines vs the post-round gate logs);
    - workspace nextest 1176 tests run, 1176 passed.
- **Dev-tool versions:** none. cargo-audit was re-read (1279 advisories · 562 crates · 7 allowed warnings, exit 0).
- **Harness / gate surface:** none.
  - `scripts/agent-run.{sh,ps1}` are byte-unchanged. No verb, selector or CI step.
  - A side effect: the `--live` suite's `rm -f "$LIVE_CAPTURE_DIR"/*.jsonl` no longer reaches the span pair, which
    has moved out of `live-suite/`.
- **Cross-project / external claims** (Pulse repo, checkout HEAD = S `03ec94481b0d6c3ba574626e7acb39e33fd40141`,
  source read by an Explore agent with four load-bearing claims re-read by hand; research.md §Pulse at S):
  - `fingerprint_refs` = `incident.evidence_refs.fingerprint_hashes` (`crates/mcp-server/src/tools.rs:439`), written
    ONLY at creation by `grounded_fingerprint_hashes` (`pulse-app/src/inference_runtime.rs:657-666`): the `det-*`
    refs, then the cue's full 32-hex fingerprint. A dedupe never updates it (`:830-858`).
  - `degraded_mode = parsed_l4.is_none()` (`tools.rs:377-381`). Since Pulse `9d14166`, deterministic L4 output is
    scrubbed per leaf (`inference_runtime.rs:675-688`) and stays parseable, so it reads `false`.
  - `compute_exception_fingerprint` (`crates/buffer/src/fingerprint.rs:79-96`) is unchanged since `83d4060`
    (`git log 83d4060..03ec944` → 0 commits). It is Conductor's `fingerprint()` derivation.
  - `metric.report.render_ms` fires from the in-app `get_report`, driven by `ReportWindow.tsx:23,32`
    (`effectiveId = clickedId ?? findings.rows[0]?.id`, `isOpen = effectiveId !== null`). The hidden-at-boot Report
    webview fires on each new incident with no click, and a Findings-row click (`FindingsWindow.tsx:189-191,222`)
    PINS the selection.
  - `app.exit` was new at S. On Windows only the `event_loop` class writes it; `Stop-Process -Force` writes none.
  - CI: **CI#36970919487** on `2a494804f6d91bb61718fc9a520cebc73b29af86`, `verdict: green`, 3/3 checks, 689 s
    (`evidence/operator-pass.md`). The verdict was taken on `2a49480`; this wrap's commit adds to that tree.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. **test-plan.md:335:** "`retrieve_report` is permanently `degraded_mode` in this mode", meaning deterministic L4.
     MEASURED false at Pulse S:
     - `degraded_mode=false` for incident 1 (`evidence/p075-leg.txt`);
     - leg R's `metric.report.render_ms` carries `degraded_mode: false` (`evidence/pulse-r.jsonl`);
     - source: per-leaf scrub since `9d14166`.

     The clause was already flagged stale by tests-history (2026-09-22-interpretation-proven-live).
     `grep -cF permanently .andromeda/test-plan.md` = 2: `:335` is this claim, and `:284` is the unrelated "DECLINED
     arm stub-ONLY and permanently so".
  2. **The chunk's own P3 premise "P-037 needs Pulse's Report window OPEN"** (research.md Q4, scope.md, plan v1) —
     disproved at P5 by source (`ReportWindow.tsx:23,32`) and confirmed live: leg R's render sample fired with NO
     desktop input. This is not a spec-master claim. It was corrected in scope / research / plan on the overseer's
     ratification.
- **Expected amendments (from plan)** — each with its site search (`grep -cF {token}` per master):
  - **arch §Occupied Resources** `runs/live-suite/{leg}.jsonl` entry + tree `live-suite/` line → **carried**
    (Symbols, `span_landing_live`; Harness side effect). Sites: `live-suite/` arch 3 · security 1 · test-plan 1;
    `span-{a,b}` arch 2 · security 1; `span-a` arch 1 and obs 1, where the obs hit is "span-attribute", a false
    positive.
  - **security-plan §Input Validation** span-landing ingest row + reader roster → **carried** (Symbols:
    `span_landing_live` path and stale refusal; `p075_round_live` is a hardened-spawn MCP caller, not a
    `capture_paths` reader). Sites: `span_landing_live` security 6 · test-plan 4; `live-pulse` security 5.
  - **test-plan** §2/§9/§11 span-landing path · §5 `mark_incident_resolved` re-graded at S · §6 harvest graders +
    the `:335` stale clause · §9 `p075_round_live` into the gated set → **carried** (Counts; Spec claims disproved 1).
    Sites: `span_landing_live` 4 · `lifecycle_live` 2 · `live-pulse` 1 · `permanently` 2 (`:335` the subject).
  - **obs-plan §4 Fingerprint-storm** "no shipped check computes an emitted-vs-read-back match" → **carried**: a
    test-tier check (`lifecycle_harvest::p075_round_assertion_1…`) now computes membership, still with no span
    attribute. Site: `grep -cF 'no span attribute or shipped check computes'` obs 1. Also §4 Delegated-timing family:
    re-graded at S (Cross-project).
  - **arch §Established Decisions [Read-Back Dependency Posture]** — the grounded-union payload fidelity and
    `degraded_mode: false` as measured at S → **carried** (Cross-project; Outcome).
  - **Pulse-facing** (the report, not an amendment): the six graded test ids + the evidence path at `2a49480` →
    stated in Outcome.
- **Coverage of new surfaces:**
  - `p075_round_live` (gated live leg) → validation n/a (no external input beyond the data-dir handle, validated by
    the shipped spawn) · instrumentation n/a (a test binary; the `mark_incident_resolved` write inherits
    `verify.readback.call_tool` + `mcp_tool`) · PII redacted✓ (stdout integers/booleans only;
    `grep -cE "[0-9a-f]{16,}" p075-leg.txt` = 0) · tests e2e (live, operator-gated) + harvest unit · a11y n/a ·
    tokens n/a.
  - `span_landing_live` stale-pair guard → validation✓ (`pair_is_current`) · instrumentation n/a · PII n/a (integers
    only) · tests unit (3 hermetic arms) · a11y n/a · tokens n/a.
  - The round graders → tests unit (digest-pinned, tamper arms) · the rest n/a.

## Deviations from intent
1. **Steps 4–5's live pins were written AFTER the round (step 9)**, because their evidence did not exist before
   it. The grading machinery and synthetic arms were written first. The plan's step order put the pins ahead of the
   round that produces their inputs.
2. **The contract sha256 was read after the round, not before leg H.** Its last write (05:01:15Z) predates the
   launch (05:23:13Z), so it is the rule the round graded by (`round-ledger.md`).
3. **The evidence dir was created** (`mkdir -p`) before the P-075 leg's entry, whose stdout redirect needs it.
4. **Legs H/D/R/F were run through a scratch sequencing script** built from the entries' own command text.
   - Each exit was read from the bare command.
   - The probe's output was filtered to its `[PRECONDITION]` line, with the exit taken from the pipeline's first
     stage.
5. **The P-037 premise was corrected at P5, after approval.**
   - The plan's "founder opens the Report window" became "nobody touches the desktop". A row click would PIN the
     selection and suppress samples.
   - The overseer ratified the correction as a P4-form fork. The corrected step-7 text is what ran.
- scope record (`gate.py scope`: `clean — changed 6 · listed 5 · recorded 1`): in-intent ×1 —
  `crates/conductor-run/tests/evidence_pin/mod.rs` · serves step 3 · self.

## Decisions & corrections
- **The hands-off correction for P-037, recorded as it happened.**
  - At P3 I read `useReport(isOpen ? id : null)` as "render only while the window is visible" and planned a founder
    click.
  - Sourcing the exact click path after approval showed that `isOpen` is `effectiveId !== null`. The hidden Report
    webview selects `findings.rows[0]` by itself, and a click pins it.
  - I stopped and asked. The overseer ratified hands-off ("nobody touches the desktop … an empty R slice records
    UNGRADED, no re-drive").
  - The round confirmed it: leg R's render sample fired with no input.
  - **A sweep hazard to curate:** a predicate name like `isOpen` is a claim about the code, not about UI
    visibility. Read its definition before inferring a precondition from it.
- **The founder pause, recorded as it happened.** The founder was told hands-off before the launch and stayed off
  the desktop from "pulse-app is up" (05:23Z) to "round complete" (05:42Z). No desktop input occurred, and the
  compact widget stayed visible as booted.
- **The overseer's slot decisions:** all four legs plus the P-075 leg, for per-scenario attribution (P4); the CARRY
  proven hermetically (P4); no re-drive for a pass; P-037 UNGRADED if empty.
- **The operator pass** — hygiene, the pre-CI commit, the clean-tree-guarded push and the CI read — was made by the
  agent on the overseer's explicit word (`evidence/operator-pass.md`), not as a skill bypass.
- **Route unchanged** (directive). Next: "Version close on measured evidence", which waits on the founder's
  `v3-09` word.
- **Measured contrast worth keeping:** at S, deterministic incident formation took 46 ms after the storm, against
  ~110 s on the 2026-09-01 leg at `83d4060` (`lifecycle_live.rs` doc). Formation latency is SUT-HEAD-dependent, so a
  poll budget sized for one HEAD can be very generous for another.
- **Sweep hazard:** `grep -cF span-a` over obs-plan matches "span-attribute". It is a token-proxy false positive.

## Outcome
**Acceptance criteria — every one met**, re-asserted against the diff:
- **Six grades at the harvest tier.** All six are `[PASS]` and none is UNGRADED:
  1. fingerprint ∈ refs (4 refs / 3 `det-*`) · opened +46 ms · `degraded_mode: false`;
  2. `ProvenByLiveness`, idle 12 ms;
  3. P-025 worst 478.56 ms in-window, the rise anchored 38.24 ms;
  4. P-027 worst 605.26 ms;
  5. P-037 0 ms;
  6. P-045 worst 5.0 ms of 163.

  Held by `lifecycle_harvest::p075_round_assertion_{1,2}_*` and
  `delegated_timing_harvest::tests::p075_round_assertion_{3..6}_*`. They pass locally and in CI#36970919487.
- **Assertion 1's UNGRADED precondition** (`active_at_open > 0`) and its absent arms are covered by synthetic tests.
  The measured capture has `active_at_open=0`.
- **Assertion 2** used `probe_resolve_lifecycle` + `attribute_by_liveness` unchanged (`src/` untouched).
- **Assertions 3–6** are graded from each leaf's own field. P-025 uses the contract window derived from `h.jsonl`
  (1790918907230..=1790919061263); the derivation is proven on the 2026-09-29 `h.jsonl` known positive.
- **Evidence:**
  - every graded file is sha256-pinned, with a tamper arm per harvest and no capture text in test source;
  - the fingerprint-hex probe reads 0 and the host-path probe reads 0;
  - `gate.py hygiene` is clean.
- **The CARRY:** the three arms report `test result: ok. 3 passed`. `capture_paths_guard` is green in the workspace
  nextest, and the scripts are unchanged.
- **The diff probe** prints nothing: no `src/`, script, scenario, manifest, verb, selector or workflow move.
- **The ledger** carries S, both sha256 values, the posture, the hands-off instruction, both censuses, the teardown
  and the six test ids + evidence path.

**For Pulse's `ref`:**
- the graded test ids:
  - `crates/conductor-run/tests/lifecycle_harvest.rs::p075_round_assertion_1_read_back_content_fidelity`;
  - `…::p075_round_assertion_2_runtime_state_fidelity`;
  - `crates/conductor-run/tests/delegated_timing_harvest.rs::tests::p075_round_assertion_3_p025_hue_update`;
  - `…_4_p027_discovery`;
  - `…_5_p037_report_render`;
  - `…_6_p045_counter_refresh`;
- the evidence: `conductor-0.3.0/chunks/2026-10-02-p-075-assert-round-against-pulse/evidence/` at `2a49480`.

**Gates** (the plan fence, by `run`):
- `cargo nextest run -p conductor-run --test lifecycle_harvest --profile ci`: green (26 passed).
- `cargo nextest run -p conductor-run --test delegated_timing_harvest --profile ci`: green (31).
- `cargo test -p conductor-run`: green.
- `cargo clippy -p conductor-run --features live-pulse --all-targets -- -D warnings`: green, after one fix
  (`manual_contains` in the new leg).
- `cargo test … --test span_landing_live -- --skip both_same_seed_drives_land_inside_one_retention_window`: green
  (`3 passed`).
- `cargo nextest run --workspace --profile ci`: green (1176).
- `cargo test --workspace --doc`: green.
- `cargo clippy --workspace --all-targets -- -D warnings`: green.
- `cargo fmt --all --check`: green.
- The `git diff --numstat e1092ce …` probe: green, no output.
- `grep -c "^name = " Cargo.lock`: green, 562.
- The advisory-db porcelain: green.
- `cargo audit`: green.
- `cargo deny check advisories bans licenses sources`: green.
- The host-path evidence grep: green, `last line 0`.
- The `[0-9a-f]{16,}` grep: green, `last line 0`.

**Leg and operator entries** (driven by hand, recorded in `evidence/round-ledger.md` and `operator-pass.md`):
- census before and after: recorded, both empty;
- sha256 re-measure: exit 0, both atoms held;
- `conductor preconditions` ×5: exit 0, `[PRECONDITION] every live-Pulse precondition is satisfied`;
- pre-leg count ×5 + post: recorded;
- the P-075 leg: exit 0, end-check `last line 1`;
- `sleep 150` ×4;
- legs H/D/R/F: `lacks [BLOCKED]` + scenario name held, with `[RESIDUAL]` / `[MANUAL]` ×3;
- the `h.jsonl` freeze;
- status smoke on `2026-10-02T05-40-50-313`: exit 0, `"scenario": "findings-counter-refresh"`;
- `gate.py hygiene`: clean;
- the guarded push: `PUSHED_SHA=2a494804…`;
- `ci.py conclusion --wait 1200`: `verdict: green`, CI#36970919487.

**Smoke:** recorded, not re-run. It was the status mint-then-read inside the round; no boot path changed.

**Watches:** none folded.

**Outcome basis:** the operator pass ran.
- Final HEAD `2a49480`'s CI#36970919487 is green (`evidence/operator-pass.md`).
- Implement's P4 report is the basis for its gate verdicts and deviations, carried in this conversation.
- A post-implement artifact: `evidence/operator-pass.md`, uncommitted, riding this wrap.

**Process hygiene** (implement P4's census, re-read here):

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` PID 56832 + conhost 33976 + msedgewebview2 18908 | implement, on the overseer's slot | terminated (PID + CreationDate) |
| sidecars / `conductor` / `cargo` / test binaries | the legs | terminated (after-census empty) |

Re-measured at wrap time (2026-10-02T12:55:51Z): 0 `pulse-app` / `andromeda-pulse-mcp` / `conductor` processes,
0 LISTENING sockets on `:4317`/`:4318`.
