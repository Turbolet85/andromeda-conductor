# Fan-out results — 2026-09-30-the-sr-pass-regrades-on-the-os-input-path

Seven Explore doc-agents in one batch, prompts built by `build-prompts.py` from `doc-agent-template.txt` (amendment-flow
§Fan-out, verbatim) and `.andromeda/drift-base.md`. No doc carries keyed contracts: `registry.py contracts` exits 3
`NOT MIGRATED` for architecture, test-plan, obs-plan and a11y-plan, and exits 0 `n/a` with no file for the other three,
so no contracts line was sent. `D-platform-claim` (doc: all seven) was sent to every doc. Entity probe: no return
carried `&lt;` / `&gt;` / `&amp;` (read by eye; every return is plain text).

## Verdicts
- architecture — 1 proposal (A1)
- security-plan — 4 proposals (S1 primary; S2–S4 dependent-of D-security-subprocess); plus 2 notes (the two ordinal
  `seventh` hits stay; the `:120` speech-log ingest closed-set list could gain `input`)
- design-system — `proposals: []`; stripping removed a comment block (per-detector no-hit notes) → raw twin
  `.raw-fanout-design-system.md`
- layout-templates — `proposals: []`; stripping removed a comment block → raw twin `.raw-fanout-layout-templates.md`
- test-plan — 2 proposals (T1 primary; T2 dependent-of D-platform-claim)
- obs-plan — `proposals: []`; stripping removed a comment block → raw twin `.raw-fanout-obs-plan.md`
- a11y-plan — 3 proposals (Y1 primary; Y2 dependent-of D-platform-claim; Y3 primary); plus comment notes (no hit on
  D-a11y-surface / D-a11y-obs-schema)

## Parsed proposals

### architecture
- **A1** · D-arch-resources · warning · §Cross-cutting Patterns → Trust boundary · change: the `sr*` suites additionally
  spawn the host NVDA, the fixed-argv window-activation script AND a per-key fixed-argv `send-keys.ps1` key-send spawn
  (closed `-Key` set, foreground-guarded, exit 4 = nothing sent); "none of which opens a listener (measured
  2026-09-02; send-keys 2026-09-30)" · basis architecture.md:261; report Symbols → Processes.

### security-plan
- **S1** · D-security-subprocess · escalate · §Security Anti-Patterns → Code Patterns → rule (b) · change: seven → eight
  governed forms; register the EIGHTH as `send-keys.ps1` (the `sr*` legs' per-key OS input spawn; `spawnSync('powershell.exe',
  [...,'-File', <send-keys.ps1>, '-Key', <constant>])`, `windowsHide`, 20 s; closed `ValidateSet`; exit 4 / exit 5; a
  non-zero exit throws, no injected fallback; dev-only driver-stack locus; no listener; founder-ratified «Да делай»
  under playbook :124; array form, so the seventh stays the sole array-SHAPE exception) · basis :367 @c2013.
- **S2** · dependent-of D-security-subprocess · change: "The four driver-stack forms" → five, naming the OS-key form
  beside the window-activation form · basis :367.
- **S3** · dependent · change: the `secret_scan_gate` carve-out's "the count stays seven" → "moves no count" (no fresh
  literal) · basis :367 @c8518.
- **S4** · dependent · change: the W/153 carve-out's "no committed form moved, and the count stays seven" → no committed
  form moved BY THEM; their «Да» stays scoped to those controls; the eighth form is a separate ratification · basis
  :367 @c9470.

### test-plan
- **T1** · D-platform-claim · warning · §1 Untestable zones (screen-reader BROWSE-MODE) · change: narrow, not delete —
  browse rows whose window carries an OS key are agent-driven via `send-keys.ps1`; only rows with no OS key stay
  operator findings (`not-run-here`, `BROWSE_NOT_DRIVEN`); keep the measured fact that injected keys never deliver
  browse commands; "14 rows" literal → the set · quoted stating sentence :47 "not a missing driver but a missing key
  path — OS-level key injection … would make these rows agent-driven (a route-owned CARRY); until then they are
  recorded findings".
- **T2** · dependent-of D-platform-claim · §6 desktop-webview row, third family · change: "the browse-mode rows stay
  findings pending OS-level key injection" → rows driven by an OS key graded on the agent arm, the rest findings; the
  firing form gains the per-key `send-keys.ps1` beside `activate-window.ps1`; the reset cycle and every other key stay
  injected; each row records `input` · quoted :307 @c10435.

### a11y-plan
- **Y1** · D-platform-claim · warning · §3 Screen reader test pattern, browse-mode clause · change: "browse-mode rows stay
  pending OS-level key injection (… 14 rows recorded as findings …)" → the agent arm sends Tab / Shift+Tab / h / d /
  ArrowDown as OS keys (`send-keys.ps1`); browse mode IS reachable on that path; the 8 browse rows driven there are
  graded on the agent arm (6 heard `announced-differently`, 2 `not-announced`); only the 7 with no OS key stay operator
  findings; injected keys still do not reach browse mode; configuration named · quoted a11y-plan:268.
- **Y2** · dependent-of D-platform-claim · §1 Scope Summary → harness specification → Screen reader test pattern ·
  change: "browse-mode rows pending OS-level injection" → the OS path exists; only undriven browse rows stay operator
  findings · basis a11y-plan:113.
- **Y3** · D-platform-claim · warning · §3 Screen reader test pattern, the focus verdict / confound clause · change: the
  confound resolves to the INPUT PATH (C1: injected 0/5 + 0/4, OS 5/5 under the leg's own driver launch; mechanism
  recorded, not established); the agent arm hears focus on the OS path (24 → 2, both content findings); the platform
  set gains 154.0.4258.37 on the OS path; the verdict bound to C1's configuration; "a physical keyboard is unmeasured"
  keeps its clause and gains "not run — founder ruling, cause already isolated by C1; retired, not pending" · quoted
  a11y-plan:268.

## Validate — dispositions
| id | disposition | deciding check |
|---|---|---|
| A1 | apply — escalation resolved | check 1: playbook `:124` *Boundary widening* (a subprocess boundary gains a new crossing) → escalate; RESOLVED by the founder's live ratification «Да делай» (2026-09-30, relayed verbatim by the overseer), whose relay (`conductor-wrap-osinput-2026-09-30` §2) names "the arch registration of the new committed spawn". Recorded in the sidecar. |
| S1 | apply — escalation resolved | check 1: playbook `:124` → escalate; RESOLVED by the same ratification, which names rule (b) seven → eight. The W/153 «Да» stays scoped to those controls. |
| S2 | apply — escalation resolved | dependent of S1 (atomic group); the same ratification. |
| S3 | apply — escalation resolved | dependent of S1; check 4 read the offset window @c8518 (the carve-out states the count in the present tense). |
| S4 | apply — escalation resolved | dependent of S1; window @c9470. |
| T1 | apply | check 1: playbook `:149` (a master's own explicitly-provisional claim — "until then", "a route-owned CARRY" — retired by the measurement the sentence names as its precondition; the report carries it) → routine. Check 5: expected entry 2. |
| T2 | apply | dependent of T1; the same rule. Check 5: expected entry 2. |
| Y1 | apply | check 1: playbook `:149` → routine ("pending OS-level key injection" names its own precondition). Check 5: expected entry 1. |
| Y2 | apply | dependent of Y1. |
| Y3 | apply | check 1: playbook `:149` for the confound clause (a stated-open discriminator, retired by C1, which the report carries). The arm-K status rides the founder's recorded ruling (the direction settles it). Check 6: disproved claim 1. |
| O1 (orchestrator-raised) | apply | security-plan §Input Validation speech-log ingest row (`:120`): its closed sets gain `input` (`os|webdriver|mixed|none`). The report's Schema bullet carries the fact; playbook `:308` *Accurate this-chunk addition* → routine. |

- Check 2 (cross-contradiction): none; no two proposals edit one section in opposing directions.
- Check 3 (intent-consistency): the report's six deviations are justified. Deviations 1–4 rest on the overseer's in-session words, 5 is the plan's own leg-mirroring, and 6 rests on the relay and the founder's K ruling. Scope record: none (`gate.py scope` clean). No escalation.
- Check 4 (absence): A1's "no second site" claim and T1/Y1's "stating sentence" claims are re-checked by the cascade sweep (`cascade-dispositions.md`), which enumerates every hit.
- Check 5 (expected amendments): all four plan entries are proposed (Y1/Y3 ← a11y §3; T1/T2 ← test §1/§6; S1–S4 ← security rule (b); A1 ← arch Trust boundary). The fifth entry, "no amendment for C1", stays not carried: no proposal names C1.
- Check 6 (disproved claims):
  - #1 (confound) → Y3;
  - #2 (browse reachable) → Y1, Y2, T1, T2;
  - #3 (the plan's 24 → 0 forecast) → no master states it; disposed as recorded in `evidence/confound-control.md` §Reds and the report Outcome.

Escalations: 5 (A1, S1–S4), all resolved on the founder's quoted ratification before apply. None remains open.
