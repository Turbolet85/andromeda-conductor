# Materialization Plan — Conductor (setup-project re-run, the U04 host-leaf card)

**Run:** `2026-10-07T15-13-34-setup-project`
**Mode:** re-run (CLAUDE.md carries 20 `GENERATED:setup` / `USER` marker lines; 138 lines)
**Development Style:** `agent-driven` (arch §Cross-cutting Patterns :220, §Inherited Defaults :246)
**Origin of the run:** the overseer's direction — `upgrade v1.6` reads U04 `behind` (the host leaf was rendered
for win32, this host is linux; the founder ruled on 2026-10-07 that Windows is not a target host). The run is to
produce the Phase 7 card with its Host leaf row and STOP there: the card is the founder's approval, brought to him
by the overseer. Nothing beyond what the re-run itself applies is changed.
**Host:** linux (`upgrade.py host`).

Phases 1–6 read ONLY this file.

---

## 0. Upstream read basis (what each decision rests on)

All 9 upstreams present (806 KB together). **The read was STRUCTURAL and DIFFERENTIAL, not whole-prose** — the
shape the 2026-09-15 and 2026-10-06 runs recorded, narrowed by one measured fact: the previous setup run committed
at `29adafa` (2026-10-06), and this run read what moved since.

| Phase 0 output | Basis actually read this run |
|---|---|
| Development Style | arch :220 and :246, both `agent-driven` |
| What moved in the masters since `29adafa` | `git diff 29adafa HEAD` over the 9 upstreams, every changed hunk read as words: architecture :70, :184, :203-208 (§Established Decisions, §Occupied Resources) · security-plan :121, :338 · test-plan :124, :260, :391 · obs-plan :221. All are the 2026-10-07 series and capture-run records. `input.md`, `design-system.md`, `layout-templates.md`, `a11y-plan.md`: unchanged |
| Whether a consumed section moved | the heading index of architecture (§Cross-cutting Patterns :217-225 · §Project Intent :226 · §Inherited Defaults :234) and the five plans' anti-pattern sections — NO changed hunk falls in a section a Tier-1 block is rendered from |
| Tier-1 blocks | `git diff 29adafa HEAD -- CLAUDE.md`: every changed line sits inside `USER:session-learnings` (wrap's curation); the eight `GENERATED:setup:*` blocks are byte-identical to what the 2026-10-06 run verified against the masters and the template |
| Tier-1 template delta | `references/claude-md-template.md` last modified 2026-09-29 — before the 2026-10-06 run that read it whole. No delta |
| Module map | `Cargo.toml` `members` 9 — unchanged since `29adafa` |
| Key directories | `ls contracts/` 7 members; `contracts/pulse-real-model-leg-posture.md` changed in content only |
| Upgrade reading | `upgrade.py detect --root .` — §Upgrade below, verbatim |
| The host leaf | `.claude/rules/host-win32.md` read whole (145 lines) · `references/rules-templates/host.md` read whole (96 lines) · `curation-tier-decision.md` §Tier definitions and §Tiebreaker rules · the `paths:` frontmatter of all seven rule files · `upgrade.py host --root .` · `upgrade.py`'s `load_sort` / `reseed_u04` (the sort file's form and where a learnings entry lands) |
| Liveness of an old-host entry | measured, not argued: `.github/workflows/ci.yml` `runs-on` (3 Windows runners) and `shell:` (9 `pwsh`, 12 `bash`) · `git ls-files '*.ps1'` (6 outside run dirs) · a fixed-string count of each entry's tool tokens in `ci.yml` and those scripts · `grep -i` and `TMPDIR` / `TEMP` probed on this host |

**NOT read this run:** the masters' body prose outside the changed hunks and the heading indexes;
`master-route.md`'s records (the cursor is `route.py`'s); the U35 key files (no keyed contract changed — the one
obs-plan hunk is body prose at :221). No decision below rests on them. The skill's letter asks for every upstream
whole; this run, like its two predecessors, did not do that, and says so here rather than claiming it.

---

## 1. Tier 1 — CLAUDE.md

Current **138 lines**; projected **138**. **Phase 1 writes nothing.**

- `setup:overview` · `setup:modules` · `setup:warnings` (11 bullets) · `setup:pointer-table` (22 rows) ·
  `setup:workflow` · `setup:architecture` · `setup:imports` (U01 `ok`) — verified CURRENT on the basis in §0.
- `setup:deeper-topics` — line 111 names `host-win32.md`. It is re-rendered from the rule files that exist ONLY at
  Phase 7.5, after the card's "yes" and the re-seed. Before that word it stands.
- `USER:session-learnings` — preserved verbatim (wrap territory). One of its 17 bullets names the old host: the
  2026-08-22 entry (the twelfth), which opens "This host is Windows-only". Listed on the card; setup edits no
  `USER:*` bullet — its door is a 0-pending `/andromeda-wrap-session` asked for the correction.
- Backup taken: `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T15-13-34` (gitignored; md5
  `35215c25ca55424b9e6387a4df8a15aa`, equal to the live file).

No warning candidate was promoted or dropped (no audit list owed).

---

## 2. Tier 2 — `.claude/rules/` (7 files, all present)

| Leaf | Disposition |
|---|---|
| `security.md` | preserve |
| `host-win32.md` | **U04 `behind` — rendered for another host. NOT touched in Phase 2**; its re-seed is the card's Host leaf row and happens at Phase 7.5 on the founder's word |
| `testing.md` | preserve |
| `observability.md` | preserve |
| `a11y.md` | preserve |
| `frontend.md` | preserve |
| `verification-harness.md` | preserve — and the PROPOSED destination of one moved item (item 12), written only at Phase 7.5 |

No planned rule file is absent, so Phase 2 writes nothing. `host-linux.md` is absent, but the absent-leaf render
is not this case: a leaf rendered for another host exists, so the tool's re-seed writes the new one.

### The host leaf's sort (step 8a)

`upgrade.py host --root .`: 22851 B · 22 items below `## Session Additions` · `leaf_md5`
`b32a9ece05a83470dfe32e4fdf37d9af`. The sort is `host-reseed.json`; its readable twin is `host-reseed.md`.

| Disposition | Items | Bytes |
|---|---|---|
| `keep` (the new leaf, every turn) | 1 · 10 · 15 · 22 | 1212 |
| `learnings` (Tier 3) | 2 · 3 · 4 · 5 · 6 · 7 · 8 · 9 · 11 · 13 · 14 · 16 · 19 · 20 · 21 — 12 of them `mixed` | 15911 |
| `.claude/rules/verification-harness.md` | 12 | 755 |
| `drop` (to the run dir) | 17 · 18 | 560 |

**The reading of "drop" this sort uses, stated because it is a judgment:** step 8a's rule 2 drops an entry whose
every directive names only the old host's shell and tools. The repository still runs Windows tooling that is not
the dev host's: three CI jobs on Windows runners with nine `pwsh` steps, and six tracked `.ps1` scripts. An entry
whose tools a committed script or the workflow still carries (11, 12, 13, 14, 19) was therefore sorted as a live
lesson carrying an old-host clause (rule 3, or rule 1 for item 12), and only the two entries about the dev host's
own scratch work (17, 18) are dropped. The card names this so the founder can move any of them.

**Noticed, not acted on:** the old leaf's body section `## Long single-line files` (lines 58-62) is host-neutral
and is not in the template the linux leaf renders from. A body line is not an item; the re-seed moves it nowhere.
It survives in git history and the gitignored backup. Items 2 and 10 refer to it. A template matter — the
pipeline overseer's.

---

## 3. Tier 3 — `.claude/docs/` (11 files, all present)

| Leaf | Disposition |
|---|---|
| `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` | preserve |
| `workflow.md` | preserve — NOTED (U09): lacks `it never commits` |
| the five `*-summary.md` | preserve — NOTED (U36): the header line |
| `session-learnings.md` | preserve (wrap territory) — and the PROPOSED destination of 15 moved items (+133 lines, inserted above its first `## ` heading), written only at Phase 7.5 |

Phase 3 writes nothing.

---

## 4. Agent harness (Phase 4)

`scripts/agent-run.sh` + `scripts/agent-run.ps1` present. **PRESERVE** (only-if-missing); no backup owed.
NOTED (U10): `agent-run.sh` has no `ensure_fresh_artifacts` hook — the project's `ensure_frontend` fills that role.
`verification-harness.md` preserved per §2. Phase 4 writes nothing.

---

## 5. Code reviewer + hooks + .gitignore + .gitattributes (Phase 5)

- **Code reviewer:** present (rust) — preserve.
- **Hooks:** U02 `ok` (write guard current · Bash guard current · PostToolUse on the stdin prologue, rustfmt only).
  `hooks-matrix.md` unmodified since before the 2026-10-06 run. No write.
- **rustfmt.toml:** U05 `ok`. **.gitignore:** U07 `ok`. **.gitattributes:** U06 `ok`.

Phase 5 writes nothing.

---

## 6. Code-graph pipeline + operational artifacts (Phase 6)

U03 `ok` (the triple current). `state.yaml` · `session-handoff.md` · `drift-base.md` · `playbook.md` (U08 `ok`) ·
`session-learnings.md` · `.andromeda/cache/` all present. Phase 6 writes nothing.

---

## Upgrade

**Setup 5b record.** HEAD `920a1e7ef89403e096a2be0928de995d1553925b` on `build/conductor-0.3.0` ·
`route.py cursor` (`route v1.6 · 1b5a5c62`): `pending 0`, `half-promote 0 of 41`, no `NOT DERIVED` line ·
porcelain path set (all expected-transient bookkeeping):

```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-07T13-18-48-wrap/evolve-2026-10-07-a-capture-run-records-the-prompt-the-model-received-in-each-drive.json
 M .claude/session-handoff.md
```

**`upgrade.py detect --root .` — verbatim (the tool elides long facts with `…`):**

```
upgrade v1.6 · 814083ff
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · ok · setup · .claude/settings.json hooks · write current · bash current · PostToolUse on the stdin prologue
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · behind · setup · .claude/rules/host-{os}.md · host-win32.md was rendered for win32, this host is linux — the re-r…
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · ok · hand · working-route markerless introducers · 2 markerless entries · 0 introducers behind markup or after th…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 1 (U04) · awaiting a door 0 · noted 3 (U09, U10, U36) · INDETERMINATE 0 · 16 detectors of 40 registry entries
```

**Reading:** setup has ONE write — **U04**, the host leaf's re-seed, and it waits for the card's word (Phase 7.5).
Nothing is handed. **U09 · U10 · U36** are noted — card rows, no write. U11 and U12 read `ok`. No setup-class row
is `INDETERMINATE`.

---

## Net change set for this run, up to the card

| Path | Change |
|---|---|
| `.claude/backup/CLAUDE.md.pre-setup-2026-10-07T15-13-34` | new (gitignored) |
| `.andromeda/runs/2026-10-07T15-13-34-setup-project/` | new (this plan · `host-reseed.json` · `host-reseed.md` · the write manifest) |

No tracked file is modified before the card. **After "yes"**, Phase 7.5 would write, through `upgrade.py apply`
alone: the old leaf's backup · `host-reseed-dropped.md` · `.claude/rules/verification-harness.md` (+1 line) ·
`.claude/docs/session-learnings.md` (+133 lines) · `.claude/rules/host-linux.md` (4073 B) · the removal of
`.claude/rules/host-win32.md` — and then CLAUDE.md :111.

---

## Internal consistency check

- ≤200-line budget: 138, unchanged.
- The sort covers items 1-22 once each, with the tool's own line spans; the tool's dry-run accepted it (exit 0)
  and its counts equal §2's table: keep 4 · learnings 15 · `verification-harness.md` 1 · drop 2; 18438 B =
  1212 + 15911 + 755 + 560.
- Every `learnings` entry carries a one-line `title`; the sort's `date` is 2026-10-07.
- The one rule-file destination exists, has `paths:` (4 globs) and holds `## Session Additions` exactly once.
  `security.md` (no `paths:`) is nobody's destination.
- §2 says Phase 2 leaves the old leaf alone and the net change set lists no rule file before the card — agreed.
- §0's NOT-read list and every decision's basis are disjoint.

---

## After the Phase 7 card (the words given at the card — appended; the sections above stand as written at Phase 0)

- **`host 10 learnings`** (relayed in the session, 2026-10-07). Item 10 moves from `keep` to `learnings`, with a
  title, so it stays beside item 2 — the "2026-09-10 clause above" it refers to. `host-reseed.json` and
  `host-reseed.md` carry it. The sort now reads keep 3 (1 · 15 · 22, 630 B) · learnings 16 (12 `mixed`, 16493 B) ·
  `verification-harness.md` 1 (755 B) · drop 2 (560 B); 18438 B = 630 + 16493 + 755 + 560. Fresh dry-run, exit 0:
  `22851 B → 3490 B every turn`, `session-learnings.md` +138 lines. §2's table and the net change set's
  `4073 B` / `+133 lines` are the first proposal's figures.
- At that point no "yes" had been given and nothing of U04 was written; the old leaf stood at md5
  `b32a9ece05a83470dfe32e4fdf37d9af`.
- **"yes" — the founder's own word, relayed by the overseer in the session** (2026-10-07). As relayed: the founder
  answered the overseer's dialog at 17:22 local and picked "yes, with the change", the change being item 10 to
  learnings (already applied above). The dialog showed him the sort (22.9 KB to about 4 KB every turn, the narrow
  reading of "drop" and its ground) and offered four options: as proposed · with that change · drop the Windows
  lessons too · keep the old leaf. The dialog itself is in no committed file; this line records the relay, not
  the dialog. The figure he saw is the card's first one; with his change the new leaf is 3490 B.
- **Phase 7.5, performed on that word.** `upgrade.py apply --id U04 --sort … --run-dir …` exit 0: the old leaf
  backed up (md5 equal to the leaf's), `host-reseed-dropped.md` written (items 17, 18), item 12 appended to
  `verification-harness.md` (+1 line), 16 items written to `session-learnings.md` (+138 lines, above its first
  `## ` heading), `host-linux.md` written (3490 B, 53 lines, items 1 · 15 · 22 below the cut), the old leaf removed
  last. The tool's own closing line: "22 items · 18438 B found on disk where the sort says". Read back by hand:
  the three diffs' shapes, the new leaf whole, `i/lf w/lf` on both destinations. Then CLAUDE.md :111 re-rendered
  (one line: the rule list names `host-linux.md`). Re-detect: U04 `ok-uncommitted`.
