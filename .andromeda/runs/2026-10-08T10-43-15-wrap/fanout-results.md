# Fan-out results — 2026-10-08-version-close-on-measured-evidence

Seven doc-agents, one batch, 28 detector slots (21 single-doc detectors + `D-platform-claim` on each of the seven).
Entity probe over every return after stripping: 0 HTML entities. Proposals: 10 — applied 5 · rejected 5 (all five by
playbook rule, none for a source or a coordinate the report does not carry: rejected-for-source 0 ·
rejected-for-coordinate 0). Rationales below are condensed by the orchestrator; `change`, `sidecar` and `basis` are the
returns' own.

## architecture — 2 proposals

Stripped: a trailing comment block (no hit on D-arch-decisions, D-arch-collision, D-platform-claim; D-arch-registry-size
is the orchestrator's after Apply; and one note outside its detectors — §Standard Contracts, Run report envelope, states
`verdict ∈ {Pass, Fail, CalibrationRegion}` with no null arm, a site the report's coordinates did not list).

1. `D-arch-resources` · warning · §Occupied Resources — On-disk artifacts / database, the
   `contracts/pulse-real-model-leg-posture.md` row
   - change: In the posture row only, in this order. (1) Free first: replace "Carries `sut_version` · `captured_at` ·
     `pinned_at` · `provenance` on its own face." with "Carries the same four face fields." (2) Then correct: replace
     "It is the **SECOND** `contracts/` member with **NO Rust reader**, taking the P-025 regime above unchanged." with
     "It is the **SECOND** `contracts/` member with **NO Rust reader** but a test-tier digest hold (its `## Regime`),
     taking the P-025 regime above unchanged." The ordinal "SECOND", the regime clause and the whole P-025 row above
     stay as written.
   - basis: `.andromeda/architecture.md:184`; `contracts/pulse-real-model-leg-posture.md:51-58`
   - rationale (condensed): report, Spec claims disproved item 2 and Expected amendments; not a collision (a test-tier
     reader, not an owner or writer); predicted 38114 → 38108 B.
   - **Disposition: apply.** Check 1: no rule matches the correction of an existing row's reader claim (rule `:229`
     governs a NEW reader-less member; its ordinal clause is honoured — "SECOND" and "FIRST" stay). The operator's
     recorded direction settles it: the plan's P5-approved expected-amendment entry names this change itself, and the
     operator's wrap word repeats it ("write the masters the relay section 2 lists on the founder ruling, both
     ordinals kept" — the operator, 2026-10-08). Check 5: matches the plan's first entry. Check 6: disposes item 2.
     Proved before any write by the registry instrument over drafts in `arch-drafts/`: `arch-registry-check: PASS`,
     AFTER §Occupied Resources 38108 B, AFTER §Established Decisions 38082 B, 2 judgment rows (both the rewrites above,
     read: the row directly above lists the same four face fields in full; the corrected sentence is the directed
     one), `selftest: every arm detected`. A true sentence fits; the stop did not apply.
2. `D-arch-resources` · warning · §Infrastructure Patterns → Directory structure (crate-per-seam Cargo workspace) ·
   dependent-of D-arch-resources
   - change: On the `contracts/` line of the tree, the closing parenthetical reads "(the two members no shipped code
     reads)" in place of "(the two members no Rust code reads)". Nothing else on the line moves.
   - basis: `.andromeda/registries/contracts/architecture/directory-structure-crate-per-seam-cargo-workspace.md:30`
   - **Disposition: apply**, with its primary (rule `:229`'s dependent-edit clause; the plan's second entry).

## security-plan — 1 proposal

Stripped: a leading comment block (no hit on D-security-subprocess, D-security-deps, D-platform-claim; the two "no Rust
code reads either" sites are the host-dev-tool handles, a different subject).

1. `D-security-input` · escalate · §Input Validation — the boundary table's row `Committed SUT-facing manifests read at
   a fixed path` (a dated note inside that row)
   - change: Add one dated note (2026-10-08) to the row, leaving its four-manifest list and both existing ordinals as
     written: `contracts/pulse-real-model-leg-posture.md` is a committed `contracts/` member with NO shipped reader and
     one TEST-tier reader — since the 2026-09-30 series the `conductor-run` real-model harvest helper `pre_registered`
     opens it and holds each dated section by a sha256 pin, resolving the root from `CARGO_MANIFEST_DIR` with no
     `CONDUCTOR_*` handle, read faults by error kind, parsing nothing from it; that digest hold is not the runtime read
     the contract's own notice names, so this row's committed-manifest duties do NOT attach to it (the founder's ruling,
     2026-10-08, verbatim: «Нет, исправить фразу (Recommended)»), and the notice stands as written for a real runtime
     read; the entry widens nothing — no new crossing, input class or write.
   - basis: `.andromeda/security-plan.md:113`; `contracts/pulse-real-model-leg-posture.md:51-58`;
     `crates/conductor-run/tests/real_model_grading/mod.rs` (no line: the report gives none)
   - **Disposition: escalated-and-resolved → apply.** Check 1: the subject is what a boundary clause covers, the
     never-routine class (rule `:124`), so it is not applied on a routine verdict. The escalation was taken before
     this wrap and is resolved by ratification: the question went to the founder, who ruled by dialog on 2026-10-08
     that a test-tier digest read does not attach the duties (his pick relayed verbatim by the pc overseer,
     `inputs#I4` §1). The note records a reader and widens nothing. Check 5: the plan's fourth entry. No new halt.

## design-system — 0 proposals

`proposals: []`. Stripped: a comment block giving the basis per detector (no new UI element; no count moved; no
sentence states the retired `pwsh` verdict; the doc's two platform lines name platforms and state no verdict). Raw
twin: `.raw-fanout-design-system.md`.

## layout-templates — 0 proposals

`proposals: []`. Stripped: a comment block giving the basis per detector (no new surface; no count moved; no sentence
states the retired verdict). Raw twin: `.raw-fanout-layout-templates.md`.

## test-plan — 4 proposals

Stripped: a leading comment block (no hit on D-tests-coverage, D-tests-framework, D-tests-derived-count; proposals 2-4
rest on the operator's conditional call, not on a literal detector trigger).

1. `D-platform-claim` · warning · §3 → 5-command implementation (label `run`, Command body)
   - change: Replace "its red path is read from the script, not measured (the Linux dev host has no `pwsh`)" with: "its
     red path is read from the script, not measured — no leg has driven it. (`pwsh` resolves on the Linux dev host
     since 2026-10-06, as measured 2026-10-08; the script was parsed there and not executed, so a missing shell is no
     longer the reason.)" The green-path sentence before it stays as written.
   - basis: `.andromeda/registries/contracts/test-plan/5-command-implementation.md:11`
   - **Disposition: apply.** Check 1: the site STATES the retired verdict (quoted), so rule `:46`'s dismissal does not
     cover it; the operator's recorded direction names the change ("The test-plan `run` contract's stale reason — …
     the PowerShell red path is still unmeasured, and no one drove it at this close. Say both." — the operator,
     2026-10-08, the wrap relay §2). Check 6: disposes item 1. The applied text is re-derived from the report's fact
     and carries the host and the evidence pointer.
2. `D-tests-obs-harness` · warning · §3 → Status endpoint shape
   - change: The envelope sample's verdict line gains the null arm: `"verdict": "Pass | Fail | CalibrationRegion |
     null"` — null on a row that carries no graded verdict. The eleven-field count and every other line of the sample
     are unchanged.
   - basis: `.andromeda/registries/contracts/test-plan/status-endpoint-shape.md:6`
   - **Disposition: reject — playbook `:46` and `:300`, routine dismiss; routed to an open residual line (P5).** The
     detector says itself that its trigger is not met: the report shows the chunk changed neither bound section, and
     test-plan §3 and obs-plan §3 agree today. Rule `:46` (confirmed with the user 2026-06-21) dismisses a
     plan↔plan-bind detector that fires on a site the chunk did not introduce; rule `:300` adds that an accurate,
     real, pre-existing drift spread over artifacts the chunk did not touch is never amended piecemeal but routed
     whole to its owned channel. The operator's wrap relay §3 offers the same two arms (apply where a detector
     proposes it, else one open residual line); a rule matches, so the direction does not override it (check 1), and
     the residual arm is taken. The family, as measured at this wrap, is wider than the report listed: see the note
     under a11y-plan.
3. `D-tests-obs-harness` · warning · §1 Test Scope Summary → `status` · dependent-of D-tests-obs-harness
   - change: `verdict ∈ {Pass, Fail, CalibrationRegion}` becomes `verdict ∈ {Pass, Fail, CalibrationRegion} or null (a
     row with no graded verdict)`.
   - basis: `.andromeda/test-plan.md:65`
   - **Disposition: reject with its primary** (a dependent group validates atomically).
4. `D-tests-obs-harness` · escalate · §1 Test Scope Summary → Status endpoint shape · dependent-of D-tests-obs-harness
   - change: In the inline envelope type, `verdict: "Pass"|"Fail"|"CalibrationRegion"` becomes `verdict:
     "Pass"|"Fail"|"CalibrationRegion"|null`.
   - basis: none cited (a third spelling, a site the report did not list)
   - **Disposition: reject with its primary.** Its escalation question (apply with the others, or name the site in
     the residual line) is answered by the primary's dismissal: the site is named in the residual line.

## obs-plan — 1 proposal

Stripped: a trailing comment block (no hit on D-obs-instrumentation, D-obs-stack, D-obs-redaction, D-obs-ci-gates; no
obs detector covers item 3, and the four obs sites the report lists were confirmed still without a null arm).

1. `D-platform-claim` · warning · §10 SLO Invariants & Telemetry Budgets — Build / deploy failure conditions, the
   `cargo clippy … -- -D warnings` line
   - change: Replace the parenthesis `(its red path unmeasured — the Linux dev host has no `pwsh`)` with one that keeps
     the true half and drops the false reason: `(its red path unmeasured — nobody has driven it: `pwsh` 7.6.6 resolves
     on the Linux dev host as measured 2026-10-08, installed 2026-10-06, where the script was parsed with 0 errors and
     never executed; its only executions are the CI runs)`. The rest of the line stays as written.
   - basis: `.andromeda/obs-plan.md:490`
   - **Disposition: apply.** The twin of test-plan's proposal 1: the same stated verdict, the same measured fact, the
     same recorded direction (a duplicate of a retired claim never survives a single-site apply). Re-derived text.

## a11y-plan — 2 proposals

Stripped: a leading comment block (no hit on D-a11y-surface, D-platform-claim; both proposals are conditional on the
obs sites taking the same string in the same apply).

1. `D-a11y-obs-schema` · warning · §3 → Structured violation JSON schema
   - change: In the reproduced envelope sample, the verdict line gains its null arm: `"verdict": "Pass | Fail |
     CalibrationRegion or null (null for declare-only rows)"` — the identical string the obs §6 sample takes at this
     wrap; if obs lands a different qualifier, this site takes obs's.
   - basis: `.andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:13`;
     `crates/conductor-core/src/run_record.rs:55`
   - **Disposition: reject — playbook `:46` and `:300`, as test-plan's proposal 2.** The detector states that its
     invariant is not violated at the base (the a11y and obs samples are identical today) and that an a11y-only apply
     would create the divergence it guards.
2. `D-a11y-obs-schema` · warning · §1 A11y Scope Summary → Structured violation JSON schema · dependent-of
   D-a11y-obs-schema
   - change: The same verdict line in the body's copy of the sample gains the same null arm.
   - basis: `.andromeda/a11y-plan.md:103`
   - **Disposition: reject with its primary.**

**The `verdict` null-arm family, measured at this wrap** (all seven masters, every key file and the leaves; patterns
`CalibrationRegion"`, `verdict ∈ {…}` and `verdict … Pass|Fail`): 10 sites in masters and key files —
`.andromeda/obs-plan.md:72`, `:93`, `:347`; `.andromeda/registries/contracts/obs-plan/log-format-json-schema.md:12`;
`.andromeda/a11y-plan.md:103`; `.andromeda/registries/contracts/a11y-plan/structured-violation-json-schema.md:13`;
`.andromeda/registries/contracts/test-plan/status-endpoint-shape.md:6`; `.andromeda/test-plan.md:65` and `:68`;
`.andromeda/architecture.md:136` — and one leaf, `.claude/rules/verification-harness.md:27`. The report listed nine of
these; `test-plan.md:68` (a third spelling, quoted alternatives with no spaces) and `architecture.md:136` came from
the detectors' sweeps. The code's type: `Option<Verdict>`, `None` for a blocked row and for a row that grades no check
(`crates/conductor-core/src/run_record.rs:55`; `crates/conductor-run/src/execute.rs`, the declare-only arm).

## Validate — checks 2 to 6

- Check 2 (cross-contradiction): none. No two surviving proposals edit one section.
- Check 3 (intent-consistency): the report's three deviations are justified and operator-accepted; the scope record is
  empty (`gate.py scope` clean). The two `pwsh` amendments go beyond the plan's list on a fact measured after the plan
  was written; the operator directed them.
- Check 4 (absence needs evidence): each sweep in the report names its pattern, its hit count and a disposition per
  hit. The item-3 site list was a list, not a caught-all claim; it under-ran by two sites, recorded above and in the
  report.
- Check 5 (expected amendments): entry 1 → architecture proposal 1 · entry 2 → architecture proposal 2 · entry 3
  (`matrix#v3-09 notes`) → P7.3 · entry 4 → security proposal 1 · entry 5 ("no amendment expected in test-plan,
  obs-plan …") → superseded for two sites by item 1, both proposed. The citation sweep left no `claim false` row.
- Check 6 (disproved claims): item 1 → test-plan proposal 1 and obs-plan proposal 1, applied · item 2 → architecture
  proposals 1 and 2 applied, the leaves by the cascade, the playbook's dated fact by curation's channel · item 3 →
  routed to an open residual line at P5.
- Escalations open: 0.
