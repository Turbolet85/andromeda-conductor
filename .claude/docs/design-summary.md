# Design Summary — Conductor

_Distilled from `.andromeda/design-system.md` + `.andromeda/layout-templates.md`. setup-project Phase 3. wrap-session does not modify._

**Conductor has no interface:** its window — the `desktop-webview` surface — retired 2026-10-11
(`2026-10-11-conductors-window-retired`, intent R1). The cli is the one surface it renders; the webview's typography,
spacing, depth, radius, motion, iconography and component sections are one-line stubs, their text at `321dc8f`.

## Brand identity
**Mission-control patience — armed, nominal, go/no-go.** A single-operator console that stays calm and resolute under load: terse status callouts, no alarm in the tone even on a Fail. "A control surface, not a dashboard." **Signature:** the *Paused-count hold-point* — on the cli, the `indicatif` spinner STOPS in place (never hides, never animates to 100%) at an operator hold. **Expression level 0.3** (honest progress feedback only).

## Key tokens (the cli's ANSI map binds the palette by name)
| Token | ANSI | Use |
|---|---|---|
| `count-nominal` | 114 | on-timeline heartbeat + `Pass` |
| `count-hold` | 179 | operator-pause HOLD + `CalibrationRegion` |
| `status-fail` | 203 | `Fail` (no blink) |
| `count-blocked` | 60 | `Blocked` (never measured) |
| `color-id-cyan` | 117 | P-IDs / run_id / SLO timings / fingerprints |
| `status-residual` | 246 | `KnownResidual` + the shared recessive tier (`hint:`, the out-of-scope Mode cell, `[ENVIRONMENT-SUSPECT]`, `[PRECONDITION]`) |

## The cli surface
- The verb set `Commands` declares (layout-templates §cli Primary screens enumerates it), line-oriented (clap + owo-colors + indicatif + comfy-table + inquire), ANSI TTY-gated per stream, NO_COLOR / `TERM=dumb` honored.
- Status is never colour-alone: every per-P-ID line carries its bracket prefix (`[PASS]`/`[HOLD]`/`[FAIL]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]`); the run-level captions sit outside the lamp column.
- Three table shapes, never conflated: the `run`/`suite` results table is 6 columns sized to their content (no terminal-width detection); `conductor coverage` is 4 columns sized to their content, with a roll-up caption whose `(N unbacked)` qualifies the auto term, never a fifth summand, omitted at zero; `conductor scenarios` is 3 columns (Scenario · P-IDs · SLO), no bracket prefix, the one table sized to the terminal width, ending in its `1 scenario` / `N scenarios` roll-up line. Never hardcode widths or wrap at arbitrary points.

## Universal bans
- No KPI-card-grid "dashboard"; no colour purely for decoration (colour = Verdict/ReportState).
- No flashing/pulsing on `Fail`; no progress spinner that hides or animates-to-100% on the operator-pause.
- No emoji in piped output; never block the headless path on a prompt.

---

**Full plans:** `.andromeda/design-system.md` + `.andromeda/layout-templates.md`. Path-scoped rules `.claude/rules/frontend.md` + `a11y.md` are retirement stubs.
