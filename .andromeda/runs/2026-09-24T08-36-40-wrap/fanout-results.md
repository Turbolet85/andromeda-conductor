# Fan-out results — 2026-09-24-architecture-registries-compacted-under-the-read-cap

Seven Explore doc-agents ran in one parallel batch. Each got the verbatim amendment-flow prompt with its own
drift-base detectors substituted. Every return was plain YAML with nothing to strip, and none contained
HTML entities.

| doc | verdict | proposals |
|---|---|---|
| architecture | 1 proposal (raw twin `.raw-fanout-architecture.md`) | D-arch-decisions · §Stack "Operator instruments (host runtime)" row gains `scripts/arch-registry-check.py`; "Neither … neither" → "None … none" |
| security-plan | `proposals: []` | none — the new CLI's inputs are validated, its only subprocess is an argv-list `git show`, and there are no deps |
| design-system | `proposals: []` | none — no UI; the moved counts have no site |
| layout-templates | `proposals: []` | none — no surface; the old values are not present |
| test-plan | `proposals: []` | none — selftest + can-fail control; the 4 `mutation-gate.py` hits state no instrument count |
| obs-plan | `proposals: []` | none — no self-obs stream; no redaction surface |
| a11y-plan | `proposals: []` | none — no interactive element; schema unchanged |

## Validate
- **Playbook:**
  - The one proposal matched no rule. `:171` fails its "moves ONLY a measured-scalar literal" precondition
    because the change adds a name.
  - It was applied under the operator's recorded direction, the P5-approved Expected amendment 3.
  - A new rule was minted on approval (operator, this wrap).
- **Cross-contradiction:** none.
- **Intent-consistency:** aligned with the scope and the working entry (headroom target, wrap-time detector).
- **Absence evidence:** the architecture agent's "only :39 states the count" matches
  `grep -c 'mutation-gate.py\|arch-registry-check'` (architecture 1) from the report.
- **Expected amendments:**
  - #1 and #2 were raised by the orchestrator as routine: plan-approved, with the instrument PASSing and the
    judgment rows reviewed.
  - #3 is the proposal above.
- **Disproved claims:** the `residuals.md:15` → `architecture.md:69` citation has no sanctioned writer, so it is
  routed to the handoff's still-carried list beside the existing `residuals.md:11` carry.
- **Escalations:** 0.
- **Approvals (AskUserQuestion, this wrap):**
  - D-arch-collision appended, severity escalate;
  - D-arch-registry-size appended, severity warning;
  - the playbook rule for operator-instrument registration minted, routine.

## Owed faithfulness review (plan item, wrap P2)
All 151 judgment rows (150 `rewritten` + 1 span-less `in-sidecar`, verbatim in its entry) were read, BEFORE
sentence against the AFTER sentence holding its anchor. 4 were found wanting and corrected in the drafts and
ledger before apply:
- `31363eb4ccbf` was anchored on the NVDA bullet's matching phrase, not its own `CONDUCTOR_MSEDGEDRIVER` bullet.
  It was re-anchored.
- `cdf78de8bd1c` / `bb63fab3388a`: the dev-host green tally and its "measured green on two configurations" basis
  were dropped. Both were restored to [CI/CD].
- `84d82f34b935`: the three-leg basis of the integrity-label discriminator was dropped. It was restored.

After the corrections, `check` → PASS (§Established Decisions 37 907 B, §Occupied Resources 37 929 B) and
`selftest` → every arm detected.
