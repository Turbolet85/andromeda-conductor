# Fan-out results — 2026-09-10-release-build-and-bundle

7 Explore doc-agents, one batch. **10 proposals across 4 docs; 3 docs clean.** Returns carried prose
commentary around the YAML (stripped) and HTML-escaped entities in one rationale (`Mutex&lt;()&gt;` →
`Mutex<()>`, decoded; `entities=0` on the decoded text). No return failed validation, so this consolidated
record is the audit artifact for all seven.

## Verdicts

| doc | proposals | disposition |
|---|---|---|
| arch | 4 (1 primary + 3 `dependent-of`) | APPLIED |
| security-plan | 2 (1 primary + 1 `dependent-of`), both `escalate` | 1 APPLIED · 1 ESCALATED → resolved → APPLIED |
| design-system | 2 (1 primary + 1 `dependent-of`) | APPLIED |
| test-plan | 2 (independent) | 1 ESCALATED → resolved → APPLIED · 1 APPLIED |
| layout-templates | `proposals: []` | — |
| obs-plan | `proposals: []` | — |
| a11y-plan | `proposals: []` | — |

## Proposals and their dispositions

**arch — D-arch-decisions ×4.** Primary at §Infrastructure Patterns — Deployment model (`:206`), the
sentence the report names as carrying two false facts; dependents at §Stack Desktop-shell row (`:27`),
§Established Decisions [Deployment] (`:58`), §Inherited Defaults — Deployment (`:266`). APPLIED as one
amendment across four sections. The agent explicitly declined the CRATE sites and said so.

**security-plan — D-security-deps ×2, `escalate` severity.**
- `:84` Hosting — the cross-master co-citation of the arch claim. APPLIED (cascade fold).
- `:181` Update policy — **ESCALATED**: the "(+ bundler)" half of the CVE-floor sentence attributes a
  tree-resolved version to something no lockfile resolves. Outside the operator's named scope and touching a
  security floor, so resolved WITH the operator: apply the split, crate half byte-intact. APPLIED.

**design-system — D-design-derived-count ×2.** `:117` Depth Strategy rationale (primary) and `:367`
desktop-webview ban (dependent), both carrying `~3 MB`. APPLIED as SET-NAMING per the detector's own fix
rule — the design argument is untouched, only the baked number is gone.

**test-plan — D-tests-coverage + D-tests-framework.**
- §11 Integration — **ESCALATED**: widening the process-global-singleton remedy from the exclusive "a test
  binary of its own" to a SET. Two sound specs collided (test-plan's exclusive remedy vs arch [Module
  Boundaries]); resolved WITH the operator → widen with the discriminator named. APPLIED.
- §4 Tool-version policy — `tauri-cli` added to the external-CLI floor roster. This is also the chunk plan's
  own `Expected amendments (wrap)` entry, reconciled at Validate check 5. APPLIED.

## Validate

| check | result |
|---|---|
| re-derivation tell | none rejected. One `basis` cited `obs.rs:393-405`, a source line the report does not carry — its substance is wholly report-derived, and the pointer was verified directly before acceptance |
| 1 playbook | **no rule governs the measured-scalar class** (`:127` needs a set-enumerating literal · `:106` needs prose already declaring the value derived · `:28` needs values preserved, which is what moved). Applied on the operator's directive; gap reported |
| 2 cross-contradiction | none — no two proposals edit one section in opposing directions |
| 3 intent-consistency | aligned; the report's Outcome table records every acceptance criterion met |
| 4 absence needs evidence | all three absence claims re-derived independently: `~3 MB` ×2 in design-system (post-edit 0) · four zero-hit probes in test-plan · a11y-plan's zero-token grep |
| 5 expected-amendments | the plan's one entry (test-plan §4 tauri-cli) was PROPOSED by its own detector; the route-resolve hand-off is correctly not an amendment |
| 6 disproved-claims | the report's single entry (architecture.md:206's two facts) is DISPOSED — matched by 8 proposals across arch, security-plan and design-system |

## Cascade

Cross-master sweep keyed on every amendment of the pass — `2.11.3`, `3 MB`, `bundler`,
`test binary of its own`, `tauri-cli` — over all seven masters, the three preserve-verbatim curation homes
and the two judgment bases.

- Retired claims now **0** across the seven masters and the derived tier (sidecars excepted — they are
  append-only history and are never swept).
- **CRATE facts intact and deliberately untouched**, verified by lock probe (`tauri` 2.11.3 present;
  `tauri-cli` / `tauri-bundler` absent): `architecture.md:204`, `security-plan.md:181` (crate half) and
  `:379`, `.claude/rules/verification-harness.md:57`.
- One sweep hit was a **false positive read and dismissed**: `CLAUDE.md:131` matched `3 MB` on
  "returning 1.3 MB of library internals" — an unrelated curated learning inside `USER:session-learnings`,
  about grep false positives. Read the hits; no action.
- `playbook.md` / `drift-base.md`: 0 hits, nothing routed to propose→approve→append.
- **Leaves re-derived:** `.claude/docs/stack.md:45` · `.claude/docs/commands.md:8` ·
  `.claude/rules/testing.md:18,34`. No plan summary (`security-` / `design-` / `tests-summary.md`) and no
  CLAUDE.md `GENERATED` block carries an amended fact — verified by marker-aware scan, so nothing further
  derives.

**drift = 0 on exit.** 10 proposals: 8 applied directly, 2 escalated and resolved with the operator before
any apply. Zero open.
