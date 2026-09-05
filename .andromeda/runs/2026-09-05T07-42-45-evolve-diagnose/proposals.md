# Evolve Diagnosis — Conductor · Epoch 6a "Verification follow-ups" · 2026-09-05T07:42:45Z

Founder-facing, evidence-backed, obligation-free. Nothing here is applied, queued or remembered by the mechanism; a re-run recomputes from the ledger alone. Evidence twins: `q-retractions.json` · `q-health.json` · `q-typed.json` · `q-chains.json` · `q-level.json` beside this file.

## Mechanism health

- **Records:** 239 in-epoch (106 step / 133 friction) across 7 chunks + 11 session starts + 2 no-op wraps. After the retraction pre-pass: **237** (131 friction) — every stage below runs over the filtered stream.
- **Retractions (whole-ledger pre-pass):** 5 retraction records. Honored: 2 friction ids, both in-epoch (`2026-09-03T06:11:16Z-b` — a swept-site enumeration derived from the post-amendment file; `2026-09-04T16:55:27Z-b` — "unprovable on this host" re-scoped to "while the SUT is down"); 2 problem-facts (`2026-08-20T21:14:37Z-a` all, Epoch 4; `2026-09-02T13:31:41Z-a#0`, Epoch 5 — out of range). **Unresolvable, reported verbatim for manual discount:** 1 — a prose-form pre-boundary retraction (Epoch 1, `2026-08-08-sut-capability-manifest` phase/plan: "the untyped code-graph-under-reports record and the first problem-block entry on…"). Retraction-of-retraction: 0.
- **Coverage per chunk:** 7/7 chunks carry every expected checkpoint — phase 5 · implement 3 · wrap 5 — **0 gaps**. 11 `new-session/orientation` records = 11 session starts. The two 0-pending wraps (`63f5e5f`, `ff08206`) carry `curation` **and** `route-resolve` with `chunk: null`; the reference expects at most `curation` for a no-op wrap — these were operator-requested adaptations that did route work (observation, not a gap).
- **Unparseable lines:** 0 (whole ledger, 1398 lines).
- **Untyped rate:** 22/131 = 17%. Per step: new-session/orientation 5/9 (56%) · wrap/gates 3/6 (50%) · phase/plan 2/6 · implement/smoke 1/4 · implement/fix-loop 3/16 · wrap/route-resolve 2/10 · every other step ≤ 1; distill and reconcile 0. The two high-rate steps are exactly where Stage 2 finds extension candidates (U1, U3).
- **Problem-fact fill:** 31/106 step records populated (29%), 34 facts — workaround 24 · removed-cause 6 · prohibition 2 · deferred 1 · overridden 1.
- **id fill:** 239/239. **Outcomes:** ok 101 · ok-degraded 3 · halted-resolved 2 (both wrap/reconcile, both operator-resolved escalations).
- **Consumed-quality non-ok verdicts:** 13 — plan ×4 (wrong 1, thin 3) · report ×3 (missing 2 on the no-op path by design, thin 1) · handoff ×2 · wrap-playbook ×2 · research ×1 · spec:a11y-plan.md ×1.
- **Calibration boundaries in range:** none apply — every in-epoch record post-dates 2026-08-18, so `id` + schema'd `retracts` are required and present; the deviation scan and the Universal types are live throughout.
- **Reader-side folds applied:** the epoch label appears as `Epoch 6a — …` (212) and `Epoch 6a - …` (27) — folded before every group-by (a producer writes the hyphen form; see U4). The `skill` field carried the `andromeda-` prefix uniformly (no phantom split).
- **Lineage blind spot (Stage 3, by construction):** implement's P4 report is consumed by wrap/report in 4 chunks (`input.implement-outcome-unsettled`) but no implement step declares it in `produced[]` (implement declares `source` · `matrix` · `conversation` only), so the join cannot see that edge at all. Worth a line in the implement playbook's produced list.

## Proposals (typed patterns)

Weight = Σ per record of `1 + iterations + retries + reformulations + 2·dialogue + 3·halted + 3·soft_exit`. Thresholds (F-2): n ≥ 3, or n ≥ 2 with halt/soft-exit impact. Universal types group across steps.

### P1 — `*/contract.premise-falsified` (Universal) — 13 cases · weight 15
take-up 5 · research 4 · distill 1 · validate 1 · fix-loop 1 · route-resolve 1 — 6 of 7 chunks + the 0-pending wrap.
**Pattern:** an authored upstream artifact stated a premise verification falsified — route-entry freight (CARRY/CONTEXT coordinates, self-locations, counts) ×7, a master's attribution or named mechanism ×2, a code-audit hypothesis, a ratified roster, a plan step's mechanism, a route BLOCKED-ON.
**Evidence:** ALL 13 —
| chunk | what | impact | evidence |
|---|---|---|---|
| conductor-run-composition-root · take-up | test-plan:500 attributes the `declares` env-reading edge to conductor-verify; `fn declares` lives at conductor-run/src/lib.rs:357 | extra_reads 1 | `grep -rn 'fn declares'` → single hit |
| sr-findings-remediation · take-up | two freight coordinates wrong: lib.rs:936 lands inside the RunStage enum (abort poll is :975); the cited `@foreground.initialFocus` record cannot evidence finding 1 — `tabsToStart` is dropped by parse-nvda-log.ts:439-444 | extra_reads 4 | scope.md#coordinate-re-verification |
| sr-findings-remediation · take-up | folded CONTEXT says the 14 browse-class rows are "the A11y CI gate's CARRY" — false since the 2026-09-04 wrap re-pinned CARRY 2b onto this very entry | — | scope.md#open-boundary-question |
| sidecar-spawn · take-up | "retire the leg's re-activation step" (singular): two activate-window.ps1 call sites, only reactivateWindow():121 is the workaround; bringToForeground():69 is a structural guard a mechanical read would have deleted | extra_reads 1 | scope.md §Folded freight |
| sidecar-spawn · take-up | CARRY 3 says finding 1's mechanism is "unidentified and nothing guessed"; screen-reader.e2e.ts:78-84 already carries a MEASURED competing candidate | extra_reads 1 | scope.md §CARRY 3 |
| conductor-run-composition-root · research | code-audit "Suspected shape" (lib.rs test coverage not following growth) false: test/prod ratio ROSE 0.535 → 0.602 over 2026-08-16..09-02 | extra_reads 1 | `git show` at four commits |
| live-pulse-preconditions · research | design-system + layout-templates name `anstream` as the TTY gate; shipped gate is owo-colors + IsTerminal (render.rs:133/:141); arch already agrees with the code | extra_reads 1 | research.md §Surfaced spec-reality divergence |
| sr-findings-remediation · research | scope [inferred] "Not a live-Pulse chunk" false: execute_scenario returns Blocked BEFORE its hold site; 7 of 8 findings carry live-subject rows | extra_reads 3 | research.md#scope-premise-closure |
| sidecar-spawn · research | a FIFTH stale `TokioChildProcess` site (spawn.rs:79) beyond scope's four; `std::process::Command` exposes no creation-flags getter — reshapes the test tier | extra_reads 2 | research.md Scope premise closure |
| sr-findings-remediation · distill | the tests distiller found test-plan §6 records the sr subject in the driven arm's live-Pulse form, contradicting scope's bullet; routed to P3 | — | runs/2026-09-04T02-08-37Z-phase/tests.md |
| conductor-tauri-survivors · validate | plan step 5 claimed the run-data kills reach resolve_under's traversal guard; the base canonicalizes first so the `..` check never runs — caught by the operator's review, not by P5 | dialogue 1 | plan.md; config_path.rs:18-31 |
| conductor-run-composition-root · fix-loop | the ratified accepted-deliberate `declares` roster (6) over-broad by TWO — `declares:358` fell to the same test as the ratified kill | — | mutants-impl-2026-09-03/caught.txt |
| null (0-pending wrap) · route-resolve | A11y CI gate's BLOCKED-ON "a CI runner (none exists)" false since the 2026-08-22 wrap that wrote it — both ci.yml jobs run on windows-latest; nothing sweeps the working-route | extra_reads 2 | runs/2026-09-04T01-35-00Z-wrap/adaptation-record.md |
**Proposal:** most rows are the designed premise-closure WORKING — every chunk's research record signals `premise-corrected`, at ≤ 4 extra reads each. The generalizable residue is the **7 route-freight rows**: drift detection sweeps the seven masters and nothing sweeps the working-route, so an annotation's coordinates, counts and self-locating phrases ("the entry that next touches X") expire silently until a promotion reads them. Direction: give wrap's route-resolve a bounded freight re-verification over the tail it touches (file:line coordinates resolved at HEAD; self-locations re-checked after any move), or stamp each annotation with its authoring commit so a promotion can read its age. The project absorbed the BLOCKED-ON instance as a CLAUDE.md extension (2026-09-04); the sweep is the pipeline-level generalization.

### P2 — `phase/validate/contract.mechanical-check` — 6 cases · weight 15 · rate 0.86
**Pattern:** the seven P5 mechanical checks are structural (sections, path existence, placeholders, size, gate listing). In 4 of 6 cases the operator's review caught plan defects no check can reach; in 2 the checks caught real authoring defects.
**Evidence:** ALL 6 —
| chunk | what | impact | evidence |
|---|---|---|---|
| mutation-tier-restored | check 4 caught `cargo audit` listed as a plain command — RED by standing deferral, so its disposition annotation was needed for wrap's noted-deferred arm | — | plan.md |
| conductor-run-composition-root | check 4 fired twice (criteria naming gates absent from Test Commands); check 5 a `{marker}` leak; the audit disposition added | — | plan.md |
| live-pulse-preconditions | operator review returned THREE defects no check reaches: the standing PREREQ absent from Test Commands (no criterion named it, so check 4 had nothing to fire on); two probe redirects into the un-ignored repo root; an unanchored drive-letter regex matching inside `http://127.0.0.1:4317` | dialogue 1 | plan.md; `git check-ignore` exit 1 |
| sr-findings-remediation | two Test Commands defects survived P5: the non-priming probe listed AFTER the legs it must precede (check 4 verifies presence, never order); its firing form named `./target/debug/conductor.exe` while the block builds release | dialogue 1 | plan.md; agent-run.sh:84 |
| preconditions-probe | no predicate for plan-INTERNAL consistency: step 5 froze `declares()` while step 8 predicted its mutation roster would shrink — each step unambiguous alone, contradictory jointly; val-1 compares plan to scope, never plan to itself | dialogue 1 · iterations 1 | plan.md vs test-plan §12:608 |
| sidecar-spawn | check 4's boot-path clause found spawn.rs on the boot path with no smoke; check 2 found a rule file named in notes but assigned to no step — both fixed at authoring | iterations 2 | plan.md |
**Proposal:** add the predicates the operator supplied by hand this epoch — (a) standing-PREREQ presence in Test Commands independent of any criterion naming it (the project minted this rule in security.md 2026-09-03; P5 is its pipeline home); (b) every Test Command output path passes `git check-ignore` or sits in the run dir; (c) a named binary resolves to what the block's build step produces, and a listed non-priming probe precedes the legs it guards; (d) plan-internal consistency — a step that FREEZES a symbol vs a step that PREDICTS a measurable change in it (curated project-side as CLAUDE.md Tier-1, 2026-09-04). Rows 1, 2, 6 show the set earning its keep; this adds, it does not replace.

### P3 — `*/contract.narrow-basis-claim` (Universal) — 5 cases · weight 7
curation 2 · report 1 · validate 1 · fix-loop 1 — 2 chunks + the 0-pending wrap; 2 of the 5 are retraction carriers.
**Pattern:** a count, enumeration, precedent or stimulus was derived from a source narrower than the claim and would have stood if not re-derived.
**Evidence:** ALL 5 —
| chunk | what | impact | evidence |
|---|---|---|---|
| null · curation | a graph LIMITATION concluded from ONE anchored query — `%/read_envelope().%` cannot match a crate-root symbol; re-measured 5 sites / 3 callers, present all along | — | runs/2026-09-02T22-20-00Z-wrap/tree-query-wrap-adaptation.json (retracts an Epoch-5 problem fact) |
| null · curation | a dictated coordinate (get_envelope 6 sites / 3 callers) measured 6 / 4; the two sibling claims verified exact, so the measured value was written | extra_reads 1 | same trace |
| conductor-tauri-survivors · report | a prior wrap's swept-site enumeration derived from the POST-amendment file, so a site that carried a swept token was recorded as carrying none | — | `git show d86ed0b~1` → 80, 93, 371 (retracts `2026-09-03T06:11:16Z-b`) |
| sr-findings-remediation · validate | a P4 fork justified deferring findings 3+8 by a handoff-inherited precedent — false: the 2026-09-02 pass ran against a LIVE Pulse; the clause referred to the driven arm | dialogue 1 | nvda-pass.json S2-07 / S3-04 / S3-05 |
| sr-findings-remediation · fix-loop | own spec edit wrote R0-01's stimulus "needs no reload" from the markup change while the leg still reloads — the row could not discriminate first-load from post-reload | — | nvda-pass-spec.md / rows.ts |
**Proposal:** neither the wrap report nor the plan template asks for the DERIVATION beside a stated count, site list or precedent. Direction: a basis clause (command, or file + revision) on every such statement in report.md's Changes and plan.md's Implementation notes — the two retractions here were separated from truth only by re-deriving. The graph-anchor half is curated project-side (CLAUDE.md 2026-09-02); the dictated-coordinate half arrives through the operator directive and needs its check at the receiving step.

### P4 — `implement/fix-loop/contract.test-expectation` — 4 cases · weight 7
live-pulse-preconditions 1 · sr-findings-remediation 3.
**Pattern:** a test's expectation or guard encoded a premise the change invalidated.
**Evidence:** ALL 4 —
| chunk | what | impact | evidence |
|---|---|---|---|
| live-pulse-preconditions | two conductor-core tests take/restore the process-global panic hook and race under `cargo test`; red exactly once, unreproducible in 3 forced runs — pre-existing, UNMASKED by +14 tests | retries 1 · extra_reads 3 | obs.rs:491-496, :532-537 |
| sr-findings-remediation | the checklist spec's subject-absent guard `$('[role="status"]').isExisting()` stopped skipping once a11y-mandated live regions were added; resolved by `aria-live` on the new regions — guard still loose for the next `role=status` | iterations 1 | accessibility.e2e.ts |
| sr-findings-remediation | the `--e2e` gate's own exit was 0 on a run wdio reported "0 passed, 1 failed"; the true exit only in the redirected log; the PRINTED verdict caught two regressions | — | scripts/agent-run.ps1 |
| sr-findings-remediation | removing the client's optimistic `aborted` left the phase line "live" (drive_run polls only between scenarios); the right shape keeps BOTH halves — only a live leg could show it | retries 1 | App.tsx / conductor-run lib.rs |
**Proposal:** rows 2-3 are project harness defects surfaced, not pipeline friction — the ps1 arm exiting 0 over a red suite is the shape test-plan §10 exists to forbid and is worth an owner. Pipeline residue: where a project's own rules say a tool's exit is uninformative (host-win32.md "the printed text outranks an unisolated `$?`"), the plan template's Test Commands could require the printed verdict as the asserted signal — row 3 was caught by discipline, not by the plan.

### P5 — `*/tooling.host-shell` (Universal) — 4 cases · weight 7
distill 2 · research 1 · validate 1 — 4 chunks. Chronic: E3 14 · E4 13 · E5 3 · E6a 4, zero halts.
**Evidence:** ALL 4 —
| chunk | what | impact | evidence |
|---|---|---|---|
| mutation-tier-restored · distill | the evolve-system append recipe rejected the record — "Invalid \escape" at char 3387 — a Windows path's backslashes in hand-escaped heredoc JSON; rebuilt as a python dict so `json.dumps` escapes | retries 1 | scratchpad/append_distill.py |
| sidecar-spawn · distill | 5 of 7 extract returns HTML-escaped `&lt;/&gt;` on load-bearing tokens (`<host-path>`, `cargo test -p <crate>`, `<body>`) — as fan-out.md predicts; decoded, twins reconstructed | — | make_twins.py re-encode counts |
| conductor-run-composition-root · research | the background-task notification reported exit 0 for the mutation tier; the true exit 2 belonged to the gate and was masked by the trailing `echo EXIT=$?` | — | log tail `EXIT=2` |
| live-pulse-preconditions · validate | an inline `python -c` regex probe lost its escaping through bash's double-quoted `-c` and returned a FALSE negative on real Windows paths; the SAME class then broke the checkpoint's own ledger append (`json.loads` at char 1860) | retries 2 | host-win32.md |
**Proposal:** rows 1 and 4 are the evolve-system append RECIPE itself (byte-synced ×4): a quoted heredoc of hand-escaped JSON has no guard against a backslash-bearing note, and on this host every path is one. Direction: document a second sanctioned form — records authored as a python list in a Write-tool file and appended by path, `json.dumps` doing the escaping — the shape both records converged on independently. Row 2 is the known transport property, handled by contract; row 3's residue is the background-task notification reporting the trailing echo's status as the command's.

### P6 — `wrap-session/reconcile/input.report-insufficient` — 2 cases · weight 7 · halted 1
Threshold met via n ≥ 2 with halt impact. sr-findings-remediation · sidecar-spawn. See chain X3.
**Evidence:** ALL 2 (+ the consumed verdict `report: thin` at sidecar-spawn reconcile) —
| chunk | what | impact | evidence |
|---|---|---|---|
| sr-findings-remediation | validate check 5 caught a design-system entry the plan listed but no detector proposed; re-deriving it showed the REPORT mis-characterised the site — design-system:257 attributes the string to the coverage-matrix pattern, a third string the report never carried; escalated | dialogue 1 · halted 1 | report.md vs design-system:257 |
| sidecar-spawn | two detectors each routed the reactivateWindow registration to the OTHER master; independent greps: a11y-plan 0 hits, test-plan's one hit describes bringToForeground — the report's expected-amendment entry wrong on BOTH halves | extra_reads 2 | `grep -icE 're-?activat\|activate-window'` |
| sidecar-spawn (consumed `report: thin`) | the report's `TokioChildProcess` site list named architecture ×2; the security detector found two more, the orchestrator's cross-master grep a fifth in test-plan | — | fanout-results.md |
**Proposal:** the report template's Expected-amendments entry carries cross-doc ownership claims — unverified by construction (the project's own 2026-08-09 rule) — and site enumerations without a derivation. Direction: each expected-amendment entry names the grep that located its sites and the hit count, and never names a master as owner without a hit in it.

### P7 — `tooling.long-line-edit` — 8 cases across two groups (wrap/route-resolve 5 · weight 6 · phase/take-up 3 · weight 3)
One mechanism, so merged here; both groups clear F-2 on their own. The problem-fact side is level candidate **L1**.
**Evidence:** ALL 8 —
| chunk | what | impact | evidence |
|---|---|---|---|
| null · route-resolve | migrating a 1334-char PREREQ span out of a 4168-char single line — anchored Edit unusable; python read-modify-write with four post-write assertions | extra_reads 2 | working-route.md |
| conductor-tauri-survivors · route-resolve | EVERY document write this wrap (route, spec bodies, sidecars, rule append) via python RMW; one redo cycle from a `\n` anchor against a CRLF file, caught by the `count!=1` guard | iterations 1 | test-plan.md 608 CRLF / 0 LF |
| conductor-run-composition-root · route-resolve | the RMW path's direction-dependent CRLF trap: prepend round-trips clean, append lands after the CR — 119/119 diff for a 2-line intent; only the diff SIZE surfaced it | extra_reads 2 | `git diff --stat` 119/119 → 3/3 |
| null · route-resolve | three 1734-3687-char entries spliced by line index with identity asserts; result re-derived from `git show` rather than the tool's success line | extra_reads 2 | working-route.md |
| sr-findings-remediation · route-resolve | four route edits via RMW; the audit PREREQ lifted off a frozen 4313-byte line and re-pinned 49th → 50th | — | working-route.md |
| conductor-run-composition-root · take-up | 2233-byte single line to stamp; the skill mandates anchored Edit, host-win32.md says it defeats it — followed the project rule, verified by read-back (+68 bytes = the marker) | extra_reads 2 | working-route.md, master-route.md |
| live-pulse-preconditions · take-up | freeze stamp via RMW with `newline=''`; second instance after the append-direction CR corruption | — | `git diff --stat` 1/1 and 1/0 |
| sr-findings-remediation · take-up | 4276-byte line; awk byte-length vs python char-length disagreed by 40 multibyte chars, reconciled before trusting either | extra_reads 2 | working-route.md |
**Proposal:** → **L1**. Wrap P7's flip-compaction already shrinks FROZEN lines (4313 B → 191 B, archived); the markerless tail, master-route records, rule-file Session Additions and the sidecars are the remaining single-line artifacts.

### P8 — `wrap-session/reconcile/contract.false-positive-proposal` — 4 cases · weight 6 · 4 chunks
**Evidence:** ALL 4 —
| chunk | what | impact | evidence |
|---|---|---|---|
| conductor-run-composition-root | D-tests-derived-count proposal 4 cited class C against §Occupied Resources — Ports; class C's barrier is sidecar resolution — citation corrected at apply | — | .raw-fanout-test-plan.md |
| live-pulse-preconditions | design-system's D-platform-claim change line carried collateral facts (indicatif 0.18, inquire 0.7) beside the anstream retirement it proposed; manifest says 0.17.11 / 0.9.4; applied the anstream half only | extra_reads 1 | Cargo.toml:64, :68 |
| preconditions-probe | D-platform-claim proposed 3 design-system edits by matching two shell NAMES in the report bullet — a token-proxy match on sites stating no platform verdict; operator dismissed all three; playbook :46 was NO MATCH (it names plan-to-plan-bind detectors only) | dialogue 1 | fanout-results.md (E2) |
| sidecar-spawn | D-arch-decisions re-derived `tokio 1.48.x → 1.52.3` from Cargo.lock:4494 against its own prompt's ban; dismissed under playbook :31 as not this chunk's drift; the staleness is real and recorded | — | fanout-results.md |
**Proposal:** three detector-side causes. (a) The apply practice "the applied text is re-derived from the invariant, never pasted" (rows 1-2) is undocumented — state it in the amendment-flow reference. (b) D-platform-claim fires on tokens in the report rather than a retired VERDICT in the doc (row 3) — tighten its invariant, and widen playbook :46's dismissal beyond plan-to-plan-bind detectors so the shape has a rule. (c) A doc-agent re-deriving from the codebase against its prompt (row 4) — the orchestrator could check each proposal's cited basis against the allowed set (report · doc · sidecar) before apply.

### P9 — `wrap-session/reconcile/contract.cascade-miss` — 4 cases · weight 6 · 4 chunks
**Evidence:** ALL 4 —
| chunk | what | impact | evidence |
|---|---|---|---|
| live-pulse-preconditions | the sweep keyed on connectives (`is a` / `=`) found 5 sites and MISSED verification-harness.md:18 — an em-dash connective on the primary operational description of `boot` | extra_reads 2 · retries 1 | verification-harness.md:18 |
| sr-findings-remediation | a SAME-master stale self-reference (security-plan:120 describing the row above as "presence-only reader") missed by the per-doc detector; caught by the orchestrator's cross-master sweep | — | security-plan.md |
| preconditions-probe | applying a proposal as written left the retired mechanism standing INSIDE the amended line — layout-templates:186 carried it twice, 60 chars apart | iterations 1 | fanout-results.md Cascade step 2 |
| sidecar-spawn | own probe malformed: `grep -clE` (`-l` suppresses `-c`) captured a FILENAME compared to "1" — could never fire; the corrected probe found five leaves, three needing re-derivation | extra_reads 2 | five leaf paths |
**Proposal:** the project curated "sweep for what the claim SAYS" (CLAUDE.md 2026-09-02) and the class recurred four times anyway — the remedy belongs in the cascade step's reference: (a) a connective-agnostic shape sweep as the documented form; (b) an intra-line duplicate check on every amended line (row 3); (c) a known-positive CONTROL before trusting a zero-hit probe (row 4 — a probe that cannot fire reads identically to a clean tree).

### P10 — `phase/plan/input.extracts-conflict` — 3 cases · weight 5 · 3 chunks
See chain X2.
**Evidence:** ALL 3 —
| chunk | what | impact | evidence |
|---|---|---|---|
| conductor-tauri-survivors | security-plan ("the six survivors must be KILLED") vs testing.md:67 + test-plan §10 ("an env-reading edge IS the accepted-deliberate case") govern the same functions; check C could not see it — the standing rule appears in no extract; resolved by measurement, BOTH ways in one chunk | — | research.md §Patterns; plan step 3 vs 9 |
| conductor-run-composition-root | scope.md ("carry the six `declares` survivors as-is") vs the arch extract ("whether they are still the foreclosed branch is research's question") — one side is scope, not an extract; the fork went to the operator | dialogue 1 | runs/2026-09-03T10-31-07-phase/arch.md; scope.md |
| sr-findings-remediation | design-system mandates an `aria-live=assertive` HOLD while the finding being fixed IS that assertiveness cancelling focus-restore; both extracts internally consistent and reciprocating, so check C silent | — | design.md / a11y.md |
**Proposal:** P2's aggregate check C compares extracts to extracts; all three conflicts had one side OUTSIDE that set — a standing rule file, scope.md, or a mandate whose tension shows only against the chunk's own finding. Direction: take scope.md's binding statements and the auto-loading rule files' governing clauses as a third input to check C (or have each distiller cite any standing rule governing the same functions), so the conflict surfaces at P2 rather than at P4 synthesis.

### P11 — `wrap-session/report/input.implement-outcome-unsettled` — 4 cases · weight 4 · 4 chunks
**Evidence:** ALL 4 —
| chunk | what | impact | evidence |
|---|---|---|---|
| mutation-tier-restored | implement ended `surfaced` (criterion 2 unresolved); the operator's wrap directive RULED it unmet and routed it — outcome basis = implement's measurements + the directive | — | report.md; mutation-tally.md |
| conductor-run-composition-root | the directive corrected an implement-ledger justification (the occupier-guard collision clause — the test binds an ephemeral port) between implement and report | extra_reads 1 | lib.rs:1176 |
| sr-findings-remediation | the operator changed a precondition after P4 (live Pulse + CONDUCTOR_NVDA) and withdrew the deferral; a further round ran three sr* suites and produced the evidence | — | leg-verdict.md |
| preconditions-probe | implement closed green-with-one-deferral; the operator relaunched Pulse and directed the witness closed — the report rests on post-implement artifacts and a ledger retraction | — | friction `2026-09-04T17:12:31Z-a` |
**Proposal:** 4 of 7 chunks, each recorded as "the attended pipeline working as designed". Epoch 5's untyped record `2026-09-01T16:51:31Z-b` asked for what these ask for: an explicit **outcome-basis** line in the wrap report (implement's P4 report · operator directive · post-implement artifacts), so a detector reading only the report never inherits a superseded claim. Template-level, cheap.

### P12 — `wrap-session/curation/ambiguity.filter-borderline` — 4 cases · weight 4 · deferred 2 (+1 untyped, same class, 5th chunk)
**Evidence:** ALL 4 + the untyped twin —
| chunk | what | impact | evidence |
|---|---|---|---|
| conductor-tauri-survivors | ALL THREE surviving candidates scored exactly 0.6; the whole keep/drop turned on the load-bearing +0.2 tiebreak (1 applied, 2 rejected) | — | testing.md, host-win32.md |
| conductor-run-composition-root | two Filter-1 calls underdetermined: additive-facet in-place vs sibling entry; a CODE-artifact facet the verify-the-artifact family does not name | — | testing.md |
| live-pulse-preconditions | two at exactly 0.6 rejected; one reached a durable home (test-plan, 2 sites), the other — a shared CARGO_TARGET_DIR across worktrees serving a stale binary — reached NO home, logged to the handoff only | — | verification-harness.md |
| preconditions-probe | two at exactly 0.6 rejected; (b) had already been identified as an additive facet at Filter 1 before Filter 4 rejected it | deferred 2 | CLAUDE.md, testing.md |
| sidecar-spawn (untyped `2026-09-04T21:05:00Z-b`) | two at exactly 0.6 rejected — "a workaround becomes harmful when the defect it compensated for is fixed"; "a live SR leg has run-to-run variance" — routed to the route as CARRY freight instead | deferred 2 | route entry *A11y CI gate* |
**Proposal:** in 5 consecutive wraps the modal strong candidate is measured (+0.4) + specific (+0.2) with no third signal, and the exact-hit rule rejects it deterministically. The records say the rule works as designed AND that at least one such fact reached no durable home. For the founder's calibration: admit a third signal class for this shape ("generalizes beyond the chunk" / "reached no other durable home this wrap"), or move the reject to strictly-below-0.6 and watch Tier-2 growth. Evidence only — the mass point is Andromeda's to move.

### P13 — `implement/code/tooling.hook-friction` — 3 cases · weight 4 · 3 chunks
**Evidence:** ALL 3 —
| chunk | what | impact | evidence |
|---|---|---|---|
| live-pulse-preconditions | the PostToolUse rustfmt hook re-ordered an import block between Read and Edit — anchored Edit failed; re-Read and re-anchored | retries 1 · extra_reads 1 | render.rs |
| preconditions-probe | four diagnostics fired against stale mid-burst state, every one already false when emitted — including a HARD-ERROR-shaped E0433 for an already-fixed cause | — | preconditions.rs |
| sidecar-spawn | dead_code on CREATE_NO_WINDOW after edit 1 of 2; clean after edit 2 | — | spawn.rs |
**Proposal:** all predicted by code-writing-discipline, ≤ 1 retry each. Observation more than proposal: the hazard is a hard-error shape mid-burst; if anything, have the hook label its diagnostics "mid-burst, may be stale" rather than suppress them.

### P14 — `wrap-session/curation/ambiguity.tier-routing` — 3 cases · weight 3
**Evidence:** ALL 3 —
| chunk | what | impact | evidence |
|---|---|---|---|
| null (0-pending wrap) | a CSSOM-assertion rule pointed at two Tier-2 homes by two routing rules (home registry → testing.md; originating proposal → frontend.md); settled by frontmatter — testing.md's paths are Rust-only | extra_reads 1 | testing.md, frontend.md |
| mutation-tier-restored | new entry vs in-place extension turned only on WHERE the match sat (Session Additions vs generated body) | — | testing.md |
| sr-findings-remediation | two of four survivors routed as extensions; the fifth-handle fact had three plausible homes in verification-harness.md, went to :59 by nearest-scope | — | verification-harness.md |
**Proposal:** each resolved deterministically; the routing rules could state the two deciding facts explicitly — the candidate's artifact language (a `.ts` spec never routes to a Rust-only-paths rule file) and that the additive-facet in-place amend is available only inside Session Additions. Minor.

### P15 — `*/contract.skill-reference-drift` (Universal) — 3 cases · weight 3
take-up 2 · research 1 — three consecutive chunks, the SAME defect.
**Evidence:** ALL 3 —
| chunk | what | impact | evidence |
|---|---|---|---|
| mutation-tier-restored · take-up | the phase skill BODY's Setup step 5 names a flat `.andromeda/cache/tree.db`; the deployed cache is per-plane (`rust/`, `ts/`) with the plane argument REQUIRED — resolved from CLAUDE.md, 0 retries | — | `ls .andromeda/cache/` |
| conductor-tauri-survivors · take-up | same: an agent following Setup literally finds no DB and would declare it stale — the silent-flat-rebuild regression security.md 2026-08-21 describes | — | same |
| sidecar-spawn · research | same; the skill's OWN `codebase-research.md` reference states the per-plane form correctly — body and reference disagree | — | andromeda-phase/SKILL.md vs references/codebase-research.md |
**Proposal:** cost was zero only because this project's CLAUDE.md documents the plane form. Direction: edit the andromeda-phase skill body's Setup step 5 to the per-plane path its own reference already states — a one-line Andromeda fix that project drift detection can never reach.

## Cross-step chains (starting heuristics)

Mechanical join in `q-chains.json`: 34 anchors, 18 with a same-chunk producer; the shapes below are the ones recurring across ≥ 2 chunks.

### X1 — phase/plan →plan→ implement/{code, fix-loop} — 4 chunks (+1 through validate's re-produced plan; +1 where validate consumed it `wrong`)
- `2026-09-02T23:35:15Z-a` plan (ok · designed-dialogue:binding-form) → `2026-09-03T05:10:12Z-b` code (`input.plan-step-ambiguous`: two readings of "replace `cargo_bin` with `env!`").
- `2026-09-03T11:04:42Z-a` plan (ok) → `2026-09-03T11:32:53Z-a` validate (`plan: wrong` — step 3's named test never enters execute_scenario) and `2026-09-03T11:56:00Z-a` code (`plan: thin` — step 1's tracing capture unreachable without the sidecar binary).
- `2026-09-03T16:16:01Z-a` plan (ok · designed-dialogue ×2) → `2026-09-03T19:12:52Z-a` fix-loop (`plan: thin` — the CLI-edge Test Command's TempDir form is rejected by resolve_under before dispatch).
- `2026-09-04T02:34:08Z-a` plan (ok) → `2026-09-04T03:01:41Z-b` code (`input.plan-step-ambiguous`: step 9 presumes a mount-time focus not present in the code).
- `2026-09-04T15:02:45Z-a` validate (plan · review-corrected) / `2026-09-04T19:29:11Z-a` plan (unresolved-questions) → `2026-09-04T20:11:43Z-a` fix-loop (`plan: thin` — Test Commands under-listed the sr suites; CARRY-4's own criterion had no producing command).
**Hypothesis:** the plan names its vehicles (a test, a function, a command form, a suite list) from research prose and graph rows rather than from the vehicle's body; P5's checks are structural (P2); so the plan is formally `ok` with designed-dialogue signals every time and implement discovers the gap. No producer signal exists for "mechanism unverified".
**Direction:** a plan-authoring rule + mechanical check pair — a step naming a vehicle cites the line where the vehicle reaches its target (read at P4, checked at P5); Test Commands derived from the acceptance criteria so a criterion with no producing command is detectable.

### X2 — phase/distill →extracts→ phase/{plan, research} — 3 chunks
- `2026-09-03T07:31:19Z-a` distill (ok · binding-convergent) → `2026-09-03T07:45:47Z-b` plan (`extracts-conflict`: security vs testing.md:67).
- `2026-09-03T10:41:09Z-a` distill (ok · binding-unilateral) → `2026-09-03T11:04:42Z-b` plan (`extracts-conflict`: scope vs the arch extract).
- `2026-09-04T02:20:04Z-a` distill (ok · binding-unilateral · history-informed-all-seven) → `2026-09-04T02:25:05Z-c` research (`extract-signal-gap`: extracts name "No scenarios found." where the shipped picker renders "No scenarios match.") and `2026-09-04T02:34:08Z-b` plan (`extracts-conflict`: assertive HOLD vs focus-restore).
**Hypothesis:** distill's convergence checks judge extracts against each other; the producer's `binding-*` signals describe agreement AMONG extracts, not with the code or the standing rules — which is where all three conflicts and the gap lived.
**Direction:** → P10; and for the signal gap, the extract contract could require a load-bearing UI string be quoted from the shipped source.

### X3 — wrap-session/report →report→ wrap-session/reconcile — 2 chunks
- `2026-09-04T07:35:44Z-b` report (ok; `implement-outcome-unsettled`) → `2026-09-04T07:51:43Z-b` reconcile (`report-insufficient`, halted).
- `2026-09-04T20:37:43Z-a` report (ok; its own `contract.detector-fact-gap` `2026-09-04T20:37:43Z-b`: Expected-amendments under-ran by one) → `2026-09-04T20:59:02Z-a` reconcile (`report: thin` — 2 sites named, 5 found) + `2026-09-04T20:59:02Z-d` (`report-insufficient`).
**Hypothesis:** the report's site enumerations and expected-amendment ownership claims are written from session memory rather than derived; reconcile's detectors and the orchestrator's cross-master grep re-derive and find more or find the claim wrong. The report step's own frictions say the template has no home for the facts the detectors need.
**Direction:** → P6.

**Anchors without a same-chunk producer (reported, not chained):** implement's P4 report ×4 (never declared produced — see Mechanism health); `wrap-playbook: thin` ×2 (an Andromeda artifact: a gate-condition-vs-its-own-rule escalation with no rule, `2026-09-03T09:52:27Z-a`; a chunk shipping the fix its master names as route-owned with no precedent, `2026-09-04T17:35:15Z-a` — the latter minted a rule at E1); `spec:a11y-plan.md: wrong` ×1 (a master's stale path — cross-chunk); `handoff: thin` ×2 + `input.handoff-git-mismatch` ×2 (null chunk; the handoff's Position named a BLOCKED-ON entry and post-handoff sessions moved the ahead-count — see U1); `report: missing` ×2 (the no-op wrap, by design); `input.carry-context-gap` ×2 (the CONTEXT was authored by a PRIOR chunk's route-resolve — outside the same-chunk bound; the freight problem is P1's).

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 9 facts (+ 11 typed correlates, + 2 CRLF removed-cause facts) — theme A: multi-KB single-line artifacts
**Facts:** all workaround —
- `2026-09-02T22:24:06Z-a#0` null · route-resolve · process — a 4168-char single line, 1334-char PREREQ span; python RMW by path with post-write assertions.
- `2026-09-03T07:20:46Z-a#0` conductor-tauri · take-up · environment — freeze stamp + master append on multi-KB single lines; RMW, verified by re-reading the artifact.
- `2026-09-03T09:52:27Z-a#0` conductor-tauri · reconcile · process — 8 spec-body edits + 3 sidecar appends via RMW: test-plan targets are multi-KB single lines (:226 > 1600 chars) in CRLF files.
- `2026-09-03T09:54:46Z-a#0` conductor-tauri · curation · process — rule-file Session Additions append via RMW; "third occurrence this session of the same deviation".
- `2026-09-03T09:57:06Z-a#0` conductor-tauri · route-resolve · process — "fourth occurrence this session of one mechanism".
- `2026-09-03T10:33:42Z-a#0` conductor-run · take-up · environment — the skill's Write/Edit-only constraint does not handle the stamp; RMW with assert-on-anchor + read-back.
- `2026-09-03T15:52:57Z-a#0` live-pulse · take-up · environment — RMW for stamp and master append, diff sizes verified (1/1, 1/0).
- `2026-09-04T01:52:00Z-a#0` null · route-resolve · process — move/re-pin as RMW with Write-tool-authored payload.
- `2026-09-04T02:11:10Z-a#0` sr-findings · take-up · environment — a 4276-byte line; RMW by line index, verified by `git diff` + a byte-level CRLF check.
- Sub-mechanism (removed-cause ×2, one wrap): `2026-09-03T12:58:00Z-a#0` (append landed after the CR — 119/119 diff for a 2-line edit) · `2026-09-03T13:05:00Z-a#0` (mixed-ending file merged the appended record; script made terminator-agnostic).
- Typed correlates: P7's 8 `tooling.long-line-edit` records; `tooling.output-cap-overflow` ×2 (`2026-09-03T16:09:04Z-b`, `2026-09-04T10:00:31Z-b` — both verification-harness.md's multi-KB single lines, 48 KB / 59 lines); `tooling.commit-mechanics` ×1 (`2026-09-03T13:05:00Z-b`).
- Prior epoch: Epoch 5 `2026-08-22T21:22:44Z-c#0` (curation: anchored Edit instead of the prescribed whole-content Write on ~10k-char entries) · `2026-09-01T20:14:51Z-a#0` (take-up: Read offset gave no file-end on multi-KB single lines); `tooling.long-line-edit` E5 3 → E6a 8 (chronic-degrade, 0 halts).
**Level hypothesis:** the cause lives in the pipeline's artifact grammar — single-line multi-KB route entries, master records, rule-file Session Additions, spec bodies and sidecars — combined with the skills' Write/Edit-only write constraint. Every fix so far is project-side (host-win32.md's read-modify-write recipe, per-step scratchpad scripts, post-write asserts) and is re-applied at every step that touches those artifacts; the wrap's own constraint is contradicted by the project's auto-loading rule at each of them.
**Proposal:** (a) a pipeline-shipped line-indexed splice helper with CRLF-preserving semantics, documented as the sanctioned write path for single-line artifacts — the workaround becomes the path and its terminator trap is fixed once; or (b) an artifact grammar permitting wrapped entries so anchored Edit works again; and extend flip-compaction's compact-and-archive idea to rule-file Session Additions, which share the shape and caused both output-cap overflows.

### L2 — band-aid — 5 facts (+ 5 typed) — theme B: POSIX-shaped command recipes vs the host's guard rails
**Facts:** all workaround —
- `2026-09-03T05:18:51Z-a#0` mutation-tier-restored · fix-loop · process — a backgrounded compound with `rm -rf` in a launch path DENIED; "the rule was in context and I did not apply it"; 2 extra calls.
- `2026-09-03T18:49:12Z-a#0` live-pulse · validate · environment — an inline, quote- and backslash-bearing `python -c` probe returned a FALSE negative on real Windows paths (exactly the case host-win32.md excludes); re-run from a file, correct on all 7 cases.
- `2026-09-03T20:16:59Z-a#0` live-pulse · reconcile · process — a 9429-byte sidecar-append call blocked by the PreToolUse hook at the 6500-byte ceiling; re-routed through a scratchpad file. "The block was correct and cost one turn."
- `2026-09-04T03:01:41Z-a#0` sr-findings · code · process — a cat-heredoc script blocked by the hook (correctly); re-issued as a Write-tool scratchpad script; first pass substituted ASCII for the file's Unicode convention.
- `2026-09-04T16:52:34Z-a#0` preconditions-probe · fix-loop · environment — a backgrounded `rm -rf target/mutants-*` + two launches DENIED; "host-win32.md predicts verbatim".
- Typed correlates: P5's 4 `tooling.host-shell` (two of them the ledger recipe's hand-escaped JSON); `recall.corpus-recurrence` `2026-09-03T06:11:16Z-c` ("host-win32.md says rm -rf compounds get denied — the rule existed and the work reproduced the failure").
- Prior epochs: Epoch 3 (`2026-08-15-canary-spans-pulse-fingerprints` reconcile: a combined write + `rm -rf` denied) · Epoch 4 (`2026-08-20T21:19:11Z-a#0`: operator denied `rm -rf` of mutants.out); `tooling.host-shell` E3 14 · E4 13 · E5 3 · E6a 4 — four epochs, never halting.
**Level hypothesis:** the guard rails are doing their job (every block was correct); the recurrence is at authoring time, where the skills' documented recipes are POSIX-shaped — inline heredocs, compound launches, inline `python -c`, hand-escaped JSON in a quoted heredoc — and the host's rule that forbids them lives in a project rule file that auto-loads and is not applied. The fixes are a project rule plus per-incident re-issue.
**Proposal:** make the SKILLS' recipes host-shaped where they are POSIX-shaped — the evolve-system append recipe (P5), any "rm + launch" step forms in implement's references — and where a host rule file exists, have the skill body point at it AT THE STEP ("write payloads per the host rule file") instead of relying on recall. The project's own 2026-09-01 curation entry already says a rule that auto-loads does not reach the moment of use.

### L3 — band-aid (at threshold by prior-epoch recurrence) — 2 facts (+ 2) — theme H: build residue / reused output locations
**Facts:** `2026-09-03T19:12:52Z-a#0` live-pulse · fix-loop · process — a worktree at HEAD with a shared CARGO_TARGET_DIR rebuilt conductor-core's test binary from HEAD, so the next run reported 263 tests instead of 277 — green over the wrong sources, caught by the COUNT. `2026-09-04T16:52:34Z-a#1` preconditions-probe · fix-loop · process — the plan's literal `--output target/mutants-core-2026-09-04` could not be reused for a re-run (stale by construction), went to `…b`; the plan anticipated one run per file. Correlates: untyped `2026-09-03T05:18:51Z-b` (a 2026-08-21 `mutants.out/` residue read as a clean sweep — cargo-mutants nests its real tallies at `mutants.out/mutants.out/`), `tooling.environmental` `2026-09-03T19:12:52Z-b`. Prior: Epoch 4 `2026-08-20T21:19:11Z-a#0` · `2026-08-20T23:25:07Z-a#0` (mutants.out residue in the tree).
**Level hypothesis:** the cause is tool semantics the pipeline's templates do not encode (cargo-mutants' nested output dir; a literal output path in a plan; a shared target dir across worktrees); the fixes are a project testing rule (2026-09-03, "a reused output dir is stale by construction") and per-run unique dirs chosen by hand.
**Proposal:** the plan template's gate command forms prescribe a per-run unique output location by construction (timestamp or run id), never a literal; a testing reference note on the nested `mutants.out/` and on asserting the test COUNT whenever a target dir is shared.

### L4 — deferred-forever — the standing cargo-audit external-decay deferral
**Facts:** `2026-09-04T08:06:00Z-b` gates — "satisfied by probe-auto-satisfy for the 49th consecutive time … this bounded wait is now 27 days and 41 chunks old" (the session handoff records the 51st re-pin at this epoch's close). `2026-09-03T06:14:24Z-b` route-resolve — the pin sat on a FROZEN line because the chunk that carried it had just completed; the reference's insertion phrasing did not cover it. P2 rows 1-3 — three plans had to carry the pin's disposition inline, and in one no check could fire because no criterion named it. Route-resolve `standing-pin-migrated` in 5 of 7 chunk wraps: `2026-09-02T22:24:06Z-a` (45th) · `2026-09-03T06:14:24Z-a` (46th) · `2026-09-03T09:57:06Z-a` (47th) · `2026-09-03T12:58:00Z-a` (48th) · `2026-09-04T07:56:05Z-a` (50th). Closure seen: none — the end condition (the advisory database heals) has not arrived. Rule-sanctioned (ratified 2026-08-10). The other two in-epoch deferrals both CLOSED (commands.rs doc comments — carried by `2026-09-03T09:57:06Z-a` and fixed in the sidecar-spawn chunk; the boot ReadyState witness — retracted same session by `2026-09-04T17:12:31Z-a`).
**Level hypothesis:** the cause is external (RustSec's advisory-db carries a duplicate id) and has no in-repo remedy; the carriage cost is paid per chunk in three project places — plan Test Commands disposition, working-route annotation migration, light-gate re-pin — and grows with every chunk.
**Proposal:** a pipeline-level standing-deferrals register (one file: origin, signature, end condition, re-pin count and age) read by P5's PREREQ check and the light gate, replacing per-chunk carriage; and/or the audit recipe pinning a known-good advisory-db revision so the gate is green-with-pin rather than red-with-signature. Whether a 51-re-pin wait is still "bounded" is the founder's call.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — new-session/orientation — 3 cases → proposed type `ambiguity.ladder-uncovered-state`
**Cluster:** `2026-09-02T21:42:00Z-b` (rung 4 has no arm for a BLOCKED-ON first-markerless entry; resolved by naming the unblocked siblings, which needed entry-body reads) · `2026-09-02T22:13:09Z-d` (same; taking a sibling first is a route reorder phase does not do) · `2026-09-04T01:28:05Z-b` (same, "second consecutive session reaching this same state"; the fork falls to the operator with no rule). Supporting: `handoff: thin` ×2, `suggestion-underdetermined` ×3.
**Draft criteria line (orientation playbook):** "5b. Ladder-uncovered state — the ladder's rung named a skill whose own Setup will refuse the cursor (a BLOCKED-ON head; a half-promote it cannot resume): record `ambiguity.ladder-uncovered-state` with the annotation class and what was suggested instead."
Also a skill-body direction: rung 4 could carry an arm for a BLOCKED-ON head — name the block's clearing event, suggest the first unblocked sibling, and name wrap's route-resolve as the reorder path. This project resolved the instance by moving the entry to Epoch 6b (2026-09-04 wrap); the pipeline generalization remains.

### U2 — implement/fix-loop + wrap/gates — 3 cases → proposed type `tooling.result-not-run-stable`
**Cluster:** `2026-09-03T05:18:51Z-b` (a stale `mutants.out/` read as a clean sweep — the run summary contradicting the file counts is what prompted the check) · `2026-09-03T06:30:00Z-b` (cargo-mutants viability flipped 19/15 → 18/16 on an identical tree; "22 standing survivors" was already in four artifacts and was corrected to "21 stable, with a 22nd that flips" before commit) · `2026-09-04T20:11:43Z-d` (the sr leg's run-to-run announcement variance; a SECOND observation was needed to exclude regression).
**Draft criteria line (fix-loop and gates playbooks):** "A gate or leg over an identical tree graded differently from its prior run, or a result was read from prior-run residue: record `tooling.result-not-run-stable` naming the tool, the delta, and whether any artifact had already frozen the first number."

### U3 — wrap/route-resolve + wrap/gates — 3 cases → no new type; a record-shape clarification
**Cluster:** `2026-09-03T20:32:00Z-b` (a measurement of the recorded-pre-direction clause WORKING: three trajectory-class edits, zero halts) · `2026-09-03T20:44:00Z-b` (two expected-red gates readable only through their inline disposition) · `2026-09-05T06:52:00Z-b` (the light gate's operator-gated arm resolved by an mtime comparison instead of a re-run). All three are positive measurements, not cost.
**Draft criteria line (evolve-system, record shape):** "A positive measurement of a mechanism working is a `produced[].signals` fact on the step record (with a note) — never an untyped friction record, which carries cost." Removing these from the friction stream would drop wrap/gates' untyped rate from 50% to 17%.

### U4 — new-session/orientation + phase/take-up — 3 cases → covered by the Universal `contract.token-proxy-check`; list it in the orientation playbook
**Cluster:** `2026-09-02T22:13:09Z-c` (separator anchored `/^↓/` where the grammar writes `   ↓` — every separator labelled MARKERLESS; a LATER session recorded the same miss typed, `2026-09-04T18:27:09Z-b`) · `2026-09-03T13:17:34Z-b` (the ledger's own epoch label in two dash spellings splits every epoch tally) · `2026-09-03T15:52:57Z-d` (a `[A-Z_]*` class truncated `ANDROMEDA_PULSE_L4_DETERMINISTIC` at the digit and returned a non-existent handle).
**Draft criteria line (orientation playbook):** "3b. A structural extractor's pattern (separator, preamble marker, epoch label, handle-name character class) mismatched the artifact's documented grammar: `contract.token-proxy-check`, recording the direction (false MARKERLESS / false handle) and that the truth came from reading the hits." Producer-side fix for the second case: the record shape could require the epoch label be copied byte-exact from the route header — 27 of this epoch's 239 records carry the hyphen form.

## Below threshold — no action

Typed groups (n < 3, no halt) not merged into a proposal above — visible for the founder's eye only:
- `wrap/route-resolve/contract.carry-no-owner` n=1 (dialogue 1 · halted 1 — a moved entry's third CARRY self-located as "the entry that next touches the ui test tree", false after the move; one armed question) — a single halt sits below the n ≥ 2 clause.
- `implement/fix-loop/contract.vacuous-check-found` n=2 (`is_unmet` survived constant-`true`; `browser.refresh()` was resetting the sequential-focus start point so R0-01's green was manufactured) + `implement/smoke/contract.vacuous-check-found` n=1 (`agent-run.sh status` returned prior-run residue) — 3 across two steps.
- `recall.corpus-recurrence` — reconcile 1 + curation 2 = **3 across steps** (graph 0-indexed rule not applied; the BLOCKED-ON-and-move entry did not police the annotation written beside it; a handoff-inherited precedent not grepped — "the rule existed and the work reproduced the failure anyway"); Epoch 5 logged the same class ×3. Not a Universal type, so per-step grouping keeps it sub-threshold — the founder may want `recall.*` grouped by type alone like the Universals.
- `implement/fix-loop/tooling.environmental` n=2 (shared CARGO_TARGET_DIR → L3; CONDUCTOR_NVDA unset so all sr* suites skipped — the chunk's primary instrument could not run).
- `phase/research/tooling.output-cap-overflow` n=2 → L1.
- `wrap/gates/tooling.gate-deferral` n=2 (the light gate re-ran a 4-min mutation tier on byte-identical source because the skip valve keys on zero language delta, not on gates already green this session; the 49th audit re-pin → L4).
- `new-session/input.handoff-git-mismatch` n=2 (post-handoff boundary-stack sessions moved the ahead-count and discharged the predicted nudge — not the bookkeeping rewrite).
- `implement/code/input.plan-step-ambiguous` n=2 → X1.
- `phase/take-up/input.carry-context-gap` n=2 (a CONTEXT citing testing.md:19 where the substantive ground is :67; a proposals.md citation whose most binding constraint sits one section away).
- `wrap/report/contract.detector-fact-gap` n=2 (the Changes bullets have no home for WHICH doc bakes a value as a literal vs names a SET; the Expected-amendments list under-ran by one — caught by the template's own detector-field scan).
- `phase/distill/contract.binding-contradiction` n=1 · `phase/distill/contract.extract-format` n=1 (6 of 7 returns HTML-escaped) · `phase/distill/input.spec-source-gap` n=1 (a11y-plan:269's stale pass-spec path, faithfully inherited — master owns it).
- `wrap/reconcile/ambiguity.playbook-no-match` n=1 (ten proposals matched no rule, both misses on a QUALIFIER not a subject).
- `phase/validate/contract.structural-blind-spot` n=1 (→ P2 row 5's mechanism) · `phase/validate/input.out-of-pipeline-source` n=1 (pulse-app DOWN while the handoff said running; a bare `boot` short-circuits at the probe) · `implement/fix-loop/input.out-of-pipeline-source` n=1 (the sr subject needs a FIFTH env handle stated only in a source comment).
- `implement/fix-loop/ambiguity.scope-pressure` n=1 (criterion 2 authored when the survivor set was unmeasurable by construction; continuation-as-surface chosen).
- `implement/fix-loop/contract.spec-reality-gap` n=1 (`conductor preconditions` could not exit 0 on a correct host — `declares()` truthy-only for a PATH handle; fixed by the next chunk).
- `implement/smoke/retry.smoke-reentry` n=1 · `implement/smoke/tooling.subprocess-bounds` n=1 (a WebDriver teardown hung 120 s after the System-menu freeze; wdio's onComplete still cleaned the whole tree — census all zero).
- `phase/plan/input.research-thin` n=1 → X2/X1 (PULSE_MCP_PROGRAM's `pub(crate)` visibility — a seam fact the graph's symbol row does not carry — relocated a step).
- `phase/research/input.extract-signal-gap` n=1 → X2. `implement/code/input.conventions-gap` n=1 (cmdk's Empty spreads props BEFORE `role`, so a passed role loses).
- `wrap/gates/tooling.commit-mechanics` n=1 → L1. `wrap/route-resolve/contract.standing-pin-carriage` n=1 → L4. `new-session/tooling.health-false-red` n=1 (check 9's `*-plan.md` glob counted 4 of 6 plans — design-system.md and layout-templates.md carry no `-plan` token). `new-session/contract.token-proxy-check` n=1 → U4.

Untyped singles (Stage 2 clusters of one):
- `2026-09-02T23:08:35Z-c` research — a plan-decision fork reaches P4 with one candidate's mechanism unproven in-tree (no Rust precedent for build-at-test-time) while its rival was measured.
- `2026-09-02T23:35:15Z-b` plan — near-miss: a security extract's CONDITIONAL warning read as a verdict would have dropped the recommended option for a strictly worse one; resolved by measurement.
- `2026-09-03T04:45:15Z-c` validate + `2026-09-04T19:29:11Z-b` plan — **code-graph 0-indexed rows transcribed as 1-indexed editor lines** (mutation-tier-restored ×3 sites, operator-caught; sidecar-spawn ×3 sites, overseer-caught). Two typed records name the same slip in two MORE chunks (`2026-09-03T06:11:16Z-c` recall; `2026-09-03T07:45:47Z-a`'s plan note and `2026-09-03T08:22:01Z-b`'s two wrong coordinates) — **4 chunks in-epoch across record kinds**, sub-threshold only because the untyped count is 2. A type (e.g. `contract.coordinate-basis-mixed`) would let Stage 1 count it; the CLAUDE.md rule states the fact and did not police the transcription — emerging, watch.
- `2026-09-04T17:12:31Z-a` smoke — the retraction record (correction cost, kept in-stream by design).
- `2026-09-04T19:39:52Z-b` code — `cargo check -p conductor-verify --lib` red at HEAD (tokio `time` dev-only), pre-existing, surfaced not fixed — owned by *Dependency polish*.
- `2026-09-04T20:11:43Z-b` fix-loop — the fix's own success broke the leg: with the pane gone, the synthetic ALT opened the app's System menu and froze the webview; the plan's conditional step-9 retire was the remedy. (Curation rejected the learning at exactly 0.6 — P12.)
- `2026-09-04T20:37:43Z-c` report — a template bullet family ("Insufficient fixes — written, kept, not the remedy") exercised for the first time; without it the fact would have mis-routed its owner.
- `2026-09-05T06:44:42Z-b` route-resolve — a trajectory halt's hidden consequence: minting the operator's option (b) in Epoch 6a would have REOPENED a completed epoch and deferred this diagnosis; found by counting the tail per epoch header, not from either option's text.

Problem-fact themes below threshold (Stage 4 Pass A): C — a read-only research step needed a MEASUREMENT (live CARGO_BIN_EXE probes; the full 12-min mutation tier run at P3) n=2, no prior-epoch match, emerging · F — a 10-minute foreground sleep-poll for a background agent against the standing do-not-poll instruction n=1 · G — a 4-site code refactor as one scripted write instead of anchored Edits n=1 · I — a P5-detected plan defect fixed by targeted plan.md edits instead of the documented P4 return n=1 · J — CARGO_HOME relocated off the default n=1 · K/L — two prohibitions on two different obstacles (a ledger do-not; a scope guard against retiring a workaround on a prediction) n=1 each · M — an extract return carried absolute host paths into the non-gitignored run dir n=1 · N — the operator corrected a factual clause in implement's ledger before commit n=1 · the one `deferred` fact CLOSED (L4) · the one `overridden` fact (`2026-09-03T11:04:42Z-a#0`, scope.md's carry-as-is overridden to attempt a kill — it landed) n=1; cross-epoch override counts E1 3 · E2 8 · E3 6 · E4 2 · E5 11 · E6a 1 are an observation only.

The document ends the mechanism's responsibility. Nothing above is applied, queued or remembered; the founder takes it from here.
