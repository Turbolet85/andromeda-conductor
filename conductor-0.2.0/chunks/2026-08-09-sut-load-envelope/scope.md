# Scope — SUT load envelope

**Marker:** `2026-08-09-sut-load-envelope`
**Version:** conductor-0.2.0 · **Epoch 1 — Foundation: re-aim at the SUT**
**Working entry:** _SUT load envelope — proven-good storm bounds for scenario authors plus environment-suspect flagging of over-envelope runs_

---

## Why this chunk exists

Intent §Theme 1 **F4 — "The SUT has a known failure mode that bounds scenario design."**

> OBSERVED: Pulse records that the DuckDB append path stalls after ~10 minutes of sustained 10k/s
> storm + deterministic L4 — ingest keeps receiving while `duckdb.append` stops, so
> `viz.query.traces` returns 0 rows and no new incidents form until restart. Conductor's whole
> method is sustained shaped load.
> EXPECT: scenario durations and storm profiles stay inside the SUT's proven-good envelope, the
> constraint is recorded where scenario authors will see it, and a run that exceeds it is flagged
> as environment-suspect rather than reported as a Pulse failure.

The harness's whole method — sustained shaped load — is also the exact input that can silently
break the SUT's read path. Without this chunk, a scenario that runs long or storms hard produces a
Pulse that answers "0 rows" to every read-back, and Conductor reports that as a Pulse **failure**.
The SUT would be blamed for a fault the harness induced.

## What this chunk builds

Two halves, mapping to two verification-matrix capabilities:

**A. The envelope, recorded and asserted (`v2-06`).**
The proven-good bounds become a first-class, committed artifact rather than a sentence in a plan —
sourced from Pulse's own record (sustained rate ceiling and sustained-duration ceiling, plus
whatever second-order terms that record actually states). Every scenario in the catalog is asserted
to sit inside it, so the catalog cannot drift out of bounds unnoticed. The constraint is surfaced
**at the scenario-authoring surface** — where someone writing a new `scenarios/*.toml` will meet it
— not only in `.andromeda/`.

**B. Over-envelope runs classified as environment-suspect, not Fail (`v2-07`).**
A run whose realized profile exceeds the envelope is reported as **environment-suspect**: a
classification distinct from `Fail`, whose report names the envelope breach as the cause. The point
is attribution — the harness drove the SUT outside its proven-good region, so the resulting
zero-row read-backs are not evidence about Pulse.

## Surfaces and contracts this touches

- **Scenario model** (`conductor-core`: `scenario.rs` · `phase_spec.rs` · `expected.rs`) — the
  envelope assertion is a load-time property of a `Scenario`, so it belongs beside the existing
  garde bounds and the `check_capabilities` membership gate. Note the profile is currently
  expressed only as per-phase `gap_ms` (+ `jitter_ms`); `EmissionSpec` carries no occurrence-count
  field, and `fingerprint-storm.toml` records that the driver realizes counts from phase names.
  Whether "storm profile" is derivable from the committed model as it stands is a **research
  question for P3**, and it bounds how strong the `v2-06` assertion can honestly be.
- **The scenario catalog** (`scenarios/*.toml`, 34 files) — every entry is in the assertion's scope.
- **Reporting / classification seam** — `ReportState` is today a **closed five-variant set**
  (`Pass` · `Fail` · `ManualCheck` · `KnownResidual` · `Blocked`), pinned in `architecture.md`
  §Standard Contracts as `state ∈ {…}` and mirrored across the CLI ASCII prefixes, the Markdown
  lamp, `runs.db`, and the webview `LAMP_META`. Where "environment-suspect" lands against that
  closed set is the chunk's central design fork — see Open questions.
- **Run-report envelope + `runs.db`** — if the classification is carried per-run or per-check, it
  must round-trip through the canonical envelope shape and the SQLite index without breaking the
  Blocked-row NULL rule.
- **Documentation surfaces** — wherever the authoring constraint is recorded, the docs that mirror
  it (design-system / layout-templates / test-plan roll-ups) inherit the derived-count discipline:
  **name the SET, never substitute a literal that re-stales.**

## Boundaries — what this chunk is NOT

- **Not a load-tester.** The standing non-goal is unchanged; this chunk *bounds* load, it does not
  generate or measure throughput as a product feature.
- **Not Pulse process management.** Conductor does not restart Pulse when the append path stalls.
  Detecting the stall's downstream signature is in scope only insofar as attribution requires it.
- **Not the live-path work.** Epoch 2 owns preflight terms, per-check read-back extraction and the
  faithful dispatcher. This chunk must not pre-build them; it may only assert against what exists.
- **No new scenario capability claims.** No `p_ids` change, no `UNBACKED_AUTO` movement — a
  classification/bounds chunk, not a coverage chunk.
- **No `deny.toml` edit, no CI edit, no cargo-audit floor raise** (see PREREQ).

## Open questions — RESOLVED at P3/P4

_Amended after planning (validation-1 `intent-incomplete`): research and two operator decisions
answered all three, and the first answer materially qualifies "asserted inside it" above._

1. **Where does `environment-suspect` live against the closed `ReportState` set?** → **A run-level
   qualifier; the state set is unchanged.** Operator-confirmed at P4. The code-graph puts
   `ReportState` at 204 references across 16 files in 6 crates plus the webview `LAMP_META` mirror,
   and five of the seven distillers independently landed on the qualifier (obs's `(N unbacked)`
   precedent; layouts' and a11y's out-of-scope Mode-cell precedent; tests calling it "the cheaper
   binding"). The qualifier persists in its own run-level table, leaving `RunRecord`'s eleven fields
   and the `runs` table's eleven columns provably untouched.
2. **What exactly does Pulse's record state?** → **Nothing in Conductor's tree carries it directly.**
   `.andromeda/refs/` (Pulse's capability spec, MCP contract, and v0.2.0 audit) never mentions the
   DuckDB append stall, 10k/s, or a ten-minute ceiling; `input.md:152` separately names Pulse's
   `crates/ingest/examples/load_profiles.rs` as the 10k-spans/s prior art. **Intent §Theme 1 F4 is
   the record we have**, so the envelope artifact carries a `provenance` field saying plainly that
   its terms are a transcribed SUT record, not a Conductor measurement.
3. **Static or dynamic detection?** → **Both, on different axes — and only one axis is derivable.**
   Duration is a static property of the committed model (the sum of per-phase `gap_ms`), so `v2-06`
   is a static gate over committed artifacts. But `PhaseSpec`/`EmissionSpec` carry **no rate or
   occurrence-count field at all**, so the envelope's storm-rate axis is not derivable from
   committed data; it is recorded in the artifact as declared-not-derivable and owed to Epoch 2's
   *Faithful emission dispatcher*.

## Qualification to "every scenario is asserted inside it" (§What this chunk builds A)

Research proved the unqualified reading is not achievable *and would be wrong*. Two committed
scenarios exceed a ten-minute duration bound — `activity-floor` at 3900s and
`incident-auto-resolution` at 731s — but `activity-floor` spends fifty of its sixty-five minutes in
**deliberate quiet gaps** (10 + 10 + 30 min), and the SUT stalls on sustained throughput, not on
elapsed time. Flagging it would be a false positive against the very scenario whose purpose is
idleness.

So the assertion is: **every scenario sits inside the envelope's duration term, or is an exact-set
pinned exemption carrying its reason** — the project's established ledger discipline
(`KNOWN_UNCLASSIFIED` / `UNBACKED_AUTO`), held to equality in both directions so it fails on ledger
rot and on a lost subject, and can only shrink under compulsion. When Epoch 2 makes intensity
derivable, the gate should assert emitting-phase duration and both exemptions should retire.

## Folded PREREQ (from `2026-08-09-in-lane-sut-scenarios`)

**Re-check `cargo audit` at this chunk's gates — the FIFTH consecutive check.**
It has been red four times running with a byte-identical
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`,
re-proven on cargo-audit **0.22.2** (the latest published). That proves an advisory-**DATABASE**
fault, not a tool fault: the duplicate id is committed data in RustSec's advisory-db, so there is
nothing to raise a floor to.

Per `playbook.md` (external-decay) · `.claude/rules/security.md` (2026-08-09) ·
security-plan §Dependency Security, the remedy is the **bounded wait alone**:

- Re-run it, and **verify `cargo deny check` actually ran green** (observed, never assumed) as the
  overlapping signal.
- Do **NOT** raise the floor · do **NOT** add a `deny.toml` ignore · do **NOT** edit CI.
- **Close the deferral the moment it parses.**

## Verification-matrix capabilities in view

| id | title | method | status |
|---|---|---|---|
| `v2-06` | SUT load-envelope constraint | unit | planned → this chunk |
| `v2-07` | Over-envelope run flagged environment-suspect | integration | planned → this chunk |

Linked at P5 only if this chunk's plan makes each **fully verifiable** at its wrap.
