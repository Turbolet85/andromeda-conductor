# Fan-out results — 2026-10-07-a-sixth-pre-registered-real-model-series-for-v3-09

Seven Explore doc-agents, one parallel batch, each sent the amendment-flow prompt verbatim with its doc, the report,
its keyed-contract render where the doc is migrated, and its detectors (28 detector slots over the seven prompts =
the 28 `doc:` names over the drift-base's 22 entries, asserted before the batch). Each return was entity-probed
(`&lt;` / `&gt;` / `&amp;` count 0 in every return) and parsed as YAML; YAML comments under a `proposals: []` are
part of the YAML return, so no return was changed by stripping and no raw twin is kept.

## design-system — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no new UI element (the report's Coverage bullet reads
none, tokens n/a); every old value of the report's Counts / qualifiers bullet grepped in the doc at 0 hits; neither
disproved claim retires a platform, runner or driver verdict.

## layout-templates — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no new surface; every old value of the Counts /
qualifiers bullet at 0 hits in the doc (0 occurrences of `v3-09`, of the pin, of "series"); no platform verdict
retired; the doc's four `real-model` mentions describe the existing selector and probe, unchanged this chunk.

## a11y-plan — proposals: 0
Verdict: `proposals: []`. The return's comments, in substance: no interactive element added; neither the a11y
violation schema nor the obs log schema moved (the drives' eleven-key envelopes agree with what the doc and its key
file `§3 → Structured violation JSON schema` hold); no platform verdict retired; two `canary` mentions in the doc
and its key files concern the preflight / dedupe canary and state no `canary:` line count.

## obs-plan — proposals: 2
Verdict: 2 proposals, both under `D-obs-instrumentation` (warning); the return says plainly that neither is a strict
violation of a listed invariant and that the §4 detector is the nearest home. Its comments, in substance: no hit on
`D-obs-stack`, `D-obs-redaction`, `D-obs-ci-gates`, `D-platform-claim`.

1. section: §4 Span / Trace Coverage → the headless-run scenario → Real-model posture · change: retire the general
   sentence "the canary fires three storms per drive and the capture prints two `canary:` lines, the third storm
   landing at the scenario's emission instant so that its digest's tick falls after it"; the capture prints two or
   three lines, the third a `pipeline-fault` token produced by the capture's strictly-before-the-emission-instant
   pairing window (d1, d2 of the sixth series), never by Pulse · coordinates cited: `obs-plan.md:221`,
   `real_model_live.rs:720-730`, `real_model_common/mod.rs:73-151` (all three stated in the report's disproved-claims
   bullet) · its own sweep: one stating site.
   → **apply** — check 6 (the report's disproved claim 1, matched by this proposal). Playbook: NO MATCH — rule
   playbook.md:28 (a spec's wording reconciled to the shipped implementation) is superseded and its clause "the
   invariant still holds … only the form or mechanism differs" fails here (the sentence is false as a general
   statement, not merely worded otherwise); rule playbook.md:149 (a master's own provisional claim retired by its
   named precondition) fails its directness clause (the sentence named no precondition). The change is neither
   structural nor surprising — one measured sentence corrected by the chunk's own measurement, with the evidence in
   the report — so it is applied under check 6's routine raise, and no rule is proposed on n=1. The applied text is
   re-derived from the report, not pasted.
2. section: the same paragraph · change: append the sixth series' dated live observation (Pulse `9bfefb8`;
   `model_identity` `gemma-4-E4B-it-Q4_K_M`, `prompt_version` `v2.6`; three envelopes `ManualCheck`, `verdict` null,
   eleven keys; no span name, span attribute, log line or allowlist entry added) · coordinates cited:
   `obs-plan.md:221` (report), `series_2026_10_07_sixth.rs:99-116` (the New text section's row).
   → **apply** — check 5 (the plan's expected amendment for obs-plan) and playbook.md:316 (Real-model series dated
   record, routine; every clause holds: the facts are in the report, the extension is add-only, nothing new crosses
   the capture boundary).

## test-plan — proposals: 2
Verdict: 2 proposals, both under `D-tests-derived-count` (warning).

1. section: §6 → Scenario: Fingerprint-storm → Real-model interpretation leg · change: add the sixth series' dated
   verdict record after the capture run's record and before the closing sentence (three drives, Pulse `9bfefb8`, all
   `Identified`, `v3-09` MET on three drives and no wider, the verdict test and the ref test named, the pass not
   attributed to the Pulse-side change) · coordinates cited: `test-plan.md:260` (report),
   `series_2026_10_07_sixth.rs:118-141` (the New text section's row) · its own sweep: one site.
   → **apply** — check 5 and playbook.md:316 (test-plan's dated verdict in the real-model leg bullet; add-only).
2. section: §9 CI Integration → Live-Pulse scenarios · change: retire "real-model formation AND pickup both still
   UNMEASURED" at `test-plan.md:391` · its rationale cites the wrap run dir's `word-1.md`, a file the report does
   not carry and the prompt did not hand it.
   → **REJECTED at Validate's opening rule** (the proposal rests on a location outside the report and the doc). It
   cited no line coordinate of that kind — the tell is the file. Its fact is real and is in the report (disproved
   claim 2), so it is **raised by the orchestrator under check 6** and applied on the operator's word given at
   this wrap's first-sweep stop ("The minimal amendment, both clauses, as you recommend; if architecture.md:184
   cannot stay byte-neutral, CARRY that one instead." — the operator, 2026-10-08); the applied text is the
   orchestrator's minimal wording, not the proposal's.

## security-plan — proposals: 4
Verdict: 4 proposals under `D-security-input` (the detector's declared severity, escalate): one primary and three
`dependent-of` it. The return's comments, in substance: no validation invariant is violated (the three captures
entered through the existing scrub chain, digest pins and harvest); what lags is the doc's dated per-series record
at the two sites the report names; no hit on `D-security-subprocess`, `D-security-deps`, `D-platform-claim`.

1. (primary) §Input Validation → the real-model capture ingest row · change: the measured set gains the 2026-10-07
   sixth series — leaf `rm-sixth-series` 0 occurrences across three captures, rendering `verbatim` on all three,
   each `## Previously Seen` suffix `<redacted>` (d2 one entry, d3 two, d1 none) · coordinates cited:
   `security-plan.md:121` (report), `series_2026_10_07_sixth.rs:46-49` and `contracts/…posture.md:761-875` (the New
   text section).
2. (dependent) the same row's launch-fallback enumeration gains the sixth series' launch.
3. (dependent) the same row's `workspace_key` derivation parenthetical gains `9bfefb8` (keeping `f70be92`) ·
   coordinate cited: `contracts/…posture.md:792-794` (the New text section's row).
4. (dependent) §Security Anti-Patterns → Data Protection: the per-series inventory gains the sixth series' d1, d2 and
   d3 captures, one report body each; the capture-run-only sentence after it is not extended ·
   coordinate cited: `security-plan.md:338` (report).
   → **apply all four as one group** — check 5 and playbook.md:316 (routine: security-plan's capture-ingest row,
   the workspace-key derivation's pin provenance and the exception's per-series inventory are named by the rule; its
   clauses hold — each fact is in the report, the extension is add-only beside the earlier records, the captures are
   of the already-ratified class through the unchanged scrub chain with no new handle reader, harness form or scrub
   shape). `Boundary widening` (playbook.md:124) was checked first and does not match: nothing new crosses the
   boundary. The detector's `escalate` severity is the detector's default; the rule governs the class. One fact
   re-read by the orchestrator beside the report: `git -C ../andromeda-pulse diff --stat f70be92 9bfefb8 --
   crates/workspace-detector` prints nothing.

## architecture — proposals: 3
Verdict: 3 proposals — 1 under `D-arch-decisions`, 2 under `D-arch-resources` (all warning). The return's comments,
in substance: `D-arch-collision` clean; `D-platform-claim` no hit; `D-arch-registry-size` is the orchestrator's
post-apply run; `v3-09` and `f70be92` each occur once in the doc (`:70`, `:184`) and 0 times in its four key files,
so no dependent proposal; it did not propose the build-system contract's dated `cargo audit` figures (a 2026-09-05
record; the report lists the 1294 / 7 reading as no moved count). It restates the registry sizes the report carries.

1. `D-arch-decisions` · §Established Decisions → [Read-Back Dependency Posture] · change: at `architecture.md:70`
   the sentence head "So the interpretation claim stays unverified (`v3-09` not met) and …" now states `v3-09` met
   on three drives by the sixth series, no more (+32 B as the proposal words it, inside the 47 B headroom) ·
   coordinates cited: `architecture.md:70` (report), `series_2026_10_07_sixth.rs:118-141` (the New text section).
   → **apply** — check 5 (the plan's expected amendment, MET arm). Playbook: NO MATCH — playbook.md:316 names this
   edit as NOT covered ("a MET series' edit to architecture's one `v3-09` verdict statement … takes its own
   validation"), and playbook.md:97 (a chunk REVERSES a locked decision) does not match: the decision stands and
   only the recorded standing of the capability it names moves. The operator's recorded direction settles it: the
   P5-approved plan entry names the change itself ("MET: it states the capability met on three drives" — approved
   by the operator's P5 review word, inputs#I16), and the operator verified the verdict himself before the operator
   pass (inputs#I20). Applied without a halt; the orchestrator's wording is shorter than the proposal's (+14 B) and
   names the series as the contract does. No rule is proposed: the class cannot recur in this version (the section's
   stop rule makes this the last series) — stated at the wrap's report for the operator to overrule.
2. `D-arch-resources` · §Occupied Resources → On-disk artifacts (the posture-contract entry) · change: at
   `architecture.md:184` the latest per-series pin reads `9bfefb8` in place of `f70be92`, a same-length swap ·
   coordinates cited: `architecture.md:184` (report), `contracts/…posture.md:761-875` (the New text section).
   → **apply** — check 5 and playbook.md:316 (the latest pin alone is replaced, as that entry's own form says).
3. `D-arch-resources` · the same entry · change: retire "no real-model formation or pickup figure exists" (53 B for
   53 B as the proposal words it) · coordinate cited: `architecture.md:184` (report); basis the chunk's ledger, no
   line.
   → **apply** — check 6 (the report's disproved claim 2), on the operator's word at this wrap's first-sweep stop
   ("The minimal amendment, both clauses, as you recommend; if architecture.md:184 cannot stay byte-neutral, CARRY
   that one instead." — the operator, 2026-10-08). The wording applied is the one the operator approved, "figure" →
   "budget" (53 B → 53 B, byte-neutral), not the proposal's.

## Totals
11 proposals over 7 docs (architecture 3 · security-plan 4 · test-plan 2 · obs-plan 2 · design-system 0 ·
layout-templates 0 · a11y-plan 0). Rejected at Validate's opening rule: 1 (test-plan 2 — it cited the run dir's
`word-1.md`, a file outside the report and the doc; it cited NO coordinate that neither the report's bullets nor
the New text section holds — every line number in all 11 proposals is one the report states or that section
lists). Applied: 10 proposals + 1 orchestrator raise (the rejected proposal's fact, check 6). Escalations: 0 open —
two no-match amendments were settled by the operator's recorded direction (the `v3-09` verdict statement; the
unmeasured-pickup clauses), neither a boundary widening. Checks 2 and 3: no opposing pair; the report's deviations
are justified and its scope record is empty. Check 5: all five expected amendments are matched (architecture ×2,
security-plan, test-plan, obs-plan). Check 6: disproved claim 1 → obs-plan 1; claim 2 → test-plan (orchestrator
raise) and architecture 3.
