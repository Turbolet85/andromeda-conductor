# design extract

## Relevance
Partial — A1 (the indicatif bump vs the stop-in-place hold spinner) and the version/floor statements that live inside design-system.md are design's; A2/B1a/B1c/B2/B3/B4/B6/B7 are outside the domain, and B5 touches it only as a deletion hazard.

## Constraints
- The operator-pause spinner MUST stop in place at the exact count value — not hide, not clear, not animate to 100% — with a bold amber `HOLD — operator pause` phase line printed **above** the `inquire` prompt; this is the CLI mirror of the project signature and the freeze *is* the event (per design-system.md §Surface: cli / Component Patterns 1 and §Motion — High-impact moments). Whether the shipped 0.17.11 code already realizes this, and whether the bumped API preserves it, is research's question.
- Spinner draw discipline that any API delta must carry over: it appears only after ~200ms (honest progress, not instant churn) and is TTY-gated so agent-captured artifacts stay clean (per design-system.md §Surface: cli / Component Patterns 1).
- If the bump forces `ProgressStyle`/template rewrites, the output must still emit only the named ANSI-map values (114 nominal green · 179 hold amber · 203 fail red · 117 ID-cyan · 246 residual mute) behind the per-stream `std::io::IsTerminal` gate (terminal AND `NO_COLOR` unset AND `TERM != dumb`), with the ASCII prefixes always standing — never color alone, never new colors (per design-system.md §Surface: cli / Tokens).
- At expression level `0.3` the CLI's entire motion budget is the spinner tick; no animation library and no new animated affordance may ride in on a dependency bump (per design-system.md §Motion — Hard limits for this expression level).
- design-system.md §Surface: cli / Toolkit-Framework names **`indicatif` 0.18** as target state while scope A1 measures the pin at 0.17.11; the same line names `inquire` 0.7 against a measured 0.9.4. Whether the bump lands on 0.18 (satisfying the plan) or elsewhere (leaving the line as drift for wrap's amend flow) is research's question — the amendment write is not this chunk's (chunk Boundaries).
- design-system.md carries **two** `Tauri ≥ 2.10.3` statements — §Surface: desktop-webview / Platform-Specific Notes and §Design Decisions Log (Security bullet). Both are security FLOORS, still satisfied at the resolved 2.11.3; neither is a version claim to reconcile. Scope B1b names only the Decisions-Log site, so the Platform-Specific-Notes one is an unnamed second floor site (per design-system.md §Platform-Specific Notes, §Design Decisions Log).
- Primitives the plan mandates but that mount in only one place — the Operator-checklist, whose shipped mount is inside the operator-pause dialog with the run-report site designed-but-not-built — must not be deleted on a `knip` dead-code report (per design-system.md §Component Patterns 7). Installing knip (B5) is in scope; acting on its output against designed primitives is not.

## Patterns to follow
- §Surface: cli / Component Patterns 1 — the exact hold-point contract (stop-not-hide, ANSI 114 green running prefix, bold ANSI 179 amber HOLD line) the bump must be measured against.
- §Surface: cli / Component Patterns 2 — the `inquire` prompt the stopped spinner sits above: `isatty`-gated, headless path never blocked, decision recorded to the artifact. Any indicatif↔inquire interleaving change must preserve this ordering.
- §Surface: cli / Tokens — the ANSI 256 map applied via `owo-colors` + `std::io::IsTerminal` is the reuse source if styling code is touched; bind by existing tier, never mint a row.
- The spec-illustration → sound-impl reconciliation routine established across this plan's amendments (`@theme`→`:root`, literal 200ms→`--motion-micro`, `anstream` retired) is the shape for the `indicatif 0.18` / `inquire 0.7` toolkit line: record the resolved mechanism/version, keep the mandated behaviour untouched.
- The de-hardcode rule from the 2026-08-08 / 2026-08-09 amendments — name the set or use a placeholder rather than substituting a fresh literal that will re-stale — applies to how the version line is reconciled.

## Anti-patterns to avoid
- Hiding the spinner, clearing it, or animating it to 100% at the operator-pause — a named Rejected Default and a cli Per-Surface Ban; it destroys the signature (per design-system.md §Anti-Patterns / Per-Surface Bans — cli, and §Rejected Defaults).
- Adding a new palette row / ANSI entry to satisfy a changed indicatif styling API — the standing precedent is reuse of the existing tier (per design-system.md §Surface: cli / Tokens; the `hint:` grey correction).
- Emitting color without the ASCII prefix, or colorizing without the pipe/`NO_COLOR`/`TERM` check, in any re-templated progress line (per design-system.md §Anti-Patterns / Per-Surface Bans — cli).

## Contract bindings
- **cli color gating ↔ a11y SC 1.4.1 (not-color-alone):** the `[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` prefixes are the color-free channel; a11y owns the criterion, design owns the token pairing.
- **Spinner stop-in-place ↔ tests harness:** the signature behaviour needs a verification route in the CLI presentation leg; whether one exists today is test-plan's/research's question, not design's to assert.
- **Tauri floor sites ↔ security-plan:** the `≥ 2.10.3` statements in design-system mirror security-plan mandates; the floor-vs-version disposition (B1b) is security's call — design only records that both mirrored sites are floors.
- **opentelemetry-proto trim (A2/B3) ↔ obs-plan §3:** no design surface — the trim changes no token, no CLI output, no journal rendering; design asserts nothing about it.

## Acceptance criteria contributions
- (design) After the bump, the operator-pause spinner stops in place at the exact value — no hide, no clear, no animate-to-100% — and a bold amber `HOLD — operator pause` line prints above the `inquire` prompt (per design-system.md §Surface: cli / Component Patterns 1).
- (design) Any spinner/style code touched by the bump emits only the existing named ANSI values (114/179/203/117/246) with their ASCII bracket prefixes, behind the per-stream `IsTerminal` + `NO_COLOR` + `TERM != dumb` gate; zero new colors and zero new palette rows (per design-system.md §Surface: cli / Tokens).
- (design) The resolved indicatif version is recorded against design-system's `indicatif 0.18` toolkit line as reconciliation SHAPE for wrap's amend flow — not silently edited in this chunk, and not left unstated (per design-system.md §Surface: cli / Toolkit-Framework).
- (design) Both `Tauri ≥ 2.10.3` statements in design-system.md (§Surface: desktop-webview Platform-Specific Notes and §Design Decisions Log Security bullet) survive the B1b sweep untouched as security floors (per design-system.md §Platform-Specific Notes).

## Relevant amendment history
- **2026-09-03-live-pulse-preconditions-probed** — retired `anstream`/`anstyle`; the named cli gate is `owo-colors` + `std::io::IsTerminal`, per-stream. Its scope note explicitly refuses the proposal's `indicatif 0.18` / `inquire 0.7` assertions (manifest read `0.17.11` / `0.9.4`) and states design-system's standing `indicatif 0.18` is **owned by the route's Dependency-polish entry** — i.e. this chunk. Directly governs A1's doc side.
- **2026-06-24-sanitized-stderr-agent-mode-logging** — the `error:`/`hint:` edge REUSED shipped tokens (203/246); a proposed new "Hint grey" palette row was corrected away at validation. The precedent for reuse-over-add if the bump forces restyling.
- **2026-06-15-design-token-typography-bundle** and **2026-06-26-component-primitives-library** — establish the spec-illustration → sound-impl reconciliation routine (playbook :28) used when an artifact contradicts an illustrative spec value while the invariant holds; the governing shape for the version-line reconciliation.
- **2026-08-08-sut-capability-manifest / 2026-08-09-current-sut-coverage-classification / 2026-08-09-sut-load-envelope** — the de-hardcode discipline (name the set, placeholder over literal, never substitute a fresh count) that reconciliations in this chunk should follow so they cannot re-stale.
- **2026-09-02-screen-reader-manual-spec** — records that the Operator-checklist primitive mounts only inside the operator-pause dialog and that the run-report site is designed-not-built; relevant as the deletion hazard when `knip` (B5) first reports dead code on the UI package.
