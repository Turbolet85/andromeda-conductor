# Curation log — 0-pending wrap, 2026-10-09T13-23-15

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   none
  Tier 3 (.claude/docs/session-learnings.md): none new
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred · 1 below the confidence threshold
  Extended: Tier 3/.claude/docs/session-learnings.md: "Tier-1 entry of 2026-08-10, moved whole: `intent.md` and
            `requirements.md` are immutable" + "one recorded exception stands, and its shape is the only one"
```

## Extended

- **Tier 3, `.claude/docs/session-learnings.md`, the entry "Tier-1 entry of 2026-08-10, moved whole"** — one
  paragraph tagged `Extended 2026-10-09`, appended after the entry's last dated extension. The Tier-1 lead in
  `CLAUDE.md` is unchanged.
  - Signals: the operator's explicit direction to record the edit as an exception to the 2026-08-10 rule with its
    reason (+0.4) · a specific technical detail with context (+0.2) · "always/never" language in the direction's
    reason, "before any chunk or ledger note rests on it" read as the exception's bound (+0.3). Total 0.9.
  - Filter 1: the corpus holds two entries on the subject, the Tier-1 lead and this Tier-3 full text; neither
    names an exception. The facet is additive: it does not make the entry false, since no skill wrote the file.
  - Proof: `adaptation-record.md` in this run dir, item A — the operator's word, the three places that forbid the
    wrap the edit, and the byte proof (sha256 `c3acafab…447b7c` → `7abbe98f…6017b2`, 19 359 → 19 455 B, 24 lines
    changed, the file byte-equal to its committed form with every asterisk pair removed).

## Filtered

- **"Run the project's ledger gate over a route's new version directory before the route commit is pushed."**
  Confidence 0.4, below the threshold: proven by a real gate failure (+0.4) · a specific technical detail with
  context (+0.2) · paraphrased from implicit behavior, since nobody in this conversation stated it (−0.2).
  Not lost: the gate's own repair is pinned as a `CARRY:` on `Linux-only base CI`, and the first
  `/andromeda-phase` reads the CI verdict of every commit since the last master flip.

## Not candidates

- The relay's misreading of `route-resolve.md:153` is a fact about a pipeline letter and an operator relay, not a
  project learning. It is recorded in `adaptation-record.md`.
