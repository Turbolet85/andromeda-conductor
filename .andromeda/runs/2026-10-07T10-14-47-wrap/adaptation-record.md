# 0-pending wrap — operator-requested curation pass (2026-10-07)

**Path:** Setup step 6, the no-op path. 161 master records, all `complete`; 0 pending, 0 gated (`route.py cursor`
and a status-word census agree). The tree at entry was dirty only with bookkeeping: `friction-log.ndjson`, the
handoff's session-end stamp, and the prior wrap's evolve trail.

**The request:** the founder's own pick by dialog at 12:14 local, relayed in the wrap's arguments — "move the long
lessons now". Re-tier each Tier-1 bullet over 600 B; lose no fact; leave `.claude/rules/host-win32.md` and the
2026-08-22 Windows-only bullet untouched (the pipeline overseer's door, PC35); route unchanged.

## Measured before and after (`health.py check`, check 1)

| | lines | Tier-1 region | bullets over 600 B |
|---|---|---|---|
| before | 138/200 | 47.8 KB | 9 of 17 (+40.3 KB) |
| after | 138/200 | 10.6 KB | 1 of 17 (+4.4 KB) |

The one bullet still over the cap is the 2026-08-22 entry (5078 B), left as directed.

## Items and dispositions

Routing rule applied to each: `curation-tier-decision.md` — a one-sentence lead under 600 B stays in Tier 1; the
full text goes to a path-scoped rule file where the scope is clear, else Tier 3. No rule file's `paths:` covers
any of the eight subjects (`a11y` and `frontend`: the Tauri UI tree; `observability`: crate sources and the
harness scripts; `testing` and `verification-harness`: test trees, scenarios, the cli and verify crates), so all
eight took Tier 3. Five of the eight are also past Tier 2's 1.5 KB entry cap.

| Entry | Subject | Before | Lead | Full text | sha256 of the old bullet line (first 16) |
|---|---|---|---|---|---|
| 2026-08-08 | code-graph call sites | 2147 B | 462 B | Tier 3, 2 paragraphs | `3305ac9e4a3867fc` |
| 2026-08-09 | grep the document before asserting what it says | 18364 B | 474 B | Tier 3, 15 paragraphs | `03e2d95c640b6945` |
| 2026-08-10 | `intent.md` / `requirements.md` immutable | 2722 B | 476 B | Tier 3, 4 paragraphs | `864b5aa8e0fe0242` |
| 2026-08-15 | named blockers vs a measured leg | 964 B | 357 B | Tier 3, 1 paragraph | `043dbf9580db1a3e` |
| 2026-08-21 | verify the artifact, not the exit code | 12793 B | 489 B | Tier 3, 12 paragraphs | `ef4e0e0d5cf06b42` |
| 2026-09-02 | the write-time hook runs `rustfmt` only | 2022 B | 406 B | Tier 3, 2 paragraphs | `d8616f46c05c0ea0` |
| 2026-09-04 | jointly contradictory plan steps | 1127 B | 385 B | Tier 3, 1 paragraph | `e033b022468f4e20` |
| 2026-09-17 | a token naming one arm of a fork | 1485 B | 500 B | Tier 3, 1 paragraph | `2d535f25541621af` |
| 2026-08-22 | Windows-only host | 5078 B | — | untouched, as directed | — |

## How the move was made, and how it was checked

- **Form.** Each full text was moved by a read-modify-write by path, never retyped: the bullet's text after its
  leading `- ` landed under a new heading in `.claude/docs/session-learnings.md`, broken into paragraphs before
  each dated extension that opens a bold span, and otherwise unchanged. The eight entries sit at the top of the
  file, newest origin date first, each headed `## 2026-10-07 — Tier-1 entry of {origin date}, moved whole: {title}`
  and carrying a one-line note of where it came from.
- **Leads.** Each Tier-1 lead keeps the entry's origin date, states the rule in one sentence and ends with a
  pointer naming the Tier-3 heading. The per-clause confidence figures stay in the full text only.
- **Read-back, against `HEAD`'s copies.** The eight changed `CLAUDE.md` lines are exactly the eight dates above;
  every other line is byte-identical, the 2026-08-22 bullet included. Each old bullet's text, whitespace
  collapsed, is contained in the new Tier-3 file. The Tier-3 file's prior content is intact before and after the
  inserted block (44667 B inserted, 102194 B to 146861 B). No inserted paragraph leaves a bold span open. Both
  files stay LF. `git diff --quiet -- .claude/rules/` exits 0.
- **Recovery.** The pre-move bullets are `git show b55f346:CLAUDE.md`, lines 127-131, 133, 134 and 136.

## Citations of the moved entries

A sweep of the rule files, `.claude/docs/`, the seven masters and their sidecars, the registries, the playbook,
the drift base and the working route found no live citation of a moved bullet by line number. The hits are
sidecar-archive records of past sweeps (append-only history, not edited) and the prior handoff's two notes,
which cite the entries by date; both dates still resolve to a Tier-1 lead.

## Other step-6 duties

- **Gated re-check:** 0 `gated` records — nothing to re-verify.
- **Route:** unchanged, as directed. No consolidation, supersession, registry migration or amendment ran.
- **Curation of this session's own conversation:** no candidate — the conversation carried the one directive
  and no correction.
- **Filter 5's three-entry cap** bounds new learnings per wrap; this pass added none and re-homed existing
  entries on the operator's word, so the cap did not apply.
