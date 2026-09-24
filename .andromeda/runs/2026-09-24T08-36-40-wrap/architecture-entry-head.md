
## 2026-09-24-architecture-registries-compacted-under-the-read-cap — registries compacted under the read cap; the operator-instrument row gains a third member
**Section:** §Established Decisions and §Occupied Resources (each replaced whole) · §Stack and Technologies ("Operator instruments (host runtime)" row)
**Change:**
(1) Both registry sections were replaced verbatim by the chunk's drafts `compaction/{established-decisions,occupied-resources}.md`.
- Sizes:
  - §Established Decisions: 49 134 → 37 907 B.
  - §Occupied Resources: 48 859 → 37 929 B.
  - The limit is 38 115 B, which is 60 % of the 25 000-token Read cap at 2.541 B/token. Measured by `scripts/arch-registry-check.py measure --file .andromeda/architecture.md` → `registries: within target`.
- Kept: every label, registered name, port, crate and scoped qualifier.
- Left the body: dated narrative (chronologies, superseded readings, CI-run stories and elimination stories).
- Per-sentence accounting is in `conductor-0.3.0/chunks/2026-09-24-architecture-registries-compacted-under-the-read-cap/compaction/disposition-ledger.toml`: 579 rows (kept 352 · rewritten 150 · moved 68 · in-sidecar 9), and `check` PASSes.
- The 151 judgment rows were reviewed at this wrap, and 4 were corrected before apply:
  - a `CONDUCTOR_MSEDGEDRIVER` skip row anchored on the NVDA bullet was re-anchored on its own bullet;
  - [CI/CD] regains the dev-host green tally and its two-configuration basis;
  - [CI/CD] regains the three-leg basis of the integrity-label discriminator.
- The passages that left the body are the entries below, each headed by this marker, one per decision label or sub-registry. They are verbatim: either history, or the BEFORE wording of a sentence the body now states more briefly. History this sidecar already carried is not repeated; the ledger's `in-sidecar` rows name the entries that hold it.

(2) §Stack row: `scripts/arch-registry-check.py` is registered as the third committed operator instrument, and "Neither … neither" becomes "None … none".
**Why:** The body holds only current truth (this file's preamble), yet it had carried its own amendment log: 132 ISO dates, 13 CI run ids and 35 sha-like tokens across the two sections. It was also growing ≈1 KB a day at wrap toward the Read cap.

Cascade sweep over the seven masters, CLAUDE.md, `.claude/rules`, `.claude/docs`, playbook and drift-base:
- (a) The retired two-instrument wording: 1 hit, `.claude/docs/stack.md:40`, which was re-derived.
- (b) 4 696 quoted fragments of ≥ 3 words, of which 17 are present in HEAD's body and absent from the new one. None needed a change:
  - 8 are text between two separate code spans (a regex join, not a quote);
  - 4 are a master's own history with no arch citation on the line (security-plan :367, test-plan :471 ×3);
  - 4 sit in curation homes (CLAUDE.md :136, `verification-harness.md` Session Additions ×3);
  - 1 is a generic token.
- Every section citation into §Established Decisions / §Occupied Resources resolves to a claim the compacted body still states: 16 in security-plan, 5 in test-plan, 1 in obs-plan, 2 in a11y-plan, and 10 in the leaves.
