# Scope — `2026-09-15-structurally-dead-assertion-class-retired`

**Working-route entry (verbatim, `conductor-0.3.0/working-route.md:24`):**

> Structurally-dead assertion class retired — three live CountAtLeast keys and one Hard Contains to
> declare-only, pinning tests paired in the same change

**Requirement:** `v3-04` — *Structurally-dead assertions retired as a class* — "every committed scenario's
declared checks are satisfiable by construction against a live Pulse, or are retired to declare-only in ONE
pass with their pinning tests updated in the same change (per intent §F2.1–2; precedent
`architecture-amendments.md:403`; absorbs residual `2026-09-10-live-pulse-in-lane-scenario-round`)."

**Epoch:** 2 — Scenario assertion hygiene. **Annotations folded:** none — the entry carries no
`PREREQ:` / `CARRY:` / `BLOCKED-ON:` / `CONTEXT:` / any ALL-CAPS introducer at an annotation position
(scanned this pass).

**Tag convention used below:** `[measured P1]` = measured this pass, command cited, closed.
`[inferred]` = carried from a prior record or not yet verified against the artifact — P3's scope premise
closure owns each one.

---

## What this chunk does

Retire the structurally-dead assertion class — declared `[[expected]]` checks that **cannot pass in any
world**, because the value they grade is never produced — from the committed scenario corpus, as a class,
in one pass, with every pinning test that asserts the retired declaration updated in the same change.

Retirement means the established **declare-only** shape: the scenario keeps its drive (phases, emission,
`p_ids`, tier) and carries **zero `[[expected]]` blocks**, with a header comment recording the retirement,
its marker, and the measured ground. Precedent: five severity-lifecycle scenarios already retired on this
exact basis (`architecture-amendments.md:403`, 2026-08-21) and shipped that way.

## The class members

Four assertions across four committed scenarios. The membership derivation is the one the originating
residual prescribes, re-run this pass — `grep -n 'kind = "CountAtLeast"' scenarios/*.toml` returns exactly
**3** live keys, while a bare-token grep returns **7** files because four mention `CountAtLeast` only inside
retirement comments. `[measured P1]`

| # | Scenario | Line | Declaration | P-ID | Ground |
|---|---|---|---|---|---|
| 1 | `scenarios/constellation-severity-live-wiring.toml` | `:52-55` | `CountAtLeast` / `Hard` / `"1"` | P-079 | grades `evidence_count`, summed from an always-empty `span_refs` |
| 2 | `scenarios/findings-counter-refresh.toml` | `:48-51` | `CountAtLeast` / `Hard` / `"3"` | P-045 | same |
| 3 | `scenarios/pulse-run-contract.toml` | `:57-60` | `CountAtLeast` / `Hard` / `"1"` | P-073 | same |
| 4 | `scenarios/cross-incident-recurrence.toml` | `:42-45` | `Contains` / `Hard` / `"Previously seen"` | P-036 | **[premise-corrected at P3]** the emitted token is `"## Previously Seen"` — a case mismatch against a case-sensitive `Contains`, not an absent producer |

All four coordinates and declaration triples read directly from the committed TOMLs this pass. `[measured P1]`

### The mechanism the retirement rests on — members 1–3 `[verified at P3]`

Carried from the residual `2026-09-10-live-pulse-in-lane-scenario-round` and **re-verified against the SUT
at Pulse HEAD `83d4060` this pass** (P3; Pulse has not moved since the last wrap):

- `pulse-app/src/inference_runtime.rs:871` still sets `EvidenceRefs.span_ids: Vec::new()` at every
  production incident. Re-enumerated this pass: every other `span_ids` writer in the Pulse tree sits inside
  a `#[cfg(test)]` module (`interpretation/src/markdown.rs:507` > cfg@376 · `triage/src/digest/assembler.rs:802`
  > cfg@165 · `digest/retrieval.rs:170` > cfg@150 · `incident/persistence.rs:359` > cfg@335 ·
  `incident/registry.rs:400` > cfg@376), so `:871` is the sole non-test writer.
- `pulse-app/src/incidents_router.rs:67` computes `evidence_count = i.evidence_refs.span_ids.len()` → always 0.
- `crates/mcp-server/src/tools.rs:430-435` maps `span_ids` 1:1 into `span_refs`
  (`format!("span:{}", hex_lower(bytes))`) — **measured coordinate; the residual cited `:430-439`**.
- Conductor's side: `conductor-verify/src/extract.rs:55` hands `evidence_count` to `CountAtLeast`, and
  `conductor-verify/src/slo.rs` `evaluate_check` sets `class = CalibrationRegion` when a `CountAtLeast`
  comparison fails, **regardless of the declared `Hard`**.
- So `0 >= 1` / `0 >= 3` is false in every world: the declaration can neither pass nor hard-fail.

### `cross-incident-recurrence`'s ground `[premise-corrected: the token IS emitted and DOES reach the graded text; the check fails on a one-character case mismatch]`

**The scope's original ground — "no producer anywhere emits that token" — is FALSE, and so is the residual's
attribution it was copied from.** Measured at P3:

- `andromeda-pulse crates/interpretation/src/markdown.rs:221` pushes the literal `"## Previously Seen"` into
  the serialized report, guarded on corpus matches existing (its own tests at `:478`/`:488` assert both
  directions). `pulse-app/src/incidents_router.rs:455` is the corpus-candidate selection behind it.
- That markdown **does** reach the graded text: `conductor-verify/src/extract.rs:101-103` pushes each
  incident's `retrieve_report` `markdown` into `observation.text`, alongside `list_text`'s
  status/severity/title from `query_incident_list`.
- The scenario declares `expected = "Previously seen"` (lower-case `s`), and `Contains` is a case-SENSITIVE
  substring test (`conductor-verify/src/slo.rs:51`, `observed.contains(check.expected.as_str())`).

So member 4 is **not** structurally dead in the sense members 1–3 are. Its declared token differs from the
emitted one by one character, which is exactly the "inferred token, never calibrated" risk the scenario's own
header records at `:15`. The live drive's DEAD measurement stands; its recorded CAUSE did not survive
re-derivation. Whether member 4 is therefore retired with the corrected ground, or its case corrected and the
check kept live, is a **fork for the operator at P4** — the route entry says "one Hard Contains to
declare-only", and that wording was authored on the disproved cause.

## The paired-edit surface — the chunk's defining hazard

The prescribed companion sweep **structurally cannot see these tests.** `.claude/rules/testing.md` prescribes
`grep -rln "<scenario-name>" crates/**/tests`; every pinning test here lives in an **inline `#[cfg(test)]`
module under `src/`**. Re-measured this pass: the prescribed form returns nothing for all four scenario
names, while the wide form (`grep -rln "<name>" crates/`) returns the file. `[measured P1]`

**Every pinning test is in `crates/conductor-core/src/scenario.rs`.** `[measured P1]`

| Test | Line | What it asserts | Effect of retirement |
|---|---|---|---|
| `cross_incident_recurrence_asserts_previously_seen_via_hard_contains` | `:1188` | the exact `Contains`/`Hard`/`"Previously seen"` triple | RED — direct |
| `constellation_context_grounding_suite_has_operator_checklist_members` | `:1207` | `!recurrence.expected.is_empty()` — "P-036 carries a Hard check" | RED — **and its premise dies**: the constellation family becomes all-empty, so the "suite exercises BOTH shapes" property it exists to hold no longer holds |
| `findings_counter_and_threshold_reload_carry_their_hard_checks` | `:1303` | P-045's `CountAtLeast`/`Hard` (first assert) | RED — first assert only; the P-056 `Absent` half stays live and correct |
| `constellation_severity_live_wiring_asserts_incident_visibility_via_hard_count_floor` | `:~1408` | P-079's `CountAtLeast`/`Hard`/`"1"` | RED — direct |
| `in_lane_sut_suite_mixes_operator_checklist_and_auto` | `:~1428` | the in-lane trio "must exercise both shapes" | RED — **premise dies**: constellation is the trio's only non-empty member (the other two are DriveObserve/empty by design) |

**Two of these five were not named by the originating residual**, which reported "at least one pinning test"
and named only `:1188`. The two suite-shape tests (`:1207`, `:~1428`) are the expensive half: retirement does
not merely break an assert, it removes the family property each test was written to pin. Whether those tests
are re-pointed, re-scoped or retired with a recorded reason is a **plan decision for P4**, not a mechanical
fix — this scope only establishes that they are in the modify-set. `[measured P1]`

**Asymmetry worth recording:** `scenarios/pulse-run-contract.toml` (P-073) has **no pinning test at all** —
no test in `crates/` asserts its declaration. The `pulse-run-contract` name does hit
`crates/conductor-core/src/run_contract.rs` and `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts`,
but both reference `contracts/pulse-run-contract.toml` — a **different artifact** (the SUT run contract), read
and excluded this pass. `[measured P1]`

**Hypothesis from the originating residual, kept at its own marking** `[inferred]`: "that blind spot is also
why this declaration was left standing when the surrounding family retired its checks — **plausible, not
measured**." P3 neither needs to prove nor disprove it; it is recorded so a later reader does not mistake it
for a finding.

## The "as a class" obligation

`v3-04` says **every** committed scenario's declared checks are satisfiable or retired. A universal claim is
falsified by one member, so the chunk owes an **enumeration of every live `[[expected]]` check across the
committed corpus** with each classified satisfiable-or-dead — not merely the retirement of the four predicted
members. The four are the prediction; the enumeration is what makes "as a class" true, and it is what the
audit gate chunk (`v3-06`) later mechanizes.

**The enumeration's population is measured and small** (P3, `grep -c '^\[\[expected\]\]' scenarios/*.toml`):
36 committed scenarios · **11 declare checks** · **12 `[[expected]]` blocks** (`high-severity-log-capture`
carries two) · **25 already declare-only**. So the eight non-member blocks to classify satisfiable are
`activity-floor` (Absent/Hard) · `exception-event-capture` (Contains/Hard) · `high-severity-log-capture`
(Contains/Hard + Absent/Hard) · `root-span-error-scope` (Contains/CalibrationRegion) · `service-went-silent`
(Contains/Hard) · `span-status-error-detection` (Contains/Hard) · `threshold-hot-reload` (Absent/Hard).
A fifth dead member found there is an expected outcome, not a scope breach.

## Boundaries — what this chunk is NOT

- **Not the tier work.** `v3-05` (`Scenario tier honesty`, the next markerless entry) owns every `slo_tier`
  question, including `constellation-severity-live-wiring`'s own `<20s`-vs-30 000 ms defect — the *same
  scenario*, a *different class*, a *different chunk*. This chunk does not touch `slo_tier` in any file.
- **Not the audit gate.** `v3-06` (`Scenario-assertion audit gate`) owns the mechanical re-runnable check.
  This chunk retires and enumerates; it does not ship a gate.
- **Not a coverage reclassification.** Measured this pass and decisive: all seven P-IDs behind the five
  already-retired declare-only scenarios (P-019…P-023, P-059, P-060) remain classified **`Auto`** in
  `coverage.rs`, and `check_scenario_backing` keys on whether a scenario **names** the id — via
  `list_scenarios`, never on `expected` emptiness (`drift.rs:140-175`). Declare-only and `Auto` coexist by
  shipped precedent, and all four target P-IDs are likewise `Auto`. `coverage.rs` and `drift.rs` stay out of
  the modify-set. `[measured P1]`
- **Not a config-validation change.** Declare-only is an existing validated shape: a scenario with no
  `[[expected]]` loads by construction (`expected_defaults_to_empty_when_omitted`, `scenario.rs:980`). The
  route's own security validation **rejected** adding "the validated config boundary held" to the entry for
  exactly this reason — the loader already enforces it, so asserting it adds nothing. No garde rule, no
  `Scenario::check_*` arm, no `from_toml_str` change is owed. `[measured P1]`
- **Not a live Pulse drive.** The evidence base is the already-committed live measurements plus P3's
  re-verification of the SUT-side premise against the Pulse repo on disk. This chunk drives nothing.
- **Not a residual-wide sweep.** The residual's tier findings (items 2 and 6) and its corrected testing-rule
  entry (item 5, already applied in place) are not this chunk's to re-do.

## Surfaces and contracts touched

- `scenarios/*.toml` — the four target files (declaration removal + retirement header comment), plus whatever
  the class enumeration adds.
- `crates/conductor-core/src/scenario.rs` — the inline `#[cfg(test)]` module holding all five affected
  pinning tests.
- **Governing rule:** `.claude/rules/security.md` §Input Validation (scenario-config surface) — the edited
  surface is the governed config boundary; the declare-only shape must stay inside it, which it does by
  construction.
- **Governing rule:** `.claude/rules/testing.md` — the companion-sweep discipline whose prescribed form is
  measurably blind to this modify-set. The rule entry was corrected in place at a prior wrap; this chunk is
  the first to execute against the corrected form.

## Done when

1. Every member of the structurally-dead class is declare-only, each carrying a header comment naming this
   chunk's marker and its measured ground.
2. Every pinning test that asserted a retired declaration is updated in the **same change**, with the two
   suite-shape tests dispositioned deliberately rather than deleted by convenience.
3. The class enumeration exists as committed evidence: every live `[[expected]]` check in the corpus
   classified satisfiable-or-dead, with its derivation command recorded.
4. The SUT-side premise is re-verified at Pulse's current HEAD and the reading recorded whatever it says.
5. Workspace gates green — `cargo nextest run --workspace --profile ci`, clippy, audit + deny.
