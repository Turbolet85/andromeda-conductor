# Evolve Diagnosis — conductor-0.3.0 · Epoch 5b — Version close · 2026-10-08T11:05:16Z

Read-only diagnosis of the friction ledger. Nothing here is applied, queued or remembered; every item is the
founder's to accept, reject, defer or modify. Evidence twins sit beside this file: `q-retractions.json`,
`q-health.json`, `q-typed.json`, `q-untyped.json`, `q-chains.json`, `q-level.json`. A record id
(`{ts}-{letter}`) points at its line in `.andromeda/friction-log.ndjson`; `#n` after an id is the 0-based index of
a problem fact in that step record.

Chunk legend (all eight are frozen in the working route and `complete` in the master route):

| code | chunk |
|---|---|
| C1 | 2026-10-04-real-model-test-surface-corrective |
| C2 | 2026-10-04-second-test-surface-corrective |
| C3 | 2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09 |
| C4 | 2026-10-07-a-fifth-pre-registered-real-model-series-for-v3-09 |
| C5 | 2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive |
| C6 | 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09 |
| C7 | 2026-10-08-capture-canary-pairing-window-corrective |
| C8 | 2026-10-08-version-close-on-measured-evidence |
| null | records with no chunk (session starts, adaptation wraps) |

## Mechanism health

- **Records:** 246 in the epoch (120 step / 126 friction), 2026-10-04T11:39:36Z to 2026-10-08T11:03:05Z. Whole
  ledger: 3211 records, 0 unparseable. Malformed `ts`: 11 in the whole ledger (kept, listed in
  `q-retractions.json`), 0 in this epoch.
- **Coverage:** all eight chunks carry phase 5 · implement 3 · wrap 5. C6 carries a fourth implement record: two
  `code` records (`2026-10-07T21:09:27Z-a`, `2026-10-08T07:00:02Z-a`), the step run in two sessions. No checkpoint
  gap. Nine `new-session/orientation` records. Six null-chunk wrap records over four adaptation or no-op wraps
  (curation ×3, route-resolve ×3).
- **Folds applied:** the epoch label has one spelling (246). `skill` has two (35 of 246 bare), folded. The
  previous epoch carries records under version `0.3.0` beside `conductor-0.3.0`; every lookback folded both.
- **One mislabelled record:** `2026-10-04T14:56:02Z-b` carries skill `andromeda-implement` with step `curation`
  inside a wrap-session curation checkpoint (its step record `-a` is wrap-session/curation). It is counted in the
  type-alone group of P1 and has no denominator of its own.
- **Untyped rate:** 20 of 126 (16%). Per step, untyped of friction: take-up 5/7 · plan 2/5 · orientation 2/7 ·
  report 2/6 · code 3/12 · gates 1/4 · route-resolve 1/8 · research 1/12 · reconcile 2/22 · curation 1/18 ·
  distill 0/5 · validate 0/11 · fix-loop 0/7 · smoke 0/1.
- **Problem-fact fill:** 52 of 120 step records, 73 facts (workaround 53 · overridden 10 · removed-cause 5 ·
  prohibition 3 · unresolved 2 · deferred 0; process 56 · environment 14 · resources 2 · product-logic 1).
- **Id fill:** 246 of 246.
- **Outcomes:** ok 115 · halted-resolved 5 · ok-degraded 0. Four friction records carry `halted`; the fifth
  halted step record (C6 wrap/reconcile, `2026-10-08T07:41:00Z-a`) has no friction record with the key.
- **Retractions (whole-ledger pre-pass):** retracted 11 (2 unresolvable) · clause-retracted 4 (kept, notes
  rendered) · retraction targeted by retraction → founder review: `2026-09-10T19:52:10Z-a`. In this epoch: 0
  retracted, 0 clause-retracted. Unresolvable, verbatim, for manual discount:
  - `2026-10-07T08:18:58Z-b` (this epoch, `id: null` form): "in step record 2026-10-07T08:18:40Z-a (implement
    fix-loop), the consumed plan note says '20 operator entries by hand'; the block holds 15 fired by hand
    (entries 1-13, 25 and 45) and 3 left for the operator pass (46-48); discount the figure 20, the rest of the
    note stands". It touches no count in this document.
  - a pre-boundary prose-form retraction outside this epoch: "the untyped code-graph-under-reports record and the
    first problem-block entry on…" (the twin holds its first 80 characters).
- **Calibration boundaries in range:**
  - `operator-pass` entry (from 2026-10-07): four of eight report records carry it (C5 to C8, each
    `as-left · commits:1 · red:0`). The four earlier ones are era, C4's (2026-10-07T09:50Z) included.
  - `new-text-*` word and `rejected-for-source` / `rejected-for-coordinate` counts (from 2026-10-08): C7 and C8
    carry both (rejections 0 and 0). C6 carries an unlisted `new-text-section-pasted-last`. The rest are era; P3
    is read accordingly.
  - `contract.in-pass-correction` (from 2026-09-27) is fully in range.
- **Threshold reading used:** "n ≥ 2 with halt impact" is read as both occurrences carrying a halt.
  `wrap/gates · contract.coverage-hold` (n = 2, one halted) therefore sits in the appendix.
- **Host change inside the epoch:** records cite `.claude/rules/host-win32.md` through 2026-10-07T13:34Z (10
  records) and `host-linux.md` from 2026-10-07T15:33Z (4 records). P5 and L1 to L3 span both.
- **Two JSON spellings in the ledger:** all 246 epoch lines are spaced JSON; the whole ledger holds 2295 spaced
  lines, 837 compact and 79 opening with neither form. A fixed-string reader keyed on one spelling reads a part;
  one in-epoch record (`2026-10-08T10:01:50Z-c`) is that event.

## Proposals (typed patterns)

54 per-step typed groups, 12 above threshold; 11 type-alone groups (universal and `recall.*`), 6 above. Merged,
that is 16 proposals, sorted by n × weight. Rate = n / step runs.

### P1 — */* · `recall.corpus-recurrence` — 10 cases · weight 17
(wrap-session/curation alone: 8 cases · weight 14 · rate 8/11)

**Pattern:** a curated rule that already states the hazard was not applied at the moment of action: four `cd`
cases, three file-target heredocs, three Tier-1 entries.

**Evidence:** ALL 10 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 | `cd` at the head of an implement probe moved the session cwd; the host rule of 2026-09-08 states it; fourth consecutive session | retries 0 | `2026-10-04T12:43:54Z-b` |
| C1 | `cat >>` heredoc append to a run-dir record at P2; guard blocked it; re-done with Edit | retries 1 | `2026-10-04T12:43:54Z-c` |
| C2 | same `cd` rule, fifth consecutive session; cwd moved into `target/phase-mutants` (the mislabelled record) | retries 0 | `2026-10-04T14:56:02Z-b` |
| C3 | `cd` at the head of a probe at the wrap's Setup; sixth consecutive session | — | `2026-10-06T21:25:38Z-d` |
| C4 | an anchored Edit dropped a heading from the attempt ledger and reported success; Tier-1 2026-08-21 states the rule; the write was not read back | retries 1 | `2026-10-07T10:04:11Z-c` · curation.md, To the handoff |
| C5 | `cat` heredoc append to a committed document at implement; guard refused; went through Edit | retries 1 | `2026-10-07T13:34:25Z-c` |
| null | `cd` into the sibling Pulse repo at the BLOCKED-ON premise re-read; guard refused; re-issued on absolute paths | retries 1 | `2026-10-07T15:33:43Z-b` |
| C6 (reconcile) | shell-heredoc append to a run-dir file; guard refused; went through Edit | retries 1 | `2026-10-08T07:41:00Z-e` |
| C7 | a site count from a grep pattern demanding fixed-width context under-read by one; Tier-1 2026-08-21 states it | extra_reads 1 | `2026-10-08T09:32:09Z-b` · `.andromeda/runs/2026-10-08T09-20-25-wrap/curation.md` |
| C8 | two findings matched Tier-1 2026-08-09: a host claim (no `pwsh`) copied into committed evidence unmeasured; a site list taken over three masters instead of seven | retries 2 | `2026-10-08T10:59:06Z-b` |

**Proposal:** the type appears in nine consecutive epochs (16 in the previous one, 10 here), so restating a
rule is measured not to change the act. The two host members now have a mechanical counterpart: the heredoc is
guard-refused, and from 2026-10-07 the `cd` is too, each costing one retry (see L1, L2). The three Tier-1
members (verify the write; read a pattern's hits; measure before copying) have none. Direction: decide which of
them can get one, for example a read-back step after an Edit in an evidence tree, rather than another
restatement.

### P2 — phase/validate · `contract.mechanical-check` — 7 cases · weight 18 · rate 7/8

**Pattern:** two sub-shapes share the type. Four records are check 4's own arms firing and resolving inside
validate; three are defects no mechanical predicate attempts, caught by the operator's review (C3, C5, C8).

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 | check 4 (6): a criterion named the `matrix.py show --unclaimed` probe no gate entry runs; moved to Implementation notes | iterations 1 | `2026-10-04T12:10:00Z-b` |
| C2 | check 4 (6): the P4 draft listed five inverse controls and a CARRY measurement as `[[gate]]` entries that mutate source; moved to one-shot prose runs; two duplicate entries dropped (33 → 25) | iterations 1 | `2026-10-04T14:19:25Z-b` |
| C2 | check 4 (9): baselines read two new probes red outside their subject (the `Finished` line of `cargo nextest list` on stderr); stderr routed away, re-baselined | iterations 1 | `2026-10-04T14:19:25Z-c` |
| C3 | the plan carried the harness rule's 2026-10-03 WebKit lever while Pulse at the series HEAD sets its own; research had both in its window; the operator's review caught it; no predicate reads a harness rule against the SUT's HEAD | dialogue_rounds 1 · iterations 1 · extra_reads 1 | `2026-10-06T19:56:00Z-b` · the chunk's `inputs/andromeda-inputs.json` |
| C3 | checks 4 (6) and 4 (4): a criterion named a census no entry listed; another read the attempt ledger no entry produces; resolved before the prompt | iterations 0 | `2026-10-06T19:56:00Z-c` |
| C5 | the external-inputs rule copied three operator files verbatim into the chunk folder at P3, one holding literal home-rooted paths; only the operator's review stopped them reaching a public repository; no check reads a snapshot's content class and the tool offers no withdrawal | dialogue_rounds 1 | `2026-10-07T12:23:15Z-b` · `.andromeda/runs/2026-10-07T11-54-53-phase/relay-2.md` |
| C8 | the plan leaned to drop a historical ordinal that a user-confirmed playbook rule note says stays; it named the tension and kept the lean; no predicate reads a plan's wrap notes against the playbook; the operator's review caught it | iterations 1 · dialogue_rounds 1 | `2026-10-08T10:17:56Z-b` · the chunk's `inputs` |

**Proposal:** (a) check 4 (6) fired in three of eight plans, twice as a criterion naming a probe no entry runs.
Direction: run that binding at P4 (planlint), so the plan is authored with it instead of repaired at P5. (b) Each review-caught
record names its own missing predicate: harness rule against SUT HEAD; a snapshot's content class plus a
withdrawal verb in `inputs.py`; a plan's wrap notes against user-confirmed playbook rules. Whether any earns a
check is the founder's call. See L10 for the override side of the same three chunks.

### P3 — wrap-session/reconcile · `contract.false-positive-proposal` — 6 cases · weight 11 · rate 6/8

**Pattern:** in five of six, a detector agent cited a source the prompt's report-alone rule bars. In four of
those (C1, C2, C3, C6) the orchestrator rejected the proposal for it and re-raised the fact itself; in C5 the
citations were let stand.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 | D-obs-ci-gates O1 proposed replacing obs §10's clippy line on a basis citing harness source the report does not carry; rejected; the wording routed to a CARRY | — | `2026-10-04T12:42:32Z-b` · `.andromeda/runs/2026-10-04T12-32-24-wrap/fanout-results.md` |
| C2 | two proposals rejected and their facts re-raised: the obs dependent cited `ci.yml:108-116`; the test primary cited `agent-run.ps1:373-384` and over-claimed the caller semantics; T2 and T3 fell with T1 | reformulations 4 | `2026-10-04T14:54:29Z-b` |
| C3 | the security-plan detector's three proposals each carried a basis in a source or capture file beyond the report; rejected; their report-carried facts applied through the expected-amendments floor | extra_reads 2 | `2026-10-06T21:24:08Z-b` · `.andromeda/runs/2026-10-06T21-10-08-wrap/fanout-results.md` |
| C4 | D-arch-registry-size proposed moving two evidence pointers out of the body to free bytes; narrowed, since a mechanism statement keeps its pointer | reformulations 1 | `2026-10-07T10:03:04Z-d` · fanout-results.md, architecture 5 |
| C5 | D-tests-derived-count proposed re-wording a keyed contract as a dependent of a retired claim it does not state; rejected at check 4. Four of seven detectors also cited non-report lines; each fact was in the report, so none was rejected for it | — | `2026-10-07T13:32:49Z-c` |
| C6 | a D-tests-derived-count proposal rejected at Validate's opening rule (it cited the run dir's word file); its fact re-raised. A second agent ran the registry-size detector its prompt reserves for the orchestrator | extra_reads 1 | `2026-10-08T07:41:00Z-c` · `.andromeda/runs/2026-10-08T07-22-11-wrap/fanout-results.md` |

Untyped sibling, not counted in n: C4 `2026-10-07T10:03:04Z-c`, 7 of 11 proposals carried a basis outside the
report (reformulations 7).

**Proposal:** the prompt's ban does not hold on the agents. Two directions: make it mechanical (a validator
that rejects by citation class before the orchestrator reads a proposal), or reverse it (a true, report-held
fact with an extra citation is recorded, not rejected, which is what C5 already did). The last two wraps (C7,
C8) record `rejected-for-source:0`; that is two data points.

### P4 — */* · `contract.premise-falsified` — 6 cases · weight 10
(phase/research alone: 4 cases · rate 4/8)

**Pattern:** four of six falsified premises arrived through a relay or a route CARRY; two were the project's
own artifacts stating a stale fact. All were caught by measurement before the write.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C4 (implement/code) | plan step 6 cites the 2026-10-06 section's reason for a home-rooted dir (the mask covers no temp-rooted path); `mask_host_paths` names the temp roots since that same chunk; the clause was written without the reason | extra_reads 1 | `2026-10-07T08:16:31Z-b` |
| C6 (research) | the relay asks for a content proof and the matrix acceptance names a string set; measured: the Pulse change adds no string literal and release strips symbols, so no token discriminates the builds | extra_reads 2 · dialogue_rounds 1 | `2026-10-07T20:49:13Z-b` · the chunk's research.md |
| C6 (research) | a relay figure marked measured gave 11 of 20 naming the right service; the sibling repository's own reading records 11 misses, so 9 right | extra_reads 1 | `2026-10-07T20:49:13Z-c` · the chunk's scope.md |
| C7 (research) | the architecture registry row and the CLAUDE.md overview state no Rust code reads the posture contract; a harvest test helper digest-pins six of its sections | extra_reads 3 | `2026-10-08T08:49:17Z-c` · the chunk's research.md |
| C8 (research) | the folded CARRY and the relay give that false claim four sites; a sweep on its wording found three more | extra_reads 4 | `2026-10-08T10:01:50Z-b` · the chunk's scope.md |
| null (route-resolve) | a relayed directive said the capture widens what a leg records, not what the repository holds; against the security plan and the architecture it widens the access channel; the relay's author agreed | extra_reads 2 · dialogue_rounds 1 | `2026-10-07T11:51:25Z-c` · `.andromeda/runs/2026-10-07T11-44-55-wrap/adaptation-record.md` |

**Proposal:** research's premise re-check is working as designed; the cost is 13 extra reads. The type runs 2
to 22 per epoch across the last eleven epochs. Direction, on the relay as an input class: a relayed figure marked
"measured" and a relayed site count carry no source the consumer can check, because the snapshot holds the
relay's text and not what it summarizes. A relay convention (each measured figure names its source file) or a
take-up arm marking relay figures unverified until re-read would move the cost to the producer. See U2.

### P5 — */* · `tooling.host-shell` — 6 cases · weight 8

**Pattern:** four of six are a persisted `cd`, all on 2026-10-04 (C1, C2). The other two are a mismatch between
the agent's shell and the gate tool's shell.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 (distill) | `cd {run_dir}` at the head of a probe persisted as the session cwd; fourth consecutive session | extra_reads 1 | `2026-10-04T11:52:00Z-b` |
| C1 (code) | `cd` into `crates/conductor-run/tests` persisted; recovered by absolute paths before any relative write | — | `2026-10-04T12:17:12Z-b` |
| C1 (validate) | the first baseline pasted the TOML `'''` literal into a `bash -c` argument; inner quotes split it (exit 2, `trap: usage`); re-run by parsing the fence with tomllib | retries 1 | `2026-10-04T12:10:00Z-c` · `target/phase-probe/baseline-11.log` |
| C2 (distill) | `cd` at the head of the extract-validation probe moved the cwd into the run dir; fifth consecutive session | — | `2026-10-04T12:55:43Z-b` |
| C2 (research) | a second `cd` (into `target/phase-mutants`) moved the cwd again, one step after the first was recorded | — | `2026-10-04T13:01:29Z-b` |
| C6 (validate) | a baseline control piped through the session shell's grep function counted 4 lines where the host grep binary, which the gate tool runs, counts 5; the controls were re-run with the host binary | retries 1 | `2026-10-07T21:00:24Z-b` · `.andromeda/cache/p5-controls/2026-10-06T19-26-05-phase/leaks.txt` |

**Proposal:** no persisted-`cd` record appears after 2026-10-04; from 2026-10-07 the records show a guard
refusing the `cd` instead (P1's null row, L1). The pipeline-level remainder is that a gate entry fired by hand
runs in a different shell from the one the gate tool uses. Direction: a gate-tool verb that fires one entry by
id, if none exists, so a hand-fired baseline or control runs where the gate will. See L3.

### P6 — wrap-session/reconcile · `contract.in-pass-correction` — 5 cases · weight 8 · rate 5/8

**Pattern:** three of five are a count or basis written from memory instead of from the list on the page (C3,
C6, C8). All five were caught by the pass's own re-read before the commit.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 | R2's sentence was inserted before test-plan §4's "One further contract item, measured the same day", re-pointing that clause's date referent; caught on re-read and moved | retries 1 | `2026-10-04T12:42:32Z-c` |
| C1 | `cascade-patterns.toml` carried a 17-character pattern id; `cascade.py` refused the file (exit 2); renamed and re-fired | retries 1 | `2026-10-04T12:42:32Z-d` |
| C3 | a sidecar payload named two dated entries as holding the earlier pins on a count-only basis; reworded before the append | — | `2026-10-06T21:24:08Z-e` |
| C6 | three run-dir texts were wrong on first write: unresolved rows counted 45 and 4+ where the listing reads 42 and 7; a superseded playbook rule named as governing; a sidecar bullet for a re-point that takes none | extra_reads 3 | `2026-10-08T07:41:00Z-d` |
| C8 | fanout-results.md said the null-arm family held 11 sites; a recount of its own printed list read 10 plus one leaf; corrected before any artifact copied it | retries 1 | `2026-10-08T10:58:04Z-d` |

**Proposal:** the re-read is doing its job (3 → 8 → 5 per epoch since the type deployed). Direction: a count in
a run-dir text comes from a command whose output is pasted, as the new-text listing already is, never typed.
The same cause sits under P11 and P12.

### P7 — wrap-session/curation · `ambiguity.filter-borderline` — 6 cases · weight 6 · rate 6/11

**Pattern:** every case is a candidate landing exactly on 0.6 from the same two signals (measured +0.4,
specific detail +0.2) and decided by one conditional signal. Ten candidates in six wraps.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C3 | a pipeline-tool mechanics candidate scored exactly 0.6 and passed only on the no-other-home signal | — | `2026-10-06T21:25:38Z-b` |
| C4 | two calls at the edge: one exactly 0.6 and rejected (its fact has a home in the contract section); one reaching 0.6 on measurement and detail alone, passed on reading the overseer's listed candidates as an explicit curation request | — | `2026-10-07T10:04:11Z-b` · curation.md |
| null | one candidate exactly on the threshold (0.4 + 0.2); both conditional signals closed; rejected by the lean default | — | `2026-10-07T11:51:25Z-e` · `.andromeda/runs/2026-10-07T11-44-55-wrap/adaptation-record.md` |
| C5 | both survivors exactly 0.6, carried over by the no-other-home signal alone; whether the finding earns the measured signal was the underdetermined call | — | `2026-10-07T13:34:25Z-b` |
| C6 | two candidates exactly 0.6: one took the load-bearing-for-the-next-entry signal on a judgment; the other rejected, its fact already homed in two masters and a CARRY | — | `2026-10-08T07:43:14Z-b` |
| C8 | two candidates exactly 0.6; neither conditional signal applied; both rejected by the exact-hit rule | — | `2026-10-08T10:59:06Z-c` |

**Proposal:** Filter 4's two commonest signals sum to its own cutoff, so the decision is carried by the
conditional signals every time; the records show both outcomes at the same score. Direction: move the cutoff
off the 0.4 + 0.2 sum, or state the exact-hit rule once in the filter itself. The type appears in 13 of the
ledger's 15 epoch groups.

### P8 — wrap-session/route-resolve · `ambiguity.trajectory-halt` — 2 cases, both halted · weight 12 · rate 2/11

**Pattern:** both halts are the class new-chunk-ahead, and both were answered through the relay channel in one
round.

**Evidence:** ALL 2 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C4 | a second not-met series left what follows undecided; one halt, one round; answered with the founder's pick relayed by the overseer and a sibling-repo route line to verify | dialogue_rounds 1 · halted 1 · extra_reads 1 | `2026-10-07T10:08:24Z-b` · route-resolve.md |
| null | a relayed pre-direction named the entry and its slot, but the relay's reading of the boundary differed from the project's rule; one question went to the operator channel before the entry was written | halted 1 · dialogue_rounds 1 | `2026-10-07T11:51:25Z-b` · `.andromeda/runs/2026-10-07T11-44-55-wrap/adaptation-record.md` |

**Proposal:** the halt is designed behaviour, and in the null row it caught a real difference. A later wrap in
this epoch records the signal `predirection-satisfied-trajectory-gate`, so a pre-direction can already satisfy
the gate. Direction: give a relayed pre-direction a fixed shape (entry · slot · boundary reading), so
route-resolve can check it against the project's rule before deciding whether a round is needed.

### P9 — implement/fix-loop · `contract.instrument-validity` — 3 cases · weight 8 · rate 3/8

**Pattern:** an instrument's reading was taken as the property before the instrument had been shown to
discriminate.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C2 | the size-line forecast rested on tokei 14.0.0's base reading (1130 code, 0 comments over 1222 raw lines); split, the files read 872 code and 50 comments; validated as instrument, not lost content, by a normalized line-multiset comparison | extra_reads 1 | `2026-10-04T14:27:20Z-b` |
| C3 | plan gate entry 8 greps the rebuilt binary for a contiguous argv literal; it reads 0 on the stale and on the correct build alike; phase validated it only on the known-negative | dialogue_rounds 1 · halted 1 · extra_reads 2 | `2026-10-06T20:40:48Z-b` · the chunk's `evidence/attempt-ledger.md` |
| C6 | the capture's canary classifier printed pipeline-fault for the third storm on two of three drives while Pulse's own log shows that digest prompted and deduped; the pairing read lines stamped before the emission instant and the tick fell 2 to 4 ms before it | extra_reads 3 | `2026-10-08T07:01:30Z-c` · the chunk's `evidence/attempt-ledger.md` |

Same type at another step, not counted: C2 phase/research `2026-10-04T13:01:29Z-c`, a scratchpad line counter
read 824 where tokei reads 1130.

**Proposal:** C3 names the missing step: the entry was validated on a known-negative only. Direction: validate's
baseline asks for both a known-positive and a known-negative reading of any new discriminating entry. C6 was
fixed in the product by chunk C7, so nothing remains there.

### P10 — */* · `contract.skill-reference-drift` — 4 cases · weight 6

**Pattern:** three of four are one drift. The wrap's trailing friction append extends a tracked evolve trail
after the wrap commit, and neither new-session's nor wrap's enumeration of expected dirt names that file.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C3 (implement/code) | the implement body gives only `inputs.py snap --source {path}` for an outside fact; a relayed answer needs `--message-file` with `--origin`, and the tool refuses any file under the temp dir | retries 2 | `2026-10-06T20:38:27Z-d` · the inputs trail in `.andromeda/runs/2026-10-06T19-56-48-implement/` |
| null (orientation) | `session-state-contract.md` enumerates the post-wrap bookkeeping as handoff, `state.yaml`, the friction log and the code metrics; the tree also held a modified tracked evolve trail in the committed wrap run dir | extra_reads 1 | `2026-10-07T07:07:21Z-c` |
| null (wrap curation) | wrap Setup step 6's dirt-check names untracked run dirs as bookkeeping; the tree carried a modified tracked run-dir file, the prior wrap's evolve trail | — | `2026-10-07T15:33:43Z-c` · `.andromeda/runs/2026-10-07T15-28-23-wrap/adaptation-record.md` |
| null (orientation) | the same enumeration (`session-state-contract.md:57`), the same tracked trail modified after the wrap commit | extra_reads 2 | `2026-10-08T09:41:32Z-b` |

**Proposal:** one cause, two letters. Direction: either name the trail in both enumerations, or stop producing
it (write the gates checkpoint before the commit, or leave the trail untracked). The C3 row was absorbed by the
project (curation homed the recipe in the host rule file, records `2026-10-06T21:25:38Z-b` and `-c`); the
pipeline-level remainder is the implement body naming the `--message-file` form. See P16.

### P11 — wrap-session/reconcile · `input.report-insufficient` — 4 cases · weight 5 · rate 4/8

**Pattern:** two of four are a site count in the report taken from a narrowed sweep; the detectors' own sweeps
caught both.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C2 | the report's Harness / gate surface bullet said the ps1 default stops "with that line's exit, whoever calls it"; under CI's preference the line throws first; caught by a doc-agent's commentary; report corrected | reformulations 1 | `2026-10-04T14:54:29Z-c` |
| C3 | the report omitted two facts the security-plan's dated row needed (the workspace-detector byte-identity at the new Pulse HEAD, the POSIX suffix rendering); one bullet added mid-pass | extra_reads 1 | `2026-10-06T21:24:08Z-c` |
| C7 | the report stated 7 hits and 3 sites for the no-Rust-reader claim from a grep requiring 60 characters before and 40 after the match; the unbounded grep reads 8 and 4; the architecture detector surfaced the missed site | extra_reads 1 | `2026-10-08T09:31:14Z-b` · `.andromeda/runs/2026-10-08T09-20-25-wrap/fanout-results.md` |
| C8 | the report listed nine sites for the verdict null-arm family; two detectors each found one more; the first list had been taken over three masters, not seven | extra_reads 2 | `2026-10-08T10:58:04Z-b` |

**Proposal:** today the detectors are the control on the report's counts. Direction: a sweep verb over a fixed
population (every master plus its key files) for any site-family count the report states, in place of a hand
grep. See X1.

### P12 — */* · `contract.narrow-basis-claim` — 4 cases · weight 4

**Pattern:** each is a count or absence claim made from a clipped or unread basis and corrected by its own
author.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 (plan) | the P4 option text promised the split leaves no citation stale, from a sweep that read v3-10's ref as a target name; the ref names arms by file; re-derived after the operator had answered on that basis | extra_reads 1 | `2026-10-04T12:06:11Z-b` |
| C3 (research) | research.md stated the callers of two functions from a tail-clipped view of the code-graph output plus one file read; the trace held two more calling tests and a third verdict test | extra_reads 1 | `2026-10-06T19:42:07Z-b` · the tree-query trail in `.andromeda/runs/2026-10-06T19-26-05-phase/` |
| C4 (smoke) | the fix-loop record's plan note counted the hand-driven operator entries from memory instead of from the block (this record carries the `id: null` retraction above) | — | `2026-10-07T08:18:58Z-b` · the chunk's plan.md, Test Commands |
| C6 (report) | two report statements about grep hits were written from the hit counts without reading the hits; one hit is a chunk marker carrying the token, the other the preconditions probe's subject | extra_reads 2 | `2026-10-08T07:26:00Z-b` |

**Proposal:** C3's cause is the tail-clip that L4 records as a routine workaround of the run-bare rule, so the
two are one trade-off. Direction: as in L4, tools print a bounded summary and write the full listing to the
trail. Counts across 0.3.0's epochs: 6 · 7 · 11 · 5 · 10 · 4.

### P13 — wrap-session/reconcile · `ambiguity.playbook-no-match` — 3 cases · weight 5 · rate 3/8

**Pattern:** two of three are one class, a real-model series' dated-record extension, which had no playbook
rule across four consecutive series wraps.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C3 | eleven proposals of two classes (a dated series record extended in four masters; a model literal removed from architecture) matched no rule; each settled by a P5-approved expected amendment; the series class had recurred at three wraps | — | `2026-10-06T21:24:08Z-d` |
| C4 | no rule matches a series' dated-record extension; 9 of 11 proposals settled by the plan's approved expected amendments, for the fourth consecutive series wrap | extra_reads 1 | `2026-10-07T10:03:04Z-b` · fanout-results.md, Checks over the whole set |
| C6 | three amendments had no governing rule: the MET verdict statement in architecture (the series rule names it as not covered), two unmeasured-pickup clauses, a canary-line sentence; two settled by the operator's recorded direction, one applied under the disproved-claims check | dialogue_rounds 1 | `2026-10-08T07:41:00Z-b` · `.andromeda/runs/2026-10-08T07-22-11-wrap/fanout-results.md` |

**Proposal:** absorbed mid-epoch for the series class: by C6 a series rule exists. The remainder is the delay,
four wraps before a rule landed. Direction: reconcile's no-match record carries how many earlier wraps met the
same class, and nudges the operator toward a rule at the second.

### P14 — wrap-session/route-resolve · `contract.no-sanctioned-channel` — 3 cases · weight 5 · rate 3/11

**Pattern:** a true fact noticed at wrap that predates the chunk, or sits outside it, had no sanctioned place
to land.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C3 | the architecture detector noted, outside its detectors, that the posture-contract entry says the contract names only registered handles while two model env handles are absent from architecture; it predates the chunk and no disposition was named; surfaced in the console report and the handoff | — | `2026-10-06T21:26:32Z-b` |
| null | a sweep found the masters and their leaves still describing the webview legs' home as the Windows dev host (architecture 4, test-plan 4, registries 4, six leaves); the 0-pending path runs no fan-out and may not amend a master on drift it noticed; written to the handoff and the adaptation record | — | `2026-10-07T15:33:43Z-e` · `.andromeda/runs/2026-10-07T15-28-23-wrap/adaptation-record.md` |
| C6 | two facts had no surface: model text on a frozen route line (the only frozen-line write is the compaction, which would have archived the quote verbatim) and the same text in a frozen chunk's report; the first edited on the operator's ruling, the second pinned as a CARRY | dialogue_rounds 1 | `2026-10-08T07:44:54Z-b` · `.andromeda/runs/2026-10-08T07-22-11-wrap/route-record.md` |

**Proposal:** C8's wrap records a residual line in use for this (`2026-10-08T11:00:19Z-a#0`,
`2026-10-08T10:58:04Z-c`), so a channel exists by the epoch's end. Direction: name the residual as the landing
for pre-existing drift on every wrap path, the 0-pending one included, and give removal of leaked content from
a frozen line its own arm.

### P15 — */* · `contract.structural-blind-spot` — 3 cases · weight 4

**Pattern:** three unrelated mechanisms, each unable to reach its subject by construction.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| C1 (research) | the rust code graph indexes no feature-gated target, so the impact query omitted `real_model_live.rs:67`, the one caller the split could break; recovered by grep | extra_reads 1 | `2026-10-04T11:58:36Z-b` · the tree-query trail in `.andromeda/runs/2026-10-04T11-44-25-phase/` |
| C4 (report) | the wrap's P1 reads an operator relay from outside the repo, and `inputs.py snap --step` admits `phase:P1`, `phase:P3` and `implement` only | extra_reads 1 | `2026-10-07T09:50:38Z-c` · exit 2, invalid choice `wrap` |
| C8 (gates) | the light gate (P7.1) runs before the coverage step's ledger note (P7.3), and a default-suite test reads the ledger file; the gate's green cannot cover the note write; the reader test was re-run beside the tool | retries 1 | `2026-10-08T11:03:05Z-b` |

Untyped sibling of the second row, not counted: C8 `2026-10-08T10:46:58Z-c`, the same refusal one day later.

**Proposal:** three small directions. (1) The code-graph refresh builds the rust plane with the gated features
on, or its output names the targets it did not index, so 0 rows is not read as no callers (third consecutive
epoch, see L9). (2) A `wrap` choice for `inputs.py snap --step`. (3) In wrap P7, the ledger note before the
light gate, or a re-run of the ledger's reader after P7.3.

### P16 — new-session/orientation · `input.handoff-git-mismatch` — 3 cases · weight 3 · rate 3/9

**Pattern:** the handoff is written before the session's last writes, and skills outside the loop do not
update it.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| null (2026-10-04) | handoff Status clean; the tree carries three untracked post-wrap run dirs (two evolve-diagnose, one code-audit) and a code-metrics append beyond the expected hook line | extra_reads 1 | `2026-10-04T11:39:36Z-b` |
| null (2026-10-06) | handoff clean; git shows three modified and one untracked path with HEAD equal to upstream: the post-commit friction append, its evolve trail, an untracked health trail, and a hook's Session End Status stamp; the documented transient names a lone friction append | extra_reads 2 | `2026-10-06T19:24:16Z-b` |
| null (2026-10-07) | the handoff's Position still names `/andromeda-setup-project` for the pending U02 as next; HEAD is that setup commit, pushed; the setup run committed without touching the handoff | — | `2026-10-07T07:07:21Z-b` · `git log -1 29adafa`; the `upgrade.py detect` summary line |

**Proposal:** the same cause as three rows of P10. Direction: widen the documented expected-transient set to
what a wrap leaves (friction append, evolve trail, health trail, hook stamp), and have the out-of-loop skills
(evolve-diagnose, code-audit, setup-project) note themselves in the handoff. This diagnosis leaves an untracked
run dir by design.

## Cross-step chains (starting heuristics)

22 verdict anchors (18 thin, 4 wrong) among 554 consumed entries. Two shapes span two or more chunks.

### X1 — wrap-session/report →report→ wrap-session/reconcile — 4 chunks (C2, C3, C7, C8)

Producers, all `ok`: `2026-10-04T14:46:18Z-a` (signal `carry-quoted-verbatim`), `2026-10-06T21:13:39Z-a` (no
signal; its note says every Changes family came "from session knowledge"), `2026-10-08T09:23:18Z-a` and
`2026-10-08T10:46:58Z-a` (`new-text-generated`). Consumers, each grading `report` thin:
`2026-10-04T14:54:29Z-a`, `2026-10-06T21:24:08Z-a`, `2026-10-08T09:31:14Z-a`, `2026-10-08T10:58:04Z-a`, with
P11's four friction records.

Hypothesis: the report step ends formally ok and signals nothing about the basis of its own counts and claims;
the detectors, sweeping independently, are the first control, and in each chunk they found the gap. Direction:
P11's sweep verb, or a producer-side signal naming the population each stated count was taken over.

### X2 — phase/plan or phase/validate →plan→ implement and wrap-session/gates — 2 chunks (C3, C8)

The strict shape splits by producing step, so each reads k = 1; it is listed for the shared cause.
- C3: phase/plan `2026-10-06T19:49:18Z-a` (ok) → implement/code `2026-10-06T20:38:27Z-a` and fix-loop
  `2026-10-06T20:40:48Z-a` and wrap/gates `2026-10-06T21:28:29Z-a`, each grading `plan` wrong: gate entry 8's
  atom cannot be satisfied on a correct build (P9, row C3).
- C8: phase/validate `2026-10-08T10:17:56Z-a` (ok) → wrap/gates `2026-10-08T11:03:05Z-a`, `plan` thin: an
  expect atom pinned the registry bytes as they stood before the wrap's own amendment (untyped
  `2026-10-08T11:03:05Z-c`).
- Also C2: phase/plan `2026-10-04T14:14:47Z-a` → validate `2026-10-04T14:19:25Z-a`, `plan` thin (P2, rows C2).

Hypothesis: a gate entry's atom is validated at phase against the tree as it stands; nothing asks whether it
holds on the tree its later consumer will see. Direction: an entry attribute saying through which step the
atom is expected to hold, and P9's known-positive rule.

Anchors with no producing step in the chunk (listed, no chain): `wrap-playbook` thin ×3 at reconcile (C3, C4,
C5; P13) · `tree-db` thin ×2 at research (C1, C7; L9) · `handoff` thin and wrong plus `matrix` thin at
orientation (P16) · `working-entry` thin at C3 take-up · `conversation` thin ×2 at C4's wrap, compacted between
the operator pass and the wrap (the join names implement/smoke as the last producer of `conversation`, a
coincidence of the artifact name) · `research` thin at C3 code and at C7 plan, one chunk each.

## Level candidates (systemic-masked-as-project)

Pass A read all 61 workaround, prohibition and removed-cause facts and clustered them by theme; 28 fall in the
themes below and 33 are in the appendix. Deferred-forever has no hit: the epoch holds 0 deferred facts, 0
`deferral-open` signals and 0 gate-deferral records.

### L1 — band-aid — 2 facts (6 in the previous epoch)

**Facts:**
- C1 phase/distill `2026-10-04T11:52:00Z-a#0` · environment · workaround: a `cd` into the run dir moved the
  session cwd; re-anchored, absolute paths after.
- null wrap/route-resolve `2026-10-07T15:33:43Z-d#0` · environment · workaround: the cwd guard refused a `cd`
  into the sibling repo; the read ran with absolute paths.
- Related, solution `unresolved`, not counted: C8 take-up `2026-10-08T09:45:46Z-a#1`, two calls opened with a
  bare `cd` into the project root.
- Typed correlates: `tooling.host-shell` ×4 (P5), `recall.corpus-recurrence` ×4 (P1).

**Level hypothesis:** the cause lives in the harness shell's persistent cwd and the habit of leading a command
with `cd`. The fixes landed in the project: a rule line in two host rule files, a standing deferred-learning
note in the handoff, and from 2026-10-07 a guard. The records count six consecutive sessions in which the rule
line did not prevent it; the guard turns it into one refused call.

**Proposal:** treat the guard as the fix and stop carrying the restatements. Pipeline side: check that no skill
reference shows a leading `cd {run_dir}` in its own examples.

### L2 — band-aid — 3 facts (7 in the previous epoch)

**Facts:**
- C4 wrap/report `2026-10-07T09:50:38Z-a#2` · environment · workaround: a scratchpad probe script written by a
  `cat` heredoc was blocked by the Bash guard; written with the Write tool and run by path.
- C5 implement/code `2026-10-07T12:58:54Z-a#0` · process · workaround: the guard refused a `cat` heredoc append
  to the attempt ledger; later appends went through Edit or a python read-modify-write.
- C8 phase/research `2026-10-08T10:01:50Z-a#0` · process · workaround: a scratchpad script first written with a
  `cat` heredoc; the guard refused; re-authored through Write.
- Typed correlates: `recall.corpus-recurrence` ×3 (P1), `tooling.hook-friction` `2026-10-07T12:58:54Z-b`.

**Level hypothesis:** the guard works. What remains is one refused call per occurrence, about one in every two
chunks, from a habit that lives outside the project.

**Proposal:** nothing project-side is left to fix. Direction: record a guard refusal as a `signals` fact (a
mechanism working), not as a recurrence record plus a problem fact each time; today it inflates P1.

### L3 — band-aid — 3 facts (1 in the previous epoch)

**Facts:**
- C2 phase/distill `2026-10-04T12:55:43Z-a#0` · environment · workaround: a sweep excluding `.andromeda/runs/`
  by a `^./`-anchored `grep -v` kept those lines (the tool's grep prints paths without `./`); re-swept with a
  python scan.
- C4 implement/code `2026-10-07T08:16:31Z-a#0` · environment · workaround: the Bash tool's grep is a shell
  function; plan entry 5 was fired with `command grep`; entries 8 to 13 from scratchpad scripts run by plain
  bash.
- C6 implement/code `2026-10-07T21:09:27Z-a#0` · environment · workaround: hand-fired binary-control entries
  run with the host grep binary, since the gate tool's own shell would run that binary.
- Typed correlate: `tooling.host-shell` `2026-10-07T21:00:24Z-b` (4 lines against 5 on one file and pattern).

**Level hypothesis:** the agent's shell wraps grep; gate entries are authored for, and run by, the gate tool
under the host binary; an entry fired by hand crosses the two. The fixes so far are per-chunk substitutions.

**Proposal:** P5's single-entry gate verb, or a planlint flag on a bare `grep` in an entry the plan marks as
fired by hand.

### L4 — band-aid — 4 facts (7 in the previous epoch)

**Facts:**
- C4 phase/distill `2026-10-07T07:20:06Z-a#1` · process · workaround: five `sidecar.py cites` calls read
  through grep and tail instead of bare.
- C4 phase/research `2026-10-07T07:25:33Z-a#0` · process · workaround: `registry.py contracts` and
  `code-graph.py query` read through tail and cut; the exit taken from PIPESTATUS.
- C5 phase/distill `2026-10-07T12:06:24Z-a#1` · process · workaround: five cites listings filtered to keep 160
  resolved rows out of the window, against the run-bare rule; the trail holds each whole.
- C5 phase/plan `2026-10-07T12:18:07Z-a#0` · process · workaround: the second and third fence dry-runs
  redirected to a scratchpad log and read by pattern and last line.
- Typed correlate: `contract.narrow-basis-claim` `2026-10-06T19:42:07Z-b`, a tail-clipped listing that hid
  three callers: the hazard the rule guards, realized once.

**Level hypothesis:** the run-bare rule prices every tool call at its full listing; agents pay it once and
filter after. Eleven facts in two epochs with no friction record, and one realized miss.

**Proposal:** change the tools, not the rule: default output is the verdict, the summary and the non-ok rows,
with the full listing in the trail where the notes say it already is.

### L5 — band-aid — 5 facts (none found in earlier epochs)

**Facts:**
- C4 phase/research `2026-10-07T07:25:33Z-a#1` · process · workaround: most P3 reads ran while the P2 batches
  were out; every write waited for the batch.
- C4 wrap/reconcile `2026-10-07T10:03:04Z-a#0` · process · workaround: three plan bodies were applied while the
  architecture detector was still running and before the dispositions were written; no proposal collided.
- C5 phase/research `2026-10-07T12:10:03Z-a#0` · process · workaround: research's read-only reads were made
  during the two fan-out waits; every P3 write came after the stage-2 path check.
- C6 phase/research `2026-10-07T20:49:13Z-a#1` · process · workaround: read-only reads while the two P2 batches
  ran; nothing written until each path check passed.
- C7 phase/distill `2026-10-08T08:43:03Z-a#0` · process · workaround: the wait was spent on read-only reads
  that belong to research; nothing written between the snapshots.

**Level hypothesis:** the letter orders P2 before P3; the fan-out wait is idle; in four of eight chunks the
agent used it for read-only lookahead under a self-imposed no-write line. No friction was ever recorded. One
member (C4 reconcile) went further and applied bodies before every detector had returned.

**Proposal:** sanction read-only lookahead during a fan-out in the phase letter, with the no-write line the
agents already keep. Whether the reconcile case belongs inside that sanction is a separate call.

### L6 — band-aid — 4 facts

**Facts:**
- C1 implement/code `2026-10-04T12:17:12Z-a#0` · process · workaround: verbatim family moves carved by a python
  script reading the base commit's file, so the moved bytes stay exact.
- C2 implement/code `2026-10-04T14:26:21Z-a#0` · process · workaround: the same form; the script's writes
  bypassed the rustfmt hook, so rustfmt ran by hand on one file.
- null wrap/curation `2026-10-07T10:17:50Z-a#0` · process · workaround: eight bullets of 1 to 18 KB on single
  lines moved by a read-modify-write script and read back against HEAD, where the tier writes are prescribed
  as anchored Edit or Write.
- C7 phase/validate `2026-10-08T08:58:17Z-a#0` · process · workaround: eight baseline keys written into plan.md
  by one scripted replace with a uniqueness assert per anchor.
- Correlates: `tooling.hook-friction` `2026-10-06T20:38:27Z-c`; untyped `2026-10-07T09:50:38Z-d`, an anchored
  Edit that dropped a heading and reported success.

**Level hypothesis:** the letters prescribe Edit and Write as the write path. For a byte-exact move, a multi-KB
single line or several same-shaped inserts a script is the exact tool, and the epoch's one recorded content
loss came from an anchored Edit.

**Proposal:** the letters name a scripted read-modify-write with a read-back as a sanctioned form for those
three cases, with the note that it bypasses the PostToolUse hook.

### L7 — band-aid — 3 facts (1 in the previous epoch; 2 read in earlier epochs)

**Facts:**
- C4 phase/distill `2026-10-07T07:20:06Z-a#0` · process · workaround: the history prompt's sidecar and index
  paths substituted as absolute where fan-out.md defines them project-relative, because the agents' file reads
  take absolute paths.
- C5 phase/distill `2026-10-07T12:06:24Z-a#0` · process · workaround: the prompt kept its relative paths; one
  line naming the project root was appended.
- C5 wrap/reconcile `2026-10-07T13:32:49Z-a#0` · process · workaround: no raw twin written for three empty
  returns; each verbatim return carried a home-rooted path the prompt had given, which the run-dir hygiene read
  refuses.
- Typed correlate: `contract.jointly-contradictory-instructions` `2026-10-07T13:32:49Z-b` (the twin rule and
  hygiene P1 cannot both hold once an agent echoes the absolute path).

**Level hypothesis:** three letters meet at one point: fan-out.md's relative paths, the twin rule, and the
run-dir hygiene read. Each chunk resolves it differently (absolute substituted; a root line appended; the twin
skipped).

**Proposal:** one pipeline answer: the prompt carries relative paths plus a single root line (C5's form), and
the twin rule gains a sanctioned path-scrub so a scrubbed return still counts as raw.

### L8 — band-aid — 2 facts (recurring from 0.3.0 Epoch 4, `2026-09-29T21:41:18Z-a`)

**Facts:**
- C5 implement/code `2026-10-07T12:58:54Z-a#1` · process · workaround: the harness rule file is 76 multi-KB
  lines the Read tool clips per line; folded into a scratchpad copy and read in two pages.
- C6 phase/research `2026-10-07T20:49:13Z-a#0` · resources · workaround: the file is past the read cap; of 39
  dated entries 24 were read whole and 15 by index line only, where the letter asks every span.
- Typed correlate: `tooling.output-cap-overflow` `2026-10-07T12:58:54Z-c` (32757 tokens against a cap of 25000).

**Level hypothesis:** wrap curation appends dated entries to `.claude/rules/verification-harness.md`; the file
has outgrown what its consumer steps can read whole, so it is loaded and partly unread. The same pressure
reached Tier 1 in this epoch (an operator-directed re-tier of eight entries, `2026-10-07T10:17:50Z-a#1`).

**Proposal:** a size cap or compaction on the curation side for path-scoped rule files, or the one-file-per-key
form the masters' registries have.

### L9 — chronic-degrade — 4 `tooling.*` types, no halt in any epoch

**Facts:**

| type | in epoch | epochs present | in-epoch records |
|---|---|---|---|
| `tooling.host-shell` | 6 | 11 consecutive (14 · 13 · 3 · 4 · 8 · 10 · 2 · 2 · 5 · 9 · 6) | P5 |
| `tooling.output-cap-overflow` | 2 | 8 | `2026-10-07T12:58:54Z-c` (the harness rule file) · `2026-10-07T20:49:13Z-d` (`matrix.py show` printed 33.4 KB of notes for one capability) |
| `tooling.hook-friction` | 2 | 7 | `2026-10-06T20:38:27Z-c` (rustfmt hook collapsed a placeholder arm) · `2026-10-07T12:58:54Z-b` (guard blocked a heredoc append) |
| `tooling.graph-symbol-missing` | 1 | 3 consecutive (3 · 2 · 1) | `2026-10-08T08:49:17Z-b`; the same cause also as `2026-10-04T11:58:36Z-b` and as `tree-db` thin at research in C1 and C7 |

Excluded: `tooling.environmental` (2 in the epoch) has halted records in two earlier epochs. Step outcomes
`ok-degraded`: 0 in this epoch, 5 in the previous one.

**Level hypothesis:** each occurrence is absorbed by a retry or an extra read and none halts, so the halt
policy never surfaces them. The causes sit outside the project's code: the tool shell (L1, L3), an oversized
rule file and an unbounded notes print (L8), and a code graph built without the gated features.

**Proposal:** the graph member is P15 (1). For `matrix.py show`, a bounded-notes form. The rest are L1, L3, L8.

### L10 — override — 4 facts at phase/validate (4 of 8 chunks; 2 in the previous epoch)

**Facts:**
- C3 `2026-10-06T19:56:00Z-a#0` · process · overridden: the operator changed lean 4 at the review; the host
  rendering lever was dropped from the launch posture and the contract section; the mechanical set re-run.
- C4 `2026-10-07T07:37:08Z-a#0` · process · overridden: the review's single yes carried a directive changing
  one step's arm; the plan was edited after the word and the mechanical set re-run after the edit.
- C5 `2026-10-07T12:23:15Z-a#0` · process · overridden: the operator ruled that three operator files
  snapshotted at P3 must not be committed; the copies and manifest entries were removed.
- C8 `2026-10-08T10:17:56Z-a#0` · process · overridden: the operator overruled the plan's lean to drop a
  historical ordinal; the plan was re-synthesized with both ordinals kept.
- Typed correlates: P2's rows C3, C5 and C8.

**Level hypothesis:** the review is the designed place for the operator's word, so an override there is the
mechanism working. The signature fires because in four of eight chunks the plan reached the review carrying
something the operator reversed, and in three the validate record says no predicate attempts the class. What
may be miscalibrated is how P4 forms a lean where a recorded rule or the SUT's current state already speaks to
the point (C3, C8), which is the pipeline's, not the project's.

**Proposal:** at P4, a lean that goes against a recorded rule is put to the operator as a fork instead of a
lean. C5's case is P2 (b).

### Observation — removed-cause theme: advisory-db fetch residue — 2 facts

- C5 implement/fix-loop `2026-10-07T13:01:34Z-a#0` · environment · removed-cause: three untracked
  pre-id-assignment advisory files removed by name from the local advisory-db copy.
- C6 implement/fix-loop `2026-10-08T07:01:30Z-a#0` · environment · removed-cause: `cargo audit`'s own fetch
  left one such file after the porcelain entry had read clean; removed, entries 38 to 40 re-run.
- Previous epoch: `2026-10-01T23:22:23Z-a` (the same residue, worked around). Typed: `tooling.environmental`
  ×2 (`2026-10-07T13:01:34Z-b`, `2026-10-08T07:01:30Z-b`).

The same cause was removed on consecutive days and returned, re-created by the scanner's own fetch. No
proposal: the signature surfaces a recurring removed cause as an observation.

## Playbook-extension candidates (untyped patterns, F-4)

20 untyped records: 5 in the two candidates below, 15 in the appendix.

### U1 — phase/take-up (2) and phase/research (1) — 3 cases → proposed type `tooling.invocation-form`

**Cluster:**
- C2 take-up `2026-10-04T12:50:16Z-b`: the first structural read opened `.andromeda/working-route.md`
  (FileNotFoundError); the working route lives in the version dir (retries 1).
- C4 research `2026-10-07T07:25:33Z-b`: the project's registry-size probe refused an absolute `--file` (a path
  argument must be repo-relative); the session's own rule is absolute paths for every call (retries 1).
- C7 take-up `2026-10-08T08:36:56Z-b`: the CARRY and the relay cite `scripts/arch-registry-check.py measure`
  with no argument; the bare call exits with a usage error; re-fired with `--file` (retries 1).
- Fact-side siblings on the same script: `2026-10-07T11:51:25Z-a#1`, `2026-10-08T09:45:46Z-a#0`.

→ **draft criteria line:** `tooling.invocation-form` — the first call of a tool, script or structural read was
refused or missed for its FORM (a path spelled the wrong way, absolute against repo-relative or the wrong
directory; a required argument absent) and the corrected form ran; record where the wrong form came from (a
letter, a CARRY, a relay, habit) and the form that ran.

Four of the five events concern one project script whose two refusals pull against the host rule (absolute
paths) and a CARRY quoting the bare form. A project-side change to that script and that CARRY would remove
them without any new type. The C2 member is the loosest fit.

### U2 — wrap-session/route-resolve (1) and wrap-session/reconcile (1) — 2 cases, 1 in the previous epoch → proposed type `ambiguity.relayed-direction`

**Cluster:**
- null route-resolve `2026-10-08T08:30:38Z-b`: the relayed direction read both ways on whether the minted entry
  adds a requirement; decided entry-only from the route's own precedent and named for the operator to overrule
  (extra_reads 2).
- C8 reconcile `2026-10-08T10:58:04Z-c`: an operator wrap directive said to apply a pre-existing schema
  correction if a detector proposed it with report-held coordinates; two detectors did, each stating its own
  trigger unmet; user-confirmed playbook rules dismiss that shape; the rule was followed and the directive's
  residual arm taken, with no halt (extra_reads 3).
- Previous epoch: `2026-10-03T07:45:09Z-b` (phase/plan), a menu answer relayed through a delegate recorded
  "drive on S" while the founder's live word was HOLD.
- Typed neighbours on the same channel in this epoch: P8's null row, three rows of P4, and the override
  `2026-10-07T10:08:24Z-a#0`.

→ **draft criteria line:** `ambiguity.relayed-direction` — a direction that arrived by relay, or an operator
directive written before the step ran, read two ways against the letter or a user-confirmed rule, and the step
decided without a halt; record both readings, which was taken and on what basis, and what would overrule it.

The previous-epoch member is a neighbouring shape (the relay's fidelity, not its underdetermination); the F-4
recurrence rests on reading it into this cluster.

## Below threshold — no action

**Typed groups (24 groups, 21 lines):**
- wrap/gates · `contract.coverage-hold` — 2 (C3, C4; one halted): the one claimed capability measured not met by its own rule; un-claim valve.
- implement/fix-loop · `tooling.environmental` — 2 (C5, C6): the advisory-db residue of the observation above.
- implement/code · `input.plan-step-ambiguous` — 2 (C2, C8).
- implement/code · `tooling.hook-friction` — 2 (C3, C5); see L9.
- implement/fix-loop · `contract.spec-reality-gap` — 2 (C3, C8).
- wrap/curation · `ambiguity.tier-routing` — 2 (C3, null).
- phase/validate · `input.out-of-pipeline-source` — 2 (C4: a sibling repo's CI verdict; C7: an 8.2 MB log over `inputs.py`'s 1 MB cap).
- */* · `contract.grammar-irregularity` — 2, two sites, each a one-off (`research.md:43` in C1; `.andromeda/architecture-amendments.md:722` in C8).
- */* · `contract.token-proxy-check` — 2 (C2 distill, a false positive; C8 research, a false negative).
- */* · `tooling.output-cap-overflow` — 2; see L9.
- phase/research · `contract.instrument-validity` — 1 (C2); sibling of P9.
- phase/research · `tooling.graph-symbol-missing` — 1 (C7); see L9.
- phase/plan · `retry.synthesis-rework` — 1 (C1).
- phase/plan · `input.research-thin` — 1 (C7).
- phase/distill · `contract.extract-format` — 1 (C1).
- phase/take-up · `input.carry-context-gap` — 1 (C3).
- implement/code · `input.research-files-wrong` — 1 (C3).
- wrap/report · `recall.change-reconstruction` — 1 (C4).
- wrap/report · `contract.detector-fact-gap` — 1 (C8).
- wrap/reconcile · `contract.jointly-contradictory-instructions` — 1 (C5); see L7.
- wrap/route-resolve · `contract.carry-no-owner` — 1 (C1).

**Untyped (15 records):**
- stamp-ahead hook on an estimated Generated stamp in plan.md — 2 (`2026-10-06T19:49:18Z-b`, `2026-10-08T08:55:00Z-c`); none in the previous epoch; emerging.
- a CI fact take-up needs that `ci.py` does not print — 2 (`2026-10-07T07:11:30Z-b` a sibling repo's verdict; `2026-10-07T11:58:34Z-b` a cancelled run has no failing subject).
- a plan entry its consumer step cannot satisfy as written — 2 (`2026-10-08T09:03:18Z-b`, `2026-10-08T11:03:05Z-c`); see X2.
- singles — 9: `2026-10-04T11:39:36Z-c` (matrix done-test against route freight) · `2026-10-07T11:54:09Z-b` (ladder rung 4 has no clause for a CARRY decision gate) · `2026-10-07T08:16:31Z-c` (auto-mode classifier returned no verdict twice) · `2026-10-07T09:50:38Z-d` (an anchored Edit dropped a heading; no gate caught it) · `2026-10-07T10:03:04Z-c` (P3's sibling) · `2026-10-07T10:17:50Z-c` (the Tier-3 write form has no promotion case) · `2026-10-08T09:45:46Z-b` (git grep exclusion pathspecs excluded nothing) · `2026-10-08T10:24:24Z-c` (a master section cited by number is a pointer line; six probes) · `2026-10-08T10:46:58Z-c` (P15's sibling).

**Problem-fact themes (33 facts):**
- `route.py` clips the BLOCKED-ON text — 2 (`2026-10-06T19:24:16Z-a#0`, `2026-10-07T07:07:21Z-a#0`).
- `inputs.py` has no verb for the use — 2 (`2026-10-07T09:50:38Z-a#0` no wrap step; `2026-10-07T12:23:15Z-a#1` no withdrawal).
- a not-green CI row with no failed check, not taken to the HALT arm — 2 (`2026-10-07T11:58:34Z-a#0`, `2026-10-07T20:37:00Z-a#0`).
- `arch-registry-check.py` argument form — 2 (U1's siblings).
- architecture registry byte headroom — 2 (`2026-10-07T10:03:04Z-a#1`, `2026-10-08T11:03:05Z-a#0`).
- prohibitions — 3, three subjects (`2026-10-07T11:51:25Z-a#0`, `2026-10-07T12:23:15Z-a#2`, `2026-10-07T20:56:19Z-a#0`).
- singles — 20, listed with their notes in `q-level.json` under "below: singles".

**Other facts outside the signatures:**
- overridden, not at validate — 6: `2026-10-04T13:01:29Z-a#0` · `2026-10-06T19:49:18Z-a#0` · `2026-10-06T20:38:27Z-a#1` · `2026-10-07T10:08:24Z-a#0` · `2026-10-07T10:17:50Z-a#1` · `2026-10-08T07:44:54Z-a#0`. Two sit at wrap/route-resolve, on two different rules.
- unresolved — 2: `2026-10-06T19:42:07Z-a#0` (the research playbook read after research.md was written) · `2026-10-08T09:45:46Z-a#1` (L1's related fact).

**One subject no grouping key sees whole:** `inputs.py` appears in 2 facts and 5 friction records of five
types (`2026-10-06T20:38:27Z-d`, `2026-10-07T09:50:38Z-c`, `2026-10-07T12:23:15Z-b`, `2026-10-08T08:58:17Z-b`,
`2026-10-08T10:46:58Z-c`). Each stage counts it below threshold, so it carries no proposal; it is listed for
the founder's eye.
