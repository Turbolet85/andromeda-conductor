# Operator relay: a 0-pending adaptation wrap on Conductor's 0.4.0 route — the route commit is red in the project's own gate (2026-10-09)

From the pc overseer. Facts are `measured 2026-10-09` in this repository at `d227e59` unless a line says otherwise.
Re-derive what you rely on.

## 1. Context

The route for 0.4.0 is committed at `d227e59` (`conductor-0.4.0/working-route.md`, 61 markerless entries in 8 epochs;
requirements `v4-01`…`v4-24`). No chunk has started. Its push run `37932450560` ended failure in the `Rust gate` job;
the two other jobs passed. The three runs before it on `build/conductor-0.3.0` (`97dea7f`, `355a678`, `42daf3e`)
concluded success.

measured, from that run's log and the source:
- one test failed of 672 run: `conductor-report::matrix_ledger_gate
  every_requirement_capability_has_exactly_one_matrix_entry`, panicking at
  `crates/conductor-report/tests/matrix_ledger_gate.rs:170` with "conductor-0.4.0/requirements.md declares no
  version-capability id — the gate would pass vacuously";
- that test's `requirement_ids` (`matrix_ledger_gate.rs:49-56`) reads a capability line only in the form
  `- **{id}** …` — its own comment: "a list item whose first token is the bolded id";
- `conductor-0.2.0/requirements.md` declares 32 ids and `conductor-0.3.0/requirements.md` 11, all in that bolded form;
  `conductor-0.4.0/requirements.md` declares its 24 as `- v4-NN · …`, the form the route letter writes
  (`andromeda-route/references/phase-A/dialogue.md:15`);
- the route's own contract allows both: "exactly one capability line opening `- {id} · ` or `- **{id}** · `"
  (`andromeda-route/references/verification-matrix-contract.md:35`), and the wrap's requirement-add writes the
  unbolded one (`andromeda-wrap-session/references/route-resolve.md:153`).

So the route's artifact is lawful by the pipeline's contract and red by this project's gate, which pins one of the
two forms. Nothing in the 24 requirements is wrong; their line form is.

## 2. Adaptation items — presented for DISPOSITION at route-resolve, never pre-placed

Show me the card before anything is written.

**A. The 24 capability lines of `conductor-0.4.0/requirements.md` take this project's declaring form** — `- **v4-NN** ·
…`, the form its two earlier versions use and its gate reads. Not a word of any requirement changes: the id's markup
only, verified by a byte comparison of each line with the two asterisk pairs removed. If the letters give a 0-pending
wrap no lawful way to touch an existing capability line even so, say where, and make it a separate commit of its own
beside the wrap rather than bending the wrap.

**B. The gate learns the contract's second form — owned by an entry, not done here.** A later adaptation wrap adds a
capability line in the unbolded form (the letter's template) and would turn this gate red again. Wanted: before any
such line can land, the matrix-ledger gate reads both forms the route contract allows, and still fails on a
requirements file that declares none. That is a source edit, so it is a chunk's work: for disposition — a `CARRY:` on
`Linux-only base CI` (the entry that names the gates Conductor's one Linux job keeps), or an entry of its own in
Foundation. hypothesis: `v4-07`, or no requirement id at all for a gate's own repair; the card says which.

**C. The red run gets its owner.** With A the next push reads green and the red on `d227e59` is explained and closed
by this wrap's own commit; the first `/andromeda-phase` reads the CI verdict of every commit since the last master
flip (phase letter, Setup 5a), so record it in the form the letters give for a red that a later commit repaired.

## 3. Anchor-only pinning

The first entry stays `Conductor's window retired`. Nothing here asks for an entry to move.

## 4. Not in this wrap

No source edit; no phase. Before the commit run the one failing test locally and print its result
(`cargo nextest run -p conductor-report --test matrix_ledger_gate`, or the project's own form of it), then stop and
print `git status --short` for my read of the tree. The branch is `build/conductor-0.4.0`; push after my word.
