# Route — P5 edits, 2026-11-03-linux-only-base-ci wrap

Directions: the mint and its placement — the overseer's wrap directive (founder-delegated; relayed by overseer1 from the
founder's 2026-10-17 ruling), `chunks/2026-11-03-linux-only-base-ci/inputs/I12-relay-3.md.txt` and
`chunks/2026-11-03-linux-only-base-ci/inputs/I1-conductor-linux-only-base-ci-2026-11-03.md.txt` item 4; the
`spawn.rs` pin — the P5 card's Beyond-the-plan row, delegate overseer, PROVISIONAL,
`chunks/2026-11-03-linux-only-base-ci/inputs/I5-relay-4.md.txt`.

## Edits
@chunk after=chk-kmy6lgqr as=n1
Architecture-registry check retired — the architecture size-gate script leaves with every test, CI step and record naming it; the product row drops the architecture pair

@carry in=@n1
CARRY: the founder's ruling, item 6 of `../additional/andromedaV3:andromeda-overseer/docs/design-slots/masters-classify-pilot-2026-10-15/rulings.md` (lines 38-39) — Conductor's own `scripts/arch-registry-check.py` size gate is retired by a Conductor chunk; the record `chunks/2026-11-03-linux-only-base-ci/inputs/I14-conductor-owed-arch-registry-check-retired-2026-10-17.md.txt` (no CI, test or script caller, measured 2026-10-17). The entry removes the script with every test, CI step or record naming it, drops `.andromeda/architecture.md` and `.andromeda/architecture-amendments.md` from `.andromeda/db/moved`'s `product` row, and runs the masters' own `rec.py render --family masters --reconcile --product none` (`chunks/2026-11-03-linux-only-base-ci/inputs/I1-conductor-linux-only-base-ci-2026-11-03.md.txt`, the appended block).

@carry in=@n1
CARRY: measured at 2026-11-03-linux-only-base-ci (report Deviations, step 9) — `rec.py render --family spine --reconcile --product none` re-rendered six product paths the chunk had already `git rm`'d before dropping them (`healed from the journal` ×6), its `--dry-run` reading `every render equals the journal`; and `git rm` refused a retired product file carrying an uncommitted store render, `-f` taken only after the store's record compared byte-equal. Read `git status` after the masters reconcile and around the architecture pair's `git rm`.

@carry in=chk-tl6z7q7l
CARRY: `crates/conductor-verify/src/spawn.rs`'s `#[cfg(windows)]` arms — `build_command`'s `console_suppressing_flags()` and the unit test `the_sidecar_spawn_suppresses_the_child_console` — compile on no leg since 2026-11-03-linux-only-base-ci moved CI to `ubuntu-latest`; they leave with this entry (the P5 card's Beyond-the-plan row, delegate overseer, PROVISIONAL: `chunks/2026-11-03-linux-only-base-ci/inputs/I5-relay-4.md.txt`).
