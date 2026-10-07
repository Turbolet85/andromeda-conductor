# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._
## 2026-10-07 — Tier-1 entry of 2026-09-17, moved whole: a token naming one arm of a fork cannot probe the fork

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-09-17: **A token that names one ARM of a fork cannot probe the fork, and a search summary is a claim ABOUT a source, never the source.** Two false readings in one chunk, each one step from a wrong terminal. (a) An attribution sweep keyed on `DevToolsActivePort` — the HIGH-integrity signature — returned 0 from the leg and was read as "the endpoint question was never asked at Medium"; at Medium the error is a DIFFERENT string (`from chrome not reachable`, wrapped across two log lines), so the sweep could only ever return 0. Every count was right and the inference was wrong. Before reading a 0 as absence, ask what OTHER wording the other arm produces. (b) A search engine's summary of an upstream issue asserted a mechanism ("WebView2 150+ hardening for elevated hosts") that fit the hypothesis already held so precisely it explained everything; fetching the issue showed it says no such thing. Most dangerous exactly when the summary matches what you already believe — fetch the source. Corollary measured the same day: **a generic error string is not a fact about its literal subject** — msedgedriver returns `DevToolsActivePort file doesn't exist` while running in PIPE mode, where no such file exists; it is its generic "could not reach the browser" text. And **on multi-KB single lines, `grep -c` counts LINES** — four corrections on one line read as 1; `grep -oE … | wc -l` is the occurrence count (mis-read twice in one wrap, both caught by re-counting).

---

## 2026-10-07 — Tier-1 entry of 2026-09-04, moved whole: a plan's steps can be jointly contradictory

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-09-04: **A plan's steps can each be individually unambiguous and jointly contradictory, and no gate looks for that.** One step froze the exact function another step predicted a measurable score change in — so the prediction was unreachable by construction, while both steps read as clear instructions on their own. Nothing caught it: the mechanical checks are structural (sections, paths, placeholders, size), validation-1 compares the plan to its SCOPE rather than to itself, and the single-executable-interpretation rule is stated per-step. The operator found it by measuring the claim. So when a plan predicts a MEASURABLE outcome — a score moving, a roster shrinking, a count falling — name the change that produces it and confirm no other step forecloses that change; and state such a prediction as something to MEASURE, with an explicit instruction to record what the measurement shows when it disagrees, never as an expectation the report can restate. The same reading applies to any two instructions that are individually fine: consistency BETWEEN them is unowned unless someone checks it. (confidence 0.8)

---

## 2026-10-07 — Tier-1 entry of 2026-09-02, moved whole: the PostToolUse hook runs `rustfmt` only

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-09-02: **The PostToolUse hook runs `rustfmt` ONLY — `cargo clippy --fix` must never run at write time, and a setup re-run must not silently restore it from the hooks-matrix default.** Four reasons, each a measured property of this project rather than a preference: clippy already gates TWICE (the chunk plan's Test Commands + wrap's light gate), so a write-time invocation adds no coverage; `--fix` is workspace-wide and can rewrite files OUTSIDE the chunk's declared modify-set mid-implement; a hook that rewrites a file after the tool call invalidates the harness's Read-before-Edit file-state tracking for anything read but not yet edited; and against this workspace's ~23 GB `target/` a cold `clippy --fix` exceeds the 30 s hook timeout and aborts under `|| true`, producing no signal AND no error — the worst of both. The generalization is the part to carry: a formatter is idempotent and file-local, so it suits a write-time hook; a LINTER with `--fix` is neither, so it belongs at a gate. (confidence 0.9)

**Extended 2026-09-06: "file-local" is the half that is not quite true — `rustfmt` on a CRATE ROOT recurses the whole module tree.** Editing `conductor-core/src/lib.rs` for a one-line re-export reformatted 16 `conductor-core/src` files, and `conductor-cli/src/main.rs` pulled in `paths.rs` — 15 files with no semantic change in a chunk whose real delta was 9. It is the formatter catching up, not corruption (each file's HEAD copy piped through `rustfmt --config-path rustfmt.toml` is byte-identical to the working copy, and 406 unformatted sites remain elsewhere), and it costs nothing but review noise — but it roughly tripled the chunk's diff and nothing predicted it. So when a change touches a crate root on a tree that is not already formatter-clean, expect the whole crate's `src` in the diff, and PARTITION the report's file list into semantic vs formatting-only with that byte-identity as the stated basis — a detector reading "coverage.rs changed" otherwise proposes on a phantom.

---

## 2026-10-07 — Tier-1 entry of 2026-08-21, moved whole: verify the artifact, not the exit code

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-08-21: **A tool reporting success is not the same as the write landing — verify the artifact, not the exit code.** Three separate silent corruptions in one session, none of which surfaced as an error: `printf '%s'` collapsed doubled backslashes so a JSON record carried invalid escapes and would not parse; the subagent transport HTML-escaped `<`/`>` in all seven returned extracts, mangling exactly the `<5s`/`<20s`/`<90s` tokens the work turned on; and a scripted `str.replace` pass NO-MATCHED twice while printing its success line, leaving edits unmade. Each was caught only by an INDEPENDENT check afterwards — re-parsing the file, re-reading the persisted text, running the compiler. So after any generated or scripted write, read back the thing you meant to change and assert on it; a substitution helper with no failure mode (`str.replace`, `sed` without `-E` verification) needs its own post-check, and a quoted heredoc is not a guarantee. Prefer the dedicated write tool over shell interpolation for structured content. (confidence 0.7)

**Extended 2026-08-21 (setup re-run): the sibling failure is a check that PASSES because it verifies the wrong property — presence instead of currency, readability instead of resolvability.** A vendored pipeline script sat two months behind the template it was copied from while the health check covering it asserted only that the file EXISTS; it existed, and it was broken. Its own on-disk data proved the drift: the cache it reads had moved to a per-plane layout the shipped script could neither produce nor address, so every query would have found no database, declared it stale, and silently rebuilt in the retired flat layout — a regression dressed as a refresh.

**Extended 2026-08-21 (delegated-timing budgets): the exit code you read may not be the one you think you read.** A standing supply-chain pin is satisfied only by a BYTE-IDENTICAL signature whose first term is the gate's true exit code; the probe was first run piped into `head`, so `$?` reported the PIPELINE's last stage (0) rather than the gate's (1) — a value that reads as a DEVIATION from the pinned signature and would have needlessly restored a full re-derivation, or mis-recorded the pin as broken. Capture the status into a variable BEFORE any pipe when the status is the evidence, and treat `cmd | head`/`| tail` as discarding it. Same family as the entry above: the tool did not lie, the shell answered a different question than the one asked. The pointer row naming that database read just as sensibly and was just as dead, and NO gate covers pointer rows at all (the import check guards `@`-imports only). So: presence is not currency — diff vendored tooling against its source template instead of trusting that it is there; and resolve a reference by looking for what it points AT, never by reading whether it sounds right. Both were settled exactly as the rule above prescribes — by an independent check that returned an artifact (the real per-plane databases answering a live query), never by an exit code.

**Extended 2026-08-22 (operator-pause-and-checklist-live-firing): the independent check has a SHAPE requirement — a count built by pattern-matching is bounded by the author's imagination of how a claim can be phrased, so it is not independent of the author at all.** One false premise (the self-obs sink) was stated in obs-plan in THREE wordings: `pretty-print`, `stdout`, and `stderr (dev only)`. A sweep on the first two produced a confident 17-site "coverage floor"; the semantic doc-detector found 21, because the third wording contains neither token. The floor was corrected twice before that — once by the operator (3→6, a pattern requiring the literal `pretty-print` missed "pretty IN dev"), once by widening the sweep (6→7, a hand-listed enumeration omitted the Decisions-Log site) — so the same claim under-counted THREE times by three different mechanisms. Two rules follow. **Every hit a pattern returns needs an explicit disposition, including "no change, and here is why"** — silent filtering is how two returned hits were dropped. And **when the artifact is prose, prefer a reader that understands the claim over a pattern that matches its wording**; where only a pattern is available, treat its count as a floor to be exceeded, never as the answer.

**Extended 2026-09-01 (webview-self-verify): the sharpest miss is a probe that searches for a token the project's own rules FORBID.** A grep for `#[tracing::instrument]` across the Tauri command handlers returned zero and was reported as an obs gap; the handlers all carry `tracing::info_span!("tauri.command.*").entered()`, which `observability.md` prescribes precisely BECAUSE the attribute does not stack with `#[tauri::command]` — so the token could only ever be absent, and its absence was evidence of compliance, not of a gap. The false finding reached a plan and a report before an auto-loaded rule file contradicted it. Three more false results landed the same session from the same shape (an anchor probe demanding a literal `per {plan} §` flagged 25 correctly-anchored bullets; an in-domain probe flagged a dependency-roster mention of `axe-core`; a `tr -d '-'` extraction reported six existing paths MISSING by eating their hyphens). So: **before grepping for a token as a proxy for a practice, confirm the project prescribes THAT token** — and read the hits, since every one of these four was resolved by looking at what the probe actually returned. (confidence 0.8)

**Extended 2026-09-02 (cross-surface-envelope-parity): the sharpest under-count is a site that states the retired claim in NONE of the retired tokens — a stale MECHANISM outlives the stale NAME.** Retiring rmcp wording from `test-plan.md`, the sweep was keyed on `rmcp` (24 lines, re-derived by `grep -c` rather than trusted from an enumeration — the originating CARRY had named ONE site, a distiller claimed 26). It still missed `:278`, which asserts the retired "negotiates DOWN to `2024-11-05`" mechanism and contains no `rmcp` at all; a doc-agent's per-occurrence grep found it, and a follow-up SEMANTIC sweep for the mechanism wording then found two more sites in the derived tier. So when retiring a claim, sweep for **what the claim SAYS, not what it is NAMED after** — the token sweep bounds the rename, never the belief. Corollary measured the same pass: three enumerations of one site-set (a CARRY, a distiller, my own report) were each incomplete in a different way, so **an enumeration is a starting point and the COUNT is the basis** — cite the derivation, never the list.

**Extended 2026-09-05 (audit-corrective): when the claim IS a token, sweep THAT token — not the neighbour it usually travels with.** Retiring the `--deny warnings` supply-chain form, the sweep was keyed on `cargo audit` and reported FIVE sites; the true count is SIX, because test-plan §12 writes the command hyphenated (`cargo-audit 0.22.2 \`--deny warnings\``) and carries no `cargo audit` token at all. `grep -- '--deny warnings'` returns 6 directly. The command name is a PROXY for the flag and an imperfect one; the doc-agent's own sweep found the site my report had missed, which is the second time in four days that a proxy token under-counted a claim it merely accompanies.

**Extended 2026-09-06: the same rule has a FALSE-POSITIVE face, and it fires far more often than the under-count.** Three times in one session a grep returned hits that were not the thing sought: a gate asserting an emitted log line matched `"message"`, a field EVERY self-obs line carries, so it could never fail; a probe checking that tautology had been removed matched the COMMENT explaining why not to use it, reporting the fix absent when it had landed; and a search for a test run's spec list matched the axe-core library source the harness INJECTS into the page, returning 1.3 MB of library internals. None was resolved by refining the pattern — each was resolved by READING the hits and asking what else could satisfy them. So before trusting a non-empty result, ask what OTHER content the pattern admits (a field every record carries, prose about the token, vendored or injected third-party source); and prefer a positive probe keyed on a stable low-cardinality string the target alone can produce over one keyed on a token the haystack also contains.

**Extended 2026-09-07 (a11y-ci-gate): when the SUBJECT is a tool's OUTPUT, grep what the tool PRINTS — not the term the tool uses internally.** A run's skip summary was searched for `pending`, mocha's own name for the state; the count was a true 0 and the conclusion drawn from it ("there is no summary line at all") was false, because the reporter prints `2 skipped`. The assertion built on it happened to be right for another reason, so nothing failed — the record was simply wrong until the operator re-measured. The same probe pass mis-counted `it()` blocks as 13 by matching the enclosing `describe(` line, and enumerated 2 of 4 `this.skip()` sites by keying on the ROLE tokens the guards happen to use rather than on `this.skip()` itself. All three are one shape: the probe asked about a proxy for the thing, and the thing was directly greppable.

**Extended 2026-09-07 (sr-findings-fixed), twice more in ONE session, and the second is the expensive kind. (a) SEVERITY is a proxy too: the decisive `session not created: DevToolsActivePort file doesn't exist` sat in two CI logs and was missed twice because the sweep keyed on `ERROR|Failed` while the driver logs its RETRIED attempts at `WARN` — the operator supplied the line. (b) A claim's own WORDING is the key, not a word describing it: a wrap report asserted no master baked a moved count, on `'10 spec\|ten spec\|12 spec'` returning 0 across the seven — `test-plan.md:307` spells it `10 passing`, and a detector found it by searching for what the doc SAYS. That false negative was one grep from reaching a committed amendment record. Ask what OTHER wording, severity or spelling could carry the claim before trusting a 0.**

**Extended 2026-09-07, the same family one level up: an INSTRUMENT reporting a cause is not the cause — validate a diagnostic form where the real path PASSES before building on what it says when it fails.** A CI gate failed at session creation; an isolation probe was written to bypass the intermediary and reproduce it, it failed identically, and a causal diagnosis was built on ITS failure mode and written into a committed matrix note. The probe had omitted one capability (`browserName: "webview2"`) that selects the driver's host-app launch mode — so the same POST fails on the DEV HOST too, against the very binary that passes 12/12 through the real path. The probe was measuring itself; the corrected form returns 200 locally and still fails in CI, which is the fact that was actually wanted. Two rules: a probe that fails where the real path passes is evidence about the probe, so run it against a KNOWN-GOOD path first; and a bypass probe is evidence only alongside its in-path twin — the twin failing identically is what makes the bypass informative at all. The retraction cost a wrap-time correction across a matrix note, a report and a friction record.

**Extended 2026-09-14 (0-pending route adaptation): a pointer can RESOLVE and still not carry the fact — a citation into a TOP-N or summary view is bounded by a rank cut the citing prose never mentions.** The 2026-08-21 clause above catches a DEAD pointer ("presence is not currency — resolve a reference by looking for what it points AT"); this is the live one, where the file exists, the key exists, and the claimed row was silently excluded. Three instances in one dictated relay's freight, each caught by reading the cited artifact before writing it into a route entry, and each wrong in a different way. Two were rank cuts: a clone pair cited to a duplication twin whose top-ten list has a fourteen-line floor sits at eleven lines and appears NOWHERE in that file; and an "under top" citation held for one of three subjects, the other two living only in the full-candidate list beside it. The third was a summary layer contradicting its own data layer INSIDE one document — the prose stated eleven survivors in a function its own table listed fourteen rows for, and only fourteen closes the arithmetic the same sentence states. So the check is never "the file and the key exist", it is whether the CLAIMED ROW is in the view you cite; a top-N, a prose roll-up or a head-limited render is evidence about what it INCLUDED and never about what it dropped. Cite the layer that carries the row, and when a past run's prose is the thing that is wrong, correct it through that artifact's own correction channel rather than editing the run's record after the fact.

---

## 2026-10-07 — Tier-1 entry of 2026-08-15, moved whole: dissolving every named blocker does not establish that none remains

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-08-15: Dissolving every NAMED blocker does not establish that none remains — a "last blocker" claim rests on the completeness of the enumeration, which is the one part research never verifies. The canary chunk's research correctly disproved all three causes a pooled capability's decline note named (one of them MISATTRIBUTED rather than outgrown: the cited gate was real but belonged to a different cue family), concluded the remaining constant was the last blocker, and the capability was claimed on that basis. The live leg then measured the fix land perfectly — the raised count reached the SUT intact — and still fail, on a fourth obstacle no document had ever named. Claim a capability on what a leg MEASURED, never on an argument that the known obstacles are gone; and when a chunk's own proof disproves its premise, the honest move is un-claiming with the measurement recorded, which costs one wrap and leaves the ledger true. (confidence 0.6)

---

## 2026-10-07 — Tier-1 entry of 2026-08-10, moved whole: `intent.md` and `requirements.md` are immutable; a disproved premise goes to the ledger

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-08-10: A version's `intent.md` and `requirements.md` are IMMUTABLE — no skill ever writes them (intent is the human record of what was ASKED; requirements' ids are contractual). Wrap's amendment flow owns the seven `.andromeda/` masters ONLY, so a detector or an `Expected amendments` entry aimed at either has no sanctioned apply path and must never be raised. When research falsifies a premise those files state, the correction goes to the LEDGER instead: a PREMISE-CORRECTION narrative in `verification-matrix.json`'s `notes` on the affected capability, plus the chunk's `scope.md`. The intent records the ask; the ledger records what turned out true. (confidence 0.9)

**Extended 2026-08-11: this holds INSIDE the matrix too — a capability's `verification.acceptance` text is not the place to absorb a disproved premise.** The only sanctioned acceptance-refinement class today is affordance honesty (`verification-matrix-contract.md` §Affordance honesty); "premise disproved" is a PROPOSED second class awaiting a founder ruling, so until that ruling every such case takes ONE channel — set `chunk`, write the PREMISE-CORRECTION into `notes` citing the evidence, leave the acceptance prose standing as the record of what was asked, and let the coverage flip stand on the `ref` proving the achievable shape. Two entries now share it (`v2-18`, `v2-31`); a third resolved differently would make the ledger inconsistent. (confidence 0.9)

**Extended 2026-08-19 (pii-scrub-live-proof): the refine-with-evidence shape now has a PHASE-TIME instance — when research proves a claimed outcome unattainable PRE-claim (not by any chunk, until the SUT changes), the operator's P4 fork + the P5 concretization preview is the ratification surface: v2-14 was claimed with its acceptance re-worded to the achievable proof and the PREMISE-CORRECTION written into `notes` at claim time — the third ratified refinement after v2-12/v2-13, and the first resolved before implement rather than at wrap. The class ruling stays the founder's; each instance remains individually operator-ratified.**

**Extended 2026-08-19 (connection-lifecycle-live-proof): the grain can be a SUB-CLAUSE — an acceptance whose outcome mechanism measured intact carried one descriptor ("with a recovery transition") that is structurally unsatisfiable, because bind status is per-process and recovery is a process replacement (no FSM transition out of ReceiverFailed exists to log). Same channel at sub-clause grain: reword the descriptor to the measured mechanism, keep the outcome (the SUT's own rebind, pinned), PREMISE-CORRECTION in `notes` — operator-pre-ratified in the wrap directives and resolved at the coverage gate.** (confidence 0.7)

---

## 2026-10-07 — Tier-1 entry of 2026-08-09, moved whole: before asserting that document A says X, grep A

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-08-09: Before asserting that document A says X, grep A. A claim about a document outside the asserting agent's remit is unverified by construction — each /andromeda-phase distiller reads only its OWN source, so its statements about another plan are speculation to check, not findings to propagate. And point every citation at the artifact that actually holds the fact: a COUNT cites the file it was counted from, a RULE cites the spec that states it. Under drift=0 a mis-aimed citation costs a real escalation — it either targets a document that does not carry the fact, or invites authoring content into a spec that was already correct. (confidence 0.6)

**Extended 2026-09-07 (dependency-polish): the sharper case is an agent asserting a false claim about its OWN source — the remit test does not catch it.** The original clause bounds the risk to "a document outside the asserting agent's remit"; here the security distiller, reading security-plan and nothing else, returned a Constraints bullet stating that plan FORBIDS leaving an advisory ignore whose subject a bump removed, anchored `(per security-plan §Dependency Security; §Security Anti-Patterns → Universal)`. The master states no such rule — its only four `ignore` lines are a screen-reader log, an advisory-DB fault remedy, the Accepted-exceptions record and a secret-scanning gate — and the sentence exists nowhere but that run's extract. It reached a plan step, a P4 decisive-lean note, an Expected amendment and a friction record before the operator caught it. So an ANCHOR is not evidence, whoever supplied it and however well-scoped they were: an extract is an inference over its source, and a plausible §-citation is exactly what stops you grepping. Grep the master for the RULE before building on it — and when the check dissolves the claim, re-base the ACTION on what does exist (here `deny.toml`'s own header requiring every accepted advisory to carry a justifying comment) rather than dropping the work.

**Extended 2026-08-11: the sibling check is that a spec describes TARGET state, so a doc-derived process cannot tell you whether the code already does it — before planning on a spec-asserted capability, verify it against the real artifact.** All seven distillers cited the shipped `timeline.execute`/`emit.batch`/`verify.readback*` spans as the in-tree exemplar to follow; none could see that `JsonObsLayer` implemented only `on_event`, so no span emitted anything and the committed `agent-latest.jsonl` held zero span records. The gap was invisible to fan-out by construction and roughly doubled the chunk's true surface. Five artifacts in one session asserted a reality that did not hold (extracts, `scope.md`, a lockfile prediction, a matrix acceptance, an obs-plan cleanup clause) — a written expectation is a hypothesis until an artifact confirms it. (confidence 0.8)

**Extended 2026-08-13: the sharpest case is a doc predicting its OWN retire-point by a NAMED MECHANISM — measure the mechanism, not just the goal.** `contracts/pulse-load-envelope.toml` named the chunk that would retire its exemptions and the term that would do it (summed emitting-phase duration); computed over all 35 scenarios that term retires neither exemption and changes no gate verdict, because summing bursts separated by quiet is not *sustained*. When the measurement falsifies the mechanism but not the goal, the resolution is to keep the goal and change the mechanism (here: bound the longest single emitting window, under which the ledger genuinely empties) — never to implement the letter into a red gate, and never to drop the work because its stated method failed. Verify a predicted end-state the same way as an asserted current one: against the artifact, before planning on it. (confidence 0.9)

**Extended 2026-08-13 (per-check read-back extraction): a spec can name every participant correctly and still invert the DIRECTION of the data — the cheapest wrong answer is the one that passes an existence check.** Both `architecture.md` §Occupied Resources and `scenarios/degraded-mode-report.toml` wrote `retrieve_report` "(with `degraded_mode`)" / "`retrieve_report(degraded_mode=true)`", reading as an argument Conductor passes; the SUT's own dispatch computes `degraded_mode = parsed_l4.is_none()` and RETURNS it, taking `{incident_id}` only. "Does this capability exist?" answers yes for both readings, so only reading the SUT's source separates them — and a design trusting the spec would have built request plumbing the SUT does not accept. When a spec describes an interaction with an external system, verify the DIRECTION (who supplies the value) and the ARITY, not just that the named thing exists. The corollary landed the same session: two record shapes in one document can be governed by different mechanisms (a span attribute is dropped unless it is in the tracing field allowlist; an envelope field is written by the report seam and is not), so a constraint proven for one is not evidence about the other. (confidence 0.9)

**Extended 2026-08-15 (Pulse consolidated visit): the claim source need not be a document — an operator's mid-session directive carries citations too, and dictation is exactly where they drift.** A visit brief naming an external system's constants cited two module paths that did not hold and stated its threshold relation loosely; one verification pass against the SUT's own source corrected both paths, sharpened the comparison to `>=` (so the named threshold value itself suffices rather than one above it — the margin floor moved by one), and surfaced a third bounding constraint the brief never mentioned. All three would otherwise have frozen into a route annotation under drift=0. When the SUT's repo is on disk, verifying a dictated cross-project citation costs one pass and is never optional. (confidence 0.6)

**Extended 2026-08-16 (canary-spans-pulse-fingerprints): the third axis is REPRESENTATION, and it is the one an existence check and a direction check both survive.** A spec claimed Conductor's exception fingerprint was "computed to match Pulse's derivation"; both sides genuinely computed a fingerprint over the same conceptual content, and the direction was right — yet they could never be equal, differing in ALGORITHM (FNV-1a vs blake3), INPUT (frame functions vs a normalized stacktrace) and WIDTH (16 hex chars vs an 8-char prefix of a 16-byte hash). Width alone settles it. When a spec asserts your value MATCHES an external system's, the claim is only verified by reading both derivations and comparing algorithm, input and encoding — never by confirming both produce "a fingerprint". The corollary is about reachability: this claim survived unfalsified for months because no span ever got far enough to be compared, so **a claim guarded by an unreached code path accumulates no evidence of being wrong** — when a fix finally opens the path, re-verify every downstream claim it newly exposes rather than assuming the ones that never fired were sound. (confidence 0.8)

**Extended 2026-08-16 (canary-fingerprint-derivation-aligned): the fourth axis is PROVENANCE — WHO PRODUCES the value — and it survives all three checks above.** Aligning the two derivations was the right fix and it landed; it still could not open the gate, because the field the assertion read is not produced by the component that computes the fingerprint. The named participant existed, the direction was right, and the representation was finally identical — yet the value is written by a different producer entirely (a model's free-text output, pinned empty in the deterministic mode every verifiable run uses), while the real computed value reaches no read-back surface at all **[corrected 2026-09-23: the field carried a constant `det-*` triple from SUT HEAD `efabe8e`, and at `83d4060` Pulse's grounded union adds the triggering cue's computed fingerprint to it — measured in the 2026-09-10 envelopes — so it now has a second, Pulse-computed writer]**. So a comparison premise needs one more question than "do both sides compute this correctly": **trace the field you assert on back to its writer, in the SUT's source, before building on it** — an existence check, a direction check and a representation check all pass on a field nobody upstream ever populates. The corollary for effort: aligning a value can be worth doing for a reason OTHER than the assertion that motivated it (here, so Conductor's expectation predicts how the SUT actually GROUPS exceptions) — separate "is this fix correct" from "does this fix unblock the thing I wanted", and state which one you are claiming. (confidence 0.6)

**Extended 2026-08-17 (Pulse token-leading semantics change): the fifth axis is TIME — all four checks above answer "is this true NOW", and not one of them expires.** An alignment verified byte-exact against an external system's derivation is a point-in-time measurement, never a durable property: the SUT can change the derivation, and one visit later Conductor's aligned surfaces — a scenario's prose, the variant unit tests, an amended capability clause, a harvest test's single-identity grading premise — were all still internally consistent with each other and all wrong against the new SUT. Three consequences worth keeping separate. (1) Those stale surfaces are owned DEBT, not a regression: nothing in this repo broke, so the response is a route entry that repays them — never a hot-fix, never a re-opened chunk. (2) A capability VERIFIED against the then-current SUT STAYS verified; it measured a state that really held, and the SUT's change invalidates only FUTURE legs, so never un-verify a ledger entry because the SUT moved. (3) The expiry is invisible from inside, which is why it needs owning: every gate here stays green over a superseded premise, because they assert Conductor against Conductor. So when you align to an external system, put the re-verification obligation somewhere that outlives the chunk, and re-check the SUT's HEAD before trusting a prior live green. (confidence 0.8)

**Extended 2026-09-06 (operator-gated-live-suite, 0-pending adaptation): the TIME axis has a second face — the SUT's OWN CLOCK, running while your tree stands still.** The 2026-08-17 face is the SUT CHANGING; this one needs no change at all. One unchanged scenario graded two ways across three runs in a single day, because its precondition (an EMPTY read-back) rode Pulse's per-service one-hour bootstrap window: inside it the arm was reachable, past it the silence evaluator raised autonomous incidents for exactly the quiet the arm depended on, so a LONGER wait made it strictly worse. The first failure was diagnosed as a margin and the window widened — correct for that run (which ran inside the window and lost to a second preflight incident) and useless for the next. Two rules follow. **When a leg over an unchanged tree grades differently, do not reach for flakiness or for a wider timeout until you have asked whether the SUT has an internal clock the leg's precondition rides** — an uptime-bound precondition is not flaky, it is conditional, and the condition belongs beside the verdict. And **when two runs of one subject fail, separate them by timeline before assigning ONE cause**: here the causes were genuinely different, and a CARRY naming only the second would have sent the next attempt to re-tune the first. (confidence 0.8)

**Extended 2026-09-07: the SUT's clock can have a LEVER, so ask whether the condition is settable before treating it as fate.** The same one-hour window is a boot-time default, not a constant — an env handle resolved once at the SUT's boot overrides it, disarming the evaluator for a whole suite, and the arm that was unreachable the day before was reached under a stretched window. So the diagnosis "this precondition rides an internal clock" has a second question after it: is that clock's bound CONFIGURABLE at the SUT's launch? Where it is, the lever is a stated launch posture recorded beside the verdict (and confirmed from the SUT's own log), not a wider timeout and not a re-tuned scenario. **But a lever that removes ONE cause is not a fix — check whether the others survive it before calling the condition solved.** Measured the same day: with the window stretched and the evaluator provably disarmed, two runs of one tree still graded differently on the SECOND cause, which the posture never touched. Setting a lever and re-running once looks exactly like a fix; only re-running under the lever, and reading WHY each run landed where it did, tells them apart.

**Extended 2026-08-21 (setup re-run): the sixth axis is OWNERSHIP — when a distillation and its master disagree, ATTRIBUTE before you fix, because the two stale kinds need OPPOSITE treatment.** A sweep of the derived tier surfaced ~15 stale claims, and the discipline that resolved every one was asking which master OWNS that fact. Where the owner is correct and the leaf diverged, the leaf is a cascade gap — fix the leaf (a crate count contradicting the architecture's own member list; a storage clause the leaf stated INVERTED from its master; a removed dependency still named in three auto-loading rule files and an agent definition though it is absent from the lockfile). Where the MASTER is the stale one, the leaf is faithfully inheriting and "fixing" it would MINT a fresh divergence — leave it standing and surface the master to whoever owns it. Two corollaries. A fact can be stale in several masters at once, so a CARRY that enumerates its sites can itself be incomplete — this sweep found a third site the ledger never named. And the SAME retired name can be right in one leaf and wrong in another depending on which master owns that sentence: a mocking-discipline mention belongs to the test plan, while the client mechanism belongs to the architecture — so attribute per SENTENCE, not per token. (confidence 0.8)

**Extended 2026-08-21 (delegated-timing budgets): the seventh axis is QUANTITY — an observable can exist, fire, and carry exactly the field you expect, and still measure something OTHER than the budget assumes.** A delegated ≤2s bound measured ~36s on two independent legs, and neither cause was slowness: the fire site computes `now − last_seen`, i.e. how STALE the subject's telemetry was when the state changed, not how long the change took — so for a subject that goes quiet first, the number reports the PRODUCER's upstream pipeline latency instead. The identical formula passed comfortably on a sibling capability whose event fires while the subject is still active, which is what proves the metric is fine and the PAIRING was wrong. Existence, direction, representation and provenance checks all pass on such a value; only reading the fire site's arithmetic separates them. Two corollaries. Attribution comes first: the samples belonged to a warm-up canary, not the scenario, because the scenario declared no emission of its own — check WHOSE event you measured before concluding anything about the budget. And when a budget proves unattainable as driven, that is a premise disproof to record with its mechanism and route forward, never a failure to hide or a threshold to quietly relax. (confidence 0.8)

**Extended 2026-08-31 (P-075 assert round, minted): the eighth axis is COMPLETION — before planning work a directive asks for, check your OWN ledger for whether it is already done, because an external ledger is not authoritative about this repo's state.** An incoming directive named a round to assert three delegated budgets; the local matrix recorded that exact scope `verified` a week earlier, ref naming three shipped tests, with the fourth clause already excluded by the same ruling the directive cited — so the requested work was evidence to QUOTE, not work to build. The same axis fired twice more in one visit on a smaller grain: two audit-ledger follow-ups presented as ownerless dissolved on re-reading their origin artifacts (one already ratified accepted-deliberate by the chunk that measured it, one already discharged by the very record carrying the complaint), which makes the sibling rule explicit — **a null `owner` field is a record gap until you have checked the origin, never proof that work is unowned.** Three instances, one shape: the external framing was authoritative about its own repo and silent about ours. Two consequences. Re-derive scope BEFORE accepting a directive's premise, and say plainly when the answer is "already done" — a round rebuilt on top of verified work is worse than no round, because it re-certifies from a weaker basis and hides the earlier proof. And when the check dissolves the ask, the honest deliverable is the RECORD (quote the existing evidence, own the disproved premise), not a manufactured substitute that keeps the original shape. (confidence 0.7)

**Extended 2026-09-06 (operator-gated-live-suite): a UNIVERSAL QUANTIFIER is falsified by one member — enumerate the family before writing "every" or "no".** Two instances in one chunk, from opposite directions, each costing a re-synthesis round. (1) A review finding stated "every committed declare-only scenario reads back 2-75s after its canary"; it propagated into a plan step, a rejected-approach line and a scope amendment before `ack-cooldown` (370s of silent phases, no emission tables) falsified it — one `grep -oE '^gap_ms' scenarios/*.toml` settles it, and the disproving evidence was already committed in a file the chunk had ALREADY READ for another purpose (`severity_harvest.rs:505-513`). (2) The reverse direction: a directive asserted "no spec master states an always-degraded claim (0 hits over the seven)"; re-derivation found 1 hit, because the claim sat mid-clause where a standalone-sentence pattern misses it. So an "every"/"no" claim about a FAMILY names the enumeration that establishes it, and one about a DOCUMENT SET uses a pattern WIDER than the one that reads naturally — a claim embedded in a longer sentence is exactly what a tight pattern drops. The corollary is the expensive half: **a correct neighbour is not evidence.** The false universal arrived inside a review whose other three findings all verified at HEAD, and that is precisely why it was accepted unchecked — verify each finding on its own basis, however good the company it keeps.

---

## 2026-10-07 — Tier-1 entry of 2026-08-08, moved whole: the code-graph is authoritative on call sites

_Moved whole from `CLAUDE.md` `USER:session-learnings` on 2026-10-07 (operator-requested curation pass). The text below is the entry as it stood, split into paragraphs at its dated extensions and otherwise unchanged; its one-sentence lead stays in Tier 1._

2026-08-08: The code-graph is authoritative on call sites — read it correctly rather than distrusting it. Query by `callee_name` and disambiguate by `callee_kind` / `callee_file`, per `scripts/code-graph-cookbook.md` **[corrected 2026-09-02: the DESCRIPTOR idiom (`%from_toml_str%`-style `LIKE` anchors) is RETIRED by the schema change — it was TS-shaped and silently missed every rust name preceded by `]`, `#` or nothing at all]**; a row count belongs to the query that produced it; and a `LIMIT`-ed or `head`-ed view is never the result — the run-dir trace's `rows` field is. Grep OVER-counts call sites (it also matches the definition, test-fn names and doc comments); graph lines are 0-indexed, grep's are 1-indexed. (confidence 0.9)

**Extended 2026-08-21: a descriptor can also be too LOOSE, and the graph can be silently INCOMPLETE — so neither a non-empty result nor an empty one is self-validating.** A `callee LIKE '%persist%'` impact query returned ~29 rows that were almost all `persistence_seconds`, an unrelated field in test binaries; the tightened `'%/persist().%'` returned 0 **[corrected 2026-09-02: that 0 was the ANCHOR, never an index gap — a crate-root rust symbol is `…conductor-run 0.1.0 persist().` with NOTHING before the name, so a `/`-anchored pattern can never match it. Both symbols were indexed all along: `persist` measures 10 call sites / 8 callers and `execute_scenario` 7 / 5, so "the DB holds no row at all" was false on both counts]**. So a predicate keying on a path grammar is the failure mode, not the loose substring alone; a 0-row result means *consulted-but-no-match* ONLY once you have confirmed the symbol IS indexed (a `symbol` probe by NAME) — otherwise it is a query-pattern miss wearing the same clothes as 'leaf, no callers'. Cross-check with grep before concluding either way, and say which basis the answer rests on. **Retroactive: every "0 callers" claim made before 2026-09-02 rests on the retired anchor — treat it as a query-pattern artifact and re-measure before reuse** (the shipped precedent re-measures 6 sites / 4 callers, though the decision it supported still held).

---

## 2026-10-04 — tokei's reading of a `tests/*.rs` file can be nearly all code, so a size forecast scaled from it overshoots

The audit's size line (`sizes.over_800`) is measured with tokei, and a split's forecast is naturally scaled from that
reading. Splitting `delegated_timing_harvest.rs` showed the base reading is not a line count of the file: tokei 14.0.0
read the base as 1130 code / 0 comments / 12 blanks over 1222 raw lines, while the five split files read 872 / 50 / 95.
The forecast root (≈490) came in at 310. Nothing was lost — a normalized line-multiset comparison of base against split
differed only by the module wiring, three `//!` lines, three `use super::*;` and one rustfmt re-wrap. Why tokei
mis-classifies the base is recorded, not established (the file holds long raw-string literals of captured JSON, a
plausible trigger).

So when a split is forecast or judged from tokei: re-read every resulting file with the same instrument, and compare
CONTENT by a line multiset before reading a code-line drop as lost content. Where an audit lists a file over the size
line, treat the figure as tokei's classification, not a line count — check its comments/blanks columns before trusting
the margin.

---

## 2026-09-30 — A sweep over the masters needs `-oiE … | wc -l`, and an escaped pipe under `-E` is a literal

Two grep mechanics returned confident wrong counts in one site sweep over the spec masters. First, under `grep -E` the sequence backslash-pipe is a LITERAL pipe character, not alternation, so a pattern written in BRE habit (`a\|b`) matched nothing and every master read 0 — a false absence. Second, `grep -c` counts LINES, and the masters carry multi-KB single lines holding several occurrences each, so a count read as sites under-counted (the Tier-1 2026-09-17 entry already names this, and it recurred). The form that answers the site question is `grep -oiE 'a|b' {file} | wc -l` for occurrences, plus `grep -noiE '.{0,90}(a|b).{0,60}'` to read each hit in context before dispositioning it.

---

## 2026-09-29 — A rule fixed before a drive is corrected after it by add-only dated brackets, never by rewording

A grading rule is committed before a live leg precisely so its grade cannot be fitted to the reading. Its
sha256 is recorded into the leg's evidence before the leg fires, and that recorded hash is the proof. When the
drive then falsifies a forecast the rule's prose carries, the correction goes BESIDE the stated text as a dated
`[corrected …]` bracket. The stated text stays untouched, so the recorded hash still describes "the rule as it
stood before the drive". Verify it both ways before the commit:
- `git diff --numstat {pre-correction commit} -- {contract}` must read `N 0` (added lines only);
- the committed pre-correction file must still hash to the recorded value.

Rewording the rule in place would silently orphan the pre-leg hash, and with it the only evidence that the rule
predated the reading. The same stance holds for any `contracts/` document with a pre-stated rule (the posture
contract, the P-025 measurement contract).

---

## 2026-09-29 — pulse-app raises its own compact widget at boot: verify it, don't ask for it

A hue-shift leg needs Pulse's COMPACT-WIDGET window mounted before the tier flips (a change older than the
canvas mount is never sampled) and visible (a minimized, throttled 1 s poll inflates `duration_ms`). This is not
an operator step: `pulse-app` calls `window::show_compact_widget` in its Tauri `setup`, so on a fresh data dir
the widget is up before the OTLP receiver even binds. What the agent owes is VERIFICATION, from a PowerShell
script run by path:
- enumerate the launched PID's top-level windows (`EnumWindows` + `GetWindowThreadProcessId`);
- read `IsWindowVisible` / `IsIconic` and the window size.

The widget is the small visible `andromeda-pulse` window (about 496×279), while the 1296×809 dashboard stays
hidden. Record that reading in the leg's evidence beside the posture lines from Pulse's own log.

---

## 2026-09-24 — Pin an undocumented platform premise with a probe that re-measures it on every run

When a gate's rule rests on a platform semantic the vendor does not document, and the semantic can only be
observed on the hosted platform itself, do not settle it with a one-off measurement. Ship a small probe pair
beside the gate that asserts the premise on every run: one step produces the condition, the next asserts it
through the exact read form the gate admits, and it fails with an annotation naming the rule it would invalidate.

The first run of the chunk's push is then the measurement. The premise also cannot rot silently: a later platform
change turns the probe red instead of widening the gate unseen. This is the TIME axis of the verify-the-claim
family (a point-in-time check expires; a standing check does not). It applies wherever a static gate's admission
depends on CI-runner behaviour no local instrument can reproduce.

---

## 2026-09-24 — Copying a precedent artifact copies its actor: re-derive every attribution clause

Mirroring a prior chunk's commit message, evidence record or report is a sound way to keep form. But the
precedent's sentences about WHO acted describe that occasion, not this one. When an artifact is modelled on a
precedent, re-derive each attribution clause ("made by X on Y's instruction", "operator-ratified", "run by the
overseer") from what actually happened this time, before the artifact is written. A pushed commit carrying the
copied actor can only be corrected forward, in the next commit, without a force-push.

---
## 2026-09-24 — `gate.py delta --defer-check` voids on a BASENAME, and `--only` will not fire a deferred entry

The deferral check greps each uncommitted file's basename fixed-string over the language's source tree. An
untracked phase-run extract named `security.md` therefore matches any Rust doc comment citing
`.claude/rules/security.md`: a different file, never read, but a hit, so the deferral is voided. Read the hit
before believing it.

When a void does stand, `gate.py run --only {n}` still prints `not run — defer (key)` for a `defer`-keyed entry
and runs nothing. The route that exists is driving the entry's exact `run` string by hand, with the exit read
from the bare command and the result recorded in the chunk evidence.

---

## 2026-09-24 — A same-section anchor check cannot tell WHICH registration a rewrite restates

`scripts/arch-registry-check.py` arm (f) verifies a `rewritten` row by finding its anchor anywhere in the AFTER
section it came from. In a registry, sibling bullets often share wording: `CONDUCTOR_MSEDGEDRIVER` and
`CONDUCTOR_NVDA` both "SKIP at exit 0 with a host-path-free precondition plus a fetch recipe". So a row can pass
while anchored on the wrong registration.

The wrap's faithfulness review caught one such row among 151. Heuristic anchors also latch onto shared paths and
chunk names rather than the restatement. When compacting a registry (the `D-arch-registry-size` remedy), choose
each anchor inside the row's OWN bullet, and read every judgment row against that bullet, not the section.

---

## 2026-09-23 — The code graph does not index feature-gated test files

A test file behind `#![cfg(feature = "live-pulse")]` (`live_suite.rs`, `real_model_live.rs`) is compiled only under that feature, and the rust plane's index is built without it — so its functions are absent from the `symbol` view and a `calls` query on their names returns only NAME COLLISIONS from other crates (here `runs_dir` resolved to the Tauri command's and `capture` to `obs.rs` tests). A plausible non-empty result is therefore the trap, not an empty one: probe `symbol` by name first, and when the definition sits in a feature-gated file, settle its callers by grep over that file and say which basis the answer rests on (the CLAUDE.md code-graph entry's grep cross-check, applied to a class of file the index never sees).

---

## 2026-09-16 — The amendment cascade sweeps the spec tier, not the committed data

An amendment retires a claim in a master and the cascade re-derives the distillations — CLAUDE.md's
generated blocks, `.claude/rules/*`, `.claude/docs/*`. It does NOT walk `scenarios/`, `contracts/`, or any
other committed file the project treats as data. So a claim can be retired in the spec tier and go on being
asserted, verbatim, by the corpus for as long as nobody happens to read it.

Measured this chunk: obs-plan retired the gloss of `read_back_observed_at − journal_emitted_at` as
"Conductor's own MCP round-trip" on 2026-09-10. Six days later four scenario TOMLs still stated it and three
more carried its sibling wording ("…is phase timing, not the SLO budget"). All seven masters swept clean the
whole time — 0 hits in each — which is exactly why nothing surfaced it: every gate that looks was looking at
the tier where the fix had already landed.

The consequence for planning: when a chunk amends a claim that committed data also states, the data sweep is
its own step, and no cascade or drift detector will raise it. The consequence for reading: a master being
current is evidence about the master, not about the corpus that quotes it.

---

## 2026-09-16 — A review edit re-opens the checks that READ the edited artifact

At a phase P5 review the operator asked for one polish: pin a required sweep FORM into a matrix acceptance
criterion. Applying it converted that criterion from a method-less absence claim into one naming a mechanical
method — and no `[[gate]]` entry performed that method. Check 4 (6) ("every gate or probe an acceptance
criterion NAMES is an entry") had passed correctly before the edit and was false after it.

What went wrong was the re-validation, not the edit: after applying, the checks that read the GATE FENCE were
re-run (the parse, the size, the new/baseline pairing) and the check that reads ACCEPTANCE CRITERIA was not —
which is precisely the one the edit had invalidated. The operator caught it in the next round and supplied
the missing entry.

The rule: after applying any review edit, re-run the checks whose SUBJECT the edit changed, not the checks
whose ARTIFACT you happened to re-run. The two sets overlap often enough to feel identical and are not.

---

## 2026-09-08 — Read the verification matrix through its tool, not through the JSON

The verification matrix (`{project}-{version}/verification-matrix.json`) has a tool read surface —
`python -X utf8 {tools_dir}/matrix.py show --dir {version_dir} --id {id}` — and reading the file directly
with a JSON load is a shortcut that costs correctness silently. Operator correction this session, after two
direct reads (phase P1 and P3).

The direct read returned every field the decision had rested on — `status`, `chunk`, `ref`, `method` and the
`notes` narratives — and looked complete. The tool additionally prints `title`, `observed_gap` and
`requirement`, and `requirement` proved STRICTER than the `acceptance` being reasoned from ("…and gated in
CI"): it is the field that settles whether a locally-green test can ever satisfy the capability. The
substance survived unchanged, which is the point worth keeping — the shortcut did not produce a wrong
answer, it produced a right answer resting on an incomplete view, one field away from a wrong one, on the
single claim/decline decision the whole chunk turned on.

Applies whenever a matrix entry's fields inform a claim, decline or refine decision: use the tool's own read
surface. More generally, a hand-rolled read of a pipeline artifact is evidence about the reader, not about
the artifact — the tool exists because it knows which fields matter.

---

## 2026-09-08 — The `.claude/` leaves are project-tailored renders, not template copies

`/andromeda-setup-project`'s `references/` templates are generic scaffolds: they carry `{Extract from …}`
placeholders and stack-agnostic web/API content. Conductor's materialized `.claude/rules/`, `.claude/docs/`
and `.claude/agents/code-reviewer.md` are project-specific renders this pipeline produced from the six
plans, and wrap's cascade has kept them current since. On a mature project a verbatim template re-render is
therefore a REGRESSION, not a refresh.

Measured at the 2026-09-08 re-run. The `verification-harness` template prescribes a daemon `boot`, a PID
file, a heartbeat and a `TIDELINE_DATA_DIR` handle — Conductor has none of those (the test plan records the
PID file as N/A), and the shipped rule carries the explicit counter-clause "Conductor has NO daemon and NO
inbound listener … Do not reintroduce daemon/PID/endpoint machinery", which a verbatim re-render would have
deleted. The `security` template is generic (constant-time compare, rate limiting, XSS, `npm audit`) where
the shipped rule is Conductor's real boundary set; the `stack` doc template is a bare placeholder skeleton;
the `code-reviewer` template lacks the `## Conductor-specific checks` section the shipped agent adds.

The hazard is that the WRITTEN preserve rules do not cover this class. `USER:*` sections, `## Session
Additions` and the agent-run scripts (only-if-missing) each have an explicit rule; the rules bodies, the
docs and the code-reviewer have none — so on every re-run the judgment has to be made deliberately rather
than inherited. When re-running setup to absorb a pipeline-template change, read each template for its
SHAPE, treat the substitution step as already applied, and change only what the trigger actually requires.

---
## 2026-08-20 — A standing PREREQ's ordinal counts probes, not the entry it rides

A gate deferral pinned as `PREREQ: re-check {gate} — Nth consecutive` numbers the FORTHCOMING probe, so
N is a position in the measurement chain, not a property of the route entry holding the pin. When a new
chunk is inserted ahead of the pin's current holder, the pin MOVES but its number does not advance — the
inserted chunk simply becomes the one that runs probe N. Advancing it would claim a probe that never ran
and leave a gap in the very chain the ratification rule reads for age.

The chain is recoverable from the artifacts rather than from the pin: each chunk's report records the
ordinal it discharged, so the next pin is always last-discharged + 1. Re-derive it there whenever the tail
is reshaped — an ordinal dictated in a route directive is a hypothesis like any other coordinate, and this
one arrived off by one against a chain the reports settled unambiguously.

Applies whenever route-resolve inserts, reorders, or re-owns a markerless entry that carries a standing
PREREQ — the pin's ORIGIN marker preserves its age, and its ordinal preserves its count; the two travel
independently.

**Extended 2026-08-22 (boundary adaptation): a THIRD thing travels with a moved pin — its SEPARATOR — and
the wrong one silently converts a live obligation into stale freight.** The migrating pin had been attached
to its previous holder by a SINGLE space. Flip-compaction strips an annotation only when its introducer sits
behind two-or-more whitespace or a space-middot-space, so at one space the pin is invisible to the strip: it
would have survived into the frozen historical record as exactly the stale-truth bait compaction exists to
remove, and no gate covers that. Re-attach a moved annotation with the conventional three-space separator
the CARRY pins on the same lines already use. The general form: when a tail reshape moves an annotation,
ORIGIN carries its age, ORDINAL carries its count, and SEPARATOR carries its eligibility to be cleaned up
later — check all three, because only the first two are visible in the text you are moving.

---

## 2026-08-19 — Wireframe sample rows bind by P-ID, not by their scenario-name column

The layout-templates idle-console sample rows pair P-ID labels with FICTIONAL scenario names (e.g. `P-003
fingerprint-identity`), so a derived-value reader keyed on scenario names concludes "other scenarios,
untouched" while the P-ID-labeled tier cell bakes a value a chunk just moved. Treat the P-ID as the binding
key for sample-staleness checks. The drift agent under-ran exactly this way this wrap; the
expected-amendments floor (validate check 5) caught it, and the rows are now de-literalized to `<slo_tier>`.

---


_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/andromeda-setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-08-18 — Host-shell probe discipline (three same-day bites)
Three Git-Bash-on-Windows traps each cost a redo cycle in one session. (1) A pipeline's `$?` is the
LAST command's — `cargo audit 2>&1 | tail` reports tail's 0; capture true exits standalone
(`cmd > file; RC=$?`). (2) `$TMPDIR` is unset in this shell — `> "$TMPDIR/x"` becomes `/x`
(permission denied); always use the session scratchpad path. (3) The persistent shell's cwd can
drift silently (one sweep round ran empty relative globs from inside `.andromeda/`); prefix
root-sensitive compound commands with an explicit `cd <repo-root> &&` or use absolute paths, and
treat a failing `cd` beside succeeding bare-name reads as the tell.

**Extended 2026-08-20 — the family's worst form is not a redo cycle but a FALSE GREEN, and trap (2)
recurred.** Two more bites, plus the same unset-variable redirect this entry already documents (it was
hit again anyway — the trap is cheap to re-hit, so prefer a literal scratchpad path over any variable).
(4) **A broken command substitution yields an EMPTY pattern, and an empty pattern matches
everything.** A verification loop built its needle with `$(… | sed …)`; the `sed` failed, the
substitution returned empty, and `grep -qF ""` matched every line — so the check printed a
confirmation for all five items while testing nothing. It was visible only because `sed` printed its
own error beside the green lines. Build verification needles as LITERALS, or assert the needle is
non-empty before using it; a probe whose failure mode is "passes vacuously" is worse than one that
errors. (5) An existence probe over several paths (`ls A B || echo missing`) exits non-zero when ANY
path is absent, so it reports the missing branch even when the path you cared about exists — check
one path per probe, or suffix `|| true` and read the output rather than the exit. The through-line
for all five: a shell default turns a partially-wrong command into a confidently-wrong ANSWER, so
the probe itself needs verifying whenever its result will be reported as evidence.

---

## 2026-08-15 — evolve-diagnose run dirs are audit-trail class at the wrap dirt-check

`/andromeda-wrap-session`'s Setup dirt-check names exactly three expected-transient bookkeeping members
(`session-handoff.md`, `state.yaml`, `.andromeda/friction-log.ndjson`). A founder-invoked
`/andromeda-evolve-diagnose` therefore leaves an untracked run dir under `.andromeda/runs/` that falls
outside the named set and reads as "anything else dirty → HALT + ask" at the next 0-pending wrap.

The operator's standing ruling: these are **audit-trail class** — absorb them into the wrap commit rather
than halting. The HALT exists to stop unattributable *work* landing without a marker; a diagnosis run dir
is telemetry output (explicitly not project state per the evolve system reference) living in the same
`.andromeda/runs/` tree that wrap's own run dir occupies and `git add -A` sweeps every wrap.

Applies to any 0-pending wrap following an evolve-diagnose invocation.

---

## 2026-08-13 — Report counts come from `git status`, not from a running tally

The operator's standing practice for the wrap report: derive the chunk's file counts from `git status` at
wrap time rather than from a tally accumulated across the session. A running tally drifts whenever a file is
touched twice, added then reverted, or created by a step that also creates bookkeeping — and that tally is
what the report's Changes bullet and the console summary both quote. `git status` is the one view that
cannot disagree with what is about to be committed.

Cross-check the derived number against the implement report's own table before quoting it (this chunk:
6 source/test files + the verification matrix + bookkeeping, consistent across both).

## 2026-08-11 — On Windows, an unstated text encoding silently corrupts what you read and crashes what you print

Python on this host defaults to the **cp1252** locale encoding for both file IO and stdout, and Andromeda's artifacts are em-dash- and arrow-dense (`—` in every epoch header and route line, `↓` as the working-route separator, `→` in every master record). Three distinct failures came from that in one session, each with a different signature:

- **Reading** `friction-log.ndjson` with a bare `open()` mis-decoded every record written with `ensure_ascii=False`, while records written with `ensure_ascii=True` decoded fine. A tally keyed on the epoch label therefore split one epoch into two phantom keys (28 + 31 for a single 59-record epoch) — the data was pristine; the reader invented the split. This nearly became a false "the telemetry is corrupted" finding, and the "fix" would have damaged a healthy file.
- **Printing** a working-route line raised `UnicodeEncodeError: 'charmap' codec can't encode character '↓'` and killed the command outright — the loud, harmless failure of the three.
- **Writing** through a `python - <<'PY'` heredoc mangles non-ASCII in transit, because the bytes cross the shell before the interpreter sees them.

The discipline: pass `encoding="utf-8"` explicitly on every `open()`, set `PYTHONIOENCODING=utf-8` when a script prints project text, and write non-ASCII as `\uXXXX` escapes in an ASCII-only source file run **by path** rather than piping a heredoc. When a result looks like data corruption, verify by **codepoint** (`ord(ch)`) before believing it — a lossy display and a lossy file are indistinguishable in a terminal, and only one of them is a real problem.

---

## 2026-08-09 — A baked count inside an illustrative sample is the same stale-derived-fact class as one in prose

When a chunk invalidates a derived fact (here: the coverage classification widening from 60 to 82 capabilities), the de-hardcoding sweep must reach **sample output and wireframe captions**, not just prose. Illustrative status is not an exemption — a reader takes a number from a sample exactly as readily as from a sentence, and leaving one behind recreates the very self-contradiction the sweep exists to remove. `layout-templates.md` had already been half-swept on 2026-08-08: its coverage header strip (`:37`) read "82 loaded (manifest set)" while the component prose (`:121`) still said "the full 60-row wall" — the document contradicted itself for a full version cycle because the earlier pass treated the two as different classes.

The sharper trap is **coupled facts inside one sample**. The suite-run caption baked two numbers that had to agree: a per-state tally (`55 Pass · 1 Calib · 1 Fail · 1 Manual · 1 Residual · 1 Blocked`) and a step denominator (`step 60/60`), the first summing to the second. Fixing only the denominator would have left the sample internally inconsistent — strictly worse than the stale total it replaced, because an inconsistent example teaches nothing and reads as a typo. Fix such a caption as a unit and verify the arithmetic afterward (`77+1+1+1+1+1 = 82 = denominator`), and prefer the form a sibling passage already established (`(manifest set) 82 loaded`) so one convention governs the file.

Practical sweep note: grep the *class*, not one phrasing — `60-P-ID`, `all 60`, `60-row`, `P-001\.\.P-060`, `60/60` each found different sites, and the eventual inventory (11 code sites + 6 spec sites) was roughly triple what the initial reading suggested. Guard the false positives explicitly: `Lamp::Blocked => 60` is an ANSI color code and `--fail-under-lines 60` is a coverage percentage, neither a capability count.

**Extended 2026-08-09 (in-lane-sut-scenarios) — the class reached recurrence #3 and is now machine-detected.** Three consecutive chunks staled a documented derived value with no drift-base invariant covering it (out-of-scope treatment ×2, then `UNBACKED_AUTO` 11 → 10 staling `layout-templates:178`'s caption *and* `test-plan:306`'s `P-001..P-060` selector range). Three things generalize from closing it:

**A detector needs a fact to bind to.** The class was invisible because the report had no bullet for it — the structural families (Files / Symbols / Crates / Dependencies / Schema) describe *what was added*, never *which documented derived value moved*. Adding a `Counts / qualifiers this chunk moved` bullet to the report is what let `D-layout-surface` fire; its proposal rationale cites that exact bullet. The report↔detector contract is load-bearing in both directions: extend the report first, then the detector has something to read. Two detectors now cover it (`D-layout-derived-count`, `D-tests-derived-count`) — drift-base scopes one detector to one doc, so a cross-doc invariant costs one entry per doc.

**The fix is to name the SET, never substitute a fresh literal.** `P-001..P-060` → "the manifest's accepted set", not → `P-001..P-082`. A new literal is the same bug with a later expiry date, and it re-stales on the next SUT release. This is why the two leaf distillations needed *no* edit this time: `design-summary.md` and `tests-summary.md` had already been written to name the mechanism (`(N unbacked)`, "the `UNBACKED_AUTO` pin") rather than the value, so re-derivation produced identical text. Prose naming the set is correct and must not be "fixed".

**A doc-agent can see a finding and decline to raise it.** The test-plan detector *found* the stale `P-001..P-060` range and wrote, in prose after its `proposals: []`, that it was "not what any of my three invariants guard". It was right — and the finding survived only because that trailing prose was read before the strip step discarded it. When a detector returns clean but explains itself, read the explanation: a scoped agent correctly refusing to exceed its remit is reporting a gap in the *detector set*, not an absence of drift.

---

## 2026-08-09 — Trace fidelity is not just the count: honoring `rows` while reconstructing composition from a truncated view

The Tier-1 code-graph rule already says a `head`-ed or `LIMIT`-ed view is never the result — the run-dir trace's `rows` field is. There is a subtler way to break it that satisfies the letter of that rule: take the **count** from the trace, then reconstruct the **composition** from the truncated console output you happened to see. The count is right, the claim built on it is wrong, and nothing about the output looks suspicious.

Concretely: a `crate_edges` query with the canonical bidirectional predicate (`WHERE to_crate = 'X' OR from_crate = 'X'`) returned 6 rows. The visible tail showed five inbound edges, so the sixth was inferred to be outbound — plausible, since the query asks for both directions. In fact all six were inbound and the sixth was simply above the window; the crate imports no workspace crate at all. **A bidirectional `OR` predicate does not imply both directions are populated.**

Read the trace's `result` array for composition, not only its `rows` scalar for arity — and when a claim rests on a direction, a subset, or a grouping, derive it from the result set, never from what scrolled past. The stakes are not cosmetic: a plan is `/andromeda-implement`'s input and the wrap report inherits whatever number stands in it, so a wrong split propagates into the permanent record. Corollary for writing it down: state the split explicitly ("6 rows, all inbound, zero outbound") rather than a bare count with an illustrative list, which invites the next reader to re-derive the same wrong inference.

---

## 2026-06-27 — Live-Pulse E2E reference: the Pulse MCP read-back surface + the run recipe

Verified live against the running Pulse this session (operator findings + direct probes):

- **Incident creation is LLM-in-the-loop + non-deterministic:** OTLP → L1 → L2 RetryStorm cue (deterministic, ≥5 same fingerprint/30s) → L3 digest (cadence 20-60s) → L4 llama.cpp Llama-3.2-3B inference (Dismiss/Severity) → incident [corrected 2026-10-06: Llama-3.2-3B then; since Pulse `5f77859` the shipped L4 model is `gemma-4-E4B-it-Q4_K_M`]. The verification-posture consequence (live incident-readback can't be deterministic; open decision) is in `verification-harness.md` Session Additions.
- **The MCP read-back surface is 8 tools, not 4:** 4 live-buffer (`query_traces`/`query_metrics`/`query_logs`/`generate_snapshot`) + the 4 persistent-corpus tools Conductor consumes (`query_incident_list`/`retrieve_report`/`retrieve_telemetry_slice`/`mark_incident_resolved`). **[corrected 2026-10-03: true at Pulse `83d4060` — at Pulse S2 `cdb6c1e` the surface is 9 tools, the fifth corpus tool `retrieve_incident_events` (a by-id read of an incident's lifecycle events), which Conductor now pins and consumes; read live in the P-075 re-round]** **Only the persistent-corpus tools work cross-process from a Conductor-spawned sidecar** — the in-memory-buffer tools return EMPTY (the spawned sidecar shares no memory with pulse-app, which owns the live OTLP buffer). Shapes: `query_incident_list` → `{items:[{id,status,severity,title,opened_at}],total,next_cursor}` (titles are SCRUBBED/generated, NOT a marker echo — **and fidelity does NOT ride the fingerprint either: corrected 2026-08-16**, `fingerprint_refs` carries the L4 model's `evidence_refs` — a constant `det-*` triple under deterministic L4 since SUT HEAD `efabe8e` (2026-08-18; payload-invariant, still not a fidelity carrier — the 2026-08-16 measurement saw `[]` at the earlier HEAD) — while Pulse's computed fingerprint reaches no read-back surface at all **[corrected 2026-09-23: true at `efabe8e` only — at `83d4060` an incident's `fingerprint_refs` also carries the triggering cue's full-hex fingerprint (the grounded union), measured in the 2026-09-10 envelopes, so fidelity DOES ride that one value]**; the canary's carrier is `opened_at_unix_nano` freshness, by choice); `retrieve_telemetry_slice(incident_id)` → `{incident_id,span_refs,fingerprint_refs,timestamps_unix_nano}`. Incidents are filtered by the `workspace` column = the sidecar's data-dir.
- **`corpus.db` is plaintext SQLite** (header "SQLite format 3") at `{data_dir}/corpus/corpus.db` — NOT encrypted-at-rest (the arch P-049 "encrypted / OS keychain" assumption was wrong). Tables: `incidents` (`id,workspace,status,created/updated/resolved/read_unix_nano,payload`), `service_registry`, `baseline_state`, `pipeline_metrics`, `digest_archive`. Conductor's production code reads the corpus via MCP read-back ONLY (the direct `sqlite3` reads this session were one-off operator-sanctioned diagnosis).
- **Pulse run recipe (future live pass):** launch pulse-app with `ANDROMEDA_PULSE_MODEL_PATH` + `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`; the build needs `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (a triage/build.rs bug truncates the 9 MB tokenizer → L4 breaks). Sidecar double-gate = `ANDROMEDA_PULSE_MCP_ENABLED=true` + the compiled feature flag. Conductor resolves the sidecar from PATH (`andromeda-pulse-mcp`); data-dir defaults to `%APPDATA%\andromeda-pulse`. RetryStorm thresholds ≥5/≥10 same fingerprint/30s; L4 ≈4 s/inference.

---

## 2026-06-27 — Obs-artifact CI conformance gate: the no-Pulse producer + grep stderr for panics

To assert Conductor's self-observation artifact in CI without a live Pulse (the obs-plan §9 obs CI gate), PRODUCE `logs/agent-latest.jsonl` with a hermetic Blocked agent-mode run: `cargo run -p conductor-cli --bin conductor -- run error-baseline-spike --seed 424242 --agent-mode` under `ANDROMEDA_PULSE_DATA_DIR=pulse;injection`. The injection metacharacter is rejected by `conductor-verify` BEFORE any sidecar spawn → a Blocked envelope (exit 0, not a hard Fail) — but `init_observability` still opens the file sink and writes every self-obs line (the `cli_smoke.rs` `agent_mode_routes_self_obs_to_the_log_file_not_stderr` precedent; reuses the 2026-06-23 verification-harness Blocked lever, now applied as a CI PRODUCER rather than a test). `cargo run -p conductor-cli` does NOT compile `conductor-tauri`, so no frontend bundle is needed. A `shell: bash` gate then asserts the §3 self-obs BASE schema per line (`jq`: `timestamp_ms`/`level`/`target`/`service.name`/`service.version`/`deployment.environment`/`run_id` — NOT the §6 run-report envelope, which lives in `runs/<run_id>.jsonl`), scans for leaked absolute host paths (anchors MIRRORING `conductor-core::redact::is_host_path_token` — drive-letter `[A-Za-z]:[\\/]`, `/home/`, `/Users/`, `%APPDATA%`, `.cargo/`, `.rustup/`… — so `::` module paths and repo-relative `crates/…` source paths are NOT flagged), and scans `^thread.*panicked`.

GOTCHA (caught by the real-smoke dogfood, not the typed tests): the zero-unlogged-panics scan MUST grep BOTH `agent-latest.jsonl` AND the captured producer stderr. A CAPTURED panic (`std::panic::set_hook`) is a structured `tracing::error!(panic=…)` JSON line in the FILE; but an UNSTRUCTURED panic (a bypassed hook — the exact regression the gate guards) is written by Rust's default handler to STDERR, never the agent-mode file sink — so a file-only grep would silently miss it. Capture it: make the producer step `shell: bash`, `mkdir -p logs` (the `2>` redirect target dir must exist first), `… --agent-mode 2> logs/producer-stderr.log`, then `grep -nE '^thread.*panicked' "$log" logs/producer-stderr.log`. obs-plan §9 already specified "greps agent-latest.jsonl + stderr" — the spec was right; the plan's softer "if captured" wording got concretized here. This was an in-scope `ci.yml` strengthening, not a spec change.

Verify locally before trusting CI: produce the artifact under a relative scratch `CONDUCTOR_RUNS_DIR` (e.g. `.obs-smoke/runs`; `resolve_under` requires a path under cwd, so an absolute temp dir is rejected — relative-under-repo, then move it OUT of the tree rather than `rm -rf`, which the sandbox blocks), run the gate logic against it (expect PASS), and against crafted negative fixtures with VALID JSON (a `/home/…` path for the leak check; a panic line on a stderr file for the panic check — a non-JSON line fails the per-line `jq` field check FIRST, so use valid JSON to exercise the leak/panic checks for the right reason). The Epoch-10 A11y CI gate + Obs gates reuse this producer + `shell: bash` gate-step + `if: always()` upload scaffold.

---

## 2026-06-27 — CI coverage-gate mechanics (cargo-llvm-cov + nextest in ci.yml): promote the pre-staged step, and the gotchas

The CI quality-gate chunk promoted `ci.yml`'s pre-staged `Coverage (measure only)` step (`--summary-only`, whose own comment flagged it Epoch-10) into a real gate: collect once with `cargo llvm-cov nextest --workspace --profile ci --no-report`, then `cargo llvm-cov report --lcov --output-path lcov.info` + `--cobertura coverage.xml`, then a final `cargo llvm-cov report --fail-under-lines 60` (the binding line floor, test-plan §10), then `actions/upload-artifact@v4` for the coverage reports + `target/nextest/ci/junit.xml` (both `if: always()` so a failing gate still publishes). `.config/nextest.toml` needed NO edit — `[profile.ci.junit]` + `retries = 0` (both profiles) were already there from the test-framework chunk; a separate `shell: bash` grep step asserts `retries = 0` at the CI level (the flakiness budget), catching `retries = N` and the `retries = {…}` backoff-map form.

cargo-llvm-cov behaviors worth knowing for the downstream Obs/A11y CI-gate chunks: (1) it reports `src/` only — so the test-plan §10 "exclude rstest/insta fixtures" `--ignore-filename-regex '[\\/]tests[\\/]'` is currently a NO-OP (integration `tests/` files aren't in the denominator; the measured 89.89% already reflects product code) — kept for intent + future-proofing, not because it changes the number. (2) LCOV (`lcov.info`) carries ABSOLUTE source paths by default (`SF:D:\dev\…`), while Cobertura keeps only the `<source>`-root absolute (per-file paths relative) and nextest JUnit is path-clean — so the only absolute-path surface is LCOV, and in CI those are ephemeral runner paths (see the security.md redaction-boundary rule — outside Conductor's redaction scope). (3) Emit multiple formats from ONE instrumented run via `--no-report` collect + per-format `cargo llvm-cov report` calls — don't re-run nextest per format.

Windows-runner shell gotcha (recurs on every multi-command CI gate step): a SINGLE-command `run:` step propagates its exit code (GitHub appends a `$LASTEXITCODE` check for the default pwsh shell), but a MULTI-command pwsh block does NOT fail-fast by default — a failing `cargo llvm-cov report --fail-under-lines` mid-block would pass silently. Use `shell: bash` (GitHub's bash runs `set -eo pipefail`) for multi-command report/gate steps, or keep the binding gate as its own single-command step (the shipped shape). Verified locally on Windows before gating (the critical safety check — the prior step only *measured*, so adding `--fail-under-lines` blind could have turned CI red): real coverage 89.89%, gate exit 0 at 60 / exit 1 at 95 (both directions), `cargo audit`+`cargo deny` green, `ci.yml` valid YAML, `agent-run status` smoke exit 0. The full `agent-run run` was not re-run — a ci.yml-only change has zero Rust delta and the nextest leg was already green via the coverage collect.

---

## 2026-06-27 — Tauri operator-pause resolver bridge: an async core seam ↔ webview across the background run thread

The GUI is the third `conductor_core::PauseResolver` shell (after the agent `HeadlessResolver` + the CLI `inquire` `PromptResolver` — the 2026-06-23 "third arm, not a parallel mechanism" entry anticipated this): `conductor_tauri::pause::TauriResolver`. The run drives on a background `std::thread` (a core-owned `current_thread` runtime), while the go/no-go answer comes from the webview — so the resolver bridges thread ↔ webview WITHOUT blocking Tauri's event loop. `resolve(hold)` `arm`s a shared managed `HoldGate` (`Arc<Mutex<Option<oneshot::Sender<Decision>>>>`) with a fresh `oneshot::Sender`, pushes a `HoldPrompt` projection (P-ID/step/prompt + `allow_no_go`) over a **2nd IPC `Channel<HoldPrompt>`** passed to `start_run` alongside the live-counter `Channel<RunEvent>` (a `Channel`, NOT a Tauri `emit`/`listen` event — that would need the `core:event:allow-listen` ACL grant; a Channel is a command ARG, ungated), then awaits the receiver. The `resolve_operator_hold` `#[tauri::command]` `deliver`s the operator's `Decision` by taking the sender from the gate and sending it. A dropped/unanswered sender ⇒ `Decision::NoGo` — the abort-safe default, mirroring the CLI cancel.

`conductor_run::drive_run` was generalized `<R: PauseResolver, E, A>` (was a hardcoded `HeadlessResolver::proceed()`) so the GUI injects the `TauriResolver`; its ONLY non-test caller is the Tauri run thread (the CLI calls `execute_scenario` directly, already generic), so the blast radius was the Tauri caller + 2 unit tests — the CLI release gate (`cli_smoke`) stayed green. The hold only fires on the live-Pulse path (empty-`expected` operator-checklist scenarios P-025/026/027/P-032; every scenario is `Blocked` BEFORE the hold without a live Pulse), so this chunk unit-tests the bridge core (the DI-free `HoldGate` methods) + gallery-demonstrates the dialog, with the live run firing in Epoch-10. `execute_scenario` currently records the decision + returns a `ManualCheck` record UNCONDITIONALLY — acting on a `NoGo` to halt the run is also Epoch-10 semantics, not this chunk. `capabilities/default.json` stayed unchanged (app command + Channel are not ACL-gated — the 2026-06-26 rule); `serde` was added to conductor-tauri's manifest (already a workspace dep — no new `Cargo.lock` package) for the `HoldPrompt` `Serialize`, and tokio's `sync` feature for `oneshot`.

---

## 2026-06-27 — Two RunRecord data-source paths: per-run = the JSONL journal (read in conductor-core); cross-run / per-P-ID = a runs.db query (Epoch-10)

The run-report view (ch7) sources a run's per-scenario `RunRecord`s by reading its `runs/<run_id>.jsonl` journal — **NOT** `runs.db`. The CLI `report` verb already did this (`latest_run_id` → `read_journal` → `serde_json::from_str::<RunRecord>` per line); this chunk **lifted those two fns to `conductor-core`** (new `run_journal` module — `latest_run_id` / `read_run_journal`, typed `CoreError::Config` on IO/parse per the verdict/error wall, the IO-in-core sibling of `scenario_files` / `list_scenarios`) so the CLI `report` verb and the Tauri `run_report` command share ONE reader (DRY). Consequence: the run-report view went **live now with zero new cross-crate edge** (`RunRecord ∈ conductor-core`, already a `conductor-tauri` dep — no `conductor-tauri → conductor-report`) and **zero new `RunsDb` query** — materially cheaper than ch6's deferred coverage join, which is why the /andromeda-phase P4 AskUserQuestion chose "live journal read now" over the ch6 "defer live join" precedent.

The distinction worth remembering for Epoch 10: a query for **all records of ONE run** is a journal read (cheap, no DB, no aggregation); a query for the **latest record per P-ID ACROSS runs** (the coverage matrix's per-P-ID lamp join) is a cross-run aggregate that DOES need a new `conductor-report::RunsDb` accessor (it exposes only `open` / `insert` / `get(run_id, scenario)` today — JSON1 over `p_ids`), still deferred to Epoch 10. Don't conflate them: surfacing a run's report ≠ the cross-run per-P-ID rollup. Two smaller carries from the same chunk: a per-run `RunRecord` already derives `Serialize`, so the Tauri command returns it directly with **no `#[derive]` add** (unlike `CapabilityRow` in ch6); and the operator-checklist **view** shipped presentational / DEV-gallery-only because the induced/observation pairs have **no structured `conductor-core` model** yet (declare-only TOML comments — a future scenario-model field, not this chunk).

---

## 2026-06-26 — Extracting the shared run composition root to a new crate (above the seams) + driving it under Tauri on a background current_thread runtime

ch3's option-A scaffold deferred real GUI execution; ch4 made `start_run` drive the real pipeline by EXTRACTING `conductor-cli`'s bin-local `pipeline.rs` (the composition root) into a NEW library crate `conductor-run`, consumed by BOTH bins. The placement is forced by the crate-per-seam DAG: the composition root imports every seam (`conductor-timeline`/`-emit`/`-verify`/`-report`), and those all depend on `conductor-core` (the base) — so a core-hosted composition root would invert every seam edge into a cycle and won't compile. It must sit ABOVE the seams and BELOW the bins (`{cli,tauri} → conductor-run → {core,timeline,emit,verify,report}`, zero cycles). This refines the 2026-06-23 "second composition root" entry: the two bins SHARE one extracted library (DRY — the CLI's `run`/`suite`/`preflight` re-point to `conductor_run`, `persist` moves there too), they don't each re-compose. `execute_scenario` generalized its resolver from the bin-local `&CliResolver` to a generic `<R: PauseResolver>` (NOT a trait object — RPITIT-async, not object-safe; the 2026-06-21 generic-over-dyn rule) so the CLI passes its `CliResolver` and Tauri the core `HeadlessResolver`. The existing `cli_smoke` E2E staying green is the parity proof the extraction was behaviour-preserving (test-plan Path 7 headless leg).

Driving the engine under Tauri needs a core-owned `current_thread` runtime OFF Tauri's multi_thread shell (the determinism flavor, arch §Async Runtime Flavor). The shipped pattern: the sync `#[tauri::command] start_run` spawns a `std::thread`, which builds `tokio::runtime::Builder::new_current_thread().enable_all().build()` and `block_on`s the pipeline; it returns IMMEDIATELY (non-blocking UI). Progress streams over a Tauri 2 IPC `Channel<RunEvent>` passed as a command ARG (frontend `new Channel()` → `channel.onmessage`), which needs NO capability entry (app-command Channels aren't ACL-gated — the deny-by-default window allowlist is unchanged). `stop_run` sets an `Arc<AtomicBool>` the run loop polls between scenarios (cooperative abort). Without a live Pulse every run resolves Blocked (the preflight gate) and emits nothing, so the live `count` is scenarios-completed (0→N), not per-emission — honest + observable now, with the faithful per-emission counter the Epoch-10 bridge. The `tauri.command.*` manual span + `sanitize_error` edge (obs rule) carry over. Reusable for the remaining Epoch-9 Tauri command chunks (ch5 component primitives, ch8 operator-pause dialog).

---

## 2026-06-24 — A full debug build of the workspace + Tauri tree needs ~33–37 GB of disk

Standing up `conductor-tauri` as a real Tauri 2 app pulls in the full Tauri/wry/tao/webview2-com/windows-* tree (hundreds of crates). A from-scratch `cargo nextest run --workspace` (all test binaries, `debuginfo=2`) drives `target/` to ~33–37 GB — `target/debug/incremental` alone reached ~14 GB. On a near-full disk this surfaces mid-compile as `rustc-LLVM ERROR: IO failure on output stream: no space on device` / `os error 112` / `STATUS_ACCESS_VIOLATION` (rustc crashing as it fails to write), NOT a code error.

Recovery: `cargo clean` frees the whole `target/` (then a full recompile); or delete the regenerable `target/debug/incremental`. The destructive-command guard blocks a raw `rm -rf target/...`, so `cargo clean` is the blessed, non-`rm` way to reclaim the space. The individual gates (per-crate nextest, targeted clippy, `cargo build -p conductor-tauri`, deny/audit) each fit — it's the *combined* full-workspace test compile that exhausts the disk. Budget the headroom before a from-scratch Tauri build.

---

## 2026-06-24 — Edition-2024 makes `std::env::set_var` unsafe; read env handles as triggers instead

Rust **edition 2024** marks `std::env::set_var` / `remove_var` as `unsafe` (mutating the process environment is not thread-safe). So a spec that says a CLI flag "sets `CONDUCTOR_AGENT_MODE=1` internally" (obs-plan §3) should NOT be implemented by writing the env from `main`. The shipped pattern (agent-mode logging chunk, decision D4): treat the env var as a **read-only trigger** — `let agent_mode = cli.agent_mode || std::env::var_os("CONDUCTOR_AGENT_MODE").is_some();` — and thread the resolved `bool` to the consumers (the obs sink selection + `CliResolver::select(…, agent_mode)`). The harness/operator exports the env OR passes the flag; `main` never writes it. Observable mode is identical, no `unsafe`, and the bool is unit-testable via a pure `resolve_kind(agent_mode, stdin_tty, stdout_tty)` (the `render::*_styled(color)` testable-core pattern). When a spec says a flag "sets" an env var, prefer this read-and-thread shape and reconcile the spec wording at wrap.

A related obs gotcha from the same chunk: `tracing_subscriber`'s `build_subscriber<W: MakeWriter>` is generic, but `set_global_default` takes ONE concrete subscriber — you cannot `if agent { build(file) } else { build(stderr) }` (the two `W` types differ). Unify the sinks behind an `ObsWriter { Stderr, File(Arc<Mutex<File>>) }` enum that impls `MakeWriter`, dispatching per-line (the 2026-06-23 `CliResolver` enum-dispatch theme applied to writer-type unification). Open the agent file with create+truncate (the `-latest` name) and fall back to `Stderr` on open failure so startup logging never blocks.

---

## 2026-06-23 — CLI operator-pause resolver: enum dispatch over a non-object-safe trait

`conductor_core::PauseResolver::resolve` returns `-> impl Future<Output = Decision>` (RPITIT), which is NOT object-safe — a `&dyn PauseResolver` will not compile. So the CLI dispatches a fixed `CliResolver { Interactive(PromptResolver), Headless(HeadlessResolver) }` enum (in `conductor-cli::pause`) that impls `PauseResolver` by `match`-ing each arm to its inner resolver's `.resolve(hold).await`. `CliResolver::select(Option<ProgressBar>)` is the isatty gate — interactive only when BOTH `std::io::stdin()` and `stdout()` are `.is_terminal()` (the same `IsTerminal` primitive `render::stdout_color()` uses); off-tty it returns `Headless(HeadlessResolver::proceed())` so the agent path is never gated on a prompt.

The `Interactive(PromptResolver)` variant carries the live `Option<ProgressBar>` (the suite spinner; `run` passes `None`) and wraps the `inquire::Confirm` in `ProgressBar::suspend(…)` so the heartbeat freezes at its current count during the prompt, then resumes. The `[HOLD]` phase-line (`render::hold_line`, amber `lamp_code(Lamp::Hold)=179`, prefix always present) + the confirm both emit to **stderr**, keeping STDOUT the machine-parseable results table. `resolve` is infallible, so an `inquire` cancel/interrupt collapses to `Decision::NoGo` (when `allow_no_go`, else `Go`) via a catch-all arm. The Epoch-9 Tauri go/no-go dialog will extend the SAME enum + `Decision` vocabulary — a third arm, not a parallel mechanism.

---

## 2026-06-23 — conductor-cli line-oriented render seam (owo-colors + indicatif + comfy-table)

The `conductor-cli::render` seam colorizes the existing `Lamp` projection (`Lamp::for_record` / `status_prefix` / `label` from conductor-core — the single status-truth source, never reclassified) and renders comfy-tables. Status is never color-alone: the ASCII `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefix is always present; color is a tty-gated overlay.

Techniques for future CLI rendering (Epoch-8 ch4/ch5 will reuse this seam): (1) ONE `stdout_color()` gate (`std::io::IsTerminal` on stdout + `NO_COLOR` unset + `TERM != dumb`) drives BOTH the owo-colors lines AND the comfy-table cells — comfy-table can't use owo-colors' `if_supports_color`, so a single shared bool keeps lines + tables consistent and guarantees piped/agent output carries zero ANSI escape bytes (this is why the planned owo-colors `supports-colors` feature was dropped — unneeded). (2) Public render fns delegate to private `*_styled(…, color: bool)` cores so tests assert BOTH the plain and colored paths deterministically, independent of how the harness wires stdout — relying on test-capture's tty status is fragile (`cargo test` keeps fd 1 a terminal via thread-local capture; nextest pipes it). (3) `comfy_table::ContentArrangement::Disabled` (content-sized, no wrap/truncate) keeps scenario names + status tokens contiguous so substring E2E assertions hold — the right call for a machine-parseable agent tool over width-detected `Dynamic` (which wraps mid-token). (4) The indicatif spinner draws to STDERR via `ProgressDrawTarget::stderr()`, explicitly swapped to `::hidden()` when `!std::io::stderr().is_terminal()`, so piped/agent runs aren't corrupted (the operator-pause spinner-freeze is ch4). (5) Exact xterm-256 colors: `owo_colors::XtermColors::from(u8)` (lines) + `comfy_table::Color::AnsiValue(u8)` (cells) both take the raw 256-index, matching the design-system §Surface: cli ANSI map (Pass 114 · Fail 203 · Hold 179 · Manual 146 · Residual 246 · Blocked 60 · ID-cyan 117).

---

## 2026-06-23 — `conductor preflight` verb: shape, the hermetic-test manifest fixture, and `.ps1` exit-code gotchas

The `agent-run boot` command shells out to `conductor preflight --json` — the verb ch1 never shipped (`boot` called it from the skeleton, unrecognized, until now). It is a thin serialize-and-exit over a new `pipeline::readiness(manifest_path) -> anyhow::Result<ReadyState>` that REUSES the hardened `ReadbackClient::connect` + `run_preflight` UNCHANGED (no new spawn boundary — the consume-shipped-hardened-infra rule held: the security/obs drift-detectors returned clean without escalating). On an unreachable read-back path it synthesizes the Blocked `ReadyState` inline, mirroring `conductor_verify::preflight_boot`'s Err arm; the `UNREACHABLE_PRECONDITION` string is DUPLICATED in `pipeline.rs` because verify's const + the `spawn::*` helpers are `pub(crate)` (widening them was out of this chunk's scope). `pipeline::preflight` (the run/suite path) was left UNTOUCHED — it retains the `ReadbackClient` for scenario reuse while `readiness()` discards it, so they can't merge without double-spawning the sidecar. Deferred DRY follow-up: expose a `conductor_verify::readiness(data_dir, manifest, canary) -> ReadyState` real-sidecar sibling of `preflight_boot` (its doc already names the `conductor preflight` verb as a driver) to retire the duplication.

Testing gotcha — the hermetic `conductor preflight` test needs MORE than the ch1 forced-unreachable pattern: it must ALSO copy `contracts/mcp-contract.toml` into the TempDir, because `readiness()` loads the manifest BEFORE connecting, so without it the verb errors with manifest-not-found instead of emitting `ready:false`. This is asymmetric with the `conductor run` unreachable leg (`pipeline::preflight`), which early-returns Blocked WITHOUT loading the manifest — so `run`/`suite`/`report` hermetic tests need no manifest fixture, but any `preflight`/`boot` one does.

`.ps1` harness-authoring gotchas (the `.sh`⇄`.ps1` parity surface): use `[Console]::Error.WriteLine(msg); exit N` (NOT `Write-Error`) for deterministic exit codes — under `$ErrorActionPreference='Stop'`, `Write-Error` throws a terminating error so the intended `exit N` becomes dead code (the script exits 1, not N). And never name a PowerShell variable `$env` (it collides with the `$env:` provider) — use `$envelope` etc. The `.sh` `boot` wrapper's `timeout 30 …` works under Git Bash on Windows because `which timeout` resolves to coreutils `/usr/bin/timeout`, not Windows' `timeout.exe`.

---

## 2026-06-23 — The Epoch-8 CLI is the sole composition root; the engine seams are mutually independent, so two content bridges are live-only (Epoch-10)

`conductor-cli`'s `run`/`suite`/`report` verbs are the FIRST place the full pipeline (timeline → emit → verify → report) composes end-to-end. The code-graph `crate_edges` show why: every seam (`conductor-timeline`/`-emit`/`-verify`/`-report`) depends ONLY on `conductor-core`, never on each other — so nothing wired them together before the CLI. The CLI bin may depend on all library seams (it is the composition root); the forbidden edges are library→library. Consequence for Epoch-9: the Tauri GUI is the second composition root and reuses the same wiring shape over the same `conductor-core` types (`Scenario`/`RunRecord`/`Lamp`/`init_observability`).

Two content bridges are a direct consequence of the seam independence and are UNBUILT + live-Pulse-only — they belong to the Epoch-10 "Live-Pulse E2E proof", not the CLI surface: (1) **faithful per-scenario emission** — `EmissionSpec` carries only a coarse `Signal` class (`convert.rs` drops the emission descriptor; the Epoch-3 emit builders were never wired to it), so ch1 coarse-emits one generic span/log per phase by `Signal`, not each scenario's specific incident shape; (2) **per-check read-back observed-extraction** — nothing maps an `ExpectedCheck` to which MCP tool to call + how to slice the observed string, so ch1 feeds a coarse marker into `evaluate_check`. The P4 decision ("coarse live + full Blocked path") wired the full orchestration with these two coarse, deferring fidelity to Epoch-10.

Practical reality for ch1: a run produces a **Blocked** envelope in BOTH CI (the sidecar spawn fails — no `andromeda-pulse-mcp`) AND a default live Pulse (the preflight data-dir **canary** round-trip requires a faithfully-emitted canary incident, which is Epoch-10). So the coarse measured path (emit→read-back→classify) is wired + compiles but is gated behind `ready` and effectively dormant until Epoch-10 lands faithful canary emission. The CI-tested spine is the no-Pulse→Blocked path + the verbs + persist/report; the live measured leg stays operator-gated as always. Empty-`expected` (operator-checklist / declare-only) scenarios produce a verdict-less `ManualCheck` record via direct `RunRecord` construction — no core change, since `Lamp::for_record` already maps `(ManualCheck, None) → Manual`. The CLI-level root `scenario.run` obs span (obs §4 must-trace) was omitted this chunk — a carried code follow-up (obs §4 stays target-state; the seam child-spans + the `run_id`-on-every-line correlation invariant via `init_observability` hold regardless).

---

## 2026-06-21 — Scenario-catalog chunks (Epoch 7): declarative config + `expected` model carrier; runtime realization is deferred to the Epoch-8 driver

A scenario-catalog chunk (the first was connection-lifecycle P-001..P-004) authors **declarative `scenarios/*.toml`** plus the model field needed to express them, and **defers all runtime realization to the Epoch-8 CLI driver**. In scope: the TOML identity (`name`/`p_ids`/`seed`/`slo_tier`), the `[[phases]]` timing structure (legible names + `gap_ms` encoding the lifecycle windows), and the `[[expected]]` read-back targets. Out of scope (the driver wires these later over existing machinery): the emit-on/off + silence realization (Listening = no spans, Idle/Stalled = stopped — `EmissionSpec` is `#[non_exhaustive]`, the extension point if in-config silence is ever wanted), the P-003 `:4317` port-occupier bind (the `conductor-faults` helper already exists; never bind in config/tests), and the live MCP read-back/verify. So the chunk's proof is purely a committed-fixture round-trip (`#[rstest]` over each TOML through `Scenario::from_toml_str` → valid `PhaseTimeline`) + determinism under `start_paused` — no live Pulse, no boot-path change (the `agent-run.sh status` smoke needs a `run_id`, so it is skipped). The per-P-ID parameters (state thresholds, SLO windows) come from `refs/pulse-capability-spec.md` §"Conductor verification" clauses, not from input.md alone.

The `expected` read-back targets wire into the model as an **additive `Scenario.expected: Vec<ExpectedCheck>`** — `#[serde(default)]` (omission stays valid; existing scenarios unaffected) + `#[garde(dive)]` (each check validates at load), reusing the pre-existing `conductor-core::ExpectedCheck`/`ClaimClass`/`ComparisonKind` (already exported) rather than a new type; the `impl From<&Scenario> for PhaseTimeline` reads only `phases`+`jitter_ms`, so the new field is non-breaking (only the in-tree `Scenario { .. }` test struct-literals need the field appended). This closes the carried "Scenario.expected/holds TOML wiring" follow-up; `holds` stayed deferred (drive+observe timing claims need no operator go/no-go — the visual badge is a `ManualCheck` report-state, not a `HoldPoint`). One model limitation to remember for the latency (P-011/P-012) and tracker (P-002) families: **`ComparisonKind` {Exact, Contains, Absent, CountAtLeast} has no tolerance-window kind** — a "±1s" or "p95-within-band" claim is expressed with the closest existing kind and the numeric tolerance left to the Epoch-8 evaluator; resist extending the comparison vocabulary in a config-authoring chunk (keep the model change additive-minimal). The chunk shape (4 files, one P-ID each; wire-expected-now) was a /andromeda-phase P4 AskUserQuestion decision.

---

## 2026-06-21 — Coverage classification (`CoverageMode`) is a third, orthogonal axis; the Epoch-6 matrix is classify-only

The coverage-matrix generator introduced `conductor-core::CoverageMode` (`auto` / `drive+observe` / `static-only`) — Conductor's **third classification axis**, distinct from `Verdict` (Pass/Fail/CalibrationRegion — a per-check *outcome*) and `ClaimClass` (hard/calibration — the assertion *policy*). `CoverageMode` answers a different question: *how* Conductor verifies a capability (drive+assert via MCP read-back · induce+operator-confirm the visual · no-telemetry-dimension/Pulse-owned). Don't conflate them — the coverage matrix classifies by `CoverageMode`, the run report lamps by `Verdict`, the assertion split routes by `ClaimClass`.

The Epoch-6 `coverage-matrix.md` is **classify-only** (user-confirmed at /andromeda-phase): the committed 60-row mode table + a pure Markdown render, with **no `runs.db` read and no `Lamp`**. Although `lamp.rs:9` names the coverage matrix a future `Lamp` consumer, the *status overlay* (latest verdict per P-ID, lamp-rendered) belongs to the **consuming surface that has run data** — the Epoch-8 cli table + Epoch-9 desktop view — not the classification model (no scenarios/runs exist until Epoch 7, so a status column would be all em-dashes). The classification is **code-native**: a committed Rust `static` table (the source of truth), seeded at authoring time from `.andromeda/refs/capability-verification-matrix.json` + input.md §Coverage classification — NOT read from disk at runtime (keeps the render pure, adds no input boundary). Mind the **lens shift**: the refs JSON carries *Pulse's* modes (`automated-nextest`/`by-construction`/…); re-map each P-ID to Conductor's three (the JSON's "Dynamic-verification (Conductor)" notes mark where Conductor owns the timing bound but the visual is operator-observed → `drive+observe`). Completeness — all 60, zero gaps, each exactly one mode — is the definition-of-done gate, asserted by test.

---

## 2026-06-21 — Artifact write lifecycle: regenerated singletons overwrite atomically; per-run artifacts use `create_new`

Two artifact write lifecycles now coexist in `conductor-report` and take **opposite** overwrite semantics — pick by whether the artifact is per-run-immutable or a regenerated singleton. **Per-run artifacts** (`<run_id>.md` report, `<run_id>.jsonl` journal) are run_id-stemmed and immutable, so they open `OpenOptions::create_new` — a repeat write is a loud `Err`, never a silent clobber. **A regenerated definition-of-done singleton** (`coverage-matrix.md`) is the opposite: one canonical file re-derived as the classification evolves, so regeneration MUST succeed — it uses an **atomic deterministic overwrite** (`.tmp` → `fs::rename`, which replaces the destination on both Unix and Windows), and the test asserts a *second* write overwrites cleanly (the inverse of the run report's never-overwrite test).

Watch for borrowed assumptions: this chunk's spec extracts both said the coverage write should be "loud-never-overwrite consistent with the run-report seam" — a wrong carry-over from the run-report lifecycle. A singleton that can't be regenerated is a bug, not a safety feature. The discriminator is artifact identity: run_id-stemmed + immutable → `create_new`; single canonical name + regenerated → atomic overwrite. Both stay "loud" on a real IO fault via the typed `ReportError` (the verdict/error wall).

---

## 2026-06-21 — Verdict-first lamp precedence is centralized in `conductor-core::Lamp`; the render seam is a pure (clock-free) function

The run-report "lamp" (the `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` status) is now a single shared type — `conductor-core::Lamp` with `Lamp::for_record(&RunRecord)` — so the four render surfaces (the Markdown report this chunk, the coverage-matrix Epoch 6 ch4, the cli Epoch 8, the desktop Epoch 9) **reuse one resolver and never re-derive the precedence**. A `RunRecord` carries `verdict` and `state` independently; the lamp is chosen *verdict-first* — but only for the Pass/Fail/Hold trichotomy (`Some(CalibrationRegion)` → `[HOLD]`, the rule that a calibration row reads HOLD, not Manual). The subtlety worth remembering: **`KnownResidual` and `Blocked` are state-driven and MUST be checked BEFORE the verdict arms**, because `RunRecord::measured()` always supplies a `Verdict`, so a `KnownResidual` row carries `Some(verdict)` — a naive "verdict-first else state" would mis-render an accepted residual as `[FAIL]`/`[PASS]`, defeating the state's whole purpose (distinguishing an accepted residual from a real Fail). `Lamp::for_record` matches `(state, verdict)` with `Blocked`/`KnownResidual` first, then the three verdict arms, then a verdict-less state fallback (`ManualCheck` without a verdict → `[MANUAL]`, the operator-checklist case).

The Markdown renderer (`conductor-report::RunReport`) is a **pure function of `(run_id, &[RunRecord])` — it reads no wall-clock**, so its output is byte-deterministic and lockable with an exact-string `assert_eq!` golden (the crate's convention, not `insta`). The temporal anchor is the `run_id` (which already embeds the run timestamp) plus each record's own RFC-3339 instants; a human-facing "generated at" line, if ever wanted, is injected by the cli edge, never read inside the seam — the same "no clock in the seam" discipline as the std::time-vs-tokio journal rule, applied to a one-shot document. The writer `RunReport::write` uses `OpenOptions::create_new` so a repeat `run_id` is a loud `Err`, never a silent clobber (mirrors `RunsDb`'s loud duplicate-key + `JournalWriter`'s never-truncate). The blocked-row null rule renders the five never-measured fields as an em-dash `—`, never the literal `null` or a struct name (artifact hygiene); a measured-but-empty fingerprint set (`Some([])`) renders `(none)`, distinct from a never-measured `None` (`—`).

---

## 2026-06-21 — Mapping a typed envelope onto rusqlite (runs.db): u64 bit-cast · serde-wire-form enums · parse OUTSIDE the row closure

Persisting a `conductor-core::RunRecord` as a `runs.db` row (the new `conductor-report::RunsDb` seam) surfaced three rusqlite mapping gotchas worth reusing for the Epoch-7 P-036 recurrence query + the Epoch-8 `status` read. (a) **`u64` overflows the SQLite signed-`INTEGER` (i64) column** — rusqlite's `ToSql for u64` *errors* when the value exceeds `i64::MAX`, so a `seed: u64` must be stored via an `as i64` bit-cast on write and read back `as u64` (lossless; tested against `u64::MAX`), never bound directly. (b) **Store serde enums as their wire string, not a hand `match`** — `Verdict`/`ReportState`/`SloTier` go into TEXT columns via `serde_json::to_value(v)?.as_str()` and come back via `serde_json::from_value(Value::String(s))`, so the stored spelling stays identical to the JSONL `#[serde(rename)]` (`<5s`, `Pass`, …) with the rename as the single source of truth — a parallel match would silently drift if a variant is renamed.

(c) **You cannot parse serde/JSON inside the `query_row` mapping closure** — that closure must return `rusqlite::Error`, but `serde_json::from_str`/`from_value` (the JSON1 array columns `p_ids`/`fingerprints`, plus the enum wire forms) produce your own error type. Read the raw column values into a private `RawRow` struct inside the closure (all `row.get(i)?`), then convert `RawRow -> RunRecord` OUTSIDE the closure where your `Result<_, RunsDbError>` + `?` compose. The `Blocked`-row NULL rule then falls out for free: the envelope's five measurement fields are already `Option`, so `None` binds to SQL `NULL` and a NULL column reads back as `None` — no special-casing.

The `RunsDb` type mirrors its sibling `JournalWriter`: `open(runs_dir: &Path)` takes the already-resolved dir (the cli edge owns `CONDUCTOR_RUNS_DIR`; the seam never reads env), `CREATE TABLE IF NOT EXISTS` bootstrap (no migration framework), bound parameters only, `#[non_exhaustive]` thiserror `RunsDbError` (`Io`/`Sqlite`/`Json`). Note: rusqlite 0.38.0's first real `bundled` compile resolves `libsqlite3-sys 0.36.0` → SQLite **3.50.4** (not the 0.38.0/3.51.1 the specs had assumed — arch §Stack + stack.md corrected this wrap); JSON1 (`json_array_length`) is present and the `≥3.38` floor holds.

---

## 2026-06-21 — clippy `too_many_arguments` counts `&self`: a 7-arg method still trips `8/7`

Under the `cargo clippy --all-targets -- -D warnings` gate, `clippy::too_many_arguments` counts the `self`/`&self` receiver toward its 7-argument threshold — a method with seven non-self parameters fires `8/7` and fails the gate, exactly like an eight-parameter free function. So a faithful many-field constructor needs `#[allow(clippy::too_many_arguments)]` whether it is an associated fn (`RunRecord::measured`, eleven params) OR a method (`CheckOutcome::to_run_record`, `&self` + seven). Do not assume the receiver is exempt. Pair the allow with a one-line justification (here: the run-report envelope is eleven fields by contract — arch §Standard Contracts). (run-report-envelope-serializer chunk, conductor-core + conductor-verify)

---

## 2026-06-21 — A public async trait under the `-D warnings` clippy gate: declare RPITIT, not `async fn`

A public trait method written as `async fn` trips the `async_fn_in_trait` lint (callers can't add a `Send`/lifetime bound on the returned future), which fails the `cargo clippy --all-targets -- -D warnings` gate. Declare it with return-position `impl Trait` instead — `fn resolve(&self, hold: &HoldPoint) -> impl Future<Output = Decision>;` — while implementors may still write `async fn` (an `async fn` in an impl satisfies an `-> impl Future` trait method, stable since Rust 1.75). Keep the orchestration **generic** over the trait (`fn resolve_hold<R: PauseResolver>(…)`) rather than `dyn Trait`: this sidesteps the dyn-incompatibility of RPITIT async methods AND avoids pulling in the `async-trait` crate, so a new async abstraction can land in an otherwise sync/dependency-light crate (here `conductor-core`'s `PauseResolver` — the crate's first async surface) with **zero new runtime dependencies** (only a `tokio` dev-dep for the test executor). (operator-pause-orchestration chunk, conductor-core)

---

## 2026-06-20 — conductor-faults: an *infallible* constructor when a helper has NO failure mode — the third branch of the fault-constructor idiom

The 2026-06-19 entry below split faults-helper construction two ways: a *named domain constraint* → `Result<Self, FaultError>`; an *anonymous pure-value bound* → `Option`. **P-014 `AbruptSilence` is the third branch: no failure mode at all → an infallible `new() -> Self`, with `FaultError` left untouched** — the crate's first infallible helper. Unlike `EmissionGap` (a 20s floor to validate) or `PortOccupier` (a socket to acquire), a *permanent* emission stop has no bound and no resource, so there is nothing to fail on; permanence is modeled as the structural *absence* of a `Duration`, and a positive `resumes() -> bool { false }` accessor makes the no-resume contract testable against `EmissionGap`.

Heuristic completing the prior entry: pick constructor fallibility by whether a *real* failure mode exists — `Result` + a named `FaultError` variant for a threshold/resource failure, `Option` for an anonymous shape bound, and an **infallible `Self`** when there is neither. Do NOT force a `Result`/`Option` "for symmetry" with the siblings — a constructor that is structurally always-`Ok` is the computed-but-never-applied anti-pattern (the same reasoning that kept the seed off the exact-gap helper). User-ratified via /andromeda-phase AskUserQuestion (Helper shape → "Infallible marker").

---

## 2026-06-19 — conductor-faults helpers fail a *named domain constraint* with `Result<_, FaultError>`; emit's pure-value bounds use `Option`

The two seam crates split their constructor-validation idiom by the KIND of invalid input. A **`conductor-faults`** helper whose construction can violate a *named domain constraint* returns `Result<Self, FaultError>`, extending the `#[non_exhaustive] FaultError` enum with a descriptive variant: `PortOccupier::occupy` → `Bind` (a refused loopback bind), and now `EmissionGap::new` → `GapTooShort`/`GapTooLong` (a gap at/below the 20s P-015 restart threshold, or above the 1h ceiling). A **`conductor-emit`** generator validating a *pure-value bound* uses an `Option`-returning constructor instead — `RateCurve::ramp`/`breathing` (`windows>0`, `amplitude<center`), `Severity::new`, `LatencyProfile::new` (the sibling 2026-06-18 emit entry below).

Heuristic for the upcoming faults chunks (P-014 abrupt-silence, P-013 bursty-train): reach for a typed `FaultError` variant when the failure has a *named cause worth surfacing* (a threshold breached, a resource denied) — the name aids the operator and the verdict/error wall; reserve `Option` for anonymous "these numbers don't form a valid shape" bounds. Both honor the rule that invalid input is a value, never a panic. Note the gap is **seed-independent** — an exact `Duration`, deterministic by construction (no seed param to thread), the same seed-only-governs-what-it-drives principle as the 2026-06-18 fingerprint entry in `.claude/rules/testing.md`.

---

## 2026-06-18 — Emission primitives self-validate their typed input in-crate (constructor); core garde is the *later* scenario-wiring validator

Each `conductor-emit` primitive owns and validates its typed input **inside the emit crate** via an `Option`-returning constructor that enforces the invariant — NOT by deferring to `conductor-core`'s garde layer. `Severity::new` (rejects outside `1..=24`, severity-logs) and now `LatencyProfile::new` (rejects unless `p50 ≤ p95 ≤ p99`, latency-shaping) are the pattern. This follows the emission-seam comment in `phase_spec`: the concrete OTLP taxonomy (severity boundaries, fingerprint identity, latency targets, ramps) lands in the Epoch-3 emission seam, not the scenario model.

Consequence for the scenario-wiring epoch: `conductor-core`'s garde is the authoritative validator only *later*, when these targets wire into scenario config by extending the `#[non_exhaustive] EmissionSpec`. The `scenario.rs` note that the p50≤p95≤p99 invariant "joins when the Epoch-3 latency spec lands" refers to that future garde wiring — distinct from the primitive's own constructor check, which is this chunk. So a new emission primitive (topology, PII, ramps) defaults to an emit-local, constructor-validated typed input; don't reach into core's scenario garde for it this early. Complements the placement heuristic in the sibling 2026-06-18 entry (primitive → producing seam crate) with the validation-location dimension.

---

## 2026-06-18 — Emission/compute *primitives* live in their producing seam crate; the *fault* that composes them lives in conductor-faults

The build route places a low-level emission/compute **primitive** in the crate that produces its raw material, even when the module-map one-liner nominally attributes the broader concern to another crate. The per-exception **fingerprint primitive** (`fingerprint()` + the exception-event builder) landed in `conductor-emit` — co-located with the exception content it derives from — NOT in `conductor-faults`, despite arch / CLAUDE.md §Modules listing "fingerprint generation" under faults. That attribution is now narrowed: `conductor-faults` owns the higher-level **fingerprint-storm FAULT** (Epoch-7), which will depend on `conductor-emit` and *compose* this primitive. This recurs from error-spans (its multi-span builder also landed in emit, not faults).

Heuristic for future phase/placement calls: a PRIMITIVE goes in its producing seam crate (`conductor-emit` owns OTLP-message construction + anything derived directly from it, like the content fingerprint); a FAULT that orchestrates/composes primitives goes in `conductor-faults`, built later (Epoch-4+). When the module-map blurb seems to conflict, prefer co-location with the data + the dependency direction (faults → emit), then reconcile the doc (arch §Modules amended this chunk). User-ratified via /andromeda-phase AskUserQuestion.

---

## 2026-06-17 — OTLP emission scaffolding (conductor-emit) notes

The Epoch-3 emission seam builds raw OTLP messages from `opentelemetry-proto` 0.32.0 directly (not the SDK exporter). Two facts for the upcoming emission chunks (error-spans, exception-events, severity-logs, latency, topology, PII, ramps):

- **No `build.rs` / `tonic-prost-build`.** opentelemetry-proto's `gen-tonic` feature ships the generated `TraceServiceClient` (+ server stub) and the message structs (`ResourceSpans` / `Span` / `Status` / …); there is no local `.proto` to compile, so the workspace needs no build script.
- **Build proto structs with `..Default::default()`.** opentelemetry-proto 0.32.0's `KeyValue` carries a third field (`key_strindex`, a newer OTLP string-table index), and other messages gain fields across proto versions. Set only the fields you control and spread `..Default::default()` for the rest — an exhaustive struct literal breaks when a proto-version bump adds a field.

---

## 2026-06-15 — `--profile ci` not defined until the test-framework chunk (regression-gate workaround)

The documented test command `cargo nextest run --workspace --profile ci` (CLAUDE.md §Workflow · `.claude/docs/commands.md` · `.claude/rules/verification-harness.md`) **fails** with `error: profile 'ci' not found (known profiles: default, default-miri)` — the `.config/nextest.toml` that defines the `ci` profile is created by the later Epoch-1 chunk "Test framework + fixtures + coverage tooling" and does not exist yet. Until that chunk lands, every implement chunk's regression gate hits this.

Workaround: run `cargo nextest run --workspace` (the default profile runs the identical test set — only the run-config differs: retries / JUnit / output — and tests are profile-independent). Do NOT create `.config/nextest.toml` ad hoc in an unrelated chunk; that profile is the test-framework chunk's deliverable. Same Foundation-sequencing class as the `playbook.md` rule about interim `cargo test`.

---

## 2026-06-15 — cargo-deny over an unpublished workspace needs `publish = false`

`cargo deny check` treats every workspace member as a *publishable* crate unless it is marked `publish = false`. For the `conductor-*` crates (no `license` field, internal `path` deps) that produced two error classes at once: `error[unlicensed]` (a public crate must declare a license) and `error[wildcard]` ("allow-wildcard-paths is enabled, but does not apply to public crates as crates.io disallows path dependencies"). Both vanish once the crates are `publish = false` — `[bans].allow-wildcard-paths = true` then covers their internal `path` deps. The license half is no longer skipped **[corrected 2026-09-29: `deny.toml` carries no `private.ignore` exemption any more — every member inherits `license = "MIT OR Apache-2.0"` from `[workspace.package]` and `cargo deny check licenses` checks it; measured red (nine `error[unlicensed]`) with the exemption removed and no license, green once the license landed]**.

The fix is `publish = false` in `[workspace.package]` + `publish.workspace = true` per crate (matching the existing version/edition/rust-version inheritance). Correct for a local-only, no-cloud tool that never publishes to crates.io — and it strengthens supply-chain posture rather than weakening the gate. Don't reach for `wildcards = "allow"` or dropping the license check to dodge it.

---

## 2026-06-15 — garde 0.22.1 API gotchas (config validation)

Conductor pins **garde 0.22.1**, not the arch's original 0.23.0: `garde_derive 0.23.0` is absent from the registry (latest 0.22.1), so garde 0.23.0 + the `derive` feature is unbuildable here. When wiring garde into a seam crate:
- `derive` is **not** a default feature — the edge must be `garde = { workspace = true, features = ["derive"] }`, or `#[derive(Validate)]` / the `#[garde(...)]` helper attribute won't resolve ("cannot find derive macro `Validate`").
- `Validate::validate(&self)` takes **no** context argument (returns `Result<(), garde::Report>`); call `.validate()`, not `.validate(&())`, for the default `()` context. The error type is `garde::Report`, bridged into `CoreError` via `#[from]`.
- `#[garde(custom(fn))]` is **field-level only** — there is no container/struct-level `custom` in 0.22.1 (it errors "unrecognized attribute"). A whole-list invariant rides on the one field it concerns (e.g. no-duplicate-P-IDs on `p_ids`); invariants spanning *distinct* fields (p50≤p95≤p99, severity-mix sums — the Epoch-2 emission spec) need garde's `Context` pattern or a manual `Validate` impl.
  - **Extended 2026-08-11 (faithful-emission-dispatcher): the predicted `Context` pattern was not needed — model the co-varying fields into ONE field first.** The emission spec's cross-field invariants (p50≤p95≤p99, severity ∈ 1..=24, breathing amplitude < center, ≥2 distinct topology services) all live inside a single `EmissionShape` enum whose variants own their own parameters, so one field-level `#[garde(custom)]` on that field validates every one of them. Reach for `Context` only when the invariant genuinely spans fields that cannot be co-located — restructuring the data is cheaper than threading a context, and threading one would have forced it onto `PId`/`PhaseSpec`/`EmissionSpec` and changed their public API.
  - **`#[garde(skip)]` on a nested struct is a silent hole, not a no-op.** garde never descends into a skipped field, so the nested type's own rules — however carefully written — never execute. `PhaseSpec.emission` was `skip` from the config-model chunk until this one, meaning every rule the specs claimed for the emission spec was unreachable. Use `dive`; a rule that cannot run is worse than an absent one, because the spec reads as covered.

Applies to every future garde validation surface (Epoch-2 `Scenario-config model` especially). See arch §Established Decisions [Validation Library] for the pinned-version decision.

---

---

## 2026-08-10 — A deferral is only real once it has an owned channel

A deferral recorded as report prose is not owned by anything: nothing re-reads a chunk report, so the
obligation evaporates at the next wrap. The owned channels are (a) a `CARRY:` pin appended to the
markerless working-route entry that owns the surface the work belongs to, or (b) a `.andromeda/residuals.md`
append when no in-version entry owns it — check the markerless tail FIRST, because a residual entry is the
cross-version escape hatch, not the default. The pin travels with the entry, so `/andromeda-phase` folds it
into `scope.md` at promotion and it cannot be silently skipped.

Two live confirmations of the cost of getting this wrong. The `sut-load-envelope` chunk deferred a webview
banner with the consequence "a GUI user sees no envelope signal" — real user-visible behavior that would
have been invisible to every later chunk had it stayed in the report. And dismissing a drift proposal in the
same wrap surfaced that the playbook's own deferred-span entries promised `db.insert_run` / `report.generate`
spans would land "with the Epoch-8 cli/timeline caller" — Epoch 8 completed and they never did, because that
deferral lived in a playbook `note` rather than on a route entry. A `note` explains a dismissal; it does not
own the work.

## 2026-08-11 — Emission timing is load-bearing: a phase's traffic must land inside its own window

Conductor drives a SUT whose detectors are **windowed** — Pulse's L2 retry-storm cue needs >=5 identical
fingerprints inside a rolling 30s, P-015 needs a >=20s silence gap, P-013 learns a quiet *pattern*. A harness
that emits the right SHAPE at the wrong TIME therefore proves nothing, and the failure is invisible from the
emit seam: every primitive's own tests pass, because each builds a correct payload.

The shipped `coarse_emit` had exactly this defect. `run_timeline` slept every phase gap and RETURNED, and only
then did emission run — so `fingerprint-storm` produced ~24s of silence followed by two spans emitted
back-to-back, and no windowed detector could ever have fired. The scenario's declared timing was computed,
elapsed, and then discarded.

The fix is structural, not a tuning knob: the scheduler drives emission through a caller-supplied per-boundary
hook, and a phase's declared occurrences are paced ACROSS that phase's own jittered gap. Two properties make it
safe — total elapsed per phase is unchanged (the seeded jitter draw stays one per phase, so the transition
stream and its determinism goldens are byte-identical), and the timeline crate gains no dependency on the emit
seam (the hook is generic). A phase declaring zero occurrences is a real silence window: the gap still elapses
and nothing is sent, which is how the activity-floor and restart families express quiet without a fault helper.

Corollary for any future emission work: if you are ever tempted to collect a stream and flush it after the
timing loop, the SUT sees one burst. Emit at the boundary.

## Entry format

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` (critical, short).
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` (loaded when matching files touched).
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read).

## Live Pulse captures are dominated by UI render telemetry (2026-08-21)

A full `pulse-app` leg capture is far larger than the evidence in it. The severity-lifecycle leg A slice was 138,065 lines / 48 MB, of which **128,820 were `metric.webgpu.frame_duration_ms`** — the Tauri webview's per-frame render metric, emitted continuously for the whole leg regardless of what the scenario drives. Filtering to the six load-bearing targets (`triage.pattern.storm.detected` · `interpretation.incident.created` · `triage.incident.auto_resolve.tick` · `triage.cue.emit` · `incidents.list_active.request` · `triage.incident.persist`, plus `ingest.tick` for the ingestion witness) took the same leg to 71 lines / 22 KB, and five legs of committed evidence to 502 KB total.

Consequence for evidence hygiene: a raw slice is not committable, and the filter is not a convenience — collapse the identical repeats too (the canary's frozen cue repeats every second, and `item_count` polls ~1,341 times per leg with only a handful of transitions). Keep transitions, drop steady state.


## Cargo workspace manifests: two build facts measured at 2026-09-07-dependency-polish

**`default-features = false` cannot be set on a MEMBER for an inherited workspace dependency.** A member
writing `dep = { workspace = true, default-features = false }` fails outright at manifest load:
`error: default-features = false cannot override workspace's default-features`. The flag must live on the
`[workspace.dependencies]` entry itself, and members then add only the features they use. This is not a
style preference — it is the only legal shape, and it has a useful side effect: one workspace entry covers
every dep site at once, which matters when several members share a dependency and a per-member trim would
be re-unified by whichever member still rode the defaults. Measured when the `opentelemetry-proto` trim was
first written at the two member sites the plan named, and cargo refused both.

**A per-member standalone-build sweep must run each member on its OWN targets.** `cargo check -p <crate>
--lib` exits **101** with `error: no library targets found in package <crate>` for a bin-only crate — here
`conductor-cli` and `conductor-tauri`, which have `src/main.rs` and no `src/lib.rs`. A uniform `--lib` loop
across the workspace therefore reports two false reds on the one sweep whose entire purpose is finding real
ones. Use `--lib` for the library crates and `--bins` for the bin-only ones. **Never `--all-targets`:** it
re-unifies dev-dependencies, which is precisely the masking the sweep exists to catch — a crate whose
feature is declared only in `[dev-dependencies]` compiles in the workspace and under `cargo test -p` while
failing alone, and `--all-targets` hides that again.

## A measured scalar that lives in several committed artifacts needs ONE canonical rendering (2026-09-14)

A value measured once and then quoted in a fixture, a doc comment, a ledger field and a plan will drift into
several renderings, and the drift is invisible to every gate — nothing compares a number across artifacts.
The failure is not that the renderings disagree in magnitude; it is that a reader comparing two of them sees
a discrepancy the underlying measurement does not have, and cannot tell which is authoritative.

Two rules make it cheap to avoid. Pick ONE canonical form for the value and use it everywhere, citing the
RAW measurement exactly once, anchored to the artifact that holds it — so a reader who wants full precision
knows where to look, and everyone else sees agreement. And check that the canonical form is a ROUNDING and
not a TRUNCATION: dropping the fractional part of a value whose fraction exceeds one half yields a number
that is wrong in the last digit while looking like a legitimate abbreviation, which is exactly the shape
that survives review. The committed artifacts are usually right and the newly authored one is usually the
outlier — when renderings disagree, check the new one against the oldest committed copy before assuming the
corpus drifted.

The generalization: a number is a claim, so it carries a basis and a canonical form like any other claim.
Where the value moves through the amendment channel, the playbook's measured-scalar rule now governs it and
requires the report to carry the basis; this entry covers the wider case, where the same value is simply
restated in several places by hand.

## An enumeration's SCOPE is established by what already sits OUTSIDE it (2026-09-16)

A drift proposal wanted `architecture.md`'s "**A second integrity gate sits beside it on a different
axis**" sentence widened to name a third gate, on the reading that it is architecture's enumerating
home for `conductor-core`'s static integrity gates. Read by offset — the sentence lives at `:53`
+1655 inside a 2 862-char line — it names `check_sut_drift` (is every accepted capability
*classified*?) and `check_scenario_backing` (does every `Auto` classification have a scenario?), both
on the accepted-capability-set axis.

What settled it was not the sentence but its neighbourhood: `check_load_envelope` is an existing,
shipped static gate of *identical shape* — a catalog-wide check returning a named `CoreError` — and it
appears nowhere in that pair, only at its own artifact row (`:177`). A sibling of the same shape
already living outside an enumeration is proof the enumeration was never exhaustive of that shape. So
the sentence was not stale, and the new gate's correct home was the artifact row a different proposal
in the same batch already covered.

The move generalizes to any doc-reconcile: before widening an enumeration because your new thing
resembles its members, look for a thing that already resembles them and is NOT in it. If one exists,
the enumeration is scoped by something narrower than resemblance, and widening it would make the doc
say less precisely what it used to say. The corollary is procedural — this is only visible from the
neighbourhood, so a proposal about an enumeration is checked against the doc's other mentions of the
same class, never against the proposed sentence alone.

## Ownerless work discovered mid-chunk takes the armed-orphan form, not a plan note (2026-09-16)

This chunk carried a finding with no owner: `scripts/mutation-gate.py` computes its verdict from
`missed.txt` and `caught.txt` and never reads `timeout.txt`, so a caught→timeout regression passes it
silently — and no route entry names the mutation gate, so nothing in the version owned the repair.
**[resolved 2026-09-30: the entry minted from this halt shipped — the gate now grades all four tallies,
`timeout.txt` included; the example above is the gate as it stood on 2026-09-16.]**

The instinct was to write it into the plan as "a route candidate". The operator's correction: that is
a CARRY, and a carry is how the previous version accumulated debt — it survives on attention rather
than on a mechanism. The pipeline already has the channel, `route-resolve.md:24`: an IN-VERSION
follow-up with no plausible owner entry is a trajectory halt that arrives **ARMED**, enumerating four
dispositions with a lean — pin to a named entry · mint an entry · `residuals.md` · drop — so the
decision reaches the operator at a named moment with the work already done, as one question rather
than a re-derivation.

Two details worth keeping. The lean has to be argued from the candidates: "pin to the nearest
plausible entry" was rejected here because the nearest was a full-gate regression sweep, and burying a
fifteen-item per-item disposition inside a sweep is the mixing hazard the finding is about. And the
operator's own sharpening on effort — reading `timeout.txt` is a small patch, but rostering the
fifteen timeouts is judgment, not a patch — is what made "mint an entry" the honest lean rather than
"absorb it here".

## 2026-09-16 — a11y CI terminal: instruments that cannot answer their own question

Five instrument failures in one chunk, all of the same family and each caught only by a control. Recorded
together because the pattern is the lesson, not any single case.

- **A probe validated outside the context it RUNS in proves nothing.** The token witness's UAC block was
  exercised in a bare `pwsh -Command` and passed; the script itself sets `Set-StrictMode -Version Latest`,
  under which member access on an absent registry property throws. In CI it threw on the first absent key
  and aborted the loop, losing exactly the two readings the diagnosis needed. Validate a probe in the
  context that will run it, not in a convenient one.
- **A self-report can under-count its own subject.** The orphan reap's `OrphansLeft` counts at the instant
  the last pass runs, while the process tree is still settling: it reported 1 while an independent audit
  against the leg window measured 6. An instrument that measures at its own convenience measures early.
- **"Registered ok" is not "ran that way".** `A11Y_LIMITED_TOKEN_REGISTER: ok` recorded what the scheduler
  was ASKED for; the task ran High-integrity anyway. Three CI runs were interpreted through that line
  before a witness inside the task settled it. Read the outcome from inside the thing, not from the
  request that configured it.
- **A diagnostic can be consistent with both answers.** The pipe-route probe concluded from "app alive +
  profile created", which an IGNORED switch produces identically to an accepted one. It could never have
  discriminated its own question.
- **The driver's advice is about the BROWSER, not itself.** msedgedriver's log recommends
  `--remote-debugging-pipe`; its `--help` lists 18 options with no pipe entry and it silently ignores the
  flag. The transport exists inside the driver with no surface to select it — so the route died at zero CI
  cost, once the binary was read instead of the log line.

**The cheapest discriminating question is often not a CI run.** Two candidate routes were eliminated on
this host for nothing: the pipe route by reading the driver binary, and the `runas`-prompts objection by
running `runas /trustlevel` once. Ask what a local read can kill before spending a twelve-minute iteration.

### Recurrence, not a new rule — clipped views, three instances in one session

The standing rule ("a count is never read off a clipped output") was breached **three times in one day by
two readers who both know it**: a per-path grep quoted from a clipped combined search; a site enumeration
taken from a narrower re-read than the code-graph query that had already answered it; and a `:397`-only
failure reading that nearly became "driver alignment made it worse" before `grep -oE` over the whole file
showed all three legs identical. Deliberately NOT written into Tier 1 or a rule file — the rule is already
stated in both always-loaded tiers, and restating it a third time buys cost, not compliance. What is new is
the frequency and that seniority is no protection; that belongs here.

A fourth instance of the same family landed in this wrap's own cascade: a sweep for `six governed forms`
returned **0** across the distillations while `.claude/rules/security.md` stated it as `**SIX**` — markdown
bold defeating the literal pattern. Caught by reading the `governed` hits instead of trusting the count.

## 2026-09-17 — the a11y CI terminal: how a six-day cause was found by removing the remedy

**The chunk's own deliverable was the blocker.** The medium-integrity launcher works exactly as claimed —
parent `S-1-16-12288` → child `S-1-16-8192` on the runner, confirmed from inside the leg. On the working
configuration (hosted `windows-2022`, native runtime 131.0.2903.86, driver pinned to match, High integrity) it
is what BREAKS the session: both High driver-alone cells returned `status 200`, both Medium cells failed. The
arm then ran 11 passing / 1 failing / 2 skipped with the launcher removed.

**Integrity's SIGN is not invariant.** Medium helped at runtime 153 on the dev host; High is required at 131 on
windows-2022. The predecessor's legs A/B/C were correctly measured and are BOUNDED by this, not retired —
writing "the earlier finding was false" would discard a sound measurement to make a tidier story.

**Every windows-2022 probe was the instrument until proven otherwise.** Probe A: the repo's own `≥152` floor
read the native 131, fetched Evergreen and installed 153 — measuring a configuration the gate had altered while
appearing to describe the native one. Probe B: the pin step never ran (`if: ${{ env.ImageOS }}`, always false).
Both would have read as "the coherent pair fails". Only checking WHAT ACTUALLY RAN separated them — and probe
A's accident became the strongest evidence in the record, a WITHIN-image variation where the failure followed
the runtime onto windows-2022.

**Verify the instrument, not just the result.** A driver-log readback (`httpAttempts`/`pipeMentions`) changed a
cell: pipe-at-Medium showed 49 HTTP attempts, so the transport switch was NOT honoured and that run re-measured
the port path. Recorded UNMEASURED for that cell rather than "pipe fails at Medium".

**A spec's named retirement mechanism can simply not exist.** security-plan's exit condition for the Evergreen
float — "the versioned Standalone Installer replaces the bootstrapper" — names a mechanism Microsoft does not
provide: the Standalone Installer is Evergreen and takes no version; only Fixed Version is versioned (>250 MB,
`WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`, latest/second-latest majors only, so 131 is unobtainable). Verify a
predicted end-state the same way as an asserted current one.

**Holding a terminal open was vindicated twice.** A permanent exclusion looked justified after the
driver-alone isolation, and again after the transport comparison. Both times the next measurement would have
made it false. The acceptance required the basis to be the MEASURED cause, and the measured cause kept moving.

**When rulings do not arrive, the sanctioned fallback is to remove the code, not to ship it commented.** Four
`ci.yml` surfaces were probe-scoped and unratified (the `windows-2022` label, the floor bypass, a second egress
host, the launcher's removal). None rode the commit on its own authority; all four came out, which resolved all
four escalations without a ruling and left the shipped arrangement unchanged.

## 2026-09-18 — the baseline for a host-tool or advisory reading is the NEWEST recorded one, found bare

When a chunk report states a DELTA for a host-tool or advisory-database reading — `cargo audit`'s advisory
count, a driver version, a bundler version — the baseline is the **most recent** reading recorded anywhere in
the chunk corpus, located by a bare sweep. It is not the figure a previous report happened to compare against,
and not the first older reading that comes to hand.

Measured twice in one session, in the same direction. This chunk's implement report compared today's 1247
against **1243** (the 2026-09-10 reading), reaching past two newer ones, and framed the delta as "wrong by
four". A wrap directive corrected the baseline to **1246** (`chunks/2026-09-16-scenario-assertion-audit-gate/report.md:229`)
— which itself reached past **1247**, recorded a day earlier at
`chunks/2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration/report.md:292` with the identical
triple (1247 advisories · 562 packages · 7 allowed). Against the correct baseline the count had moved by
**zero**. Both intermediate figures were artifacts of where the search started, not measurements — and the
directive's own stated principle ("the four is an artifact of reaching past the newer reading") applied one
step further than the directive itself reached.

The sweep that settles it: `grep -rnoE '1[0-9]{3} advisories' conductor-*/chunks/*/report.md`, run **bare**.
Every reading comes back in path order; the newest chunk's is the baseline. The same shape applies to any
figure the corpus records per chunk rather than per commit — a clipped or filtered view of that sweep is what
produces a plausible wrong baseline, and the report template's "a stated number carries its basis" rule is
satisfied by naming this command, not by naming the older report you compared against.
