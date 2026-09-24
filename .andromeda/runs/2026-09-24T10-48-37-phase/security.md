# security extract

## Relevance
Relevant. Part A (the scanning gate and the `.gitignore` key/cert entries) is this plan's own `secret-scanning-ci-gate` bootstrap item. Part B (the undeclared `env.X` CI check) and Part C (the backslash fix in the `settings.json` write guard) fall outside the security domain; the only security touch they have is the CI-spawn and egress constraints below.

## Constraints
- security-plan §Secret Management ("Secret scanning in CI") and §Security Anti-Patterns → Secrets (the `.env` / secret-shaped-string ban and "NEVER skip the secret-scanning CI gate") require a CI gate that fails the build when a secret-shaped string enters the tracked `conductor-*` workspace. It must be blocking: no `continue-on-error`, no `if:` that can bypass it. Whether any such gate already exists in `ci.yml` is a question for research.
- security-plan §Secret Management ("What counts as secret") defines the pattern classes the scanner must cover: API keys, DB passwords, JWT signing keys, OAuth client secrets, encryption keys, signing keys, webhook secrets and third-party tokens. The same section and §Threat Model Summary → Data classification (credential: none) set the expected baseline: 0 true hits.
- security-plan §Bootstrap phases (`secret-scanning-ci-gate`) requires `.gitignore` entries for `*.p12`, `*.pem`, `*.cer` and `.env*`. Scope measured `.env` / `.env.*` present and the three key/cert globs missing. Research should confirm this against the tree.
- security-plan §Secret Management ("do not adopt an unvetted tool") and §Security Decisions Log (Open question 3) leave tool selection to the operator. No scanner was researched. The choice must be recorded as a Decisions Log entry through wrap's amendment flow, never by editing the plan inside the chunk.
- security-plan §Dependency Security (third dependency class: exactly one member) and §Threat Model Summary → Infrastructure → Networking (one non-loopback egress, CI-only, count one before and one after) mean that a scanner fetched at job time would be a second member of the third dependency class and a second egress. Examples: a marketplace action, a downloaded binary, or a package pulled from a registry. That is a boundary widening that needs operator escalation, not a routine addition. An in-repo, stdlib-only scanner adds no member.
- security-plan §Security Anti-Patterns → Logging ("NEVER log secrets") and the evidence-hygiene bullet (no absolute host paths in committed `evidence/`) require hit reports to name the repo-relative file and line and never echo the matched secret in full. Any captured scanner output in `evidence/` must carry no host path.
- security-plan §Security Anti-Patterns → Code Patterns, rule (b), covers CI-workflow harness spawns. If the scanner step or the new `env`-key check runs a fixed toolchain binary on a repo script with fixed arguments, it stays outside rule (b)'s governed forms. If it builds argv from a value or runs through a shell string or an `eval`-equivalent (`-Command`, `Invoke-Expression`), it becomes a new governed form and must be escalated.

## Patterns to follow
- The deny.toml "every accepted exception carries a justifying comment" idiom (security-plan §Dependency Security → Accepted exceptions; the retirement of the `number_prefix` ignore cites `deny.toml:6-7`). Every scanner allowlist entry needs a stated reason. Remove an entry once its reason stops describing the tree.
- The shape of the third dependency class's admitting control (§Dependency Security): verify first, fail non-zero before anything runs, and never use `continue-on-error`. The scanner step should fail closed the same way.
- The CI arm's handle-named precondition output (§Input Validation, `CONDUCTOR_MSEDGEDRIVER` producer sentence): print a handle name and a boolean or location, never the value. Hit lines should follow this shape.
- The headless harness as source of truth (§Security Anti-Patterns → Authentication, the `agent-run.sh` bullet; the agent-driven Development Style). The local form of the check should run non-interactively from the harness.

## Anti-patterns to avoid
- Adding a scanner fetched at job time (GitHub Action, `curl`'d binary, `pip`/`npm` install) without escalation. It silently grows the third dependency class and the CI egress count (§Dependency Security; §Threat Model Summary → Networking).
- Committing the seeded known-bad fixture as a real secret-shaped literal, or allowlisting it with no reason. That weakens the invariant the gate enforces (§Security Anti-Patterns → Secrets, first bullet). Build the fixture at test time, or give it one reasoned allowlist entry.
- Echoing the matched string in CI logs or evidence (§Security Anti-Patterns → Logging, first bullet).

## Contract bindings
- security ↔ tests: the security gate joins the single CI workflow as a step or job (test-plan §CI Integration). Its fail arm needs a seeded known-bad input, like the new `env`-key check (scope item 7).
- security ↔ architecture: CI egress stays at one non-loopback fetch (architecture.md §Occupied Resources — Ports, as cited by security-plan §Threat Model Summary → Networking).
- security ↔ wrap amendment flow: the tool-selection record goes to the §Security Decisions Log, and any gate registration goes to the §Bootstrap phases wording ("optional" becomes realized). Both land at wrap, not in the chunk.

## Acceptance criteria contributions
- Over the tracked tree, the scanner exits 0 at baseline. With a known-bad input seeded at test time, it exits non-zero and each hit names the repo-relative `file:line` without the full matched secret (per security-plan §Secret Management; §Security Anti-Patterns → Secrets / Logging).
- The CI step running the scanner has no `continue-on-error` and no bypassing `if:`, and a red scanner turns the job red (per security-plan §Security Anti-Patterns → Secrets, "NEVER skip the secret-scanning CI gate").
- `.gitignore` covers `*.p12`, `*.pem`, `*.cer` and `.env*` (`git check-ignore` returns a match for a probe file of each class) (per security-plan §Bootstrap phases, `secret-scanning-ci-gate`).
- `ci.yml` still has exactly one non-loopback network fetch (the pinned msedgedriver), and no lockfile or third-class member is added by the scanner (per security-plan §Dependency Security, third dependency class; §Threat Model Summary → Networking).

## Relevant amendment history
- No amendment has touched §Secret Management, the `secret-scanning-ci-gate` bootstrap item, or Open question 3 of the §Security Decisions Log. The gate has never been worked on before.
- `2026-09-08-webview2-runtime-152-installed-in-job`: created the third dependency class (a CI-time-fetched binary that no lockfile gate can see) and scoped the Networking egress to the CI-only exception. The operator ratified it as a boundary widening. This is why a scanner fetched at job time would escalate.
- `2026-09-17-a11y-routine-arm-terminal-on-the-measured-configuration`: swapped the class's member (Evergreen replaced by the pinned msedgedriver) while keeping it at exactly one. Its re-derivation found `ci.yml` has two network calls, one of them loopback. That count is the baseline this chunk must preserve.
- `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate`: established that a plain `run:` step running a fixed toolchain binary with fixed arguments is not a rule (b) governed form, and that adding a CI step changes the §Threat Model CI/CD list ("what CI runs"). That list will need the same kind of cascade amendment at wrap when the scanner step lands.
- `2026-09-07-dependency-polish`: retired the `number_prefix` ignore because its justifying comment no longer described the tree. This is the precedent for the allowlist-reason discipline.
