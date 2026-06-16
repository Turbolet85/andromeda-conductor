# Fan-out drift results + main's verdicts — 2026-06-16-emission-journal-writer wrap

7 doc-agents, one per spec source. Raws consolidated here (proposals + main validation).

## security-plan · design-system · layout-templates · obs-plan · a11y-plan → `proposals: []`
Clean. No UI surface (design/layout/a11y), no MCP/sidecar/new-input (security-input/subprocess), no new external crate (security-deps — serde_json/thiserror already in tree, assert_fs test-plan-named; audit/deny to run at the wrap gate), no OTel SDK (obs-stack), no host-path/struct-name leak (obs-redaction — negative test proves it), no a11y schema divergence.

## arch → D-arch-resources (warning) — **DISMISSED (misfire)**
Proposed: add an "Exported Library Types" registry to §Occupied Resources listing `RunRecord`/`now_rfc3339`/`JournalWriter`/`JournalError`.
**Verdict: dismiss.** The invariant enumerates IPC/endpoint/event/socket/port/env-var/workspace-crate — the chunk added NONE. Internal library types are out of the invariant's scope, the arch deliberately registers cross-seam *contracts* (the envelope SHAPE is already in §Standard Contracts; the JSONL journal artifact in §Occupied Resources), and the public type names are tracked by `api-surface.md` (reconciled in P4). Adding a register-every-type convention is scope creep. (Akin to the playbook dismiss-misfire precedent.)

## test-plan → 2 proposals
### D-tests-framework (warning) — **ROUTINE (apply)**
Golden mechanism: the chunk used exact-string `assert_eq!` unit goldens (matching the established `verdict.rs`/`report_state.rs`/`scenario.rs` canonical-serialization pattern) + dropped `insta` from `conductor-report` dev-deps. test-plan §6/§7 reserve insta (with redaction) for the **E2E journal + Markdown** goldens (real runs, varying run_id/timestamps) — a different test level, unchanged. Apply a §4 clarification: unit canonical-serialization goldens may use exact-assert; E2E journal/Markdown goldens use insta+redaction (§6/§7). Routine per playbook rule 5.

### D-tests-obs-harness (warning → **RECLASSIFIED ESCALATE**)
The agent proposed clarifying that the JSONL omits `read_back_observed_at`. On inspection this is NOT a safe routine clarification — it would amend the spec to match the code, and the evidence says the **code is the outlier**:
- My `RunRecord` JSONL = 10 fields (no `read_back_observed_at`), following **obs-plan §3**.
- But the schema OWNER **test-plan §3** (status-shape + polled fields) carries `read_back_observed_at`; **test-plan §6/§7** journal goldens **redact** `read_back_observed_at` (you can only redact a field that's present); **arch §Standard Contracts** example carries it; the **runs.db data model** stores both timestamps. obs-plan §3 (which I followed) is the lone omitter, and obs is supposed to *align to* test-plan, not the reverse.
- The JSONL line is written post-read-back (it carries verdict/state/latency), so `read_back_observed_at` IS known at write time — no temporal reason to omit it.
→ **Escalated to the user** (do not auto-amend a spec to match possibly-wrong code; do not silently ship a wrong canonical contract).
