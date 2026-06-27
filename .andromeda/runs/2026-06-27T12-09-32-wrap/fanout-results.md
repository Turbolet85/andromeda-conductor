# Fan-out results — 2026-06-27-desktop-a11y-harness-setup wrap

7 doc-agents, 2 proposals, 5 clean.

## security-plan — D-security-deps (escalate)
- section: §Dependency Security
- change: npm-audit gate bare `npm audit` clean (0) → dev-aware `npm audit --omit=dev` clean (0 production) [or justified suppression], the npm analogue of the deny.toml justified-ignore pattern for non-shipping transitive dev advisories.
- rationale: chunk added npm devDeps (axe-core/lighthouse/webdriverio/colorjs.io/@crabnebula/tauri-driver/tsx) → 20 dev-only transitive advisories (19 mod + 1 high), none fixable non-breaking; `npm audit --omit=dev` = 0. User decided "Dev-aware gate (wrap amends)".

## arch — D-arch-decisions (warning)
- section: §Inherited Defaults (Frontend)
- change (TWO parts): (a) update the npm-audit gate one-liner to dev-aware [CONSISTENCY with security-plan — APPLY]; (b) register the a11y test devDeps into §Inherited Defaults [OVER-REACH — DISMISS per playbook 2026-06-26 frontend-component-package rule; a11y-plan §3.5 + test-plan §6 already name the harness tooling; arch §Inherited Defaults summarizes the frontend stack (React/Vite/Tailwind/npm), never enumerated test/component tooling].

## design-system — clean (proposals: [])
No new product UI; tokens✓ on the a11y specs (colorjs.io reads :root, zero literals).

## layout-templates — clean (proposals: [])
No new user-facing surface/region; only test code + display-gated config.

## test-plan — clean (proposals: [])
Tests landed at the right tier (10/10 conductor-tauri mock-runtime + Path-7); cargo-nextest + @crabnebula/tauri-driver match §4/§6; display-gating expected per §6/§9; envelope §3 ↔ obs §3 unchanged.

## obs-plan — clean (proposals: [])
No new must-trace op (tests only); transitive OTel via lighthouse→sentry is dormant/never-initialized (acceptable per §3); no host-path/struct-name leak (TempDir + sanitized env).

## a11y-plan — clean (proposals: [])
No new interactive UI element; no violation-schema change (a11y-JSON→obs envelope is Epoch-10).

## Validation (main)
- The 2 proposals are ONE substantive change: the npm-audit dev-aware gate. Authoritative = security-plan §Dependency Security; arch §Inherited Defaults carries a consistency one-liner.
- D-security-deps (escalate) RESOLVED by the user during implement ("Dev-aware gate (wrap amends)" — recorded in report §Decisions). Apply.
- arch part (b) devDep-registration → DISMISS (over-reach, established playbook rule, test-tooling flavor).
- New playbook rule proposed: npm dev-only transitive advisory → dev-aware gate (npm analogue of the deny.toml floors/justified-ignore rules).
