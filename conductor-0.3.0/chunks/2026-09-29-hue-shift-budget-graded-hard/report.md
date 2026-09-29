# Report — 2026-09-29-hue-shift-budget-graded-hard

**Chunk:** `halo-hue-encoding` was re-driven live against Pulse's P-025 observable, and `duration_ms ≤ 2000` is now
graded hard at the harvest tier.
**Date:** 2026-09-29
**Commits:** `9da18e1 chore(2026-09-29-hue-shift-budget-graded-hard): operator pre-CI commit, for the run this
chunk's verdict reads` (since `last_wrap` 2026-09-29T19:26:57Z; base `cdb7082`, the pre-CI commit's parent).

## Changes (structured — detectors read this)
- **Files:** 5 modified and 5 new (basis: `git diff --name-only cdb7082` plus `git status --short`; `gate.py
  scope` at this wrap reads `clean — changed 5 · listed 5 · recorded 0`).
  - Modified:
    - `contracts/pulse-p025-measurement-contract.md`;
    - `crates/conductor-run/tests/delegated_timing_harvest.rs`;
    - `scenarios/halo-hue-encoding.toml` (comments only);
    - `scripts/agent-run.sh` and `scripts/agent-run.ps1` (the leg H comment only).
  - New, all under `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/`:
    - `h.jsonl` (784 lines of frozen Conductor self-obs, one `run_id`);
    - `pulse-hue-lines.jsonl` (9 closed-field Pulse lines);
    - `hue-verdict.md`;
    - `operator-pass.md`.
  - The chunk also writes this `report.md`.
- **Symbols / APIs:** test-private helpers in the one integration-test binary, with no production symbol. All
  of them live in `delegated_timing_harvest.rs`, and every caller is in that file (code-graph basis at research:
  `hue_samples_in_window` has 4 callers, all in-file).
  - `grade_in_window(lines, bound, window)`: the existing worst-observation `grade()` run over only the
    in-window lines. Its `Err` names P-025 and the window.
  - `tiered_hue_samples_in_window(...)`: the in-window samples whose `severity_tier` is not `none`.
  - `incident_anchor_error_ms(sample, lines)`: `|(at_ms − duration_ms) − t|`, where `t` is the nearest
    `interpretation.incident.created` line at or before the sample with `created=true`. It returns `None`
    when no such line precedes the sample.
  - Consts `INCIDENT_CREATED_TARGET` and `P025_ANCHOR_TOLERANCE_MS = 1000.0`. The tolerance is Pulse's
    `xtask/src/hue_shift.rs:41` `ANCHOR_TOLERANCE_MS` at `226554a`.
  - 6 new tests (`grep -c '#\[test\]'` over the file: 17 → 23):
    - 5 synthetic arms (i)–(v): `p025_in_window_samples_within_budget_grade_ok_with_the_worst`,
      `p025_an_in_window_sample_over_budget_is_a_hard_err_carrying_its_value`,
      `p025_an_empty_leg_window_is_ungraded_never_met`,
      `p025_a_stale_sixty_second_sample_before_phase_two_is_excluded` and
      `p025_the_anchor_is_the_nearest_preceding_fresh_incident_not_a_dedupe`;
    - the live pin `p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value`, over
      `leg_2026_09_29_lines()` (9 verbatim lines).
  - The retired P-025 tests' ASSERTIONS are unchanged. Only their doc comments now name them as the record of the
    RETIRED instrument at Pulse `83d4060`; the module doc now states all four bounds grade hard.
- **Crates / modules:** none changed (the `git diff --numstat cdb7082 -- Cargo.toml Cargo.lock crates/
  ':!…delegated_timing_harvest.rs'` probe is green, empty).
- **Dependencies:** none. Zero delta; `Cargo.lock` is byte-unchanged at 562 packages. `cargo audit` read 1277
  advisories · 562 crates · 7 allowed warnings, exit 0; `cargo deny check advisories bans licenses sources` read
  all ok. The advisory-db copy is current (porcelain clean, `HEAD` = `FETCH_HEAD` =
  `f23b768236fe2880e4cfa167da662cad8ca79240`).
- **Schema / config:** no config key, env handle, envelope key, `Verdict`/`ReportState` variant, span or
  `ALLOWLISTED_FIELDS` entry. `halo-hue-encoding.toml` keeps zero `[[expected]]` blocks and byte-identical
  phase data, seed, tier and checklist (`git diff -U0 cdb7082 -- scenarios/halo-hue-encoding.toml | grep -cE
  '^[-+][^-+#]'` → 0).
- **Spec-master edits:** none by implement. The seven masters are this wrap's to amend (below).
- **Contract document (not a master):** `contracts/pulse-p025-measurement-contract.md` was brought to the
  measured state:
  - It is re-pinned: `pinned_at = "Pulse HEAD 226554a"` and `captured_at = "2026-09-29"`. Provenance is now MIXED
    and stated per clause: Pulse coordinates are transcribed SUT records read by `git show` at `226554a`, and
    the readings are Conductor measurements, 2026-08-21 against `f0c38f5` and 2026-09-07 against `83d4060`.
  - Six dated corrections (a)–(f) sit beside the claims they correct.
  - The retired instrument is recorded in past tense, with the raw `36 704.983642578125` kept. The six
    section headings are kept verbatim, because `v3-07`'s verified ref re-derives from an end-anchored
    six-heading count; they gain a retirement bracket instead.
  - It adds **§The grading rule (stated before the drive)**, whose sha256 `9da09cc1…6d61` was recorded pre-leg.
  - One post-drive dated correction was added at this wrap on the operator's word. It is add-only: `git diff
    --numstat 9da18e1` → 7/0, and the committed pre-correction file still hashes to `9da09cc1…`.
  - 284 → 291 lines.
- **Counts / qualifiers moved:**
  - Delegated-timing bounds graded hard at a real value: **3 → 4**. P-025 moves from "unmeasurable, mechanism
    pin, no pass arm" to "graded hard, PASS, worst 684.98 ms".
    - Sites: `obs-plan.md:350` (`grep -n 'UNMEASURABLE' .andromeda/obs-plan.md` → 1 hit, that line) and
      `architecture.md:181` ("what Pulse would have to emit … to become measurable at all").
    - The module doc in `delegated_timing_harvest.rs` is updated in-chunk.
  - Harvest test count: 17 → 23 (the gate logs: nextest `23 tests run: 23 passed`, `cargo test` `23 passed`).
  - Workspace nextest: 1116 → 1117 (gate logs before and after step 7).
  - The contract's P-025 write count ("eight writes") corrected to 11 non-test writes, 5 on the incident chain.
- **Dev-tool versions:** none — no host tool was installed or upgraded in Conductor. SUT-side: Pulse's
  `andromeda-pulse-mcp.exe` was rebuilt (mtime 2026-09-01 → 2026-09-29T23:05:46 local) from Pulse checkout
  `4502d5d`. It is the SUT's sidecar, not a Conductor tool.
- **Harness / gate surface:**
  - The leg H comment in `scripts/agent-run.sh:109-114` and `scripts/agent-run.ps1:175-179` changed from "the
    hue observable fires against a tick-refreshed last_seen" to "the canvas witnesses that tier change,
    emitting its hue sample". No verb, arm, `live_leg_order` or budget changed.
  - No CI step changed.
- **Cross-project / external claims:**
  - **SUT:** Pulse `andromeda-pulse` checkout HEAD `4502d5dcaf0d0859b6e6a12c540221b7d7e4e483` on branch
    `chore/migrate-pulse-to-v3`. It is one commit past the plan's `226554a`: `git -C <pulse> diff --stat
    226554a HEAD` shows 9 files, none of them in `pulse-app/src`, the MCP crate or `Cargo.lock`.
    - `merge-base --is-ancestor e98d838 HEAD` → exit 0.
    - `grep -ac tier_effective_at_unix_nano pulse-app.exe` → 2.
    - Every contract coordinate was read with `git show 226554a:<path>`, never the dirty worktree.
  - **SUT fact measured live:** under deterministic L4, with the widget visible, the rise sample was 684.98 ms
    and a fall sample was 430.79 ms. The rise anchored to `interpretation.incident.created` at 29.98 ms.
  - **CI:** CI#36632527433 on `9da18e122bd3` read `verdict: green · checks 3/3 · wall 584 s` (`ci.py
    conclusion --sha HEAD --wait 1200`). This wrap's commit then adds to that tree.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. The contract's six claims at `83d4060` wording, each re-read at `226554a` (research.md table, 28 rows):
     - the fall source `transitioned_at_unix_nano`, which is really `resolved_at_unix_nano`
       (`tier_effective.rs:30-35`; CARRY premise 1);
     - acknowledgement lowering the tier, which it does not (`services_router.rs:104`, `tier_effective.rs:167-171`;
       CARRY premise 2);
     - `ServiceListItem` carrying no tier timestamp;
     - "that site holds both timestamps", which was true for the rise only;
     - "eight writes" (11 / 5);
     - the redaction-precedent sentence, which is history.
     All six were corrected in the contract in-chunk. The fall-source and acknowledgement halves are ALSO stated
     at `obs-plan.md:350` ("`transitioned_at_unix_nano` when it falls"; `grep -n 'transitioned_at_unix_nano'
     .andromeda/obs-plan.md` → 1 hit, :350). DISPOSITION: an obs-plan amendment at this wrap.
  2. "P-025's ≤2s bound is UNMEASURABLE through `metric.constellation.hue_update_ms` … P-025 therefore grades
     as a MECHANISM PIN with no pass arm" (`obs-plan.md:350`). It is TRUE of `83d4060` and superseded at `e98d838`:
     the leaf now carries paint − `tier_effective_at`, and the leg graded it hard (PASS, worst 684.976318359375
     ms). DISPOSITION: an obs-plan amendment at this wrap, keeping the `83d4060` measurement as history.
  3. "what Pulse would have to emit for its delegated ≤2 000 ms hue-shift budget to become measurable at all …
     a Conductor measurement, not a transcribed SUT record … as measured at HEAD `83d4060` on 2026-09-13"
     (`architecture.md:181`). The ask was satisfied at `e98d838`; the document now also states the grading rule,
     and its provenance is per clause at `226554a`. DISPOSITION: an architecture amendment at this wrap.
  4. The grading rule's forecast that "its end-of-storm fall lands while the dot is hidden". It was stated in the
     contract §The grading rule, plan.md step 1, research.md §Open questions and scope.md.
     - Measured: the incident auto-resolved at the storm's end (tick 1790716377928, `resolved_count: 2`, 1 068 ms
       after phase 2's nominal end at 1790716376860) while the dot was live. The fall landed in-window at 430.79 ms.
     - DISPOSITION: a dated correction beside it in the contract (operator directive, applied at this wrap,
       add-only). The scenario comment was corrected at implement. plan/research/scope are immutable records.
       The grade was not affected: the rule grades every in-window sample.
  5. plan.md step 1's provenance pairing "2026-08-21 / 2026-09-07 … against `83d4060` / `f0c38f5`" is inverted
     against the harvest file's own record (2026-08-21 at `f0c38f5`; 2026-09-07 at `83d4060`). DISPOSITION: the
     contract follows the file; the plan is an immutable record.
- **Expected amendments (from plan):**
  - obs-plan §4 Known-residual classification path → Delegated-timing family (P-025 statement, SCOPE-clause fall
    source, mechanism pin named as the retired record) — **carried**: Counts bullet (3 → 4) and Spec claims
    disproved 1 and 2. Search: `grep -n 'UNMEASURABLE' .andromeda/obs-plan.md` → 1 hit (:350);
    `grep -n 'transitioned_at_unix_nano' .andromeda/obs-plan.md` → 1 hit (:350). No other master states the claim:
    the seven-master sweep `unmeasurable|quantiz|transitioned_at_unix_nano|hue_update_ms|pulse-p025-measurement-contract|halo-hue|P-025`
    (per-line `grep -noiE`) hits architecture on 6 lines (:70 and :85 a prior-leg citation and the
    two-scenario count, :135 the family roster, :181 the contract row, :182 the posture contract "taking the
    P-025 regime above unchanged", :242 the directory tree — all read; only :181 states the contract's ask) ·
    test-plan 2 lines (:155 leg order, :467 a measurement citation — no P-025 claim) · obs-plan 1 line (:350) ·
    layout-templates 1 line (:64 the `P-025 — operator-checklist` wireframe row, no timing claim) · security /
    design / a11y 0.
  - architecture §Occupied Resources → the `contracts/pulse-p025-measurement-contract.md` row — **carried**:
    Spec claims disproved 3. Search: `grep -n 'pulse-p025-measurement-contract' .andromeda/architecture.md` → 1
    hit as a row (:181; :242 names the member in the `contracts/` directory tree, count unchanged, no claim).
  - The `v3-08` ledger note rides the P5 claim, not the wrap — **not a wrap item**: written by phase P5's
    `matrix.py claim --note-file`. The capability's `ref` + `implemented` were written at implement; P7.3 flips it.
- **Coverage of new surfaces:**
  - `grade_in_window` / `incident_anchor_error_ms` (test-private harvest helpers) → validation n/a (reads
    committed literals, no path handle) · instrumentation n/a (test-only) · PII n/a (closed-field lines) · tests
    unit (5 synthetic arms + 1 live pin; inverse-control mutants redden exactly their arms) · a11y n/a · tokens n/a.
  - `evidence/pulse-hue-lines.jsonl` + `evidence/h.jsonl` (committed SUT/self-obs lines) → validation n/a · PII
    closed-field only (hue leaf `duration_ms`/`severity_tier`, incident-created `created`/`deduped`/`severity`/
    `priority_tier`, tick counters; no corpus text, fingerprint or host path) · `gate.py hygiene` clean · tests
    n/a.

## Deviations from intent
- **Pulse HEAD `4502d5d` instead of `226554a`.** The plan permits "any HEAD carrying `e98d838`"; the diff since
  `226554a` touches no binary source. Recorded in `evidence/hue-verdict.md`.
- **The dictated sidecar build command was corrected.** The overseer's `cargo build --release -p
  andromeda-pulse-mcp` does not resolve (the package is `mcp-server`, the bin feature-gated); run as `cargo build
  --release -p mcp-server --bin andromeda-pulse-mcp --features mcp-server` (verification-harness.md 2026-08-10).
  Pulse's `triage` build script fetched a tokenizer over the network — Pulse's build, not Conductor's.
- **`pulse-app` launched by the agent** on the overseer's word (the plan allowed "the operator's, or the session's
  on the operator's word"), on the fresh letters-only data dir `%TEMP%/pulse-legs/huegradedhard`; stopped by the
  agent after the capture.
- **The contract's six headings kept verbatim** (plan step 1 said they "become the record of the RETIRED
  instrument"): the verified `v3-07` ref is an end-anchored six-heading count; a retirement bracket carries the
  change.
- **The contract's provenance pairing follows the harvest file**, not plan step 1's inverted pairing (Spec claims
  disproved 5).
- **All nine sliced Pulse lines pinned**, not only the in-window hue lines and their anchor: the canary's
  out-of-window rise then proves the exclusion on real data.
- **The operator pass was fired by the session** (entries 26–28 + the pre-CI commit + push), on the overseer's
  direction.
- **Post-drive contract correction** added at this wrap on the operator's word (Spec claims disproved 4).
- scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- Overseer rulings (founder-delegated) carried from P4: either verdict meets `v3-08`; the leg is rise-only with
  phases unchanged, the fall witnessed by Pulse's `smoke:hue-shift` at `e98d838`.
- Overseer, the slot: stop at the live leg and wait for the word; "no port use" before it; use the EXISTING
  `pulse-app.exe`; rebuild the stale sidecar; launch on a letters-only data-dir suffix ("no digit run the scrubber
  could read as a card"); stop what you started.
- Overseer, the wrap: a DATED correction beside the forecast, the pre-leg sha256 staying valid for the rule text
  as it stood; the P-025 grade (worst 684.98 ms, fall 430.79 ms, Pulse `4502d5d` carrying `e98d838`) is the
  evidence Pulse's Conductor-return entry cites for its P-075. No route change asked.
- **A dictated build command is a claim to check** (CLAUDE.md 2026-08-09 as extended 2026-08-15): the package
  name was one `git show HEAD:crates/mcp-server/Cargo.toml` away.
- **A frozen-rule document can still be corrected without breaking its pre-drive hash**: append-only dated
  brackets, verified by `git diff --numstat` (N/0) and the committed file's hash.
- Sweep hazards: the gate tool's `--skip` reason list is comma-split, so a reason carrying a comma is truncated
  at it (seen in the first gate run's printed reasons); the bash-guard hook blocks any command carrying a doubled
  backslash, so a drive-path hygiene probe needs the backslash class dropped or a file.
- The `(assert!(CONST >= x))` shape was avoided in the arm (v) test: clippy `assertions_on_constants` — the anchor
  assertion compares the COMPUTED error to the tolerance instead.

## Outcome
Acceptance criteria, re-asserted against the diff:
- **`v3-08`** — MET. Driven live once against a `pulse-app` proven by content (2 hits) from a checkout carrying
  `e98d838`. `grade_in_window` grades every hue sample in `[1790716226860, 1790716380720]` (phase-2 start
  1790716196860 + 30000; `scenario.run` close) by the worst-observation `grade()`: **PASS, worst
  684.976318359375 ms ≤ 2000**; breach → `Err` naming the value (arm ii); absence UNGRADED (arm iii); grade over
  verbatim lines pinned from the committed capture; rule stated before the leg with its sha256 recorded pre-leg;
  the one rise anchored at 29.98 ms ≤ 1000. The in-window fall (430.79 ms) is graded too. Proven by
  `delegated_timing_harvest::tests::p025_the_graded_leg_meets_its_two_second_budget_at_its_real_value`.
- Synthetic arms (i)–(v) green under both runners — MET (inverse control: window filter off → arms iii/iv red;
  created filter off → arm v red; file restored byte-identical, sha `4ce3418b…`).
- Retired P-025 tests keep their assertions; doc comments name the `83d4060` record; `36704.983642578125` stays
  pinned — MET.
- `git diff --numstat cdb7082 -- Cargo.toml Cargo.lock crates/ ':!…harvest.rs'` empty — MET (no variant, envelope
  key, span, allowlist entry, env reader, dependency or production edit in the diff).
- `halo-hue-encoding.toml` zero `[[expected]]`, byte-identical phase data — MET (both probes read 0); the run
  rendered the closed lamp label `[RESIDUAL] halo-hue-encoding` with no per-check line.
- Contract re-pinned with per-clause provenance, six corrections, grading rule predating the drive — MET.
- Stale-claim probe `last line 0` — MET.
- `evidence/` passes `gate.py hygiene` — MET (`clean`, entry 26). Every committed Pulse line is closed-field.
- `cargo audit` + `cargo deny` exit 0 after a clean porcelain read — MET.
- Workspace nextest, doctests, clippy, fmt green; CI `verdict: green` on the pushed sha — MET (CI#36632527433 on
  `9da18e1`).

Gates (implement's second full run, after step 7 — the final tree save the post-drive contract bracket, which
this wrap's light gate re-covers):
- `cargo nextest run -p conductor-run --profile ci --test delegated_timing_harvest` — green (23/23).
- `cargo test -p conductor-run --test delegated_timing_harvest` — green (23 passed).
- `cargo nextest run --workspace --profile ci` — green (1117/1117).
- `cargo test --workspace --doc` — green.
- `cargo clippy --workspace --all-targets -- -D warnings` — green.
- `cargo fmt --all --check` — green.
- `git diff --numstat cdb7082 -- Cargo.toml Cargo.lock crates/ …` — green (exit 0, no output).
- `git diff -U0 cdb7082 -- scenarios/halo-hue-encoding.toml | grep -cE …` — green (exit 1, last line 0).
- `grep -c '^\[\[expected\]\]' scenarios/halo-hue-encoding.toml` — green (exit 1, 0).
- `test -f … && cat … | grep -ciE 'unmeasurable|tick-refreshed'` — green (exit 1, 0).
- `grep -cE '^ *pinned_at *= *"Pulse HEAD 226554a' …` — green (1).
- `grep -c 'tier_effective_at_unix_nano' …` — green.
- `grep -c 'The grading rule' …` — green.
- `cargo deny check advisories bans licenses sources` — green.
- `git -C "$CARGO_HOME/advisory-db" status --porcelain` — green.
- `cargo audit` — green.
- `grep -ac tier_effective_at_unix_nano …/pulse-app.exe` — leg operator, driven by hand: 2, exit 0 (evidence/hue-verdict.md).
- `git -C … merge-base --is-ancestor e98d838 HEAD` — leg operator, by hand: exit 0.
- `tasklist … (census before)` — not run — skipped: driven by hand in the slot (exit 1, none; + parentage census none; 4317/4318 no listener).
- `… conductor -- preconditions` — not run — skipped: driven by hand in the slot (exit 0, `[PRECONDITION] every live-Pulse precondition is satisfied`).
- `wc -l < …agent-latest.jsonl.*` — leg operator, by hand: 34573.
- `… conductor -- run halo-hue-encoding --agent-mode` — leg live, driven once: exit 0, lacks `[BLOCKED]`, contains `halo-hue-encoding` (`[RESIDUAL] halo-hue-encoding`).
- `cp logs/agent-latest.jsonl …/evidence/h.jsonl && grep -o '"run_id"…'` — leg operator, by hand: exit 0, `"run_id":"2026-09-29T21-09-10-754"`; artifact fresh.
- `bash scripts/agent-run.sh status <id>` — leg operator (the smoke, mint-then-read) with `2026-09-29T21-09-10-754`: exit 0, `"scenario": "halo-hue-encoding"`, lacks `Blocked` (`KnownResidual`, latency_ms 183860).
- `tasklist … (census after)` — not run — skipped: driven by hand after the leg (only the agent's `pulse-app` tree; no conductor/sidecar survivor).
- `gate.py hygiene` — leg operator: exit 0, `hygiene: clean` (evidence/operator-pass.md).
- `git diff --quiet && … git push origin HEAD …` — leg operator: exit 0, `PUSHED_SHA=9da18e1…`.
- `ci.py conclusion --sha HEAD --wait 1200` — leg operator: exit 0, `verdict: green` · 3/3 · 584 s · CI#36632527433.

Watches: none folded.

Outcome basis: the operator pass ran — `9da18e1` (Setup 4's commit list) and CI#36632527433 on it, recorded in
`evidence/operator-pass.md`; implement's conversation (this session) for everything else; one post-pass artifact:
the contract's 7-line dated correction (operator directive at this wrap), re-covered by this wrap's light gate.

Process hygiene (implement P4 census, re-measured at this wrap's start: census none matched, 4317/4318 no
listener):

| process | started by | final state |
|---|---|---|
| `pulse-app` 22800 + `conhost` 46240 + 9 `msedgewebview2` (root 29704) | this run (agent, overseer's word) | terminated (PID + CreationDate matched; `CloseMainWindow` then `Stop-Process -Force`; pass 1 none alive) |
| `conductor.exe` / `andromeda-pulse-mcp.exe` | the leg | exited with the leg — absent from the post-leg census |
| `cargo` (the Pulse sidecar build) | this run | exited (exit 0) |
| `tail` 36352 | another project's session (17:14Z) | left running — not this session's to stop |
