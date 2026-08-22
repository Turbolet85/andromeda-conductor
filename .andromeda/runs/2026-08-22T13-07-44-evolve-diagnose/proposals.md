# Evolve Diagnosis — Conductor · Epoch 4 "Lifecycle & delegated timing" · 2026-08-22T13:07:44Z

Target epoch chosen in dialogue (latest fully-complete; the 2026-08-20 run targeted Epoch 3 and
named Epoch 4 only in passing). Everything below is **obligation-free**: accept, reject, defer, or
modify any item with no mechanism-side consequence. Nothing here has been applied, queued, or
remembered — a re-run recomputes from the ledger alone.

**Chunk legend** — `VSH` 2026-08-20-verifier-self-hardening · `RSC` 2026-08-20-read-back-seam-survivors-closed ·
`SLL` 2026-08-21-severity-lifecycle-live-proof · `PCL` 2026-08-21-per-check-latency-measurement ·
`DTB` 2026-08-21-delegated-timing-budgets-proven · `OPC` 2026-08-22-operator-pause-and-checklist-live-firing ·
`(—)` chunkless (a new-session start, or a 0-pending wrap).

---

## Mechanism health

| | |
|---|---|
| Records | **176** (90 step / 86 friction) across **6 chunks** + 8 session starts |
| Unparseable lines | **0** (whole ledger: 927 records, 0 unparseable) |
| Step-record coverage | 5 of 6 chunks fired all 13 checkpoints. **`OPC` is missing `implement/code`** — 12 records, the only checkpoint gap in the epoch |
| Chunkless step records | 13 — 8 `new-session/orientation` (by design) + 5 from **two 0-pending wraps** (`route-resolve` ×2, `curation` ×2, `gates` ×1). Wrap totals therefore read 35 against a nominal 30 |
| Untyped rate | **10 / 86 = 11.6%** — concentrated at `wrap-session/route-resolve` (3/6), `wrap-session/gates` (2/4), `wrap-session/curation` (2/9). Down from Epoch 3's 19.0% |
| Problem-fact fill | **36 / 90** step records carry ≥1 fact — **38 facts** (23 workaround · 6 deferred · 6 removed-cause · 2 overridden · 1 unresolved); natures process 18 · environment 18 · resources 2 |
| Outcomes | ok 78 · halted-resolved 6 · **ok-degraded 4** · **soft-exit 2** · aborted 0 |
| id fill | **176 / 176 (100%)** |
| Retractions | 1 honored (`2026-08-20T21:14:37Z-a`, problem-scope, index absent → all facts dropped) · 1 unresolvable |

**Calibration boundaries.** Every Epoch-4 record post-dates all four boundaries (deviation scan,
`graph-not-applicable`, Universal types, `id`+schema'd `retracts`), so nothing inside the target
range needs era-reading. The prior-epoch lookbacks used below (Stages 2 and 4) **do** cross them:
Epoch-1/2 records legitimately carry no `id`, and Universal-type absence there is era, not evidence.

**Unresolvable retraction (reported verbatim, for manual discount).** One pre-boundary prose-form
retraction, `id: null`, at `phase/plan` in **Epoch 1**: *"retracts: the untyped
code-graph-under-reports record and the first problem-block entry on the plan step record, both
appended earlier at this step."* Its `what` establishes the correction — the code-graph did **not**
under-report; `rows=1` came from piping a 41-row result through `head -30`. It is outside this
epoch and does not touch any evidence below, but it bears on L2: **one prior claim of graph
under-reporting is retracted**, and L2's Epoch-1 lookback rows are the *unretracted* remainder.

**One granularity limit observed in the retraction schema.** `2026-08-20T21:40:24Z-a` retracts a
problem block whose note explicitly says *"HALF superseded, not false when written… discount the
step-7 half"*. The target step record carries that obstacle as a **single** fact covering both
step 3 and step 7, and `scope:"problem"` with no `index` can only drop the whole fact. The author's
intent was finer than the schema's grain. Observation only — the honored exclusion is correct per
the contract.

---

## Proposals (typed patterns)

### P1 — `tooling.host-shell` (Universal, grouped by type alone) — 13 cases · weight 27 · 8 steps · 5 chunks + 3 chunkless runs

**Pattern:** the host shell answers a *different question* than the one asked, and the wrong answer
looks successful. Two distinct sub-mechanisms, both present every epoch measured. **(a) The write
transport corrupts or refuses a structured payload** (5 cases): a quoted heredoc carrying ~105–110
lines of Markdown aborts at parse time with `unexpected EOF`, and `printf '%s'` / a quoted heredoc
collapses doubled backslashes into invalid JSON or Python escapes. **(b) A probe's exit status or
match result is not the thing the author meant to read** (8 cases): `$?` after a pipe reports the
last stage, an unset variable expands to a root-absolute redirect, an empty substitution makes
`grep -qF` match unconditionally, `ls A B` fails on the *second* path, unquoted BRE escapes return a
false negative, and `str.replace` has no failure mode at all. Sub-mechanism (b) produced **three
confidently inverted verdicts** — a failing test reported as exit 0, a vacuous all-pass on five
survivors that tested nothing, and a green gate reading as exit 1.

Case (7) below is itself the capture side naming the family: *"Third distinct shell-probe misfire
this session… all the same family: a shell default turning a partially-true probe into a
wrong-but-successful-looking answer."*

**Subsumes** two per-step groups that clear the threshold on their own and are counted once here:
`implement/fix-loop/tooling.host-shell` (n=4, w=9) and `phase/take-up/tooling.host-shell` (n=3, w=6).

**Evidence:** ALL 13 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| (—) | wrap/route-resolve | python stdout defaulted to cp1252; printing the applied route back raised `UnicodeEncodeError` on U+2193 and truncated the check mid-loop — the utf-8 write itself was fine | — | `2026-08-20T20:20:57Z-c` |
| VSH | phase/take-up | quoted bash heredoc writing `scope.md` failed `unexpected EOF … matching single quote` despite a quoted delimiter; inline code spans / escaped pipes / apostrophes tripped the parser, nothing partial written | retries 1 | `2026-08-20T20:38:28Z-c` |
| VSH | implement/fix-loop | the premise-closing measurement ran as `cargo test … \| tail -40`, so the reported exit was tail's (0) — **a run whose test actually FAILED announced exit 0** | — | `2026-08-20T20:53:20Z-c` |
| RSC | phase/take-up | a `<<QUOTED-EOF` heredoc carrying ~105 lines of Markdown aborted at parse time and wrote nothing; the terminator was not recognised so the body consumed the rest of the command list. Same-shell python heredocs parsed fine before and after | retries 1 | `2026-08-20T22:24:00Z-b` |
| RSC | implement/fix-loop | the mutation gate redirected to an **unset** variable; with no `set -u` the redirect became `/mutants.log` → Permission denied, so cargo-mutants never started while the background task reported a normal completion | retries 1 | `2026-08-20T22:58:46Z-b` |
| RSC | implement/fix-loop | the five-survivor check built its grep pattern through a command substitution whose `sed` failed; the substitution yielded EMPTY, so `grep -qF` matched every line and printed CAUGHT for all five **having tested nothing** — a vacuous all-pass | retries 1 | `2026-08-20T22:58:46Z-c` |
| RSC | wrap/report | `ls A B \|\| echo not-ready` printed not-ready though A existed, because `ls` exits non-zero on the **missing second** path | — | `2026-08-20T23:07:08Z-b` |
| PCL | wrap/curation | the prescribed `printf '%s'` append collapsed doubled backslashes, so a quoted regex became invalid JSON escapes and the line did not parse; re-appended through a UTF-8 `json.dumps` writer | retries 1 | `2026-08-21T10:11:12Z-c` |
| PCL | phase/take-up | a `cat` heredoc with a single-quoted delimiter carrying ~110 lines of Markdown failed at bash parse time; the compound command aborted before its leading `mkdir` ran | retries 1 | `2026-08-21T10:19:56Z-b` |
| PCL | implement/fix-loop | a scratchpad python `.replace` pass silently **NO-MATCHED twice** on multi-line call sites, reporting success while leaving the edits unmade; caught only by the next `cargo check` | retries 1 | `2026-08-21T10:57:18Z-f` |
| PCL | phase/research | a clippy probe captured **grep's** exit status (`\| grep -cE …; echo $?` reports 1 when grep finds zero matches — the healthy outcome), so a green gate momentarily read as exit 1 | — | `2026-08-21T11:26:10Z-b` |
| DTB | implement/fix-loop | the `cargo audit` PREREQ signature turns on exit **exactly 1**; the probe ran piped into `head`, so `$?` was head's (0) — a value reading as a DEVIATION that would have needlessly restored the full re-check form | retries 1 | `2026-08-21T18:20:31Z-b` |
| OPC | wrap/curation | escape semantics bit **twice in one step, in opposite directions**, on the same Windows path literal: writing collapsed the doubled backslashes (invalid Python escape); verifying passed the read-back pattern unquoted-for-BRE so grep read `\W \S \b` as classes and returned **zero matches against a file that did contain the string** — a false negative that would have read as a failed write | — | `2026-08-22T12:39:40Z-b` |

**Proposal:** two directions, separable.
(a) For the **write** half — the references already prescribe the remedy in one place
(`codebase-research.md`: inline Python becomes a scratchpad file run by path) but not in the others.
Consider making "structured payload ⇒ dedicated write tool or a scratchpad file invoked by path"
the *prescribed* idiom across `scope.md` / `research.md` / `plan.md` / `report.md` / curation
appends, rather than the documented fallback from a heredoc that has now failed at parse time in
every epoch. See **L1** — the reference text is currently the *cause*, not the cure.
(b) For the **probe** half — a short "shell probes that lie" checklist in the shared reference
(capture `$?` into a variable **before** any pipe; `set -u` or literal paths for redirects; guard
command-substituted patterns against empty; `grep -F` for literal patterns; treat `str.replace` /
`sed` as having no failure mode and assert after) would address the class the capture side has
already named for itself. The Tier-1 session-learnings corpus states parts of this and did not
prevent recurrence (see **L5**), so the placement question — reference vs. project corpus — is the
substantive part of this proposal, not the content.

---

### P2 — `contract.narrow-basis-claim` (Universal, grouped by type alone) — 6 cases · weight 15 · 6 steps · 4 chunks

**Pattern:** a count or absence claim inherits the narrowness of the view or pattern that produced
it, and returns a plausible wrong number rather than an error. Every one of the six was caught only
by an **independent re-derivation** — never by the check that produced it. Two are the *same claim*
under-counted in consecutive sessions by two different mechanisms.

**Evidence:** ALL 6 cases —

| chunk | step | what | impact | evidence |
|---|---|---|---|---|
| (—) | new-session/orientation | coverage computed against a **guessed matrix schema path** returned verified=0 of 32; caught only because the handoff independently stated 17/32 | reformulations 1 | `2026-08-20T17:37:44Z-b` |
| VSH | phase/validate | two claims the code-audit makes about **its own artifacts** failed re-derivation: §B1 says "all 28 survivors" and enumerates 27; §B3 says the `--test-tool=nextest` re-run is recorded in the JSON `command` fields, where no such flag appears — replaying the recorded form reproduces the abort, not the scores | dialogue 2, extra_reads 2 | `2026-08-20T20:59:01Z-b` |
| SLL | phase/research | a caller-count (`rows: 10`) authored into research.md from a **tail-truncated printout** instead of the trace `rows` field; re-derivation read 7 | extra_reads 1 | `2026-08-21T09:12:00Z-b` |
| DTB | phase/take-up | stated the three Pulse frontend timing procedures had **no production callers**, from a grep whose include-set surfaced one file; a broader search found real call sites reached through a camelCase TS wrapper the first pattern could not match. **The absence claim would have inverted the chunk premise had it reached scope** | extra_reads 1 | `2026-08-21T17:48:55Z-b` |
| OPC | wrap/reconcile | the Expected-amendments site enumeration was too narrow **twice, by two mechanisms**: a pattern requiring the literal `pretty-print` missed "pretty in dev" (operator corrected 3, a case-insensitive sweep then showed 6 — two returned hits had been **dropped without disposition**); and the stdout axis offered 6 where the sweep found 7 | extra_reads 3 | `2026-08-22T10:46:19Z-b` |
| OPC | wrap/gates | **third under-count of the same enumeration**, and the first caught by something other than a human: the plan's 17-site floor was built from two case-insensitive patterns; the semantic obs detector found 21, because four sites word the premise as "stderr (dev only)" — a third wording containing **neither** pattern | extra_reads 1 | `2026-08-22T12:35:53Z-b` |

**Recurrence:** Epoch 3 carried 7 of this type, Epoch 4 carries 6 — flat across two epochs.

**Proposal:** the epoch's own last case states the generalization the capture side reached — *"a
pattern-built count is bounded by the author's imagination of how a claim can be phrased"* — and
also demonstrates the working countermeasure: a **semantic reader beat the pattern floor**. Two
directions worth the founder's judgment. First, where a step must produce a site count over prose,
consider prescribing the count as an explicit **floor to be exceeded by a semantic pass**, rather
than as the answer — this is what actually worked at `OPC`, twice. Second, case 5 names a smaller,
sharper rule that needs no new machinery: **every hit a pattern returns needs an explicit
disposition, including "no change, and here is why"** — two of that enumeration's misses were hits
that *were* returned and silently filtered. (Both were curated into CLAUDE.md as a Tier-1 extension
at the `OPC` wrap; the remaining proposal is whether the mechanism should carry it as a step
criterion rather than as project memory — see **L5** for why that distinction has teeth.)

---

### P3 — `new-session/orientation` · the named contract is not read, the schema is probed — 4 records · 3 sessions

**Pattern:** the orientation body names `references/verification-matrix-contract.md` (and, for the
route, the working-route's own documented grammar) as the **basis** for a read, and the artifact was
opened and its shape inferred instead. Every occurrence returned a **plausible wrong value that
maps onto a valid dashboard state rather than erroring** — `verified=0 of 32`, and an empty
markerless tail that reads as ladder rung 5 (*version complete*). The `contract.schema-assumed`
type exists in the orientation playbook for exactly this act, fired as designed, and the act
recurred in the next two sessions.

**Evidence:** ALL 4 records — two typed frictions and two problem-facts, across 3 distinct sessions.
(The first also appears in P2's table; it is the same event seen through the universal grouping.)

| session | kind | what | impact | evidence |
|---|---|---|---|---|
| 2026-08-20 | friction `contract.narrow-basis-claim` | coverage computed against a **guessed** matrix schema path → verified=0 of 32; caught only by the handoff's independent 17/32 | reformulations 1 | `2026-08-20T17:37:44Z-b` |
| 2026-08-20 | step problem-fact (process/workaround) | *"Phase 1 step 5 names references/verification-matrix-contract.md as the basis for the coverage read; read verification-matrix.json directly and inferred its schema instead, so `status` (nested under `verification`) was looked up at top level"* | — | `2026-08-20T17:37:44Z-a` |
| 2026-08-21 | step problem-fact (process/workaround) | *"verification-matrix-contract.md was not read; the matrix container key was inferred with a fallback chain (capabilities\|entries\|root) … against the body constraint 'read the contract, never probe the schema'"* — the figures happened to cross-check, so the probe cost no redo cycle | — | `2026-08-21T08:04:09Z-a` |
| 2026-08-21 | friction `contract.schema-assumed` | the markerless-tail scan **guessed a list-item line format** instead of reading the grammar the working-route header documents; returned an EMPTY set — a plausible wrong value mapping onto rung 5 | extra_reads 1, reformulations 1 | `2026-08-21T10:11:12Z-b` |

**Proposal:** the constraint is already written and already typed; what recurs is the *reach* for the
JSON before the contract. A cheap direction: inline the two field paths the orientation actually
needs — `capabilities[]`, `verification.status`, top-level `chunk` — directly into the Phase 1
step-5 body, so satisfying the step requires no schema decision at all. The contract reference stays
authoritative for everything else. (Note the third record's honest detail: the probe *agreed* with
the handoff that session, so the guard cannot rely on the wrong value being noticed.)

---

### P4 — `phase/distill` · `contract.extract-format` — 4 cases · weight 5 · 4 chunks

**Pattern:** three of the four are one defect: **the subagent return transport HTML-escapes `<`, `>`
and `&`** in most of the 7 returned extracts, mangling exactly the load-bearing tokens the chunks
turn on — the closed `<5s`/`<20s`/`<90s` tier set, `Option<FaultSpec>`, `(&field, &())`,
`<run_id>`, `conductor-<seam>`. It **passes all five named per-extract checks** (stripping is clean;
no check tests for entity escaping), so it is caught only by reading the returned text. Two of the
three records explicitly call themselves a recurrence of an already-written session learning. The
fourth case is unrelated: a Constraints-anchor form judged compliant in substance.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| VSH | the tests extract cited its Constraints anchors inline rather than in the trailing `(per {plan} §…)` form the format note shows; judged a pass in substance rather than a re-spawn, since every bullet names an identifiable anchor | — | `2026-08-20T20:48:19Z-c` |
| PCL | **all 7** extracts arrived with angle brackets HTML-escaped, corrupting the closed tier set the chunk is about plus generic types and path templates; un-escaped at strip time, and **none of the 5 named per-extract checks tests for it** | iterations 1 | `2026-08-21T10:27:18Z-b` |
| DTB | the transport escaped `< > &` in **5 of 7**; stripping was clean in all 7, so this **sat outside the five named checks and passed every one**. *"This is a REPEAT of the same defect already recorded in CLAUDE.md session learnings for a prior 7-distiller run"* | — | `2026-08-21T17:57:38Z-b` |
| OPC | the transport escaped `< > &` in **5 of 7** (arch and obs unaffected only because they happened to contain no such characters). *"Recurrence of the failure already recorded as a session learning on 2026-08-21"* | — | `2026-08-22T09:46:47Z-b` |

**Recurrence:** Epoch 1 = 1, Epoch 3 = 1, **Epoch 4 = 4** — the only type in the epoch with a rising
trend, and the rise is entirely the escaping defect.

**Proposal:** the repair is already uniform and mechanical (a single entity-decode on save, applied
in all three instances). The gap is that it is re-discovered by reading, every time, because the
per-extract check list does not include it. Consider adding a **sixth per-extract check** —
"contains no HTML entity escapes (`&lt;` / `&gt;` / `&amp;`)" — with the decode as its prescribed
remedy, in the distill fan-out reference. That converts three consecutive manual catches into a
check the step already runs. Related: `DTB`'s problem-fact records that the escaping also broke the
raw-twin convention (a re-encoded file would be "a reconstruction wearing the name of a raw record")
— see **L3**.

---

### P5 — `implement/smoke` · `tooling.headless-skip` — 2 cases · weight 5 · soft-exit impact · 2 chunks

**Pattern:** the chunk's deliverable is gated on an observable that **no automated step on this host
can reach** — a running Pulse GUI window, an operator opening a Report, a DISPLAY plus `tauri-driver`
— so implement can reach green gates and still be structurally unable to reach the chunk's
acceptance. Meets threshold at n=2 on soft-exit impact.

**Evidence:** ALL 2 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| DTB | three of four observables fire only from a running Pulse **webview** (useEffect on dots/items, a 1000 ms findings re-poll) and the fourth only on `incidents.get_report` — i.e. only when a human opens a Report. *"An implement run on an unattended host can therefore reach green gates but can never reach the chunk acceptance, no matter how complete the code is"* | soft_exit 1 | `2026-08-21T18:22:11Z-b` |
| OPC | lands a webview surface on a host with no DISPLAY and `tauri-driver` absent from PATH **despite `@crabnebula/tauri-driver` sitting in package.json devDependencies**; the Rust-side contract is fully unit-pinned but nothing proves the dialog renders the rows | deferred 1 | `2026-08-22T11:03:12Z-b` |

**Proposal:** this is the *detection* half of **L6**; the deferral half is there. The narrow
proposal here is about **when** it is detected: both chunks discovered un-reachability at
`implement/smoke`, after code and gates were complete — `DTB` then spent a soft-exit and an
unsanctioned wrap-time reorder to recover (see the appendix's highest-impact untyped record).
Consider whether `phase/plan` (or `phase/validate`) should carry an explicit **"can this host reach
the acceptance?"** determination when the acceptance names a GUI/attended observable, so the
attended leg is scheduled as part of the plan rather than discovered as a smoke-time blocker. Note
the second case shows the cheap proxy is insufficient: the devDependency **is** declared, and the
binary is still absent from PATH.

---

### P6 — `wrap-session/curation` · `ambiguity.filter-borderline` — 3 cases · weight 3 · 3 sessions

**Pattern:** the curation filters produce edge calls that the numeric criteria do not settle, and a
tie-breaker decides. Two of the three turn on the **same clause** — Filter 1's dedup-reject-only
against text a *different channel* wrote, in one case text **this same wrap's cascade had written
minutes earlier**.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| VSH | two candidates on a filter edge: one scored **exactly 0.6** at Filter 4 (one signal either way flips it); the other was an arguable Filter-1 additive-facet call, resolved as duplicate on the ground that *"the existing rule is what CAUGHT the drift, so the rule working as written is not evidence it is incomplete"* | — | `2026-08-20T20:23:09Z-b` |
| (—) | three of four rejections turned on Filter 1 dedup-reject-only **against a GENERATED body this same wrap had written minutes earlier** (the cascade put three learnings into `rules/testing.md`); real learnings that arrived already-recorded by a different channel — *"the clause exists for exactly this and fired three times in one pass"* | — | `2026-08-20T22:09:15Z-b` |
| (—) | Filter 1 routed a candidate to in-place **extension** on conceptual-family grounds while its measured token overlap was almost certainly under the 0.7 bar — the vocabularies differ though the failure class is the same; **the additive-facet tie-breaker decided it, not the similarity metric** | — | `2026-08-21T17:19:28Z-b` |

**Proposal:** all three resolved correctly and none cost a redo cycle, so this is a calibration
signal rather than a defect. Two observations for the founder: the **0.7 similarity bar is not what
decides** the family-level cases (the additive-facet tie-breaker is, twice), and the
cascade-versus-curation ordering means a wrap regularly deduplicates its own fresh writes. If either
is intended, no change; if not, the direction would be to state the tie-breaker's precedence over
the metric explicitly, and to say whether same-run cascade output counts as an existing entry.
(A related untyped record, appendix `U-b3`, is the same ordering seen from the other side.)

---

### P7 — `wrap-session/curation` · `ambiguity.tier-routing` — 3 cases · weight 3 · 3 sessions

**Pattern:** a surviving learning has **no correct home in the three-tier system**, and each
instance resolves differently — demoted, dropped as better-homed, or re-routed against the
operator's own nomination.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| VSH | the sole survivor is an imperative about route-tail edits, which **reads Tier 2** — but no rule file's `paths:` scope covers `.andromeda/` or the version dir, and minting `rules/route.md` for one convention was forced; **demoted to Tier 3** per the fallback chain | — | `2026-08-20T20:23:09Z-c` |
| (—) | a survivor (per-plane code-graph query mechanics) had **no correct home in any tier** — its real home is `scripts/code-graph-cookbook.md`, a setup-owned doc **outside the tier system** — so it was dropped as better-homed; *"the dedup filter reads the three tiers only and would not have caught the duplication"* | — | `2026-08-21T17:19:28Z-c` |
| OPC | the operator directive nominated Tier 3, but Filter 1 matched an existing **Tier-2** entry; the candidate added the payoff and mechanism that entry lacked, so the additive-facet tie-breaker applied — **routed to Tier 2 against the directive nomination**, which also lands it where it auto-loads on the harness paths | deferred 1 | `2026-08-22T12:39:40Z-c` |

**Proposal:** two structural gaps are named by the records themselves, both independent of any
individual call. (1) There is **no rule-file `paths:` scope covering `.andromeda/` or the version
dir**, so any learning about route/spec conventions cannot reach Tier 2 on merit and falls to Tier 3
by mechanics rather than by judgment. (2) **Setup-owned docs outside the tier system** (here
`code-graph-cookbook.md`) are real homes the dedup filter cannot see, so a learning can be silently
duplicated into a tier while its true home already states it. Directions: add a route/spec-scoped
rule file (or an explicit "no Tier-2 home exists for this class" branch in the fallback chain), and
extend the dedup read-set to the setup-owned docs that are legitimate destinations.

---

## Cross-step chains (starting heuristics)

### X1 — `phase/research` →`research`→ `implement/{code, fix-loop}` — 3 chunks

The strongest lineage signal in the epoch, and invisible to Stage 1: in **every** case the producer
is formally `ok`, carries the `unresolved-questions` signal, and the consumer records the artifact
`thin` for the **same specific reason** — the *Files-to-modify* / *Test companions* enumeration
omits companions the change class forces **mechanically**.

| chunk | producer | consumer verdict |
|---|---|---|
| VSH | `phase/research` outcome ok · signals `[unresolved-questions]` · own frictions `input.extract-signal-gap`, `tooling.host-shell` | `implement/code` — research **thin**: *"the Files-to-modify list was right for steps 1/2/4/5/6/8 but never checked VISIBILITY of the targets, which is what makes steps 3 and 7 unreachable; it also missed that conductor-verify lacked tokio test-util"* (also `implement/fix-loop` thin, same run) |
| SLL | `phase/research` outcome ok · signals `[unresolved-questions]` · own friction `contract.narrow-basis-claim` | `implement/code` — research **thin**: *"omitted crates/conductor-core/src/scenario.rs, which holds five catalog guard tests that DATA-PIN the retired checks — the codebase-research caller-threading note **names that class** but the list did not carry it"* |
| OPC | `phase/research` outcome ok · signals `[unresolved-questions]` · own friction untyped | `implement/fix-loop` — research **thin**: *"the Test companions list named the class but enumerated only 4 sites; adding a field to Scenario and HoldPoint forced 5 further crate-local fixture files research never listed"* |

**Corroborating problem-facts** (theme T6, sub-threshold on its own): `implement/code` @ SLL —
*"edited scenario.rs though research did not list it… retiring without moving them leaves an
incoherent tree"*; `implement/fix-loop` @ OPC — *"Edited 5 files outside the plan's Files-to-modify…
adding a struct field forces every literal by construction… a re-plan would produce the identical
list plus these."*

**Chain hypothesis:** the research reference **already names the class** (tests that data-pin a
changed artifact; caller-threading) — two records say so explicitly. What is missing is not the
concept but the *enumeration*: the class is stated qualitatively and then hand-listed, and the hand
list is short by exactly the mechanically-forced members. **Proposal direction:** for a change that
is mechanically forcing (a new struct field, a retired config key, a changed data shape), consider
requiring the companion set to be **derived** — a compiler/grep enumeration recorded as such —
rather than hand-listed under a named class. Note the `unresolved-questions` signal was present and
correct in all three; the producer flagged its own incompleteness and the flag did not prevent the
downstream thin verdict.

### X2 — `wrap-playbook` **thin** at `wrap-session/reconcile` — 3 chunks (no in-chunk producer)

Three chunks record the same consumed-verdict on the same artifact, each for a *different* missing
rule class:

- VSH — *"no rule covered adding a NEW anti-pattern ban from a measured defect, nor the recurring obs cli-row mis-attribution"*
- RSC — *"no rule covers a chunk measuring a tool's semantics"*
- OPC — *"no rule covered a NEW declarative KEY inside an already-registered scenario-config artifact — the nearest is scoped to new artifacts under `contracts/`"*

**Heuristic limit, reported honestly:** the Stage-3 lineage join found **no chain** here, because the
producer of `wrap-playbook` is an Andromeda reference, not an in-chunk step — the join only sees
`produced[]` artifacts within a chunk. The repeated-anchor shape is nonetheless a real cross-chunk
pattern the per-step grouping cannot see.

**Hypothesis:** the playbook grows one rule per encounter (the `OPC` wrap records its new rule as
the **4th of its class**), which means the rule-set trails the work at a steady rate rather than
converging. That may be exactly the intended design — a playbook that accretes from real cases. The
observation for the founder is only the **rate**: three of six chunks in one epoch hit a class the
playbook did not cover, each resolved by hand-authored judgment plus, in one case, operator
direction. If convergence is expected, this is evidence it has not begun; if accretion is the
design, no action.

### X3 — `tree-db` / `cookbook` **thin|wrong** at `phase/research` — 2 chunks

`PCL` — tree-db **thin** (fresh and authoritative for most symbols, but not all). `DTB` — tree-db
**wrong**: *"3 queries, plane=rust, db_state=fresh on all three; the impact query returned 0 callers
for `effective_deadline_ms` while a real production caller exists"*, plus cookbook **thin**: *"its
zero-rows protocol names two cases (confirmed-indexed = real finding, vs index gap) but not the
third one that actually occurred."* Feeds directly into **L2**; the cookbook verdict is the
actionable half — the protocol has a third case it does not describe.

---

## Level candidates (systemic-masked-as-project)

### L1 — **band-aid** — the prescribed shell-write idiom vs. the host — 12 in-epoch facts, 28 across four epochs

**Facts (12, Epoch 4):** `phase/take-up` @ VSH, RSC, PCL · `phase/research` @ RSC, SLL ·
`phase/plan` @ RSC · `phase/validate` @ RSC · `wrap/report` @ RSC · `wrap/curation` @ (—), OPC ·
`implement/code` @ PCL · `phase/distill` @ OPC. Natures environment ×8 / process ×4; solutions
workaround ×10 / removed-cause ×2. Every one routes the write to the **Write/Edit tool or a
scratchpad file invoked by path**.

The obstacle in the records' own words: quoted heredocs abort at parse time on Markdown bodies
(`unexpected EOF`, four facts); `$TMPDIR` is unset in this Git Bash so `cat > $TMPDIR/…` resolves to
a root path and is denied (two facts); heredocs collapse doubled backslashes (one fact). One run
recorded the sequence as it happened — *"third consecutive document write routing around the P1
heredoc parse failure"*, then *"fourth occurrence this run"*.

**Prior-epoch recurrence (read-only lookback): 16 further facts** — Epoch 1 ×3, Epoch 2 ×5,
Epoch 3 ×8. Epoch 3's are the most explicit: *"persisting the 7 extracts per the Bash-first
directive via quoted heredocs failed for the 6 largest (~10KB+) — shell transport truncated/corrupted
them into unmatched-quote parse errors"*, and *"the Bash spawn layer rejected a 3-heredoc save
(ENAMETOOLONG ~35KB)"*.

**Level hypothesis:** the cause is not the host alone — it is a **conflict between two documented
mechanisms**, and one Epoch-4 fact says so directly: *"tier writes used the harness Edit tool rather
than the prescribed .tmp-then-rename atomic write; the standing Tier-1 guidance prefers the
dedicated write tool over shell interpolation for structured content, **so the two documented
mechanisms conflict**."* The references prescribe an idiom this host cannot execute for the payloads
these steps actually carry; the project has absorbed that 28 times as per-chunk "workaround" facts.
The fixes live in the chunks; the cause lives in the reference text.

**Proposal direction:** reverse the default in the references that still prescribe the shell idiom
(`codebase-research.md`'s atomic `.tmp`-then-rename, `plan-template.md`, the wrap constraints'
atomic-write clause, the curation guide's `.tmp`-then-rename) so the dedicated write tool / a
scratchpad file by path is the **prescribed** route for a Markdown or code payload, with the shell
idiom retained where it genuinely applies (small, ASCII, single-line). That would retire the
recurring fact class outright rather than continuing to record it. See also **P1(a)**.

### L2 — **band-aid** — the code-graph answers, grep decides — 3 in-epoch facts + 2 prior

**Facts (3):** `phase/plan` @ VSH — *"no caller row for `declares` because its only call site sits
inside a closure… resolved by direct grep"*; `phase/research` @ PCL — *"the graph holds no symbol
row for `execute_scenario` or `persist`, so those two caller lists were enumerated by grep instead
and labelled as grep-derived"*; `phase/research` @ DTB — *"returned a false leaf (0 callers on an
indexed symbol that has a real caller), so the impact basis was re-derived with grep and **every
impact statement in research.md is recorded as resting on the grep basis**"*. Plus the typed
`tooling.graph-unavailable` (n=1) and X3's `tree-db` wrong / `cookbook` thin verdicts.

**Prior-epoch recurrence:** Epoch 1 ×2 — a graph/grep disagreement and a `plan.md` **prohibition**
(*"treat the graph calls view as under-reporting and grep before acting on a call-site count"*).
**Important qualification:** the Epoch-1 under-reporting claim was later **retracted** (the
unresolvable prose-form retraction reported in mechanism health — `rows=1` was a `head -30`
artifact, not a graph defect). The two Epoch-1 rows above are the *unretracted* remainder, and the
Epoch-4 facts stand on their own measurements.

**Level hypothesis:** the cause looks like indexer/query coverage in the code-graph pipeline
(closure call sites; symbols absent from the index; a false leaf on an indexed symbol), while the
fixes are per-chunk grep substitutions plus one hand-authored plan prohibition. The mitigation is
disciplined — every substitution was **labelled as grep-derived** in the artifact, which is the
right local behavior — but it means the graph's answer is now routinely not the basis the reference
names it as.

**Proposal direction:** the cookbook's own gap is the cheapest lever — X3 records that its
zero-rows protocol names two cases and not the third that actually occurred (*symbol indexed, real
caller, zero rows returned*). Documenting that third case, and what distinguishes it, would convert
a per-chunk judgment into a protocol step. Whether the indexer coverage itself is worth pursuing is
a separate and larger call.

### L3 — **band-aid** — an evidence-trace convention whose letter defeats its purpose — 3 in-epoch facts

**Facts (3), three different skills/steps:** `wrap/reconcile` @ RSC — amendment-flow requires a raw
twin per doc whose return needed stripping or carried proposals; five clean returns qualified but
*"a twin each would have been near-duplicate"*, so evidence was consolidated into
`fanout-results.md`. `phase/distill` @ DTB — fan-out.md requires a `.raw-{specialty}.md` twin per
modified return; the transport's HTML-escaping counted as a modification, but *"a re-encoded file
would be a **reconstruction wearing the name of a raw record**"*, so a single `.raw-NOTE.md`
recording the uniform delta was written instead. `wrap/gates` @ (—) — the 0-pending path produces no
`fanout-results.md` and git cannot stage an empty directory, so the Setup-created run dir *"would
have left NO trace in the commit"*; an `adaptation-record.md` was written so the audit trail
survives.

**Level hypothesis:** each convention is stated in **artifact-count** terms ("a twin per modified
return", "a run dir per invocation") rather than **evidence-purpose** terms ("the reviewer can
reconstruct what the transport changed", "the commit carries a trace of this invocation"). When the
count-form and the purpose diverge, the run satisfies the purpose by hand and records a workaround.
The fixes are three bespoke artifact shapes; the cause is the phrasing of the conventions.

**Proposal direction:** consider restating those clauses purpose-first with the artifact as the
default realization — e.g. a raw twin is required *unless* the delta is uniform and recorded once,
in which case one delta note substitutes. That is what all three runs actually did.

### L4 — **chronic-degrade** — `tooling.host-shell`, two epochs, zero halts

**Facts:** Epoch 3 = 14 records, Epoch 4 = 13 records. Impacts across the epoch are `retries` and
`extra_reads` only — **`halted` 0, `soft_exit` 0**. Alongside it, `ok-degraded` step outcomes run
Epoch 3 = 6, Epoch 4 = 4.

**Level hypothesis:** a two-epoch-stable rate that the halt policy structurally never surfaces —
every instance is absorbed inside the step that hit it, correctly, and the loop never sees a signal.
The three inverted verdicts in P1 are what makes this more than cosmetic: a probe that reports the
opposite of the truth is caught here only because the operator/author read the surrounding output.
The appendix's `U-b7` is the same shape from the other direction — cargo-mutants exiting 3 on a run
that **fully met** its acceptance, where *"anything gating on the exit code rather than the tallies
would invert this chunk's verdict."*

**Proposal direction:** this is the *level* reading of P1(b) — the content of the fix is proposed
there; what L4 adds is that no policy currently escalates it, so the rate is stable rather than
declining. If the founder wants it to decline, the lever is a reference-level checklist (P1b), not
per-chunk vigilance, which has now been applied 27 times across two epochs.

### L5 — **chronic-degrade** — a written learning did not prevent its own recurrence — 3 in-epoch records

**Facts (3):** `phase/distill` @ DTB — *"This is a REPEAT of the same defect already recorded in
CLAUDE.md session learnings for a prior 7-distiller run."* `phase/distill` @ OPC — *"Recurrence of
the failure already recorded as a session learning on 2026-08-21."* `wrap/curation` @ RSC (untyped)
— *"The unset-variable redirect hit at implement P2 is ALREADY documented verbatim in
docs/session-learnings.md 2026-08-18 as trap (2), including the exact failure and the exact remedy.
The lesson existed, was correct, and **did not prevent its own recurrence one session later** — the
corpus is authored but not consulted at the moment of risk. Recorded because it is a RETRIEVAL
signal rather than an authoring gap: **writing the entry a third time would not help.**"*

**Level hypothesis:** the corrective that the pipeline actually applies to a recurring defect is *an
entry in the learnings corpus*, and for these three the corpus did not fire at the moment of risk.
The cause sits in the mechanism (the transport that escapes entities; the shell that expands an
unset variable); the fix landed in project memory. Note this is the **capture side's own
conclusion**, not an inference imposed here — one record states it in exactly those terms and rules
out the obvious response.

**Proposal direction:** where a recurring defect has a **mechanical detector** available, prefer
moving it into the step's own check list over adding a corpus entry — P4's proposed sixth
per-extract check is the concrete instance, and the `OPC` wrap's semantic obs detector (which beat a
human-built pattern floor, P2 case 6) is evidence the approach works. This does not argue against
curation; it argues that a *third* identical entry is not the remedy the ledger supports.

### L6 — **deferred-forever** — the unattended-host live-leg and display gate — 4 in-epoch facts

**Facts (4):** `implement/code` @ DTB (env/deferred) — the live-leg toolchain is absent from this
host (*no `andromeda-pulse-mcp` or `conductor` on PATH, no running `pulse-app`*) and the legs need a
GUI window plus an operator action. `implement/fix-loop` @ DTB (env/deferred) — soft-exit Trigger 2,
same cause; *"no gate was skipped or mocked to compensate."* `implement/smoke` @ DTB (env/deferred)
— *"the plan step requiring an operator to open a Report cannot be performed by the implementer at
all"*, and launching `pulse-app` is a stated project non-goal. `implement/smoke` @ OPC
(env/**unresolved**) — no DISPLAY and no `tauri-driver`; *"the standing Epoch-5 display-gated CARRY,
not new damage."*

**Recurrence:** `tooling.headless-skip` — Epoch 1 = 3, Epoch 3 = 1, Epoch 4 = 2.

**Closure status — partial, and the closure route was itself the epoch's highest-impact friction.**
The `DTB` deferral closed only because wrap performed the deferred implement work **inside the wrap
invocation**: *"That is a deliberate reorder of a skill that forbids reordering; it was the only
route to a commit, but the loop currently makes it an unscripted judgment call rather than a defined
transition"* (`halted` 1, `dialogue_rounds` 2 — appendix `U-b1`). The `OPC` fact is **unresolved**
and carried forward as an open Epoch-5 CARRY.

**Level hypothesis:** the cause is an environmental boundary that is stable and known (this host
cannot drive an attended GUI leg), while the fixes have been per-chunk deferrals plus one
out-of-contract reorder. Because the boundary does not move, each affected chunk pays the discovery
cost again at smoke time.

**Proposal direction:** two separable pieces. (1) A **defined transition for wrap-on-a-soft-exited
chunk** — the `DTB` record argues the state is reachable by design and the loop has no sanctioned
path for it; a named branch (finish-in-wrap under operator ruling, or return-to-implement) would
convert a judgment call into a contract. (2) Detect the boundary at plan time rather than smoke time
(see **P5**), so an attended leg is scheduled rather than discovered.

---

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — `wrap-session/{route-resolve, gates}` — 3 cases → proposed type `contract.token-proxy-check`

**Cluster:** a check, guard, or derived condition matches a **token** and is read as testing a
**property** — so it returns a confident wrong answer on content that is incidental, self-referential,
or differently phrased. All three were settled by *reading* the hits rather than counting them.

| chunk | step | the `what`, cited |
|---|---|---|
| (—) | wrap/route-resolve | *"clearing an external-dependency block nearly left its own flag token behind: the replacement CONTEXT annotation named the flag in prose, so the literal token survived a clear that had structurally removed it. The route grammar uses a bare ALL-CAPS token as a structural flag, so **prose ABOUT the flag is indistinguishable from the flag itself** to any consumer that greps for it — and phase Setup HALTs on promoting a blocked entry, so a false HALT on the very next promotion was the failure mode"* (reformulations 1) |
| OPC | wrap/route-resolve | *"The freeze-contract self-check produced a false alarm… Asserting P5 touched no frozen line by grepping the working-route diff for a bracketed-marker prefix **returns a hit on the CURRENT chunk's own phase-time stamp**, which /andromeda-phase wrote at promotion and which is therefore in the diff against HEAD through no action of the wrap"* (extra_reads 1) |
| OPC | wrap/gates | *"The evolve-nudge derived condition (no proposals.md NAMES that epoch) nearly suppressed a nudge that should fire… both are incidental references inside findings about other epochs… **A bare name-match proxy for has-been-diagnosed reads a passing mention as a diagnosis**; resolved by reading the two hits rather than counting them"* (extra_reads 1) |

**Same-mechanism correlate outside the untyped set** (Stage-4 theme T5, n=1, counted there):
`phase/distill` @ SLL — *"the documented anchor check keys on the literal `per-{plan} §` form; three
extracts carried real §anchors in other grammatical positions, so **the check as written flags
compliant content**"*.

**Cross-epoch note (read-only, not counted toward the threshold):** the mechanism fired a **fourth**
time at `new-session/orientation` on 2026-08-22 (Epoch 5) — the same evolve-nudge predicate, this
time actually matching the Epoch-3 diagnosis's passing mention of Epoch 4, resolved by opening the
file. That instance is *this* diagnosis run's own setup.

**Draft criteria line** for the owning playbooks (`wrap-session/route-resolve`,
`wrap-session/gates`; the shape also fits `phase/distill`):

> `contract.token-proxy-check` — a check, guard, or derived condition tested for a **token's
> presence** where the intended property is **semantic** (is this flag structurally set? has this
> epoch been diagnosed? did this edit touch a frozen line?), and the match was satisfied by
> incidental, self-referential, or prose-about-the-token content. Record when the wrong answer was
> caught by reading the hits — and note the failure direction, since both a false positive (a
> spurious HALT) and a false negative (a suppressed nudge) have occurred.

Extending a playbook is an Andromeda change; this is surfaced for the founder's judgment only.

---

## Below threshold — no action

**Typed groups, n=1 unless noted** (all sub-threshold; listed for the founder's eye):

- `phase/validate/contract.narrow-basis-claim` — n=1 but **weight 7**, the highest single-record weight in the epoch (dialogue 2, extra_reads 2): the code-audit's prose contradicting its own JSON. Counted in P2.
- `wrap-session/reconcile/ambiguity.escalation-rounds` — n=1, weight 6.
- `wrap-session/reconcile/contract.false-positive-proposal` — n=2, 2 chunks, no halt impact.
- `implement/fix-loop/contract.spec-reality-gap` — n=2 · `implement/fix-loop/contract.test-expectation` — n=2 · `wrap-session/reconcile/contract.cascade-miss` — n=2 · `phase/distill/input.spec-source-gap` — n=2 · `phase/validate/contract.intent-divergence` — n=2 · `implement/code/input.conventions-gap` — n=2 · `implement/fix-loop/ambiguity.scope-pressure` — n=2 · `implement/code/tooling.hook-friction` — n=2.
- `contract.premise-falsified` (Universal, by type) — n=2, weight 4, 1 chunk: below the n≥3 bar and carrying no halt impact.
- `contract.structural-blind-spot` (Universal, by type) — n=1.
- Singletons: `wrap-session/reconcile/ambiguity.playbook-no-match` · `phase/research/tooling.graph-unavailable` · `phase/plan/ambiguity.scope-question` · `phase/validate/contract.matrix-claim` · `phase/validate/contract.mechanical-check` · `implement/fix-loop/tooling.environmental` · `phase/research/retry.query-reformulation` · `phase/research/tooling.output-cap-overflow` · `new-session/orientation/tooling.output-cap-overflow` · `new-session/orientation/tooling.health-false-red` · `new-session/orientation/input.handoff-git-mismatch` · `phase/take-up/input.carry-context-gap` · `phase/take-up/input.working-entry-thin` · `phase/research/input.extract-signal-gap` · `phase/research/input.cookbook-gap` · `phase/plan/input.extracts-conflict` · `phase/plan/input.research-thin` · `implement/code/input.research-files-wrong` · `implement/code/input.plan-step-ambiguous` · `wrap-session/route-resolve/contract.carry-no-owner` · `wrap-session/curation/recall.dedup-vs-cascade` · `wrap-session/gates/tooling.commit-mechanics` · `wrap-session/report/recall.change-reconstruction`.

**Untyped clusters below F-4** (n=1 each; no prior-epoch recurrence found for any):

- **`U-b1` — the loop has no sanctioned path for wrap-on-a-soft-exited chunk.** `wrap/report` @ DTB, **`halted` 1 · `dialogue_rounds` 2 — the highest-impact untyped record in the epoch.** Running the phases in order would have spent a 7-agent fan-out and a full curation pass only to hit a mandated HALT at the P7 coverage gate. Resolved by halting before P1, putting the two contract-sanctioned valves to the operator, then performing the remaining **implement** work inside the wrap invocation. Feeds **L6**'s proposal (1). *A prior-epoch scan for a recurring "no sanctioned path" record found none — this is n=1 and stays here, despite its impact.*
- **`U-b2` — the corpus is authored but not consulted at the moment of risk.** `wrap/curation` @ RSC. Counted in **L5**.
- **`U-b3` — dedup had to treat freshly-cascaded body text as an existing entry.** `wrap/curation` @ PCL, extra_reads 2. The other side of P6's second case.
- **`U-b4` — a pre-authorized halt satisfied *ahead of* the phase.** `wrap/route-resolve` @ VSH: a trajectory edit that normally HALTs was applied with zero dialogue rounds under a standing operator ruling — *"worth distinguishing in the ledger from a trajectory applied unilaterally."* A vocabulary observation, not a defect.
- **`U-b5` — flip-compaction fired for the first time and did its one-time backfill in the same sweep.** `wrap/gates` @ VSH, zero impact: 28 lines compacted, `route-archive.md` born with 28 verbatim entries, working-route 76,790 → 18,201 bytes, marker↔archive set-equality 28/28. A success record.
- **`U-b6` — an `Edit` `old_string` reproduced a paragraph with the line wrap in a different position**; one of four batched scope-closure edits failed to match, the other three applied. `phase/research` @ OPC, retries 1. *"The tool errored rather than silently no-matching, so the miss was visible immediately."*
- **`U-b7` — cargo-mutants exited 3 on a run that fully met its acceptance** (0 missed, all five named survivors killed) because two pre-existing timeouts remain in the same files. `implement/fix-loop` @ RSC. *"The project rules record the trap in one direction only… this is the converse, a complete success reading as a non-zero failure."* Cited in **L4**.

**Stage-4 themes below threshold:**

- **T4** — the named contract not read, schema probed (n=2 in Pass A). Surfaced at full strength as **P3** (4 records incl. 2 typed frictions).
- **T5** — a literal-form check flags compliant content (n=1). Counted in **U1**.
- **T6** — the plan's file list forced wider by the change class (n=2). Counted in **X1**.
- **T8** — a host/tool resource ceiling forced a different execution route (n=2): the tool-result size cap on a whole-file `working-route.md` read (recovered by an awk structural filter), and a ~4-minute mutation gate over the foreground Bash ceiling (backgrounded to the scratchpad).
- **Override signature** — 2 facts, and they are **two different rules**, not one rule recurring: the operator denied an `rm -rf` of `mutants.out` (leaving untracked, un-gitignored output that would ride a `git add -A`), and the operator selected the webview branch over a marked CLI recommendation at a P4 fork the phase directive had reserved as their call. No miscalibration signal.
- **`removed-cause` observation** — 6 facts, no single cause recurring at n≥3, so no "the same cause keeps returning" hit. Two of the six belong to **L1**; the others are distinct one-offs, including one worth its own line: `wrap/reconcile` @ DTB caught an obs amendment that had landed as an **8th row in a fixed 7-path table**, silently moving a derived count that three documents state as 7 — *"caught during the cascade leaf-recompute, not by any detector — obs has no derived-count detector, unlike layout/tests/design."*

**Unclustered Pass-A facts (4):** the cp1252 stdout `UnicodeEncodeError` on U+2193 (counted in P1);
the `rows: 10` tail-clipped re-derivation (counted in P2); the obs 8th-row derived-count catch
(above); and the `phase/validate` @ OPC decision to fold a second stale axis into scope rather than
defer it, with the deliberate non-take of `CLAUDE.md:41` because *"it sits in a GENERATED block
whose owner is the master, so hand-editing the leaf would mint a divergence."*

---

*Evidence twins: `q-retractions.json` · `q-health.json` · `q-typed.json` · `q-untyped.json` ·
`q-chains.json` · `q-level-raw.json` · `q-level.json`. Scripts: `analyze.py` · `level.py`.*
