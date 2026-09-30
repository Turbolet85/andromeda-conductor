# Cascade dispositions — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

**The search.**
- Patterns: `cascade-patterns.toml`, 9 patterns, one per retired claim, keyed on what the claim says:
  - `pending-os` `pending OS-level (key )?injection`;
  - `missing-keypath` `missing key path|would make these rows agent-driven`;
  - `count-seven` `count stays seven|seven governed forms|governs **SEVEN**|SEVEN forms`;
  - `four-driver` `four driver-stack forms`;
  - `confounded` `stay confounded|still confounded|confounded (arm S`;
  - `kbd-unmeasured` `physical keyboard is unmeasured`;
  - `neither-listener` `neither of which opens a listener`;
  - `fourteen-rows` `14 rows`;
  - `injected-arm` `agent arm (WebDriver-injected keys|the agent arm — WebDriver-injected keys`.
- Every control fired on the pre-pass masters (baseline `ff4f571`).
- Tool listings:
  - `cascade-sweep.txt` — the first run, after the body apply, before the leaves;
  - `cascade-sweep-final.txt` — after every leaf and the in-pass fix.
- Sections read beyond the sweep:
  - the SR-leg children restatements, `grep -rnoE 'window-activation|activate-window|spawns the host NVDA|screen-reader leg additionally'` over CLAUDE.md, `.claude/docs`, `.claude/rules` — 4 hits;
  - the rule (b) `seven` sweep over the seven masters — 14 hits, dispositioned in the report's Counts bullet.

**First run (after the body apply) — every row:**

| row | disposition |
|---|---|
| `.andromeda/a11y-plan.md:268` `injected-arm` @c1653 (standing, edited) | AMENDED in this pass. The sentence still named the injected path "the agent arm"; it now reads "the WebDriver-INJECTED path — … the agent arm's path until 2026-09-30", and "those agent-arm focus rows" became "the agent-arm focus rows graded on that path". Y3 had asked for it and the first apply had missed it. |
| `.andromeda/a11y-plan.md:268` `kbd-unmeasured` @c3811 (standing, edited) | no change — the clause is deliberately kept and now carries its status ("not run, by founder ruling … retired, not pending"). |
| `.claude/rules/a11y.md:39` `pending-os` @c1196 · `injected-arm` @c3027 · `confounded` @c3601 | leaf → RE-DERIVED from a11y-plan §3 (the OS key path, the input-path-bound browse rows, C1's confound resolution, the K ruling). |
| `.claude/docs/a11y-summary.md:27` `pending-os` | leaf → RE-DERIVED (the same; "the agent arm's focus rows are silent" reworded to "WebDriver-injected focus moves are silent"). |
| `.claude/docs/tests-summary.md:46` `missing-keypath` @c2168 | leaf → RE-DERIVED from test-plan §1 (untestable only where no OS key is sent in the row's window). |
| `.claude/rules/security.md:27` `count-seven` ×2 @c25,2697 | leaf → RE-DERIVED from security-plan rule (b): EIGHT forms; the eighth registered; "A NINTH crossing escalates again"; the W/153 clause no longer carries a count. |
| `CLAUDE.md:132` `missing-keypath` @c2295 (curation — `USER:session-learnings` 2026-08-22 entry, 2026-09-02 extension clause (3)) | ROUTED to P3 curation as an in-place extension; never cascade-edited. |
| `.claude/rules/verification-harness.md:59` `missing-keypath` @c2432 (curation — `## Session Additions` 2026-09-02, clause (5)) | ROUTED to P3 curation as an in-place extension; never cascade-edited. |
| `four-driver`, `neither-listener`, `fourteen-rows` — 0 rows, control fired | a statement about each pattern: the only sites were the amended master lines themselves (security-plan `:367`, architecture `:261`, test-plan `:47`). |

**The children-restatement read (4 hits):**
- `CLAUDE.md:34` (`GENERATED:setup:warnings`) → RE-DERIVED from arch Trust boundary. It named NVDA only; it now names NVDA, the window-activation script and `send-keys.ps1`, none opening a listener.
- `.claude/docs/gotchas.md:54` → RE-DERIVED, the same.
- `.claude/rules/security.md:27` → the rule (b) leaf already re-derived above (its hit is the new eighth-form text).
- `.claude/rules/verification-harness.md:59` → a curation home. Its `activate-window.ps1` clause (re-activation breaks a run) stays true; no change.

**Final run:** 3 rows, all dispositioned above — the two curation homes (routed to P3) and the kept `kbd-unmeasured`
clause.

**Leaves re-derived:** `.claude/rules/a11y.md`, `.claude/rules/security.md`, `.claude/docs/a11y-summary.md`,
`.claude/docs/tests-summary.md`, `.claude/docs/gotchas.md`, CLAUDE.md `GENERATED:setup:warnings`.

**Not looked for:**
- Leaves of design-system, layout-templates and obs-plan: those masters did not change.
- `.claude/docs/security-summary.md`: the `count-seven` pattern returned no row in it.
- The judgment bases (`playbook.md`, `drift-base.md`): 0 `base` rows on every pattern.
