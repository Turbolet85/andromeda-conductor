# Scope — Remaining structurally-dead declarations retired

**Marker:** `2026-09-15-remaining-structurally-dead-declarations-retired`
**Version:** conductor-0.3.0 · **Epoch:** 2 — Scenario assertion hygiene
**Working entry:** `conductor-0.3.0/working-route.md:26`

> **P3 premise closure applied 2026-09-15.** Every `[inferred]` bullet below has been verified or
> corrected against research; corrections carry `[premise-corrected: …]`. Three consequences research
> found that the working entry, the CARRY freight and the committed enumeration all missed are recorded
> in §Newly discovered. P4 consumes this corrected text.

---

## What this chunk builds

The six remaining `[[expected]]` declarations that **cannot pass against a live Pulse** are retired to
declare-only, each carrying a header that states its measured ground; the pinning tests and guards that
hold those declarations move in the same change.

This is the second half of a class the previous chunk opened. That chunk retired the four members its
route entry ratified; its own enumeration measured six more, and the disposition of those six was left
to the operator. This entry is that disposition.

### Measured population (re-derived at take-up, not inherited)

| Quantity | Value | Derivation |
|---|---|---|
| Committed scenarios | 36 | `ls scenarios/*.toml \| wc -l` |
| Live `[[expected]]` blocks (pre-change) | 8 | `cat scenarios/*.toml \| grep -c '^\[\[expected\]\]'` |
| Scenarios declaring ≥1 block | 7 | per-file `grep -c` |
| — of those, structurally dead | **6 blocks / 5 scenarios** | this chunk's subject |
| — of those, satisfiable survivors | **2 blocks / 2 scenarios** | explicitly NOT this chunk's |
| Live blocks (post-change, expected) | **2** | 8 − 6 |

The arithmetic closes: 6 + 2 = 8. The two survivors are the only `[[expected]]` blocks that will remain
committed after this chunk.

---

## The six blocks (folded from the entry's CARRY — all six grounds VERIFIED at Pulse HEAD `83d4060`)

Each ground arrived as folded freight and is preserved verbatim. **All six re-derived at HEAD in P3**
(`research.md` §Mechanism-claim re-derivation); the `[inferred]` tags are dropped accordingly.

- `activity-floor` — `Absent`/`Hard`/`"ServiceWentSilent"` — dead because *"`CueKind` carries
  `#[serde(rename_all = "snake_case")]` and every render emits `service_went_silent`"*. **VERIFIED** —
  `crates/triage/src/contract.rs:181`. Direction: **vacuous PASS**.
- `service-went-silent` — `Contains`/`Hard`/`"ServiceWentSilent"` — same ground. **VERIFIED.**
  Direction: permanent FAIL.
- `high-severity-log-capture` — `Contains`/`Hard`/`"ERROR"` — dead *"against a severity label Pulse
  renders lowercase"*. **VERIFIED** — `severity_label` in `interpretation/src/markdown.rs` maps to
  `"info"|"warn"|"error"|"critical"`. Direction: permanent FAIL.
- `high-severity-log-capture` — `Absent`/`Hard`/`"WARN"` — same ground. **VERIFIED.** Direction:
  **vacuous PASS**.
- `exception-event-capture` — `Contains`/`Hard`/`"exception"` — dead because the token *"is absent from
  every composed-text field"*. **VERIFIED, and the basis is sufficient** — see the closure below.
  Direction: permanent FAIL.
- `threshold-hot-reload` — `Absent`/`Hard`/`"RetroactiveReeval"` — *"has zero occurrences tree-wide"*.
  **VERIFIED as stated** — see the closure below. Direction: **vacuous PASS**.

**Three of the six fail in the vacuous-PASS direction** — `activity-floor`, `high-severity-log-capture`'s
`Absent`, `threshold-hot-reload` — which test-plan §6 records as the more dangerous half, since nothing
ever fails to alert you.

### Premise closures on the grounds

- **Premise 1 (`exception-event-capture`, basis width) — CLOSED, concern dissolved.** The worry was that
  the ground's evidence (one file: `crates/interpretation/src/markdown.rs`) was narrower than its claim
  (every composed-text field). Re-derived: `dispatch_retrieve_report` composes markdown through
  `assemble_report` → `serialize_report`, and **both live in that one file** (`:249`, `:115`), so it IS
  the sole composer. The enumeration's basis was sufficient. No correction owed.
- **Premise 2 (`threshold-hot-reload`, which ground holds) — CLOSED in favour of the stated one.**
  `RetroactiveReeval` has zero occurrences tree-wide (bare `git grep -c --fixed-strings` ⇒ exit 1) and
  **no snake_case sibling exists**; the only `retroactive` hits are two doc comments
  (`crates/triage/src/lifecycle/mod.rs:57,103`) stating Pulse applies changes forward. So this is a
  genuine inferred-token absence — no producer ever emits it — **not** the serde-rename mechanism that
  explains its siblings. The header must state the absence, not borrow the rename.
- **Premise 4 (DECLARE-ONLY header shape) — CLOSED, VERIFIED.** The precedent is uniform across the
  prior chunk's retirements: marker-attributed header, numbered mechanism statements with SUT `file:line`
  citations, and an explicit record of what was deliberately not done and why
  (`scenarios/cross-incident-recurrence.toml:24-36`).

### Ground currency

**VERIFIED at take-up and re-confirmed in P3:** `../andromeda-pulse` is still at `83d4060`, the commit the
enumeration measured — so these are confirmation spot-checks, not re-measurements against a moved SUT.

---

## Newly discovered in P3 (not named by the entry, the CARRY, or the enumeration)

These three are the substance research added, and they materially enlarge the change:

1. **`hard_signal_fixtures_load_and_validate` must SPLIT, not move.** Its four `#[case]` rows mix two
   retirees (`exception-event-capture`, `high-severity-log-capture`) with the two deliberately-untouched
   survivors (`span-status-error-detection`, `root-span-error-scope`), and its `!expected.is_empty()`
   assertion must keep holding for the survivors (`scenario.rs:551-567`).
2. **A family guard BREAKS.** `threshold-hot-reload` is the last non-empty member of the
   `scrub_pipeline_degraded` family, so `scrub_pipeline_degraded_suite_mixes_hard_and_declare_only`'s
   `assert!(any(!empty))` (`scenario.rs:1364-1367`) becomes false. The severity-lifecycle precedent
   retires such a guard rather than weakening it.
3. **The prior chunk's own output contradicts itself, and this chunk must resolve it.**
   `scenario.rs:1299-1302` states P-056's `Hard Absent` "remains satisfiable" while the same chunk's
   committed enumeration classifies it structurally dead in the vacuous-PASS direction. P3's
   re-derivation settles it in the enumeration's favour; the plan must retire the check **and** correct
   that comment.

**Pin inventory (premise 3) — CLOSED, MEASURED.** The wide sweep finds exactly **one** check-membership
pin site — `crates/conductor-core/src/scenario.rs` — carrying **eight** tests to change and one to leave
(`research.md` §The eight pin changes). CARRY 2's "mint a guard for every unpinned member" resolves to
**nothing to mint**: all five scenarios are already pinned there. The narrow `crates/**/tests` form
returns a false zero for all five, as CARRY 2 warned.

---

## Boundaries — what this chunk is NOT

- **The two satisfiable survivors stay untouched.** `root-span-error-scope` and
  `span-status-error-detection` (`Contains "error"`) both match the lowercase severity label and are
  therefore satisfiable. That what they discriminate is *the fixture's severity constant rather than
  their own stimulus* is a weaker defect — the "what counts as satisfiable" question — and it belongs to
  the `Scenario-assertion audit gate` entry (`working-route.md:30`), not here.
- **No new checks are authored.** Correcting a dead token into a live one makes a check *reachable*, not
  *proven*, and can hard-Fail a live run on an unproven premise.
- **No scenario is deleted and no `p_ids` change.** Retirement leaves the scenario committed, validated,
  and carrying its Pulse P-ID.
- **No capability is claimed on a widened basis.** `v3-04` was un-claimed at the previous chunk's wrap
  with its acceptance deliberately NOT re-worded. Whether this chunk can claim it is a P4/P5 question
  over the FULL population; the prior PREMISE-CORRECTION instructs the next claiming chunk not to
  re-derive that dead end.
- **No guard is minted** — see the closed pin inventory above.

---

## Folded annotations (both CARRYs on the working entry)

### CARRY 1 — the enumeration is INPUT, not a thing to re-derive
`conductor-0.3.0/chunks/2026-09-15-structurally-dead-assertion-class-retired/evidence/assertion-class-enumeration.md`
— coordinate re-verified at take-up (exists, 6613 B). Consumed, not rebuilt; its mechanisms confirmed at
HEAD in P3 rather than re-enumerated.

### CARRY 2 — mint a pinning guard for every member retired that lacks one
Verbatim freight preserved: *"`pulse-run-contract` was the class's single unpinned member … the sweep
that finds the pins is the WIDE form `grep -rln "<scenario-name>" crates/`, never `crates/**/tests`,
which returned nothing for all four names that chunk retired."* **Discharged by measurement:** the wide
sweep ran, the narrow form's false zero reproduced, and all five scenarios are already pinned — so the
directive's guard-minting arm has no subject. Recorded rather than silently dropped.

---

## Surfaces and contracts touched

| Surface | Involvement (P3-closed) |
|---|---|
| `scenarios/{activity-floor,service-went-silent,high-severity-log-capture,exception-event-capture,threshold-hot-reload}.toml` | six `[[expected]]` blocks removed at known line ranges; a DECLARE-ONLY header added to each |
| `crates/conductor-core/src/scenario.rs` | **the sole pin site** — eight tests change, one unchanged |
| `contracts/pulse-capabilities.toml` | **read-only**, byte-unchanged — no P-ID changes |
| Coverage-completeness gate (`check_scenario_backing` / `UNBACKED_AUTO`) | **unaffected by construction** — `[premise-corrected: it keys on the P-IDs a scenario NAMES (`drift.rs:165`), never on graded checks; no `p_ids` change ⇒ `covered` byte-identical]` |
| Scenario validation (garde + `Scenario::check_*`) | **unaffected — VERIFIED, not inferred.** None of the five declares `[[checklist]]` (only `halo-breathing-encoding` / `halo-hue-encoding` do) and no retired block carries `budget_ms`, so `check_checklist`'s outcome does not invert and `check_budgets` loses no carrier |
| `crates/conductor-tauri/ui/test/a11y/screen-reader/nvda-pass-spec.md:73` | **no change** — an `a11y:sr` fixture roster line, not a pin; the fixture needs the scenario committed and validated, which retirement preserves |
| `contracts/pulse-load-envelope.toml` | **no change** — names `activity-floor` in a duration-exemption comment, unrelated to checks |
| `.andromeda/` spec masters | **read-only in this phase.** `[inferred]` `architecture.md` §Standard Contracts' declare-only registry (currently NINE families) and obs-plan §4's roster plausibly move again — wrap's amendment flow, never a phase edit |
| `conductor-0.3.0/verification-matrix.json` | `v3-04` is the candidate; claimability decided at P4/P5 |

---

## Remaining open premise

- `[inferred]` **Test-count movement is unpredicted.** The last measured workspace figure is 983
  (prior chunk's wrap record). This chunk removes six declarations and re-shapes eight tests; the net is
  a measurement for implement to REPORT, never a number scope or the plan should forecast — test-plan's
  de-hardcode line forbids pinning a count as a gate.
