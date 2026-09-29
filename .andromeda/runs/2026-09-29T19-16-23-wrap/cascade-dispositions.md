# Cascade dispositions — 2026-09-29-dual-license-mit-or-apache-2-0

The search: `cascade.py sweep` over `cascade-patterns.toml` (this dir), run AFTER all three amendments (X1 security
§Dependency Security · X2 arch Directory structure · X3 arch Build system). Patterns: `license` (regex
`licen[cs]e`, ci) · `private` (ci) · `publish` (ci) · `deny-toml` (fixed `deny.toml`) · `rootfiles` (fixed
`rustfmt.toml`, the root-file enumeration). The retired claim is the own-crate license EXEMPTION ("unpublished — skip
license checks"; `private = { ignore = true }`); its verbs (`skip`, `ignore`, `exempt`) ride inside the `private` /
`publish` / `license` hits. Every control fired (`architecture.md:212` · `:70` · `:60` · `:212` · `:223`). Long
lines were read by offset window (`@c`), never by the grep view. Totals (tool's count lines): license new 1 · standing
8 · leaf 2 · curation 5 · base 3; private new 2 · standing 5 · leaf 2 · curation 4 · base 1; publish new 1 · standing
10 · leaf 2 · curation 5 · base 1; deny-toml new 0 · standing 10 · leaf 6 · curation 4 · base 3; rootfiles standing 1
· curation 1.

## Masters
- `architecture.md:212` (license ×6, deny-toml ×4, publish ×2, private new @c5274) — AMENDED (X3). Re-read whole
  window @c4700-6766: the Tauri-tree license-allow narrative, the new own-crate sentence, the BSD-2-Clause and the
  dated 2026-09-05 audit figures stand true; no intra-line duplicate of the retired exemption (there was none before).
  `publish` @c1113 = `EDGEWEBDRIVER` publishing — a true claim sharing the token.
- `architecture.md:225` (license new ×3) — AMENDED (X2), the new tree row.
- `architecture.md:223` (rootfiles) — the tree's sibling row; no change.
- `architecture.md:70` · `:60` · `:113` · `:160` · `:199` · `:279` — `crate-private` read-back / env-handle and
  workspace-key PUBLISHES / the npm-grain deny analogue: true claims sharing the token; no change.
- `security-plan.md:184` (license ×5, deny-toml ×4, private new, publish new) — AMENDED (X1); the counts clause
  (16 + 9) verified against `deny.toml` (tomllib count 16 / 9, `private` key absent).
- `security-plan.md:176 :179 :180 :181 :186 :189 :192 :222 :226` — the deny gate named as `advisories bans licenses
  sources` / the tool floors / the audit-deferral history: none states the exemption; no change.
- `security-plan.md:111` — a private builder parameter (garde); unrelated.
- `test-plan.md:459 :534 :543 :605 :616 :307 :463` — the supply-chain gate row / private-fn testing rules / the
  `EDGEWEBDRIVER` publish: none states the exemption; no change.
- `obs-plan.md:646` · `a11y-plan.md:275` — `publishes` (host-path disclosure; a stale lib's last publish): unrelated.

## Leaves (step 3 set — recomputed from the amended masters)
- `.claude/docs/security-summary.md` — RE-DERIVED: gained the own-crate license-check bullet under Universal
  anti-patterns (from security-plan §Dependency Security, X1). Rows `:44` (license, publish) — the audit-deferral
  decision; no change.
- `.claude/rules/security.md` §Dependencies — RE-DERIVED: gained the license-policy bullet (X1). Rows `:30 :34`
  (deny-toml leaf) — the audit / npm-analogue bullets; true, no change.
- `.claude/docs/commands.md:38 :39` · `.claude/docs/stack.md:41` · `.claude/docs/gotchas.md:23 :67` — the deny gate
  as a command / stack entry / a dated audit reading / the workspace-key publish: none states the exemption; no change.
- `.claude/rules/testing.md:22 :34` — private-fn testing rules; unrelated.
- CLAUDE.md `GENERATED:setup:*` — recomputed for the two arch sections: `overview` lists key DIRECTORIES only (the
  root license files are not one), `warnings` Supply-chain line and the pointer table stay true; no change.

## Curation homes (never edited here)
- `.claude/docs/session-learnings.md:488-492` (license ×5, private, publish ×3) — the 2026-06-15 entry states "then
  `[licenses].private.ignore = true` skips their license check" as the fix: now STALE (the exemption is gone; the
  crates carry a license). → P3 curation, an in-place `[corrected 2026-09-29]` extension; its `publish = false` /
  `allow-wildcard-paths` half stays true.
- `.claude/rules/security.md:48 :49 :52 :53` (Session Additions) — deny/audit gate history and license-allow
  entries for dependencies; none states the own-crate exemption; no route.
- `.claude/rules/testing.md:72` · `.claude/docs/session-learnings.md:301 :361 :421` · `CLAUDE.md:128 :133` — other
  subjects sharing the token; no route.

## Judgment bases (never edited here)
- `.andromeda/playbook.md:60 :87 :93 :105 :219` — deny-overlap / audit-deferral rules and a `license` verb; none quotes
  the exemption; no route.
