# Evolve Diagnosis — Conductor · conductor-0.3.0 Epoch 1 "Foundation: the measurements the closures rest on" · 2026-09-14T18:23:25Z

## Mechanism health

**160 records (78 step / 82 friction) · 5 chunks · 0 unparseable (whole ledger, 2029 lines).**

| chunk | step records | note |
|---|---|---|
| `2026-09-11-hosted-runner-endpoint-cause-probed` | 13/13 | complete |
| `2026-09-11-hosted-runner-endpoint-cause-closed` | 18/13 | `implement/code` ×3, `implement/fix-loop` ×4 — implement re-entered, not a gap |
| `2026-09-12-ledger-gate-id-space-generalised` | 13/13 | complete |
| `2026-09-13-audit-debt-retired-before-epoch-1-closes` | 12/13 | **missing `wrap-session/gates`** |
| `2026-09-13-p-025-measurement-contract-for-pulse` | 13/13 | complete |

`chunk: null` step records: 7 × `new-session/orientation` (one per session start, expected) + one
`wrap-session/curation` + `wrap-session/route-resolve` pair — the operator-requested adaptation wrap,
which `diagnosis-pass.md` §Stage 0 names as legitimate, not a gap.

**The one coverage gap is explained outside this skill's inputs.** The `audit-debt` chunk's missing
`wrap-session/gates` record is not a checkpoint that failed to fire: the session handoff records that the
checkpoint fired and its records — the step record and its friction sibling — were discarded by an operator
`reset --hard` re-checkout after the commit. Basis: the handoff narrative, which is not an input to this
pass; stated so the diagnosis does not propose a fix for a non-defect. Nothing in the ledger can confirm or
refute it, and no retraction can target records that never landed.

**Retraction pre-pass (whole ledger, per the reference):** 11 retracted friction ids · 2 retracted
problem-facts · 0 clause-retractions · **0 of any kind inside this epoch** (Stages 1–4 ran over an
unfiltered target stream). Reported verbatim for manual discount:
- 1 **unresolvable** — a pre-boundary prose-form `retracts` (`id: null`), discounting "the untyped
  code-graph-under-reports record and the first problem-block entry on…".
- 1 **retraction targeted by a retraction** → founder review (retracting a retraction reinstates nothing).

**Malformed `ts`: 11, kept and listed.** All eleven belong to `2026-08-08-dependency-advisory-remediation`
(conductor-0.2.0, outside this epoch) and carry a Windows `date`-builtin prompt string instead of a
timestamp — the pre-recipe shell append the reference documents. They remain in every count; no stage
orders by `ts`.

**Quality trend — this epoch is the cleanest on record:**

| metric | this epoch | prior 7 epochs |
|---|---|---|
| untyped rate | **5/82 = 6 %** | 10 % · 17 % · 15 % · 12 % · 19 % · 31 % · 25 % |
| `id` fill | **160/160 = 100 %** | (required from 2026-08-18) |
| problem-fact fill | 29/78 = 37 % | — |
| outcomes | 74 `ok` · 3 `soft-exit` · 1 `ok-degraded` | **first epoch with zero `halted-resolved`** |

**Calibration boundaries in range:** the deviation scan, `graph-not-applicable`, the Universal types and
the `id`/`retracts` schema are all live for the whole of this epoch — no era discount applies to any record
here. One observation: **`contract.grammar-irregularity` has zero records in the entire ledger** since its
deploy. A type with no uptake is either a class that has not occurred or a checkpoint question nobody
reaches; the diagnosis cannot tell which, and says so rather than reading zero as evidence.

---

## Proposals (typed patterns)

8 of 36 groups cleared the F-2 thresholds. Universal and `recall.*` types are grouped by type alone across
steps, per the reference.

### P1 — `tooling.host-shell` — 10 cases · weight 16 · 6 chunks (incl. orientation)

**Pattern:** the Windows/MSYS↔native boundary produced a wrong or empty reading, or refused a first-reach
form, in every chunk of the epoch — and in five of the ten the corpus already documented that exact trap
by name before it fired.

**Evidence:** ALL 10 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| probed | `git check-ignore -v PATH \| head -2; echo exit=$?` reported `head`'s 0, reading as IGNORED where the bare command returns 1 (NOT ignored); would have passed health check 7 over two redirect targets in the committed evidence dir | extra_reads 1 | host rule documents this under *Exit codes* |
| probed | evolve append as a hand-escaped JSON heredoc died `Invalid \escape` at char 2060 on two backslash-bearing values (the host-path gate's drive-letter anchor, and a PowerShell registry-hive provider prefix — both described rather than quoted here, per the host rule's own output-fix clause); the validated append refused the payload and nothing landed | retries 1 | `evolve-system.md` requires the python-dict raw-literal form |
| null (orientation) | a 14-marker completeness enumeration piped through `sort\|uniq -c\|head -5` truncated to 5 rows, leaving the 14-member claim uncitable | extra_reads 1 | re-run bare with an aggregate miss-counter |
| cause-closed | two traps in one pass: an MSYS-form user-profile path handed to a native python raised FileNotFoundError until rewritten in the Windows-native form (the two spellings are described, not reproduced — neither was a real host path even in the source record, both being elided prefixes); `grep -oE` with `[^ ]*` clipped every match at the letter `r`, silently truncating eleven readings | extra_reads 2 | both already in the corpus |
| cause-closed | `runs/a11y-e2e.log` is UTF-16 with a BOM (PowerShell redirect), so a grep for `msedgedriver`/`session`/`DevToolsActivePort` returned ZERO on a file containing all three — a zero that would have read as "the leg never got that far" | extra_reads 1 | caught because an empty result on a 3956-byte capture was implausible |
| ledger-gate | an inline `python -c` probe used a backslash inside a quoted split; the shell consumed the escape and python raised `SyntaxError: unterminated string literal` | retries 1 | `…-phase/tree-query-…json`; host rule prescribes the heredoc for any quote-bearing probe |
| audit-debt | a `cat`-heredoc **with a file target** was used to persist the entity-decode script; the host PreToolUse guard blocked it | retries 1 | `.andromeda/runs/2026-09-13T17-30-00-phase/` |
| audit-debt | python default text-mode write translated LF→CRLF across the saved extracts, so a11y's saved copy differed from its raw twin by 41 line endings with **zero entities decoded** — the twin rule keys on `raw != saved`, so it would have recorded a11y as decoded | retries 1 | `cmp` differed at +41 bytes over 41 lines |
| audit-debt | `$TMPDIR` is unset in the Bash tool's shell, so `cargo check > $TMPDIR/emit_check.txt` wrote to `/emit_check.txt` and failed permission-denied, discarding the output; the bare exit then read 1 for the redirect, not the build | retries 1 | third host-shell event in that one phase run |
| p-025 | a text-mode python append wrote the host terminator into the LF-pinned friction ledger, taking a 2003-line all-LF file to `w/mixed` on its first append after the repo-wide `.gitattributes` LF pin | retries 1 | `git ls-files --eol` → `i/lf w/mixed`; repaired byte-precisely, re-measured `w/lf` |

**Proposal:** the corpus is not the binding constraint — `host-win32.md` already names five of these ten
before they fired, so more prose is unlikely to move the number. Two mechanism-side directions, both
narrower than a rule: (a) a **harness-level default** for the two forms that recur with a single correct
spelling — a python invocation wrapper that opens files `newline=''` and a scratchpad-path helper — so the
correct form is the *reachable* one rather than the remembered one; (b) the `| head`/`| tail` exit-code and
coverage traps are detectable **mechanically** in a command string, which makes them a candidate for the
existing PreToolUse guard that already blocks heredoc-to-file (that guard fired correctly twice this epoch
and is the one intervention in this table with a measured catch rate).

### P2 — `phase/validate` · `contract.mechanical-check` — 7 cases · weight 14 · rate 1.40 per validate run

**Pattern:** validate's mechanical checks found something in every run (7 events / 5 runs, the highest count
in any epoch: `[3,1,2,1,2,6,6,7]`), but **4 of the 7 were defects no mechanical predicate attempts at all**,
and every one of those four was caught by the operator at the P5 review.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| cause-closed | gate fence failed check 4(0): a baseline value authored as a TOML **literal** string containing an escaped apostrophe, which literal strings do not support, so the string terminated early — the plan template states this rule verbatim | iterations 1 | caught by the re-parse |
| audit-debt | check 4(4): two acceptance criteria asserted runtime artifacts `caught.txt`/`missed.txt` while no gate entry carried an `artifact` key | iterations 1 | resolved by adding `artifact = target/mutation-gate/` |
| audit-debt | check 4(9): the NEW rustdoc entry baselined **RED at HEAD** (exit 101) on five pre-existing `private_intra_doc_links` errors in `conductor-emit` — a gate the chunk would have inherited; the authoring pass assumed `-D warnings` was clean because a sibling chunk used that form against a different crate | iterations 1 | `.andromeda/runs/2026-09-13T17-30-00-phase/` |
| audit-debt | **no predicate attempts this** — step 3 prescribed a `#[cfg(test)]` pin that kills the mutant but leaves an unused `pub fn` the next audit still counts as a zero-reference candidate. No check inspects whether a kill's DISPOSITION KIND is right | iterations 1 | settled on the seam; disposition became removal |
| audit-debt | **no predicate attempts this** — `scripts/mutation-roster.toml` and test-plan §12 were two copies of one set with nothing comparing them, so a wrap amending one would diverge silently | iterations 1 | resolved by making the roster coordinate-free, joined by member id |
| p-025 | **no predicate attempts either property** — the plan framed the contract's start instant as a value Pulse must MINT and left the effective-at rule unstated; the operator supplied the backend source that already holds it (expose, not mint) and flagged that an unstated rule fails the capability's own implementable-without-a-follow-up-question qualifier | iterations 1 | plan step 4 rewritten; research gained a P5 addendum |
| p-025 | **no predicate compares a number across artifacts** — one measured value carried three renderings across committed artifacts and the plan added the only wrong one (a truncation where every other artifact rounds) | iterations 1 | all four artifacts now agree on the rounded form |

**Proposal:** two of the four operator-caught defects are **mechanically expressible and currently
unexpressed**, which is the actionable half: a *scalar-consistency* predicate (one measured number rendered
identically wherever it appears, with the raw value cited once at its anchor) and a *duplicate-set* predicate
(two artifacts holding one set with no comparator between them). The remaining two — whether a kill's
disposition KIND is right, whether a spec's asked-for value is minted or exposed — are semantic and belong
to the review, so the proposal there is the opposite one: record in the playbook that P5 is the *only* owner
of those classes rather than leaving them to read as gaps in validate. Note the epoch already absorbed one
of these at project level (the coordinate-free roster shipped); the pipeline generalization is what remains.

### P3 — `contract.token-proxy-check` — 7 cases · weight 9 · 5 chunks

**Pattern:** a probe, gate or derived condition tested for a TOKEN where the intended property is semantic.
Direction split **4 false-positive / 3 false-negative**, and in all 7 the truth came from *reading the hits*
— not one was resolved by refining the pattern. Chronic since the type deployed: `[…,7,1,24,7]`.

**Evidence:** ALL 7 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| probed | twice in one step, both false-NEGATIVE: `grep -rln 'journal_conformance' crates/` missed `crates/conductor-run/tests/journal_conformance.rs` because a test file need not contain its own name (resolved by `find -name`); and a registration probe keyed on the backticked `` `TEMP` `` form would have reported an absence a wider pattern had to refute | extra_reads 2 | re-probed with a form the target could satisfy |
| probed | false-POSITIVE: the host-path hygiene gate keys on `[A-Za-z]:[\/]`, also satisfied by the PowerShell registry provider form, so a verbatim reading would have reported a leak that is not one. Found while writing the probe, not at the gate; fixed the OUTPUT (colon-free `reg.exe` form), not the pattern | — | — |
| cause-closed | false-POSITIVE twice on one artifact: first a quoted URL **scheme** (`p`+`:`+`/` satisfies the drive-letter anchor), then AGAIN on the prose written to explain that false positive, because the explanation spelled the offenders. Fixed the output twice | iterations 2 | `grep -cE` over reading.md: 2 → 2 → 0 |
| null (orientation) | false-MARKERLESS: the working-route structural parse tested for **asterisk**-delimited italics while the file writes its preamble **underscore**-delimited, so preamble lines 3–8 classified as ENTRY and the cursor resolved to line 3 instead of line 17 | extra_reads 1 | truth came from reading the labelled hits the parse printed |
| null (orientation) | false-EMPTY: the master-route record sweep guessed a bullet-list prefix where records are BARE lines, returning 0 rows — reading as "this version has no records"; the status tally in the same call returned 124 complete, which exposed it | extra_reads 1 | — |
| audit-debt | false-POSITIVE: a check-5 placeholder-leak probe matched `{[a-z_]*}` and returned 7 hits, every one deliberate parameter notation inside code spans. The documented check is a predicate over **non-code** text, which the probe did not encode | — | truth came from reading the hits |
| p-025 | false-NEGATIVE: **the evolve nudge's own suppression test** compares the epoch LABEL TOKEN only, and that token is not unique across versions — a diagnosis titled for the *previous* version's Epoch 1 matches, and would suppress the nudge for this one | dialogue_rounds 0 | that run's title names `conductor-0.2.0 Epoch 1`, the closed epoch is 0.3.0's |

**Proposal (main):** every case was resolved by reading the hits, so the generalizable change is to make
*reading them* the cheap default — where a probe stands in for a semantic property, have it print the matched
text (`grep -o`) and an expected hit count/identity rather than a bare count or a boolean. Two of the four
false positives came from one gate's `[A-Za-z]:[\/]` anchor, which the project has now tripped four times on
four non-path classes; that specific anchor is a candidate for a scoped rewrite (or an output-form rule) in
its own right.

**Proposal (sub-shape, strongest cross-epoch evidence): the evolve-nudge "has this epoch been diagnosed?"
predicate.** This one predicate has now failed in **five epochs, in both directions, on three distinct
axes**, and each hardening fixed the previously-observed axis while the next axis went unguarded:
- *Epoch 4* (`wrap/gates`, 2026-08-22) — a bare name-match nearly suppressed a due nudge: the 2026-08-20 run
  contains "Epoch 4" twice, both incidental references inside findings about other epochs.
- *Epoch 5* (`orientation`, 2026-08-22) — the predicate matched a passing mention; a literal grep would have
  suppressed a due nudge, and opening the file to read its target heading cost the extra read.
- *Epoch 5* (`orientation`, 2026-09-02) — the epoch list was piped through `head -8`, truncating before the
  epoch in question; "a head-limited view is never the answer when the question is set membership."
- *Epoch 5* (`wrap/gates`, 2026-09-02, typed) — grepping for "Epoch 5" hit the 2026-08-22 run, which is
  titled Epoch 4 and mentions Epoch 5 only inside a timestamp annotation. False negative.
- *this epoch* (`wrap/gates`, 2026-09-14, typed) — **the version axis**: the label token repeats across
  versions, so 0.2.0's Epoch 1 diagnosis matches 0.3.0's Epoch 1.

The deployed rule already carries the scar tissue of the first four ("mis-evaluated four times across two
epochs, both directions"; "never a body substring, never a head-limited match") — and the fifth still got
through, because the rule says to compare *the label token only*. Direction: make the predicate key on the
**(version, epoch) pair** the title line actually carries, and treat a title that names no version as
not-targeting rather than as matching every version. Two of the seven deployed diagnosis titles carry a
version token and five do not, so the fix has an inventory problem as well as a predicate one — a stable
machine-readable target field in the title line would close both.

### P4 — `contract.narrow-basis-claim` — 6 cases · weight 7 · 5 chunks

**Pattern:** a count, absence or ownership claim was authored **as if measured** with the command never run.
All six were caught by a pipeline re-derive step before any consumer read them. Chronic:
`[…,7,6,3,5,28,6]`.

**Evidence:** ALL 6 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| cause-closed | the report asserted a SOLE owner per expected amendment ("the same grep over the other six masters → 0 hits") on a basis never run; re-deriving found a co-owner for BOTH (`architecture.md:205`, `test-plan.md:469`) | extra_reads 2 | caught at the template's re-derive-before-fan-out step |
| ledger-gate | the directive supplied a grep basis explicitly so the plan could cite rather than re-derive it; the pattern anchors the token to a preceding double-quote, so a bare sweep returned 13 lines against the pattern's 11 — the two missed being a provenance citation and, decisively, the fixture INPUT driving the assertion on the function under repair | extra_reads 2 | `…/scope.md`; the supported conclusion re-verified true |
| null (route-resolve) | the adaptation directive said the freight was "nothing to re-derive here"; re-derivation found its hygiene coordinate true only for RESOLVABLE references — three hits exist, two resolve to nothing | extra_reads 6 | `…2026-09-13T16-53-33-wrap/adaptation-record.md` |
| audit-debt | **four** bases written into the report's Expected-amendments bullets as measured without the commands run: a §12 locating grep never executed; llvm-cov hits 4 stated vs 6 measured; `conductor-verify/tests/common` 1 vs 2; `gitattributes` master hits 2 vs 3 — the last inverting an ownership claim that was false | extra_reads 4 | all four caught by the report template's own re-derive instruction |
| audit-debt | the arch sidecar claimed a sweep returning 0 before the lateral fix had landed, and omitted the `CLAUDE.md:21` distillation hit; running the claimed sweep found 1 master hit and 1 distillation hit. **Second instance this session** | extra_reads 2 | corrected in the sidecar before the pass closed |
| p-025 | reported the advisory-database local copy clean **from the wrong copy** — probed a path under the user profile while `CARGO_HOME` points elsewhere on this host, so the 0-porcelain reading described a directory `cargo audit` does not read | retries 1 | corrected by taking the path from `cargo audit`'s own output line |

**Proposal:** the pipeline's re-derive steps caught 6/6 — they are the load-bearing control, and the
proposal is to **extend that mechanism rather than add corpus text** (see P6 for why text is not working
here). Two concrete extensions the evidence names: the same re-derive discipline the report template
enforces before fan-out is absent from the **sidecar** authoring step (two of the six landed there, one
explicitly "second instance this session"); and a *directive-supplied* basis is currently exempt from
re-derivation by convention — two cases here were wrong bases the directive supplied, so the direction is
that a cited basis is re-run once regardless of who supplied it, with the citation recording that it was.

### P5 — `wrap-session/reconcile` · `ambiguity.playbook-no-match` — 3 cases · weight 5 · rate 0.60

**Pattern:** a proposal group matched no playbook rule, and in each case the nearest candidate was **tested
and correctly failed a load-bearing qualifier** — so the cost is the qualifier test, not a wrong decision.
Two of three escalated to the operator; none proposed a new rule.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| probed | the directory-tree proposal matched no rule; the nearest (`:49`) dismisses per-item content inside an already-registered directory, but its own reasoning scopes it to a directory tracked at DIRECTORY grain holding config CONTENT, while `scripts/` is tracked per-FILE and the new file is an executable entry point. Resolved on the no-match-not-surprising branch; testing the qualifier cost one full read of the rule | extra_reads 1 | plan had anticipated it; report substantiated it |
| cause-closed | 7 of 10 proposals matched NO rule; `:149` is the only candidate and fails clause (a) DIRECTNESS because both primaries name their retire-condition explicitly and it is measurably unmet. Escalated; resolved by operator directive. No rule proposed: routinising a posture change whose retire-condition is unmet is the precedent-widening the escalate branch exists to prevent | dialogue_rounds 0 | A11y job failure at run 34654076633 |
| audit-debt | the §12 coordinate-identity retirement (5 proposals) matched no rule; `:127` was tested and correctly failed its SECOND qualifier, because prior wraps had already substituted fresh literals that re-staled (2026-09-04, 2026-09-05), so that qualifier holds rather than fails | dialogue_rounds 1 | escalated by unease; resolved with the operator; no rule proposed |

**Proposal:** this group reads as the mechanism working — the escalate branch fired on exactly the classes it
exists for, and the deliberate non-minting of a rule in two of three is the designed outcome. The only
recurring *cost* is that establishing "no rule governs this" requires a full read of the nearest candidate
to test a qualifier. Direction, if any: record the tested-and-failed qualifier beside the rule (a
"tested-against, failed on clause X" note) so the next no-match at the same rule inherits the test instead of
repeating it. Rate 0.60 per reconcile run makes this the most frequent per-run group in the epoch.

### P6 — `recall.corpus-recurrence` — 3 cases · weight 4 · 3 chunks

**Pattern:** a documented corpus entry states the exact rule that would have prevented the event, and did
not prevent it — in all three the catch came from a pipeline **re-derive step**, never from recall.
Chronic: `[…,1,3,12,3]`.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| probed | THREE session findings deduped against entries that already state the rule correctly, so no copy was minted: the evolve heredoc append dying on backslash values (`evolve-system.md` already REQUIRES the dict form); the report naming a spec master with zero hits as an amendment owner (the report template already bans it in those words); a `-o` pattern clipping its matches (CLAUDE.md and `host-win32.md` both cover it). "Each is the entry failing to prevent its own recurrence, not a gap in the corpus" | — | logged to the handoff, deliberately not re-curated |
| audit-debt | wrote a sweep result into an artifact before running it, twice in one session — four false bases in the report, then a premature 0-hit claim in the arch sidecar. CLAUDE.md's Tier-1 2026-08-21 entry states this rule and the narrow-basis family states it again | extra_reads 6 | logged as recurrence-despite-learning rather than curated as a third entry |
| p-025 | wrote the cascade sweep's basis into the architecture sidecar BEFORE running the sweep; running it then contradicted the text, finding two stale leaf sites the entry never named and a row it omitted | retries 1 | CLAUDE.md's Tier-1 2026-08-21 entry states this rule **and the immediately preceding session's handoff records the same failure twice** |

**Proposal:** this is the epoch's sharpest mechanism signal and it points away from curation. The same rule
is now stated in CLAUDE.md Tier 1, in a rule file, in the report template and in `evolve-system.md`, and it
recurred in three consecutive chunks — the third time in a session whose *own predecessor handoff* recorded
it twice. The project's response each time was correct and deliberate (dedupe, log, do not mint a copy), so
the corpus is not under-written; it is being consulted after the fact rather than before. Direction: for the
one class that recurs most (a sweep result authored before the sweep runs), the effective control is already
known and mechanical — the re-derive-before-consume step — so the proposal is to extend that step to the
authoring surfaces that lack it (see P4) and to stop treating recurrence of this class as a curation
question. A secondary observation for the founder: `recall.corpus-recurrence` firing at all is the signal
that a *rule* has been chosen where a *mechanism* was needed.

### P7 — `implement/fix-loop` · `tooling.gate-deferral` — 2 cases · weight 5 · rate 0.25 · **soft-exit impact**

**Pattern:** above threshold on the halt clause (n=2 with a `soft_exit`). The source-delta-proportional
deferral rule is designed behaviour and worked correctly in the first case; in the second it **hid a red gate
that the same commit introduced**, because the rule's definition of "delta" does not cover data files a test
reads.

**Evidence:** ALL 2 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| probed | the workspace unit gate was deferred under the rule; the plan carried the defer key and its reason was re-verified against P1's actual diff before honouring it — the diff is the workflow file, one new PowerShell script and bookkeeping, zero `.rs`/`Cargo.toml`/`Cargo.lock`/toolchain delta. Every changed-surface gate ran | deferred 1 | designed behaviour, honoured correctly |
| cause-closed | the rust gate the PREREQ discharged is RED on a defect that landed **in the same commit as the deferral that hid it**: `b3b51e6` added `conductor-0.3.0/{requirements.md,verification-matrix.json}` while deferring `cargo nextest --workspace` for zero `.rs` delta, and the ledger gate filters requirement ids by a baked `v2-` prefix, so the newly-scanned 0.3.0 dir yields an empty required-set and the anti-vacuity assert fires. **The rule defines delta as manifests/lockfile/build scripts/codegen inputs/toolchain pin — it does NOT cover DATA FILES A TEST READS**, which is exactly what moved this verdict with zero `.rs` edits | soft_exit 1 | 906 run, 905 passed, 1 failed |

**Proposal:** the second case is a precise, self-diagnosed definitional gap with a one-line shape: extend the
source-delta definition to include **files a committed test reads as input** (fixture and manifest data under
version control), so adding a version directory the gate scans counts as delta. The epoch already paid this
debt at project level — the `ledger-gate` chunk repaired the gate and discharged the red — but the deferral
rule that hid it is unchanged, and the same shape recurs whenever a new version directory appears. Worth
noting alongside: the project's `wrap/gates` light gate then had to pass on a *recorded disposition* rather
than a green for this same red (see L2), so one definitional gap produced friction at two steps.

### P8 — `contract.premise-falsified` — 3 cases · weight 3 · 2 chunks

**Pattern:** verification falsified a premise an authored artifact states — in all three the falsified
artifact was the chunk's own scope or the working entry that produced it, and in two the correction
*narrowed the ask before implement*. Chronic: `[…,16,2,10,13,24,3]`.

**Evidence:** ALL 3 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| cause-closed | scope asserted the probe's completion marker is asserted by a committed gate; a grep over `*.yml *.ps1 *.sh *.ts *.rs` returns exactly one hit, the emit site itself — the only assertion ever written is a `[[gate]]` entry in the PREDECESSOR chunk's plan fence, which is per-chunk and not re-run, so no standing contract exists and the marker's arity is this chunk's to choose | extra_reads 0 | `…/research.md` |
| audit-debt | the phase directive and the working entry classed 12 of 17 non-stub mutation survivors as holes or ratify-by-class CANDIDATES; test-plan §12's roster already covers all 12 as RATIFIED members with cited rules. **The true split is 5 unowned / 12 already-ratified, not 12/5** | extra_reads 6 | `…/scope.md` |
| audit-debt | the entry's stated outcome "survivors 25 → ≤ 8" is unreachable by the project's own gate mechanism: `.claude/rules/testing.md:19` requires `missed.txt` to hold exactly the accepted-deliberate survivors "since such a survivor by construction survives", and the workspace ships no `mutants.toml`, so the 12 ratified members stay counted. Best reachable outcome is 25 → 20 | extra_reads 3 | `…/scope.md` |

**Proposal:** all three were caught at `take-up`/`research` — the earliest possible point — and all three
resulted in a narrowed, honest ask rather than work built on a false premise, which is the mechanism working
as designed. The pattern worth surfacing is the *source* of the false premise: two of three came from the
**working-route entry and its directive**, which state a target outcome and a population split authored well
before the measurement. Direction: nothing in the route grammar distinguishes a measured figure from an
estimated one, so a take-up premise check has no way to know which claims to re-derive. Marking a route
entry's quantitative claims as measured-vs-estimated at authoring time would tell take-up where to spend its
re-derivation, and would stop an estimate hardening into an acceptance under drift=0.

---

## Cross-step chains (starting heuristics)

### X1 — `implement/fix-loop` (and `implement/smoke`) →`implement-outcome`→ `wrap-session/report` — 2 chunks, 7 links

**Both ends cited.** Producers: 5 `implement/fix-loop` records and 2 `implement/smoke` records across
`probed` and `cause-closed`, outcomes `ok`/`soft-exit`, carrying `produced[].signals` of `surfaced`,
`stuck` and `green`. Consumer: `wrap-session/report` recorded `consumed[].quality = thin` on
`implement-outcome` in both chunks, with the verdict note in each naming what stood in instead.

**Chain hypothesis:** implement's P4 report is **systematically not the final word on its own chunk** when
operator relays land after it. In `probed`, a directive arriving between implement's P4 report and wrap's
report changed three things (a capability's resolution from open to UN-CLAIM, a residual re-wording, and the
re-attribution of a mid-implement commit from a process defect to an operator act). In `cause-closed`,
implement soft-exited and **three operator relays plus two operator commits** landed after its P4 report.
Both times the consumer's repair was the same and it worked: name the basis explicitly in the report's
Outcome-basis slot as "implement's P4 report AS AMENDED by that directive", so the fan-out detectors cannot
inherit a framing the operator had already replaced.

**Why the producers look fine:** a formally-`ok` producer with honest signals is exactly the shape this
heuristic is built to find — nothing was wrong with either implement run. The staleness is structural, and
one problem-fact in the epoch names the mechanism directly: *"The run crossed a COMMIT boundary
mid-implement. The capability's acceptance needs a reading from a CI run of committed code, and CI fires only
on push, while this skill's constraints reserve committing for wrap."* The operator performed the commit and
the push, and the run resumed. So a capability whose acceptance requires CI evidence **cannot be discharged
inside one implement run** as the skills are currently bounded.

**Proposal direction:** the report-side repair is already sound and is now used twice, so the candidate is to
promote it from improvisation to contract — an explicit Outcome-basis slot that must name every post-P4
amendment source. The deeper direction is the commit boundary: either a sanctioned mid-implement
commit+push step for CI-consuming acceptances (the epoch's own operator directive went the other way and
added a standing prohibition — "the skill must not commit and the plan must stop asking it to" — with
commit+push named as an operator step before any CI-consuming entry), or an explicit chunk-shape rule that a
CI-consuming acceptance always spans two runs. The prohibition resolved the immediate ambiguity; whether it
is the intended long-run shape is the founder's call.

### X2 — `wrap-session/report` →`report`→ `wrap-session/reconcile` — 2 chunks

**Both ends cited.** Producer `wrap/report` in `probed` and `cause-closed`, outcome `ok`, signals
`co-owner-found` / `insufficient-fix-recorded`. Consumer `wrap/reconcile` recorded `thin` on `report` in
both, plus two `input.report-insufficient` frictions across the same two chunks.

**Chain hypothesis:** the report's Expected-amendments **site set systematically under-runs the true set**,
and the gap is closed by the dependent-of sweeps rather than by the report. In `probed` the direction was an
**over-claim** instead of the usual gap — the report named `a11y-plan.md §9` as co-owner of a fact that doc
does not state (`grep -ci diagnostic` → 0), which would have produced a hand-raised amendment against a doc
saying nothing. In `cause-closed` the set under-ran the true set by 7 (2 named vs 10 applied) **even after**
its own pre-fanout re-derivation had corrected a sole-owner claim to two owners; five of amendment 1's seven
sites carry no `always-latest` token at all, and `test-plan §6` needed the dependent-of sweep for the second
consecutive chunk.

**Proposal direction:** the sweeps that close the gap work by reading for the claim's **meaning**, while the
report authors by its **tokens** — which is the same mechanism as P3 and P4, one step later. Direction: have
the report's Expected-amendments step state, per bullet, the derivation it ran (semantic sweep vs token
grep), so reconcile knows which bullets are token-derived and therefore likely incomplete. Both directions
of error appear here, so a bullet needs its basis recorded, not just its site list.

---

## Level candidates (systemic-masked-as-project)

### L1 — chronic-degrade — `tooling.host-shell`, 6 consecutive epochs, zero halts ever

**Facts:** per-epoch counts `[0, 0, 14, 13, 3, 4, 8, 10]` (Epoch 1→this epoch); 10 this epoch across **all
five chunks plus orientation**; summed impact 6 retries + 5 extra_reads; **no `halted` or `soft_exit` in any
epoch.** Correlate: Pass-A theme **T1** (n=6 problem facts — a heredoc-to-file blocked twice, LF→CRLF
corruption twice, an unset `$TMPDIR`, a script written to the scratchpad per the host rule), natures
`process` and `environment`.

**Level hypothesis:** the cause lives in the environment (the Windows/MSYS↔native boundary and the Bash
tool's transport), while every fix so far has landed in the project — a rule-file clause, a corpus entry, a
handoff note. The halt policy structurally never surfaces it: each event costs one retry or one extra read
and resolves inside the step, so no gate ever goes red and the class has accumulated for six epochs at a
steady rate. That is the definition of silent degradation, and the count is not falling as the corpus grows
(the corpus named 5 of this epoch's 10 before they fired).

**Proposal:** treat the recurring subset as **environment to be fixed, not knowledge to be written** — the
two forms with a single correct spelling (python file opens; scratchpad paths) are wrapper candidates, and
the `| head`/`| tail` exit-and-coverage traps are string-detectable and therefore PreToolUse-guard
candidates. The guard that already exists for heredoc-to-file fired correctly twice this epoch, which is the
only intervention in the class with a measured catch.

### L2 — override — the letter of a wrap rule cannot express the true state; the operator supplies the disposition

**Facts:** 5 `overridden` problem facts, 4 of them `nature: process`, 3 at wrap steps — plus the typed
`contract.no-sanctioned-channel` at **n=2 in-epoch and n=2 in the prior epoch** (threshold met by the
recurrence clause; the type exists in no earlier epoch):
- `wrap/gates` (cause-closed): *"the light gate's ASSERT passed on a RECORDED DISPOSITION rather than on a
  green: `cargo nextest --workspace` is red on a pre-existing defect that landed in `b3b51e6` with zero `.rs`
  delta in this chunk. **The letter of the gate cannot express 'red, attributed elsewhere, owner minted'**;
  the operator directive supplied the disposition. **THIRD such wrap in four days.**"*
- `wrap/route-resolve` (ledger-gate): *"the `a11y-plan:115` dittography is a master-body defect with no
  detector and no report fact, so P2 had no channel for it; the operator directive supplied one by naming
  both the owner entry and the disposition (CARRY, explicitly not a residual, which would leave it
  ownerless)."*
- `wrap/route-resolve` (probed): re-worded an EXISTING dispositioned `residuals.md` entry, which that file's
  own header scopes to route Phase A rather than to route-resolve — done on explicit operator directive.
- `wrap/report` (probed): the operator directive settled a capability's resolution as UN-CLAIM where
  implement had surfaced it as refine-or-unclaim.
- `implement/code` (cause-closed): `/implement` modified `plan.md`, which the skill's MUST NOT forbids;
  the operator directed it explicitly and named the precedent channel.

**Level hypothesis:** the cause appears to live in the **rules' expressible vocabulary**, not in the project.
Three of the five are not the operator correcting a wrong call — they are the operator supplying a
disposition the rule has no way to state: a gate that can say green or red but not "red, attributed
elsewhere, owner minted"; a route step with no channel for a master-body defect that no detector owns; a
residuals file whose writer-scoping blocks the edit the situation needs. `contract.no-sanctioned-channel`
firing twice in each of two consecutive epochs is the same finding arriving through the typed stream. Per the
reference's signature 4, recurrence on the same rule means the **rule** is miscalibrated.

**Proposal:** the highest-value single direction in this diagnosis — give the light gate a third verdict
besides green and red: **attributed-elsewhere, with a named owner**, so a red the chunk did not cause is
expressible without an operator ruling (three wraps in four days needed exactly this). Secondarily, the
route step needs a sanctioned channel for a master-body defect with no detector and no report fact —
currently CARRY-vs-residual is decided by operator directive each time, and the one wrong choice
(residual) is explicitly the one that leaves the defect ownerless.

### L3 — band-aid — the route files' own grammar overflows the tool-result cap, absorbed session by session

**Facts:** Pass-A theme **T2** (n=3, natures `resources`/`environment`/`process`) — a `sed` range read of
master-route exceeded the cap at 59.3 KB and was persisted to a file; a marker+status grep returned 30.7 KB
and overflowed; a working-route read was taken as a whole-file `cat` rather than the prescribed scoped
extraction. Typed correlate `tooling.output-cap-overflow` n=2 in-epoch, per-epoch `[0,0,0,2,5,2,10,2]`.
Cross-epoch lookback shows the same absorption in **every epoch since Epoch 2** — a 33 KB working-route read
and a first grep both overflowing (Epoch 2); a 96-line file no longer fitting one Read, a 71 KB `cat`, a
70 KB bundle needing 5 chunked re-reads (Epoch 3); a 72.4 KB `cat` recovered by an awk structural pass
(Epoch 4); a 29.7 KB master-route grep (Epoch 6b).

**Level hypothesis:** the cause lives in the artifact format — route entries are multi-KB single lines by
design, so any whole-record read of a mature route overflows — and the fixes have all landed in the *reader*,
one session at a time. The skill body has since absorbed the workaround into its own text (it now prescribes
structural extraction and names this exact overflow), which is the band-aid becoming doctrine: correct, and
still re-derived by hand every session, with three instances in this epoch alone.

**Proposal:** the derivation needs three facts (the last `complete` marker, the epoch headers, the first
markerless entry) and none of them requires entry bodies. Direction: a tiny pipeline tool in
`andromeda-tools/` that prints exactly those three, so the cursor derivation stops being an ad-hoc
extraction each session — which would also close most of L4 below, since every route-parse token-proxy
failure this epoch was in a hand-rolled extractor.

### L4 — band-aid — hand-rolled structural extractors vs the route's documented grammar

**Facts:** this is L3's sibling and the evidence is the `new-session/orientation` half of **P3**: an
asterisk-vs-underscore italic-preamble test resolved the cursor to line 3 instead of line 17; a guessed
bullet-list prefix returned 0 master-route records. Cross-epoch, the same shape recurs in almost every
epoch — a guessed list-item format returning an EMPTY tail read as version-complete (Epoch 4,
`contract.schema-assumed`); the separator anchored `/^↓/` against a three-space-indented grammar, labelling
all 47 separators as entries (Epoch 6a ×2, Epoch 6b ×2); the italic marker again (Epoch 5); a `BLOCKED-ON`
token grep hitting backtick-quoted prose 2793 characters into a 3001-character line (Epoch 6b). **The
bullet-prefix guess specifically has now occurred three times** — Epoch 4, this epoch, and once more at the
session start that ran this diagnosis.

**Level hypothesis:** the cause lives in the **absence of a parser**, not in any session's care. The route
grammar is documented in the file's own preamble, and each session re-implements it in awk/grep from that
prose; every failure is a different clause of the same grammar being guessed. The project's response has
been to add more grammar detail to the skill body, which is why the failures keep moving clause to clause
instead of stopping.

**Proposal:** same direction as L3, and the two would be fixed by one artifact — a single route-parsing tool
that owns the grammar once. Note for the founder: this is the clearest case in the diagnosis of a *pipeline*
cause being absorbed as a *per-session* cost, and its recurrence spans every epoch in the ledger.

### L5 — band-aid — a written prescription does not fit the measured reality; narrowed in flight

**Facts:** Pass-A theme **T5** (n=6, natures `process`/`environment`/`product-logic`): the smoke's prescribed
`agent-run.sh run` would have voided a legitimate deferral, so the already-green P2 entry was recorded with
the substitution stated; the flip-compaction's one-whole-file-Write rule met a one-line change and a single
anchored Edit was used; a new rustdoc gate baselined RED on five pre-existing errors, so the gate's asserted
signal was narrowed and a route candidate named as owner; a plan step prescribed "at least three incidents"
where `StubConfig` serves 0 or 1, so the kill was implemented at ONE incident where the mutation still
discriminates; a crate-root re-export absent from the plan's touchpoints was edited as a transitive
requirement; a one-line pointer became three comment lines so it would not sit beside a contradiction the
chunk had just disproved.

**Level hypothesis:** every one was executed well — read the convention first, state the substitution, record
the deviation — and every one is a *written prescription* meeting a reality it did not model. Two of the six
are authoring defects P2 could plausibly catch (the stub's missing capability; the touchpoint list's missing
transitive requirement); the other four are rules stated at a grain that cannot express a legitimate
exception (a one-line sweep under a whole-file rule; a smoke verb that voids a deferral; a gate inheriting a
pre-existing red).

**Proposal:** the four rule-grain cases share L2's shape — a rule whose letter has no way to say "this case,
for this stated reason" — and the cheap direction is the same: let the rule carry its own exception clause
rather than relying on the executor to state a substitution each time. The two authoring cases belong with
P2: a plan step that names a capability (a stub knob, an argument count) is mechanically checkable against
whether that capability exists, and `implement` recorded that *"P3 never checked whether the capability
existed."*

### L6 — observation — a recurring `removed-cause`: a sweep result authored before the sweep is run

**Facts:** Pass-A theme **T7** (n=3 `removed-cause` facts): a measurement error corrected in the chunk's own
evidence before commit; the report's false ownership claim corrected at Validate check 5 before it could
become a hand-raised amendment against a doc stating nothing; a "not-to-re-derive" freight verified anyway,
with four of five coordinates reproducing exactly and the fifth refined. Typed correlates: **P4** (n=6) and
**P6** (n=3).

**Observation (not a level verdict):** per Stage 4 Pass A, a recurring `removed-cause` theme means the same
cause keeps returning despite removal. It is removed *successfully* every time — all three caught before any
consumer read the claim — which is why it produces no halt and no failed gate, and why it has recurred for
six epochs. The removals are working; the cause is not being removed at its source.

### Deferred-forever — **does not fire**, and the check is recorded

All 5 `deferred` facts in the epoch name a destination, which per Stage 4 signature 3 makes them routed
deferrals rather than orphans: the `:4444` tauri-driver bind race → *"recorded with mechanism and coordinates
for v3-02 or its own entry"* (and `residuals.md` carries it as `absorbed:v3-02`); a stale `observed_gap`
field → *"routed into the plan's Implementation notes as an instruction for /implement"*; two reading
follow-ups → both **measured by the successor chunk** (the probe was moved into the live window, elevation
was varied); the workspace nextest gate → the source-delta-proportional pin, designed behaviour.

One caveat stated rather than buried: the fifth — pre-existing `ci.yml` registration drift (`$env:TEMP` at
`:396/:440`, `$env:LOCALAPPDATA` at `:490/:501` appearing in no architecture doc) — was *"left as an Expected
amendment for wrap"*. No later ledger record names its closure, and the amendment sidecars are **not an input
to this skill**, so its closure is *unverified here*, not absent. Worth one founder glance; it is not a
deferred-forever hit.

---

## Playbook-extension candidates (untyped patterns, F-4)

### U1 — plan authoring · 2 cases (1 in-epoch + 1 recurring from Epoch 6b) → proposed type `contract.jointly-contradictory-instructions`

**Cluster:**
- *this epoch, `implement/code`, cause-closed:* *"inserting arm C as step 5 created a jointly-contradictory
  pair the individual steps did not show: step 4 said 'leave every other step of the a11y job untouched'
  while step 5 edits the install step. Caught at the numbering re-check and resolved by excepting the install
  step in step 4 — the same class CLAUDE.md 2026-09-04 records, recurring at an insert rather than at
  authoring."*
- *Epoch 6b, `halo-hue-budget-re-driven` (also untyped):* *"Two plan steps were individually correct and
  jointly inconsistent: step 1 NAMED the 1/s jitter beat … and then chose 1/s anyway, while step 6's
  mechanism pin — authored later — is exactly what that beat breaks, spuriously in ~1–5 % of runs. Nothing in
  the mechanical checks or validation catches it."*

Meets F-4 by the recurrence clause (n ≥ 2 recurring from the previous epoch). Both records are untyped, both
concern a plan, and both state that no existing check looks for the property — the project corpus already
records the class (CLAUDE.md, 2026-09-04: *"a plan's steps can each be individually unambiguous and jointly
contradictory, and no gate looks for that"*), but the friction vocabulary has no type for it, so each
instance lands as untyped and cannot aggregate.

**Draft criteria line** (for the `phase/plan`, `phase/validate` and `implement/code` playbooks): *"Two or
more instructions that are each individually executable proved jointly unsatisfiable — one step forecloses
what another requires, or a later insert contradicts an earlier scope clause; record which pair, what
exposed the conflict, and whether any mechanical check attempted it."*

Note the two instances differ in *when* the contradiction was introduced — one at authoring, one at a
mid-run insert — which the criteria line should keep distinguishable, since an insert-time conflict argues
for a re-check after any step insertion while an authoring-time one argues for a validate predicate.

---

## Below threshold — no action

**Typed groups (28):** `input.extracts-conflict` n=2 (phase/plan — both were extract-vs-extract or
extract-vs-matrix tensions that P2's aggregate check C structurally cannot see, since neither extract
declares a binding on the other) · `retry.synthesis-rework` n=2 (phase/plan) ·
`tooling.output-cap-overflow` n=2 (folded into L3) · `input.report-insufficient` n=2 (folded into X2) ·
`ambiguity.filter-borderline` n=2 (wrap/curation) · `contract.extract-format` n=2 (phase/distill — one is
that fan-out's six per-extract checks cover no absolute-host-path case though run-dir extracts are tracked
files) · `tooling.result-not-run-stable` n=2 (implement/fix-loop) · `contract.skill-reference-drift` n=2 ·
`ambiguity.scope-pressure` n=1 · `retry.fix-iterations` n=1 · `contract.false-positive-proposal` n=1 ·
`contract.vacuous-check-found` n=1 each at implement/code, implement/fix-loop, implement/smoke ·
`retry.distiller-respawn` n=1 · `retry.query-reformulation` n=1 · `recall.curated-rule-not-applied` n=1 ·
`input.handoff-git-mismatch` n=1 · `recall.anomaly-classification` n=1 · `contract.spec-reality-gap` n=1 ·
`input.implement-outcome-unsettled` n=1 (folded into X1) · `contract.no-sanctioned-channel` n=1 at
wrap/route-resolve and n=1 at wrap/gates (grouped and above threshold in L2) · `tooling.gate-deferral` n=1 at
wrap/gates · `input.working-entry-thin` n=1 · `input.research-files-wrong` n=1 · `input.plan-step-ambiguous`
n=1 · `contract.proposal-format` n=1.

**Untyped clusters (4 below F-4):**
- *A probe's subject is bounded by its POSITION in the step list* — *"Probe (2) returned no reading because of
  WHERE its step sits… nothing at planning time modelled step adjacency as a precondition."* n=1; no prior-epoch
  sibling found. Emerging — watch next epoch.
- *The evolve reading-order rule made a definition needed DURING the step unreadable until after it* —
  the `graph-not-applicable` distinction is defined only in `references/evolve/research.md`, which the
  reading-order rule forbids opening until the checkpoint. n=1. Named explicitly because it is a *pipeline*
  contract conflict rather than a project one, and the reading-order rule is load-bearing by design.
- *The fan-out transport escapes per-RETURN, not per-batch* (6 of 7 extracts escaped, the 7th clean) — a
  refinement of the well-typed `contract.extract-format` class (5 records in Epoch 6b), not a new class. n=1.
- *A refuted hypothesis recorded as a negative* (elevated-shell PATH tested and refuted) — per
  `evolve-system.md`, a measurement of a mechanism is a `produced[].signals` fact, so this reads as a
  mis-filed positive rather than friction. n=1.

**Pass-A themes below threshold (3):** T6 *a false-positive-prone hygiene gate is satisfied by changing the
OUTPUT, never the pattern* (n=2; folded into P3, where the same anchor accounts for 2 of 4 false positives) ·
T4 *a step executes outside its documented repertoire rather than assert from memory* (n=2 — a live
PowerShell feasibility probe outside P3's repertoire; three candidate gate commands executed during P4,
which changed what one gate entry asserts) · T8 *a guard added so a future run cannot discharge an undecided
outcome by wording* (n=2 — the commit prohibition and "Do not force the v3-01 claim"; both fold into X1).

**Problem facts with no theme sibling — explicit dispositions, no change proposed:**
- `implement/code` (probed): diverged from the neighbouring script's `ErrorActionPreference Stop` convention
  because an always-exit-0 diagnostic cannot use a terminating error preference — read the convention first
  and recorded the reason at the line. Correct, singular, nothing to generalize.
- `implement/fix-loop` (probed): the run crossed a commit boundary mid-implement. Not dropped — it is the
  mechanism evidence inside **X1**, where its real weight lies.
- `implement/code` (cause-closed): a warning comment added above the param block because `[CmdletBinding()]`
  is load-bearing and its removal degrades the selector silently. Admissible comment discipline, n=1.
- `phase/research` (p-025): a prohibition written into scope directing that the contract cite both named
  SUT-side fixes as insufficient and never present either as satisfying it. n=1; the durable finding shipped
  in the contract.
