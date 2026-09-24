# Fan-out results — 2026-09-24-secret-scanning-ci-gate (wrap 2026-09-24T14-02-12)

Seven doc-agents ran in one batch against `conductor-0.3.0/chunks/2026-09-24-secret-scanning-ci-gate/report.md`.
Entity probe: every return decoded to 0 `&lt;` / `&gt;` / `&amp;`. Three returns carried a harness
instruction-pattern prefix line, which was stripped as preamble.

| doc | verdict | proposals | raw twin |
|---|---|---|---|
| architecture | proposals | 9 (D-arch-resources ×1 · D-arch-decisions ×6, 4 dependent · D-platform-claim ×2, 1 dependent) | `.raw-fanout-architecture.md` |
| security-plan | clean | 0 — notes: the expected-amendment sites :87 / :224 / :259 / :396 are not detector-scoped; the test binary's `git` spawn is flagged out of D-security-subprocess's scope | — |
| design-system | clean | 0 | — |
| layout-templates | clean | 0 | — |
| test-plan | proposals | 2 (D-tests-derived-count ×2, 1 dependent) | `.raw-fanout-test-plan.md` |
| obs-plan | clean | 0 — note: the §1 / §9 / §10 gate-enumeration expected amendment is covered by no obs detector | — |
| a11y-plan | clean | 0 — note: its three quotes of arch §Stack's CI rationale (:115, :280, :471) re-sync only if that cell changes | — |
