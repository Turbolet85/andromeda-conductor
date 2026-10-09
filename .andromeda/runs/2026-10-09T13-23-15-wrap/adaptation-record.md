# Adaptation record — 0-pending wrap, 2026-10-09T13-23-15

No chunk wrapped. Setup read 0 `pending` and 0 `gated` records in `.andromeda/master-route.md` through
`route.py cursor` (165 records, 165 `complete`). `HEAD` was `d227e59`, 0 ahead of
`origin/build/conductor-0.4.0`. The tree held only expected-transient bookkeeping at entry: the handoff's
session-end stamp and the friction ledger's trailing append.

## The request

An operator-requested adaptation from the pc overseer: `conductor-wrap-0pending-adapt-ledger-form-2026-10-09.md`,
copied byte for byte into this run dir as `relay-1.md` (`cmp` exit 0; sha256
`6026cac8c14050fc2c2d32b73a0019c5413641a981d8e9f1fb636ae0e5b5f783`, 4 152 B). Not snapshotted through
`inputs.py snap`: that call takes a chunk dir, and this path has none.

The card was shown before anything was written. The operator's word on it, verbatim:

> Operator word on the card: write it. 1 A as a separate commit, on my word - the pc overseer as operator,
> founder-delegated, not the founder own word; record it as an exception to the 2026-08-10 rule with its reason: the
> file is put into this project declaring form on the day it was born, before any chunk or ledger note rests on it, id
> markup only, byte-proof kept in the record. 2 B as the CARRY on Linux-only base CI, no id, as you worded it. 3 One
> stop, two commits. You are right about route-resolve.md:153 - my relay misread it; say so in the record. Run the
> ledger gate, print it and git status --short, and stop before the commits.

Authority, in the form of `gate-contract.md` §Scope, *Whose word*: the pc overseer as operator, founder-delegated,
2026-10-09, given in this conversation. It is a delegate word, not the founder's own; by that rule the founder's own
later word supersedes it.

## The relay's facts, re-derived

Each fact the dispositions rest on was read at its source before the card.

- **The red run.** Push run `37932450560` on `d227e59`: conclusion `failure`; the Rust gate job failed, the frontend
  and a11y jobs passed. Read from that job's own log: `672/1242 tests run: 671 passed, 1 failed` (the `ci` profile
  stops at the first failure), the failing test
  `conductor-report::matrix_ledger_gate every_requirement_capability_has_exactly_one_matrix_entry`, its message
  "conductor-0.4.0/requirements.md declares no version-capability id — the gate would pass vacuously".
- **The runs before it.** The seven push runs before it, all on `build/conductor-0.3.0`, concluded `success`
  (`97dea7f`, `355a678`, `42daf3e` among them).
- **The gate's reading.** `crates/conductor-report/tests/matrix_ledger_gate.rs:49-56`: `requirement_ids` takes a line
  only when it opens `- **`, and its discrimination arm asserts "only the bolded list-item declaring form counts".
- **The line forms.** `conductor-0.2.0/requirements.md` 32 bolded, 0 bare; `conductor-0.3.0/requirements.md` 11
  bolded, 0 bare; `conductor-0.4.0/requirements.md` 0 bolded, 24 bare, as committed at `d227e59`.
- **The route letter's template.** `andromeda-route/references/phase-A/dialogue.md:15` writes
  `- {id} · {title} — …`, the bare form.
- **The contract.** `verification-matrix-contract.md:35` allows both: "exactly one capability line opening
  `- {id} · ` or `- **{id}** · `". Read in the wrap skill's copy, which the contract says is byte-identical to the
  route skill's; the route skill's copy was not opened.
- **Local reproduction, before any edit.** `cargo nextest run -p conductor-report --test matrix_ledger_gate
  --no-fail-fast`: exit 100, `4 tests run: 3 passed, 1 failed`, the same test and the same message.

**One fact of the relay did not hold, and the operator confirmed the correction.** The relay's §1 says the wrap's
requirement-add "writes the unbolded one (`andromeda-wrap-session/references/route-resolve.md:153`)". The letter at
that line writes the capability line "in the file's own form", and the ledger tool's `add --dry-run` prints how many
lines of each form the file holds. So once item A stands, a later add in this version writes the bolded form. What
still writes the bare form is the next version's route run, through the template above. The operator's word: "You are
right about route-resolve.md:153 - my relay misread it; say so in the record."

## Items and dispositions

### A. The 24 capability lines of `conductor-0.4.0/requirements.md` take the bolded declaring form — APPLIED, outside the wrap, as a commit of its own

**Why not through the wrap.** Three places forbid it:

- the wrap letter's constraints: no write to `requirements.md` "beyond the one capability line route-resolve
  §Operator-requested adaptation adds";
- `route-resolve.md:157-160`: "no other byte of the file moves: an existing line, an id, the header and the headings
  are not this path's";
- this project's rule of 2026-08-10 (`CLAUDE.md`, session learnings; full text in
  `.claude/docs/session-learnings.md`): a version's `requirements.md` is immutable and no skill ever writes it.

**The exception.** The edit is a recorded exception to the 2026-08-10 rule, given on the operator's delegate word
above. Its reason, in the operator's words: "the file is put into this project declaring form on the day it was born,
before any chunk or ledger note rests on it, id markup only, byte-proof kept in the record." No skill wrote the file:
the edit was made by hand outside the wrap's phases and is staged alone in its own commit, beneath the wrap's.

**The edit.** 24 anchored edits, each `- v4-NN · ` to `- **v4-NN** · `. Not a word of any requirement changed.

**The byte proof**, the committed blob (`git show HEAD:conductor-0.4.0/requirements.md`) against the working tree:

- sha256 `c3acafab41a42d4fe0c9b5dcb1a2757e72422b9fce298c183eb937f208447b7c` →
  `7abbe98f68efdcb30d26ad7870e97812fe0e565dd44ccf385b26a86a476017b2`;
- 19 359 B → 19 455 B, +96 B, which is 24 lines × two asterisk pairs × 2 B;
- 132 lines → 132 lines; no carriage return; `git ls-files --eol` reads `i/lf w/lf`;
- 24 lines differ, lines 15-22, 26-29, 33-36, 40-43, 47-49 and 53; `git diff --numstat` reads `24 24`;
- each changed line opens `- **v4-NN** · `, carries exactly two asterisk pairs where the committed line carried
  none, and equals the committed line byte for byte once those two pairs are removed;
- the ids read `v4-01` … `v4-24`, 24 of them, 24 distinct;
- the whole file equals the committed file byte for byte with every asterisk pair removed (the committed file held
  none);
- after the edit: 24 bolded capability lines, 0 bare.

**The gate, after the edit.** The same command: exit 0, `4 tests run: 4 passed, 0 skipped`, taken at
2026-10-09T13:26Z.

**What else reads the file.** A search for `requirements.md` over `crates/`, `scripts/` and `.github/` (Rust, shell,
PowerShell, Python, YAML and TOML files) returned one file, `matrix_ledger_gate.rs`. No md5 or sha256 of the
committed file is written anywhere under `.andromeda/`, `conductor-0.4.0/`, `crates/`, `contracts/` or `scripts/`.
The ledger tool reads both forms (`matrix.py`, the form count its `add` prints).

**Commit.** `fix(route): conductor 0.4.0 capability ids take the project's declaring form — id markup only`, the
file alone.

### B. The matrix-ledger gate learns the contract's second form — PINNED, a `CARRY:` on `Linux-only base CI`, no requirement id

The operator's direction named both the entry and the disposition, which satisfies the trajectory gate
(`route-resolve.md` §Edits + gradient).

- **Slot:** `conductor-0.4.0/working-route.md:15`, the entry `Linux-only base CI`, markerless. The annotation follows
  the entry's own text after two spaces.
- **Text, as the card worded it and the operator approved it:**

  `CARRY: the matrix-ledger gate reads both capability-line forms the route contract allows, the bolded id and the
  bare id, and still fails on a requirements file that declares none (measured:
  crates/conductor-report/tests/matrix_ledger_gate.rs:49-56 reads the bolded form only and its discrimination arm
  pins that; the contract is verification-matrix-contract.md:35; origin: push run 37932450560 red on d227e59; no
  requirement id, a gate's own repair; per the operator's direction relayed by the pc overseer, 2026-10-09)`

  (wrapped here for reading; on the route it is one line).
- **Why no requirement id:** `v4-07` says the other operating systems are gone; it says nothing of the ledger gate,
  and a gate's own repair proves no requirement.
- **Why not an entry of its own:** all 61 route entries end in a `(v4-NN)` tail, and this would be the only one
  without; it would also add a chunk to Foundation.
- **Why that entry is early enough:** it is the third entry, and after item A nothing a wrap lawfully writes puts a
  bare capability line into this version's file before it.
- **Read-back:** `route.py pins` lists one freight block on the tail, line 15, introducer `CARRY:`, 518 characters;
  `route.py cursor` still reads 61 of 61 entries markerless and the next entry `Conductor's window retired` at line
  11; the diff of the file is one line.

### C. The red run gets its owner — RECORDED

The wrap and phase letters give no form of their own for a red that a later commit repaired; a search of both for one
found none. What they give is phase's Setup step 5a: the first `/andromeda-phase` reads the CI verdict of every
commit from the last master flip (`355a678`) through `HEAD`, so it will read `d227e59` red whatever is written here,
and it carries that red into its Phase 1, where the chunk's `scope.md` dispositions it.

So the red is recorded where that read will meet it:

- here;
- in the handoff's Notes, with the run id, the sha, the failing test, the cause and the repairing commit named by its
  subject;
- in the wrap commit's body.

Its two owners: the red itself is repaired by item A's commit; the gate's narrowness that made a lawful route
artifact red is owned by item B's `CARRY:`.

**Not measured at this record's writing:** the CI verdict of the push that carries the repair. The repair is measured
green on the dev host only. The push waits for the operator's word.

## Not done, as directed

- No source edit. The gate's own repair is a chunk's work (item B).
- No phase. The first entry is still `Conductor's window retired`; no entry moved.
- `intent.md` untouched. No word of any requirement changed.
- No master, sidecar, registry, residual or ledger write; `master-route.md` untouched.
- No commit and no push before the operator's read of the tree.

## Path duties

- Gated premises: none to re-verify (0 `gated` records).
- Curation: one Tier-3 entry extended in place, one candidate filtered; `curation.md` in this run dir.
- Bookkeeping: `state.yaml` (`last_wrap`, `session_count` 190 → 191) and the handoff.
