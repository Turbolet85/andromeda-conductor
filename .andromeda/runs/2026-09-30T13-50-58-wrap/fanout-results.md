# Fan-out results — 2026-09-30-the-sr-cause-isolated-on-this-host

Seven Explore doc-agents, one parallel batch, prompts built by `build-prompts.py` from the verbatim template (0
unsubstituted placeholders; no master migrated, so no contracts line). Every return was plain YAML plus `#` notes;
stripping removed only the notes (their substance is summarised per doc below). No return carried an HTML entity.

## Verdicts
- **architecture** — 5 proposals (AR1 D-arch-resources · AR2 D-arch-collision · AR3 dependent-of AR2 · AR4, AR5
  D-arch-collision). Notes: D-arch-decisions no hit (no dependency; the 153/153 pair is coherent, arch:212);
  D-arch-registry-size is the orchestrator's post-apply tooling check; D-platform-claim no stating sentence (arch
  :60/:212/:253 state CI coherence and endpoint readings, not the SR cause).
- **security-plan** — `proposals: []`. Notes: no detector covers expected amendment 5 (D-security-subprocess is
  sidecar-only, D-security-deps keys on the empty Dependencies bullet) — rule (b) is incomplete, not false; the
  `:4445` binder lists at `:72` / `:378` name only the three committed suite families.
- **design-system** — `proposals: []` (no UI; no count moved; no stating sentence — hits :63 :77 :88 :197 :261
  :360 name tools/surfaces only).
- **layout-templates** — `proposals: []` (no surface; no count; :139 :188 :191 state no retired verdict).
- **test-plan** — 1 proposal (T1 D-platform-claim). Notes: :47 / :307's "injected keys never reach browse mode" is
  CONFIRMED by W (0 `Input:`) and S (OS keys logged), not retired; §1 :56 platform set unchanged.
- **obs-plan** — `proposals: []` (no operation, no dependency, no logging write, no CI gate; :234 :350 :621 name
  tools only).
- **a11y-plan** — 2 proposals (A1, A2, both D-platform-claim, both `:268`). Notes: the verdict and the candidate list
  appear only at :268 (sweep: first burst · focus change · hears/heard · CONFIGURATION-BOUND · 154.0.4258.37 ·
  26200 · NVDA 2026 · stay candidates · runtime/driver · allowInChromium · not-announced; :83 :113 :298 :465 read);
  the browse-mode clause not proposed (the S finding says "may be" reachable; routed to route-resolve).

## Proposals and dispositions

### A1 — a11y-plan §3 Screen reader test pattern (`:268`) — D-platform-claim, warning
Scope the focus silence to the WebDriver-injected, driver-launched path; retire "so an NVDA user on that
configuration hears no focus change in Conductor after the first burst" with the OS-path reading (S-conductor);
keep the confound unseparated. **Disposition: APPLY** — check 1: no playbook rule governs (:149 fails clause (a):
the sentence is stated as fact, not as a provisional claim naming its precondition); the operator's RECORDED
direction settles it (relay `conductor-wrap-srcause-2026-09-30.md` §1: "The user consequence is corrected … Write
that correction where the earlier claim sits, in the words the earlier claim used"); a rule is proposed at the wrap
card. Checks 2-6 pass (no opposing proposal; intent = the chunk's entry; the sweep is cited; expected amendments
1-2; disproved claim 1).

### A2 — a11y-plan §3 (`:268`) — D-platform-claim, warning
Retire "the runtime/driver pair … stay candidates": R153-empty silent at 153.0.4234.48 / 153.0.4234.48; the
cumulative updates and the desktop remain against the 2026-09-07 heard run. **Disposition: APPLY** — check 1:
:149 marginal (the sentence names candidates, not a precondition — clause (a) not met) → no match; settled by the
relay's recorded direction (§1: "So the runtime pair is ruled out"). Expected amendment 1; disproved claim 2.

### T1 — test-plan §6 desktop-webview row (`:307`) — D-platform-claim, warning
The measured driver × runtime PAIR set gains the dev-host {153.0.4234.48 × 153.0.4234.48} pair (sr-empty CONTROL,
loader-folder runtime, operator-supplied Authenticode-verified driver, «Да» quoted). **Disposition: ESCALATE (E1)** —
check 1: playbook :124 *Boundary widening* matches (the proposal records the ratified 153 crossing) → escalate,
never routine. Expected amendment 4.

### AR1 — architecture §Occupied Resources → Environment variables (`:202`) — D-arch-resources, warning
Register `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` beside the two loader handles: dev host, per command, 153 CONTROL,
no committed reader, «Да» quoted. **Disposition: ESCALATE (E1)** — check 1: :124 matches (records the ratified
loader crossing); :137 (external handle registered only when a SHIPPED artifact names it) FAILS its precondition —
only the plan, research, report and chunk evidence name this handle — so :137 is not a participant, and its own
note ("a handle named only in a chunk report, a plan, a research note … is NOT registered") is the counter-argument
the operator weighs. Expected amendment 3; relay §1 asks for «Да» in arch §Occupied Resources.

### AR2 — architecture §Occupied Resources → Ports (`:150`) — D-arch-collision, escalate
Record `:4445` bound ONCE (2026-09-30, arm W) by msedgedriver 154 from a ratified, uncommitted session script
driving the Edge browser (no tauri-driver `:4444`); the committed binder (the driver stack) unchanged.
**Disposition: ESCALATE (E1)** — escalate severity; :124 matches too.

### AR3 — architecture §Cross-cutting Patterns → Trust boundary (`:261`) — dependent-of AR2
The trust boundary's listener enumeration notes the one-off ratified `:4445` binder. **Disposition: with AR2 (E1)**
(a dependent group applies atomically).

### AR4 — architecture §Occupied Resources → Environment variables (`:193`) — D-arch-collision, escalate
`CONDUCTOR_MSEDGEDRIVER` "Read ONLY by `crates/conductor-tauri/ui/wdio.conf.ts`" → the only COMMITTED reader; the
W session script also read it. **Disposition: ESCALATE (E2).**

### AR5 — architecture §Occupied Resources → Environment variables (`:194`) — D-arch-collision, escalate
`CONDUCTOR_NVDA` "the ONLY reader" → the only COMMITTED reader; the S and W session scripts also read it.
**Disposition: ESCALATE (E2).**

### S1 — security-plan §Security Anti-Patterns → Code Patterns rule (b) (`:367`) — orchestrator-raised (check 5)
Record the ratified W crossing (session-script WebDriver session to the Edge browser on `:4445`, outside the three
loci, no committed form, the count stays seven) and the operator-supplied 153 driver's pre-execution Authenticode
admission on the dev host; «Да» quoted. **Disposition: ESCALATE (E1)** — :124 matches. Expected amendment 5; the
security doc-agent's note names exactly this gap.

## Check 5 — expected amendments, all covered
1 → A1+A2 · 2 → A1 · 3 → AR1 · 4 → T1 · 5 → S1 (raised). None under-ran.

## Check 6 — disproved claims, all disposed
1 → A1 · 2 → A2 · 3 (prior `cause-control.md`, a chunk evidence record) → routed: left unedited by plan and relay;
this chunk's evidence §USER consequence re-states it.

## Escalation resolutions (HALT, resolved WITH the operator — overseer, founder-delegated for the RECORD; the crossings were ratified live by the founder at P4)
- **E1** (T1 · AR1 · AR2+AR3 · S1) → "Security (b) + test-plan only". APPLIED: S1 (security-plan rule (b)) and T1
  (test-plan §6 pair set), each with the founder's quote in the body. REJECTED: AR1, AR2, AR3 — the operator's note:
  "the arch registry carries STANDING committed readers and binders, in the spirit of :137. W and the 153 arm were
  one-off dated controls … No arch env row and no Ports/trust-boundary edit. If the successor entry ever COMMITS a
  reader or binder, it registers it then." Expected amendment 3 → superseded by this ruling.
- **E2** (AR4 · AR5) → "Narrow to 'only committed reader'". APPLIED on both arch env rows; no session script named.
- A1, A2 → APPLIED under the relay's recorded direction; a playbook rule is PROPOSED at the wrap card, not appended.
- Tally: applied A1 · A2 · T1 · S1 · AR4 · AR5 (4 sidecar entries: a11y-plan, test-plan, security-plan, architecture) ·
  rejected AR1 · AR2 · AR3 · escalations resolved 2 (E1, E2) · open 0.
- `D-arch-registry-size` after apply: §Established Decisions 38111 B · §Occupied Resources 38110 B · threshold 38115 B —
  within target.
