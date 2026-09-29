# Adaptation record — the 2026-09-29T05-42-49-wrap 0-pending wrap

A 0-pending operator wrap opening the two upgrade doors `upgrade.py detect` named at `67e8cb1` (`upgrade v1.0 ·
3cfde0e0`: for setup 0 · awaiting a door 2 · noted 3 · INDETERMINATE 0). Direction: the overseer relay
`conductor-wrap-0pending-2026-09-29` (founder-delegated), each disposition answered in-session by the overseer.

## Items and dispositions

1. **`cargo clean` before Setup** (operator): exit 0. It removed 214 141 files and 89.0 GiB, and D: went from 75 GB to
   149 GB free. Nothing in this wrap compiles.
2. **Three severed sidecar entries restored** (relay step 1). Restored byte-identical from `00181df`. The relay's
   "lost" was corrected to DISPLACED, and the three orphan tails were removed on the operator's word. Detail:
   `consolidation-record.md` §Before the consolidation.
3. **U13, the sidecar consolidation:** done. It is the backfill of all seven sidecars (398 → 396 entries, 634 453 →
   505 853 B, off-form 0 everywhere). 7 of 9 Supersedes claims were PARTIAL and dropped (W178). Detail:
   `consolidation-record.md`.
4. **U08, the judgment-base supersession:** five seed rules of `seed-templates/playbook.md` held neither by pattern
   name nor by a `seed:` tag (`Boundary widening` is already held at `:124`). Each seed rule was shown verbatim beside
   its nearest project rules (`supersession-sidebyside.md`), and the operator named each fate:

   | seed rule | fate | nearest project rule(s) | basis |
   |---|---|---|---|
   | Sequencing deferral | **add** | `:22`, `:16` | `:22` is Foundation-scoped and carries a guard the seed lacks ("no actual leak"), so superseding it would retire that guard |
   | Not this chunk's drift | **add** | `:46`, `:31` | the seed brings the CARRY/residuals caution (0 hits in `:46`); `:46` keeps its mention-not-claim arm |
   | Registry over-reach | **add** | `:37` `:49` `:61` `:67` `:73` `:82` | the six stay as measured instances of the generic |
   | Accurate this-chunk addition | **supersede `:28`** | `:28`; `:25`, `:97` stand | the same class, missing the registration half and the general reversal guard; `:97` covers only a SUT-driven arch-decision reversal ratified at P4 |
   | External decay | **add** | `:91`, `:103` | the seed brings "it ALWAYS produces an owner" (0 hits in `:91`) and the general classes; `:91`/`:103` keep the supply-chain fork |

   Applied: the five seed rules were appended verbatim under `## Rules` in the seed's order, and `:28`'s `note:` was
   opened `SUPERSEDED 2026-09-29 (by the seed rule Accurate this-chunk addition) —` with the rest kept verbatim. The
   playbook went from 53 to 58 rules and 86 616 to 92 204 B. Every byte before `:28`'s note and after it is unchanged
   (asserted on read-back).
5. **Dev-tool line** (relay part 4, owed since 2026-09-24): the host `cargo-nextest` reads `0.9.146 (8af696ddc
   2026-09-21)`, measured 2026-09-29 in this session. The last records named 0.9.133.

## Not this wrap

These are the relay's items, left untouched:
- The `stop-everything-you-start` memory vs `.claude/rules/verification-harness.md:58` conflict, and the auto-memory
  drain. Both belong to the epoch boundary, with the founder's word.
- U04 (`host-win32.md` 32 template lines behind). It is `noted`; regeneration happens only when the operator names it.
- No route adaptation. `:52` stays BLOCKED-ON Pulse's P-025 release.
