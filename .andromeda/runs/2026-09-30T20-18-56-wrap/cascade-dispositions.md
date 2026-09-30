# Cascade dispositions — 2026-09-30-the-screen-reader-content-findings-fixed

**The search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (11 patterns, every control fired on the
pre-pass masters at baseline `d7da5d07`), run twice — before and after the last amendment (a11y-plan `:113`); the
second listing is the one dispositioned. Patterns cover the retired claims' wording AND mechanism phrasings: the
footer "NOT in the shipped DOM" / "footer strip is unbuilt" / "no footer strip" / "no `contentinfo`" / "window footer
strip"; the landmark set "two `region`s"; the footer's "height `space-sm`" and "HOLD step 14"; the roll-up sample
"1 Calib"; the live leg's "one preflight canary"; the browse zone's "no OS key in (its|their) window stay(s)".
Not searched (no pre-pass control possible, 0 hits): "three `h2`", "no `h1`", "un-stopped", "contentinfo absent".
Sections read beside the grep: a11y-plan §3 (`:113`, `:268`, `:269`), §4 (`:320`, `:323`, `:334`), §5 (`:356`);
layout-templates `:48`, `:75`, `:99`, `:107`, `:154`, `:158`; design-system `:265`; test-plan `:47`, `:124`, `:307`, `:548`.

## Rows (second listing)
| row | class | disposition |
|---|---|---|
| `.claude/rules/a11y.md:19` notshippeddom + tworegions | leaf | re-derived (step 3) — the Landmarks bullet now states the shipped `contentinfo` |
| `.claude/rules/a11y.md:39` nooskeystay | leaf | re-derived (step 3) — the Testing bullet's browse-row clause |
| `.claude/docs/a11y-summary.md:27` nooskeystay | leaf | re-derived (step 3) |
| `CLAUDE.md:132` nooskeystay | curation (USER:session-learnings, 2026-08-22 clause (3) as corrected 2026-09-30) | no change — states the rule conditionally ("only a row with no OS key in its window stays unreached"), still true; preserve-verbatim |
| `.andromeda/layout-templates.md:245` calib | standing | no change — the cli coverage roll-up caption (a cli surface sample), not the webview footer |
| `.claude/docs/session-learnings.md:291` calib | curation | no change — a dated historical entry |
| `.andromeda/design-system.md:265` windowfooter | standing, edited | this pass's own new text (the token is the subject, not the retired claim) |
| `.andromeda/layout-templates.md:154` windowfooter | standing, edited | this pass's own new text |

First listing's extra row `.andromeda/a11y-plan.md:113` nooskeystay (standing) → amended in this pass (the same claim
O1 retired at `:268`; a same-master duplicate); absent from the second listing.
