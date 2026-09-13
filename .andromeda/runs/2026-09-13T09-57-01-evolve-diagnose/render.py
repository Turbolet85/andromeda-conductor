"""Render proposals.md from the q-*.json twins plus authored analysis. Writes only into the run dir."""
import json, os, re, collections

RUN = os.path.dirname(os.path.abspath(__file__))
EPOCH = 'Epoch 6b — Polish & ship'
STAMP = '2026-09-13T09:57:01Z'
J = lambda n: json.load(open(f'{RUN}/{n}', encoding='utf-8'))
health, typed, chains, level, untyped, retr = (
    J('q-health.json'), J('q-typed.json'), J('q-chains.json'),
    J('q-level.json'), J('q-untyped.json'), J('q-retractions.json'))


def cell(s, n=175):
    s = (s or '').replace('|', '\\|').replace('\n', ' ').strip()
    # Scheme elision: a quoted record may spell a URL scheme, whose `letter + colon + slash`
    # run satisfies the project's host-path anchor [A-Za-z]:[\/] though it is not a path
    # (host-win32.md, 2026-09-11 as extended 2026-09-12). Elide in the OUTPUT, never loosen
    # the anchor; the elision is stated in Mechanism health.
    s = re.sub(r'([A-Za-z]):(//)', r'\1[:]\2', s)
    return s[:n] + ('…' if len(s) > n else '')


def imp(d):
    if not d:
        return '—'
    return ', '.join(f'{k} {v}' for k, v in d.items() if v)or '—'


# ---------------------------------------------------------------- authored prose
P = {
'TYPE-ALONE :: contract.narrow-basis-claim': (
 "A count, absence or availability claim was stated from a source narrower or less authoritative than the claim itself, in **12 of the epoch's 14 chunks** — the widest-spread and highest-weight class in the ledger. Rate per step-record roughly tripled against the prior epoch (5/106 = 0.047 at Epoch 6a → 27/194 = 0.139 here). Whether that is more errors or better detection is not separable from the ledger alone: the type has existed since 2026-08-15, so the rise is not a deploy artifact, and the project curated three separate sweep-discipline rules during this epoch.",
 "The class is already curated as a project T1 rule and recurred anyway (see P5 and L4). A corpus entry is a recall mechanism; the direction worth considering is an **affordance at the point of claim** — a plan/report field that requires the derivation command beside any count, absence, or 'every/no' quantifier, so the basis travels with the number instead of being reconstructible only by re-deriving it."),

'TYPE-ALONE :: contract.premise-falsified': (
 "Verification falsified a premise an authored artifact states, in **13 of 14 chunks** — the most evenly distributed class in the epoch. Rate per step-record is flat against the prior epoch (13/106 = 0.123 → 22/194 = 0.113), unlike the two classes above it.",
 "Premise falsification is verification working, so the count is not itself a defect signal; the flat rate across two epochs supports reading it as the pipeline's steady state. The actionable half is **where the premise entered**: none of these records names the authoring step that introduced the falsified premise. A provenance marker on authored premises (which step wrote it, from what source) would turn a falsification into a targeted signal about one authoring surface rather than a generic one."),

'TYPE-ALONE :: contract.token-proxy-check': (
 "A check, guard or derived condition tested for a TOKEN's presence where the intended property is semantic, and the match was satisfied by incidental, self-referential or prose-about-the-token content — **24 cases across 12 chunks**. The type was deployed 2026-08-31 and was used **once** in Epoch 6a before **24** times here, so the jump is uptake plus incidence together and the two cannot be separated from the ledger.",
 "Four of the epoch's six `prohibition` problem-facts are hand-written do-not-re-derive-this-with-a-bare-grep guards, each authored into ONE chunk's `scope.md` / `research.md` / `plan.md` (see L6). The knowledge is real, correct, and local: the next chunk does not see it. The direction is a **shared token-trap registry** — a place where 'this token is not that property, here is the pattern that misleads' survives the chunk that discovered it."),

'wrap-session/reconcile :: ambiguity.playbook-no-match': (
 "In **7 of 14 chunks** a drift proposal escalated to the operator because no playbook rule governed its class — carrying **3 halts and 6 dialogue rounds**, the only above-threshold group in the epoch bearing halt impact. The near-misses are strikingly uniform: in six of the seven the record says a rule SUBJECT-matched and then failed a narrowing qualifier (`:109`, `:58`, `:115`, `:137`, `:127` are each named as the near-miss).",
 "The escalations are not noise — each reached the operator and resolved — but a 50 %-of-wraps escalation rate on one step is a rule-set shape signal, not a project signal. Two directions: a **qualifier review** over the 43 rules whose narrowing clauses are producing the near-misses these records name site-by-site, or an explicit **residual-class rule** that governs 'subject matched, qualifier missed' so the escalation carries its own classification instead of arriving unclassified."),

'TYPE-ALONE :: recall.corpus-recurrence': (
 "A curated learning that states the rule CORRECTLY did not prevent its own recurrence — **12 times across 11 of 14 chunks**. The records are explicit about it: 'a curated T1 entry states the exact rule I then broke, twice in one wrap'; 'testing.md:86 already stated the rule, dated the SAME day, and the failure reproduced anyway'; 'the read-the-hits / narrow-basis corpus entry did not prevent its own recurrence'.",
 "Curation is the pipeline's primary improvement mechanism, and this group is the direct measurement of its transfer rate: the corpus is correct and is not reaching the moment of the act. The direction is **recall at the point of use rather than at the point of capture** — surfacing the governing entry when a step is about to perform the act it governs (a sweep, a count, a smoke-command choice), instead of relying on the whole corpus being in context. Note the diagnosis cannot judge whether the fix is worth its cost; the measurement is that 11 of 14 chunks paid the recurrence."),

'TYPE-ALONE :: tooling.host-shell': (
 "The host shell's semantics, quoting or encoding corrupted a command or its output — 8 cases across 6 chunks. Rate is **flat** against prior epochs (0.038 at 6a → 0.041 here), and every case is a mechanism `host-win32.md` either already documents or gained an entry for.",
 "A flat rate under a growing corpus of host rules suggests the rules are catching the class after the fact rather than preventing it. No pipeline change is obviously indicated; the founder may prefer to read this as the steady cost of the host and leave it. Listed because it clears threshold, not because it has an evident remedy."),

'TYPE-ALONE :: tooling.output-cap-overflow': (
 "A read exceeded the tool-result size cap and was recovered by chunked or structural re-extraction — 10 cases across 6 chunks, up 2.7× in rate on the prior epoch (0.019 → 0.052). **Every case names the same root artifact shape**: `master-route.md` records and `.claude/rules/verification-harness.md` entries stored as multi-KB single lines. One record marks a same-day recurrence on one artifact; another marks a third occurrence in a single session.",
 "This is the read-side face of L1 and shares its cause. The chunk-local remedy (structural re-extraction) is already prescribed and works; what recurs is that every reader must discover the need for it. Direction: see L1 — the proposal is about the artifact format, not about the readers."),

'phase/validate :: contract.mechanical-check': (
 "The P5 mechanical checks did not cover a plan defect the operator's review then caught, or fired by construction on a chunk shape they do not fit — 6 cases across 5 chunks. Three further phase/validate records carry the same shape UNTYPED (see U2), taking the class to 9 in-epoch.",
 "The records split two ways and the split matters: some are **coverage gaps** (a defect outside every mechanical predicate) and some are **fit failures** (check 4's producer clause firing by construction on a CI-probe chunk; check 7 WARNing unavoidably on a formatting-only chunk because its SKIP keys on file plane rather than symbol change). The first asks for new predicates; the second asks for the existing ones to carry a chunk-shape precondition. Worth considering separately."),

'wrap-session/route-resolve :: contract.carry-no-owner': (
 "A CARRY or follow-up had no self-evident owner among the markerless entries — 3 cases, one carrying a halt and two carrying dialogue rounds. Each resolved by an operator placement call.",
 "Ownership assignment is currently a judgment made per CARRY with no rule to fall back on, and the one halt shows it can block. A direction: a **default-owner rule** (for instance, the first entry whose scope opens the artifact the CARRY names — which is exactly how the `a11y-plan.md:115` dittography was placed on `v3-03` at this epoch's last wrap), with the operator call reserved for CARRYs that rule cannot place."),

'TYPE-ALONE :: contract.structural-blind-spot': (
 "A documented mechanism failed to reach something BY CONSTRUCTION — no amount of correct execution would have caught it — 6 cases across 5 chunks. One names this skill's own sibling: 'orientation's documented input set contains no external-CI read', which is how a stale BLOCKED-ON premise survived.",
 "By-construction blind spots are the class that never improves through diligence, so they are the highest-value entries here per case. Each names its own boundary precisely; the direction is to treat these six as a **standing list of known-unreachable surfaces** rather than as incidents, so a future step can consult what the pipeline structurally cannot see before designing a check that assumes it can."),

'phase/distill :: input.spec-source-gap': (
 "A spec master was self-inconsistent, contradicted a sibling master, or carried a physically damaged entry — 6 cases across 6 chunks. Two are structural corruption in the same sidecar: a TRUNCATED amendment entry breaking mid-word, and an entry whose body is physically interleaved with the following entry (both `layout-templates-amendments.md`, the 2026-09-07 a11y-ci-gate amendment).",
 "The corruption pair is a different thing from the inconsistency four and is the more urgent: a damaged sidecar entry silently degrades every future distillation that reads it. Direction: a **sidecar integrity check** at write time (entry boundaries well-formed, no interleave) would catch the class the distiller can currently only report after the fact."),

'wrap-session/curation :: ambiguity.filter-borderline': (
 "A curation candidate scored EXACTLY 0.6 — the contract's named scoring mass point, which rejects — so its fate turned entirely on a conditional signal, in 6 cases across 5 chunks. Four records state the 0.6 coincidence explicitly; one reports three of four candidates at or near it.",
 "A scoring scheme whose reject threshold sits exactly on the modal score of real candidates decides by tie-break rather than by score. The direction is a **threshold or signal-weight review** — either move the mass point off the decision boundary, or make the conditional signals that are currently deciding into first-class scored terms."),

'implement/fix-loop :: contract.vacuous-check-found': (
 "A check that could not fail was found during the fix loop — 4 cases across 4 chunks: a `git diff --exit-code` over an artifact untracked at implement time; a `Select-Object -First 1` over a directory the driver creates two of; a shell status verb reading the last journal line as the envelope when a per-check record is last.",
 "All four were found by measurement, none by review, and each had been authored past a plan review. The direction pairs with P8: a **vacuity probe** on authored gates — run the check against a state where it MUST fail, before accepting it as a gate — which is precisely the control the epoch's last chunk used on `the_gate_discriminates`."),

'implement/fix-loop :: contract.spec-reality-gap': (
 "A spec master states a gate, tool posture or capability the shipped tree does not satisfy — 4 cases across 4 chunks, including `test-plan` §9 naming `cargo fmt --check` as a Lint-stage gate that the tree had apparently never satisfied and no CI job ran.",
 "Each was discovered by an implement step attempting to rely on the spec. This is the measured instance of the project's own curated rule that a spec describes TARGET state; the pipeline-level direction is to mark spec claims that assert a CURRENT capability distinctly from those that assert an intended one, so a planner can tell which it may build on."),

'phase/take-up :: input.out-of-pipeline-source': (
 "The chunk's decisive fact lived outside every artifact the pipeline enumerates — 5 cases across 4 chunks: a constant in the SUT's source, a GitHub Actions run, an operator's launcher script present in neither repo, an operator-tunable SUT env handle.",
 "take-up's input set is defined by the pipeline's own artifacts, and four of these five facts were load-bearing for the chunk's central design call. Direction: an explicit **external-source slot** on the route entry or the CARRY — naming where a fact lives when it lives outside the tree — so take-up fetches it rather than discovering its absence."),

'wrap-session/report :: input.implement-outcome-unsettled': (
 "Implement's P4 report could not stand as the wrap report's outcome basis — 5 cases across 5 chunks. **Every case has one mechanism**: an operator wrap directive issued BETWEEN implement and the report superseded, corrected or re-dispositioned implement's findings.",
 "This pairs with cross-step chain X1/X2, where the same handoff artifact reads `thin` in 12 of 14 chunks. The handoff is not failing — the operator directive is doing real work — but the pipeline treats implement's report as the outcome basis while in practice a later directive is. Direction: make the **directive an explicit input to the report step** with its own slot, rather than something the report absorbs while still nominally sourcing implement."),

'wrap-session/reconcile :: contract.cascade-miss': (
 "An amendment's cascade under-enumerated its own sites — 5 cases across 5 chunks: a duplicate left standing one line from a site that was found; a body edit landing inside a quotation of another master; a citation sweep finding one of three sites.",
 "In two cases the orchestrator's sweep caught what a doc-agent's own sweep missed, which is the overlap working. The direction is narrower than 'sweep better': three of the five turn on **quotation and duplication within a master** — a master quoting another master's text is the shape that defeats per-document sweeps. A quote-aware site enumeration would address the class the generic sweep structurally cannot."),

'wrap-session/gates :: tooling.result-not-run-stable': (
 "A gate graded differently at the light gate than at implement over the same tree — 4 cases across 3 chunks: a live leg grading ManualCheck vs KnownResidual, an nsis installer differing by 4 264 bytes, a mutation index artifact, and an exit-status side file colliding with the /implement run's file of the same name.",
 "The last is a plain collision with a clear fix (per-run gate log names). The other three are genuine non-determinism in surfaces the project's determinism bar does not cover. Direction: the **light gate's re-run semantics** need a stated position on which command classes are expected to reproduce byte-exactly and which are not — currently every divergence must be diagnosed from scratch."),

'phase/take-up :: tooling.long-line-edit': (
 "Reading and editing the taken-up entry required chunked offset-bounded extraction because the entry is a single multi-KB line — 4 cases across 4 chunks (7561, 4808, 2873 and 2031 characters named). One produced a **false coordinate-mismatch finding stated to the operator before correction**, from a truncating probe.",
 "The write-side face of L1. The false finding is the notable cost: the workaround is not merely slower, it has produced a wrong claim. See L1."),

'TYPE-ALONE :: contract.skill-reference-drift': (
 "A skill body or reference states a path, enumeration or rule that mismatches deployed reality or a sibling reference — 4 cases across 2 chunks. Named: `curation-guide` leaving the correction-against-a-generated-body case unowned; setup-project's `rules-templates/host-win32` body stating a false host mechanic (`cd` 'does not persist'); `promotion.md` step 2 enumerating foldable annotations without CONTEXT, which route-resolve also writes; check 4 clause (9)'s definition of `new`.",
 "These are pipeline defects invisible to project drift detection, which is exactly why the type exists. All four are small and concrete. The direction is simply that they have **no routing surface today** — they are recorded here and nowhere else, so the proposal is a place for them to land."),

'new-session/orientation :: input.handoff-git-mismatch': (
 "The handoff's Branch field disagreed with measured git state — 4 cases. **All four are the same mechanism in the same direction**: the handoff predicts 'N ahead and unpushed' and names the push load-bearing; the operator then pushes; the next session measures 0 ahead.",
 "The handoff is written before an act it correctly predicts, and read after it. That is not a defect in either side — it is a field whose truth expires between write and read. Direction: have the wrap write the **push as a stated expectation** ('1 ahead at wrap; the operator pushes after') rather than as a state claim, which is what the current handoff has in fact started doing in prose while the Branch field still reads as a measurement."),

'phase/take-up :: input.carry-context-gap': (
 "A folded CARRY mis-described or under-counted its own subject, or cited coordinates that had drifted since authoring — 4 cases across 4 chunks. In two the promotion fold's mandated re-verify caught it; in one, three of seven CARRYs cited drifted coordinates and three under-counted their site sets.",
 "The re-verify is working and is the reason these are recorded rather than propagated. The direction is upstream: a CARRY cites coordinates in an artifact that keeps moving, so the citation decays by construction. Consider having a CARRY name its subject by a **stable anchor plus a re-derivation command** rather than by line coordinates — the same shape P1 proposes for counts."),

'wrap-session/report :: contract.detector-fact-gap': (
 "The report template had no bullet shaped for the fact the chunk produced — 3 cases across 3 chunks: a Dependencies bullet shaped for lockfile deps against a CI-fetched binary; no stock bullet distinguishing an amended claim from a correct one sharing the same token; a report-time coordinate sweep that under-counted.",
 "Small and specific. Direction: the template's bullet set is enumerable and these three name the gaps precisely."),

'phase/distill :: contract.extract-format': (
 "A distiller return arrived in a shape the consuming step did not expect — 3 cases across 3 chunks: twice the subagent transport HTML-entity-escaped 6 of 7 returns (`&lt;run_id&gt;`, `&lt;host-path&gt;`), once a distiller added an unexpected line between the header and the first section.",
 "The transport escaping is anticipated by `fan-out.md` and handled by the raw-twin rule, so the recurrence is the handling cost, not a surprise. It is listed because it clears threshold; the direction, if any, is to move the decode into the transport boundary rather than into each consumer."),
}

LEVEL = [
 ("L1", "band-aid", "Multi-KB single-line entries in Andromeda's own artifacts defeat the prescribed read AND edit forms, at every skill",
  ["**12 `workaround` problem-facts** across two faces, plus two typed groups and one untyped record.",
   "**Write face (6 facts)** — the skill bodies prescribe an anchored Edit for a point change and a whole-file Write for a compaction; both are defeated by an entry stored as one multi-KB line, so every skill substitutes a scratchpad python read-modify-write run by path: `wrap/route-resolve` (route/playbook/drift-base entries), `phase/take-up` ×2 (`2026-09-05-audit-corrective`, `2026-09-06-coverage-completeness-gate`), `wrap/curation` (`verification-harness.md`, a 26 KB line), `wrap/route-resolve` (`residuals.md`, `testing.md:54`), and `wrap/gates` inverting the prescription the other way — one anchored Edit instead of the prescribed whole-file rewrite, to avoid re-emitting 130 untouched multi-KB lines byte-exactly.",
   "**Read face (6 facts + typed group `tooling.output-cap-overflow` n=10)** — a structural grep over `master-route.md` or `verification-harness.md` returns whole multi-KB records and blows the tool-result cap: 29.7 KB, 38.5 KB, 38.6 KB, 40.8 KB, 60.4 KB, 56.2 KB, 31.6 KB measured. One record marks a same-day recurrence on the same artifact; another marks a third occurrence in one session.",
   "**Typed correlates:** `tooling.output-cap-overflow` n=10 / 6 chunks (rate 2.7× on the prior epoch) and `phase/take-up :: tooling.long-line-edit` n=4 / 4 chunks — the latter including a **false coordinate-mismatch finding stated to the operator** from a truncating probe.",
   "Natures: environment 7, process 5."],
  "The cause lives in the artifact FORMAT — Andromeda's route, master-route, residual and rule-file entries are authored as one line each. The fixes have all lived in the project: a per-skill workaround, re-invented at `take-up`, `route-resolve`, `curation`, `gates`, `research` and `orientation` independently, and now partly codified as a host rule. No skill owns the format; every skill pays for it.",
  "Two directions, both pipeline-side. (a) **Change the format** — if entries could wrap, anchored Edit and ordinary grep would both work and twelve workarounds plus ten cap overflows would have no occasion. (b) **Own the workaround once** — a single shared extraction/patch helper in `andromeda-tools/` that every skill calls, instead of each skill's body prescribing a form its own artifacts defeat. The measured cost of the status quo is 22 recorded events in one epoch and one wrong claim reaching the operator."),

 ("L2", "band-aid", "The session-level auto-mode directive and this project's PreToolUse guard point in opposite directions",
  ["**8 events**: 3 `workaround` facts, 2 `removed-cause` facts, 2 untyped frictions, 1 typed `tooling.host-shell`.",
   "`new-session/orientation` ×2 (untyped): 'the session-level auto-mode directive instructs making file changes via Bash heredocs rather than the Write tool, which the project's own PreToolUse guard blocks outright for file-target heredocs; the two rules point opposite ways and the conflict surfaces only as a blocked call' — and, a second session, 'the first script-authoring call exited 2 and was re-authored via Write'.",
   "`wrap/reconcile` (`2026-09-08-webview2-runtime-152-installed-in-job`): 'first attempted the sidecar appends as a `cat >> file <<EOF` heredoc; the PreToolUse hook blocked it … which is the same rule the skill's own constraints state'.",
   "`phase/distill` (`2026-09-08-hosted-runner-webview2-session`, typed `tooling.host-shell` + a `removed-cause` fact): 'the guard fired exactly as designed and cost one call'.",
   "`new-session/orientation` ×2 more (`workaround` + `removed-cause`): the health-check script and three structural extractors authored via Write instead, per `host-win32.md`.",
   "Natures: environment 2, process 3 (plus the untyped pair)."],
  "The cause is a **configuration contradiction between two layers** — a session-mode directive that is not project-aware, and a project guard that is correct. Every fix has been one agent, mid-step, absorbing one blocked call. The guard is doing its job in all 8 cases; what recurs is the collision, not a failure.",
  "The guard should stay. The direction is to make the **session-mode directive project-aware** — or to have the project's own rules state the exception in a place the directive-following agent reads BEFORE its first authoring call, rather than discovering it as an exit-2. Cost measured: 8 events, at least 3 wasted calls, and it is the only theme in the epoch where the pipeline contradicts itself rather than the environment."),

 ("L3", "band-aid", "First-authored shell command forms are defeated by this host, at a flat rate across five epochs",
  ["**5 `workaround` facts** plus typed group `tooling.host-shell` n=8 / 6 chunks.",
   "Facts: a python probe assuming one result shape crashed on a KeyError (`phase/research`); `grep -rn` over `crates/` returned 1.6 MB by matching untracked `node_modules` (`phase/research`); `tr -d '`- '` aborted on a reverse-collating character range (`phase/validate`); Bash cwd persistence broke a second parallel call's relative `cd` (`new-session/orientation`); an `rm -rf`-bearing compound was denied by the permission layer (`implement/fix-loop`).",
   "Typed cases name: `rev` absent from this MSYS coreutils; backslash mangling in an inline `python -c`; cwd persistence again; `sed` reading a Windows backslash path as an unterminated s-command; unescaped `(` under `grep -E` returning a confident wrong answer twice; `\\+` under BASIC regex; a Windows-native python subprocess reporting all nine gate entries red.",
   "Rate across epochs: 0.088 (E3) → 0.144 (E4) → 0.032 (E5) → 0.038 (6a) → 0.041 (6b) — **flat for three epochs**."],
  "The cause lives in the host. Every fix has landed in `host-win32.md`, which is now substantial and is working — each of these was caught. But a flat rate under a growing rule file means the rules are catching the class after it fires, not preventing it: the agent authors the POSIX-shaped form first and learns on the failure.",
  "This may be the correct equilibrium and the founder may wish to leave it. If not, the direction is prevention rather than more rules: the recurring mechanisms are enumerable (backslash paths, ERE metacharacters, absent coreutils, cwd persistence, recursive-delete permission), and a **pre-flight lint on authored shell commands** against that short list would move the cost from post-failure to pre-execution."),

 ("L4", "override", "The operator's review is systematically supplying plan corrections the mechanical checks do not produce",
  ["**12 `overridden` problem-facts**, concentrated at two steps: `phase/validate` (4) and `wrap-session/report` (3), then `phase/take-up` (2), `implement/fix-loop` (1), `wrap/reconcile` (1), `wrap/gates` (1).",
   "At `phase/validate`: 'the operator returned five fixes at the review instead of approving. Four were plan defects I had shipped' (`dependency-polish`); 'the operator's R1 overrode the plan's central course … turning the chunk from remedy-plus-diagnostic into diagnose-only' (`hosted-runner-webview2-session`); 'the operator's review directed a scope expansion the plan did not contain … taking the gate from eight touched crates to all nine members' (`port-occupier-test-hygiene`); 'the operator's P5 review overrode two authored gate details the mechanical checks passed' (`release-build-and-bundle`).",
   "At `wrap-session/report`: the directive overrode default report content in four places (`dependency-polish`); corrected two measurements the report would have carried forward from implement (`port-occupier`); directed an out-of-band edit to a COMPLETED chunk plan before P7.1 (`live-pulse-in-lane-scenario-round`).",
   "**Correlates:** typed `phase/validate :: contract.mechanical-check` n=6/5 chunks; 3 further untyped phase/validate records (U2); `contract.structural-blind-spot` naming 'two real gate defects passed every P5 mechanical check and were caught only by the operator's review'; cross-step chains X3 (`phase/plan → plan → phase/validate`, 3 chunks) and X4 (`phase/plan → plan → implement/fix-loop`, 4 chunks)."],
  "The signature's reading is that the RULE is miscalibrated where an override recurs on the same call. Here the override recurs on the same STEP rather than the same rule: the P5 review is not correcting a rule the mechanical checks apply wrongly — it is supplying a class of judgment the checks structurally do not attempt (gate vacuity, cross-step consistency, scope completeness, chunk-shape fit). The fixes have all landed per-chunk, in that chunk's plan.",
  "Two readings, and the diagnosis cannot choose between them. Either the review IS the designed gate for this class and the mechanical checks are correctly scoped — in which case the finding is simply that P5 is load-bearing and its cost is real (4 review rounds with fixes returned, in 4 of 14 chunks) — or the classes the operator keeps supplying are enumerable and some are mechanizable. The records name what was supplied each time, which makes the enumeration cheap to attempt: vacuity of an authored gate (P13's control is the worked example), consistency BETWEEN individually-correct steps, and whether a check's SKIP condition fits the chunk's shape."),

 ("L5", "chronic-degrade", "`implement/fix-loop` degraded in 5 of 14 chunks and never halted — a 4× jump on every prior epoch",
  ["`ok-degraded` at `implement/fix-loop`: **5** here, against 1 (E3), 1 (E4), 2 (E5), 1 (6a). Plus **1 `soft-exit`** (`2026-09-06-halo-hue-budget-re-driven`).",
   "Chunks: `halo-hue-budget-re-driven`, `sr-findings-fixed` ×2, `webview2-runtime-152-installed-in-job`, `release-build-and-bundle`.",
   "The two `unresolved` problem-facts in the epoch both sit at this step: 'the CI a11y job's session creation never completes on the hosted windows-2025 runner … the remaining cause is unnamed and its resolution is a design decision, not a chunk-level code fix'; and 'the 3 failing atoms live in plan.md, which /implement must not modify, so the defect could not be fixed in-flight'.",
   "Epoch-wide `ok-degraded` is 5 (all at fix-loop) against 6, 4, 3, 3 in prior epochs — so the TOTAL is flat and the concentration is what moved."],
  "The cause appears to be a chunk-mix effect rather than a mechanism failure: this epoch's fix-loops ran against CI, a hosted runner, a screen reader and a live SUT — surfaces where a fix can be correct and still not close. The degradation is recorded honestly every time and never halts, which is the signature's concern: a step that always completes degraded never triggers the halt policy, so the trend is visible only in aggregate, exactly here.",
  "No fix is indicated for the degradations themselves. The direction worth considering is **visibility**: `ok-degraded` currently carries the same weight as `ok` in every consumer, so a step degrading in 5 of 14 chunks reads as a clean epoch everywhere except this ledger. A degraded-outcome count in the wrap report would make the trend legible without changing any policy."),

 ("L6", "band-aid", "Token-trap knowledge is written as a chunk-local prohibition and does not travel",
  ["**4 of the epoch's 6 `prohibition` facts** are guards against a naive token re-derivation, each authored into ONE chunk's artifact:",
   "`phase/take-up` (`dependency-polish`) — a hazard note in `scope.md` forbidding still-true security FLOORS from being swept up as stale version statements, enumerating 6 sites.",
   "`phase/research` (`dependency-polish`) — a do-not-touch guard in `research.md` naming `PiiCorpus::seeded(7)` at four sites, 'a SEED numerically equal to the arity, which any arity-7 sweep returns as a false positive'.",
   "`wrap/report` (`webview2-runtime-152`) — 'Detectors: do not propose retiring test-plan.md:455', written into the report because the naive reading of a measured falsity is a retirement proposal.",
   "`phase/validate` (`live-pulse-in-lane-scenario-round`) — a plan guard: 'Do NOT re-derive this with a bare token grep: `grep -c CountAtLeast` returns hits in seven files', because four scenarios carry the token only in retirement comments.",
   "**Typed correlate:** `contract.token-proxy-check` n=24 across 12 chunks."],
  "The cause is that the pipeline has no home for a fact of the form 'this token does not mean this property, and here is the pattern that misleads'. Each discovery is real and correct, and each is absorbed at the project level into the one artifact of the chunk that found it — a `scope.md`, a `research.md`, a `plan.md`, a `report.md` — none of which the next chunk reads.",
  "Direction: a **token-trap registry** at project scope (the nearest existing surface is the rule files, which already carry several such facts in prose). The measurement supporting it is that the class fired 24 times in one epoch while four chunks independently wrote local guards against it."),

 ("L7", "deferred-forever", "Two deferrals carry no named destination and no closure appears in the ledger",
  ["11 `deferred` facts in the epoch. **Nine carry an explicit destination** — routed to wrap as an Expected amendment, to P4, to a route-entry candidate, to the residuals append, or pinned as a PREREQ on the next markerless entry — which is designed behavior and not this signature.",
   "**Two do not.** (a) `implement/code` (`dependency-polish`): 'The eighth PiiCategory is unreachable from any scenario: conductor-core's `PiiCategorySpec` still has 7 variants and conductor-run's `wire_category` maps it 1:1 into the now-8-variant emit enum, which is not a match-exhaustiveness error, so the compiler stayed silent.' (b) `implement/fix-loop` (`dependency-polish`): 'knip ships usable only with an entry-point config: its 20 findings are 20 false positives'.",
   "Neither subject appears again in the ledger after 2026-09-07/09-08 — through the epoch's remaining 3 days, nor in the 97 records of the following epoch."],
  "The cause is that a deferral without a named destination has no reader. The nine routed deferrals demonstrate the mechanism works when a destination is supplied; these two simply have nowhere to have been noticed since.",
  "The lookback window is short (3 days in-epoch, plus one partial epoch), so this is a weak observation, not a demonstration of abandonment — stated as such. Direction, if any: make the **destination a required field** on a `deferred` problem-fact, so a deferral with nowhere to go is visible at capture rather than at diagnosis."),
]

U = [
 ("U1", "implement/smoke · wrap-session/gates", 3, "produced[].signals (no new type needed)",
  ["`2026-09-06-operator-gated-live-suite` / implement/smoke — 'A POSITIVE worth recording against a standing worry rather than a defect: the live suite's process teardown was exact … the pre-leg census and the post-leg census matched byte for byte.'",
   "`2026-09-06-coverage-completeness-gate` / wrap-session/gates — 'POSITIVE measurement of a control working: the commit printed LF will be replaced by CRLF warnings for every tracked text file it touched EXCEPT coverage-matrix.md, the one path .gitattributes pins eol=lf.'",
   "`2026-09-10-live-pulse-in-lane-scenario-round` / wrap-session/gates — 'A POSITIVE measurement worth keeping, recorded here because no listed type covers it and the closest one (tooling.result-not-run-stable) is its exact inverse. The light gate re-ran all three live legs … and REPRODUCED the round.'"],
  "**This is a known issue recurring, not a new one.** `evolve-system.md` already prescribes the remedy verbatim — 'a positive measurement of a mechanism WORKING … is a `signals` fact here with a note — never an untyped friction record, which carries cost (measured: three positives logged as untyped across two wrap steps — two of them held wrap/gates at 50 % untyped)'. The guidance exists and three more positives were logged as untyped anyway, two of them again at `wrap-session/gates`. So the proposal is NOT a new type: it is that the guidance is in the record shape section, where an author reaching for a friction type does not encounter it. Draft criteria line for the step playbooks' interrogation sections: *'A mechanism you expected to fail that held — a control firing as designed, a gate reproducing, a teardown that was exact — is a `produced[].signals` fact with a note, never a friction record.'*"),

 ("U2", "phase/validate", 3, "(existing type under-reached: `contract.mechanical-check`)",
  ["`2026-09-06-coverage-completeness-gate` — 'Two operator-caught plan defects no P5 mechanical check covers. (a) INCOMPLETE SWEEP OF A KNOWN CLASS … (b) UNCONSIDERED PORTABILITY AXIS: a byte-for-byte artifact comparison was planned with no line-ending consideration.'",
   "`2026-09-06-halo-hue-budget-re-driven` — 'Two plan steps were individually correct and jointly inconsistent … Nothing in the mechanical checks or val-1 looks for consistency BETWEEN steps; the operator's round-2 note found it.'",
   "`2026-09-07-a11y-ci-gate` — 'the plan authored a SECOND COPY of the envelope schema into the CI job … when the repo's own comment at ci.yml:167-171 explicitly rejects exactly that … The miss is not a research gap: P3 had ALREADY recorded the established gate shape under Patterns detected, and P4 still [authored it].'"],
  "These three are the same class as the typed group `phase/validate :: contract.mechanical-check` (n=6) — its first case reads 'four Test-Command defects outside every mechanical predicate', which is exactly what these say. Recorded untyped anyway, in three separate chunks. **Proposal is a criteria clarification, not a new type**: the playbook line for `contract.mechanical-check` evidently does not read as covering 'a defect NO check attempts', only 'a check that misfired'. Draft criteria line: *'`contract.mechanical-check` covers both a check that fired wrongly and a plan defect that NO mechanical predicate attempts — if the operator's review caught it and no check could have, it is this type.'* Folding them in takes the class to n=9 across 8 chunks, which would make it the epoch's fourth-largest."),

 ("U3", "phase/research · implement/fix-loop", 3, "proposed type `contract.instrument-validity`",
  ["`2026-09-07-dependency-polish` / implement/fix-loop — 'A dead-code tool installed to close an audit column produced a signal with a 100 percent false-positive rate on first run … knip reported 3 unused files, 5 unused devDependencies and 12 unused exports/types — all 20 wrong, and all one class: WebdriverIO discovers specs through wdio.conf.ts … so knip's import-graph reachability cannot see the package's largest consumer.'",
   "`2026-09-07-sr-findings-fixed` / implement/fix-loop — 'my own isolation probe (b) POSTed a session straight at msedgedriver, bypassing tauri-driver — which is the component that owns the WebView2-host translation — so a failure there could not by itself discriminate driver-image behaviour from a bypassed translation … A bypass probe needs its in-path twin to be evidence.'",
   "`2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` / phase/research — 'a diff classifier keyed on line.startswith(+) returned 0 added and 0 removed lines against 282 real hunks, because cargo fmt --check prefixes every +/- with an ANSI colour escape; the meaningless zero was visible only by dumping the raw output through cat -A.'"],
  "The distinguishing mechanism is that the INSTRUMENT could not see the subject it was credited with measuring — not a narrow basis (the basis was the right subject), not a token proxy (in two of three there is no token), not host-shell (the shell behaved). The project has already curated the rule in prose ('an INSTRUMENT reporting a cause is not the cause — validate a diagnostic form where the real path PASSES before building on what it says when it fails'), so the class is recognized; the evolve taxonomy has no name for it and three records went untyped. Draft criteria line: *'`contract.instrument-validity` — a probe, tool or classifier returned a result that was structurally incapable of reflecting its subject (a reachability model blind to the real discovery mechanism, a bypass probe missing its in-path twin, a parser blind to the data's encoding); record how the instrument was validated, or that it was not.'*"),
]
# ---------------------------------------------------------------- render
out = []
w = out.append
w(f"# Evolve Diagnosis — Conductor · {EPOCH} · {STAMP}")
w("")
w(f"Target epoch resolved in dialogue; **fully complete** (14 frozen working-route entries, 0 markerless, "
  f"all master records `complete`). Read-only pass; the only writes are this file and its `q-*.json` twins.")
w("")

# --- mechanism health
cov = health['chunk_coverage']
gapped = {k: v for k, v in cov.items() if v['gaps'] or k == 'None'}
w("## Mechanism health")
w("")
w(f"**Records:** {health['records']} in-epoch ({health['step']} step / {health['friction']} friction) across "
  f"{health['chunks']} chunks. **Unparseable:** {retr['unparseable']} over the whole {retr['records']}-record ledger. "
  f"**id fill:** {health['id_fill']} (all post-boundary). **problem-fact fill:** {health['problem_fact_fill']} step records carry at least one deviation fact.")
w("")
w(f"**Retractions (whole-ledger pre-pass):** {len(retr['retracted_ids'])} friction records retracted, "
  f"{len(retr['retracted_problems'])} problem-fact targets; **9 of the retracted ids fall inside this epoch and were dropped before every stage below**. "
  f"{len(retr['unresolvable'])} unresolvable, reported verbatim for manual discount:")
for u in retr['unresolvable']:
    w(f"- `id: {u[0]}` · {u[1]} · “{cell(u[2], 200)}”")
w(f"- **Retraction targeted by a retraction → founder review:** `{retr['retraction_of_retraction'][0]}`.")
w("")
w("**Checkpoint coverage** — step records present vs the expected playbook set (phase 5 · implement 3 · wrap 5). "
  "10 checkpoint firings are missing across 4 chunks; each was confirmed absent from the WHOLE ledger, not merely from this epoch's slice (i.e. not an epoch-label boundary artifact):")
w("")
w("| chunk | present | missing |")
w("|---|---|---|")
w("| `2026-09-05-audit-corrective` | phase 5 · implement 2 · **wrap 0** | `implement/code`, and every wrap checkpoint |")
w("| `2026-09-06-operator-gated-live-suite` | phase 5 · implement 3 · wrap 1 | `wrap/curation`, `wrap/reconcile`, `wrap/report`, `wrap/route-resolve` |")
w("| `2026-09-07-a11y-ci-gate` | phase 5 · implement 2 · wrap 4 | `implement/code`, `wrap/gates` |")
w("| `2026-09-07-sr-findings-fixed` | phase 4 · implement 3 · wrap 4 | `phase/plan`, `wrap/gates` |")
w("| `chunk: null` | new-session 1 · wrap 2 | — (a no-op/adaptation wrap legitimately carries `curation` + `route-resolve`) |")
w("")
w(f"**Untyped rate:** {sum(health['untyped_by_step'].values())}/{health['friction']} = {health['untyped_rate_overall']:.1%} overall. Per step: " +
  " · ".join(f"`{k}` {v}" for k, v in health['untyped_by_step'].items()) + ".")
w("")
w("**Outcomes:** " + " · ".join(f"`{k}` {v}" for k, v in health['outcomes'].items()) +
  ". All 6 `halted-resolved` sit at wrap (`reconcile` ×4, `route-resolve` ×2); all 5 `ok-degraded` and the 1 `soft-exit` sit at `implement/fix-loop` (see L5).")
w("")
w("**Calibration boundaries in range** — all Universal types were live for the whole epoch (earliest 2026-08-15, latest `contract.skill-reference-drift` 2026-09-02), so no absence here is an era artifact. Two observations:")
w("- **`contract.grammar-irregularity` has never been used, ledger-wide** (deployed 2026-09-09, live for this epoch's final 2 chunks). Whether no tool printed `UNPARSED:`/`INDETERMINATE:` or the type was never reached for is not separable from the ledger.")
w("- **`contract.token-proxy-check` was used once in Epoch 6a and 24 times here**, having been live since 2026-08-31. The jump mixes uptake with incidence; P3 states this rather than resolving it.")
w("")
w("**One elision in the quoted evidence** — a URL scheme appearing inside a quoted ledger record is rendered `x[:]//` rather than verbatim. The scheme's `letter + colon + slash` run satisfies this project's host-path anchor `[A-Za-z]:[\\/]` though it is not a path, and this run dir rides the next wrap's commit; per `host-win32.md` (2026-09-11, as extended 2026-09-12) the fix is to change the OUTPUT, never to loosen the anchor. One cell is affected, in P5.")
w("")
w("**Epoch-label spelling** — this epoch's records carry two spellings (`Epoch 6b — Polish & ship` ×427 and `### Epoch 6b — Polish & ship` ×38, the working-route header copied with its heading prefix). Folded before every group-by per the tolerant-parsing rule; a diagnosis that grouped by exact label would have under-counted by 38. One of this epoch's own `new-session` records predicted exactly this.")
w("")

# --- typed proposals
above = [g for g in typed['groups'] if g['above_threshold']]
below = [g for g in typed['groups'] if not g['above_threshold']]
w("## Proposals (typed patterns)")
w("")
w(f"{len(typed['groups'])} groups; **{len(above)} above the F-2 threshold** (n ≥ 3, or n ≥ 2 with halt/soft-exit impact). "
  "Universal and `recall.*` types are grouped by type across steps, per the diagnosis-pass rule. Ordered by weight. "
  "Every proposal carries ALL its cases.")
w("")
for i, g in enumerate(above, 1):
    pat, prop = P.get(g['key'], ("—", "—"))
    scope = g['scope'] if g['scope'] != 'TYPE-ALONE' else 'all steps'
    w(f"### P{i} — {scope} · `{g['type']}` — {g['n']} cases · weight {g['weight']}"
      + (f" · **{g['halt_bearing']} halt/soft-exit**" if g['halt_bearing'] else ""))
    w("")
    w(f"**Pattern:** {pat}")
    w("")
    rate = f" · rate {g['rate']} per step-run" if g['rate'] else ""
    w(f"**Evidence:** all {g['n']} cases · {len(g['chunks'])} chunks{rate}")
    w("")
    w("| chunk | step | what | impact |")
    w("|---|---|---|---|")
    for c in g['cases']:
        ch = c['chunk'] or '—'
        w(f"| `{ch}` | `{c['skill_step']}` | {cell(c['what'])} | {imp(c['impact'])} |")
    w("")
    w(f"**Proposal:** {prop}")
    w("")

# --- chains
w("## Cross-step chains (starting heuristics)")
w("")
multi = [s for s in chains['shapes'] if s['n_chunks'] >= 2]
w(f"{len(chains['shapes'])} distinct (producer → artifact → consumer) shapes; **{len(multi)} recur across ≥ 2 chunks**. "
  f"{len(chains['input_frictions'])} `input.*` frictions anchor the consumer side.")
w("")
CH = {
 'implement/fix-loop -> implement-outcome -> wrap-session/report':
  ("**The epoch's strongest chain by reach: 12 of 14 chunks.** The producer (`implement/fix-loop`) completes and its outcome artifact is consumed by `wrap-session/report` with quality `thin` in every one of the twelve. "
   "The typed correlate is P16 (`wrap-session/report :: input.implement-outcome-unsettled`, n=5), whose five records all name ONE mechanism: an operator wrap directive issued between implement and the report supersedes or corrects implement's findings. "
   "The chain is wider than that group because the thinness is recorded even where no friction was.",
   "The handoff is not broken — the directive is doing real work and the report absorbs it correctly. But the pipeline nominally sources the report from implement while in practice the later directive is the settling input, so `thin` is the honest verdict 12 times running. "
   "Direction: give the wrap directive **its own declared slot** in the report step's input set, so implement's report is consumed as one of two inputs rather than as the outcome basis it cannot be."),
 'implement/smoke -> implement-outcome -> wrap-session/report':
  ("The same shape from implement's other producing step, also 12 chunks, also `thin` throughout. The two chains share their consumer and their cause.",
   "Folds into X1 — one remedy addresses both."),
 'phase/plan -> plan -> implement/fix-loop':
  ("`plan.md` consumed `thin` or `wrong` by the implement fix-loop across **4 chunks**. The matching problem-facts are concrete: `plan step 6 specified EmissionShape::spans_per_dispatch()` where only `EmissionSpec` can see `signal`; `plan step 4 said 'the production render path in coverage.rs' without naming the function`, so the line first landed in a function the CLI's no-write path never calls; `plan steps 4 and 5 prescribed default-features = false on the MEMBER entries`, which cargo rejects outright for an inherited dependency.",
   "Every case is the plan naming a symbol, site or mechanism that reality rejects, corrected in place by implement. This is the implement-side face of L4; the validate-side face is X3. "
   "Direction: the three cases above are all **verifiable at plan time against artifacts the planner already has** (the type's own definition, the call graph, cargo's inheritance rule) — which is what makes them a candidate for a mechanical predicate rather than a review judgment."),
 'phase/plan -> plan -> phase/validate':
  ("`plan.md` consumed `thin` or `wrong` by its own validation step across **3 chunks** — the defect is visible one step after authoring, inside phase, before any review.",
   "Pairs with P8 and L4. That validate itself records the plan as thin in 3 chunks suggests some of what the operator supplies at P5 is already detectable at P4→P5."),
 'phase/research -> research -> implement/code':
  ("`research.md` consumed `thin` by implement across 2 chunks.",
   "Below the weight of X1–X4; recorded for completeness."),
 'wrap-session/report -> report -> wrap-session/reconcile':
  ("The wrap report consumed `thin` by reconcile within the same wrap, across 2 chunks.",
   "Correlates with P23 (`contract.detector-fact-gap`, n=3) — the report template lacking a bullet shaped for the fact reconcile then needs."),
}
for i, s in enumerate(multi, 1):
    hyp, prop = CH.get(s['shape'], ("—", "—"))
    w(f"### X{i} — {s['shape'].replace(' -> ', ' → ')} — {s['n_chunks']} chunks · {s['n']} joins · quality {'/'.join(s['quality'])}")
    w("")
    w(f"**Chain hypothesis:** {hyp}")
    w("")
    w("| chunk | producer outcome | producer signals | consumer verdict |")
    w("|---|---|---|---|")
    for c in s['cases'][:14]:
        sig = ', '.join(c['producer_signals'] or []) or '—'
        w(f"| `{c['chunk']}` | `{c['producer_outcome']}` | {cell(sig, 70)} | `{c['quality']}` — {cell(c['consumer_note'], 110)} |")
    w("")
    w(f"**Proposal:** {prop}")
    w("")

# --- level
w("## Level candidates (systemic-masked-as-project)")
w("")
w(f"Pass A clustered the epoch's {len(level['facts'])} problem-facts by the obstacle each routes around "
  f"(`workaround` {level['by_solution'].get('workaround',0)} · `overridden` {level['by_solution'].get('overridden',0)} · "
  f"`removed-cause` {level['by_solution'].get('removed-cause',0)} · `deferred` {level['by_solution'].get('deferred',0)} · "
  f"`prohibition` {level['by_solution'].get('prohibition',0)} · `unresolved` {level['by_solution'].get('unresolved',0)}; "
  f"natures: process {level['by_nature'].get('process',0)} · environment {level['by_nature'].get('environment',0)} · "
  f"product-logic {level['by_nature'].get('product-logic',0)} · resources {level['by_nature'].get('resources',0)}). "
  "Detection is independent of whether a typed correlate exists; where one does, it is named.")
w("")
for tag, sig, title, facts, hyp, prop in LEVEL:
    w(f"### {tag} — {sig} — {title}")
    w("")
    w("**Facts:**")
    for f in facts:
        w(f"- {f}")
    w("")
    w(f"**Level hypothesis:** {hyp}")
    w("")
    w(f"**Proposal:** {prop}")
    w("")

# --- playbook extensions
w("## Playbook-extension candidates (untyped patterns, F-4)")
w("")
w(f"{len(untyped)} untyped records in the epoch ({health['untyped_rate_overall']:.1%} of friction), hand-clustered. "
  "Three clusters clear the F-4 bar (n ≥ 3 in-epoch). Extending a playbook is an Andromeda change only the founder applies.")
w("")
for tag, where, n, ty, cases, draft in U:
    w(f"### {tag} — {where} — {n} cases → {ty}")
    w("")
    w("**Cluster:**")
    for c in cases:
        w(f"- {c}")
    w("")
    w(f"**Draft:** {draft}")
    w("")

# --- appendix
w("## Below threshold — no action")
w("")
w(f"**{len(below)} typed groups below F-2**, one line each:")
w("")
for g in sorted(below, key=lambda x: (-x['n'], x['key'])):
    scope = g['scope'] if g['scope'] != 'TYPE-ALONE' else 'all steps'
    w(f"- `{g['type']}` · {scope} — n={g['n']}, weight {g['weight']}, {len(g['chunks'])} chunk(s)")
w("")
w("**Untyped clusters below F-4:**")
w("")
w("- **Rust code-graph index incomplete at symbol grain** — n=2 untyped in-epoch (`operator-gated-live-suite`: `execute_scenario` returns 0 rows with only the enclosing module indexed, against three production callers found by grep; `live-pulse-in-lane-scenario-round`: `observe` returns 0 rows while two siblings in the same file are indexed and the crate carries 323 symbols), plus one `workaround` fact in the same epoch. **One instance short of the bar, and worth the founder's eye anyway**: the same phenomenon recurs at Epoch 4 (`tooling.graph-unavailable`), Epoch 4 again (`input.cookbook-gap`) and Epoch 5 (`retry.query-reformulation`) — *four different typings and two non-typings for one mechanism*, which is the shape a missing type makes. It does not clear F-4 because the PREVIOUS epoch (6a) carries no instance.")
w("- **Session-mode directive vs the project's PreToolUse guard** — n=2 untyped; not raised as a type candidate because the event is a configuration contradiction rather than a step outcome. Carried at full strength as **L2** instead (8 events).")
w("- **Measurement-basis divergence** — n=2: reading the run-report envelope from the frozen per-leg self-obs rather than the per-run journal (probe returned empty, self-announcing); and `+1581/−486` (rustfmt hunk rendering) vs `+1582/−487` (`git diff --numstat`) for one pass, both correct measurements of different things. Adjacent to U3 but a distinct mechanism.")
w("- **Singletons** (1 each): a directive citing a measurement no artifact carries; the handoff's Position field understating a head entry's CARRY count; two Tier-2 curation candidates with no rendered home falling to Tier 3; a take-up re-verify surfacing a CI defect the directive did not name; `fan-out.md`'s raw-twin rule applied mechanically producing a byte-identical twin; an NVDA `role=alert` region read whole, speaking a re-announce twice; a route entry having to carry an explicit retraction of a causal reading the same chunk committed; the friction ledger's own epoch-label split; a review round spent on tool FORM (`matrix.py show`) rather than substance; `implement` P1's self-check asking 'all New files written?' for a file the plan defers past implement by construction; a recurrence despite a curated learning.")
w("")
w("**Problem-facts below theme threshold** (1 each): the 13-minute mutation tier exceeding the Bash call cap (the epoch's only `resources` fact); `@crabnebula/tauri-driver` shipping an 8-line stub so its behaviour is unreadable from source; `curation-guide` prescribing an in-place correction to a body its own Tier-2 logic forbids touching; a failing test emitting no diagnostic through libtest because the product's panic hook routes it to the file sink; the light gate run through a Windows-native python wrapper reporting all nine entries red (the wrapper, not the gates); a stale `21.exit` deleted from a shared per-marker gate log dir; a spec body amended at P7 outside the phase that owns amendments, because drift = 0 could not hold over a claim the gate had just falsified.")
w("")
w("---")
w("")
w(f"*Read-only diagnosis. Evidence twins: `q-health.json` · `q-typed.json` · `q-untyped.json` · `q-chains.json` · `q-level.json` · `q-retractions.json` in this run dir. "
  f"Nothing here is applied, queued, or remembered: a re-run recomputes from the ledger alone, and a rejected proposal leaves nothing pending.*")

open(f'{RUN}/proposals.md', 'w', encoding='utf-8').write('\n'.join(out) + '\n')
print(f"wrote proposals.md · {len(out)} lines · proposals {len(above)} · chains {len(multi)} · "
      f"level {len(LEVEL)} · extensions {len(U)} · below-threshold {len(below)} typed groups")
