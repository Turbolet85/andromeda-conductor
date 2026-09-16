# Scope — `2026-09-16-scenario-assertion-audit-gate`

**Working-route entry (verbatim surface):** Scenario-assertion audit gate — one mechanical re-runnable
check establishing both outcomes, named as a CI gate or an operator instrument.

**Version:** conductor-0.3.0 · **Epoch 2 — Scenario assertion hygiene** (this entry closes the epoch:
4 of 5 entries already frozen).

---

## What this chunk builds

One mechanical, re-runnable check over the 36 committed scenarios that establishes **both** epoch-2
outcomes without a reader re-deriving them from prose:

1. **the assertion outcome** — no committed scenario declares an unsatisfiable `[[expected]]` assertion;
2. **the tier outcome** — every scenario's declared `slo_tier` is accounted for against its own summed
   phase duration.

The check is **registered** — either as a CI gate that fails the build or as a named operator
instrument — so the property is enforced by something rather than by memory, and it is **demonstrated
able to fail as well as to pass** (a known-positive control per probe).

## Boundaries

- **In scope:** a mechanical check (or check set) plus its registration surface and its failure
  demonstration; the probe forms themselves; whatever enumeration/classification the check needs to
  make the currently-prose-held facts machine-establishable.
- **Not in scope:** changing any scenario's declared assertions or tiers. Both corpus properties are
  already TRUE at HEAD — `v3-04` and `v3-05` are `verified`. This chunk makes them *checkable*, not
  *true*. A probe that goes red here means the probe is wrong, not the corpus.
- **Not in scope:** retiring either of the two surviving `[[expected]]` blocks. They were deliberately
  left standing; the route entry states that retiring live coverage on a false equivalence "would be
  the opposite of solving".
- **Not in scope:** the SUT. No Pulse change, no live leg. This is a static check over committed data.
- **Scope law:** the check opens no listener and spawns no sidecar — it reads committed files only.

## Surfaces and contracts touched

- `scenarios/*.toml` — **read-only** (36 files; 2 carry a live `[[expected]]`, 36 carry `slo_tier`).
- `scripts/` — the likely home of the check, beside `scripts/mutation-gate.py` (7119 B) and
  `scripts/agent-run.{sh,ps1}`.
- `.github/workflows/ci.yml` — if the registration is a CI gate.
- `conductor-0.3.0/verification-matrix.json#v3-06` — the capability this chunk claims.
- `crates/conductor-core` — the catalog-gate home if the check ships as Rust. **Verified at P3:** the
  crate already owns `SloTier::deadline_ms()` (a code-native closed enum, `scenario.rs:52-73`), the
  `check_*` gate family (`drift.rs:79/155`, `load_envelope.rs:367`) and a crate-local `tests/` dir
  (2 files). The `coverage_gate.rs` precedent sits in `conductor-report` ONLY because it must reach
  both sides of a dependency edge (`coverage_gate.rs:11-13`) — a constraint that does NOT apply here,
  so `conductor-core` is reachable. The language/host fork itself stays open for P4.

---

## Folded annotations

All three annotations standing at an annotation position on the taken-up entry are `CARRY:`. There is
no `PREREQ:` and no `BLOCKED-ON:`. Every named coordinate below was re-verified against the artifact
at promotion; **mechanism claims are preserved verbatim with their original marker text and carry
`[inferred]`** — statedness is not measurement at HEAD, and P3's scope premise closure sets the depth.

### CARRY 1 — the mutation gate cannot enforce a timeout criterion

> the mutation gate cannot enforce a timeout criterion — `scripts/mutation-gate.py` computes its
> verdict from `missed.txt` against the roster and never reads `timeout.txt`, so a caught→timeout
> regression passes it silently (measured at
> `2026-09-14-emit-scrubber-and-percentile-math-under-test`: the file's only `timeout` token is in its
> module docstring; that run's FIFTEEN timeouts are held solely by the chunk's own
> `evidence/disposition-ledger.md`, classified by class and rostered nowhere). Carried here because
> this entry authors a mechanical check that must establish BOTH outcomes, and the same failure shape
> applies: grade on every tally the claim rests on, not only the one the verdict keys off.
> Operator-directed at the 2026-09-15 wrap; target chosen at an armed halt, no markerless entry naming
> the mutation gate.

**Coordinate re-verification (promotion, 2026-09-16) — all confirmed:**
- `scripts/mutation-gate.py` exists; its **only** `timeout` token is line 3, inside the module
  docstring. Confirmed exactly as stated.
- It parses `missed.txt` (`:137`) and diffs it against the roster's expected set (`:140-141`); it
  prints `missed · caught · expected` (`:142`) and never reads `timeout.txt`. Confirmed.
- `…/2026-09-14-emit-scrubber-and-percentile-math-under-test/evidence/disposition-ledger.md` exists
  and tallies `| timeouts | 16 | 15 |` — **15** is the final run. Confirmed.

- **Verified at P3.** The carried *design obligation* — this chunk's check must grade **every tally its
  claim rests on**, not only the one its verdict keys off. This is the CARRY's reason for being
  routed here and is the binding constraint on the check's design, not an observation about the
  mutation gate. The repo has solved this shape twice already: `check_load_envelope`
  (`load_envelope.rs:367-398`) grades THREE conditions — `unpinned`, `rotted`, `lost_subject` — and
  the a11y CI gate asserts the runner's PRINTED verdict rather than its exit code.
- **Still open — a P4 fork.** Whether this chunk also *fixes* `mutation-gate.py`'s timeout blindness.
  The CARRY routes the *failure shape* here, not the repair; the entry names no mutation-gate work and
  no markerless entry does either (re-confirmed at promotion). P4 decides: absorb, or record as a
  separate route candidate.

### CARRY 2 — "satisfiable" is now the whole question

> "satisfiable" is now the whole question, because the corpus has exactly TWO live `[[expected]]`
> blocks left — `root-span-error-scope` and `span-status-error-detection`, both `Contains "error"`
> (`grep -l '^[[expected]]' scenarios/*.toml` → 2 of 36 at
> `2026-09-15-remaining-structurally-dead-declarations-retired`). Both are satisfiable and both are
> WEAK for the same measured reason: the token they match is the deterministic fixture's pinned
> severity, not their own stimulus — `severity_label` maps every severity to a lowercase literal and
> the fixture pins one severity for every incident (measured at that chunk against `andromeda-pulse`
> HEAD `83d4060`: `crates/interpretation/src/markdown.rs` `severity_label`;
> `pulse-app/src/deterministic_inference.rs:42-43`). A check that asks only "can this token appear?"
> passes both, so this entry's mechanical check has to decide whether satisfiable is the right bar or
> whether discrimination-against-own-stimulus is. They were deliberately left standing rather than
> retired — retiring live coverage on a false equivalence would be the opposite of solving.

**Coordinate re-verification (promotion, 2026-09-16) — all confirmed:**
- Exactly **2 of 36** scenarios carry `[[expected]]`: `root-span-error-scope.toml` and
  `span-status-error-detection.toml`. Confirmed.
- Both are `kind = "Contains"`, `expected = "error"`; classes differ (`CalibrationRegion` vs `Hard`).
  Confirmed — the class difference is NOT stated by the CARRY and matters to any grading design.
- SUT repo present at `D:/dev/projects/andromeda-pulse`, HEAD **`83d4060`** — matches the cited HEAD
  exactly.
- `severity_label` at `crates/interpretation/src/markdown.rs:359` maps
  `Info|Warn|Error|Critical → "info"|"warn"|"error"|"critical"`. Confirmed lowercase.
- `pulse-app/src/deterministic_inference.rs:42` is `"severity": "autonomous"` inside
  `CANNED_L4_OUTPUT_JSON`. Confirmed present at the cited line.

- **[premise-corrected: the CARRY's arithmetic DOES close — `map_l4_incident_severity` is the link it
  never named, and the full chain re-derives TRUE at SUT HEAD `83d4060`.]** At promotion this bullet
  recorded that the CARRY's linking arithmetic failed to close on its own coordinates, because the
  fixture pins `"severity": "autonomous"` while the assertions match `"error"`. That doubt was MY
  error, not the CARRY's: a one-hop mapping the CARRY omitted joins the two. The chain, measured at
  P3 against `andromeda-pulse` HEAD `83d4060`:
  1. `deterministic_inference.rs:126` returns `CANNED_L4_OUTPUT_JSON` **regardless of prompt or
     schema** whenever `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy (`:18`);
  2. that fixture pins `"severity": "autonomous"` (`:42`) → `interpretation::schema::Severity`
     (`schema.rs:121-128`, `#[serde(rename_all = "snake_case")]`) `::Autonomous`;
  3. `map_l4_incident_severity` (`inference_runtime.rs:697-703`) maps
     `L4Severity::Autonomous → IncidentSeverity::Error`;
  4. `inference_runtime.rs:784` binds that to `severity`, and `:847-868` — the SOLE production
     `Incident` construction site — writes it to `incident.severity`;
  5. `assemble_report` (`markdown.rs:257`) renders `severity_label(incident.severity)`, and
     `severity_label` (`markdown.rs:359-366`) maps `Severity::Error → "error"`;
  6. `markdown.rs:134` pushes that label into the report markdown, which Conductor's
     `observe` folds into `Observation.text` (`extract.rs:99-104`) — the exact haystack
     `ComparisonKind::Contains` searches (`slo.rs:51`).

  **So CARRY-2 is correct: in deterministic mode every incident renders `severity_label = "error"`
  irrespective of what the scenario emitted, and both surviving assertions pass on a token their own
  stimulus did not produce.** `v3-04`'s verified acceptance is ALSO correct and not in conflict — it
  claims *satisfiability*, which holds. The two statements sit at different bars, which is precisely
  the distinction this chunk must decide rather than a contradiction to resolve.
- **Verified at P3, and the tests domain supplies the ruling.** The bar choice — *satisfiable* vs
  *discrimination-against-own-stimulus* — is a scope fork the entry leaves open, and `v3-06`'s
  acceptance asks only for "no committed scenario declares an unsatisfiable assertion", i.e. the
  *satisfiable* bar. The tests extract settles which bar a CI check may carry: the satisfiable bar is
  a declarative property of committed files and is gradeable here, whereas a
  discrimination-against-own-stimulus bar asks whether a token reaches the report *because of* the
  scenario's stimulus — a SUT-behaviour claim test-plan §11 routes to the live/operator gate and never
  to a static check. The measured chain above is what makes that ruling bite: the discrimination
  question is answerable only by driving a live Pulse in non-deterministic mode.

### CARRY 3 — the tier half is already satisfied and its sweep FORM is pinned

> the tier half of this gate is already satisfied and its sweep FORM is pinned —
> `verification-matrix.json#v3-05` is verified, and its acceptance names a comment-marker-stripped,
> wrap-tolerant sweep (strip the leading `#` per line, join, collapse whitespace) as the REQUIRED form,
> with the plain single-line grep recorded there as a known false negative. Measured at
> `2026-09-15-scenario-tier-honesty`: `grep -l 'MCP round-trip' scenarios/*.toml` returns 3 where the
> truth is 4, because `service-constellation-discovery.toml` wraps the phrase across two comment lines.
> A gate built on the single-line form reads a false green on a file that still carries the retired
> claim. Three probes from that chunk's plan are the re-runnable starting point (closed-set ·
> non-ceiling tier exceeded by own summed `gap_ms` · either-retired-gloss), each with a known-positive
> control proving it can fail; what is NOT yet mechanical is the third situation's separation, which
> rests on that chunk's authored `evidence/tier-ledger.md`.

**Coordinate re-verification (promotion, 2026-09-16):**
- `verification-matrix.json#v3-05` is `status: verified`, `chunk: 2026-09-15-scenario-tier-honesty`.
  Its acceptance limb (c) names the comment-marker-stripped wrap-tolerant sweep as REQUIRED and records
  the plain grep as a KNOWN FALSE NEGATIVE, citing `service-constellation-discovery.toml` lines 18/19.
  Confirmed verbatim.
- `…/2026-09-15-scenario-tier-honesty/evidence/tier-ledger.md` exists (7045 B), classifying all 36
  scenarios 9 / 2 / 6 / 19. Confirmed.
- The three probes exist verbatim in that chunk's `plan.md` `[[gate]]` fence, each `role = 'probe'`,
  `expect = ['exit 0', 'last line 0']`, each with a `baseline` recording its known-positive control.
  Confirmed.
- **SUPERSEDED MEASUREMENT — the one coordinate that did not re-verify as current.** The cited
  3-vs-4 split is **historical**. Re-measured at promotion, *both* forms now return **0**: the plain
  `grep -l 'MCP round-trip' scenarios/*.toml` → 0, and the stripped/joined/collapsed sweep → 0. That
  chunk retired the gloss (7 → 0), so `service-constellation-discovery.toml` no longer carries it.
  The CARRY's *rule* stands unchanged; only its live example is gone.

- **Verified at P3, with the discipline named.** Probe 3's known-positive control can no longer be "a
  real committed file that carries the gloss" — none does. The control must be a **synthetic fixture**,
  which is what that chunk's own `baseline` already describes. `testing.md:75` (2026-09-02) states the
  binding three-part form: seed the subject, keep its CONTENT in a committed file under `crates/`
  (never inline in harness config), and pin its MEANING with a production-reader round-trip — because
  "a parse failure is loud, a semantic drift is not", and the fixture must be built so the assertion
  CAN fail.
- **Verified at P3, and confirmed narrower than it reads.** The three probes are re-runnable but are
  **not committed anywhere** — `grep -rln 'slo_tier'` over `scripts/` and `.github/` returns only
  `agent-run.{sh,ps1}`, and that hit is the report-envelope jq projection (`agent-run.sh:293`), not a
  probe. They live solely in the prior chunk's `plan.md` gate fence. So the tier-side gap is exactly
  (a) **registration** and (b) a mechanical form for **the third situation's separation**, currently
  resting on the authored `tier-ledger.md`.
- **New at P3 — the in-repo mechanism for (b) already exists.** `check_load_envelope`
  (`load_envelope.rs:367-398`) is the exact-set ledger shape `testing.md:66` prescribes: it grades a
  committed `[[exempt]]` ledger of `Exemption { scenario, reason }` (`load_envelope.rs:63`) against the
  measured over-bound set in BOTH directions, failing on `unpinned` / `rotted` / `lost_subject`. The
  ledger is currently EMPTY by design. That is the template for turning `tier-ledger.md`'s authored
  classification into a mechanical separation carrying a REASON per exemption — a judgment, not a debt.

---

## Capability this chunk targets

`verification-matrix.json#v3-06` — *The scenario-assertion audit is mechanical and re-runnable*
(`method: unit`, `status: planned`, unclaimed). Its acceptance:

> One check, runnable by a reader who did not write it, establishes both that no committed scenario
> declares an unsatisfiable assertion and that every scenario's tier is accounted for. It is registered
> either as a gate that fails the build or as a named operator instrument, so the property is enforced
> by something rather than by memory, and it is demonstrated able to fail as well as to pass.

"Both outcomes above" resolves to `v3-04` (assertions satisfiable or retired — verified) and `v3-05`
(tiers honest — verified). **Verified at P3:** both are the immediately preceding ids, their subjects
match the acceptance's two clauses one-for-one ("no committed scenario declares an unsatisfiable
assertion" ↔ v3-04; "every scenario's tier is accounted for" ↔ v3-05), and `v3-05`'s own acceptance
closes the loop by naming `v3-06` as the capability that makes its limb (d) mechanical.

## Known-state summary at promotion

| Property | State at HEAD | Mechanical today? |
|---|---|---|
| Assertions satisfiable or retired (`v3-04`) | true — 2 live blocks of 36, both satisfiable | **no** — prose + a `grep -l` in the ref |
| Tier closed-set (`v3-05` limb b) | true — 0 violations | yes — probe 1 |
| Non-ceiling tier exceeded (`v3-05` limb a) | true — 0, from 8 | yes — probe 2 |
| Retired-gloss absent (`v3-05` limb c) | true — 0, from 7 | yes — probe 3 |
| Three situations separated (`v3-05` limb d) | true | **no** — authored `tier-ledger.md` |
| Any of the above registered/enforced | — | **no** — this chunk's job |
