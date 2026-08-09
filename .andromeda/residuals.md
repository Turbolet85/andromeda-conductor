<!-- Cross-version residuals. One line per entry, master-route grammar minus the link tail:
  {origin-marker} · open · {text} (target: {version-hint|next})
Two writers total: /andromeda-wrap-session route-resolve APPENDS `open` entries; /andromeda-route
Phase A decides dispositions and Phase 6 flips `open → absorbed:{cap-id} | re-carried:{version} |
dropped ({why})`. Nothing else writes here. -->

2026-08-09-interpretation-correctness-posture · open · Interpretation-correctness real-model leg — under deterministic L4 (`ANDROMEDA_PULSE_L4_DETERMINISTIC=true`) a canned `L4Output` replaces the inference, so every live leg exercises the plumbing and never the interpretation: "Conductor green" does not mean Pulse's interpretation is trustworthy. Prove the real thing — deterministic mode OFF, a known root cause injected, the top hypothesis asserted to identify it (the P-033 path Pulse's own memo calls the single most important test in all of Pulse). Needs real per-check read-back extraction (v2-09) AND a non-deterministic live leg that can never be a CI gate, which is why it is not an 0.2.0 entry. Covers the diagnostic-quality cluster P-031/P-033/P-034/P-044, pinned meanwhile in `conductor_core::UNBACKED_AUTO` and asserted by `check_scenario_backing`. (target: 0.3.0)
