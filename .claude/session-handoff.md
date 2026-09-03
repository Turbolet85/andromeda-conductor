# Session Handoff

**Last Updated:** 2026-09-03T12:35:28Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **48 ahead** after this commit)
**Status:** clean
**Last Commit:** `feat(2026-09-03-conductor-run-composition-root-survivors-dispositioned): the roster that was
over-broad by two, and the three sites no hermetic test can reach`

## Position
- Done: **2026-09-03-conductor-run-composition-root-survivors-dispositioned** — the crate's standing survivor
  ledger closed: **25 → 8**, 17 killed, 8 accepted-deliberate against cited rules. Production source
  **byte-unchanged**; `Cargo.lock` un-drifted at 564 packages.
- Next: **`/andromeda-phase`** to promote + plan **_Live-Pulse preconditions probed before a leg is
  scheduled_** — the next markerless entry. It carries the **standing cargo-audit PREREQ, now the 48th**.
- Coverage **26/32 verified · 6 unclaimed** (`v2-04`, `v2-21`, `v2-24`, `v2-26`, `v2-27`, `v2-32`) —
  unchanged; this chunk claimed nothing, so the coverage gate was a no-op.

## Work done
The tier was re-measured at HEAD **before** planning, not after: `Found 118 mutants` both runs, which is what
makes the two scores comparable. **25 missed / 65 caught → 8 missed / 82 caught**, 28 unviable, **0 timeout**
either way — so unlike the sibling crate, every remedy here was *strengthen the assertion*, never *bound the
await*. 11 tests added across `lib.rs`'s `#[cfg(test)]` module and `tests/dispatch_wire.rs`.

**Two results the plan did not predict.** The operator's P4 ruling to re-open `observe_run_contract:346` paid
off — and the same test also killed `declares:358 → true`, because the env read lives in `declares` and not
in its caller, so the cited env-at-the-caller rule never covered either. **The ratified `declares` class was
over-broad by two: 6 → 4.**

**Three sites are accepted-deliberate for a measured reach barrier, not a coverage gap.** `:518`/`:559` ×2 sit
past `probe_egress`/`Dispatcher::connect` on the fixed `http://127.0.0.1:4317` const — reaching them means
binding the SUT's own port, contended between `--jobs 2` mutant processes. `:66` needs `preflight()` to hold a
connected client from a spawned `andromeda-pulse-mcp` absent on a hermetic host. That third one is a
**deviation from plan** (it was scoped as a kill) and took the plan's own sanctioned exit.

## Drift resolved
**4 amendments to test-plan · 0 escalations.** Six of seven doc-agents returned `proposals: []`, each with a
cited basis rather than a bare null.
- **§12** — the `declares` bullet's `x6` literal retired for the enumerated arm SET, with the 2026-09-03
  shrink and its cause recorded; a **new bullet** adds `conductor-run`'s accepted-deliberate classes B and C
  with their citations and evidence home.
- **§10** — the inline roster reference de-counted and **re-attributed `conductor-verify` → `conductor-run`**
  (`fn declares` has never been in conductor-verify; a workspace grep returns one hit).
- Sweep re-derived by the orchestrator, not accepted from the detector: 3 `declares` hits in test-plan, 2
  retired-count hits, **zero hits outside test-plan** across all seven masters, the three preserve-verbatim
  curation homes and the two judgment bases. `security-plan.md:114` already had it right.
- **Cascade: no leaf re-derivation needed.** `.claude/rules/testing.md:19` and `.claude/docs/tests-summary.md:12`
  name the roster as "a SET recorded in test-plan §12" and carry no count — consistent by construction. That
  set-naming shape is exactly what made them immune.

## Notes
- **Operator directive item 1 applied.** The evidence ledger claimed reaching `:518`/`:559` would collide with
  this crate's occupier guard test; that test passes `occupier_port = 0` and binds an **ephemeral** port,
  never `:4317`. Verified at HEAD, corrected in the ledger; the barrier now rests on its two real legs.
  `plan.md` retains the original clause as the record of what was planned, superseded by the report.
- **Operator directive item 2 applied at P5**, owner validated rather than assumed: a **CARRY** on
  _Operator-gated live suite_ (6b) naming the three live-path observables — `degraded == true` on a
  declare-only empty read-back, `latency_ms == observed − emitted` on the manual path, and the
  `preflight blocked` line under a live sidecar. Landing them does **not** retire the tier acceptances; it
  buys the behavioural coverage the accepted class cannot.
- **Curation:** Tier 2 × 2 into `testing.md` (33 → 34). One is an **in-place extension** of the 2026-08-20
  killing-assertion entry with the facet it lacked — *reachability before observability*, three instances in
  one chunk. The other records that an accepted-deliberate classification covers only the code the cited
  rule's mechanism lives in and does **not** inherit up to the enclosing function. Two candidates rejected as
  duplicates of the standing verify-the-artifact family; one rejected below threshold. CLAUDE.md untouched at
  **133/200**.
- **Self-inflicted, recorded:** my route-annotation append landed **after** the line's carriage return,
  embedding a mid-line CR and rewriting every line ending — `git diff` showed 119/119 for a 2-line edit.
  Caught by checking diff size against intent, repaired to the intended 3/3. The phase's *prepend* of the same
  helper round-trips clean; only the append direction has the trap.
- **Process hygiene:** re-measured against the host process list by name — **zero stragglers**. No listener
  opened; no Pulse, WebDriver or screen-reader process involved.
- **Disk residue for you (not processes):** `target/mutants-run-2026-09-03/` and `target/mutants-impl-2026-09-03/`
  (both gitignored under `/target/`), plus the prior session's `fresh-target-0903` (~3.0 GB) still in the
  scratchpad.
- **Last failed command:** none.
