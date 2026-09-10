# Session Handoff

**Last Updated:** 2026-09-10T07:37:05Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **0 ahead at wrap start** — the
operator's push of the prior chunk had landed and CI run `34365300658` completed. This wrap's chunk commit
makes it **1 ahead and unpushed**, and the push is **NOT load-bearing this time**: the chunk ships no CI
change (`ci.yml` byte-unchanged), so no gate's first CI run waits on it — the contrast with the prior two
wraps is deliberate.)
**Status:** clean
**Last Commit:** `feat(2026-09-09-port-occupier-test-hygiene)` (this wrap)

## Position
- Done: **`2026-09-09-port-occupier-test-hygiene`** — flipped `complete` at this wrap.
- Next: **`/andromeda-phase`** on the sole markerless entry, now the head — **_Release build and bundle_**
  (`working-route.md:133`). No `BLOCKED-ON`, so phase will not halt. It carries **three CARRYs**, one
  DISCHARGED: the panic-hook race (coordinates repaired this wrap), the `--e2e` exit-code leak (discharged at
  `2026-09-07-a11y-ci-gate`), and the SR announcement variance (open).
- Coverage **28/32 verified · 1 deferred · 3 unclaimed** (`v2-04`, `v2-21`, `v2-27`) — **unchanged**. This
  chunk claimed nothing, deliberately: no pooled id is its subject, and `matrix.py flip` reported
  `claimed 0 — gate no-op`.
- **Evolve:** Epoch 6b at **13 chunks**. Surfaced again per the growth valve; the operator ruled **no split**
  last wrap. With one markerless entry left, the epoch closes on the next chunk either way.

## Work done
One chunk, a pure relocation. `the_hold_is_bracketed_by_a_fault_span_on_the_emitted_lines` moved out of
`crates/conductor-faults/tests/port_occupier.rs` (7 tests → 6) into its own binary
`crates/conductor-faults/tests/port_occupier_span_witness.rs` (1 test, 60 lines) — the remedy `test-plan`
§11:538 prescribes and three sibling crates already apply, making this its **fourth** in-tree application.
**−48 lines, one new file, no `src/` delta, no manifest delta.**

The assertion moved **byte-identically** (a `diff` gate against the parent commit, exit 0 / 0 bytes), which
is what preserves its power to fail if `port` leaves `ALLOWLISTED_FIELDS` — proven the other way too by a
one-shot control that removed `"port"`, observed the witness go red, and reverted.

The defect was **first-match record selection** over a shared process-global sink, not a diffuse race: six
concurrent `new` records with six distinct ports, `.find()` returning a sibling's.

## Gates
All nine green, **0 fix iterations**: `cargo test -p conductor-faults` **exit 0** (baseline 101) · nextest
`-p` 30/30 · the portability sweep **`measured 9 crates, red 0`** · byte-identity `diff` 0/0 bytes ·
lockfile probe 0 · clippy 0 · `cargo fmt --all --check` 0 · `cargo nextest run --workspace --profile ci`
**902/902 across 56 binaries** (was 902/55 — a relocation, not an addition) · `agent-run.sh status` 0.
**Smoke:** `agent-run.sh run` 0 (nextest 902/902, `--doc` 3 passed, clippy 0).

No gate deferred — the chunk has `.rs` delta, so every compiled-language gate was mandatory and all ran.

**Widened at the P5 review (operator):** the sweep enumerated only the route entry's **eight** touched
crates while the workspace has **nine** members, leaving `conductor-report` outside the gate entirely. It
measured green (51+3), and the gate now **derives its crate list from `cargo metadata --no-deps`** and
asserts the crate COUNT beside the failure count, so a failed derivation reads `measured 0 crates` instead
of passing vacuously.

## Drift resolved
**None — 0 proposals across all 7 detectors, 0 amendments, 0 escalations, 0 open.** No master was edited and
no sidecar written; no cascade pass is owed (the cascade walks CHANGED sources). The chunk touched no
spec-governed surface: two crate-local test files, no `src/` delta.

Each detector swept independently and recorded its own basis — the three moved counts (binaries 55 → 56,
file tests 7 → 6, doctests 0-inherited → 3-measured) have **no baked occurrence** in any master, and numeric
near-misses were read and dismissed (`comfy-table 7` a crate version, `6 columns` a table shape,
`P-003 port-occupier` coverage-matrix samples). The test-plan detector independently surfaced
`test-plan.md:226`'s `(full, non-doc)` phrasing against the measured 3 doctests and reached the same
**non-escalation** the report had recorded: the phrase reads as "the full suite, as opposed to the doc-only
arm" and bakes no count.

**One disproved claim, DISPOSED:** the `--doc` arm's "0 doctests" is the 2026-06-19 record; measured **3** at
HEAD (`gap.rs:35`, `silence.rs:23`, `train.rs:42`), and `conductor-faults` is the only workspace crate
carrying any. No master states it, so no amendment was owed — it rode the **master-record desc** at the flip
(operator wrap directive 4) and corrected `testing.md:88`'s supporting figure in place.

## Notes
- **Curation: T1 0 · T2 2 · T3 0 · 1 correction** (cap-exempt), 0 conflicts. Both survivors scored 0.8
  (user correction + verified by measurement), so neither conditional signal was needed.
  - `host-win32.md` — **a line-granular grep cannot DATE a clause inside a multi-KB single-line entry**: it
    collapses every dated extension into ONE hit, so "one hit, dated X" neither refutes nor dates a clause
    written later. Resolve by offset; cite as "the `{lead-date}` entry as extended `{clause-date}`".
  - `testing.md` — **`cargo test -p` runs targets in SEQUENCE and aborts at the first failure**, so a red
    tells you nothing about the targets behind it, and what a red *masked* is a claim about the BASELINE's
    target set (differencing the two runs' lists overcounts by the targets the chunk itself added).
  - **Correction** (`testing.md:88`): its supporting detail said the `--doc` arm "executed zero doctests" —
    tagged `[corrected 2026-09-10]` with the measured 3; the half its argument rests on (the arm never
    touches the crate's integration tests) is unaffected.
- **Route repair** (operator wrap directive 3): the panic-hook CARRY's coordinates `obs.rs:491-496`/`:532-537`
  → **`:535-540`**/**`:582-587`**, re-measured date inline. Mechanism unchanged. I first wrote that the old
  ranges "now hold the rfc3339 timestamp test" — true of `:491-496`, but `:532-537` straddles
  `build_subscriber` plus the first pair's head, so the note was tightened to "no longer bound these sites".
- **Two report corrections, both operator-caught** (directives 1–2): the masked-target count was **one**
  target, not two (the witness binary is created by this chunk and could not have been masked); and the
  census citation is `verification-harness.md:58` — the 2026-09-02 census rule **as extended 2026-09-08** —
  not a standalone 2026-09-08 entry. StartTime remains the basis.
- **My own P4 defect, owned in the report:** the plan's `role='smoke'` entry was a bare `status`, the form
  `verification-harness.md:42` forbids for a no-boot-path chunk. Remedied by execution — P3 drove the
  prescribed `agent-run.sh run` — not by a note.
- **Process census:** this chunk's runs started nothing. Six `msedgewebview2` are **not** its own: StartTime
  `2026-08-13` at implement, re-measured `2026-09-10 08:02:05` at this wrap — the set turned over between the
  two readings, and both attributions rest on StartTime rather than the census pattern.
- **Last failed command:** none.

## Deferred learnings
None over the cap. **Two `recurrence-despite-learning`, both logged rather than re-curated** — a third
statement of a rule is not a remedy:
- `verification-harness.md:42` states that a no-boot-path chunk's smoke is `run`, not `status`; the plan
  listed `status` anyway. The sharper half is **availability**: that file's `paths:` include
  `crates/**/tests/**`, which this chunk touches, so the rule auto-loaded and was present at plan-authoring
  time — it simply was not reached. **No plan-authoring gate compares a listed smoke entry against the
  harness rules**, so an available-and-unapplied rule costs nothing at P5 and surfaces only at implement.
- CLAUDE.md's read-the-hits / narrow-basis entry did not prevent the two report claims above — the **fourth
  and fifth** instances of that family this session. Two of the five produced NEW rules this wrap because
  their mechanisms were previously uncurated; the other three recurred against entries already correct.
