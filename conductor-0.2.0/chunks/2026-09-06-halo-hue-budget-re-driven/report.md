# Report — 2026-09-06-halo-hue-budget-re-driven

**Chunk:** Halo hue budget re-driven — an error stream sustained through incident formation so the severity
tier flips while the service is still emitting (P-025)
**Date:** 2026-09-07
**Commits:** none since `last_wrap` beyond the prior chunk's `3d69573
feat(2026-09-06-coverage-completeness-gate)` — this chunk's work is uncommitted at report time.

## Changes (structured — detectors read this)

- **Files:** `scenarios/halo-hue-encoding.toml` · `scenarios/auto-resolve-idle-window.toml` ·
  `crates/conductor-run/tests/delegated_timing_harvest.rs` ·
  `crates/conductor-run/tests/operator_pause_harvest.rs` · `scripts/agent-run.sh` ·
  `scripts/agent-run.ps1` · new `conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/`
  (4 leg captures + `hue-verdict.md`). Basis: `git status --short`.
- **Symbols / APIs:** no production symbol added, changed or removed — **every source edit is inside
  `crates/conductor-run/tests/`** (basis: the modify-set above; no `src/` path in it). Test-binary-local
  helpers added to `delegated_timing_harvest.rs`: `LIFECYCLE_TICK_TARGET`, `TICK_OFFSET_TOLERANCE_MS`,
  `timestamp_ms`, `days_from_civil`, `HueSample`, `hue_samples_in_window`, `tick_times_ms`,
  `tick_offset_ms`. Existing `bounds()` / `observations()` / `grade()` / `is_target()` / `num_field()`
  unchanged — `grade()`'s worst-wins fold KEEPS its three other callers (P-027 / P-037 / P-045 tests),
  which is why the re-driven bound got a separate selection path rather than a changed `grade()`.
- **Crates / modules:** none added, removed or changed. No `Cargo.toml` touched.
- **Dependencies:** none. `Cargo.lock` untouched (basis: `git status --short Cargo.lock Cargo.toml
  crates/*/Cargo.toml` → empty).
- **Schema / config:** `scenarios/halo-hue-encoding.toml` — both phases gain a `[phases.emission]` table
  (phase 1 `kind="plain"`, `occurrences=60`, `gap_ms` 3000→30000; phase 2 `kind="exception"`,
  `occurrences=300`, `variants=["identical"]`, `gap_ms` 3000→150000); `slo_tier` `"<5s"`→`"<90s"`;
  `[[checklist]].induced` text replaced. Scenario stays DECLARE-ONLY (no `[[expected]]` added), so
  `Scenario::check_checklist` is never tripped and the scenario emits zero `CheckRecord` rows.
  `scenarios/auto-resolve-idle-window.toml` — header comment only; `gap_ms = 200000` unchanged.
- **Spec-master edits:** none at report time (P2 owns them).
- **Counts / qualifiers moved:**
  - `halo-hue-encoding.slo_tier` `<5s` → `<90s` (the only declared-value move). Docs stating a tier for
    this scenario: **0** — basis `grep -rc 'halo-hue-encoding' .andromeda/*.md` → architecture 2,
    obs-plan 1, test-plan/a11y-plan/layout-templates/design-system/security-plan 0; neither architecture
    hit states a tier (`:84` is the `[[checklist]]` clause, `:134` the declare-only family list).
  - `--live` leg count 4 → 5 (H added ahead of B1). Docs stating the composition: **1** — test-plan `:155`
    (basis: `grep -rn 'quiet window'` over test-plan/architecture/obs-plan; the tighter `B1 → B2` pattern
    returned 0 because the prose reads "leg B1 → leg B2").
- **Dev-tool versions:** none.
- **Harness / gate surface:** `scripts/agent-run.{sh,ps1}` `live_suite()` gains `live_leg h
  halo-hue-encoding` / `Invoke-LiveLeg 'h'` ordered FIRST, at budget `live_leg_budget_sec 180`; the
  `live_leg_order` comment and the `--live` composition comment updated in both; a boot-wide
  bootstrap-posture banner added to both. No new command, verb, port, or `CONDUCTOR_*` handle — the
  5-command discipline holds. Both shells parse clean (`bash -n` exit 0; PowerShell
  `Parser::ParseFile` 0 errors) and the leg order is identical in each.
- **Cross-project / external claims:** ground truth in `andromeda-pulse` @ HEAD `83d4060`, read read-only.
  - **The fire site's three terms** (`pulse-app/ui/src/widget/ConstellationCanvas.tsx:117-155`): slowest
    changed dot wins; `visibleDots` drops a service quiet > `LIVE_RECENCY_WINDOW_NANOS = 60 s`
    (`constellation-types.ts:27`); and `last_seen_unix_nano` has **no ingest-path writer** — basis
    `grep -rn "last_seen_unix_nano\s*=[^=]" crates/ pulse-app/src` → 4 hits, of which `registry.rs:257`
    (restart transition) and `registry.rs:336` (tick, gated on `current_quiet_duration_seconds == 0`,
    integer-truncated at `activity_floor.rs:142-149`) are the writers; the other two
    (`buffer/src/drain.rs:492`, `corpus/src/contract.rs:546`) are unrelated structs.
    `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL = 15 s` (`lifecycle/mod.rs:52`), wired unchanged at
    `main.rs:1489`, no env override.
  - **The hue leaf carries no service identifier** — allowlist is exactly `["duration_ms",
    "severity_tier"]` with the SUT's own comment "Aggregate-only: no service identifier"
    (`pulse-app/src/observability.rs:986-990`).
  - **A dot's tier is incident-derived** ("max priority tier across active service-scoped incidents",
    `crates/triage/src/lifecycle/registry.rs:59-65`) and incidents form only from Autonomous cues
    (`crates/triage/src/cadence/coordinator.rs:395`).
  - **The bootstrap knob is reached at boot** — `Thresholds::from_env()` at `pulse-app/src/main.rs:472`,
    flowing to `bootstrap_state` via `baseline/mod.rs:736`, `:473`.
  - **`hypothesis:` — span-idle semantics of Pulse's auto-resolve.** On this leg the `conductor` storm
    incident auto-resolved at the 07:46:38.154Z tick — 120 s after its LAST cue re-touch (07:44:17.890Z,
    `created:false`) — while the storm itself ran until 07:46:31.7Z. So the 120 s idle clock appears to
    run from the last cue re-touch, not from the last span. **No master states span-idle semantics**
    (basis: `grep -E '120 s idle|idle window|goes idle|idle for'` over architecture / test-plan /
    obs-plan → 0 relevant hits), so this is NOT a disproof of anything stated here; it rides as a Pulse
    intake item only.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** the 2/s dispatch rate. It is correct and shipped
  — it makes every 15 s tick refresh `last_seen` by construction, which the mechanism pin depends on —
  but it does **not** make the ≤2 s bound attainable, and was never expected to. What remains is
  SUT-side (see the disproof below); owner: Pulse intake.
- **Spec claims disproved by measurement:**
  1. **The plan's own step-1 premise** (authored, caught at the P5 review before implement): "a sub-2 s
     dispatch interval leaves `last_seen` ≤ ~1000 ms at the tier flip". FALSE — `last_seen` is written by
     the 15 s tick, never by ingest (writer sweep above). Already corrected in `plan.md` §Goal and in
     `scope.md` DoD 2 (`[premise-corrected]`) before implement ran; no master stated it, so nothing is
     owed to a spec here. Evidence: the leg measured 14 525.9 ms on a continuously-emitting service.
  2. **P-025's ≤2 s delegated bound is UNMEASURABLE through `metric.constellation.hue_update_ms`.**
     Stated as a live budget in `obs-plan.md:349` (§4 Known-residual → Delegated-timing family, the sole
     hit for `hue_update_ms` across the masters; test-plan has 0). The entry currently records the
     over-budget reading with a STALENESS cause; the measured cause is TICK QUANTIZATION, which is a
     different and stronger claim (staleness alone would predict ~0 for an emitting service; the leg
     measured 14.5 s). Evidence: `evidence/hue-verdict.md` + the harvest pin. **Owed to obs-plan §4.**
- **Expected amendments (from plan):**
  1. `obs-plan §4` Delegated-timing family — P-025's recorded CAUSE gains the quantization →
     **carried**; fact in *Spec claims disproved* #2. Search: `grep -c 'hue_update_ms'` → obs-plan **1**
     (`:349`), test-plan **0**. Owner obs-plan.
  2. `test-plan §3` + `architecture §Occupied Resources — On-disk artifacts` — the `--live` composition
     gains leg H → **carried for test-plan only**; fact in *Harness / gate surface* and *Counts moved*.
     Search: `grep -rn 'quiet window'` → test-plan **1** (`:155`, "leg B1 → leg B2 → a 150 s quiet
     window → leg A"), architecture **0**, obs-plan **0**. Architecture's `run --live` hits (`:169`,
     `:225`) describe the capture directory, not the leg order — **not an owner**; the plan naming it
     was a precaution, and the grep does not support it.
  3. `architecture §Standard Contracts — Run report envelope` — check the surrounding prose for a
     "declares no emission of its own" claim → **not carried: no such claim exists.** Search:
     `grep -rc 'declares no emission of its own'` → architecture **0**, test-plan **0**, obs-plan **0**.
     Both architecture sites naming this scenario remain TRUE and unchanged: `:134` lists it among the
     eight declare-only families with "zero `[[expected]]` checks each" (still true — it stayed
     declare-only), and `:84` states "exactly two committed scenarios declare `[[checklist]]`:
     `halo-hue-encoding` and `halo-breathing-encoding`" (still true). Fact stated here anyway.
  4. `matrix#v2-20 notes` — a dated note recording this chunk's re-drive outcome for P-025 →
     **ledger-note — owner P7.3**.
  5. `test-plan §9` — the auto-resolve leg's not-run-stable record gains the boot-wide posture as the
     resolution now used → **carried**; fact in *Harness / gate surface* + Outcome. Search:
     `grep -c 'not run-stable\|not-run-stable'` → test-plan **1**. Owner test-plan.
  6. **Added by the wrap directive (not in the plan's list):** the bootstrap witness is named by its
     emitting FUNCTION (`warn_bootstrap_window`) where the persisted line's TARGET is
     `triage.baseline.bootstrap_window.override`. Basis: `grep -rn 'warn_bootstrap_window'` → 5 sites
     (plan ×2, `auto-resolve-idle-window.toml:42`, `agent-run.sh:146`, `agent-run.ps1:170`); the target
     string appeared in **1** file (`evidence/hue-verdict.md`). The three code/config sites were
     corrected at this wrap so a light-gate grep hits; `plan.md` is immutable to wrap and keeps the
     function name as the record of what was planned. The **test-plan §9 amendment must carry the TARGET
     string** — routed to P2.
- **Coverage of new surfaces:**
  - `scenarios/halo-hue-encoding.toml` (re-shaped declarative config) → validation **garde✓** (loads via
    `Scenario::from_toml_str`; `conductor-core` catalog tests + `check_load_envelope` green) ·
    instrumentation **n/a** (no new span; the phases ride the existing `timeline.execute` /
    `emit.batch`) · PII **n/a** · tests **unit✓** (`conductor-core` catalog + load-envelope;
    `conductor-run` harvest + operator-pause) · a11y **n/a** · tokens **n/a**
  - `--live` leg `h` (harness surface) → validation **n/a** · instrumentation **✓** (the leg freezes its
    self-obs to `runs/live-suite/h.jsonl`, 790 lines) · PII **redacted✓** (committed captures scan clean
    for host paths — 0 hits for the word-anchored drive-letter form, `%APPDATA%`, `/Users/`, `/home/`,
    `.cargo`, `.rustup`, `pulse-legs`, `AppData`) · tests **e2e✓** (the live suite itself; exit 0) ·
    a11y **✓** (the driven arm ran, 1 passing on WebView2 152.0.4191.66) · tokens **n/a**

## Deviations from intent

1. **`crates/conductor-run/tests/operator_pause_harvest.rs` joined the modify-set — operator-approved
   scope widening.** The plan's Files-to-modify omitted it; it pins the checklist `induced` text, a
   property step 3 deliberately inverts. **Cause: a research miss** (already typed
   `input.research-files-wrong` in the friction ledger). The companion class is *"a test whose INPUT is
   the changed artifact"* (`committed("halo-hue-encoding")`) — **not only goldens**, which is all my P3
   sweep enumerated. The pipeline rule already exists (codebase-research §Files to modify names data
   pins); the missing piece was the mechanical form: `grep -rln "<scenario-name>" crates/**/tests`
   before the touchpoint list is final. Implement soft-exited rather than widening scope itself; the
   operator approved, and only the pin + its comment changed.
2. **Plan step 6 said "replace" the P-025 test; it was renamed and a second added.** The 2026-08-21
   verbatim lines are genuine evidence of what the *un-driven* shape measured, and step 6 itself said to
   carry the mechanism paragraph forward. Both ship:
   `p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass` (origin) and
   `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency` (live pin).
3. **The shipped pin is scoped to the `k=0` case.** It asserts against the *preceding* tick, correct here
   because the re-driven scenario guarantees continuous emission. A quiet service freezes `last_seen`
   k ticks back — all 7 samples fit `duration = offset + k × 15 000`, k ∈ {0,1,2}, residual ≤ 2 ms.
   Recorded in `evidence/hue-verdict.md` so the helper is not reused blind on a silent scenario.
4. **The witness-coordinate correction** (Expected amendments #6) was made at wrap, not implement.
5. **Four amendments authored at this wrap's P2 were NARROWED at its P7, before commit.** As first
   applied, `architecture` §69 / `test-plan` §9 / `verification-harness.md` item (e) / the
   `auto-resolve-idle-window.toml` header said the stretched boot-wide posture makes the
   `AutoResolved` arm "reachable independent of uptime" and called it the RESOLUTION. The light
   gate's literal re-run falsified the sufficiency half: same tree, same posture, same process, and
   leg A graded `ManualCheck`. All four now state that the posture removes cause (a) only, that cause
   (b) is untouched and still decides the leg, and that the arm is REACHABLE rather than RELIABLE —
   with both runs' incident timings as the evidence. The two affected sidecar entries were corrected
   in the same pass so they claim only what the bodies show. **This is why the light gate re-runs the
   live leg literally rather than re-reading the session's artifacts: a second run is what turned a
   plausible sufficiency claim into a measured insufficiency one.**

## Decisions & corrections

- **Operator P5 review, finding #1 — the deliverable's meaning changed before implement.** The ≤2 s bound
  is unmeasurable through this leaf; the chunk ships a mechanism pin + the measured value, and **no pass
  arm**. Two candidate attribution designs died to measurement and are recorded as rejected in the plan:
  by service identity (no such field on the leaf) and by `severity_tier` (a Suggested-band scenario forms
  no incident, so it would emit no sample at all).
- **Operator P5 round 2** — 1/s → 2/s, because step 1 named the jitter beat and step 6's pin is what that
  beat breaks (~1–5 % of runs). The rate change removes the cause; widening the pin to accept `k × 15 s`
  was rejected as the weaker letter (it would stop distinguishing a refreshed tick from a skipped one).
- **Operator forks at P4**: stretch the bootstrap window for leg A (boot-wide) rather than holding the
  default hour; and the hue leg joins `--live` ordered first rather than standing alone.
- **A nextest filter that matched nothing read as success**: `-E 'test(delegated_timing)'` → `0 tests run,
  195 skipped` at exit 4. `binary(delegated_timing_harvest)` is the correct selector.
- **The bootstrap witness is grepped by TARGET, not by emitting function name.**

## Outcome

**Acceptance criteria, re-asserted against the diff:**

- (tests) hue assertion pins the quantization mechanism, no arm under 2 000 ms — **MET**. The live pin
  asserts `|duration − offset| ≤ 1100` (measured 1.1 ms) and `duration_ms > budget_ms`; no `< 2000`
  assertion exists in the diff.
- (tests) window opens at phase-2 start, closes at read-back, from `runs/live-suite/{label}.jsonl`;
  empty window ⇒ `Err` — **MET** (`hue_samples_in_window` + `an_empty_window_yields_no_samples...`).
- (tests) `cargo nextest run -p conductor-run` 0 · `cargo test -p conductor-run` 0 — **MET**.
- (tests) live legs only via `agent-run.{sh,ps1} run --live`; no CI gate — **MET** (the leg is inside
  `live_suite()`, reachable from no default path).
- (obs) budget read from `metric.constellation.hue_update_ms` / `duration_ms` at the harvest tier, never
  via `budget_ms` — **MET** (`bounds()[0]` unchanged; no `budget_ms` in the new path).
- (obs) no new span name, no `info` per-record line — **MET** (no `src/` change at all).
- (arch) both envelope terms pass unaided, `[[exempt]]` empty — **MET**
  (`an_in_envelope_catalog_with_an_empty_ledger_passes` green over the re-shaped catalog).
- (arch) `slo_tier` in the closed set, scenario stays declare-only, zero `CheckRecord` rows — **MET**
  (`<90s`; no `[[expected]]` in the diff).
- (security) TOML loads clean; committed evidence has zero host paths — **MET** (scan above; 0
  `<redacted>` markers too, so the captures are clean **by construction**, not by the scrubber firing).
- (layouts) an in-envelope leg prints no `[ENVIRONMENT-SUSPECT]` — **MET** (absent from the suite log).
- (a11y) the graded `verdict`/`state` pair resolves to one lamp under the verdict-first crosswalk —
  **MET** (`verdict: null` / `ManualCheck` → Manual; a pair the crosswalk already names).
- (design) declared `slo_tier` renders from the closed set, no per-scenario literal at the render site —
  **MET** (no render-site change in the diff).

**Gates green** — `cargo nextest run --workspace --profile ci --no-fail-fast` **901/901**, exit 0 ·
`cargo test -p conductor-run` exit 0 · `cargo clippy --workspace --all-targets -- -D warnings` exit 0 ·
`cargo audit` exit 0 (18 deny.toml-adjudicated warnings) · `cargo deny check advisories bans licenses
sources` exit 0 · advisory-db checkout porcelain-clean before the audit result was read · zero dependency
delta · `bash scripts/agent-run.sh status` exit 0.

**Smoke** — boot-path changed (both shells in the modify-set): both parse clean, leg order identical.
`status` exits 0 but is proof-free by design (it printed the prior chunk's `run_id`), so it stands as
shell-parse evidence only.

**Live suite (run 1, 07:42Z)** — `run --live` exit 0, all four legs at their predicted states: H
`ManualCheck` (183 768 ms, `<90s`) · B1 `ManualCheck` · B2 `Blocked` (the deduped gate) ·
**A `KnownResidual`** — the `ReadBack::AutoResolved` arm that did NOT reproduce on 2026-09-06
(CARRY 3.3 discharged). Driven a11y arm 1 passing.

**Live suite (run 2 — the light gate's literal re-run, 09:06Z)** — exit 0; H `ManualCheck` ·
B1 `ManualCheck` · B2 `Blocked` · **A `ManualCheck`** · a11y arm 1 passing. **Leg A did not
reproduce, and that is a finding this chunk owns rather than a flake.** The stretched posture is
provably still working in both runs (`silence_cues_emitted: 0`, `services_in_bootstrap: 2`; the only
cues near either window are `error_rate_spike`/`suggested`), so cause (a) is genuinely removed even
at ~1 h 28 m uptime. What separated the runs is cause (b), which the posture does not touch: the
window's only `created:true` incident landed at **+45 s** in run 1 (≈155 s old at read-back, cleared
the 120 s idle + 30 s tick → arm fired) and at **+137 s** in run 2 (≈62 s old → arm missed).
**Consequence: the posture makes the arm REACHABLE, not RELIABLE, and the leg stays not run-stable
under it.** Four amendments authored earlier in this same wrap claimed sufficiency and were NARROWED
to this before commit (see Deviations 5).

**The measurement** — leg H drove 360 dispatches at 2/s (`emission_count: 360`), so the subject never
stopped emitting; its one in-window sample read **14 525.947 ms against a 2 000 ms budget**, matching its
offset to the preceding lifecycle tick (14 527 ms) to **1.1 ms**. Tick spacing n=76, median 15 000 ms.

**Outcome basis** — implement's P4 report **plus** post-implement artifacts: the live suite of
2026-09-07 (`runs/live-suite/{h,b1,b2,a}.jsonl`, the per-run journals, and
`{LEG_DIR}/logs/agent-latest.jsonl.2026-09-07`), the operator-approved scope widening, and this wrap's
witness-coordinate correction. A detector must not read implement's earlier `surfaced` verdict as
current — the final verdict is green.

**Process hygiene** — re-measured at this wrap from the host process list (the friction ledger carries
**no smoke checkpoint for the live pass** — its last implement record is the 08:26:51Z fix-loop `green`
— so this census is stated from artifacts and a live re-measurement, not from a checkpoint that was
never written):

| Process | Started by | Final state |
|---|---|---|
| `pulse-app` (PID 10112) | the operator, for this run | **left running** — operator stops it after this commit |
| `msedgewebview2` ×15 | children of `pulse-app` | left running with their parent |
| `conductor-tauri` · `msedgedriver` · `tauri-driver` · `node` | the driven a11y arm | **terminated** (wdio `onComplete` → `tauriDriver.kill()`) |
| `andromeda-pulse-mcp` | Conductor's sidecar spawn, per leg | **terminated** |
| `cargo` / shell probes | this run | **terminated** |
