
## 2026-09-22-interpretation-proven-live — the real-model posture on Critical Path 1, the fingerprint-storm read-back re-based, parity scoped

**Section:** §1 Must-trace table, the Fingerprint-storm row (`:131`) · §4 Critical Path 1 (Headless deterministic scenario run) — NEW "Real-model posture" bullet (`:305`) · §4 Fingerprint-storm (`:315` `verify.readback_fingerprints` attribute · `:316` required log fields) · §4 Critical Path 7 required log fields (`:369`)

**Change:**
**(O1)** Critical Path 1 records the real-model posture: the declare-only `real-model-interpretation` scenario adds NO critical path, span name or span attribute — it rides this chain with the eleven-field envelope, `verdict` null, a non-degraded read-back landing `ManualCheck`; its interpretation is graded at the harvest tier, never through the envelope; `execute_scenario`'s posture-mismatch `Blocked` (before the ready check) is one `info` line on the allowlisted `message` field — never a sixth gate precondition. The one drive was blocked at the preflight, so the chain was not observed live under that posture.
**(A17 cascade) ESCALATED, operator "Amend now, as measured"** — the Fingerprint-storm row and §4 block: at `efabe8e` no read-back surface carried Pulse's computed fingerprint; at `83d4060` `fingerprint_refs` also carries each incident's triggering-cue fingerprint (the grounded union, measured 2026-09-10 — architecture [Read-Back Dependency Posture]), so an emitted-vs-read-back match is now computable, though no span attribute or shipped check computes it; `fingerprints_read_back_count` stays a COUNT.
**(T12 cascade) ESCALATED, operator "Narrow + record"** — Critical Path 7's parity (identical `verdict`/`state` for the same seed) is stated for DETERMINISTIC-posture scenarios; a real-model scenario is outside parity by design (the Tauri path keeps the deterministic gate and records it scenario-level `Blocked`).

**Why:** report §Changes (Symbols/APIs — the mismatch `Blocked` and its message-borne line; Schema/config — no envelope key, verdict word, state, span name or attribute added; Spec claims disproved 4 — `ManualCheck`; Cross-project claims — `grounded_fingerprint_hashes`) and the plan's Expected amendment "obs-plan §4 — the family recorded in the existing path's detail block". The plan's "§5/§10 — the pickup figure" entry is dispositioned with NO edit: no figure was measured, and obs-plan carries 0 `pickup|formation` hits, so nothing states a figure to correct.

**Sweep (full ledger `.andromeda/runs/2026-09-23T08-03-55-wrap/reconcile-sweep.md`):**
- **G5 payload** — `:131`, `:315` ×2, `:316` amended; no other obs-plan hit (0 remaining for `payload-invariant|no read-back surface`).
- **G4 parity** — `:369` amended; `:49`, `:146`, `:590` left — they state parity's MECHANISM (envelope comparison, not trace correlation), while the scope lives at `:369`; `:136`, `:361` are path names; `:370` sits inside the scoped block.
- `--live|live-suite|preconditions --|pickup|formation|real-model|posture` over obs-plan before the edit: 0 hits — the §3 bind with test-plan §3 is untouched (no harness, status-read or log-format change).
- Leaves: `docs/obs-summary.md` and `.claude/rules/observability.md` state parity's mechanism only (no scope, no fingerprint claim) — checked, no re-derivation needed.
