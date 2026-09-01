# Session Handoff

**Last Updated:** 2026-09-01T19:19:00Z
**Branch:** build/conductor-0.2.0 (tracks `origin/build/conductor-0.2.0`; **38 ahead** after this commit)
**Status:** clean
**Last Commit:** `feat(2026-09-01-webview-self-verify-windows-host): …`

## Position
- Done: **2026-09-01-webview-self-verify-windows-host** — an agent CAN drive the real Tauri window on this
  host. Live WebView2 151.0.4129.107 session, axe injecting and analyzing, `browser.execute` reading `:root`
  tokens, colorjs.io contrast computed from the running app. The route entry's premise ("no driver binary
  exists on this host") was measured **false**.
- Next: **`/andromeda-phase`** to promote + plan **_Desktop a11y sweep_** — **reordered ABOVE _Live per-P-ID
  verdict lamps_** by operator pre-direction, so the harness is calibrated to zero before any UI chunk
  develops against it. It carries the **42nd `cargo audit` PREREQ (COMPACT form — basis restored)**.
- Coverage unchanged at **21/32 verified · 11 unclaimed** — this chunk deliberately claimed no capability
  (making v2-22 runnable is a partial advance, which stays `chunk: null` per the contract).

## Work done
Repointed `--e2e` in both harness shells from `cargo nextest -p conductor-cli` to the tauri-driver `wdio run`
leg that test-plan §3/§9 already specified, behind a `CONDUCTOR_MSEDGEDRIVER` guard that **skips at exit 0**
with a fetch recipe when unset. Reused the operator's host driver rather than acquiring one; `setx`-persisted
and verified in the registry. Also re-scoped the CARRY's over-scoped dedupe claim in source.

**Four pre-existing defects surfaced, none in the plan** — all hidden because the config had never once been
executed since June:
1. **`__dirname` in a `"type": "module"` package** — the committed `wdio.conf.ts` could not LOAD on ANY host,
   Linux included. `typecheck:e2e` passed throughout, because tsc accepts it under the node types.
2. **`cargo build --release` does not embed the bundle.** At source: `tauri`'s `build.rs` computes
   `dev = !custom_protocol`, so the **FEATURE** decides, not the profile. The plain release binary opened a
   Chromium error page at `localhost:5173`.
3. **No readiness synchronisation** — and `waitUntil` treats a THROWING condition as fatal, so the first fix
   aborted instantly instead of waiting.
4. **wdio v9 routes `execute` through BiDi**, which answers "Page/Frame is not ready" indefinitely against
   wry/WebView2 while classic WebDriver succeeds — `wdio:enforceWebDriverClassic` required.

## Drift resolved
**~40 amendments across 6 masters · 5 escalations resolved with the operator · 6 sidecar entries · cascade
closed over 8 leaves · 1 new detector · 1 playbook rule refreshed.**
- `test-plan.md` ×15 — the "Linux+`xvfb` only" verdict retired as a CAPABILITY claim at **8** sites (the
  plan's list named 6; `:287` and `:369` also deferred GUI legs), "headless only" → "non-interactive only",
  `--e2e`'s custom-protocol precondition, the loopback allowlist, and §4's ESM-loadability limit.
- `architecture.md` ×9 — the **CARRY** (one-active-incident re-scoped to the `(kind, scope, scope_id)` tuple,
  citing `inference_runtime.rs:811`), `CONDUCTOR_MSEDGEDRIVER` + ports `4444`/`4445` registered, the
  custom-protocol build mechanism, and two operator-ratified qualifications (trust boundary → shipped
  binaries; **"no UI automation" scoped to PULSE's UI**).
- `security-plan.md` ×8 (**escalated**, operator-ratified) — the spawn ban SPLIT into MCP-sidecar +
  harness-spawn rules, the inbound-listener ban scoped to shipped binaries, and the new harness boundary
  registered at five enumeration sites. Playbook rule @58 did NOT dismiss these: this chunk genuinely adds a
  boundary and a spawn, so its precondition was unmet.
- `a11y-plan.md` ×7 (**orchestrator-raised** — see below) · `obs-plan.md` ×3 · `layout-templates.md` ×1.

## Notes
- **A detector blind spot cost nothing only because the plan's list is a floor.** The a11y agent returned
  `proposals: []` — correctly, since its two detectors cover new-UI-element coverage and violation-schema —
  while its document carried six stale platform sites. It flagged the gap itself. **Resolved: a new
  `D-platform-claim` detector is now in `drift-base.md`** (operator-approved), the first cross-doc one.
- **The CARRY's own site table was incomplete**, exactly as the directive anticipated by mandating a
  re-sweep: it named four sites; the sweep found a **fifth** (`crates/conductor-run/tests/lifecycle_live.rs:128`),
  which no detector would ever reach since the cascade never greps source comments. Corrected in-code.
- **My own P3 finding was wrong and is retracted.** "Zero `#[tracing::instrument]` across 7
  `#[tauri::command]` sites" — all 7 carry the manual `tracing::info_span!("tauri.command.*")` that
  `observability.md` prescribes *because* the attribute doesn't stack. The grep tested a token the project
  forbids. Obs-plan was the divergent side and was amended; **no route entry minted** (directive item 2).
- **`--e2e` is now legitimately RED (exit 1)** and that is recorded, not hidden: 2 specs pass live, 1 fails on
  a **genuine axe violation**, 2 fail because their subject (HOLD dialog / checklist) exists only in a driven
  live-Pulse run. The sweep entry carries this verbatim with the context-skip shape to give those two.
- **42nd `cargo audit` PREREQ, COMPACT form.** Signature reproduced byte-identically this wrap — audit true
  exit **1** / `duplicate advisory ID: RUSTSEC-2026-0244`, deny true exit **0**. The compact basis is restored
  because this chunk touched no manifest and `Cargo.lock` is byte-unchanged.
- `crates/conductor-tauri/ui/logs/` is now git-ignored — the driven app writes its self-obs sink relative to
  the launching cwd, which the root-anchored `/logs/` rule did not cover.

## Deferred learnings
Filter 4's 0.6 mass point rejected three individually (custom-protocol-not-profile; tsc-does-not-prove-ESM-
loadability; wdio-BiDi-vs-classic) — each scoring measurement + specific-detail and nothing more. None was
lost: two landed as **corrections** to existing entries (exempt from the cap) and all three compose into the
one new Tier-2 entry in `frontend.md`. The two candidates parked at the 2026-09-01 wrap remain parked.

- Audit trail: `.andromeda/runs/2026-09-01T18-49-38Z-wrap/` (fan-out results + escalation dispositions).
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-09-01 19:19:00
