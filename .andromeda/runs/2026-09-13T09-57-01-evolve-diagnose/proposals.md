# Evolve Diagnosis — Conductor · Epoch 6b — Polish & ship · 2026-09-13T09:57:01Z

Target epoch resolved in dialogue; **fully complete** (14 frozen working-route entries, 0 markerless, all master records `complete`). Read-only pass; the only writes are this file and its `q-*.json` twins.

## Mechanism health

**Records:** 456 in-epoch (194 step / 262 friction) across 14 chunks. **Unparseable:** 0 over the whole 1963-record ledger. **id fill:** 456/456 (all post-boundary). **problem-fact fill:** 69/194 step records carry at least one deviation fact.

**Retractions (whole-ledger pre-pass):** 11 friction records retracted, 2 problem-fact targets; **9 of the retracted ids fall inside this epoch and were dropped before every stage below**. 1 unresolvable, reported verbatim for manual discount:
- `id: None` · prose-form-pre-boundary · “the untyped code-graph-under-reports record and the first problem-block entry on”
- **Retraction targeted by a retraction → founder review:** `2026-09-10T19:52:10Z-a`.

**Checkpoint coverage** — step records present vs the expected playbook set (phase 5 · implement 3 · wrap 5). 10 checkpoint firings are missing across 4 chunks; each was confirmed absent from the WHOLE ledger, not merely from this epoch's slice (i.e. not an epoch-label boundary artifact):

| chunk | present | missing |
|---|---|---|
| `2026-09-05-audit-corrective` | phase 5 · implement 2 · **wrap 0** | `implement/code`, and every wrap checkpoint |
| `2026-09-06-operator-gated-live-suite` | phase 5 · implement 3 · wrap 1 | `wrap/curation`, `wrap/reconcile`, `wrap/report`, `wrap/route-resolve` |
| `2026-09-07-a11y-ci-gate` | phase 5 · implement 2 · wrap 4 | `implement/code`, `wrap/gates` |
| `2026-09-07-sr-findings-fixed` | phase 4 · implement 3 · wrap 4 | `phase/plan`, `wrap/gates` |
| `chunk: null` | new-session 1 · wrap 2 | — (a no-op/adaptation wrap legitimately carries `curation` + `route-resolve`) |

**Untyped rate:** 27/262 = 10.3% overall. Per step: `phase/validate` 4 · `new-session/orientation` 4 · `implement/fix-loop` 4 · `phase/research` 3 · `implement/code` 3 · `wrap-session/route-resolve` 2 · `wrap-session/gates` 2 · `implement/smoke` 1 · `implement/None` 1 · `wrap-session/curation` 1 · `phase/take-up` 1 · `phase/distill` 1.

**Outcomes:** `ok` 182 · `halted-resolved` 6 · `soft-exit` 1 · `ok-degraded` 5. All 6 `halted-resolved` sit at wrap (`reconcile` ×4, `route-resolve` ×2); all 5 `ok-degraded` and the 1 `soft-exit` sit at `implement/fix-loop` (see L5).

**Calibration boundaries in range** — all Universal types were live for the whole epoch (earliest 2026-08-15, latest `contract.skill-reference-drift` 2026-09-02), so no absence here is an era artifact. Two observations:
- **`contract.grammar-irregularity` has never been used, ledger-wide** (deployed 2026-09-09, live for this epoch's final 2 chunks). Whether no tool printed `UNPARSED:`/`INDETERMINATE:` or the type was never reached for is not separable from the ledger.
- **`contract.token-proxy-check` was used once in Epoch 6a and 24 times here**, having been live since 2026-08-31. The jump mixes uptake with incidence; P3 states this rather than resolving it.

**One elision in the quoted evidence** — a URL scheme appearing inside a quoted ledger record is rendered `x[:]//` rather than verbatim. The scheme's `letter + colon + slash` run satisfies this project's host-path anchor `[A-Za-z]:[\/]` though it is not a path, and this run dir rides the next wrap's commit; per `host-win32.md` (2026-09-11, as extended 2026-09-12) the fix is to change the OUTPUT, never to loosen the anchor. One cell is affected, in P5.

**Epoch-label spelling** — this epoch's records carry two spellings (`Epoch 6b — Polish & ship` ×427 and `### Epoch 6b — Polish & ship` ×38, the working-route header copied with its heading prefix). Folded before every group-by per the tolerant-parsing rule; a diagnosis that grouped by exact label would have under-counted by 38. One of this epoch's own `new-session` records predicted exactly this.

## Proposals (typed patterns)

60 groups; **24 above the F-2 threshold** (n ≥ 3, or n ≥ 2 with halt/soft-exit impact). Universal and `recall.*` types are grouped by type across steps, per the diagnosis-pass rule. Ordered by weight. Every proposal carries ALL its cases.

### P1 — all steps · `contract.narrow-basis-claim` — 27 cases · weight 43

**Pattern:** A count, absence or availability claim was stated from a source narrower or less authoritative than the claim itself, in **12 of the epoch's 14 chunks** — the widest-spread and highest-weight class in the ledger. Rate per step-record roughly tripled against the prior epoch (5/106 = 0.047 at Epoch 6a → 27/194 = 0.139 here). Whether that is more errors or better detection is not separable from the ledger alone: the type has existed since 2026-08-15, so the rise is not a deploy artifact, and the project curated three separate sweep-discipline rules during this epoch.

**Evidence:** all 27 cases · 12 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `—` | `new-session/orientation` | the standing cargo-audit deferral (51 re-pins) rests on 'advisory-DATABASE fault — no released tool can read it', derived only from runs against the SAME local cache; CI's fre… | extra_reads 5 |
| `2026-09-05-audit-corrective` | `implement/fix-loop` | the file-scoped obs.rs mutation run covers 203 mutants where the audit's --shard 1/4 reported 126, so it surfaced 7 obs.rs survivors the audit never named (install_panic_hook,… | extra_reads 1 |
| `2026-09-05-audit-corrective` | `wrap-session/curation` | the block-8 evidence-hygiene grep was typed vacuous because its subject was a file this run wrote, but it matched a REAL resolved $CARGO_HOME expansion in the ledger's audit t… | — |
| `2026-09-06-operator-gated-live-suite` | `phase/validate` | A universal quantifier over the declare-only family was accepted and written into a plan step, a rejected-approach line and a scope amendment without enumerating the family's … | iterations 1, dialogue_rounds 1 |
| `—` | `wrap-session/route-resolve` | A cause was assigned from a single failing run without checking whether the SUT's own state changed between runs. The prior wrap saw one non-reproduction, computed 165s agains… | iterations 1, dialogue_rounds 1 |
| `2026-09-06-run-report-envelope-conformance-gate` | `wrap-session/report` | The plan's Expected amendment E3 routed 'the conductor CLI gains a cleanup verb' to architecture.md section Occupied Resources. Grepping the target before proposing showed arc… | extra_reads 2 |
| `2026-09-06-run-report-envelope-conformance-gate` | `wrap-session/reconcile` | The report's expected-amendment E4 located the status-contract sites with grep on '**`status`**' (bolded), reporting 1 hit at test-plan:158. The tests detector found a SECOND … | — |
| `2026-09-07-dependency-polish` | `phase/validate` | I asserted that security-plan section Dependency Security forbids leaving an advisory ignore whose subject a bump removed, and cited the master for it in four places: plan ste… | extra_reads 3 |
| `2026-09-07-dependency-polish` | `phase/validate` | Three separate plan defects the operator caught, all one class: a claim asserted from a partial read of an artifact I already had open. (1) The step-7 sweep ran cargo check -p… | extra_reads 3 |
| `2026-09-07-a11y-ci-gate` | `phase/research` | the P2 check-C record asserted a contradiction BETWEEN MASTERS on the violation-JSON shape, basing that on the two extracts alone without grepping either master. Grepping both… | extra_reads 2 |
| `2026-09-07-a11y-ci-gate` | `phase/validate` | the P3 report and plan step 6 stated the routine spec has 13 it() blocks and TWO context-skip sites; the truth is 12 blocks and FOUR this.skip() sites (accessibility.e2e.ts :2… | extra_reads 1 |
| `2026-09-07-a11y-ci-gate` | `wrap-session/report` | the implement-stage record concluded mocha prints no skip summary line, on the basis of a grep for the token 'pending' returning zero. The reporter's actual wording is 'skippe… | extra_reads 1 |
| `2026-09-07-sr-findings-fixed` | `implement/fix-loop` | my isolation POST omitted browserName='webview2', the capability that selects msedgedriver's WebView2-host launch mode; the operator measured the same POST failing identically… | iterations 1, dialogue_rounds 1 |
| `2026-09-07-sr-findings-fixed` | `wrap-session/reconcile` | security-plan's detector proposed 4 amendments recording 'a SECOND, CI-only reader' of CONDUCTOR_MSEDGEDRIVER; ci.yml WRITES the handle at :300 and never reads it, so wdio.con… | dialogue_rounds 1 |
| `2026-09-08-hosted-runner-webview2-session` | `wrap-session/report` | My first report draft asserted three counts from memory and all three were wrong when re-derived before the fan-out: (1) 'no other master states a driver-version pairing' — a1… | extra_reads 4 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/validate` | a P5 corpus count reported '7 defaulted / 0 bare' CARGO_HOME forms and the 0 reached a committed artifact (entry 1's note) as an absence claim. Wrong by two independent mechan… | dialogue_rounds 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `wrap-session/report` | the operator directive stated the red gate basis as `cargo test -p conductor-faults is named by no prior plan or report in either version (the gate prior history covers conduc… | extra_reads 2 |
| `2026-09-09-port-occupier-test-hygiene` | `phase/research` | I stated the graph trace file carries no db_state field, on the basis of one failed accessor (calling .keys() on a JSON list) - in the SAME command whose grep had already foun… | extra_reads 1 |
| `2026-09-09-port-occupier-test-hygiene` | `phase/validate` | the plan's sweep claimed to close the hiding-a-violation hole while enumerating only the EIGHT crates the route entry named (the crates the formatting pass touched); the works… | extra_reads 2, iterations 1 |
| `2026-09-09-port-occupier-test-hygiene` | `phase/validate` | two factual claims in plan.md and research.md were stated more specifically than the evidence gathered for them. (a) 'six construct a PortOccupier, and the seventh reads the l… | extra_reads 2 |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/report` | both corrected measurements were claims stated more precisely than the basis I had gathered. (a) 'the red gate was masking two further test targets' - I counted the targets a … | extra_reads 3 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/plan` | scope.md and research.md both framed the P-079 span_refs finding as NEW - research titled it 'the load-bearing equality ... MEASURED, and it decides P-079' and called it 'stro… | iterations 1, extra_reads 3 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/validate` | A master citation was MIS-AIMED by six lines and asserted twice (plan.md Constraints and the scope amendment): the plan claimed architecture.md:63 ratifies the accepted gradin… | iterations 1, extra_reads 3 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/validate` | A stated number shipped without its derivation and was then restated ambiguously. plan.md wrote the class basis as 'constellation-severity-live-wiring floor 1 . findings-count… | extra_reads 1 |
| `2026-09-10-release-build-and-bundle` | `phase/plan` | An audit over all 36 scenario TOMLs found 17 declaring an SLO tier smaller than their own summed gap_ms, and the first reading was that 17 scenarios are defective — a claim de… | extra_reads 2 |
| `2026-09-10-release-build-and-bundle` | `implement/fix-loop` | The three live legs were selected at P4 on TIER ATTAINABILITY alone — summed gap_ms against the tier deadline — which is a basis narrower than the claim the legs were chosen t… | — |
| `2026-09-10-release-build-and-bundle` | `implement/fix-loop` | The preceding record's ts/id was hand-written from a remembered earlier value instead of being read from `date -u +%FT%TZ` first — the clock was printed in the same call but A… | — |

**Proposal:** The class is already curated as a project T1 rule and recurred anyway (see P5 and L4). A corpus entry is a recall mechanism; the direction worth considering is an **affordance at the point of claim** — a plan/report field that requires the derivation command beside any count, absence, or 'every/no' quantifier, so the basis travels with the number instead of being reconstructible only by re-deriving it.

### P2 — all steps · `contract.premise-falsified` — 22 cases · weight 35

**Pattern:** Verification falsified a premise an authored artifact states, in **13 of 14 chunks** — the most evenly distributed class in the epoch. Rate per step-record is flat against the prior epoch (13/106 = 0.123 → 22/194 = 0.113), unlike the two classes above it.

**Evidence:** all 22 cases · 13 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `—` | `wrap-session/route-resolve` | the directive's dictated basis said `cargo audit` now exits 0 with 'one unmaintained warning'; measured: exit 0, `warning: 18 allowed warnings found` (17 unmaintained + 1 unso… | — |
| `2026-09-05-audit-corrective` | `phase/research` | the route freight (and the scope that folded it) named ONE test-plan §12 coordinate moving with the split (`lib.rs:362:5`); §12 :609 + :611 carry FOUR groups inside lib.rs (:6… | extra_reads 1 |
| `2026-09-06-operator-gated-live-suite` | `phase/validate` | A Test Command was labelled a hermetic scenario-load check while its hermeticity depended entirely on an environment the plan never controlled. The reasoning cited was real bu… | dialogue_rounds 1 |
| `2026-09-06-run-report-envelope-conformance-gate` | `phase/research` | scope.md (and, following it, four extracts) stated the gate would mirror conductor-core::redact::is_host_path_token as the single source of the host-path notion. That function… | extra_reads 1 |
| `2026-09-06-run-report-envelope-conformance-gate` | `phase/validate` | plan.md step 1 stated as load-bearing that a successful typed parse is simultaneously the shape verdict and the key-presence check, and three acceptance criteria rested on it.… | dialogue_rounds 1, iterations 1 |
| `2026-09-06-coverage-completeness-gate` | `implement/code` | The CARRY's claim that a Latency/Ramp phase puts `samples`/`windows` spans on the wire per dispatch rode unchallenged into scope and into plan step 6, and is wrong: rate_trace… | extra_reads 4, reformulations 1 |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/research` | Scope 1.1 asserted the conductor / conductor-canary service-identity split as the discriminator that would settle hue-sample attribution; the SUT's allowlist for that leaf is … | — |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/validate` | The plan's central mechanism - that a sub-2s dispatch interval keeps last_seen fresh at the tier flip - was false, and P3 did not catch it: research re-derived the fire site's… | iterations 1, dialogue_rounds 1 |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/gates` | Four amendments this same wrap authored at P2 claimed the stretched bootstrap posture makes the AutoResolved arm reachable INDEPENDENT of uptime - a sufficiency claim the P2 m… | — |
| `2026-09-07-dependency-polish` | `phase/research` | The chunk's second headline item is built on a goal that measurement shows is unattainable, and the goal is stated by a SPEC MASTER, not only by the folded CARRY. obs-plan sec… | extra_reads 4 |
| `2026-09-07-dependency-polish` | `phase/research` | An acceptance criterion an extract authored is already false at HEAD, so P4 cannot adopt it verbatim. The security extract contributed: no sk_live_ / sk-proj- / ghp_ / AKIA li… | extra_reads 1 |
| `—` | `new-session/orientation` | the A11y CI gate entry's BLOCKED-ON premise states origin/build/conductor-0.2.0 at dd15dc3 with the local branch 49 commits ahead and unpushed, CI not run in a month; measured… | extra_reads 1 |
| `2026-09-07-a11y-ci-gate` | `phase/take-up` | the taken-up head's BLOCKED-ON annotation states origin at dd15dc3 with the branch 49 commits ahead and unpushed and CI not run in a month; all three clauses are dead at promo… | extra_reads 2 |
| `—` | `new-session/orientation` | the route CARRY at working-route.md:123 names the WebView2 Evergreen runtime on the windows-2025 image as the first suspect for a red a11y job, asserting the Edge Driver itsel… | extra_reads 2 |
| `2026-09-07-sr-findings-fixed` | `phase/research` | the folded CARRY states the sequential-focus start point is held by 'the picker input at mount' (echoing screen-reader.e2e.ts:80-83); re-derived at HEAD the attribution is uns… | extra_reads 3 |
| `2026-09-07-sr-findings-fixed` | `implement/code` | plan step 6 (fix the sequential-focus start point) became a NO-OP: the step-5 instrument measured firstTabTarget 'Minimize window', firstTabProbe focusableIndex 0/5 and tabsTo… | — |
| `2026-09-07-sr-findings-fixed` | `implement/fix-loop` | a third probe measured every remaining candidate FALSE: runtime present, driver/runtime/Edge all 151.0.4129.101, app-alone stays up on the runner with three msedgewebview2.exe… | iterations 3 |
| `2026-09-08-hosted-runner-webview2-session` | `phase/research` | scope.md's inferred bullet 4 framed hypothesis 3 as a steerability question (can the runner's msedgedriver version be changed); research falsified the causal premise underneat… | — |
| `2026-09-08-hosted-runner-webview2-session` | `phase/validate` | The plan's whole remedy rested on WEBVIEW2_USER_DATA_FOLDER being a lever; the operator's dev-host measurement retired it. MECHANISM of the false conclusion: I traced the hand… | iterations 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/take-up` | the working entry's CONTEXT states 282 fmt sites across '~40 files'; re-measuring at HEAD gives 282 sites across 60 distinct files (30 src, 30 tests), so the entry's own revie… | extra_reads 2 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/take-up` | The phase directive stated a UNIVERSAL over the three in-lane scenarios - "each read-back sees its own canary still open and the honest outcome is a graded MANUAL row ... That… | extra_reads 1 |
| `2026-09-10-release-build-and-bundle` | `phase/take-up` | CARRY C names the 'nvda_named_window: false correlation' as the thread to pull first for the SR announcement variance; re-verifying its own cited evidence at fold time shows t… | extra_reads 2 |

**Proposal:** Premise falsification is verification working, so the count is not itself a defect signal; the flat rate across two epochs supports reading it as the pipeline's steady state. The actionable half is **where the premise entered**: none of these records names the authoring step that introduced the falsified premise. A provenance marker on authored premises (which step wrote it, from what source) would turn a falsification into a targeted signal about one authoring surface rather than a generic one.

### P3 — all steps · `contract.token-proxy-check` — 24 cases · weight 34

**Pattern:** A check, guard or derived condition tested for a TOKEN's presence where the intended property is semantic, and the match was satisfied by incidental, self-referential or prose-about-the-token content — **24 cases across 12 chunks**. The type was deployed 2026-08-31 and was used **once** in Epoch 6a before **24** times here, so the jump is uptake plus incidence together and the two cannot be separated from the ledger.

**Evidence:** all 24 cases · 12 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-05-audit-corrective` | `phase/distill` | the per-extract anchor check (fan-out check 3) keyed on `- ` bullets; the obs extract's Constraints are a numbered list, so the check found zero bullets and passed VACUOUSLY (… | retries 1 |
| `2026-09-05-audit-corrective` | `phase/validate` | the orchestrator's P5 checker read `# ` shell comments inside the Test Commands fence as markdown headings, so check 4a saw an EMPTY section and reported a false FAIL (unit/sm… | retries 1 |
| `2026-09-06-operator-gated-live-suite` | `phase/validate` | A plan Test Command tested for a TOKEN where the intended property was semantic, in the false-POSITIVE direction: `conductor run ... coverage \| grep -E 'P-022'` was authored … | extra_reads 2 |
| `—` | `wrap-session/route-resolve` | Verifying the directive's cited timing nearly produced a false contradiction. A bare grep for service_went_silent over Pulse's log returns 7632 hits with the FIRST at 09:02:03… | extra_reads 3 |
| `2026-09-06-run-report-envelope-conformance-gate` | `phase/take-up` | Verifying the CARRY's 'gated by valid_run_id' clause, grepped agent-run.ps1 for a guessed token (Test-ValidRunId) rather than for the practice; the zero hits read momentarily … | extra_reads 1 |
| `2026-09-06-coverage-completeness-gate` | `phase/validate` | TWICE in one step, both false POSITIVES, both resolved only by reading what the pattern would actually match. (1) In the plan: Test Command 4 asserted the obs emission with `g… | reformulations 2 |
| `2026-09-06-coverage-completeness-gate` | `implement/fix-loop` | Reading the --e2e leg's asserted signal, a grep for 'passing\|failing\|skipped\|✓' over the run log matched the axe-core library source the harness INJECTS into the page (1.3 … | reformulations 1 |
| `—` | `new-session/orientation` | My working-route structural extractor keyed MARKERLESS on 'line does not start with [' and classified every '   ↓' separator as a markerless entry - a FALSE MARKERLESS in both… | — |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/report` | A FALSE NEGATIVE while locating an Expected-amendment's sites: `grep -rcE 'B1 (to\|arrow) B2'` returned 0 across all three masters, which would have dispositioned the --live c… | extra_reads 2 |
| `—` | `new-session/orientation` | My structural extractor for the working-route tail classified every intra-epoch separator as an ENTRY: the awk non-entry character class was anchored at ^ but the route gramma… | — |
| `2026-09-07-dependency-polish` | `phase/take-up` | Verifying CARRY 6's rustdoc link targets, my first grep looked for the bare bracketed names the CARRY itself writes (RunRecord, persist, canary_gate) and returned ONE hit that… | extra_reads 2 |
| `2026-09-07-dependency-polish` | `implement/fix-loop` | Verifying step 8's predicted deny.toml count, my probe grepped unique RUSTSEC ids inside the advisories block and returned 17 where 16 was predicted — reading as if the remova… | extra_reads 1 |
| `2026-09-07-a11y-ci-gate` | `phase/distill` | the per-extract in-domain check (fan-out.md check 4) flagged the security extract on its literal marker axe-core; reading the single hit showed it sits in a dependency ROSTER … | extra_reads 1 |
| `2026-09-07-sr-findings-fixed` | `phase/research` | grepping '\.focus()' against crates/conductor-tauri/ui/node_modules/cmdk/dist/index.mjs returned a single 40KB+ minified bundle line - vendored third-party source, the documen… | extra_reads 1 |
| `2026-09-07-sr-findings-fixed` | `implement/fix-loop` | the decisive line 'session not created: DevToolsActivePort file doesn't exist' was present in BOTH earlier probe logs and I missed it twice, because I grepped for ERROR-level … | iterations 1 |
| `2026-09-07-sr-findings-fixed` | `wrap-session/reconcile` | my report claimed no master baked the moved routine-arm count, on a probe keyed on '10 spec\|ten spec\|12 spec' returning 0 hits across the seven masters; test-plan:307 spells… | extra_reads 1 |
| `—` | `new-session/orientation` | The BLOCKED-ON probe on the first markerless entry was a bare token grep rather than one keyed on the annotation grammar, so it returned 2 hits from backtick-quoted prose late… | extra_reads 1 |
| `2026-09-08-hosted-runner-webview2-session` | `phase/take-up` | Positive direction, recorded because the same check false-HALTed a promotion before: the head entry's two BLOCKED-ON tokens were located by character offset (2793 and 2958 of … | — |
| `—` | `new-session/orientation` | markerless-entry extractor anchored the separator as ^↓ while working-route.md indents separators four spaces (its own header documents '   ↓'); direction was a false MARKERLE… | — |
| `—` | `new-session/orientation` | health check 14's pointer-table extractor was authored with a guessed closing-marker pattern ('/GENERATED:...' or 'END GENERATED') and printed BLOCK NOT FOUND — a false FAIL d… | extra_reads 1, retries 1 |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/reconcile` | the mandated entities=0 probe on my own consolidated fan-out artifact returned 2, and the hits were my own prose: I had written the HTML-escape sequences literally in order to… | retries 1 |
| `—` | `new-session/orientation` | Two structural extractions over master-route.md mismatched its documented {marker} - {status} - grammar in one call, in OPPOSITE directions: a pipe-table status-tally regex re… | reformulations 1, extra_reads 1 |
| `—` | `new-session/orientation` | Check 14's first extractor invented a closing-marker grammar (/GENERATED: and END:GENERATED) that the section-markers convention does not use — it closes with '<!-- GENERATED:… | extra_reads 1, retries 1 |
| `2026-09-10-release-build-and-bundle` | `phase/take-up` | The inherited CARRY count (4, from the handoff and from a bare token count at session start) over-counted the entry's annotations: applying the annotation-position grammar (to… | extra_reads 1 |

**Proposal:** Four of the epoch's six `prohibition` problem-facts are hand-written do-not-re-derive-this-with-a-bare-grep guards, each authored into ONE chunk's `scope.md` / `research.md` / `plan.md` (see L6). The knowledge is real, correct, and local: the next chunk does not see it. The direction is a **shared token-trap registry** — a place where 'this token is not that property, here is the pattern that misleads' survives the chunk that discovered it.

### P4 — wrap-session/reconcile · `ambiguity.playbook-no-match` — 7 cases · weight 28 · **3 halt/soft-exit**

**Pattern:** In **7 of 14 chunks** a drift proposal escalated to the operator because no playbook rule governed its class — carrying **3 halts and 6 dialogue rounds**, the only above-threshold group in the epoch bearing halt impact. The near-misses are strikingly uniform: in six of the seven the record says a rule SUBJECT-matched and then failed a narrowing qualifier (`:109`, `:58`, `:115`, `:137`, `:127` are each named as the near-miss).

**Evidence:** all 7 cases · 7 chunks · rate 0.58 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `wrap-session/reconcile` | 11 of 16 proposals escalated, in 5 units, none by a clean rule match. Two playbook rules named the subject but failed a qualifier and so did not govern: :109 dismisses the cli… | dialogue_rounds 1, halted 1 |
| `2026-09-06-coverage-completeness-gate` | `wrap-session/reconcile` | Two escalations reached the operator because no playbook rule governed. (1) D-security-input recording a new TEST-BINARY reader of pulse-capabilities.toml: rule :58 subject-ma… | dialogue_rounds 1 |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/reconcile` | arch's proposal pair (register ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS + qualify the uptime bound) matched no playbook rule, and the unease was precedent-shaped: arch's thr… | dialogue_rounds 1, halted 1 |
| `2026-09-07-a11y-ci-gate` | `wrap-session/reconcile` | the new CONDUCTOR_A11Y_STRICT handle SUBJECT-matched two playbook rules and satisfied neither's narrowing clauses, so it escalated as a fourth handle class. Rule :115 requires… | dialogue_rounds 1, halted 1 |
| `2026-09-08-hosted-runner-webview2-session` | `wrap-session/reconcile` | The arch proposal's class — an external env handle a SHIPPED artifact SETS but never READS — was governed by no rule among 43. The two nearest failed in opposite directions: :… | dialogue_rounds 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `wrap-session/reconcile` | Class C (retiring a master's own explicitly-provisional claim once the measurement it names as its precondition lands) matched no playbook rule across 4 sites in 2 masters; es… | dialogue_rounds 1 |
| `2026-09-10-release-build-and-bundle` | `wrap-session/reconcile` | Eight of ten proposals retired a MEASURED-SCALAR literal — a bundler version and an installer size — and no playbook rule governs that class. The three near-misses each fail a… | — |

**Proposal:** The escalations are not noise — each reached the operator and resolved — but a 50 %-of-wraps escalation rate on one step is a rule-set shape signal, not a project signal. Two directions: a **qualifier review** over the 43 rules whose narrowing clauses are producing the near-misses these records name site-by-site, or an explicit **residual-class rule** that governs 'subject matched, qualifier missed' so the escalation carries its own classification instead of arriving unclassified.

### P5 — all steps · `recall.corpus-recurrence` — 12 cases · weight 17

**Pattern:** A curated learning that states the rule CORRECTLY did not prevent its own recurrence — **12 times across 11 of 14 chunks**. The records are explicit about it: 'a curated T1 entry states the exact rule I then broke, twice in one wrap'; 'testing.md:86 already stated the rule, dated the SAME day, and the failure reproduced anyway'; 'the read-the-hits / narrow-basis corpus entry did not prevent its own recurrence'.

**Evidence:** all 12 cases · 11 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `wrap-session/curation` | The token-grep candidate deduped against the T1 entry minted 2026-08-22 (a count built by pattern-matching is bounded by the author's imagination of how a claim can be phrased… | — |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/curation` | The chunk's central premise failure - building on a fire site's arithmetic without sweeping for the WRITER of the field it reads - is ALREADY curated as a T1 rule ('trace the … | — |
| `2026-09-07-dependency-polish` | `wrap-session/reconcile` | A curated T1 entry states the exact rule I then broke, twice in one wrap. CLAUDE.md's session-learnings carry: when retiring a claim, sweep for what the claim SAYS, not what i… | extra_reads 2 |
| `2026-09-07-a11y-ci-gate` | `wrap-session/curation` | the session reproduced the exact failure a correct T1 entry already describes. The entry states that a proxy token under-counts a claim it merely accompanies and that a count … | extra_reads 2 |
| `2026-09-07-sr-findings-fixed` | `wrap-session/curation` | testing.md:86 already stated the nextest-filter-matches-nothing rule, dated the SAME day, and the failure reproduced anyway: a plan Test Command used a bare positional argumen… | retries 1 |
| `—` | `wrap-session/curation` | A hooks smoke test was authored as a probe and run cold; it false-reported all five arms failing because the shell it spawned lacked the host tool, and only then was the probe… | retries 1 |
| `2026-09-08-hosted-runner-webview2-session` | `wrap-session/curation` | The operator's directive item 5 asked that the host-path hygiene grep gain a left boundary and a :// exclusion, because the bare [A-Za-z]:[/] matched p[:]// in URLs and produc… | — |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `wrap-session/reconcile` | the curated rule says a non-empty grep result is evidence to READ, not a verdict - and I had the hits and misread them. A sweep of a11y-plan for runtime/driver literals return… | dialogue_rounds 1, extra_reads 2 |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/curation` | verification-harness.md already states that a no-boot-path-change chunk's smoke is `agent-run.sh run` and explicitly NOT `status`, and my P4 plan listed `status` anyway. The e… | — |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/curation` | the read-the-hits / narrow-basis corpus entry in CLAUDE.md did not prevent its own recurrence: two claims in my implement report were stated more precisely than their basis su… | — |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `wrap-session/curation` | TWO findings this session deduped against corpus entries that state their rule CORRECTLY, and the work reproduced the failure anyway. (a) A bare `grep -c 'service_went_silent'… | extra_reads 2 |
| `2026-09-10-release-build-and-bundle` | `wrap-session/route-resolve` | A one-off python patch script used str.replace with no post-check and printed its own success line while matching nothing, so the intended fix never landed and the next run fa… | retries 1 |

**Proposal:** Curation is the pipeline's primary improvement mechanism, and this group is the direct measurement of its transfer rate: the corpus is correct and is not reaching the moment of the act. The direction is **recall at the point of use rather than at the point of capture** — surfacing the governing entry when a step is about to perform the act it governs (a sweep, a count, a smoke-command choice), instead of relying on the whole corpus being in context. Note the diagnosis cannot judge whether the fix is worth its cost; the measurement is that 11 of 14 chunks paid the recurrence.

### P6 — all steps · `tooling.host-shell` — 8 cases · weight 15

**Pattern:** The host shell's semantics, quoting or encoding corrupted a command or its output — 8 cases across 6 chunks. Rate is **flat** against prior epochs (0.038 at 6a → 0.041 here), and every case is a mechanism `host-win32.md` either already documents or gained an entry for.

**Evidence:** all 8 cases · 6 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-operator-gated-live-suite` | `phase/take-up` | rev is absent from this MSYS host's coreutils (bash: rev: command not found), so a tail-of-line probe printed two error lines instead of the line ending; answered in the same … | — |
| `2026-09-07-sr-findings-fixed` | `implement/fix-loop` | the first scrub of the evidence log ran as an inline python -c carrying Windows path literals; the backslash escaping was mangled by the shell so str.replace no-matched, and t… | retries 1 |
| `—` | `new-session/orientation` | Two Bash calls issued in one message both began with a relative 'cd conductor-0.2.0'; cwd persisted from the first, so the second's cd failed and its half-promote and Epoch-6b… | retries 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/take-up` | parsing rustfmt's 'Diff in <path>:<line>:' output failed twice in the shell - sed read the Windows backslash path as an unterminated s-command, then tr warned on a trailing un… | retries 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/research` | two token probes used an unescaped '(' under grep -E ('garde(' and 'Fixed('), which ERE reads as an unterminated group, so both returned a confident 0 against a tree that hold… | retries 1, extra_reads 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `wrap-session/reconcile` | two verification greps in the post-apply pass used a pattern with a backslash-plus under BASIC regex, where it means one-or-more rather than a literal plus, so both returned a… | retries 1 |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/gates` | a light gate run through a Windows-native python subprocess reported ALL NINE entries red at exit 1, including `git diff --quiet` and `cargo fmt --all --check` - a uniformity … | retries 1, extra_reads 2 |
| `2026-09-10-release-build-and-bundle` | `phase/distill` | The scripted entity-decode was first authored as a cat heredoc writing the script to a scratchpad path; the project's PreToolUse guard blocked it ('documents go through the Wr… | retries 1 |

**Proposal:** A flat rate under a growing corpus of host rules suggests the rules are catching the class after the fact rather than preventing it. No pipeline change is obviously indicated; the founder may prefer to read this as the steady cost of the host and leave it. Listed because it clears threshold, not because it has an evident remedy.

### P7 — all steps · `tooling.output-cap-overflow` — 10 cases · weight 12

**Pattern:** A read exceeded the tool-result size cap and was recovered by chunked or structural re-extraction — 10 cases across 6 chunks, up 2.7× in rate on the prior epoch (0.019 → 0.052). **Every case names the same root artifact shape**: `master-route.md` records and `.claude/rules/verification-harness.md` entries stored as multi-KB single lines. One record marks a same-day recurrence on one artifact; another marks a third occurrence in a single session.

**Evidence:** all 10 cases · 6 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-operator-gated-live-suite` | `phase/research` | The mandated in-full read of the harness rule files overflowed the tool-result cap on the first attempt — .claude/rules/verification-harness.md alone returned 51.7 KB and a se… | extra_reads 5 |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/research` | A prescribed in-full read of .claude/rules/verification-harness.md returned 56.2 KB from a 60-line file and was persisted to a tool-results file instead of my context; recover… | extra_reads 3 |
| `2026-09-07-dependency-polish` | `phase/research` | A grep -rn for secret-shaped literals across crates/ blew the tool-result cap at 1.6 MB because the untracked node_modules tree contains base64 WASM payloads and a binary font… | extra_reads 1 |
| `—` | `new-session/orientation` | Grepping master-route.md for the active version's records returned 29.7KB and was truncated to a persisted file; recovered by re-extracting only the 'marker · status' prefixes… | extra_reads 1 |
| `2026-09-08-hosted-runner-webview2-session` | `phase/research` | Indexing the 63.7KB verification-harness.md returned 60.4KB and was truncated to a persisted file: the per-entry-introducer patterns matched whole multi-KB prose lines rather … | extra_reads 1 |
| `—` | `new-session/orientation` | grep over master-route's conductor-0.2.0 range printed full record lines, each multi-KB, and blew the tool-result cap at 38.5 KB; recovered by a scoped grep -oE returning only… | extra_reads 1 |
| `—` | `new-session/orientation` | RECURRENCE, same artifact and same day: a grep over master-route's conductor-0.2.0 range returned full record lines and blew the tool-result cap at 38.6 KB, recovered by a sco… | extra_reads 1 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `phase/research` | THIRD occurrence of one mechanism in a single session, and the first where the governing doc had already prescribed the fix. codebase-research.md says an over-cap rule file's … | extra_reads 1 |
| `—` | `new-session/orientation` | grep -nE '(complete\|pending\|·)' over master-route.md matched whole route records (each multi-KB) and returned 31.6KB, exceeding the tool-result cap and persisting to a file;… | extra_reads 1, retries 1 |
| `—` | `new-session/orientation` | The master-route status grep returned whole records and overflowed the tool-result cap at 40.8KB (persisted to a file), because 0.2.0's records are multi-KB single-line entrie… | extra_reads 1, retries 1 |

**Proposal:** This is the read-side face of L1 and shares its cause. The chunk-local remedy (structural re-extraction) is already prescribed and works; what recurs is that every reader must discover the need for it. Direction: see L1 — the proposal is about the artifact format, not about the readers.

### P8 — phase/validate · `contract.mechanical-check` — 6 cases · weight 12

**Pattern:** The P5 mechanical checks did not cover a plan defect the operator's review then caught, or fired by construction on a chunk shape they do not fit — 6 cases across 5 chunks. Three further phase/validate records carry the same shape UNTYPED (see U2), taking the class to 9 in-epoch.

**Evidence:** all 6 cases · 5 chunks · rate 0.43 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-05-audit-corrective` | `phase/validate` | the operator's P5 review returned four Test-Command defects outside every mechanical predicate: a false-green probe form (`test -z` over the porcelain of a MISSING directory p… | dialogue_rounds 1 |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/validate` | Two Test Command gaps were caught by my own mechanical pass rather than by the plan-template's authoring self-check: the boot-path smoke was missing although scripts/agent-run… | extra_reads 1 |
| `2026-09-07-sr-findings-fixed` | `phase/validate` | the operator's review supplied two REQUIRED-RESOLUTIONs the skill's own check 4 did not fire on: the (obs) criterion about a RED arm's output had no producer among the listed … | dialogue_rounds 1 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `phase/validate` | Mechanical check 4's producer clause fires BY CONSTRUCTION on a CI-probe chunk, and its two remedies are not equally available. The clause requires that an acceptance criterio… | extra_reads 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/validate` | I ran check 4 clause (6) as a STRUCTURAL test - is every gate an acceptance criterion names present as an entry - and passed it, while the clause exists to stop a criterion be… | dialogue_rounds 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/validate` | check 7 WARNs unavoidably on a formatting-only chunk: its SKIP condition keys on whether the modify-set holds files of an indexed plane, but the substantive question is whethe… | — |

**Proposal:** The records split two ways and the split matters: some are **coverage gaps** (a defect outside every mechanical predicate) and some are **fit failures** (check 4's producer clause firing by construction on a CI-probe chunk; check 7 WARNing unavoidably on a formatting-only chunk because its SKIP keys on file plane rather than symbol change). The first asks for new predicates; the second asks for the existing ones to carry a chunk-shape precondition. Worth considering separately.

### P9 — wrap-session/route-resolve · `contract.carry-no-owner` — 3 cases · weight 10 · **1 halt/soft-exit**

**Pattern:** A CARRY or follow-up had no self-evident owner among the markerless entries — 3 cases, one carrying a halt and two carrying dialogue rounds. Each resolved by an operator placement call.

**Evidence:** all 3 cases · 3 chunks · rate 0.2 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `wrap-session/route-resolve` | CARRY 5 had no natural owner among the four markerless entries, and the operator directive was PARTIAL — it named the disposition class ('append the re-CARRY to a later marker… | dialogue_rounds 1, extra_reads 2 |
| `2026-09-07-a11y-ci-gate` | `wrap-session/route-resolve` | the still-owed clearing event had no self-evident owner and forced a placement decision one entry early. A CI gate's first run can only follow the commit that ships it and the… | dialogue_rounds 1, halted 1 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `wrap-session/route-resolve` | An in-version follow-up has no owner entry and was routed to a surface route-resolve does not enumerate. The v2-24 claim-or-defer fork belongs, per the operator's wrap directi… | — |

**Proposal:** Ownership assignment is currently a judgment made per CARRY with no rule to fall back on, and the one halt shows it can block. A direction: a **default-owner rule** (for instance, the first entry whose scope opens the artifact the CARRY names — which is exactly how the `a11y-plan.md:115` dittography was placed on `v3-03` at this epoch's last wrap), with the operator call reserved for CARRYs that rule cannot place.

### P10 — all steps · `contract.structural-blind-spot` — 6 cases · weight 9

**Pattern:** A documented mechanism failed to reach something BY CONSTRUCTION — no amount of correct execution would have caught it — 6 cases across 5 chunks. One names this skill's own sibling: 'orientation's documented input set contains no external-CI read', which is how a stale BLOCKED-ON premise survived.

**Evidence:** all 6 cases · 5 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `phase/research` | read_run_journal parses every journal line as RunRecord, so any run emitting a per-check record fails the read outright - measured, not reasoned: conductor report on the one l… | extra_reads 2 |
| `—` | `new-session/orientation` | orientation's documented input set (health-inputs, handoff, state, master-route, working-route, matrix, git-state) contains no external-CI read, so the head entry's own named … | extra_reads 5 |
| `2026-09-09-port-occupier-test-hygiene` | `phase/research` | a test whose own process installs the product's panic hook cannot report its failure through the test runner: the panic is routed into the observability file sink, so libtest … | extra_reads 2, retries 1 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `wrap-session/reconcile` | The detector set cannot reach latency SEMANTICS in obs-plan, by construction. All seven doc-agents returned proposals: [] and each was correct on its own invariants; obs-plan'… | — |
| `2026-09-10-release-build-and-bundle` | `phase/validate` | Two real gate defects passed every P5 mechanical check and were caught only by the operator's review. (1) The three live legs' `stop` key held a SENTENCE ('none started by thi… | dialogue_rounds 1 |
| `2026-09-10-release-build-and-bundle` | `implement/smoke` | The chunk's CARRY C measurement arm was unreachable by construction, and no authoring gate could have caught it. Plan STEP 10 directs re-running the sr leg in its full firing … | deferred 1 |

**Proposal:** By-construction blind spots are the class that never improves through diligence, so they are the highest-value entries here per case. Each names its own boundary precisely; the direction is to treat these six as a **standing list of known-unreachable surfaces** rather than as incidents, so a future step can consult what the pipeline structurally cannot see before designing a check that assumes it can.

### P11 — phase/distill · `input.spec-source-gap` — 6 cases · weight 6

**Pattern:** A spec master was self-inconsistent, contradicted a sibling master, or carried a physically damaged entry — 6 cases across 6 chunks. Two are structural corruption in the same sidecar: a TRUNCATED amendment entry breaking mid-word, and an entry whose body is physically interleaved with the following entry (both `layout-templates-amendments.md`, the 2026-09-07 a11y-ci-gate amendment).

**Evidence:** all 6 cases · 6 chunks · rate 0.43 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-05-audit-corrective` | `phase/distill` | test-plan §9/§10/§11 state the supply-chain gate as `cargo audit --deny warnings` (5 sites) while ci.yml:75 runs bare `cargo audit`; the stated form exits 1 by construction ag… | extra_reads 1 |
| `2026-09-06-run-report-envelope-conformance-gate` | `phase/distill` | The obs distiller reported its own master self-inconsistent on the exact field list this chunk's gate must assert: obs-plan section 3 and section 6's schema block both show el… | — |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/distill` | test-plan.md and its amendment sidecar carry NO entry for this chunk's subject - the tests distiller re-derived zero hits for 'halo' / 'hue' / 'P-025' / 'delegated' across bot… | — |
| `2026-09-07-dependency-polish` | `phase/distill` | Three spec masters carry CONTRADICTORY values for one dependency version and no arbitration rule exists between them: design-system names indicatif 0.18 as target state while … | — |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/distill` | the layouts distiller reports that layout-templates-amendments.md carries a TRUNCATED entry - the 2026-09-07-a11y-ci-gate amendment breaks mid-word at 'would ha' with its tail… | — |
| `2026-09-10-release-build-and-bundle` | `phase/distill` | An amendment sidecar entry's body is physically interleaved with the following entry: in .andromeda/layout-templates-amendments.md the 2026-09-07-a11y-ci-gate entry's **Why:**… | extra_reads 2 |

**Proposal:** The corruption pair is a different thing from the inconsistency four and is the more urgent: a damaged sidecar entry silently degrades every future distillation that reads it. Direction: a **sidecar integrity check** at write time (entry boundaries well-formed, no interleave) would catch the class the distiller can currently only report after the fact.

### P12 — wrap-session/curation · `ambiguity.filter-borderline` — 6 cases · weight 6

**Pattern:** A curation candidate scored EXACTLY 0.6 — the contract's named scoring mass point, which rejects — so its fate turned entirely on a conditional signal, in 6 cases across 5 chunks. Four records state the 0.6 coincidence explicitly; one reports three of four candidates at or near it.

**Evidence:** all 6 cases · 5 chunks · rate 0.4 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `wrap-session/curation` | THREE of four candidates scored at or near the 0.6 mass point and the dispositions turned on fine readings of Filter 4. The sibling-readers candidate reached 0.7 only because … | — |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/curation` | The nextest-filter candidate scored EXACTLY 0.6 on its unconditional signals (verified-by-measurement 0.4 + specific-technical-detail 0.2), which the threshold rejects by desi… | — |
| `—` | `wrap-session/curation` | Filter 4's repeated-pattern +0.3 on the host-boundary candidate turned on whether a third failing invocation (MSYS eating backslashes in a quoted Windows path passed to bash) … | — |
| `—` | `wrap-session/curation` | The one applied candidate scored EXACTLY 0.6 on its base signals (measurement 0.4 + specific-technical-detail 0.2) — the contract's named scoring mass point, which rejects — a… | extra_reads 1 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `wrap-session/curation` | BOTH surviving candidates scored EXACTLY 0.6 on base signals — the contract's named scoring mass point, which rejects — and BOTH survived only because the conditional no-other… | extra_reads 2 |
| `2026-09-10-release-build-and-bundle` | `wrap-session/curation` | A survivor's unconditional signals totalled EXACTLY 0.6, the documented reject point, so its fate turned entirely on the conditional no-other-home +0.2; establishing that sign… | extra_reads 2 |

**Proposal:** A scoring scheme whose reject threshold sits exactly on the modal score of real candidates decides by tie-break rather than by score. The direction is a **threshold or signal-weight review** — either move the mass point off the decision boundary, or make the conditional signals that are currently deciding into first-class scored terms.

### P13 — implement/fix-loop · `contract.vacuous-check-found` — 4 cases · weight 6

**Pattern:** A check that could not fail was found during the fix loop — 4 cases across 4 chunks: a `git diff --exit-code` over an artifact untracked at implement time; a `Select-Object -First 1` over a directory the driver creates two of; a shell status verb reading the last journal line as the envelope when a per-check record is last.

**Evidence:** all 4 cases · 4 chunks · rate 0.24 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `implement/fix-loop` | The harness status verb in BOTH shells reads the last journal line as the run envelope (sh: tail -n 1 \| jq; ps1: Select-Object -Last 1). Per-check records ride the same journ… | iterations 1 |
| `2026-09-06-coverage-completeness-gate` | `implement/fix-loop` | Test Command 1's `git diff --exit-code coverage-matrix.md` returned 0, but the artifact is UNTRACKED at implement time (wrap commits it), and git diff reports nothing for an u… | — |
| `2026-09-08-hosted-runner-webview2-session` | `implement/fix-loop` | The transcribed scoped_dir probe could not discriminate: it took Select-Object -First 1 over TEMP's scoped_dir* dirs, but msedgedriver creates TWO siblings per session (same p… | iterations 1 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `implement/fix-loop` | A SECOND structural defect in constellation-severity-live-wiring, not predicted at P3/P4 and found only by measuring the live row: its declared slo_tier '<20s' is UNATTAINABLE… | — |

**Proposal:** All four were found by measurement, none by review, and each had been authored past a plan review. The direction pairs with P8: a **vacuity probe** on authored gates — run the check against a state where it MUST fail, before accepting it as a gate — which is precisely the control the epoch's last chunk used on `the_gate_discriminates`.

### P14 — implement/fix-loop · `contract.spec-reality-gap` — 4 cases · weight 6

**Pattern:** A spec master states a gate, tool posture or capability the shipped tree does not satisfy — 4 cases across 4 chunks, including `test-plan` §9 naming `cargo fmt --check` as a Lint-stage gate that the tree had apparently never satisfied and no CI job ran.

**Evidence:** all 4 cases · 4 chunks · rate 0.24 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-07-a11y-ci-gate` | `implement/fix-loop` | the folded CARRY 5 states knip returned '20 findings, all 20 false positives of ONE class' (WebdriverIO discovering specs by config rather than by import). Measured at the bas… | iterations 2 |
| `2026-09-08-hosted-runner-webview2-session` | `implement/fix-loop` | POINTER ONLY, surfaced to P4 — nothing authored. The dev host's msedgedriver now reports 152.0.4191.53 against WebView2 Runtime 152.0.4191.66, whereas test-plan.md:307's measu… | — |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `implement/fix-loop` | test-plan §9 (Pipeline structure, Lint stage) names `cargo fmt --check` as a gate; the shipped tree has apparently never satisfied it and no CI job has ever run it. Measured t… | — |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `implement/fix-loop` | the runner-portability gate (test-plan §4 Framework + §12) is mandated because only the shared-process runner exposes tests relying on per-test-process isolation; its first ru… | — |

**Proposal:** Each was discovered by an implement step attempting to rely on the spec. This is the measured instance of the project's own curated rule that a spec describes TARGET state; the pipeline-level direction is to mark spec claims that assert a CURRENT capability distinctly from those that assert an intended one, so a planner can tell which it may build on.

### P15 — phase/take-up · `input.out-of-pipeline-source` — 5 cases · weight 5

**Pattern:** The chunk's decisive fact lived outside every artifact the pipeline enumerates — 5 cases across 4 chunks: a constant in the SUT's source, a GitHub Actions run, an operator's launcher script present in neither repo, an operator-tunable SUT env handle.

**Evidence:** all 5 cases · 4 chunks · rate 0.36 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `phase/take-up` | The chunk's central premise — that 'the CURRENT SUT set' is current — is underivable from Conductor's tree: Conductor has no refs/ dir and the manifest is a transcribed record… | extra_reads 4 |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/take-up` | The decisive fact for this chunk's central design call - that Pulse's bootstrap window is operator-tunable via ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS - lives in the siblin… | extra_reads 4 |
| `2026-09-06-halo-hue-budget-re-driven` | `phase/take-up` | pulse-a11y.sh - named in the directive as the operator's Pulse launcher and load-bearing for the LIVE-LEG firing form - exists in NEITHER repo's working tree nor either git in… | extra_reads 2 |
| `2026-09-07-sr-findings-fixed` | `phase/take-up` | the chunk's decisive fact - that the CARRY's clearing event realized RED - lives in GitHub Actions run 34148079506, outside every artifact phase's input contract reads (workin… | extra_reads 4 |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/take-up` | The constant that decides whether each read-back sees its own preflight canary still open - DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120 - lives in the SUT's repo (andromed… | extra_reads 1 |

**Proposal:** take-up's input set is defined by the pipeline's own artifacts, and four of these five facts were load-bearing for the chunk's central design call. Direction: an explicit **external-source slot** on the route entry or the CARRY — naming where a fact lives when it lives outside the tree — so take-up fetches it rather than discovering its absence.

### P16 — wrap-session/report · `input.implement-outcome-unsettled` — 5 cases · weight 5

**Pattern:** Implement's P4 report could not stand as the wrap report's outcome basis — 5 cases across 5 chunks. **Every case has one mechanism**: an operator wrap directive issued BETWEEN implement and the report superseded, corrected or re-dispositioned implement's findings.

**Evidence:** all 5 cases · 5 chunks · rate 0.42 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `wrap-session/report` | Implement's P4 report was superseded as the outcome basis by an overseer wrap directive issued between implement and this report. It did not contradict implement — it added fo… | extra_reads 4 |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/report` | Implement's report could not stand as the outcome basis: it soft-exited on an out-of-scope test, reached green only after an operator-approved scope widening, ran a live leg a… | extra_reads 1 |
| `2026-09-07-sr-findings-fixed` | `wrap-session/report` | implement's P4 report shipped a causal diagnosis (msedgedriver launching the app in Edge mode) that a later operator ruling falsified as a probe artifact; two further CI probe… | extra_reads 2 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `wrap-session/report` | implement's P4 report declared the chunk SURFACED on a red cargo fmt --check and left the disposition open for the operator. The operator's wrap directive then settled it diff… | — |
| `2026-09-09-port-occupier-test-hygiene` | `wrap-session/report` | implement's P4 report was superseded between implement and this report by an operator wrap directive that corrected two of its measurements, so the report could not take it as… | extra_reads 2 |

**Proposal:** This pairs with cross-step chain X1/X2, where the same handoff artifact reads `thin` in 12 of 14 chunks. The handoff is not failing — the operator directive is doing real work — but the pipeline treats implement's report as the outcome basis while in practice a later directive is. Direction: make the **directive an explicit input to the report step** with its own slot, rather than something the report absorbs while still nominally sourcing implement.

### P17 — wrap-session/reconcile · `contract.cascade-miss` — 5 cases · weight 5

**Pattern:** An amendment's cascade under-enumerated its own sites — 5 cases across 5 chunks: a duplicate left standing one line from a site that was found; a body edit landing inside a quotation of another master; a citation sweep finding one of three sites.

**Evidence:** all 5 cases · 5 chunks · rate 0.42 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `wrap-session/reconcile` | The applied a11y amendment would have left its own duplicate standing. The operator-approved NARROW set named §9:457's 'where tests' webview E2E already runs'; after applying … | extra_reads 3 |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/reconcile` | The cross-master sweep found the retired 4-leg --live composition standing in a rule file's GENERATED body (.claude/rules/verification-harness.md:19) and a second unqualified … | — |
| `2026-09-07-a11y-ci-gate` | `wrap-session/reconcile` | a body edit landed INSIDE a quotation of another master and had to be repaired in the same pass. The a11y-plan §1 CI-integration bullet quotes architecture's [CI/CD] decision … | extra_reads 3 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `wrap-session/reconcile` | A doc-agent's own duplicate sweep missed a site ONE LINE from a site it found, and only the orchestrator's cascade sweep caught it. The security-plan detector was told to swee… | extra_reads 3 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `wrap-session/reconcile` | the cross-master citation sweep under-enumerated: a11y-plan quotes arch §Stack's CI rationale cell at THREE sites (:115, :280, :471) and my apply covered two. Caught BEFORE sh… | — |

**Proposal:** In two cases the orchestrator's sweep caught what a doc-agent's own sweep missed, which is the overlap working. The direction is narrower than 'sweep better': three of the five turn on **quotation and duplication within a master** — a master quoting another master's text is the shape that defeats per-document sweeps. A quote-aware site enumeration would address the class the generic sweep structurally cannot.

### P18 — wrap-session/gates · `tooling.result-not-run-stable` — 4 cases · weight 5

**Pattern:** A gate graded differently at the light gate than at implement over the same tree — 4 cases across 3 chunks: a live leg grading ManualCheck vs KnownResidual, an nsis installer differing by 4 264 bytes, a mutation index artifact, and an exit-status side file colliding with the /implement run's file of the same name.

**Evidence:** all 4 cases · 3 chunks · rate 0.36 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-coverage-completeness-gate` | `wrap-session/gates` | A Test Command graded differently at the light gate than at implement over the same tree, and BOTH readings were artifacts of the index rather than of the artifact. `git diff … | extra_reads 2 |
| `2026-09-06-halo-hue-budget-re-driven` | `wrap-session/gates` | The light gate's literal live re-run graded leg A ManualCheck where implement's run gave KnownResidual - identical tree, identical boot posture, same pulse-app process, 75 min… | — |
| `2026-09-10-release-build-and-bundle` | `wrap-session/gates` | `cargo tauri build` over an identical tree produced a DIFFERENT nsis installer: 4 418 544 B at /implement, 4 414 280 B at the light gate hours later (delta 4 264 B); the msi r… | extra_reads 3 |
| `2026-09-10-release-build-and-bundle` | `wrap-session/gates` | An exit-status side file in the prescribed per-marker gate log dir collided with the /implement run's file of the same name, and was read as this run's result while the backgr… | retries 1 |

**Proposal:** The last is a plain collision with a clear fix (per-run gate log names). The other three are genuine non-determinism in surfaces the project's determinism bar does not cover. Direction: the **light gate's re-run semantics** need a stated position on which command classes are expected to reproduce byte-exactly and which are not — currently every divergence must be diagnosed from scratch.

### P19 — phase/take-up · `tooling.long-line-edit` — 4 cases · weight 5

**Pattern:** Reading and editing the taken-up entry required chunked offset-bounded extraction because the entry is a single multi-KB line — 4 cases across 4 chunks (7561, 4808, 2873 and 2031 characters named). One produced a **false coordinate-mismatch finding stated to the operator before correction**, from a truncating probe.

**Evidence:** all 4 cases · 4 chunks · rate 0.29 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-07-dependency-polish` | `phase/take-up` | The taken-up working entry is one 7561-char line. Reading it needed a chunked python extraction (the host rule's prescribed recipe), and then the Read-before-Edit precondition… | extra_reads 1 |
| `2026-09-07-a11y-ci-gate` | `phase/take-up` | a truncating probe on a long spec line produced a false coordinate-mismatch finding that was stated to the operator before correction: CARRY 4 asks for four a11y-plan.md line … | extra_reads 1, reformulations 1 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `phase/take-up` | Both promotion writes target multi-KB single-line artifacts (working-route entry 2873 chars, master-route last record 2031 chars), and the anchored-Edit path needs the file re… | extra_reads 4 |
| `2026-09-10-release-build-and-bundle` | `phase/take-up` | The taken-up entry is one 4808-char line, so reading it took three offset-bounded windows per the host rule, and the Read-before-Edit contract then forced a fourth read of the… | extra_reads 4 |

**Proposal:** The write-side face of L1. The false finding is the notable cost: the workaround is not merely slower, it has produced a wrong claim. See L1.

### P20 — all steps · `contract.skill-reference-drift` — 4 cases · weight 5

**Pattern:** A skill body or reference states a path, enumeration or rule that mismatches deployed reality or a sibling reference — 4 cases across 2 chunks. Named: `curation-guide` leaving the correction-against-a-generated-body case unowned; setup-project's `rules-templates/host-win32` body stating a false host mechanic (`cd` 'does not persist'); `promotion.md` step 2 enumerating foldable annotations without CONTEXT, which route-resolve also writes; check 4 clause (9)'s definition of `new`.

**Evidence:** all 4 cases · 2 chunks

| chunk | step | what | impact |
|---|---|---|---|
| `—` | `wrap-session/curation` | curation-guide leaves the correction-against-a-generated-body case unowned: its Corrections section says to edit the false entry in place, its Tier 2 write logic says to prese… | extra_reads 2 |
| `—` | `wrap-session/curation` | A shipped rules TEMPLATE states a false host mechanic: setup-project's rules-templates/host-win32 body says a `cd` in a compound command 'does not persist', while the Bash too… | retries 1 |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/take-up` | promotion.md step 2 enumerates the foldable annotations as PREREQ / CARRY / BLOCKED-ON, but route-resolve also writes CONTEXT freight, which on this entry carried the entire s… | — |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `phase/validate` | check 4 clause (9) defines `new` as no prior plan or report naming the run program + subcommand, and by that test `cargo fmt` is NOT new (the 2026-09-08-webview2 plan and repo… | — |

**Proposal:** These are pipeline defects invisible to project drift detection, which is exactly why the type exists. All four are small and concrete. The direction is simply that they have **no routing surface today** — they are recorded here and nowhere else, so the proposal is a place for them to land.

### P21 — new-session/orientation · `input.handoff-git-mismatch` — 4 cases · weight 4

**Pattern:** The handoff's Branch field disagreed with measured git state — 4 cases. **All four are the same mechanism in the same direction**: the handoff predicts 'N ahead and unpushed' and names the push load-bearing; the operator then pushes; the next session measures 0 ahead.

**Evidence:** all 4 cases · 1 chunks · rate 0.24 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `—` | `new-session/orientation` | handoff said 53 ahead / unpushed / CI silent since 2026-08-09 and evolve-diagnose due; git shows upstream at HEAD 59d5b7c (0 ahead, reflog 'update by push'), CI run 3395434768… | extra_reads 4 |
| `—` | `new-session/orientation` | Handoff Branch field declares 5 ahead of origin at 59d5b7c; git measures 0 ahead / 0 behind at b2f60c8 — the branch was pushed after the wrap wrote the handoff, so the ahead-c… | extra_reads 1 |
| `—` | `new-session/orientation` | handoff Branch field says '1 ahead and unpushed' and calls the push load-bearing; git measures 0 ahead, origin at HEAD 6ec89dd, and the push-triggered CI run 34280136892 compl… | extra_reads 3 |
| `—` | `new-session/orientation` | the handoff's Branch field predicted '1 ahead and unpushed' with the push named load-bearing; git measured 0 ahead with HEAD == upstream (both 266da34). Not a handoff defect —… | extra_reads 1 |

**Proposal:** The handoff is written before an act it correctly predicts, and read after it. That is not a defect in either side — it is a field whose truth expires between write and read. Direction: have the wrap write the **push as a stated expectation** ('1 ahead at wrap; the operator pushes after') rather than as a state claim, which is what the current handoff has in fact started doing in prose while the Branch field still reads as a measurement.

### P22 — phase/take-up · `input.carry-context-gap` — 4 cases · weight 4

**Pattern:** A folded CARRY mis-described or under-counted its own subject, or cited coordinates that had drifted since authoring — 4 cases across 4 chunks. In two the promotion fold's mandated re-verify caught it; in one, three of seven CARRYs cited drifted coordinates and three under-counted their site sets.

**Evidence:** all 4 cases · 4 chunks · rate 0.29 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-06-operator-gated-live-suite` | `phase/take-up` | A CARRY's source-line coordinates decayed between authoring and take-up, and the wrap that moved the code re-pointed only ONE of the two artifacts naming those exact sites: th… | extra_reads 4 |
| `2026-09-07-dependency-polish` | `phase/take-up` | Three of the seven folded CARRYs cite coordinates that have DRIFTED since they were authored, and three under-count their own site sets. promotion.md prescribes re-verifying n… | extra_reads 6 |
| `2026-09-07-a11y-ci-gate` | `phase/take-up` | two of the five folded CARRYs mis-described their own subject and the promotion fold's mandated re-verify caught both: CARRY 1 named ONE README sentence but crates/conductor-t… | extra_reads 3 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `phase/take-up` | The PREREQ annotation names its subject by ORIGIN MARKER only — 'close rust gate deferral (deferred since 2026-09-08-hosted-runner-webview2-session)' — and never says WHICH ga… | extra_reads 1 |

**Proposal:** The re-verify is working and is the reason these are recorded rather than propagated. The direction is upstream: a CARRY cites coordinates in an artifact that keeps moving, so the citation decays by construction. Consider having a CARRY name its subject by a **stable anchor plus a re-derivation command** rather than by line coordinates — the same shape P1 proposes for counts.

### P23 — wrap-session/report · `contract.detector-fact-gap` — 3 cases · weight 3

**Pattern:** The report template had no bullet shaped for the fact the chunk produced — 3 cases across 3 chunks: a Dependencies bullet shaped for lockfile deps against a CI-fetched binary; no stock bullet distinguishing an amended claim from a correct one sharing the same token; a report-time coordinate sweep that under-counted.

**Evidence:** all 3 cases · 3 chunks · rate 0.25 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-07-dependency-polish` | `wrap-session/report` | A report-time sweep for the Expected amendments' site coordinates under-counted, in the same class this session has now hit five times. My pattern indicatif 0\.1[78] found ONE… | extra_reads 2 |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `wrap-session/report` | The report template's Dependencies bullet is shaped for lockfile dependencies — its stock form is '{added · bumped}' and detector D-security-deps reads it as 'the report's new… | extra_reads 1 |
| `2026-09-10-release-build-and-bundle` | `wrap-session/report` | The amended claim and a CORRECT claim share the token `2.11.3`, and no stock Changes bullet distinguishes them. The bundler/CLI sites (architecture 27/58/206/266, security-pla… | extra_reads 2 |

**Proposal:** Small and specific. Direction: the template's bullet set is enumerable and these three name the gaps precisely.

### P24 — phase/distill · `contract.extract-format` — 3 cases · weight 3

**Pattern:** A distiller return arrived in a shape the consuming step did not expect — 3 cases across 3 chunks: twice the subagent transport HTML-entity-escaped 6 of 7 returns (`&lt;run_id&gt;`, `&lt;host-path&gt;`), once a distiller added an unexpected line between the header and the first section.

**Evidence:** all 3 cases · 3 chunks · rate 0.21 per step-run

| chunk | step | what | impact |
|---|---|---|---|
| `2026-09-07-sr-findings-fixed` | `phase/distill` | 6 of 7 returns arrived HTML-entity-escaped by the subagent transport (arch/security/layouts/tests/obs/a11y carried &lt;run_id&gt;, &lt;host-path&gt;, tabindex &gt; 0, () =&gt;… | — |
| `2026-09-08-hosted-runner-webview2-session` | `phase/distill` | Six of seven distiller returns carried HTML entity escapes (&lt;/&gt;/&amp;) — the transport escaping fan-out.md anticipates — so raw twins were owed for six and the mechanica… | — |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `phase/distill` | The tests distiller returned an extra "Source plan: <path> (amendments: <path>)" line between the "# tests extract" header and "## Relevance". It is not preamble under fan-out… | — |

**Proposal:** The transport escaping is anticipated by `fan-out.md` and handled by the raw-twin rule, so the recurrence is the handling cost, not a surprise. It is listed because it clears threshold; the direction, if any, is to move the decode into the transport boundary rather than into each consumer.

## Cross-step chains (starting heuristics)

12 distinct (producer → artifact → consumer) shapes; **6 recur across ≥ 2 chunks**. 37 `input.*` frictions anchor the consumer side.

### X1 — implement/fix-loop → implement-outcome → wrap-session/report — 12 chunks · 15 joins · quality thin

**Chain hypothesis:** **The epoch's strongest chain by reach: 12 of 14 chunks.** The producer (`implement/fix-loop`) completes and its outcome artifact is consumed by `wrap-session/report` with quality `thin` in every one of the twelve. The typed correlate is P16 (`wrap-session/report :: input.implement-outcome-unsettled`, n=5), whose five records all name ONE mechanism: an operator wrap directive issued between implement and the report supersedes or corrects implement's findings. The chain is wider than that group because the thinness is recorded even where no friction was.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `ok` | green | `thin` — implement's P4 report was NOT the final word: an operator wrap directive landed after it, adding Expected amen… |
| `2026-09-06-coverage-completeness-gate` | `ok` | green | `thin` — NOT the final word — an overseer wrap directive arrived after implement's P4 report and changed four things (t… |
| `2026-09-06-halo-hue-budget-re-driven` | `soft-exit` | surfaced | `thin` — implement's P4 report is NOT the final word - it soft-exited first, went green only after the operator approve… |
| `2026-09-06-halo-hue-budget-re-driven` | `ok-degraded` | surfaced | `thin` — implement's P4 report is NOT the final word - it soft-exited first, went green only after the operator approve… |
| `2026-09-06-halo-hue-budget-re-driven` | `ok` | green | `thin` — implement's P4 report is NOT the final word - it soft-exited first, went green only after the operator approve… |
| `2026-09-07-dependency-polish` | `ok` | green | `thin` — NOT the final word on its own chunk: an operator wrap directive arrived between implement's P4 and this report… |
| `2026-09-07-a11y-ci-gate` | `ok` | green | `thin` — implement's P4 report was NOT the final word: the operator's wrap directive changed three of its conclusions -… |
| `2026-09-07-sr-findings-fixed` | `ok-degraded` | surfaced | `thin` — implement's P4 report was NOT the final word: two operator directives followed it (the diagnose-here ruling an… |
| `2026-09-07-sr-findings-fixed` | `ok-degraded` | green | `thin` — implement's P4 report was NOT the final word: two operator directives followed it (the diagnose-here ruling an… |
| `2026-09-08-hosted-runner-webview2-session` | `ok` | green | `thin` — implement's P4 report was NOT the final word: the operator's wrap directive changed four things after it — the… |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `ok-degraded` | surfaced | `thin` — implement's P4 report was NOT the final word on its own chunk: the operator's wrap directive arrived after it … |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `ok` | surfaced | `thin` — implement reported outcome=surfaced and was NOT the final word: an operator directive landed between its P4 re… |
| `2026-09-09-port-occupier-test-hygiene` | `ok` | green | `thin` — NOT the final word on its own chunk: the operator's wrap directive corrected two of its measurements before th… |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `ok` | green | `thin` — implement's P4 report was NOT the final word: an operator wrap directive issued after it changed the plan (the… |

**Proposal:** The handoff is not broken — the directive is doing real work and the report absorbs it correctly. But the pipeline nominally sources the report from implement while in practice the later directive is the settling input, so `thin` is the honest verdict 12 times running. Direction: give the wrap directive **its own declared slot** in the report step's input set, so implement's report is consumed as one of two inputs rather than as the outcome basis it cannot be.

### X2 — implement/smoke → implement-outcome → wrap-session/report — 12 chunks · 12 joins · quality thin

**Chain hypothesis:** The same shape from implement's other producing step, also 12 chunks, also `thin` throughout. The two chains share their consumer and their cause.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `ok` | green | `thin` — implement's P4 report was NOT the final word: an operator wrap directive landed after it, adding Expected amen… |
| `2026-09-06-coverage-completeness-gate` | `ok` | green | `thin` — NOT the final word — an overseer wrap directive arrived after implement's P4 report and changed four things (t… |
| `2026-09-06-halo-hue-budget-re-driven` | `ok` | surfaced | `thin` — implement's P4 report is NOT the final word - it soft-exited first, went green only after the operator approve… |
| `2026-09-07-dependency-polish` | `ok` | green | `thin` — NOT the final word on its own chunk: an operator wrap directive arrived between implement's P4 and this report… |
| `2026-09-07-a11y-ci-gate` | `ok` | green | `thin` — implement's P4 report was NOT the final word: the operator's wrap directive changed three of its conclusions -… |
| `2026-09-07-sr-findings-fixed` | `ok` | surfaced | `thin` — implement's P4 report was NOT the final word: two operator directives followed it (the diagnose-here ruling an… |
| `2026-09-08-hosted-runner-webview2-session` | `ok` | green | `thin` — implement's P4 report was NOT the final word: the operator's wrap directive changed four things after it — the… |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `ok` | surfaced | `thin` — implement's P4 report was NOT the final word on its own chunk: the operator's wrap directive arrived after it … |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `ok` | surfaced | `thin` — implement reported outcome=surfaced and was NOT the final word: an operator directive landed between its P4 re… |
| `2026-09-09-port-occupier-test-hygiene` | `ok` | green | `thin` — NOT the final word on its own chunk: the operator's wrap directive corrected two of its measurements before th… |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `ok` | green | `thin` — implement's P4 report was NOT the final word: an operator wrap directive issued after it changed the plan (the… |
| `2026-09-10-release-build-and-bundle` | `ok` | surfaced | `thin` — implement's P4 report was NOT the final word: two operator directives landed after it — the leg-3 swap (with i… |

**Proposal:** Folds into X1 — one remedy addresses both.

### X3 — phase/plan → plan → implement/fix-loop — 4 chunks · 4 joins · quality thin/wrong

**Chain hypothesis:** `plan.md` consumed `thin` or `wrong` by the implement fix-loop across **4 chunks**. The matching problem-facts are concrete: `plan step 6 specified EmissionShape::spans_per_dispatch()` where only `EmissionSpec` can see `signal`; `plan step 4 said 'the production render path in coverage.rs' without naming the function`, so the line first landed in a function the CLI's no-write path never calls; `plan steps 4 and 5 prescribed default-features = false on the MEMBER entries`, which cargo rejects outright for an inherited dependency.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `ok` | designed-dialogue:carry-teardown-form | `thin` — every Test Command ran as written EXCEPT the sequencing: gates 4-7 each begin with a seed and only gates 4 and… |
| `2026-09-08-webview2-runtime-152-installed-in-job` | `ok` | decisive-lean-resolved-3, no-capability-claimed, expected-amendments-4… | `thin` — seven of eight Test Commands ran as written and passed first time. The eighth, cargo fmt --check, CANNOT pass … |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `ok` | designed-dialogue:step-placement | `thin` — 11 of the 12 Test Commands ran green as written. The plan-quality defect is gate 7 (the per-crate cargo test r… |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `ok` | fork-resolved-by-artifact, first-live-leg-gate-entries | `wrong` — all 14 Test Command entries ran as written and 11 met their expect in full, but 3 (the live legs) carry an ato… |

**Proposal:** Every case is the plan naming a symbol, site or mechanism that reality rejects, corrected in place by implement. This is the implement-side face of L4; the validate-side face is X3. Direction: the three cases above are all **verifiable at plan time against artifacts the planner already has** (the type's own definition, the call graph, cargo's inheritance rule) — which is what makes them a candidate for a mechanical predicate rather than a review judgment.

### X4 — phase/plan → plan → phase/validate — 3 chunks · 3 joins · quality thin/wrong

**Chain hypothesis:** `plan.md` consumed `thin` or `wrong` by its own validation step across **3 chunks** — the defect is visible one step after authoring, inside phase, before any review.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-06-operator-gated-live-suite` | `ok` | designed-dialogue:invocation-shape, designed-dialogue:canary-identity | `wrong` — passed every mechanical check and my own validation-1 on the first pass, then the operator review falsified FO… |
| `2026-09-06-run-report-envelope-conformance-gate` | `ok` | designed-dialogue:carry-teardown-form | `wrong` — all 7 mechanical checks passed on the plan as first written, and it still carried a load-bearing FALSE mechani… |
| `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` | `ok` | designed-dialogue:step-placement | `thin` — passed mechanical checks 1-6 and val-1, then the operator review found THREE real defects in it: an a11y crite… |

**Proposal:** Pairs with P8 and L4. That validate itself records the plan as thin in 3 chunks suggests some of what the operator supplies at P5 is already detectable at P4→P5.

### X5 — phase/research → research → implement/code — 2 chunks · 2 joins · quality thin

**Chain hypothesis:** `research.md` consumed `thin` by implement across 2 chunks.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-06-run-report-envelope-conformance-gate` | `ok` | unresolved-questions | `thin` — the Files-to-modify list was one file short: step 9 placed the verb dispatch in commands/mod.rs, but the Comma… |
| `2026-09-06-halo-hue-budget-re-driven` | `ok` | unresolved-questions, sut-mechanism-re-derived, fire-site-arithmetic-d… | `thin` — the Files-to-modify boundary list OMITTED a crate-local data pin - see friction -b |

**Proposal:** Below the weight of X1–X4; recorded for completeness.

### X6 — wrap-session/report → report → wrap-session/reconcile — 2 chunks · 2 joins · quality thin

**Chain hypothesis:** The wrap report consumed `thin` by reconcile within the same wrap, across 2 chunks.

| chunk | producer outcome | producer signals | consumer verdict |
|---|---|---|---|
| `2026-09-07-dependency-polish` | `ok` | — | `thin` — sufficient for every proposal — each cited a report fact and no detector re-derived from git or a manifest — b… |
| `2026-09-10-live-pulse-in-lane-scenario-round` | `ok` | — | `thin` — THE consumer verdict, and it is thin for one measured reason: the report SUBSTANTIATED the latency fact (the d… |

**Proposal:** Correlates with P23 (`contract.detector-fact-gap`, n=3) — the report template lacking a bullet shaped for the fact reconcile then needs.

## Level candidates (systemic-masked-as-project)

Pass A clustered the epoch's 78 problem-facts by the obstacle each routes around (`workaround` 36 · `overridden` 12 · `removed-cause` 11 · `deferred` 11 · `prohibition` 6 · `unresolved` 2; natures: process 45 · environment 21 · product-logic 11 · resources 1). Detection is independent of whether a typed correlate exists; where one does, it is named.

### L1 — band-aid — Multi-KB single-line entries in Andromeda's own artifacts defeat the prescribed read AND edit forms, at every skill

**Facts:**
- **12 `workaround` problem-facts** across two faces, plus two typed groups and one untyped record.
- **Write face (6 facts)** — the skill bodies prescribe an anchored Edit for a point change and a whole-file Write for a compaction; both are defeated by an entry stored as one multi-KB line, so every skill substitutes a scratchpad python read-modify-write run by path: `wrap/route-resolve` (route/playbook/drift-base entries), `phase/take-up` ×2 (`2026-09-05-audit-corrective`, `2026-09-06-coverage-completeness-gate`), `wrap/curation` (`verification-harness.md`, a 26 KB line), `wrap/route-resolve` (`residuals.md`, `testing.md:54`), and `wrap/gates` inverting the prescription the other way — one anchored Edit instead of the prescribed whole-file rewrite, to avoid re-emitting 130 untouched multi-KB lines byte-exactly.
- **Read face (6 facts + typed group `tooling.output-cap-overflow` n=10)** — a structural grep over `master-route.md` or `verification-harness.md` returns whole multi-KB records and blows the tool-result cap: 29.7 KB, 38.5 KB, 38.6 KB, 40.8 KB, 60.4 KB, 56.2 KB, 31.6 KB measured. One record marks a same-day recurrence on the same artifact; another marks a third occurrence in one session.
- **Typed correlates:** `tooling.output-cap-overflow` n=10 / 6 chunks (rate 2.7× on the prior epoch) and `phase/take-up :: tooling.long-line-edit` n=4 / 4 chunks — the latter including a **false coordinate-mismatch finding stated to the operator** from a truncating probe.
- Natures: environment 7, process 5.

**Level hypothesis:** The cause lives in the artifact FORMAT — Andromeda's route, master-route, residual and rule-file entries are authored as one line each. The fixes have all lived in the project: a per-skill workaround, re-invented at `take-up`, `route-resolve`, `curation`, `gates`, `research` and `orientation` independently, and now partly codified as a host rule. No skill owns the format; every skill pays for it.

**Proposal:** Two directions, both pipeline-side. (a) **Change the format** — if entries could wrap, anchored Edit and ordinary grep would both work and twelve workarounds plus ten cap overflows would have no occasion. (b) **Own the workaround once** — a single shared extraction/patch helper in `andromeda-tools/` that every skill calls, instead of each skill's body prescribing a form its own artifacts defeat. The measured cost of the status quo is 22 recorded events in one epoch and one wrong claim reaching the operator.

### L2 — band-aid — The session-level auto-mode directive and this project's PreToolUse guard point in opposite directions

**Facts:**
- **8 events**: 3 `workaround` facts, 2 `removed-cause` facts, 2 untyped frictions, 1 typed `tooling.host-shell`.
- `new-session/orientation` ×2 (untyped): 'the session-level auto-mode directive instructs making file changes via Bash heredocs rather than the Write tool, which the project's own PreToolUse guard blocks outright for file-target heredocs; the two rules point opposite ways and the conflict surfaces only as a blocked call' — and, a second session, 'the first script-authoring call exited 2 and was re-authored via Write'.
- `wrap/reconcile` (`2026-09-08-webview2-runtime-152-installed-in-job`): 'first attempted the sidecar appends as a `cat >> file <<EOF` heredoc; the PreToolUse hook blocked it … which is the same rule the skill's own constraints state'.
- `phase/distill` (`2026-09-08-hosted-runner-webview2-session`, typed `tooling.host-shell` + a `removed-cause` fact): 'the guard fired exactly as designed and cost one call'.
- `new-session/orientation` ×2 more (`workaround` + `removed-cause`): the health-check script and three structural extractors authored via Write instead, per `host-win32.md`.
- Natures: environment 2, process 3 (plus the untyped pair).

**Level hypothesis:** The cause is a **configuration contradiction between two layers** — a session-mode directive that is not project-aware, and a project guard that is correct. Every fix has been one agent, mid-step, absorbing one blocked call. The guard is doing its job in all 8 cases; what recurs is the collision, not a failure.

**Proposal:** The guard should stay. The direction is to make the **session-mode directive project-aware** — or to have the project's own rules state the exception in a place the directive-following agent reads BEFORE its first authoring call, rather than discovering it as an exit-2. Cost measured: 8 events, at least 3 wasted calls, and it is the only theme in the epoch where the pipeline contradicts itself rather than the environment.

### L3 — band-aid — First-authored shell command forms are defeated by this host, at a flat rate across five epochs

**Facts:**
- **5 `workaround` facts** plus typed group `tooling.host-shell` n=8 / 6 chunks.
- Facts: a python probe assuming one result shape crashed on a KeyError (`phase/research`); `grep -rn` over `crates/` returned 1.6 MB by matching untracked `node_modules` (`phase/research`); `tr -d '`- '` aborted on a reverse-collating character range (`phase/validate`); Bash cwd persistence broke a second parallel call's relative `cd` (`new-session/orientation`); an `rm -rf`-bearing compound was denied by the permission layer (`implement/fix-loop`).
- Typed cases name: `rev` absent from this MSYS coreutils; backslash mangling in an inline `python -c`; cwd persistence again; `sed` reading a Windows backslash path as an unterminated s-command; unescaped `(` under `grep -E` returning a confident wrong answer twice; `\+` under BASIC regex; a Windows-native python subprocess reporting all nine gate entries red.
- Rate across epochs: 0.088 (E3) → 0.144 (E4) → 0.032 (E5) → 0.038 (6a) → 0.041 (6b) — **flat for three epochs**.

**Level hypothesis:** The cause lives in the host. Every fix has landed in `host-win32.md`, which is now substantial and is working — each of these was caught. But a flat rate under a growing rule file means the rules are catching the class after it fires, not preventing it: the agent authors the POSIX-shaped form first and learns on the failure.

**Proposal:** This may be the correct equilibrium and the founder may wish to leave it. If not, the direction is prevention rather than more rules: the recurring mechanisms are enumerable (backslash paths, ERE metacharacters, absent coreutils, cwd persistence, recursive-delete permission), and a **pre-flight lint on authored shell commands** against that short list would move the cost from post-failure to pre-execution.

### L4 — override — The operator's review is systematically supplying plan corrections the mechanical checks do not produce

**Facts:**
- **12 `overridden` problem-facts**, concentrated at two steps: `phase/validate` (4) and `wrap-session/report` (3), then `phase/take-up` (2), `implement/fix-loop` (1), `wrap/reconcile` (1), `wrap/gates` (1).
- At `phase/validate`: 'the operator returned five fixes at the review instead of approving. Four were plan defects I had shipped' (`dependency-polish`); 'the operator's R1 overrode the plan's central course … turning the chunk from remedy-plus-diagnostic into diagnose-only' (`hosted-runner-webview2-session`); 'the operator's review directed a scope expansion the plan did not contain … taking the gate from eight touched crates to all nine members' (`port-occupier-test-hygiene`); 'the operator's P5 review overrode two authored gate details the mechanical checks passed' (`release-build-and-bundle`).
- At `wrap-session/report`: the directive overrode default report content in four places (`dependency-polish`); corrected two measurements the report would have carried forward from implement (`port-occupier`); directed an out-of-band edit to a COMPLETED chunk plan before P7.1 (`live-pulse-in-lane-scenario-round`).
- **Correlates:** typed `phase/validate :: contract.mechanical-check` n=6/5 chunks; 3 further untyped phase/validate records (U2); `contract.structural-blind-spot` naming 'two real gate defects passed every P5 mechanical check and were caught only by the operator's review'; cross-step chains X3 (`phase/plan → plan → phase/validate`, 3 chunks) and X4 (`phase/plan → plan → implement/fix-loop`, 4 chunks).

**Level hypothesis:** The signature's reading is that the RULE is miscalibrated where an override recurs on the same call. Here the override recurs on the same STEP rather than the same rule: the P5 review is not correcting a rule the mechanical checks apply wrongly — it is supplying a class of judgment the checks structurally do not attempt (gate vacuity, cross-step consistency, scope completeness, chunk-shape fit). The fixes have all landed per-chunk, in that chunk's plan.

**Proposal:** Two readings, and the diagnosis cannot choose between them. Either the review IS the designed gate for this class and the mechanical checks are correctly scoped — in which case the finding is simply that P5 is load-bearing and its cost is real (4 review rounds with fixes returned, in 4 of 14 chunks) — or the classes the operator keeps supplying are enumerable and some are mechanizable. The records name what was supplied each time, which makes the enumeration cheap to attempt: vacuity of an authored gate (P13's control is the worked example), consistency BETWEEN individually-correct steps, and whether a check's SKIP condition fits the chunk's shape.

### L5 — chronic-degrade — `implement/fix-loop` degraded in 5 of 14 chunks and never halted — a 4× jump on every prior epoch

**Facts:**
- `ok-degraded` at `implement/fix-loop`: **5** here, against 1 (E3), 1 (E4), 2 (E5), 1 (6a). Plus **1 `soft-exit`** (`2026-09-06-halo-hue-budget-re-driven`).
- Chunks: `halo-hue-budget-re-driven`, `sr-findings-fixed` ×2, `webview2-runtime-152-installed-in-job`, `release-build-and-bundle`.
- The two `unresolved` problem-facts in the epoch both sit at this step: 'the CI a11y job's session creation never completes on the hosted windows-2025 runner … the remaining cause is unnamed and its resolution is a design decision, not a chunk-level code fix'; and 'the 3 failing atoms live in plan.md, which /implement must not modify, so the defect could not be fixed in-flight'.
- Epoch-wide `ok-degraded` is 5 (all at fix-loop) against 6, 4, 3, 3 in prior epochs — so the TOTAL is flat and the concentration is what moved.

**Level hypothesis:** The cause appears to be a chunk-mix effect rather than a mechanism failure: this epoch's fix-loops ran against CI, a hosted runner, a screen reader and a live SUT — surfaces where a fix can be correct and still not close. The degradation is recorded honestly every time and never halts, which is the signature's concern: a step that always completes degraded never triggers the halt policy, so the trend is visible only in aggregate, exactly here.

**Proposal:** No fix is indicated for the degradations themselves. The direction worth considering is **visibility**: `ok-degraded` currently carries the same weight as `ok` in every consumer, so a step degrading in 5 of 14 chunks reads as a clean epoch everywhere except this ledger. A degraded-outcome count in the wrap report would make the trend legible without changing any policy.

### L6 — band-aid — Token-trap knowledge is written as a chunk-local prohibition and does not travel

**Facts:**
- **4 of the epoch's 6 `prohibition` facts** are guards against a naive token re-derivation, each authored into ONE chunk's artifact:
- `phase/take-up` (`dependency-polish`) — a hazard note in `scope.md` forbidding still-true security FLOORS from being swept up as stale version statements, enumerating 6 sites.
- `phase/research` (`dependency-polish`) — a do-not-touch guard in `research.md` naming `PiiCorpus::seeded(7)` at four sites, 'a SEED numerically equal to the arity, which any arity-7 sweep returns as a false positive'.
- `wrap/report` (`webview2-runtime-152`) — 'Detectors: do not propose retiring test-plan.md:455', written into the report because the naive reading of a measured falsity is a retirement proposal.
- `phase/validate` (`live-pulse-in-lane-scenario-round`) — a plan guard: 'Do NOT re-derive this with a bare token grep: `grep -c CountAtLeast` returns hits in seven files', because four scenarios carry the token only in retirement comments.
- **Typed correlate:** `contract.token-proxy-check` n=24 across 12 chunks.

**Level hypothesis:** The cause is that the pipeline has no home for a fact of the form 'this token does not mean this property, and here is the pattern that misleads'. Each discovery is real and correct, and each is absorbed at the project level into the one artifact of the chunk that found it — a `scope.md`, a `research.md`, a `plan.md`, a `report.md` — none of which the next chunk reads.

**Proposal:** Direction: a **token-trap registry** at project scope (the nearest existing surface is the rule files, which already carry several such facts in prose). The measurement supporting it is that the class fired 24 times in one epoch while four chunks independently wrote local guards against it.

### L7 — deferred-forever — Two deferrals carry no named destination and no closure appears in the ledger

**Facts:**
- 11 `deferred` facts in the epoch. **Nine carry an explicit destination** — routed to wrap as an Expected amendment, to P4, to a route-entry candidate, to the residuals append, or pinned as a PREREQ on the next markerless entry — which is designed behavior and not this signature.
- **Two do not.** (a) `implement/code` (`dependency-polish`): 'The eighth PiiCategory is unreachable from any scenario: conductor-core's `PiiCategorySpec` still has 7 variants and conductor-run's `wire_category` maps it 1:1 into the now-8-variant emit enum, which is not a match-exhaustiveness error, so the compiler stayed silent.' (b) `implement/fix-loop` (`dependency-polish`): 'knip ships usable only with an entry-point config: its 20 findings are 20 false positives'.
- Neither subject appears again in the ledger after 2026-09-07/09-08 — through the epoch's remaining 3 days, nor in the 97 records of the following epoch.

**Level hypothesis:** The cause is that a deferral without a named destination has no reader. The nine routed deferrals demonstrate the mechanism works when a destination is supplied; these two simply have nowhere to have been noticed since.

**Proposal:** The lookback window is short (3 days in-epoch, plus one partial epoch), so this is a weak observation, not a demonstration of abandonment — stated as such. Direction, if any: make the **destination a required field** on a `deferred` problem-fact, so a deferral with nowhere to go is visible at capture rather than at diagnosis.

## Playbook-extension candidates (untyped patterns, F-4)

27 untyped records in the epoch (10.3% of friction), hand-clustered. Three clusters clear the F-4 bar (n ≥ 3 in-epoch). Extending a playbook is an Andromeda change only the founder applies.

### U1 — implement/smoke · wrap-session/gates — 3 cases → produced[].signals (no new type needed)

**Cluster:**
- `2026-09-06-operator-gated-live-suite` / implement/smoke — 'A POSITIVE worth recording against a standing worry rather than a defect: the live suite's process teardown was exact … the pre-leg census and the post-leg census matched byte for byte.'
- `2026-09-06-coverage-completeness-gate` / wrap-session/gates — 'POSITIVE measurement of a control working: the commit printed LF will be replaced by CRLF warnings for every tracked text file it touched EXCEPT coverage-matrix.md, the one path .gitattributes pins eol=lf.'
- `2026-09-10-live-pulse-in-lane-scenario-round` / wrap-session/gates — 'A POSITIVE measurement worth keeping, recorded here because no listed type covers it and the closest one (tooling.result-not-run-stable) is its exact inverse. The light gate re-ran all three live legs … and REPRODUCED the round.'

**Draft:** **This is a known issue recurring, not a new one.** `evolve-system.md` already prescribes the remedy verbatim — 'a positive measurement of a mechanism WORKING … is a `signals` fact here with a note — never an untyped friction record, which carries cost (measured: three positives logged as untyped across two wrap steps — two of them held wrap/gates at 50 % untyped)'. The guidance exists and three more positives were logged as untyped anyway, two of them again at `wrap-session/gates`. So the proposal is NOT a new type: it is that the guidance is in the record shape section, where an author reaching for a friction type does not encounter it. Draft criteria line for the step playbooks' interrogation sections: *'A mechanism you expected to fail that held — a control firing as designed, a gate reproducing, a teardown that was exact — is a `produced[].signals` fact with a note, never a friction record.'*

### U2 — phase/validate — 3 cases → (existing type under-reached: `contract.mechanical-check`)

**Cluster:**
- `2026-09-06-coverage-completeness-gate` — 'Two operator-caught plan defects no P5 mechanical check covers. (a) INCOMPLETE SWEEP OF A KNOWN CLASS … (b) UNCONSIDERED PORTABILITY AXIS: a byte-for-byte artifact comparison was planned with no line-ending consideration.'
- `2026-09-06-halo-hue-budget-re-driven` — 'Two plan steps were individually correct and jointly inconsistent … Nothing in the mechanical checks or val-1 looks for consistency BETWEEN steps; the operator's round-2 note found it.'
- `2026-09-07-a11y-ci-gate` — 'the plan authored a SECOND COPY of the envelope schema into the CI job … when the repo's own comment at ci.yml:167-171 explicitly rejects exactly that … The miss is not a research gap: P3 had ALREADY recorded the established gate shape under Patterns detected, and P4 still [authored it].'

**Draft:** These three are the same class as the typed group `phase/validate :: contract.mechanical-check` (n=6) — its first case reads 'four Test-Command defects outside every mechanical predicate', which is exactly what these say. Recorded untyped anyway, in three separate chunks. **Proposal is a criteria clarification, not a new type**: the playbook line for `contract.mechanical-check` evidently does not read as covering 'a defect NO check attempts', only 'a check that misfired'. Draft criteria line: *'`contract.mechanical-check` covers both a check that fired wrongly and a plan defect that NO mechanical predicate attempts — if the operator's review caught it and no check could have, it is this type.'* Folding them in takes the class to n=9 across 8 chunks, which would make it the epoch's fourth-largest.

### U3 — phase/research · implement/fix-loop — 3 cases → proposed type `contract.instrument-validity`

**Cluster:**
- `2026-09-07-dependency-polish` / implement/fix-loop — 'A dead-code tool installed to close an audit column produced a signal with a 100 percent false-positive rate on first run … knip reported 3 unused files, 5 unused devDependencies and 12 unused exports/types — all 20 wrong, and all one class: WebdriverIO discovers specs through wdio.conf.ts … so knip's import-graph reachability cannot see the package's largest consumer.'
- `2026-09-07-sr-findings-fixed` / implement/fix-loop — 'my own isolation probe (b) POSTed a session straight at msedgedriver, bypassing tauri-driver — which is the component that owns the WebView2-host translation — so a failure there could not by itself discriminate driver-image behaviour from a bypassed translation … A bypass probe needs its in-path twin to be evidence.'
- `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` / phase/research — 'a diff classifier keyed on line.startswith(+) returned 0 added and 0 removed lines against 282 real hunks, because cargo fmt --check prefixes every +/- with an ANSI colour escape; the meaningless zero was visible only by dumping the raw output through cat -A.'

**Draft:** The distinguishing mechanism is that the INSTRUMENT could not see the subject it was credited with measuring — not a narrow basis (the basis was the right subject), not a token proxy (in two of three there is no token), not host-shell (the shell behaved). The project has already curated the rule in prose ('an INSTRUMENT reporting a cause is not the cause — validate a diagnostic form where the real path PASSES before building on what it says when it fails'), so the class is recognized; the evolve taxonomy has no name for it and three records went untyped. Draft criteria line: *'`contract.instrument-validity` — a probe, tool or classifier returned a result that was structurally incapable of reflecting its subject (a reachability model blind to the real discovery mechanism, a bypass probe missing its in-path twin, a parser blind to the data's encoding); record how the instrument was validated, or that it was not.'*

## Below threshold — no action

**36 typed groups below F-2**, one line each:

- `retry.fix-iterations` · implement/fix-loop — n=2, weight 5, 2 chunk(s)
- `tooling.result-not-run-stable` · implement/fix-loop — n=2, weight 2, 2 chunk(s)
- `input.extracts-conflict` · phase/plan — n=2, weight 4, 2 chunk(s)
- `retry.synthesis-rework` · phase/plan — n=2, weight 4, 2 chunk(s)
- `input.extract-signal-gap` · phase/research — n=2, weight 2, 2 chunk(s)
- `retry.query-reformulation` · phase/research — n=2, weight 4, 2 chunk(s)
- `contract.matrix-claim` · phase/validate — n=2, weight 3, 2 chunk(s)
- `input.out-of-pipeline-source` · phase/validate — n=2, weight 3, 2 chunk(s)
- `tooling.commit-mechanics` · wrap-session/gates — n=2, weight 3, 2 chunk(s)
- `tooling.gate-deferral` · wrap-session/gates — n=2, weight 2, 2 chunk(s)
- `contract.false-positive-proposal` · wrap-session/reconcile — n=2, weight 2, 2 chunk(s)
- `contract.no-sanctioned-channel` · wrap-session/route-resolve — n=2, weight 2, 2 chunk(s)
- `tooling.long-line-edit` · wrap-session/route-resolve — n=2, weight 2, 2 chunk(s)
- `recall.curated-rule-not-applied` · all steps — n=1, weight 2, 1 chunk(s)
- `ambiguity.scope-pressure` · implement/None — n=1, weight 4, 1 chunk(s)
- `contract.vacuous-check-found` · implement/None — n=1, weight 2, 1 chunk(s)
- `input.research-files-wrong` · implement/None — n=1, weight 4, 1 chunk(s)
- `tooling.environmental` · implement/None — n=1, weight 1, 1 chunk(s)
- `input.conventions-gap` · implement/code — n=1, weight 1, 1 chunk(s)
- `input.research-files-wrong` · implement/code — n=1, weight 1, 1 chunk(s)
- `contract.test-expectation` · implement/fix-loop — n=1, weight 1, 1 chunk(s)
- `tooling.environmental` · implement/fix-loop — n=1, weight 1, 1 chunk(s)
- `tooling.hook-friction` · implement/fix-loop — n=1, weight 1, 1 chunk(s)
- `tooling.harness-friction` · implement/smoke — n=1, weight 2, 1 chunk(s)
- `tooling.subprocess-bounds` · implement/smoke — n=1, weight 1, 1 chunk(s)
- `contract.binding-contradiction` · phase/distill — n=1, weight 1, 1 chunk(s)
- `retry.distiller-respawn` · phase/distill — n=1, weight 2, 1 chunk(s)
- `input.research-thin` · phase/plan — n=1, weight 4, 1 chunk(s)
- `input.cookbook-gap` · phase/research — n=1, weight 2, 1 chunk(s)
- `input.out-of-pipeline-source` · phase/research — n=1, weight 1, 1 chunk(s)
- `contract.intent-divergence` · phase/validate — n=1, weight 3, 1 chunk(s)
- `ambiguity.tier-routing` · wrap-session/curation — n=1, weight 1, 1 chunk(s)
- `contract.test-expectation` · wrap-session/gates — n=1, weight 1, 1 chunk(s)
- `input.report-insufficient` · wrap-session/reconcile — n=1, weight 1, 1 chunk(s)
- `ambiguity.trajectory-halt` · wrap-session/route-resolve — n=1, weight 1, 1 chunk(s)
- `contract.standing-pin-carriage` · wrap-session/route-resolve — n=1, weight 1, 1 chunk(s)

**Untyped clusters below F-4:**

- **Rust code-graph index incomplete at symbol grain** — n=2 untyped in-epoch (`operator-gated-live-suite`: `execute_scenario` returns 0 rows with only the enclosing module indexed, against three production callers found by grep; `live-pulse-in-lane-scenario-round`: `observe` returns 0 rows while two siblings in the same file are indexed and the crate carries 323 symbols), plus one `workaround` fact in the same epoch. **One instance short of the bar, and worth the founder's eye anyway**: the same phenomenon recurs at Epoch 4 (`tooling.graph-unavailable`), Epoch 4 again (`input.cookbook-gap`) and Epoch 5 (`retry.query-reformulation`) — *four different typings and two non-typings for one mechanism*, which is the shape a missing type makes. It does not clear F-4 because the PREVIOUS epoch (6a) carries no instance.
- **Session-mode directive vs the project's PreToolUse guard** — n=2 untyped; not raised as a type candidate because the event is a configuration contradiction rather than a step outcome. Carried at full strength as **L2** instead (8 events).
- **Measurement-basis divergence** — n=2: reading the run-report envelope from the frozen per-leg self-obs rather than the per-run journal (probe returned empty, self-announcing); and `+1581/−486` (rustfmt hunk rendering) vs `+1582/−487` (`git diff --numstat`) for one pass, both correct measurements of different things. Adjacent to U3 but a distinct mechanism.
- **Singletons** (1 each): a directive citing a measurement no artifact carries; the handoff's Position field understating a head entry's CARRY count; two Tier-2 curation candidates with no rendered home falling to Tier 3; a take-up re-verify surfacing a CI defect the directive did not name; `fan-out.md`'s raw-twin rule applied mechanically producing a byte-identical twin; an NVDA `role=alert` region read whole, speaking a re-announce twice; a route entry having to carry an explicit retraction of a causal reading the same chunk committed; the friction ledger's own epoch-label split; a review round spent on tool FORM (`matrix.py show`) rather than substance; `implement` P1's self-check asking 'all New files written?' for a file the plan defers past implement by construction; a recurrence despite a curated learning.

**Problem-facts below theme threshold** (1 each): the 13-minute mutation tier exceeding the Bash call cap (the epoch's only `resources` fact); `@crabnebula/tauri-driver` shipping an 8-line stub so its behaviour is unreadable from source; `curation-guide` prescribing an in-place correction to a body its own Tier-2 logic forbids touching; a failing test emitting no diagnostic through libtest because the product's panic hook routes it to the file sink; the light gate run through a Windows-native python wrapper reporting all nine entries red (the wrapper, not the gates); a stale `21.exit` deleted from a shared per-marker gate log dir; a spec body amended at P7 outside the phase that owns amendments, because drift = 0 could not hold over a claim the gate had just falsified.

---

*Read-only diagnosis. Evidence twins: `q-health.json` · `q-typed.json` · `q-untyped.json` · `q-chains.json` · `q-level.json` · `q-retractions.json` in this run dir. Nothing here is applied, queued, or remembered: a re-run recomputes from the ledger alone, and a rejected proposal leaves nothing pending.*
