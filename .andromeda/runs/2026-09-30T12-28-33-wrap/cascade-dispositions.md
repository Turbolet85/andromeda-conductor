# Cascade dispositions — 2026-09-30-the-screen-reader-pass-grades-again-on-this-host

The pass's one amendment: O1 — a11y-plan §Screen reader test pattern → *Per-surface test spec* (`:268`), the NVDA
measured platform set bound to its configuration (the 154.0.4258.37 / 26200.9457 / NVDA 2026.2 focus silence under both
object models). It EXTENDS the standing claim ("agent-driven with all three subjects attached (2026-09-02)"), it retires
nothing — so the sweep looks for every other site that states the SR verdict without the configuration binding.

**Search.** `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1), baseline `2c9b37d4`, over the seven
masters, `.andromeda/registries/**`, the curation homes, the judgment bases and the leaves:
- `platform-set` fixed `measured platform set` — 2 standing · 0 leaf (control a11y-plan.md:268)
- `three-attached` fixed `all three subjects attached` — 1 standing (control a11y-plan.md:268)
- `nvda-agent` regex `NVDA[^.;]{0,60}agent-driven|agent-driven[^.;]{0,60}NVDA` (i) — 5 standing · 3 leaf (control test-plan.md:47)
- `speech-log` regex `via (NVDA.s|its) (own )?speech log` (i) — 3 standing · 2 leaf (control test-plan.md:47)
- dropped before the run: `sr-heard` (`focus rows?… heard|announced|agent-driven`) — its control never fired on the
  pre-pass masters (the phrasing is this pass's own), so it is no absence proof and was not used as one.
0 curation rows, 0 base rows for every pattern.

**Rows.**
| row | disposition |
|---|---|
| a11y-plan.md:268 (platform-set, three-attached, nvda-agent ×2, speech-log) — `edited` | AMENDED (O1). The ×2 is the section's own "AGENT-DRIVEN via NVDA's own speech log" plus the new "NVDA's Chromium object models" clause; no retired mechanism stands (nothing was retired). |
| a11y-plan.md:333 (platform-set) | no change — the ROUTINE webview arm's measured platform set (Linux/xvfb target, the Windows dev host, `windows-2022`), a different arm; no SR claim. |
| a11y-plan.md:83 (nvda-agent) | no change — §1 surfaces row states NVDA is agent-driven and supplemental (measured 2026-09-02); it states driven-ness, not a focus-hearing verdict; the configuration binding lives in §3, which it defers to. |
| a11y-plan.md:113 (nvda-agent, speech-log) | no change — §1 summary of the SR pattern: agent-driven + the LIVE-region must-announce items (empty-state prose, HOLD `aria-live`), which the 2026-09-30 legs still hear (R0-01); no focus-hearing claim (read whole, 1 238 bytes). |
| a11y-plan.md:298 (nvda-agent) | no change — bootstrap-phase pointer to the pattern. |
| test-plan.md:47 (nvda-agent, speech-log) | no change — "The `focus` / `live` rows ARE agent-driven via NVDA's speech log" — the rows were driven and graded this chunk; driven-ness, not a hearing verdict (test-plan doc-agent concurs). |
| .claude/rules/a11y.md:39 @c2830 (nvda-agent, speech-log) — leaf | RE-DERIVED — the Testing bullet's SR clause now carries the configuration-bound focus verdict (window `cascade.py window --line 39 --at 2830` read). |
| .claude/docs/a11y-summary.md:27 (nvda-agent) — leaf | RE-DERIVED — bootstrap item 5 carries the configuration-bound focus verdict. |
| .claude/docs/a11y-summary.md:35 (nvda-agent, speech-log) — leaf | no change — a never-rule (no second automation stack; SR supplemental only); true under both configurations. |

**Leaves by provenance** (step 3 floor + enumeration): `grep -ln a11y-plan .claude/docs/*.md .claude/rules/*.md` → a11y-summary,
rules/a11y, rules/frontend, rules/observability, rules/testing. frontend / observability / testing carry 0 `NVDA` and no SR
verdict (frontend's one `screen-reader` hit is a path, `test/a11y/screen-reader/`) → no change. CLAUDE.md: its only a11y-plan
reference is the pointer-table row (`:65`, §3) — still correct; the `GENERATED:setup:warnings` block carries no SR claim.
