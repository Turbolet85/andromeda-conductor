# Fan-out results — 2026-08-20-verifier-self-hardening

7 doc-agents, one per spec source. **11 proposals: 6 applied · 5 rejected · 4 escalations resolved with the operator · 0 open.**

## Per-doc verdicts

| doc | proposals | verdict |
|---|---|---|
| arch | 0 | clean — no new symbol/crate/port/env var; the only dependency deltas are a REMOVAL and a dev-feature flag, neither contradicting §Stack or a locked decision |
| security-plan | 0 | clean — no new external-input surface, no sidecar/spawn/data-dir change; §Dependency Security's 2026-08-16 clause honored explicitly (deny verified green over the post-removal lock) |
| design-system | 0 | clean — no UI element; none of the moved counts appear in the doc |
| layout-templates | 0 | clean — no surface/region; the one literal collision (`step 14/27` in the indicatif sample) is a coincidental digit, not the survivor count |
| a11y-plan | 0 | clean — no interactive element; violation schema untouched on both sides |
| **test-plan** | **8** | 6 applied · 2 rejected (see below) |
| **obs-plan** | **3** | **all 3 rejected** — mis-citation (see below) |

## Applied (6, all test-plan)

1. **§4 Framework** — `cargo test -p <crate>` named a standing runner-portability gate beside nextest.
2. **§4 Mutation instrument** — cargo-mutants 27.1.0 registered with its run discipline (workspace-root `-f`, mandatory `--test-tool=nextest`, `Found 0 mutants` is a NO-OP, gitignored output) and added to the floors-not-pins policy.
3. **§9 CI Integration** — scoped mutation recorded as an operator/local instrument on the live-Pulse footing, explicitly not a CI stage.
4. **§10 Quality Gates** — mutation-survivor disposition added as a non-blocking audit-tier rule (killed OR accepted-deliberate against a cited rule, never a numeric threshold); runner-dependence recorded as a determinism break.
5. **§11 Integration** — NEW ban: a process-global first-install-wins singleton is not isolated by nextest's per-test process; give it its own test binary, never serialize the file.
6. **§12 Decisions Log** — 2026-08-20 entry recording the adoption, the measured run-discipline findings, and the `declares` accepted-deliberate ruling.

Sidecar: 5 entries in `.andromeda/test-plan-amendments.md` (written AFTER re-reading each edited body).

## Rejected (5)

**obs-plan ×3 — rejected on verified evidence, operator-confirmed.** All three rest on the premise that obs-plan §4's **cli** row mandates `#[tracing::instrument]` on `fn main()`. It does not: line 48 (the cli row) names "`tracing` 0.1.x crate + `tracing-subscriber` JSON formatter" as the surface's hook; the `#[tracing::instrument]` mandate is on the **desktop-webview** (:49) and **ipc-internal** (:50) rows, both about `#[tauri::command]` handlers. The cli row describes the SURFACE's mechanism, which `conductor-core`'s `init_observability` supplies (`main.rs:28`), so removing `conductor-cli`'s direct edge does not falsify it. The §1 proposal targets that same line 48 (§1's "Telemetry surfaces:" heading at :44 owns the table), and the §6 target (line 477, `conductor-cli | info`) is a level *policy* that stays valid for a target emitting nothing.

**This is the SECOND occurrence in one session** — the phase-time obs distiller made the identical mis-attribution (corrected in `research.md` §Extract correction). A `playbook.md` rule was appended so the third occurrence is classified rather than re-litigated.

**test-plan ×2 — rejected, operator-confirmed.**
- §11 Test Data: proposed REPLACING "must be caught by a golden test" with this chunk's three-assertion unit-tier set. Rejected — the chunk ADDED a mechanism, it did not disprove the mandate, and the two catch different things (a stamp helper frozen at a constant vs a paused-clock leak reaching the journal artifact). Retiring an unsuperseded check is not warranted.
- §11 Universal: proposed carving an exception into "NEVER use real time without injection" for the new stamp test's 2ms `std::thread::sleep`. Rejected — the line's own second clause ("`std::time` only for ground-truth journal stamps") already scopes the ban to scheduling and excepts exactly this subject.

## Validation notes

- **Check 5 (expected-amendments floor):** the plan's `Expected amendments (wrap)` named test-plan §4/§9/§10 for cargo-mutants. All three were proposed by the detector and applied; §12 was added as the natural home for the adoption decision. Floor met, not under-run.
- **Check 6 (disproved-claims disposition):** the report's two entries are prior-run code-audit artifacts (§B1's 27-vs-28 enumeration, §B3's `--test-tool=nextest` command-field claim). Both are run history, never edited; dispositioned to the audit-ledger note in the report for the next boundary's audit to consume. Not spec-master amendments.
- **Check 2 (cross-contradiction):** none — the two docs with proposals touch disjoint sections.
- **Cascade:** grepped all seven masters + the three preserve-verbatim curation homes + the two judgment bases for the amended wording. No cross-master citation of the §4 framework line or the tool inventory exists, so no re-basing was owed. Leaves re-derived: `.claude/rules/testing.md` (Frameworks · Test data & isolation · Quality gates) and `.claude/docs/tests-summary.md` (harness contract), both with `## Session Additions` preserved verbatim (25 entries intact).
