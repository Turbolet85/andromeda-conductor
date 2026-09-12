# Scope — Ledger gate id-space generalised

**Marker:** `2026-09-12-ledger-gate-id-space-generalised`
**Version:** `conductor-0.3.0` · Epoch 1 — Foundation: the measurements the closures rest on
**Working entry (verbatim):**
> Ledger gate id-space generalised — `requirement_ids` filters `starts_with("v2-")` while the same file's
> directory resolution was deliberately generalised, so the gate goes vacuous-then-red at every version
> transition; it is red now on `conductor-0.3.0`'s `v3-` ids (906 run, 905 passed, 1 failed) and red in CI on
> `c93a379` and `eecc7f4`. Surface measured: seven `v2-` literals at `:219 :221 :228 :250 :256-259` plus the
> id-space assertion at `:208` in `crates/conductor-report/tests/matrix_ledger_gate.rs`  BLOCKING: the
> version's workspace gate is red until this lands — nothing is scheduled ahead of it (operator directive,
> 2026-09-12 wrap)

---

## What this chunk is

`crates/conductor-report/tests/matrix_ledger_gate.rs` is the workspace's verification-ledger gate: three arms
asserting, over every version directory carrying BOTH ledger files, that the id sets in `requirements.md` and
`verification-matrix.json` are equal, that the matrix's ids are unique, and that no Pulse `P-NNN` occupies an
id position.

Its own module header (`:13-16`) records that the version directories are **resolved by scan rather than
named**, because "a baked `conductor-0.2.0` would silently stop covering the ledger at 0.3.0, which is the
failure mode this gate exists to prevent." One line of the same file then bakes the version space anyway:

```rust
:49        .filter(|id| id.starts_with("v2-"))
```

`conductor-0.3.0/requirements.md` declares 11 `v3-` ids and zero `v2-`, so at 0.3.0 the filter returns an
empty required set and the gate's own anti-vacuity assertion (`:151-154`) fires. The failure is therefore the
exact one the header claims the design prevents, arriving one axis over: the DIRECTORY resolution was
generalised and the ID-SPACE predicate was not.

This chunk generalises the id-space predicate so the gate covers every version's ledger on the same basis it
already resolves every version's directory, and so the transition to `conductor-0.4.0` needs no edit here.

**Status of the red:** `cargo nextest run --workspace --profile ci` is RED at HEAD — 906 run, 905 passed, 1
failed — and red in CI on `c93a379` and `eecc7f4`. It landed in `b3b51e6` and was dispositioned by the two
subsequent chunks as not-theirs. This chunk owns it. The gate returning green is this chunk's headline
acceptance.

## The surface, re-derived

The working entry's coordinates and the PHASE directive's grep basis both fold as hypotheses
(`promotion.md` §Atomic order step 2). Re-verified against the artifact:

**The directive's conclusion holds.** `grep -rn '"v2-\|v2-NN\|starts_with("v2' crates scripts .github` returns
hits in `matrix_ledger_gate.rs` only. Bare per-path counts confirm it: `crates` 1 file, `scripts` 0, `.github`
0. No second code site bakes the version space.

**The enumeration was two sites short, inside that conclusion.** The directive's pattern anchors `v2-` to a
preceding `"`, so a bare `grep -n 'v2-'` on the file returns **13** lines where the anchored pattern returns 11.
The two it misses:

- `:2` — `//! is complete (v2-21; test-plan §1 Critical Path 6).` — a provenance citation naming the v2
  capability that created this gate. **Correctly immutable**: it is a historical fact, not a live predicate.
- `:218` — `requirement_ids("- **v2-01** · a\n- **v2-99** · b\n")` — the fixture INPUT driving the `:219`
  assertion, i.e. the discrimination test of the very function being fixed. It is in the modify-set by
  necessity, and the entry's list does not name it.

**Sites elsewhere in the repo are citations, not predicates.** Seven other files carry a `v2-` token
(`cross_surface_parity.rs:1`, `drift.rs:32,48`, `scenario.rs:597,697`, `envelope.rs:150`,
`operator_pause_harvest.rs:1,3,225`, `severity_harvest.rs:243`) — every one a historical capability reference
in a comment or doc-comment. `.github/workflows/ci.yml:420` matches on `wv2-control` (WebView2), a false
positive. None is in the modify-set.

**Classification of the 13 in-file sites:**

| lines | class | disposition |
|---|---|---|
| `:49` | the defect — the baked id-space filter | generalise |
| `:153`, `:208` | assertion MESSAGES naming `v2-NN` | re-word to the generalised space; `:208` must keep stating the id-space separation contract |
| `:218`, `:219`, `:221` | fixtures driving `requirement_ids` — the function under repair | must discriminate the NEW predicate |
| `:228`, `:250`, `:256`, `:257`, `:259`, `:260` | fixtures for `id_positions` / `is_pulse_p_id` / `set_difference` — pure functions with no id-space knowledge | arbitrary data; may stay as-is |
| `:2` | provenance citation | unchanged |

## The constraint that bounds the generalisation

The filter is **load-bearing**, not incidental — it discriminates declared capability ids from other bolded
list items in the same document. Measured:

- `conductor-0.3.0/requirements.md` has **14** `- **`-declaring lines: 11 are `v3-NN` ids; three are prose
  titles (`:36`, `:38`, `:40` — e.g. `- **Unbuilt console surfaces — …**`).
- `conductor-0.2.0/requirements.md` has **33**: 32 are `v2-NN`; one is a prose title
  (`` - **`secret-scanning-ci-gate` — unrealized security-plan bootstrap item.** ``).
- `conductor-0.1.0/requirements.md` bolds a TITLE in every one of its 31 items
  (`- **Crate-per-seam Cargo workspace** — …`) and carries **no** `verification-matrix.json`, so the
  both-files filter skips it today.

So **removing the filter is falsified as an option**: it would admit 3 prose titles into 0.3.0's required set
and 1 into 0.2.0's, and the set-equality arm would fail against matrices that are in fact correct
(0.2.0: 32 matrix entries vs 32 ids; 0.3.0: 11 vs 11 — both currently consistent).

The replacement predicate must therefore, at minimum: accept `v2-NN` and `v3-NN` and any future `v{N}-{NN}`;
reject prose titles; and reject `P-NNN`, keeping the version space and Pulse's P-ID space visibly separate —
the contract `:208` states and `refs/`'s capability spec shares. The exact predicate form is a P4 decision
informed by P3; this scope fixes only the properties it must satisfy.

## Where the fix may NOT go

`requirements.md` is **immutable** — a version's `intent.md` and `requirements.md` are the human record of
what was asked and their ids are contractual; no skill writes them. The three prose-title lines in 0.3.0 and
the one in 0.2.0 are therefore fixed inputs the gate must tolerate, never content to normalise. The entire
repair lives in the gate file.

The pipeline's own ledger tool is out of scope and unaffected: `andromeda-tools/scripts/matrix.py` and
`_lib/matrixio.py` carry no `v2-`/`v3-` literal and no prefix filter (verified by direct grep of both installed
files), so it reads and writes the ledger across the transition whatever this chunk chooses. The
generalisation owes no compatibility to a pipeline-side convention.

## Boundaries — what this chunk does NOT do

- Does **not** edit any `requirements.md` or `verification-matrix.json` — the ledger data is correct; the gate
  reading it is wrong.
- Does **not** touch the other two arms' logic (`matrix_ids_are_unique`, `no_pulse_p_id_occupies_an_id_position`)
  beyond the `:208` message wording. `id_positions`, `is_pulse_p_id` and `set_difference` are id-space-agnostic
  and stay behaviourally identical.
- Does **not** change `ledger_dirs()`' both-files scan, nor bring `conductor-0.1.0` into coverage — 0.1.0's
  exclusion is a documented property of the design (`:15-16`), and its title-bolding declaring form is a second,
  independent reason it could not join without its own work.
- Does **not** widen to the seven citation sites elsewhere in the repo, nor to the `:2` provenance line.
- No dependency delta — VERIFIED at HEAD (P3): 562 packages, `Cargo.lock` un-drifted, `cargo audit` and
  `cargo deny check advisories bans licenses sources` both exit 0 over a locally-current advisory-db. The
  property is now a CONSTRAINT on the fixture choice rather than a free expectation: `rstest` is a
  workspace dependency but NOT one of `conductor-report`'s dev-dependencies, so adopting it would add a
  manifest line and move the lock (package count holding at 562 — which is the basis to state, never
  byte-identity). The chunk keeps the file's own inline-assertion idiom and adds no dependency.

## Folded annotations

The working entry carries one annotation at an annotation position: **`BLOCKING:`** — "the version's workspace
gate is red until this lands — nothing is scheduled ahead of it (operator directive, 2026-09-12 wrap)". This is
the blocks-others sense, not a `BLOCKED-ON:` external dependency, so Setup's HALT does not apply and nothing
external must clear first. It folds as a **sequencing fact**: this chunk precedes every other Epoch 1 entry,
and its completion is what unblocks the version's gate.

No `PREREQ:` or `CARRY:` freight on this entry. The project's standing external-decay pin (the cargo-audit
advisory-DB deferral) was **closed** on 2026-09-05 at `2026-09-05-audit-corrective`, so there is no standing
re-check obligation to fold off a skipped head.

## Capability targeted

None — VERIFIED at P3 against the full unclaimed pool. `matrix.py show --dir conductor-0.3.0 --unclaimed`
returns 10 (`v3-02` … `v3-11`), covering the a11y CI terminal, keyboard/focus ownership, scenario-assertion
hygiene, the P-025 measurement contract, the two live-Pulse legs, diagnostic quality and secret scanning.
None names the ledger gate. This chunk links nothing — a chunk addressing no capability links none.

## Touchpoints (provisional — P3 confirms)

- `crates/conductor-report/tests/matrix_ledger_gate.rs` — the whole modify-set, expected.
- Read-only inputs the gate consumes at runtime: `conductor-0.2.0/{requirements.md,verification-matrix.json}`,
  `conductor-0.3.0/{requirements.md,verification-matrix.json}`.
- Gate to return green: `cargo nextest run --workspace --profile ci` (currently 906 run / 905 passed / 1 failed).
