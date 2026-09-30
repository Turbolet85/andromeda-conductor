# Cascade dispositions — 2026-09-30-the-sr-cause-isolated-on-this-host wrap

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (this run dir), run AFTER every body edit of the pass
(A1+A2 a11y-plan:268 · T1 test-plan:307 · S1 security-plan:367 · AR4+AR5 architecture:193-194). Ten patterns, each
control fired on the pre-pass masters: `first-burst` · `user-hears-none` (what the retired user sentence SAYS, any
wording: user/operator … hears no focus) · `no-focus-change` · `cfg-bound` · `stay-cand` (the candidate-list verb) ·
`pair-cand` · `read-only-by` · `only-reader` (sole/only/solely reader, any case) · `count-seven` · `pair-set`. Scope: the
seven masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases, the leaf bodies. Sections
also read whole: a11y-plan §3 :268 (the amended clause re-read for an intra-line duplicate of the retired user
sentence — none: `user-hears-none` 0 rows after the pass).

## Every row
| row | disposition |
|---|---|
| a11y-plan.md:268 first-burst @c939 (edited) | amended — the new text keeps "first burst" deliberately, bound to the agent arm's injected, driver-launched path |
| a11y-plan.md:268 no-focus-change @c960 (edited) | amended — "no later focus change" now describes the agent arm only; the user sentence is replaced |
| a11y-plan.md:268 stay-cand @c2444 (edited) | amended — the candidates now named are the updates and the desktop; the pair is ruled out |
| a11y-plan.md:268 pair-cand ×2 @c1393,2241 (edited) | amended — "the runtime/driver pair 153.0.4234.48" (the new silent reading) and "the runtime/driver pair are ruled out" |
| .claude/rules/a11y.md:39 first-burst (leaf) | re-derived (step 3) |
| .claude/docs/a11y-summary.md:27 first-burst (leaf) | re-derived (step 3) |
| test-plan.md:56 / :307 @c774 / :472 cfg-bound | no change — the hosted endpoint question, a different subject |
| architecture.md:179 only-reader | no change — a crate-local gate test binary's reader, another handle |
| architecture.md:195 only-reader | no change — `CONDUCTOR_E2E_SEED_DIR`'s test-binary reader, another handle |
| security-plan.md:116 only-reader | no change — "`wdio.conf.ts` stays the sole READER" is scoped to the CI arm (ci.yml writes, wdio reads); no session script ran in CI, so it holds |
| security-plan.md:118 / :325, test-plan.md:87 only-reader | no change — `CONDUCTOR_E2E_SEED_DIR`, another handle |
| CLAUDE.md:132 only-reader (curation) | no change — a route-annotation reader, another subject |
| playbook.md:115 / :123 / :142 only-reader (base) | no change — "read solely by wdio" is a rule's CLASS criterion over committed readers, which the AR4/AR5 narrowing keeps true |
| playbook.md:148 / :191 only-reader (base) | no change — the WebView2 loader / a `contracts/` artifact, other subjects |
| security-plan.md:367 count-seven ×3 @c2013,8506,+9458 (edited) | amended — the head count ("seven governed forms") and both "count stays seven" statements stand true: S1 records two crossings that add no form |
| .claude/rules/security.md:27 count-seven (leaf) | re-derived (step 3) |
| test-plan.md:307 pair-set ×3 (edited) | amended — the PAIR set gains the 153 control pair; "the working set is a set of pairs" holds |

## Not amended, and why
- AR1, AR2, AR3 (arch env row for `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER`; Ports `:4445`; trust boundary) — REJECTED at
  E1 by the operator (founder-delegated): the arch registry carries STANDING committed readers and binders; the
  one-off dated controls live in security rule (b), test-plan §6 and the chunk evidence. A successor that COMMITS a
  reader or binder registers it then.
- security-plan `:72` / `:378` (the `4444`/`4445` binder lists) — no change: they enumerate the committed suite
  families; W was a one-off session crossing, recorded in rule (b).
- a11y-plan's "browse-mode rows stay pending OS-level key injection" — no change: S shows OS keys reach NVDA's hook,
  a "may be reachable" finding routed to route-resolve, not a disproof.
- CLAUDE.md `GENERATED:setup:*` — recomputed by structural read against the amended sections: overview, modules,
  warnings (scope law, trust boundary, `4444`/`4445`), pointer table and architecture block restate neither the
  sole-reader wording nor the SR verdict; no change.
