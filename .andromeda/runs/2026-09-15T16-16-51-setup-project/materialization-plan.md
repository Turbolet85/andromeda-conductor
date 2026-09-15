# Materialization Plan — Conductor (setup-project re-run, seeded-script refresh)

**Run:** `2026-09-15T16-16-51-setup-project`
**Mode:** re-run (CLAUDE.md carries 18 `GENERATED:setup` marker lines; byte-unchanged since `dc757b3`)
**Development Style:** `agent-driven` (arch §Cross-cutting Patterns:248, :274)
**Directive (operator relay, this session):** absorb the pipeline-template delta in
`scripts/code-graph.py` (three replacements, quoted verbatim in §6) and `scripts/code-graph-cookbook.md`
(head replaced above its learnings marker). Preserve everything else as at `dc757b3`. Harness scripts: keep.

Phases 1–6 read ONLY this file.

---

## 0. Upstream read basis (what each decision rests on)

All 9 upstreams verified present (`input.md` 14 KB · `architecture.md` 129 KB · the 6 plans · `master-route.md`).
The directive scopes this run to two seeded scripts, so the read was aimed at the surfaces Phase 0 actually
consumes — re-synthesizing ~690 KB of plan prose would manufacture a diff the directive excludes. Basis stated
per decision so a later reader knows how far to trust each row:

| Phase 0 output | Basis actually read this run |
|---|---|
| Module map currency | `Cargo.toml` `members` (9) ↔ arch:163 "Crate names (workspace members)" — set-equal, incl. the 2026-09-13 `conductor-emit` exception CLAUDE.md already states |
| Pointer-table anchor validity | every cited `§` grepped in its target: arch §Infrastructure Patterns / §Occupied Resources / §Standard Contracts / §Cross-cutting Patterns (1 each); security-plan §Input Validation / §Dependency Security / §Security Anti-Patterns; design-system §Color Palette / §Typography; test-plan §3/§9/§10; obs-plan §3/§6; a11y-plan §3 — **all resolve** |
| Tier-1 template delta | marker set + maintainer note diffed against `references/claude-md-template.md` — identical |
| Workflow block | 5-command discipline re-measured in both harness scripts |
| Rule / docs branch | on-disk inventory ↔ plan presence |
| Phase 4 / 5 / 6 drift | per-file byte-diff of every seeded artifact against its template (results §4–§6) |

**Verification outcome:** every `GENERATED:setup:*` block was checked against its upstream and is CURRENT.
Unlike the 2026-09-08 run (which absorbed the CE-2 template change), **no Tier-1 template delta exists this
run** — so no block is owed a content re-derivation and Phase 1 writes nothing.

---

## 1. Tier 1 — CLAUDE.md

Current **136 lines**; projected **136** (net 0 — no block changes).

### Blocks verified CURRENT, not rewritten
`setup:overview` · `setup:modules` · `setup:warnings` · `setup:pointer-table` · `setup:workflow` ·
`setup:architecture` · `setup:imports` · `setup:deeper-topics`

- **modules** — 9 members, set-equal to `Cargo.toml` and arch:163. The `conductor-emit` exception clause
  CLAUDE.md carries matches arch's own 2026-09-13 wording.
- **pointer-table** — 22 data rows; every `§` anchor grepped and resolving (§0 table). The code-graph row
  keeps `plane REQUIRED (two are detected)` against the template's generic *"plane needed only when several
  are detected"* — a correct project specialization, since both planes are present (§6). **Preserve.**
- **imports** — one line, `@.claude/session-handoff.md`, target exists (health check 3 pass).
- **maintainer note** — byte-identical to the template (866 B, both).
- **marker set** — 9 `GENERATED` + 1 `USER`, same idents in the same order as the template. Parser contract holds.

### Preserved verbatim
`USER:session-learnings` (lines 118–135) — wrap territory. **NOTE for the P7 card, not an action:** this
region is **45.6 KiB across 15 bullets, 8 over the 600 B cap — those 8 hold 43.7 KiB, of which 39.0 KiB is
above the cap**. Basis: 45.6 KiB (46 672 B) is the marked block between the markers, including its heading
and prologue line; the bullets alone are 45.3 KiB (46 356 B). The removable debt is 39 940 B — the 8 × 600 B
each bullet is entitled to is not debt. Health check 1 flags it; **setup never promotes** (curation is
wrap/operator territory). Surfaced only.

### Backup
`.claude/backup/CLAUDE.md.pre-setup-2026-09-15T16-16-51` (taken; 60 466 B).

**Action: Phase 1 validates and writes nothing.**

---

## 2. Tier 2 — `.claude/rules/` (7 files, all present)

`security.md` · `host-win32.md` (both unconditional — no `paths:` frontmatter) · `testing.md` ·
`observability.md` · `a11y.md` · `frontend.md` · `verification-harness.md`.

**Action: preserve — no regeneration.** The directive touches neither a rule template nor a master;
re-rendering would re-synthesize prose and discard cascade-maintained wording. Frontmatter parses on all
five that carry it; all seven end with `## Session Additions`.

**Surfaced, not acted on:** two files exceed the Read tool's 25 000-token cap —
`testing.md` 79.4 KB (45 Session Additions, 19 over 1.5 KB) and `verification-harness.md` 69.8 KB
(27 / 13). The handoff names only the first; the second is equally past the cap. Promotion is the
operator's call.

---

## 3. Tier 3 — `.claude/docs/` (11 files, all present)

Core 5 (`stack` · `conventions` · `commands` · `gotchas` · `workflow`) + summaries 5
(`security-` · `design-` · `tests-` · `obs-` · `a11y-summary`) + `session-learnings.md` (wrap territory).

No `services/` subtree — Conductor is a crate-per-seam workspace, not a service fleet; the Tier-1 module map
is the right altitude. (Disposition unchanged from the 2026-09-08 plan §3.)

**Action: preserve — Phase 3 validates presence only.**

---

## 4. Agent harness (Phase 4)

`scripts/agent-run.sh` (327 lines, `test -x` pass) + `scripts/agent-run.ps1` (346 lines). All 5 commands
present in both (boot · run · status · cleanup · logs), re-measured this run.

**Action: PRESERVE (only-if-missing model; both present).** Legitimately project-evolved far past the
template — `ensure_frontend`, `run --e2e`, `run --live` per-leg capture, `assert_a11y_verdict`, the
`conductor preconditions` leading arm, run-contract term parsing. A fresh render would differ substantially:
expected evolution, **not** drift to remediate. Nothing overwritten, so no backup owed.

`.claude/rules/verification-harness.md` — preserved per §2.

---

## 5. Code reviewer + hooks + .gitignore + .gitattributes (Phase 5)

- **Code reviewer:** `.claude/agents/code-reviewer.md` present (rust). **No change.**
- **Hooks:** `.claude/settings.json` parses; `env` carries `PYTHONUTF8=1` + `PYTHONIOENCODING=utf-8`;
  PreToolUse ×2 (the Edit/Write path guard + the Bash heredoc/6500-byte guard), PostToolUse ×1.
  **PostToolUse is rustfmt-only — `grep -c clippy` over settings.json returns 0, and this run MUST NOT
  restore it from the hooks-matrix default.** Basis: CLAUDE.md Tier 1 (2026-09-02) — clippy already gates
  twice, `--fix` is workspace-wide and can rewrite outside the chunk's modify-set, it invalidates
  Read-before-Edit file-state tracking, and against this **77 GB** `target/` a cold run exceeds the 30 s
  hook timeout under `|| true` (no signal AND no error). The skill's own Phase 5 now agrees
  ("Rust: formatter only … clippy stays a gate"). `rustfmt.toml` present (`edition = "2024"`).
  **Action: no change.**
- **.gitignore:** every base entry satisfied. `scripts/__pycache__/` is covered by the broader bare
  `__pycache__/` at line 8 — `git check-ignore -v scripts/__pycache__/` resolves to `.gitignore:8`, so a
  literal-string miss is a false negative, not a gap. **Action: no change.**
- **.gitattributes:** carries the template's two lines verbatim (`# Line endings…` + `* text=auto eol=lf`)
  plus a project-owned named control for `coverage-matrix.md`. Index measured **all-LF** — 3 194 `i/lf`,
  4 `i/-text`, 13 `i/none`, **zero `i/crlf`, zero `i/mixed`** — so the wildcard renormalizes nothing.
  **Action: no change; no operator re-checkout owed.**

---

## 6. Code-graph pipeline (Phase 6) — **the only change set this run**

Planes detected: **rust** (root `Cargo.toml`) + **ts** (tracked `tsconfig.json` ×2 under
`crates/conductor-tauri/ui/`). Host tools all resolve: `rust-analyzer`, `scip-typescript`,
python `duckdb` 1.5.3 + `protobuf`.

Per-file byte-diff against `references/scripts-templates/`:

| File | Result | Action |
|---|---|---|
| `scripts/code-graph.py` | **DIFFERS** — 386 live vs 387 template | **replace (3 hunks)** |
| `scripts/code-graph-cookbook.md` | **DIFFERS** — 91 live vs 107 template, above the marker | **replace head** |
| `scripts/code-graph-views.sql` | IDENTICAL (64 lines) | preserve — **no view rebuild owed** |
| `scripts/requirements.txt` | IDENTICAL (16 B) | preserve |
| `scripts/scip_pb2.py` | IDENTICAL (15 502 B) | preserve |

### 6a. `code-graph.py` — three replacements (the template's current text)

1. **`:265` — the committed-trace write.** `newline=""` so the terminator is the one written, not the host's.
2. **the zero-row diagnostic's comment** — gains `/ an index gap`.
3. **the same diagnostic's message** — gains *"or the index missed its definition"*.

Hunks 2–3 are the substantive half: they add the **index-gap clause** to the 0-row rule that CLAUDE.md
Tier 1 (2026-08-08, as extended 2026-09-02) explicitly rests on — *"a 0-row result means consulted-but-no-match
ONLY once you have confirmed the symbol IS indexed"*. The shipped script could not state the fourth cause.

### 6b. `code-graph-cookbook.md` — head replaced

Template head adds: **query 5 (EXTERNAL SURFACE)** with its by-directory companion (11 lines), the
**never-`| head`/`| tail`** reading rule (2 lines — the cookbook-local form of CLAUDE.md Tier 1 and
host-win32 §Exit codes), and the **index-gap** extension of the 0-row rule (3 lines, the prose twin of 6a).

Both files END at `<!-- Project-specific query learnings accumulate below via wrap curation. -->` with an
**empty tail** — template and live alike — so "preserve the tail verbatim" is satisfied by a whole-file copy.
No project learnings are at risk.

### 6c. The relay's CRLF premise — corrected by measurement

The relay states *"every committed code-graph trace is CRLF in an LF-pinned repo (196 CRLF lines, 0 LF-only)"*
and asks for a re-stamp. **Measured across all 128 tracked `tree-query-*.json`:**

| State | Count |
|---|---|
| `i/lf w/lf` | 121 |
| `i/lf w/crlf` | 5 |
| `i/none w/none` | 2 |

**Every index blob carries 0 CR** (`git show ":$f" \| tr -dc '\r' \| wc -c` → 0 on all five). The 196 figure is
real and reproduces exactly — on the **worktree** copy of this wrap's trace — but `.gitattributes`'
`* text=auto eol=lf` normalized every one at `git add` time, which is what the warning the relay saw was
announcing. The five `w/crlf` files are the five most recent, i.e. the ones python wrote and no checkout has
since rewritten.

**Consequence:** `git add --renormalize` on those paths would stage nothing, so the operator fork dissolves —
**nothing is owed on history.** The diagnosis and the fix stand: 6a hunk 1 stops new traces being born CRLF.
Recorded here rather than silently performing a no-op.

---

## 7. Operational artifacts (Phase 6 cont.)

All present — every one create-only-if-missing, so all preserved: `.andromeda/state.yaml` (schema 3, lean:
`last_wrap` · `tree_db_refreshed_at` · `session_count`) · `.claude/session-handoff.md` ·
`.andromeda/drift-base.md` · `.andromeda/playbook.md` · `.claude/docs/session-learnings.md` ·
`.andromeda/cache/` (gitignored, populated, both planes).

---

## 8. Net change set for this run

| Path | Change |
|---|---|
| `scripts/code-graph.py` | 3 hunks → 387 lines |
| `scripts/code-graph-cookbook.md` | head replaced above the marker → 107 lines |
| `.claude/backup/code-graph.py.pre-setup-2026-09-15T16-16-51` | new (drift backup) |
| `.claude/backup/code-graph-cookbook.md.pre-setup-2026-09-15T16-16-51` | new (drift backup) |
| `.claude/backup/CLAUDE.md.pre-setup-2026-09-15T16-16-51` | new (re-run backup, unused — Phase 1 writes nothing) |
| `.andromeda/runs/2026-09-15T16-16-51-setup-project/` | new (this plan + validation log) |

Everything else: **preserved, verified unchanged.**

---

## 9. Internal consistency check

- ≤200-line budget: 136 projected, unchanged — pass with headroom.
- Marker set unchanged (9 GENERATED + 1 USER); no marker added or removed — parser contract holds.
- Imports block non-empty and its one target exists — health check 3 pass.
- §6a hunk 1 and §6c are consistent: the fix is forward-looking, and §6c establishes there is no backlog for
  it to clean — neither claims the other's ground.
- §6a hunks 2–3 and §6b's index-gap lines are the same clause in code and prose; applying one without the
  other would leave the script's message and the cookbook's rule disagreeing. Both are in the change set.
- §5's rustfmt-only clause is a **prohibition on this run's own default**, not an observation — it is the one
  place where following the hooks-matrix would regress a measured project invariant.
- No block claims an artifact this run did not verify present.
