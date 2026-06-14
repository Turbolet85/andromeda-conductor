## Signature Candidates

### Candidate 1: "The verdict lamp — in-place status-light resolve on the P-ID row"

**What it is:** An interaction-on-a-structural-element. Each capability row (P-001..P-060) in the coverage matrix carries a single fixed-position status glyph that begins as a dim outlined ring ("armed / pending"), and at the moment that capability's reaction is verified against its SLO the ring fills to a solid lamp in its verdict tier — phosphor/nominal green `#7EE787` (Pass), degraded amber `#E3B341` (CalibrationRegion / operator-pause), reserved red (Fail), plus dim-grey hatch (Blocked precondition). The fill is a single 180-200ms opacity-and-fill crossfade in place — the row never moves, reflows, or animates position; only the one lamp transitions, exactly like an annunciator lamp lighting on a dark console. The verdict text token (mono face) appears beside it in the same beat.

**Implementation:** A shadcn/ui table/row with the lamp as an inline SVG (outline circle + fill circle) toggled by a Tailwind v4.1 utility class on verdict state; transition is `transition-[opacity,fill] duration-200` only — no framer-motion, no layout animation, satisfying the ~3 MB artifact and 0.3 budget. In the CLI surface the identical convention renders in comfy-table as an owo-colors-tiered cell glyph (green/yellow/red `●` vs dim `○`), so both surfaces share one verdict-lamp vocabulary.

**Core-action tie:** Fires on the product's single most-repeated moment — a capability's reaction being verified and resolving to a typed verdict (Pass/Fail/CalibrationRegion/Blocked), which the operator does 60 times per full suite run.

**Brand fit:** This is "armed, nominal, go/no-go" rendered literally — a dim ring is *armed*, the fill is the lamp lighting *nominal/hold/no-go*, no alarm theatrics. It is the k9s "status light, not a sentence" and Apollo annunciator-lamp read on the cool-slate `#1A1B26` ground, and the in-place 200ms fill is precisely the functional-motion-only 0.3 ceiling (no spring, no entrance).

### Candidate 2: "Paused-count hold-point — the frozen heartbeat at the operator-pause"

**What it is:** A structural + interaction element centered on the run's count clock. While a suite runs, a single prominent mono count (elapsed run time / step index, the run's "heartbeat") ticks in place at the top of the control surface. When an operator-pause prompt fires before a committed timeline step, the count does NOT blank or keep running — it freezes at the exact hold value and the phase label beside it flips to "HOLD — operator pause" in amber, with the count's color shifting from green-nominal to amber-hold while held. On proceed, the count resumes from the frozen value; on abort, it stops and goes dim. This borrows the NASA Open MCT "paused count shows where it stopped" convention directly.

**Implementation:** A React 19 component holding run-elapsed state, count rendered in the reserved mono face; the freeze is just halting the tick interval and applying an amber Tailwind text class — zero animation library, a CSS color transition at most. The CLI mirror is an indicatif progress/spinner that stops (not hides) at the hold and a colored owo-colors "HOLD" phase line above the inquire operator-pause prompt, keeping TTY artifacts clean.

**Core-action tie:** Triggered by the operator-pause go/no-go prompt — the staged "hold before you commit the next step" action that gates every committed timeline step in a run.

**Brand fit:** The held, frozen count is the literal "Mission-control patience" and the orchestral "held silence / fermata" spine in one element — the run waits, visibly, at a named hold-point. Amber-on-slate for hold, calm and resolute, no flashing; the freeze (absence of motion) IS the signature, which is the purest possible expression of a 0.3 "exact downbeat, held silence" budget.

### Candidate 3: "Block-per-step run stack with the verdict edge"

**What it is:** A structural element. Each scenario step renders as a discrete collapsible block (Warp-style "git commit for your run") stacking down the surface as the run accrues — step header + emission summary + verdict. The signature detail: each completed block carries a single 2px left border-edge in its verdict tier (green/amber/red/grey-blocked), and is collapsed-by-default to a one-line summary; a Fail block can be expanded for the emission detail without the run timeline losing its footing. The colored left edge is the only chroma on an otherwise flat inset-border block, so a scan down the stack reads as a column of status edges — the run's whole verdict history at a glance.

**Implementation:** shadcn/ui Collapsible/Accordion primitives (already vendored) with a Tailwind `border-l-2` verdict-tier class; flat inset-border elevation (no fill, no shadow-heavy cards), matching Linear's depth model. Collapse/expand is the Radix default height transition kept instant/short — within 0.3. CLI analog: Warp-style discrete step blocks already map to line-grouped output with a tiered status glyph per step header.

**Core-action tie:** Contains the core action as its payload — each block IS one scenario's emission + verdict, the unit the operator watches accrue across a run.

**Brand fit:** Warp's "calm stack of committed, inspectable steps" + Linear's flat inset-border-with-one-accent depth on the cool slate — armed-and-orderly, not busy. Progressive disclosure (collapsed by default, expand the Fail) is the 2026 calm-interface pattern and the 0.3 restraint: structure and a colored edge carry the signature, not motion.

## Recommended

**Recommended signature:** 1 — "The verdict lamp — in-place status-light resolve on the P-ID row"

**Reasoning:** It is the only candidate that fires on the product's true core action at its highest frequency (a capability resolving to a typed verdict, 60× per suite) AND renders identically on *both* of Conductor's surfaces, so the one recognizable element — a dim ring arming, then a single lamp lighting green/amber/red in place — becomes the product's shared visual signature in the webview (shadcn row + Tailwind transition) and the CLI (comfy-table + owo-colors glyph), which a solo technical operator will read instantly and a team-dashboard never offers. It synthesizes every prior pick without strain: "armed, nominal, go/no-go" is the literal arm-then-light mechanic, the green/amber/red `#7EE787`/`#E3B341` tiers on `#1A1B26` slate ARE the k9s-on-Warp mood, and the in-place 200ms opacity fill (no position change, no spring) sits exactly on the 0.3 functional-motion ceiling. Candidate 2 (paused-count) is the strongest complement and should ship alongside as the operator-pause moment, but the verdict lamp is the signature because the verdict — not the pause — is the thing the operator is here to produce.

**Research basis:** WebSearch June 2026 — Muzli "50 Best Dashboard Design Examples 2026" + 925studios "SaaS Dashboard Design Examples 2026" (minimal green-pass/red-fail status color system, "is everything okay?" answered by a single status element, progressive disclosure behind deliberate interaction); Wikipedia "Launch status check" + "Built-in hold" + ESA/NASA Artemis II terminal-count go/no-go poll (armed → go/no-go lamp, paused-count hold-point convention). Reinforced by training-data-2026 surface anchors carried in prior Q research: Linear (flat inset-border depth, mono status-token tier, 0.3 webview), k9s (in-place color-tiered status lamps under load), NASA Open MCT (annunciator/go-no-go vocabulary), Warp (calm committed-step stack).

Sources: [Muzli — Best Dashboard Design 2026](https://muz.li/blog/best-dashboard-design-examples-inspirations-for-2026/), [925studios — SaaS Dashboard Design 2026](https://www.925studios.co/blog/saas-dashboard-design-examples-2026), [Wikipedia — Launch status check](https://en.wikipedia.org/wiki/Launch_status_check), [Wikipedia — Built-in hold](https://en.wikipedia.org/wiki/Built-in_hold)
