# Fan-out results — 2026-10-04-host-portable-tauri-ipc-tests

Report: `conductor-0.3.0/chunks/2026-10-04-host-portable-tauri-ipc-tests/report.md`. Seven Explore doc-agents, one
parallel batch; prompt per `amendment-flow.md` §Fan-out, `{contracts_line}` dropped for all seven (`registry.py
contracts` → `NOT MIGRATED` for architecture / test-plan / obs-plan / a11y-plan; `n/a` for the other three).
`D-platform-claim` (cross-doc) was sent to every doc. Returns arrived as plain YAML with `#` commentary; no HTML
entities in any return (no `&lt;` / `&gt;` / `&amp;`), so the decode probe reads `entities=0` for all seven.

## Verdicts
- architecture — 1 proposal (below). Commentary stripped: per-detector no-hit notes for D-arch-resources /
  D-arch-decisions / D-arch-collision / D-arch-registry-size, and a duplicate-occurrence sweep over the doc
  (`tauri.localhost`, `tauri://`, `localhost`, `custom-protocol`, `origin`, mock symbols, `opened on`, the denial text)
  naming `:214` the only stating site and `:150` (devUrl `http://localhost:5173`) a non-hit.
- security-plan — `proposals: []`. Stripped: no new input surface / spawn / dependency; `:184 :370 :384 :396` name the
  origin-confusion CVE, not an origin value or a platform verdict.
- design-system — `proposals: []`. Stripped: no UI; `26`/`27` hits are hex values and line numbers.
- layout-templates — `proposals: []`. Stripped: no surface; `step 14/27` at `:211` is a sample timeline counter.
- test-plan — `proposals: []`. Stripped: the new test sits at the standing mock-runtime dispatch tier (`:290-291`,
  `:307`, `:440`); `26`/`27` hits are tool versions and a column number; no sentence states a Windows-only IPC tier.
- obs-plan — `proposals: []`. Stripped: no operation, dependency, log write or CI gate change.
- a11y-plan — `proposals: []`. Stripped: no UI element, no schema change; `origin` hits at `:76`/`:333` unrelated.

## architecture — the one proposal
```yaml
- detector: D-platform-claim
  severity: warning
  section: §Infrastructure Patterns → Build system
  change: In the 2026-09-01 measurement parenthetical, qualify the custom-protocol origin by OS instead of stating it as one value — "...opened on `http://tauri.localhost/` (measured on the Windows host; tauri 2.11.3's `tauri_protocol_url` is per-OS: `http://tauri.localhost` under `cfg!(windows) || cfg!(target_os = "android")`, `tauri://localhost` on Linux/macOS — the mock-runtime IPC tests therefore derive the request origin from `window.url()`, never a literal)". Keep the Windows value; never swap it for a Linux literal.
  sidecar: §Infrastructure Patterns → Build system — the 2026-09-01 custom-protocol origin `http://tauri.localhost/` is now marked as the Windows form; the origin is per-OS (`tauri://localhost` on Linux/macOS) per tauri 2.11.3 manager/mod.rs:339-346, confirmed on the Linux host by a 27/27 green run and an inverse control.
  rationale: report Spec-claims-disproved (c) + Expected amendments; the stating text is "the same build with `--features tauri/custom-protocol` opened on `http://tauri.localhost/`"; the agent notes the fit to D-platform-claim is loose (a single-platform measured fact stated as if it held everywhere, not a where-the-harness-can-run verdict).
  basis: tauri 2.11.3 manager/mod.rs:339-346; webview/mod.rs:1698-1706; .andromeda/architecture.md:214
```
**Disposition: APPLY (routine).**
- Check 0 (re-derivation tell): the basis cites the tauri registry source, which the report's Cross-project bullet
  carries (`manager/mod.rs:339-346`, `webview/mod.rs:1698-1706`) — not a re-derivation.
- Check 1 (playbook): GOVERNED by the seed rule "Accurate this-chunk addition" (`routine`) — the per-OS origin is this
  chunk's measured fact (report Changes: Cross-project + Spec claims disproved), landing in an existing section, and the
  invariant survives (the Windows measurement stays true; only its scope is named). Under its DETECTOR label it would
  fall to the playbook `:46` dismissal (D-platform-claim fired on a sentence stating no platform / runner / driver
  verdict — an origin value is not the detector's class), so the detector attribution is recorded as loose; the
  amendment stands on check 5 below, not on the detector.
- Check 2 (cross-contradiction): no other proposal touches the section.
- Check 3 (intent-consistency): within the plan's own Expected-amendments entry; scope record none.
- Check 4 (absence needs evidence): "`:214` is the only site" rests on two searches — the report's python sweep of all
  seven masters (1 hit) and the agent's per-doc sweep. `:214` is a multi-KB line (the Build-system paragraph): the hit
  is resolved by reading the whole line (read in full at P1 of this wrap), and the stating clause is the 2026-09-01
  parenthetical "the same build with `--features tauri/custom-protocol` opened on `http://tauri.localhost/`".
- Check 5 (expected amendments): plan entry 1 (architecture §Infrastructure Patterns → Build system) → MATCHED by this
  proposal. Plan entry 2 (`.claude/rules/testing.md:65`) is a curation item → P3.
- Check 6 (disproved claims): (a) the `commands.rs` doc comment — fixed in the chunk's own diff; (b)
  `.claude/rules/testing.md:65` — routed to P3 curation (an in-place correction of the 2026-06-27 Session Addition);
  (c) `architecture.md:214` — this proposal. All three DISPOSED.
- Applied text is re-derived from the report, not pasted: the `window.url()` test-design clause is NOT carried into
  arch (it is test-plan / rules territory and arch §Build system states build facts) — the body gains the host scope and
  the per-OS rule only.

## Escalations
None.
