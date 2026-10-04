# Evolve Diagnosis — conductor-0.3.0 · Epoch 4 — Live proof against a real Pulse · 2026-10-04T08:53:35Z

Founder-ruled run order (2026-10-04, relayed by the overseer): Epoch 4 first, then Epoch 5 in its own run. Every
proposal below is obligation-free. Accept, reject, defer or modify any of them; nothing is applied, queued or
remembered.

**Chunk legend** (the `chunk` column uses these codes; the evidence pointer is the ledger record id unless a path is given):
`PG` 2026-09-18-real-model-leg-posture-and-grading-rule · `IL` 2026-09-22-interpretation-proven-live ·
`CP` 2026-09-23-real-model-capture-path-handles-guarded-and-stale-read-back-texts-corrected ·
`RC` 2026-09-24-architecture-registries-compacted-under-the-read-cap · `DQ` 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin ·
`DL` 2026-09-29-dual-license-mit-or-apache-2-0 · `HS` 2026-09-29-hue-shift-budget-graded-hard ·
`RP` 2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir · `NS` a new-session orientation (chunk null)

## Mechanism health
- **Records:** 255 (119 step / 136 friction) across 8 chunks and 15 new-session orientations. The records run from
  2026-09-17T18:02Z to 2026-09-30T07:46Z. They overlap Epoch 5's first records (from 2026-09-24), because the route
  interleaved the two epochs' chunks.
- **Coverage:** all 8 chunks carry the full playbook set (phase 5 · implement 3 · wrap 5). No checkpoint failed to
  fire. There were no 0-pending or adaptation wraps in range. Every friction chunk has step records.
- **Unparseable:** 0 of 2965 ledger lines. **Malformed `ts`:** 0 in this epoch. There are 11 ledger-wide, all in
  `2026-08-08-dependency-advisory-remediation` (0.2.0, a pre-recipe shell append). They are kept and listed in
  `q-retractions.json`.
- **Retractions (whole-ledger pre-pass):** 11 records retracted and 5 problem-facts retracted ledger-wide, none of
  them in this epoch. Clause-retracted: 3 in this epoch (`2026-09-23T09:51:34Z-d/-e/-f`). They are kept, and their
  notes are rendered in P3. 1 is unresolvable: a pre-boundary prose-form retraction, `id: null`, reading "the untyped
  code-graph-under-reports record and the first problem-block entry on…", which needs the founder's manual discount.
  **Retraction targeted by retraction → founder review:** `2026-09-10T19:52:10Z-a` (0.2.0 Epoch 6b; outside this
  epoch, reported because the pre-pass is whole-ledger).
- **Untyped rate per step:** new-session/orientation 3/19 · phase/research 2/13 · phase/take-up 1/5 ·
  implement/fix-loop 2/17 · implement/smoke 1/1 · wrap/reconcile 4/16 · wrap/route-resolve 1/8. Every other step is
  0. Total: 14/136.
- **Problem-fact fill:** 44/119 step records. **id fill:** 255/255.
- **Writer-convention drift (folded before grouping):** 13 records carry `version: "0.3.0"` rather than
  `conductor-0.3.0`. They are 10 phase and 3 new-session records, from 2026-09-29. The `skill` field mixes the bare
  form and the `andromeda-` prefix: 46 bare and 209 prefixed.
- **Calibration boundaries in range:** the deviation scan, the universal types and the required `id` all predate this
  epoch and apply throughout. **Reconcile's `contract.in-pass-correction` went live on 2026-09-27, mid-epoch.** The
  two earlier reconcile records of that class (`2026-09-23T20:58:44Z-c`, `2026-09-24T09:06:34Z-d`) are untyped, which
  is era, not evidence. Stage 2 reads them as members of that type, not as an extension candidate.

## Proposals (typed patterns)

### P1 — phase/validate · `contract.mechanical-check` — 8 cases · weight 21
**Pattern:** every validate run in the epoch (8/8, 6 chunks) recorded P5 catching defects that P4 authored. Six were
caught by mechanical predicates the P4 self-check (dry-run + planlint) does not run. Two were caught only by the
operator review, of a kind no predicate attempts.
**Evidence:** all 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | mechanical checks passed a plan the operator review found unsafe in 5 blocker-class ways + 13 defects (canned-SUT satisfiability, outcome/fault crossover, attribution, ref-flip on a Blocked pin, stdout capture) | dlg 1 · it 1 · deferred 1 | `.andromeda/runs/2026-09-22T21-39-08-phase/p5-review-operator.md` |
| CP | check 4(9) baseline: two multi-file count probes (`grep -c files \| grep -vc`) read GREEN on the untouched tree (stage-1 exit 2 masked by pipefail) | it 1 | 2026-09-23T20:33:32Z-b |
| DQ | check 4(4): 2 evidence files with no producing entry; 4(6): 3 criterion-named probes missing from the fence; check 5: 7 brace placeholders | it 1 | 2026-09-29T06:46:09Z-b |
| DQ | operator review caught a provenance misattribution (founder vs overseer's founder-delegated ruling) that no predicate attempts | dlg 1 · it 1 | 2026-09-29T06:46:09Z-c |
| DL | check 4(9): a multi-file CR count vacuous when both files are missing (host-win32 2026-09-23 class, rule already loaded), and an eol probe that the repo-wide wildcard satisfies | it 1 | 2026-09-29T18:45:12Z-b |
| HS | check 2: an Expected-amendments entry used the `matrix#id` ledger-note form for unverified v3-08 | it 1 | 2026-09-29T20:49:57Z-b |
| HS | check 4(4): criteria named evidence files with no producing entry | it 1 | 2026-09-29T20:49:57Z-c |
| RP | 5 defects the P4 self-check missed: 4(3) UI e2e not role self-verify; 4(6) ×2; check 2 matrix-notes on an unverified cap; check 5 brace spellings | it 2 | 2026-09-30T04:48:01Z-b |

**Proposal:** the same predicates fire repeatedly: 4(6) ×3, 4(4) ×2, 4(9) multi-file probe ×2, check 2 unverified
`matrix#id` ×2, and check 5 braces ×2. They fire at P5 because P4's self-check does not run them. One direction is to
move those P5 predicates into planlint, or the P4 dry-run, so that P4 stops authoring what P5 catches. For the
review-only class (IL, DQ-c), a review-prompt checklist line could cover criterion satisfiability by a canned SUT and
ruling provenance. Recurrence context, read-only: this type appears in every epoch since 0.2.0 Epoch 1. It was 7, 7
and 5 in 0.3.0 Epochs 1–3 and 19 in Epoch 5.

### P2 — implement/code · `input.plan-step-ambiguous` — 8 cases · weight 16
**Pattern:** every code run (8/8 runs, 5 chunks) met at least one plan step that was underdetermined, stale, or in
conflict with another step or with existing code. Two of the eight were jointly unsatisfiable pairs.
**Evidence:** all 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | B3 names preflight.rs:202-205 as the attribute-by-fingerprint precedent — a stale comment; the plan never says how the capture obtains the cue fingerprint | reads 6 | 2026-09-23T07:29:00Z-b |
| IL | the P5 fold's B1 overrides step 1's standing clause without the step text being updated; resolved by the fold's precedence clause | reform 1 | 2026-09-23T07:29:00Z-c |
| IL | B3 requires a pre-poll-resolved incident to be graded by id, but no MCP list tool returns resolved incidents and no id source is named | reads 2 | 2026-09-23T07:29:00Z-d |
| CP | step 2 named rstest `#[case]` rows while conductor-run has no rstest dev-dep, beside "No manifest change" + a Cargo.lock byte-identity gate — jointly unsatisfiable, operator-surfaced | dlg 1 | 2026-09-23T20:42:32Z-b |
| RC | size forecast assumed history removal alone reaches the threshold; OR needed ~1.5 KB more condensation, 5 draft iterations | it 5 | 2026-09-24T08:06:24Z-c |
| DQ | step 4 appends inside the rule markers while `rule_predates_the_drive` asserts exact equality with the 2026-09-23 rule — resolved as prefix-extension | reads 1 | 2026-09-29T06:59:06Z-b |
| DQ | three outcomes the plan left open, settled in-intent (missing P-031 section; deduped incident; non-ok parse) | — | 2026-09-29T06:59:06Z-c |
| HS | step 1 "become the record of the RETIRED instrument" without saying whether headings move; kept verbatim because the v3-07 ref counts headings | reads 1 | 2026-09-29T20:57:49Z-b |

**Proposal:** two shapes recur. (a) A P5 fold lands as a block with a precedence clause, and the affected step text is
left unedited (IL-c). A fold could rewrite the steps it overrides in place. (b) A step conflicts with something
already true of the code: a test's exact-equality pin, a missing dev-dep, a stale cited precedent. Validate could add
a check that each step naming a file, test or dependency agrees with that artifact at HEAD. Cross-step chains X2/X3
show where several of these entered: the research hand-off.

### P3 — new-session/orientation + wrap/route-resolve · `contract.grammar-irregularity` (universal, by type) — 10 cases · weight 12
**Pattern:** route.py printed UNPARSED/INDETERMINATE at four working-route sites. Two sites recurred
(`:48` ×4, `:57` ×4), and per the per-site rule that points at the grammar or its writer. Two were one-offs (`:46`,
`:54`) with the same writer shape: an ALL-CAPS word plus a colon inside a wrap's own new CONTEXT prose.
**Evidence:** all 10 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| NS | INDETERMINATE: working-route.md:48 — after ONE space — 'BLOCKED-ON:' | reads 1 | 2026-09-18T06:26:34Z-c |
| NS | UNPARSED: working-route.md:57 — introducer shape 'CARRY 2:' | — | 2026-09-18T06:26:34Z-d |
| NS | INDETERMINATE: :48 'BLOCKED-ON:' (second session) | — | 2026-09-20T22:55:54Z-b |
| NS | UNPARSED: :57 'CARRY 2:' (second session) | — | 2026-09-20T22:55:54Z-c |
| NS | INDETERMINATE: :48 'BLOCKED-ON:' (third session) | — | 2026-09-22T21:36:16Z-c |
| NS | UNPARSED: :57 'CARRY 2:' (third session) | — | 2026-09-22T21:36:16Z-d |
| IL | INDETERMINATE: :48 'BLOCKED-ON:' [clause retracted: evidence named `.andromeda/runs/2026-09-23T08-03-55-wrap/route-2026-09-22-interpretation-proven-live.json`, which does not exist; route.py ran without --marker, so the trail is `route-no-marker.json` in the same run dir] | — | 2026-09-23T09:51:34Z-d |
| IL | UNPARSED: :57 'CARRY 2:' [clause retracted: same note] | — | 2026-09-23T09:51:34Z-e |
| IL | INDETERMINATE: :46 'PREFLIGHT:' — this wrap's own new CONTEXT prose, rephrased the same pass [clause retracted: same note] | retries 1 | 2026-09-23T09:51:34Z-f |
| DQ | INDETERMINATE: :54 'CLEARED:' — an ALL-CAPS word in this wrap's own CONTEXT text, reworded to lowercase | retries 1 | 2026-09-29T18:16:45Z-b |

**Proposal:** the recurring sites were repaired by a route-resolve on 2026-09-23 (removed-cause fact
`2026-09-23T09:51:34Z-a#0`). Until then, three orientations re-recorded the same two sites with no channel to fix
them. The one-offs were introduced by the writer itself. One direction is a route.py lint verb that route-resolve and
take-up run on the line they drafted before writing it, so a writer catches the irregularity it authors. The clause
retractions add a second direction: when route.py runs without `--marker`, it could print its trail filename so the
evidence pointer cannot be guessed wrong.

### P4 — several steps · `contract.skill-reference-drift` (universal, by type) — 9 cases · weight 11
**Pattern:** nine pipeline letters disagree with the deployed tool, with a sibling reference, or with themselves. The
project's own drift detection cannot see these by construction.
**Evidence:** all 9 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| PG | curation-guide §Corrections prescribes a Session Additions entry quoting a false clause, but the arising case had the false clause INSIDE Session Additions (preserved verbatim, never re-derived), so an in-place edit is correct | reads 1 | 2026-09-18T10:11:50Z-b |
| PG | route-resolve.md gives opposite whole-file-Write risk guidance for the same file in two rows (reorder row vs flip-compaction paragraph) | — | 2026-09-18T10:18:22Z-b |
| IL | fan-out.md persist rule says the transport HTML-escapes entities; this batch returned 0 entities and a 2-space indent the rule does not name | — | 2026-09-22T21:57:19Z-b |
| IL | implement SKILL.md says run a leg=operator entry with `--only {n} --id {v}`; gate.py prints "not run — leg operator" for that exact call | retries 1 | 2026-09-23T07:56:00Z-b |
| IL | the four specialist summaries' header says "wrap-session does not modify" while amendment-flow's cascade table re-derives them | — | 2026-09-23T09:44:05Z-d |
| IL | rules-templates/host-win32.md renders "the quoted heredoc itself is sound below the cut", falsified for backslash pairs; rendered rule corrected, template source still false | — | 2026-09-23T09:48:36Z-e |
| RC | fan-out.md mandates a No-domain-coverage extract of 3 lines while validation check 5 requires ≥4 — a compliant extract fails by construction | — | 2026-09-24T06:28:31Z-c |
| RC | implement SKILL.md P2: void a defer by re-running `--only {n}`; gate.py printed "not run — defer (key)" and ran nothing | retries 1 | 2026-09-24T08:08:14Z-c |
| DL | SKILL.md P2's advisory-db currency test ("head equal to upstream") reads false on a current copy — cargo-audit fetches into FETCH_HEAD and never moves origin/main | reads 2 | 2026-09-29T19:02:06Z-b |

**Proposal:** a fix list by owning file. The implement SKILL.md ↔ gate.py contract has 2 cases (operator-leg route
and defer-void route), which are also level candidate L2. fan-out.md ↔ distill validation has 2 cases, also L4.
route-resolve.md has an internal contradiction, also L3. curation-guide §Corrections needs a Session-Additions arm.
The specialist-summary header text conflicts with amendment-flow. The host-win32 template source needs the corrected
clause. SKILL.md's currency test should name FETCH_HEAD. The project absorbed the last one on 2026-09-29 (security.md
Session Additions), and the pipeline letter is what remains. Recurrence: 9 in this epoch against 1–2 in each other
0.3.0 epoch.

### P5 — new-session/orientation · phase/distill · wrap/report · `contract.token-proxy-check` (universal, by type) — 7 cases · weight 13
**Pattern:** ad-hoc extractors keyed on a token the target's grammar does not use. Four were orientation extractors
over the working-route or CLAUDE.md, two were distill H2 cite checks keyed on the whole bold span, and one was a crate
name matched as a substring.
**Evidence:** all 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| NS | working-route extractor tested `###`/`##` but not the H1, so the title line read as a markerless ENTRY (false MARKERLESS) | — | 2026-09-17T18:02:24Z-b |
| NS | health extractor keyed CLAUDE.md markers as `GENERATED:x` while the file uses the start/end suffix form (false negative on checks 2, 14) | reads 1 | 2026-09-23T21:18:22Z-b |
| NS | route-title extractor keyed on a `- ` introducer; working-route entries are not list items (false EMPTY) | reform 1 · reads 1 | 2026-09-29T06:06:19Z-c |
| DQ | orchestrator H2 script took the whole bold span as the marker; 31 cites read unresolved (false positive), all 170 resolved on the marker token | reform 1 · reads 1 | 2026-09-29T06:24:21Z-b |
| NS | sibling-title extractor keyed on `- ` list items returned only epoch headers (false EMPTY) | retries 1 · reads 1 | 2026-09-30T04:13:20Z-b |
| RP | H2 probe mis-read twice (shell word-split of multi-word cites, 93 spurious; then whole-cite prefix match, 4) | retries 2 · reads 1 | 2026-09-30T04:28:12Z-c |
| RP | expected-amendment site count grepped `sha2` and matched `sha256` (false positive) | reform 1 | 2026-09-30T07:25:34Z-b |

**Proposal:** five of the seven hand-write an extractor for a document whose grammar a shipped tool already parses
(route.py for the working-route) or that recurs every run (the distill H2 cite check, also `2026-09-29T20:21:47Z-b`
under `contract.extract-format`). The same `- `-introducer mistake recurred on 2026-09-29 and 2026-09-30. Directions:
a route.py verb that prints entry titles, so orientation stops writing its own; and the H2 cite check shipped as a
script keyed on the marker token.

### P6 — wrap/curation (+ reconcile) · `recall.corpus-recurrence` (by type) — 7 cases · weight 11
**Pattern:** a curated rule that already covered the event failed to prevent it. Four of the seven are the same
measure-before-claiming family (CLAUDE.md T1 2026-08-09 "grep A before asserting A says X", and the 2026-09-06
false-positive extension).
**Evidence:** all 7 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| PG | the verification sweep's trailing `.{60}` anchor suppressed a line-final hit (1 site vs true 2); third instance of one family in the session | reads 1 | 2026-09-18T10:08:41Z-b |
| IL | host-win32 2026-09-08 cd-persistence entry did not prevent a cd re-basing later calls (implement + 3× in the wrap) | — | 2026-09-23T09:48:36Z-c |
| IL | the COMPLETION axis (T1 2026-08-09 / 2026-08-31) did not prevent A17 reaching validate as "code reading only" while committed envelopes measured it | reads 3 | 2026-09-23T09:48:36Z-d |
| DQ | an agent-launched pulse-app omitted the model env the Tier-3 Pulse run recipe names | retries 1 | 2026-09-29T18:13:36Z-b |
| DL | an evidence line claimed the a11y job ran on windows-latest without reading ci.yml (pins windows-2022); T1 2026-08-09 covers it | reads 1 | 2026-09-29T19:26:10Z-b |
| HS | a grep result was written into fanout-results check 6 before the grep ran, and was false; T1 2026-08-09 | retries 1 | 2026-09-29T21:41:18Z-b |
| RP | the T1 false-positive entry recurred twice (an e2e log grep matched injected axe source; `sha2` matched `sha256`) | reform 2 | 2026-09-30T07:40:37Z-c |

**Proposal:** a prose extension in the corpus did not stop recurrence of this family. It now carries many dated
extensions on one line, and P9 is its typed sibling. A direction at a different level is to make the writer steps
(report, fanout-results, evidence lines) refuse a stated count or "file says X" claim that lacks its producing command
beside it. That turns the rule into a write-time check rather than a recall. Recurrence: 2–16 per epoch since 0.2.0
Epoch 5.

### P7 — phase/research (+ fix-loop) · `contract.premise-falsified` (universal, by type) — 8 cases · weight 9
**Pattern:** research or a leg falsified a premise an authored artifact stated. Four of the six research cases came
from one chunk (IL), and all four trace to the posture contract the previous chunk (PG) authored without a drive.
**Evidence:** all 8 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| PG | verification-harness.md 2026-08-16 entry says the deterministic fixture pins evidence_refs to []; at Pulse 83d4060 it populates a det-* triple | reads 1 | 2026-09-18T07:23:26Z-c |
| IL | posture clause (1) "a single-storm leg has no hypothesis observable" is false — parsed L4 output attached at creation (inference_runtime.rs:884) | reads 6 | `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/research.md` P-B |
| IL | the "~110 s real-model formation" figure is a deterministic-L4 measurement re-attributed to the real model, carried into 4 artifacts | reads 5 | same, P-E |
| IL | CONTEXT's "no new machinery" fails: the posture launch is unreachable at HEAD and nothing persists the report text | reads 6 | same, P-A/P-F |
| IL | the posture's null-verdict disposition holds only on failure paths; a non-degraded read-back lands ManualCheck | reads 2 | same, P-D |
| HS | the grading rule forecast the end-of-storm fall after the dot hid; measured IN-window (430.79 ms) | — | `conductor-0.3.0/chunks/2026-09-29-hue-shift-budget-graded-hard/evidence/hue-verdict.md` |
| RP | folded a11y amendment :42 (BiDi Page/Frame not ready) was the scope's CI-red hypothesis; the log shows axe's 1000 ms FRAME_LOAD_TIMEOUT | reads 2 | `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/research.md` |
| RP | research/scope premised the workspace key as the data dir's BASENAME; Pulse's workspace_key is a full PATH | it 1 | 2026-09-30T06:19:14Z-b |

**Proposal:** research falsifying premises is designed behaviour. This type appears in every epoch, and every case
here was caught before damage. The level-relevant shape is IL's four: a contract authored against an external SUT
with no drive (PG) was consumed as settled. One direction is for such an authoring chunk to tag each SUT-behaviour
clause `[unmeasured]`, so that the consuming research re-verifies the tagged clauses as a list rather than finding them
one at a time.

### P8 — several steps · `tooling.host-shell` (universal, by type) — 5 cases · weight 11
**Pattern:** the Bash tool transport or host shell semantics corrupted a command. The causes were doubled-backslash
collapse ×2, cd persistence, a PowerShell `$_` expanded inside a bash double-quoted string, a python regex `chr(92)`
issue, and an inline python quote collision.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | a single-quoted printf's doubled backslash collapsed; a drive-letter control became `C:<form-feed>ake`, every pattern read 0 | retries 1 · reads 1 | plan.md entry 16 baseline (2026-09-23T05:27:17Z-c) |
| IL | a quoted-heredoc python edit's doubled-backslash `n` collapsed; two Rust literals received real newlines | retries 1 | 2026-09-23T07:56:00Z-c |
| RC | a `cd` into the phase run dir persisted into the next call (recurrence of host-win32 2026-09-08) | retries 1 | 2026-09-24T06:34:29Z-b |
| DQ | PowerShell `$_` in a bash double-quoted `-Command` expanded by bash; an inline python regex with a lone `chr(92)` failed | retries 2 | 2026-09-29T17:37:55Z-f |
| DL | inline `python -c` with an empty-string dict key died on a quote collision | retries 1 | 2026-09-29T18:37:32Z-c |

**Proposal:** see level candidates L1 (the workarounds this class forces) and L6 (chronic across 10 epochs). Four of
the five recoveries were the same move: a Write-tool edit or a script file run by path. The cd case was re-anchored
instead. The direction is to make that
the default form in the skill letters for anything beyond a one-line probe, rather than the recovery. Every record in
range comes from the Windows/MSYS host era.

### P9 — wrap/report (+ take-up, reconcile) · `contract.narrow-basis-claim` (universal, by type) — 5 cases · weight 7
**Pattern:** a figure or absence claim rested on a narrower source than it stated. Causes were an older reading, a
clipped or narrower grep, a hand count from a sed dump, and a doc-agent framing.
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| PG | advisory delta computed vs a 2026-09-10 reading while two newer existed; 986-test count attributed to a chunk recording no count | reads 6 | 2026-09-18T09:47:02Z-b |
| PG | a `grep … \| tail -8` clip nearly supported a false absence; a site count of 2 was asserted without running the grep (measures 1) | reads 2 | 2026-09-18T09:47:02Z-c |
| IL | A17-A19 framed "code reading only, not live-measured"; one grep of committed evidence found ten live envelopes measuring it | reads 3 | `.andromeda/runs/2026-09-23T08-03-55-wrap/reconcile-sweep.md` |
| DQ | scope.md cited moved line ranges from hand-counting a sed dump; `grep -n` showed the originals held | reform 1 · reads 1 | 2026-09-29T06:18:09Z-b |
| HS | the sweep line cited "architecture ×4" from a `grep -c` narrower than the listing beside it (6 lines) | retries 1 | 2026-09-29T21:32:59Z-b |

**Proposal:** this is the same family as P6. The record itself names it a recurrence despite the learning
(host-win32's clipped-sweep clause and the report template's stated-number rule). P6's write-time direction (a stated
number carries its producing command) would cover this class too.

### P10 — wrap/reconcile · `ambiguity.playbook-no-match` — 3 cases · weight 9
**Pattern:** a proposal that the plan's P5-approved direction or Expected amendment had already sanctioned matched no
wrap-playbook rule. Each case took an operator dialogue and minted or extended a rule. One of the three reconcile steps (IL) halted.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | security S1/S2 (READ-SET posture arm), S4 (closed unit enum fails rule 112's garde qualifier), S5 (CLI flag) had no rule; applied under P5-approved direction, three rules minted with refinements | dlg 1 | 2026-09-23T09:44:05Z-c |
| CP | D-security-input ×5 matched :134 except its route-entry clause for journal_conformance.rs; applied on P4 direction, :134 extended | dlg 1 | 2026-09-23T20:58:44Z-b |
| RC | registering scripts/arch-registry-check.py in the Stack row matched no rule (:171 precondition fails); applied under the P5-approved Expected amendment, rule minted | dlg 1 | 2026-09-24T09:06:34Z-b |

**Proposal:** cross-step chain X1 shows the wrap playbook consumed as `thin` in 4 chunks. Each time, the amendment
was already approved upstream. One direction is to let a P5-approved Expected amendment be its own governing basis at
reconcile. The playbook rule would then become optional, minted at leisure rather than through a halt-and-dialogue.

### P11 — new-session/orientation · `tooling.health-false-red` — 5 cases · weight 5
**Pattern:** health-check arms authored ad hoc each session reported false reds. Three were on check 11's template
currency arm, with three different extraction mechanisms. The others were check 8 (an exact-match `.gitignore`
fragment) and check 6 (an mtime signal erased by consolidation).
**Evidence:** all 5 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| NS | check 11: cookbook template is not fenced, so the fence regex measured an inner code block; a direct diff is byte-identical | reads 2 | 2026-09-18T06:26:34Z-b |
| NS | check 8 exact-matched `target/` and `/target` against .gitignore lines; the file carries `/target/` | — | 2026-09-22T21:36:16Z-b |
| NS | check 11: compared the whole two-fence views.sql.md template against the script; a fence-wise diff shows only a trailing blank | reads 1 | 2026-09-23T16:20:07Z-b |
| NS | check 11: a greedy end-anchored regex spanned both fences of the two-fence template; first-fence re-extraction showed 0 diff | reads 1 | 2026-09-24T10:39:09Z-b |
| NS | check 6 read arch.md 28h newer than CLAUDE.md with no recent sidecar; git shows co-amendment, and the 0-pending consolidation rewrote every sidecar's mtime | reads 1 | 2026-09-29T06:06:19Z-b |

**Proposal:** check 11's comparison could ship as a fence-aware script in the new-session skill, covering
unfenced, one-fence and two-fence templates. Then the agent no longer re-authors it each session. Check 6's mtime
signal could read git co-amendment instead of mtime. Chronic across six epochs (L6).

### P12 — implement/fix-loop · `contract.instrument-validity` — 3 cases · weight 7
**Pattern:** three instruments a chunk built measured the wrong thing. In two of them (DQ), only Pulse's own log
exposed the error after a graded drive. One (RP) was exposed by its known-positive inverse control reading green.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| DQ | attribution sweep anchored at the lowest ACTIVE incident id never read auto-resolved incidents below it; drive a3 graded NoAttributableIncident | it 1 | `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/attempt-ledger.md` |
| DQ | canary pairing closed each digest at the next cadence tick of ANY kind; b1/b2 printed as pipeline-fault | it 1 | same |
| RP | the step-9 stall arm passed with the readiness wait bypassed (inverse control GREEN); the stall was absorbed by the scheduling execute | it 2 | `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/race-witness.md` |

**Proposal:** RP's inverse control is the mechanism that worked. The DQ pair had no equivalent before a graded drive.
A plan-template direction is to require a known-positive control for every new measuring instrument, run before its
first graded use.

### P13 — wrap/reconcile · `contract.in-pass-correction` — 3 cases · weight 7
**Pattern:** corrections made inside the reconcile pass before apply: a grep cited before it ran, a registry-size
overrun caught by tooling, and a prompt builder that parsed a detector's doc field as its first token only.
**Evidence:** all 3 cases (all after the type's 2026-09-27 deploy) —
| chunk | what | impact | evidence |
|---|---|---|---|
| HS | fanout-results check 6 first cited a grep (f0c38f5 in obs-plan/architecture) before running it; it returned 0 in all seven | retries 1 | 2026-09-29T21:39:38Z-b |
| RP | the first architecture apply put Established Decisions 76 B and Occupied Resources 15 B over target; D-arch-registry-size caught it, two trims | it 2 | `.andromeda/runs/2026-09-30T07-22-03-wrap/fanout-results.md` |
| RP | the prompt builder parsed a detector's `doc:` field as its first token, scoping D-platform-claim to architecture alone; a per-prompt detector count caught it | it 1 | `.andromeda/runs/2026-09-30T07-22-03-wrap/prompt-architecture.md` |

**Proposal:** the prompt-builder case is a pipeline script defect. It could parse the full `doc:` list, or assert the
per-prompt detector count itself. The HS case belongs to the P6/P9 family. Era note: the pre-deploy members
`2026-09-23T20:58:44Z-c` and `2026-09-24T09:06:34Z-d` sit untyped (see Mechanism health).

### P14 — several steps · `contract.structural-blind-spot` (universal, by type) — 3 cases · weight 5
**Pattern:** three documented mechanisms could not reach their subject by construction. The posture contract is
silent on the data dir's NAME. The hygiene read never opens tool trails. No drift-base detector covers a repo-root
file-set or a license-policy change.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| DQ | the posture contract's launch terms say nothing about the data dir NAME; a digit-bearing basename tripped Pulse's credit_card scrubber; founder ruling: v3-09 not met with this cause | dlg 1 | `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/attempt-ledger.md` |
| DQ | gate.py hygiene read the run dir clean while 8 tool trails + the implement ci trail held 65 absolute host paths; second consecutive wrap | reads 2 | `.andromeda/runs/2026-09-29T17-50-46-wrap/` |
| DL | all 7 detectors returned no proposals against 3 planned amendments; no detector covers a repo-root file-set or deny.toml policy-scope change | reads 2 | `.andromeda/runs/2026-09-29T19-16-23-wrap/fanout-results.md` |

**Proposal:** the hygiene case is pipeline-level. Either gate.py hygiene reads tool trails, or the tools write
repo-relative paths at source. The fact `2026-09-29T18:22:32Z-a#0` shows a scratchpad rewrite standing in, "as the
prior wrap did". The detector case pairs with chain X6 (drift-base `thin` in 2 chunks), and the direction is
drift-base detectors for those two change classes. The DQ case is project-level and already ruled.

### P15 — implement/fix-loop · `tooling.environmental` — 2 cases · weight 6 (halt/soft-exit threshold)
**Pattern:** a live leg started in an environment that could not produce a reading: a missing model env, and a
driver/runtime major mismatch. One case cost a soft exit.
**Evidence:** both cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| DQ | agent-launched pulse-app set no model paths; every inference errored model_not_configured; model files found by disk search; a1 re-fired once | retries 1 · reads 6 | `conductor-0.3.0/chunks/2026-09-29-diagnostic-quality-cluster-off-the-drift-pin/evidence/attempt-ledger.md` |
| RP | the --e2e driver/runtime pair went incoherent (driver 152 vs runtime 154); the inverse-control run failed at session creation, one release build + leg for no reading | soft_exit 1 | `conductor-0.3.0/chunks/2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/evidence/race-witness.md` |

**Proposal:** a pre-leg environment check in the harness could run before any live leg: model env present, and
driver major equal to runtime major. CI already derives the driver from the runtime, so this would be the dev-host
counterpart. The DQ case also appears in P6 (`2026-09-29T18:13:36Z-b`) as a recall failure.

### P16 — wrap/curation · `ambiguity.filter-borderline` — 3 cases · weight 4
**Pattern:** in every case, the deciding candidates scored exactly 0.6 (2, 2 and 3 candidates) and were settled by a
judgment call.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | both surviving candidates scored exactly 0.6 before Filter 4; one passed on the load-bearing call alone | — | 2026-09-23T09:48:36Z-b |
| DQ | two candidates scored exactly 0.6, rejected because their facts took route or master homes | — | `.andromeda/runs/2026-09-29T17-50-46-wrap/curation.md` |
| RP | three candidates exactly at 0.6, rejected; one lives only in a reader-less contract section, which the no-other-home signal's list does not name | reform 1 | 2026-09-30T07:40:37Z-b |

**Proposal:** seven candidates landed exactly on the threshold, which suggests the score's granularity puts
candidates on the line. One direction is to state the tie rule explicitly, and to add "contract section" to the
no-other-home list.

### P17 — phase/research · `tooling.graph-symbol-missing` — 3 cases · weight 3
**Pattern:** the fresh rust plane lacked symbols from feature-gated (`live-pulse`) test files (2 cases) and one
generic `pub async fn`. Callers were settled by grep.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| PG | `execute_scenario` (generic pub async fn, execute.rs:44) absent from the symbol view; 15 grep hits | reads 2 | `.andromeda/runs/2026-09-18T07-01-01-phase/tree-query-2026-09-18-real-model-leg-posture-and-grading-rule.json` |
| CP | runs_dir/pulse_log/capture/journal_of in live_suite.rs / real_model_live.rs (cfg feature live-pulse) absent; 12 rows were name collisions | reads 1 | `.andromeda/runs/2026-09-23T19-58-21-phase/tree-query-…corrected.json` |
| RP | real_model_live.rs (live-pulse-gated) contributes no refs; uses found only by grep | reads 1 | `.andromeda/runs/2026-09-30T04-14-05-phase/tree-query-…data-dir.json` |

**Proposal:** chain X5 shows tree-db consumed as `thin` in 2 chunks. One direction is for the rust-plane indexer to
build with the project's feature set (at least `live-pulse`), and to check generic-fn indexing. It is a pipeline
script change in `scripts/code-graph.py` or its template.

### P18 — phase/take-up · `input.out-of-pipeline-source` — 3 cases · weight 3
**Pattern:** decisive scope facts lived outside every artifact take-up reads: the operator's auto-memory, the sibling
Pulse repo's chunk report, and an overseer relay file plus the sibling repo's HEAD.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | the attended-seam convention and the real-model launch recipe read from the operator's auto-memory; folded as an [inferred] bullet | reads 1 | `.andromeda/runs/2026-09-22T21-39-08-phase/` |
| HS | the decisive facts (shipped field name, witnessed-only emission, the 9986 ms rise) lived only in the Pulse repo's chunk report | reads 3 | 2026-09-29T20:14:24Z-b |
| RP | a relay coordinate (conductor-wrap-50-2026-09-29 §2.3) not resolvable here, and the blocker state read from the sibling repo's HEAD | reads 3 | 2026-09-30T04:18:58Z-b |

**Proposal:** cross-project facts now arrive through relays and sibling repos as a matter of course. One direction is
a sanctioned external-inputs channel: the relay or directive is persisted into the chunk folder as a take-up input
with its source named. The untyped `2026-09-30T04:18:58Z-c` (directive W182 living in no rule) is the same shape.

### P19 — phase/plan · `input.research-thin` — 3 cases · weight 3
**Pattern:** plan synthesis found SUT facts and claim-sweep sites that research had not established.
**Evidence:** all 3 cases —
| chunk | what | impact | evidence |
|---|---|---|---|
| IL | P3 did not establish Pulse's incident idle criterion; the emission design moved from an 8-burst keep-alive to 3 bursts + a polling capture | reads 2 | `conductor-0.3.0/chunks/2026-09-22-interpretation-proven-live/plan.md` |
| IL | P3 did not record that query_incident_list items carry no service field; attribution moved to the span line's ms timestamp | reads 1 | same |
| CP | research's line-granular sweep listed 9 sites; a P4 multi-line probe found a tenth and two wrapped sites | reads 2 | 2026-09-23T20:25:08Z-b |

**Proposal:** see chain X2: research consumed as `thin` in 3 chunks, each time from a producer that signalled
`unresolved-questions` and still closed `ok`. One direction is a P3 checklist line for SUT-interaction chunks:
enumerate the fields each read-back tool returns. Another is for claim sweeps to default to a multi-line form.

## Cross-step chains (starting heuristics)

### X1 — (pipeline artifact, no producing step) →wrap-playbook→ wrap/reconcile — 4 chunks
Consumer records: PG `2026-09-18T10:08:41Z-a` (48 rules, none for a new reader-less contracts/ member), IL
`2026-09-23T09:44:05Z-a` (S1/S2/S4/S5), CP `2026-09-23T20:58:44Z-a` (:134 route-entry clause), and RC
`2026-09-24T09:06:34Z-a` (operator-instrument registration). Two of the four reconcile steps (PG, IL) halted-resolved. **Hypothesis:** the playbook is grown one rule per encounter, after the upstream approval
already exists. Direction: P10.

### X2 — phase/research →research→ phase/plan — 3 chunks
The producers (`2026-09-22T22:16:00Z-a`, `2026-09-23T20:07:00Z-a`, and DQ's research step) each closed `ok` while
signalling `unresolved-questions`. The consumers (`2026-09-22T22:37:47Z-a`, `2026-09-23T20:25:08Z-a`,
`2026-09-29T06:41:01Z-a`) marked research `thin`. **This is the formally-ok-but-signalled shape**: the signal was
raised and the plan still had to close it with synthesis-time reads. Direction: P19. It could also let an
`unresolved-questions` signal block the `ok` close, or carry a named list P4 must discharge.

### X3 — phase/plan →plan→ implement/code — 2 chunks
CP: plan `ok` with `designed-dialogue` and `authority-resolved` signals feeds the rstest/no-manifest conflict
(`2026-09-23T20:42:32Z-a`). DQ: plan `ok` with 4 designed-dialogue/authority signals feeds the exact-equality conflict
and 3 open outcomes (`2026-09-29T06:59:06Z-a`). **Hypothesis:** the plans that carried the most resolved forks are the
ones code found underdetermined. Direction: P2.

### X4 — implement/smoke →implement-outcome→ wrap/report — 2 chunks
PG: smoke `ok` (green) but the outcome mis-stated two measurements (report `2026-09-18T09:47:02Z-a`; the friction is
P9). RP: smoke `ok-degraded` with `deferral-open`, and the report was superseded by work in the same window
(`2026-09-30T07:25:34Z-a`). These are two different mechanisms, so there is no single direction.

### X5 — (code-graph) →tree-db→ phase/research — 2 chunks
PG `2026-09-18T07:23:26Z-a`, CP `2026-09-23T20:07:00Z-a`. Direction: P17.

### X6 — (drift-base) →drift-base→ wrap/reconcile — 2 chunks
DL `2026-09-29T19:24:17Z-a` (no detector for a repo-root file set / deny.toml policy) and HS `2026-09-29T21:39:38Z-a`
(no obs detector for a retired SUT-measurement claim; filed under the nearest fit). Direction: P14.

Single-chunk shapes (not aggregated): distill→extracts→research (IL), plan→plan→validate `wrong` (IL), take-up→working-route→route-resolve (IL),
plan→plan→fix-loop (CP), validate→plan→gates (DL), plan→plan→report (HS), smoke→conversation→curation (DQ),
handoff→take-up (DQ), handoff→orientation (NS). IL is the one chunk where distill, research, plan and validate all
recorded a downstream `thin`/`wrong`. Transitive chains are deliberately not scored.

## Level candidates (systemic-masked-as-project)

### L1 — band-aid — 9 facts (T1: the Bash guard hook and transport force a re-route)
**Facts:**
- `2026-09-18T06:26:34Z-a#0` · NS · orientation · environment/workaround — a 7285-byte cat-heredoc health script was
  refused on both guard arms while auto mode directs Bash for file writes. It was re-authored with Write and run by
  path.
- `2026-09-23T05:27:17Z-a#2` · IL · validate · environment/workaround — a printf control came out as `C:<form-feed>ake`.
  It was re-authored with Write.
- `2026-09-23T09:48:36Z-a#1` · IL · curation · environment/workaround — a backslash grep spot-check read back 0
  through the transport. It was verified by a Write-tool script.
- `2026-09-24T08:06:24Z-a#1` · RC · code · process/workaround — the hook blocked a cat-heredoc patch script. It went
  through Write.
- `2026-09-29T18:11:24Z-a#0` · DQ · reconcile · environment/workaround — the hook blocked a cat-heredoc append. It was
  appended with Edit.
- `2026-09-29T18:11:24Z-a#1` · DQ · reconcile · environment/workaround — the hook blocked an inline python edit with
  a doubled backslash. It was edited with Edit.
- `2026-09-29T20:57:49Z-a#0` · HS · code · environment/workaround — the hook blocked a host-path probe with a doubled
  backslash. It was re-run without the class.
- `2026-09-30T04:18:58Z-a#1` · RP · take-up · environment/workaround — the hook blocked a Windows-path grep. It was
  rewritten without backslashes.
- `2026-09-30T04:48:01Z-a#0` · RP · validate · environment/workaround — the hook blocked the P5 control script. It
  moved into a Write-tool script.

There are typed correlates: `tooling.host-shell` ×5 (P8) and `tooling.hook-friction` ×2. Prior epoch: Epoch 3 had
environment workarounds of the same host-shell class (`2026-09-16T17:16:33Z-a#0`, `2026-09-17T14:31:28Z-a#0`).
**Level hypothesis:** the cause appears to live in the transport and harness layer. One layer collapses backslashes,
and a project hook that guards against it refuses the forms the session's auto-mode guidance prefers. The fixes so
far live in the project: the host-win32.md extensions and the hook itself, absorbed one chunk at a time. Two of the
facts (`06:26:34Z-a#0`, and the take-up fact `2026-09-18T07:05:35Z-a#0`) name the tension directly: auto mode says
prefer Bash, while the guard or the Edit tool's Read-first rule require the dedicated tool.
**Proposal:** a pipeline-level direction is for the skill letters to name the Write-tool-plus-script-by-path route as
the first form for any multi-line or backslash-bearing payload, so the hook never needs to fire. Whether the hook's
arms still fit the Linux dev host is a separate check for the founder.

### L2 — band-aid — 5 facts (T2: gate.py cannot drive an entry the letter says it drives)
**Facts:**
- `2026-09-23T07:56:00Z-a#1` · IL · fix-loop · process/workaround — gate.py refused the operator-leg status smoke
  even with `--id`. It was run by hand.
- `2026-09-23T20:44:24Z-a#0` · CP · fix-loop · process/workaround — `${S}` was parsed as a required env handle. The
  entry was driven by hand from a script file, with a known-positive control.
- `2026-09-23T21:04:05Z-a#0` · CP · gates · process/workaround — the same entry was skipped via `--skip` and driven
  by hand.
- `2026-09-24T08:08:14Z-a#0` · RC · fix-loop · process/workaround — the delta tool voided a deferral on a basename
  false positive, and `--only 8,9` still reported "not run — defer". nextest and clippy were driven by hand.
- `2026-09-24T09:11:27Z-a#0` · RC · gates · process/workaround — gate.py still refused defer-keyed 8/9. They were
  driven by hand.

Prior epoch: `2026-09-16T11:40:17Z-a#0` (defer not voidable, run by hand) and `2026-09-16T19:18:09Z-a#0` (the `$NAME`
env predicate refused inline pwsh). Typed correlates: P4's two gate.py drift rows, `tooling.gate-deferral`
`2026-09-24T08:08:14Z-b`, and the untyped `2026-09-23T20:44:24Z-b`.
**Level hypothesis:** three gate.py behaviours are at fault: the env predicate reads in-command shell variables as
launch handles, a voided defer stays unrunnable by `--only`, and the operator-leg mode is refused. Every chunk hits
them and drives the entry by hand beside the tool. The fix lives in each chunk's report; the cause lives in the tool.
**Proposal:** a gate.py change could treat a `${NAME}` assigned in the same run string as local, add a run verb that
honours a voided defer, and align the operator-leg mode with the implement letter (or the letter with the tool).

### L3 — band-aid — 3 facts (T3: flip-compaction has no route.py verb)
**Facts:**
- `2026-09-18T10:18:22Z-a#0` · PG · gates · process/workaround — an anchored Edit was used instead of the prescribed
  whole-file Write (~14 KB of freight).
- `2026-09-23T09:59:23Z-a#1` · IL · gates · process/workaround — a scratchpad python whole-file rewrite was used,
  because route.py has no compaction subcommand.
- `2026-09-24T09:11:27Z-a#1` · RC · gates · process/workaround — a scratchpad script did archive-before-strip.

Correlate: P4's route-resolve.md self-contradiction (`2026-09-18T10:18:22Z-b`).
**Level hypothesis:** every wrap that compacts re-implements the operation by hand, a different way each time.
**Proposal:** a `route.py compact` verb (archive-before-strip, single-line scope) that the letter names, replacing
the whole-file-Write prescription.

### L4 — band-aid — 6 facts (T4: distill extract validation vs actual extract shapes)
**Facts:**
- `2026-09-18T07:16:39Z-a#0` · PG · distill · process/workaround — the entity decode was done by hand while
  transcribing, instead of one scripted decode.
- `2026-09-18T07:16:39Z-a#1` · PG · distill · process/workaround — an extract was persisted to the raw-twin path
  before its entity status was known, then moved.
- `2026-09-22T21:57:19Z-a#0` · IL · distill · environment/workaround — the transport indented all 7 reports by 2
  spaces. The indent was stripped outside fan-out.md's strip rule.
- `2026-09-23T20:03:51Z-a#0` · CP · distill · process/workaround — the design extract failed check 5 by one line
  (3 < 4) and was accepted without the prescribed retry.
- `2026-09-24T06:28:31Z-a#0` · RC · distill · process/workaround — the same check-5 vs prompt-form conflict was
  accepted as passing.
- `2026-09-24T06:28:31Z-a#1` · RC · distill · process/workaround — an anchor sat on a nested child line, and the item
  was judged anchored by reading it rather than by the line-grain probe.

Correlates: P4 rows `2026-09-22T21:57:19Z-b` and `2026-09-24T06:28:31Z-c`; `contract.extract-format`
`2026-09-22T21:57:19Z-c`, `2026-09-29T20:21:47Z-b`; P5's two H2 cases; `retry.distiller-respawn`
`2026-09-24T06:28:31Z-b`, `2026-09-30T04:28:12Z-b`. Prior epoch: `2026-09-16T10:47:25Z-a#0`.
**Level hypothesis:** fan-out.md's prompt forms, its persist/strip rule and its validation checks disagree with each
other and with what the transport returns. Each distill run reconciles them by hand.
**Proposal:** align fan-out.md in one pass. Set the No-domain-coverage floor to the prompt's own form, name indent
framing in the strip rule, and make the anchor and H2 checks item-grain and marker-token keyed. A shipped
decode-and-check helper would replace the per-run hand transforms.

### L5 — override — 2 facts with halt impact (take-up's BLOCKED-ON head rule)
**Facts:**
- `2026-09-29T06:18:09Z-a#0` · DQ · take-up · process/overridden — the operator cancelled the prior take-up's
  `--chunk` skip of three entries under a founder ruling that nothing is skipped. The phase was rolled back and the
  BLOCKED-ON entry taken up. The step was halted-resolved, halted 1.
- `2026-09-30T04:18:58Z-a#0` · RP · take-up · process/overridden — the BLOCKED-ON head was taken up anyway on the
  invocation argument (the same founder ruling), and the drive series carries the block as its gate. The step was
  halted-resolved, halted 2.

**Level hypothesis:** the phase rule (a BLOCKED-ON head halts, and the remedy is to skip) and a standing founder
policy (nothing is skipped or deferred) disagree. The operator corrects the same call at every take-up the policy
touches. Read-only lookback: Epoch 5 holds one more take-up halted-resolved.
**Proposal:** encode the policy as a take-up mode in the phase skill: take up a BLOCKED-ON head with the block
carried as the drive gate. Then the rule and the policy agree, and the halt-and-override stops recurring.

### L6 — chronic-degrade — three `tooling.*` specifics, never halting
**Facts (per-epoch counts, read-only lookback):**
- `tooling.host-shell`: 0.2.0 E3:14 · E4:13 · E5:3 · E6a:4 · E6b:8; 0.3.0 E1:10 · E2:2 · E3:2 · **E4:5** · E5:9.
- `tooling.health-false-red`: 0.2.0 E1:1 · E3:4 · E4:1 · E6a:1; 0.3.0 E2:1 · **E4:5**.
- `tooling.gate-deferral`: 0.2.0 E1:3 · E2:9 · E3:8 · E5:4 · E6a:2 · E6b:2; 0.3.0 E1:3 · E3:2 · **E4:1** · E5:6.

**Level hypothesis:** these are silent degradations the halt policy never surfaces. Each recurrence costs a retry or
a few reads, and none ever stops a chunk.
**Proposal:** the in-epoch evidence and directions are P8/L1 (host-shell), P11 (health-false-red) and L2
(gate-deferral). This entry records that each has persisted across versions, which is a reason to fix it at the tool
rather than add another project rule.

### L7 — removed-cause observation — advisory-db local clone residue (T5)
**Facts:** `2026-09-23T09:59:23Z-a#0` · IL · gates · environment/removed-cause. An untracked placeholder
`crates/connectrpc/RUSTSEC-0000-0000.md` was left in the local advisory-db clone after an upstream rename, and was
moved aside by hand. Prior: `2026-09-05T16:56:23Z-a` (0.2.0 Epoch 6b), the residue that ended a 4-week deferral.
Correlates: `tooling.light-gate-red` `2026-09-23T09:59:23Z-b`, and P4's FETCH_HEAD row (same probe). Read-only
lookahead: Epoch 5 records the same residue again (`2026-10-01T23:22:23Z-a/-c`).
**Level hypothesis:** cargo-audit's fetch-into-existing-clone leaves untracked files whenever upstream renames an
advisory. The probe detects the residue, and a human removes it each time.
**Proposal:** the currency probe could self-heal. It could clean untracked advisory files in the cache clone, or use a
fresh `--db` clone for the gate, so the residue stops reaching the light gate.

**Deferred-forever:** no hit. All six deferral facts in range close or name their destination. See the appendix and
`q-level.json` → `deferrals_checked`.

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — new-session/orientation — 3 cases → proposed type `input.version-dir-path-assumed`
**Cluster:** `2026-09-18T06:26:34Z-e` (matrix.py coverage `--dir` pointed at `.andromeda/conductor-0.3.0`; "neither
the skill body, the matrix contract nor route.py cursor says where version_dir resolves"), `2026-09-23T21:18:22Z-c`
(the same wrong path printed a plausible "no verification matrix — coverage off" verdict, although route.py cursor
had printed the version dir's location), and `2026-09-30T04:13:20Z-c` (a hand read assumed
`.andromeda/working-route.md`, FileNotFoundError). **Draft criteria line:** *record `input.version-dir-path-assumed`
when an orientation read or tool call resolves `{version_dir}` under `.andromeda/` instead of the repo root, naming
whether any consulted letter or tool output stated the location.* The cheaper fix may be the letter rather than the
type: the new-session skill body could state `{version_dir}` = `<repo-root>/{project}-{version}`. Case 1 says no
letter does.

## Below threshold — no action
**Typed groups (per step, n < threshold).** A group marked → Pn is counted inside a universal or by-type proposal
above.
- implement/fix-loop/tooling.host-shell 2 → P8 · phase/research/tooling.host-shell 2 → P8 · phase/validate/tooling.host-shell 1 → P8
- phase/distill/contract.token-proxy-check 2 → P5 · wrap/report/contract.token-proxy-check 1 → P5
- implement/fix-loop/contract.premise-falsified 2 → P7
- wrap/curation/contract.skill-reference-drift 2 · phase/distill/… 2 · wrap/gates/… 1 · wrap/reconcile/… 1 → P4
- phase/take-up/contract.narrow-basis-claim 1 · wrap/reconcile/contract.narrow-basis-claim 1 → P9
- implement/fix-loop/contract.structural-blind-spot 1 · wrap/gates/… 1 · wrap/reconcile/… 1 → P14
- wrap/reconcile/recall.corpus-recurrence 1 → P6
- wrap/gates/tooling.light-gate-red 2 — advisory-db residue (→ L7) and HEAD-anchored probes after the pre-CI commit (T9)
- phase/distill/retry.distiller-respawn 2 — anchors / H2 marker abbreviation (→ L4 correlate)
- implement/code/tooling.hook-friction 2 — cat-heredoc refused; rustfmt re-wrap broke the next Edit's old_string (→ L1 correlate)
- phase/plan/retry.synthesis-rework 2 (HS only) — the window-open instant, and a probe keyed on a term step 3 told implement to write
- phase/distill/contract.extract-format 2 → L4 correlate
- wrap/route-resolve/contract.no-sanctioned-channel 2 — test-plan :335 (closed 2026-10-02) and residuals.md :11/:15 (routed to the handoff, no later record)
- wrap/curation/ambiguity.tier-routing 2 — over-cap rule files force Tier-3 routing (with T6)
- implement/fix-loop/tooling.gate-deferral 1 → L2 · implement/fix-loop/contract.spec-reality-gap 1 (capture printed fingerprints vs the ratified elision)
- wrap/reconcile/contract.proposal-format 1 · contract.cascade-miss 1 (stack.md windows-2025 vs 2022) · route-resolve/contract.carry-no-owner 1
- new-session/orientation/input.handoff-git-mismatch 1 · phase/distill/contract.binding-contradiction 1 · input.spec-source-gap 1 (test-plan :44 calls corpus.db encrypted)
- phase/validate/contract.matrix-claim 1 (a claim on a deferred cap keeps status deferred; no verb resets it) · wrap/gates/contract.coverage-hold 1

**Untyped clusters below F-4 (watch next epoch).**
- Reconcile detector/predicate under-reach, 2: `2026-09-24T09:06:34Z-d` (the anchor-in-section predicate cannot tell
  which bullet an anchor is in) and `2026-09-29T18:11:24Z-c` (duplicate-sweeps under-ran). The first is
  in-pass-correction era (see Mechanism health).
- Reconcile audit-trail ordering, 1: `2026-09-23T20:58:44Z-c`. This is in-pass-correction era.
- Singletons: `2026-09-18T10:08:41Z-c` (splice.py summary says chars, counts bytes) · `2026-09-23T07:56:00Z-d`
  (libtest ignored mark in the `--nocapture` stream) · `2026-09-23T09:53:10Z-a` (the clause-retraction record itself)
  · `2026-09-29T18:37:32Z-b` (cargo-deny `--config` placement) · `2026-09-29T20:38:49Z-b` (a tally written before
  it was derived; P6 family) · `2026-09-29T21:17:20Z-d` (SUT HEAD moved between phase and implement) ·
  `2026-09-30T04:18:58Z-c` (W182 lives in no rule; P18 shape) · `2026-09-23T20:44:24Z-b` (gate.py `${S}`; L2).

**Problem-fact themes below threshold.**
- T6 read-cap shapes routing and reading: 2 facts (`2026-09-29T21:41:18Z-a#0`, `2026-09-29T18:47:24Z-a#0`), plus 2
  tier-routing frictions.
- T7 context ceiling forces condensation or deferral: 2 (`2026-09-23T05:27:17Z-a#0`, `2026-09-23T09:44:05Z-a#0`).
- T8 tool trails carry host paths, rewritten by script: 1 (`2026-09-29T18:22:32Z-a#0`), "as the prior wrap did" (→ P14).
- T9 light-gate HEAD-anchored probes go vacuous after the operator pre-CI commit: 1 overridden fact
  (`2026-09-29T19:43:16Z-a#0`), plus a friction and a plan=`thin` consumer.
- T10 route grammar repaired on the tail: 1 removed-cause (`2026-09-23T09:51:34Z-a#0`) (→ P3).
- Single-rule overrides (each on a different rule): `2026-09-23T05:27:17Z-a#1` (the review replaced the lean) ·
  `2026-09-23T07:56:00Z-a#0` (the agent launched pulse-app on direction) · `2026-09-23T09:44:05Z-a#3` (playbook rules
  rewritten) · `2026-09-23T20:42:32Z-a#0` (plain `#[test]` over rstest) · `2026-09-29T07:06:57Z-a#0` (no drive
  against a worktree Pulse) · `2026-09-29T18:11:24Z-a#2` (a widening waits for the founder) ·
  `2026-09-29T19:43:16Z-a#0` (base-anchored re-verify).
- Unresolved facts: `2026-09-29T18:22:32Z-a#1` (the gates playbook was read before its checkpoint during a
  classifier outage) · `2026-09-30T05:06:34Z-a#0` (driver 152 vs runtime 154; → P15).
- Deferrals, all closed or routed: `2026-09-18T07:27:32Z-a#0` · `2026-09-23T05:27:17Z-a#0` · `2026-09-23T09:44:05Z-a#2` ·
  `2026-09-23T09:51:34Z-a#1` · `2026-09-29T18:11:24Z-a#3`, and the friction `2026-09-24T09:08:57Z-b`.
