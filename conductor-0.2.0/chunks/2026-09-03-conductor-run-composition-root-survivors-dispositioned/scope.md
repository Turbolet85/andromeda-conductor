# Scope — conductor-run composition-root survivors dispositioned

**Marker:** `2026-09-03-conductor-run-composition-root-survivors-dispositioned`
**Version:** conductor-0.2.0 · **Epoch:** 6a — Verification follow-ups
**Working entry (verbatim intent):** _conductor-run composition-root survivors dispositioned — every standing
mutation survivor gains a killing test or a cited accepted-deliberate entry_

---

## Goal

Close `conductor-run`'s standing mutation-survivor ledger the way `2026-09-03-conductor-tauri-survivors-dispositioned`
closed `conductor-tauri`'s: **every named survivor ends either killed by a test or classified
accepted-deliberate against a cited standing rule.** The score is a consequence of that dispositioning, never
the acceptance (test-plan §10).

This is the sibling chunk of the one just completed — same shape, one crate over. The crate differs in two
ways that matter: `conductor-run` is the composition root (its survivors sit in live production paths, not in
a GUI shell's `main`), and six of its survivors already carry a ratified disposition, so the ledger starts
partially closed rather than empty.

## In scope

- **Re-measure the tier for `conductor-run` at HEAD** and treat that measurement — not the 2026-09-02 audit
  table — as the authoritative survivor set. The audit's table is a hypothesis until re-run (see §Inferred
  premises); the sibling chunk's comparability rested precisely on both runs finding the same mutant count.
- **Disposition every standing survivor the re-measurement names**, by one of exactly two exits:
  - a **killing test**, or
  - an **accepted-deliberate** classification citing a standing rule, extending the roster recorded in
    test-plan §12 (per test-plan §10, a new accepted disposition EXTENDS the roster rather than contradicting
    a single-case literal).
- **Carry the ratified `declares` survivors forward — with one member re-opened by operator ruling.** They are
  a real disposition, not debt; the five `declares` mutants proper (`:358` ×2, `:360` ×3) are re-asserted
  against their citation, not re-litigated. **Amended at P4 (operator-ratified):** the sixth member,
  `observe_run_contract:346`, is re-opened for a kill attempt — P3 found the env read lives in `declares`, so
  the cited env-at-the-caller rule does not obviously cover the enclosing function's own mutant. If a kill
  lands with no `unsafe` and no process-env write, the roster shrinks 6 → 5; if none is reachable, it stays
  accepted with its citation re-asserted. This supersedes this scope's original "carry all six as-is".
- **Read out the tier by the tallies, never the exit code** — `missed.txt` holding EXACTLY this run's
  accepted-deliberate survivors, every other named survivor present in `caught.txt` (test-plan §4, as
  reconciled at the sibling chunk's wrap; `.claude/rules/testing.md:19`).
- **Discharge the standing PREREQ** (folded below).

## Out of scope (boundaries)

- **Splitting `lib.rs` along the cluster seam.** The source proposal offers two directions and states plainly
  that the disposition half "is the cheaper one" and "does not depend on" the split. This chunk is direction
  (b) only. Whether the file itself should split is research territory and, if it earns a chunk, a separate
  one. Do not restructure the composition root to make a mutant convenient to kill.
- **Production-code behaviour change.** The sibling chunk landed with production code byte-unchanged; that is
  the expected shape here too, with one admitted exception class: where a survivor is a genuine
  *observability* gap (the sibling's three timeouts were an unbounded `rx.await`), bounding or tightening the
  shipped construct is the correct remedy rather than writing a test around it. Any such edit is named
  explicitly and justified in the plan, never taken silently.
- **A numeric mutation threshold.** `--fail-under` is deliberately not the gate (test-plan §10).
- **Workspace-wide mutation.** The instrument runs against the crate this chunk touches, not the workspace
  (test-plan §9 / §464).
- **`conductor-verify` and `conductor-tauri` survivors.** Already closed by their own chunks.

## Surfaces and contracts touched

- `crates/conductor-run/src/lib.rs` (the composition root — 1530 raw lines, verified at HEAD) and
  `crates/conductor-run/src/dispatch.rs` (one survivor row).
- `crates/conductor-run/tests/` and the crate's in-file `#[cfg(test)]` modules — the expected home of the
  killing tests.
- **Read-only:** test-plan §4 (mutation instrument + read-out discipline) · §10 (disposition rule + roster) ·
  §12 (the roster's recorded entries) · `.claude/rules/testing.md:19` (the procedural accept-with-a-cited-rule
  rule) and `:67` (the 2026-08-10 env-at-the-caller rule that grounds the `declares` acceptance).
- **No new dependency edge is anticipated.** Any dev-dependency addition moves `Cargo.lock` and must be
  reported as a package-count delta, never as "byte-unchanged" (`.claude/rules/security.md`, 2026-09-02).

## Folded freight

### CONTEXT (operator WRAP directive 2026-09-02, item 2) — coordinates re-verified at HEAD

Each named coordinate was resolved against its artifact before it shaped this scope:

- `.andromeda/runs/2026-09-02T15-49-17-code-audit/proposals.md:78-154` — **exists**, and §M2 at `:78` is
  exactly the `monotonic` · `sizes.file_max` finding claimed. ✓
- `crates/conductor-run/src/lib.rs` — **1530 raw lines at HEAD**, matching the cited raw endpoint. ✓
- `.claude/rules/testing.md:19` — resolves to the mutation bullet, i.e. the rule that *requires* a
  disposition. ✓ **Nuance recorded:** the *substantive* ground for the `declares` acceptance is `:67` (the
  2026-08-10 env-at-the-caller rule), which is what test-plan §12 cites. Both coordinates are live; they
  answer different questions (`:19` = must be dispositioned, `:67` = why this one is accepted).
- `fn declares` — **`crates/conductor-run/src/lib.rs:357`** at HEAD, inside `observe_run_contract`'s
  filter. ✓ **Observation for wrap, not for this chunk to fix:** `test-plan.md:500` attributes the `declares`
  ×6 edge to `conductor-verify`; it is in `conductor-run`. A candidate amendment, surfaced here, not amended
  by phase (phase is read-only on the specs).

### PREREQ (standing, external decay) — the 47th cargo-audit re-check

Re-check `cargo audit`. Standing deferral since 2026-08-09, ratified at the 2026-08-10 wrap. Basis: the
RustSec advisory DB itself will not parse — a **database** fault, not a tool fault, so no floor raise exists
to make. Overlap: `cargo deny check advisories bans licenses sources` runs green every chunk.

**Pinned signature:** `cargo audit` exit **1**, first diagnostic
`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`; overlap
`cargo deny` exit **0**. Reproduced byte-identically at the 2026-09-03 sibling wrap (the 46th); **this is the
47th**. Read the exit **directly, never through a pipe** (`.claude/rules/host-win32.md` — a piped `$?`
reports the pipeline's last stage and has measurably masked a red gate). **Any deviation restores the FULL
form and is reported.**

## Premise closure — resolved at P3 by measurement (2026-09-03)

The tier was re-measured at HEAD before planning: **118 mutants tested in 12m — 25 missed, 65 caught, 28
unviable, 0 timeout** (`cargo mutants -f crates/conductor-run/src/lib.rs -f crates/conductor-run/src/dispatch.rs
--test-tool=nextest --jobs 2`; tallies read from the nested `mutants.out/` per `.claude/rules/testing.md`
2026-09-03). Every premise below is now VERIFIED or FALSIFIED against that run.

- **VERIFIED** — the standing survivor set is **25 rows**, composition-identical to the audit's 2026-09-02
  table: same sites, same mutation texts. The sibling chunk changed nothing in this crate.
- **VERIFIED** — **"24 of the 25 `conductor-run` mutation survivors live in it"**; the 25th is
  `dispatch.rs:163:26`.
- **VERIFIED, and the "6 vs 5" grain question is RESOLVED** — the ratified `declares` class of 6 is
  `observe_run_contract:346` **plus** the five `declares` mutants (`:358` ×2, `:360` ×3). The enclosing
  function is grouped with its body because both survive for the same env-at-the-caller reason. `declares`
  itself has exactly 5 mutants, which is what the table showed.
- **VERIFIED** — the cluster arithmetic is exact: **preflight/contract (rows 7-18) = 12** (`observe_run_contract`
  1 + `declares` 5 + `warm_up_canary_service` 6); **run (rows 19-24) = 6** (`execute_scenario` 3 +
  `FaultKind::label` 2 + `fault_span` 1).
- **VERIFIED** — `conductor-run` has a runnable mutation tier at HEAD and needs **no build-graph repair**:
  118 mutants enumerate (95 `lib.rs` + 23 `dispatch.rs`, matching the audit's "planned 118"), the baseline
  suite is **156 tests / 156 passed / exit 0**, and `crates/conductor-tauri/ui/dist` is present so the
  workspace compile the tier's baseline performs succeeds. This is the one place the crate differs sharply
  from its sibling, which could not score at all until its build graph was fixed.
- **FALSIFIED** `[premise-corrected: lib.rs test lines grew +94% against production's +72% over 2026-08-16 →
  2026-09-02 (297→575 vs 555→955); the test/production ratio ROSE 0.535 → 0.602]` — the audit's
  hypothesis-marked **"Suspected shape"** that `lib.rs` "grows every epoch while its own test coverage does
  not follow" is not supported. Test volume more than kept pace. The OBSERVATION it was offered to explain
  (survivor concentration in this file) stands and is re-measured above; only the stated MECHANISM fails.
  Consequence for the plan: this is **not** a "catch coverage up with growth" chunk — each survivor needs its
  own cause read at its own site. (Line share is a coarse proxy for coverage; it is sufficient to falsify a
  claim stated in terms of growth keeping pace, which is how the audit stated it.)
- **PARTLY FALSIFIED** `[premise-corrected: 0 timeouts measured at HEAD]` — of the sibling's cheaper-than-planned
  mechanisms, the **timeout/unbounded-await remedy does not transfer**: this crate has no timeout survivors, so
  every one of its 25 is a MISSED, i.e. the harness observed the mutant and shrugged. Per
  `.claude/rules/testing.md` 2026-09-03 that means the remedy class here is **strengthen the assertion**, never
  **bound the await**. The general lesson the sibling generalised — survivor causes differ per site and must be
  read individually — is carried forward unchanged.
- **NOT RE-DERIVED, and not leaned on** — the audit's **"the epoch killed ZERO standing survivors while the
  score rose 70.59 → 72.22 on a denominator grown by 26 mutants"** and its causal reading **"The score improved
  because the *denominator* grew"** describe the E4→E5 transition, not HEAD. This chunk's acceptance rests on
  the measured 25-row set, not on any score movement, so the claim is recorded as the audit's and left
  unverified rather than restated as this chunk's own.

**Disposition status of the measured 25:** 6 already ratified accepted-deliberate (rows 7-12), leaving
**19 with no recorded disposition** (rows 1-6, 13-25). **After the P4 operator ruling above, this chunk's
in-scope set is 20** — those 19 plus `observe_run_contract:346` — and **5** are carried as accepted-deliberate
without re-examination.

## Acceptance shape (concretized at P4/P5, recorded here as intent)

Every survivor the re-measurement names is, at wrap: **killed** (present in `caught.txt`) or
**accepted-deliberate** against a cited rule (present in `missed.txt`, and `missed.txt` holds exactly that
set). The roster in test-plan §12 is extended, not contradicted. The PREREQ is discharged with its signature
matched or its deviation reported.
