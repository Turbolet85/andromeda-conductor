# Session Handoff

**Last Updated:** 2026-09-12T10:45:00Z
**Branch:** `build/conductor-0.3.0`, tracking `origin/build/conductor-0.3.0`. The operator pushed TWICE
mid-implement (`c93a379`, `eecc7f4`), so the branch was **0 ahead at wrap start** and is **1 ahead and
unpushed** after this wrap's commit. Both pushes were load-bearing — each produced the CI run an arm reads.
The operator pushes after this commit.
**Status:** clean — with one RED gate the chunk does not own (below)
**Last Commit:** `feat(2026-09-11-hosted-runner-endpoint-cause-closed)` (this wrap)

## Position
- Done: **`2026-09-11-hosted-runner-endpoint-cause-closed`** — the hosted-runner endpoint cause
  **ESTABLISHED as elevation**, by direct variation with a control on both sides. Three arms: (A) the module
  reading taken inside the isolation step's live window; (C) driver/runtime major skew varied and RETIRED;
  (B) elevation varied on the known-good dev host and established as the cause.
- Next: **`Ledger gate id-space generalised`** — minted this wrap directly after this chunk in Epoch 1 on
  the operator's directive, and **BLOCKING**: the version's workspace gate is red until it lands, so nothing
  is scheduled ahead of it.
- Coverage **1/11 verified · 10 unclaimed** — `v3-01` claimed, implemented and verified this chunk.

## The red this chunk does not own

`cargo nextest run --workspace --profile ci` is RED: **906 run, 905 passed, 1 failed.**
`crates/conductor-report/tests/matrix_ledger_gate.rs:49` filters requirement ids by a baked
`starts_with("v2-")` while the same file's header records that its version-directory resolution was
deliberately generalised — "a baked `conductor-0.2.0` would silently stop covering the ledger at 0.3.0,
which is the failure mode this gate exists to prevent." `conductor-0.3.0/requirements.md` declares 11 `v3-`
ids and 0 `v2-`, so the required set is empty and the gate's own anti-vacuity assertion fires.

It landed in `b3b51e6` — the PREDECESSOR's wrap commit, which also deferred the very gate that would have
caught it — and this chunk has **zero `.rs`/`Cargo.*` delta**, so it cannot have caused it. Red in CI too, on
both `c93a379` and `eecc7f4`. /implement soft-exited on it (Trigger 3) rather than fixing it, after measuring
that a correct generalisation touches seven `v2-` literals (`:219 :221 :228 :250 :256-259`) plus the id-space
assertion at `:208`. **The light gate's ASSERT passed on this recorded disposition, not on a green** — per the
operator's directive, dispositioned exactly as `2026-09-09-workspace-formatting-pass-and-a-fmt-ci-gate` did.

**Process note for the record:** this is the THIRD wrap in four days where an operator directive had to supply
a disposition the light-gate letter cannot express. The recurrence is the finding, not the workaround.

## What the chunk measured

| candidate | status |
|---|---|
| WebView2 / Edge policy state | RETIRED (predecessor) — five keys absent on both sides |
| session 0 / non-interactive | RETIRED (predecessor) — `SessionId 2`, `UserInteractive: True` |
| loaded module version | MEASURED (arm A), and its **skew RETIRED** (arm C) by controlled variation |
| **elevation** | **ESTABLISHED** (arm B) — the cause |

**Decision: REMEDIABLE IN PRINCIPLE** — the app must not run elevated. The **MECHANISM is recorded, not
established**: no probe measured *why* an elevated token suppresses the endpoint, only the correlation under
single-variable variation. Scope is bounded to the hosted `windows-2025` image and the Windows dev host.

## Drift resolved
**10 proposals across 3 docs · all 10 applied · 1 escalation resolved · drift = 0.** Four of seven detectors
returned `proposals: []`.
- **Amendment 1 (7 sites)** — the always-latest Evergreen posture → floor-conditional. `security-plan` ×3
  (§Dependency Security `:185`, §Threat Model Networking `:86`, §Threat Model CI/CD `:87`) · `architecture`
  ×4 (§Infrastructure Patterns `:205`, §Established Decisions `:59`, §Occupied Resources — Ports `:147`,
  §Cross-cutting — Trust boundary `:248`). **ESCALATED**: playbook `:149` fails its clause (a) — both
  primaries name "once the `a11y` job actually GATES" as their retire-condition and it is measurably UNMET
  (A11y job `failure`, `DevToolsActivePort` never seen at run `34654076633`), while the posture changed
  because the probe's SUBJECT moved. Resolved by operator directive item 3. No playbook rule proposed.
- **Amendment 2 (3 sites)** — the unread module probe and the unvaried elevation. `architecture:59` ·
  `test-plan` §9 `:469` · `test-plan` §6 drivers row. **ROUTINE** under `:149`, all three clauses holding.
- **The `dependent-of` sweep earned its keep:** the report named 2 sites; the sweep found 10. Five of
  amendment 1's seven carry no `always-latest` token, and amendment 2's third site carries neither `0.645`
  nor `demonstrated cause` — found by reading for the claim's meaning, not its tokens. A single-site apply
  would have left eight live.
- Cascade: **3 leaves re-derived** (`CLAUDE.md:34`, `docs/security-summary.md` ×2, `rules/security.md:31` —
  body side of the `## Session Additions` boundary, checked before editing). Residue sweep across the seven
  masters + leaves + `playbook.md` + `drift-base.md` + the three curation homes: CLEAR on 9 patterns, with a
  known-positive control fired through the same grep form to prove the zeros.

## Curation
**T1 0 · T2 2 · T3 0** (cap not reached; 2 filtered as dedup-recurrence).
- `host-win32.md` — **extended in place** per the operator's directive: the drive-letter anchor's
  false-positive class is now a standing CHECK, naming all three instances (registry provider form;
  a URL scheme; and the prose explaining the false positive, which tripped it by spelling the tokens).
  Deliberately NOT a third corpus entry.
- `verification-harness.md` — **new**: a RED baseline validates only the failing direction. Two measured
  captured-log hazards under it — GitHub echoes each step's source so a literal double-counts (the gate read
  `2` where it asserted `1`), and a PowerShell-redirected capture is UTF-16 and greps as **zero** on content
  it plainly contains.

## Notes
- **Both commits and both pushes were the OPERATOR's act**, mid-implement, for the same structural reason as
  the predecessor: the readings require a CI run of committed code, CI fires only on push, and
  `/andromeda-implement` reserves committing for wrap. Recorded as a deviation with that attribution.
- **/implement edited `plan.md`** — forbidden by its own constraints, directed by the operator, who named the
  precedent channel. Recorded as an operator override, not a defect.
- **A pre-existing cosmetic defect surfaced, not fixed:** `a11y-plan.md:115` repeats the clause "the one
  webview-automation stack running on the measured platform SET — " twice in a row, from an earlier verbatim
  apply. No detector covers it and it is not this chunk's; editing a master outside the proposal flow is
  exactly what the constraints forbid. Operator to route.
- **Arm B's first attempt produced no reading** and is recorded rather than omitted — it died with its error
  invisible (`[Console]::Error.WriteLine` writes the raw console handle, which a PowerShell `*>` redirect
  does not capture). A capture-correct retry with a sentinel produced it. One hypothesis for that death
  (toolchain unresolvable on the inherited PATH) was tested and REFUTED.
- **Last failed command:** none.
