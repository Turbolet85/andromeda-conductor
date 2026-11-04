# Cascade dispositions — 2026-11-03-architecture-registry-check-retired

## Step 2 — the sweep
Search: `cascade.py sweep --patterns-file .andromeda/runs/2026-11-03T23-24-25-wrap/apply-architecture.md --patterns-file .andromeda/runs/2026-11-03T23-24-25-wrap/cascade-patterns.toml` (trail `cascade-2026-11-03-architecture-registry-check-retired.json`). Three patterns:
- `arch-01` (fixed «sizes §Established Decisions / §Occupied Resources against 60 % of the Read cap»)
- `arch-02` (fixed «proves a drafted compaction lossless») — the retired clause's own verbs, from the apply writer
- `main-arch-reg` (regex `arch-registry-check(\.py)?|arch-registry|D-arch-registry-size|det-i5onq3b3|registry-size`) — the instrument's name tokens, main's

Each control fired at `architecture.md:36` (the baseline). Every master body and key file was swept.

Listing: `total (3 patterns) · 4 rows over 3 files` · `per class · new 0/0 · standing 0/0 · leaf 1/1 · curation 2/1 · base 1/1`.

- `arch-01` · `arch-02` · 0 rows: the retired clause stands nowhere else in the seven masters or their key files after the landing.
- `main-arch-reg` · standing 0: no master site remains (pre-pass: 1, `fct-22zuwdf4`, amended at tx 110).
- `.claude/docs/stack.md:40` leaf: re-derived at step 3 (it mirrors the amended Operator instruments row).
- `.claude/docs/session-learnings.md:411` and `:417` curation: no change. These are dated Tier-3 learnings (the 2026-09-24 instrument, its `D-arch-registry-size` remedy) and stay history, preserve-verbatim; routed to P3, which leaves them.
- `rows:rule:337` base (`rul-aepmkfvj`): no change. It names the script only as the dated first occurrence of the instrument class it governs (scope §What it builds, premise-corrected). No propose→approve→append is owed.

Main's three sidecar ratification entries (tokio) change no body, so they carry no pattern.

## Leaves
Set: `cascade.py leaves --add .claude/docs/stack.md` → `total · pass · 1 group(s) · 7 leaves · 49190 B · unmapped 1`.
- group `architecture` · pass mode (no epoch close: Epoch 1 holds 4 markerless entries, the `epoch` view) · 7 leaves (`commands.md`, `conventions.md`, `gotchas.md`, `stack.md`, `workflow.md`, `rules/frontend.md`, `CLAUDE.md`) · findings 1 · applied 1 (`.claude/docs/stack.md:40#1` · retired — the architecture-registry check's clause leaves the Operator instruments line) · by hand 0 · declined 0. Tool: `total · blocks 1 · applied 1`, `wrote .claude/docs/stack.md: 1 finding(s), read back`.
- unmapped `.claude/rules/host-linux.md` (host): no source states the host leaf, so no comparer and no change.
- Not re-derived by this pass (pass mode reads the changed source's delta): the leaves' standing pointers to `.andromeda/architecture.md` (`conventions.md:3/9/54/55`, `gotchas.md:3/14…72`, `stack.md:3/53/56`, `workflow.md:15/40`, `commands.md:3`, `CLAUDE.md:101`), a file this chunk removed. They predate the delta and are carried to the operator's card as the «old `.andromeda/{doc}.md` reach» (report).
