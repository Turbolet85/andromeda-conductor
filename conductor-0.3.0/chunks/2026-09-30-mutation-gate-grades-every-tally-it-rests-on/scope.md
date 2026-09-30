# Scope — 2026-09-30-mutation-gate-grades-every-tally-it-rests-on

**Working entry (`working-route.md:61`):** Mutation gate grades every tally it rests on — `scripts/mutation-gate.py`
reads `timeout.txt` beside `missed.txt` and `caught.txt`, and the timeout class is rostered rather than held only in a
chunk's own disposition ledger.

**Matrix target:** none named by the entry. The only unclaimed capability at take-up is `v3-03` (keyboard/focus-order
ownership), which the `:63` entry owns — not this chunk's. P4 re-reads the pool through `matrix.py show --unclaimed`.

**Chunk base (W182):** `f33d6b7` (`f33d6b7f98677361cd125b6fa67459bd0a66c8d4`), HEAD at take-up. Every diff-shaped gate
probe names it explicitly (`git diff --numstat f33d6b7 -- <f>`), because the operator pre-CI commit moves HEAD before
the wrap (founder directive at take-up, W182 — as applied at
`2026-09-30-interpretation-re-proven-on-a-clean-named-data-dir/scope.md:15-17`).

## Founder rulings at take-up (2026-09-30, verbatim intent)
- **No cargo mutants run in this chunk.** Mutation testing runs at the epoch-boundary code audit only, never per
  chunk. This chunk proves the gate on FIXTURE tallies — synthetic `missed` / `caught` / `timeout` files — graded by
  the gate's own logic.
- **Evolve-diagnose waits** until the epoch's carried work is done — not now, and not this chunk's work.
- **Diff-shaped probes name the chunk base (W182)** — `f33d6b7`, above.

## What this chunk builds
1. **The gate reads and grades `timeout.txt`.** Today `scripts/mutation-gate.py` computes its verdict from
   `missed.txt` (`:137`) and `caught.txt` (`:138`) and never reads `timeout.txt` — VERIFIED at take-up: the file's only
   `timeout` token is its module docstring (`:3`), so a caught→timeout regression passes silently. The gate reads
   `timeout.txt` with the same `parse_tally` and compares it against a rostered expected multiset for the unit, with
   the same unexpected/absent failure lines `missed` gets.
2. **The roster gains a timeout class keyed the same way `missed` is** — coordinate-free `(file, mutation)`, a
   multiset, per unit, in `scripts/mutation-roster.toml` (the entry's own expectation: "expect the roster to gain a
   timeout tally keyed the same way `missed` is, and expect `scripts/mutation-roster.toml` to move").
3. **The gate is provable without running cargo mutants.** The grading must run against an existing tally directory
   so fixture tally trees can drive it: pass, unexpected-timeout FAIL, rostered-timeout-absent FAIL, and the existing
   `missed` arms, each as a separate fixture. A seam is needed. VERIFIED at P3: the grading is inline in `main()`
   (`:137-153`) after an unconditional `subprocess.run(cmd, cwd=ROOT)` (`:122`), and `expected_for` reads the module
   constant `ROSTER` (`:37`), so neither a fixture tally nor a fixture roster can reach the grading today.
   [premise-corrected: "fixture `mutants.out/` trees" — `.gitignore:62` (`mutants.out/`, no leading slash) ignores a
   directory of that name at any depth (`git check-ignore -v --no-index` on such a path → `.gitignore:62`), so a
   committed fixture must not be named `mutants.out`, and the seam takes the tally directory ITSELF.]
4. **The gate's regression proof is a committed, re-runnable test**, not a one-off manual reading. VERIFIED at P3:
   the project's own precedent for a Python operator instrument is an in-script `selftest` verb
   (`scripts/arch-registry-check.py:470-484`, gated `expect = ['exit 0', 'last line selftest: every arm detected']` at
   `2026-09-24-architecture-registries-compacted-under-the-read-cap/plan.md:234-238`). It is stdlib only,
   operator-local, and uses no test framework and no CI step. No Python test file exists under `scripts/`, and
   `ci.yml` has no `python` token. A Rust test binary shelling out to the script would put the instrument under a CI
   step, against architecture's "Operator instruments" row ("none is invoked by any CI step"). That is an escalation
   class, not a routine choice.

## The timeout roster — the per-item half, and its constraint
- The fifteen timeouts of `2026-09-14-emit-scrubber-and-percentile-math-under-test` are recorded in its
  `evidence/disposition-ledger.md` ("## Timeouts — fifteen, classified by class, none rostered", `:87-110`) by HELPER
  and SHAPE only — `skip_absolute_path` 6 · `skip_line_number_suffix` 4 · `skip_hex_address` 4 · `normalize_frame` 1
  (`:97-102`) — never as exact `(file, mutation)` strings. VERIFIED at take-up.
- The raw `timeout.txt` that run wrote is GONE: `target/mutation-gate/` does not exist on the host, and the only
  surviving `mutants.out*` in the repo is `mutants.out.old/` (2026-08-20, conductor-run, an EMPTY `timeout.txt`,
  git-ignored at `.gitignore:63`). So the exact keys cannot be copied from a measured artifact.
- Consequence under the founder ruling — VERIFIED and sharpened at P3: a timeout's identity is coordinate-free
  (`replace += with -= in {fn}`), and it COLLIDES within a helper. `skip_absolute_path` has three `+=` advances
  (`exception.rs:326/328/331`) against the ledger's "the two advances", and `skip_line_number_suffix` has four. So the
  ledger's shape table does not determine the multiset. The conductor-emit timeout rows can only be (a) reconstructed
  as a hypothesis the NEXT audit's run confirms or refutes, (b) left unrostered, with the unit failing closed at the
  audit whose measured `timeout.txt` seeds them, or (c) enumerated as candidates by a non-testing `cargo mutants
  --list`, which still cannot say which ones time out. This is a fork for P4, not a lean.
- Timeout membership is contention-sensitive exactly as `missed` already is: run 1's sixteenth timeout became
  `unviable` in run 2 on LNK1104 link contention (ledger `:104-106`). An exact timeout multiset extends the existing
  residual (a rostered member lost to contention reads as absent) to a second tally. It is not a new flakiness class.
- The per-item classification itself (hang = detection; synchronous loops with no await to bound) is already
  dispositioned in that ledger and cited from test-plan; this chunk ROSTERS, it does not re-judge.

## Decided at P4 (validation-1: intent-incomplete, amended here)
- **Timeout rows:** the chunk ships the timeout CLASS with NO conductor-emit rows. The entry's "rostered rather than
  held only in a chunk's own disposition ledger" is met for the class (schema, grading, fixture proof). For the
  fifteen members it is met at the epoch-boundary code audit, which the overseer (founder-delegated, 2026-09-30) named
  the owner of seeding them from a measured `timeout.txt`. Measuring needs `cargo mutants`, which the founder allows
  only at that audit.
- **Proof harness:** an in-script `selftest` verb over committed fixtures (overseer, founder-delegated). No CI crossing.

## "Every tally it rests on" — the boundary
- `caught.txt` is read today but only PRINTED (`:142`); it grades nothing. The title's "every tally" is met by
  CONSERVATION — VERIFIED at P3: at cargo-mutants 27.1.0 `outcomes.json` carries top-level `total_mutants · missed ·
  caught · timeout · unviable · success`. On `mutants.out.old/` each tally file's line count equals its count
  (21 · 29 · 0 · 18), and `missed + caught + timeout + unviable = 68 = total_mutants` (`success` is the baseline,
  outside the sum). So a truncated or missing tally file, or a mutant absent from every tally, is gradeable against
  the tool's own numbers with no roster.
- `unviable.txt` is NOT graded as an expected set: the 2026-09-14 ledger measured it moving 1 → 3 on host link
  contention alone (`:39-49`), so an exact unviable roster would make the gate host-flaky (§10 zero-flakiness).
  VERIFIED: it enters the conservation sum and its own count equality only, never an equality against a roster.
- FINDING at P3, in scope: `complete()` prints `no outcomes.json under {out}` with `out` absolute (`mutation-gate.py:72`)
  — a host path on the not-readable FAIL path that a fixture arm exercises. It is made repo-relative here.

## Boundaries
- No `cargo mutants` invocation of any kind is planned as a test command or a step (founder ruling).
- No production Rust change; no change to test-plan §12's ratified classes. The roster file's own header names §12 as
  the OWNER of accepted-deliberate classes; a timeout class is a NEW class in the executable form, so its §12 wording is
  a wrap amendment, not a phase edit.
- The gate stays an operator/local instrument — no CI step invokes the MUTATION RUN (test-plan §9 `:469`).
- Ordered before `Full-gate regression over the moved surfaces` (`:65`) deliberately, so that sweep runs over a
  repaired gate (entry CONTEXT 1).

## Folded freight (the entry's two CONTEXT blocks — route.py pins: 2 blocks on `:61`, 748 + 606 chars)
- CONTEXT 1: minted at the 2026-09-16 wrap from the armed-orphan halt carried by
  `2026-09-16-scenario-assertion-audit-gate` (CARRY 1), operator-dispositioned "mint an entry". Coordinates re-verified
  at take-up: `:137` / `:138` hold, and the only `timeout` token is the docstring. "No CI step invokes the gate and no
  other route entry named it" — the CI half re-checked at take-up: no `mutation-gate` hit under `.github/`.
- CONTEXT 2: reading `timeout.txt` is a small patch; ROSTERING the fifteen is a per-item disposition — "judgment, not a
  patch" — and the operator's 2026-09-16 correction kept it out of the audit-gate chunk. The judgment is already made
  — VERIFIED: the ledger classifies all fifteen as detection by hang (`:91-95`, "A hang IS detection"). What remains
  is the KEYING, which the missing raw tally constrains (above).

## CI at take-up (Setup 5a)
- `f33d6b7` (the last wrap commit; the only sha since the last flip): **verdict not yet available** — CI#36685658389
  in progress, checks 3/3, the oldest running check (`Rust gate`) at 347 s when read. Not folded, not read as green.

## Watch items
- The working tree carried three post-wrap bookkeeping files at take-up (the handoff's session-end stamp, a friction
  append, the wrap's evolve JSON) — expected-transient, not this chunk's delta.
