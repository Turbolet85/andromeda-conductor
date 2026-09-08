# Fan-out results — 2026-09-08-hosted-runner-webview2-session

7 Explore doc-agents, one per spec source, each given only its doc + the chunk report + its scoped
detectors. All 7 returned; none needed stripping or an entity decode (`entities=0` on every return), so no
raw twins are warranted — this file is the sanctioned audit artifact for the clean returns.

| doc | detectors run | verdict |
|---|---|---|
| architecture | D-arch-resources · D-arch-decisions · D-platform-claim | **1 proposal** (D-arch-resources) |
| security-plan | D-security-input · D-security-subprocess · D-security-deps · D-platform-claim | `proposals: []` |
| design-system | D-design-tokens · D-design-derived-count · D-platform-claim | `proposals: []` |
| layout-templates | D-layout-surface · D-layout-derived-count · D-platform-claim | `proposals: []` |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness · D-tests-derived-count · D-platform-claim | **6 proposals** (1 derived-count + 1 platform primary + 4 dependents) |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-redaction · D-platform-claim | `proposals: []` |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema · D-platform-claim | **1 proposal** (D-platform-claim) |

**8 proposals total.** Dispositions below.

## Validate

- **Check 1 (playbook).** The arch proposal matched no rule: `:137` requires the handle be one Conductor
  "neither SETS nor READS" (fails — `ci.yml` sets both) and `:143` requires a shipped artifact READ it
  (fails — nothing reads them back). Per amendment-flow, a rule failing a qualifying clause is NO MATCH and
  not a collision participant, so this was the no-match branch on a structurally new class → **escalated**.
  The test-plan and a11y proposals matched no rule either but are the D-platform-claim detector doing
  exactly what its own `check` prescribes; their SCOPE was escalated rather than their substance.
- **Check 2 (cross-contradiction).** None — no two proposals edit the same section in opposing directions.
  The two `:307` proposals touch different clauses of one table cell and were sequenced (pairing first,
  then the host-SET opening clause).
- **Check 3 (intent-consistency).** No divergence: the report matches the chunk's working-route entry and
  plan acceptance criteria, and the wrap directive's four corrections were folded into the report before
  the fan-out read it.
- **Check 4 (absence needs evidence).** Every "no other site" claim was re-derived by the orchestrator, not
  taken from a proposal — see the sweep basis in the test-plan sidecar. One claim in the ORCHESTRATOR's own
  first report draft failed this check and was corrected before the fan-out ran (a11y-plan:115 had been
  missed).
- **Check 5 (expected-amendments reconciliation).** The plan listed `test-plan` §9/§6, `a11y-plan` §11 →
  Strategy, and `matrix#v2-24 notes`; `architecture` + `security-plan` were superseded. All dispositioned.
  The a11y entry was **raised by the orchestrator**, not by a proposal, on a fact the report does not carry
  in Changes (three sites claiming the a11y job's first push-triggered run still pending, measured false by
  run 34209940695 `event: push`) — routine under check 5.
- **Check 6 (disproved-claims disposition).** Both entries of the report's `Spec claims disproved` bullet
  end DISPOSED: #1 by the test-plan amendment (split into its true and false halves); #2 (the chunk's own
  retracted probe-(b) reading) was already corrected in `evidence/probe-verdict.md` before P1, and no spec
  states it.

## Escalations (2) — both resolved WITH the operator, before any apply

1. **arch registry, unruled class.** Register the two `WEBVIEW2_*` names? → **Register, diagnostic-scoped,
   and mint the rule.** Applied; playbook rule appended (44 rules, was 43).
2. **test-plan capability-retirement scope.** → **Bound to the measured runtime.** The retirement names the
   hosted image AS SHIPPED (WebView2 Runtime 151.0.4129.101) rather than a flat incapacity, because probe
   (a) measured that image at that runtime and nothing wider — and a flat claim would contradict the
   directive's own follow-up probe entry before it runs.

## Rejected (1)

**a11y-plan `:115`, D-platform-claim — premise disproved by measurement.** The proposal read the site's
`sr*` runs as postdating the 2026-09-02 14:45 driver refresh, conflating
`2026-09-02-screen-reader-manual-spec` with `2026-09-07-sr-findings-fixed`. That chunk's own
`evidence/nvda-pass.json` timestamps its three subjects **10:41:57Z / 10:42:59Z / 10:44:50Z**, all before
the 12:45Z refresh, so the site's "under a 151 msedgedriver … cross-major pair" is **true as written** and
stays untouched. Recorded because it is the reason the amended `test-plan:307` SPLITS the claim (morning
half preserved) instead of discarding it.

## Applied (8 body edits across 3 masters + 5 leaves)

- `architecture.md` §Occupied Resources — Environment variables: the `WEBVIEW2_*` diagnostic pair (1 edit).
- `test-plan.md`: the `:307` pairing clause re-stated as a PAIR SET; five host-capability sites bounded to
  the measured runtime (`:469` primary, `:56`, `:123`, `:372`, `:307` Mode-cell opening) (6 edits).
- `a11y-plan.md`: three "first push-triggered run pending" sites corrected to the run that happened (3 edits).
- Cascade leaves re-derived: `.claude/docs/a11y-summary.md` ×3, `.claude/docs/tests-summary.md` ×1,
  `.claude/rules/a11y.md` ×1. CLAUDE.md carries none of the amended tokens (0 hits for `windows-2025` /
  `WebView2` / `push-triggered` / `msedgedriver` / `WEBVIEW2_`), so no `GENERATED:setup:*` block was owed.
- Sidecars appended to all three amended masters, each after re-reading the edited body.

## Cascade sweep basis

Patterns derived from every amendment of the pass before the first grep, with `windows-2025` as a
known-positive control (fired: arch 3, test-plan 6, a11y-plan 7). Retired-wording sweeps — `un-refreshed`,
`under a 151`, `151 msedgedriver`, `cross-major`, `first push-triggered`, `WebView2 host set`,
`Windows WebView2 host` — over all seven masters and all distillations. Surviving hits are exactly two
classes, both intended: this pass's own new correcting text, and `a11y-plan:115` / `tests-summary`'s
cross-major clause, both verified TRUE and deliberately left standing. No hit in the three
preserve-verbatim curation homes, and none in `playbook.md` / `drift-base.md`.
