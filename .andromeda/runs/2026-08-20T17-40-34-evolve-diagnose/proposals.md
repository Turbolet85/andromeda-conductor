# Evolve Diagnosis — Conductor · Epoch 3 "Live proof: the five families" · 2026-08-20T17:40:34Z

Target epoch chosen in dialogue (latest fully-complete). Everything below is **obligation-free**:
accept, reject, defer, or modify any item with no mechanism-side consequence. Nothing here has been
applied, queued, or remembered — a re-run recomputes from the ledger alone.

---

## Mechanism health

| | |
|---|---|
| Records | **296** (159 step / 137 friction) across **11 chunks** |
| Unparseable lines | **0** |
| Step-record coverage | **every chunk fired all 13 checkpoints** (phase 5 · implement 3 · wrap 5). Two chunks ran extra implement iterations (`canary-spans` 4, `fingerprint-storm` 5) |
| new-session step records | 10 (`chunk:null` by design) |
| Untyped rate | **26 / 137 = 19.0%** — concentrated at `new-session/orientation` (8/15 = 53%), `wrap-session/route-resolve` (3/8), `wrap-session/curation` (3/8), `phase/validate` (3/10) |
| Problem-fact fill | 60 / 159 step records carry ≥1 fact — **77 facts** total (46 workaround · 9 deferred · 8 removed-cause · 7 prohibition · 6 overridden · 1 unresolved) |
| id fill | post-2026-08-18: **106/106 (100%)** · pre-boundary: 190 records, 0 — era, not a gap |
| Outcomes | ok 147 · ok-degraded 6 · halted-resolved 6 · soft-exit 0 · aborted 0 |

**Retraction pre-pass** (whole ledger, 752 records): 1 retractor · **0 records retracted** · 1 unresolvable.
The sole retractor is the **pre-2026-08-18 prose form** (ledger line 12, `id: null`, `retracts` carried as a
prose string rather than the schema'd list), authored 2026-08-08 in `2026-08-08-sut-capability-manifest`.
Unresolvable **by design** per the calibration boundary; reported here verbatim for manual discount:

> *"the untyped code-graph-under-reports record and the first problem-block entry on the plan step record,
> both appended earlier at this step"* — retracting a claim that the code-graph calls view under-reports
> cross-crate call sites (the `rows=1` belonged to a different query; a `head -30` had cut 41 rows to 10).

It targets **Epoch 1** records, so Epoch 3's exclusion sets are empty and every stage below ran on the
unfiltered target stream. No retraction-of-retraction.

**Calibration boundaries in range.** The deviation scan is live for all Epoch-3 records. The **Universal
types were deployed at the Epoch-3 boundary** (`contract.premise-falsified` first 2026-08-15T21:54:16Z,
`contract.narrow-basis-claim` 2026-08-15T22:45:44Z, `contract.structural-blind-spot` 2026-08-15T22:16:23Z,
`tooling.host-shell` 2026-08-16T12:21:01Z) — their absence in Epochs 1–2 is **era, not evidence**, and
§L7 re-derives those mechanisms by note-search to avoid reading the deploy as a regression.

### ⚠ Two normalization axes were required, not one

`diagnosis-pass.md` documents the **dash fold** for epoch labels. It was needed: Epoch 3 carries 25
ASCII-hyphen labels and 271 em-dash labels, and an exact-match group-by buckets one epoch as two.

A **second, undocumented axis** was also load-bearing: the `skill` field switched convention mid-epoch —
`andromeda-phase`/`andromeda-implement`/`andromeda-wrap-session`/`andromeda-new-session` up to
2026-08-17T22:22:43Z, then bare `phase`/`implement`/`wrap-session`/`new-session` from 2026-08-18T17:43:58Z.
A clean cutover: no chunk mixes both. Unfolded, this produced **five phantom zero-coverage chunks**
(every 2026-08-18-onward chunk read as "0/5 phase, 0/3 implement, 0/5 wrap") and split every
`(skill, step, type)` group into two sub-threshold halves — `implement/fix-loop` alone appeared as
14 + 9 rather than 23. Both artifacts were caught only by re-derivation.

*Observation, not a proposal (Stage 0 is observations-only): the mechanics reference names one fold axis;
this epoch needed two, and the second one silently manufactures both false gaps and false sub-threshold
counts.*

---

## Proposals (typed patterns)

Universal types are grouped **by type alone across steps** per the mechanics; their per-step splits are
shown inside each proposal rather than as separate entries, so nothing is double-counted.

### P1 — Universal · `tooling.host-shell` — 14 cases · weight 35 · 8 steps · 6 chunks
**Pattern:** the Windows/Git-Bash command transport corrupts, truncates, or mis-decodes commands and their
output across five distinct mechanisms; every occurrence was absorbed by substituting a different tool.

Per-step split (invisible to per-step grouping): `phase/distill` 5 · `new-session/orientation` 3 ·
`wrap-session/route-resolve` 1 · `wrap-session/reconcile` 1 · `phase/research` 1 · `phase/validate` 1 ·
`implement/fix-loop` 1 · `implement/smoke` 1.

**Evidence — all 14 cases:**

| chunk | what | impact |
|---|---|---|
| — (orientation) | `TMPDIR` unset in Git Bash → scratch redirect expanded to a root-absolute path, permission denied; fell back to a project-root temp file | — |
| — (orientation) | Bash tool cwd persists between calls → a second call's relative `cd` failed and its whole body was skipped; re-issued absolute | retries 1 |
| — (orientation) | friction log's own epoch key splits Epoch 3 across two label strings (25 ASCII-hyphen vs 82 em-dash); any diagnosis grouping on epoch buckets one epoch as two | extra_reads 2 |
| fingerprint-storm-live-proof | inline `python -c` died `UnicodeEncodeError` — host stdout defaults to cp1252 and the segment carried a downward-arrow separator | retries 1 |
| — (route-resolve) | printing U+2192 raised `UnicodeEncodeError` though the file is valid UTF-8 opened as UTF-8; diagnostics forced through `ascii()`, the edit moved to a script file | reformulations 1, extra_reads 1 |
| error-baseline-spike-live-proof | `bash -c` with ~10KB+ quoted heredocs arrived truncated (unexpected-EOF at content-proportional offsets); **only the smallest of 7 extract writes survived, 6 re-written via Write** | retries 6 |
| restart-suppression-live-proof | a ~13KB single-quoted heredoc hit a parse failure mid-content — the heredoc start line was never honored | retries 1 |
| restart-suppression-live-proof | a pipeline `tail` captured its own exit instead of `cargo audit`'s; plus unset `TMPDIR` → `/audit-probe.txt` permission denied | retries 2 |
| restart-suppression-live-proof | persistent-shell cwd had moved into `.andromeda`, so one validate-sweep grep round silently matched nothing via relative globs | retries 1 |
| pii-scrub-live-proof | quoted heredocs carrying 7–9KB bodies failed with a quote-parse error at heredoc depth; identical content wrote cleanly via Write | retries 2 |
| pii-scrub-live-proof | a quoted python heredoc arrived with a double backslash collapsed to one → `unicode-escape SyntaxError` | retries 1 |
| connection-lifecycle-live-proof | `uv_spawn` length ceiling: a 3-heredoc command failed `ENAMETOOLONG`; two split ~10KB heredocs truncated mid-heredoc | retries 2, reformulations 1 |
| latency-regression-re-proof | ~12KB heredoc failed to parse where an ~8KB one had succeeded — length or CRLF mangling on the win32 Git Bash bridge | retries 1 |
| latency-regression-re-proof | a Windows-form absolute path prepended to `PATH` is split on the drive-letter colon → sidecar resolved nowhere → `preflight blocked`, **indistinguishable at row level from a genuine SUT-side gate failure** | retries 1 |

**Proposal:** the dominant sub-mechanism is **large heredoc payloads through `bash -c`** (6 of 14 cases,
12 retries, always resolved the same way — switch to the Write tool). A direction: state a payload ceiling
in the shell-discipline reference and route extract/artifact persistence to the Write tool **by default**
rather than as the documented fallback, so `phase/distill` stops paying the same 1–6 retries per chunk.
The remaining four mechanisms (cp1252 stdout · cwd persistence · unset `TMPDIR` · Windows-path `PATH`
splitting) are separable and each already has a known-good form. Note the last case is a **safety** item,
not just cost: a `PATH` resolution failure and a real SUT gate failure are indistinguishable at row level.
See §L1/§L2 — this type's problem-facts also constitute the epoch's largest band-aid themes.

### P2 — Universal · `contract.premise-falsified` — 16 cases · weight 21 · 5 steps · 6 chunks
**Pattern:** an authored artifact's stated premise (route entry, scope, plan step, matrix acceptance,
spec section, or an operator-dictated coordinate) was falsified by verification against the real artifact
or the live SUT — repeatedly, and in every phase of the loop.

Per-step split: `phase/research` 5 · `implement/code` 4 · `implement/smoke` 3 · `phase/take-up` 2 ·
`implement/fix-loop` 2.

**Evidence — all 16 cases:**

| chunk | step | what | impact |
|---|---|---|---|
| canary-storm-autonomous-band | take-up | directive located a six-item live recipe in the frozen CARRY, which enumerates five and labels itself FIVE; the sixth lives in `rules/verification-harness.md` | extra_reads 1 |
| canary-storm-autonomous-band | smoke | the chunk's central premise — the tier band was the LAST blocker — disproved by its own live leg: the raised count reached Pulse intact yet zero fingerprints tracked across 13 ticks | — |
| canary-spans-pulse-fingerprints | research | an operator-verified coordinate carried a correct mechanism but over-strong inference: `record_feed_counts` sits after the `is_empty` early return, so `0` is the never-written state | extra_reads 10 |
| canary-spans-pulse-fingerprints | fix-loop | plan step 6 said the identity goldens must be re-locked; they passed unchanged — both `dispatch_wire` snapshots are driven by the Exception shape | extra_reads 2 |
| canary-fingerprint-derivation-aligned | research | the working entry, `scope.md` and `architecture.md` all state aligning the derivation lets the canary's last precondition pass; `fingerprint_refs` actually carries the L4 model's `evidence_refs` | extra_reads 6 |
| fault-application-spans | research | obs-plan §4 places the three fault spans in `conductor-faults`, but that crate has 0 edges in either direction — a span there would fire on no scenario run | extra_reads 3 |
| fingerprint-storm-live-proof | take-up | matrix v2-11's acceptance states the identity triple resolves to one fingerprint; measurement falsified it for the triple as declared | deferred 1 |
| fingerprint-storm-live-proof | research | the directive's item 1, labelled VERIFIED and planning-shaping, concluded the family leg fails as declared; every component fact confirmed, the **conclusion** falsified | extra_reads 4 |
| fingerprint-storm-live-proof | research | three dictated citations drifted: `UNBACKED_AUTO` holds nine not ten; the recipe is five not six; three Pulse storm log-line citations sit 1–3 lines off | extra_reads 2 |
| fingerprint-storm-live-proof | smoke | the authored `[[expected]]` token checks are unverifiable under deterministic L4 — read-back served `degraded_mode: true` | deferred 1 |
| fingerprint-storm-live-proof | smoke | the `<20s` slo_tier is structurally unattainable for this shape — the storm's own emission window is ~24s | deferred 1 |
| fingerprint-storm-live-proof | fix-loop | the v2-11 `>60s` separation clause measured INSUFFICIENT — Pulse dedupes against any OPEN incident, not by fingerprint | — |
| restart-suppression-live-proof | code | plan step 2 + the concretized v2-13 acceptance rest on persistence-as-spike-duration; Pulse computes `persistence_seconds` = cumulative service samples | dialogue 1, reformulations 1 |
| restart-suppression-live-proof | code | the relative-magnitude bypass arm is unreachable at SUT defaults — `alpha_short/alpha_long = 10` equals the bypass multiplier | reformulations 1 |
| restart-suppression-live-proof | code | preflight canary and dispatcher shared `DEFAULT_SERVICE_NAME`, so the redesign's fresh-service premise was false | reformulations 1 |
| restart-suppression-live-proof | code | the acceptance names bypass labels `absolute_error_rate`/`relative_magnitude`; Pulse emits `absolute`/`relative` | extra_reads 1 |

**Proposal:** this is the epoch's highest-count type and it is **working as designed** — every case is
verification catching a false premise before it shipped, which is the mechanism's purpose. The proposal is
therefore not "reduce it" but **shorten the distance to it**: 6 of 16 were falsified at `implement/code` or
later, after a plan had already been authored and approved on the false premise (the `restart-suppression`
cluster of 4 is the clearest — four premises in one chunk, all disproved pre-write). A direction: give
`phase/research` an explicit *premise-inventory* obligation — enumerate the load-bearing premises the plan
will rest on and mark each verified-against-artifact vs assumed — so the falsification lands at P3 where it
is cheap, not at P6 where it forces a redesign. This connects to §X1 (research→implement is the epoch's
strongest chain shape).

### P3 — `implement/fix-loop` · `contract.spec-reality-gap` — 8 cases · weight 14 · 7 chunks
**Pattern:** during the fix loop, a spec/plan/matrix statement is measured against the shipped code or the
live SUT and found to describe something that does not hold — surfaced for wrap rather than hot-fixed.

**Evidence — all 8 cases:**

| chunk | what | impact |
|---|---|---|
| canary-fingerprint-derivation-aligned | adopting Pulse's derivation made the triple fail on PathVariant only — Pulse's `normalize_frame` strips ABSOLUTE paths alone, and Conductor's frames are relative by construction | iterations 1 |
| fault-application-spans | obs-plan §6's log-level table assigns `debug` to fault application, but the three fault spans shipped at INFO like every sibling; at DEBUG they would be invisible | — |
| fingerprint-storm-live-proof | the plan's chosen mechanism (reshape PathVariant to an ABSOLUTE path) is false — the base carries a RELATIVE path whose leading segment survives normalization | iterations 2 |
| fingerprint-storm-live-proof | `degraded_mode` is PERMANENTLY true under deterministic L4, so the Contains/Absent read-back checks were **structurally ungradeable rather than merely failing** | iterations 1 |
| error-baseline-spike-live-proof | the concretized v2-12 acceptance's latency half disproved by two live measurements (canary auto-resolve; long-window t-digest p99 absorbs the ramp) | iterations 1 |
| restart-suppression-live-proof | `cues_suppressed`/`bypass_triggered` read `<redacted>` live — Pulse's default-deny allowlist predates the fields, so acceptance and harvest predicate were built on an unreadable surface | reformulations 1 |
| connection-lifecycle-live-proof | the v2-15 acceptance sub-clause "with a recovery transition" is unsatisfiable — bind status is per-process, recovery is a process replacement | deferred 1 |
| latency-regression-re-proof | `max_sustained_rate_spans_per_s` is computed `occurrences/gap_ms` = DISPATCHES/s while a latency/ramp phase emits many spans per dispatch — a 50× divergence at the shipped shape | — |

**Proposal:** the surfaced-not-authored discipline is clearly working (every case routed to wrap intact).
The pattern worth the founder's eye is *where the gap lives*: 5 of 8 are **matrix acceptance text** written
before the measurement existed. A direction: allow the P5 concretization step to mark an acceptance clause
`unverified-premise` when it rests on something no chunk has yet measured, so fix-loop is confirming a
flagged risk rather than discovering it.

### P4 — Universal · `contract.narrow-basis-claim` — 7 cases · weight 11 · 5 steps · 4 chunks
**Pattern:** a count, absence, or availability claim was made from a source narrower than the claim, and
would have stood unnoticed if not re-derived.

**Evidence — all 7 cases:**

| chunk | step | what | impact |
|---|---|---|---|
| canary-storm-autonomous-band | report | claimed the context budget nearly exhausted and proposed halting the wrap **from impression**; the operator measured 492.9k/1m used, 506.4k free — would have cost a session boundary | dialogue 1 |
| fingerprint-storm-live-proof | distill | the layouts extract derived the unbacked qualifier "9→8" from its own document's sample caption while the scope's CARRY records 10 | deferred 1 |
| fingerprint-storm-live-proof | research | the directive named one stale source twin; research found **four** sites carrying the same falsified claim — the enumeration was narrower than the defect | extra_reads 1 |
| fingerprint-storm-live-proof | research | claimed the operator recipe is FIVE items from the working-route entry alone; the operator corrected to SIX — the accumulated chain lives in `rules/verification-harness.md:47` | extra_reads 2, dialogue 1 |
| fingerprint-semantics-token-leading | fix-loop | the obs hygiene criterion read as met (0 host-path matches) but the leg took the Blocked spine and **emitted nothing** — a green whose producer never ran | — |
| fingerprint-semantics-token-leading | route-resolve | a bare `grep -c` for the audit PREREQ returned 6, reading as six live entries; splitting by freeze state showed ALL six are frozen historical pins and the markerless tail carried none | extra_reads 1 |
| latency-regression-re-proof | route-resolve | a dictated route cross-reference ("first Epoch-4 markerless entry carries BLOCKED-ON") did not hold — BLOCKED-ON sits on the third entry | extra_reads 1 |

**Proposal:** two recurring sub-shapes are mechanical and cheaply guarded — (a) **a bare `grep -c` over a
route file counts frozen and live entries alike** (2 cases), and (b) **a dictated cross-reference is a
hypothesis** (2 cases). A direction: add a route-counting idiom to the route-resolve reference that splits
hits by freeze state, and make "re-verify dictated coordinates against the artifact" an explicit take-up
obligation rather than a per-chunk scope guard (it was authored by hand as a guard twice this epoch — see
the `prohibition` facts in §L-appendix). *This diagnosis hit shape (a) itself: an initial `tail`-mechanism
count of E1=4/E2=5/E3=6 was over-counted by a loose regex and re-derived to E1=1/E2=1/E3=4.*

### P5 — `tooling.gate-deferral` — 8 cases · `wrap-session/gates` 5 + `implement/fix-loop` 3 · 5 chunks
**Pattern:** `cargo audit` exits 1 on a byte-identical external advisory-DATABASE parse fault
(`duplicate advisory ID: RUSTSEC-2026-0244`) at every gate, is classified a ratified standing deferral,
re-measured, and re-pinned. Presented as one proposal because both groups are the same phenomenon.

**Evidence — all 8 cases:** `canary-storm-autonomous-band` (18th, gates + fix-loop) ·
`canary-fingerprint-derivation-aligned` (22nd, gates) · `fault-application-spans` (23rd, gates + fix-loop) ·
`fingerprint-storm-live-proof` (24th, gates + fix-loop) · `fingerprint-semantics-token-leading` (fix-loop,
discharged by re-running both probes) · one `chunk:null` wrap (25th, moved onto a newly inserted entry with
origin preserved). Every case records `deferred: 1`, `cargo deny` verified true exit 0 across all four
classes as the named overlap, and the pin re-verified rather than echoed.

**Proposal:** the deferral is operator-ratified (2026-08-10) and the per-chunk handling is exactly what the
playbook prescribes — nothing is being done wrong. What the batch shows is **cost**: the same measurement,
classification, and re-pin ran 8 times in this epoch (33 consecutive chunks at epoch end) and its closure
condition — the upstream DB parsing again — is untestable from inside Conductor. A direction: a single
cheap probe (`cargo audit` exit + first error line) whose *unchanged* result auto-satisfies the pin without
re-authoring the rationale each chunk, leaving the full form for the moment the signature changes. See §L6.

### P6 — `phase/plan` · `ambiguity.scope-question` — 3 cases · weight 9 · 3 chunks
**Pattern:** the plan step halts into an operator dialogue round because the materials genuinely cannot
decide the question. All three are one round with bundled questions.

| chunk | what | impact |
|---|---|---|
| canary-fingerprint-derivation-aligned | two bundled questions: what the canary should assert once its designed carrier was proven unsatisfiable (architecture-touching; specs describe but do not decide), and whether to add the tr… | dialogue 1 |
| fingerprint-storm-live-proof | two questions: the variant re-shape (the operator's stated preference was formed under an arithmetic premise research falsified, so re-confirming was necessary not deferential) and the log-ha… | dialogue 1 |
| pii-scrub-live-proof | v2-14 disposition as a recommended-first fork (claim+reword vs notes-only vs stay pooled) — the matrix contract routes not-provable-by-any-chunk to the operator | dialogue 1 |

**Proposal:** no change indicated — each is a correctly-escalated decision, bundled to one round, and two
were forced by a falsified premise rather than by under-preparation. Recorded so the founder can see the
dialogue cost is 3 rounds across 11 chunks. If anything, the shape worth keeping is the **bundling**.

### P7 — `new-session/orientation` · `tooling.health-false-red` — 4 cases · weight 4
**Pattern:** a health check's probe flags a state that inspection shows is fine; every case passed on
inspection, and two cost an extra read.

| what | impact |
|---|---|
| Check 4 flagged `.claude/rules/security.md` as having no frontmatter; the file states it applies to all files and loads unconditionally with no `paths:` frontmatter — nothing exists to parse | — |
| Check 13's sh-executable clause reads git index mode `100644`; on win32 the exec bit is never set and MSYS execs by shebang | — |
| Check 4: the one-line grep heuristic flagged 5 rule files SUSPECT — valid `paths:` lists whose indented items tripped the pattern | extra_reads 1 |
| Check 13: index mode `100644` suggests a fail; `test -x` is true on-host via the shebang heuristic; index mode matters first at a Linux checkout | extra_reads 1 |

**Proposal:** two checks account for all four. A direction: make Check 4 pass explicitly on
*absent* frontmatter (the criterion is "parses as YAML"; nothing to parse is a pass, and one rule file is
deliberately unconditional-load), and make Check 13 evaluate `test -x` on-host with the git index mode
reported as a Linux-checkout note rather than a fail signal. *This diagnosis's own session hit Check 4 a
fifth time — recorded in Epoch 4.*

### P8 — `implement/fix-loop` · `tooling.environmental` — 3 cases · weight 4
**Pattern:** an external supply-chain state change lands against an untouched tree mid-chunk.

| chunk | what | impact |
|---|---|---|
| canary-storm-autonomous-band | `cargo audit` exit 1, 18th consecutive, byte-identical advisory-DATABASE parse fault — upstream, not fixable by code | deferred 1 |
| canary-fingerprint-derivation-aligned | 22nd consecutive; unlike prior pins the tree is no longer zero-delta since blake3 landed, so the new dep is covered by the `cargo deny` overlap alone | — |
| error-baseline-spike-live-proof | **RUSTSEC-2026-0258 (h2 0.4.15) landed between wraps against the untouched tonic tree** — deny went red mid-chunk; lock-only bump to 0.4.16 restored exit 0 | iterations 1 |

**Proposal:** overlaps P5 for two of three. The third is the distinct and more interesting case — the
*overlap gate itself* went red from external decay, which is the scenario the audit-deferral rationale
depends on not happening. A direction: nothing to change in the handling (the lock-only bump is the
sanctioned remedy); worth recording that the deferral's safety argument rests on `cargo deny` staying
green, and that assumption was falsified once this epoch.

### P9 — `wrap-session/reconcile` · `contract.cascade-miss` — 3 cases · weight 3
**Pattern:** the amendment cascade's grep-based sweep does not reach a surface that carries the retired
claim, and the miss is caught by hand or by a floor check.

| chunk | what | impact |
|---|---|---|
| canary-spans-pulse-fingerprints | the retired wording lives verbatim in a **preserve-verbatim curation home** (`rules/verification-harness.md` Session Additions) which the cascade is forbidden to edit | extra_reads 1 |
| fault-application-spans | the cascade found the retired wording nowhere outside obs-plan but DID find the three span names inside `.andromeda/playbook.md`'s dismiss rule — **which the cascade DAG does not cover** | — |
| fingerprint-storm-live-proof | the leaf sweep found a stale distillation the master-level grep did not: `.claude/docs/tests-summary.md:20` carried "fingerprints populated" in its own compressed wording, matching none of the retired phrasings | extra_reads 2 |

**Proposal:** three distinct blind spots, all structural rather than execution errors — (a) preserve-verbatim
homes are unreachable by construction, (b) `playbook.md` is outside the cascade DAG, (c) a
*re-worded* distillation defeats a phrasing-match sweep. A direction: name (a) and (b) explicitly in the
cascade contract as known-uncovered surfaces requiring a manual pass, and for (c) consider matching on the
claim's subject rather than its phrasing. This is the epoch's clearest `contract.structural-blind-spot`
family even though only 2 records carry that type.

### P10 — `wrap-session/curation` · `ambiguity.filter-borderline` — 3 cases · weight 3
**Pattern:** a curation candidate sits at the Filter-1 similarity edge against an entry written in the same
or previous pass, and resolution turns on the additive-facet tiebreaker.

| chunk | what |
|---|---|
| fault-application-spans | the operator-requested T2 candidate scored far over Filter 1's 0.7 Jaccard bar against a `verification-harness` entry the PREVIOUS chunk extended the same morning; survived only via the additive-facet tiebreaker |
| — (chunk:null wrap) | a T2 candidate sat at the Filter-1 edge against the Tier-1 extension being written in the same pass; an existing rules-file entry pushed it toward the rules file |
| fingerprint-semantics-token-leading | two candidates were scored as separate entries, then recognised as the same underlying lesson at two sites — folded into one in-place extension |

**Proposal:** all three resolved correctly, and the tiebreaker is what did the work. The recurring trigger
is **same-session self-similarity** — a candidate measured against an entry this very pass authored. A
direction: have Filter 1 treat entries written in the current pass as a distinct comparison class, so the
tiebreaker is reached deliberately rather than as a rescue.

---

## Cross-step chains (starting heuristics)

Anchors: 10 `consumed[].quality ∈ {thin,wrong,missing}` verdicts + 11 `input.*` frictions.
Faulted artifacts: `research` thin ×4 · `report` thin ×2 · `plan` thin ×2 · `plan` **wrong** ×1 ·
`health-inputs` thin ×1.

### X1 — `phase/research` →`research`→ `implement/*` — 5 links across 4 chunks *(strongest shape)*
`research` → `implement/code` recurs in **3 chunks** (the aggregation threshold), plus 2 more into
`implement/fix-loop`. In 3 of 5 the producer is formally `outcome: ok` while carrying the
`unresolved-questions` signal — a formally-ok, signalled producer feeding a downstream `thin` verdict,
which is precisely the chain hypothesis shape.

| chunk | consumer verdict note |
|---|---|
| canary-spans-pulse-fingerprints | Files-to-modify **under-enumerated one compile-forced caller**, `conductor-emit/tests/egress.rs:61` |
| fingerprint-storm-live-proof (code) | Files-to-modify **omitted** `conductor-core/src/scenario.rs`, whose rstest case pins the fixture's exact `p_ids` |
| fingerprint-storm-live-proof (fix-loop) | research verified an absolute path is STRIPPED but not that stripping yields **equality** with a relative-path base — *the half that decided the design* |
| restart-suppression-live-proof | research **missed** `cue/evaluate.rs:55` (persistence=samples) and the shared `DEFAULT_SERVICE_NAME` identity |
| restart-suppression-live-proof (plan, **wrong**) | phase-design premises falsified pre-write: persistence=samples, shared canary identity, relative arm unreachable, bypass label literals |

**Hypothesis:** the research step's **file/callsite enumeration** is the failing sub-deliverable, in two
distinct ways — it misses call sites the change compiles-forces or a test pins (3 cases), and it verifies
a mechanism's *existence* without verifying the *equality/conclusion* the design rests on (2 cases). The
second is the same failure the project's own session-learnings describe for external systems, appearing
here inside the pipeline. **Proposal direction:** make the compile-forced/test-pinned callsite sweep an
explicit research obligation with the code-graph query named, and require the plan's load-bearing
mechanism to be stated as the *equality it needs*, not the *property it observed*. Corroborated
independently by the §L4 band-aid theme (3 in-scope edits outside the Files-to-modify list).

### X2 — `wrap-session/report` →`report`→ `wrap-session/reconcile` — 2 chunks
Both producers formally `ok`, no signals; both consumers rate the report `thin`.

| chunk | consumer verdict note |
|---|---|
| canary-storm-autonomous-band | one bullet under-filled: "Counts/qualifiers moved" said none, when the canary count 6→12 is a documented derived value `architecture.md:161` states in prose |
| canary-fingerprint-derivation-aligned | sufficient for 5 of 7 docs, but the `assert_canary` bullet did not say another caller of `retrieve_telemetry_slice` remains — which drove two obs proposals that would have int… |

**Hypothesis:** the report's "counts/qualifiers moved" and remaining-caller bullets under-fill in a way the
downstream detector fan-out then pays for (one case explicitly drove two proposals that would have been
wrong). At n=2 this meets the aggregation threshold but is the weaker of the two shapes.
**Proposal direction:** treat those two bullets as checklist items with an explicit "none, verified" form,
so an empty bullet is distinguishable from an unfilled one.

---

## Level candidates (systemic-masked-as-project)

Pass A clustered all 61 `workaround`/`prohibition`/`removed-cause` facts by the obstacle each routes
around — by theme, across steps and skills. Five themes met the F-2 threshold.

### L1 — **band-aid** — Bash-transport payload limits — 13 facts · 6 steps · 5 chunks · `environment`/`resources`
**Facts:** 6 heredoc-write failures (`phase/distill` ×5 across `error-baseline-spike`,
`restart-suppression`, `pii-scrub`, `connection-lifecycle`, `latency-regression`; `phase/validate` ×1) and
7 tool-result-cap overflows on reads (`new-session/orientation` ×4, `phase/take-up` ×1, `phase/research`
×1, plus one recovery re-overflow). Every single one resolved by **substituting a different tool** —
Write instead of heredoc, chunked `sed`/`grep`/`awk` instead of a bundled read.
**Level hypothesis:** the cause lives in the **host command transport and the tool-result cap** — a
pipeline/environment property. The fixes have all landed **in the project's chunks**, one substitution at a
time, 13 times in one epoch, and the ledger shows the same mechanism at E1=3 → E2=9 → E3=12 (heredoc
alone) — *growing*, because the artifacts being written (extracts) and read (`working-route.md`, now 71KB
and CARRY-heavy) keep getting larger.
**Proposal:** two directions, independent. (1) Change the *default* rather than the fallback: extract and
artifact persistence goes through the Write tool, with the heredoc form removed from the Bash-first
directive. (2) The read side is an **artifact-size** problem, not only a tool one — `working-route.md`
overflowing every session-start is a signal the route file wants a per-epoch split or a CARRY sidecar. The
project-level workaround (re-derive via grep) is being paid at every single session start.

### L2 — **band-aid** — cp1252 console encoding — 5 facts · 4 steps · `environment`
**Facts:** `wrap-session/route-resolve` ×2, `phase/take-up` ×1, `wrap-session/gates` ×1, `phase/research`
×1. The records themselves count the recurrence — *"second occurrence this session"*, *"third
occurrence"*, *"fourth occurrence this session of writing an edit as a UTF-8-explicit script file instead
of in-place edits, because the host console's cp1252 default cannot round-trip the route grammar's middot
and arrow characters through stdout"*. Recurs in Epoch 2 (6 hits) as well.
**Level hypothesis:** the cause is the **host console's cp1252 default**, an environment property. The fix
each time is a project-level act (write a UTF-8-explicit script file for this one edit), re-derived by
whoever hits it, four times in a single session.
**Proposal:** one environment-level setting (`PYTHONIOENCODING=utf-8` / UTF-8 console mode established
once for the session) would remove the cause the four workarounds each route around. Note the route grammar
itself — `·`, `→`, `↓`, em-dashes — is what makes the route files unsafe to round-trip, so an ASCII route
grammar is the alternative direction if the environment cannot be fixed.

### L3 — **band-aid** — the documented harness invocation is routed around for live legs — 5 facts · 5 chunks · `process`
**Facts:** `implement/smoke` on `canary-spans` (bare `status` returned exit 0 on a **stale prior-chunk
envelope**; rejected as evidence) · `fault-application-spans` (drove `SCENARIO=… agent-run.sh run` instead
of the plan's bare `status`, *"because a bare status exits 0 on a prior chunk's residue and would have
proven nothing about this run"*) · `pii-scrub` fix-loop (invoked the run-verb leg branch directly to avoid
a double full-gate) · `connection-lifecycle` fix-loop (*"the plan-listed boot-and-run pairing was replaced
by direct `conductor.exe run` per leg — boot fires its own canary whose open incident dedupe-blocks the run
canary"*) · `latency-regression` smoke (same substitution, *"the standing 2026-08-19 rule, same plan defect
the pri…"*).
**Level hypothesis:** the cause lives in the **plan template's Test Commands convention** and the harness's
`boot`+`run` pairing — a pipeline property. The fix landed five times as a per-chunk deviation, and by the
last two chunks the record explicitly calls it *the same plan defect* recurring.
**Proposal:** the substitution is now stable and known-correct; a direction is to make it the documented
form — plans stop listing `boot` then `run` for live legs, and the bare `status` verb is removed from
Test Commands entirely (it cannot distinguish this run from residue). This theme's own records already
carry the rule; what is missing is the plan template catching up to it.

### L4 — **band-aid** — the named reference is not read; schema/parser inferred instead — 5 facts · 2 steps · `process`
**Facts:** all at `new-session/orientation` (4) plus one at `phase/research`. Three are the
verification-matrix schema (*"inferred `verification-matrix.json`'s schema by probing the file instead of
reading `references/verification-matrix-contract.md`"*; *"derived the schema by inspecting the JSON
instead of reading the named reference first"*; *"matrix coverage extracted by ad-hoc python probing
instead of consulting the contract"*), one is Check 2 (*"verified by pairing GENERATED/USER markers
directly from a CLAUDE.md grep instead of reading the named `section-markers.md` parser reference"* —
same verdict reached), and one is `phase/research` using Bash grep rather than the Grep tool the research
reference names.
**Level hypothesis:** the cause is that **the skill body's inline shorthand reads as a complete spec**, so
the named reference feels redundant. The Epoch-1 record states it outright: the probe read `status` as a
top-level field *"per the skill body's `chunk:null & status:planned` shorthand"*. The fixes have landed as
per-session re-derivation (2 extra reads, every time).
**Proposal:** see §U1 — this is the same root cause as the epoch's largest untyped cluster, and the
cheapest fix is a two-word correction in the skill body (`verification.status`), not a discipline change.

### L5 — **band-aid** — Pulse's open-incident dedupe blocks back-to-back legs — 3 facts · 3 chunks · `environment`/`process`
**Facts:** `fingerprint-storm` smoke (*"first storm leg blocked because Pulse deduped its canary incident
against the boot leg's still-open one; waited for auto-resolve (~5 min) and re-ran rather than resolving
via `mark_incident_resolved`, which would have had Conductor mutate SUT state mid-leg"*) ·
`connection-lifecycle` fix-loop · `latency-regression` smoke. Corroborated by the untyped record
*"Pulse dedupes a new incident against an OPEN one and the dedup key is not the fingerprint … No document
names this"*.
**Level hypothesis:** a **SUT property** absorbed at the project level three times, each chunk paying
either a ~5-minute wait or a fresh data dir. The first record notes the clean fix is off-limits by scope
law (Conductor must not mutate SUT state mid-leg), so the workaround is correct — but it is undocumented
in any contract, which is why each chunk rediscovers it.
**Proposal:** record the dedupe behaviour in the run contract or the live-leg recipe as a named
precondition (one fresh data dir per leg), so it stops being rediscovered. This is a documentation-level
fix to a SUT-level cause; no code change is implied.

### L6 — **deferred-forever** — the `cargo audit` standing deferral
**Facts:** `tooling.gate-deferral` at E1=3 · E2=9 · **E3=8**, spanning all three epochs; 33 consecutive
chunks at epoch end; 9 `solution: deferred` problem-facts in this epoch alone. Every re-pin preserves the
original origin (`deferred since 2026-08-08-sut-capability-manifest`) and re-verifies rather than echoes.
**Level hypothesis:** the deferral is **operator-ratified and correctly handled** — this is not a
discipline failure, and the ledger shows the bounded-wait rule working exactly as written. What the
signature catches is that its **closure condition has never appeared** and cannot be caused from inside
Conductor (it requires RustSec's advisory-db to stop shipping a duplicate advisory ID). The security rules
already anticipate this shape and required one explicit ratification; the re-pinning is silent by design.
**Proposal:** no corrective action indicated. Surfaced because the batch view is the only place the
**33-chunk age** is visible at once — the founder may wish to set a review date rather than a per-chunk
re-check, given the remedy is a wait and the overlap gate is what actually provides coverage. §P8's third
case is the one thing that would change this judgment: the overlap went red once this epoch.

### L7 — **chronic-degrade** — two independent instances
**(a) Host-shell mechanisms.** The `tooling.host-shell` *type* first appears 2026-08-16, so its E1=0/E2=0
is era. Re-derived by note-search across epochs, the **mechanisms** are chronic and mostly growing:
heredoc-write failure E1=3 → E2=9 → E3=12 · cp1252 stdout E1=0 → E2=6 → E3=7 · tool-output-cap overflow
E1=0 → E2=4 → E3=8 · cwd-drift/`TMPDIR` E1=0 → E2=1 → E3=5 · tail-swallows-result E1=1 → E2=1 → E3=4.
**None ever halted** — every one was worked around, which is exactly the degradation the halt policy
structurally cannot surface. *(These are keyword-mechanism probes over `what` + problem notes —
indicative counts, not exact classification.)*

**(b) `implement/smoke` `ok-degraded` ×5** — `canary-storm-autonomous-band`, `canary-spans`,
`canary-fingerprint-derivation-aligned`, `fingerprint-storm` ×2. The recurring cause is one thing: *"the
live leg could not run: `pulse-app` is not running, `andromeda-pulse-mcp` is built at
`../andromeda-pulse/target/debug` but not on PATH, and all three of `ANDROMEDA_PULSE_MCP_ENABLED` /
`_L4_DETERMINISTIC` / `_DATA_DIR` are unset"*. The step degrades and continues rather than halting.
**Level hypothesis:** a **environment-provisioning** gap absorbed per-chunk. It resolved mid-epoch (the
last five chunks all ran live legs), so the fix has effectively been absorbed by the project already.
**Proposal:** the remaining pipeline-level generalization is a **preconditions probe** the smoke step can
run first — the four conditions above are checkable in one command — so a degraded leg is announced before
the work rather than discovered during it. §P1's last case sharpens this: a PATH-resolution failure and a
genuine SUT gate failure are indistinguishable at row level, so the probe is also a correctness guard.

**Override signature: not met.** 6 `overridden` facts, but no single rule recurs at n≥3.
`wrap-session/route-resolve` carries 2 across 2 chunks on *different* rules (the desc-verbatim master flip;
the trajectory-edit halt). Listed in the appendix.

---

## Playbook-extension candidates (untyped patterns, F-4)

26 untyped records (19.0%), clustering into 3 above-threshold groups.

### U1 — `new-session/orientation` — 4 cases in-epoch, **recurring in Epochs 1, 2 and 4** → proposed type `contract.schema-assumed`
**Cluster** (all four say the same thing):
- *"coverage extraction probed `verification-matrix.json`'s schema instead of reading the contract
  reference; per-capability status lives at `verification.status`, not at the capability root, so two
  probes returned `status=None` for all 32 entries and reported 0 verified before the third probe found
  the real field"* (extra_reads 2)
- *"Coverage counting assumed a top-level `status` field on each matrix capability; every one of the 32
  records read as null, and two extra inspection passes were needed"* (extra_reads 2)
- *"Coverage extraction assumed a top-level `status` key … the first two queries reported 32 nulls and 0
  verified before the entry shape was inspected"* (extra_reads 2)
- *"verification-matrix summarized with a schema-guess probe batched parallel to the contract read; the
  flat `status` guess missed `verification.status` and read 32× None → one corrected recount"*
  (retries 1, extra_reads 1)

**Recurrence — this is the epoch's most reliably repeating event.** Epoch 1: *"Coverage probe read `status`
as a top-level capability field **per the skill body's `chunk:null & status:planned` shorthand**"*.
Epoch 2: *"First coverage query read `status` at the capability top level and returned a plausible-looking
but wrong result (total 32, verified 0, statuses [None]) **instead of erroring**"*. And it fired again in
**Epoch 4**, during the session that ran this diagnosis. That is **7+ occurrences across 4 consecutive
epochs**, at one step, always untyped, always with the same recovery.

**Draft criteria line for `references/evolve/orientation.md`:**
> `contract.schema-assumed` — an artifact's field layout was inferred by probing rather than read from its
> named contract reference, and the guess returned a plausible wrong value instead of an error.

**Proposal (the cause, not just the type):** the Epoch-1 record names it — the skill body's own shorthand
`count status:verified / total` and `collect chunk:null & status:planned` **reads as a flat schema**, and
it is half-right (`chunk` *is* top-level; `status` is not), which is exactly why it misleads rather than
fails loudly. A two-word correction in the `andromeda-new-session` Phase 1 step 5 wording —
`verification.status:verified`, `chunk:null & verification.status:planned` — would remove the cause of a
7-occurrence, 4-epoch pattern. The failure is also **silent by shape**: the wrong guess yields
`0 verified / 32 planned`, a plausible number, caught only because the handoff independently states the
true one.

### U2 — `new-session/orientation` — 3 cases in-epoch, recurring from Epoch 2 → proposed type `tooling.output-cap-overflow`
**Cluster:**
- *"`working-route.md` no longer fits one Read (96 lines, ~26k tokens vs 25k cap) — per-entry CARRY/PREREQ
  freight forced a paged second read during position derivation"* (extra_reads 1)
- *"bundled cat of the 5 skill references (35KB) and the route+ls bundle (70KB) exceeded the tool-result
  cap leaving 2KB previews; one recovery `sed` re-overflowed; full recovery cost 5 chunked re-reads plus
  re-querying working-route from source"* (extra_reads 6)
- *"`working-route.md` cat exceeded the tool-output cap (71KB persisted to file, preview only);
  markerless-line derivation re-issued as a grep structure extraction"* (reformulations 1, extra_reads 1)

Epoch-2 recurrence: *"reading the route pair in one call blew the inline output cap and was persisted to a
file; the first narrowed working-route grep then matched only `[marker]`-prefixed lines, so the markerless
…"* — note that recovery introduced a *second* error.

**Draft criteria line:**
> `tooling.output-cap-overflow` — a read exceeded the tool-result size cap and was recovered by chunked or
> structural re-extraction; record the artifact and the recovery cost.

**Proposal:** see §L1(2) — the type would make the cost visible, but the cause is `working-route.md`'s
size, which grows every chunk. Worth the founder's judgment on whether the route file wants a per-epoch
split before this becomes every session's opening tax.

### U3 — `wrap-session/curation` + `reconcile` — 3 cases in-epoch, recurring from Epochs 1–2 → proposed type `contract.curation-correction`
**Cluster:**
- *"Two curation homes carried entries the chunk's measurement made FALSE rather than merely incomplete —
  `testing.md` asserted that choosing a stable hash makes the expected value match the SUT's, and
  `session-learnings.md` asserted fidelity rides the fingerprint; the cascade routes such hits to curation
  as in-place extensions, but these were **corrections of false claims rather than additive facets**, and
  the max-3 candidate cap does no…"*
- *"a preserve-verbatim curation home holds a now-falsified prescription: `rules/testing.md` Session
  Additions 2026-06-22 tells future chunks to verify a fingerprint capability THROUGH Contains/Absent
  read-back tokens … the exact vacuous-green trap this chunk measured. The cascade must never edit that
  block, so it routed to P3 curation as an in-place extensi…"*
- *"the highest-value curation candidate this wrap was a CORRECTION to an existing Session Additions entry
  rather than a new learning … Filter 1's additive-facet tie-breaker was the mechanism that kept the
  correction attached to the claim it corrects instead of minting a sibling en…"*

**Draft criteria line:**
> `contract.curation-correction` — a curation home carries a claim the chunk's measurement made FALSE, and
> the only available route is the additive-extension mechanism, which is shaped for new facets rather than
> corrections.

**Proposal:** the curation machinery has one verb (extend additively) for two acts (add a facet; correct a
falsified claim). It resolved correctly all three times via the additive-facet tiebreaker, but by
coincidence of that tiebreaker rather than by design. A direction: a distinct `correction` disposition
that attaches to the falsified entry and is exempt from the max-3 candidate cap — the first record notes
the cap was a live constraint.

---

## Below threshold — no action

**Typed groups (44 of 57 below threshold)** — n=2 with no halt/soft-exit impact, or n=1:
`wrap-session/reconcile`/`ambiguity.playbook-no-match` (2, wt 6) · `phase/validate`/`contract.intent-divergence`
(2, wt 4) · `phase/research`/`contract.narrow-basis-claim` (2 — rolled into P4) ·
`phase/validate`/`contract.mechanical-check` (2) · `implement/code`/`input.research-files-wrong` (2 — see
X1) · `phase/plan`/`input.extracts-conflict` (2) · `phase/take-up`/`contract.premise-falsified` (2 — rolled
into P2) · `phase/research`/`contract.structural-blind-spot` (2) ·
`wrap-session/reconcile`/`contract.false-positive-proposal` (2) ·
`wrap-session/reconcile`/`input.report-insufficient` (2 — see X2) ·
`wrap-session/curation`/`ambiguity.tier-routing` (2) · `implement/fix-loop`/`contract.premise-falsified` (2) ·
`implement/smoke`/`tooling.harness-friction` (2) · `phase/distill`/`contract.binding-contradiction` (2) ·
`wrap-session/route-resolve`/`contract.narrow-basis-claim` (2) — plus 29 singletons across
`ambiguity.review-cycles` · `retry.distiller-respawn` · `contract.matrix-claim` ×3 · `contract.extract-format` ·
`input.conventions-gap` · `retry.query-reformulation` · `ambiguity.scope-pressure` · `tooling.headless-skip` ·
`input.research-thin` · `contract.detector-fact-gap` · `ambiguity.filter-borderline` (route-resolve) ·
`input.carry-context-gap` · `tooling.environmental` (smoke) · `input.outcome-unclear` · `tooling.hook-friction` ·
`input.deviations-unjustified` · and the per-step `tooling.host-shell` singletons rolled into P1.

**Universal:** `contract.structural-blind-spot` n=2 across 1 step, 2 chunks — below threshold, but
thematically the strongest reading of P9's three cascade blind spots.

**Untyped clusters below F-4:**
- **guessed-ahead timestamp** (n=2, no prior-epoch recurrence) — two step records carry an envelope `ts`
  composed in the same command as its `date` call, reading ~4min and ~2min into the future
  (`2026-08-18T18:29:00Z-a`, `2026-08-18T19:27:30Z-a`); content accurate, append order intact. Both
  self-disclosed **for the diagnosis to discount** — doing so here as instructed.
- **playbook undefined at a boundary path** (n=2) — route-resolve's master-flip contract has no path for
  correcting a `desc` its own chunk's proof falsified; the gates playbook is undefined on the 0-pending
  no-op path (3 of 5 criteria structurally inapplicable). Related to the below-threshold
  `ambiguity.playbook-no-match`.
- **standing PREREQ compact-form lapse** (n=2) — both records state the rule worked as written (any basis
  change restores the full form); recorded as designed behaviour, not friction.
- **harness/config traps** (n=2) — `agent-run.sh` forces `SEED=424242` over the TOML-declared seed;
  `CONDUCTOR_*` dir handles reject absolute out-of-repo values by design.
- **tail swallows the true result** (n=2 in-epoch problem-facts; mechanism E1=1/E2=1/E3=4) — the
  documented never-read-through-tail trap, self-caught both times.
- Singletons: live-leg re-run impossible at the light gate · cross-leg contamination via shared
  `base_exception` · Pulse dedupe key is not the fingerprint · unplanned determinism confirmation
  (a positive) · whole-file `json.dumps` normalizing unrelated unicode escapes · detector under-run caught
  by the expected-amendments floor.

**Override facts (signature not met, 6):** context-budget halt reversed by operator measurement ·
route-resolve `desc` verbatim-flip overridden to describe actuals · plan amended in 4 post-approval edits ·
trajectory-edit halt satisfied in advance by the wrap invocation · sidecar append-only contract — proposal
applied in a different form · plan step 2 phase table replaced wholesale by the young-window redesign.

**Removed-cause facts (observation, 8):** two are the same recurring cause — releasing `:4317` from a
GUI-subsystem `pulse-app` that `taskkill` does not stop gracefully (`canary-spans`, `fingerprint-storm`).
The remaining six are one-off cause removals (temporary probe test added then removed; `research.md`
edited mid-synthesis to drop a spec master from Files-to-modify; citation de-literalization;
`CANARY_SERVICE_NAME` split; Windows-form `PATH` entry corrected to POSIX; route-directive target
verified against the file).

---

*Diagnosis is read-only and stateless. Nothing above is applied, queued, or remembered; a re-run
recomputes from the ledger alone. Evidence twins: `q-health.json` · `q-typed.json` · `q-chains.json` ·
`q-level.json`.*
