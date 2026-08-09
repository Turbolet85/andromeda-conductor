# Scope — In-lane SUT scenarios (P-067, P-072, P-079)

**Marker:** `2026-08-09-in-lane-sut-scenarios` · **Version:** conductor-0.2.0 · **Epoch:** 1 — Foundation: re-aim at the SUT

**Working-route entry (verbatim intent):**
> In-lane SUT scenarios — live service truth, Investigate result and single-sourced workspace key
> (P-067, P-072, P-079)

**Requirement provenance:** `v2-04` — "In-lane SUT-capability scenarios: the capabilities that fall squarely
in Conductor's lane are covered by scenarios, proving Pulse P-067, P-072 and P-079" (intent §4 F2).
Observed gap: *no scenario exists for any capability above P-060*, so the in-lane subset of Pulse's v0.3.0
work has no Conductor coverage at all.

---

## What this chunk builds

The **first scenario-catalog entries above P-060**. Three declarative `scenarios/*.toml` files keyed to the
three Pulse capabilities intent §4 F2 places squarely in Conductor's lane, plus the one code edit their
existence compels (shrinking the `UNBACKED_AUTO` pin).

### The three capabilities (authoritative statements, read from the SUT's own route)

| P-ID | Pulse's statement | Conductor's classified mode (`coverage.rs`) |
|---|---|---|
| **P-067** | Live-only service truth — the constellation shows only currently-live services; persisted/stale registry entries hidden or marked historical, with honest recency labels | `DriveObserve` |
| **P-072** | Investigate actions produce a real L4-backed result (intent: "candidate drive+observe via report read-back") | `DriveObserve` |
| **P-079** | Constellation severity live-wiring — reconcile the incident workspace key (resolver `data_dir` vs producer detected-root) so per-service severity + the incidents panel light up under a live storm | `Auto` |

P-079's two names in this repo (`coverage.rs` "Constellation severity live-wiring" · intent §4 F2
"the incident workspace key is single-sourced") denote **one** capability — Pulse's own record reads
"Constellation severity live-wiring — reconcile incident workspace key". No contradiction to resolve.

### Deliverables

1. **Three scenario TOMLs** under `scenarios/`, each carrying its P-ID, seed, `slo_tier`, phase sequence and
   — where the mode allows one — an `[[expected]]` block. Mode drives the shape:
   - **P-067 / P-072 are `DriveObserve`** → operator-checklist scenarios on the established ManualCheck
     path: DRIVE the stimulus, no `[[expected]]` block, the operator confirms the visual claim
     (`scenarios/service-constellation-discovery.toml` is the shipped pattern for exactly this).
   - **P-079 is `Auto`** → Conductor drives the stimulus *and* asserts the reaction, so it carries a real
     `[[expected]]` block over the read-back surface.
2. **Shrink `conductor_core::UNBACKED_AUTO`** by `P-079` (11 → 10 entries) — see the hard CARRY below.
3. **Verify** — not edit — the derived `(N unbacked)` qualifier on all three coverage surfaces: they currently
   read `43 auto (11 unbacked)` and must read `(10 unbacked)` after the pin shrinks.

   > **Amended at P5 (validation-1, intent-incomplete).** This deliverable was written before P3 research and
   > assumed three render edits. The code-graph shows all 13 `UNBACKED_AUTO` consumers derive the count —
   > `summary_line(rows, unbacked)` takes it as a parameter, `conductor-cli/src/render.rs:209` and
   > `conductor-tauri/src/commands.rs:140` read `.len()`, and grep confirms **zero** hardcoded `11` in
   > `crates/`. The pin edit therefore propagates to the Markdown report, the CLI table and the webview with
   > **no render change**. The intent is unchanged (the three surfaces must read correctly); only the assumed
   > mechanism was wrong, so this is an assertion to hold, not code to write.

## Boundaries — what this chunk is NOT

- **Not the live proof.** This chunk authors the CATALOG. `v2-04`'s acceptance ("each drive a live Pulse and
  yield a non-blocked verdict via MCP read-back, with journal and `runs.db` evidence per scenario") cannot
  pass here: the faithful per-phase dispatcher (`v2-08`), real per-check read-back extraction (`v2-09`) and
  a live `ready: true` preflight (`v2-10`) are all Epoch-2 entries, and `execute_scenario` still grades every
  check against the literal `"incidents-listed"` (`conductor-run/src/lib.rs:216`). This chunk therefore
  **partially advances `v2-04`** and leaves it `chunk: null` per the verification-matrix multi-chunk rule,
  citing it in provenance rather than claiming it.
- **Not a Pulse fix.** Intent §5 states the new non-goal explicitly: Conductor does not fix Pulse. F10's
  workspace-key divergence has a Pulse-side remedy; Conductor records the requirement, detects the
  condition and fails loudly. Authoring the P-079 scenario is the *record*, not the fix. The detection
  itself ("Workspace-key divergence probe") is a separate Epoch-2 entry.
- **Not UI automation.** The standing non-goal holds: visual claims are operator-checklist items, which is
  precisely why P-067 and P-072 are `DriveObserve` and not `Auto`.
- **Not P-073 / P-074.** They stay in `UNBACKED_AUTO`; they are owned by the Epoch-2 "Pulse run contract"
  and Epoch-3 "fingerprint-storm live proof" entries. Only `P-079` leaves the pin here.
- **No new crate, no new seam, no dependency delta** expected.

---

## Folded-in annotations (carried onto this entry by prior wraps)

### PREREQ (from `2026-08-09-interpretation-correctness-posture`) — `cargo audit`, bounded wait only

`cargo audit` has been red at **three consecutive checks**, and is now proven an **advisory-DATABASE fault,
not a tool fault**: installing the latest published **0.22.2** reproduced
`duplicate advisory ID: RUSTSEC-2026-0244` byte-identically, so the duplicate id is committed data in
RustSec's advisory-db and **there is nothing to raise a floor to**.

Per the amended rule (`playbook.md` external-decay · `.claude/rules/security.md` 2026-08-09 ·
security-plan §Dependency Security), the remedy at this chunk's gates is the **bounded wait alone**:
re-run it, and **verify `cargo deny check` actually ran green** as the overlapping signal.
Do **NOT** raise the floor · do **NOT** add a `deny.toml` ignore · do **NOT** edit CI.
**Close the deferral the moment it parses.** CI's own `cargo audit` step hits the same wall until upstream
heals.

### CARRY (from `2026-08-09-interpretation-correctness-posture`) — the hard gate

**`P-073` / `P-074` / `P-079` sit in `conductor_core::UNBACKED_AUTO`** (`conductor-core/src/drift.rs:60`) —
the pin of `Auto`-classified capabilities no scenario names, asserted by `check_scenario_backing` at
**EXACT-SET equality**. Authoring a scenario that names any of them **without** removing it from the pin
fails the gate with *"in the unbacked ledger but now named by a scenario — shrink the ledger"*.

**Shrinking the pin is part of this chunk's work, not a follow-up.** This chunk names `P-079`, so the pin
goes from 11 entries to 10. `P-073` and `P-074` remain (their scenarios are later entries), and the four
interpretation-cluster entries (`P-031`/`P-033`/`P-034`/`P-044`) remain as the recorded `v2-05` deferral
owned by conductor-0.3.0 — **do not disturb them**.

---

## Surfaces and contracts touched

- **`scenarios/`** — three new TOMLs; declarative config validated at load by serde + garde, with
  `Scenario::check_capabilities` asserting P-ID membership against `contracts/pulse-capabilities.toml`
  (all three ids are in the accepted set, `sut_version = "v0.3.0"`).
- **`conductor-core`** — `drift.rs` `UNBACKED_AUTO` (data edit) and its `check_scenario_backing` tests.
- **The three coverage roll-up surfaces** — Markdown report · CLI table · webview — for the `(N unbacked)`
  qualifier only. Note the recorded denominator semantics: the `(N unbacked)` term is a **derived
  qualifier, not a fifth summand** (obs-plan §4), so the four per-mode counts must keep summing to the row
  total.
- **Scope law** — "no scenario without a P-ID" holds by construction; every new TOML carries one.

## Acceptance shape (refined at P4)

- Three scenario files exist, load cleanly through the validating path, and are visible in the catalog
  (`list_scenarios`) with their P-IDs accepted by the manifest.
- `check_scenario_backing` is green with `UNBACKED_AUTO` at 10 entries — i.e. the gate proves P-079's
  `Auto` claim is now backed by a scenario that names it.
- The two `DriveObserve` scenarios carry no `[[expected]]` block and route to the ManualCheck /
  operator-checklist path; the `Auto` scenario carries a real expectation.
- Every roll-up surface reports the shrunk unbacked count consistently — held by the existing derived-count
  assertions, with no render edit (see the P5 amendment on deliverable 3).
- `cargo audit` re-checked per the PREREQ, with `cargo deny check` verified green.
