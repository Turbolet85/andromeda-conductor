# Fan-out results — 2026-08-09-sut-load-envelope

7 Explore doc-agents, one parallel batch. Proposals embedded verbatim below, so this file is the
complete audit record (no separate `.raw-fanout-*` twins needed — no return required stripping).

| doc | verdict | proposals | disposition |
|---|---|---|---|
| arch | drift | 2 (D-arch-resources, D-arch-decisions) | both **routine → applied** |
| security-plan | drift | 1 (D-security-input, **escalate**) | **escalated → resolved with user → applied** (widened) |
| design-system | clean | 0 | **self-raised 1** under validation check 5 → applied |
| layout-templates | drift | 2 (D-layout-surface, D-layout-derived-count) | both **routine → applied** |
| test-plan | drift | 1 (D-tests-derived-count) | **routine → applied** |
| obs-plan | drift | 1 (D-obs-instrumentation) | **dismissed** (playbook: not-introduced) |
| a11y-plan | clean | 0 | — |

**Totals:** 7 proposed · 1 self-raised · **7 applied** · 1 dismissed · 1 escalated-and-resolved · **0 open**.

---

## Validation (the 5 checks)

1. **Playbook check.**
   - `D-arch-resources` → not the library-symbol over-reach rule (2026-06-19/21): a committed **on-disk
     artifact** is exactly what §Occupied Resources tracks, and the 2026-06-24 precedent explicitly
     registered `logs/conductor-tauri.jsonl`. Routine.
   - `D-arch-decisions` → 2026-06-15 spec-wording → sound-impl rule: the no-ORM decision is preserved,
     only its count qualifier moved. Routine.
   - `D-security-input` → **no rule matched.** The 2026-06-23 consuming-shipped-infra dismiss rule does
     NOT apply (its precondition is "the chunk added NO new external-input boundary"; this chunk added
     one). Escalated per severity.
   - `D-layout-surface` / `D-layout-derived-count` / `D-tests-derived-count` → the detectors' own `check`
     fields prescribe the fix (name the set; document the new surface). Routine.
   - `D-obs-instrumentation` → 2026-06-21 not-introduced rule + the 2026-06-20/21 deferred-span family.
     **Dismissed** — see below.

2. **Cross-contradiction.** None. The two arch proposals touch different sections and agree on the
   `runs.db` two-table fact.

3. **Intent-consistency.** One divergence: the webview banner named in `plan.md` step 7 was not
   implemented (report Deviation 3), with justification (v2-07's acceptance names the *report*; the
   webview would have widened scope into `conductor-tauri/src/commands.rs` and shipped a UI change whose
   a11y gate is display-gated to Linux+xvfb and unverifiable on this host). **Justified ⇒ intent was
   incomplete** → carried as an owned route CARRY in P5, not left as report prose.

4. **Absence needs evidence.** The design-system agent's "no derived-count detector is scoped to
   design-system" and the obs dismissal's "the report seam has no spans" were both established by grep,
   not inference (`grep -rn 'tracing::' crates/conductor-report/src/` → zero hits).

5. **Expected-amendments reconciliation** (the plan's list is the coverage floor):
   - arch §Occupied Resources → **proposed** by D-arch-resources ✓
   - test-plan §3 run-level table → **not owed**: the plan's entry was conditional ("if the status read
     touches it") and it does not — `runs` is byte-unchanged and the harness `status` read is untouched.
   - layout-templates §Component + §cli Primary screens → **proposed** ✓
   - design-system §Surface: cli reused-token entry → **NOT proposed** by any detector, and the report
     substantiates it (§Color Palette states the tier has "two NON-lamp uses"; this chunk makes it three).
     **Self-raised as routine** per check 5.
   - a11y-plan §6 crosswalk → **not owed**: its condition was a webview element, which Deviation 3 did
     not land; a11y-plan's six-lamp assertion and the eleven-field envelope it reproduces both stay true.

---

## Escalation (resolved with the user)

**`D-security-input` — register `contracts/pulse-load-envelope.toml` in §Input Validation.**
Resolution: **apply + append a playbook rule** (user-approved). While applying, a grep of the body found
the *capability manifest* row equally absent — although `security-plan-amendments.md` records it as
landed on 2026-08-08. The applied row therefore covers **both** committed SUT-facing manifests rather
than documenting one and leaving its sibling silent. The new playbook rule carries that trap in its
`note`: grep the body for siblings rather than trusting the sidecar.

**Detector growth (user-approved):** `D-design-derived-count` appended — the derived-count class recurred
on a third doc, and design-system had no detector scoped to it.

---

## Dismissal (recorded, not silent)

**`D-obs-instrumentation`** proposed flagging `RunsDb::insert_envelope` / `classify_run` as an
uninstrumented rusqlite write on obs §4 Critical Path 1. Dismissed: `crates/conductor-report/src/` contains
**zero** `tracing::` calls, so `db.insert_run` and `report.generate` carry no spans either — the new write
is consistent with the seam, not an asymmetry this chunk introduced (playbook 2026-06-21 not-introduced).

**But the dismissal surfaced a standing gap worth tracking:** the playbook's 2026-06-20/21 deferred-span
entries record those spans as landing "with the Epoch-8 cli/timeline caller". **Epoch 8 is complete and
they never landed.** That is a route-sequenced deferral whose promised chunk has passed — recorded in the
handoff so the next wrap does not re-dismiss it reflexively.

---

## Proposals (verbatim)

### arch
```yaml
- detector: D-arch-resources
  severity: warning
  section: Occupied Resources — On-disk artifacts / database
  change: Add a bullet for `contracts/pulse-load-envelope.toml` … and extend the `runs.db` bullet to two
    tables (`runs` + additive `run_envelope`); correct the §Infrastructure tree `contracts/` comment from
    two manifests to three.
  rationale: Report §Changes/Files + §Schema/config land both; §Counts records "contracts/ artifacts 2 → 3"
    and "runs.db tables 1 → 2" naming arch as the enumerating site.
- detector: D-arch-decisions
  severity: warning
  section: Established Decisions — [ORM] (+ §Stack ORM row)
  change: Keep "None — raw SQL" but restate "~one indexed table" as a small fixed set of hand-written
    tables added additively under CREATE TABLE IF NOT EXISTS.
  rationale: §Counts states runs.db tables 1 → 2; no new library or runtime (Dependencies: none added).
```

### security-plan
```yaml
- detector: D-security-input
  severity: escalate
  section: Input Validation
  change: Add a boundary row for contracts/pulse-load-envelope.toml (LoadEnvelope/EnvelopeTerms/Exemption):
    non-empty identity+provenance, positive [envelope] terms, no duplicate/reason-less [[exempt]]; fixed
    default_path() → resolve_under, no CONDUCTOR_* override; e.kind()-only read faults; absent = hard fault.
  rationale: The report labels it verbatim a "new external-input surface" while §Input Validation's
    boundary table does not enumerate it.
```
*(D-security-subprocess and D-security-deps returned no drift — the sidecar spawn / preflight / data-dir
are untouched, and Dependencies reads "none added, none bumped" with un-drifted lockfiles.)*

### design-system
```yaml
proposals: []
```
*(D-design-tokens clean: both new UI elements flagged `tokens ✓` — ANSI 246 reused with zero new palette
entries; the Markdown banner has no color channel and carries label + emphasis. The agent explicitly noted
the derived-count staleness as outside its invariant — which is what the self-raise and the new detector
now cover.)*

### layout-templates
```yaml
- detector: D-layout-surface
  severity: warning
  section: "Surface: cli → Output structure — `conductor run <scenario>`"
  change: Add the run-level [ENVIRONMENT-SUSPECT] caption to the wireframe — printed once above the
    per-P-ID verdict lines, outside the lamp column, ANSI 246, ASCII label survives NO_COLOR, omitted
    when in-envelope or exempt.
  rationale: Report §Coverage lists the cli caption as a landed new UI element; no cli wireframe shows it.
- detector: D-layout-derived-count
  severity: warning
  section: "Surface: cli → Component — verdict / report-state lines"
  change: Name the six as the closed per-P-ID set and record [ENVIRONMENT-SUSPECT] as a run-level non-lamp
    label outside it; lamp set unchanged.
  rationale: §Counts "Bracket labels in cli stdout: 6 → 7" names layout-templates as an enumerating site.
```

### test-plan
```yaml
- detector: D-tests-derived-count
  severity: warning
  section: "§1 Surfaces under test (cli Signal); §6 cli driver row + Selector strategy"
  change: Present the six labels as the lamp/report-state set and note that cli stdout also carries the
    non-lamp run-level [ENVIRONMENT-SUSPECT] qualifier, so selectors assert the set plus that qualifier.
  rationale: §Counts names test-plan §1 and the §6 driver table as sites enumerating the six as exhaustive.
```
*(D-tests-coverage / D-tests-framework / D-tests-obs-harness all clean — every landed path is tested at its
mandated tier, every command is on-spec, and neither the harness, the status read, nor the log format moved.)*

### obs-plan
```yaml
- detector: D-obs-instrumentation
  severity: warning
  section: "6. Log Coverage — Boundary-call wrappers"
  change: Name the envelope-artifact load (info-level, landed) and flag the run_envelope insert /
    classify_run() as an uninstrumented rusqlite write on Critical Path 1.
  rationale: The report marks the table writes instrumentation n/a while the sibling artifact load is ✓.
```
**→ dismissed** (see above).

### a11y-plan
```yaml
proposals: []
```
*(Both detectors clean: the two landed UI elements are on non-assertable surfaces — cli per §11 Universal,
Markdown by nature — and the one element that would have touched the "View run report" must-be-accessible
path is explicitly NOT LANDED. The eleven-field envelope a11y §3 reproduces is unchanged.)*

---

## Cascade (single pass, fixed DAG)

**Step 2 — lateral binds + cross-master citations.**
- `test-plan §3 ↔ obs-plan §3` — intact (§3 untouched; §1/§6 amended).
- `a11y-plan schema ↔ obs-plan schema` — intact (no schema change).
- Cross-master grep of the amended wording: `"one indexed table"` / `"one-table"` → no other master carries
  it. **Intra-doc hit:** design-system's own §Surface: cli enumerations (status-prefix list + verdict/
  report-state lines) still read the six labels as exhaustive — fixed in the same pass, so the four docs
  now agree.

**Step 3 — leaf re-derivation** (`USER:*` / `## Session Additions` preserved):
| Changed source | Leaf | Edit |
|---|---|---|
| architecture.md | `CLAUDE.md` §Key directories | `contracts/` now names all three manifests |
| architecture.md | `.claude/docs/conventions.md` | same, directory-layout line |
| architecture.md | `.claude/docs/stack.md` | **no edit** — its rusqlite line never claimed a table count |
| security-plan.md | `.claude/rules/security.md` | new committed-manifest input-boundary bullet |
| security-plan.md | `.claude/docs/security-summary.md` | **no edit** — its row is category-grain (`contracts/` manifest) and still true |
| design-system.md | `.claude/docs/design-summary.md` | **no edit** — "six visually distinct treatments" is the LAMP set, deliberately unmoved |
| test-plan.md | `.claude/rules/testing.md` | brand-anchor selector list names the set + the qualifier |
| test-plan.md | `.claude/docs/tests-summary.md` | **no edit** — carries no label enumeration |
| layout-templates.md | — | no specialist-summary doc (per playbook 2026-06-23) |
