# Materialization Plan — Conductor (setup-project re-run, the U02 upgrade)

**Run:** `2026-10-06T21-31-03-setup-project`
**Mode:** re-run (CLAUDE.md carries 20 `GENERATED:setup` / `USER` marker lines; 138 lines)
**Development Style:** `agent-driven` (arch §Cross-cutting Patterns:219, §Inherited Defaults:245)
**Origin of the run:** the handoff's Next — "`/andromeda-setup-project` for the pending `U02` (the Bash pre-cd
guard) — the overseer's direction".
**Host:** Linux dev host (the earlier setup runs were on the Windows host).

Phases 1–6 read ONLY this file.

---

## 0. Upstream read basis (what each decision rests on)

All 9 upstreams present (`input.md` 14 KB · `architecture.md` 117 KB · the 6 plans · `master-route.md` 113 KB —
865 KB together). **The read was STRUCTURAL, not whole-prose** — the same shape the 2026-09-15 run recorded: the
heading index of every upstream, then the sections Phase 0 actually consumes, each read in full (long lines read
by offset, never through a capped view). Stated per decision so a reader knows how far each row reaches:

| Phase 0 output | Basis actually read this run |
|---|---|
| Development Style | arch:219 and :245, both `agent-driven` |
| Overview / architecture block | arch §Design Philosophy :1-7 · §Project Intent :225-231 · §Inherited Defaults :233-245, in full |
| Module map | `Cargo.toml` `members` (9) ↔ arch:164 "Crate names (workspace members)" — set-equal, the `conductor-emit` exception included |
| Top warnings | arch §Cross-cutting Patterns :216-223 in full · the plans' Universal anti-pattern sections in full: security-plan :377-386 · design-system :341-348 · test-plan :514-522 · obs-plan :551-566 · a11y-plan :484-490 |
| Pointer-table anchors | every cited `§` found in its target's heading index: arch §Standard Contracts / §Occupied Resources / §Infrastructure Patterns / §Cross-cutting Patterns · security-plan §Input Validation / §Dependency Security / §Security Anti-Patterns · design-system §Color Palette / §Typography · test-plan §3 / §9 / §10 · obs-plan §3 / §6 · a11y-plan §3 · input §Coverage classification — all resolve |
| Keyed contracts (U35) | `registry.py contracts` per master: architecture 4 keys · test-plan 6 · obs-plan 9 · a11y-plan 9 · security-plan / design-system / layout-templates `n/a`; the four section bodies are one-line stubs naming their `.andromeda/registries/{master}-contracts.toml` index |
| Key directories | `ls contracts/` (7 members, 2 of them `.md`) · `ls scripts/` |
| Tier-1 template delta | `references/claude-md-template.md` read whole against CLAUDE.md :1-116 |
| Rule / docs / reviewer branch | on-disk inventory ↔ plan presence |
| Hooks | `references/hooks-matrix.md` whole · a parsed (JSON) comparison of each `.claude/settings.json` entry against the template's fenced entry |
| Health baseline | one read-only `health.py check` (no run dir) |

**NOT read this run:** the plans' body prose outside those sections, `master-route.md`'s records (the cursor is
`route.py`'s), `layout-templates.md` beyond its heading index. No decision below rests on them.

---

## 1. Tier 1 — CLAUDE.md

Current **138 lines**; projected **138** (five rows change in place, no line added).

### Blocks verified CURRENT, not rewritten
`setup:overview` · `setup:modules` · `setup:warnings` · `setup:workflow` · `setup:architecture` ·
`setup:imports` · `setup:deeper-topics`

- **overview** — the product paragraph is arch §Project Intent :227-228; the stack line is §Inherited Defaults;
  the `contracts/` bullet names 7 members and the directory holds 7.
- **modules** — 9 members, set-equal to `Cargo.toml` and arch:164.
- **warnings** — 11 bullets (the template's range is 5-12). Each traced to what was read: scope law + trust
  boundary (arch:222-223, security-plan:381, test-plan:522) · capability set as data · verdict/error wall
  (arch:221, security-plan:382) · preflight integrity with FIVE preconditions (security-plan:383-384) · supply
  chain (security-plan:379-380, :386) · subprocess hardening (obs-plan:562, :564) · artifact hygiene · self-obs
  never OTLP (obs-plan:556, :558) · wall-clock (arch:220, obs-plan:563) · status never color-alone
  (design-system:346) · determinism with the per-execution span-identity salt (arch:220, test-plan:520-521).
  No candidate was promoted or dropped this run (no audit list owed).
- **architecture** — the "real-model-posture scenario runs headless only" clause matches arch:4.
- **imports** — one line, `@.claude/session-handoff.md`; U01 reads `ok`.

### Block rewritten — `setup:pointer-table` (5 rows, the U35 migrated form)
The template's rows for arch §Infrastructure Patterns and the three `§3` harness contracts now carry a
migrated-project form (`{migrated (U35): … → .andromeda/registries/{master}-contracts.toml, one file per key}`).
The project IS migrated (U35 `ok`, since 2026-09-29) and the live rows still name the body sections alone —
which resolve, but to a one-line stub. Rendered rows:

| Topic | Source (new) |
|---|---|
| Directory tree · resource registry | `.andromeda/architecture.md` §Infrastructure Patterns (keyed: `.andromeda/registries/architecture-contracts.toml`, one file per key) / §Occupied Resources |
| Run-report envelope | `.andromeda/architecture.md` §Standard Contracts · `.andromeda/test-plan.md` §3 (keyed: `.andromeda/registries/test-plan-contracts.toml`) |
| Test harness (5-command) | `.andromeda/test-plan.md` §3 (keyed: `.andromeda/registries/test-plan-contracts.toml`, one file per key) |
| Observability / log JSON schema | `.andromeda/obs-plan.md` §3 (keyed: `.andromeda/registries/obs-plan-contracts.toml`, one file per key) / §6 |
| WCAG / a11y harness | `.andromeda/a11y-plan.md` §3 (keyed: `.andromeda/registries/a11y-plan-contracts.toml`, one file per key) |

The other 17 rows stand. The code-graph row keeps its project specialization (`plane REQUIRED — two are
detected`). Row count stays 22.

### Preserved verbatim
`USER:session-learnings` — wrap territory. Health check 1 reads it at 47.8 KB, 9 of 17 bullets over 600 B
(+40.3 KB above the cap). Surfaced on the card; setup never promotes.

### Backup
`.claude/backup/CLAUDE.md.pre-setup-2026-10-06T21-31-03` (taken; md5 `59c13552802c2cc60cbcd9a3b807c6ac`).

**Action: Phase 1 makes five anchored row edits inside `setup:pointer-table`, then validates.**

---

## 2. Tier 2 — `.claude/rules/` (7 files, all present)

| Leaf | Disposition |
|---|---|
| `security.md` | preserve |
| `host-win32.md` | preserve — NOTED (U04): 35 template lines missing above `## Session Additions`; regenerate is the operator's word |
| `testing.md` | preserve |
| `observability.md` | preserve |
| `a11y.md` | preserve |
| `frontend.md` | preserve |
| `verification-harness.md` | preserve |

No rule file is absent, so Phase 2 writes nothing. `host-win32.md` was rendered when the generating host was
Windows; this run's host is Linux, where the skill would not render it — an existing file is kept, inert where
it does not apply (its Session Additions already carry Linux-host notes). Frontmatter 5/5 parsed; `testing.md`
(85.2 KB) and `verification-harness.md` (80.6 KB) are past the Read cap — surfaced only.

---

## 3. Tier 3 — `.claude/docs/` (11 files, all present)

| Leaf | Disposition |
|---|---|
| `stack.md` · `conventions.md` · `commands.md` · `gotchas.md` | preserve |
| `workflow.md` | preserve — NOTED (U09): lacks `it never commits` |
| `security-summary.md` · `design-summary.md` · `tests-summary.md` · `obs-summary.md` · `a11y-summary.md` | preserve — NOTED (U36): the header line predates `cascade re-derives it` (one probe stands for the five) |
| `session-learnings.md` | preserve (wrap territory) |

No `services/` subtree (a crate-per-seam workspace; the Tier-1 module map is the altitude). Phase 3 writes
nothing.

---

## 4. Agent harness (Phase 4)

`scripts/agent-run.sh` + `scripts/agent-run.ps1` present, 5 verbs each (health check 13: sh 5/5 · ps1 5/5).
**Action: PRESERVE** (only-if-missing). Project-evolved far past the skeleton — expected, not drift; nothing
overwritten, so no backup owed.

- NOTED (U10): `agent-run.sh` has no `ensure_fresh_artifacts` hook; the project's own `ensure_frontend`
  (:55, called at five sites) fills that role under its own name.
- **Host note for the card:** `scripts/agent-run.sh` is index mode `100644` and not executable on this host
  (`test -x` exit 1, `core.fileMode` true) — generated on Windows, where the bit never set. CLAUDE.md's
  workflow line reads `scripts/agent-run.sh run`; here it runs as `bash scripts/agent-run.sh run`. Setting the
  bit is a tracked mode change to a project-evolved script — proposed on the card, not performed.

`.claude/rules/verification-harness.md` — preserved per §2.

---

## 5. Code reviewer + hooks + .gitignore + .gitattributes (Phase 5)

- **Code reviewer:** `.claude/agents/code-reviewer.md` present (rust). Disposition: preserve.
- **Hooks — the one upgrade write (U02 `behind`: "write current · bash pre-cd").** Parsed comparison of
  `.claude/settings.json` against `hooks-matrix.md`:

  | Entry | Reading | Action |
  |---|---|---|
  | `env` | `PYTHONUTF8=1` · `PYTHONIOENCODING=utf-8` | keep |
  | PreToolUse `Edit\|MultiEdit\|Write\|NotebookEdit` | equal to the template | keep |
  | PreToolUse `Bash` | DIFFERS — the pre-cd form (1030 B); the template's command (3858 B) begins with the present one minus its closing quote, then adds the top-level-`cd` arm | **replace** with the template entry |
  | PostToolUse `Edit\|MultiEdit\|Write` | equal to the template prologue (rustfmt row), timeout 30, no clippy | keep |

  Method: a python read-modify-write by path (the entry is a multi-KB single line) that takes the entry from
  the template's fenced JSON by parsing it, refuses unless its serializer reproduces the present file byte for
  byte, and reads the write back against the template. `settings.json` is backed up first to
  `.claude/backup/settings.json.pre-setup-2026-10-06T21-31-03`.
  **PostToolUse stays rustfmt-only — clippy is never restored at write time** (CLAUDE.md Tier 1, 2026-09-02;
  the matrix's Rust linter row now says the same).
- **rustfmt.toml:** present (`edition = "2024"`); U05 `ok`. No formatter config written, so no reflow step owed.
- **.gitignore:** U07 `ok`; health check 8: 6/6 entries satisfied (fragment `rust`). No change.
- **.gitattributes:** U06 `ok` — carries `* text=auto eol=lf` plus the project's named `coverage-matrix.md`
  control. No change; no re-checkout owed.

---

## 6. Code-graph pipeline (Phase 6)

Planes **rust** + **ts**; pipeline 4/4 present; health check 11 currency: `py 0 · sql 0 · cookbook 0` lines
behind. U03 `ok`. **Action: preserve all five files; no backup, no view rebuild.**

---

## 7. Operational artifacts (Phase 6 cont.)

All present, every one create-only-if-missing: `.andromeda/state.yaml` (schema 3; `last_wrap` ·
`tree_db_refreshed_at` · `session_count` only) · `.claude/session-handoff.md` · `.andromeda/drift-base.md` ·
`.andromeda/playbook.md` (U08 `ok`: 6 seed rules) · `.claude/docs/session-learnings.md` · `.andromeda/cache/`.
Phase 6 writes nothing.

---

## Upgrade

**Setup 5b record.** HEAD `fa6a374ebc49d951f452f061a998bb18865929a6` · `route.py cursor`: `pending 0`,
`half-promote 0 of 39` · porcelain path set (all expected-transient bookkeeping):

```
 M .andromeda/friction-log.ndjson
 M .andromeda/runs/2026-10-06T21-10-08-wrap/evolve-2026-10-06-a-fourth-pre-registered-real-model-series-for-v3-09.json
 M .claude/session-handoff.md
```

**`upgrade.py detect --root .` — verbatim (the tool elides long facts with `…`):**

```
upgrade v1.5 · eeed2076
U01 · ok · setup · CLAUDE.md @imports block · 1 import line(s) = the template's
U02 · behind · setup · .claude/settings.json hooks · write current · bash pre-cd
U03 · ok · setup · scripts/code-graph.py · code-graph-views.sql · code-graph-cookbook.md · lines behind: code-graph.py …
U04 · noted · setup · .claude/rules/host-win32.md · 35 template line(s) missing above `## Session Additions` — regenera…
U05 · ok · setup · rustfmt.toml · rustfmt.toml present
U06 · ok · setup · .gitattributes · .gitattributes carries `* text=auto eol=lf`
U07 · ok · setup · .gitignore base ignores · every base ignore decided by the root .gitignore (depth 2 included)
U08 · ok · hand · .andromeda/playbook.md seed rules · all 6 seed rules present by name or `seed:` tag
U09 · noted · noted · .claude/docs/workflow.md · .claude/docs/workflow.md lacks `it never commits`
U10 · noted · noted · scripts/agent-run.sh ensure_fresh_artifacts hook · scripts/agent-run.sh lacks `ensure_fresh_artif…
U11 · ok · setup · .andromeda/friction-log.ndjson · code-metrics.ndjson line endings · friction-log.ndjson LF · code-me…
U12 · ok · setup · .andromeda/residuals.md header · header = route-resolve's template
U13 · ok · hand · .andromeda/{doc}-amendments.md entry form · 7 sidecars · every entry on the form
U14 · behind · hand · working-route markerless introducers · introducer behind markup / after the em-dash at working-ro…
U35 · ok · hand · masters' logs + keyed contracts · ok: infra K, test K, test L, obs K, obs L, a11y K, a11y L, security…
U36 · noted · noted · .claude/docs/{security,design,tests,obs,a11y}-summary.md header line · .claude/docs/security-summ…
upgrade: for setup 1 (U02) · awaiting a door 1 (U14→next wrap P5) · noted 4 (U04, U09, U10, U36) · INDETERMINATE 0 · 16 detectors of 36 registry entries
```

**Reading:** setup writes **U02** (Phase 5). **U14** is handed — the next chunk wrap's P5 re-spells the
introducer. **U04 · U09 · U10 · U36** are noted — card rows, no write. U11 and U12 read `ok`, so Phase 7.5
applies nothing and only re-detects. No setup-class row is `INDETERMINATE`.

---

## Net change set for this run

| Path | Change |
|---|---|
| `.claude/settings.json` | the PreToolUse `Bash` entry replaced with the template's (U02) |
| `CLAUDE.md` | 5 rows inside `setup:pointer-table` (the U35 migrated form) |
| `.claude/backup/settings.json.pre-setup-2026-10-06T21-31-03` | new (gitignored) |
| `.claude/backup/CLAUDE.md.pre-setup-2026-10-06T21-31-03` | new (gitignored) |
| `.andromeda/runs/2026-10-06T21-31-03-setup-project/` | new (this plan · write manifest · validation log · health trail) |

Everything else: preserved.

---

## Internal consistency check

- ≤200-line budget: 138 projected, unchanged.
- Marker set unchanged (the pointer-table edits sit between its own start/end markers); parser contract holds.
- The five new pointer rows name four index files; all four exist under `.andromeda/registries/`.
- §5's hook write touches one entry; the write guard and the formatter entry are equal to the template already,
  so "replaced by this render" and "kept" coincide for them.
- §4's exec-bit note is a proposal only — no phase performs it, and the net change set does not list it.
- §0's NOT-read list and every decision's basis are disjoint: no row cites a section this run did not read.

---

## After the Phase 7 card (the operator's rulings — appended, the sections above stand as written at Phase 0)

- The five U35 pointer rows are kept.
- §4's exec-bit proposal was ACCEPTED and performed: `scripts/agent-run.sh` `100644 => 100755`, mode only
  (content md5 unchanged). It joins the net change set and the write manifest.
- U14 is left as it is — the detector's hit is a backticked literal in the working-route :94 CONTEXT prose, not
  an introducer; the next wrap's P5 re-spells it.
