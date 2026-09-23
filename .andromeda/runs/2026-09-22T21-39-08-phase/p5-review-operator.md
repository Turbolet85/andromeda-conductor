# P5 review — operator, 2026-09-23 (verbatim record)

Given on the plan as it stood at 555 lines / 43 594 B, measured by the operator at Conductor `74a0872` + the chunk's
working files and Pulse `83d4060`. The orchestrator re-verified B1's three SUT facts before folding (Pulse
`deterministic_inference.rs:86-89` truthy set `1|true|yes`; `:48` the canned statement; `main.rs:520-530`
`interpretation.model.load` `inference_mode`). Everything below is the operator's text.

---

The research under this plan is strong: P-A…P-H hold on re-derivation, and the posture's grading rule is the right
thing to keep. What follows is a read-only review in four disjoint sweeps (aim · mechanics · semantics · adversarial),
measured at Conductor 74a0872 + the chunk's working files and Pulse 83d4060. Every coordinate below was read;
re-verify before acting, as always. Ordered by weight, so if the window runs short the first two blocks are the ones
that must land.

OPERATOR DECISIONS (P5 review, 2026-09-23)
1. v3-09 is met on Identified only. NotIdentified records its CalibrationRegion/ManualCheck row and v3-09 goes to
   deferred (un-claim at wrap, operator defer, the measurement in notes), never re-driven. A model-side Blocked goes the
   same way; a new attempt is a new chunk with a changed design, never a re-drive of this leg.
2. Record the leg's limit as its stated property. The cue line reaches the model verbatim: assembler.rs:664-673 renders
   "[autonomous] retry_storm — retry_storm scope_id=conductor", prompt.rs:229-235 inserts the digest whole, and
   assembler.rs:261-276 carries one cue per digest. That cue string itself passes identifies_cause. So Identified means
   the real, non-canned model carried the cue's scope and kind into rank 1, n=1 — not inference of an unstated cause,
   not a choice among competing causes. Say that in the scenario header, the v3-09 notes and the posture correction;
   the Goal's "interpretation named the cause" wording goes.
3. The one-sided Pass needs an architecture amendment, not "no arch-body record is owed": architecture.md:71 says
   hypothesis-quality claims "are calibration-region", and conductor-verify verdict.rs classify() routes a
   CalibrationRegion claim to CalibrationRegion "regardless of matched". Add an Expected amendment to §Established
   Decisions [Probabilistic-Assertion Policy]: a match against a rule stated before the drive may Pass, a miss is
   CalibrationRegion, never Fail.

BLOCKERS
B1. A canned-L4 Pulse can meet the criterion. The canned rank-1 statement "Deterministic fixture hypothesis: the retry
    storm originates in the synthetic verification scope." (deterministic_inference.rs:48) has no whole-word
    "conductor", so it grades NotIdentified — which the (matrix) criterion accepts as written. Truthiness also differs
    across the repo boundary: flag_declared is true|1 (core preconditions.rs:48-53), Pulse's
    deterministic_mode_enabled_for is 1|true|yes (deterministic_inference.rs:86-89), so "=yes" passes the real-model
    gate over a canned Pulse. The step-14 det- test does fire on canned output (evidence_refs at :68-72, rendered one
    bullet each at markdown.rs:194-203), but the criterion does not cite it, no synthetic arm is a canned capture, and a
    red det- has no route. A direct witness exists and is unread: Pulse logs target interpretation.model.load with
    field inference_mode at boot (main.rs:519-532). Worth: reading that witness in the capture (whole file, like the
    bootstrap override), making "real model ran" (witness + no det- ref) part of the verifying test, a canned-capture
    synthetic arm, a canned run classed as an operator-launch fault, and Pulse's truthy set counted as declared by the
    absence check.
B2. Model outcomes and environment faults cross over, in both directions. A canary the model dismisses returns before
    any incident (inference_runtime.rs:756-761); the preflight poll then ends EmptyCorpus and names
    WORKSPACE_KEY_PRECONDITION (preflight.rs:130, :304-318), which the code itself calls "indistinguishable on the wire"
    (:125-129). Drive accounting re-fires "a protocol/tool/workspace precondition" as an environment fault, so it would
    re-drive a model outcome — the posture's "never re-driven until it passes". The other way, launch faults read as
    model outcomes: a data-dir mismatch (Conductor's own sidecar writes a log into the wrong dir, so the capture finds
    one), model/CUDA/tokenizer faults (interpretation.inference.error, …inference.skipped), a reused data dir (the
    canary deduped=true). Worth: a classification table in Drive accounting keyed on the Pulse-log witnesses of the leg
    window (L4 activity, parse decision, incident.created, inference.error, the 15 s heartbeat), so each Blocked is
    assigned before anything is re-fired.
B3. Attribution by open time plus uniqueness admits the wrong incident, or none. opened_at_unix_nano is stamped when
    the L4 loop receives the digest, before inference (inference_runtime.rs:153-155 -> :875), so a second canary
    identity or a reflection digest (1800 s cadence, cadence/coordinator.rs:240, config.rs:25; any surfaced non-summary
    digest becomes an incident, inference_runtime.rs:243-265) can open after the emission instant and be graded as the
    scenario's; list items carry no kind or scope (tools.rs:348-354). And the scenario's incident can resolve before
    the first poll: query_incident_list is active-only (tools.rs:336-340) and a damper-suppressed re-emission never
    bumps updated_at (digest/damper.rs:1-8), so "attribution: none" would be recorded as "never formed". Worth:
    attributing by the cue's fingerprint (the canary gate's own precedent, preflight.rs:202-205; the canary's
    fingerprint is logged at canary.rs:219), reading by id without the active filter (retrieve_report by id,
    tools.rs:477-487), recording the creating digest's prompt_version and pulse-app's uptime, stating ms vs ns, and
    asserting formation >= 0.
B4. A Blocked pin flips v3-09 mechanically: grade(pinned) == measured is green whatever the outcome, implemented is
    "ref set, test passes" and wrap flips on a green light gate (verification-matrix-contract.md:63, :107-110). With
    decision 1: v3-09's ref is a separate test asserting the pinned grade is Identified and the real-model witnesses
    hold, written only when the drive's outcome is Identified — otherwise no ref, and the coverage gate's halt routes to
    un-claim and defer.
B5. The capture's command is never given, and libtest swallows a passing test's stdout: without "-- --nocapture" (the
    precedent, crates/conductor-run/tests/live_suite.rs:23) rm-capture.txt holds runner lines only and the one drive is
    spent; plan.md has 0 occurrences of nocapture. Say how stderr is handled too (cargo's Running lines, the sidecar's
    inherited stderr).

DEFECTS (measured by the review's sweeps at the coordinates given)
- The rule is not frozen: rule and pin share one uncommitted file and /implement never commits, so nothing shows the
  rule predates the drive. Worth: a sha256 of the rule section printed into rm-capture.txt before the leg and
  re-hashed by a harvest test; and a test that the pinned literal equals the committed capture's sections.
- Step 12 misses falsified posture sentences — mechanism corrections, not re-decisions: §The grading rule "The
  assertion is a substring comparison against the composed observation text" (:103-106, falsified by P-F; the plan
  grades a rank-1 statement from an MCP re-read); §Disposition "a harvest-style test over the leg's verbatim captures"
  (:141-143); the table's "(… or the second generation never landed)" (:115, P-B); and :19-20 "the drive chunk
  confirms rather than inherits".
- Step 14's trace and envelope tests assume the read-back was reached: a preflight-Blocked capture has no
  timeline.execute span and no retrieve_report line, and AutoResolved neither — a planned outcome that would force
  post-drive test edits. Condition them on the route.
- Step 5's mismatch-arm test is vacuous: testkit.rs:19-24 blocked_preflight() is ready:false, and without the mismatch
  check the ready check returns the same Blocked record. ready:true fails it.
- Rule strings: say "the first )** " (labels close at high|medium|low, markdown.rs:161-164); a statement may carry a
  newline (schema.rs:224 bounds length only). Arms for false Identified: "conductor canary" (a space), "not a retry
  storm", "no retry", "non-retryable"; for false NotIdentified: "retried", "conductor-side", "_conductor_".
- The Blocked routes empty-placeholder, attribution none and ambiguous land ManualCheck on the leg's own row
  (non-degraded read-back, execute.rs:280), so they render [MANUAL], not [BLOCKED] — the design criterion is false on
  them, and the posture wants Blocked "lexically and visually distinct".
- Committed text: the envelope is printed verbatim and its fingerprints field is the union of raw model-written
  evidence_refs over every active incident (extract.rs:111-117; free text up to 256, schema.rs:103; never scrubbed),
  so the canary's model text lands raw in evidence/ and the harvest literal. redact_value masks only
  whitespace-separated tokens (redact.rs:79-143), so a backticked ref, "(D:\…", "path=D:\…", "\\?\D:\…" and "/d/…"
  pass. No gate reads evidence/rm-capture.txt, though a criterion calls it clean.
- pulse-app's cwd reaches the prompt as "PROJECT: <basename>" and "workspace=<abs path>" (assembler.rs:208, :629;
  main.rs:746-748): launched from this repo, "conductor" reaches the model and satisfies the whole-word check. The boot
  line app.boot.workspace_key workspace_root_basename (main.rs:259-266) observes it.
- Missed sites — each a T3 soft-exit at implement: Scenario has no Default, and four literals sit outside the
  modify-set: conductor-core/src/error.rs:73, conductor-core/src/load_envelope.rs:496,
  conductor-run/tests/dispatch_wire.rs:24, conductor-timeline/src/convert.rs:43 (a crate the plan names nowhere).
  crates/conductor-run/src/testkit.rs builds Preflight at :20 and ContractTerm at :88. observed_env() with two L4-keyed
  terms breaks run_contract.rs:324-330 (pins exactly [L4, MCP]) and crates/conductor-run/tests/run_contract_pin.rs
  :80-92 unless deduped. Nine doc comments go false: run_contract.rs:14, :62, :80-81, :155; core
  preconditions.rs:26-27; run preconditions.rs:39-40, :180; run_contract_pin.rs:3, :70.
- Step 6's arm: cli_smoke.rs:450 run_preconditions sets no current_dir, so "--for" from the crate dir hits a load
  fault and the env_remove half passes vacuously — the conductor(dir) + copy_scenario root is the working form.
  "--for <P-ID>" resolves in find_by_pid's unsorted read_dir order (paths.rs:112-131), and five scenario files name
  P-018.
- The stop form kills by image name, every conductor.exe and sidecar, against "stops only what it started" (posture
  :171-172) and host-win32.md:135 (reap by start time). On a timeout, `timeout` kills only cargo, leaving conductor.exe
  and the sidecar running. The arm runs about 24 min — longer than one foreground tool call — so say it is
  backgrounded.
- Amendment wording: architecture.md:135 says the declare-only families' rows land KnownResidual, so adding this leg as
  a tenth family leaves it false (its success row is ManualCheck); :198 (the L4 row "ONE OF TWO shell-declaration
  proxies…") and :182 ("the emission profile a hypothesis observable requires") are named by no amendment; the v3-09
  2026-09-18 note carries the single-storm claim and the ~110 s figure; contracts/scenario-audit-ledger.toml:32 says
  "the deterministic interpretation mode every verifiable run uses". The count basis grep -l '^[[expected]]' returns 0
  as written (unescaped brackets); '^\[\[expected\]\]' returns 2 — the same form sits at architecture.md:135.
- The posture-document host-path probe is [A-Za-z]:[\\/] without the \b its harvest sibling carries (testing.md
  2026-09-03 word-anchoring); latent, since the posture holds no "://" today.
- .ps1 parity: it binds only $args[1] (agent-run.ps1:17) while the selector is the third token; removing RUST_LOG needs
  save/remove/restore (process-wide when the script is dot-invoked); name the UTF-8 method (`*>` writes UTF-16 on
  Windows PowerShell). No entry exercises either shell's new arm, though "run --live <bogus>" -> exit 2 needs no Pulse.
- formation_ms = opened_at - emission instant measures pickup, not formation (inference is not in it) — the same
  mis-attribution P-E corrects; name what it measures before it enters the posture's provenance line.
