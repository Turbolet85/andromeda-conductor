# Evolve Diagnosis — Conductor · conductor-0.3.0 Epoch 2 "Scenario assertion hygiene" · 2026-09-17T18:15:51Z

Read-only diagnosis over `.andromeda/friction-log.ndjson`. Every proposal below is
obligation-free: accept, reject, defer or modify with no mechanism-side consequence. Nothing here
is applied, queued, or remembered — a re-run recomputes from the ledger alone.

## Mechanism health

**Records:** 143 in-epoch (76 step / 67 friction) across 5 chunks + 9 chunkless `new-session`
orientations. **Unparseable: 0** of 2299 ledger lines.

**Coverage per chunk** — **every chunk complete, zero checkpoint gaps:**

| chunk | phase | implement | wrap |
|---|---|---|---|
| `2026-09-14-emit-scrubber-and-percentile-math-under-test` | 5/5 | 3/3 | 5/5 |
| `2026-09-15-structurally-dead-assertion-class-retired` | 5/5 | 3/3 | 5/5 |
| `2026-09-15-remaining-structurally-dead-declarations-retired` | 5/5 | 3/3 | 5/5 |
| `2026-09-15-scenario-tier-honesty` | 5/5 | 3/3 | 5/5 |
| `2026-09-16-scenario-assertion-audit-gate` | 5/5 | 3/3 | 5/5 |

The chunkless step records are `new-session/orientation` ×9 plus `wrap-session/route-resolve` ×1
and `wrap-session/curation` ×1 — exactly the documented operator-requested **adaptation wrap**
shape, which legitimately carries `curation` and adds `route-resolve` with `chunk: null`. Not a
gap. This is the cleanest capture coverage in the ledger.

**Outcomes: 76/76 `ok`.** Zero `ok-degraded`, zero `halted`, zero `soft-exit` at the step level.
(One *friction* record carries `halted: 1` — see P5.)

**Untyped rate:** 1 of 67 friction records (1.5 %) — `implement/fix-loop` 1/6; every other step 0.
**Problem-fact fill:** 21/76 step records (27.6 %). **`id` fill:** 143/143 (100 %).

**Retractions** (pre-pass over the WHOLE ledger, per contract). Ledger-wide: 11 retracted friction
ids · 5 retracted problem-facts · 1 clause-retraction · 1 unresolvable · 1
retraction-of-retraction. **In range:** **1 retracted problem-fact** (`2026-09-15T05:22:32Z-a`,
`emit-scrubber-and-percentile-math-under-test`); 0 retracted friction records, 0 clause-retractions.
Stages 1–4 ran over the filtered stream.

Reported verbatim, never honored (both out of this epoch's range, carried here because the pre-pass
is ledger-wide by contract):
- **unresolvable ×1** — a pre-`id`-boundary prose-form retraction; target unknowable.
- **retraction-of-retraction ×1** — `2026-09-10T19:52:10Z-a` (0.2.0 Epoch 6b); reinstates nothing,
  flagged for founder review.

**Malformed `ts`: 11, kept and listed** — all belong to `2026-08-08-dependency-advisory-remediation`
(0.2.0), written by the pre-recipe shell append. **Zero in this epoch's range.**

**Calibration boundaries in range:** none bind. The deviation scan, the `id`/`retracts` schema and
every Universal type predate 2026-09-14, so an absence here is evidence, not era.

---

## Proposals (typed patterns)

6 groups above the F-2 threshold of 31 total.

### P1 — `phase/validate` · `contract.mechanical-check` — 7 cases · weight 19 · rate 1.4/run · 5 chunks

**Pattern:** the plan-validation checks fired on a real plan defect in **every chunk of the epoch**
— the highest per-run rate in either 0.3.0 epoch diagnosed.

**Evidence:** ALL 7 cases —

| chunk | what | impact |
|---|---|---|
| emit-scrubber | The plan stated 5 of `quantile`'s 14 survivors as predicted-equivalent; the operator re-measured under EXACT f64 comparison and only 1 is — the other four differ at 6, 8, 1616 and 2012 points. Cause: the supporting model compared with a 1e-9 epsilon | iterations 1, dialogue_rounds 1 |
| emit-scrubber | The plan's own security criterion forbids rostering any host-path scrub-path survivor, so the expected outcome is an EMPTY roster — but the gate the same plan listed treats an empty set as an unregistered unit and FAILs (`mutation-gate.py:85-87`) | iterations 1, dialogue_rounds 1 |
| structurally-dead-assertion-class | Check 5 (no placeholder leak) FAILED on one occurrence: an implementation step described the retirement header format as DECLARE-ONLY, which reads as an unsubstituted template placeholder though written deliberately | iterations 1 |
| remaining-structurally-dead | Check 8 (mechanism reach) caught an acceptance criterion asserting more than its listed gate proves — it credited one test binary with a set-equality that file's three `#[test]` arms do not establish | iterations 1 |
| scenario-tier-honesty | Check 4(6) went un-re-run after a REVIEW EDIT to an acceptance criterion; the operator's polish converted a method-less absence claim into one naming a sweep form, which the check would have required as an entry | dialogue_rounds 1 |
| scenario-assertion-audit-gate | Check 8 (mechanism reach) FAILED and was right to: the plan's step 2 and its security criterion both stated the manifest resolution one way; the shipped model states the opposite in its own doc comment | iterations 1, extra_reads 2 |
| scenario-assertion-audit-gate | Check 4(0) FAILED with UNPARSED after P5 wrote the baseline string into a new gate entry: the text contained an apostrophe escaped by doubling inside a TOML LITERAL string, which has no escape mechanism at all | retries 1 |

**Proposal:** the suite is working hard and mostly correctly — five of seven are checks catching
genuine plan defects before implement. Two observations for the founder rather than one proposal.
(a) The **first two cases are the same underlying defect in different clothes**: a plan naming a
gate whose semantics it has not verified against the gate's source (a mutation gate that fails on
an empty set; a survivor-equivalence claim resting on an epsilon the gate does not use). A check
asking "does the plan's expected OUTCOME satisfy the listed gate's own pass condition?" would have
caught both, and neither was caught by a check — one came from the operator. (b) The
`scenario-tier-honesty` case is a **re-run gap, not a check gap**: an operator review edit to a
criterion changed what check 4(6) would require, and the check was not re-run after the edit. That
is a cheap sequencing fix — re-run the criterion-derived checks after any review edit to a
criterion.

### P2 — Universal · `contract.token-proxy-check` — 9 cases · weight 15 · 3 chunks + 6 orientations

**Pattern:** a probe keyed on a token where the property is structural, and matched — or missed —
on incidental content. **Six of the nine are in `new-session/orientation`, and all six are the same
artifact: the route files' documented grammar.**

**Evidence:** ALL 9 cases —

| chunk | what | direction |
|---|---|---|
| — `new-session` | Probed master-route records with a list-item pattern (`^- `), returning 0 lines: records carry no bullet and the documented grammar is marker·status | false EMPTY |
| — `new-session` | Master-route records probed with a list-item anchored pattern plus a date; records are BARE lines per the file's own legend, so the probe returned empty | false negative |
| — `new-session` | The multi-pending anomaly scan counted the bare token `pending` and returned 2; both hits were non-records — the format legend's `pending\|complete` spelling and the word inside a complete record's prose | false POSITIVE |
| — `new-session` | Structural tail extraction anchored the separator exclusion at line start while the route writes separators indented by three spaces, so four separator lines returned as entries | false MARKERLESS |
| — `new-session` | Checks 1b/2/14 were probed with a guessed marker grammar instead of the authoritative parser regex, which `section-markers.md` spells as a separate start/end word | false ABSENCE |
| structurally-dead-assertion-class | The per-extract in-domain check (fan-out check 4) was a heuristic foreign-token list and flagged the a11y extract for carrying a build-tool token; both hits use it to state a fact about that tool | false positive |
| scenario-tier-honesty | The first corpus measurement keyed phase duration on `duration_ms` and returned 0 ms for all 36 scenarios; the field in use is `gap_ms` | false negative (an implausible all-zero column, not an error) |
| scenario-tier-honesty | A sweep written to be wrap-tolerant was not — it collapsed whitespace but left the TOML comment marker in place, so a phrase wrapping across two comment lines did not match | false negative (3 files where the truth is 4) |
| scenario-assertion-audit-gate | A post-promotion probe counted remaining markerless route entries as 23; the truth is 9 — it tested the separator prefix against the RAW line, so indented separators counted as entries | false MARKERLESS |

**Proposal:** this is the strongest single pipeline signal in the epoch. Five distinct
`new-session` probes, across five separate sessions, each re-derived the route/marker grammar by
guess and each got it wrong in a different way — while both grammars are *documented*
(`section-markers.md` carries an authoritative parser regex; the working-route header documents its
own entry/separator grammar). The skill body already warns about several of these by name, which
means prose warnings are not closing it. Candidate direction: ship the route-tail and marker
extraction as a small **shared, tested helper** the skills call, rather than a grammar each session
re-implements from prose. Three of the five would have been impossible against a helper that
matched the documented grammar once.

*(Recurrence note, measured from the ledger: this class continues — the following epoch carries 6
more cases, and the orientation run that produced this diagnosis hit the same class again on the
working-route H1.)*

### P3 — Universal · `contract.narrow-basis-claim` — 7 cases · weight 11 · 3 chunks

**Pattern:** a count or identity stated from a source narrower than the claim — a summary layer, a
roll-up contradicting its own data, a partial read.

**Evidence:** ALL 7 cases —

| chunk | what | impact |
|---|---|---|
| — `wrap-session/route-resolve` | Three facts dictated in an operator relay for a new entry's evidence freight were falsified against the artifacts they cited, all three sourced from a summary layer narrower than the claim | extra_reads 6 |
| emit-scrubber | Research recorded a fingerprint transcription's currency as "unverified at Pulse's present HEAD" on the basis that its pinned sha is older than one a recent chunk measured against — a sha being older is not evidence the content moved | dialogue_rounds 1 |
| emit-scrubber | The report asserted that the gate script's survivor identity contradicts a rule file's stated identity, from reading the gate source and the rule file but NOT the citation home that already reconciles them | extra_reads 1, reformulations 1 |
| structurally-dead-assertion-class | The originating residual and the route outline each named ONE pinning test for the retirement; direct measurement found FIVE affected tests, two asserting family membership | extra_reads 4 |
| structurally-dead-assertion-class | Concluded from one composition site plus a helper body that the report markdown never reaches the graded text, and was about to record it as corrected ground | extra_reads 1 |
| — `new-session` | The handoff re-seed carry named one file as the code-graph drift; the Check 11 currency arm measures TWO files behind their templates | extra_reads 2 |
| scenario-assertion-audit-gate | A caller count was stated from a source narrower than the claim and was wrong in two artifacts — research prose said "4 callers" then listed FIVE sites in the same sentence, a roll-up contradicting its own data layer | extra_reads 2, iterations 1 |

**Proposal:** the distinctive sub-shape here, and the one worth the founder's attention, is the
**roll-up that contradicts its own data in the same artifact** (case 7) and the **summary layer
cited in place of the row** (case 1, three facts at once). Both are cheap to detect mechanically:
where a prose count sits beside an enumeration in one artifact, the two can be compared. The rest
are the standing class already covered by the project's Tier-1 corpus — see P4, which is precisely
about corpus entries that state the rule correctly and do not prevent the failure.

### P4 — Universal · `recall.corpus-recurrence` — 7 cases · weight 8 · **5 of 5 chunks**

**Pattern:** a failure occurred whose governing rule was **already stated correctly and completely**
in the project's own corpus. Every chunk in the epoch produced at least one, and curation deduped
each against an existing entry rather than minting a new one — so the ledger records the recurrence
and the corpus grows by nothing.

**Evidence:** ALL 7 cases —

| chunk · step | the entry that already stated the rule | what recurred |
|---|---|---|
| emit-scrubber · `curation` | CLAUDE.md Tier-1 2026-08-09 ("Before asserting that document A says X, grep A", extended to an agent asserting a false claim about its own source) | the session asserted one doc contradicts another without reading the section that OWNS the claim |
| structurally-dead-assertion-class · `reconcile` | `host-win32.md` 2026-09-10 (a line-granular grep cannot disposition a clause inside a multi-KB single-line entry; resolve by OFFSET) + CLAUDE.md 2026-08-21 (every hit needs an explicit disposition) | the same multi-KB-line disposition failure |
| structurally-dead-assertion-class · `curation` | the same `host-win32.md` 2026-09-10 entry, "present, in context, and correct" | deduped; not re-minted |
| remaining-structurally-dead · `reconcile` | CLAUDE.md Tier-1 ("a stale MECHANISM outlives the stale NAME — sweep for what the claim SAYS, not what it is NAMED after") | the cascade sweep was keyed on the five scenario NAMES; only a second pass keyed on the retired TOKENS found the rest |
| remaining-structurally-dead · `curation` | CLAUDE.md Tier-1 2026-09-04 (jointly-contradictory plan steps) | "an entry that states the rule correctly and did not prevent this chunk's plan from shipping one" |
| scenario-tier-honesty · `curation` | CLAUDE.md Tier-1 2026-08-22 ("before grepping for a token as a proxy for a practice, confirm the project prescribes THAT token") | the `duration_ms`→`gap_ms` probe miss; "the entry was not consulted" |
| scenario-assertion-audit-gate · `curation` | CLAUDE.md Tier-1 enumeration-basis rule (and a second entry) | two findings deduped against entries that already state their rule correctly; neither re-minted |

**Proposal:** this is the epoch's most consequential finding and it is not a project defect — it is
a signal about the **corpus mechanism itself**. Seven times in five chunks, the failure's governing
rule existed, was correct, was complete, and did not fire. Curation's dedup filter then correctly
declined to mint a duplicate, which is the right local call and leaves the mechanism with no
response at all: the recurrence is recorded in the friction ledger and nowhere the next chunk will
read.

Three directions, offered without ranking. (a) **Recurrence is currently invisible where it
matters.** A rule that has been recurred-against N times is a different object from one recurred
against zero times, and nothing surfaces that. Curation could annotate the deduped-against entry
with a recurrence count, making high-recurrence rules visible to the founder and to the next
session. (b) **Several notes say the entry "was not consulted."** That points at retrieval, not
content — the Tier-1 corpus is now ~46.7 KB across 16 bullets (measured this session; 9 bullets
over the 600 B cap, +39.9 KB above it), which is a lot of prose to consult at the moment a probe is
being written. Whether the always-loaded tier has passed the size at which it functions as
guidance is a founder question this data speaks to directly. (c) At least three of the seven are
the *same family* as P2's token-proxy cases — a rule about probe-writing that keeps not reaching
the moment a probe is written. Where a rule governs a mechanical act, a mechanical check may be the
only form that fires.

### P5 — `wrap-session/reconcile` · `ambiguity.playbook-no-match` — 3 cases · weight 8 · 3 chunks

**Pattern:** the amendment pass met a class no rule of the playbook governs, three times in one
epoch. One case **halted**.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| emit-scrubber | The pass's one amendment — a mutation survivor accepted on proven EQUIVALENCE rather than on a standing rule foreclosing the test — matched none of the playbook's 46 rules; the prior wording admitted only "when a standing rule prescribes the untested shape" | dialogue_rounds 0 |
| structurally-dead-assertion-class | No rule of the 47 governs "a spec body states a scenario's check-membership that a retirement falsifies"; rule @171 subject-matched the count half but its qualifier is false here | dialogue_rounds 0 |
| scenario-assertion-audit-gate | Playbook `:100` was minted for exactly this class — a new committed runtime-parsed artifact under `contracts/` earning a security Input-Validation row — but one of its qualifying clauses requires the artifact be read at a fixed `default_path()` through `resolve_under`, and this ledger has no such reader | **halted 1**, dialogue_rounds 1 |

**Proposal:** the third case is the instructive one and the reason this group clears threshold on
impact: a rule minted *for this exact class* still failed to match, because a qualifying clause
encoded an implementation detail of the earlier instance (how the artifact is resolved) rather than
the property the rule is about (a committed runtime-parsed contract artifact). It halted the pass.
Candidate direction: when a playbook rule is minted from one instance, its qualifying clauses are
the place where over-fitting happens — a minting-time prompt to distinguish the *class property*
from the *instance's incidental mechanism* would have caught it. The playbook grew 46→47→48 across
the epoch and still met three unmatched classes, which may itself be worth reading as a rate.

### P6 — Universal · `contract.premise-falsified` — 4 cases · weight 4 · 3 chunks

**Pattern:** verification falsified a premise an authored artifact states. Notably, **two of the
four falsify the chunk's own promotion-time doubt or a CARRY's present-tense claim** — i.e. the
premise proved *over*-cautious, not over-confident.

**Evidence:** ALL 4 cases —

| chunk · step | what | impact |
|---|---|---|
| emit-scrubber · `take-up` | The phase directive stated a gate form and separately named the instrument that would run it; reading that instrument at HEAD shows it cannot express that command | extra_reads 3 |
| structurally-dead-assertion-class · `research` | scope.md stated, inheriting the residual's live-round attribution, that no producer emits the token a scenario asserts; re-derivation at the SUT's HEAD falsified it — a source file pushes the literal | extra_reads 4 |
| scenario-assertion-audit-gate · `take-up` | CARRY-3's present-tense clause — a gate on the single-line form reads a false green on a file still carrying the retired claim — is false at HEAD; both sweeps return clean | extra_reads 1 |
| scenario-assertion-audit-gate · `research` | Research falsified a premise SCOPE stated — and the falsified premise was the chunk's own promotion-time DOUBT about a CARRY, not the CARRY | extra_reads 6 |

**Proposal:** at n=4 over 5 chunks this is a materially lower rate than the following epoch's 15,
and the two "doubt falsified" cases are arguably the mechanism working — a promotion-time doubt
recorded, then measured and dissolved. The one with a clear direction is the first: a phase
directive naming an instrument that cannot express the command it also states. That is the same
shape as P1(a) — an artifact naming a gate without checking the gate's source — and the two
together make the case that *naming a gate* should carry a cheap obligation to read it.

---

## Cross-step chains (starting heuristics)

### X1 — `wrap-session/report` →`report`→ `wrap-session/reconcile` — **3 chunks**

The producer closed formally `ok` with confident signals each time; the consumer's verdict was
`thin`, `thin`, `wrong`:

| chunk | producer signals | consumer verdict + note |
|---|---|---|
| emit-scrubber | `reconstructed`, `directive-figures-reproduced`, `ledger-claim-verified` | **thin** — "sufficient for all seven detectors (none reported a gap, several cited specific bullets) but it shipped one FALSE finding" |
| structurally-dead-assertion-class | `detector-scan-performed`, `dictated-coordinate-corrected` | **thin** — "thin on one axis: the report's Expected-amendments bullet made a caught-ALL sweep claim that under-ran by two of three arch sites" |
| remaining-structurally-dead | `detector-scan-clean`, `every-count-carries-its-derivation`, `plan-prediction-measured-not-inherited` | **wrong** — "six of seven detectors ran clean off it and every other bullet held, but ONE clause it shipped was falsified at validate" |

**Chain hypothesis:** the report step reliably produces output that is *mostly* right and carries
one falsified or over-claiming clause, and the downstream detectors do not catch it — in the third
case six of seven detectors ran clean off a report carrying a falsified clause. The failure is
concentrated in **sweep-completeness and count claims** ("caught ALL", a count bullet), which is
the same class as P3. Note the third producer's own signal was
`every-count-carries-its-derivation` — the report asserted the discipline and still shipped the
falsified clause, which suggests the signal is self-reported rather than checked.

**Proposal direction:** the detectors consume the report and evidently cannot falsify its
completeness claims. A narrow candidate: treat a report's "swept all / caught all" claims as
requiring the sweep's own output attached, so a detector has something to check rather than prose
to read. Whether that is worth the weight is the founder's call.

Below the ≥2-chunk threshold (single-chunk, in the appendix): `implement/smoke`→conversation→
`wrap-session/report` · `wrap-session/curation`→conversation→`wrap-session/report` ·
`implement/fix-loop`→implement-outcome→`wrap-session/report` · `implement/smoke`→
implement-outcome→`wrap-session/report`. The last two share a note worth recording even below
threshold: *"no report.md in the chunk dir — implement never authored a P4 report"*, and *"implement
ran in a PRIOR session; this wrap opened fresh at new-session, so no conversational trace of the
chunk's work existed."* That is the same seam X1 of the following epoch measures from the other
side.

---

## Level candidates (systemic-masked-as-project)

Problem-fact pile: **22 facts** — `workaround` 10 · `removed-cause` 5 · `deferred` 4 ·
`prohibition` 3; nature `process` 15 · `product-logic` 4 · `environment` 3.

### L1 — band-aid — the Bash tool's MSYS shell vs POSIX-shaped recipes — 4 facts

**Facts:**

| chunk · step | nature · solution | note |
|---|---|---|
| emit-scrubber · `wrap-session/reconcile` | environment · workaround | body-edit verification aborted mid-chain: `grep -c` returned 0 on a mis-written pattern and the non-zero exit killed the `&&` chain; re-ran with `;` separators and `\|\| true` |
| — · `new-session/orientation` | environment · workaround | `bc` absent on the host, so the Check 4 rule-file size render printed 0.0 KB for all 7 files; re-derived in python |
| scenario-tier-honesty · `phase/distill` | environment · removed-cause | a `cd` into the run dir persisted and re-based the working directory (*the documented host-win32 hazard*); removed by re-anchoring every later call absolutely |
| scenario-tier-honesty · `wrap-session/route-resolve` | process · removed-cause | a verification used `grep -c` to count occurrences on a single line and read 1 where the truth is 3 — `grep -c` counts matching LINES, and the entry is one 3093-char line. *"This is the exact hazard curated into"* the corpus |

**Level hypothesis:** the cause lives in the PIPELINE (health-check and verification recipes
authored POSIX-shaped) and every fix landed in the PROJECT — a per-run adaptation, or a clause in
the project's own host rule file. Two of the four notes explicitly say the hazard was already
documented when they hit it, which makes this theme and P4 (`recall.corpus-recurrence`) the same
story told by two instruments.

**Measured recurrence:** the same theme carries **6 facts in the following epoch** (5 of them in
`new-session/orientation`, including the `bc` failure a second and third time). It is growing, not
decaying.

**Proposal:** the `bc` case is the cheapest and most clearly pipeline-side: it fails *silently*,
printing 0.0 KB and an empty total rather than an error, so a dashboard reader takes it as a
measurement. Computing the size column in python — already every project's fallback — would close
it once for every project.

### L2 — band-aid — a whole-file-Write mandate meeting multi-KB single-line artifacts — 2 facts, recurring

**Facts:**

| chunk · step | nature · solution | note |
|---|---|---|
| emit-scrubber · `phase/distill` | process · workaround | `fan-out.md` mandates persisting extracts via the Write tool; the 4 raw twins were instead generated by a python read-transform-write script. Obstacle: hand-re-encoding four multi-KB bodies is error-prone |
| emit-scrubber · `wrap-session/gates` | process · workaround | flip-compaction: route-resolve and `line-write-contract` both prescribe ONE whole-file Write, but `working-route.md` carries multi-KB lines (one at 3086 chars) and only ONE line changed; used a python read-modify-write by path |

**Level hypothesis:** two references prescribe a whole-file Write as the safe form; the project's
route and extract artifacts carry multi-KB single lines that make a whole-file re-author the
*riskier* option. The practitioner routed around the mandate both times and recorded why. The
mandate is right about its hazard (shell quoting) and silent about this one.

**Proposal:** a stated exception — a validated read-modify-write by path, with a pre/post assertion
— for artifacts carrying lines past some width, so the safe act is the prescribed act rather than a
documented deviation. Note the project's own host rule file already prescribes exactly this for
long single-line files; the pipeline references do not acknowledge it. *(Related, sub-threshold:
one further fact records the hand-escaped JSON heredoc form being refused whole because a record
value carried a backslash — the same collision between a prescribed transport and real content.)*

### L3 — observation — host-path hygiene in committed run dirs — 2 facts, recurring

**Facts:** `structurally-dead-assertion-class` · `phase/validate` — deleted the two known-positive
control input files from the phase run dir after recording their result, because the leaky control
deliberately contained a drive-letter path and the run dir is committed. ·
`remaining-structurally-dead` · `phase/distill` — the design distiller's return carried two absolute
host paths *(the ones this orchestrator supplied in its own prompt)* and phase run dirs are
committed, so the persist contract stripped them.

**Observation, not a proposal:** sub-threshold at n=2 in-epoch, but it recurs in the following
epoch (a distiller returning an absolute host path in a Patterns bullet). The second fact names its
own cause — the orchestrator supplied the paths in the prompt — which is a pipeline-side detail a
founder may want: sub-agent prompts carrying absolute paths produce extracts that then need
scrubbing before they can be committed.

### Signatures checked and NOT fired

- **Chronic-degrade — does not fire.** Zero `ok-degraded` outcomes (76/76 `ok`).
- **Deferred-forever — does not fire.** 4 `deferred` facts, every one routed and three
  demonstrably closed: the `mutation-gate.py`/`timeout.txt` gap (recorded twice, at `code` and
  `fix-loop`) is carried by a named Epoch-5 route entry; the 6 further structurally-dead blocks
  became the *next chunk* in this epoch; the stale count comment names its disposition in its own
  note. Per the mechanics reference, a note that names its destination is a routed deferral, not an
  orphan.
- **Override — does not fire.** Zero `solution: overridden` facts.
- **Recurring `removed-cause` theme — does not fire.** The 5 `removed-cause` facts remove
  different causes.

---

## Playbook-extension candidates (untyped patterns, F-4)

**None.** The epoch carries exactly **one** untyped friction record (1.5 % untyped rate — the
lowest in the ledger), so no cluster can form. It is listed in the appendix.

---

## Below threshold — no action

**Typed groups (25 of 31 below threshold).**
`wrap-session/route-resolve`/`contract.carry-no-owner` n=1 **weight 6** — the epoch's
highest-weight singleton, worth an eye · `tooling.host-shell` n=2 (2 chunks) ·
`wrap-session/curation`/`ambiguity.filter-borderline` n=2 · `phase/research`/`input.cookbook-gap`
n=1 (weight 3) · `implement/code`/`input.plan-step-ambiguous` n=2 ·
`wrap-session/reconcile`/`contract.false-positive-proposal` n=2 ·
`phase/plan`/`input.extracts-conflict` n=1 · `implement/fix-loop`/`contract.test-expectation` n=1 ·
`tooling.output-cap-overflow` n=1 · `phase/plan`/`retry.synthesis-rework` n=1 ·
`contract.skill-reference-drift` n=1 · `phase/research`/`input.extract-signal-gap` n=1 ·
`implement/fix-loop`/`tooling.result-not-run-stable` n=1 ·
`implement/fix-loop`/`contract.instrument-validity` n=1 ·
`new-session/orientation`/`input.handoff-git-mismatch` n=1 ·
`wrap-session/report`/`input.implement-outcome-unsettled` n=1 ·
`wrap-session/gates`/`tooling.result-not-run-stable` n=1 ·
`new-session/orientation`/`tooling.health-false-red` n=1 ·
`implement/code`/`input.research-files-wrong` n=1 ·
`implement/fix-loop`/`contract.spec-reality-gap` n=1 ·
`implement/fix-loop`/`contract.matrix-claim` n=1 ·
`wrap-session/reconcile`/`input.report-insufficient` n=1 ·
`phase/take-up`/`input.out-of-pipeline-source` n=1 ·
`wrap-session/gates`/`tooling.commit-mechanics` n=1 · `contract.structural-blind-spot` n=1.

Note `contract.instrument-validity` is n=1 here and n=6 in the following epoch — a class that
arrived with the CI-probe work rather than a standing one.

**Untyped clusters (1, n=1).** `implement/fix-loop` — the sole untyped record of the epoch; no
cluster.

**Chain shapes below the ≥2-chunk threshold.** `implement/smoke`→conversation→
`wrap-session/report` · `wrap-session/curation`→conversation→`wrap-session/report` ·
`implement/fix-loop`→implement-outcome→`wrap-session/report` · `implement/smoke`→
implement-outcome→`wrap-session/report` (see the note under X1 — the missing P4 report and the
cross-session wrap).

**Problem-fact themes below threshold.** Prescribed-transport collisions (the hand-escaped JSON
heredoc refused whole on a backslash-bearing value) — 1 fact, related to L2 · a plan step
delegating a correction to a step that structurally cannot make it — 1 fact · curation tiebreaker
sending a candidate past the Tier-2 cap into files already over it — 1 fact, which touches the same
corpus-size question P4(b) raises · a guard test and two warning comments added as `prohibition`
facts — 3 facts, all deliberate and each naming its reason.

---

*Evidence twins in this run dir: `q-retractions.json` · `q-health.json` · `q-typed.json` ·
`q-untyped.json` · `q-chains.json` · `q-level.json`.*
