# Materialization Plan — Conductor (setup-project re-run, CE-2 absorption)

**Run:** `2026-09-08T19-00-00-setup-project`
**Mode:** re-run (CLAUDE.md carries `GENERATED:setup` markers; byte-unchanged since `a25a425`)
**Development Style:** `agent-driven` (arch §Cross-cutting Patterns)
**Directive:** absorb pipeline-template change **CE-2**; preserve everything else as at `a25a425`;
harness scripts: keep.

Phases 1–6 read ONLY this file.

---

## 0. Upstream read basis (what each decision rests on)

All 9 upstreams verified present. Read basis, stated per decision so a later reader knows how far to
trust each row (the plans total ~600 KB; a full re-read would re-synthesize prose the directive says to
preserve, so the read was aimed at the surfaces Phase 0 actually consumes):

| Phase 0 output | Basis actually read |
|---|---|
| Overview · Modules · Architecture · Workflow | `architecture.md` **in full** (it is CLAUDE.md's current import, so the whole file was in context) |
| Universal warnings (top 10) | all 6 plans' `### Universal` / `## *-Anti-Patterns` sections, read directly + arch §Cross-cutting Patterns |
| Pointer table | arch §Standard Contracts + §Infrastructure Patterns + §Occupied Resources; all 6 plans' `^##` heading lists (for §-anchor accuracy); `master-route.md` structure |
| Rule-file branch | on-disk `.claude/rules/` inventory + plan presence |
| Docs branch | fixed 5 core + 5 summaries; no `services/` (crate workspace, not a service fleet) |
| Phase 4 / 6 drift | byte-diff of every seeded file against its template (results §6/§7) |

**Verification outcome:** every existing `GENERATED:setup:*` block was checked against its upstream and
is CURRENT. The last wrap's finding holds — "CLAUDE.md carries none of the amended tokens" — so no block
is owed a content re-derivation. The only owed change is the CE-2 template delta.

---

## 1. Tier 1 — CLAUDE.md

Target ≤200 lines. Current 135; projected **135** (net 0: imports loses a line, pointer table gains one).

### Blocks regenerated (content UNCHANGED, verified against upstream)
`setup:overview` · `setup:modules` · `setup:warnings` · `setup:workflow` · `setup:deeper-topics`

Each re-verified this run:
- **overview** — stack one-liner + 5 key dirs match arch §Stack / §Infrastructure Patterns tree.
- **modules** — 9 members, set-equal to arch §Occupied Resources "Crate names (workspace members)".
- **warnings** — 11 bullets; every one traced to a plan's `### Universal` section or arch
  §Cross-cutting Patterns this run (security 6 · tests 3 · obs 2 · a11y 1 · design 1 · arch 3, with
  overlap). No plan states a Universal ban absent from the block.
- **workflow** — 5 commands match arch §Workflow key commands.
- **deeper-topics** — lists exactly the 7 rule files + 11 docs that exist on disk.

### Blocks CHANGED (the CE-2 delta — four surfaces, all named by the directive)

**(a) `setup:imports` — architecture.md dropped, handoff only**
```
-@.andromeda/architecture.md
 @.claude/session-handoff.md
```
Rationale (template): an import rides every turn of every session; arch grows every version and every
skill that needs it reads it explicitly.

**(b) `setup:architecture` — final line re-worded**
```
-**Primary source:** architecture.md (imported below).
+**Primary source:** `.andromeda/architecture.md` (the pointer table's row — not imported; read explicitly where a step needs it).
```
This is load-bearing, not cosmetic: with (a) applied, "imported below" would be FALSE.

**(c) Maintainer note — new wording**
Replaced with the template's current text. Substantive changes: drops the stale "Imported files may be
300-800 lines each" clause; states that **both** `architecture.md` and `master-route.md` are deliberately
not imported, and that the loop reads arch's directory tree + resource registry structurally where a plan
creates files or mints a resource. (HTML comment — stripped from runtime context, zero token cost.)

**(d) `setup:pointer-table` — one new stock row**
Inserted second, directly after `Architecture decisions`, matching template order:
```
| Directory tree · resource registry | `.andromeda/architecture.md` §Infrastructure Patterns / §Occupied Resources |
```
Both section anchors **verified to exist** in `architecture.md` this run. This row is what carries (a)'s
dropped affordance: the sections the loop needs structurally are now named, not imported.

Table goes 21 → 22 rows (check 14 threshold ≥5: pass).

### Preserved verbatim
`USER:session-learnings` (lines 118–135, 17 lines) — wrap-session territory, copied byte-for-byte.

### Backup
`.claude/backup/CLAUDE.md.pre-setup-2026-09-08T19-00-00`

---

## 2. Tier 2 — `.claude/rules/` (7 files, all present)

`security.md` (unconditional) · `testing.md` · `observability.md` · `a11y.md` · `frontend.md` ·
`verification-harness.md` (agent-driven) · `host-win32.md` (Windows generating host).

**Action: preserve — no regeneration.** Basis: the directive scopes this run to CE-2, which touches
CLAUDE.md alone; no rule template changed; and every rule file's governing plan sections were verified
current above. Re-rendering would re-synthesize prose and manufacture a diff the directive excludes.
Each file's `## Session Additions` region is wrap-owned and would be preserved regardless.

Phase 2 therefore **validates** (frontmatter parses where present; each ends with `## Session
Additions`) and writes nothing.

---

## 3. Tier 3 — `.claude/docs/` (11 files, all present)

Core 5: `stack` · `conventions` · `commands` · `gotchas` · `workflow`.
Summaries 5: `security-` · `design-` · `tests-` · `obs-` · `a11y-summary`.
Plus `session-learnings.md` (wrap territory — never regenerated, create-only-if-missing).

No `services/` subtree: Conductor is a crate-per-seam workspace, not a service fleet; the module map in
Tier 1 is the right altitude and the per-crate detail lives in arch.

**Action: preserve — no regeneration**, same basis as §2. Phase 3 validates presence only.

---

## 4. Agent harness (Phase 4)

`scripts/agent-run.sh` (327 lines, executable) + `scripts/agent-run.ps1` (346 lines). All 5 commands
present in both (boot · run · status · cleanup · logs), verified this run.

**Action: PRESERVE (only-if-missing model; both present).** Directive says keep.

These are legitimately project-evolved far past the template: `ensure_frontend`, `run --e2e`,
`run --live` with per-leg capture, `assert_a11y_verdict` / `Assert-A11yVerdict`, the
`conductor preconditions` leading arm, run-contract term parsing. A fresh render would differ
substantially — that is expected evolution, **not** drift to remediate, and no backup is taken because
nothing is overwritten. Surfaced in the P7 card for visibility.

`.claude/rules/verification-harness.md` — preserved per §2.

---

## 5. Code reviewer + hooks + .gitignore (Phase 5)

- **Code reviewer:** `.claude/agents/code-reviewer.md` present (rust). No change.
- **Hooks:** `.claude/settings.json` valid; `env` carries `PYTHONUTF8=1` + `PYTHONIOENCODING=utf-8`;
  `PreToolUse` + `PostToolUse` present. **PostToolUse is rustfmt-only — clippy deliberately absent**,
  and this run MUST NOT restore it from the hooks-matrix default (session learning 2026-09-02: clippy
  already gates twice, `--fix` is workspace-wide and can rewrite outside the chunk's modify-set, it
  invalidates Read-before-Edit file-state tracking, and against this ~23 GB `target/` a cold run exceeds
  the 30 s hook timeout under `|| true` — no signal AND no error). `rustfmt.toml` present
  (`edition = "2024"`). **Action: no change.**
- **.gitignore:** all base entries + the rust/node stack fragments present. **Action: no change.**

---

## 6. Code-graph pipeline (Phase 6, plane-conditional)

Planes detected: **rust** (root `Cargo.toml`) + **ts** (tracked `tsconfig.json` under
`crates/conductor-tauri/ui/`) — two planes, which is why the pointer-table row keeps `plane REQUIRED`.

Byte-diff against templates, measured this run:

| File | Result |
|---|---|
| `scripts/code-graph.py` | IDENTICAL (386 lines) |
| `scripts/code-graph-views.sql` | IDENTICAL (64 lines) |
| `scripts/requirements.txt` | IDENTICAL (2 lines) |
| `scripts/scip_pb2.py` | IDENTICAL |
| `scripts/code-graph-cookbook.md` | IDENTICAL |

**Zero drift — no backup owed, no proposed update, no view rebuild.** (Contrast the 2026-08-21 re-run,
which found the pipeline two months behind its template; the per-file check is what establishes this,
not the pipeline's mere presence.)

---

## 7. Operational artifacts (Phase 6 cont.)

All present — every one create-only-if-missing, so all are preserved:
`.andromeda/state.yaml` (schema 3, lean) · `.claude/session-handoff.md` ·
`.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` ·
`.andromeda/cache/` (gitignored, populated).

---

## 8. Net change set for this run

| Path | Change |
|---|---|
| `CLAUDE.md` | 4 edits (imports −1 line · architecture Primary-source line · maintainer note · +1 pointer row) → 135 lines |
| `.claude/backup/CLAUDE.md.pre-setup-2026-09-08T19-00-00` | new (backup) |
| `.andromeda/runs/2026-09-08T19-00-00-setup-project/` | new (this plan + validation log) |

Everything else: **preserved, verified unchanged.**

---

## 9. Internal consistency check

- ≤200-line budget: 134 projected — pass, with headroom.
- Marker set unchanged (9 GENERATED + 1 USER); no marker added or removed, so the parser contract holds.
- Imports block non-empty (1 line) and its target `.claude/session-handoff.md` exists — check 3 pass.
- The dropped import is compensated by the added pointer row, and the architecture block's own
  "Primary source" line now points at the pointer table rather than at a nonexistent import — the three
  CE-2 edits are mutually consistent, and (b) is REQUIRED by (a) rather than incidental.
- No block claims an artifact this run did not verify present.
