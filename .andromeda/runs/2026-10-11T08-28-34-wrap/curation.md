# Curation — 2026-10-11-conductors-window-retired

Session-local scope. This wrap resumed a run whose first window ended after P1: that window's conversation is gone,
so only this window and the report's *Decisions & corrections* section were sources.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  2 corrections (cap-exempt)
    ~ "Scope law …" — [corrected 2026-10-11]: the dev-only webview legs' 4444/4445 bind retired with the window
      Proof: report.md Changes → Symbols / APIs (Ports: "4444 / 4445 lose every binder"); `wdio.conf.ts` deleted.
    ~ "Status is never color-alone …" — [corrected 2026-10-11]: the desktop label + glyph form retired
      Proof: report.md Changes → Files (`crates/conductor-tauri/` deleted whole, its `ui/` webview included).
  Tier 2 (.claude/rules/*):                   + security.md: "A silent `cargo deny check licenses` is no evidence an allowed license still has a holder …" (confidence 0.8)
      Proof: report.md Changes → Schema / config (`Apache-2.0 WITH LLVM-exception` kept on 16 holders counted from `cargo metadata`) and Decisions & corrections ("`cargo deny`'s silence on a license is the reading only because `unused-allowed-license` defaults to warn"); evidence/supply-chain.md.
      Signals: verified by measurement +0.4 · specific technical detail +0.2 · reached no other durable home +0.2 (no master, route annotation, playbook/drift-base rule or matrix note carries it).
  Tier 3 (.claude/docs/session-learnings.md): + "Predict a lockfile prune by pruning, and prove it by the name+version set" (confidence 0.8)
      Proof: report.md Spec claims disproved (plan's 562 → 270 vs the measured 268 / 294; the base resolve's unified features kept `futures-io` and `ipnet`) and Dependencies (68 inserted lock lines are unqualified re-writes of 13 single-version names); evidence/lock-delta.md.
      Signals: verified by measurement +0.4 · specific technical detail +0.2 · reached no other durable home +0.2.
  Filters: 0 dup · 2 task-specific (the "Conductor has no `main` branch" note — branch names; the operator-pass evidence-after-push ordering — one-off) · 0 conflict · 0 deferred
  Not candidates: "a classifier refusal of a founder-ruled act is a halt with options" (the prior window's act, paraphrased from the report — −0.2, below threshold).
  No-other-home: "a silent cargo deny is no evidence an allow has a holder" · "predict a lockfile prune by pruning"
  CLAUDE.md size: 137/200 · T1 6.1 KB, 0 over 600 B
