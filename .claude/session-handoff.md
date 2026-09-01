# Session Handoff

**Last Updated:** 2026-09-01T17:10:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **37 ahead** after this commit)
**Status:** clean
**Last Commit:** `feat(2026-08-31-p-075-assert-round): …`

## Position
- Done: **2026-08-31-p-075-assert-round** — the read-back fidelity Conductor can honestly assert, proven
  live: `mark_incident_resolved` fired across the wire for the first time and the incident left the active
  set **0.0s after its last emission**, against Pulse's 120s auto-resolve idle threshold.
- Next: **`/andromeda-phase`** to promote + plan the next Epoch-5 entry — **_Webview self-verify on the
  Windows host_** — which now carries the **41st** `cargo audit` PREREQ (re-pinned in FULL form; see Notes).
- Coverage unchanged at **21/32 verified · 11 unclaimed** — this chunk deliberately claimed no capability.

## Work done
Drove a live Pulse (HEAD `83d4060`, release binaries, deterministic L4, fresh data dir) to a known incident
and resolved it through MCP. Landed the first production caller of `mark_incident_resolved`
(`conductor_run::probe_resolve_lifecycle`), the applied/declined stub arms, and 11 harvest tests over the
pinned capture. Recorded the payload-fidelity premise disproved on a second axis (`incident_events` reaches
no MCP tool) and quoted `v2-20`'s existing fixtures as P-075's external evidence instead of re-asserting
three already-verified budgets.

## Drift resolved
**17 amendments across 4 masters · 2 escalations resolved with the operator · 9 sidecar entries · cascade
closed over 4 derived leaves (8 more dispositioned no-change).**
- `architecture.md` ×4 — payload fidelity disproved on a second axis; **"no stronger claim exists" RETIRED**
  (runtime-STATE fidelity is the stronger claim and is live-proven); the one-active-incident dedupe
  constraint; the workspace-key mechanism is now the SUT's *published* key with `data_dir` as fallback
  (3 sites).
- `security-plan.md` ×4 (**escalated**, operator-ratified) — "corpus access via MCP read-back ONLY" retired
  at four sites for "via the MCP tool surface only — read-back plus the `mark_incident_resolved` write —
  never the file". The file ban and non-persistence guarantee are unchanged in force.
- `test-plan.md` ×6 — §5's "`mark_incident_resolved` has NO production call site and no live exercise"
  falsified on both halves; stub item-key fidelity recorded; a cargo-feature gate registered as a third
  sanctioned live-leg path (§9 + §11).
- `obs-plan.md` ×3 — the MCP boundary carries a write; §6 must-log set names it. **Plus an
  operator-ratified incidental correction:** `obs-plan.md:35` still said "via rmcp 1.7.0" (removed
  2026-06-27) and sat inside the sentence being rewritten.

## Notes
- **The leg's proof is attributional, not just observational.** Pulse's corpus independently records
  incident 6 active `16:42:38 → 16:43:23` — a 45s life against a 120s idle threshold — so the auto-resolver
  is excluded by construction and Conductor's write is the only remaining cause.
- **Two plan premises died to measurement.** (1) The **two-incident control** (operator-selected at P4) is
  structurally unattainable: Pulse dedupes a new incident against any OPEN one regardless of fingerprint, so
  at most one is active per workspace. Replaced by liveness attribution. (2) The **declined arm** is
  unreachable through MCP — `DeclinedStale` is a monotonic-timestamp guard the dispatch cannot trip — so it
  is stub-proven and permanently so.
- **The bug that cost four legs:** the live wire emits `incident_id`, the stub emitted `id`. A reader keyed
  on `id` returns an empty list from a populated response — green against every stub test. Three incidents
  sat active 131–142s each while the leg logged `[]`. Fixed at three sites; pinned by
  `lifecycle_harvest::the_live_item_key_is_incident_id`.
- **41st `cargo audit` PREREQ, re-pinned in FULL form.** The 40th was discharged HERE (it was an acceptance
  criterion but absent from the plan's Test Commands, so it would otherwise have re-pinned unmeasured):
  signature reproduced byte-identically — audit true exit 1 / `duplicate advisory ID: RUSTSEC-2026-0244`,
  deny true exit 0. The compact form was NOT available because the basis changed: this chunk touched
  `crates/conductor-run/Cargo.toml` (+6). It admits **zero packages** (empty `[features]`, `Cargo.lock`
  byte-unchanged), so deny's coverage set is identical and the conclusion stands.
- **Cross-version residual appended:** `incident_events`-through-MCP → Pulse 0.4.0 (a SUT-side capability
  gap, not Conductor work).

## Deferred learnings
2 candidates landed at EXACTLY 0.6 on Filter 4 — the documented mass point that rejects. Both are sound and
will return with more signal:
- *Pre-existing drift inside a sentence an amendment is rewriting is not dismissible on pre-existing
  grounds.* Operator-ratified this session for `obs-plan.md:35` (rmcp) and applied by analogy to
  `security-summary.md:14` ("bounded prost decode" → JSON-RPC/serde_json).
- *Pulse's `agent-run boot` pre-builds a DEBUG profile* — with release binaries warm, spawn
  `target/release/pulse-app.exe` directly, which is what its boot ultimately does.

**recurrence-despite-learning:** the 2026-06-27 stub-fidelity entry (`verification-harness.md` — the
rmcp-server stub HID the raw-shape bug) did not prevent this chunk's recurrence one level down. Envelope
fidelity was fixed then; ITEM-KEY fidelity was never stated, so the stub drifted again on a different axis
of the same contract. The new `testing.md` entry extends the class rather than restating it.

- Audit trail: `.andromeda/runs/2026-09-01T16-50-00Z-wrap/` (report is at the chunk dir; fan-out results +
  per-doc detectors here).
- **Last failed command:** none.
