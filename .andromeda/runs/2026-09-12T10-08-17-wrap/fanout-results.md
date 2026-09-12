# Fan-out results — 2026-09-11-hosted-runner-endpoint-cause-closed

7 doc-agents, one per spec source, one parallel batch. Entity-decode applied to every return
(`&lt;`/`&gt;`/`&amp;` → literal; the security and arch returns carried `&lt;` and YAML folded-scalar
`&gt;-`); `entities=0` on each decoded body.

| doc | verdict | proposals |
|---|---|---|
| architecture | **drift** | 5 — D-arch-decisions ×4 (one primary + 3 `dependent-of`) · D-platform-claim ×1 |
| security-plan | **drift** | 3 — D-security-deps ×3 (one primary + 2 `dependent-of`) |
| test-plan | **drift** | 2 — D-platform-claim ×2 (one primary + 1 `dependent-of`) |
| design-system | clean | `proposals: []` |
| layout-templates | clean | `proposals: []` |
| obs-plan | clean | `proposals: []` |
| a11y-plan | clean | `proposals: []` |

Raw twins saved for the three docs carrying proposals: `.raw-fanout-architecture.md`,
`.raw-fanout-security-plan.md`, `.raw-fanout-test-plan.md`. The four clean returns are recorded here;
each walked its detectors explicitly rather than returning a bare empty, and each cited the grep that
established its zero.

## The two amendment groups

**Amendment 1 — the always-latest Evergreen posture (7 sites).** The plan named 1 site; this report's
authoring sweep found 2; the `dependent-of` mechanism found 7. Five of the seven carry no `always-latest`
token at all — they restate the unconditional fetch in the threat model, the ports registry and the trust
boundary — so a token-keyed single-site apply would have left five live.

| doc | section | site | role |
|---|---|---|---|
| security-plan | §Dependency Security — third dependency class | `:185` | primary |
| security-plan | §Threat Model Summary → Networking | `:86` | dependent |
| security-plan | §Threat Model Summary → CI/CD | `:87` | dependent |
| architecture | §Infrastructure Patterns — Build system | `:205` | primary |
| architecture | §Established Decisions [CI/CD] | `:59` | dependent |
| architecture | §Occupied Resources — Ports (fwlink entry) | `:147` | dependent |
| architecture | §Cross-cutting Patterns — Trust boundary | `:248` | dependent |

**Amendment 2 — the unread module probe and the unvaried elevation (3 sites).** The plan named 1; this
report's sweep found 2; the sweep found a third.

| doc | section | site | role |
|---|---|---|---|
| architecture | §Established Decisions [CI/CD] | `:59` | D-platform-claim |
| test-plan | §9 CI Integration — Matrix builds | `:469` | primary |
| test-plan | §6 E2E Test Strategy — Drivers per surface (desktop-webview row) | — | dependent |

`architecture.md:59` receives one proposal from each group. They are complementary, not opposing
(Validate check 2): one narrows the provisioning mechanism, the other retires the cause verdict. Applied
as one coherent rewrite of that bullet.

## Clean-return notes worth keeping

- **a11y-plan** surfaced a PRE-EXISTING cosmetic defect it correctly did NOT propose (no detector covers
  it, not this chunk's): `a11y-plan.md:115` repeats the clause "the one webview-automation stack running on
  the measured platform SET — " twice in a row, from an earlier verbatim apply. Verified by the
  orchestrator (`t.count(p)` = 2 on that line). Surfaced to the operator; not edited, because editing a
  master outside the proposal flow is exactly what the constraints forbid.
- **a11y-plan** also checked the one skew-adjacent sentence (`:115`, a cross-major pair measured
  *working*) and found it consistent with the retirement rather than falsified by it.
- **architecture** verified two sites as still TRUE and correctly did not propose them: `:240` ("fails at
  WebView2 session creation") — the endpoint is still closed, arm C did not open it; and `:59`'s
  run-pinned historical literal, which is past-tense and run-scoped rather than a stale current value.
- **test-plan** dismissed every `152.0.4191.66` occurrence (`:56`, `:307`, `:469`) as an attributed sample
  tied to a named run inside an explicitly-named SET — the de-literalisation convention working.
