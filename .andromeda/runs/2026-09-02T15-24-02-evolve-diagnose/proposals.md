# Evolve Diagnosis — Conductor · Epoch 5 "Verification surfaces" · 2026-09-02T15:24:02Z

Target epoch chosen in dialogue: the latest fully-complete epoch, and the only complete one with no
prior diagnosis (Epochs 1–4 each have a `proposals.md`). Everything below is **obligation-free**:
accept, reject, defer, or modify any item with no mechanism-side consequence. Nothing here is
applied, queued, or remembered — a re-run recomputes from the ledger alone.

---

## Mechanism health

| | |
|---|---|
| **Records** | 233 in-epoch — 95 `step` / 138 `friction` · 6 chunks |
| **Unparseable** | **0** across the whole 1160-record ledger |
| **Step-record coverage** | **No checkpoint gaps.** All 6 chunks fired phase 5 · implement 3 · wrap 5. Two chunks carry an EXTRA record each, both explained by a recorded event: `screen-reader-manual-spec` phase = 7 (the operator's consolidated relay answered "review" at the first P5 card, sending the run back to P4 once — recorded as an override) and `p-075-assert-round` implement = 4 (the smoke re-ran after the operator supplied warm binaries). Extras, never gaps. |
| **Null-chunk step records** | 14 — 8 `new-session/orientation` (one per session start, by design) + 6 on the 0-pending no-op wrap path (route-resolve ×3, curation ×2, gates ×1). All expected. |
| **Untyped rate** | **19 / 138 = 13.8%** (up from Epoch 4's 11.6%). Concentrated at `wrap-session/route-resolve` (4), `new-session/orientation` (4), `wrap-session/curation` (3). |
| **Schema defect** | **2 records carry BOTH a valid `type` AND `untyped: true`** — `2026-09-01T22:52:18Z-b` (`recall.curated-rule-not-applied`) and `2026-09-02T06:30:00Z-b` (`recall.corpus-recurrence`). The contract is `type` XOR `null + untyped:true`. Counted as TYPED here (they carry real playbook types); the raw `untyped:true` count is 21, the true untyped count 19. |
| **Problem-fact fill** | 37 / 95 step records carry ≥1 deviation fact (43 facts total: 24 workaround · 11 overridden · 6 deferred · 2 removed-cause) |
| **`id` fill** | **233 / 233** — complete, well past the 2026-08-18 boundary |
| **Retractions** | 1 honored (`problem` scope, all facts, on step `2026-08-20T21:14:37Z-a` — Epoch-4 era, outside this target range) · **1 unresolvable, reported verbatim** for manual discount: `{id: null}` prose-form, note *"the untyped code-graph-under-reports record and the first problem-block entry on…"*. No retraction-of-retraction. |
| **Calibration boundaries** | All in range. The deviation scan, `graph-not-applicable`, the Universal types and the `id`/`retracts` schema all predate 2026-08-31, so no Epoch-5 record may be read as "nothing occurred" for era reasons. |

**Degradation is DOWN across the boundary.** Epoch 4: 6 `halted-resolved` · 4 `ok-degraded` · 2
`soft-exit`. Epoch 5: 1 · 3 · 0. Every other outcome was `ok`.

---

## Proposals (typed patterns)

Universal types are grouped by type ALONE across steps (per `diagnosis-pass.md`); where a
per-step group is subsumed by one, the per-step concentration is named inside it rather than
double-counted.

### P1 — `phase/plan` · `ambiguity.scope-question` — 5 cases · weight 18
**Pattern:** Every chunk but one raised exactly one scope question at plan time, and in all five
the author independently judged the materials *could not* have answered it — the deciding fact was
an operator preference, a host reality, or a strength-vs-time trade no spec adjudicates. Rate 5/7
step-runs, 5 of 6 chunks.

**Evidence:** ALL 5 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| p-075-assert-round | Whether the live leg resolves the preflight canary's own incident or drives a second as a control. test-plan §6 argues FOR a control; the run contract's poll floor doubles the leg. "A strength-versus-time trade the specs do not adjudicate." | 1 dialogue round | plan §Constraints |
| webview-self-verify | How to obtain msedgedriver. The decisive fact was a binary at an operator-managed host path, outside the repo, outside both audited trees, named in no document. All three drafted options were acquisition strategies; the answer was "it is already here". | 2 rounds, 1 iter | research.md |
| desktop-a11y-sweep | Whether to claim v2-22 at all. The materials settle the MECHANISM (a driven arm is mandatory for a claim) but not whether this chunk should claim — the predecessor set a claim-nothing precedent. | 1 round | — |
| live-per-p-id-verdict-lamps | The p_id→Lamp collision rule. A precedent search found `Lamp::for_record` has 0 callers and no surface has ever joined run outcomes to P-IDs — no artifact decided it. Material: 5 of 45 P-IDs are multi-owned. | 1 round | code-graph `%/for_record().%` → 0 rows |
| cross-surface-envelope-parity | How to produce the banner subject. Research falsified the directive's necessity premise while leaving the live leg defensible on proof STRENGTH. "The materials could settle what was possible, never which proof the operator wanted to own." | 1 round | — |

**Proposal:** This may be the mechanism working as designed rather than friction — five for five, each
author reached the same verdict independently, and each answer was genuinely a preference. If so, the
change-shape is *recording* rather than *removing*: a `phase/plan` playbook line distinguishing a
**scope question the materials could have answered** (real friction — the author under-read) from a
**preference question** (designed dialogue), so the type stops conflating them and the count means
something. As it stands, `ambiguity.scope-question` is this epoch's highest-weight group while
plausibly describing zero defects.

### P2 — `wrap-session/reconcile` · `ambiguity.playbook-no-match` — 4 cases · weight 16 (2 halts)
**Pattern:** Four times the wrap playbook's pattern set had no verdict for a proposal class, twice
halting into dialogue. Two cases are the SAME new class arriving twice (a dev-harness `CONDUCTOR_*`
host-tool handle), and the second occurrence is sharper: a rule minted for the first *read as
governing* and would have widened a precedent silently.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| p-075-assert-round | The security cluster matched none of the playbook's 33 patterns. New class: a chunk WIDENS what crosses an already-hardened boundary (a read-only channel gains a write). Nearest pattern covers the opposite shape. Escalated; operator ratified apply. **No rule proposed — "a boundary-widening is exactly the class that SHOULD reach a human."** | 1 round, **1 halt** | security-plan 46/118/148/315 re-derived |
| live-per-p-id-verdict-lamps | Rule 58 dismisses a detector DEMANDING re-confirmation of hardening the chunk consumes; this proposal DOCUMENTS an already-correct practice for a rule the table never stated — a different act. Escalated on unease rather than force-fitting. Operator added the row, declined to mint a rule on one instance. | 1 round | rule 58 vs the proposal's rationale |
| screen-reader-manual-spec | Two classes uncovered: (a) the SECOND dev-harness `CONDUCTOR_*` host-tool handle (`CONDUCTOR_NVDA` after `CONDUCTOR_MSEDGEDRIVER`), (b) an escalate detector firing on a product DEFECT the operator had already dispositioned in the wrap directive. Both resolved on recorded direction; rules for both proposed in the report. | 0 | — |
| cross-surface-envelope-parity | **Rule 115 — minted for (a) above — named exactly the sites the 8 handle proposals targeted, so it read as governing, but its precondition fails on 4 of 5 clauses for this handle.** Applying it would have widened a precedent whose guard model this class does not share, silently. Escalated; operator ratified a THIRD handle class; a new rule was minted distinguishing it from 115 explicitly. | 1 round, **1 halt** | — |

**Proposal:** The corroborating consumed-quality verdicts say the same thing — `wrap-playbook` was
rated `thin` in both p-075 and screen-reader. Two directions, not exclusive. (1) A playbook rule
carries no *precondition* the matcher must check before applying it: rule 115 was matched on its
subject (`CONDUCTOR_*` handle sites) while its five preconditions were what actually governed. A
mandatory-preconditions field on each rule, checked before a rule is treated as governing, would
turn the cross-surface near-miss from a caught-by-unease into a caught-by-mechanism. (2) The
p-075 case argues *against* growing the pattern set for boundary-widening — the author's own
reasoning is that this class should reach a human. Worth an explicit "classes that must always
escalate" list, so a future author does not mint a rule that suppresses the escalation.

### P3 — `implement/fix-loop` · `tooling.environmental` — 4 cases · weight 12 (1 soft-exit)
**Pattern:** Every environmental blocker in the epoch is the live-Pulse toolchain, and two of the
four are *self-inflicted by the harness's own design* rather than mere absence.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| p-075-assert-round | No live Pulse reachable: sidecar absent from PATH, nothing on `:4317`, all three `ANDROMEDA_PULSE_*` unset. Live acceptance stands UNPROVEN rather than satisfied — deliberately not driven, since a run would mint a ~0s `[BLOCKED]` row indistinguishable from a real SUT gate failure. All 5 gates green on their own terms. | 1 deferred | module doc STATUS; verify 101 / run 145 passed |
| desktop-a11y-sweep | Two corrections needed. The FIRST reproduced the exact trap `security.md` predicts — BLOCKED in 2 ms with all four tools "absent" is `andromeda-pulse-mcp` failing to resolve on PATH (the binary existed, unlisted). The SECOND is unfixable by code: `pulse-app` runs without `ANDROMEDA_PULSE_L4_DETERMINISTIC`. | 2 iters, **1 soft-exit** | preflight `ready:false`; 90 dispatches ok |
| desktop-a11y-sweep | **The live precondition is self-colliding.** `conductor preflight` runs its OWN canary storm and Pulse dedupes against any open incident on the tuple, so running `agent-run boot` as a go/no-go immediately before the driven leg *primes exactly the state the run then collides with*. Two 12-minute runs lost before a ~200 s quiet window let the hold fire. | 2 retries | runs 5–6 timed out at 720 s; run 7 raised in ~50 s |
| screen-reader-manual-spec | Starting a run from the GUI raises a Windows Terminal pane for the spawned sidecar (`conductor-verify` sets no creation flags), taking the OS foreground and silently detaching NVDA. Found only because the speech log named the pane. Worked around in the leg; product fix routed as a finding. | 1 iter | — |

**Proposal:** Case 3 is the actionable one and it is a *pipeline* fact, not a project one: the
`boot`-then-`run` pairing that the plan template's LIVE-LEG rule already warns against cost 24
minutes because the warning names the pairing without naming *this* mechanism (preflight's own
canary is the collider). A worked example in that rule — "a live leg whose subject is an incident
must not be preceded by any command that runs preflight" — would have prevented it. Cases 1, 2 and
4 feed level candidate **L5**.

### P4 — UNIVERSAL · `contract.premise-falsified` — 10 cases · weight 13 · 5 steps · 5 chunks
**Pattern:** The single most frequent class in the epoch. Ten authored premises were falsified by
verification, at every stage of the loop — 6 at `phase/research` (rate **1.0** — every research step
falsified at least one premise), plus `phase/take-up`, `phase/plan`, `implement/smoke` and
`wrap-session/route-resolve`. The sources are evenly split between the project's own specs, the
working-route's stated basis, an operator directive, and a shipped test's NAME.

**Evidence:** ALL 10 cases —

| chunk | step | what | impact |
|---|---|---|---|
| p-075-assert-round | research | Scope's premise that the declined arm is hard to induce live: the guard is a monotonic-timestamp compare, so the MECHANISM was falsified while the conclusion held | 1 extra read |
| webview-self-verify | research | **The working-route entry's stated MEASURED basis — "NO driver binary exists on this host" — was false.** Both pieces of evidence true, conclusion false: a win32-x64 tauri-driver prebuilt had shipped | 5 extra reads |
| live-per-p-id-lamps | research | The security extract faithfully carried a §Anti-Patterns requirement that a new command owes an allowlist edit; research falsified its applicability — `capabilities/default.json` holds only `core:window:*` | 1 extra read |
| screen-reader | research | **`a11y-plan` §3's "No automated SR tool exists for the stack" — the stated reason the pass is manual — is false**: `@guidepup/guidepup` 0.34.0 is actively maintained | 1 extra read |
| screen-reader | research | Four spec-vs-shipped divergences: the plan's four run states vs the shipped `RunState` set (`aborted` unnamed by the plan) | — |
| cross-surface | research | **The operator's PHASE directive item 2** ("the banner CARRY needs an OVER-envelope run against a LIVE Pulse") measured false as a structural requirement — `classify_run` is pure and runs BEFORE preflight on both shells | 2 extra reads |
| — (route-resolve) | route-resolve | The adaptation directive named "the Epoch-5 OPC CARRY about the unresolved display gate"; no such annotation exists — the text is a clause inside a different entry's CARRY | 1 round, 2 reads |
| p-075-assert-round | implement/smoke | The two-incident control, operator-selected at P4, is STRUCTURALLY UNATTAINABLE — the producer dedupes against any OPEN incident regardless of fingerprint | 1 iter |
| live-per-p-id-lamps | take-up | PREREQ-1's stated basis ("zero .rs, zero manifest, zero lockfile delta") falsified at npm grain — `ece1d24` DID touch `ui/package.json` | 1 extra read |
| cross-surface | plan | **A shipped test's NAME asserts more than its body**: `path7_tauri_persists_the_same_blocked_envelope_as_the_cli` never runs the CLI — it compares against hand-written literals with a comment claiming `cli_smoke` proves the other side | 2 extra reads |

**Proposal:** n=10 with zero halts is the signature of a class the loop **absorbs well** — every one
was caught by the step whose job it is. Two observations for judgment. (1) `phase/research` at rate
1.0 suggests the research step is doing exactly what it exists for, and the count is a health signal
rather than a defect count. (2) Three cases falsify an artifact that *states its own basis as
measured* — a route-entry CONTEXT, an operator directive, a test's name. Those are the expensive
ones (5, 2 and 2 extra reads). A possible direction: a convention that any artifact asserting a
measured basis carries the measurement's DATE and INSTRUMENT, so a reader can see at a glance
whether it is a fact or an inference. This would not have prevented the falsifications, but it
would have made the webview entry's "no driver exists" visibly an inference from two absences.

### P5 — UNIVERSAL · `contract.structural-blind-spot` — 4 cases · weight 12 · 3 steps · 2 chunks
**Pattern:** Four failures no correct execution could have caught. Concentrated in the two chunks
that first drove the webview — the region where the pipeline had no coverage at all.

**Evidence:** ALL 4 cases —

| chunk | step | what | impact |
|---|---|---|---|
| webview-self-verify | validate | The plan DISPOSITIONED the CARRY's source-side twin in Implementation notes but gave it **no EXECUTOR** — absent from Files-to-modify, named by no step. P5's mechanical checks read the modify-set, so nothing would have caught it | 1 round |
| webview-self-verify | fix-loop | **The webview config shipped 2026-06-27 was UNLOADABLE on every host** (`type: module` + `__dirname`), while `typecheck:e2e` passed for two months. Three further defects sat behind it | 4 iters |
| webview-self-verify | reconcile | **The a11y doc-agent returned `proposals: []` and was RIGHT to** — its two detectors cover new-UI-element coverage and violation-JSON, and a platform/runner/driver claim maps onto neither, while its document carried six sites stating a verdict this chunk measured false | 1 round |
| screen-reader | validate | Research read both `wdio.conf.ts` sites and recorded the first as a design input **without connecting the two**: by construction any driven or sr session persisting a run journal lands in the fixture dir | — |

**Proposal:** Two of the four are the same shape — **a gate that reads a narrower surface than the
claim it guards** (P5 reads the modify-set, so a dispositioned-but-unassigned file is invisible; a
detector reads its own domain, so a cross-domain verdict is invisible). The `drift-base` consumed
verdict in this chunk records the second being fixed mid-epoch ("Grown this wrap: D-plat…"). The
remaining generalization: at P5, cross-check *Implementation notes* against *Files to modify* — any
file named in the former and absent from the latter is an unassigned executor. That is one
mechanical check and it would have caught case 1 exactly.

### P6 — `phase/validate` · `contract.matrix-claim` — 4 cases · weight 10
**Pattern:** Four times the coverage-claim machinery gave a wrong or unreachable answer, and in
three the *operator* caught it rather than a gate. Rate 4/7, 4 of 6 chunks.

**Evidence:** ALL 4 cases —

| chunk | what | impact | evidence |
|---|---|---|---|
| p-075-assert-round | The claim-candidate scan surfaced a keyword CANDIDATE (v2-28) matching on the substring `mcp`; reading the entry settled it as another chunk's cap. Chunk claims nothing — recorded deliberately in scope + plan | 1 extra read | 11 ids scanned; chunk-owned = 0 |
| desktop-a11y-sweep | **The concretized acceptance I proposed was weaker than I judged it to be** and I flagged it as a wording loosening: dropping "the harness is run, not re-authored" removed the only clause forbidding a failing spec from being deleted. The never-weaken checkpoint caught it; the operator restored the guard | 1 round | plan.md |
| live-per-p-id-lamps | The P5 premise-check falsified v2-30's opening clause: "After a run" — but `/runs/` is gitignored and the routine arm has no live Pulse, so **on a clean checkout the populated-lamp half has NO SUBJECT**, and on this dev host it would have passed off ~60 uncommitted local journals | 1 round | `.gitignore:17`; `git ls-files runs/` empty |
| cross-surface | **A claim path that halts the coverage gate BY CONSTRUCTION is invisible to every P5 mechanical check.** v2-28's decisive artifact can only be applied by wrap, so implement could never set `status=implemented`. Nothing in the checks, val-1, or the matrix contract tests "can the claiming chunk's own lifecycle reach implemented?" | 1 round | — |

**Proposal:** Cases 2 and 4 are both *the claim machinery cannot check itself*. Case 4 names the
missing predicate exactly and it is cheap: at P5, for each claimed capability, assert that the
artifact its `ref` will cite is producible by a step this chunk's own lifecycle reaches — an entry
whose evidence is authored by wrap cannot be claimed by implement. Case 2 argues the never-weaken
checkpoint is load-bearing and working; case 3 argues a claim's acceptance should name the SUBJECT
it will be measured against, not only the outcome.

### P7 — `implement/fix-loop` · `contract.spec-reality-gap` — 4 cases · weight 10
**Pattern:** Four specs contradicted by measurement at implement time — two project rule-files, one
a11y-plan prescription, one plan premise. Rate 4/6.

**Evidence:** ALL 4 cases —

| chunk | what | impact |
|---|---|---|
| webview-self-verify | `verification-harness.md`'s 2026-08-22 entry (b) — "only a release build reads the bundle" — measured FALSE at source: `tauri`'s `build.rs` computes `dev = !custom_protocol`, so the FEATURE decides and the PROFILE does not enter it | 1 iter |
| desktop-a11y-sweep | `a11y-plan` §6 (:419) and its Decisions Log (:592) both prescribe `browser.emulate('prefers-reduced-motion','reduce')`; installed webdriverio 9.x declares exactly six emulate scopes and reduced-motion is not among them **on any platform** | 1 iter |
| desktop-a11y-sweep | A GENUINE SC 2.4.3 defect on a must-be-accessible path (focus to `<body>` after a hold). My first hypothesis — an unfocusable disabled Start — was measured FALSE; switching to `aria-disabled` kept it focusable and focus still fell to body | 2 iters |
| screen-reader | Two plan premises contradicted by product behaviour: `drive_run` polls the abort flag only before a scenario (so the two-scenario catalog ended Done/idle), and the focus-restore row expected Start while the app correctly restores to the pre-hold focus | 2 iters |

**Proposal:** Cases 1 and 2 are stale *curated* content — a `verification-harness.md` Session
Addition and an `a11y-plan` prescription, both stating a mechanism that was never true or has since
changed. Neither is reachable by drift detection (they assert about external tools). A direction:
curated entries and spec prescriptions that name an EXTERNAL tool's behaviour carry the tool version
they were measured against, and a version bump makes them re-verification candidates. This is the
same shape as the Tier-1 curation rule the project already holds ("an alignment verified byte-exact
is a point-in-time measurement, never a durable property") — the proposal is to make it mechanical
for the pipeline's own artifacts, not only for the SUT.

### P8 — UNIVERSAL · `contract.token-proxy-check` — 7 cases · weight 7 · 3 steps · 5 chunks
**Pattern:** Seven times a check tested for a TOKEN where the intended property was semantic. **All
seven failed in the FALSE-POSITIVE direction except one**, and in every case the truth came from
reading the hits. Concentrated at `phase/distill` (3) and `wrap-session/gates` (2).

**Evidence:** ALL 7 cases —

| chunk | step | what | direction | impact |
|---|---|---|---|---|
| p-075 | distill | Anchor check probed for the literal `per `; re-probing on the actual property (does the bullet cite a § anchor) returned 0 | false-pos | 1 read |
| webview-self-verify | distill | Anchor check required the literal `per <plan> §` and flagged **25 of 53** bullets as unanchored — every one cites a real anchor | false-pos | 1 read |
| webview-self-verify | validate | Re-verifying modify-path existence, `tr -d '-'` stripped hyphens INSIDE the paths (`conductor-tauri` → `conductortauri`), reporting existing paths missing | false-pos | 1 read |
| live-per-p-id-lamps | distill | The fan-out check-3 anchor probe returned **19 FALSE POSITIVES**; all 19 carry a leading anchor or a bare in-domain `§N` | false-pos | 1 read |
| live-per-p-id-lamps | validate | Mechanical check 3 asserted the string `(none)` in the New-files block, where the actual rule is "every New-files path's parent exists" — two legitimate new files flipped it to FAIL | false-pos | 1 read |
| screen-reader | gates | The hygiene grep (drive-letter / `%APPDATA%` tokens as a proxy for "no host path") matched **the parser's own HOST_PATH regex — the scrub that implements the property** | false-pos | 1 read |
| cross-surface | gates | **The evolve-nudge predicate** ("no proposals.md NAMES that epoch") grepped for `Epoch 5` and hit the 2026-08-22 run, which is titled Epoch 4 and mentions Epoch 5 only in a timestamp annotation | **false-NEG** | 1 read |

**Proposal:** This is the epoch's most mechanically fixable class. Four of seven are **the same
check** — the per-extract/fan-out anchor probe, re-implemented as a literal-token regex at three
different steps in three different chunks, each time producing a large false-positive count (25, 19,
7). It is not a knowledge gap: a Tier-1 curation entry dated 2026-09-01 states the rule verbatim and
the *next* wrap reproduced it (see U2). Direction: ship the anchor check as a **single reference
implementation** in the distill/validate references (a predicate that accepts a leading anchor, a
trailing `per X §`, or a bare in-domain `§N`) rather than as prose each author re-implements. The
last case is separately notable as the only false NEGATIVE — and it would have suppressed *this
diagnosis's own nudge*; the orientation author caught it by opening the file.

### P9 — `wrap-session/reconcile` · `input.report-insufficient` — 3 cases · weight 7
**Pattern:** Three times the chunk report omitted a fact, and **detectors read the report ALONE by
contract**, so no detector could propose the amendment. The second case is a verbatim recurrence of
the first, one chunk later. Rate 3/6. See also chain **X1**.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| desktop-a11y-sweep | The plan named the layout-templates 200 ms vs design-system 150 ms dialog-fade divergence; the report omitted it (this chunk changed neither value), so no detector could propose it. Escalated per Validate check 5, operator-ratified to fold in | 1 round |
| live-per-p-id-lamps | Expected-amendment #4 (coverage row anatomy) was proposed by NO detector because the report never carried the fact — **"a verbatim recurrence of the escalation the PREVIOUS wrap recorded (the 200 ms fade, same mechanism), so the corpus entry did not prevent it"** | 1 round |
| cross-surface | The report's site enumerations under-ran twice; the second is structural — `test-plan:278` carries the retired "negotiates DOWN" mechanism and **no `rmcp` token**, so the report's token-keyed classification could not reach it by construction | 4 extra reads |

**Proposal:** The mechanism is named precisely and identically in cases 1 and 2: **the plan's
expected-amendments list already holds the fact, and the report is the only channel to the
detectors.** The cheapest closure is to make the report's authoring step read the plan's
expected-amendments list and require a disposition line for each entry — including "not carried,
because this chunk changed neither value", which is exactly what both authors would have written.
That is Validate check 5's floor moved one step earlier, to where the omission happens rather than
where it is caught. Case 3 is a different cause and feeds P16.

### P10 — UNIVERSAL · `tooling.output-cap-overflow` — 5 cases · weight 6 · 3 steps · 3 chunks
**Pattern:** Five reads exceeded the tool-result cap; **four of the five name the same artifact
shape** — a file of a few dozen lines where each line is a multi-KB single-line entry.

**Evidence:** ALL 5 cases —

| chunk | step | artifact | recovery |
|---|---|---|---|
| desktop-a11y-sweep | research | a 6 MB WebdriverIO driver log holding the decisive axe rule identity as an embedded JSON payload | anchor-grep + structural re-extraction; 1 reformulation, 2 reads |
| live-per-p-id-lamps | research | `.claude/rules/verification-harness.md` — **42.7 KB in 57 lines** | grep the 5 section headers, then two offset-bounded Reads; 2 reads |
| screen-reader | take-up | two directive-hunting greps (42.5 KB, then 36.4 KB) | persisted + targeted re-extraction; 2 reads |
| screen-reader | research | `.claude/rules/verification-harness.md` — **95 lines, ~45.6 KB, one multi-KB line per Session Addition** (mandated in-full read) | three targeted extractions; 3 reads |
| screen-reader | report | `.andromeda/playbook.md` — **114 lines, 49 KB** | grep index + sed ranges for 8 rules; 2 reads |

**Proposal:** Feeds level candidate **L2** — the cause is an artifact shape Andromeda's own skills
author, and every fix so far has been a per-read workaround. Two narrower directions available
immediately: (1) `verification-harness.md` is the repeat offender and its in-full read is *mandated*
by the research step — either the mandate or the file's shape has to give; (2) a standard
"structural read" recipe for one-entry-per-line artifacts (header grep → offset reads) belongs in
the references rather than being re-derived by each author, which is what all five recoveries did.

### P11 — UNIVERSAL · `tooling.host-shell` — 3 cases · weight 6 · 3 steps · 2 chunks
**Pattern:** Three host-shell semantics failures. **One of them is the evolve system's own mandated
recipe failing.**

**Evidence:** ALL 3 cases —

| chunk | step | what | impact |
|---|---|---|---|
| p-075-assert-round | distill | **5 of 7 distiller returns arrived HTML-escaped** — `<`/`>` as `&lt;`/`&gt;`, mangling exactly the tokens the content turns on (`<run_id>`, `<slo_tier>`, the literal `"<redacted>"`) | 0 |
| — | orientation | Bash cwd persisted after an in-project `cd` inside a compound command, silently rebasing later calls; an out-of-project `cd` emitted a reset notice, so **the two cases were observably asymmetric and only the silent one misled** | 2 retries |
| desktop-a11y-sweep | reconcile | **The six sidecar appends died with "unexpected EOF while looking for matching quote" — attempted as the inline python heredoc, `<<'PY'`-quoted, exactly as `evolve-system.md` prescribes.** Rewritten as a scratchpad file run by path | 1 retry |

**Proposal:** Case 3 is the sharp one: `evolve-system.md` mandates a quoted-heredoc python append
*because* shell quoting has corrupted JSON on this host — and the mandated form itself failed on
entry bodies carrying backticks and apostrophes. Direction: change the mandated recipe to
**write-the-payload-to-a-file-then-run-by-path**, which is what both this case and Epoch 4's
identical failures actually did. Feeds **L1**.

### P12 — `wrap-session/curation` · `ambiguity.filter-borderline` — 5 cases · weight 6
**Pattern:** Five borderline dispositions, and **three of the five are the same numeric artifact**:
a candidate scoring EXACTLY 0.6 on Filter 4 — the documented mass point that rejects
deterministically — because measurement (+0.4) plus specific-technical-detail (+0.2) is the natural
score of a single measured discovery with no repeat signal. Rate 5/8, 3 of 6 chunks.

**Evidence:** ALL 5 cases —

| chunk | what | impact |
|---|---|---|
| — (no-op wrap) | The route-annotation separator candidate sat at the Filter-1 dedup edge — same trigger context and heavy token overlap, but naming a failure mode the existing entry lacks; resolved as an additive facet amended in place | 1 read |
| — (no-op wrap) | The single surviving candidate scored EXACTLY 0.6 on a first pass; re-counting against the repeated-pattern signal changed the disposition rather than the judgment (the same shape fired on three separate events) | 2 reads |
| p-075-assert-round | **THREE of four candidates landed at EXACTLY 0.6** and the deterministic rule carried all three | 1 read |
| webview-self-verify | **Filter 4 rejected THREE separate candidates at EXACTLY 0.6** — custom-protocol-not-profile, tsc-does-not-prove-ESM-loadability, wdio-BiDi-vs-classic. Each scored measurement + specific-detail and nothing else, being a single discovery event. **"All three are load-bearing for the very next ch…"** | 0 |
| screen-reader | Two measured gotchas scored exactly 0.6 and were rejected under the exact-hit lean default — the abort-flag polling placement and the PowerShell `$null` P/Invoke behaviour; the second was carried anyway as a facet of a surviving entry | 1 iter |

**Proposal:** The scoring rubric has a **structural mass point at exactly the threshold**, and the
class that lands there is identifiable: a fact verified by measurement, technically specific, and
occurring once. Across this epoch that rubric rejected at least eight such candidates, and the
webview author recorded that all three of its rejects were load-bearing for the immediately
following chunk. Directions: (a) move the threshold off the mass point (0.6 rejects → 0.6 accepts,
or the rubric's weights change so a two-signal hit is not exactly at the boundary); (b) add a
"load-bearing for a named upcoming entry" signal worth +0.2, which is the property the authors kept
reaching for; (c) leave it and accept that single-event measured facts are deliberately not
curated. Note case 5 shows authors already routing around it via the additive-facet rule.

### P13 — `implement/code` · `input.plan-step-ambiguous` — 5 cases · weight 5
**Pattern:** Five plan steps under-determined the mechanism at implement time. Rate 5/6 — nearly
every chunk. **None under-determined the OUTCOME**; in four of five, implement settled the mechanism
and recorded the deviation.

**Evidence:** ALL 5 cases —

| chunk | what | impact |
|---|---|---|
| webview-self-verify | Plan step 2 carries a clause the operator's later mid-turn directive supersedes (PATH fallback vs skip-at-exit-0). **Both cannot hold.** Implemented the directive | 0 |
| desktop-a11y-sweep | Step 5 and the concretized acceptance both say all SIX lamp labels carry label + glyph, but **no release-bundle surface renders all six** — the all-six Gallery is `import.meta.env.DEV`-gated and stripped from the custom-protocol build, a fact neither plan nor research established | 3 reads |
| live-per-p-id-lamps | Step 1 said to wrap the envelope DB read in a manual `tracing::info_span!` — **unsatisfiable as written**: obs-plan's bounded span-name set has no `db.*`, so it would widen a spec enumeration implement may not author | 2 reads |
| screen-reader | Step 5 placed the parser in the spec's `after` hook and step 7 required a post-leg census with NVDA terminated; **the two cannot both hold** | 0 |
| cross-surface | Step 2 offered "a test, or a small `#[cfg(test)]`-reachable helper" without saying WHO invokes it at e2e time, and step 4 said the seed should "invoke the committed Rust seeder" without naming the invocation | 2 reads |

**Proposal:** Three of the five are **internal contradictions the plan could have caught** (steps 2
vs a later directive; step 5 vs step 7; step 1 vs a spec enumeration the plan cites). A P5 check
that reads the plan's steps against each other and against the enumerations they name — rather than
each step against the materials — would cover that class. The other two are honest
under-determination that implement resolved correctly; the deviation records are doing their job.

### P14 — `implement/fix-loop` · `contract.test-expectation` — 3 cases · weight 5
**Pattern:** Three reds that were the harness's own assertion rather than the product. Rate 3/6.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| desktop-a11y-sweep | The reduced-motion assertion substring-matched the AUTHORED spelling `animation: none` while the CSSOM re-serializes it as the expanded shorthand — **a correct rule failed** | 1 iter |
| desktop-a11y-sweep | The driven arm's failure message dumped the ENTIRE rendered document (~8 KB) because the activeElement helper fell back to `textContent` and landed on BODY; the fact was legible only after re-reading | 1 read |
| live-per-p-id-lamps | The first draft used wdio's `expect(value, message)` two-arg form, which does not exist in `@wdio/globals` (TS2554 at three sites); rewritten to fold the observed context INTO the asserted value | 1 retry |

**Proposal:** Cases 1 and 3 are both *asserting against an authored representation rather than the
runtime one*. A `.claude/rules/frontend.md` or a11y-harness line — "assert against the CSSOM's
serialization, never the authored declaration; fold context into the asserted value rather than a
message argument" — would carry both. Case 2 is a bounded-failure-output convention.

### P15 — `wrap-session/reconcile` · `contract.cascade-miss` — 4 cases · weight 4
**Pattern:** Four facts that no detector owned, each found by the orchestrator's own post-apply
sweep rather than by the fan-out. Rate 4/6, 4 of 6 chunks.

**Evidence:** ALL 4 cases —

| chunk | what | impact |
|---|---|---|
| p-075-assert-round | **Two FALSE claims sat inside sentences the cascade was already rewriting**, neither surfaced by any detector: `obs-plan:35` said the MCP client was "via rmcp 1.7.0" (removed 2026-06-27) and `security-summary:14` said the child-stdout boundary used a "bounded prost decode" | 3 reads |
| desktop-a11y-sweep | The a11y detector proposed an edit to `rules/frontend.md`'s **Session Additions** — a preserve-verbatim curation home the cascade must NEVER edit. Content right, channel wrong; routed to P3 curation | 0 |
| live-per-p-id-lamps | A cross-master citation grep found `a11y-plan` §9 restating `test-plan` §6's retired wording — **a lateral citation no detector owned**, since each master's detectors evaluate its own body | 1 read |
| screen-reader | The a11y detector left **three restatements of the retired wording standing** that only the orchestrator's post-apply retired-token grep over all seven masters caught | 3 reads |

**Proposal:** Cases 1, 3 and 4 are one shape: **a claim restated in a document whose own detectors
do not evaluate that claim's domain.** The orchestrator's post-apply cross-master sweep caught all
three, which argues it is load-bearing and should be a named, mandatory step rather than a habit —
and, per case 4 and the epoch's Tier-1 curation, it must sweep for **what the claim SAYS, not what
it is NAMED after** (the `test-plan:278` miss in P9 is the same lesson). Case 2 is a channel rule
the fan-out reference could state to the doc-agents directly.

### P16 — UNIVERSAL · `contract.narrow-basis-claim` — 3 cases · weight 3 · 3 steps · 1 chunk
**Pattern:** Three claims made from a basis narrower than the claim — an external repo's ledger, a
sub-agent's enumeration, and a dependency-graph inference. All three would have passed unnoticed.

**Evidence:** ALL 3 cases —

| chunk | step | what | impact |
|---|---|---|---|
| — | orientation | An incoming cross-repo relay requested a round asserting three delegated budgets; **Conductor's own ledger records that exact scope already delivered** | 3 reads |
| cross-surface | research | The tests distiller reported "26 lines of test-plan.md carry rmcp" with a partial enumeration; `grep -c` measures **24**, and the distiller's list omitted 8 sites the author's own grep found. **"Neither enumeration is the answer — the count is the basis"** | 1 read |
| cross-surface | fix-loop | Both the plan and the operator's review edit predicted `Cargo.lock` BYTE-UNCHANGED; measured, **the lock DOES move by exactly one line** (a dependency EDGE), while the package count held 564 → 564 | 0 |

**Proposal:** All three were corrected in-session and two are already curated to Tier 1 /
`rules/security.md`. The remaining pipeline generalization is case 2: **a sub-agent's enumeration is
routinely treated as a count.** The fan-out reference could require distillers to return the
DERIVATION (the command and its output) alongside any count, so the consumer inherits a basis rather
than a list. Note this class is confined to one chunk — it may be that chunk's character rather than
a trend.

### P17 — `wrap-session/gates` · `tooling.gate-deferral` — 3 cases · weight 3
**Pattern:** Three gate deferrals, all rule-sanctioned. Rate 3/7. Included because the *first* is
not a deferral at all but a gate that would never have run.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| p-075-assert-round | **An ACCEPTANCE CRITERION named a gate the plan's Test Commands did not list** (the 40th cargo-audit probe). The light gate re-runs Test Commands only, so the criterion would have been reported met on the strength of its own text while nothing ran it | 1 read |
| webview-self-verify | The chunk's own ui-smoke gate is DRIVEN-RED by design; its skip arm ran green (exit 0, host-path-free precondition, fetch recipe), which is the reading the operator recorded in the route CARRY | 1 deferred |
| screen-reader | nextest + clippy deferred under the source-delta rule (TS/Markdown/PowerShell/NVDA-ini only, `Cargo.lock` byte-unchanged); **pinned as PREREQ on the next chunk, which touches Rust and ran them as mandatory — closure confirmed in the handoff** | 1 deferred |

**Proposal:** Cases 2 and 3 are designed behaviour with case 3's closure verified — no action. Case
1 is a real gap and it is the same class as **P6 case 4**: an acceptance criterion naming a gate
that no step will run. One check covers both — at P5, every gate an acceptance criterion names must
appear in the plan's Test Commands.

### P18 — `wrap-session/report` · `contract.detector-fact-gap` — 3 cases · weight 3
**Pattern:** Three facts with no natural home in the report template — and since detectors read the
report alone, a fact with no bullet is a fact no detector sees. Rate 3/6. Same mechanism as P9/X1,
seen from the producer's side.

**Evidence:** ALL 3 cases —

| chunk | what | impact |
|---|---|---|
| desktop-a11y-sweep | **A fix that was WRITTEN, KEPT, and proved INSUFFICIENT.** "Reverted / negative API facts" covers written-then-reverted; "Spec claims disproved" covers a falsified assertion; neither fits "correct and shipped yet not the remedy it was written to be" | 0 |
| live-per-p-id-lamps | The Counts bullet needed an extra read to establish the OLD value detectors would grep for — `obs-plan` §1:49 bakes the literal "all SEVEN handlers" plus the enumeration; the bullet had to carry doc+line, not just the delta | 1 read |
| cross-surface | Authoring found a fact **NO gate had caught**: the chunk introduces `CONDUCTOR_E2E_SEED_DIR`, an eleventh handle absent from architecture's enumeration — **and the chunk's own plan acceptance said "no new `CONDUCTOR_* env var"** | 2 reads |

**Proposal:** Two directions. (1) A report-template bullet for case 1's class — a change that is
correct, shipped, and insufficient — since the existing two bullets bracket it without covering it.
(2) Case 3 is the epoch's clearest instance of the report doing a gate's job: the implement gates,
the light gate and P5's checks all passed a chunk that contradicted its own acceptance criterion, and
only report authoring caught it. A P5 or light-gate check asserting each acceptance criterion against
the actual modify-set would move that catch upstream — the same check P6 case 4 and P17 case 1 want.

---

## Cross-step chains (starting heuristics)

### X1 — `wrap-session/report` →`report`→ `wrap-session/reconcile` — **3 chunks** (above the ≥2 bar)
The only shape recurring across chunks, and it is the epoch's cleanest chain. In all three the
producer's outcome is `ok` with **no quality signals**, and the consumer rated the artifact `thin`:

| chunk | producer | consumer verdict |
|---|---|---|
| desktop-a11y-sweep | `report`, outcome `ok`, signals `[]` | "sufficient for 41 of 42 proposals, but two gaps: it omitted the self-obs log LANDING-SITE move and the 200 ms/150 ms fade divergence **its own plan listed as an expected amendment**" |
| live-per-p-id-lamps | `report`, outcome `ok`, signals `[]` | "sufficient for 26 of 27, but OMITTED the coverage-row-anatomy divergence the plan listed as an expected amendment — **detectors read the report alone**, so layout-templates proposed nothing" |
| cross-surface | `report`, outcome `ok`, signals `[]` | "the Spec-claims-disproved enumeration under-ran at TWO sites the detectors found — `a11y-plan:465` and `test-plan:278` (carrying no `rmcp` token)" |

**Chain hypothesis:** the report step has no way to know it is under-producing. It has no
completeness predicate — it reports `ok` with empty signals in all three — while the consumer is
the only party that can tell, one step too late. The pairing with P9 and P18 is exact: the producer
records `contract.detector-fact-gap` (no home for the fact) and the consumer records
`input.report-insufficient` (the fact never arrived).

**Proposal direction:** give the report step a *closing* predicate it can evaluate itself — the
plan's expected-amendments list, each entry dispositioned. That converts a downstream `thin` verdict
into an upstream checklist, and it is the same fix P9 proposes from the other end.

### X2 — `phase/plan` →`plan`→ `wrap-session/gates` — 1 chunk (below the ≥2 bar; surfaced)
p-075: producer `ok`, no signals; consumer rated the plan `thin` — "the plan's Test Commands OMITTED
the cargo audit probe that its own acceptance criteria named, so the light gate would not have
re-run it; caught at P5 instead and discharged there." One chunk only, but it is the *same missing
predicate* three separate proposals arrive at (P6 case 4, P17 case 1, P18 case 3), which is why it
is worth naming despite being under threshold as a chain.

### X3 — `implement/smoke` →`conversation`→ `wrap-session/{report, curation}` — 1 chunk, 2 consumers
screen-reader: the producer carried the signal `recorded-not-rerun`; **the context window was
compacted before wrap started**, and both wrap consumers rated `conversation` thin. Both recovered
from the summary and `evidence/leg-verdict.md`, but the report had to re-read the symbol set, the
evidence identity block and the a11y-plan claim sites. A producer that *signalled* honestly and a
consumer that still suffered — the artifact simply does not survive compaction. Below threshold at
one chunk; worth watching, since long live-leg chunks will keep hitting it.

### Non-join recurrences (the heuristics do not reach these — no in-epoch producer)
- **`tree-db` rated `thin` in 2 chunks**, both FALSE NEGATIVES from the calls view: p-075 — "the
  calls view under-resolves `&self` async methods — 0 rows for `query_incident_list` despite two
  production call sites"; cross-surface — "cannot see a caller inside a macro-expanded
  `#[tauri::command]` body — `read_envelope` returned rows=0 while grep found a real caller at
  `commands.rs:243`". Feeds **L4**.
- **`wrap-playbook` rated `thin` in 2 chunks** — the consumed-quality twin of P2.
- **Master specs rated `thin`/`wrong` in 4 records** across 2 chunks (`layout-templates` 3 stale
  200 ms sites; `obs-plan` §1:49 contradicting itself inside one sentence; `a11y-plan` §6 missing a
  contrast pair §1 requires, and a dangling §3 pointer). Detector-invisible by construction — this
  is the same surface P15 covers.

---

## Level candidates (systemic-masked-as-project)

Pass A clustered all 26 `workaround`/`prohibition`/`removed-cause` facts by the obstacle each routes
around. Six themes reached threshold; cross-epoch recurrence was checked read-only against Epoch 4.

### L1 — band-aid — **host-shell defeats the mandated write recipe** — 6 facts in-epoch, 17 in Epoch 4
**Facts (Epoch 5):** orientation — host grep rejected `-oP` (non-UTF-8 locale), re-extracted with sed
· orientation — Bash cwd persisted after an in-project `cd`, two relative reads failed · reconcile
(lamps) — **"the validated-python sidecar append via a quoted heredoc died on the host shell with
'unexpected EOF' despite `<<'PY'` quoting; re-ran the identical payload as a scratchpad FILE by
path"** · research (webview) — captured to `/tmp` instead of the mandated session scratchpad ·
implement/code (webview) — same deviation again, **"a REPEAT of the same deviation recorded at this
session's research step, so the corrective did not carry across steps"** · research (a11y-sweep) —
`$TMPDIR` is empty in this shell, so a redirect resolved to `/e2e-run.log` and died Permission
denied. Typed correlate: `tooling.host-shell` n=3 (P11).

**Epoch 4 recurrence:** **12 host-shell facts + 5 scratchpad facts**, including four consecutive
document writes routing around one heredoc parse failure across take-up → research → plan → validate
→ report, and two `removed-cause` facts where the heredoc corrupted a payload silently (a `$TMPDIR`
probe resolving to `/val.py`; a shell collapsing doubled backslashes into an invalid escape).

**Level hypothesis:** the cause lives in the **environment** (Git Bash on Windows: no `-oP`, empty
`$TMPDIR`, heredoc parse failures on markdown/backtick bodies) and in the **pipeline references**
that prescribe shell-based writes; every fix so far has landed in the project, one chunk at a time,
as an individual author re-deriving "write it to a file and run it by path." Epoch 4→5 shows a real
improvement (17 → 6), which is the corrective working — but the residue is the *mandated* recipes
themselves: `evolve-system.md`'s quoted-heredoc append and the `.tmp`-then-rename atomic writes are
both still prescribed and both still fail here.

**Proposal:** update the prescriptions to match what works on this host — payload-to-file-then-run
for structured appends, the Write/Edit tools for document writes — rather than leaving each author
to route around them and record a deviation. The scratchpad half needs a different lever: two of the
six are "used `/tmp` instead of the mandated scratchpad", and the second explicitly notes the
correction did not carry across steps within one session, which suggests exporting the scratchpad
path as a shell variable rather than restating it.

### L2 — band-aid — **multi-KB single-line artifacts defeat the Read/Write/Edit tools** — 5 facts in-epoch, 1 in Epoch 4 (**growing**)
**Facts (Epoch 5):** route-resolve — "the skill prescribes whole-content writes via the Write tool;
the working-route edit was made with an **index-slicing Python script** instead, because the target
entry line is **2646 chars** and the migrating PREREQ 1347" · curation — "curation-guide prescribes a
whole-content Write for the Tier-1 append to CLAUDE.md; made with an anchored Edit instead, because
re-transcribing a 131-line file whose USER block holds **four entries of ~10k chars each**…" ·
take-up (a11y-sweep) — "the Read tool's offset addressing gave no file-end signal on a 108-line file
of multi-KB single-line entries, so a bash `tail -c`/`cat -A` probe was used" · research (lamps) —
`verification-harness.md` (42.7 KB / 57 lines) exceeded the cap; structural re-extraction · plus
untyped: "the Epoch-6 split is a REORDER of multi-KB single-line entries, which the anchored-Edit
path cannot do without reproducing each entry's full text (and risking silent truncation)".

**Typed correlates:** `tooling.output-cap-overflow` n=5 (P10 — four of five name this exact shape);
`tooling.long-line-edit` n=3 across two steps (below threshold per-step, cited here): "Edit's
read-before-edit precondition is not satisfied by a Bash sed read, so each line already read
structurally had to be re-read"; "swapping two entries with two Edits makes the first edit duplicate
a line, so the second edit's anchor becomes ambiguous".

**Epoch 4 recurrence:** 1 fact — orientation reading `working-route.md` whole exceeded the cap.

**Level hypothesis:** the cause is an **artifact shape Andromeda's own skills author** — one
multi-KB line per route entry, per master-route record, per Session Addition, per curation entry —
interacting with tool constraints (a result-size cap, offset addressing with no end signal,
read-before-edit, anchor ambiguity). Every fix has been a per-read or per-write workaround inside a
chunk. This is the epoch's clearest chronic-degrade too: 5 occurrences, **zero halts**, growing 1 → 5
across the boundary — silent degradation the halt policy structurally never surfaces.

**Proposal:** the shape is a deliberate choice (one line per entry keeps `grep`/`sed` line-oriented
and makes freeze-checking byte-exact), so the direction is probably not to change it but to supply
the tooling it needs: a small reference script for line-addressed entry read/replace/reorder over
these artifacts, invoked by path — which is independently what three separate authors built ad hoc
this epoch. Alternatively, a hard length budget per entry with overflow moving to a sidecar.

### L3 — band-aid — **live-Pulse unavailability absorbed per chunk** — 5 facts in-epoch, 4 in Epoch 4
**Facts (Epoch 5):** implement/smoke (p-075) `deferred` — "sidecar absent from PATH, nothing on
`:4317`, all three `ANDROMEDA_PULSE_*` unset. Deliberately NOT driven anyway" · fix-loop
(a11y-sweep) `deferred` — "Arm B could not execute: no `pulse-app` process… Conductor does no Pulse
process management by scope law, so launching it is the operator's" · gates (a11y-sweep)
`workaround` — "the driven arm is an operator-gated live leg and `pulse-app` was down at gate time,
so it re-verified by THIS session's recorded artifact rather than a cold re-run, **which would have
measured the absence and false-redded the gate**" · report (screen-reader) `deferred` — "Pulse was
already down at wrap, so the hold-dialog assertions could not re-run" · gates (screen-reader)
`workaround` — same treatment, plus "the orchestrator-added driven-arm re-run went red on the same
absent precondition".

**Epoch 4 recurrence:** 4 `deferred` facts, including a **soft-exit** ("the four live-leg Test
Commands could not run — `andromeda-pulse-mcp` and `conductor` absent from PATH and no `pulse-app`
running") and "launching `pulse-app` is long-running GUI process management on the operator desktop".

**Level hypothesis:** the cause lives in the **environment/process boundary** — Pulse's lifecycle is
operator-owned by scope law, and the loop has no way to know whether it is up before scheduling work
that needs it. The fixes live in the project: each chunk deferring, each wrap re-verifying from
recorded artifacts, each plan absorbing a live leg it may not be able to run. Note this is *correct*
per-instance behaviour every time — which is precisely why it never halts and never accumulates a
visible cost.

**Proposal:** a cheap probe the loop could run at plan time and at gate time — is anything on
`:4317`, is the sidecar on PATH, are the three handles set — turning "deferred, environmental" into
a *stated precondition with a known state at scheduling time*. That would let phase decline to
schedule a live-leg chunk into a session that cannot run it, instead of discovering it at implement.
It also directly serves P3's finding, since the same probe would catch the boot/preflight collision.

### L4 — band-aid — **the code-graph's calls view returns false negatives** — 3 facts in-epoch, 3 in Epoch 4
**Facts (Epoch 5):** research (cross-surface) `workaround` — "The code-graph could not answer the
chunk's central impact question — `read_envelope`'s calls view returned rows=0 with the symbol
confirmed indexed and the plane fresh, because its only caller sits inside a macro-expanded
`#[tauri::command]` body" · plus the two `tree-db` `thin` consumed verdicts (`&self` async methods;
the same macro case).

**Epoch 4 recurrence:** "no caller row for `declares` because its only call site sits inside a
closure" · "the code-graph was the intended first instrument for impact and returned **a false leaf**
(0 callers on an indexed symbol that has a real caller), so the impact basis was re-derived with
grep" · a `deferred` fact about the refresh premise.

**Level hypothesis:** the cause lives in the **pipeline tooling** — the SCIP calls view misses at
least three Rust shapes (closure call sites, `&self` async methods, macro-expanded attribute bodies),
each producing a 0-row answer indistinguishable from "leaf, no callers". The project has absorbed
this as a Tier-1 curation rule ("a 0-row result means consulted-but-no-match ONLY once you have
confirmed the symbol IS indexed") — a correct project-level band-aid over a tooling defect that has
now recurred across two epochs with three distinct mechanisms.

**Proposal:** the three shapes are now individually named, which makes them testable. Either the
`code-graph-views.sql` calls view grows cases for them, or — cheaper and honest — the query wrapper
emits an explicit `INCONCLUSIVE` rather than `rows=0` when the symbol is indexed but no call edge is
found, so the caller never has to distinguish "leaf" from "blind spot" by memory.

### L5 — band-aid — **the 0-pending / no-op adaptation path is underspecified** — 3 facts + 3 `missing` verdicts
**Facts:** route-resolve `workaround` — **"The 0-pending no-op path has no sanctioned write channel
for an AUDIT-LEDGER record-integrity correction. Its permitted surface is state.yaml, curation
tiers, the markerless route tail, adaptation-record.md, the handoff, and amendments for facts the
wrap itself measured — a committed code-audit `record.json` is none of those, yet the operat…"** ·
route-resolve `workaround` — "**Ran P5 route-resolve BEFORE P3 curation, inverting the order** the
no-op path lists them in; done so curation could scan the route outcome as material, but it is a
reorder of the phase sequence the skill marks NEVER reorder" · route-resolve (screen-reader) — the
directive named no slot for the sidecar-spawn entry; the orchestrator chose one and recorded it.

**Consumed verdicts on that path:** `report` `missing` ×2 ("0-pending adaptation path runs no P1, so
there is no chunk report"), `plan` `missing` ×1 ("light gate is a no-op by construction, not a
skip").

**Level hypothesis:** the cause lives in the **process** — the no-op path was specified as a
degenerate case of the full wrap and inherits its phase order and channel list, but the work it
actually does (operator-directed route adaptations, ledger corrections) has different needs. The
fixes are per-invocation: one author inverted a NEVER-reorder sequence, another had no channel for a
required write.

**Proposal:** specify the 0-pending path in its own right — its phase order, its permitted write
channels (including whether a committed audit-ledger correction is one), and what stands in for the
report as the outcome basis. Note this path ran **9 route-resolve / 8 curation / 7 gates** steps this
epoch against 6 chunks, so it is a third of the wrap traffic, not an edge case.

### L6 — override — **operator directives supersede derived dispositions: 2 → 11 across the boundary**
**Facts:** 11 `overridden` facts in Epoch 5, all `nature: process`, versus 2 in Epoch 4. By step:
`phase/validate` 6 · `phase/plan` 1 · `phase/take-up` 1 · `wrap-session/{reconcile, route-resolve}`
2 · `new-session/orientation` 1.

Two distinguishable sub-themes:

**(a) The `AskUserQuestion` dialog form is declined and answered in plain text — 2 in-epoch, 0 in
Epoch 4, + 1 untyped record.** webview/plan: "the operator declined the AskUserQuestion
multiple-choice form for the msedgedriver fork, asked to clarify first, then supplied the resolution
directly as a mid-turn message"; screen-reader/validate: "the AskUserQuestion dialog carrying the
Apply? / directive / findings questions **was denied by the operator**, who answered in plain text;
the prompt was re-issued as text". The untyped record adds the mechanism: **"the skill's literal
prompt form 'Apply? (yes / review / cancel)' is a text prompt, and the dialog tool is not what the
skill prescribes."**

**(b) An out-of-band directive supersedes the loop's derived disposition — 6 facts.** The priority
ladder resolved to rung 4 and the relay pre-directed a different chunk · a P5 approval carried a
directive changing steps 8–9 · three escalate-class proposals were applied **without the prescribed
HALT** on a pre-recorded wrap directive · a PHASE directive overrode a CARRY's stated mechanism · a
directive's enumeration contradicted the file's order, escalated as trajectory · a first P5 "yes"
was superseded by a consolidated relay answering "review", sending the run back to P4.

**Level hypothesis:** these need opposite readings and only the founder can separate them. (a) looks
like a **mechanism defect**: two skills reach for a dialog tool their own prescribed prompt form does
not call for, and the operator declined both times — the RULE (or the skill's prompt spec) is what is
miscalibrated. (b) may not be friction at all: an operator steering an autonomous pipeline through
recorded directives is the design, and every one of the six was absorbed without loss. But the 5.5×
growth is real and concentrated at `phase/validate` (6 of 11), which is the step whose job is to
converge on an approved plan.

**Proposal:** for (a), settle what the P5 review prompt actually is — a text prompt or a dialog —
and state it once, so the form stops being re-decided per author. For (b), no proposal; surfaced as
a rate the founder may want to read as a signal about where directives are landing relative to where
the loop asks for them.

### Observations (below the band-aid signature, recorded)
- **A `product-logic` workaround theme at n=5 does NOT meet the band-aid signature** (which requires
  `nature ∈ environment/resources/process`), and should not be read as one: implement measured a
  plan's product premise false and worked around it in the leg — `drive_run`'s abort polling; the
  sidecar console pane; `latest_run_id` taking the second seeded run's slot; `StatusLamp` exposing no
  role handle; a manual span that would widen a bounded spec enumeration. These are the loop finding
  real product facts, which is what it is for.
- **`removed-cause` n=2** (below the n≥3 threshold): a root-anchored `/logs/` gitignore letting an
  untracked webview log ride `git add -A`; a latent flake where the driver spawn forces
  `CONDUCTOR_RUNS_DIR` for every suite. No recurring removed-cause theme this epoch.
- **The standing cargo-audit deferral is now 45 chunks old** (43 consecutive + 1 un-probed + this
  epoch's). It is an *externally-caused* bounded wait, operator-ratified at the 6th re-check per
  `rules/security.md`, and the audit↔deny overlap is verified green each time — so it is not a
  deferred-forever hit. Recorded because the age is the thing the ratification exists to keep visible.
- **No deferred-forever signature fired.** The one closure this epoch could verify — screen-reader's
  nextest/clippy source-delta deferral, pinned as a PREREQ — is confirmed closed in the next chunk.

---

## Playbook-extension candidates (untyped patterns, F-4)

19 true-untyped records (13.8%, up from 11.6%). Four clusters reach the F-4 threshold.

### U1 — `wrap-session/curation` + `wrap-session/reconcile` — 3 cases → **the `recall.*` family's criteria, not a new type**
**Cluster:** the same phenomenon — *a curated entry existed and did not prevent its own recurrence* —
occurred three times and was recorded three different ways.

| chunk | recorded as | what |
|---|---|---|
| p-075 | **untyped** | "RECURRENCE-DESPITE-LEARNING. The surviving candidate's dedup match is the 2026-06-27 verification-harness entry recording that the rmcp-server stub HID a raw-shape bug — a defect-plus-remedy entry that did not prevent this recurrence one level down (envelope fidelity was fixed; ITEM-KEY fidelity was never stated, so the stub drifted again)" |
| desktop-a11y-sweep | `recall.curated-rule-not-applied` (+ mis-flagged `untyped:true`) | "Filter 1 dedup turned three of my session 'findings' into RECURRENCE-DESPITE-LEARNING: **every one was already curated in verification-harness.md and I rediscovered each from scratch**" |
| live-per-p-id-lamps | `recall.corpus-recurrence` (+ mis-flagged `untyped:true`) | "**SECOND consecutive wrap** in which a literal-token probe produced false results that an existing Tier-1 curation entry ('before grepping for a token as a proxy for a practice…') was supposed to prevent" |

**Epoch 4 recurrence:** "The unset-variable redirect hit at implement P2 is ALREADY documented
verbatim in `docs/session-learnings.md` 2026-08-18 as trap (2), including the exact failure."

**Draft criteria line (for the `wrap-session/curation` playbook):** *"Record
`recall.corpus-recurrence` whenever a session finding turns out to be already curated — including
when the curated entry states the rule correctly and the work reproduced the failure anyway. Use
`recall.curated-rule-not-applied` only when the entry was consulted and misapplied; the two are not
interchangeable, and an author unsure between them should prefer `recall.corpus-recurrence` rather
than leaving the record untyped."*

Both types already exist and each sits at n=1, below threshold — the class is n≥3 and only looks
sub-threshold because it splits three ways. Worth noting the substance too: the epoch's own Tier-1
curation about token-proxy checks was written 2026-09-01 and violated at the next wrap (P8 case 7),
which is the strongest evidence that curation alone does not close this class.

### U2 — `new-session/orientation` + `wrap-session/gates` — 3 cases → **the evolve-nudge predicate itself**
**Cluster:** the derived condition "no `proposals.md` NAMES this epoch" was mis-evaluated three
times across two epochs, in both directions.

| chunk | what |
|---|---|
| — (orientation) | "matched a passing mention: the 2026-08-20 run TARGETS Epoch 3 and only cites Epoch 4 in one sentence, so **a literal grep would have suppressed a due nudge**; opening the file to read its target heading cost the one extra read" |
| — (orientation) | "`grep -ioE 'Epoch [0-9]+' proposals.md \| head -8` truncated before Epoch 4, so the first pass could not tell whether the 2026-08-22 diagnosis covered Epoch 4… **A head-limited view is never the answer when the question is set membership.**" |
| cross-surface (gates, typed `contract.token-proxy-check`) | "grepping for 'Epoch 5' returned a HIT in the 2026-08-22 run — which is titled 'Epoch 4' and mentions Epoch 5 only inside a timestamp annotation. **A false negative: the nudg…**" |

**Epoch 4 recurrence:** "The evolve-nudge derived condition nearly suppressed a nudge that should
fire. The 2026-08-20 diagnosis contains the string 'Epoch 4' twice, but both are incidental."

**Draft criteria line (for `new-session/orientation` and `wrap-session/gates`):** *"The evolve-nudge
predicate is satisfied only by a diagnosis whose TARGET epoch is this one — read the run's title
heading, never a substring match over its body, and never a `head`-limited view of the matches."*

A universal type (`contract.token-proxy-check`) already covers the class, so no new type is
proposed; what is missing is the specific guard, on the one derived condition that has now been
mis-evaluated four times across two epochs — including one occasion that would have suppressed *this
very diagnosis*.

### U3 — `wrap-session/{report, gates}` — 2 cases across 2 epochs → proposed type `input.implement-outcome-unsettled`
**Cluster:** wrap inherited an implement outcome that was not settled, and the loop's contract
assumes it is.

| epoch | what |
|---|---|
| Epoch 5 (p-075) | "The report had to carry a CORRECTION OF ITS OWN AUTHOR'S PRIOR REPORT: the implement P4 output stated the smoke was skipped for want of a live Pulse, and **the operator then supplied warm binaries so the leg ran and passed**. The template's Decisions-and-corrections bullet absorbed it, but **the flow assumes implement's report is a settled inpu…**" |
| Epoch 4 | "wrap-session was invoked on a chunk whose /implement had SOFT-EXITED (environmental), leaving its claimed capability v2-20 at `ref:null`/`status:planned`. **The loop has no sanctioned path for that state**" |

Meets F-4 on the recurrence arm at exactly n=2 (1 in-epoch + 1 recurring) — borderline, flagged as
such.

**Draft criteria line (for the `wrap-session/report` playbook):** *"Record
`input.implement-outcome-unsettled` when implement's report is not the final word on its own chunk —
because it soft-exited, because a step ran after it, or because the operator changed a precondition
between the two. Name which, and what stood in as the outcome basis."*

### U4 — `wrap-session/route-resolve` — 2 cases + 1 problem-fact → proposed type `contract.no-sanctioned-channel`
**Cluster:** the 0-pending path needed a write it has no channel for, or an ordering it forbids.

| what |
|---|
| "The 0-pending no-op path has no sanctioned write channel for an AUDIT-LEDGER record-integrity correction. Its permitted surface is state.yaml, curation tiers, the markerless route tail, adaptation-record.md, the handoff, and amendments for facts the wrap itself measured — a committed code-audit `record.json` is none of those, yet the operat…" |
| "surfaced but NOT acted on (trajectory, unnamed by the operator): two markerless Epoch-6 entries carry the platform framing the last two chunks measured false… and the first's subject appears already shipped" |
| problem-fact: "Ran P5 route-resolve BEFORE P3 curation, inverting the order the no-op path lists them in… it is a reorder of the phase sequence the skill marks NEVER reorder" |

**Draft criteria line (for the `wrap-session/route-resolve` playbook):** *"Record
`contract.no-sanctioned-channel` when a required write has no permitted surface on the path being
run, or when a surfaced fact has no owner and the directive names no disposition — say which write or
fact, and what was done instead."* Feeds **L5**; two of the three are the same underspecified path.

---

## Below threshold — no action

**Typed groups (n=2, no halt/soft-exit):** `implement/fix-loop/contract.matrix-claim` ·
`phase/distill/contract.extract-format` · `phase/take-up/input.carry-context-gap` ·
`phase/take-up/input.out-of-pipeline-source` · `phase/take-up/tooling.long-line-edit` (cited in L2) ·
`phase/validate/contract.mechanical-check` · `phase/validate/contract.structural-blind-spot` (in P5) ·
`phase/validate/contract.token-proxy-check` (in P8) · `wrap-session/curation/ambiguity.tier-routing` ·
`wrap-session/gates/contract.token-proxy-check` (in P8) ·
`wrap-session/route-resolve/contract.carry-no-owner` ·
`wrap-session/route-resolve/contract.standing-pin-carriage`.

**Typed groups (n=1):** `implement/code/{ambiguity.scope-pressure, input.conventions-gap}` ·
`implement/fix-loop/{contract.narrow-basis-claim, contract.structural-blind-spot, retry.fix-iterations (weight 13 — the epoch's single heaviest record), tooling.gate-deferral}` ·
`implement/smoke/{contract.premise-falsified, contract.spec-reality-gap, contract.vacuous-check-found}` ·
`new-session/orientation/{contract.narrow-basis-claim, contract.skill-reference-drift, input.handoff-git-mismatch, input.route-state-ambiguous, tooling.host-shell}` ·
`phase/distill/{contract.binding-contradiction, input.spec-source-gap, tooling.host-shell}` ·
`phase/plan/{contract.premise-falsified, input.extracts-conflict, input.out-of-pipeline-source}` ·
`phase/research/{contract.narrow-basis-claim, retry.query-reformulation}` ·
`phase/take-up/{contract.premise-falsified, tooling.output-cap-overflow}` ·
`phase/validate/contract.intent-divergence` ·
`wrap-session/curation/{recall.corpus-recurrence, recall.curated-rule-not-applied}` (both cited in U1) ·
`wrap-session/gates/{tooling.commit-mechanics, tooling.light-gate-red}` ·
`wrap-session/reconcile/{contract.false-positive-proposal, contract.structural-blind-spot, tooling.host-shell}` ·
`wrap-session/report/{recall.change-reconstruction, tooling.output-cap-overflow}` ·
`wrap-session/route-resolve/{contract.premise-falsified, tooling.long-line-edit}`.

`contract.skill-reference-drift` at n=1 is noted separately because it is a **pipeline** defect
invisible to project drift detection: *"The skill body states Check 11 two ways — its Constraints
block says 'presence-only at session start' while its Phase 1 step 1 and `health-criteria.md`
prescribe presence + per-plane host-tool detection; resolved by running the superset."*

**Untyped clusters below threshold (emerging — watch next epoch):**
- **Timestamp locale (n=1):** `state.yaml` `last_wrap` and the handoff `Last Updated` carry a `Z`
  suffix on a LOCAL-time value, so the dashboard's "last wrap {ago}" yields a negative age.
- **Route-grammar underspecification (n=1):** the skill says "not the italic preamble" without
  naming the marker; `working-route.md` uses underscore-italic, so an extractor testing `*…*`
  labelled 4 preamble lines markerless.
- **`AskUserQuestion` form (n=1):** the P5 review prompt issued as a dialog was denied and answered
  in plain text — cited in **L6(a)**, where the pattern reaches threshold via the problem-facts.
- **Master-vs-master divergence found by an aggregate check (n=1):** layouts "names no `.ps1`
  variant" vs test-plan "REQUIRES `agent-run.sh` and `.ps1` expose identical…".
- **A CARRY's own site table incomplete (n=1):** the fresh sweep found a FIFTH site.
- **Scope expansion at take-up (n=1):** the chunk had more buildable content than the directive's
  "research-and-record" framing implied.
- **Script console output mislabelling (n=1):** the flip-compaction script called 36 already-clean
  lines "freight with no recognizable separator".
- **Reorder tooling (n=1):** cited in **L2**.
- **Filter-4 mass point (n=1):** cited in **P12**.
- **Under-asserting shipped spec (n=1):** the axe assertion checks only `violations.length`, so a
  real failure names no rule; the same spec checks 3 of the 9 contrast pairs its plan requires.
- **Positive observations (n=2, no friction):** bounded-subprocess discipline held with nothing to
  intervene on — five headful driver sessions left zero stray processes; a real `tasklist` census
  before and after the `--e2e` leg found ports 4444/4445 released.

**Chain shapes below the ≥2-chunk bar:** X2 (`plan` → `gates`, 1 chunk) · X3 (`conversation` →
`report`/`curation`, 1 chunk, 2 consumers).

---

*Diagnosis complete. Nothing above has been applied, queued, or remembered — the mechanism's
responsibility ends with this document.*
