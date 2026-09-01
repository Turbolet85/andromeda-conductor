# Scope — Desktop a11y sweep

**Marker:** `2026-09-01-desktop-a11y-sweep`
**Version:** conductor-0.2.0 · Epoch 5 — Verification surfaces
**Working entry:** _Desktop a11y sweep — operator judgment pass over the accessible paths, with axe and
routine affordance proof agent-driven_

---

## What this chunk is

The predecessor chunk (`2026-09-01-webview-self-verify-windows-host`) proved the harness **runs** on this
host. This chunk makes what it runs **true**: it calibrates the `--e2e` a11y leg from its measured
2-of-5-passing state to a green verdict that actually asserts the accessible paths, and it draws the line
between what an agent proves routinely and what stays the operator's judgment call.

The chunk's own definition of done is a leg whose red, if any, is a real defect — not an artefact of a spec
asserting a subject that was never on screen.

## Boundaries — what this chunk does NOT do

- **Not the screen-reader manual spec.** NVDA/VoiceOver/Orca must-announce specs are the separate working
  entry _Screen-reader manual spec_ (matrix `v2-23`). [verified against `verification-matrix.json`]
- **Not the A11y CI gate or the violation JSON envelope.** Emitting service-tagged violation JSON, the
  redaction boundary over it, and a CI gate consuming it are the separate entry _A11y CI gate_ (matrix
  `v2-24`). This chunk's leg stays an operator/local gate. [verified against `verification-matrix.json`]
- **Not the coverage lamps.** _Live per-P-ID verdict lamps_ (matrix `v2-30`) is the entry this one was
  deliberately reordered ABOVE; no coverage-view work here.
- **Not re-authoring the driver harness.** `wdio.conf.ts`, the tauri-driver stack, the
  `CONDUCTOR_MSEDGEDRIVER` guard and the skip-arm all landed last chunk and are used as-is. Extending the
  *assertion* surface (the specs) to cover the paths the acceptance names is in scope; rebuilding the
  *transport* is not. [verified: `wdio.conf.ts` loads and drives a real session; the specs are the assertion surface]
- **No new automation stack.** Scope law: one WebdriverIO + tauri-driver session, never a second stack;
  Conductor's own webview only, never Pulse's UI.

## The display-gate question — resolved HERE, at route level

The working entry states this explicitly, and it is the chunk's organising decision:

- **Agent-driven legs** (routine affordance proof, no operator eyes): axe, contrast, not-color-alone,
  keyboard-trap escape, focus restoration, reduced-motion.
- **Operator-gated**: the **judgment class** only — the ManualCheck items and an occasional holistic pass.

This resolves the inherited "operator-gated live-Pulse pass" framing of the entry's earlier wording. The
operator's eyes are not spent on affordance proof an agent can drive.

## Inherited state — the measured contents of the red (CARRY, 2026-09-01)

The `--e2e` leg is **driven-red (exit 1)** on entry, and the red has exactly two contents, both measured:

- **(a) 1 genuine axe violation** on the Minimal-tier baseline (`wcag2a`/`wcag2aa`/`wcag21aa`).
- **(b) 2 specs whose SUBJECT does not exist outside a driven live-Pulse run** — the operator-pause
  `alertdialog` and the operator-checklist rows.

**Two specs already pass live** — Blocked-vs-Fail token distinctness, and WCAG AA contrast at 4.5:1 / 3:1 —
so the sweep starts from **2/5 passing, not from zero**.

The CARRY prescribes the treatment for (b): **the context-skip shape — subject absent ⇒ skip-with-reason,
never fail.**

**Re-verified at fold time** (named coordinates only): `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts`
carries exactly the five `it()` blocks the CARRY describes, in the stated pass/fail split; the harness leg is
`npm run a11y` → `wdio run wdio.conf.ts`, dispatched from `agent-run.{sh,ps1} run --e2e` after a
`--features tauri/custom-protocol` release build.

- **The rule is named, by measurement (P3, leg re-run 2026-09-01).** The predecessor recorded the violation
  as a COUNT (`exactly 1`); decoding the axe payload out of the driver log resolves it to
  **`color-contrast`** (wcag2aa / SC 1.4.3, impact `serious`) with **18 nodes, every one the same
  foreground token `--text-tertiary` (`#717aa0`)** — 3.50:1 on `--color-raised-2` (the coverage table
  header strip) and 3.78:1 on `--color-raised-1` (the rows), against a required 4.5:1. So the defect is
  **one token value, not eighteen markup sites**.
  [premise-corrected: measured — `research.md` §The violation, named]
- **The token failure is WIDER than the leg measured.** axe scores only what was rendered. Computing
  a11y-plan §6's nine required pairs from the shipped `tokens.css` finds a third failing background
  (`--text-tertiary` / `--color-base` at 4.06:1 — latent, used by six empty-state prose sites in
  `App.tsx`) and a **second failing token in both themes** (`--text-muted` / `--color-inset` at 2.91:1
  dark, 3.86:1 light) that axe never saw. Fixing only the 18 measured nodes would leave the capability's
  own acceptance ("every named token pair meeting its required ratio") unmet.
  [premise-corrected: measured — `research.md` §The 18 nodes are a PARTIAL view]

## The tension this chunk must resolve

The context-skip shape and the capability's acceptance pull against each other, and the resolution is the
chunk's central design question:

- `v2-22`'s acceptance requires **keyboard-trap escape and focus restoration working** across the accessible
  paths. Those live in the operator-pause dialog spec — one of the two whose subject is absent.
- If that spec merely **skips**, the acceptance is not proven; claiming `v2-22` on a skipped assertion would
  be a hollow `verified`.
- Therefore a **driven leg in which the subject IS present** (a live run that actually raises the hold and
  renders the checklist) is what separates a green leg from a proven capability. The context-skip shape is
  the correct behaviour when the subject is absent — it is not a substitute for asserting it once.

`v2-22` is additionally an **affordance cap** under the matrix contract: its acceptance asserts keyboard
interaction (Escape resolving NoGo, a row toggling on Space), so it requires affordance-level verification —
a real keypress through a headful driver — and is not exempt as pure-visual. tauri-driver supplies exactly
that. [verified: `OperatorPauseDialog.tsx:33` is the Radix `AlertDialog`; the driver drives real keys]

**The gating mechanism is measured (P3, read at source).** `conductor-run/src/lib.rs:533-543` fires the hold
only when preflight is READY, `route_read_back` is not `Blocked`, and the scenario is declare-only; checklist
rows additionally require a `[[checklist]]` declaration, which exactly two committed scenarios carry
(`halo-hue-encoding` P-025, ~6s — the cheaper target; `halo-breathing-encoding` P-026). Both subjects are
correctly built already — `role="alertdialog"` comes from Radix, and `OperatorChecklistView.tsx:18` already
carries `role="status" aria-live="polite"` — so **specs 4 and 5 fail on absence alone, not on a defect.**
Making them present requires a live, preflight-ready Pulse, which is precisely why this leg is
operator/local and never CI.

## Surfaces and contracts touched

| Surface | Expected involvement |
|---|---|
| `crates/conductor-tauri/ui/test/a11y/accessibility.e2e.ts` | the assertion surface — repair, extend to the acceptance's paths, add the context-skip shape |
| `crates/conductor-tauri/ui/src/styles/tokens.css` | the failing token VALUES in both theme blocks — resolved by P3 to a token defect, not markup; no renames, no new colours |
| `crates/conductor-tauri/ui/test/a11y/operator-hold.e2e.ts` | NEW — the driven arm's spec (the real HOLD dialog) |
| `crates/conductor-tauri/ui/wdio.conf.ts` | a `suites.driven` entry + routine-only default `specs`; the driver guard, capability and readiness hook stay untouched |
| `crates/conductor-tauri/ui/package.json` | an `a11y:driven` script for the driven suite |
| `scripts/agent-run.{sh,ps1}` | unchanged — Arm B runs via npm directly; the `--e2e` mapping is settled |

**The two-arm shape (operator decision, P4).** The sweep splits into an **unattended Arm A** (`--e2e`, which
must reach exit 0) and a **driven Arm B** (operator/local, live preflight-ready Pulse) that raises the real
HOLD so `v2-22`'s keyboard clauses are asserted rather than skipped. Arm B is what makes the capability
claimable instead of hollow; it is separated by a wdio *suite*, never a second config or automation stack.
| `.andromeda/a11y-plan.md` §3/§5/§6 | the binding contract for paths, keyboard, contrast pairs, motion |
| `contracts/` | untouched — no SUT-facing manifest changes expected |

- **A stale platform claim sits in the spec source itself** — `accessibility.e2e.ts`'s header comment still
  reads "runs on Linux+xvfb + live Pulse … **never on the Windows dev host**", which the predecessor measured
  false. It is the same class as the CARRY's fifth site: source comments no detector greps. The same header
  also attributes the SR manual spec to "the next chunk", conflating two separate route entries. [verified: read at source, `accessibility.e2e.ts:1-6`]

## Invariants this chunk must not break

- **Status is never color-alone** — every state carries text label + glyph; the not-color-alone assertion is
  a path, not decoration.
- **Never weaken a gate to make it green.** A spec that fails on a real defect is fixed at the defect; a spec
  whose subject is absent is skipped *with a reason*, never deleted or asserted vacuously.
- **Minimal tier stays the baseline** — `wcag2a`/`wcag2aa`/`wcag21aa`, no `wcag22aa`.
- **Selectors stay role/text/aria-live** — never xpath, never hashed CSS classes.
- **Skip-arm green without `CONDUCTOR_MSEDGEDRIVER`** — the unset handle must still exit 0 with the
  host-path-free precondition and fetch recipe.
- **Redaction** — no absolute host paths in any artifact this chunk produces.

## PREREQ — the 42nd consecutive `cargo audit` re-check (COMPACT form)

Standing deferral since `2026-08-08-sut-capability-manifest`; ratified at the
`2026-08-10-workspace-key-divergence-probe` wrap. The **compact form** is available because the basis is
restored: the predecessor touched no Cargo manifest, admitted zero package nodes, and left `Cargo.lock`
byte-unchanged.

**Discharge (unchanged):** reproduce the PROBE-AUTO-SATISFY signature —

- `cargo audit` true exit **1**, first diagnostic `duplicate advisory ID: RUSTSEC-2026-0244`
- `cargo deny check advisories bans licenses sources` true exit **0**

capturing **each exit code BEFORE any pipe**. Any deviation restores the full form. Remedy stays the bounded
wait: **no floor raise, no `deny.toml` ignore, no CI edit.** Close the moment the advisory DB parses.

Full rationale: `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md` (verified
present at fold time).

## Capability this chunk is expected to claim

`v2-22` — _Desktop a11y sweep asserted_. Its acceptance is inherited from a route-era framing that names
**Linux+xvfb** and an **operator-gated** run; both premises were superseded by measurement last chunk (the
Windows host drives the real webview, and affordance proof is agent-driven by this entry's own ruling).
Concretization at P5 must re-aim the acceptance onto what this host actually proves **without weakening the
outcome** — the same paths, asserted for real. [verified: both premises measured false by the predecessor; concretization is a P5 task, not a premise]

`v2-23` / `v2-24` stay pooled — they are their own route entries.
