# Evolve Diagnosis — Conductor · conductor-0.3.0 Epoch 3 "The a11y capability's terminal" · 2026-09-17T18:06:51Z

Read-only diagnosis over `.andromeda/friction-log.ndjson`. Every proposal below is
obligation-free: accept, reject, defer or modify with no mechanism-side consequence. Nothing here
is applied, queued, or remembered — a re-run recomputes from the ledger alone.

## Mechanism health

**Records:** 128 in-epoch (55 step / 73 friction) across 4 chunks + 4 chunkless `new-session`
orientations. **Unparseable: 0** of 2299 ledger lines.

**Coverage per chunk** (distinct steps vs expected — `phase` 5 · `implement` 3 · `wrap-session` 5;
`new-session` is per session start and carries `chunk: null` by design, so it is not a per-chunk row):

| chunk | phase | implement | wrap | gaps |
|---|---|---|---|---|
| `2026-09-16-a11y-ci-gate-at-an-honest-terminal` | 5/5 | 3/3 (`fix-loop` ×5) | **2/5** | curation, reconcile, route-resolve |
| `2026-09-16-medium-integrity-launch-for-the-a11y-routine-arm` | 5/5 | 3/3 | 5/5 | — |
| `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration` | 5/5 | 3/3 | **4/5** | curation |
| `2026-09-17-keyboard-and-focus-order-coverage-ownership` | **4/5** | 3/3 | 5/5 | plan |

Three checkpoint gaps. Two are `curation` — legitimately absent from a `--no-curation` wrap, so
those need no explanation. The two that do: `a11y-ci-gate` is missing `reconcile` and
`route-resolve` as well (3 of 5 wrap checkpoints did not fire), and
`keyboard-and-focus-order-coverage-ownership` is missing `phase/plan`. `fix-loop` ×5 on
`a11y-ci-gate` is not over-coverage — that step fires once per fix iteration.

**Outcomes: 55/55 `ok`.** Zero `ok-degraded`, zero `halted`, zero `soft-exit` in the whole epoch.
Across the ledger `ok-degraded` runs 1 · 6 · 4 · 3 · 3 · 5 for 0.2.0's epochs and 1 · 0 · 0 for
0.3.0's — the last two epochs are the first with none. This is why the chronic-degrade signature
does not fire below; it is a measured absence, not an unexamined one.

**Untyped rate:** 2 of 73 friction records (2.7 %) — `new-session/orientation` 1/5,
`phase/take-up` 1/4; every other step 0. **Problem-fact fill:** 22/55 step records (40 %).
**`id` fill:** 128/128 (100 %).

**Retractions** (pre-pass over the WHOLE ledger, per contract — a false conclusion can be found an
epoch late). Ledger-wide: 11 retracted friction ids · 5 retracted problem-facts · 1
clause-retraction · 1 unresolvable · 1 retraction-of-retraction. **In range:** 0 retracted
friction records, **2 retracted problem-facts** (both `medium-integrity-launch`), **1
clause-retraction** — record `2026-09-16T12:05:17Z-b`, whose `what` carries its note in the P7
evidence table below. Stages 1–4 ran over the filtered stream.

Reported verbatim, never honored, for the founder's manual discount:
- **unresolvable ×1** — `[null, "prose-form-pre-boundary", "the untyped code-graph-under-reports
  record and the first problem-block entry on"]`. Pre-`id`-boundary prose form; target unknowable.
- **retraction-of-retraction ×1** — `2026-09-10T19:52:10Z-a` (0.2.0 Epoch 6b). Retracting a
  retraction reinstates nothing; flagged for founder review by contract.

**Malformed `ts`: 11, kept and listed** — all eleven belong to
`2026-08-08-dependency-advisory-remediation` (0.2.0), each carrying a Windows `date` prompt string
instead of a timestamp, written by the pre-recipe shell append. **Zero are in this epoch's range.**
They stay in every count: they prove their checkpoints fired, and no stage orders by `ts`.

**Calibration boundaries in range:** none bind. The deviation scan, the `id`/`retracts` schema
(2026-08-18) and every Universal type (2026-09-08/09-09) were all live before this epoch opened on
2026-09-16, so an absence here is evidence, not era.

---

## Proposals (typed patterns)

7 groups above the F-2 threshold of 25 total. Universal and `recall.*` types group by type alone
across steps, per the mechanics reference.

### P1 — Universal · `contract.premise-falsified` — 15 cases · weight 27 · 4 chunks

**Pattern:** every chunk in this epoch spent measurement disproving a premise that an authored
artifact — a route entry, a matrix note, a plan step, a scope bullet, a security-plan clause —
stated as settled fact.

**Evidence:** ALL 15 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| a11y-ci-gate | The limited-token remedy was never exercised: the witness reports IsElevatedAdmin True / IntegrityLevelSid S-1-16-12288 (High) despite the register step reporting ok with RunLevel Limited | iterations 2 | probe run 35102123985 |
| a11y-ci-gate | Hypothesis that hosted runners disable UAC is falsified — the runner reports EnableLUA 1, so a split token should exist | iterations 1 | probe run 35103823579 |
| a11y-ci-gate | Limited-token arm is structurally unavailable: the job account is the built-in Administrator (RID 500) with FilterAdministratorToken absent, so no filtered token exists | iterations 3 | probe run 35105588216 |
| a11y-ci-gate | Pipe route eliminated, not for the assumed reason: msedgedriver has pipe support internally but exposes no surface to select it; `--help` lists 18 options, none pipe-related | extra_reads 3 | msedgedriver 152.0.4191.53 `--help` |
| a11y-ci-gate | Driver/runtime major skew eliminated as the dev-host cause — aligning to the 153 runtime changed nothing across three legs | iterations 1 | failing sites `accessibility.e2e.ts:384`, `:397` |
| a11y-ci-gate | The Evergreen runtime float charged a cost BEFORE the gate bounding it ever lit; an unprompted overnight bump broke the rig | — | runtime dir 153.0.4234.32 created five days after |
| a11y-ci-gate | The token question SPLITS the variable the elevation finding rested on: `runas /trustlevel:0x20000` drops the administrator ROLE but not the INTEGRITY LEVEL | iterations 1 | probe run 35111618735 |
| a11y-ci-gate | Integrity level, not the administrator role, decides WebView2 session creation — variation with a control on both sides across three legs; retires both routes the chunk built | iterations 3 | A admin+High → no session; B False+High → no session |
| medium-integrity | The predecessor's dev-host finding does NOT transfer to the runner: non-admin+Medium created a session in 6 s locally; on the runner Medium fails too, different signature | iterations 1 | CI run 35150449243 |
| a11y-routine-arm-terminal | The working entry and the v3-02 matrix note both cite "expected and received bracket lists are identical to the character" as proof the same six controls were reached; both sides of the `.toBe()` interpolate the SAME expression | extra_reads 1 | `accessibility.e2e.ts:384` |
| a11y-routine-arm-terminal | The entry's PREMISE — "the arm RUNS and is one assertion from green" — is false on HEAD's configuration (the green run used a different image, a coherent driver/runtime pair and no launcher) | extra_reads 2 | `ci.yml:250`, `:282` |
| a11y-routine-arm-terminal | The entry and ledger note prescribe recording the defect as "12 visits / 6 distinct"; enumeration shows six distinct ELEMENTS but only five distinct NAMES | extra_reads 3 | `CoverageMatrix.tsx:60`, `RunReport.tsx:47` |
| a11y-routine-arm-terminal | Plan step 7 stated the token-witness readback's subject "only exists inside a launched process" and directed removing it; the script's own synopsis falsifies that | extra_reads 2 | `scripts/a11y-token-witness.ps1:1-16`, `:269-291` |
| keyboard-and-focus-order | P3 falsified two scope premises authored at P1: the unattributed set is five rows not seven, and the hold-free pair's attribution claim | extra_reads 2 | research.md F1 · §Scope premise closure |
| keyboard-and-focus-order | CARRY 3's premise disproved: all 12 `msedgewebview2` processes are children of ordinary desktop apps by parentage — no orphan defect exists in the routine arm | extra_reads 1 | `Win32_Process` parentage census |

**Proposal:** this is the epoch's dominant class by a wide margin, and its shape is consistent —
the falsified premise is almost always carried FORWARD in an artifact (route entry, matrix note,
plan step) rather than authored fresh. Two directions worth the founder's judgment. (a) The
premises that cost the most here were *inherited* ones: a route entry and a matrix note that
asserted a configuration still held. A `phase/take-up` obligation to re-measure the entry's
load-bearing premise against HEAD *before* P1 authors scope would move this cost from fix-loop
(where 8 of 15 landed) to take-up. (b) Nothing in the pipeline distinguishes a premise that was
*measured* from one that was *reasoned*; the artifacts read identically. A provenance marker on
premise-bearing claims in route entries and matrix notes would let a later reader see which ones
were never measured.

### P2 — Universal · `contract.narrow-basis-claim` — 11 cases · weight 13 · 3 chunks

**Pattern:** a count, absence or site-list was stated from a basis narrower than the claim — a
clipped view, a recollection, a targeted re-grep that dropped what a broader basis had already
established.

**Evidence:** ALL 11 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| a11y-ci-gate | A standing learning attributes CRLF extracts to a python text-mode write and states the Write tool "stays LF"; all seven extracts persisted this step were Write-tool-only and are 100 % CRLF | extra_reads 3 | 47/47, 46/46, 42/42, 43/43, 46/46, 43/43, 46/46 CRLF |
| a11y-ci-gate | The take-up directive stated the arm failed "after three session attempts over ~63 s"; the log carries ONE failure event rendered on four lines at a single instant | extra_reads 2 | run 35079315258 log |
| a11y-ci-gate | Reported `census()`'s call sites as two after a targeted grep, when this session's own earlier code-graph query had returned three | extra_reads 1 | `census()` at 240, 396, 450 |
| a11y-ci-gate | `runas` was excluded by reasoning rather than measurement — plan and an evidence record both state it is barred because it prompts; `/trustlevel` does NOT prompt (only `/user:` does) | extra_reads 2 | local probe, exit 0 non-interactively |
| a11y-ci-gate | Discharged v3-02's obligation (2) by citing a 2026-09-10 verdict and showing the console unchanged — that established one INPUT had not moved, not the outcome | extra_reads 2 | two local legs minutes apart |
| a11y-ci-gate | Reported the skewed legs as failing at `:397` only; both earlier readings came from clipped views (`tail -6`, `head -4`) — re-derived over the whole file, all three legs fail at the same sites | extra_reads 1 | `grep -oE` over each log |
| a11y-ci-gate | Third instance in one session, across implementer and reviewer, of one class: a count or site list read off a tail/head view and different when re-derived over the whole artifact | — | the project's standing rule already forbids it |
| medium-integrity | research.md's Scope line was written "Graph queries: 3 (2 rust, 1 ts)" from recollection; the trace file holds 2 | extra_reads 1 | phase run dir tree-query trace |
| medium-integrity | Concluded "tauri-driver never came up" from a grep covering driver ERRORS but not its STARTUP lines; the log said the opposite | iterations 1 | CI run 35143892765 |
| medium-integrity | The endpoint-at-Medium question WAS asked and answered; read as unasked because the sweep keyed on the HIGH-run signature — at Medium the error is a different string wrapped across two lines | iterations 1 | CI runs 35145664132 + 35147042449 |
| keyboard-and-focus-order | The report's Symbols bullet asserted a grep returns 1 file from reasoning about what the module imports rather than running it; run, it returns 2 | extra_reads 1 | corrected in place before fan-out read it |

**Proposal:** 11 cases and the project's own Tier-1 corpus already forbids the central instance
("a `LIMIT`-ed or `head`-ed view is never the result"), so a further *rule* is not the gap — the
rule exists and was violated 11 times in one epoch. What is absent is any mechanical moment where
a claim's basis is stated. A candidate: where a step record's `what` or a report asserts a COUNT
or an ABSENCE, require the derivation beside it (the bare command whose output was read), making
an unstated basis visible rather than relying on recall. Note three cases were self-caught and
corrected in place — the class is being detected, just late.

### P3 — `implement/fix-loop` · `contract.instrument-validity` — 6 cases · weight 13 · rate 0.75/run

**Pattern:** the instrument built to answer a question was structurally incapable of answering it,
and its output looked like a valid reading. 6 cases over 8 `fix-loop` runs, in 2 chunks.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| a11y-ci-gate | The witness's first two integrity readings could not answer: a bare `whoami /groups` resolved to the MSYS POSIX build and rejected the flag, and the .NET fallback omits the mandatory label entirely | retries 2, extra_reads 3 | validated on a known-positive before shipping |
| a11y-ci-gate | The pipe-route diagnostic cannot discriminate ACCEPTED from IGNORED — a loader silently ignoring an unknown switch leaves the app alive and the profile created identically | — | its three lines are shape-identical to the port-mode case |
| a11y-ci-gate | `A11Y_LIMITED_TOKEN_REGISTER: ok` was treated as evidence the leg ran limited; it only records what the scheduler was ASKED to register, and the neighbouring reading came from the job's own shell, not the task | retries 1 | closed by a witness running AS the task's program |
| a11y-ci-gate | The witness's UAC block threw on an absent property under `Set-StrictMode -Version Latest`, aborting the loop and losing the two keys that would have explained the High-integrity task | retries 1, iterations 1 | probe run 35103823579 printed UNREADABLE |
| a11y-ci-gate | The orphan reap's own OrphansLeft reading under-reports — it counts while the app tree is still settling; self-reported 1 left, an independent audit measured 6 genuine in-window | retries 2 | leg window 18:33:32–18:35:00 |
| a11y-routine-arm-terminal | `gate.py delta --defer-check rust` erred in BOTH directions on one call — a false positive from a doc comment citing a different file, and a matching false negative | extra_reads 2 | gate.py delta output |

**Proposal:** this is the epoch's highest per-run rate (0.75 instrument-validity failures per
`fix-loop` run) and it is concentrated in one chunk's CI-probe work. The corpus already carries the
rule that closes it — "run a probe against a KNOWN-GOOD path first; a probe that fails where the
real path passes is evidence about the probe" — and one of these six records shows it being
applied ("validated on a known-positive before shipping"). The proposal is to make that
known-positive control a *stated* step of the fix-loop playbook when the fix ships a new
diagnostic, rather than a rule the implementer may or may not recall: the five failures and the one
success differ precisely on whether the control was run.

### P4 — Universal · `contract.token-proxy-check` — 6 cases · weight 10 · 2 chunks + 2 orientations

**Pattern:** a check keyed on a TOKEN where the intended property is semantic, and the match was
satisfied by incidental, self-referential or prose-about-the-token content. Both directions occur.

**Evidence:** ALL 6 cases —

| chunk | what | direction | impact |
|---|---|---|---|
| — (`new-session/orientation`) | Check 11 currency applied a fenced-template extraction to the cookbook, whose template is a whole `.md` with no wrapping fence; the regex caught an inner ```sql block and reported a false DIFFERS +75 L | false positive | extra_reads 2, retries 1 |
| — (`new-session/orientation`) | The marker extractor assumed a `…:start` grammar; the deployed grammar puts a space before `start`/`end`, so the pointer-table block reported NOT FOUND | false absence | reformulations 2 |
| a11y-ci-gate | Reported the dev-host account as having no admin rights on `IsInRole(...)` returning False — which is what a DENY-ONLY membership returns under a UAC split token | false absence | extra_reads 2 |
| a11y-ci-gate | The cascade sweep for "six governed forms" returned 0 while the rule file stated it as bolded **SIX** — markdown emphasis defeating the literal pattern | false absence | extra_reads 2 |
| keyboard-and-focus-order | Validation-check-3 keyed on a section-anchor token and returned 3 FALSE POSITIVES; reading the hits dissolved all three | false positive | retries 0 |
| keyboard-and-focus-order | `matrix.py audit`'s ledger-note listing keys on the literal Expected-amendment token, so a parenthetical stating that this chunk routes NO such amendment was matched as a real entry — a phantom instruction into wrap P7.3's input | false positive | retries 1 |

**Proposal:** two of the six are in pipeline TOOLING rather than an agent's ad-hoc probe — Check
11's currency arm in `new-session` and `matrix.py audit`'s ledger-note listing — and those two are
the ones a founder can fix once for every project. The `matrix.py` case is the more serious: it
fabricated an instruction into a downstream step's input, and was caught only because the author
read the hit. Both are candidates for a semantic check (or, for `matrix.py`, matching the
Expected-amendment ENTRY grammar rather than its token anywhere in the section). The remaining four
are agent-side and are already covered by the project's Tier-1 rule; they recurred anyway — see
the recurrence note in L1.

### P5 — `phase/validate` · `contract.mechanical-check` — 5 cases · weight 12 · rate 1.25/run

**Pattern:** the plan-validation checks fired on real defects in the plan under review — five
findings over four validate runs, three of them caught by the operator rather than the check.

**Evidence:** ALL 5 cases —

| chunk | what | impact |
|---|---|---|
| medium-integrity | Gate entry refused at the check 4(9) baseline with "not run — env f, null unset": the gate tool's env predicate treats every `$NAME` in a run string as a shell handle that must be set, so any inline PowerShell/awk/jq carrying its own variables is unfireable | iterations 1 |
| medium-integrity | Check 5 found two `{…}` placeholder leaks in non-code text; check 4(10) found the ci-probe's recency selector pinned to this run only in PROSE, not inside `run` | iterations 1 |
| a11y-routine-arm-terminal | The plan shipped a false ABSENCE claim into its own forecast — "the image's own driver coherence was never exercised" — and the operator caught it; it was exercised twice, both readings negative | dialogue_rounds 2, extra_reads 2 |
| a11y-routine-arm-terminal | The plan closed the vacuous-by-construction assertion class at ONE site and asserted the neighbour two lines away was already correct; the operator caught the second site | iterations 1 |
| keyboard-and-focus-order | Check 4(5) fired a REQUIRED-RESOLUTION correctly: the plan carried a `leg = 'live'` entry while research.md's slot read "none — no live leg in this chunk" — P3 wrote that slot before P4 authored the leg | extra_reads 1 |

**Proposal:** the check suite is working (2 of 5 are the mechanical checks catching plan defects
exactly as designed, and one is a correct cross-artifact consistency catch). The signal is in the
other three. Two were caught by the **operator**, not by any check, and both are the same shape: a
plan asserting an ABSENCE or a completeness claim about the code it is about to change ("never
exercised", "the neighbour is already correct"). Nothing in validate tests a plan's absence-claims
against the artifact. That is a candidate check. The `env f, null unset` case is not a plan defect
at all — it is the gate tool refusing a valid entry, and it appears again in L2.

### P6 — Universal · `contract.structural-blind-spot` — 4 cases · weight 6 · 2 chunks

**Pattern:** a documented mechanism failed to reach something BY CONSTRUCTION — no amount of
correct execution would have caught it.

**Evidence:** ALL 4 cases —

| chunk | what | impact |
|---|---|---|
| a11y-ci-gate | Everything the harness does BEFORE wdio — frontend build, cargo build — printed only to a console a detached launch discards, so a leg failing in that window produced NO visible output anywhere, in CI as much as locally; three CI runs and two local legs passed through it blind | extra_reads 2 |
| a11y-ci-gate | One defect with three surfaces, all previously recorded as separate items: orphaned processes survive every leg because stopping a driver does not kill the browser hosts it launched | retries 1 |
| a11y-ci-gate | Three duplicate-occurrence sites the fan-out did not propose were found only by the cascade's own sweep — the fan-out agents each see one master and cannot see a sibling restating a retired claim | extra_reads 3 |
| keyboard-and-focus-order | The plan listed `npm run knip` with `expect exit 0`, unsatisfiable at HEAD (12 pre-existing unused exports); P5's check 4(9) could not catch it BY CONSTRUCTION — its novelty check asks whether a command was *named* before, and named is not green | iterations 1 |

**Proposal:** the fourth case names a precise, cheap fix and is the one most likely to recur: P5's
novelty check treats "this command was named in a prior plan" as evidence the expectation is
satisfiable. A plan can therefore ship an `expect` the tree has never satisfied. Having the check
compare against the command's last recorded *outcome* rather than its mere prior appearance would
close it. The third case (fan-out agents structurally unable to see a sibling master) is the
recurrence of a known Tier-1 class and may be worth a standing cascade sweep rather than a
per-chunk hope.

### P7 — `implement/code` · `input.plan-step-ambiguous` — 3 cases · weight 3 · rate 0.75/run

**Pattern:** a plan step named a target that did not match the code — already satisfied, wrongly
enumerated, or bundling two subjects a single row cannot own.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| a11y-ci-gate | Plan step 5 named the session-isolation census as the site needing the browser-host image name, but that site already carries it at HEAD; the omission the step aimed at survives in a different file the plan does not list | extra_reads 3 |
| medium-integrity | Plan step 6 enumerated the stale scheduled-task naming as two comments plus one prefix; the witness's entire synopsis block and four body comment blocks were also scheduled-task prose describing a mechanism that never shipped | extra_reads 1 |
| keyboard-and-focus-order | Step 1 prescribes "one row per a11y-plan §5 keyboard/focus claim", but §5's run-console-idle bullet bundles TWO SC claims asserted by two DIFFERENT specs — a bullet-keyed row could name only one owner | extra_reads 0 |

One further case sits at `implement/fix-loop` (n=1, in the appendix): the continuation of the first
row above, settling the census target with an address and an owner. Its `what` carries a clause
retraction — `[clause retracted: discounts the clause "BOTH its call sites are screen-reader-gated"
— census() has THREE call sites, not two: :240 inside stopNvda, plus :396 and :450. The record's
TYPE and its ownership conclusion stand unchanged, since all three are SR-territory]`.

**Proposal:** all three are the same failure — the plan's step names a SITE, and the site was
established by reading rather than by a command whose output the step could carry. Requiring a
step that names an edit target to carry the enumeration that found it (the bare command and its
hit count) would make "already satisfied" and "incompletely enumerated" visible at validate rather
than at code. This correlates with the L3 theme below, which measures the same cause from the
deviation scan.

---

## Cross-step chains (starting heuristics)

### X1 — `implement/fix-loop` →`implement-outcome`→ `wrap-session/report` — 2 chunks (6 hypothesis rows)
### X2 — `implement/smoke` →`implement-outcome`→ `wrap-session/report` — 2 chunks (2 rows)

These are one family and are reported together. In both chunks the producer (`implement`) closed
formally `ok` with `signals: ["green"]` / `["surfaced"]`, and `wrap-session/report` then consumed
`implement-outcome` with quality **`thin`** — and in every case the consumer's note says the same
thing:

- `a11y-ci-gate` — *"SUPERSEDED rather than wrong: implement reported surfaced with the terminal
  open, and four CI probes plus three operator/overseer-run legs ran AFTER it, establishing the
  cause it could not reach."*
- `medium-integrity` — *"implement's P4 report was SUPERSEDED by later steps in the same session:
  after it, three operator reading-notes corrected three of my readings and five further CI probes
  ran, culminating in run 351928…"*

**Chain hypothesis:** for a chunk whose evidence is a CI run, `implement` structurally cannot be
the last word — the decisive runs happen after it closes, so its report is stale by the time wrap
reads it. The producer is not defective; the SEAM is mis-timed. Note the consumer marks this
`thin` rather than `wrong`, and says so explicitly — the capture is behaving well; it is the
pipeline shape the records are pointing at.

**Proposal direction:** a CI-gated chunk may warrant an implement outcome that says "terminal open,
awaiting run N" as a first-class value rather than `green`/`surfaced`, so wrap reads a state rather
than a superseded conclusion. Whether that is worth a vocabulary change is the founder's call.

Below the ≥2-chunk threshold (all single-chunk, listed in the appendix): `phase/plan`→plan→
`implement/fix-loop` (3 rows, 1 chunk) · `phase/validate`→plan→`implement/fix-loop` (3 rows, 1
chunk) · `phase/take-up`→scope→`phase/validate` · `phase/research`→scope→`phase/validate` ·
`phase/research`→research→`phase/plan` · two plan→`wrap-session/report` rows.

---

## Level candidates (systemic-masked-as-project)

Problem-fact pile: **22 facts** — `workaround` 11 · `removed-cause` 7 · `deferred` 3 ·
`prohibition` 1; nature `process` 15 · `environment` 6 · `product-logic` 1.

### L1 — band-aid — the Bash tool's MSYS shell vs POSIX-shaped probes — 6 facts

**Facts:**

| chunk · step | nature · solution | note |
|---|---|---|
| — · `new-session/orientation` | environment · workaround | `df -h D:` printed nothing on this MSYS host; re-ran as `df -h .` |
| — · `new-session/orientation` | environment · workaround | `bc` is not installed, so the Check 4 KB conversion printed 0.0 KB for all seven rule files plus an empty total — a readable but wholly wrong table rather than a failure; recomputed in python |
| — · `new-session/orientation` | environment · workaround | `bc` absent — the Check 4 sizing printed 0.0 KB for every file; re-ran in python (**a second, separate session**) |
| — · `new-session/orientation` | environment · workaround | the Check 11 currency probe passed the template dir as an MSYS path to a Windows-native python `open()` and got FileNotFoundError; re-ran in the native form. *Class already documented in the project's host rule file, 2026-09-08 clause (a)* |
| — · `new-session/orientation` | environment · workaround | the master-route probe chained a zero-match `grep -cE … pending` with `&&`, which exited 1 and aborted the rest of the chain; re-run with `;` separators *per the project's host rule file's zero-is-healthy-count clause* |
| `medium-integrity` · `phase/research` | process · removed-cause | a `cd` inside one Bash call persisted into the next, *the hazard the project's host rule file documents at 2026-09-08*; re-anchored absolutely |

**Level hypothesis:** the cause lives in the PIPELINE — `new-session`'s health-check probes are
authored as POSIX recipes — and every fix landed in the PROJECT, either as a per-run adaptation or
as a clause in `.claude/rules/host-win32.md`. Three of the six notes explicitly cite that project
rule file as already documenting the class they just hit. The knowledge exists; it sits one tier
below the thing that keeps tripping. Two sharper details: 5 of 6 are in one step
(`new-session/orientation`), and the `bc` failure occurred in **two separate sessions in this
epoch** plus at least once in 0.3.0 Epoch 2 — the same probe producing the same wrong-but-readable
answer three times, which no threshold in the mechanism would ever surface because each session
records it once.

**Proposal:** the `bc` case is the one to decide first, because its failure mode is silent: the
probe does not error, it reports 0.0 KB and an empty total, and a dashboard consumer reads that as
a measurement. Computing sizes in python (already every project's fallback) rather than shell
arithmetic would close it for every project at once. More generally: where a health check's recipe
is POSIX-shaped, the per-project host rule file is currently doing the work of making it portable,
one measured failure at a time.

### L2 — band-aid — the gate tool's env predicate refuses valid entries — 2 facts in-epoch + 1 typed correlate, recurring

**Facts:**

| chunk · step | nature · solution | note |
|---|---|---|
| `a11y-ci-gate` · `implement/fix-loop` | process · workaround | the gate tool refuses to void a defer by contract, so `--only 5` reported `not run — defer (key)` and the workspace unit gate was run by hand (bare command, exit read before any pipe): 986 tests run, 986 passed |
| `medium-integrity` · `phase/validate` | process · workaround | a `role=lint` PowerShell parse entry could never fire — the gate env predicate reads every `$NAME` in a `run` as a required shell handle, so inline pwsh referencing its own variables was refused as "env f, null unset". Routed around by withdrawing the entry |

Typed correlate (P5, `contract.mechanical-check`): *"the gate tool's env predicate treats every
`$NAME` in a run string as a shell handle that must be set, so ANY inline PowerShell/awk/jq code
carrying its own `$variables` is unfireable."* **Recurrence:** the same predicate appears in 0.2.0
Epoch 6a (`2026-09-02-mutation-tier-restored-for-conductor`), which meets the F-2 recurring
threshold (n ≥ 2 in-epoch, recurring from a prior epoch).

**Level hypothesis:** a pipeline tool's input predicate cannot distinguish a shell handle the
runner must supply from a variable the command defines for itself. The fixes landed in the project
each time — withdraw the entry, run the gate by hand — which is exactly the shape that leaves the
tool untouched and the next chunk paying again.

**Proposal:** the predicate could scope itself to variables the run string does not itself bind, or
offer an explicit escape for entries carrying their own variables. Worth noting the workaround has a
cost beyond the inconvenience: one of the two facts records a gate being **run by hand instead of
through the tool**, which moves that gate outside whatever the tool records.

### L3 — band-aid — plan steps whose named target does not match the code — 3 facts

**Facts:**

| chunk · step | nature · solution | note |
|---|---|---|
| `a11y-ci-gate` · `implement/code` | process · deferred | plan step 5 asked to extend the census; that location ALREADY covers it at HEAD, so no edit was made there — the live remnant is elsewhere |
| `a11y-routine-arm-terminal` · `implement/code` | process · workaround | deviated from plan step 7's clause directing removal of the token-witness readback — kept it, because the witness survives as the leg's entry point |
| `keyboard-and-focus-order` · `implement/fix-loop` | process · removed-cause | plan step 3 directed adding the checker to `knip.json`'s entry array; knip itself reported that pattern REDUNDANT, because it already reaches the checker through the package.json script the same step adds. Reverted the edit |

**Level hypothesis:** the cause is at `phase/plan` (a step naming a target established by reading
rather than by measurement) and every fix landed at `implement` — deviate, defer, revert. The typed
correlate is P7 (`input.plan-step-ambiguous`, n=3 at `implement/code`), so this theme and that
group are two views of one cause: 6 records in total across the two instruments.

**Proposal:** see P7. The distinctive contribution of the deviation-scan view is that it shows the
*cost location* — implement absorbed all three, and in two of them the implementer had to overrule
a written instruction to do the right thing.

### Signatures checked and NOT fired

- **Chronic-degrade — does not fire.** Zero `ok-degraded` outcomes in the epoch (55/55 `ok`), and
  the trend across epochs is downward to zero. Recorded because a null result here is evidence.
- **Deferred-forever — does not fire.** All 3 `deferred` facts name their destination in their own
  notes (two routed to P4, one naming the live remnant's address and owner), and the census
  deferral's closure appears in a later in-epoch record. Per the mechanics reference, a note that
  names its destination is a routed deferral, not an orphan.
- **Override — does not fire.** Zero `solution: overridden` facts in the epoch.
- **Recurring `removed-cause` theme — does not fire.** All 7 `removed-cause` facts remove
  *different* causes; no single cause was removed repeatedly.

---

## Playbook-extension candidates (untyped patterns, F-4)

**None reach the F-4 threshold.** The epoch's untyped rate is 2 of 73 (2.7 %) and the two records
share no subject, so no cluster forms. Both are listed in the appendix below. One observation
rather than a proposal: the first untyped record (a probe emitting a plausible wrong answer instead
of an error) appears to fit the existing Universal type `tooling.host-shell` — if so, that is a
type-SELECTION gap rather than a missing type, and it belongs to L1's theme.

---

## Below threshold — no action

**Typed groups (18 of 25 below threshold).** `recall.corpus-recurrence` n=2 · `tooling.host-shell`
n=2 (both chunkless, `new-session`) · `implement/fix-loop`/`tooling.gate-deferral` n=2 ·
`contract.skill-reference-drift` n=1 · `implement/fix-loop`/`tooling.environmental` n=1 ·
`phase/plan`/`retry.synthesis-rework` n=1 · `phase/plan`/`input.research-thin` n=1 ·
`wrap-session/gates`/`tooling.light-gate-red` n=1 · `phase/take-up`/`input.out-of-pipeline-source`
n=1 · `phase/research`/`input.out-of-pipeline-source` n=1 ·
`implement/fix-loop`/`contract.spec-reality-gap` n=1 ·
`implement/fix-loop`/`input.plan-step-ambiguous` n=1 (the clause-retracted record; see P7) ·
`implement/fix-loop`/`tooling.result-not-run-stable` n=1 ·
`wrap-session/report`/`contract.detector-fact-gap` n=1 ·
`phase/take-up`/`input.carry-context-gap` n=1 · `wrap-session/gates`/`tooling.subprocess-bounds`
n=1 · `wrap-session/reconcile`/`contract.cascade-miss` n=1 ·
`wrap-session/route-resolve`/`contract.standing-pin-carriage` n=1.

Worth an eye though sub-threshold: `tooling.host-shell` n=2 and `recall.corpus-recurrence` n=2 both
reinforce L1 and the P2/P4 recurrence note respectively.

**Untyped clusters (2, both n=1).**
- `new-session/orientation` — `bc` absent, so a shell-arithmetic KB conversion printed 0.0 KB for
  all seven rule files and an empty total; recomputed in python. *(Belongs to L1; may fit the
  existing `tooling.host-shell`.)*
- `keyboard-and-focus-order` · `phase/take-up` — v3-03's acceptance turns on the text of a spec
  master that neither phase nor implement may write (spec amendments are wrap's flow), so the
  claim-reachability rule may make the cap unclaimable by this chunk as written; recorded as an
  open fork carried to P4 rather than settled at take-up. **Watch next epoch** — this is a
  cross-skill authority question, not a local one, and the epoch closed with v3-03 still pooled.

**Chain shapes below the ≥2-chunk threshold.** `phase/plan`→plan→`implement/fix-loop` (3 rows, 1
chunk) · `phase/validate`→plan→`implement/fix-loop` (3 rows, 1 chunk) ·
`phase/take-up`→scope→`phase/validate` (1) · `phase/research`→scope→`phase/validate` (1) ·
`phase/plan`→plan→`wrap-session/report` (1) · `phase/validate`→plan→`wrap-session/report` (1) ·
`phase/research`→research→`phase/plan` (1).

**Problem-fact themes below threshold.** Edit-tool unreliability forcing a scripted
read-modify-write — 2 facts in-epoch (a `replace_all` silently dropping a trailing space across 22
lines; a scripted RMW preferred for a byte-identical repeated phrase). A keyword sweep finds
mentions across prior epochs, but that sweep is a proxy and was not confirmed record-by-record, so
this is recorded as sub-threshold rather than as a recurring theme. · Escalations needing operator
rulings that did not arrive, resolved by removing the code — 1 fact. · Host-path hygiene in
run-dir extracts — 1 fact (a distiller returned an absolute host path in a Patterns bullet;
persisted repo-relative).

---

*Evidence twins in this run dir: `q-retractions.json` · `q-health.json` · `q-typed.json` ·
`q-untyped.json` · `q-chains.json` · `q-level.json`.*
