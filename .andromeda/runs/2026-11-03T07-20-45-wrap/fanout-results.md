# Fan-out results — 2026-11-03-linux-only-base-ci (wrap 2026-11-03T07-20-45-wrap)

Batch stage1 · 7 agents · `fanout.py check` → `proposals: architecture 15 · security-plan 8 · design-system 1 ·
layout-templates 2 · test-plan 25 · obs-plan 2 · a11y-plan 0` · `VERDICT: 7 of 7 pass`. No extracted doc (`detail.py
check` → `whole` ×7): no cascade, sampler off. One scripted fix, after the apply render refused (`UNSUBSTITUTED {marker} in apply-architecture ·
apply-security-plan · apply-test-plan.p1`, exit 3, nothing written — the self-check `fanout.py:1336` reading the
literal path text `crates/conductor-run/tests/evidence/{marker}/` in three proposals as an unfilled name, against
`fanout-contract.md:112` "a value holding braces is never substituted again"): `fanout-architecture.md` (3),
`fanout-security-plan.md` (1) and `fanout-test-plan.md` (4) twinned byte-exact to `.raw-fanout-{doc}.md` first, then
`{marker}` respelled `<marker>` by script and nothing else; `check` re-run on the same two lines → `7 of 7 pass`. The
substance is unchanged: `<marker>` names the source chunk's folder, as `{marker}` did. The other four files are their
own raw record.

- architecture: 15 proposals · fanout-architecture.md — note: no D-arch-resources / collision / platform hit; the
  Stack CI/CD row (fct-k7rekguw) and Inherited Defaults CI/CD (fct-3rco7f56) name no OS; GHA_ENV_CONTEXT_PROBE
  (fct-an2jncn6) carries no date; fct-bjducqnw's 2026-09-04 two-shell runs dated history; fct-22zuwdf4's
  arch-registry-check.py clause stays (its retirement is the entry P5 mints).
- security-plan: 8 proposals · fanout-security-plan.md — note: D-security-deps / input / subprocess clear; dated
  history left (fct-gympmcsi 2026-09-04 bash+pwsh, fct-sbjbvw42's retired send-keys.ps1 / a11y-limited-token-launch.ps1,
  fct-eorfa32l Pulse's %APPDATA% default).
- design-system: 1 proposal · fanout-design-system.md — note: the design-system find hits fct-5opjjle6 alone; macOS
  dropped with Windows (the measured set), the plan's wording "drops Windows" narrower.
- layout-templates: 2 proposals · fanout-layout-templates.md — note: fct-g276wtgy / fct-dudaehek dated history.
- test-plan: 25 proposals · fanout-test-plan.md — note: D-tests-obs-harness bind — the ps1 retirement edits test-plan
  §3 and obs-plan §10 together; §10's citation-home prose names no `citation_home` value.
- obs-plan: 2 proposals · fanout-obs-plan.md — note: the two tokio sites (fct-tia6ntih, fct-lqqdem2q) and the
  `verification-matrix.json#v2-15` address (fct-amqff6tu) left to the orchestrator (raised below); §9 (fct-ecdrgile)
  names no shell or runner.
- a11y-plan: 0 proposals · fanout-a11y-plan.md — note: every section a 2026-10-11 retirement stub; no drift.

Citation sweep: `citation-dispositions.md` line 1 — re-pointed 1 · changed 2 · stretched 0; hand re-points 2 landed
(tx 55); one `claim false` row raised below (architecture 16).

## Dispositions

fanout-architecture.md
1. apply — check 1: measured-scalar literal (playbook :171) — the report's Dependencies carries tokio 1.54.1 with its basis (Cargo.lock, RUSTSEC-2026-0386)
2. apply — check 1: measured-scalar literal (playbook :171), the same reading
3. apply — check 1: measured-scalar literal (playbook :171), the same reading
4. apply — check 1: D-retired, the report's Retired bullet names the Windows CI runner; the plan's Expected amendment (Build system) names it
5. apply — check 1: D-retired, the Retired runner; the plan's Expected amendment ([CI/CD] sites take the runner)
6. apply — check 1: D-retired, the Retired runner; the plan's Expected amendment (§Established Decisions [CI/CD])
7. apply — check 1: D-retired, the Retired bullet names `scripts/agent-run.ps1`; the plan's Expected amendment (§Occupied Resources)
8. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§Occupied Resources)
9. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§Occupied Resources)
10. apply — check 1: D-retired, the Retired twin with its `RUST_LOG` save/restore; the plan's Expected amendment (§Occupied Resources)
11. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§Occupied Resources, `runs/live-suite`)
12. apply — check 1: D-retired, the Retired twin (Counts: harness scripts 2 → 1)
13. apply — check 1: D-retired, the Retired twin (Counts: harness scripts 2 → 1)
14. apply — check 1: D-retired, the Retired twin (Counts: harness scripts 2 → 1)
15. escalate → Q1 — check 1: Boundary widening (playbook :124) — restates the Data Protection exception at its widened locus (inputs#I3) · resolved: apply (Q1)
16. apply — check 5: `citation-dispositions.md` row `held fct-4rt3oej6 Cargo.toml:154 → claim false`; §Stack and Technologies (fct-4rt3oej6, the Hashing / digest row's SUT-pin citation): the claim holds, the number is stale — `andromeda-pulse Cargo.toml:154` → `andromeda-pulse Cargo.toml:153`, the line holding `blake3 = "1"` (inputs#I15, pointer committed@6d9fc93d) · as: re-point the digits only, nothing else on the line moves

fanout-security-plan.md
1. apply — check 1: Accurate this-chunk addition (playbook :317) — the report's Harness bullet "No leg witnesses the `#[cfg(windows)]` `spawn.rs` arms (compiled out)" with the Windows runner Retired; the owner from inputs#I5's row (PROVISIONAL, delegate overseer)
2. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§Input Validation manifests row)
3. apply — check 1: D-retired, the Retired twin with `Test-RunId`
4. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (Code Patterns rule (b))
5. apply — check 1: D-retired, the Retired twin
6. apply — check 1: D-retired, the Retired chunk-file renders; the plan's Expected amendment (§Input Validation capture ingest row)
7. escalate → Q1 — check 1: Boundary widening (playbook :124) — the Data Protection exception extends to `crates/conductor-run/tests/evidence/` (inputs#I3, the founder's own word to the P4 fork) · resolved: apply (Q1)
8. apply — check 1: Accurate this-chunk addition (playbook :317) — the host-path ban's committed-evidence scope follows the relocated captures (a narrowing of what escapes the ban, never a widening)

fanout-design-system.md
1. apply — check 1: Accurate this-chunk addition (playbook :317) — the report's Counts moves the cli Platform line `Windows / macOS / Linux terminal` → Linux; the plan's Expected amendment names the line · as: `**Platform:** Linux terminal — headless …` (the report's moved value), the rest of the line unchanged

fanout-layout-templates.md
1. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§cli Primary screens) · as: drop the parity parenthetical, keeping one pointer "the one harness script — test-plan §3"; drop both "in both shells"
2. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§cli Primary screens)

fanout-test-plan.md
1. apply — check 1: Accurate this-chunk addition (playbook :317) — the report's Counts: first-interrupt CI half pending → discharged (CI#41595128664); the plan's Expected amendment (§1 Untestable zones)
2. apply — check 1: D-retired, the Retired runner; the plan's Expected amendment (§9 Matrix builds, the OS bullet)
3. apply — check 1: D-retired, the Retired runner; the report's Harness bullet (the `#[cfg(windows)]` arms compiled out on every leg); the owner from inputs#I5's row (PROVISIONAL)
4. apply — check 1: D-retired, the Retired twin and pwsh dogfood step; the plan's Expected amendment (§3)
5. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§3)
6. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§3)
7. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§3)
8. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§3)
9. apply — check 1: D-retired, the Retired twin; the plan's Expected amendment (§3)
10. apply — check 1: D-retired, the Retired twin (Counts: harness scripts 2 → 1)
11. apply — check 1: D-retired, the Retired twin and pwsh dogfood step; the plan's Expected amendment (§3, key bootstrap-phases)
12. apply — check 1: D-retired, the Retired twin
13. apply — check 1: D-retired, the Retired twin
14. apply — check 1: D-retired, the Retired twin
15. apply — check 1: D-retired, the Retired twin
16. apply — check 1: D-retired, the Retired twin
17. apply — check 1: D-retired, the Retired twin
18. apply — check 1: D-retired, the Retired twin
19. apply — check 1: D-retired, the Retired twin
20. apply — check 1: D-retired, the Retired twin
21. apply — check 1: D-retired, the Retired twin
22. apply — check 1: D-retired, the Retired twin
23. apply — check 2: the proposal drops the tokio patch literal where architecture 1–3 replace it, opposite directions on the same reading; resolved by the report's fact (Dependencies: 1.52.3 → 1.54.1) and playbook :171 · as: replace `tokio 1.52.3` with `tokio 1.54.1` in the time-mocking row, nothing else moves
24. apply — check 2: test-plan quotes arch §Stack's row, which architecture 1 amends to 1.54.1 · as: replace `tokio 1.52.3` with `tokio 1.54.1` in the quote, nothing else moves
25. escalate → Q1 — check 1: Boundary widening (playbook :124) — §7 restates the capture locus under the widened exception; inputs#I3 names "test-plan §7 amended to match" · resolved: apply (Q1)

fanout-obs-plan.md
1. apply — check 1: D-retired, the Retired twin and pwsh dogfood step; the plan's Expected amendment (obs-plan §10)
2. apply — check 1: Accurate this-chunk addition (playbook :317) — the report's Harness bullet (the `#[cfg(windows)]` arms compiled out on every leg) with the Windows runner Retired
3. apply — check 5: the report's Dependencies sweep `tokio` names obs-plan ×2, no detector proposed them; §1. Obs Scope Summary (fct-tia6ntih, the conductor-timeline row): `tokio 1.52.3` → `tokio 1.54.1` (playbook :171) · as: replace the literal only
4. apply — check 5: the same reading; §1. Obs Scope Summary (fct-lqqdem2q, the runtime row): `tokio 1.52.3` → `tokio 1.54.1` (playbook :171) · as: replace the literal only
5. apply — check 5: the report's `spine-pair` row (obs-plan:288, a capability address in a file retired this chunk — "P2's to re-word or keep"); §Fault-injection spans (fct-amqff6tu): `verification-matrix.json#v2-15` → `cap:conductor-0.2.0/v2-15` (the store address the report names) · as: re-word that address only

## The plan's floor (check 5) — entries with no proposal
- architecture §Stack and Technologies (CI/CD row), §Inherited Defaults (CI/CD): no change — read fct-k7rekguw and
  fct-3rco7f56 whole: neither names an OS or runner (the report's own find: "no OS literal").
- architecture `GHA_ENV_CONTEXT_PROBE` "re-dated to the ubuntu run": no change — fct-an2jncn6 read whole carries no
  date and no OS; the probe's pair runs on whichever runner `rust` uses.
- test-plan §10 roster citation homes: no change — the report: no master names a `citation_home` value
  (control unfired; fct-pzb6nngw, fct-36bpudhj, fct-7efishf5 name §12 as the home, not a value).
- obs-plan §9: no change — fct-ecdrgile read whole names no shell or runner.
- tokio at 7 sites: architecture 1–3, test-plan 23–24, obs-plan 3–4 (raised); the leaf `.claude/docs/stack.md:8` is
  the cascade's.

## Checks 2–7
- check 2: test-plan 23 against architecture 1 — resolved above by the report's fact; no other pair opposes.
- check 3: the scope record's one line — widening `Cargo.lock` · word: the overseer's, founder-delegated, PROVISIONAL
  (inputs#I9) — the justified branch; it moves no master fact beyond the tokio literals above.
- check 4: each "no other site" note rests on the report's sweep rows (`cascade.py sweep --pre-pass`, 11 patterns) or
  a named `rec.py find`; the facts above read whole.
- check 6: Spec claims disproved — none in the report.
- check 7: no extracted doc — n/a.

Totals: 53 proposals + 4 raised = 57 · apply 54 · escalate 3 (Q1) · reject 0 · rejected for a source the report does
not carry 0.
